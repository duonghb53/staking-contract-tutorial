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

        self.accounts.insert(&account_id, &UpgradebleAccount::from(account));
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

        let diff_block = last_block - self.last_block_balance_change;
        let reward: Balance =
            (self.total_stack_balance * self.config.reward_numerator as u128 * diff_block as u128)
                / self.config.reward_denumerator as u128;
        reward
    }
}
