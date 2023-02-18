use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Channel, Uri};
use tower::service_fn;

pub async fn create_channel_to_unix_socket() -> Result<Channel, Box<dyn std::error::Error>> {
    // magic incantation for endpoint creation acquired from
    // https://github.com/hyperium/tonic/blob/master/examples/src/uds/client.rs
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(|_: Uri| {
            let path = "/run/containerd/containerd.sock";

            // Connect to a Unix Domain Socket
            UnixStream::connect(path)
        }))
        .await?;
    Ok(channel)
}
