mod decoder;
mod printer;
mod rpc;

use anyhow::Result;

use crate::decoder::{ decode_compute_budget_instruction, instruction_name, program_name, token_instruction_name};

#[tokio::main]
async fn main() -> Result<()> {
    let signature = std::env::args()
        .nth(1)
        .expect("Usage: cargo run -- <signature>");

    let tx = rpc::fetch_transaction(&signature).await?;

    // let json_tx =  serde_json::to_string_pretty(&tx)?;

    let result = &tx["result"];
    let slot = result["slot"].as_u64().expect("Missing slot");
    let status = if result["meta"]["err"].is_null() {
        "Success"
    }else{
        "Failed"
    };

    let fee = result["meta"]["fee"].as_u64().expect("Missing fee");



    // println!("{}",tx);


    println!("Transaction\n-------------------------");
    println!("Slot: {}\nStatus: {}\nFee: {} lamports\n",slot, status, fee);

    println!("Account Keys\n-------------------------");
    let account_keys = result["transaction"]["message"]["accountKeys"].as_array().expect("Missing keys array");
    for (index, key) in account_keys.iter().enumerate(){
        println!("{}: {}",index,key.as_str().expect("Missing key"));
    }

    println!("\nInstructions\n-------------------------");
    let instructions = result["transaction"]["message"]["instructions"].as_array().expect("Missing keys array");
    for (index,inst) in instructions.iter().enumerate(){
        let program_index = inst["programIdIndex"].as_u64().expect("Missing Program index");
        let program_id = account_keys[program_index as usize].as_str().unwrap();
        let program_name = program_name(program_id);
        let data = bs58::decode(inst["data"].as_str().unwrap()).into_vec()?;
        println!("Instruction {}",index);
        println!("Program Index: {}",program_index);
        println!("Program ID   : {}",program_id);
        println!("Program      : {}",program_name);
        println!("Data         : {:?}",data);
        if program_name == "System Program" {
            let instruction = u32::from_le_bytes([
                data[0],
                data[1],
                data[2],
                data[3],
            ]);

            match instruction {
                2 => {
                    let lamports = u64::from_le_bytes(data[4..12].try_into().expect("Invalid transfer instruction"));
                    let accounts = inst["accounts"].as_array().expect("Missing accounts");
                    let from = account_keys[accounts[0].as_u64().unwrap() as usize].as_str().unwrap();
                    let to = account_keys[accounts[1].as_u64().unwrap() as usize].as_str().unwrap();
                    // println!("Data         : {:?}",data);
                    println!("Instruction  : {}",instruction_name(instruction));
                    println!("Sender       : {}",from);
                    println!("Receiver     : {}",to);
                    println!("Amount       : {} SOL",lamports as f32 / 1_000_000_000.0);
                }
                4 => {
                    println!("Instruction : Advance Nonce");
                }
                _ => {
                    println!("Instruction: {}", instruction_name(instruction))
                }
            }

        }else if program_name == "Compute Budget"{
            decode_compute_budget_instruction(&data);

        }else if program_name == "SPL Token" {
            let instruction = data[0];

            match instruction {
                12 => {
                    let amount = u64::from_le_bytes(data[1..9].try_into().unwrap());
                    let decimals = data[9];
                    let ui_amount = amount as f64 / 10f64.powi(decimals as i32);

                    let accounts = inst["accounts"]
                        .as_array()
                        .expect("Missing accounts");

                    println!("Instruction   : {}", token_instruction_name(instruction));
                    println!("Source        : {}", account_keys[accounts[0].as_u64().unwrap() as usize].as_str().unwrap());
                    println!("Mint          : {}",account_keys[accounts[1].as_u64().unwrap() as usize].as_str().unwrap());
                    println!("Destination   : {}",account_keys[accounts[2].as_u64().unwrap() as usize].as_str().unwrap());
                    println!("Authority     : {}",account_keys[accounts[3].as_u64().unwrap() as usize].as_str().unwrap());
                    println!("Amount        : {}", ui_amount);
                    println!("Decimals      : {}", decimals);
                }
                _ => {
                    println!("Instruction  : {}",token_instruction_name(instruction));
                }
            }
        }
        println!("");


        
    }

    println!("Balance Changes\n-------------------------");
        for (index, key) in account_keys.iter().enumerate(){
            let pre_balance = result["meta"]["preBalances"][index].as_u64().expect("Missing balance");
            let post_balance = result["meta"]["postBalances"][index].as_u64().expect("Missing balance");
            if pre_balance!=post_balance {
                let account = account_keys[index].as_str().unwrap();
                let balance_change = ( post_balance as i64 - pre_balance as i64 ) as f64 /  1_000_000_000.0;
                println!("{}",account);
                println!("{:+.9} SOL\n",balance_change);

            }
        }

    // println!("{}",serde_json::to_string_pretty(&result["meta"]["preTokenBalances"])?);
    // println!("{}",serde_json::to_string_pretty(&result["meta"]["postTokenBalances"])?);
    println!("Token Balance Changes\n-------------------------");

    let pre_token_balances = result["meta"]["preTokenBalances"].as_array().expect("Missing preTokenBalances");
    let post_token_balances = result["meta"]["postTokenBalances"].as_array().expect("Missing postTokenBalances");

    for (pre, post) in pre_token_balances.iter().zip(post_token_balances.iter()) {
        let account_index = pre["accountIndex"].as_u64().expect("Missing account index") as usize;
        let token_account = account_keys[account_index].as_str().unwrap();

        let mint = pre["mint"].as_str().unwrap();
        let owner = pre["owner"].as_str().unwrap();

        let decimals = pre["uiTokenAmount"]["decimals"].as_u64().unwrap() as u32;
        let pre_amount = pre["uiTokenAmount"]["amount"].as_str().unwrap().parse::<i64>()?;
        let post_amount = post["uiTokenAmount"]["amount"].as_str().unwrap().parse::<i64>()?;
        let diff = post_amount - pre_amount;

        let ui_diff = diff as f64 / 10f64.powi(decimals as i32);

        if diff != 0 {
            println!("Token Account : {}", token_account);
            println!("Mint          : {}", mint);
            println!("Owner         : {}", owner);
            println!("Change        : {:+}", ui_diff);
            println!();
        }
    }

    Ok(())
}