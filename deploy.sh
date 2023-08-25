#!/bin/bash

source .env

printf "\nDeploying multiclique ...\n"
CORE_ADDRESS="$(
soroban contract deploy \
    --wasm wasm/multiclique.wasm \
    --source "${SECRET_KEY}" \
    --rpc-url "${RPC_URL}" \
    --network-passphrase "${NETWORK_PASSPHRASE}"
)"
export CORE_ADDRESS
printf $CORE_ADDRESS

printf "\nInitialising core ...\n"
soroban contract invoke \
    --id "${CORE_ADDRESS}" \
    --source "${SECRET_KEY}" \
    --rpc-url "${RPC_URL}" \
    --network-passphrase "${NETWORK_PASSPHRASE}" \
    -- \
    init \
    --default_threshold 2 \
    --signers "[GALS3QDUTVWEGEB57BBMUPO5LVI4GGNEW6FWMXYCWBDOY2WZYWPHEJ7I, GALS3QDUTVWEGEB57BBMUPO5LVI4GGNEW6FWMXYCWBDOY2WZYWPHEJ7I]"

# deploy policy
# spend limit initialisen