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
- **Soroban Integration**: CustomAccount Protocol employing Soroban’s __check_auth mechanism for member management and signing thresholds.

## Motivation

Developed by Deep Ink Ventures, MultiClique is inspired by the need for a secure, user-friendly, and flexible solution in the blockchain space. It is an open-source protocol with potential applications beyond DAOs, making it a core infrastructure within the Soroban ecosystem.

## Presets
MultiClique provides a set of preset that have been developed together with the targeted protocols! They aim to provide sensible defaults to interact with the protocol in question. They are ready for use in production or to fork for customization!

Currently we support:

- [Elio DAO](https://github.com/deep-ink-ventures/multiclique-protocol/tree/main/contracts/policies/elio-dao) - a DAO protocol for the Soroban/Stellar network.