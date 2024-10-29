const { Quantum } = require("quantum-sdk")
const { ProofType } = require("quantum-sdk/dist/src/enum/proof_type")
const dotenv = require('dotenv')
const args = require('yargs').argv;

dotenv.config()


const RPC_ENDPOINT = process.env.RPC_ENDPOINT
const ACCESS_KEY = process.env.ACCESS_KEY
const SCHEMES = { "gnark_groth16": ProofType.GNARK_GROTH16, "snarkjs_groth16": ProofType.GROTH16 }

const main = async () => {
  const quantum = new Quantum(RPC_ENDPOINT, ACCESS_KEY);
  let rpcLive = await quantum.checkServerConnection();
  if (!rpcLive) {
    throw new Error(`${rpcLive}`)
  }

  const scheme = args.scheme

  if (!(scheme in SCHEMES)) {
    throw new Error(`invalid ${scheme} provided!`)
  }

  var circuitPath = `circuits/${scheme}/circuit_data`

  let circuitHash, proofResponse
  if (scheme == "gnark_groth16") {
    let vKeyPath = `${circuitPath}/vKey.bin`
    let proofPath = `${circuitPath}/proof.bin`
    let pisPath = `${circuitPath}/pis.json`
    console.log("circuitPath", circuitPath)
    circuitHash = (await quantum.registerGnarkGroth16Circuit(vKeyPath)).circuitHash["hash"];
    proofResponse = (await quantum.submitGnarkGroth16Proof(proofPath, pisPath, circuitHash));
  } else if (scheme == "snarkjs_groth16") {
    let vKeyPath = `${circuitPath}/verification_key.json`
    let proofPath = `${circuitPath}/proof.json`
    let pisPath = `${circuitPath}/public.json`
    circuitHash = (await quantum.registerSnarkJSGroth16Circuit(vKeyPath)).circuitHash["hash"];
    proofResponse = (await quantum.submitSnarkJSGroth16Proof(proofPath, pisPath, circuitHash));
  }
  console.log("circuitHash", circuitHash)
  console.log("proofResponse", proofResponse)

}

if (require.main == module) {
  main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
  });
}