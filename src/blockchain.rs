use crate::block::Block;
use std::borrow::Borrow;

pub struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new_blockchain() -> BlockChain {
        let mut blocks = Vec::new();
        let genesis_block = Block::new_genesis_block();
        blocks.push(genesis_block);
        BlockChain{
            blocks
        }
    }

    /// @param data The content in the block
    pub fn add_block(&mut self, data: String) {
        let pre_block_hash = self.blocks[self.blocks.len() - 1].borrow().get_hash();
        let new_block = Block::new_block(pre_block_hash, data);
        self.blocks.push(new_block);
    }
}

#[cfg(test)]
mod tests {
    use super::BlockChain;

    #[test]
    fn test_block_chain() {
        let mut chain = BlockChain::new_blockchain();
        chain.add_block(String::from("Send 100 BTC to LEVI_104"));
        chain.add_block(String::from("Send 1 BTC to tom"));
        for block in chain.blocks {
            println!("Pre block hash: {}", block.get_pre_block_hash());
            println!("Cur block hash: {}", block.get_hash());
            println!("Data: {}", block.get_data());
            println!("Timestamp: {}\n", block.get_timestamp())
        }
    }
}