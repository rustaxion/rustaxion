#![allow(unused)]

use std::collections::VecDeque;
use std::error::Error;
use std::net::SocketAddr;
use std::panic;
use std::{backtrace::Backtrace, env};

use anyhow::Context;
use futures_util::future;
use tokio::{
    net::{TcpListener, TcpStream},
    task::futures,
};
use tokio_util::codec::{Decoder, Encoder, Framed};
use types::{packet::Packet, session::SessionData};

mod enums;
mod proto;
mod server;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    panic::set_hook(Box::new(|info| {
        let stacktrace = Backtrace::force_capture();
        println!("Got panic. @info:{}\n@stackTrace:{}", info, stacktrace);
        std::process::abort();
    }));

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6969".to_string());

    let server = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (stream, addr) = server.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process(stream, addr).await {
                println!("failed to process connection; error = {}", e);
            }
        });
    }
}

async fn process(stream: TcpStream, addr: SocketAddr) -> anyhow::Result<()> {
    use futures_util::sink::SinkExt;

    let mut transport = Framed::new(stream, types::PacketGlue);
    let mut session = SessionData {};

    while let Some(request) = tokio_stream::StreamExt::next(&mut transport).await {
        let packet = request.context("Failed to parse an incoming packet.")?;
        println!("Req {:?}", packet);

        let responses = server::handle(&mut session, packet)?;

        for resp in responses {
            let packet = Into::<Packet>::into(resp);
            println!("Resp {:?}", packet);
            transport.send(packet).await?;
        }
    }

    Ok(())
}
