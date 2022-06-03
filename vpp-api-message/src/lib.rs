pub trait VppApiMessage {
    fn get_message_name_and_crc() -> String;
}

pub trait VppApiResponse: VppApiMessage {}
pub trait VppApiRequest: VppApiMessage {
    fn set_client_index(&mut self, client_index: u32);
    // fn set_context(&mut self, context: u32);
}
