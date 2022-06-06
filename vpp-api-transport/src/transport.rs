use bincode::Options;
use log::info;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use vpp_api_message::{VppApiMessage, VppApiRequest, VppApiRequestBuilder, VppApiResponse};

use crate::{
    connection::{Connection, ConnectionContext, MsgFrame},
    error::Result,
    get_encoder,
};

//
// #[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
// #[message_name_and_crc(sw_interface_set_flags_f5aec1b8)]
// pub struct SwInterfaceSetFlags {
// 	pub client_index : u32,
// 	pub context : u32,
// 	pub sw_if_index : InterfaceIndex,
// 	 pub flags : EnumFlag<IfStatusFlags>,
// }
// #[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
// #[message_name_and_crc(sw_interface_set_flags_reply_e8d4e804)]
// pub struct SwInterfaceSetFlagsReply {
// 	pub context : u32,
// 	pub retval : i32,
// }
pub trait VppMessage: Serialize {
    fn as_frame(&self, msg_id: u16) -> MsgFrame<u16> {
        let message = get_encoder().serialize(self).unwrap();
        MsgFrame {
            header: msg_id,
            message,
        }
    }
    fn get_message_name_and_crc(&self) -> String;
}
impl<M> VppMessage for M
where
    M: VppApiMessage + Serialize,
{
    fn get_message_name_and_crc(&self) -> String {
        Self::get_message_name_and_crc()
    }
}
pub trait VppRequest: VppMessage + VppApiRequest + Serialize + Debug {}
pub trait VppResponse: VppMessage + VppApiResponse + DeserializeOwned + Debug {}

impl<M> VppRequest for M where M: VppMessage + VppApiRequest + Serialize + Debug {}
impl<M> VppResponse for M where M: VppApiResponse + Serialize + DeserializeOwned + Debug {}

// for some reason this conflicts iwth the above impl of VppMessage
// impl<B> VppMessage for B
// where
//     B: VppApiRequestBuilder,
// {
//     fn as_frame(&self, msg_id: u16) -> MsgFrame<u16> {
//         let vppmsg = self.build().unwrap();
//         let message = get_encoder().serialize(vppmsg).unwrap();
//         MsgFrame {
//             header: msg_id,
//             message,
//         }
//     }
// }

pub trait Transport {
    fn send<M: VppRequest>(&mut self, message: M) -> Result<()>;
    fn recv<M: VppResponse>(&mut self) -> Result<M>;
}

pub struct DefaultTransport<C> {
    ctx: ConnectionContext,
    conn: C,
}

impl<C> DefaultTransport<C>
where
    C: Connection,
{
    pub fn new(mut conn: C) -> Self {
        let ctx = conn.connect("default transport").unwrap();

        Self { ctx, conn }
    }
}

impl<C> Transport for DefaultTransport<C>
where
    C: Connection,
{
    fn send<M: VppRequest>(&mut self, mut message: M) -> Result<()> {
        println!("Setting client index on message: {:?}", &message);
        let name = message.get_message_name_and_crc();
        message.set_client_index(self.ctx.client_index);

        println!("Set client index on message: {:?}", &message);
        match self.ctx.get_msg_index(&name) {
            Some(vl_msg_id) => self.conn.write_msg(message.as_frame(vl_msg_id)),
            None => Err(format!("could not find message: {}", name,).into()),
        }
    }

    fn recv<M: VppResponse>(&mut self) -> Result<M> {
        println!("Attempt to read message");
        let frame = self.conn.read_msg()?;
        println!("read message.");

        let MsgFrame { header, message } = frame;
        println!("Got frame: {:?}", &message);
        let result = get_encoder()
            .allow_trailing_bytes()
            .deserialize::<M>(&message)?;
        Ok(result)
    }
}
