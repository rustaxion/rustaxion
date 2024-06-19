#![allow(unused)]

use std::env;
use std::error::Error;

use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Decoder, Encoder, Framed};

mod enums;
mod types;
mod proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6969".to_string());

    let server = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (stream, _) = server.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process(stream).await {
                println!("failed to process connection; error = {}", e);
            }
        });
    }
}

async fn process(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut transport = Framed::new(stream, types::PacketGlue);

    while let Some(request) = transport.next().await {
        match request {
            Ok(packet) => {
                println!("{:?}", packet)
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}
