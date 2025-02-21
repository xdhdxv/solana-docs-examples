use solana_cli_config::{CONFIG_FILE, Config};

use solana_client::rpc_client::RpcClient;

use solana_sdk::{
    signature::{Signer, keypair},
    pubkey::Pubkey,
    transaction::Transaction,
};

fn main() {
    let config_file = CONFIG_FILE.as_ref().unwrap();
    let config = Config::load(config_file).unwrap();

    let usdc_mint_address = Pubkey::from_str_const("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr");

    let client = RpcClient::new(config.json_rpc_url);

    let sender = keypair::read_keypair_file(config.keypair_path).unwrap();

    let receiver = Pubkey::new_unique();

    let create_receiver_ata_ix = spl_associated_token_account::instruction::create_associated_token_account_idempotent(
        &sender.pubkey(), 
        &receiver, 
        &usdc_mint_address, 
        &spl_token::ID
    ); 

    let transfer_ix = spl_token::instruction::transfer(
        &spl_token::ID, 
        &spl_associated_token_account::get_associated_token_address(&sender.pubkey(), &usdc_mint_address), 
        &spl_associated_token_account::get_associated_token_address(&receiver, &usdc_mint_address), 
        &sender.pubkey(), 
        &[], 
        100000
    ).unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[create_receiver_ata_ix, transfer_ix], 
        Some(&sender.pubkey()), 
        &[&sender], 
        client.get_latest_blockhash().unwrap()
    );

    let tx_signature = client.send_and_confirm_transaction_with_spinner(&tx).unwrap();

    println!("tx signature: {}", tx_signature);
}