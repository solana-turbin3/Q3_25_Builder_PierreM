// plusieurs pb avec creators, l'instruction attendait un tableau non null
import wallet from "../Turbin3-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
    createMetadataAccountV3,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

// Adresse du mint SPL
const mint = publicKey("5RgxDvGrhQvtYaonNNzu7zqRmrhkqtPgNUdRhvH7qagH");

// Connexion à la Devnet et création du signer
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Définition des métadonnées du token
        let data: DataV2Args = {
            name: "RUGGED",
            symbol: "RUG",
            uri: "https://raw.githubusercontent.com/Solenjoyer/NFT-host/main/rug-metaplex.json",
            sellerFeeBasisPoints: 0,
            creators: [         //pb avec creators
                {
                    address: keypair.publicKey,
                    verified: true,
                    share: 100
                }
            ],
            collection: null,
            uses: null
        };

        let tx = createMetadataAccountV3(
            umi,
            {
                mint: mint,
                mintAuthority: signer,
                updateAuthority: signer,
                payer: signer,
                data: data,
                isMutable: true,
                collectionDetails: null         //cf Solana docs Accounts
            }
        );

        let result = await tx.sendAndConfirm(umi);
        console.log("Signature tx:", bs58.encode(result.signature));
    } catch (e) {
        console.error("Oops, something went wrong:\n", e);
    }
})();
