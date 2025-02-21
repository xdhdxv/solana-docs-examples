use solana_cli_config::{CONFIG_FILE, Config};

use solana_client::rpc_client::RpcClient;

use solana_sdk::{
    signature::{Signer, keypair},
    pubkey::Pubkey,
    transaction::Transaction
};

fn main() {
    let config_file = CONFIG_FILE.as_ref().unwrap();
    let config = Config::load(config_file).unwrap();

    let client = RpcClient::new(config.json_rpc_url);

    let payer = keypair::read_keypair_file(config.keypair_path).unwrap();

    let usdc_devnet_mint = Pubkey::from_str_const("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr");

    let create_ata_instruction = spl_associated_token_account::instruction::create_associated_token_account(
        &payer.pubkey(), 
        &payer.pubkey(), 
        &usdc_devnet_mint, 
        &spl_token::ID
    );

    let tx: Transaction = Transaction::new_signed_with_payer(
        &[create_ata_instruction], 
        Some(&payer.pubkey()), 
        &[&payer], 
        client.get_latest_blockhash().unwrap()
    );

    let tx_signature = client.send_and_confirm_transaction_with_spinner(&tx).unwrap();

    println!("tx signature: {}", tx_signature);
}