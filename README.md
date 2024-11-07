# quantum-demo

Contains demos for different proving schemes supported by Quantum.

### Test Quantum API (Circuit Registration + Proof Submission)

- schemes = {gnark_groth16, snarkjs_groth16, risc0, sp1, plonky2, gnark_plonk}
- `node quantum_test.js --scheme gnark_groth16`

### Generate circuit data

- gnark_groth16
  - `go run circuits/gnark_groth16/circuit.go`
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
  - `go run circuits/gnark_plonk/circuit.go`
- halo2_kzg
  - `cd circuits/halo2_kzg`
  - `cargo run -r --package halo2_kzg --bin halo2_kzg`
