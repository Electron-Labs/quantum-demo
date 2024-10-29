package main

import (
	"encoding/json"
	"os"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
)

type SimpleCircuit struct {
	X frontend.Variable `gnark:"x"`
	Y frontend.Variable `gnark:",public"`
}

// x**3+x+5 == y
func (circuit *SimpleCircuit) Define(api frontend.API) error {
	x3 := api.Mul(circuit.X, circuit.X, circuit.X)
	api.AssertIsEqual(circuit.Y, api.Add(x3, circuit.X, 5))
	return nil
}

const circuitDataDir = "circuits/gnark_groth16/circuit_data/"

func main() {
	var circuit SimpleCircuit
	ccs, _ := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
	pk, vk, err := groth16.Setup(ccs)
	if err != nil {
		panic(err)
	}
	assignment := SimpleCircuit{X: 3, Y: 35}
	witness, _ := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
	publicWitness, _ := witness.Public()
	proof, err := groth16.Prove(ccs, pk, witness)
	if err != nil {
		panic(err)
	}
	groth16.Verify(proof, vk, publicWitness)

	// dump vk
	fileVK, err := os.Create(circuitDataDir + "vKey.bin")
	if err != nil {
		panic(err)
	}
	_, err = vk.WriteTo(fileVK)
	if err != nil {
		panic(err)
	}

	// dump proof
	fileProof, err := os.Create(circuitDataDir + "proof.bin")
	if err != nil {
		panic(err)
	}
	_, err = proof.WriteTo(fileProof)
	if err != nil {
		panic(err)
	}

	// dump pis
	pisFrVector := publicWitness.Vector().(fr.Vector)
	pis := make([]string, len(pisFrVector))
	for i := range pisFrVector {
		pis[i] = pisFrVector[i].String()
	}
	filePis, _ := json.MarshalIndent(pis, "", " ")
	_ = os.WriteFile(circuitDataDir+"pis.json", filePis, 0644)
}
