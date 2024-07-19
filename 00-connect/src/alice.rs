use common::{tcp_connect, Role, DEFAULT_LOCAL};
use serio::{
    codec::{Bincode, Codec},
    stream::IoStreamExt,
    SinkExt,
};

#[tokio::main]
async fn main() {
    // Open a connection.
    let tcp = tcp_connect(Role::Alice, DEFAULT_LOCAL).await.unwrap();
    let mut channel = Bincode::default().new_framed(tcp);

    // Send a number to Bob and wait for Bob's number.
    channel.send(42_u32).await.unwrap();
    let received: u32 = channel.expect_next().await.unwrap();

    println!("Alice received: {received}");
}
