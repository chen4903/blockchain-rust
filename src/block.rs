use data_encoding::HEXLOWER;
use ring::digest::{Context, SHA256};
use core::time;
use std::{thread::current, time::{SystemTime, UNIX_EPOCH}};

pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String, 
    data: String
}

impl Block {

    pub fn new_block(pre_block_hash: String, data: String) -> Block{
        let timestamp = current_timestamp();
        let hash = caculate_hash(timestamp, pre_block_hash.clone(), data.clone());

        Block {
            timestamp,
            pre_block_hash,
            hash,
            data
        }
    }

    pub fn new_genesis_block() -> Block {
        return Block::new_block(String::new(), String::from("Genesis Block"));
    }

    pub fn get_pre_block_hash(&self) -> String {
        return self.pre_block_hash.clone();
    } 

    pub fn get_data(&self) -> String {
        self.data.clone()
    }

    pub fn get_hash(&self) -> String {
        return self.hash.clone();
    }

    pub fn get_timestamp(&self) -> i64 {
        return self.timestamp.clone();
    }
}

fn current_timestamp() -> i64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64;
    // .as_millis() as i64: as_millis() 是 Duration 类型的一个方法，用于获取时间间隔的毫秒数部分。
    // 由于返回类型是 u128，但我们需要的是 i64 类型的时间戳，因此使用 as 关键字进行类型转换
}

fn caculate_hash(timestamp: i64, pre_block_hash: String, data: String) -> String {
    let block_data = format!("{}{}{}", timestamp, pre_block_hash, data);

    return sha256_digest(block_data);
}

fn sha256_digest(block_data: String) -> String { // SHA2-256
    let mut context = Context::new(&SHA256);
    context.update(block_data.as_bytes());
    let digest = context.finish();
    return HEXLOWER.encode(digest.as_ref());
}

#[cfg(test)]
mod tests {
    use super::Block;

    #[test]
    fn test_new_block() {
        let block = Block::new_block(
            "lsfdjfjsdlfjsdlfsjdlk".to_string(), 
            "hello".to_string()
        );
        println!("new block hash is {}", block.hash);
    }

    #[test]
    fn test_sha3_256_digest() {
        let digest = super::sha256_digest("world".to_string());
        println!("SHA3-256 digest is {}", digest);
    }
}