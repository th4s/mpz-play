use common::{tcp_connect, Role, DEFAULT_LOCAL};
use mpz_common::executor::STExecutor;
use mpz_core::Block;
use mpz_ot::{
    chou_orlandi::{Sender, SenderConfig},
    OTSender, OTSetup,
};
use serio::codec::{Bincode, Codec};

#[tokio::main]
async fn main() {
    // Open a connection.
    let tcp = tcp_connect(Role::Alice, DEFAULT_LOCAL).await.unwrap();
    let channel = Bincode.new_framed(tcp);

    // Create an executor.
    let mut executor = STExecutor::new(channel);

    // Create an OT sender and set it up.
    let sender_config = SenderConfig::default();
    let mut sender = Sender::new(sender_config);

    sender.setup(&mut executor).await.unwrap();

    // Create messages.
    let zero = Block::ZERO;
    let one = Block::ONE;

    // Send OTs to Bob.
    sender.send(&mut executor, &[[zero, one]]).await.unwrap();
}
