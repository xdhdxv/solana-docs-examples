use solana_sdk::pubkey::Pubkey;

fn main() {
    let program_id = Pubkey::from_str_const("11111111111111111111111111111111");
    let string = "helloWorld";
    let bump = 254;

    let pda = Pubkey::create_program_address(
        &[string.as_bytes(), &[bump]],
        &program_id
    ).unwrap();

    println!("pda: {}", pda);
}