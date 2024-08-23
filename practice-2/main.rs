use dotenv::dotenv;
use std::env;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    system_instruction,
    signature::{Keypair, Signer},
    transaction::Transaction as SolanaTransaction,
    pubkey::Pubkey,
    program_pack::Pack,
};
use spl_token::{
    instruction::initialize_mint,
    state::Mint,
};

fn main() {
    dotenv().ok();
    
    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");

    let as_vec: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format");
    let sender = Keypair::from_bytes(&as_vec).expect("Failed to create keypair");

    let rpc_url = "https://api.devnet.solana.com";
    let connection = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    println!("🔑 Our public key is: {}", sender.pubkey());

    let token_mint = create_token_mint(&connection, &sender).expect("Failed to create token mint");

    println!("✅ Token Mint: https://explorer.solana.com/address/{}?cluster=devnet", token_mint);
}

fn create_token_mint(
    connection: &RpcClient,
    sender: &Keypair,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let mint_rent_exempt = connection.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    let mint_account = Keypair::new();
    let transaction = SolanaTransaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &sender.pubkey(),
                &mint_account.pubkey(),
                mint_rent_exempt,
                Mint::LEN as u64,
                &spl_token::id(),
            ),
            initialize_mint(
                &spl_token::id(),
                &mint_account.pubkey(),
                &sender.pubkey(),
                None,
                2,
            )?,
        ],
        Some(&sender.pubkey()),
        &[sender, &mint_account],
        connection.get_latest_blockhash()?,
    );

    connection.send_and_confirm_transaction_with_spinner(&transaction)?;
    Ok(mint_account.pubkey())
}