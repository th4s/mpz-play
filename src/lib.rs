use anyhow::Error as Anyhow;
use mux::{attach_mux, MuxControl, MuxFuture, Role};
use tokio::net::TcpSocket;
use tokio_util::compat::TokioAsyncReadCompatExt;

pub mod mux;

pub async fn mux_with_tcp(role: Role) -> Result<(MuxFuture, MuxControl), Anyhow> {
    let socket = TcpSocket::new_v6()?;
    let addr = "[::1]:8080".parse()?;

    let tcp_stream = match role {
        Role::Alice => {
            socket.bind(addr)?;
            let listener = socket.listen(1024)?;
            let (tcp_stream, _) = listener.accept().await?;
            tcp_stream
        }
        Role::Bob => socket.connect(addr).await?,
    };

    Ok(attach_mux(tcp_stream.compat(), role))
}
