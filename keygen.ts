import { Keypair } from "@solana/web3.js";

// Génère une nouvelle keypair (paire de clés)
const kp = Keypair.generate();

// Affiche la clé publique (adresse Solana)
console.log(`Vous avez généré un nouveau wallet Solana :\n${kp.publicKey.toBase58()}`);

// Affiche la clé privée (à sauvegarder dans un fichier !)
console.log('\nPour sauvegarder ce wallet, copiez-collez la clé privée dans un fichier JSON :');
console.log(`[${kp.secretKey}]`);
