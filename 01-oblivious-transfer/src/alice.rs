use common::{tcp_mux, Role, DEFAULT_LOCAL};
use mpz_common::executor::MTExecutor;
use mpz_core::Block;
use mpz_ot::{
    chou_orlandi::{Sender, SenderConfig},
    OTSender, OTSetup,
};

#[tokio::main]
async fn main() {
    // Open connection and poll it in the background.
    let (future, mut ctrl) = tcp_mux(Role::Alice, DEFAULT_LOCAL).await.unwrap();
    let join_handle = tokio::spawn(future);

    // Create an executor and spawn a context.
    let mut executor = MTExecutor::new(ctrl.clone(), 32);
    let mut context = executor.new_thread().await.unwrap();

    // Create an OT sender and set it up.
    let sender_config = SenderConfig::builder().build().unwrap();
    let mut sender = Sender::new(sender_config);
    sender.setup(&mut context).await.unwrap();

    // Create a message.
    let zero = Block::ZERO;
    let one = Block::ONE;

    // Send OTs to Bob.
    sender.send(&mut context, &[[zero, one]]).await.unwrap();

    // Properly close the connection.
    ctrl.mux_mut().close();
    join_handle.await.unwrap().unwrap();
}
