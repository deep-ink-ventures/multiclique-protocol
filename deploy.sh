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
 --ledgers-to-expire 6312000

soroban contract bump \
 --source "${SECRET_KEY}" \
 --rpc-url "${RPC_URL}" \
 --network-passphrase "${NETWORK_PASSPHRASE}" \
 --id $CORE_ADDRESS \
 --key-xdr AAAAFA== \
 --durability persistent \
 --ledgers-to-expire 6312000

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
 --wasm wasm/elio_dao_policyl.wasm \
 --durability persistent \
 --ledgers-to-expire 6312000

soroban contract bump \
 --source "${SECRET_KEY}" \
 --rpc-url "${RPC_URL}" \
 --network-passphrase "${NETWORK_PASSPHRASE}" \
 --id $POLICY_ADDRESS \
 --key-xdr AAAAFA== \
 --durability persistent \
 --ledgers-to-expire 6312000

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
    --signers  '["2fd30eab6f5a0f1a3cb83fa2cb70f0262cbc83beec84951c943f697b69463660", "f3bf772b0d713c62bb24b18074994be7dcbaa743ee013bddcf063931c16c2f10"]'
