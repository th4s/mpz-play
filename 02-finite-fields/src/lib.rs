//! Now we want to explore what mpz has to offer for finite fields. We want to use
//! [`mpz_share_conversion`] in order to convert a shared sum into a shared product and again back
//! into a shared sum.
//!
//! To achieve that, depending on your role instantiate an [`mpz_ole::rot::OLESender`] or an
//! [`mpz_ole::rot::OLEReceiver`]. They can be used for creating an
//! [`mpz_share_conversion::ShareConversionSender`]/[`mpz_share_conversion::ShareConversionReceiver`].
//! Then you can use [`mpz_share_conversion::AdditiveToMultiplicative`] to convert the shared sum
//! into a shared product and [`mpz_share_conversion::MultiplicativeToAdditive`] to convert it back
//! again. In the end check that both sums are equal by sending over the missing summands to each
//! other.

use mpz_core::Block;
use mpz_ot::{
    chou_orlandi::{Receiver as BaseReceiver, Sender as BaseSender},
    kos::{Receiver as KOSReceiver, ReceiverConfig, Sender as KOSSender, SenderConfig},
    rot::{
        any::{AnyReceiver, AnySender},
        randomize::{RandomizeRCOTReceiver, RandomizeRCOTSender},
    },
};
use rand::{rngs::StdRng, SeedableRng};

/// Sets up an OT sender.
pub async fn setup_ot_sender(
) -> Result<AnySender<RandomizeRCOTSender<KOSSender<BaseReceiver>>>, Box<dyn std::error::Error>> {
    let mut rng = StdRng::seed_from_u64(0);

    let kos_sender = KOSSender::new(
        SenderConfig::default(),
        Block::random(&mut rng),
        BaseReceiver::new(),
    );
    let sender = AnySender::new(RandomizeRCOTSender::new(kos_sender));

    Ok(sender)
}

/// Sets up an OT receiver.
pub async fn setup_ot_receiver(
) -> Result<AnyReceiver<RandomizeRCOTReceiver<KOSReceiver<BaseSender>>>, Box<dyn std::error::Error>>
{
    let kos_receiver = KOSReceiver::new(ReceiverConfig::default(), BaseSender::new());
    let receiver = AnyReceiver::new(RandomizeRCOTReceiver::new(kos_receiver));

    Ok(receiver)
}
