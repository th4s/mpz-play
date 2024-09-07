use anyhow::Error as Anyhow;
use futures::{AsyncRead, AsyncWrite};
use matchbox_socket::WebRtcSocket;
use tokio::{
    net::TcpSocket,
    time::{sleep, Duration},
};
use tokio_util::compat::TokioAsyncReadCompatExt;

/// The default address we use for all examples.
pub const DEFAULT_LOCAL: &str = "127.0.0.1:8083";

// The default address for the matchbox signaling server.
pub const DEFAULT_MATCHBOX: &str = "ws://localhost:3536/";

/// The role of the party, either `Alice` or `Bob`.
#[derive(Debug, Clone, Copy)]
pub enum Role {
    Alice,
    Bob,
}

/// Opens a TCP connection.
///
/// Depending on the `role` either listens or connects to `address`.
/// Returns a tcp stream that implements [`AsyncRead`] and [`AsyncWrite`].
pub async fn tcp_connect(
    role: Role,
    address: impl AsRef<str>,
) -> Result<impl AsyncRead + AsyncWrite, Anyhow> {
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

    Ok(tcp_stream.compat())
}

/// Opens a WebRTC datachannel.
///
/// Make sure that you have a matchbox server running in the background,
/// c.f. https://github.com/johanhelsing/matchbox/tree/main/matchbox_server
///
/// You can call [`WebRtcSocket::take_raw`] on the returned socket to get a channel to the other
/// peer which implements [`AsyncRead`] and [`AsyncWrite`].
pub async fn web_rtc() -> Result<WebRtcSocket, Anyhow> {
    let (mut web_rtc, loop_fut) = WebRtcSocket::new_reliable(DEFAULT_MATCHBOX);
    tokio::spawn(loop_fut);

    loop {
        if web_rtc.connected_peers().count() > 0 {
            break;
        } else {
            sleep(Duration::from_millis(500)).await;
            web_rtc.update_peers();
        }
    }
    Ok(web_rtc)
}

fn open_socket() -> Result<TcpSocket, Anyhow> {
    let socket = TcpSocket::new_v4()?;
    socket.set_reuseaddr(true).unwrap();
    socket.set_reuseport(true).unwrap();

    Ok(socket)
}
