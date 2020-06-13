use blake2::{Blake2b, Digest};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u32,
    logtype: String,
    current_hash: Vec<u8>,
    data: String,
    previous_hash: Vec<u8>,
}
fn hash_calc(data: &String, salt: &str, previous_hash: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Blake2b::new();
    hasher.input(previous_hash);
    hasher.input(data);
    hasher.input(salt);
    hasher.result().to_vec()
}

fn init_genesis_block(salt: &str) -> Block {
    let genesis_block: Block = Block {
        index: 0,
        logtype: String::from("None"),
        current_hash: hash_calc(&"0".to_string(), salt, &vec![]),
        data: "".to_string(),
        previous_hash: vec![],
    };
    println!("Initial genesis block defined");
    genesis_block
}

fn create_block(
    previous_index: u32,
    logt: String,
    log_data: String,
    previous_hash: Vec<u8>,
    salt: String,
) -> Block {
    Block {
        index: previous_index + 1,
        logtype: logt,
        current_hash: hash_calc(&log_data, &salt, &previous_hash),
        data: log_data,
        previous_hash: previous_hash,
    }
}

fn load_blockchain() -> Vec<Block> {
    let file = fs::File::open("/data/workspace/git_repos/logchain/blockchain.json")
        .expect("File not found");
    let deserialized_json: Vec<Block> =
        serde_json::from_reader(file).expect("Error while reading file");
    println!("Blockchain Loaded");
    deserialized_json
}

fn save_blockchain(chain: &Vec<Block>) {
    serde_json::to_writer(
        &fs::File::create("/data/workspace/git_repos/logchain/blockchain.json").unwrap(),
        &chain,
    )
    .expect("error");
    println!("Blockchain_written");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn log_parser() -> Vec<Block> {
    let salt = String::from("Random_Salt123");
    let mut chain: Vec<Block> = load_blockchain();

    if chain.is_empty() {
        chain.push(init_genesis_block(&salt));
        save_blockchain(&chain);
        println!("Initial genesis_block created");
    }

    let iter_logs = read_lines("/data/workspace/git_repos/logchain/logs/bash_log.txt").unwrap();
    for line in iter_logs {
        let last_block = chain.clone();
        let data = line.unwrap();
        let previous_index = &chain.last().unwrap().index;
        let logt = String::from("Bash_Log");
        let previous_hash = last_block.last().unwrap().current_hash.to_vec();
        chain.push(create_block(
            *previous_index,
            logt,
            data,
            previous_hash,
            String::from(&salt),
        ));
    }
    chain
}

fn print_chain(chain: &Vec<Block>) {
    for data in chain {
        println!("{:?}", data);
    }
}

fn main() {
    let test: Vec<Block> = vec![init_genesis_block(&"Random_Salt".to_string())];
    save_blockchain(&test);
    // let test = load_blockchain();
    // println!("{:?}", test);
    let chain = log_parser();
    print_chain(&chain);

    // log_parser(&chain, &salt);
}
