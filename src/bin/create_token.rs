use solana_cli_config::{CONFIG_FILE, Config};

use solana_client::rpc_client::RpcClient;

use solana_sdk::{
    signature::{Signer, Keypair, keypair},
    program_pack::Pack,
    system_instruction, 
    transaction::Transaction
};

use spl_token_2022::state::Mint;

fn main() {
    let config_file = CONFIG_FILE.as_ref().unwrap();
    let config = Config::load(config_file).unwrap();

    let client = RpcClient::new(config.json_rpc_url);

    let payer = keypair::read_keypair_file(config.keypair_path).unwrap();

    // generate keypair to use as address of mint account
    let mint = Keypair::new();

    // calculate minimum lamports for space required by mint account
    let rent_lamports = client.get_minimum_balance_for_rent_exemption(Mint::LEN).unwrap();

    // instruction to create new account with space for new mint account
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(), 
        &mint.pubkey(), 
        rent_lamports, 
        Mint::LEN as u64, 
        &spl_token_2022::ID
    );

    // instruction to initialize mint account
    let initialize_mint_ix = spl_token_2022::instruction::initialize_mint2(
        &spl_token_2022::ID, 
        &mint.pubkey(), 
        &payer.pubkey(), 
        Some(&payer.pubkey()), 
        2
    ).unwrap();

    // build transaction with instructions to create new account and initialize mint account
    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix, initialize_mint_ix], 
        Some(&payer.pubkey()), 
        &[&payer, &mint], 
        client.get_latest_blockhash().unwrap()
    );

    let tx_signature = client.send_and_confirm_transaction(&tx).unwrap();

    println!("tx signature: {}",tx_signature);

    println!("mint account: {}", mint.pubkey());
}