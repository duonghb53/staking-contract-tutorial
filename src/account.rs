use crate::*;
use near_sdk::Timestamp;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    pub stake_balance: Balance,
    pub pre_reward: Balance,
    pub last_block_balance_change: Balance,
    pub unstake_balance: Balance,
    pub unstake_start_timestamp: Timestamp,
    pub unstake_avaiable_epoch: EpochHeight,
}

// Timeline: t1 ---------------- t2 -------------- now
// Balance: 100k --------------- 200k ------------
