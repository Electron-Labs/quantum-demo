package main

import (
	"encoding/json"
	"os"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
	"github.com/consensys/gnark/backend/plonk"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/scs"
	"github.com/consensys/gnark/test/unsafekzg"
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

const circuitDataDir = "circuits/gnark_plonk/circuit_data/"

func main() {
	var circuit SimpleCircuit
	ccs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
	if err != nil {
		panic(err)
	}
	srs, srsLagrange, err := unsafekzg.NewSRS(ccs)
	if err != nil {
		panic(err)
	}
	pk, vk, err := plonk.Setup(ccs, srs, srsLagrange)
	if err != nil {
		panic(err)
	}
	assignment := SimpleCircuit{X: 3, Y: 35}
	witness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
	if err != nil {
		panic(err)
	}
	publicWitness, err := witness.Public()
	if err != nil {
		panic(err)
	}
	proof, err := plonk.Prove(ccs, pk, witness)
	if err != nil {
		panic(err)
	}
	plonk.Verify(proof, vk, publicWitness)

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
