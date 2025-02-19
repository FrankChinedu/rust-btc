use std::fmt;

use crate::U256;
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Hash(U256);

#[allow(clippy::self_named_constructors)]
impl Hash {
    pub fn hash<T: serde::Serialize>(data: &T) -> Self {
        let mut serialized = vec![];
        if let Err(e) = ciborium::into_writer(data, &mut serialized) {
            panic!(
                r"Failed to serialize data: {:?}. \ 
                This should not happen",
                e
            )
        }
        let hash = digest(&serialized);
        let hash_bytes = hex::decode(hash).unwrap();
        let hash_array: [u8; 32] = hash_bytes.as_slice().try_into().unwrap();
        Hash(U256::from(hash_array))
    }

    pub fn matches_target(&self, target: U256) -> bool {
        self.0 <= target
    }

    pub fn zero() -> Self {
        Hash(U256::zero())
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
