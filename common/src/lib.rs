use anyhow::Error as Anyhow;
use mux::attach_mux;
use tokio::net::TcpSocket;
use tokio::time::{sleep, Duration};
use tokio_util::compat::TokioAsyncReadCompatExt;

mod mux;

pub use mux::{MuxControl, MuxFuture};
pub use uid_mux::FramedUidMux;

/// The default address we use for all examples.
pub const DEFAULT_LOCAL: &str = "127.0.0.1:8083";

#[derive(Debug, Clone, Copy)]
pub enum Role {
    Alice,
    Bob,
}

/// Opens a multiplexed TCP connection.
pub async fn tcp_mux(
    role: Role,
    address: impl AsRef<str>,
) -> Result<(MuxFuture, MuxControl), Anyhow> {
    let addr = address.as_ref().parse()?;

    let tcp_stream = match role {
        Role::Alice => loop {
            let socket = open_socket()?;
            let res_tcp = socket.connect(addr).await;
            match res_tcp {
                Ok(tcp) => break tcp,
                Err(_) => sleep(Duration::from_millis(100)).await,
            }
        },
        Role::Bob => {
            let socket = open_socket()?;
            socket.bind(addr)?;
            let listener = socket.listen(1024)?;
            let (tcp, _) = listener.accept().await?;
            tcp
        }
    };

    Ok(attach_mux(tcp_stream.compat(), role))
}

/// Opens a multiplexed WebRTC datachannel.
pub async fn webrtc_mux(_role: Role) -> Result<(MuxFuture, MuxControl), Anyhow> {
    todo!()
}

fn open_socket() -> Result<TcpSocket, Anyhow> {
    let socket = TcpSocket::new_v4()?;
    socket.set_reuseaddr(true).unwrap();
    socket.set_reuseport(true).unwrap();

    Ok(socket)
}
