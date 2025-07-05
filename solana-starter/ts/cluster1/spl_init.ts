import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from '@solana/spl-token';
import wallet from "../Turbin3-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const publicKey = keypair.publicKey;

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
    try {
        // Start here
        const mint = await createMint(connection, keypair, publicKey, null, 6);
        console.log(`Mint Address: ${mint.toBase58()}`)
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()