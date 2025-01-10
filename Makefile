.PHONY: all clean gnark_groth16 snarkjs_groth16 risc0 sp1 plonky2 gnark_plonk halo2_kzg halo2_kzg_evm sudoku_groth16 setup

setup:
	@bash setup.sh && npm install && node test_credentials.js

all: setup gnark_groth16 snarkjs_groth16 risc0 sp1 plonky2 gnark_plonk halo2_kzg halo2_kzg_evm sudoku_groth16

gnark_groth16:
	cd circuits/gnark_groth16 && go run circuit.go
	node quantum_test.js --scheme gnark_groth16

snarkjs_groth16:
	cd circuits/snarkjs_groth16 && ./build.sh
	node quantum_test.js --scheme snarkjs_groth16

risc0:
	cd circuits/risc0 && cargo run -r --package risc0 --bin risc0
	node quantum_test.js --scheme risc0

sp1:
	cd circuits/sp1/program && cargo prove build --output-directory ../elf
	cd circuits/sp1 && cargo run -r --package sp1 --bin sp1
	node quantum_test.js --scheme sp1

plonky2:
	cd circuits/plonky2 && cargo run -r --package plonky2:0.1.0 --bin plonky2
	node quantum_test.js --scheme plonky2

gnark_plonk:
	cd circuits/gnark_plonk && go run circuit.go
	node quantum_test.js --scheme gnark_plonk

halo2_kzg:
	cd circuits/halo2_kzg && cargo run -r --package halo2_kzg --bin halo2_kzg
	node quantum_test.js --scheme halo2_kzg

halo2_kzg_evm:
	cd circuits/halo2_kzg_evm && cargo run -r --package halo2_kzg_evm --bin halo2_kzg_evm
	node quantum_test.js --scheme halo2_kzg_evm

sudoku_groth16:
	cd circuits/sudoku_groth16 && ./build.sh
	node quantum_test.js --scheme sudoku_groth16

clean:
	cd circuits/risc0 && cargo clean
	cd circuits/sp1 && cargo clean
	cd circuits/plonky2 && cargo clean
	cd circuits/halo2_kzg && cargo clean
	cd circuits/halo2_kzg_evm && cargo clean
	cd circuits/snarkjs_groth16 && \
	rm -f *.ptau *.zkey *.r1cs *.sym witness.wtns proof.json public.json verification_key.json input.json && \
	rm -rf circuit_js circuit_data && \
	cd circuit_cpp && make clean && cd .. && \
	rm -rf circuit_cpp/circuit 