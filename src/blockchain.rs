use crate::block::{check_difficulty, Block};
use crate::hashable::Hashable;
use crate::Hash;
use std::collections::HashSet;

#[derive(Debug)]
pub enum BlockValidationError {
    MismatchedIndex,
    InvalidHash,
    AchronicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct BlockChain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationError> {
        let i = self.blocks.len();

        if block.index != i as u32 {
            return Err(BlockValidationError::MismatchedIndex);
        } else if !check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationError::InvalidHash);
        } else if i != 0 {
            let previous_block = &self.blocks[i - 1];
            if block.timestamp <= previous_block.timestamp {
                return Err(BlockValidationError::AchronicalTimestamp);
            } else if block.previous_block_hash != previous_block.hash {
                return Err(BlockValidationError::MismatchedPreviousHash);
            }
        } else {
            if block.previous_block_hash != vec![0; 32] {
                return Err(BlockValidationError::InvalidGenesisBlockFormat);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationError::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();
                if !(&input_hashes - &self.unspent_outputs).is_empty()
                    || !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidationError::InvalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if output_value > input_value {
                    return Err(BlockValidationError::InsufficientInputValue);
                }

                let fee = input_value - output_value;
                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if coinbase.output_value() < total_fee {
                return Err(BlockValidationError::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created)
        }

        self.blocks.push(block);
        Ok(())
    }
}
