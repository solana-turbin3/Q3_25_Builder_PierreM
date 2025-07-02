//adaptation et peaufinement d'une ancienne version

import bs58 from 'bs58';
import promptSync from 'prompt-sync';

// Initialisation de prompt-sync pour lire l'entrée utilisateur dans le terminal
const prompt = promptSync();

// Fonction 1 : Convertir une clé privée Phantom (base58) en tableau (pour dev-wallet.json)
function base58ToWallet() {
  const base58 = prompt('Colle ici ta clé privée Phantom (base58) : ');
  const wallet = bs58.decode(base58);
  console.log('Voici le tableau à mettre dans dev-wallet.json :');
  console.log(JSON.stringify(Array.from(wallet)));
}

// Fonction 2 : Convertir un tableau (dev-wallet.json) en base58 (pour Phantom)
function walletToBase58() {
  const input = prompt('Colle ici le tableau de ta clé privée (ex : [12,34,56,...]) : ');
  try {
    const arr = JSON.parse(input);
    const base58 = bs58.encode(Uint8Array.from(arr));
    console.log('Voici la clé privée base58 à importer dans Phantom :');
    console.log(base58);
  } catch (e) {
    console.error('Erreur de format : vérifie que tu as bien collé un tableau JSON valide.');
  }
}

// Menu pour choisir la conversion
console.log('1. Convertir base58 → tableau (pour dev-wallet.json)');
console.log('2. Convertir tableau → base58 (pour Phantom)');
const choix = prompt('Tape 1 ou 2 selon ce que tu veux faire : ');

if (choix === '1') base58ToWallet();
else if (choix === '2') walletToBase58();
else console.log('Choix non reconnu.');
