use common::{tcp_connect, Role, DEFAULT_LOCAL};
use serio::{
    codec::{Bincode, Codec},
    stream::IoStreamExt,
    SinkExt,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection.
    let tcp = tcp_connect(Role::Bob, DEFAULT_LOCAL).await?;
    let mut channel = Bincode.new_framed(tcp);

    // Wait for Alice to send her number, then increment and send it back.
    let mut received: u32 = channel.expect_next().await?;
    println!("Bob received: {received}");

    received += 1;
    channel.send(received).await?;

    Ok(())
}
