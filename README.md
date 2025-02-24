# quantum-demo

Contains demos for different proving schemes supported by Quantum.

### Test Quantum API (Circuit Registration + Proof Submission)

- schemes = {gnark_groth16, snarkjs_groth16, risc0, sp1, plonky2, gnark_plonk, halo2_kzg, halo2_kzg_evm, nitro_attestation}
- `node quantum_test.js --scheme gnark_groth16`

### Generate circuit data

- gnark_groth16
  - `cd circuits/gnark_groth16`
  - `go run circuit.go`
- snarkjs_groth16
  - circuits/snarkjs_groth16/circuit.circom
- risc0
  - `cd circuits/risc0`
  - `cargo run -r --package risc0 --bin risc0`
- sp1
  - `cd circuits/sp1/program`
  - `cargo prove build --output-directory circuits/sp1/elf`
  - `cd ..`
  - `cargo run -r --package sp1 --bin sp1`
- plonky2
  - `cd circuits/plonky2`
  - `cargo run -r --package plonky2:0.1.0 --bin plonky2`
- gnark_plonk plonky2
  - cd `circuits/gnark_plonk`
  - `go run circuit.go`
- halo2_kzg
  - `cd circuits/halo2_kzg`
  - `cargo run -r --package halo2_kzg --bin halo2_kzg`
- halo2_kzg_evm
  - `cd circuits/halo2_kzg_evm`
  - `cargo run -r --package halo2_kzg_evm --bin halo2_kzg_evm`
- nitro_attestation
  - [run](https://docs.aws.amazon.com/enclaves/latest/user/cmd-nitro-run-enclave.html?utm_source=chatgpt.com#cmd-nitro-run-enclave-syntax "run") your [AWS Nitro enclave](https://docs.aws.amazon.com/enclaves/latest/user/nitro-enclave.html "AWS Nitro enclave") [Note down its CID and PORT]
  - `cd circuits/nitro_attestation`
  - `cargo run --package nitro_attestation --bin nitro_attestation -- --cid <cid> --port <port>`
