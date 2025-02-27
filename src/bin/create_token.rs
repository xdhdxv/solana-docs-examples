use solana_cli_config::{CONFIG_FILE, Config};

use solana_client::rpc_client::RpcClient;

use solana_sdk::{
    signature::{keypair, Keypair, Signer}, 
    system_instruction,
    program_pack::Pack,
    transaction::Transaction
};

use spl_token_2022::state::Mint;

fn main() {
    let config_file = CONFIG_FILE.as_ref().unwrap();
    let config = Config::load(config_file).unwrap();

    let client = RpcClient::new(config.json_rpc_url);

    let payer = keypair::read_keypair_file(config.keypair_path).unwrap();

    let mint = Keypair::new();

    let rent_lamports = client.get_minimum_balance_for_rent_exemption(Mint::LEN).unwrap();

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(), 
        &mint.pubkey(), 
        rent_lamports, 
        Mint::LEN as u64, 
        &spl_token_2022::ID
    );

    let initialize_mint_ix = spl_token_2022::instruction::initialize_mint2(
        &spl_token_2022::ID, 
        &mint.pubkey(), 
        &payer.pubkey(), 
        Some(&payer.pubkey()), 
        2
    ).unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix, initialize_mint_ix], 
        Some(&payer.pubkey()), 
        &[&payer, &mint], 
        client.get_latest_blockhash().unwrap()
    );

    let tx_signature = client.send_and_confirm_transaction_with_spinner(&tx).unwrap();

    println!("tx signature: {}", tx_signature);
    println!("mint account: {}", mint.pubkey());
}