use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    pub reward_numerator: u32,
    pub reward_denumerator: u32,
}

// APR 15% = (token_stacking * 15/100) * total_block
// Moi block se tra thuong 715 / 1_000_000_000 --> tinh ra duoc thanh APR 15%

impl Default for Config {
    fn default() -> Self {
        Self {
            reward_numerator: 715,
            reward_denumerator: 1_000_000_000,
        }
    }
}
