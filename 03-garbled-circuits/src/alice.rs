use common::{tcp_connect, Role, DEFAULT_LOCAL};
use garbled_circuits::setup_garbler;
use mpz_circuits::circuits::AES128;
use mpz_common::Context;
use mpz_memory_core::{binary::U8, Array, MemoryExt, ViewExt};
use mpz_vm_core::{Call, CallableExt, Execute};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection.
    let tcp = tcp_connect(Role::Alice, DEFAULT_LOCAL).await?;

    // Instantiate a vm for garbled circuits.

    // Define input types.

    // Define input visibility.

    // Define output

    // Assign the key.

    // Commit the values

    // Execute the circuit.

    Ok(())
}
