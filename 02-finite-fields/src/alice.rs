use common::{tcp_connect, Role, DEFAULT_LOCAL};
use serio::codec::{Bincode, Codec};

#[tokio::main]
async fn main() {
    // Open a connection.
    let tcp = tcp_connect(Role::Alice, DEFAULT_LOCAL).await.unwrap();
    let _channel = Bincode.new_framed(tcp);

    // Create an executor and setup OT.

    // Setup OLE and share conversion.

    // Choose a number.

    // Perform the conversion.

    // Get the channel and send/receive starting and final numbers.

    // Check that conversion worked correctly.
}
