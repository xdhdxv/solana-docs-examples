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

    let mint = Pubkey::from_str_const("mint account address");

    let ata = spl_associated_token_account::get_associated_token_address_with_program_id(
        &payer.pubkey(), 
        &mint, 
        &spl_token_2022::ID
    );

    let mint_ix = spl_token_2022::instruction::mint_to(
    &spl_token_2022::ID, 
        &mint, 
        &ata, 
        &payer.pubkey(), 
        &[], 
        10000
    ).unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[mint_ix], 
        Some(&payer.pubkey()), 
        &[&payer], 
        client.get_latest_blockhash().unwrap()
    );

    let tx_signature = client.send_and_confirm_transaction_with_spinner(&tx).unwrap();

    println!("tx signature: {}", tx_signature);
}