use std::io::{Cursor, Read, Write};
use thiserror::Error;
use std::mem::size_of;

use anyhow::Context;
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

#[derive(Error, Debug)]
pub enum PacketDecodeError {
    #[error("Couldn't read the pkg_len field.")]
    NoDataToRead,
}

impl Packet {
    pub fn encode(&self) -> anyhow::Result<Vec<u8>> {
        use byteorder::{LittleEndian, BigEndian, WriteBytesExt};

        anyhow::ensure!(self.pkg_len >= PACKET_HEADER_SIZE as i32, "Can't encode a partial packet.");
        let mut writer = Vec::with_capacity(self.pkg_len as usize + std::mem::size_of::<i32>());

        writer.write_i32::<LittleEndian>(self.pkg_len)?;
        writer.write_u8(i8::from(self.main_cmd.clone()) as u8)?;
        writer.write_u8(self.para_cmd.get_value())?;
        writer.write_u16::<LittleEndian>(self.data_len)?;
        writer.write(self.data.as_slice())?;

        Ok(writer)
    }

    pub fn decode(buffer: &mut BytesMut) -> anyhow::Result<Self> {
        use byteorder::{LittleEndian, BigEndian, ReadBytesExt};

        let mut cursor = Cursor::new(buffer);

        let pkg_len = cursor.read_i32::<LittleEndian>()?;
        if pkg_len == 0 {
            return Err(PacketDecodeError::NoDataToRead.into());
        }

        let mut pkg = BytesMut::zeroed(pkg_len as usize - std::mem::size_of::<i32>());
        cursor.read_exact(&mut pkg).context(format!("Failed to read {} bytes", pkg_len))?;

        let mut cursor = Cursor::new(pkg);

        let Ok(main_cmd) = MainCmd::try_from(cursor.read_i8()?) else { anyhow::bail!("Invalid main_cmd.") };
        let Ok(para_cmd) = ParaCmd::from_value(&main_cmd, cursor.read_u8()?) else { anyhow::bail!("Invalid para_cmd.") };

        let data_len = cursor.read_u16::<LittleEndian>()?;
        let mut data = vec![0u8; data_len as usize];

        let bytes_read = cursor.read_exact(&mut data).context("Failed to Packet::data")?;

        Ok(Packet {
            pkg_len,
            main_cmd,
            para_cmd,
            data_len,
            data
        })
    }
}

impl Encoder<Packet> for PacketGlue {
    type Error = anyhow::Error;

    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> anyhow::Result<()> {
        dst.writer().write_all(item.encode()?.as_slice());

        Ok(())
    }
}

impl Decoder for PacketGlue {
    type Item = Packet;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> anyhow::Result<Option<Self::Item>> {
        let packet = Packet::decode(src);
        if packet.is_ok() {
            return Ok(Some(packet.unwrap()));
        }

        let error = packet.unwrap_err();
        if error.is::<PacketDecodeError>() {
            let Some(error) = error.downcast_ref::<PacketDecodeError>() else { unreachable!() };
            match error {
                PacketDecodeError::NoDataToRead => Ok(None),
            }
        } else {
            match error.downcast_ref::<std::io::Error>() {
                Some(io_error) => {
                    match io_error.kind() {
                        _ => {}
                    }
                },
                _ => {}
            };

            eprintln!("{:#?}", error);
            return Err(error);
        }
    }
}
