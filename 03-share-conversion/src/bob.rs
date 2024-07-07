use common::{tcp_mux, FramedUidMux, Role, DEFAULT_LOCAL};
use mpz_common::{executor::MTExecutor, Allocate, Preprocess};
use mpz_fields::p256::P256;
use mpz_ole::rot::OLEReceiver;
use mpz_share_conversion::{
    AdditiveToMultiplicative, MultiplicativeToAdditive, ShareConversionReceiver,
};
use serio::{stream::IoStreamExt, SinkExt};
use share_conversion::setup_ot_receiver;

#[tokio::main]
async fn main() {
    // Open connection and poll it in the background.
    let (future, mut ctrl) = tcp_mux(Role::Bob, DEFAULT_LOCAL).await.unwrap();
    let join_handle = tokio::spawn(future);

    // Create an executor and setup OT.
    let mut executor = MTExecutor::new(ctrl.clone(), 32);
    let ot_receiver = setup_ot_receiver(&mut executor).await.unwrap();

    // Setup OLE and share conversion.
    let mut context = executor.new_thread().await.unwrap();
    let mut ole_receiver = OLEReceiver::<_, P256>::new(ot_receiver);
    ole_receiver.alloc(2);
    ole_receiver.preprocess(&mut context).await.unwrap();

    let mut receiver = ShareConversionReceiver::<_, P256>::new(ole_receiver);

    // Choose a number.
    let number = P256::new(3).unwrap();

    // Perform the conversion.
    let factor = receiver
        .to_multiplicative(&mut context, vec![number])
        .await
        .unwrap();

    let summand = receiver
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
    assert_eq!(number + number2, summand + summand2);

    // Properly close the connection.
    ctrl.mux_mut().close();
    join_handle.await.unwrap().unwrap();
}
