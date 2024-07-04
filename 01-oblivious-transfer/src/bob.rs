use common::{tcp_mux, Role, DEFAULT_LOCAL};
use mpz_common::executor::MTExecutor;
use mpz_ot::{
    chou_orlandi::{Receiver, ReceiverConfig},
    OTReceiver, OTSetup,
};

#[tokio::main]
async fn main() {
    // Open connection and poll it in the background.
    let (future, mut ctrl) = tcp_mux(Role::Bob, DEFAULT_LOCAL).await.unwrap();
    let join_handle = tokio::spawn(future);

    // Create an executor and spawn a context.
    let mut executor = MTExecutor::new(ctrl.clone(), 32);
    let mut context = executor.new_thread().await.unwrap();

    // Create an OT receiver and set it up.
    let receiver_config = ReceiverConfig::builder().build().unwrap();
    let mut receiver = Receiver::new(receiver_config);
    receiver.setup(&mut context).await.unwrap();

    // Create a choice.
    let choice = true;

    // Receive OTs from Alice.
    let output = receiver.receive(&mut context, &[choice]).await.unwrap();
    println!("Received from Alice: {:?}", output.msgs.first().unwrap());

    // Properly close the connection.
    ctrl.mux_mut().close();
    join_handle.await.unwrap().unwrap();
}
