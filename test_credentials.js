require('dotenv').config();
const { Quantum } = require("quantum-sdk");

async function testCredentials() {
  const hint = "inside your .env file, make sure that RPC_ENDPOINT and ACCESS_KEY are correct."
  try {
    const quantum = new Quantum(process.env.RPC_ENDPOINT, process.env.ACCESS_KEY);
    const rpcLive = await quantum.checkServerConnection();
    if (!rpcLive) {
      console.error(`Failed to connect to Quantum server:\n ${hint}`);
      process.exit(1);
    }
    console.log('API credentials verified successfully!');
  } catch (error) {
    console.error(`Invalid API credentials:\n ${hint}`);
    process.exit(1);
  }
}

testCredentials(); 