mod modules;

use chrono::Local;
use crate::modules::blockchain::BlockChain;
use crate::modules::transaction::Action;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let hardness = 3;

    
    let mut chain = BlockChain::load("blocks.json", hardness)?;

    println!("✅ Current chain length: {}", chain.block_lists.len());
    println!("✅ Is chain valid? {}", chain.is_valid_chain());

    
    let tx = Action {
        from: "alice".to_string(),
        to: "bob".to_string(),
        time: Local::now(),
        amount: 42.0,
        digital_signature: "dummy-signature".to_string(),
    };

    
    chain.add_block(vec![tx])?;

    println!("✅ Added new block.");
    println!("✅ Chain length after add: {}", chain.block_lists.len());
    println!("✅ Is chain still valid? {}", chain.is_valid_chain());

    Ok(())
}
