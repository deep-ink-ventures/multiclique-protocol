#!/bin/bash

source .env

printf "\nDeploying core ...\n"
CORE_ADDRESS="$(
soroban contract deploy \
    --wasm wasm/multiclique.wasm \
    --source "${SECRET_KEY}" \
    --rpc-url "${RPC_URL}" \
    --network-passphrase "${NETWORK_PASSPHRASE}"
)"
export CORE_ADDRESS

soroban contract bump \
 --source "${SECRET_KEY}" \
 --rpc-url "${RPC_URL}" \
 --network-passphrase "${NETWORK_PASSPHRASE}" \
 --wasm wasm/multiclique.wasm \
 --durability persistent \
 --ledgers-to-expire 200000

soroban contract bump \
 --source "${SECRET_KEY}" \
 --rpc-url "${RPC_URL}" \
 --network-passphrase "${NETWORK_PASSPHRASE}" \
 --id "$CORE_ADDRESS" \
 --key-xdr "AAAAFA==" \
 --durability persistent \
 --ledgers-to-expire 200000

POLICY_ADDRESS="$(
soroban contract deploy \
    --wasm wasm/elio_dao_policy.wasm \
    --source "${SECRET_KEY}" \
    --rpc-url "${RPC_URL}" \
    --network-passphrase "${NETWORK_PASSPHRASE}"
)"
export POLICY_ADDRESS

soroban contract bump \
 --source "${SECRET_KEY}" \
 --rpc-url "${RPC_URL}" \
 --network-passphrase "${NETWORK_PASSPHRASE}" \
 --wasm wasm/elio_dao_policy.wasm \
 --durability persistent \
 --ledgers-to-expire 200000

soroban contract bump \
 --source "${SECRET_KEY}" \
 --rpc-url "${RPC_URL}" \
 --network-passphrase "${NETWORK_PASSPHRASE}" \
 --id "$POLICY_ADDRESS" \
 --key-xdr "AAAAFA==" \
 --durability persistent \
 --ledgers-to-expire 200000

# signer 1 (test purposes only)
#GAGB4TVOQWTLK5Y4AVZNKQVOMU2RBTD42MOYROMNV3E64Y6O6LLPYUB2
#SAX5GDVLN5NA6GR4XA72FS3Q6ATCZPEDX3WIJFI4SQ7WS63JIY3GAM3D
#base viable spin remove buffalo grid invite water crawl rival frost biology

# signer 2 (test purposes only)
#GAPXNWJTQVF3WLPCH3KI2DVKYDMVZR4KJHPMV5GXP6WAK6ZOEVPZPICO
#SDZ365ZLBVYTYYV3ESYYA5EZJPT5ZOVHIPXACO65Z4DDSMOBNQXRALZR
#state mention miracle fame infant lens total health item electric diary drink

printf "\nInitialising core ...\n"
soroban contract invoke \
    --id "${CORE_ADDRESS}" \
    --source "${SECRET_KEY}" \
    --rpc-url "${RPC_URL}" \
    --network-passphrase "${NETWORK_PASSPHRASE}" \
    -- \
    init \
    --default_threshold 1 \
    --signers  '["0c1e4eae85a6b5771c0572d542ae653510cc7cd31d88b98daec9ee63cef2d6fc", "1f76d933854bbb2de23ed48d0eaac0d95cc78a49decaf4d77fac057b2e255f97"]'

printf "CORE_ADDRESS=%s\n" "$CORE_ADDRESS"
printf "POLICY_ADDRESS=%s\n" "$POLICY_ADDRESS"