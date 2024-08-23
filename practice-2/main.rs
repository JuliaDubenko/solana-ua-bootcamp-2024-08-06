use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::instruction::{create_associated_token_account};
use spl_token::id as token_program_id;
use std::env;
use std::str::FromStr;

fn main() {
    dotenv().ok();

    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let as_vec: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format");
    let sender = Keypair::from_bytes(&as_vec).expect("Failed to create keypair");

    let rpc_url = "https://api.devnet.solana.com";
    let connection = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    println!("🔑 Our public key is: {}", sender.pubkey());

    let token_mint_account = Pubkey::from_str("FdUzKJvs5dRXzXuUXZenRJK3HDtqawHWspvx6ybKzFPA").expect("Invalid mint account");
    let recipient = Pubkey::from_str("ECq56tKxckgqep9ioKKeyazowNUU4Uw4bPEC4cJGzt1F").expect("Invalid recipient account");

    let token_account_address = spl_associated_token_account::get_associated_token_address(&recipient, &token_mint_account);
    let create_associated_token_account_ix = create_associated_token_account(
        &sender.pubkey(),         // Публічний ключ платника
        &recipient,               // Публічний ключ отримувача
        &token_mint_account,      // Публічний ключ токен-міта
        &token_program_id()       // Публічний ключ програми токенів
    );

    let recent_blockhash = connection.get_latest_blockhash().expect("Failed to get latest blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[create_associated_token_account_ix],
        Some(&sender.pubkey()),     // Публічний ключ платника
        &[&sender],
        recent_blockhash,
    );

    connection.send_and_confirm_transaction_with_spinner(&transaction).expect("Failed to create token account");

    println!("Token Account: {}", token_account_address);

    let link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        token_account_address
    );

    println!("✅ Created token account: {}", link);
}
