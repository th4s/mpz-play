use common::{tcp_mux, FramedUidMux, Role, DEFAULT_LOCAL};
use serio::{stream::IoStreamExt, SinkExt};

#[tokio::main]
async fn main() {
    // Open connection and poll it in the background.
    let (future, mut ctrl) = tcp_mux(Role::Bob, DEFAULT_LOCAL).await.unwrap();
    let join_handle = tokio::spawn(future);

    // Your code
    // ######################################################################

    let mut channel = ctrl.open_framed(b"1").await.unwrap();

    let mut received: u32 = channel.expect_next().await.unwrap();
    println!("Bob received: {received}");

    received += 1;
    channel.send(received).await.unwrap();

    // ######################################################################
    // Properly close the connection.
    ctrl.mux_mut().close();
    join_handle.await.unwrap().unwrap();
}
