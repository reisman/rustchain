use rustchainlib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_address: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_address: "Bob".to_owned(),
                    value: 7,
                },
            ],
        }],
        difficulty,
    );

    genesis_block.mine();
    println!("{:?}", &genesis_block);

    let last_hash = genesis_block.hash.clone();

    let mut blockchain = BlockChain::new();
    blockchain
        .update_with_block(genesis_block)
        .expect("Error detected");

    let mut block = Block::new(
        1,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_address: "Chris".to_owned(),
                    value: 536,
                }],
            },
            Transaction {
                inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
                outputs: vec![
                    transaction::Output {
                        to_address: "Alice".to_owned(),
                        value: 36,
                    },
                    transaction::Output {
                        to_address: "Bob".to_owned(),
                        value: 12,
                    },
                ],
            },
        ],
        difficulty,
    );
    block.mine();
    println!("{:?}", &block);

    blockchain.update_with_block(block).expect("Error detected");
}
