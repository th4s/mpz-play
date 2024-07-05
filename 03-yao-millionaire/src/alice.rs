use common::{tcp_mux, Role, DEFAULT_LOCAL};
use mpz_common::executor::MTExecutor;
use mpz_garble::{DecodePrivate, Execute, Memory};
use yao_millionaire::{setup_garble, yao_circuit};

#[tokio::main]
async fn main() {
    // Open connection and poll it in the background.
    let (future, mut ctrl) = tcp_mux(Role::Alice, DEFAULT_LOCAL).await.unwrap();
    let join_handle = tokio::spawn(future);

    // Create an executor and use it to instantiate a vm for garbled circuits.
    let mut executor = MTExecutor::new(ctrl.clone(), 32);
    let mut garble_vm = setup_garble(Role::Alice, &mut executor, 256).await.unwrap();

    // Define input and output types.
    let input_alice = garble_vm.new_private_input::<u32>("input_alice").unwrap();
    let input_bob = garble_vm.new_blind_input::<u32>("input_bob").unwrap();

    let output_alice = garble_vm.new_output::<bool>("output_alice").unwrap();
    let output_bob = garble_vm.new_output::<bool>("output_bob").unwrap();

    // Assign the input.
    garble_vm.assign(&input_alice, 5_000_000_u32).unwrap();

    // Load the AES circuit.
    let circuit = yao_circuit().unwrap();

    // Execute the circuit.
    garble_vm
        .execute(
            circuit.into(),
            &[input_alice, input_bob],
            &[output_alice.clone(), output_bob.clone()],
        )
        .await
        .unwrap();

    // Get output information.
    let mut output = garble_vm.decode_private(&[output_alice]).await.unwrap();
    garble_vm.decode_blind(&[output_bob]).await.unwrap();

    // Print the outcome.
    let is_richer: bool = output.pop().unwrap().try_into().unwrap();

    if is_richer {
        println!("You are the richer person");
    } else {
        println!("You are so poor.")
    }

    // Properly close the connection.
    ctrl.mux_mut().close();
    join_handle.await.unwrap().unwrap();
}
