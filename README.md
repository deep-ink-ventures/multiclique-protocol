# MultiClique Policy Framework

[![Stellar](https://img.shields.io/badge/Stellar-Compatible-brightgreen)](https://www.stellar.org/)
[![Rust](https://img.shields.io/badge/Rust-1.73.0-blue.svg)](https://www.rust-lang.org)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-yellow.svg)](https://opensource.org/licenses/Apache-2.0)

## Introduction

MultiClique is a flexible policy framework and multisig wallet designed for the Soroban/Stellar network. It aims to enhance blockchain adoption by addressing challenges such as complexity, risk of fund loss, and technical difficulties in managing wallets and key pairs. MultiClique is not only a protocol but a complete application stack with a user interface that simplifies the usage of multi-signature policies.

## Features

- **Multi-Signature Security**: Enhances security through a multi-signature mechanism, reducing the risk of fund theft.
- **Customizable Policies**: Introduces flexibility by allowing customizable policies for executing transactions.
- **User-Friendly Interface**: Provides an intuitive interface for managing digital assets and interacting with Soroban.
- **Council Management Support**: Ideal for DAOs and other organizational groups requiring advanced workflows.
- **Policy Extensibility**: The core policy can be extended through hook points, making the policy system adaptable.
- **Soroban Integration**: CustomAccount Protocol employing Soroban’s `__check_auth` mechanism for member management and signing thresholds.

## Motivation

Developed by Deep Ink Ventures, MultiClique is inspired by the need for a secure, user-friendly, and flexible solution in the blockchain space. It is an open-source protocol with potential applications beyond DAOs, making it a core infrastructure within the Soroban ecosystem.

## Presets
MultiClique provides a set of preset that have been developed together with the targeted protocols! They aim to provide sensible defaults to interact with the protocol in question. They are ready for use in production or to fork for customization!

Currently we support:

- [Elio DAO](https://github.com/deep-ink-ventures/multiclique-protocol/tree/main/contracts/policies/elio-dao) - a DAO protocol for the Soroban/Stellar network.

## Getting started with development

We assume that you have `soroban` and the [Rust Toolchain](https://www.rust-lang.org/) installed and configured. If not, please follow the [soroban-cli installation guide](https://soroban.stellar.org/docs/getting-started/setup).

Next up, create a `.env` file in the root of the project:

```shell
cp .env.example .env
```

Head over to [the lab](https://laboratory.stellar.org/#account-creator?network=futurenet) and create a public/private keypair and fund it.

Now, add both keys to the `.env` file:

```shell
PUBLIC_KEY=REPLACE_ME
SECRET_KEY=REPLACE_ME
```

That's it. You can now run the test suite:

```shell
cargo test
```

You can now start developing your own policy presets and share it with a community!

## Creating your own policy preset
`cd` into `./contracts/policies` and run `cargo new --lib my-policy`.

Give your package a nice name and make it a member of our workspace by adding your crate to the projects root cargo toml.

```shell
members = [
    "contracts/core",
    "contracts/policies/elio-dao",
    "contracts/policies/my-policy",
]
```

Replace the toml file of your preset with this content:

```toml
[package]
name = "my-policy"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
soroban-sdk = { workspace = true }
commons = { workspace = true}

[dev_dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }
ed25519-dalek = { version = "1.0.1" }
rand = { version = "0.7.3" }

[features]
testutils = ["soroban-sdk/testutils"]
```

Now open `./src/lib.rs` and replace the content with this:

```rust
use soroban_sdk::{Address, BytesN, Env, Symbol, Val};
use commons::traits::MultiCliquePolicyTrait;

#[contract]
pub struct Contract;


#[contractimpl]
impl MultiCliquePolicyTrait for Contract {
    fn get_threshold(
        env: Env,
        num_signers: u32,
        signers: Vec<BytesN<32>>,
        address: Address,
        fn_name: Symbol,
        args: Vec<Val>,
    ) -> u32 {
        num_signers
    }

    fn run_policy(
        env: Env,
        num_signers: u32,
        signers: Vec<BytesN<32>>,
        address: Address,
        fn_name: Symbol,
        args: Vec<Val>,
    ) {
        // Do nothing
    }
}
```

That's it. You can now start implementing custom logic for your policy!

Once you're done add a nice `README.md`, a *testsuite* and proper *documentation* and you're ready to use your policy.

Look at our sample `deploy.sh` on how to attach a policy to a multiclique account.

If it's helpful for the community we are happy to merge a PR into the main repository!