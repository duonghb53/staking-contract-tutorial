use crate::*;

impl StakingContract {
    pub(crate) fn internal_register_account(&mut self, account_id: AccountId) {
        let account = Account {
            stake_balance: 0,
            pre_reward: 0,
            last_block_balance_change: env::block_index(),
            unstake_balance: 0,
            unstake_start_timestamp: 0,
            unstake_avaiable_epoch: 0,
            new_account_data: U128(0),
        };

        self.accounts
            .insert(&account_id, &UpgradebleAccount::from(account));
    }

    pub(crate) fn internal_calculate_account_reward(&self, account: &Account) -> Balance {
        let last_block = if self.is_pause {
            self.pause_is_block
        } else {
            env::block_index()
        };

        let diff_block = last_block - account.last_block_balance_change;
        let reward: Balance =
            (account.stake_balance * self.config.reward_numerator as u128 * diff_block as u128)
                / self.config.reward_denumerator as u128;
        reward
    }

    pub(crate) fn internal_calculate_global_reward(&self) -> Balance {
        let last_block = if self.is_pause {
            self.pause_is_block
        } else {
            env::block_index()
        };

        //log!("last_block ==> {}", last_block);

        let diff_block = last_block - self.last_block_balance_change;
        let reward: Balance =
            (self.total_stack_balance * self.config.reward_numerator as u128 * diff_block as u128)
                / self.config.reward_denumerator as u128;
        reward
    }

    pub(crate) fn internal_deposit_and_stake(&mut self, account_id: AccountId, amount: U128) {
        // Validate data
        let upgradable_account = self.accounts.get(&account_id);
        assert!(upgradable_account.is_some(), "ERR_ACCOUNT_ID_NOT_FOUND");
        assert_eq!(self.is_paused(), false, "ERR_CONTRACT_PAUSE");
        assert_eq!(
            self.ft_contract_id,
            env::predecessor_account_id(),
            "ERR_INVALID_FT_CONTRACT_ID"
        );

        let mut account = Account::from(upgradable_account.unwrap());
        if account.stake_balance == 0 {
            self.total_stacker += 1;
        }
        let new_reward = self.internal_calculate_account_reward(&account);

        // Update Account Data
        account.pre_reward += new_reward;
        account.stake_balance += amount.0;
        account.last_block_balance_change = env::block_index();

        self.accounts
            .insert(&account_id, &UpgradebleAccount::from(account));

        // Update Pool Data
        let new_contract_reward = self.internal_calculate_global_reward();
        self.total_stack_balance += amount.0;
        self.pre_reward += new_contract_reward;
        self.last_block_balance_change = env::block_index();
    }
}


