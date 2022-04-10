use crate::*;
use near_sdk::Timestamp;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    pub stake_balance: Balance,
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub unstake_balance: Balance,
    pub unstake_start_timestamp: Timestamp,
    pub unstake_avaiable_epoch: EpochHeight,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountJson {
    pub account_id: AccountId,
    pub stake_balance: U128,
    pub unstake_balance: U128,
    pub reward: U128,
    pub can_withdraw: bool,
    pub unstake_start_timestamp: Timestamp,
    pub unstake_avaiable_epoch: EpochHeight,
    pub current_epoch: EpochHeight,
}

impl AccountJson {
    pub fn from(account_id: AccountId, new_reward: Balance, account: Account) -> Self {
        AccountJson {
            account_id,
            stake_balance: U128(account.stake_balance),
            unstake_balance: U128(account.unstake_balance),
            unstake_start_timestamp: account.unstake_start_timestamp,
            reward: U128(new_reward),
            can_withdraw: false,
            current_epoch: env::epoch_height(),
            unstake_avaiable_epoch: account.unstake_avaiable_epoch,
        }
    }
}

// Timeline: t1 ---------------- t2 -------------- now
// Balance: 100k --------------- 200k ------------
