use std::env;
use std::str::FromStr;
use serde_json;
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    pubkey::Pubkey,
    signer::keypair::Keypair,
    system_instruction,
    transaction::Transaction,
    signature::Signer,
};
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcSendTransactionConfig,
};

fn main() {
    dotenv::dotenv().ok(); 
    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let secret_key_vec: Vec<u8> = serde_json::from_str(&private_key).expect("Failed to parse SECRET_KEY as JSON!");
    let sender = Keypair::from_bytes(&secret_key_vec).expect("Invalid secret key!");

    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    println!("🔑 Our public key is: {}", sender.pubkey());

    let recipient = Pubkey::from_str("KstbtPJMBASgrjFQ9stPHfeQToYNwf4SrNZkMj1PmqL").unwrap();
    println!("🤑 Trying to send a transfer of 0.01 SOL to {}...", recipient);

    let lamports = (0.01 * solana_sdk::native_token::LAMPORTS_PER_SOL as f64) as u64;

    let send_sol_instruction = system_instruction::transfer(&sender.pubkey(), &recipient, lamports);

    let transaction = Transaction::new_signed_with_payer(
        &[send_sol_instruction],
        Some(&sender.pubkey()),
        &[&sender],
        connection.get_latest_blockhash().unwrap(),
    );

    match connection.send_and_confirm_transaction_with_spinner_and_config(
        &transaction,
        CommitmentConfig::confirmed(), // Використання CommitmentConfig::confirmed()
        RpcSendTransactionConfig {
            skip_preflight: false,
            preflight_commitment: Some(CommitmentLevel::Confirmed), // Використання CommitmentLevel замість CommitmentConfig
            ..RpcSendTransactionConfig::default()
        },
    ) {
        Ok(signature) => println!("🚀 The transfer was made successfully, signature: {}!", signature),
        Err(e) => println!("Failed to send transaction: {:?}", e),
    }

    // Додавання мемо
    let memo_program = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr").unwrap();
    let memo_text = "Hello! I show up at solana-training!";
    
    let add_memo_instruction = solana_sdk::instruction::Instruction::new_with_bytes(
        memo_program,
        memo_text.as_bytes(),
        vec![],
    );

    let memo_transaction = Transaction::new_signed_with_payer(
        &[add_memo_instruction],
        Some(&sender.pubkey()),
        &[&sender],
        connection.get_latest_blockhash().unwrap(),
    );

    match connection.send_and_confirm_transaction_with_spinner(&memo_transaction) {
        Ok(_) => println!("📝 memo is: {}", memo_text),
        Err(e) => println!("Failed to send memo transaction: {:?}", e),
    }
}
