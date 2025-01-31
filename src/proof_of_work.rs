use crate::block::Block;
use data_encoding::HEXLOWER;
use num_bigint::{BigInt, Sign};
use ring::digest::{Context, SHA256};
use std::borrow::Borrow;
use std::ops::ShlAssign;

pub struct ProofOfWork{
    block: Block,
    target: BigInt,
}

const TARGET_BITS: i32 = 20; // difficulty
const MAX_NONCE: i64 = i64::MAX; // To prevent nonce overflow

impl ProofOfWork {
    pub fn new_pow(block: Block) -> ProofOfWork {
        let mut target = BigInt::from(1);
        target.shl_assign(256 - TARGET_BITS); // target is equal to `1 << TARGET_BITS`
        ProofOfWork{
            block,
            target
        }
    }

    /// The data will be used in PoW
    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let pre_block_hash = self.block.get_pre_block_hash();
        let transactions_hash = self.block.hash_transactions();
        let timestamp = self.block.get_timestamp();
        
        let mut data_bytes = vec![];
        data_bytes.extend(pre_block_hash.as_bytes());
        data_bytes.extend(transactions_hash);
        data_bytes.extend(timestamp.to_be_bytes());
        data_bytes.extend(TARGET_BITS.to_be_bytes());
        data_bytes.extend(nonce.to_be_bytes());

        return data_bytes;
    }

    /// To find the valid hash
    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        let mut hash = Vec::new();
        println!("⛏️  Start mining👷, the block contains [{:?}] ", self.block.get_transactions());

        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = crate::sha256_digest(data.as_slice());
            // We use BigInt type to represent difficulty, because maybe it will be very large 
            // from_bytes_be: translate big-endian bytes order into BigInt
            // Sign::Plus: the BigInt will be noted as positive number
            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());

            if hash_int.lt(self.target.borrow()) {
                println!("🎉 Mining successfully! The hash is {}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
            }
        }

        println!();
        return (nonce, HEXLOWER.encode(hash.as_slice()));
    }
}

#[cfg(test)]
mod tests{
    use super::TARGET_BITS;
    use data_encoding::HEXLOWER;
    use num_bigint::BigInt;
    use std::ops::ShlAssign;

    #[test]
    fn test_sha256_digest() {
        let digest = crate::sha256_digest("hello".as_bytes());
        let hex_digest = HEXLOWER.encode(&digest.as_slice());
        println!("SHA-256 digest is {}", hex_digest);
    }

    #[test]
    fn test_bigint_from_bytes() {
        let a = BigInt::from(256);
        let (s, vec) = a.to_bytes_be();
        println!("{:?}, {:?}", s, vec);

        // big-endian
        let b = BigInt::from_signed_bytes_be(vec.as_slice());
        println!("{}", b);
    }

    #[test]
    fn test_target_bits() {
        let mut target = BigInt::from(1);
        target.shl_assign(256 - TARGET_BITS);
        println!("{}", target); // output: 110427941548649020598956093796432407239217743554726184882600387580788736

        let (_, vec) = target.to_bytes_be();
        let target_hex = HEXLOWER.encode(vec.as_slice());
        println!("{}", target_hex) // output: 100000000000000000000000000000000000000000000000000000000000
    }
}