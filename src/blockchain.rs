use crate::block::Block;
use sled::Db;
use std::env::current_dir; // to get the current directory

// it pointes to the latest block hash, so that we can search for any block
const CURRENT_HASH_POINTER: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";

pub struct BlockChain {
    tip: String, // the newest block's hash
    db: Db, // to store data, use third-party crate: sled::Db
}

/// @dev Help to iterate the blockchain
pub struct BlockChainIterator{
    current_hash: String,
    db: Db,
}

impl BlockChainIterator {
    fn new(tip: String, db: Db) -> BlockChainIterator{
        BlockChainIterator {
            current_hash: tip,
            db
        }
    }

    /// @dev next() means starting from the latest block and tracing back to the Genesis block
    pub fn next(&mut self) -> Option<Block> {
        // we can get the data by hash in the database
        let data = self.db.get(self.current_hash.clone()).unwrap();
        if data.is_none() {
            return None;
        }
        // The data is not Block type, so we need to deserialize it to Block type
        let block = Block::deserialize(data.unwrap().to_vec().as_slice());

        // Update the iterator state, so that we can iterate it the next time
        self.current_hash = block.get_pre_block_hash().clone();
        return Some(block);
    }
}

impl BlockChain {
    /// @dev When we create a new blockchain, its data will be store in the current directory
    pub fn new_blockchain() -> BlockChain {
        // db is None or fill with data
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();
        // get the data from database
        let data = db.get(CURRENT_HASH_POINTER).unwrap();
        let tip; // current hash
        if data.is_none() { // If it is the first time to init the blockchain
            let block = Block::new_genesis_block();
            let block_hash = block.get_hash();
            // <genesis block's hash, block>
            let _ = db.insert(block_hash.clone(), block);
            // update the current hash pointer
            let _ = db.insert(CURRENT_HASH_POINTER, block_hash.as_bytes().to_vec());
            tip = block_hash;
        } else {
            tip = String::from_utf8(data.unwrap().to_vec()).unwrap();
        }

        return BlockChain{tip, db}
    }

    /// @param data The content in the block
    pub fn add_block(&mut self, data: String) {
        // create a new block with current hash and data
        let block = Block::new_block(self.tip.clone(), data);
        let block_hash = block.get_hash();

        // update the block into our database
        let _ = self.db.insert(block_hash.clone(), block);
        // update the current hash pointer
        let _ = self.db.insert(CURRENT_HASH_POINTER, block_hash.as_bytes().to_vec());
        self.tip = block_hash;
    }

    pub fn new_iterator(&self) -> BlockChainIterator {
        return BlockChainIterator::new(self.tip.clone(), self.db.clone());
    }
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    #[test]
    fn test_block_chain() {
        let mut blockchain = super::BlockChain::new_blockchain();
        blockchain.add_block(String::from("Send 1 BTC to Mars"));
    }

    #[test]
    fn test_sled() {
        // get the db handle
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();
        // get the value of key="name"
        let ret = db.get("name").unwrap();
        if ret.is_none() {
            println!("Not found the value of key=\"name\"");
        }
        let _ = db.insert("name", "LEVI_104");
        if let Some(v) = db.get("name").unwrap() {
            println!("data = {}", String::from_utf8(v.to_vec()).unwrap());
            let _ = db.remove("name");
        }
    }
}