use common::{tcp_connect, Role, DEFAULT_LOCAL};
use garbled_circuits::setup_garble;
use mpz_circuits::circuits::AES128;
use mpz_common::Context;
use mpz_memory_core::{binary::U8, Array, MemoryExt, ViewExt};
use mpz_vm_core::{Call, CallableExt, Execute};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection.
    let tcp = tcp_connect(Role::Alice, DEFAULT_LOCAL).await?;
    let mut context = Context::new_single_threaded(tcp);

    // Instantiate a vm for garbled circuits.
    let (mut garble_vm, _) = setup_garble().await?;

    // Define input types.
    let key: Array<U8, 16> = garble_vm.alloc()?;
    let message: Array<U8, 16> = garble_vm.alloc()?;

    // Define input visibility.
    garble_vm.mark_private(key)?;
    garble_vm.mark_blind(message)?;

    // Define output
    let ciphertext: Array<U8, 16> = garble_vm.call(
        Call::builder(AES128.clone())
            .arg(key)
            .arg(message)
            .build()?,
    )?;

    let mut ciphertext = garble_vm.decode(ciphertext)?;

    // Assign the key.
    garble_vm.assign(
        key,
        [
            0x2b_u8, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ],
    )?;

    // Commit the values
    garble_vm.commit(key)?;
    garble_vm.commit(message)?;

    // Execute the circuit.
    garble_vm.execute_all(&mut context).await?;

    let output = ciphertext.try_recv()?.unwrap();
    println!("Ciphertext: {:x?}", output);

    Ok(())
}
