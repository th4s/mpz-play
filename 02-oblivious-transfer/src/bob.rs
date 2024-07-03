use common::{tcp_mux, Role, DEFAULT_ADDRESS};

#[tokio::main]
async fn main() {
    // Open connection and poll it in the background.
    let (future, mut ctrl) = tcp_mux(Role::Bob, DEFAULT_ADDRESS).await.unwrap();
    let join_handle = tokio::spawn(future);

    // Your code
    // ######################################################################

    // ######################################################################
    // Properly close the connection.
    ctrl.mux_mut().close();
    join_handle.await.unwrap().unwrap();
}
