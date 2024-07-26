use common::{tcp_connect, Role, DEFAULT_LOCAL};
use finite_fields::setup_ot_sender;
use mpz_common::{executor::STExecutor, Allocate, Context, Preprocess};
use mpz_fields::{p256::P256, Field};
use mpz_ole::rot::OLESender;
use mpz_share_conversion::{
    AdditiveToMultiplicative, MultiplicativeToAdditive, ShareConversionSender,
};
use serio::{
    codec::{Bincode, Codec},
    stream::IoStreamExt,
    SinkExt,
};

#[tokio::main]
async fn main() {
    // Open a connection.
    let tcp = tcp_connect(Role::Alice, DEFAULT_LOCAL).await.unwrap();
    let channel = Bincode.new_framed(tcp);

    // Create an executor and setup OT.
    let mut executor = STExecutor::new(channel);
    let ot_sender = setup_ot_sender(&mut executor).await.unwrap();

    // Setup OLE and share conversion.
    let mut ole_sender = OLESender::<_, P256>::new(ot_sender);
    ole_sender.alloc(2);
    ole_sender.preprocess(&mut executor).await.unwrap();

    let mut sender = ShareConversionSender::<_, P256>::new(ole_sender);

    // Choose a number.
    let number = P256::new(42).unwrap();

    // Perform the conversion.
    let factor = sender
        .to_multiplicative(&mut executor, vec![number])
        .await
        .unwrap();

    let summand = sender
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
