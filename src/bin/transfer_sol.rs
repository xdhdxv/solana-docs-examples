use solana_cli_config::{CONFIG_FILE, Config};

use solana_client::rpc_client::RpcClient;

use solana_sdk::{
    signature::{Signer, keypair},
    pubkey::Pubkey, 
    native_token,
    system_instruction,
    transaction::Transaction
};

fn main() {
    let config_file = CONFIG_FILE.as_ref().unwrap();
    let config = Config::load(config_file).unwrap();

    let client = RpcClient::new(config.json_rpc_url);

    let sender = keypair::read_keypair_file(config.keypair_path).unwrap();

    let receiver = Pubkey::new_unique();

    let sol_amount = 0.01;

    let transfer_ix = system_instruction::transfer(
        &sender.pubkey(), 
        &receiver, 
        native_token::sol_to_lamports(sol_amount)
    ); 

    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix], 
        Some(&sender.pubkey()), 
        &[&sender], 
        client.get_latest_blockhash().unwrap()
    );

    let tx_signature = client.send_and_confirm_transaction_with_spinner(&tx).unwrap();

    println!("tx signature: {}", tx_signature);
}