#!/bin/bash

source .env

printf "\nInstalling core ...\n"
MULTICLIQUE_WASM_HASH="$(
soroban contract install \
    --wasm wasm/multiclique.wasm \
    --source "${SECRET_KEY}" \
    --rpc-url "${RPC_URL}" \
    --network-passphrase "${NETWORK_PASSPHRASE}"
)"
export MULTICLIQUE_WASM_HASH

soroban contract bump \
 --source "${SECRET_KEY}" \
 --rpc-url "${RPC_URL}" \
 --network-passphrase "${NETWORK_PASSPHRASE}" \
 --wasm wasm/multiclique.wasm \
 --durability persistent \
 --ledgers-to-expire 200000


printf "\nInstalling policy ...\n"
POLICY_WASM_HASH="$(
soroban contract install \
    --wasm wasm/elio_dao_policy.wasm \
    --source "${SECRET_KEY}" \
    --rpc-url "${RPC_URL}" \
    --network-passphrase "${NETWORK_PASSPHRASE}"
)"
export POLICY_WASM_HASH

soroban contract bump \
 --source "${SECRET_KEY}" \
 --rpc-url "${RPC_URL}" \
 --network-passphrase "${NETWORK_PASSPHRASE}" \
 --wasm wasm/elio_dao_policy.wasm \
 --durability persistent \
 --ledgers-to-expire 200000


if [[ -n "${SERVICE_URL}" ]];
then
printf "\nUpdating Service ...\n"
curl -XPATCH -H "Config-Secret: ${CONFIG_SECRET}" -H "Content-type: application/json" -d "{
  \"multiclique_wasm_hash\": \"${MULTICLIQUE_WASM_HASH}\",
  \"policy_wasm_hash\": \"${POLICY_WASM_HASH}\"
}" "${SERVICE_URL}/update-config/"
fi

printf "MULTICLIQUE_WASM_HASH=%s\n" "$MULTICLIQUE_WASM_HASH"
printf "POLICY_WASM_HASH=%s\n" "$POLICY_WASM_HASH"