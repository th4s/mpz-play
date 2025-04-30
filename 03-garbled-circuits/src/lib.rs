//! In this unit we want to play with garbled circuits. Let's encrypt a block with AES in ECB mode,
//! where Alice provides the key and Bob provides the message. They work together to compute the
//! ciphertext, which is returned only to Alice in the end. You can use a test vector from
//! https://github.com/ircmaxell/quality-checker/blob/master/tmp/gh_18/PHP-PasswordLib-master/test/Data/Vectors/aes-ecb.test-vectors
//! to check if everything worked correctly.
//!
//! After setting up our connection and creating an executor we can use [`setup_garble`] to
//! instantiate a VM for garbled circuits. Using [`Memory`] you can define inputs and outputs. You
//! will need [`Memory::new_private_input`], [`Memory::new_blind_input`], [`Memory::new_output`]. Do
//! not forget to also assign a value for the private inputs with [`Memory::assign`].
//!
//! After that the [`mpz_circuits::circuits::AES128`] circuit can be used and you should be able
//! to call [`Execute::execute`] and make use of [`DecodePrivate`] so that only Alice gets to see
//! the output.

use anyhow::Error as Anyhow;
use mpz_core::Block;
use mpz_garble::protocol::semihonest::{Evaluator, Garbler};
use mpz_memory_core::correlated::Delta;
use mpz_ot::{
    chou_orlandi::{Receiver as BaseReceiver, Sender as BaseSender},
    cot::{DerandCOTReceiver, DerandCOTSender},
    kos::{Receiver, ReceiverConfig, Sender, SenderConfig},
};
use rand::{rngs::StdRng, SeedableRng};

pub async fn setup_garbler() -> Result<Garbler<DerandCOTSender<Sender<BaseReceiver>>>, Anyhow> {
    let base_receiver = BaseReceiver::new();

    let mut rng = StdRng::seed_from_u64(0);
    let delta = Block::random(&mut rng);

    let sender = Sender::new(SenderConfig::default(), delta, base_receiver);
    let sender = DerandCOTSender::new(sender);

    let garbler = Garbler::new(sender, [0u8; 16], Delta::new(delta));

    Ok(garbler)
}

pub async fn setup_evaluator() -> Result<Evaluator<DerandCOTReceiver<Receiver<BaseSender>>>, Anyhow>
{
    let base_sender = BaseSender::new();

    let receiver = Receiver::new(ReceiverConfig::default(), base_sender);
    let receiver = DerandCOTReceiver::new(receiver);

    let evaluator = Evaluator::new(receiver);

    Ok(evaluator)
}
