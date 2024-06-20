use crate::{
    enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd},
    proto::comet_login::{LanguageType, PlatformType, ReqGameVersion},
    types::response::Response,
};
use byteorder::{LittleEndian, ReadBytesExt};
use prost::{bytes::Buf, Message};
use std::io::{Cursor, Read};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_util::bytes::{BufMut, BytesMut};
use types::packet::Packet;

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

    let mut dst = BytesMut::new();
    dst.extend_from_slice(packet.pkg_len.to_le_bytes().as_slice());
    dst.put_u8(i8::from(packet.main_cmd.clone()) as u8);
    dst.put_u8(packet.para_cmd.get_value());
    dst.extend_from_slice(packet.data_len.to_le_bytes().as_slice());

    dst.extend_from_slice(packet.data.as_slice());

    stream.write(dst.to_vec().as_slice()).await?;
    stream.flush().await?;

    let mut response: Vec<u8> = vec![];
    stream.read_to_end(&mut response).await?;

    let bytes = BytesMut::from(response.as_slice());
    let mut reader = bytes.reader();
    let reader = reader.get_mut();

    let mut reader = Cursor::new(reader);

    let pkg_len = ReadBytesExt::read_i32::<LittleEndian>(&mut reader)?;

    let main_cmd = MainCmd::try_from(ReadBytesExt::read_i8(&mut reader)?);
    let Ok(main_cmd) = main_cmd else {
        panic!();
    };

    let para_cmd = ParaCmd::from_value(&main_cmd, ReadBytesExt::read_u8(&mut reader)?);
    let Ok(para_cmd) = para_cmd else { panic!() };

    let data_len = ReadBytesExt::read_i16::<LittleEndian>(&mut reader)?;

    let mut data = Vec::<u8>::with_capacity(data_len as usize);
    Read::read_to_end(&mut reader, &mut data)?;

    let response = Packet {
        pkg_len,
        main_cmd,
        para_cmd,
        data_len,
        data,
    };

    println!("{:?}", response);

    Ok(())
}
