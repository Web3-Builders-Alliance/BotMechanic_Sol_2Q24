import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js'
// import * as prompt from 'prompt-sync'
import wallet from './dev-wallet.json'

console.log(wallet)

// let kp = Keypair.generate()

// console.log(`You've generated a new Solana wallet:
// ${kp.publicKey.toBase58()}`)

// console.log(`Private Key: [${kp.secretKey}]`)

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet.wallet));
const connection = new Connection('https://api.devnet.solana.com');

(async () => {
  try {
      // We're going to claim 2 devnet SOL tokens
      const txhash = await
connection.requestAirdrop(keypair.publicKey, 2 * LAMPORTS_PER_SOL);
      console.log(`Success! Check out your TX here:
      https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch(e) {
      console.error(`Oops, something went wrong: ${e}`)
  }
})()