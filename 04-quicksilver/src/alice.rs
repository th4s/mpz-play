use std::sync::Arc;

use common::{tcp_connect, Role, DEFAULT_LOCAL};
use mpz_common::Context;
use mpz_memory_core::{binary::U8, MemoryExt, ViewExt};
use mpz_vm_core::{Call, CallableExt, Execute};
use quicksilver::{get_circuit, setup_prover};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection.
    let tcp = tcp_connect(Role::Alice, DEFAULT_LOCAL).await?;
    let mut context = Context::new_single_threaded(tcp);

    // Instantiate the prover
    let mut prover = setup_prover().await?;

    // Get the circuit
    let circuit = Arc::new(get_circuit()?);

    // Alloc space for inputs
    let a: U8 = prover.alloc()?;
    let b: U8 = prover.alloc()?;

    // Setup circuit
    let ciphertext: U8 = prover.call(Call::builder(circuit).arg(a).arg(b).build()?)?;

    // Define inputs visibility
    prover.mark_private(a)?;
    prover.mark_public(b)?;

    // Assign inputs values
    let input1: u8 = 95;
    let input2: u8 = 107;

    prover.assign(a, input1)?;
    prover.assign(b, input2)?;

    // Commit values
    prover.commit(a)?;
    prover.commit(b)?;

    // Decode ciphertext
    let mut ciphertext = prover.decode(ciphertext)?;

    // Prove
    prover.execute_all(&mut context).await?;
    let result = ciphertext.try_recv()?;
    assert_eq!(result.unwrap(), input1 ^ input2);

    Ok(())
}
