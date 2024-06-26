use crate::enums::comet::{ MainCmd, ParaCmd };
use super::packet::{ Packet, PACKET_HEADER_SIZE };

#[derive(Debug, Clone)]
pub struct Response {
    pub main_cmd: MainCmd,
    pub para_cmd: ParaCmd,
    pub body: Vec<u8>,
}

#[rustfmt::skip]
impl Into<Packet> for Response {
    fn into(self) -> Packet {
        let pkg_len = (PACKET_HEADER_SIZE + self.body.len()) as i32;

        Packet {
            pkg_len,
            main_cmd: self.main_cmd,
            para_cmd: self.para_cmd,
            data_len: self.body.len() as u16,
            data: self.body
        }
    }
}
