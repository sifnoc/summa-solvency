use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
    thread::{sleep, spawn},
};

use csv::WriterBuilder;
use ethers::{
    abi::{encode, Token},
    prelude::SignerMiddleware,
    signers::{Ledger, LocalWallet, Signer, WalletError},
    types::Signature,
    utils::{keccak256, to_checksum},
};
use serde_json::{from_str, to_string_pretty};

use summa_backend::apis::csv_parser::SignatureRecord;

async fn signer(message: &str) -> Vec<SignatureRecord> {
    // Using private keys directly is insecure.
    // Instead, consider leveraging hardware wallet support.
    // `ethers-rs` provides support for both Ledger and Trezor hardware wallets.
    //
    // For example, you could use the Ledger wallet as shown below:
    // let signing_wallets = (0..2).map(|index| Ledger::new(HDPath::LedgerLive(index), 1).await.unwrap()).collect();
    //
    // Refers to: https://docs.rs/ethers/latest/ethers/signers/index.html
    let private_keys = &[
        "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        "0xde9be858da4a475276426320d5e9262ecfc3ba460bfac56360bfa6c4c28b4ee0",
    ];

    let signing_wallets: Vec<LocalWallet> = private_keys
        .iter()
        .map(|private_key| LocalWallet::from_str(private_key).unwrap())
        .collect();

    let encoded_message = encode(&[Token::String(message.to_owned())]);
    let hashed_message = keccak256(encoded_message);

    let mut signatures: Vec<SignatureRecord> = Vec::new();

    // Iterating signing wallets and generate signatures to put `signatures` vector
    for wallet in signing_wallets {
        let signature = wallet.sign_message(hashed_message).await.unwrap();
        let record = SignatureRecord::new(
            "ETH".to_string(),
            to_checksum(&wallet.address(), None), //
            format!("0x{}", signature.to_string()),
            message.to_string(),
        );
        signatures.push(record);
    }
}
