#!/bin/bash
export MAIN_ACCOUNT=duonghb3.testnet
export NEAR_ENV=testnet
export CONTRACT_STAKING_ID=staking.$MAIN_ACCOUNT
export CONTRACT_FT_ID=ft.$MAIN_ACCOUNT
export ONE_YOCTO=0.000000000000000000000001
export ACCOUNT_TEST=test.duonghb3.testnet

# 1. Build smart contract and deploy
./build.sh

echo "################### DELETE ACCOUNT ###################"
near delete $CONTRACT_FT_ID $MAIN_ACCOUNT
near delete $ACCOUNT_TEST $MAIN_ACCOUNT

echo "################### CREATE ACCOUNT ###################"
near create-account $CONTRACT_FT_ID --masterAccount $MAIN_ACCOUNT --initialBalance 10
near create-account $ACCOUNT_TEST --masterAccount $MAIN_ACCOUNT --initialBalance 10

# 2. Deploy:
near deploy --wasmFile token-test/vbi-ft.wasm --accountId $CONTRACT_FT_ID

# 3. Init contract
near call $CONTRACT_FT_ID new_default_meta '{"owner_id": "'$MAIN_ACCOUNT'", "total_supply": "1000000000000000000000000000000000"}' --accountId $CONTRACT_FT_ID