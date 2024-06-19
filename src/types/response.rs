use prost::Message;

use crate::enums::comet::{MainCmd, ParaCmd};

use super::packet::Packet;

#[derive(Debug)]
pub struct Response {
    pub main_cmd: MainCmd,
    pub para_cmd: ParaCmd,
    pub body: Vec<u8>,
}

#[rustfmt::skip]
impl Into<Packet> for Response {
    fn into(self) -> Packet {
        let pkg_len: i32 =
            (/* u32 */ 32 / 8) +
            (/* i8 */ 8/8) +
            (/* u8 */ 8/8) +
            (/* i16 */ 16 / 8) +
            (/* ?? */ self.body.len() as i32);

        Packet {
            pkg_len,
            main_cmd: self.main_cmd,
            para_cmd: self.para_cmd,
            data_len: self.body.len() as i16,
            data: self.body
        }
    }
}
