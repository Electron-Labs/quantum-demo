#!/bin/bash

mkdir -p build
mkdir -p circuit_data

# Downloads Powers of Tau file if not exists
if [ ! -f "pot12_final.ptau" ]; then
    echo "Downloading Powers of Tau file..."
    curl -L https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_12.ptau -o pot12_final.ptau
fi

# Compiles the circuit
circom circuit.circom --r1cs --wasm --sym --c --output build

cp input.json build/circuit_js/
cd build/circuit_js

# Generates the witness
node generate_witness.js circuit.wasm input.json witness.wtns

# Always generate fresh keys
snarkjs groth16 setup ../circuit.r1cs ../../pot12_final.ptau circuit_0000.zkey
snarkjs zkey contribute circuit_0000.zkey circuit_final.zkey --name="1st Contributor" -e="random text"
snarkjs zkey export verificationkey circuit_final.zkey verification_key.json

# Generates proof
snarkjs groth16 prove circuit_final.zkey witness.wtns proof.json public.json

# Verifies the proof
snarkjs groth16 verify verification_key.json public.json proof.json

cd ../..

# Copies files to circuit_data
mkdir -p circuit_data
cp build/circuit_js/verification_key.json circuit_data/
cp build/circuit_js/proof.json circuit_data/
cp build/circuit_js/public.json circuit_data/

# Clean up intermediate files
rm -f build/circuit_js/circuit_0000.zkey build/circuit_js/circuit_final.zkey build/circuit_js/witness.wtns 