use common::{tcp_connect, Role, DEFAULT_LOCAL};
use serio::{
    codec::{Bincode, Codec},
    stream::IoStreamExt,
    SinkExt,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection.
    let tcp = tcp_connect(Role::Alice, DEFAULT_LOCAL).await?;
    let mut channel = Bincode.new_framed(tcp);

    // Send a number to Bob and wait for Bob's number.
    channel.send(42_u32).await?;
    let received: u32 = channel.expect_next().await?;

    println!("Alice received: {received}");

    Ok(())
}
