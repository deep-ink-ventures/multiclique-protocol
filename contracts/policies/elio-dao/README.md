# Elio DAO Policy - MultiClique

[![Stellar](https://img.shields.io/badge/Stellar-Compatible-brightgreen)](https://www.stellar.org/)
[![Rust](https://img.shields.io/badge/Rust-1.73.0-blue.svg)](https://www.rust-lang.org)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-yellow.svg)](https://opensource.org/licenses/Apache-2.0)

## Introduction

Elio DAO Policy is a predefined policy preset within the MultiClique framework, specifically tailored for Elio DAO. It is designed to provide sensible defaults to interact with the Elio DAO protocol, offering both readiness for production use and flexibility for customization.

## Features

- **Ready for Production**: Developed in conjunction with the targeted Elio DAO protocol for a smooth integration.
- **Sensible Defaults**: Provides a set of reasonable default configurations for immediate use or further customization.
- **Multi-Signature Security**: Inherits MultiClique's multi-signature mechanism, ensuring enhanced security.
- **Flexible and Extendable**: Can be easily forked and customized to fit specific needs.
- **Integrated with MultiClique**: Fully compatible with MultiClique's flexible policy framework and multisig wallet.

## Thresholds and Functionality

Elio DAO Policy utilizes different thresholds to govern the execution of various functions within the contracts. Here's a breakdown of those thresholds:

### Core Functions

- **`destroy_dao` & `change_owner`**: Requires 80% of the signers. Used to perform critical administrative tasks.
- **Other Core Functions**: Requires 66% of the signers. Used for general core management.

### Vote Functions

- **`fault_proposal`**: Requires only 1 signer. Used to mark a proposal as faulty.
- **`mark_implemented`**: Requires 50% of the signers. Used to mark a proposal as implemented.
- **Other Vote Functions**: Requires 66% of the signers. Used for general vote management.

### Asset Functions

- **`set_owner` & `set_core_address`**: Requires 80% of the signers. Used for critical asset management.
- **Other Asset Functions**: Requires 50% of the signers. Used for general asset management.

### Spend Limit Policy

- **Spend Limit Execution**: DAOs can define a spend limit policy on their treasury on a per asset basis with their default threshold. Those send offs require only 50% of signers.
## Customizing Thresholds

These thresholds are part of the preset but can be customized to fit specific needs. You can modify them within the code as per your governance model or specific requirements. Details on customization can be found within the code comments.

## Security

The thresholds are carefully designed to balance convenience, flexibility, and security. Critical functions require a higher consensus among signers, ensuring robust protection against unauthorized access or malicious actions.

For further information on threshold settings and customization, please refer to the in-depth documentation available within the codebase.
