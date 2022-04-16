use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, BlockHeight, BorshStorageKey, EpochHeight,
    PanicOnDefault, Promise, PromiseOrValue
};
use near_sdk::log;

mod account;
mod config;
mod enumeration;
mod internal;
mod utils;
mod core_impl;
use crate::account::*;
use crate::config::*;
use crate::enumeration::*;
use crate::internal::*;
use crate::utils::*;
use crate::core_impl::*;

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AccountKey,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct StakingContractV1 {
    pub owner_id: AccountId,
    pub ft_contract_id: AccountId,
    pub config: Config, // Cấu hình công thức trả thưởng cho user
    pub total_stake_balance: Balance,
    pub total_paid_reward_balance: Balance,
    pub total_stacker: Balance,
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub accounts: LookupMap<AccountId, UpgradebleAccount>, // Thong tin chi tiet accout theo AccountId
    pub is_pause: bool,
    pub pause_is_block: BlockHeight,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
pub struct StakingContract {
    pub owner_id: AccountId,
    pub ft_contract_id: AccountId,
    pub config: Config, // Cấu hình công thức trả thưởng cho user
    pub total_stake_balance: Balance,
    pub total_paid_reward_balance: Balance,
    pub total_stacker: Balance,
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub accounts: LookupMap<AccountId, UpgradebleAccount>, // Thong tin chi tiet accout theo AccountId
    pub is_pause: bool,
    pub pause_is_block: BlockHeight,
    pub new_data: U128,
}

#[near_bindgen]
impl StakingContract {
    #[init]
    pub fn new_default_config(owner_id: AccountId, ft_contract_id: AccountId) -> Self {
        Self::new(owner_id, ft_contract_id, Config::default())
    }

    #[init]
    pub fn new(owner_id: AccountId, ft_contract_id: AccountId, config: Config) -> Self {
        StakingContract {
            owner_id,
            ft_contract_id,
            config,
            total_stake_balance: 0,
            total_paid_reward_balance: 0,
            total_stacker: 0,
            pre_reward: 0,
            last_block_balance_change: 0,
            accounts: LookupMap::new(StorageKey::AccountKey),
            is_pause: false,
            pause_is_block: 0,
            new_data: U128(0),
        }
    }

    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
        assert_at_least_one_yocto();
        let account_id = account_id.unwrap_or_else(|| env::predecessor_account_id());
        let account_stake = self.accounts.get(&account_id);
        if account_stake.is_some() {
            // Refund toan bo token deposit
            refund_deposit(0);
        } else {
            // Tao account moi
            let before_storage_usage = env::storage_usage();
            self.internal_register_account(account_id);
            let after_storage_usage = env::storage_usage();
            // Refund token deposit con thua
            refund_deposit(after_storage_usage - before_storage_usage);
        }
    }

    pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {


        let account = self.accounts.get(&account_id);
        if account.is_some() {
            U128(1)
        } else {
            U128(0)
        }
    }

    pub fn is_paused(&self) -> bool {
        self.is_pause
    }

    pub fn get_new_data(&self) -> U128 {
        self.new_data
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let contractV1: StakingContractV1 = env::state_read().expect("Can not read state data");
        StakingContract {
            owner_id: contractV1.owner_id,
            ft_contract_id: contractV1.ft_contract_id,
            config: contractV1.config,
            total_stake_balance: contractV1.total_stake_balance,
            total_paid_reward_balance: contractV1.total_paid_reward_balance,
            total_stacker: contractV1.total_stacker,
            pre_reward: contractV1.pre_reward,
            last_block_balance_change: contractV1.last_block_balance_change,
            accounts: contractV1.accounts,
            is_pause:contractV1.is_pause,
            pause_is_block: contractV1.pause_is_block,
            new_data: U128(10),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);

        builder
    }

    #[test]
    fn test_init_contract() {
        let context = get_context(false);
        testing_env!(context.build());

        let config: Config = Config {
            reward_numerator: 500,
            reward_denumerator: 100_000,
        };

        let contract =
            StakingContract::new(accounts(1).to_string(), "ft_contract".to_string(), config);

        assert_eq!(contract.owner_id, accounts(1).to_string());
        assert_eq!(contract.ft_contract_id, "ft_contract".to_string());
        assert_eq!(config.reward_numerator, contract.config.reward_numerator);
        assert_eq!(contract.is_pause, false);
    }
}
