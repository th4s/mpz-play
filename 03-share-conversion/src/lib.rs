//! Now we want to explore what mpz has to offer for finite fields. We want to use
//! [`mpz_share_conversion`] in order to convert a shared sum into a shared product and again back
//! into a shared sum.
//!
//! To achieve that, depending on your role instantiate an [`mpz_ole::rot::OLESender`] or an
//! [`mpz_ole::rot::OLEReceiver`]. They can be used for creating an
//! [`mpz_share_conversion::ShareConversionSender`]/[mpz_share_conversion::ShareConversionReceiver].
//! Then you can use [`mpz_share_conversion::AdditiveToMultiplicative`] to convert the shared sum
//! into a shared product and [`mpz_share_conversion::MultiplicativeToAdditive`] to convert it back
//! again. In the end check that both sums are equal by sending over the missing summands to each
//! other.

use anyhow::Error as Anyhow;
use common::MuxControl;
use mpz_common::executor::MTExecutor;
use mpz_ot::{
    chou_orlandi::{
        Receiver as BaseReceiver, ReceiverConfig as BaseReceiverConfig, Sender as BaseSender,
        SenderConfig as BaseSenderConfig,
    },
    kos::{Receiver, ReceiverConfig, Sender, SenderConfig},
    OTSetup,
};

/// Sets up an OT sender.
///
/// # Arguments
///
/// * `executor` - An executor for creating contexts.
pub async fn setup_ot_sender(
    executor: &mut MTExecutor<MuxControl>,
) -> Result<Sender<BaseReceiver>, Anyhow> {
    // Create a base OT receiver.
    let base_receiver_config = BaseReceiverConfig::builder().build()?;
    let base_receiver = BaseReceiver::new(base_receiver_config);

    // Create an OT sender and set it up.
    let sender_config = SenderConfig::builder().build()?;
    let mut sender = Sender::new(sender_config, base_receiver);

    let mut context = executor.new_thread().await?;

    sender.setup(&mut context).await?;

    Ok(sender)
}

/// Sets up an OT receiver.
///
/// # Arguments
///
/// * `executor` - An executor for creating contexts.
pub async fn setup_ot_receiver(
    executor: &mut MTExecutor<MuxControl>,
) -> Result<Receiver<BaseSender>, Anyhow> {
    // Create a base OT sender.
    let base_sender_config = BaseSenderConfig::builder().build()?;
    let base_sender = BaseSender::new(base_sender_config);

    // Create an OT receiver and set it up.
    let receiver_config = ReceiverConfig::builder().build()?;
    let mut receiver = Receiver::new(receiver_config, base_sender);

    let mut context = executor.new_thread().await?;

    receiver.setup(&mut context).await?;

    Ok(receiver)
}
