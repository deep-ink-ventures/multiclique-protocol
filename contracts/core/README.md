# MultiClique Core Module

[![Stellar](https://img.shields.io/badge/Stellar-Compatible-brightgreen)](https://www.stellar.org/)
[![Rust](https://img.shields.io/badge/Rust-1.73.0-blue.svg)](https://www.rust-lang.org)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-yellow.svg)](https://opensource.org/licenses/Apache-2.0)

## Introduction
MultiClique is a protocol designed for the Soroban/Stellar network, enhancing security and introducing customizable transaction policies. This core module provides a multi-signature mechanism and policy management for digital assets, offering both convenience and robust security.

## Features
- Multi-Signature Mechanism: Ensures that transactions are authenticated by multiple signers, mitigating the risk of fund theft.
- Customizable Policies: Allows the creation of flexible and extendable policies for managing members and signing thresholds.
- Default Actions: Can be run out of the box without policies to manage varying thresholds in soroban

## Contract Interface

The core module exposes a trait MultiCliqueTrait defining the main functionalities:

- Initialization with default threshold and signers.
- Managing signers (addition/removal).
- Setting and retrieving default thresholds.
- Attaching and detaching policies.
- Retrieving policies for a specific context.
- Authenticating signatures with custom and default thresholds.