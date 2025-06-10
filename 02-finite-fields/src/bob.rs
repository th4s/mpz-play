use common::{tcp_connect, Role, DEFAULT_LOCAL};
use mpz_common::Context;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection.
    let tcp = tcp_connect(Role::Bob, DEFAULT_LOCAL).await?;
    let mut context = Context::new_single_threaded(tcp);

    // Setup OT.

    // Setup OLE and share conversion.

    // Choose a number.

    // Allocate space for pre-processing.

    // Perform the conversion.

    // Get the channel and send/receive starting and final numbers.

    // Check that conversion worked correctly.

    Ok(())
}
