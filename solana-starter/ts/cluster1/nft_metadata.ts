import wallet from "../Turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://devnet.irys.xyz/'); // Irys devent car Solana devnet KO
let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // URL image to upload
        const imageUri = "https://gateway.irys.xyz/4KXx1xGuDVjmHcifXpzkKeAM9YNZsstv1ve5M5WCmwfk";

        const metadata = {
            name: "Nice Rug",
            symbol: "Rug",
            description: "A nice rug",
            image: imageUri,
            attributes: [
                { trait_type: 'swag', value: 'max swag' }       //trait
            ],
            properties: {
                files: [
                    {
                        type: "image/jpeg", //
                        uri: imageUri
                    },
                ]
            },
            creators: []
        };


        // Convert metadata to a generic file
        const metadataBuffer = Buffer.from(JSON.stringify(metadata));
        const metadataFile = createGenericFile(metadataBuffer, "metadata.json", {contentType: "application/json"});
        const [myUri] = await umi.uploader.upload([metadataFile]);
        console.log("Your metadata URI:", myUri);
    } catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
