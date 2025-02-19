use byteorder::{LittleEndian, WriteBytesExt};

use solana_cli_config::{CONFIG_FILE, Config};

use solana_client::rpc_client::RpcClient;

use solana_sdk::{
    signature::{Signer, keypair}, 
    pubkey::Pubkey, 
    native_token,
    system_program, 
    message::{Message, MessageHeader},
    instruction::CompiledInstruction, 
    transaction::Transaction
};


const TRANSFER_INSTRUCTION_DISCRIMINATOR: u32 = 2;

fn main() {
    let config_file = CONFIG_FILE.as_ref().unwrap();
    let config = Config::load(config_file).unwrap();

    let client = RpcClient::new(config.json_rpc_url);

    let payer = keypair::read_keypair_file(config.keypair_path).unwrap();

    let receiver = Pubkey::new_unique();

    let sol_amount = 0.1;

    let mut transfer_ix_data = Vec::with_capacity(12);

    transfer_ix_data.write_u32::<LittleEndian>(TRANSFER_INSTRUCTION_DISCRIMINATOR).unwrap();
    transfer_ix_data.write_u64::<LittleEndian>(native_token::sol_to_lamports(sol_amount)).unwrap();

    let tx_message = Message {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts:1,
        },
        account_keys: vec![payer.pubkey(), receiver, system_program::ID],
        recent_blockhash: client.get_latest_blockhash().unwrap(),
        instructions: vec![
            CompiledInstruction {
                program_id_index: 2,
                accounts: vec![0, 1],
                data: transfer_ix_data
            }
        ]
    };

    let payer_signature = payer.sign_message(&tx_message.serialize());

    let transfer_tx = Transaction {
        signatures: vec![payer_signature],
        message: tx_message,
    };

    let tx_signature = client.send_and_confirm_transaction_with_spinner(&transfer_tx).unwrap();

    println!("tx signature: {}", tx_signature);
}