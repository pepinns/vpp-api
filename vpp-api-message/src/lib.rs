use std::error::Error;

pub trait VppApiMessage {
    fn get_message_name_and_crc() -> String;
}

pub trait VppApiResponse: VppApiMessage {}
pub trait VppApiRequest: VppApiMessage {
    fn set_client_index(&mut self, client_index: u32);
    fn set_context(&mut self, context: u32);
}
pub trait VppApiRequestBuilder {
    type Item;
    fn with_client_index(&mut self, client_index: u32) -> &mut Self;
    fn with_context(&mut self, context: u32) -> &mut Self;
    fn build(&mut self) -> Result<Self::Item, Box<dyn Error>>;
}
