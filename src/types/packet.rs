use std::alloc;
use std::alloc::Layout;
use std::io::{Cursor, Read, Write};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};
use crate::types::comet::{MainCmd, ParaCmd};

pub struct PacketGlue;

#[derive(Debug)]
pub struct Packet {
    pkg_len: i32,
    main_cmd: MainCmd,
    para_cmd: ParaCmd,
    data_len: i16,
    data: Vec<u8>
}

impl Encoder<Packet> for PacketGlue {
    type Error = io::Error;

    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> io::Result<()> {
        use std::fmt::Write;

        dst.extend_from_slice(item.pkg_len.to_le_bytes().as_slice());
        dst.put_u8(item.main_cmd.get_value() as u8);
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
        use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};

        let mut reader = src.reader();
        let mut reader = reader.get_mut();

        let mut reader = Cursor::new(reader);

        let pkg_len = ReadBytesExt::read_i32::<LittleEndian>(&mut reader)?;

        let main_cmd = MainCmd::from_value(ReadBytesExt::read_i8(&mut reader)?)?;
        let para_cmd = ParaCmd::from_value(&main_cmd, ReadBytesExt::read_u8(&mut reader)?)?;

        let data_len = ReadBytesExt::read_i16::<LittleEndian>(&mut reader)?;

        let mut data = vec![];
        let read_len = Read::read_to_end(&mut reader, &mut data)?;

        if data_len != read_len as i16 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, format!("Unexpected data length, expected {} bytes, got {}", data_len, read_len)))
        }

        Ok(Some(Packet {
            pkg_len,
            main_cmd,
            para_cmd,
            data_len,
            data
        }))
    }
}
