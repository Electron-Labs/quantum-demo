# quantum-demo

This repo contains demos for different proving schemes supported by Quantum.

## Quick Start
Install dependencies and setup credentials
```bash 
make setup
```
To clean build artifacts, run:
```bash
make clean
```

## Testing Individual Schemes

You can build and test individual proving schemes using:

```bash
make <scheme_name>
```

Available schemes:
- `gnark_groth16`
- `snarkjs_groth16`
- `risc0`
- `sp1`
- `plonky2`
- `gnark_plonk`
- `halo2_kzg`
- `halo2_kzg_evm`

