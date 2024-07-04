//! In this unit we want to play with garbled circuits. Let's encrypt a block with AES in ECB mode,
//! where Alice provides the key and Bob provides the message. They work together to compute the
//! ciphertext, which is returned only to Alice in the end. You can use a test vector from
//! https://github.com/ircmaxell/quality-checker/blob/master/tmp/gh_18/PHP-PasswordLib-master/test/Data/Vectors/aes-ecb.test-vectors
//! to check if everything worked correctly.
//!
//! After setting up our connection and creating an executor we can use [`setup_garble`] to
//! instantiate a VM for garbled circuits. Using [`Memory`] you can define inputs and outputs. You
//! will need [`Memory::new_private_input`], [Memory::new_blind_input], [Memory::new_output]. Do
//! not forget to also assign a value for the private inputs with [Memory::assign].
//!
//! After that the [`mpz_circuits::circuits::AES128`] circuit can be used and you should be able
//! to call [`Execute::execute`] and make use of [`DecodePrivate`] so that only Alice gets to see
//! the output.

use anyhow::Error as Anyhow;
use common::{MuxControl, Role};
use mpz_common::executor::MTExecutor;
use mpz_common::{Allocate, Preprocess};
use mpz_garble::protocol::deap::DEAPThread;
use mpz_garble::{config::Role as DEAPRole, DecodePrivate, Execute, Memory};
use mpz_ot::chou_orlandi::{
    Receiver as BaseReceiver, ReceiverConfig as BaseReceiverConfig, Sender as BaseSender,
    SenderConfig as BaseSenderConfig,
};
use mpz_ot::kos::{Receiver, ReceiverConfig, Sender, SenderConfig};
use mpz_ot::OTSetup;

/// Sets up a VM for garbled circuits.
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
) -> Result<impl Memory + Execute + DecodePrivate, Anyhow> {
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

    let deap_role = match role {
        Role::Alice => DEAPRole::Leader,
        Role::Bob => DEAPRole::Follower,
    };

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

    // Instantiate a vm for garbled circuits.
    let context3 = executor.new_thread().await?;
    let garble_vm = DEAPThread::new(deap_role, [0; 32], context3, sender, receiver);

    Ok(garble_vm)
}
