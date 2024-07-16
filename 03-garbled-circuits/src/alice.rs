use common::{tcp_mux, Role, DEFAULT_LOCAL};
use garbled_circuits::setup_garble;
use mpz_circuits::circuits::AES128;
use mpz_common::executor::MTExecutor;
use mpz_garble::{DecodePrivate, Execute, Memory};

#[tokio::main]
async fn main() {
    // Open connection and poll it in the background.
    let (future, mut ctrl) = tcp_mux(Role::Alice, DEFAULT_LOCAL).await.unwrap();
    let join_handle = tokio::spawn(future);

    // Create an executor and use it to instantiate a vm for garbled circuits.
    let mut executor = MTExecutor::new(ctrl.clone(), 32);
    let mut garble_vm = setup_garble(Role::Alice, &mut executor, 256).await.unwrap();

    // Define input and output types.
    let key = garble_vm.new_private_input::<[u8; 16]>("key").unwrap();
    let msg = garble_vm.new_blind_input::<[u8; 16]>("msg").unwrap();
    let ciphertext = garble_vm.new_output::<[u8; 16]>("ciphertext").unwrap();

    // Assign the key.
    garble_vm
        .assign(
            &key,
            [
                0x2b_u8, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09,
                0xcf, 0x4f, 0x3c,
            ],
        )
        .unwrap();

    // Load the AES circuit.
    let circuit = AES128.clone();

    // Execute the circuit.
    garble_vm
        .execute(circuit, &[key, msg], &[ciphertext.clone()])
        .await
        .unwrap();

    // Receive output information from Bob.
    let mut output = garble_vm.decode_private(&[ciphertext]).await.unwrap();

    // Print the encrypted text.
    let encrypted: [u8; 16] = output.pop().unwrap().try_into().unwrap();
    println!("Encrypted text is {:x?}", encrypted);

    // Properly close the connection.
    ctrl.mux_mut().close();
    join_handle.await.unwrap().unwrap();
}