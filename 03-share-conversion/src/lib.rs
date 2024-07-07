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
