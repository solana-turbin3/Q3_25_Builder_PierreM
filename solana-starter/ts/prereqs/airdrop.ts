import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import wallet from "./dev-wallet.json";

// reconstitue la keypair à partir du fichier JSON
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//connecte au réseau devnet de Solana
const connection = new Connection("https://api.devnet.solana.com");

//demande 2 SOL de test (2 * LAMPORTS_PER_SOL)
(async () => {
  try {
    const txhash = await connection.requestAirdrop(
      keypair.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    console.log(`Success TX : https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch(e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();