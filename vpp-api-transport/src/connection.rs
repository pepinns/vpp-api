use crate::error::Result;
use bincode::Options;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

fn get_encoder() -> impl bincode::config::Options {
    bincode::DefaultOptions::new()
        .with_big_endian()
        .with_fixint_encoding()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecvHeader {
    _q: u64,
    pub msglen: u32,
    gc_mark: u32,
}
// impl RecvHeader {
//     pub(crate) const LENGTH: usize = (3 * std::mem::size_of::<u32>()) + std::mem::size_of::<u64>();
//     // pub fn read_from(buf: &mut BytesMut) -> Result<Self> {
//     //     let h = get_encoder().deserialize(buf).unwrap();
//     //     Ok(h)
//     // }
// }

pub trait FrameHeader: Serialize {
    fn serialize(&self) -> Result<Vec<u8>> {
        let data = get_encoder().serialize(&self)?;

        Ok(data)
    }
    // fn put(&self, writer: impl io::Write) -> Result<()>;
}
impl FrameHeader for u16 {}
impl FrameHeader for RecvHeader {}

pub struct MsgFrame<H: FrameHeader> {
    pub header: H,
    pub message: Vec<u8>,
}

pub struct ConnectionContext {
    pub message_name_to_id: HashMap<String, u16>,
    pub message_max_index: u16,
    pub client_index: u32,
}
impl ConnectionContext {
    pub fn get_msg_index(&self, name: &str) -> Option<u16> {
        self.message_name_to_id.get(name).map(|x| x.to_owned())
    }
    pub fn get_table_max_index(&self) -> u16 {
        self.message_max_index
    }
    pub fn get_client_index(&self) -> u32 {
        self.client_index
    }
}

pub trait Connection {
    fn connect(&mut self, name: &str) -> Result<ConnectionContext>;
    fn disconnect(&mut self) -> Result<()>;
    fn read_msg(&mut self) -> Result<MsgFrame<RecvHeader>>;
    fn write_msg(&mut self, frame: MsgFrame<u16>) -> Result<()>;
}
