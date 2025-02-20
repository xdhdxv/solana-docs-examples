use solana_cli_config::{CONFIG_FILE, Config};

use solana_client::rpc_client::RpcClient;

use solana_sdk::{
    signature::{Signer, keypair},
    pubkey::Pubkey,
    compute_budget::ComputeBudgetInstruction,
    system_instruction,
    native_token,
    transaction::Transaction
};

fn main() {
    let config_file = CONFIG_FILE.as_ref().unwrap();
    let config = Config::load(config_file).unwrap();

    let client = RpcClient::new(config.json_rpc_url);

    let payer = keypair::read_keypair_file(config.keypair_path).unwrap();

    let receiver = Pubkey::new_unique();

    let set_cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(450);

    let set_cu_price_ix = ComputeBudgetInstruction::set_compute_unit_price(100000);

    let transfer_ix = system_instruction::transfer(
        &payer.pubkey(), 
        &receiver, 
        native_token::sol_to_lamports(0.05)
    );

    let transfer_tx = Transaction::new_signed_with_payer(
        &[set_cu_limit_ix, set_cu_price_ix, transfer_ix], 
        Some(&payer.pubkey()), 
        &[&payer], 
        client.get_latest_blockhash().unwrap()
    );

    let tx_signature = client.send_and_confirm_transaction_with_spinner(&transfer_tx).unwrap();

    println!("tx signature: {}", tx_signature);
}