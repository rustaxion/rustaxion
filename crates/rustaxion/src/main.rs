#![allow(unused_imports)]

use anyhow::Context;
use futures_util::{
    future::{self, abortable},
    stream::Aborted,
    StreamExt, TryStreamExt,
};
use moka::future::Cache;
use prost::bytes::BytesMut;
use proto::packet::Packet;
use sea_orm::DatabaseConnection;
use std::{env, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    select,
    sync::Mutex,
};
use tokio_util::{codec::Framed, sync::CancellationToken};
use types::session::{self, SessionData};

mod database;
mod server;
mod types;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename(".env").ok();
    color_backtrace::install();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let db = crate::database::establish_connection().await?;
    let cache = Cache::<i32, SessionData>::builder()
        .time_to_idle(Duration::from_secs(5 * 60))
        .initial_capacity(1_000)
        .build();

    let cancellation_token = CancellationToken::new();
    let ctrl_cancel = cancellation_token.clone();

    let host = env::var("APP_HOST").unwrap();
    let port: u16 = env::var("APP_PORT").unwrap().parse().unwrap();

    let addr = format!("{}:{}", host, port);
    let server = TcpListener::bind(&addr).await?;

    println!("Listening on: tcp://{}", addr);

    let (task, handle) = abortable(async move {
        loop {
            if cancellation_token.is_cancelled() {
                break;
            }

            let (stream, addr) = server.accept().await?;
            println!("-  Incoming connection from: {}:{}", addr.ip(), addr.port());

            let cache = cache.clone();
            let db = db.clone();
            let ctoken = cancellation_token.clone();

            tokio::spawn(async move {
                if let Err(e) = process(stream, addr, cache, db, ctoken).await {
                    eprintln!("\nError: {}", indent::indent_by(4, format!("{:?}", e)));
                }
            });
        }

        Ok::<(), anyhow::Error>(())
    });

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        ctrl_cancel.cancel();
        handle.abort();
    });

    match tokio::spawn(task).await? {
        Ok(res) => {
            return res;
        }
        Err(Aborted) => { /* do nothing */ }
    }

    println!("Shutting down server...");

    Ok(())
}

async fn process(
    stream: TcpStream,
    _addr: SocketAddr,
    cache: Cache<i32, SessionData>,
    db: DatabaseConnection,
    cancellation_token: CancellationToken,
) -> anyhow::Result<()> {
    use futures_util::sink::SinkExt;

    let ws = tokio_tungstenite::accept_async(stream).await.unwrap();
    let (mut write, mut read) = ws.split();

    let session = Arc::new(Mutex::new(SessionData::new()));
    let cache = Arc::new(Mutex::new(cache));

    loop {
        let msg: tungstenite::Message;
        select! {
            _ = cancellation_token.cancelled() => {
                break;
            }

            next = read.next() => {
                if next.is_none() {break;}
                let next = next.unwrap();

                if next.is_err() {
                    eprintln!("Error: {:?}", next.err().unwrap());
                    break;
                }

                msg = next.unwrap();
            }
        }

        if msg.is_close() {
            break;
        }

        if !msg.is_binary() {
            eprintln!("Received a non-binary message: {:?}", msg);
            continue;
        }

        let bytes = msg.into_data();
        let request = Packet::decode(&mut BytesMut::from_iter(bytes.iter()));

        let packet = request
            .context("Failed to parse an incoming packet.")
            .unwrap();
        eprintln!("-> {:?}::{:?}", packet.main_cmd, packet.para_cmd);

        let responses = server::handle(session.clone(), db.clone(), packet)
            .await
            .unwrap();

        for resp in responses {
            let packet = Into::<Packet>::into(resp);
            eprintln!("<- {:?}::{:?}", packet.main_cmd, packet.para_cmd);
            write
                .send(tungstenite::Message::binary(packet.encode().unwrap()))
                .await
                .unwrap();
        }

        let session = session.lock().await.clone();
        if session.player_id.is_some() {
            cache
                .lock()
                .await
                .insert(session.player_id.unwrap(), session.clone())
                .await;
        }
    }

    let session = session.lock().await.clone();
    if session.player_id.is_some() {
        cache
            .lock()
            .await
            .invalidate(&session.player_id.unwrap())
            .await;
    }

    Ok(())
}
