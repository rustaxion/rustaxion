#![allow(unused)]

use anyhow::Context;
use dotenvy::dotenv;
use futures_util::{ future, StreamExt };
use prost::bytes::BytesMut;
use sea_orm::{ ConnectOptions, Database, DatabaseConnection };
use std::collections::VecDeque;
use std::error::Error;
use std::net::SocketAddr;
use std::panic;
use std::{ backtrace::Backtrace, env };
use tokio::io::AsyncWriteExt;
use tokio::{ net::{ TcpListener, TcpStream }, task::futures };
use tokio_util::codec::{ Decoder, Encoder, Framed };
use types::packet::PacketDecodeError;
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

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port: u16 = env::var("PORT").unwrap_or("6969".to_string()).parse()?;

    let addr = format!("{}:{}", host, port);
    let server = TcpListener::bind(&addr).await?;
    println!("Listening on: tcp://{}", addr);

    let mut last_session_id = 0;

    loop {
        let (stream, addr) = server.accept().await?;
        let db = db.clone();

        tokio::spawn(async move {
            if let Err(e) = process(stream, addr, db).await {
                eprintln!("\nError: {}", indent::indent_by(4, format!("{:?}", e)));
            }
        });
    }
}

async fn process(
    stream: TcpStream,
    addr: SocketAddr,
    db: DatabaseConnection
) -> anyhow::Result<()> {
    use futures_util::sink::SinkExt;

    let mut transport = Framed::new(stream, types::packet::PacketGlue);
    let mut session = SessionData::new();

    while let Some(request) = transport.next().await {
        let packet = request.context("Failed to parse an incoming packet.")?;
        eprintln!("-> {:?}::{:?}", packet.main_cmd, packet.para_cmd);

        let responses = server::handle(&mut session, db.clone(), packet).await?;
        eprintln!("-- {:?}", session);

        for resp in responses {
            let packet = Into::<Packet>::into(resp);
            eprintln!("<- {:?}::{:?}", packet.main_cmd, packet.para_cmd);
            transport.send(packet).await?;
        }
    }

    Ok(())
}
