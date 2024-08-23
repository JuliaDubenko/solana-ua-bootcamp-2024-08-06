use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::{instruction::mint_to, id as token_program_id};
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
    let recipient = Pubkey::from_str("BkaXBj1YqCtitU53tJXD6ihUsxYZZN2mj1Bb3gwa2475").expect("Invalid recipient account");

    let recipient_associated_token_account = spl_associated_token_account::get_associated_token_address(&recipient, &token_mint_account);

    let create_associated_token_account_ix = create_associated_token_account(
        &sender.pubkey(),
        &recipient,
        &token_mint_account,
        &token_program_id()
    );

    let mint_instruction = mint_to(
        &token_program_id(),                             // ID програми токенів SPL
        &token_mint_account,                             // Публічний ключ облікового запису токена
        &recipient_associated_token_account,             // Публічний ключ облікового запису отримувача
        &sender.pubkey(),                                // Публічний ключ власника токенів
        &[],                                             // Масив додаткових підписувачів (може бути порожнім)
        1000                                             
    ).expect("Failed to create mint_to instruction");

    let recent_blockhash = connection.get_latest_blockhash().expect("Failed to get latest blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[create_associated_token_account_ix, mint_instruction],
        Some(&sender.pubkey()),
        &[&sender],
        recent_blockhash,
    );

    connection.send_and_confirm_transaction_with_spinner(&transaction).expect("Failed to mint tokens");

    println!("✅ Minted tokens successfully!");
}
