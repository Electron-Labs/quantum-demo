use risc0::{DEMO_ELF, DEMO_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};
use std::fs;

pub fn main() {
    let input = 3 as usize;
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();
    let prover = default_prover();
    let receipt = prover
        .prove_with_opts(env, DEMO_ELF, &ProverOpts::succinct())
        .unwrap()
        .receipt;
    receipt.verify(DEMO_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );

    let path = "circuit_data";
    fs::create_dir_all(path).unwrap();

    // dump ID
    serde_json::to_writer(
        std::fs::File::create(format! {"{path}/method_id.json"}).unwrap(),
        &DEMO_ID,
    )
    .unwrap();

    // dump receipt
    let receipt_bytes = bincode::serialize(&receipt).unwrap();
    fs::write(format! {"{path}/receipt.bin"}, receipt_bytes).unwrap();
}
