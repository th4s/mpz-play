use common::{tcp_connect, Role, DEFAULT_LOCAL};
use finite_fields::setup_ot_receiver;
use mpz_common::{executor::STExecutor, Allocate, Context, Preprocess};
use mpz_fields::{p256::P256, Field};
use mpz_ole::rot::OLEReceiver;
use mpz_share_conversion::{
    AdditiveToMultiplicative, MultiplicativeToAdditive, ShareConversionReceiver,
};
use serio::{
    codec::{Bincode, Codec},
    stream::IoStreamExt,
    SinkExt,
};

#[tokio::main]
async fn main() {
    // Open a connection.
    let tcp = tcp_connect(Role::Bob, DEFAULT_LOCAL).await.unwrap();
    let channel = Bincode::default().new_framed(tcp);

    // Create an executor and setup OT.
    let mut executor = STExecutor::new(channel);
    let ot_receiver = setup_ot_receiver(&mut executor).await.unwrap();

    // Setup OLE and share conversion.
    let mut ole_receiver = OLEReceiver::<_, P256>::new(ot_receiver);
    ole_receiver.alloc(2);
    ole_receiver.preprocess(&mut executor).await.unwrap();

    let mut receiver = ShareConversionReceiver::<_, P256>::new(ole_receiver);

    // Choose a number.
    let number = P256::new(3).unwrap();

    // Perform the conversion.
    let factor = receiver
        .to_multiplicative(&mut executor, vec![number])
        .await
        .unwrap();

    let summand = receiver
        .to_additive(&mut executor, factor)
        .await
        .unwrap()
        .pop()
        .unwrap();

    // Get the channel and send/receive starting and final numbers.
    let channel = executor.io_mut();
    channel.send(number).await.unwrap();
    channel.send(summand).await.unwrap();

    let number2: P256 = channel.expect_next().await.unwrap();
    let summand2: P256 = channel.expect_next().await.unwrap();

    // Check that conversion worked correctly.
    println!("Original sum: {:?}", (number + number2).to_be_bytes());
    println!("Final sum: {:?}", (summand + summand2).to_be_bytes());
}
