use anyhow::Error as Anyhow;
use rand::{rngs::StdRng, SeedableRng};

use mpz_circuits::Circuit;
use mpz_core::Block;
use mpz_memory_core::correlated::Delta;
use mpz_ot::{
    chou_orlandi::{Receiver as BaseReceiver, Sender as BaseSender},
    kos::{Receiver, ReceiverConfig, Sender, SenderConfig},
};
use mpz_zk::{Prover, Verifier};

pub async fn setup_prover() -> Result<Prover<Receiver<BaseSender>>, Anyhow> {
    let base_sender = BaseSender::new();
    let receiver = Receiver::new(ReceiverConfig::default(), base_sender);

    let prover = Prover::new(receiver);

    Ok(prover)
}

pub async fn setup_verifier() -> Result<Verifier<Sender<BaseReceiver>>, Anyhow> {
    let base_receiver = BaseReceiver::new();

    let mut rng = StdRng::seed_from_u64(0);
    let delta = Block::random(&mut rng);

    let sender = Sender::new(SenderConfig::default(), delta, base_receiver);

    let verifier = Verifier::new(Delta::new(delta), sender);

    Ok(verifier)
}

pub fn get_circuit() -> Result<Circuit, Anyhow> {
    let circ = Circuit::parse_str(
        r#"
8 24
2 8 8
1 8

2 1 0 8 16 XOR
2 1 1 9 17 XOR
2 1 2 10 18 XOR
2 1 3 11 19 XOR
2 1 4 12 20 XOR
2 1 5 13 21 XOR
2 1 6 14 22 XOR
2 1 7 15 23 XOR
    "#,
    )?;

    Ok(circ)
}

#[cfg(test)]
mod tests {
    use super::*;
    use itybity::{FromBitIterator, ToBits};

    #[test]
    fn simple_circuit() {
        /*
                export default (io: Summon.IO) => {
                    const input1 = io.input("alice", "input1", summon.number());
                    const input2 = io.input("alice", "input2", summon.number());

                    let res = input1 & input2;

                    io.outputPublic("res", res);
                };
        */
        // let circ = Circuit::parse("circuit.txt").unwrap();
        let circ = Circuit::parse_str(
            r#"
8 24
2 8 8
1 8

2 1 0 8 16 AND
2 1 1 9 17 AND
2 1 2 10 18 AND
2 1 3 11 19 AND
2 1 4 12 20 AND
2 1 5 13 21 AND
2 1 6 14 22 AND
2 1 7 15 23 AND
    "#,
        )
        .unwrap();

        let a: u8 = 5;
        let b: u8 = 6;
        let res = circ.evaluate(a.iter_lsb0().chain(b.iter_lsb0())).unwrap();
        let res: u8 = FromBitIterator::from_lsb0_iter(res);
        assert_eq!(res, a & b);
    }

    #[test]
    fn simple_circuit2() {
        let circ = Circuit::parse_str(
            r#"
5 13
2 4 4
1 4

2 1 0 4 8 AND
2 1 1 5 9 XOR
2 1 2 6 10 AND
2 1 3 7 11 XOR
2 1 0 8 12 AND
    "#,
        )
        .unwrap();

        let a = vec![true, false, true, false];
        let b = vec![true, true, true, false];
        let res = circ.evaluate(a.iter_lsb0().chain(b.iter_lsb0())).unwrap();
        println!("res: {:?}", res);
    }

    #[test]
    fn simple_circuit3() {
        let circ = Circuit::parse_str(
            r#"
4 8
2 2 2
1 2

2 1 0 2 4 AND
2 1 1 3 5 XOR
2 1 1 5 6 AND
2 1 2 4 7 XOR
    "#,
        )
        .unwrap();

        let a = vec![false, true];
        let b = vec![true, false];
        let res = circ.evaluate(a.iter_lsb0().chain(b.iter_lsb0())).unwrap();
        println!("res: {:?}", res);
    }

    #[test]
    fn mul_circuit() {
        // z = x * y
        let circ = Circuit::parse_str(
            r#"
1 3
2 1 1
1 1

2 1 0 1 2 AND
    "#,
        )
        .unwrap();

        let a: bool = false;
        let b = true;
        let res = circ.evaluate(a.iter_lsb0().chain(b.iter_lsb0())).unwrap();
        assert_eq!(res, vec![a & b]);
    }

