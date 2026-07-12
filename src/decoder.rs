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