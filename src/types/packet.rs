use std::io::{Cursor, Read};
use std::mem::size_of;

use tokio::io::{self, AsyncReadExt};
use tokio_util::bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::enums::comet::{MainCmd, ParaCmd};

pub struct PacketGlue;

#[derive(Debug, Clone)]
pub struct Packet {
    pub pkg_len: i32,
    pub main_cmd: MainCmd,
    pub para_cmd: ParaCmd,
    pub data_len: u16,
    pub data: Vec<u8>,
}

pub const PACKET_HEADER_SIZE: usize =
    /* pkg_len */ std::mem::size_of::<i32>() + 
    /* main_cmd */ std::mem::size_of::<i8>() + 
    /* para_cmd */ std::mem::size_of::<u8>() + 
    /* data_len */ std::mem::size_of::<u16>();

impl Encoder<Packet> for PacketGlue {
    type Error = io::Error;

    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> io::Result<()> {
        dst.extend_from_slice(item.pkg_len.to_le_bytes().as_slice());
        dst.put_u8(i8::from(item.main_cmd.clone()) as u8);
        dst.put_u8(item.para_cmd.get_value());
        dst.extend_from_slice(item.data_len.to_le_bytes().as_slice());

        dst.extend_from_slice(&*item.data);

        return Ok(());
    }
}

impl Decoder for PacketGlue {
    type Item = Packet;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        use byteorder::{LittleEndian, ReadBytesExt};

        let mut reader = src.reader();
        let reader = reader.get_mut();

        let mut reader = Cursor::new(reader);

        let pkg_len = ReadBytesExt::read_i32::<LittleEndian>(&mut reader)?;

        let main_cmd = MainCmd::try_from(ReadBytesExt::read_i8(&mut reader)?);
        let Ok(main_cmd) = main_cmd else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not valid for MainCmd", main_cmd.unwrap_err()),
            ));
        };

        let para_cmd = ParaCmd::from_value(&main_cmd, ReadBytesExt::read_u8(&mut reader)?);
        let Ok(para_cmd) = para_cmd else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not valid for ParaCmd", para_cmd.unwrap_err()),
            ));
        };

        let data_len = ReadBytesExt::read_u16::<LittleEndian>(&mut reader)?;
        let mut data = vec![0; data_len as usize];

        std::io::Read::read_exact(&mut reader, &mut data)?;
        assert_eq!(pkg_len as usize, PACKET_HEADER_SIZE - std::mem::size_of::<i32>() + data.len());

        Ok(Some(Packet {
            pkg_len,
            main_cmd,
            para_cmd,
            data_len,
            data,
        }))
    }
}
