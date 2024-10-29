# quantum-demo
Contains demos for different proving schemes supported by Quantum.

### Test Quantum API (Circuit Registration + Proof Submission)
- schemes = {gnark_groth16, snarkjs_groth16}
- `node quantum_test.js --scheme gnark_groth16`

### Generate circuit data
- `go run circuits/gnark_groth16/circuit.go` (gnark_groth16)
- circuits/snarkjs_groth16/circuit.circom (snarkjs_groth16)
- `RISC0_DEV_MODE=0 cargo run --package risc0 --bin risc0` (risc0)
