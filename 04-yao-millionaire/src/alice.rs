use common::{web_rtc, Role};
use mpz_common::executor::STExecutor;
use mpz_garble::{DecodePrivate, Execute, Memory};
use serio::codec::{Bincode, Codec};
use std::sync::Arc;
use yao_millionaire::{millionaire_circuit, setup_garble};

const MONEY_ALICE: u32 = 5_000_000;

#[tokio::main]
async fn main() {
    let mut socket = web_rtc().await.unwrap();

    let web_rtc = socket.take_raw().unwrap();
    let channel = Bincode.new_framed(web_rtc);

    let executor = STExecutor::new(channel);
    let mut garble_vm = setup_garble(Role::Alice, executor, 256).await.unwrap();

    let money_alice = garble_vm.new_private_input::<u32>("money_alice").unwrap();
    let money_bob = garble_vm.new_blind_input::<u32>("money_bob").unwrap();

    let is_alice_richer = garble_vm.new_output::<bool>("is_alice_richer").unwrap();
    let is_bob_richer = garble_vm.new_output::<bool>("is_bob_richer").unwrap();

    // Assign the money.
    garble_vm.assign(&money_alice, MONEY_ALICE).unwrap();

    // Load the millionaire circuit.
    let circuit = Arc::new(millionaire_circuit().unwrap());

    // Execute the circuit.
    garble_vm
        .execute(
            circuit,
            &[money_alice, money_bob],
            &[is_alice_richer.clone(), is_bob_richer.clone()],
        )
        .await
        .unwrap();

    // Receive output information from Bob.
    let mut am_i_richer = garble_vm.decode_private(&[is_alice_richer]).await.unwrap();
    garble_vm.decode_blind(&[is_bob_richer]).await.unwrap();

    let am_i_richer: bool = am_i_richer.pop().unwrap().try_into().unwrap();

    if am_i_richer {
        println!("Yes, money, money money!");
    } else {
        println!("Oh nooo, I am so poor...");
    }
}
