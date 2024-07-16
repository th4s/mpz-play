use common::{tcp_mux, FramedUidMux, Role, DEFAULT_LOCAL};
use finite_fields::setup_ot_sender;
use mpz_common::{executor::MTExecutor, Allocate, Preprocess};
use mpz_fields::{p256::P256, Field};
use mpz_ole::rot::OLESender;
use mpz_share_conversion::{
    AdditiveToMultiplicative, MultiplicativeToAdditive, ShareConversionSender,
};
use serio::{stream::IoStreamExt, SinkExt};

#[tokio::main]
async fn main() {
    // Open connection and poll it in the background.
    let (future, mut ctrl) = tcp_mux(Role::Alice, DEFAULT_LOCAL).await.unwrap();
    let join_handle = tokio::spawn(future);

    // Create an executor and setup OT.
    let mut executor = MTExecutor::new(ctrl.clone(), 32);
    let ot_sender = setup_ot_sender(&mut executor).await.unwrap();

    // Setup OLE and share conversion.
    let mut context = executor.new_thread().await.unwrap();
    let mut ole_sender = OLESender::<_, P256>::new(ot_sender);
    ole_sender.alloc(2);
    ole_sender.preprocess(&mut context).await.unwrap();

    let mut sender = ShareConversionSender::<_, P256>::new(ole_sender);

    // Choose a number.
    let number = P256::new(42).unwrap();

    // Perform the conversion.
    let factor = sender
        .to_multiplicative(&mut context, vec![number])
        .await
        .unwrap();

    let summand = sender
        .to_additive(&mut context, factor)
        .await
        .unwrap()
        .pop()
        .unwrap();

    // Open a channel and send/receive starting and final numbers.
    let mut channel = ctrl.open_framed(b"1").await.unwrap();
    channel.send(number).await.unwrap();
    channel.send(summand).await.unwrap();

    let number2: P256 = channel.expect_next().await.unwrap();
    let summand2: P256 = channel.expect_next().await.unwrap();

    // Check that conversion worked correctly.
    println!("Original sum: {:?}", (number + number2).to_be_bytes());
    println!("Final sum: {:?}", (summand + summand2).to_be_bytes());

    // Properly close the connection.
    ctrl.mux_mut().close();
    join_handle.await.unwrap().unwrap();
}
