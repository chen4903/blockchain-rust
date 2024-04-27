# Brief

- block.rs
  - variable: 
  - structure: Block
  - Setter: create a new block, create the genesis block
  - Getter: data, hash, timestamp
  - helper: serialize and deserialize
- blockchain.rs
  - variable: current hash pointer
  - structure: blockchain, blockchain iterator
  - Setter: 
    - blockchain: create a new blockchain, add a block, create a new iterator
    - blockchain iterator: create a new iterator, find the next block(From the last block to the genesis block)
  - Getter: 
  - Helper: 
- proof_of_work.rs
  - variable: difficulty, nonce
  - structure: ProofOfWork
  - Setter: create a new PoW, start the PoW
  - Getter: prepare the data that would be used in PoW,
  - Helper: SHA2-256
- main.rs
  - add command cli

```
blockchain_rust 0.1.0

USAGE:
    blockchain_rust.exe <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    addblock      Add new block to chain
    help          Prints this message or the help of the given subcommand(s)
    printchain    Print blockchain all block
```







