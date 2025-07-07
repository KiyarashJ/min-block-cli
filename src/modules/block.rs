use std::fs;
use std::io;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use hex;

use crate::modules::transaction::Action;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockNum {
    pub num: u32,
    pub timestamp: DateTime<Local>,
    pub tr_list: Vec<Action>,
    pub previous_hash: String,
    pub current_hash: String,
    pub nonce: u32,
}

impl BlockNum {
    
    fn material_for_hash(&self) -> String {
        #[derive(Serialize)]
        struct Hashable<'a> {
            num: u32,
            timestamp: &'a DateTime<Local>,
            tr_list: &'a Vec<Action>,
            previous_hash: &'a String,
            nonce: u32,
        }
        let h = Hashable {
            num: self.num,
            timestamp: &self.timestamp,
            tr_list: &self.tr_list,
            previous_hash: &self.previous_hash,
            nonce: self.nonce,
        };
        serde_json::to_string(&h).expect("hash serialisation failed")
    }

    
    pub fn calculate_hash(&self) -> String {
        let bytes = self.material_for_hash();
        let digest = Sha256::new().chain_update(bytes.as_bytes()).finalize();
        hex::encode(digest)
    }

    
    pub fn mine_block(&mut self, hardness: usize) -> io::Result<()> {
        let target = "0".repeat(hardness);
        loop {
            self.current_hash = self.calculate_hash();
            if self.current_hash.starts_with(&target) {
                break;
            }
            self.nonce += 1;
        }
        println!("🪙 mined block #{} (nonce {}, hash {})", self.num, self.nonce, self.current_hash);
        self.save("blocks.json")
    }

    
    pub fn save(&self, path: &str) -> io::Result<()> {
        let mut blocks: Vec<BlockNum> = match fs::read_to_string(path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => Vec::new(),
        };
        blocks.push(self.clone());
        let json = serde_json::to_string_pretty(&blocks)?;
        fs::write(path, json)
    }
}


pub fn load_or_init_chain(path: &str, hardness: usize) -> io::Result<Vec<BlockNum>> {
    if let Ok(data) = fs::read_to_string(path) {
        if let Ok(chain) = serde_json::from_str::<Vec<BlockNum>>(&data) {
            return Ok(chain);
        }
    }
    let mut genesis = BlockNum {
        num: 0,
        timestamp: Local::now(),
        tr_list: Vec::new(),
        previous_hash: "0".into(),
        current_hash: String::new(),
        nonce: 0,
    };
    genesis.mine_block(hardness)?;
    Ok(vec![genesis])
}