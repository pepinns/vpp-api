use crate::transport::{VppRequest, VppResponse};
use crate::{error::Result, transport::Transport};
use log::info;

pub struct Client<T> {
    transport: T,
}

impl<T> Client<T>
where
    T: Transport,
{
    pub fn new(transport: T) -> Self {
        Self { transport }
    }
    pub fn send_recv_one<M: VppRequest, R: VppResponse>(&mut self, message: M) -> Result<R> {
        info!("Sending message: {:?}", message);
        self.transport.send(message)?;
        info!("Sent message.");
        self.transport.recv()
    }
}