    #[test]
    fn mul_circuit2() {
        // z = x * y
        let circ = Circuit::parse_str(
            r#"
136 152
2 8 8
1 8

2 1 0 8 144 AND
2 1 0 9 16 AND
2 1 1 8 17 AND
2 1 16 17 145 XOR
2 1 0 10 18 AND
2 1 1 9 19 AND
2 1 18 19 20 XOR
2 1 16 17 21 AND
2 1 20 21 22 XOR
2 1 2 8 23 AND
2 1 22 23 146 XOR
2 1 0 11 24 AND
2 1 1 10 25 AND
2 1 24 25 26 XOR
2 1 18 19 27 AND
2 1 21 20 28 AND
2 1 27 28 29 XOR
2 1 26 29 30 XOR
2 1 2 9 31 AND
2 1 3 8 32 AND
2 1 31 32 33 XOR
2 1 30 33 34 XOR
2 1 22 23 35 AND
2 1 34 35 147 XOR
2 1 0 12 36 AND
2 1 1 11 37 AND
2 1 36 37 38 XOR
2 1 24 25 39 AND
2 1 29 26 40 AND
2 1 39 40 41 XOR
2 1 38 41 42 XOR
2 1 2 10 43 AND
2 1 3 9 44 AND
2 1 43 44 45 XOR
2 1 31 32 46 AND
2 1 45 46 47 XOR
2 1 42 47 48 XOR
2 1 30 33 49 AND
2 1 35 34 50 AND
2 1 49 50 51 XOR
2 1 48 51 52 XOR
2 1 4 8 53 AND
2 1 52 53 148 XOR
2 1 0 13 54 AND
2 1 1 12 55 AND
2 1 54 55 56 XOR
2 1 36 37 57 AND
2 1 41 38 58 AND
2 1 57 58 59 XOR
2 1 56 59 60 XOR
2 1 2 11 61 AND
2 1 3 10 62 AND
2 1 61 62 63 XOR
2 1 43 44 64 AND
2 1 46 45 65 AND
2 1 64 65 66 XOR
2 1 63 66 67 XOR
2 1 60 67 68 XOR
2 1 42 47 69 AND
2 1 51 48 70 AND
2 1 69 70 71 XOR
2 1 68 71 72 XOR
2 1 4 9 73 AND
2 1 5 8 74 AND
2 1 73 74 75 XOR
2 1 72 75 76 XOR
2 1 52 53 77 AND
2 1 76 77 149 XOR
2 1 0 14 78 AND
2 1 1 13 79 AND
2 1 78 79 80 XOR
2 1 54 55 81 AND
2 1 59 56 82 AND
2 1 81 82 83 XOR
2 1 80 83 84 XOR
2 1 2 12 85 AND
2 1 3 11 86 AND
2 1 85 86 87 XOR
2 1 61 62 88 AND
2 1 66 63 89 AND
2 1 88 89 90 XOR
2 1 87 90 91 XOR
2 1 84 91 92 XOR
2 1 60 67 93 AND
2 1 71 68 94 AND
2 1 93 94 95 XOR
2 1 92 95 96 XOR
2 1 4 10 97 AND
2 1 5 9 98 AND
2 1 97 98 99 XOR
2 1 73 74 100 AND
2 1 99 100 101 XOR
2 1 6 8 102 AND
2 1 101 102 103 XOR
2 1 96 103 104 XOR
2 1 72 75 105 AND
2 1 77 76 106 AND
2 1 105 106 107 XOR
2 1 104 107 150 XOR
2 1 0 15 108 AND
2 1 1 14 109 AND
2 1 108 109 110 XOR
2 1 78 79 111 AND
2 1 83 80 112 AND
2 1 111 112 113 XOR
2 1 110 113 114 XOR
2 1 2 13 115 AND
2 1 3 12 116 AND
2 1 115 116 117 XOR
2 1 85 86 118 AND
2 1 90 87 119 AND
2 1 118 119 120 XOR
2 1 117 120 121 XOR
2 1 114 121 122 XOR
2 1 84 91 123 AND
2 1 95 92 124 AND
2 1 123 124 125 XOR
2 1 122 125 126 XOR
2 1 4 11 127 AND
2 1 5 10 128 AND
2 1 127 128 129 XOR
2 1 97 98 130 AND
2 1 100 99 131 AND
2 1 130 131 132 XOR
2 1 129 132 133 XOR
2 1 6 9 134 AND
2 1 7 8 135 AND
2 1 134 135 136 XOR
2 1 133 136 137 XOR
2 1 101 102 138 AND
2 1 137 138 139 XOR
2 1 126 139 140 XOR
2 1 96 103 141 AND
2 1 107 104 142 AND
2 1 141 142 143 XOR
2 1 140 143 151 XOR
    "#,
        )
        .unwrap();

        let a: u8 = 7;
        let b: u8 = 5;
        let res: u8 = circ
            .evaluate(a.iter_lsb0().chain(b.iter_lsb0()))
            .map(|i| FromBitIterator::from_lsb0_iter(i))
            .unwrap();

        assert_eq!(res, a * b);
    }
}
