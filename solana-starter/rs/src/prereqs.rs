#[cfg(test)]
mod tests {
    use solana_sdk;

    #[test]
    fn keygen() {
        use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};
        
        // Create a new keypair
        let kp = Keypair::new();
        
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        use solana_client::rpc_client::RpcClient;
        use solana_sdk::{
            signature::{Keypair, Signer, read_keypair_file},
        };

        const RPC_URL: &str = "https://api.devnet.solana.com";

        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // Establish connection to Solana devnet
        let client = RpcClient::new(RPC_URL);

        // Request 2 devnet SOL tokens (2 b lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }

    #[test]
    fn transfer_sol() {
        use solana_client::rpc_client::RpcClient;
        use solana_program::{pubkey::Pubkey, system_instruction::transfer};
        use solana_sdk::{
            signature::{Keypair, Signer, read_keypair_file},
            transaction::Transaction,
            message::Message,
        };
        use std::str::FromStr;

        const RPC_URL: &str = "https://api.devnet.solana.com";

        // Load your devnet keypair from file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");


        // Generate a signature from the keypair (verification step)
        let pubkey = keypair.pubkey();
        println!("Your public key: {}", pubkey);

        // Define the destination (Turbin3) address
        let to_pubkey = Pubkey::from_str("J3rmS5HFCSfB482ZiTweN5CmPSBDfwop9NAzA6gTKVi4").unwrap();

        // Connect to devnet
        let rpc_client = RpcClient::new(RPC_URL);

        // Fetch recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // First transfer: 0.1 SOL
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)], // 0.1 SOL
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

        #[test]
    fn empty_wallet() {
        use solana_client::rpc_client::RpcClient;
        use solana_program::{pubkey::Pubkey, system_instruction::transfer};
        use solana_sdk::{
            message::Message,
            signature::{Keypair, Signer, read_keypair_file},
            transaction::Transaction,
        };
        use std::str::FromStr;

        const RPC_URL: &str = "https://api.devnet.solana.com";

        // Load your devnet keypair from file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // Define the destination (Turbin3) address
        let to_pubkey = Pubkey::from_str("J3rmS5HFCSfB482ZiTweN5CmPSBDfwop9NAzA6gTKVi4").unwrap();

        // Connect to devnet
        let rpc_client = RpcClient::new(RPC_URL);

        // Get current balance
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        println!("Current balance: {} lamports", balance);

        // Fetch recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Build a mock transaction to calculate fee
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        // Estimate transaction fee
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        println!("Transaction fee: {} lamports", fee);
        println!("Transferring: {} lamports", balance - fee);

        // Create final transaction with balance minus fee
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send transaction and verify
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send final transaction");

        println!(
            "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn submit() {
        use solana_client::rpc_client::RpcClient;
        use solana_program::{pubkey::Pubkey, system_program};
        use solana_sdk::{
            instruction::{AccountMeta, Instruction},
            signature::{Keypair, Signer, read_keypair_file},
            transaction::Transaction,
        };
        use std::str::FromStr;

        const RPC_URL: &str = "https://api.devnet.solana.com";

        // Utiliser le portefeuille Turbin3 (pas le dev wallet vide)
        let signer = read_keypair_file("turbin3-wallet.json").expect("Couldn't find wallet file");

        // Define program and account public keys
        let mint = Keypair::new();
        let turbin3_prereq_program = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let system_program = system_program::id();

        // Get the PDA (Program Derived Address)
        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);

        // Authority PDA avec les bonnes seeds selon l'IDL
        let authority_seeds = &[b"collection", collection.as_ref()];
        let (authority, _auth_bump) = Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);

        // RPC client
        let rpc_client = RpcClient::new(RPC_URL);

        // Get recent blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        //Ajouter le github handle dans les données
        let github_handle = "solenjoyer"; // github handle
        let mut data = vec![77, 124, 82, 163, 21, 133, 181, 206]; // Discriminator
        
        //github handle (longueur + string)
        let github_bytes = github_handle.as_bytes();
        data.extend_from_slice(&(github_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(github_bytes);

        // 7 comptes selon l'IDL
        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true),            // user signer
            AccountMeta::new(prereq_pda, false),                // PDA account
            AccountMeta::new(mint.pubkey(), true),              // mint keypair
            AccountMeta::new(collection, false),                // collection (writable)
            AccountMeta::new_readonly(authority, false),        // authority (PDA)
            AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
            AccountMeta::new_readonly(system_program, false),   // system program
        ];

        // Build the instruction
        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };

        // Create and sign the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            blockhash,
        );

        // Send and confirm the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }





}
