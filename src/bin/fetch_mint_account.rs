use solana_cli_config::{CONFIG_FILE, Config};

use solana_client::rpc_client::RpcClient;

use solana_sdk::pubkey::Pubkey;

use spl_token_2022::{
    state::Mint,
    extension::StateWithExtensions
};

fn main() {
    let config_file = CONFIG_FILE.as_ref().unwrap();
    let config = Config::load(config_file).unwrap();

    let client = RpcClient::new(config.json_rpc_url);

    let mint_address = Pubkey::from_str_const("C33qt1dZGZSsqTrHdtLKXPZNoxs6U1ZBfyDkzmj6mXeR");

    let account_info = client.get_account(&mint_address).unwrap();

    let mint_data = StateWithExtensions::<Mint>::unpack(&account_info.data).unwrap().base;

    println!("{:#?}", mint_data);
}