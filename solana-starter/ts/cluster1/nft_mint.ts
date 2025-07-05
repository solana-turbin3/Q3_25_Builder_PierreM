import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import wallet from "../Turbin3-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://devnet.helius-rpc.com/?api-key=be2389bb-14d3-4276-a7cf-5892fdb19b28"; // RPC pris avec HELIUS car devnet solana KO
//fait avec mes identifiants



const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

//le lien obtenu dans nft_metadata.ts
const metadataUri = "https://gateway.irys.xyz/7XfVbfCxDqmeS1sisqSisaxxsn3txt93zjGn3zV5CE7u"; //

(async () => {
    try {
        // Mint le NFT
        const tx = await createNft(umi, {
            mint,
            authority: myKeypairSigner,
            name: "Nice Rug",
            symbol: "Rug",
            uri: metadataUri,
            sellerFeeBasisPoints: percentAmount(5), // 5% royalties
        });

        const result = await tx.sendAndConfirm(umi);
        const signature = base58.encode(result.signature);

        console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);

        console.log("Mint Address: ", mint.publicKey);
    } catch (e) {
        console.error("Oops.. Something went wrong", e);
    }
})();
