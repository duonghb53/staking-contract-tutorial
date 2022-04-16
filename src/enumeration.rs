use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PoolJson {
    pub total_stake_balance: U128,
    pub total_reward: U128,
    pub total_paid_reward_balance: U128,
    pub total_stakers: U128,
    pub is_paused: bool,
}

#[near_bindgen]
impl StakingContract {
    pub fn get_account_info(&self, account_id: AccountId) -> AccountJson {
        let upgradable_account = self.accounts.get(&account_id);
        assert!(upgradable_account.is_some(), "ERR_ACCOUNT_ID_NOT_FOUND");

        let account = Account::from(upgradable_account.unwrap());
        let new_reward = self.internal_calculate_account_reward(&account);
        AccountJson::from(account_id, new_reward, account)
    }

    pub fn get_account_reward(&self, account_id: AccountId) -> Balance {
        let upgradeable_account = self.accounts.get(&account_id).unwrap();
        let account = Account::from(upgradeable_account);
        let new_reward = self.internal_calculate_account_reward(&account);

        account.pre_reward + new_reward
    }

    pub fn get_pool_info(&self) -> PoolJson {
        PoolJson {
            total_stake_balance: U128(self.total_stake_balance),
            total_reward: U128(self.pre_reward + self.internal_calculate_global_reward()),
            total_paid_reward_balance: U128(self.total_paid_reward_balance),
            total_stakers: U128(self.total_stacker),
            is_paused: self.is_pause,
        }
    }
}
