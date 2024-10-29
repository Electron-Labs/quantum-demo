# quantum-demo

Contains demos for different proving schemes supported by Quantum.

### Test Quantum API (Circuit Registration + Proof Submission)

- schemes = {gnark_groth16, snarkjs_groth16, risc0, sp1, plonky2, gnark_plonk}
- `node quantum_test.js --scheme gnark_groth16`

### Generate circuit data

- `go run circuits/gnark_groth16/circuit.go` (gnark_groth16)
- circuits/snarkjs_groth16/circuit.circom (snarkjs_groth16)
- `RISC0_DEV_MODE=0 cargo run --package risc0 --bin risc0` (risc0)
- sp1
  - `cd circuits/sp1/program`
  - `cargo prove build --output-directory circuits/sp1/elf`
  - `cd ../../../`
  - `cargo run -r --package sp1 --bin sp1`
  - `cargo run --package plonky2:0.1.0 --bin plonky2` (plonky2)
- `go run circuits/gnark_plonk/circuit.go` (gnark_plonk)
