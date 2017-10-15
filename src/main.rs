extern crate sha2;
extern crate digest;
extern crate time;
extern crate hyper;
extern crate futures;
extern crate serde;
extern crate serde_json;


#[macro_use]
extern crate serde_derive;

use std::env;
use sha2::Sha256;
use sha2::Digest;
use digest::Input;
use hyper::{Method, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use serde_json::Error;

const HTTP_PORT: i32 = match env::var("HTTP_PORT") {
    Ok(val) => val.parse::<i32>().unwrap(),
    Err(e) => 3001,
};

const P2P_PORT: i32 = match env::var("P2P_PORT") {
    Ok(val) => val.parse::<i32>().unwrap(),
    Err(e) => 6001,
};

/*
static initial_peers: [&str] = match env::var("PEERS") {
    Ok(val) => val.split(','),
    Err(e) => [],
};*/


/// ブロックチェーン
static mut block_chain: Vec<Block> = vec![get_genesis_block()];

static mut sockets: Vec<&str> = Vec::new();

struct HttpServer;

impl Service for HttpServer {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = Error;
    // The future representing the eventual Response your call will resolve to.
    // This can change to whatever Future you need.
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/blocks") => response.set_body(serde_json::to_string(&block_chain).unwrap()),
            (&Method::Get, "/peers") => response.set_body("get peers"),
            (&Method::Post, "/mineBlock") => response.set_body("mine blcok"),
            (&Method::Post, "/addPeer") => response.set_body("add peer"),
            _ => response.set_status(StatusCode::NotFound),
        };
        futures::future::ok(response)
    }
}


/// 最新のブロックを取得する
///
/// # Return value
/// 最新のブロック
fn get_latest_block() -> &'static Block { block_chain.last().unwrap() }


#[derive(Serialize, Deserialize, Debug)]
struct Block {
    index: u64,
    previous_hash: &'static str,
    hash: String,
    timestamp: i64,
    data: String,
}

enum MessageType {
    QueryLatest = 0,
    QueryAll = 1,
    ResponseBlockChain = 2,
}

fn calculate_hash(index: u64, previous_hash: &str, timestamp: i64, data: &str) -> String {
    let seed: String = format!("{}{}{}{}", index, previous_hash, timestamp, data);
    let mut sha256 = Digest::new();
    sha256.input_str(seed);
    sha256.result_str()
}

fn calculate_hash_for_block(block: &Block) -> String {
    calculate_hash(block.index, block.previous_hash, block.timestamp, &block.data)
}

fn add_block(new_block: Block) -> () {
    if is_valid_new_block(&new_block, get_latest_block()) {
        block_chain.push(new_block)
    }
}

fn generate_next_block(block_data: String) -> Block {
    let previous_block: &Block = get_latest_block();
    let next_index = previous_block.index + 1;
    let next_timestamp = time::now().to_timespec().sec;
    let next_hash = calculate_hash(
        next_index, &previous_block.hash, next_timestamp, &block_data);
    Block {
        index: next_index,
        previous_hash: &previous_block.hash,
        hash: next_hash,
        timestamp: next_timestamp,
        data: block_data
    }
}

fn get_genesis_block() -> Block {
    return Block {
        index: 0,
        previous_hash: "0",
        hash: "816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7".to_string(),
        timestamp: 1465154705,
        data: "my genesis block!!".to_string()
    };
}


fn is_valid_new_block(new_block: &Block, previous_block: &Block) -> bool {
    if previous_block.index + 1 != new_block.index {
        println!("invalid index");
        false
    } else if previous_block.hash != new_block.previous_hash {
        println!("invalid previous hash");
        false
    } else if calculate_hash_for_block(new_block) != new_block.hash {
        println!("invalid hash: {:?} {:?}", calculate_hash_for_block(new_block), new_block.hash);
        false
    } else {
        true
    }
}

fn replace_chain(new_blocks: &[Block]) -> () {
    if is_valid_chain(new_blocks) && new_blocks.len() > block_chain.len() {
        println!("Received blockchain is valid. Replacing current blockchain with received blockchain");
        block_chain = new_blocks;
        // TODO: bloadcast
    } else {
        println!("Received blockchain invalid");
    }
}


fn is_valid_chain(block_chain_to_validate: &[Block]) -> bool {

}

fn main() {
    println!("Hello, world!");
}
