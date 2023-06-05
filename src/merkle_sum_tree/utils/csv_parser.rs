use crate::merkle_sum_tree::Entry;
use num_bigint::BigInt;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct CsvEntry {
    username: String,
    balance: Vec<String>,
}

pub fn parse_csv_to_entries<P: AsRef<Path>, const N_ASSETS: usize>(
    path: P,
) -> Result<Vec<Entry<N_ASSETS>>, Box<dyn Error>> {
    let mut entries = Vec::new();
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut balances_acc: Vec<BigInt> = vec![BigInt::from(0); N_ASSETS];

    for result in rdr.deserialize() {
        let record: CsvEntry = result?;
        let mut balances_big_int: Vec<BigInt> = Vec::new();
        for balance in record.balance {
            balances_big_int.push(BigInt::parse_bytes(balance.as_bytes(), 10).unwrap());
        }
        balances_acc = balances_acc
            .iter()
            .zip(balances_big_int.iter())
            .map(|(x, y)| x + y)
            .collect();
        let entry = Entry::new(record.username, balances_big_int.try_into().unwrap())?;
        entries.push(entry);
    }

    // For preventing overflow, we set the maximum value limit to 2^251 - 1
    const MAX_VALUE_STR: &str = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

    // throw error if balance is larger than the modulus
    if balance_acc >= BigInt::parse_bytes(MAX_VALUE_STR.as_bytes(), 16).unwrap() {
        return Err("Balance is larger than the maximum value limit".into());
    }

    Ok(entries)
}
