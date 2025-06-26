use std::sync::Arc;

use common::{tcp_connect, Role, DEFAULT_LOCAL};
use mpz_common::Context;
use mpz_memory_core::{binary::U8, MemoryExt, ViewExt};
use mpz_vm_core::{Call, CallableExt, Execute};
use quicksilver::{get_circuit, setup_verifier};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection.
    let tcp = tcp_connect(Role::Bob, DEFAULT_LOCAL).await?;
    let mut context = Context::new_single_threaded(tcp);

    // Instantiate the verifier
    let mut verifier = setup_verifier().await?;

    // Get the circuit
    let circuit = Arc::new(get_circuit()?);

    // Alloc space for inputs
    let a: U8 = verifier.alloc()?;
    let b: U8 = verifier.alloc()?;

    // Setup circuit
    let ciphertext: U8 = verifier.call(Call::builder(circuit).arg(a).arg(b).build()?)?;

    // Define input visibility
    verifier.mark_blind(a)?;
    verifier.mark_public(b)?;

    // Assign inputs values
    let input1: u8 = 95;
    let input2: u8 = 107;

    verifier.assign(b, input2)?;

    // Commit values
    verifier.commit(a)?;
    verifier.commit(b)?;

    // Decode ciphertext
    let mut ciphertext = verifier.decode(ciphertext)?;

    // Verify
    verifier.execute_all(&mut context).await?;
    let result = ciphertext.try_recv()?;
    assert_eq!(result.unwrap(), input1 ^ input2);

    Ok(())
}
