use crate::proof_of_work::ProofOfWork;
use crate::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use sled::IVec;
use crate::utils::sha256_digest;

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String, 
    transactions: Vec<Transaction>,
    nonce: i64
}

impl Block {

    pub fn new_block(pre_block_hash: String, transactions: Vec<Transaction>) -> Block{
        let mut block = Block {
            timestamp: current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions,
            nonce: 0
        };

        // After create the block, we should execute PoW to make it valid
        let pow = ProofOfWork::new_pow(block.clone());
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;

        return block;
    }

    /// @dev Deserializing from a byte array
    pub fn deserialize(bytes: &[u8]) -> Block {
        return bincode::deserialize(bytes).unwrap();
    }

    pub fn new_genesis_block(transaction: Transaction) -> Block {
        return Block::new_block(String::new(),  vec![transaction]);
    }

    pub fn hash_transactions(&self) -> Vec<u8>{
        let mut txhashes = vec![];
        for transaction in self.transactions.clone() {
            let txid = transaction.get_id();
            txhashes.extend(txid.as_slice());
        }

        return sha256_digest(txhashes.as_slice())
    }

    pub fn get_pre_block_hash(&self) -> String {
        return self.pre_block_hash.clone();
    } 

    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    pub fn get_hash(&self) -> String {
        return self.hash.clone();
    }

    pub fn get_timestamp(&self) -> i64 {
        return self.timestamp.clone();
    }
}

impl From<Block> for IVec {
    fn from(b: Block) -> Self {
        let bytes = bincode::serialize(&b).unwrap();
        Self::from(bytes)
    }
}

/// @dev Get the current time of unix
fn current_timestamp() -> i64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64;
}

#[cfg(test)]
mod tests {
    use super::Block;
    use crate::utils::sha256_digest;
    use crate::transaction::Transaction;
    use data_encoding::HEXLOWER;

    #[test]
    fn test_sha256_digest() {
        let digest = sha256_digest("hello".as_bytes());
        let hex_digest = HEXLOWER.encode(digest.as_slice());
        println!("SHA-256 digest is {}", hex_digest)
    }

    #[test]
    fn test_new_block() {
        let block = Block::new_block(
            String::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"),
            vec![],
        );
        println!("new block hash is {}", block.hash) 
    }

    #[test]
    fn test_serialize() {
        let tx = Transaction::new_coinbase_tx(String::from("Genesis"), String::from("Genesis data"));
        let block = Block::new_block(
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".to_string(), 
            vec![tx]
        );
        let bytes = bincode::serialize(&block).unwrap();
        let desc_block = Block::deserialize(&bytes);
        assert_eq!(block.hash, desc_block.hash);

    }

}