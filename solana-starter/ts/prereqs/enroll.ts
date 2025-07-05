import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, Turbin3Prereq } from "../programs/Turbin3_prereq";
import wallet from "../Turbin3-wallet.json";

const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");

// Import keypair from wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), { 
    commitment: "confirmed"
});

// Create program
const program: Program<Turbin3Prereq> = new Program(IDL, provider);

// Create the PDA for nrollment account
const account_seeds = [
    Buffer.from("prereqs"),
    keypair.publicKey.toBuffer(),
];
const [account_key, _account_bump] = PublicKey.findProgramAddressSync(account_seeds, program.programId);

// Declare the address of the mint Collection
const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");

// Create the mint Account for the new asset
const mintTs = Keypair.generate();

// Create the PDA for collection authority (le compte manquant!)
const collection_seeds = [
    Buffer.from("collection"),
    mintCollection.toBuffer(),
];
const [collection_authority, _collection_bump] = PublicKey.findProgramAddressSync(collection_seeds, program.programId);

// Execute the submit_ts transaction
(async () => {
    try {
        console.log("Submitting TS prerequisites...");
        const txhash = await program.methods
            .submitTs() // ✅ Nom exact de l'IDL
            .accountsPartial({
                user: keypair.publicKey,
                account: account_key,
                mint: mintTs.publicKey,
                collection: mintCollection,
                authority: collection_authority, // Le compte manquant
                mpl_core_program: MPL_CORE_PROGRAM_ID,
                system_program: SystemProgram.programId,
            })
            .signers([keypair, mintTs])
            .rpc();
        console.log(`Success Check TX https://explorer.solana.com/tx/${txhash}?cluster=devnet`);

    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();