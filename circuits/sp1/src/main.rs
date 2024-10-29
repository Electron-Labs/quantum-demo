use sp1_sdk::{ProverClient, SP1Stdin};
use std::fs;

pub const ELF: &[u8] = include_bytes!("../elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let client = ProverClient::new();

    // Setup the inputs.
    let input = 3 as usize;
    let mut stdin = SP1Stdin::new();
    stdin.write(&input);

    let (pk, v_key) = client.setup(ELF);

    // Generate the proof
    let proof = client
        .prove(&pk, stdin)
        .compressed()
        .run()
        .expect("failed to generate proof");

    println!("Successfully generated proof!");

    // Verify the proof.
    client
        .verify(&proof, &v_key)
        .expect("failed to verify proof");
    println!("Successfully verified proof!");

    // write proof and vk1
    let path = "circuits/sp1/circuit_data";
    fs::create_dir_all(path).unwrap();

    // dump v_key
    let v_key_bytes = bincode::serialize(&v_key).unwrap();
    fs::write(format! {"{path}/v_key.bin"}, v_key_bytes).unwrap();

    // dump proof
    let proof_bytes = bincode::serialize(&proof).unwrap();
    fs::write(format! {"{path}/proof.bin"}, proof_bytes).unwrap();
}
