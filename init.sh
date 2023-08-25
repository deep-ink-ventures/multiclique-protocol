#!/bin/sh

DIR="$(dirname "$0")"

PROFILE="release";

mkdir -p "${DIR}"/wasm/

printf "> Compiling multiclique...\n"
cargo build -p multiclique --target wasm32-unknown-unknown --profile "${PROFILE}" &&
		cp "${DIR}"/target/wasm32-unknown-unknown/"${PROFILE}"/multiclique.wasm "${DIR}"/wasm/

printf "> Compiling elio-dao0policy...\n"
cargo build -p elio-dao-policy --target wasm32-unknown-unknown --profile "${PROFILE}" &&
	  cp "${DIR}"/target/wasm32-unknown-unknown/"${PROFILE}"/elio_dao_policy.wasm "${DIR}"/wasm/
