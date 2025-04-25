use common::{tcp_connect, Role, DEFAULT_LOCAL};
use garbled_circuits::setup_garble;
use mpz_circuits::circuits::AES128;
use mpz_common::Context;
use mpz_memory_core::{binary::U8, Array, MemoryExt, ViewExt};
use mpz_vm_core::{Call, CallableExt, Execute};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection.
    let tcp = tcp_connect(Role::Bob, DEFAULT_LOCAL).await?;
    let mut context = Context::new_single_threaded(tcp);

    // Instantiate a vm for garbled circuits.
    let (_, mut evaluator) = setup_garble().await?;

    // Define input types.
    let key: Array<U8, 16> = evaluator.alloc()?;
    let msg: Array<U8, 16> = evaluator.alloc()?;

    // Define input visibility.
    evaluator.mark_blind(key)?;
    evaluator.mark_private(msg)?;

    // Define output.
    let ciphertext: Array<U8, 16> =
        evaluator.call(Call::builder(AES128.clone()).arg(key).arg(msg).build()?)?;

    let mut ciphertext = evaluator.decode(ciphertext)?;

    // Assign the message.
    evaluator.assign(
        msg,
        [
            0x6b_u8, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96, 0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93,
            0x17, 0x2a,
        ],
    )?;

    // Commit the values
    evaluator.commit(key)?;
    evaluator.commit(msg)?;

    // Execute the circuit.
    evaluator.execute_all(&mut context).await?;

    let output = ciphertext.try_recv()?.unwrap();
    println!("Ciphertext: {:x?}", output);

    Ok(())
}
