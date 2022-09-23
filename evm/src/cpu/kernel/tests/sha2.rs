use std::str::FromStr;

use anyhow::Result;
use ethereum_types::U256;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};

use crate::cpu::kernel::aggregator::combined_kernel;
use crate::cpu::kernel::interpreter::run_with_kernel;

#[test]
fn test_sha2() -> Result<()> {
    let kernel = combined_kernel();
    let sha2 = kernel.global_labels["sha2"];

    let mut rng = thread_rng();

    // Generate a random message, between 0 and 9999 bytes.
    let num_bytes = rng.gen_range(0..10000);
    let message: Vec<u8> = (0..num_bytes).map(|_| rng.gen()).collect();

    // Hash the message using a standard Sha256 implementation.
    let mut hasher = Sha256::new();
    hasher.update(message.clone());
    let expected = U256::from(&hasher.finalize()[..]);

    // Load the message onto the stack.
    let mut initial_stack = vec![U256::from(num_bytes)];
    let bytes: Vec<U256> = message.iter().map(|&x| U256::from(x as u32)).collect();
    initial_stack.extend(bytes);
    initial_stack.push(U256::from_str("0xdeadbeef").unwrap());
    initial_stack.reverse();

    // Run the sha2 kernel code.
    let result = run(&kernel.code, sha2, initial_stack, &kernel.prover_inputs)?;
    let actual = result.stack()[0];

    // Check that the result is correct.
    assert_eq!(expected, actual);

    Ok(())
}
