use common::{tcp_connect, Role, DEFAULT_LOCAL};
use finite_fields::setup_ot_sender;
use mpz_common::{io::Io, Context, Flush};
use mpz_core::Block;
use mpz_fields::{p256::P256, Field};
use mpz_ole::Sender as OLESender;
use mpz_share_conversion::{
    AdditiveToMultiplicative, MultiplicativeToAdditive, ShareConversionSender,
};
use rand::{rngs::StdRng, SeedableRng};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection.
    let tcp = tcp_connect(Role::Alice, DEFAULT_LOCAL).await?;
    let io = Io::from_io(tcp);
    let mut context = Context::from_io(io);

    // Setup OT.
    let ot_sender = setup_ot_sender().await?;

    // Setup OLE and share conversion.
    let mut rng = StdRng::seed_from_u64(0);
    let ole_sender = OLESender::<_, P256>::new(Block::random(&mut rng), ot_sender);
    let mut sender = ShareConversionSender::<_, P256>::new(ole_sender);

    // Choose a number.
    let input = [P256::new(5).unwrap()];

    // Allocate space for pre-processing.
    AdditiveToMultiplicative::alloc(&mut sender, input.len())?;
    MultiplicativeToAdditive::alloc(&mut sender, input.len())?;

    // Perform the conversion.
    let a2m = sender.queue_to_multiplicative(&input)?;
    sender.flush(&mut context).await?;
    let [factor] = a2m.await?.shares.try_into().unwrap();

    let m2a = sender.queue_to_additive(&[factor])?;
    sender.flush(&mut context).await?;
    let [summand] = m2a.await?.shares.try_into().unwrap();

    println!("let a = P256::try_from({:?})?;", factor.to_le_bytes());
    println!("let a = P256::try_from({:?})?;", summand.to_le_bytes());

    // Get the channel and send/receive starting and final numbers.
    // let channel = context.io_mut();
    // channel.start_send("ok").await.unwrap();
    // channel.send("ok").await.unwrap();
    // channel.send("ok").await.unwrap();

    // let number2: P256 = channel.expect_next().await.unwrap();
    // let summand2: P256 = channel.expect_next().await.unwrap();

    // // Check that conversion worked correctly.
    // println!("Original sum: {:?}", (number + number2).to_be_bytes());
    // println!("Final sum: {:?}", (summand + summand2).to_be_bytes());

    Ok(())
}
