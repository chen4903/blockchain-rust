use crate::block::Block;
use crate::transaction::TXOutput;
use sled::Db;
use std::collections::HashMap;
use data_encoding::HEXLOWER;
use std::env::current_dir; // to get the current directory
use crate::transaction::Transaction;

// it pointes to the latest block hash, so that we can search for any block
const TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";
const GENESIS_ADDRESS: &str = "Genesis";
const GENESIS_COINBASE_DATA: &str =
    "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";

pub struct BlockChain {
    tip: String, // the newest block's hash
    db: Db, // to store data, use third-party crate: sled::Db
}

impl BlockChain {
    /// @dev When we create a new blockchain, its data will be store in the current directory
    pub fn new_blockchain() -> BlockChain {
        // init the db
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();
        // get the data from database
        let data = db.get(TIP_BLOCK_HASH_KEY).unwrap();
        let tip; // current hash

        if data.is_none() { // If it is the first time to init the blockchain
            let coinbase_tx = Transaction::new_coinbase_tx(
                String::from(GENESIS_ADDRESS),
                String::from(GENESIS_COINBASE_DATA),
            );
            let block = Block::new_genesis_block(coinbase_tx);
            let block_hash = block.get_hash();
            // <genesis block's hash, block>
            let _ = db.insert(block_hash.clone(), block);
            // update the current hash pointer
            let _ = db.insert(TIP_BLOCK_HASH_KEY, block_hash.as_bytes().to_vec());
            tip = block_hash;
        } else {
            tip = String::from_utf8(data.unwrap().to_vec()).unwrap();
        }

        return BlockChain{tip, db}
    }

    pub fn mine_block(&mut self, transactions: Vec<Transaction>) {
        // create a new block with current hash and data
        let block = Block::new_block(self.tip.clone(), transactions);
        let block_hash = block.get_hash();

        // update the block into our database
        let _ = self.db.insert(block_hash.clone(), block);
        // update the current hash pointer
        let _ = self.db.insert(TIP_BLOCK_HASH_KEY, block_hash.as_bytes().to_vec());
        self.tip = block_hash;
    }

    pub fn new_iterator(&self) -> BlockChainIterator {
        return BlockChainIterator::new(self.tip.clone(), self.db.clone());
    }

    pub fn clear_data(&self) {
        let _ = self.db.clear();
    }

    /// Find unspent transaction outputs
    /// 1. Some outputs are not associated with any inputs,E.g., coinbase mining rewards.
    /// 2. The inputs of a transaction can reference outputs from multiple previous transactions.
    /// 3. Each input must reference one output.
    pub fn find_unspent_transactions(&self, address: &str) -> Vec<Transaction> {
        let mut unspent_txs = vec![];
        let mut spent_txos: HashMap<String, Vec<i32>> = HashMap::new();

        let mut block_iterator = self.new_iterator();
        loop {
            let block = block_iterator.next();
            if block.is_none() {
                break;
            }
            for tx in block.unwrap().get_transactions() {
                let txid_hex = HEXLOWER.encode(tx.get_id().as_slice());
                let txout = tx.get_vout();

                'outer:
                    for idx in 0..txout.len() {
                        let txout = txout[idx].clone();

                        // filter spent outputs
                        if spent_txos.contains_key(txid_hex.as_str()) {
                            let outs = spent_txos.get(txid_hex.as_str()).unwrap();
                            for out in outs {
                                if out.eq(&(idx as i32)) {
                                    continue 'outer;
                                }
                            }
                        }
                        if txout.can_be_unlocked_with(address) {
                            unspent_txs.push(tx.clone())
                        }
                    }
                
                if tx.is_coinbase() {
                    continue;
                }
                // Find spent outputs in the inputs
                for txin in tx.get_vin() {
                    if txin.can_unlock_output_with(address) {
                        let txid_hex = HEXLOWER.encode(txin.get_txid().as_slice());
                        if spent_txos.contains_key(txid_hex.as_str()) {
                            let outs = spent_txos.get_mut(txid_hex.as_str()).unwrap();
                            outs.push(txin.get_vout());
                        } else {
                            spent_txos.insert(txid_hex, vec![txin.get_vout()]);
                        }
                    }
                }
            }
        }
        return unspent_txs;
    }

    pub fn find_utxo(&self, address: &str) -> Vec<TXOutput> {
        let transactions = self.find_unspent_transactions(address);
        let mut utxos = vec![];
        for transaction in transactions {
            for out in transaction.get_vout() {
                if out.can_be_unlocked_with(address) {
                    utxos.push(out);
                }
            }
        }
        return utxos;
    }

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


#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use crate::transaction::Transaction;

    #[test]
    fn test_block_chain() {
        let mut blockchain = super::BlockChain::new_blockchain();
        let transaction = Transaction::new_coinbase_tx(String::from("mars"), String::from("miko"));
        blockchain.mine_block(vec![transaction]);
    }

    #[test]
    fn test_sled() {
        // get the db handle
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("data");
        let db = sled::open(db_path).unwrap();
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