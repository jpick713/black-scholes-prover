//! A simple script to generate and verify the proof of a given program.

use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let s = 100.0;
    let r = 0.05;
    let v = 0.4;
    let strike_factors = [0.8, 1.0, 1.2];
    let tenors = [0.25, 0.5, 0.75, 1.0];
    stdin.write(&s);
    stdin.write(&r);
    stdin.write(&v);
    stdin.write(&strike_factors);
    stdin.write(&tenors);
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read output.
    let call_prices: [[f64; 3]; 4] = proof.stdout.read::<[[f64; 3]; 4]>();
    println!("call_prices: {:?}", call_prices);

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
