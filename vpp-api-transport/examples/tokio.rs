use tokio::{self, net::UnixStream};
use tokio_tower;
use tokio_tower::pipeline::Client;
use tokio_util::codec::Framed;
use tower::Service;
use tower::ServiceExt;
use vpp_api_transport::tokio::client::{self, Codec, MsgFrame};

#[tokio::main]
async fn main() {
    let framed = client::framed("/tmp/vpp-api-sock.sock".to_string())
        .await
        .unwrap();

    // let client = tokio_tower::pipeline::Client::new(framed);
    let client: Client<
        Framed<UnixStream, Codec>,
        tokio_tower::Error<Framed<UnixStream, Codec>, MsgFrame>,
        MsgFrame,
    > = Client::new(framed);

    //TODO: implement connect() using this call() req/response and get the client_id

    let _req = latest_vpp_api::interface::SwInterfaceDump::builder()
        .build()
        .unwrap();

    let msg = MsgFrame {
        header: todo!(),
        message: todo!(),
    }; // impl into?
    let _response: MsgFrame = client.call(msg).await.unwrap();
}
