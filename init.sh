#!/bin/sh

DIR="$(dirname "$0")"

PROFILE="release";

mkdir -p "${DIR}"/wasm/

printf "> Compiling multiclique...\n"
cargo build -p multiclique --target wasm32-unknown-unknown --profile "${PROFILE}" &&
		cp "${DIR}"/target/wasm32-unknown-unknown/"${PROFILE}"/multiclique.wasm "${DIR}"/wasm/

for POLICY in "elio-dao"; do
	printf "> Compiling ${POLICY} policy...\n"
	cargo build -p ${POLICY}-policy --target wasm32-unknown-unknown --profile "${PROFILE}" &&
	  cp "${DIR}"/target/wasm32-unknown-unknown/"${PROFILE}"/${POLICY//-/_}_policy.wasm "${DIR}"/wasm/
done