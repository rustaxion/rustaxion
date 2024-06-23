#![allow(unused)]

use crate::{
    enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd},
    proto::comet_login::{LanguageType, PlatformType, ReqGameVersion},
    types::response::Response,
};
use byteorder::{LittleEndian, ReadBytesExt};
use futures_util::{SinkExt, StreamExt};
use prost::{bytes::Buf, Message};
use std::{
    alloc::GlobalAlloc,
    io::{Cursor, Read},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_util::{
    bytes::{BufMut, BytesMut},
    codec::Framed,
};
use types::packet::Packet;

mod enums;
mod proto;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6969").await?;
    let mut transport = Framed::new(stream, types::packet::PacketGlue);

    let req = ReqGameVersion {
        r#type: PlatformType::Android as i32,
        language: LanguageType::DefaultLanguage as i32,
    };

    let packet = Into::<Packet>::into(Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::RequestGameVersion),
        body: req.encode_to_vec(),
    });

    transport.send(packet).await?;

    let response = transport.next().await;
    println!("{:?}", response);

    transport.close().await?;

    Ok(())
}
