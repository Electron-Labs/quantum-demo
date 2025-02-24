const { Quantum } = require("quantum-sdk")
const dotenv = require('dotenv');
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

  let combinedVKeyHash, proofResponse
  try {
    if (scheme == "gnark_groth16") {
      let vKeyPath = `${circuitPath}/vKey.bin`
      let proofPath = `${circuitPath}/proof.bin`
      let pisPath = `${circuitPath}/pis.json`
      combinedVKeyHash = (await quantum.registerGnarkGroth16Circuit(vKeyPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitGnarkGroth16Proof(proofPath, pisPath, combinedVKeyHash));
    } else if (scheme == "snarkjs_groth16") {
      let vKeyPath = `${circuitPath}/verification_key.json`
      let proofPath = `${circuitPath}/proof.json`
      let pisPath = `${circuitPath}/public.json`
      combinedVKeyHash = (await quantum.registerSnarkJSGroth16Circuit(vKeyPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitSnarkJSGroth16Proof(proofPath, pisPath, combinedVKeyHash));
    } else if (scheme == "risc0") {
      let vKeyPath = `${circuitPath}/method_id.json`
      let receiptPath = `${circuitPath}/receipt.bin`
      combinedVKeyHash = (await quantum.registerRisc0Circuit(vKeyPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitRisc0Proof(receiptPath, combinedVKeyHash));
    } else if (scheme == "sp1") {
      let vKeyPath = `${circuitPath}/v_key.bin`
      let proofPath = `${circuitPath}/proof.bin`
      combinedVKeyHash = (await quantum.registerSp1Circuit(vKeyPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitSp1Proof(proofPath, combinedVKeyHash));
    } else if (scheme == "plonky2") {
      let commonDataPath = `${circuitPath}/common_data.bin`
      let verifierOnlyDataPath = `${circuitPath}/verifier_only.bin`
      let proofPath = `${circuitPath}/proof.bin`
      console.log("commonDataPath", commonDataPath)
      combinedVKeyHash = (await quantum.registerPlonky2Circuit(commonDataPath, verifierOnlyDataPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitPlonky2Proof(proofPath, combinedVKeyHash));
    } else if (scheme == "gnark_plonk") {
      let vKeyPath = `${circuitPath}/vKey.bin`
      let proofPath = `${circuitPath}/proof.bin`
      let pisPath = `${circuitPath}/pis.json`
      combinedVKeyHash = (await quantum.registerGnarkPlonkCircuit(vKeyPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitGnarkPlonkProof(proofPath, pisPath, combinedVKeyHash));
    } else if (scheme == "halo2_kzg") {
      let sg2Path = `${circuitPath}/sg2.json`
      let protocolPath = `${circuitPath}/protocol.json`
      let proofPath = `${circuitPath}/proof.bin`
      let instancesPath = `${circuitPath}/instances.json`
      combinedVKeyHash = (await quantum.registerHalo2KZGCircuit(sg2Path, protocolPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitHalo2KZGProof(proofPath, instancesPath, combinedVKeyHash));
    } else if (scheme == "halo2_kzg_evm") {
      let sg2Path = `${circuitPath}/sg2.json`
      let protocolPath = `${circuitPath}/protocol.json`
      let proofPath = `${circuitPath}/proof.bin`
      let instancesPath = `${circuitPath}/instances.json`
      combinedVKeyHash = (await quantum.registerHalo2KZGEvmCircuit(sg2Path, protocolPath)).circuitHash["hash"];
      proofResponse = (await quantum.submitHalo2KZGEvmProof(proofPath, instancesPath, combinedVKeyHash));
    } else if (scheme == "nitro_attestation") {
      let pcr0Path = `${circuitPath}/pcr0.bin`
      let attDocPath = `${circuitPath}/attestation_doc.bin`
      combinedVKeyHash = (await quantum.registerNitroAttCircuit(pcr0Path)).circuitHash["hash"];
      proofResponse = (await quantum.submitNitroAttProof(attDocPath, combinedVKeyHash));
    }
  } catch (e) {
    console.log("error:", e)
    if (combinedVKeyHash == undefined) console.log("Circuit Registration failed!")
    if (proofResponse == undefined) console.log("Proof Submission failed!")
  } finally {
    console.log("combinedVKeyHash", combinedVKeyHash)
    console.log("proofResponse", proofResponse)
  }
}

if (require.main == module) {
  main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
  });
}