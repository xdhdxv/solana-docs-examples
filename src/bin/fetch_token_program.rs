use solana_cli_config::{CONFIG_FILE, Config};

use solana_client::rpc_client::RpcClient;

use solana_sdk::pubkey::Pubkey;

fn main() {
    let config_file = CONFIG_FILE.as_ref().unwrap();
    let config = Config::load(config_file).unwrap();

    let client = RpcClient::new(config.json_rpc_url);

    let address = Pubkey::from_str_const("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

    let account_info = client.get_account(&address).unwrap();

    println!("{:#?}", account_info);
}