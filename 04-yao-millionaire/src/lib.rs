use std::sync::Arc;

use anyhow::{anyhow, Result as Anyhow};
use mpz_circuits::{types::ValueType, Circuit, CircuitBuilder};

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

const COMPARATOR_FILENAME: &str = "./32-bit_less-than-comparator.txt";

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
