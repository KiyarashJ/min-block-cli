use std::{fs, io};
use chrono::{DateTime, Local};
use p256::{ecdsa::{Signature, SigningKey}};
use p256::ecdsa::signature::Signer;
use rand::rngs::OsRng;
use hex;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    pub from: String,
    pub to: String,
    pub time: DateTime<Local>,
    pub amount: f64,
    pub digital_signature: String
}

pub fn transactions() {
    println!("Enter your destination Key: ");
    let mut destination_key = String::new();
    io::stdin().read_line(&mut destination_key).expect("failed to read you destination key :|");
    let mut currency_amount_raw = String::new();
    println!("Enter your Currency Amount: ");
    io::stdin().read_line(&mut currency_amount_raw).expect("failed to read you currency amount :|");
    // here we convert String variable to f64!
    let currency_amount = currency_amount_raw.trim().parse::<f64>().unwrap();
    // generate secret_key and an ability to sign it (SecretKey::random() has not sign ability)
    let sec_key = SigningKey::random(&mut OsRng);
    // generate public key from secret_key (if we use signingkey , he has its own formal method verifying_key() instead using VerifyingKey::from() in SecretKey)!!
    let verify_key_raw = sec_key.verifying_key();
    // here verify key must be converted to String , but cannot convert it to String directly because of that we're gonna convert it to bytes after that to string
    let public_verify_key = verify_key_raw.to_encoded_point(false);
    let verify_key = hex::encode(public_verify_key);
    // message 
    let msg = format!("{:?} gave {} currency to {} with amount of {}", sec_key, "bitCoin",  destination_key, currency_amount );
    // sign the message as u[8] ==> as_bytes
    let signature: Signature = sec_key.sign(msg.as_bytes());

    println!("\n✅ Message signed!");
    println!("Public Key: {:?}", verify_key);
    println!("Signature: {:?}", signature);

    let tr = Action {
        from: verify_key,
        to: destination_key.trim().to_string(),
        time: Local::now(),
        amount: currency_amount,
        digital_signature: signature.to_string()
    };

    let mut get_transactions = if let Ok(content) = fs::read_to_string("transaction.json") {
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };
    get_transactions.push(tr);
    let stringifying = serde_json::to_string_pretty(&get_transactions).unwrap();
    let _ = fs::write("transaction.json", stringifying);

    
}