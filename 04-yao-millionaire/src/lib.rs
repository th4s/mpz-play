use anyhow::Result as Anyhow;
use mpz_circuits::{types::ValueType, Circuit};

pub fn parse_lt_comparator(filename: impl AsRef<str>) -> Anyhow<Circuit> {
    let circuit = Circuit::parse(
        filename.as_ref(),
        &[ValueType::U32, ValueType::U32],
        &[ValueType::Bit],
    )?;

    let circuit = circuit.reverse_input(0).reverse_input(1).reverse_output(0);

    Ok(circuit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mpz_circuits::evaluate;

    const COMPARATOR_FILENAME: &str = "./32-bit_less-than-comparator.txt";

    #[test]
    fn test_comparator() {
        let circuit = parse_lt_comparator(COMPARATOR_FILENAME).unwrap();

        let is_bob_richer_or_as_rich = |alice: u32, bob: u32| {
            let wealth_alice: u32 = alice;
            let wealth_bob: u32 = bob;

            evaluate!(circuit, fn(wealth_alice, wealth_bob) -> bool).unwrap()
        };

        #[allow(clippy::bool_assert_comparison)]
        {
            assert_eq!(true, is_bob_richer_or_as_rich(0, 1));
            assert_eq!(true, is_bob_richer_or_as_rich(0, u32::MAX));
            assert_eq!(true, is_bob_richer_or_as_rich(1_000_000, 1_000_001));
            assert_eq!(true, is_bob_richer_or_as_rich(999_999, 1_000_000));

            assert_eq!(false, is_bob_richer_or_as_rich(0, 0));
            assert_eq!(false, is_bob_richer_or_as_rich(100, 100));
            assert_eq!(false, is_bob_richer_or_as_rich(2_000_000, 1_000_000));
            assert_eq!(false, is_bob_richer_or_as_rich(u32::MAX, u32::MAX - 1));
        }
    }
}