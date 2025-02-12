use proto::packet::{Packet, PacketDecodeError};
use std::io::{Cursor, Read, Write};
use thiserror::Error;

use anyhow::Context;
use tokio_util::bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use proto::enums::comet::{MainCmd, ParaCmd};

pub struct PacketGlue;

impl Encoder<Packet> for PacketGlue {
    type Error = anyhow::Error;

    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> anyhow::Result<()> {
        let _ = dst.writer().write(item.encode()?.as_slice());

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
            let Some(error) = error.downcast_ref::<PacketDecodeError>() else {
                unreachable!()
            };
            match error {
                PacketDecodeError::NoDataToRead => Ok(None),
            }
        } else {
            match error.downcast_ref::<std::io::Error>() {
                Some(io_error) => match io_error.kind() {
                    _ => {}
                },
                _ => {}
            }

            return Err(error);
        }
    }
}
