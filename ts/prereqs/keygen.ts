import { Keypair } from "@solana/web3.js";

// Génère une nouvelle keypair (paire de clés)
const kp = Keypair.generate();

// Affiche la clé publique (adresse Solana)
console.log(`nouveau wallet Solana :\n${kp.publicKey.toBase58()}`);

// Affiche la clé privée (à sauvegarder dans un fichier !)
console.log('\ncollez la clé privée dans unJSON :');
console.log(`[${kp.secretKey}]`);