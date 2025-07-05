import { Transaction, SystemProgram, Connection, Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from "@solana/web3.js";
import wallet from "./dev-wallet.json";

// importe la keypair de ton wallet dev
const from = Keypair.fromSecretKey(new Uint8Array(wallet));

// Turbin3
const to = new PublicKey("J3rmS5HFCSfB482ZiTweN5CmPSBDfwop9NAzA6gTKVi4");

// Connexion devnet
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
  try {
    // 0.1 SOL = LAMPORTS_PER_SOL / 10
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: LAMPORTS_PER_SOL / 10, // 0.1 SOL
      })
    );
    transaction.recentBlockhash = (
      await connection.getLatestBlockhash('confirmed')
    ).blockhash;
    transaction.feePayer = from.publicKey;

    // Signature, envoi et confirmation
    const signature = await sendAndConfirmTransaction(
      connection,
      transaction,
      [from]
    );
    console.log(`Success TX : https://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();