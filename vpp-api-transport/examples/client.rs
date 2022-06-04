use latest_vpp_api::vlib::{CliInband, CliInbandReply};
use std::convert::TryInto;
use vpp_api_message::VppApiMessage;
use vpp_api_message::VppApiRequest;
use vpp_api_transport::{
    afunix,
    client::Client,
    connection::Connection,
    transport::{DefaultTransport, Transport},
};

fn main() {
    let conn = afunix::Transport::new("/var/run/vpp/api.sock");
    let transport = DefaultTransport::new(conn);
    let mut client = Client::new(transport);

    let request = CliInband::builder()
        .cmd("show version".try_into().unwrap())
        .build()
        .unwrap();

    let response: CliInbandReply = client.send_recv_one(request).unwrap();

    print!("Reply {:?}", response);

    let request_builder = CliInband::builder().cmd("show version".try_into().unwrap());
    let response: CliInbandReply = client.send_recv_one(request_builder).unwrap();

    print!("Reply {:?}", response);
}
