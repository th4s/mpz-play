use common::{tcp_connect, Role, DEFAULT_LOCAL};
use mpz_common::{io::Io, Context, Flush};
use mpz_ot::{chou_orlandi::Receiver, ot::OTReceiver};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection.
    let tcp = tcp_connect(Role::Bob, DEFAULT_LOCAL).await.unwrap();
    let io = Io::from_io(tcp);

    // Create an executor.
    let mut context = Context::from_io(io);

    // Create an OT receiver.
    let mut receiver = Receiver::new();

    // Make a choice.
    let choices = [true];

    // Receive OTs from Alice.
    receiver.alloc(choices.len())?;
    let output = receiver.queue_recv_ot(&choices)?;
    receiver.flush(&mut context).await?;

    let output = output.await?;
    println!("Bob received: {:?}", output.msgs);

    Ok(())
}
