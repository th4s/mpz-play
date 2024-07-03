use mpz_play::{mux::Role, mux_with_tcp};
use serio::SinkExt;
use uid_mux::FramedUidMux;

#[tokio::main]
async fn main() {
    let (future, ctrl) = mux_with_tcp(Role::Alice).await.unwrap();
    let mut channel = ctrl.open_framed(b"1").await.unwrap();
    tokio::spawn(future);

    channel.send(42_u32).await.unwrap();
}
