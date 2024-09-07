use futures::{select, AsyncRead, AsyncWrite, FutureExt};
use mpz_common::executor::STExecutor;
use mpz_garble::{DecodePrivate, Execute, Memory};
use serio::codec::{Bincode, Codec};
use std::sync::Arc;
use yao_millionaire::{millionaire_circuit, setup_garble, web_rtc, Role};

const MONEY_BOB: u32 = 2_000_000;

fn main() {
    wasm_bindgen_futures::spawn_local(async_main());
}

async fn async_main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    let (mut web_rtc, loop_fut) = web_rtc().unwrap();

    let loop_fut = loop_fut.fuse();
    let mut loop_fut = std::pin::pin!(loop_fut);

    let millionaire_fut = async {
        loop {
            if web_rtc.connected_peers().count() > 0 {
                break;
            } else {
                web_rtc.update_peers();
            }
        }
        let channel = web_rtc.take_raw().unwrap();
        bob(channel).await
    }
    .fuse();
    let mut millionaire_fut = std::pin::pin!(millionaire_fut);

    select! {
        _ = &mut millionaire_fut => (),
        _ = &mut loop_fut => (),
    }
}

async fn bob(channel: impl AsyncRead + AsyncWrite + Unpin + Send + Sync + 'static) {
    let channel = Bincode.new_framed(channel);

    let executor = STExecutor::new(channel);
    let mut garble_vm = setup_garble(Role::Bob, executor, 256).await.unwrap();

    let money_alice = garble_vm.new_private_input::<u32>("money_alice").unwrap();
    let money_bob = garble_vm.new_blind_input::<u32>("money_bob").unwrap();

    let is_alice_richer = garble_vm.new_output::<bool>("is_alice_richer").unwrap();
    let is_bob_richer = garble_vm.new_output::<bool>("is_bob_richer").unwrap();

    // Assign the money.
    garble_vm.assign(&money_bob, MONEY_BOB).unwrap();

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
    garble_vm.decode_blind(&[is_alice_richer]).await.unwrap();
    let mut am_i_richer = garble_vm.decode_private(&[is_bob_richer]).await.unwrap();

    let am_i_richer: bool = am_i_richer.pop().unwrap().try_into().unwrap();

    if am_i_richer {
        println!("Yes, money, money money!");
    } else {
        println!("Oh nooo, I am so poor...");
    }
}
