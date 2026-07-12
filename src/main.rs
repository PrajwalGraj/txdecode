mod decoder;
mod printer;
mod rpc;

use anyhow::Result;

use crate::decoder::{compute_budget_instruction_name, instruction_name, program_name};

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
            let lamports = u64::from_le_bytes(data[4..12].try_into().expect("Invalid transfer instruction"));
            let accounts = inst["accounts"].as_array().expect("Missing accounts");
            let from = account_keys[accounts[0].as_u64().unwrap() as usize].as_str().unwrap();
            let to = account_keys[accounts[1].as_u64().unwrap() as usize].as_str().unwrap();
            // println!("Data         : {:?}",data);
            println!("Instruction  : {}",instruction_name(instruction));
            println!("Sender       : {}",from);
            println!("Receiver     : {}",to);
            println!("Amount       : {} SOL",lamports as f32 / 1_000_000_000.0);
        }else if program_name == "Compute Budget"{
            let compute_name = compute_budget_instruction_name(data[0]);

            println!("Instruction  : {}",compute_name);
            if compute_name == "Set Compute Unit Price"{
                let units = u64::from_le_bytes(data[1..9].try_into().expect("Invalid compute budget instruction"));
                println!("Price        : {} micro-lamports",units);
            }else{
                let units = u32::from_le_bytes(data[1..5].try_into().expect("Invalid compute budget instruction"));
                println!("Units        : {}",units);
            }
        }
        println!("");

        
    }


    Ok(())
}