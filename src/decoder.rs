use serde_json::Value;

pub fn program_name(program_id: &str) -> &str {
    match program_id {
        "11111111111111111111111111111111" => "System Program",

        "ComputeBudget111111111111111111111111111111" =>
            "Compute Budget",

        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" =>
            "SPL Token",

        "ATokenGPvbdGVxr1..." =>
            "Associated Token Account",

        _ => "Unknown Program",
    }
}

pub fn instruction_name(instruction_number: u32) -> &'static str{
    match instruction_number{
        0 => "Create Account",
        2 => "Transfer",
        3 => "Create Account With Seed",
        4 => "Advance Nonce",
        _ => "Instruction Name not defined yet"
    }
}

pub fn decode_compute_budget_instruction(data: &Vec<u8>) {
    let discriminator = data[0];
    match discriminator {
        2 => {
            println!("Instruction  : Set Compute Unit Limit");
            let units = u32::from_le_bytes(data[1..5].try_into().expect("Invalid compute budget instruction"));
            println!("Units        : {}",units);
        },
        3 => {
            println!("Instruction  : Set Compute Unit Price");
            let units = u64::from_le_bytes(data[1..9].try_into().expect("Invalid compute budget instruction"));
            println!("Price        : {} micro-lamports",units);
        },
        _ => println!("Unknown Data")
    }
}