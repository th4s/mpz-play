use common::{tcp_connect, Role, DEFAULT_LOCAL};
use mpz_common::executor::STExecutor;
use mpz_ot::{
    chou_orlandi::{Receiver, ReceiverConfig},
    OTReceiver, OTSetup,
};
use serio::codec::{Bincode, Codec};

#[tokio::main]
async fn main() {
    // Open a connection.
    let tcp = tcp_connect(Role::Bob, DEFAULT_LOCAL).await.unwrap();
    let channel = Bincode.new_framed(tcp);

    // Create an executor.
    let mut executor = STExecutor::new(channel);

    // Create an OT receiver and set it up.
    let receiver_config = ReceiverConfig::default();
    let mut receiver = Receiver::new(receiver_config);

    receiver.setup(&mut executor).await.unwrap();

    // Make a choice.
    let choice = true;

    // Receive OTs from Alice.
    let output = receiver.receive(&mut executor, &[choice]).await.unwrap();
    println!("Received from Alice: {:?}", output.msgs.first().unwrap());
}
