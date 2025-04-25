use common::{tcp_connect, Role, DEFAULT_LOCAL};
use finite_fields::setup_ot_receiver;
use mpz_common::{io::Io, Context, Flush};
use mpz_fields::{p256::P256, Field};
use mpz_ole::Receiver as OLEReceiver;
use mpz_share_conversion::{
    AdditiveToMultiplicative, MultiplicativeToAdditive, ShareConversionReceiver,
};
use serio::{stream::IoStreamExt, SinkExt};

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
    let [factor] = a2m.await?.shares.try_into().unwrap();

    let m2a = receiver.queue_to_additive(&[factor])?;
    receiver.flush(&mut context).await?;
    let [summand] = m2a.await?.shares.try_into().unwrap();

    // Get the channel and send/receive starting and final numbers.
    let channel = context.io_mut();
    channel.send(input[0]).await?;
    channel.send(factor).await?;
    channel.send(summand).await?;

    let input_alice: P256 = channel.expect_next().await?;
    let factor2: P256 = channel.expect_next().await?;
    let summand2: P256 = channel.expect_next().await?;

    // Check that conversion worked correctly.
    println!("Original sum: {:?}", (input[0] + input_alice).to_be_bytes());
    println!("Multiplication: {:?}", (factor * factor2).to_be_bytes());
    println!("Addition: {:?}", (summand + summand2).to_be_bytes());

    Ok(())
}
