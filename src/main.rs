mod block;
mod blockchain;

use blockchain::BlockChain;

fn main() {
    // 创建一个新的区块链
    let mut chain = BlockChain::new_block_chain();

    // 添加一些块到区块链
    chain.add_block(String::from("Send 100 BTC to LEVI_104"));
    chain.add_block(String::from("Send 1 BTC to tom"));

    // 打印区块链中的所有块的信息
    for block in chain.blocks {
        println!("Pre block hash: {}", block.get_pre_block_hash());
        println!("Cur block hash: {}", block.get_hash());
        println!("Data: {}", block.get_data());
        println!("Timestamp: {}\n", block.get_timestamp());
    }
}
