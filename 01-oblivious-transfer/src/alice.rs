use common::{tcp_connect, Role, DEFAULT_LOCAL};
use mpz_common::{io::Io, Context, Flush};
use mpz_core::Block;
use mpz_ot::{chou_orlandi::Sender, ot::OTSender};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection.
    let tcp = tcp_connect(Role::Alice, DEFAULT_LOCAL).await?;
    let io = Io::from_io(tcp);

    // Create a context.
    let mut context = Context::from_io(io);

    // Create an OT sender.
    let mut sender = Sender::new();

    // Create messages.
    let messages = [Block::ZERO, Block::ONES];

    // Send OTs to Bob.
    sender.alloc(messages.len())?;
    let output = sender.queue_send_ot(&[messages])?;
    sender.flush(&mut context).await?;

    let output = output.await?;
    println!("Alice sent: {:?}", output);

    Ok(())
}
