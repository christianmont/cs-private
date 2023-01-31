//! This module implements the blockchain.
//! 
//! You need to implement the `Blockchain` struct and its methods.

use crate::block::Block;
use crate::crypto::hash::{H256, Hashable};
use std::collections::HashMap;

pub struct Blockchain {
    // TODO: add fields here as you need, e.g.:
    //
    // hash_to_block: HashMap<H256, Block>,
    pub hash_to_block: HashMap<H256, Block>,
    pub hash_to_height: HashMap<H256, u64>,
    pub hash_to_tip: H256,
}

impl Blockchain {
    /// Create a new blockchain, only containing the genesis block
    pub fn new() -> Self {
        // TODO
        let gen = Block::genesis();
        let mut b = Blockchain {
            hash_to_block: HashMap::new(),
            hash_to_height: HashMap::new(),
            hash_to_tip: gen.hash(),
        };
        let gen_hash = gen.hash();
        b.hash_to_block.insert(gen_hash, gen);
        b.hash_to_height.insert(gen_hash, 0);
        return b;
    }

    /// Insert a block into blockchain
    pub fn insert(&mut self, block: &Block) {
        let par = block.header.parent;
        let mut parent_height = 0;
        if self.hash_to_height.contains_key(&par) {
            parent_height = self.hash_to_height[&par];
        }
        self.hash_to_block.insert(block.hash(), block.clone());
        self.hash_to_height.insert(block.hash(), parent_height+1);
        if parent_height+1 > self.hash_to_height[&self.hash_to_tip] {
            self.hash_to_tip = block.hash();
        }
    }

    /// Get the last block's hash of the longest chain
    pub fn tip(&self) -> H256 {
        return self.hash_to_tip;
    }

    /// Get all the blocks' hashes along the longest chain
    #[cfg(any(test, test_utilities))]
    pub fn all_blocks_in_longest_chain(&self) -> Vec<H256> {
        let mut tip = &self.hash_to_block[&self.hash_to_tip];
        let mut ans = Vec::new();
        let h = self.hash_to_height[&self.hash_to_tip];
        for _ in 1..h {
            ans.push(tip.hash());
            tip = &self.hash_to_block[&tip.header.parent];
        }
        return ans
    }
}


#[cfg(any(test, test_utilities))]
mod tests {
    use super::*;
    use crate::block::test::generate_random_block;

    #[test]
    fn insert_one() {
        let mut blockchain = Blockchain::new();
        let genesis_hash = blockchain.tip();
        let block = generate_random_block(&genesis_hash);
        blockchain.insert(&block);
        assert_eq!(blockchain.tip(), block.hash());
    }

    #[test]
    fn mp1_insert_chain() {
        let mut blockchain = Blockchain::new();
        let genesis_hash = blockchain.tip();
        let mut block = generate_random_block(&genesis_hash);
        blockchain.insert(&block);
        assert_eq!(blockchain.tip(), block.hash());
        for _ in 0..50 {
            let h = block.hash();
            block = generate_random_block(&h);
            blockchain.insert(&block);
            assert_eq!(blockchain.tip(), block.hash());
        }
    }

    #[test]
    fn mp1_insert_3_fork_and_back() {
        let mut blockchain = Blockchain::new();
        let genesis_hash = blockchain.tip();
        let block_1 = generate_random_block(&genesis_hash);
        blockchain.insert(&block_1);
        assert_eq!(blockchain.tip(), block_1.hash());
        let block_2 = generate_random_block(&block_1.hash());
        blockchain.insert(&block_2);
        assert_eq!(blockchain.tip(), block_2.hash());
        let block_3 = generate_random_block(&block_2.hash());
        blockchain.insert(&block_3);
        assert_eq!(blockchain.tip(), block_3.hash());
        let fork_block_1 = generate_random_block(&block_2.hash());
        blockchain.insert(&fork_block_1);
        assert_eq!(blockchain.tip(), block_3.hash());
        let fork_block_2 = generate_random_block(&fork_block_1.hash());
        blockchain.insert(&fork_block_2);
        assert_eq!(blockchain.tip(), fork_block_2.hash());
        let block_4 = generate_random_block(&block_3.hash());
        blockchain.insert(&block_4);
        assert_eq!(blockchain.tip(), fork_block_2.hash());
        let block_5 = generate_random_block(&block_4.hash());
        blockchain.insert(&block_5);
        assert_eq!(blockchain.tip(), block_5.hash());
    }

}