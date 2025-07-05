import wallet from "../Turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://devnet.irys.xyz/');
let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // 1. Load image
        const imageBuffer = await readFile("/home/monti/images Turbin3/rug.jpg");
        // 2. Convert image to generic file
        const image = createGenericFile(imageBuffer, "abims.jpeg", {contentType: "image/jpeg"});
        // 3. Upload image
        const [myUri] = await umi.uploader.upload([image]);      // ici await et passer un tableau

        console.log("Your image URI:", myUri);
        
    } catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
