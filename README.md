# 🪙 Simple Crypto Wallet & Blockchain CLI

Welcome to a minimal **cryptocurrency wallet** and **blockchain** implementation in Rust!  
This project demonstrates the basic concepts behind crypto transactions, block mining, and blockchain integrity — all in a simple command-line app.

---

## 🚀 Features

- 🔐 **Digital Signatures:** Sign and verify transactions securely using ECDSA (P-256 curve).  
- 📦 **Block Structure:** Store multiple transactions in blocks, linked with hashes.  
- ⛏️ **Proof-of-Work Mining:** Mine blocks by finding a hash with a set difficulty (leading zeros).  
- 🔗 **Blockchain:** Maintain an immutable chain of blocks stored in JSON files.  
- ✅ **Chain Validation:** Check the entire blockchain for consistency and tampering.

---

## 🛠️ Getting Started

1. Clone the repo:

```bash
git clone https://github.com/yourusername/simple-crypto-wallet.git
cd simple-crypto-wallet
```

# Build and run:
```bash
cargo run
```

# 📁 File Structure
## transaction.rs - Handles creation and signing of transactions.

## block.rs - Defines blocks, mining, and saving/loading from disk.
 
## blockchain.rs - Manages the chain of blocks and validation.

## main.rs - CLI entry point for user interaction.


# ⚙️ How It Works
## User inputs transaction data and signs it.

## Transactions are grouped into a block.

## The block is mined to meet the difficulty requirement.

## The mined block is added to the blockchain and saved.

## Blockchain validity is checked after each addition.


# 🎯 Goals
### This project is for learning purposes — to understand the basics of cryptography, blockchain structure, and Rust programming.


# 🌟 Thanks for checking this out!
### Keep coding & stay curious! 🚀✨

Made with ❤️ and Rust 🦀
