# Seismic Alloy

This repository contains Seismic's fork of Alloy

The upstream repository lives [here](https://github.com/alloy-rs/alloy). This fork is up-to-date with it through commit `de01884`. You can see this by viewing the [main](https://github.com/SeismicSystems/seismic-alloy/tree/main) branch on this repository

You can view all of our changes vs. upstream on this [pull request](https://github.com/SeismicSystems/seismic-alloy/pull/2). The sole purpose of this PR is display our diff; it will never be merged in to the main branch of this repo

## Main Changes

### Seismic Transaction Type

This new EIP-2718 transaction type (`0x4a` or `74`) introduces additional fields that Seismic uses to secure its blockchain.

#### Fields

- **encryption_pubkey**  
  Represents the EOA's ephemerally generated public key. **Note:** This is _not_ the public key associated with the Ethereum address. When a Seismic transaction is sent to the chain, its calldata is encrypted using a shared secret derived from the network's key and the ephemeral key. This shared secret is included in the transaction so the network can decrypt the EOA's calldata (located in the `input` field, as with other transactions).

- **message_version**  
  Determines the method used to send the transaction. Seismic currently supports two approaches:

  1. **Standard Method:**  
     The transaction is signed using `signTransaction` and sent as raw transaction bytes (indicated by `0`).

  2. **EIP-712 Typed Data:**  
     The transaction is sent as EIP-712 signed typed data (indicated by `2`).

  > **Note:**  
  > We added support for EIP-712 because browser extension wallets couldn’t sign Seismic transactions using the traditional method. This support might be removed in the future. The value `1` is reserved for supporting transactions signed via `personal_sign` (for example, by hardware wallets).

### New Enums for Seismic RPC Extensions

Seismic extends Ethereum's RPC methods by introducing two new enums:

- `SeismicCallRequest` for `eth_call`
- `SeismicRawTxRequest` for `eth_sendRawTransaction`

#### SeismicCallRequest

On Seismic, you can perform an `eth_call` in two ways:

- **Standard Call:**  
  Submit a transaction request normally. However, if you set the `from` field, it will be overridden to the zero address to prevent users from making calls from addresses they do not own.

- **Signed Call:**  
  Submit a transaction request accompanied by a signature. In this case, the `from` field is populated with the signer's address and passed to smart contracts, ensuring that `msg.sender` cannot be spoofed. A signed call can be made using either:

  - A raw transaction payload (e.g., bytes)
  - EIP-712 signed typed data (to support browser wallets)

#### SeismicRawTxRequest

For sending a raw transaction on Seismic, you have two options:

- **Standard Method:**  
  Use raw transaction bytes.
- **EIP-712 Method:**  
  Send the transaction using EIP-712 signed typed data, as discussed in the `message_version` section.

### New Provider for Shielded Transaction

- When a TxSeismic transaction is created, we:
  1. Generate an ephemeral key pair
  2. Use the ephemeral private key and network's public key to generate a shared secret via ECDH
  3. Use the shared secret to encrypt the transaction's calldata
  4. Include the ephemeral public key in the transaction so the network can decrypt the calldata
- Support for decrypting `eth_call` output. When a signed `eth_call` is made, the network encrypts the output using the ephemeral public key provided in the request. The client can then decrypt this output using the ephemeral private key it generated
- Please see `create_seismic_provider` for detailed provider configuration for shielded transaction.

## Structure

Seismic's forks of the [reth](https://github.com/paradigmxyz/reth) stack all have the same branch structure:

- `main` or `master`: this branch only consists of commits from the upstream repository. However it will rarely be up-to-date with upstream. The latest commit from this branch reflects how recently Seismic has merged in upstream commits to the seismic branch
- `seismic`: the default and production branch for these repositories. This includes all Seismic-specific code essential to make our network run

## Overview

This repository contains the following crates:

- [`alloy`]: Meta-crate for the entire project, including [`alloy-core`]
- [`alloy-consensus`] - Ethereum consensus interface
  - [`alloy-consensus-any`] - Catch-all consensus interface for multiple networks
- [`alloy-contract`] - Interact with on-chain contracts
- [`alloy-eips`] - Ethereum Improvement Proposal (EIP) implementations
- [`alloy-genesis`] - Ethereum genesis file definitions
- [`alloy-json-rpc`] - Core data types for JSON-RPC 2.0 clients
- [`alloy-network`] - Network abstraction for RPC types
  - [`alloy-network-primitives`] - Primitive types for the network abstraction
- [`alloy-node-bindings`] - Ethereum execution-layer client bindings
- [`alloy-provider`] - Interface with an Ethereum blockchain
- [`alloy-pubsub`] - Ethereum JSON-RPC [publish-subscribe] tower service and type definitions
- [`alloy-rpc-client`] - Low-level Ethereum JSON-RPC client implementation
- [`alloy-rpc-types`] - Meta-crate for all Ethereum JSON-RPC types
  - [`alloy-rpc-types-admin`] - Types for the `admin` Ethereum JSON-RPC namespace
  - [`alloy-rpc-types-anvil`] - Types for the [seismic-anvil] development node's Ethereum JSON-RPC namespace
  - [`alloy-rpc-types-any`] - Types for JSON-RPC namespaces across multiple networks
  - [`alloy-rpc-types-beacon`] - Types for the [Ethereum Beacon Node API][beacon-apis]
  - [`alloy-rpc-types-debug`] - Types for the `debug` Ethereum JSON-RPC namespace
  - [`alloy-rpc-types-engine`] - Types for the `engine` Ethereum JSON-RPC namespace
  - [`alloy-rpc-types-eth`] - Types for the `eth` Ethereum JSON-RPC namespace
  - [`alloy-rpc-types-mev`] - Types for the MEV bundle JSON-RPC namespace
  - [`alloy-rpc-types-trace`] - Types for the `trace` Ethereum JSON-RPC namespace
  - [`alloy-rpc-types-txpool`] - Types for the `txpool` Ethereum JSON-RPC namespace
- [`alloy-serde`] - [Serde]-related utilities
- [`alloy-signer`] - Ethereum signer abstraction
  - [`alloy-signer-aws`] - [AWS KMS] signer implementation
  - [`alloy-signer-gcp`] - [GCP KMS] signer implementation
  - [`alloy-signer-ledger`] - [Ledger] signer implementation
  - [`alloy-signer-local`] - Local (private key, keystore, mnemonic, YubiHSM) signer implementations
  - [`alloy-signer-trezor`] - [Trezor] signer implementation
- [`alloy-transport`] - Low-level Ethereum JSON-RPC transport abstraction
  - [`alloy-transport-http`] - HTTP transport implementation
  - [`alloy-transport-ipc`] - IPC transport implementation
  - [`alloy-transport-ws`] - WS transport implementation

[`alloy`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/alloy
[`alloy-core`]: https://docs.rs/alloy-core
[`alloy-consensus`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/consensus
[`alloy-consensus-any`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/consensus-any
[`alloy-contract`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/contract
[`alloy-eips`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/eips
[`alloy-genesis`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/genesis
[`alloy-json-rpc`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/json-rpc
[`alloy-network`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/network
[`alloy-network-primitives`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/network-primitives
[`alloy-node-bindings`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/node-bindings
[`alloy-provider`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/provider
[`alloy-pubsub`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/pubsub
[`alloy-rpc-client`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/rpc-client
[`alloy-rpc-types`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/rpc-types
[`alloy-rpc-types-admin`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/rpc-types-admin
[`alloy-rpc-types-anvil`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/rpc-types-anvil
[`alloy-rpc-types-any`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/rpc-types-any
[`alloy-rpc-types-beacon`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/rpc-types-beacon
[`alloy-rpc-types-debug`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/rpc-types-debug
[`alloy-rpc-types-engine`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/rpc-types-engine
[`alloy-rpc-types-eth`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/rpc-types-eth
[`alloy-rpc-types-mev`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/rpc-types-mev
[`alloy-rpc-types-trace`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/rpc-types-trace
[`alloy-rpc-types-txpool`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/rpc-types-txpool
[`alloy-serde`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/serde
[`alloy-signer`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/signer
[`alloy-signer-aws`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/signer-aws
[`alloy-signer-gcp`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/signer-gcp
[`alloy-signer-ledger`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/signer-ledger
[`alloy-signer-local`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/signer-local
[`alloy-signer-trezor`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/signer-trezor
[`alloy-transport`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/transport
[`alloy-transport-http`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/transport-http
[`alloy-transport-ipc`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/transport-ipc
[`alloy-transport-ws`]: https://github.com/SeismicSystems/seismic-alloy/tree/seismic/crates/transport-ws
[publish-subscribe]: https://en.wikipedia.org/wiki/Publish%E2%80%93subscribe_pattern
[AWS KMS]: https://aws.amazon.com/kms
[GCP KMS]: https://cloud.google.com/kms
[Ledger]: https://www.ledger.com
[Trezor]: https://trezor.io
[Serde]: https://serde.rs
[beacon-apis]: https://ethereum.github.io/beacon-APIs
[seismic-anvil]: https://github.com/SeismicSystems/seismic-foundry

## Credits

None of these crates would have been possible without the great work done in:

- [`ethers.js`](https://github.com/ethers-io/ethers.js/)
- [`rust-web3`](https://github.com/tomusdrw/rust-web3/)
- [`ruint`](https://github.com/recmo/uint)
- [`ethabi`](https://github.com/rust-ethereum/ethabi)
- [`ethcontract-rs`](https://github.com/gnosis/ethcontract-rs/)
- [`guac_rs`](https://github.com/althea-net/guac_rs/)
- and of couse [`alloy`](https://github.com/alloy-rs/alloy/)

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in these crates by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
