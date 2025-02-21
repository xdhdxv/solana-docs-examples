use solana_sdk::pubkey::Pubkey;

fn main() {
    let wallet_address = Pubkey::new_unique();
    let usdc_mint_address = Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

    let ata_address = spl_associated_token_account::get_associated_token_address(&wallet_address, &usdc_mint_address);
    // let (ata_address, bump) = Pubkey::find_program_address(&[wallet_address.as_array(), spl_token::ID.as_array(), usdc_mint_address.as_array()], &spl_associated_token_account::ID);

    println!("ata: {}", ata_address);
}