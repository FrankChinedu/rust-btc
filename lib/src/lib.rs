use serde::{Deserialize, Serialize};
use uint::construct_uint;

construct_uint! {
// Construct an unsigned 256-bit integer
// consisting of 4 x 64-bit words
#[derive(Serialize, Deserialize)]
    pub struct U256(4);
}

// initial reward in bitcoin - multiply by 10^8 to get satoshis
pub const INITIAL_REWARD: u64 = 50;
// halving interval in blocks
pub const HALVING_INTERVAL: u64 = 210;
// ideal block time in seconds
pub const IDEAL_BLOCK_TIME: u64 = 10;
// minimum target
pub const MIN_TARGET: U256 = U256([
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0x0000_FFFF_FFFF_FFFF,
]);
// difficulty update interval in blocks
pub const DIFFICULTY_UPDATE_INTERVAL: u64 = 50;
// maximum mempool transaction age in seconds
pub const MAX_MEMPOOL_TRANSACTION_AGE: u64 = 600;
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
