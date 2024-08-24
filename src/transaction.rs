use data_encoding::HEXLOWER;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::utils::sha256_digest;

const SUBSIDY: i32 = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TXInput {
    txid: Vec<u8>,      // A transaction input refers to an output of a previous transaction, 
                        // and the ID indicates which previous transaction it was
    vout: i32,          // output index
    script_sig: String, // unlocking output data
}

impl TXInput{
    pub fn can_unlock_output_with(&self, unlocking_data: &str) -> bool {
        self.script_sig.eq(unlocking_data)
    }

    pub fn get_txid(&self) -> Vec<u8> {
        self.txid.clone()
    }

    pub fn get_vout(&self) -> i32 {
        self.vout
    }

    pub fn get_script_sig(&self) -> String {
        self.script_sig.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TXOutput {
    value: i32,           
    script_pub_key: String,
}

impl TXOutput {
    pub fn new(value: i32, address: String) -> TXOutput {
        TXOutput {
            value,
            script_pub_key: address,
        }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_script_pub_key(&self) -> String {
        self.script_pub_key.clone()
    }

    pub fn can_be_unlocked_with(&self, unlocking_data: &str) -> bool {
        self.script_pub_key.eq(unlocking_data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    id: Vec<u8>,         //
    vin: Vec<TXInput>,   // input
    vout: Vec<TXOutput>, // output
}

impl Transaction{
    /// create a coinbase transaction, no input, one output
    pub fn new_coinbase_tx(to: String, mut data: String) -> Transaction {
        if data.len() == 0 {
            data = format!("Reward to {}", to)
        }

        let txin = TXInput {
            txid: vec![],
            vout: -1,
            script_sig: data,
        };
        let txout = TXOutput {
            value: SUBSIDY,
            script_pub_key: to,
        };
        let mut tx = Transaction {
            id: vec![],
            vin: vec![txin],
            vout: vec![txout],
        };

        tx.set_id();

        return tx;
    }

    pub fn is_coinbase(&self) -> bool {
        return self.vin.len() == 1 && self.vin[0].txid.len() == 0 && self.vin[0].vout == -1;
    }

    fn set_id(&mut self) {
        let data = bincode::serialize(self).unwrap();
        self.id = sha256_digest(data.as_slice());
    }

    pub fn get_id(&self) -> Vec<u8> {
        return self.id.clone();
    }

    pub fn get_vin(&self) -> Vec<TXInput> {
        self.vin.clone()
    }

    pub fn get_vout(&self) -> Vec<TXOutput> {
        self.vout.clone()
    }

    /// create a UTXO transaction
    pub fn new_utxo_transaction(
        from: String,
        to: String,
        amount: i32,
        unspent_transaction: Vec<Transaction>,
    ) -> Transaction {
        let mut accumulated = 0;
        let mut valid_outputs: HashMap<String, Vec<i32>> = HashMap::new();

        // Check weather we have enough money
        'outer:
            // iterates through the list of unspent transactions
            for tx in unspent_transaction {
                let txid_hex = HEXLOWER.encode(tx.id.as_slice());
                // iterates over each output (vout) of the current transaction.
                for idx in 0..tx.vout.len() {
                    let txout = tx.vout[idx].clone();

                    // checks if the current output can be unlocked by the from address
                    if txout.can_be_unlocked_with(from.as_str()) {
                        accumulated += txout.value;
                        // The index of this output (idx) is recorded in the valid_outputs hashmap.
                        if valid_outputs.contains_key(txid_hex.as_str()) {
                            valid_outputs
                                .get_mut(txid_hex.as_str())
                                .unwrap()
                                .push(idx as i32);
                        } else {
                            valid_outputs.insert(txid_hex.clone(), vec![idx as i32]);
                        }

                        // we have enough money
                        if accumulated >= amount {
                            break 'outer;
                        }
                    }
                }
            }
        
        if accumulated < amount {
            panic!("Error: Not enough funds")
        }

        // assemble our input
        let mut inputs = vec![];
        for (txid_hex, outs) in valid_outputs {
            let txid = HEXLOWER.decode(txid_hex.as_bytes()).unwrap();
            for out in outs {
                let input = TXInput {
                    txid: txid.clone(),
                    vout: out,
                    script_sig: from.clone(),
                };
                inputs.push(input);
            }
        }
        
        // assemble our output
        let mut outputs = vec![TXOutput::new(amount, to.clone())];
        // If the total number of UTXO exceeds the required amount, change will be generated
        if accumulated > amount {
            outputs.push(TXOutput::new(accumulated - amount, from))
        }
        let mut tx = Transaction {
            id: vec![],
            vin: inputs,
            vout: outputs,
        };
       

        tx.set_id();
        return tx;
    }

}