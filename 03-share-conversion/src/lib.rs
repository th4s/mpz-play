use anyhow::Error as Anyhow;
use common::{MuxControl, Role};
use mpz_common::executor::MTExecutor;
use mpz_common::{Allocate, Preprocess};
use mpz_ot::{
    chou_orlandi::{
        Receiver as BaseReceiver, ReceiverConfig as BaseReceiverConfig, Sender as BaseSender,
        SenderConfig as BaseSenderConfig,
    },
    kos::{Receiver, ReceiverConfig, Sender, SenderConfig},
    OTSetup,
};

/// Sets up OT.
///
/// # Arguments
///
/// * `role` - Set up the vm for either Alice or Bob.
/// * `executor` - An executor for creating contexts.
/// * `ot_count` - How many OTs to set up.
pub async fn setup_garble(
    role: Role,
    executor: &mut MTExecutor<MuxControl>,
    ot_count: usize,
) -> Result<(Sender<BaseReceiver>, Receiver<BaseSender>), Anyhow> {
    // Create base OT sender and receiver.
    let base_sender_config = BaseSenderConfig::builder().build()?;
    let base_sender = BaseSender::new(base_sender_config);

    let base_receiver_config = BaseReceiverConfig::builder().build()?;
    let base_receiver = BaseReceiver::new(base_receiver_config);

    // Create OT sender and receiver and set them up.
    let sender_config = SenderConfig::builder().build()?;
    let mut sender = Sender::new(sender_config, base_receiver);

    let receiver_config = ReceiverConfig::builder().build()?;
    let mut receiver = Receiver::new(receiver_config, base_sender);

    let mut context1 = executor.new_thread().await?;
    let mut context2 = executor.new_thread().await?;

    if let Role::Bob = role {
        std::mem::swap(&mut context1, &mut context2);
    }

    sender.alloc(ot_count);
    receiver.alloc(ot_count);

    tokio::try_join!(
        async {
            sender.setup(&mut context1).await?;
            sender.preprocess(&mut context1).await
        },
        async {
            receiver.setup(&mut context2).await?;
            receiver.preprocess(&mut context2).await
        }
    )
    .unwrap();

    Ok((sender, receiver))
}
