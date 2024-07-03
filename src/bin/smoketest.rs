use mpz_play::{mux::Role, tcp_mux};
use serio::{stream::IoStreamExt, SinkExt};
use uid_mux::FramedUidMux;

#[tokio::main]
async fn main() {
    let (future, ctrl) = tcp_mux(Role::Alice).await.unwrap();
    let mut channel = ctrl.open_framed(b"1").await.unwrap();
    tokio::spawn(future);

    channel.send(42_u32).await.unwrap();
    let received: u32 = channel.expect_next().await.unwrap();
    println!("Received: {received}");
}
