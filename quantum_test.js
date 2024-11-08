const { Quantum } = require("quantum-sdk")
const { ProofType } = require("quantum-sdk/dist/src/enum/proof_type")
const dotenv = require('dotenv')
const args = require('yargs').argv;

dotenv.config()


const RPC_ENDPOINT = process.env.RPC_ENDPOINT
const ACCESS_KEY = process.env.ACCESS_KEY

const main = async () => {
  const quantum = new Quantum(RPC_ENDPOINT, ACCESS_KEY);
  let rpcLive = await quantum.checkServerConnection();
  if (!rpcLive) {
    throw new Error(`${rpcLive}`)
  }

  const scheme = args.scheme
  var circuitPath = `circuits/${scheme}/circuit_data`

  let circuitHash, proofResponse
  try {
    if (scheme == "gnark_groth16") {
      let vKeyPath = `${circuitPath}/vKey.bin`
      let proofPath = `${circuitPath}/proof.bin`
      let pisPath = `${circuitPath}/pis.json`
      circuitHash = (await quantum.registerGnarkGroth16Circuit(vKeyPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitGnarkGroth16Proof(proofPath, pisPath, circuitHash));
    } else if (scheme == "snarkjs_groth16") {
      let vKeyPath = `${circuitPath}/verification_key.json`
      let proofPath = `${circuitPath}/proof.json`
      let pisPath = `${circuitPath}/public.json`
      circuitHash = (await quantum.registerSnarkJSGroth16Circuit(vKeyPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitSnarkJSGroth16Proof(proofPath, pisPath, circuitHash));
    } else if (scheme == "risc0") {
      let vKeyPath = `${circuitPath}/method_id.json`
      let receiptPath = `${circuitPath}/receipt.bin`
      circuitHash = (await quantum.registerRisc0Circuit(vKeyPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitRisc0Proof(receiptPath, circuitHash));
    } else if (scheme == "sp1") {
      let vKeyPath = `${circuitPath}/v_key.bin`
      let proofPath = `${circuitPath}/proof.bin`
      circuitHash = (await quantum.registerSp1Circuit(vKeyPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitSp1Proof(proofPath, circuitHash));
    } else if (scheme == "plonky2") {
      let commonDataPath = `${circuitPath}/common_data.bin`
      let verifierOnlyDataPath = `${circuitPath}/verifier_only.bin`
      let proofPath = `${circuitPath}/proof.bin`
      console.log("commonDataPath", commonDataPath)
      circuitHash = (await quantum.registerPlonky2Circuit(commonDataPath, verifierOnlyDataPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitPlonky2Proof(proofPath, circuitHash));
    } else if (scheme == "gnark_plonk") {
      let vKeyPath = `${circuitPath}/vKey.bin`
      let proofPath = `${circuitPath}/proof.bin`
      let pisPath = `${circuitPath}/pis.json`
      circuitHash = (await quantum.registerGnarkPlonkCircuit(vKeyPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitGnarkPlonkProof(proofPath, pisPath, circuitHash));
    } else if (scheme == "halo2_kzg") {
      let sg2Path = `${circuitPath}/sg2.json`
      let protocolPath = `${circuitPath}/protocol.json`
      let proofPath = `${circuitPath}/proof.bin`
      let instancesPath = `${circuitPath}/instances.json`
      circuitHash = (await quantum.registerHalo2KZGCircuit(sg2Path, protocolPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitHalo2KZGProof(proofPath, instancesPath, circuitHash));
    } else if (scheme == "halo2_kzg_evm") {
      let sg2Path = `${circuitPath}/sg2.json`
      let protocolPath = `${circuitPath}/protocol.json`
      let proofPath = `${circuitPath}/proof.bin`
      let instancesPath = `${circuitPath}/instances.json`
      circuitHash = (await quantum.registerHalo2KZGEvmCircuit(sg2Path, protocolPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitHalo2KZGEvmProof(proofPath, instancesPath, circuitHash));
    }
  } catch (e) {
    console.log("error:", e)
    if (circuitHash == undefined) console.log("Circuit Registration failed!")
    if (proofResponse == undefined) console.log("Proof Submission failed!")
  } finally {
    console.log("circuitHash", circuitHash)
    console.log("proofResponse", proofResponse)
  }
}

if (require.main == module) {
  main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
  });
}