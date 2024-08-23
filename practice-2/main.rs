use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey as SolanaPubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_token_metadata::{
    id as metadata_program_id,
    state::DataV2,
    instruction::create_metadata_accounts_v3,
};
use std::env;
use std::str::FromStr;
use anchor_lang::prelude::*;

fn main() {
    dotenv().ok();
    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let as_vec: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format");
    let sender = Keypair::from_bytes(&as_vec).expect("Failed to create keypair");
    let rpc_url = "https://api.devnet.solana.com";
    let connection = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
    println!("🔑 Our public key is: {}", sender.pubkey());
    let token_mint_account = SolanaPubkey::from_str("BkaXBj1YqCtitU53tJXD6ihUsxYZZN2mj1Bb3gwa2475").expect("Invalid mint account");
    let metadata_data = DataV2 {
        name: "Solana UA Bootcamp 2024-08-06".to_string(),
        symbol: "UAB-2".to_string(),
        uri: "https://arweave.net/1234".to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };
    let (metadata_pda, _bump) = SolanaPubkey::find_program_address(
        &[b"metadata", &metadata_program_id().to_bytes(), &token_mint_account.to_bytes()],
        &metadata_program_id(),
    );
    let create_metadata_account_ix = create_metadata_accounts_v3(
        &metadata_program_id(),
        &metadata_pda,
        &token_mint_account,
        &sender.pubkey(),
        &sender.pubkey(),
        &sender.pubkey(),
        metadata_data,
        None,
        None,
        None,
    );

    let recent_blockhash = connection.get_latest_blockhash().expect("Failed to get latest blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[create_metadata_account_ix],
        Some(&sender.pubkey()),
        &[&sender],
        recent_blockhash,
    );

    connection.send_and_confirm_transaction_with_spinner(&transaction).expect("Failed to create metadata account");

    println!("✅ Metadata account created successfully!");
}
