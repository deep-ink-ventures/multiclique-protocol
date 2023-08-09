#!/bin/sh

DIR="$(dirname "$0")"

PROFILE="release";

mkdir -p "${DIR}"/wasm/

for CRATE in policy core; do
	printf "> Compiling ${CRATE} contract...\n"
	cargo build -p multiclique-${CRATE} --target wasm32-unknown-unknown --profile "${PROFILE}" &&
		cp "${DIR}"/target/wasm32-unknown-unknown/"${PROFILE}"/multiclique_${CRATE}.wasm "${DIR}"/wasm/
done