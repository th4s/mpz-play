use common::{tcp_mux, Role, DEFAULT_LOCAL};
use garbled_circuits::setup_garble;
use mpz_circuits::circuits::AES128;
use mpz_common::executor::MTExecutor;
use mpz_garble::DecodePrivate;
use mpz_garble::{Execute, Memory};

#[tokio::main]
async fn main() {
    // Open connection and poll it in the background.
    let (future, mut ctrl) = tcp_mux(Role::Bob, DEFAULT_LOCAL).await.unwrap();
    let join_handle = tokio::spawn(future);

    // Create an executor and use it to instantiate a vm for garbled circuits.
    let mut executor = MTExecutor::new(ctrl.clone(), 32);
    let mut garble_vm = setup_garble(Role::Bob, &mut executor, 256).await.unwrap();

    // Define input and output types.
    let key = garble_vm.new_blind_input::<[u8; 16]>("key").unwrap();
    let msg = garble_vm.new_private_input::<[u8; 16]>("msg").unwrap();
    let ciphertext = garble_vm.new_output::<[u8; 16]>("ciphertext").unwrap();

    // Assign the message.
    garble_vm
        .assign(
            &msg,
            [
                0x6b_u8, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96, 0xe9, 0x3d, 0x7e, 0x11, 0x73,
                0x93, 0x17, 0x2a,
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

    // Send output information to Alice.
    garble_vm.decode_blind(&[ciphertext]).await.unwrap();

    // Properly close the connection.
    ctrl.mux_mut().close();
    join_handle.await.unwrap().unwrap();
}