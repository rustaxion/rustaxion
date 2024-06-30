use anyhow::Context;
use futures_util::StreamExt;
use moka::future::Cache;
use sea_orm::DatabaseConnection;
use std::{ env, time::Duration };
use std::net::SocketAddr;
use tokio::net::{ TcpListener, TcpStream };
use tokio_util::codec::Framed;
use types::{ packet::Packet, session::SessionData };

mod enums;
mod proto;
mod server;
mod types;
mod database;

#[allow(non_snake_case)]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename("env/app.ini").ok();
    dotenvy::from_filename("env/database.ini").ok();

    tracing_subscriber::fmt::init();
    color_backtrace::install();

    let db = crate::database::establish_connection().await?;
    let cache = Cache::<i64, SessionData>
        ::builder()
        .time_to_idle(Duration::from_secs(5 * 60))
        .initial_capacity(1_000)
        .build();

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port: u16 = env::var("PORT").unwrap_or("20017".to_string()).parse()?;

    let addr = format!("{}:{}", host, port);
    let server = TcpListener::bind(&addr).await?;
    println!("Listening on: tcp://{}", addr);

    loop {
        let (stream, addr) = server.accept().await?;

        let cache = cache.clone();
        let db = db.clone();

        tokio::spawn(async move {
            if let Err(e) = process(stream, addr, cache, db).await {
                eprintln!("\nError: {}", indent::indent_by(4, format!("{:?}", e)));
            }
        });
    }
}

async fn process(
    stream: TcpStream,
    _addr: SocketAddr,
    cache: Cache<i64, SessionData>,
    db: DatabaseConnection
) -> anyhow::Result<()> {
    use futures_util::sink::SinkExt;

    let mut transport = Framed::new(stream, types::packet::PacketGlue);
    let mut session = SessionData::new();

    while let Some(request) = transport.next().await {
        let session_snapshot = session.clone();

        let packet = request.context("Failed to parse an incoming packet.")?;
        eprintln!("-> {:?}::{:?}", packet.main_cmd, packet.para_cmd);

        let responses = server::handle(&mut session, db.clone(), packet).await?;

        for resp in responses {
            let packet = Into::<Packet>::into(resp);
            eprintln!("<- {:?}::{:?}", packet.main_cmd, packet.para_cmd);
            transport.send(packet).await?;
        }

        if session != session_snapshot && session.player_id.is_some() {
            cache.insert(session.player_id.unwrap(), session.clone()).await;
        }
    }

    if session.player_id.is_some() {
        cache.invalidate(&session.player_id.unwrap()).await;
    }

    Ok(())
}
