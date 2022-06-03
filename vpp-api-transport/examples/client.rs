use latest_vpp_api::vlib::{CliInband, CliInbandReply};
use std::convert::TryInto;
use vpp_api_message::VppApiMessage;
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
        .client_index(0) // need to make this not panic the build() step below if unset.
        .context(0) // same here
        .cmd("show version".try_into().unwrap())
        .build()
        .unwrap();

    let response: CliInbandReply = client.send_recv_one(request).unwrap();

    print!("Reply {:?}", response);
}
