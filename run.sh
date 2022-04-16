#!/bin/bash
export MAIN_ACCOUNT=duonghb3.testnet
export NEAR_ENV=testnet
export CONTRACT_STAKING_ID=staking.$MAIN_ACCOUNT
export CONTRACT_FT_ID=ft.$MAIN_ACCOUNT
export ONE_YOCTO=0.000000000000000000000001
export ACCOUNT_TEST=test.duonghb3.testnet
export GAS=300000000000000
export AMOUNT=100000000000000000000000000


# 1. Register account to ft contract
near call $CONTRACT_FT_ID storage_deposit '{"account_id": "'$CONTRACT_STAKING_ID'"}' --accountId $MAIN_ACCOUNT --deposit 0.01
near call $CONTRACT_FT_ID storage_deposit '{"account_id": "'$ACCOUNT_TEST'"}' --accountId $MAIN_ACCOUNT --deposit 0.01
near call $CONTRACT_FT_ID storage_deposit '{"account_id": "'$MAIN_ACCOUNT'"}' --accountId $MAIN_ACCOUNT --deposit 0.01

# 2. Add account to storage staking
near call $CONTRACT_STAKING_ID storage_deposit '{"account_id": "'$ACCOUNT_TEST'"}' --accountId $ACCOUNT_TEST --deposit 0.01
near call $CONTRACT_STAKING_ID storage_deposit '{"account_id": "'$MAIN_ACCOUNT'"}' --accountId $MAIN_ACCOUNT --deposit 0.01

# 3. Deposit ft token to account
near call $CONTRACT_FT_ID ft_transfer '{"receiver_id": "'$ACCOUNT_TEST'", "amount": "'$AMOUNT'"}' --accountId $MAIN_ACCOUNT --amount 0.000000000000000000000001
near call $CONTRACT_FT_ID ft_transfer '{"receiver_id": "'$MAIN_ACCOUNT'", "amount": "'$AMOUNT'"}' --accountId $ACCOUNT_TEST --amount 0.000000000000000000000001
# ??? Tại sao accountId lúc gọi không phải là $CONTRACT_FT_ID vẫn có thể transfer được token vào $ACCOUNT_TEST? Vậy ai cũng có thể transfer token được à?
# Vì đang dùng cho testnet đúng không?

# 4. View balance of account
near view $CONTRACT_FT_ID ft_balance_of '{"account_id": "'$CONTRACT_STAKING_ID'"}'
near view $CONTRACT_FT_ID ft_balance_of '{"account_id": "'$MAIN_ACCOUNT'"}'
near view $CONTRACT_FT_ID ft_balance_of '{"account_id": "'$ACCOUNT_TEST'"}'

# 5. Staking ft token to pool
near call $CONTRACT_FT_ID ft_transfer_call '{"receiver_id": "'$CONTRACT_STAKING_ID'", "amount": "'$AMOUNT'", "msg": ""}' --accountId $MAIN_ACCOUNT --deposit $ONE_YOCTO --gas $GAS
near call $CONTRACT_FT_ID ft_transfer_call '{"receiver_id": "'$CONTRACT_STAKING_ID'", "amount": "10000000000000000000000000", "msg": ""}' --accountId $ACCOUNT_TEST --deposit $ONE_YOCTO --gas $GAS
near call $CONTRACT_FT_ID ft_transfer_call '{"receiver_id": "'$CONTRACT_STAKING_ID'", "amount": "50000000000000000000000000", "msg": ""}' --accountId $ACCOUNT_TEST --deposit $ONE_YOCTO --gas $GAS

# 6. Get account info
near view $CONTRACT_STAKING_ID get_account_info '{"account_id": "'$ACCOUNT_TEST'"}'
near view $CONTRACT_STAKING_ID get_account_info '{"account_id": "'$MAIN_ACCOUNT'"}'
near view $CONTRACT_STAKING_ID get_account_info '{"account_id": "'$CONTRACT_STAKING_ID'"}'


# 6. Get pool info
near view $CONTRACT_STAKING_ID get_pool_info

# 7. Harvest all reward
near call $CONTRACT_STAKING_ID harvest --accountId $ACCOUNT_TEST --deposit $ONE_YOCTO --gas $GAS


# 8. Unstake
near call $CONTRACT_STAKING_ID unstake '{"amount": "10000000000000000000000000"}' --accountId $ACCOUNT_TEST --deposit $ONE_YOCTO

# 9. Withdraw
near call $CONTRACT_STAKING_ID withdraw '' --accountId $ACCOUNT_TEST --deposit $ONE_YOCTO --gas $GAS
