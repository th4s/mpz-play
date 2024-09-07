use anyhow::{anyhow, Result as Anyhow};
use matchbox_socket::{MessageLoopFuture, WebRtcSocket};
use mpz_circuits::{types::ValueType, Circuit, CircuitBuilder};
use mpz_common::{Allocate, Context, Preprocess};
use mpz_garble::protocol::deap::DEAPThread;
use mpz_garble::{config::Role as DEAPRole, DecodePrivate, Execute, Memory};
use mpz_ot::{
    chou_orlandi::{
        Receiver as BaseReceiver, ReceiverConfig as BaseReceiverConfig, Sender as BaseSender,
        SenderConfig as BaseSenderConfig,
    },
    kos::{Receiver, ReceiverConfig, Sender, SenderConfig},
    OTSetup,
};
use std::sync::Arc;

// The default address for the matchbox signaling server.
const DEFAULT_MATCHBOX: &str = "ws://localhost:3536/";

// The bristol circuit file for the u32-comparator.
const COMPARATOR_FILENAME: &str = "./32-bit_less-than-comparator.txt";

/// The role of the party, either `Alice` or `Bob`.
#[derive(Debug, Clone, Copy)]
pub enum Role {
    Alice,
    Bob,
}

/// Sets up a VM for garbled circuits.
///
/// # Arguments
///
/// * `role` - Set up the vm for either Alice or Bob.
/// * `context` - A context for IO.
/// * `ot_count` - How many OTs to set up.
pub async fn setup_garble(
    role: Role,
    mut context: impl Context,
    ot_count: usize,
) -> Anyhow<impl Memory + Execute + DecodePrivate> {
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

    let deap_role = match role {
        Role::Alice => DEAPRole::Leader,
        Role::Bob => DEAPRole::Follower,
    };

    sender.alloc(ot_count);
    receiver.alloc(ot_count);

    if let Role::Alice = role {
        sender.setup(&mut context).await?;
        sender.preprocess(&mut context).await?;
    } else {
        receiver.setup(&mut context).await?;
        receiver.preprocess(&mut context).await?;
    }

    if let Role::Bob = role {
        sender.setup(&mut context).await?;
        sender.preprocess(&mut context).await?;
    } else {
        receiver.setup(&mut context).await?;
        receiver.preprocess(&mut context).await?;
    }

    Ok(DEAPThread::new(
        deap_role, [0; 32], context, sender, receiver,
    ))
}

pub fn millionaire_circuit() -> Anyhow<Circuit> {
    let lt_comparator = parse_lt_comparator(COMPARATOR_FILENAME)?;
    let lt_comparator = Arc::new(lt_comparator);

    let builder = CircuitBuilder::new();
    let alice_input = builder.add_input::<u32>();
    let bob_input = builder.add_input::<u32>();

    let mut output_alice = builder.append(
        &lt_comparator.clone(),
        &[bob_input.into(), alice_input.into()],
    )?;
    let mut output_bob = builder.append(
        &lt_comparator.clone(),
        &[alice_input.into(), bob_input.into()],
    )?;

    let output_alice = output_alice
        .pop()
        .ok_or(anyhow!("Unable to pop circuit output"))?;
    let output_bob = output_bob
        .pop()
        .ok_or(anyhow!("Unable to pop circuit output"))?;

    builder.add_output(output_alice);
    builder.add_output(output_bob);
    let circuit = builder
        .build()
        .map_err(|err| anyhow!("Cannot build circuit: {}", err))?;

    Ok(circuit)
}

fn parse_lt_comparator(filename: impl AsRef<str>) -> Anyhow<Circuit> {
    let circuit = Circuit::parse(
        filename.as_ref(),
        &[ValueType::U32, ValueType::U32],
        &[ValueType::Bit],
    )?;

    let circuit = circuit.reverse_input(0).reverse_input(1).reverse_output(0);

    Ok(circuit)
}

/// Opens a WebRTC datachannel.
///
/// Make sure that you have a matchbox server running in the background,
/// c.f. https://github.com/johanhelsing/matchbox/tree/main/matchbox_server
///
/// You can call [`WebRtcSocket::take_raw`] on the returned socket to get a channel to the other
/// peer which implements [`futures::AsyncRead`] and [`futures::AsyncWrite`].
///
/// Make sure to continuously poll the future.
pub fn web_rtc() -> Anyhow<(WebRtcSocket, MessageLoopFuture)> {
    Ok(WebRtcSocket::new_reliable(DEFAULT_MATCHBOX))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mpz_circuits::evaluate;

    #[test]
    fn test_comparator() {
        let circuit = parse_lt_comparator(COMPARATOR_FILENAME).unwrap();

        let is_bob_richer = |alice: u32, bob: u32| {
            let wealth_alice: u32 = alice;
            let wealth_bob: u32 = bob;

            evaluate!(circuit, fn(wealth_alice, wealth_bob) -> bool).unwrap()
        };

        #[allow(clippy::bool_assert_comparison)]
        {
            assert_eq!(true, is_bob_richer(0, 1));
            assert_eq!(true, is_bob_richer(0, u32::MAX));
            assert_eq!(true, is_bob_richer(1023, 1024));
            assert_eq!(true, is_bob_richer(1024, 1025));

            assert_eq!(false, is_bob_richer(2, 0));
            assert_eq!(false, is_bob_richer(100, 100));
            assert_eq!(false, is_bob_richer(2_000_000, 1_000_000));
            assert_eq!(false, is_bob_richer(u32::MAX, u32::MAX - 1));
        }
    }

    #[test]
    fn test_millionaire_circuit() {
        let circuit = millionaire_circuit().unwrap();

        let who_is_richer = |alice: u32, bob: u32| {
            let wealth_alice: u32 = alice;
            let wealth_bob: u32 = bob;

            evaluate!(circuit, fn(wealth_alice, wealth_bob) -> (bool, bool)).unwrap()
        };

        #[allow(clippy::bool_assert_comparison)]
        {
            assert_eq!((false, false), who_is_richer(4, 4));
            assert_eq!((false, true), who_is_richer(2, 4));
            assert_eq!((true, false), who_is_richer(8, 3));
        }
    }
}
