extern crate sha2;
extern crate time;

use sha2::Sha256;

// ブロックチェーン
static mut block_chain: Vec<Block> = !vec[get_genesis_block()];

fn get_latest_block() -> Block {
    block_chain.last().unwrap()
}

struct Block<'a> {
    index: u64,
    previous_hash: &'a str,
    hash: String,
    timestamp: i64,
    data: String,
}

fn calculate_hash(index: u64, previous_hash: &str, timestamp: i64, data: &str) -> String {
    let mut sha256 = Sha::new();
    sha256.process(index + previous_hash + timestamp + data).to_string()
}

fn calculate_hash_for_block(block: Block) -> String {
    calculate_hash(block.index, block.previous_hash, block.timestamp, block.data)
}

fn add_block(new_block: Block) -> () {
    if is_valid_new_block(new_block,get_latest_block()) {
        block_chain.push(new_block)
    }
}

fn generate_next_block(block_data: String) -> Block {
    let previous_block: Block = get_latest_block();
    let next_index = previous_block.index + 1;
    let next_timestamp = time::now().to_timespec().sec;
    let next_hash = calculate_hash(
        next_index, previous_block.hash, next_timestamp, block_data);
    Block {
        index: next_index,
        previous_hash: previous_block.hash,
        hash: next_hash,
        timestamp: next_timestamp,
        data: block_data
    }
}

fn get_genesis_block() -> Block {
    return Block {
        index: 0,
        previous_hash: "0".to_string(),
        hash: "816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7".to_string(),
        timestamp: 1465154705,
        data: "my genesis block!!".to_string()
    };
}


fn is_valid_new_block(new_block: Block, previous_block: Blcok) -> bool {
    if previous_block.index + 1 != new_block.index {
        println!("invalid index");
        false
    } else if previous_block.hash != new_block.previous_hash {
        println!("invalid previous hash");
        false
    } else if calculate_hash_for_block(new_block) != new_block.hash {
        println!("invalid hash: {:?} {:?}", calculate_hash_for_block(new_block), new_block.hash);
        false
    }

    true
}

fn replace_chain(new_blocks: Block) -> {

}


fn is_valid_chain(block_chain_to_validate: Vec<Block>) -> bool {

}

fn main() {
    println!("Hello, world!");
}
