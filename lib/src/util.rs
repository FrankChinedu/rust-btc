use crate::sha256::Hash;
use crate::types::Transaction;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Result as IoResult, Write};
use std::path::Path;

pub trait Saveable
where
    Self: Sized,
{
    fn load<I: Read>(&self, reader: I);
    fn save<O: Write>(&self, writer: O);
    fn save_to_file<P: AsRef<Path>>(&self, path: P) -> IoResult<()> {
        let file = File::create(&path)?;
        self.save(file);
        Ok(())
    }
    fn load_from_file<P: AsRef<Path>>(self, path: P) -> IoResult<Self> {
        let file = File::open(&path)?;
        self.load(file);
        Ok(self)
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct MerkleRoot(Hash);

impl MerkleRoot {
    pub fn calculate(transactions: &[Transaction]) -> MerkleRoot {
        let mut layer = vec![];

        for transaction in transactions {
            layer.push(Hash::hash(transaction));
        }

        while layer.len() > 1 {
            let mut new_layer = vec![];
            for pair in layer.chunks(2) {
                let left = pair[0];
                // if there is no right, use the left hash again
                let right = pair.get(1).unwrap_or(&pair[0]);
                new_layer.push(Hash::hash(&[left, *right]));
            }
            layer = new_layer;
        }
        MerkleRoot(layer[0])
    }
}
