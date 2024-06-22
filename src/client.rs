#![allow(unused)]

use crate::{
    enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd},
    proto::comet_login::{LanguageType, PlatformType, ReqGameVersion},
    types::response::Response,
};
use byteorder::{LittleEndian, ReadBytesExt};
use prost::{bytes::Buf, Message};
use std::{
    alloc::GlobalAlloc,
    io::{Cursor, Read},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_util::bytes::{BufMut, BytesMut};
use types::packet::{Packet, PACKET_HEADER_SIZE};

mod enums;
mod proto;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6969").await?;
    let req = ReqGameVersion {
        r#type: PlatformType::Android as i32,
        language: LanguageType::DefaultLanguage as i32,
    };

    let packet = Into::<Packet>::into(Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::RequestGameVersion),
        body: req.encode_to_vec(),
    });

    stream.write_all(packet.encode()?.as_slice()).await?;
    stream.flush().await?;

    let mut response: Vec<u8> = vec![];
    stream.read_to_end(&mut response).await?;

    let response = Packet::decode(&mut BytesMut::from(response.as_slice()))?;
    println!("{:?}", response);

    Ok(())
}
