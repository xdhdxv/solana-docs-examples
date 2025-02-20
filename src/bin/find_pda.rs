use solana_sdk::pubkey::Pubkey;

fn main() {
    let program_id = Pubkey::from_str_const("11111111111111111111111111111111");
    let string = "helloWorld";

    let (pda, bump) = Pubkey::find_program_address(
        &[string.as_bytes()], 
        &program_id
    );

    println!("pda: {}", pda);
    println!("bump: {}", bump);
}