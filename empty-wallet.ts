import {
  Transaction,
  SystemProgram,
  Connection,
  Keypair,
  sendAndConfirmTransaction,
  PublicKey
} from "@solana/web3.js";
import wallet from "./dev-wallet.json";

// Ton wallet devnet
const from = Keypair.fromSecretKey(new Uint8Array(wallet));

// Ton adresse Turbin3 (remplace bien ici si besoin)
const to = new PublicKey("J3rmS5HFCSfB482ZiTweN5CmPSBDfwop9NAzA6gTKVi4");

// Connexion devnet
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
  try {
    //Récupère le solde
    const balance = await connection.getBalance(from.publicKey);

    //Crée une transaction fictive pour calculer les frais
    const dummyTx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance, // On fait semblant d'envoyer tout
      })
    );
    dummyTx.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
    dummyTx.feePayer = from.publicKey;

    //Calcule les frais exacts
    const fee = (await connection.getFeeForMessage(dummyTx.compileMessage(), 'confirmed')).value || 0;

    //Supprime l'ancienne instruction
    dummyTx.instructions.pop();

    //Ajoute la vraie instruction avec le bon montant (balance - fee)
    dummyTx.add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance - fee, // ⚠️ Correction ici !
      })
    );

    //Envoie la transaction
    const signature = await sendAndConfirmTransaction(
      connection,
      dummyTx,
      [from]
    );
    console.log(`Success TX : https://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
