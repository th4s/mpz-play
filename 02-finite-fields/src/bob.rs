use common::{tcp_connect, Role, DEFAULT_LOCAL};
use finite_fields::setup_ot_receiver;
use mpz_common::{io::Io, Context, Flush};
use mpz_fields::{p256::P256, Field};
use mpz_ole::Receiver as OLEReceiver;
use mpz_share_conversion::{
    AdditiveToMultiplicative, MultiplicativeToAdditive, ShareConversionReceiver,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection.
    let tcp = tcp_connect(Role::Bob, DEFAULT_LOCAL).await?;
    let io = Io::from_io(tcp);
    let mut context = Context::from_io(io);

    // Setup OT.
    let ot_receiver = setup_ot_receiver().await?;

    // Setup OLE and share conversion.
    let ole_receiver = OLEReceiver::<_, P256>::new(ot_receiver);
    let mut receiver = ShareConversionReceiver::<_, P256>::new(ole_receiver);

    // Choose a number.
    let input = [P256::new(7).unwrap()];

    // Allocate space for pre-processing.
    AdditiveToMultiplicative::alloc(&mut receiver, input.len())?;
    MultiplicativeToAdditive::alloc(&mut receiver, input.len())?;

    // Perform the conversion.

    let a2m = receiver.queue_to_multiplicative(&input)?;
    receiver.flush(&mut context).await?;
    let [a] = a2m.await?.shares.try_into().unwrap();

    let m2a = receiver.queue_to_additive(&input)?;
    receiver.flush(&mut context).await?;
    let [x] = m2a.await?.shares.try_into().unwrap();

    println!("factor: {:?}", a.to_be_bytes());
    println!("summand: {:?}", x.to_be_bytes());

    // // Get the channel and send/receive starting and final numbers.
    // let channel = executor.io_mut();
    // channel.send(number).await.unwrap();
    // channel.send(summand).await.unwrap();

    // let number2: P256 = channel.expect_next().await.unwrap();
    // let summand2: P256 = channel.expect_next().await.unwrap();

    // // Check that conversion worked correctly.
    // println!("Original sum: {:?}", (number + number2).to_be_bytes());
    // println!("Final sum: {:?}", (summand + summand2).to_be_bytes());

    Ok(())
}
