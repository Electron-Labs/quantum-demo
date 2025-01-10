#!/bin/bash

# Constants
INPUT_VALUE=3
CIRCUIT_NAME="circuit"
POWER_LEVEL=12
CURVE="bn128"

# Directory structure
CIRCUIT_JS_DIR="circuit_js"
CIRCUIT_CPP_DIR="circuit_cpp"
CIRCUIT_DATA_DIR="circuit_data"

setup_directories() {
    mkdir -p "$CIRCUIT_JS_DIR"
    mkdir -p "$CIRCUIT_DATA_DIR"
    printf "{\"x\": \"$INPUT_VALUE\"}\n" > input.json
}

compile_circuit() {
    circom "$CIRCUIT_NAME.circom" --r1cs --wasm --sym
    cd "$CIRCUIT_CPP_DIR" && make && cd ..
}

setup_powers_of_tau() {
    local POT_0="pot12_0000.ptau"
    local POT_1="pot12_0001.ptau"
    local POT_FINAL="pot12_final.ptau"

    snarkjs powersoftau new "$CURVE" "$POWER_LEVEL" "$POT_0"
    snarkjs powersoftau contribute "$POT_0" "$POT_1" --name="First contribution" -e="random text"
    snarkjs powersoftau prepare phase2 "$POT_1" "$POT_FINAL"
}

generate_proving_key() {
    local ZKEY_0="circuit_0000.zkey"
    local ZKEY_FINAL="circuit_final.zkey"

    snarkjs groth16 setup "$CIRCUIT_NAME.r1cs" pot12_final.ptau "$ZKEY_0"
    snarkjs zkey contribute "$ZKEY_0" "$ZKEY_FINAL" --name="1st Contributor" -e="random text"
    snarkjs zkey export verificationkey "$ZKEY_FINAL" "$CIRCUIT_DATA_DIR/verification_key.json"
}

generate_proof() {
    node "$CIRCUIT_JS_DIR/generate_witness.js" "$CIRCUIT_JS_DIR/$CIRCUIT_NAME.wasm" input.json witness.wtns
    snarkjs groth16 prove circuit_final.zkey witness.wtns "$CIRCUIT_DATA_DIR/proof.json" "$CIRCUIT_DATA_DIR/public.json"
    rm input.json
}

main() {
    setup_directories
    compile_circuit
    setup_powers_of_tau
    generate_proving_key
    generate_proof
}

# Executes main function
main 