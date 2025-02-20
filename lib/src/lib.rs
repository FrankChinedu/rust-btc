use serde::{Deserialize, Serialize};
use uint::construct_uint;

construct_uint! {
// Construct an unsigned 256-bit integer
// consisting of 4 x 64-bit words
#[derive(Serialize, Deserialize)]
    pub struct U256(4);
}
pub mod crypto;
pub mod error;
pub mod sha256;
pub mod types;
pub mod util;

// Implement From<[u8; 32]> for U256
impl From<[u8; 32]> for U256 {
    fn from(bytes: [u8; 32]) -> Self {
        let mut result = U256::zero();
        for (i, chunk) in bytes.chunks(8).enumerate() {
            let word = u64::from_be_bytes(chunk.try_into().unwrap());
            result.0[i] = word;
        }
        result
    }
}
