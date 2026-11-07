<!-- source: https://docs.midnight.network/llms.txt fetched_at: 2026-11-07T20:47:59Z status: 200 untrusted -->

# Midnight Docs

> Comprehensive documentation for building on Midnight.

- [Midnight Docs](https://docs.midnight.network/index.md)

## what-is-midnight

Introduces Midnight, a data protection blockchain, covering public and private state, selective disclosure, zero-knowledge proofs, Compact, and use cases.

- [What is Midnight?](https://docs.midnight.network/what-is-midnight.md): Introduces Midnight, a data protection blockchain, covering public and private state, selective disclosure, zero-knowledge proofs, Compact, and use cases.

## getting-started

Start building on Midnight.

- [Get started](https://docs.midnight.network/getting-started.md): Start building on Midnight.

### hello-world

Check that your environment is ready for Midnight: write a Hello World Compact contract, compile it, and test it on a local network.

- [Create your first Midnight contract](https://docs.midnight.network/getting-started/hello-world.md): Check that your environment is ready for Midnight: write a Hello World Compact contract, compile it, and test it on a local network.

### installation

Get your Midnight development environment running to set up tools, configure wallets, and deploy your first zero-knowledge application.

- [Install the toolchain](https://docs.midnight.network/getting-started/installation.md): Get your Midnight development environment running to set up tools, configure wallets, and deploy your first zero-knowledge application.

## tutorials

Step-by-step tutorials for building DApps on Midnight Network, from your first Compact contract to a full-stack deployment with wallet integration.

- [Tutorials](https://docs.midnight.network/tutorials.md): Step-by-step tutorials for building DApps on Midnight Network, from your first Compact contract to a full-stack deployment with wallet integration.

### battleship

Defensive programming in Compact: a two-player game contract that uses a state machine, role checks, and on-chain verification to catch cheating.

- [Battleship tutorial](https://docs.midnight.network/tutorials/battleship.md): Defensive programming in Compact: a two-player game contract that uses a state machine, role checks, and on-chain verification to catch cheating.
- [Battleship contract](https://docs.midnight.network/tutorials/battleship/smart-contract.md): Read a Compact contract written defensively: a state machine, role checks, and hash commitments that verify each player's private answers on chain.
- [Battleship tests](https://docs.midnight.network/tutorials/battleship/test-suite.md): Play Battleship on a local devnet with MidnightJS, then try to cheat and prove the contract rejects every attempt.

### leaderboard

Build a full-stack leaderboard DApp with privacy controls, Lace wallet integration, a React frontend, and Vercel deployment.

- [Leaderboard tutorial](https://docs.midnight.network/tutorials/leaderboard.md): Build a full-stack leaderboard DApp with privacy controls, Lace wallet integration, a React frontend, and Vercel deployment.
- [Part 2: TypeScript integration](https://docs.midnight.network/tutorials/leaderboard/api-layer.md): Set up the TypeScript witness provider, contract exports, and a shared API class for deploying and interacting with the leaderboard DApp on Midnight.
- [Part 3: Browser DApp](https://docs.midnight.network/tutorials/leaderboard/browser-dapp.md): Build a React frontend that connects to the Lace wallet, reads leaderboard state from the Midnight indexer, and submits scores on-chain with privacy modes.
- [Part 4: Production deployment](https://docs.midnight.network/tutorials/leaderboard/deployment.md): Deploy a Midnight leaderboard DApp frontend to Vercel on Preprod.
- [Build a privacy-preserving leaderboard DApp](https://docs.midnight.network/tutorials/leaderboard/overview.md): Build a full-stack DApp on the Midnight Network with privacy modes, ownership verification, and Lace wallet integration.
- [Part 1: The smart contract](https://docs.midnight.network/tutorials/leaderboard/smart-contract.md): Write a Compact smart contract with privacy modes, ownership verification, and arcade-style scoring for a leaderboard Dapp on Midnight.

### private-party

The privacy boundary in Midnight DApps: guests RSVP privately and only become public when they pay an unshielded NIGHT entry fee.

- [Private party tutorial](https://docs.midnight.network/tutorials/private-party.md): The privacy boundary in Midnight DApps: guests RSVP privately and only become public when they pay an unshielded NIGHT entry fee.
- [Private party contract](https://docs.midnight.network/tutorials/private-party/smart-contract.md): Read a Compact contract that keeps party guests private behind salted commitments, then makes them public at one point: an unshielded NIGHT entry fee.
- [Private party tests](https://docs.midnight.network/tutorials/private-party/test-suite.md): Run the private party on a local devnet with MidnightJS: RSVP privately, check in with NIGHT, claim the fees, and deploy a contract step by step.

### private-tip-jar

A first shielded token: anonymous tips into a contract-held pot, an owner-only withdrawal, and two contracts deployed and wired from one test suite.

- [Private tip jar tutorial](https://docs.midnight.network/tutorials/private-tip-jar.md): A first shielded token: anonymous tips into a contract-held pot, an owner-only withdrawal, and two contracts deployed and wired from one test suite.
- [Private tip jar contracts](https://docs.midnight.network/tutorials/private-tip-jar/smart-contract.md): Read the tip jar's two Compact contracts: an anonymous tip stored as a fresh contract-owned coin, an owner-only withdrawal, and a demo token to tip with.
- [Private tip jar tests](https://docs.midnight.network/tutorials/private-tip-jar/test-suite.md): Run the tip jar in memory and on a local devnet with MidnightJS: mint a demo token, tip it anonymously, check the public ledger for leaks, and withdraw.

### zk-loan

A ZK loan DApp that privately evaluates credit data against a Schnorr-signed attestation, verified inside the circuit, with only the loan outcome on chain.

- [ZK Loan tutorial](https://docs.midnight.network/tutorials/zk-loan.md): A ZK loan DApp that privately evaluates credit data against a Schnorr-signed attestation, verified inside the circuit, with only the loan outcome on chain.
- [Attestation API](https://docs.midnight.network/tutorials/zk-loan/attestation-api.md): Read the attestation API for the ZK loan DApp: a REST server that Schnorr-signs credit data so the contract can verify it inside the proof.
- [ZK Loan contract](https://docs.midnight.network/tutorials/zk-loan/smart-contract.md): Read a Compact contract that verifies a Schnorr-signed credit attestation inside the ZK proof, scores the applicant privately, and records only the outcome.
- [ZK Loan tests](https://docs.midnight.network/tutorials/zk-loan/test-suite.md): Run the ZK loan DApp on a local devnet with MidnightJS, and check that tampered credit data, blacklisted borrowers, and an ex-admin are rejected.

## guides

Task-focused guides for building on Midnight, from choosing a network and funding a wallet to deploying and securing a smart contract.

- [Guides](https://docs.midnight.network/guides.md): Task-focused guides for building on Midnight, from choosing a network and funding a wallet to deploying and securing a smart contract.

### acquire-tokens

Get tNIGHT from the faucet and register it for DUST generation, in Lace or with the wallet SDK, so your wallet can pay transaction fees.

- [Funding a wallet](https://docs.midnight.network/guides/acquire-tokens.md): Get tNIGHT from the faucet and register it for DUST generation, in Lace or with the wallet SDK, so your wallet can pay transaction fees.

### browser-dapp

Connect a browser DApp to a wallet, build the Midnight.js providers from it, then deploy a contract, join it, call a circuit, and read private state.

- [Deploying and calling a contract from a browser DApp](https://docs.midnight.network/guides/browser-dapp.md): Connect a browser DApp to a wallet, build the Midnight.js providers from it, then deploy a contract, join it, call a circuit, and read private state.

### build-a-private-smart-contract

Step-by-step guide to building a private smart contract in Compact: keep data on-device, disclose only what's needed, compile, and test locally.

- [How to build a private smart contract on Midnight](https://docs.midnight.network/guides/build-a-private-smart-contract.md): Step-by-step guide to building a private smart contract in Compact: keep data on-device, disclose only what's needed, compile, and test locally.

### build-cross-chain-dapp-with-effectstream

Run the EVM and Midnight template, learn how a rollup joins two chains without a bridge, and extend it with a field of your own.

- [Build a cross-chain DApp with EffectStream](https://docs.midnight.network/guides/build-cross-chain-dapp-with-effectstream.md): Run the EVM and Midnight template, learn how a rollup joins two chains without a bridge, and extend it with a field of your own.

### compact-javascript-runtime

Run a Compact contract's logic from JavaScript: implement witnesses, call circuits, write unit tests, and read the generated module's exports and errors.

- [Using Compact contracts from JavaScript](https://docs.midnight.network/guides/compact-javascript-runtime.md): Run a Compact contract's logic from JavaScript: implement witnesses, call circuits, write unit tests, and read the generated module's exports and errors.

### deploy-and-operate

Wire the Midnight.js providers, deploy a Compact contract to a network, observe its state, and operate the maintenance authority that keeps it updatable.

- [Deploying and operating a contract](https://docs.midnight.network/guides/deploy-and-operate.md): Wire the Midnight.js providers, deploy a Compact contract to a network, observe its state, and operate the maintenance authority that keeps it updatable.

### dust-sponsorship

Pay DUST fees on behalf of your users. The sponsorship model, the contract rule, both sides of the flow, and a runnable reference example.

- [Sponsor transaction fees with DUST](https://docs.midnight.network/guides/dust-sponsorship.md): Pay DUST fees on behalf of your users. The sponsorship model, the contract rule, both sides of the flow, and a runnable reference example.

### index-state-with-effectstream

Run an EffectStream node that watches a Midnight contract, decode its public ledger with a schema, and serve the result over HTTP.

- [Index contract state with EffectStream](https://docs.midnight.network/guides/index-state-with-effectstream.md): Run an EffectStream node that watches a Midnight contract, decode its public ledger with a schema, and serve the result over HTTP.

### local-proving

Run a local proof server or delegate proving to the wallet, see what a proof server receives, and why remote proving isn't recommended.

- [Proving transactions locally](https://docs.midnight.network/guides/local-proving.md): Run a local proof server or delegate proving to the wallet, see what a proof server receives, and why remote proving isn't recommended.

### manage-dust

Read DUST parameters and balances, check which NIGHT generates DUST, estimate fees, send several transactions from one wallet, and resolve DUST errors.

- [Managing DUST for transaction fees](https://docs.midnight.network/guides/manage-dust.md): Read DUST parameters and balances, check which NIGHT generates DUST, estimate fees, send several transactions from one wallet, and resolve DUST errors.

### networks-and-environments

Understand Midnight's networks, connect a DApp with the right network ID and endpoints, fund a wallet, and use Blockfrost tokens on Preprod and Mainnet.

- [Networks and environments](https://docs.midnight.network/guides/networks-and-environments.md): Understand Midnight's networks, connect a DApp with the right network ID and endpoints, fund a wallet, and use Blockfrost tokens on Preprod and Mainnet.

### nextjs-wallet-connect

Learn how to create a Next.js application that connects to a wallet using the Midnight DApp Connector API.

- [Create a Next.js wallet connector](https://docs.midnight.network/guides/nextjs-wallet-connect.md): Learn how to create a Next.js application that connects to a wallet using the Midnight DApp Connector API.

### node-server-wallet

Build a Midnight wallet in a Node.js process: derive keys from a seed, sync, keep state across restarts, and deploy a smart contract from a script.

- [Running a wallet in a Node.js service](https://docs.midnight.network/guides/node-server-wallet.md): Build a Midnight wallet in a Node.js process: derive keys from a seed, sync, keep state across restarts, and deploy a smart contract from a script.

### private-state-witnesses

Declare witnesses in Compact, implement them in TypeScript with the Compact runtime, wire up private state, and avoid common witness pitfalls.

- [Manage private state with witnesses](https://docs.midnight.network/guides/private-state-witnesses.md): Declare witnesses in Compact, implement them in TypeScript with the Compact runtime, wire up private state, and avoid common witness pitfalls.

### react-wallet-connect

Learn how to create a React application that connects to a wallet using the Midnight DApp Connector API.

- [Create a React wallet connector](https://docs.midnight.network/guides/react-wallet-connect.md): Learn how to create a React application that connects to a wallet using the Midnight DApp Connector API.

### run-proof-server

Learn how to run the proof server for the Midnight Network.

- [Proof server](https://docs.midnight.network/guides/run-proof-server.md): Learn how to run the proof server for the Midnight Network.

### security-best-practices

Secure a Midnight DApp: concepts, procedures, and references for authentication, access control, arithmetic, deadlines, replay, keys, proving, and privacy.

- [Security and best practices](https://docs.midnight.network/guides/security-best-practices.md): Secure a Midnight DApp: concepts, procedures, and references for authentication, access control, arithmetic, deadlines, replay, keys, proving, and privacy.

### windows-compact-setup

Learn how to set up the Midnight development environment on Windows using Windows Subsystem for Linux (WSL).

- [Windows Compact setup](https://docs.midnight.network/guides/windows-compact-setup.md): Learn how to set up the Midnight development environment on Windows using Windows Subsystem for Linux (WSL).

## how-to


### bun-runtime-midnight

Set up Bun package manager and the Compact compiler to build smart contracts and applications on the Midnight blockchain.

- [Set up Bun for Midnight development](https://docs.midnight.network/how-to/bun-runtime-midnight.md): Set up Bun package manager and the Compact compiler to build smart contracts and applications on the Midnight blockchain.

### decode-1010-transaction-rejection-errors

Decode the inner u8 in Substrate code 1010 "Invalid Transaction" responses to identify the underlying Midnight ledger error variant.

- [Decode 1010 transaction rejection errors](https://docs.midnight.network/how-to/decode-1010-transaction-rejection-errors.md): Decode the inner u8 in Substrate code 1010 "Invalid Transaction" responses to identify the underlying Midnight ledger error variant.

### fix-package-repository-access-failures

Resolve 403 Forbidden and authentication errors when installing @midnight-ntwrk npm packages. Covers registry, network, configuration, and token fixes.

- [Fix package repository access failures](https://docs.midnight.network/how-to/fix-package-repository-access-failures.md): Resolve 403 Forbidden and authentication errors when installing @midnight-ntwrk npm packages. Covers registry, network, configuration, and token fixes.

### fix-version-mismatches

Resolve version compatibility issues between Midnight compiler, runtime, and other components using the official compatibility matrix.

- [Fix version mismatch errors](https://docs.midnight.network/how-to/fix-version-mismatches.md): Resolve version compatibility issues between Midnight compiler, runtime, and other components using the official compatibility matrix.

## examples

Browse every documented Midnight example by the feature or problem it demonstrates, then open the contract or DApp closest to what you are building.

- [Examples](https://docs.midnight.network/examples.md): Browse every documented Midnight example by the feature or problem it demonstrates, then open the contract or DApp closest to what you are building.

### contracts

Standalone Compact contract examples that isolate specific language features like witness functions, token operations, and private state.

- [Contract examples](https://docs.midnight.network/examples/contracts.md): Standalone Compact contract examples that isolate specific language features like witness functions, token operations, and private state.
- [Battleship Simple Version](https://docs.midnight.network/examples/contracts/battleship-simple.md): Example Compact contract for a simple two-player Battleship game that shows explicit state management, private state verification, and circuit access control.
- [Calculator Contract](https://docs.midnight.network/examples/contracts/calculator.md): Example contract for math operations
- [Election Contract](https://docs.midnight.network/examples/contracts/election.md): Example contract for private state access
- [Private Guest List Contract](https://docs.midnight.network/examples/contracts/private-guest-list.md): Example Compact contract for a party with a private guest list that shows Set operations, NIGHT token payments, and the privacy boundary in Midnight DApps.
- [Private Reserve Auction Contract](https://docs.midnight.network/examples/contracts/private-reserve-auction.md): Example Compact contract for an auction with a private reserve and public bids that hides data on the ledger and uses DApp-specific keys to limit tracking.
- [Shielded and Unshielded Token Transfers](https://docs.midnight.network/examples/contracts/token-transfers.md): Token transfer examples

### dapps

End-to-end DApp examples with contract code, CLI tooling, and deployment steps you can study and adapt to your own project.

- [DApp examples](https://docs.midnight.network/examples/dapps.md): End-to-end DApp examples with contract code, CLI tooling, and deployment steps you can study and adapt to your own project.
- [Leaderboard DApp](https://docs.midnight.network/examples/dapps/leaderboard.md): Learn how the leaderboard DApp demonstrates privacy-preserving score submission and ownership verification with zero-knowledge proofs on Midnight Network.
- [ZK Loan DApp](https://docs.midnight.network/examples/dapps/zkloan.md): Privacy-preserving loan application on Midnight Network — credit data stays on the user's machine, only the loan outcome lands on-chain.

## concepts

Introduces Midnight's core concepts and links to pages on accounts, ledgers, the UTXO model, Web3, zero-knowledge proofs, Kachina, and Zswap.

- [Concepts](https://docs.midnight.network/concepts.md): Introduces Midnight's core concepts and links to pages on accounts, ledgers, the UTXO model, Web3, zero-knowledge proofs, Kachina, and Zswap.

### account

A reference on the account model.

- [Account model](https://docs.midnight.network/concepts/account.md): A reference on the account model.

### blockchain-space-tokenization

Learn how blockchain space tokenization (BST) resolves the trade-off between low transaction fees and fast processing times.

- [Blockchain space tokenization (BST)](https://docs.midnight.network/concepts/blockchain-space-tokenization.md): Learn how blockchain space tokenization (BST) resolves the trade-off between low transaction fees and fast processing times.

### dual-component-tokenomics

Learn why Midnight uses a dual-component tokenomics model with NIGHT and DUST, and how it solves the limitations of single-token proof-of-stake systems.

- [Dual-component tokenomics in Midnight](https://docs.midnight.network/concepts/dual-component-tokenomics.md): Learn why Midnight uses a dual-component tokenomics model with NIGHT and DUST, and how it solves the limitations of single-token proof-of-stake systems.

### dust-architecture

A reference on the architecture of DUST and network usage.

- [DUST architecture](https://docs.midnight.network/concepts/dust-architecture.md): A reference on the architecture of DUST and network usage.

### fully-homomorphic-encryption

What is fully homomorphic encryption, how does it compare to zero-knowledge proofs, and which platforms provide it? A plain explainer.

- [What is FHE (fully homomorphic encryption)?](https://docs.midnight.network/concepts/fully-homomorphic-encryption.md): What is fully homomorphic encryption, how does it compare to zero-knowledge proofs, and which platforms provide it? A plain explainer.

### how-midnight-works

- [Building blocks](https://docs.midnight.network/concepts/how-midnight-works/building-blocks.md): Learn how Midnight transactions combine intents, Zswap offers, contract calls, and deployments, and how transaction merging and binding commitments work.
- [Compact as a privacy-first language](https://docs.midnight.network/concepts/how-midnight-works/compact-privacy-first-language.md): Explains why Midnight uses its own smart contract language and how Compact treats privacy-aware circuits as first-class citizens.
- [End-to-end architecture](https://docs.midnight.network/concepts/how-midnight-works/end-to-end-architecture.md): One diagram connecting private computation on your device to public verification on the Midnight network, and what each layer can and cannot see.
- [The Impact VM](https://docs.midnight.network/concepts/how-midnight-works/impact.md): Explains Impact, the stack-based, non-Turing-complete on-chain VM language, covering transcripts, state values, the opcode table, and context and effects.
- [Private data](https://docs.midnight.network/concepts/how-midnight-works/keeping-data-private.md): Strategies for keeping data private in Midnight smart contracts, covering hashes and commitments, Merkle trees, and the commitment and nullifier pattern.
- [Midnight's hybrid architecture](https://docs.midnight.network/concepts/how-midnight-works/midnight-combined-model.md): Explains how Midnight builds account-based smart contracts on a UTXO foundation, how NIGHT generates DUST, and when to choose ledger tokens or contract tokens.
- [Transaction semantics](https://docs.midnight.network/concepts/how-midnight-works/semantics.md): Explains Midnight transaction semantics, covering ledger and contract state, the well-formedness check, and how the guaranteed and fallible phases execute.
- [Smart contracts on Midnight](https://docs.midnight.network/concepts/how-midnight-works/smart-contracts.md): Explains how smart contracts on Midnight differ from public smart contracts, using a guessing game to cover transcripts, zero-knowledge proofs, and coins.
- [Zswap](https://docs.midnight.network/concepts/how-midnight-works/zswap.md): Explains Zswap, a shielded token mechanism that extends Zerocash with native token support and atomic swaps, covering offers, inputs, outputs, and token types.

### how-privacy-blockchains-work

How privacy blockchains work, the techniques they use to protect data, and which platforms enable compliant privacy for enterprise use cases.

- [How privacy blockchains work](https://docs.midnight.network/concepts/how-privacy-blockchains-work.md): How privacy blockchains work, the techniques they use to protect data, and which platforms enable compliant privacy for enterprise use cases.

### identity

Learn what Midnight Passport, did:midnight, and the verifiable credentials libraries are, how they relate to Compact contracts, and which you can use today.

- [Identity on Midnight](https://docs.midnight.network/concepts/identity.md): Learn what Midnight Passport, did:midnight, and the verifiable credentials libraries are, how they relate to Compact contracts, and which you can use today.

### kachina

A reference on the Kachina model.

- [Kachina](https://docs.midnight.network/concepts/kachina.md): A reference on the Kachina model.

### ledgers

A reference on the ledger model.

- [Ledgers](https://docs.midnight.network/concepts/ledgers.md): A reference on the ledger model.

### network-architecture

- [Consensus](https://docs.midnight.network/concepts/network-architecture/consensus.md): What blockchain consensus is, the main consensus mechanism families, and how Midnight implements it with AURA block production and GRANDPA finality.
- [Cryptography](https://docs.midnight.network/concepts/network-architecture/cryptography.md): A reference on cryptography in Midnight nodes.
- [Onchain logic and state](https://docs.midnight.network/concepts/network-architecture/onchain-logic.md): A reference on onchain logic and state in Midnight nodes.
- [P2P networking](https://docs.midnight.network/concepts/network-architecture/p2p-networking.md): A reference on P2P networking in Midnight nodes.
- [RPC interface](https://docs.midnight.network/concepts/network-architecture/rpc-networking.md): A reference on the RPC interface.
- [Storage](https://docs.midnight.network/concepts/network-architecture/storage.md): A reference on storage in Midnight nodes.
- [Transactions](https://docs.midnight.network/concepts/network-architecture/transactions.md): A reference on the Midnight transaction format, its classification vocabulary, and the non-ledger transaction categories the node owns.

### sidechains-partnerchains

Understanding the research foundation of the partner chain framework

- [The origins of sidechains and partner chains](https://docs.midnight.network/concepts/sidechains-partnerchains.md): Understanding the research foundation of the partner chain framework

### utxo

Explains the UTXO model Midnight uses, covering indivisible outputs, the nullifier set, atomic transactions, and how it compares with the account model.

- [UTXO model](https://docs.midnight.network/concepts/utxo.md): Explains the UTXO model Midnight uses, covering indivisible outputs, the nullifier set, atomic transactions, and how it compares with the account model.

### web3

A reference on the Web3 model.

- [Web3](https://docs.midnight.network/concepts/web3.md): A reference on the Web3 model.

### zero-knowledge-proofs

A reference on how zero-knowledge proofs work and how they are used in Midnight.

- [Zero-knowledge proofs](https://docs.midnight.network/concepts/zero-knowledge-proofs.md): A reference on how zero-knowledge proofs work and how they are used in Midnight.

### zk-snark-vs-zk-stark

zk-SNARK vs zk-STARK compared on trusted setup, proof size, verification speed, and quantum resistance, with real platform examples for each.

- [zk-SNARK vs zk-STARK: which approach do privacy-focused blockchains use?](https://docs.midnight.network/concepts/zk-snark-vs-zk-stark.md): zk-SNARK vs zk-STARK compared on trusted setup, proof size, verification speed, and quantum resistance, with real platform examples for each.

### zk-vs-fhe-vs-mpc

Compare zero-knowledge proofs, FHE, and MPC for privacy-preserving DApps: what each protects, example platforms, and which approach to choose.

- [ZK vs FHE vs MPC: which should you use for a privacy-focused DApp?](https://docs.midnight.network/concepts/zk-vs-fhe-vs-mpc.md): Compare zero-knowledge proofs, FHE, and MPC for privacy-preserving DApps: what each protects, example platforms, and which approach to choose.

### zswap

A reference on how Zswap works and how it is used in Midnight.

- [Zswap](https://docs.midnight.network/concepts/zswap.md): A reference on how Zswap works and how it is used in Midnight.

## tokens


### overview

Understand NIGHT, DUST, shielded and unshielded tokens, custom tokens, fees, and how a contract receives and holds tokens on Midnight.

- [Tokens on Midnight](https://docs.midnight.network/tokens/overview.md): Understand NIGHT, DUST, shielded and unshielded tokens, custom tokens, fees, and how a contract receives and holds tokens on Midnight.

### shielded-token

Build, deploy, and transfer a privacy-preserving (shielded) token in Compact on Midnight, where value lives in immutable coins and balances stay hidden.

- [Create and transfer a shielded token](https://docs.midnight.network/tokens/shielded-token.md): Build, deploy, and transfer a privacy-preserving (shielded) token in Compact on Midnight, where value lives in immutable coins and balances stay hidden.

### unshielded-token

Build, deploy, and transfer a transparent (unshielded) token in Compact on Midnight, with on-chain balances you read straight from wallet state.

- [Create and transfer an unshielded token](https://docs.midnight.network/tokens/unshielded-token.md): Build, deploy, and transfer a transparent (unshielded) token in Compact on Midnight, with on-chain balances you read straight from wallet state.

## compact

Midnight comes with its own programming language, Compact, which enables you to write smart contracts for the Midnight ecosystem. The Compact compiler outputs zero-knowledge circuits that are used to prove the correctness of interactions with the ledger.

- [The Compact language](https://docs.midnight.network/compact.md): Midnight comes with its own programming language, Compact, which enables you to write smart contracts for the Midnight ecosystem. The Compact compiler outputs zero-knowledge circuits that are used to prove the correctness of interactions with the ledger.

### compilation-and-tooling

- [Compact compiler usage page](https://docs.midnight.network/compact/compilation-and-tooling/compiler-usage.md): This is the usage page of compactc, a compiler for Compact.
- [Compact command-line tool usage page](https://docs.midnight.network/compact/compilation-and-tooling/dev-tool-usage.md): This is the usage page of the Compact command-line tool. This tool allows the
- [Compact fixup usage page](https://docs.midnight.network/compact/compilation-and-tooling/fixup-usage.md): This is the usage page of fixup-compact, a fix up tool for Compact.
- [Compact formatter usage page](https://docs.midnight.network/compact/compilation-and-tooling/formatter-usage.md): This is the usage page of format-compact, a formatter for Compact.
- [Neovim setup for Compact](https://docs.midnight.network/compact/compilation-and-tooling/neovim-setup.md): compact.vim is a community-driven plugin that provides Compact language support for Neovim.
- [Visual Studio Code extension for Compact](https://docs.midnight.network/compact/compilation-and-tooling/vscode-plugin.md): The [Visual Studio Code extension] for Compact is a plugin

### data-types

- [Ledger data types](https://docs.midnight.network/compact/data-types/ledger-adt.md): Compact language version 0.28.0, compiler version 0.36.0.
- [Opaque data types](https://docs.midnight.network/compact/data-types/opaque_data.md): Data types whose representations are visible are called transparent. These expose the inner structure of the data so that operations on the data can be understood.

### reference

- [Compact grammar](https://docs.midnight.network/compact/reference/compact-grammar.md): Compact language version 0.28.0.
- [Compact keywords](https://docs.midnight.network/compact/reference/compact-keywords.md): Module-related keywords
- [Compact reference](https://docs.midnight.network/compact/reference/compact-reference.md): Reviewed 25 September 2026 for compatibility with Compact language version 0.28.0.
- [Explicit disclosure in Compact: The Midnight "Witness Protection Program"](https://docs.midnight.network/compact/reference/explicit-disclosure.md): Introduction
- [Ledger data types](https://docs.midnight.network/compact/reference/ledger-adt.md): Compact language version 0.28.0, compiler version 0.36.0.
- [Writing a contract](https://docs.midnight.network/compact/reference/writing.md): This page walks

### smart-contract-security

Compact smart contracts on Midnight combine privacy-preserving computation with cryptographic guarantees. The compiler enforces certain rules and restrictions, but much of the responsibility for secure contract interaction is up to the developer implementing the smart contract. For this reason, it is very important for smart contract developers to understand common pitfalls and adhere to security best practices.

- [Smart contract security](https://docs.midnight.network/compact/smart-contract-security.md): Compact smart contracts on Midnight combine privacy-preserving computation with cryptographic guarantees. The compiler enforces certain rules and restrictions, but much of the responsibility for secure contract interaction is up to the developer implementing the smart contract. For this reason, it is very important for smart contract developers to understand common pitfalls and adhere to security best practices.

### standard-library

CompactStandardLibrary ∙ Detailed API reference

- [Compact standard library](https://docs.midnight.network/compact/standard-library.md): CompactStandardLibrary ∙ Detailed API reference
- [Detailed API reference](https://docs.midnight.network/compact/standard-library/exports.md): Top-level exports and native types and functions

### test-and-debug

Learn how to test and debug Compact smart contracts.

- [Test and debug](https://docs.midnight.network/compact/test-and-debug.md): Learn how to test and debug Compact smart contracts.

## sdks

Overview of the Midnight SDKs for building DApps and wallets, covering the official Midnight.js and Wallet SDK and libraries built by the community.

- [Midnight SDKs](https://docs.midnight.network/sdks.md): Overview of the Midnight SDKs for building DApps and wallets, covering the official Midnight.js and Wallet SDK and libraries built by the community.

### community

Third-party libraries and tools built on Midnight by the wider community.

- [Community projects](https://docs.midnight.network/sdks/community.md): Third-party libraries and tools built on Midnight by the wider community.
- [EchoMKB](https://docs.midnight.network/sdks/community/ai-tools/echomkb.md): Community agent skill reporting per-network supported versus released versions, citing docs URLs with compiler stamps, and searching upstream issue trackers.
- [Midnight agent skills](https://docs.midnight.network/sdks/community/ai-tools/midnight-agent-skills.md): midnight_agent_skills is a set of four modular agent skills that give AI coding assistants accurate knowledge of Compact syntax, Midnight SDK, network configuration, and common gotchas sourced from the developer community.
- [Edda Midnight starter template](https://docs.midnight.network/sdks/community/edda-midnight-starter.md): A full-stack starter template for building DApps on Midnight with a React frontend, Compact smart contract, CLI tooling, and educational materials.
- [Midnames](https://docs.midnight.network/sdks/community/midnames.md): Learn how to use the Midnames Compact playground and name service to register human-readable names for Midnight addresses.
- [Midnight Explorer](https://docs.midnight.network/sdks/community/midnight-explorer.md): Explore blocks, transactions, addresses, and contracts on the Midnight Network with Midnight Explorer, a community-built web explorer.
- [Midnight Live View](https://docs.midnight.network/sdks/community/midnight-live-view.md): A real-time terminal dashboard and persistent block monitor for Midnight validator nodes, inspired by CNTool's gLiveView.
- [Nightforge](https://docs.midnight.network/sdks/community/nightforge.md): Compile, deploy, and operate Compact contracts from one CLI with Nightforge, a community development environment for Midnight.
- [OpenZeppelin contracts for Compact](https://docs.midnight.network/sdks/community/openzeppelin-compact-contracts.md): A library of token standards, access control, and security primitives for Compact smart contracts, modeled on OpenZeppelin's Solidity contracts.
- [Develop with the wallet CLI and MCP](https://docs.midnight.network/sdks/community/wallets/community-wallets-cli-mcp.md): Develop and test on the Midnight Network without a browser wallet using the wallet CLI, and drive a self-custody wallet from AI agents over MCP.
- [Integrate a wallet](https://docs.midnight.network/sdks/community/wallets/community-wallets-integration.md): Integrate a community wallet on the Midnight Network with the DApp Connector (CAIP-372): discovery, connect, ZK proving, fees, and portable React code.
- [Community wallets](https://docs.midnight.network/sdks/community/wallets/community-wallets-overview.md): Choose a community wallet on the Midnight Network using a custody and interface model, with per-wallet profiles for Lace, 1AM, Kuira, and more.
- [Wallet reference](https://docs.midnight.network/sdks/community/wallets/community-wallets-reference.md): Reference for Midnight Network community wallets: the functionality matrix, troubleshooting and error codes, a security checklist, ecosystem, and glossary.

### error-reference

- [Midnight.js error reference](https://docs.midnight.network/sdks/error-reference/midnight-js.md): Error reference for the Midnight.js SDK.
- [Wallet SDK error reference](https://docs.midnight.network/sdks/error-reference/wallet-sdk.md): Error reference for the Wallet SDK.

### official

- [Build a Midnight wallet](https://docs.midnight.network/sdks/official/build-a-wallet.md): Build a Midnight wallet: run keys and sync with the Wallet SDK, and implement the DApp Connector API so DApps can connect, transact, and prove.
- [Midnight.js](https://docs.midnight.network/sdks/official/midnight-js.md): Comprehensive TypeScript SDK for building DApps on Midnight Network.
- [Midnight wallet SDK](https://docs.midnight.network/sdks/official/wallet-developer-guide.md): Developer guide for the Midnight Wallet SDK, covering unshielded, shielded, and DUST wallet operations.

### troubleshoot

Troubleshoot common issues when building on Midnight Network.

- [Common SDK integration issues](https://docs.midnight.network/sdks/troubleshoot.md): Troubleshoot common issues when building on Midnight Network.

## ai-integration

Build on Midnight with AI coding assistants that verify generated code against the real compiler and source, so you ship working code, not hallucinations.

- [AI integration](https://docs.midnight.network/ai-integration.md): Build on Midnight with AI coding assistants that verify generated code against the real compiler and source, so you ship working code, not hallucinations.

### kapa-mcp-server

The Kapa MCP server gives AI coding assistants documentation-grounded answers about Midnight directly inside your editor.

- [Kapa MCP server](https://docs.midnight.network/ai-integration/kapa-mcp-server.md): The Kapa MCP server gives AI coding assistants documentation-grounded answers about Midnight directly inside your editor.

### midnight-expert

Midnight Expert is a suite of Claude Code plugins that help you write and review Compact, scaffold DApps, and verify generated code against the real compiler.

- [Midnight Expert](https://docs.midnight.network/ai-integration/midnight-expert.md): Midnight Expert is a suite of Claude Code plugins that help you write and review Compact, scaffold DApps, and verify generated code against the real compiler.

## nodes

A reference on Midnight nodes.

- [Midnight Node overview](https://docs.midnight.network/nodes.md): A reference on Midnight nodes.

### boot-node

A reference on setting up boot nodes for Midnight.

- [Set up boot node](https://docs.midnight.network/nodes/boot-node.md): A reference on setting up boot nodes for Midnight.

### cardano-db-sync

A reference on setting up Cardano-db-sync for Midnight.

- [Set up Cardano-db-sync](https://docs.midnight.network/nodes/cardano-db-sync.md): A reference on setting up Cardano-db-sync for Midnight.

### cardano-node

A reference on setting up Cardano Node for Midnight.

- [Set up Cardano node](https://docs.midnight.network/nodes/cardano-node.md): A reference on setting up Cardano Node for Midnight.

### error-codes

A list of Midnight node error codes and their troubleshooting steps.

- [Midnight node error codes](https://docs.midnight.network/nodes/error-codes.md): A list of Midnight node error codes and their troubleshooting steps.

### full-node

A reference on setting up full and archive nodes for Midnight.

- [Set up full node](https://docs.midnight.network/nodes/full-node.md): A reference on setting up full and archive nodes for Midnight.

### node-endpoints

A reference for Midnight Network RPC endpoints, including the Blockfrost Mainnet RPC that needs a project token, and common RPC queries.

- [Node endpoints](https://docs.midnight.network/nodes/node-endpoints.md): A reference for Midnight Network RPC endpoints, including the Blockfrost Mainnet RPC that needs a project token, and common RPC queries.

### node-toolkit

Install the Midnight node toolkit and use it to inspect blocks, read ledger parameters, derive addresses, and send test transactions on a local network.

- [Using the Midnight node toolkit](https://docs.midnight.network/nodes/node-toolkit.md): Install the Midnight node toolkit and use it to inspect blocks, read ledger parameters, derive addresses, and send test transactions on a local network.

### rpc-node

A reference on setting up RPC nodes for Midnight.

- [Set up RPC node for Midnight](https://docs.midnight.network/nodes/rpc-node.md): A reference on setting up RPC nodes for Midnight.

## relnotes


### compact

Release notes for the Compact toolchain (compiler, formatter, and fixup tool), plus how to install the toolchain version that each public network uses.

- [Compact toolchain](https://docs.midnight.network/relnotes/compact.md): Release notes for the Compact toolchain (compiler, formatter, and fixup tool), plus how to install the toolchain version that each public network uses.
- [Compact compiler 0.18.2 (Compact language 0.10.1)](https://docs.midnight.network/relnotes/compact/compact-0-10-1.md): Release notes for Compact compiler 0.18.2 (language 0.10.1), which shipped with the public Testnet and replaces the ledger block with per-field declarations.
- [Compact compiler 0.19.0 (Compact language 0.11.0)](https://docs.midnight.network/relnotes/compact/compact-0-11-0.md): Release notes for Compact compiler 0.19.0 (language 0.11.0), with breaking syntax changes such as angle-bracket generics and Uint replacing Unsigned Integer.
- [Compact compiler 0.20.0 (Compact language 0.13.0)](https://docs.midnight.network/relnotes/compact/compact-0-13-0.md): Release notes for Compact compiler 0.20.0 (language 0.13.0), which adds module imports from files and changes the include search order.
- [Compact compiler 0.21.0 (Compact language 0.14.0)](https://docs.midnight.network/relnotes/compact/compact-0-14-0.md): Release notes for Compact compiler 0.21.0 (language 0.14.0), which adds disclose for witness return values, removes Void, and aligns for loops with TypeScript.
- [Compact compiler 0.22.0 (Compact language 0.14.0)](https://docs.midnight.network/relnotes/compact/compact-0-14-22-0.md): Release notes for Compact compiler 0.22.0 (language 0.14.0), a bug fix release that fixes ZK proofs for Uint circuit parameters and several compiler crashes.
- [Compact compiler 0.23.0 (Compact language 0.15.0)](https://docs.midnight.network/relnotes/compact/compact-0-15-23-0.md): Release notes for Compact compiler 0.23.0 (language 0.15.0), the minimum version for the upgraded Testnet, with a breaking change to the maximum Field value.
- [Compact compiler 0.24.0 (Compact language 0.16.0)](https://docs.midnight.network/relnotes/compact/compact-0-16-24-0.md): Release notes for Compact compiler 0.24.0 (language 0.16.0), with camelCase standard library names and a disclose requirement for exported circuit arguments.
- [Compact compiler 0.25.0 (Compact language 0.17.0)](https://docs.midnight.network/relnotes/compact/compact-0-17-25-0.md): Release notes for Compact compiler 0.25.0 (language 0.17.0), which adds casts between Bytes and Vector types and standard library circuits for block time.
- [Compact toolchain 0.28.0 (Compact language 0.20.0)](https://docs.midnight.network/relnotes/compact/compact-0-20-28-0.md): Release notes for Compact toolchain 0.28.0 (language 0.20.0), the first to target Preprod and Preview, with unshielded token APIs and ES module output.
- [Compact compiler 0.26.0 (Minokawa language 0.18.0)](https://docs.midnight.network/relnotes/compact/minokawa-0-18-26-0.md): Release notes for Compact compiler 0.26.0 (Minokawa language 0.18.0), the first under the Minokawa name, with bytes value features and a new slice keyword.
- [Compact toolchain 0.30.0](https://docs.midnight.network/relnotes/compact/toolchain-0.30.0.md): Release notes for Compact toolchain 0.30.0 (language 0.22.0), which targets Midnight ledger 8, renames NativePoint to JubjubPoint, and adds version flags.
- [Compact toolchain 0.31.0](https://docs.midnight.network/relnotes/compact/toolchain-0.31.0.md): Release notes for Compact toolchain 0.31.0 (language 0.23.0), which works around erroneous proof failures from untaken branches and allows generic loop bounds.
- [Compact toolchain 0.31.1](https://docs.midnight.network/relnotes/compact/toolchain-0.31.1.md): Release notes for Compact toolchain 0.31.1 (language 0.23.0), a patch release on the 0.31 line that fixes code generation for certain casts.
- [Compact toolchain 0.34.0](https://docs.midnight.network/relnotes/compact/toolchain-0.34.0.md): Release notes for Compact toolchain 0.34.0 (language 0.26.0), a major release for ledger 9 that lets cross-contract callees perform shielded coin operations.
- [Compact toolchain 0.35.0](https://docs.midnight.network/relnotes/compact/toolchain-0.35.0.md): Compact toolchain 0.35.0 targets ledger 9, adds sha512 and kernel.caller(), and changes the Compact runtime's cross-contract API.
- [Compact toolchain 0.36.0](https://docs.midnight.network/relnotes/compact/toolchain-0.36.0.md): Compact toolchain 0.36.0 targets ledger 9, makes ZKIR version 3 the only backend, removes the --feature-zkir-v3 flag, and disallows pure circuits in contract types.

### compact-js

Release notes for Compact.js, the TypeScript-based execution environment that runs the circuits of smart contracts compiled with the Compact compiler.

- [Compact.js](https://docs.midnight.network/relnotes/compact-js.md): Release notes for Compact.js, the TypeScript-based execution environment that runs the circuits of smart contracts compiled with the Compact compiler.
- [Compact.js 2.4.0 release notes](https://docs.midnight.network/relnotes/compact-js/compact-js-2-4-0.md): Release notes for Compact.js 2.4.0, the initial release, with a core library for running compiled Compact contracts and a command package for CLI tools.
- [Compact.js 2.5.0 release notes](https://docs.midnight.network/relnotes/compact-js/compact-js-2-5-0.md): Release notes for Compact.js 2.5.0, which adds an --input-ledger-params flag to the circuit command and needs contracts compiled with Compact toolchain 0.30.0.
- [Compact.js 2.5.3 release notes](https://docs.midnight.network/relnotes/compact-js/compact-js-2-5-3.md): Release notes for Compact.js 2.5.3, a maintenance release that fixes the package's CommonJS exports and applies a security update to the tar dependency.
- [Compact.js 3.0.0 release notes](https://docs.midnight.network/relnotes/compact-js/compact-js-3-0-0.md): Release notes for Compact.js 3.0.0, the major release for the ledger 9 network upgrade on Mainnet, used by Midnight.js 5.

### compact-tools

Release notes for the Compact developer tools, the command-line utility for installing, updating, managing, and running the Compact toolchain and compiler.

- [Compact developer tools](https://docs.midnight.network/relnotes/compact-tools.md): Release notes for the Compact developer tools, the command-line utility for installing, updating, managing, and running the Compact toolchain and compiler.
- [Compact developer tools 0.1.0](https://docs.midnight.network/relnotes/compact-tools/compact-tools-0-1-0.md): Release notes for Compact developer tools 0.1.0, the initial release of the compact command-line tool, which supports toolchain versions 0.22, 0.23, and 0.24.
- [Compact developer tools 0.2.0](https://docs.midnight.network/relnotes/compact-tools/compact-tools-0-2-0.md): Release notes for Compact developer tools 0.2.0, which adds the compact format command, with a --check mode, for the formatter released with compiler 0.25.0.
- [Compact developer tools 0.3.0](https://docs.midnight.network/relnotes/compact-tools/compact-tools-0-3-0.md): Release notes for Compact developer tools 0.3.0, which enables the fixup subcommand that can update contracts to new language versions in many cases.
- [Compact developer tools 0.4.0](https://docs.midnight.network/relnotes/compact-tools/compact-tools-0-4-0.md): Release notes for Compact developer tools 0.4.0, which makes the fixup command edit files in place by default and adds a --check mode for CI workflows.
- [Compact developer tools 0.5.0](https://docs.midnight.network/relnotes/compact-tools/compact-tools-0-5-0.md): Release notes for Compact developer tools 0.5.0, which lets you abbreviate compact subcommands and pass partial version numbers to compact update.
- [Compact developer tools 0.5.1](https://docs.midnight.network/relnotes/compact-tools/compact-tools-0-5-1.md): Release notes for Compact developer tools 0.5.1, which adds native ARM64 Linux binaries so the zkir tool no longer crashes in ARM Docker environments.
- [Compact developer tools 0.5.2](https://docs.midnight.network/relnotes/compact-tools/compact-tools-0-5-2.md): Release notes for Compact developer tools 0.5.2, a maintenance release in which compact compile --help shows the full compiler help for your toolchain.
- [Compact developer tools 0.5.3](https://docs.midnight.network/relnotes/compact-tools/compact-tools-0-5-3.md): Release notes for Compact developer tools 0.5.3, which installs toolchains without unzip and repairs a failed install the next time you run compact update.

### dapp-connector-api

Release notes for the DApp Connector API, which DApps use to request wallet access, verify authorization, and retrieve service URIs for the Midnight Network.

- [DApp Connector API](https://docs.midnight.network/relnotes/dapp-connector-api.md): Release notes for the DApp Connector API, which DApps use to request wallet access, verify authorization, and retrieve service URIs for the Midnight Network.
- [DApp Connector API](https://docs.midnight.network/relnotes/dapp-connector-api/dapp-connector-api-1-2-3.md): Release notes for Midnight DApp Connector API 1.2.3, which bundles the package as both CommonJS and ES modules for wider runtime support.
- [DApp Connector API](https://docs.midnight.network/relnotes/dapp-connector-api/dapp-connector-api-2-0-0.md): Release notes for Midnight DApp Connector API 2.0.0, which makes Bech32m the default address format while keeping support for the legacy hex format.
- [DApp Connector API](https://docs.midnight.network/relnotes/dapp-connector-api/dapp-connector-api-3-0-0.md): Release notes for DApp Connector API 3.0.0, which DApps use to request wallet access and retrieve service URIs, with BLS support as its key change.
- [DApp Connector API v4.0.0 release notes](https://docs.midnight.network/relnotes/dapp-connector-api/dapp-connector-api-4-0-0.md): Release notes for DApp Connector API 4.0.0, a complete redesign with network-aware connections, granular balance and address methods, and proving delegation.
- [DApp Connector API v4.0.1 release notes](https://docs.midnight.network/relnotes/dapp-connector-api/dapp-connector-api-4-0-1.md): Release notes for DApp Connector API 4.0.1, a patch release that adds the payFees option to every transacting method, not only makeIntent.
- [DApp Connector API v4.1.0 release notes](https://docs.midnight.network/relnotes/dapp-connector-api/dapp-connector-api-4-1-0.md): Release notes for DApp Connector API 4.1.0, which moves to the @midnightntwrk npm scope, adds ECDSA signatures to signData, adds the InsufficientFunds error code, and fixes type resolution.

### dapp-examples-deprecated

- [Midnight Example Applications](https://docs.midnight.network/relnotes/dapp-examples-deprecated/dapp-examples-0-1-11.md): Release notes for Midnight example applications 0.1.11 (17 February 2024), with bug fixes and performance improvements for the bulletin board and counter DApps.
- [Midnight Example Applications](https://docs.midnight.network/relnotes/dapp-examples-deprecated/dapp-examples-0-1-12.md): Release notes for Midnight example applications 0.1.12 (6 June 2024), with bug fixes and performance improvements for the bulletin board and counter DApps.
- [Midnight Example Applications](https://docs.midnight.network/relnotes/dapp-examples-deprecated/dapp-examples-0-1-13.md): Release notes for Midnight example applications 0.1.13 (12 July 2024), with bug fixes and performance improvements for the bulletin board and counter DApps.
- [Midnight Example Applications](https://docs.midnight.network/relnotes/dapp-examples-deprecated/dapp-examples-0-1-14.md): Release notes for Midnight example applications 0.1.14 (2 October 2024), with bug fixes and performance improvements for the bulletin board and counter DApps.
- [Midnight Example Applications](https://docs.midnight.network/relnotes/dapp-examples-deprecated/dapp-examples-0-1-15.md): Release notes for Midnight example applications 0.1.15 (16 October 2024), with bug fixes and performance improvements for the bulletin board and counter DApps.
- [Midnight Example Applications](https://docs.midnight.network/relnotes/dapp-examples-deprecated/dapp-examples-0-1-16.md): Release notes for Midnight example applications 0.1.16 (25 November 2024), with bug fixes and performance improvements for the bulletin board and counter DApps.
- [Midnight Example Applications](https://docs.midnight.network/relnotes/dapp-examples-deprecated/dapp-examples-0-1-17.md): Release notes for Midnight example applications 0.1.17 (15 January 2025), with bug fixes and performance improvements for the bulletin board and counter DApps.
- [Midnight Example Applications](https://docs.midnight.network/relnotes/dapp-examples-deprecated/dapp-examples-0-2-0.md): Release notes for Midnight example applications 0.2.0 (18 February 2025), with bug fixes and performance improvements for the bulletin board and counter DApps.
- [Midnight Example Applications](https://docs.midnight.network/relnotes/dapp-examples-deprecated/dapp-examples-0-2-2.md): Release notes for Midnight example applications 0.2.2 (9 April 2025), with bug fixes and performance improvements for the bulletin board and counter DApps.

### ledger

Release notes for Midnight Ledger, the public record of contract and token states, plus the on-chain runtime and ZKIR versions in each Ledger release.

- [Ledger](https://docs.midnight.network/relnotes/ledger.md): Release notes for Midnight Ledger, the public record of contract and token states, plus the on-chain runtime and ZKIR versions in each Ledger release.
- [Ledger](https://docs.midnight.network/relnotes/ledger/ledger-3-0-6.md): Release notes for Midnight Ledger 3.0.6, which adds coinCommitment and coinNullifier, stops direct construction of transaction types, and fixes Schnorr proofs.
- [Ledger](https://docs.midnight.network/relnotes/ledger/ledger-4-0-0.md): Release notes for Midnight Ledger 4.0.0, which switches from Pluto-Eris to BLS12-381 and moves to a new storage model and data providers for key material.
- [Ledger 7.0.0 release notes](https://docs.midnight.network/relnotes/ledger/ledger-7-0-0.md): Release notes for Ledger 7.0.0, which switches to the Midnight SRS with midnight-zk 1.0, adds the addCalls endpoint, and disables treasury access.
- [Ledger 8.0.2 release notes](https://docs.midnight.network/relnotes/ledger/ledger-8-0-2.md): Release notes for Ledger 8.0.2, which fixes Merkle tree canonicity and DUST change computation during registration and adds a with_genesis_settings constructor.
- [Ledger 8.0.3 release notes](https://docs.midnight.network/relnotes/ledger/ledger-8-0-3.md): Release notes for Ledger 8.0.3, a patch release that fixes transcript partitioning and fee accounting for transactions combining transfers with contract calls.
- [Ledger v8.1.0 release notes](https://docs.midnight.network/relnotes/ledger/ledger-8-1-0.md): Release notes for Ledger 8.1.0, which adds an incremental garbage collector to storage-core and fixes storage race conditions, deadlocks, and a memory leak.
- [Ledger v8.1.1 release notes](https://docs.midnight.network/relnotes/ledger/ledger-8-1-1.md): Release notes for Ledger 8.1.1, a patch release that publishes the ledger npm packages under the @midnightntwrk scope instead of @midnight-ntwrk.
- [Ledger v8.1.2 release notes](https://docs.midnight.network/relnotes/ledger/ledger-8-1-2.md): Release notes for Ledger 8.1.2, a security patch that hardens deserialization to reject non-canonical encodings and values that break their type invariants.
- [Ledger v8.1.3 release notes](https://docs.midnight.network/relnotes/ledger/ledger-8-1-3.md): Release notes for Ledger 8.1.3, a security patch that fixes advisory GHSA-wr7g-rr4v-jmj8 by requiring canonical field values in contract call transcripts.
- [Ledger v9.1.0.1 release notes](https://docs.midnight.network/relnotes/ledger/ledger-9-1-0-1.md): Release notes for Ledger 9.1.0.1 and proof server 9.0.0, the ledger that Mainnet runs after the ledger 9 network upgrade.

### ledger-9

What the ledger 9 network upgrade changes on Midnight Mainnet, and what DApp developers, wallet and indexer operators, and node operators need to do.

- [Ledger 9 upgrade](https://docs.midnight.network/relnotes/ledger-9.md): What the ledger 9 network upgrade changes on Midnight Mainnet, and what DApp developers, wallet and indexer operators, and node operators need to do.

### midnight-indexer

Release notes for the Midnight Indexer, which stores indexed block data from a Midnight node and provides a GraphQL API for queries and subscriptions.

- [Midnight Indexer](https://docs.midnight.network/relnotes/midnight-indexer.md): Release notes for the Midnight Indexer, which stores indexed block data from a Midnight node and provides a GraphQL API for queries and subscriptions.
- [Midnight Indexer](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-1-0-0.md): Release notes for Midnight Indexer 1.0.0, the first production release of the Rust-based indexer, which replaces the legacy Scala implementation.
- [Midnight Indexer](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-1-0-1.md): Release notes for Midnight Indexer 1.0.1, which fixes wallet subscriptions restarting from index 0 and covers the move from the Scala to the Rust indexer.
- [Midnight Indexer](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-2-0-0.md): Release notes for Midnight Indexer 2.0.0, which retires the Scala-based indexer and makes breaking naming and wallet subscription changes to GraphQL API v1.
- [Midnight Indexer](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-2-1-0.md): Release notes for Midnight Indexer 2.1.0, which adds permissive CORS middleware to the GraphQL API so DApps running in a browser can access the indexer.
- [Midnight Indexer](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-2-1-1.md): Release notes for Midnight Indexer 2.1.1, a patch release that fixes queries so they return the correct transactions and contract actions.
- [Midnight Indexer](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-2-1-2.md): Release notes for Midnight Indexer 2.1.2, which makes the viewing update stream infinite, works with Midnight node 0.12, and requires wallet-sdk 5.0.0 or later.
- [Midnight Indexer](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-2-1-3.md): Release notes for Midnight Indexer 2.1.3, which extends PostgreSQL pool configuration and minimizes storage access, and is compatible with Midnight node 0.12.
- [Midnight Indexer](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-2-1-4.md): Release notes for Midnight Indexer 2.1.4, which fixes index calculation for standalone mode and a race condition when saving relevant transactions.
- [Midnight Indexer v3.0.0 release notes](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-3-0-0.md): Release notes for Midnight Indexer v3.0.0
- [Midnight Indexer v3.1.0 release notes](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-3-1-0.md): Release notes for Midnight Indexer v3.1.0, adding Node 0.21 support, SPO indexing, ledger storage for ledger state, and wallet streaming fixes.
- [Midnight Indexer v4.0.0 release notes](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-4-0-0.md): Release notes for Midnight Indexer v4.0.0
- [Midnight Indexer v4.0.1 release notes](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-4-0-1.md): Release notes for Midnight Indexer v4.0.1, a hotfix for the timestamp unit mismatch in the currentCapacity calculation.
- [Midnight Indexer v4.0.2 release notes](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-4-0-2.md): Release notes for Midnight Indexer v4.0.2, which corrects the paidFees computation and requires a re-index from genesis.
- [Midnight Indexer v4.1.0 release notes](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-4-1-0.md): Release notes for Midnight Indexer v4.1.0, adding Node 1.0 and ledger 8.1 support, the zswapMerkleTreeCollapsedUpdate query, dust wallet sync endpoints, the shieldedNullifierTransactions subscription, and a fee field.
- [Midnight Indexer v4.2.0 release notes](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-4-2-0.md): Release notes for Midnight Indexer v4.2.0, adding the dustGenerationMerkleTreeUpdate query, dust generation fields, a block fullness fix, and faster chain indexing.
- [Midnight Indexer v4.2.1 release notes](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-4-2-1.md): Release notes for Midnight Indexer v4.2.1, a dependency release that moves to Node 1.0.0-rc.3 and storage-core 1.2.0-rc.3.
- [Midnight Indexer v4.3.0 release notes](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-4-3-0.md): Release notes for Midnight Indexer v4.3.0, adding the SPO indexer to standalone mode, --version without config, dtime updates in the dustGenerations subscription, and three bug fixes.
- [Midnight Indexer v4.3.1 release notes](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-4-3-1.md): Release notes for Midnight Indexer v4.3.1, adding transactionHash to the dust and shielded nullifier subscription types and replacing remaining production panics with error returns.
- [Midnight Indexer v4.3.2 release notes](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-4-3-2.md): The Midnight Indexer indexes on-chain data and exposes it through a GraphQL API for DApps and wallets on the Midnight Network.
- [Midnight Indexer v4.3.3 release notes](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-4-3-3.md): Release notes for Midnight Indexer v4.3.3, including subscription quotas, the @beta directive, per-tree end indexes, and bug fixes.
- [Midnight Indexer v4.3.7 release notes](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-4-3-7.md): Release notes for Midnight Indexer v4.3.7, a Mainnet maintenance patch that fixes the DUST validity window and keeps the runtime-upgrade enactment-block tolerance.
- [Midnight Indexer v4.4.101 release notes](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-4-4-101.md): Release notes for Midnight Indexer v4.4.101, the indexer line for the ledger 9 network upgrade on Mainnet. Operators must re-index from genesis.

### midnight-js

Release notes for Midnight.js, the client library that helps developers build decentralized applications on the Midnight blockchain.

- [Midnight.js](https://docs.midnight.network/relnotes/midnight-js.md): Release notes for Midnight.js, the client library that helps developers build decentralized applications on the Midnight blockchain.
- [Midnight.js](https://docs.midnight.network/relnotes/midnight-js/midnight-js-0-2-5.md): Release notes for Midnight.js 0.2.5, which updates dependency ranges for ledger and Zswap to ^3.0.6, DApp Connector API to ^1.2.3, and wallet to ^3.7.5.
- [Midnight.js](https://docs.midnight.network/relnotes/midnight-js/midnight-js-1-0-0.md): Release notes for Midnight.js 1.0.0, which supports Wallet 4.0.0 with Bech32m addresses by default and lets PrivateStateProvider hold multiple private states.
- [Midnight.js](https://docs.midnight.network/relnotes/midnight-js/midnight-js-2-0-0.md): Release notes for Midnight.js 2.0.0, which adds BLS support and renames contract to contractAction in the GraphQL schema, along with dependency updates.
- [Midnight.js](https://docs.midnight.network/relnotes/midnight-js/midnight-js-2-0-1.md): Release notes for Midnight.js 2.0.1, which treats EncPublicKey values as Bech32m strings and makes createUnprovenOutput in zSwap-utils use segment 0.
- [Midnight.js](https://docs.midnight.network/relnotes/midnight-js/midnight-js-2-0-2.md): Release notes for Midnight.js 2.0.2 (11 June 2025), which adds a captcha header to faucet requests and ensures the segment number defaults to 0.
- [Midnight.js](https://docs.midnight.network/relnotes/midnight-js/midnight-js-2-1-0.md): Release notes for Midnight.js 2.1.0, which adds block time support, upgrades to Compact compiler 0.26.0 and compact-runtime 0.9.0, and applies security patches.
- [Midnight.js v3.0.0 release notes](https://docs.midnight.network/relnotes/midnight-js/midnight-js-3-0-0.md): Release notes for Midnight.js 3.0.0, which adds encrypted private state storage and makes transaction submission and contract calls asynchronous.
- [Midnight.js v3.2.0 release notes](https://docs.midnight.network/relnotes/midnight-js/midnight-js-3-2-0.md): Release notes for Midnight.js 3.2.0, which requires accountId for private state storage and adds export, import, and password rotation APIs.
- [Midnight.js v4.0.1 release notes](https://docs.midnight.network/relnotes/midnight-js/midnight-js-4-0-1.md): Release notes for Midnight.js 4.0.1, which introduces the v4 line with ledger v8 support and passes LedgerParameters through circuit execution.
- [Midnight.js v4.0.2 release notes](https://docs.midnight.network/relnotes/midnight-js/midnight-js-4-0-2.md): Release notes for Midnight.js 4.0.2, which reverts the 4.0.1 addCalls transaction path that broke shielded coin flows and adds custom proof server headers.
- [Midnight.js v4.0.3 release notes](https://docs.midnight.network/relnotes/midnight-js/midnight-js-4-0-3.md): Release notes for Midnight.js 4.0.3, which adds a barrel package and a DApp Connector proof provider but was never fully published to npm, so use 4.0.4.
- [Midnight.js v4.0.4 release notes](https://docs.midnight.network/relnotes/midnight-js/midnight-js-4-0-4.md): Release notes for Midnight.js 4.0.4, which ships the 4.0.3 barrel package and DApp Connector proof provider and encrypts each coin output for its recipient.
- [Midnight.js v4.1.0 release notes](https://docs.midnight.network/relnotes/midnight-js/midnight-js-4-1-0.md): Release notes for Midnight.js v4.1.0, covering the async StorageEncryption API, the midnight-js-protocol package for version-agnostic protocol imports, Web Crypto and React Native support, and security fixes.
- [Midnight.js v4.1.1 release notes](https://docs.midnight.network/relnotes/midnight-js/midnight-js-4-1-1.md): Release notes for Midnight.js v4.1.1, covering breaking error handling changes, security hardening, and qanet support in testkit-js.
- [Midnight.js v5.0.0 release notes](https://docs.midnight.network/relnotes/midnight-js/midnight-js-5-0-0.md): Release notes for Midnight.js v5.0.0, the major release for the ledger 9 network upgrade on Mainnet, with breaking provider and contract state changes.

### midnight-wallet-api

Release notes for the deprecated Wallet API, with a notice that the Wallet SDK now handles wallet functionality and that integrations should migrate to it.

- [Wallet API](https://docs.midnight.network/relnotes/midnight-wallet-api.md): Release notes for the deprecated Wallet API, with a notice that the Wallet SDK now handles wallet functionality and that integrations should migrate to it.
- [Midnight Wallet API](https://docs.midnight.network/relnotes/midnight-wallet-api/midnight-wallet-api-3-5-0.md): Release notes for Midnight Wallet API 3.5.0, the @midnight-ntwrk/wallet-api package, which exposes coin nullifiers in the wallet state.
- [Midnight Wallet API](https://docs.midnight.network/relnotes/midnight-wallet-api/midnight-wallet-api-3-6-0.md): Release notes for Midnight Wallet API 3.6.0, which exposes additional fields for Bech32 addresses and updates the API documentation.
- [Midnight Wallet API](https://docs.midnight.network/relnotes/midnight-wallet-api/midnight-wallet-api-4-0-0.md): Release notes for Midnight Wallet API 4.0.0, which makes Bech32m the default address format and warns that wallet balances are not visible after upgrading.
- [Midnight Wallet API](https://docs.midnight.network/relnotes/midnight-wallet-api/midnight-wallet-api-5-0-0.md): Release notes for Midnight Wallet API 5.0.0, which adds BLS support and a sync progress API for wallet developers, integration engineers, and node operators.

### network

Midnight Preview, Preprod, and Mainnet endpoints, including the Blockfrost Preprod and Mainnet indexer and node RPC that need a project token.

- [Environments and endpoints](https://docs.midnight.network/relnotes/network.md): Midnight Preview, Preprod, and Mainnet endpoints, including the Blockfrost Preprod and Mainnet indexer and node RPC that need a project token.

### node

Release notes for the Midnight node, a core component of the Midnight Network that syncs, validates transactions, and maintains the chain state.

- [Node](https://docs.midnight.network/relnotes/node.md): Release notes for the Midnight node, a core component of the Midnight Network that syncs, validates transactions, and maintains the chain state.
- [Node](https://docs.midnight.network/relnotes/node/node-0-12.md): Release notes for Midnight node 0.12.0, which switches from Pluto-Eris to BLS12-381, moves to the new storage model, and requires a chain reset of testnet-02.
- [Node](https://docs.midnight.network/relnotes/node/node-0-12-1.md): Release notes for Midnight node 0.12.1, which is backwards compatible with 0.12.0 and deprecates NODE_KEY and SEED_PHRASE in favor of mounted secrets.
- [Node v0.18.0 release notes](https://docs.midnight.network/relnotes/node/node-0-18-0.md): Release notes for Midnight node 0.18.0, which adds unshielded transactions and DUST fees generated from NIGHT, with breaking changes that require a chain reset.
- [Node v0.20.0 release notes](https://docs.midnight.network/relnotes/node/node-0-20-0.md): Release notes for Midnight node 0.20.0, which removes runtime sudo in favor of governance-based workflows and upgrades to Ledger 7.0.0.
- [Node v0.22.0 release notes](https://docs.midnight.network/relnotes/node/node-0-22-0.md): Release notes for Midnight node 0.22.0, which upgrades to Ledger 8.0.2 and moves to ledger storage layout v2, so nodes must resync from genesis or a snapshot.
- [Node v0.22.1 release notes](https://docs.midnight.network/relnotes/node/node-0-22-1.md): Release notes for Midnight node 0.22.1, a security patch that fixes a Yamux networking vulnerability and restricts peer info RPC methods to unsafe-only access.
- [Node v0.22.3 release notes](https://docs.midnight.network/relnotes/node/node-0-22-3.md): Release notes for Midnight node 0.22.3, a config-only patch release that regenerates the Preview genesis state and chain specifications.
- [Node v0.22.5 release notes](https://docs.midnight.network/relnotes/node/node-0-22-5.md): Release notes for Midnight node 0.22.5, which adds an optional gas cost limit for transactions entering the pool and reduces db-sync load from cNIGHT queries.
- [Node](https://docs.midnight.network/relnotes/node/node-0-3-3-RC1.md): Devnet release notes for Midnight node 0.3.3 RC1, which makes the seed parameter required and updates the chain spec format for the public devnet.
- [Node](https://docs.midnight.network/relnotes/node/node-0-3-3-RC2.md): Release notes for Midnight node 0.3.3 RC2, which updates the node to Sidechains v0.0.26, adds upgrade scripts, and replaces the chain name with a network ID.
- [Node](https://docs.midnight.network/relnotes/node/node-0-3-5-RC1.md): Release notes for Midnight node 0.3.5 RC1 (Plonk-only), which speeds up transaction processing by removing an extra deserialization check.
- [Node](https://docs.midnight.network/relnotes/node/node-0-3-6-RC1.md): Release notes for Midnight node 0.3.6 RC1 for the Plonk-based qa-net and devnet, which makes the seed argument optional and adds a d parameter override.
- [Node](https://docs.midnight.network/relnotes/node/node-0-6-3.md): Testnet release notes for Midnight node 0.6.3 RC, which integrates HALO2 support, upgrades to ledger 3.0.0-beta.1, and renames SEED back to SEED_PHRASE.
- [Node](https://docs.midnight.network/relnotes/node/node-0-6-4.md): Testnet release notes for Midnight node 0.6.4, which quadruples the maximum allocation limit and restricts ledger state access to native platforms.
- [Node](https://docs.midnight.network/relnotes/node/node-0-6-6.md): Release notes for Midnight node 0.6.6, an emergency Testnet release that prepares for an upcoming Cardano hardfork by updating to partnerchains 1.2.0.
- [Node](https://docs.midnight.network/relnotes/node/node-0-7-0.md): Release notes for Midnight node 0.7.0, which reworks RPC error handling, enables Ledger API v2, and adds governance to pause and unpause extrinsic calls.
- [Node](https://docs.midnight.network/relnotes/node/node-0-7-0-RC1.md): Release notes for Midnight node 0.7.0 RC1, which adds transaction pausing and block rewards and exposes the transaction cost model and ledger parameters.
- [Node](https://docs.midnight.network/relnotes/node/node-0-7-0-RC2.md): Release notes for Midnight node 0.7.0 RC2, which exposes the upgrade vote threshold configuration and adds known bootnodes to chain specs.
- [Node](https://docs.midnight.network/relnotes/node/node-0-7-0-RC3.md): Release notes for Midnight node 0.7.0 RC3, which lets RPC endpoints process multiple versions of the MidnightRuntimeApi without compatibility issues.
- [Node](https://docs.midnight.network/relnotes/node/node-0-7-0-RC5.md): Release notes for Midnight node 0.7.0 RC5, which restores transaction validation, fixes its memory consumption, and adds bootnode addresses to chain specs.
- [Node](https://docs.midnight.network/relnotes/node/node-0-7-1-RC1.md): Release notes for Midnight node 0.7.1 RC1, a patch release that expands the LRU cache size for state management and fixes CI so Docker images publish again.
- [Node](https://docs.midnight.network/relnotes/node/node-0-8-0.md): Release notes for Midnight node 0.8.0, with Prometheus metrics for the core ledger, an upgrade to ledger 3.0.6, and release configuration for testnet-02.
- [Node](https://docs.midnight.network/relnotes/node/node-0-8-0-RC1.md): Release notes for Midnight node 0.8.0 RC1, which upgrades to ledger 3.0.5, adds Prometheus metrics to the ledger, and requires Rust compiler 1.83.
- [Node](https://docs.midnight.network/relnotes/node/node-0-8-0-RC2.md): Release notes for Midnight node 0.8.0 RC2, which moves to ledger 3.0.6, requires regenerated qanet and undeployed genesis files, and adds Prometheus metrics.
- [Node](https://docs.midnight.network/relnotes/node/node-0-8-0-RC3.md): Release notes for Midnight node 0.8.0 RC3, a pre-release that adds Prometheus ledger metrics and testnet-02 configurations and upgrades the ledger to 3.0.6.
- [Node](https://docs.midnight.network/relnotes/node/node-0-8-0-RC4.md): Release notes for Midnight node 0.8.0 RC4, a pre-release that adds support for testnet-02 and upgrades the ledger to 3.0.5 and then 3.0.6.
- [Node v1.0.0 release notes](https://docs.midnight.network/relnotes/node/node-1-0-0.md): Release notes for Midnight node 1.0.0, the Mainnet GA release, which requires a runtime upgrade and adds a self-describing rpc.discover JSON-RPC endpoint.
- [Node v1.0.1 release notes](https://docs.midnight.network/relnotes/node/node-1-0-1.md): Release notes for Midnight node 1.0.1, which fixes the boot error on nodes without a TOML configuration file and regenerates the Preview genesis and chain spec.
- [Node v1.0.2 release notes](https://docs.midnight.network/relnotes/node/node-1-0-2.md): Release notes for Midnight node 1.0.2, a dependency and security patch that moves to ledger 8.1.2 and clears npm audit findings in the toolkit image.
- [Node v1.0.300 release notes](https://docs.midnight.network/relnotes/node/node-1-0-300.md): Release notes for Midnight node 1.0.300, which ships runtime 1.0.300 (older nodes cannot load it) and adds a block timestamp correction for historical blocks.
- [Node v1.0.400 release notes](https://docs.midnight.network/relnotes/node/node-1-0-400.md): Release notes for Midnight node 1.0.400, a security patch that moves to ledger 8.1.3 and fixes Mainnet sync from genesis stopping at block 1788979.
- [Node v2.1.101 release notes](https://docs.midnight.network/relnotes/node/node-2-1-101.md): Release notes for Midnight node 2.1.101, the node release that every Mainnet validator and RPC node must run before the ledger 9 network upgrade.
- [Node](https://docs.midnight.network/relnotes/node/node-devnet-1-0.md): Release notes for Midnight 1.0 devnet (10 January 2024), the first devnet release of the Midnight node, which handles syncing and transaction validation.

### onchain-runtime

Release notes for on-chain runtime 4.0.0, the last release listed separately. Starting with Ledger 7.0.0, on-chain runtime versions appear on the Ledger page.

- [Onchain Runtime](https://docs.midnight.network/relnotes/onchain-runtime.md): Release notes for on-chain runtime 4.0.0, the last release listed separately. Starting with Ledger 7.0.0, on-chain runtime versions appear on the Ledger page.
- [Onchain Runtime](https://docs.midnight.network/relnotes/onchain-runtime/onchain-runtime-4-0-0.md): Release notes for Midnight Onchain Runtime 4.0.0, a stable release that switches from Pluto-Eris to BLS12-381 and adds segment IDs to Zswap constructors.

### overview

Overview of the latest stable Midnight release, with links to the network endpoints, the compatibility matrix, and the release notes for each component.

- [Latest stable release](https://docs.midnight.network/relnotes/overview.md): Overview of the latest stable Midnight release, with links to the network endpoints, the compatibility matrix, and the release notes for each component.

### proof-server

Release notes for Midnight proof server 3.0.6 to 4.0.0. Starting with Ledger 7.0.0, proof server releases appear on the Ledger page.

- [Proof Server](https://docs.midnight.network/relnotes/proof-server.md): Release notes for Midnight proof server 3.0.6 to 4.0.0. Starting with Ledger 7.0.0, proof server releases appear on the Ledger page.
- [Proof Server](https://docs.midnight.network/relnotes/proof-server/proof-server-3-0-6.md): Release notes for Midnight proof server 3.0.6, which adds coinCommitment and coinNullifier functions and fixes security bugs in Schnorr proofs.
- [Proof Server](https://docs.midnight.network/relnotes/proof-server/proof-server-3-0-7.md): Release notes for Midnight proof server 3.0.7, which removes excessive debug printing from its logs and fixes the performance regression that logging caused.
- [Proof Server](https://docs.midnight.network/relnotes/proof-server/proof-server-4-0-0.md): Release notes for Midnight proof server 4.0.0, which switches from Pluto-Eris to BLS12-381 and uses data providers instead of direct prover keys and parameters.

### security-audits

Independent security audit reports for Midnight components, with the audited code revisions and SHA-256 checksums for each report.

- [Security audits](https://docs.midnight.network/relnotes/security-audits.md): Independent security audit reports for Midnight components, with the audited code revisions and SHA-256 checksums for each report.

### support-matrix

Compatibility matrix, or support matrix, of the Midnight node, Compact, SDK, indexer, and proof server versions tested on Preview, Preprod, and Mainnet.

- [Compatibility matrix](https://docs.midnight.network/relnotes/support-matrix.md): Compatibility matrix, or support matrix, of the Midnight node, Compact, SDK, indexer, and proof server versions tested on Preview, Preprod, and Mainnet.

### wallet

Release notes for the Midnight Wallet SDK, listed by umbrella package version from 1.0.0 onward and by facade sub-package version for earlier entries.

- [Wallet SDK](https://docs.midnight.network/relnotes/wallet.md): Release notes for the Midnight Wallet SDK, listed by umbrella package version from 1.0.0 onward and by facade sub-package version for earlier entries.
- [Wallet SDK v1.0.0 release notes](https://docs.midnight.network/relnotes/wallet/wallet-1-0-0.md): Release notes for the Midnight Wallet SDK 1.0.0 (January 2026), the initial stable release, with shielded, unshielded, and DUST wallets behind one facade API.
- [Wallet SDK v2.0.0 release notes](https://docs.midnight.network/relnotes/wallet/wallet-2-0-0.md): Release notes for the Midnight Wallet SDK 2.0.0
- [Wallet SDK v3.0.0 release notes](https://docs.midnight.network/relnotes/wallet/wallet-3-0-0.md): Release notes for the Midnight Wallet SDK 3.0.0
- [Wallet SDK v1.0.0 release notes](https://docs.midnight.network/relnotes/wallet/wallet-sdk-1-0-0.md): Release notes for the Midnight Wallet SDK 1.0.0 umbrella package (April 2026), which adds DUST wallet transaction history and a shared history storage layer.
- [Wallet SDK v1.1.0 release notes](https://docs.midnight.network/relnotes/wallet/wallet-sdk-1-1-0.md): Release notes for the Midnight Wallet SDK 1.1.0
- [Wallet SDK v1.2.0 release notes](https://docs.midnight.network/relnotes/wallet/wallet-sdk-1-2-0.md): Release notes for the Midnight Wallet SDK 1.2.0
- [Wallet SDK v2.0.0 release notes](https://docs.midnight.network/relnotes/wallet/wallet-sdk-2-0-0.md): Release notes for the Midnight Wallet SDK 2.0.0, the major release for the ledger 9 network upgrade on Mainnet.

## glossary

Glossary of terms used in Midnight Network.

- [Midnight Network Glossary](https://docs.midnight.network/glossary.md): Glossary of terms used in Midnight Network.

## troubleshoot

Find fixes for Midnight installation problems, Compact compiler errors, version mismatches, and rejected transactions, and learn where to get help.

- [Troubleshoot](https://docs.midnight.network/troubleshoot.md): Find fixes for Midnight installation problems, Compact compiler errors, version mismatches, and rejected transactions, and learn where to get help.

### compiler-errors

Common Compact compiler errors and their solutions, including lexer, parser, witness, disclosure, ZKIR, and runtime errors with fix instructions.

- [Troubleshoot compiler errors](https://docs.midnight.network/troubleshoot/compiler-errors.md): Common Compact compiler errors and their solutions, including lexer, parser, witness, disclosure, ZKIR, and runtime errors with fix instructions.

### faq

Frequently asked questions about Midnight Network.

- [Frequently asked questions](https://docs.midnight.network/troubleshoot/faq.md): Frequently asked questions about Midnight Network.

### getting-help

Get help with Midnight Network development and troubleshooting.

- [Support](https://docs.midnight.network/troubleshoot/getting-help.md): Get help with Midnight Network development and troubleshooting.

### installation-problems

Fix Midnight toolchain installation problems, such as a compact command that your shell cannot find, Docker not running, and port 6300 in use.

- [Troubleshoot installation problems](https://docs.midnight.network/troubleshoot/installation-problems.md): Fix Midnight toolchain installation problems, such as a compact command that your shell cannot find, Docker not running, and port 6300 in use.

### troubleshoot-compact-nixos

Troubleshoot Midnight Compact tools installation issue on NixOS

- [Troubleshoot Compact toolchain installation issue on NixOS](https://docs.midnight.network/troubleshoot/troubleshoot-compact-nixos.md): Troubleshoot Midnight Compact tools installation issue on NixOS

## api-reference

Complete reference for Midnight TypeScript APIs, covering smart contracts, wallets, indexer, and blockchain interactions.

- [API reference](https://docs.midnight.network/api-reference.md): Complete reference for Midnight TypeScript APIs, covering smart contracts, wallets, indexer, and blockchain interactions.

### compact-runtime

@midnight-ntwrk/compact-runtime v0.21.0

- [Compact runtime API](https://docs.midnight.network/api-reference/compact-runtime.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ChargedState](https://docs.midnight.network/api-reference/compact-runtime/classes/ChargedState.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactError](https://docs.midnight.network/api-reference/compact-runtime/classes/CompactError.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeBoolean](https://docs.midnight.network/api-reference/compact-runtime/classes/CompactTypeBoolean.md): @midnight-ntwrk/compact-runtime v0.9.0
- [CompactTypeBytes](https://docs.midnight.network/api-reference/compact-runtime/classes/CompactTypeBytes.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeCurvePoint](https://docs.midnight.network/api-reference/compact-runtime/classes/CompactTypeCurvePoint.md): @midnight-ntwrk/compact-runtime v0.9.0
- [CompactTypeEnum](https://docs.midnight.network/api-reference/compact-runtime/classes/CompactTypeEnum.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeField](https://docs.midnight.network/api-reference/compact-runtime/classes/CompactTypeField.md): @midnight-ntwrk/compact-runtime v0.9.0
- [CompactTypeMerkleTreeDigest](https://docs.midnight.network/api-reference/compact-runtime/classes/CompactTypeMerkleTreeDigest.md): @midnight-ntwrk/compact-runtime v0.9.0
- [CompactTypeMerkleTreePath](https://docs.midnight.network/api-reference/compact-runtime/classes/CompactTypeMerkleTreePath.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeMerkleTreePathEntry](https://docs.midnight.network/api-reference/compact-runtime/classes/CompactTypeMerkleTreePathEntry.md): @midnight-ntwrk/compact-runtime v0.9.0
- [CompactTypeOpaqueString](https://docs.midnight.network/api-reference/compact-runtime/classes/CompactTypeOpaqueString.md): @midnight-ntwrk/compact-runtime v0.9.0
- [CompactTypeOpaqueUint8Array](https://docs.midnight.network/api-reference/compact-runtime/classes/CompactTypeOpaqueUint8Array.md): @midnight-ntwrk/compact-runtime v0.9.0
- [CompactTypeUnsignedInteger](https://docs.midnight.network/api-reference/compact-runtime/classes/CompactTypeUnsignedInteger.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeVector](https://docs.midnight.network/api-reference/compact-runtime/classes/CompactTypeVector.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ContractInterfaceMismatchError](https://docs.midnight.network/api-reference/compact-runtime/classes/ContractInterfaceMismatchError.md): @midnight-ntwrk/compact-runtime v0.19.0
- [ContractMaintenanceAuthority](https://docs.midnight.network/api-reference/compact-runtime/classes/ContractMaintenanceAuthority.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ContractOperation](https://docs.midnight.network/api-reference/compact-runtime/classes/ContractOperation.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ContractState](https://docs.midnight.network/api-reference/compact-runtime/classes/ContractState.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CostModel](https://docs.midnight.network/api-reference/compact-runtime/classes/CostModel.md): @midnight-ntwrk/compact-runtime v0.21.0
- [QueryContext](https://docs.midnight.network/api-reference/compact-runtime/classes/QueryContext.md): @midnight-ntwrk/compact-runtime v0.21.0
- [QueryResults](https://docs.midnight.network/api-reference/compact-runtime/classes/QueryResults.md): @midnight-ntwrk/compact-runtime v0.21.0
- [StateBoundedMerkleTree](https://docs.midnight.network/api-reference/compact-runtime/classes/StateBoundedMerkleTree.md): @midnight-ntwrk/compact-runtime v0.21.0
- [StateMap](https://docs.midnight.network/api-reference/compact-runtime/classes/StateMap.md): @midnight-ntwrk/compact-runtime v0.21.0
- [StateValue](https://docs.midnight.network/api-reference/compact-runtime/classes/StateValue.md): @midnight-ntwrk/compact-runtime v0.21.0
- [VmResults](https://docs.midnight.network/api-reference/compact-runtime/classes/VmResults.md): @midnight-ntwrk/compact-runtime v0.21.0
- [VmStack](https://docs.midnight.network/api-reference/compact-runtime/classes/VmStack.md): @midnight-ntwrk/compact-runtime v0.21.0
- [NetworkId](https://docs.midnight.network/api-reference/compact-runtime/enumerations/NetworkId.md): @midnight-ntwrk/compact-runtime v0.9.0
- [addField](https://docs.midnight.network/api-reference/compact-runtime/functions/addField.md): @midnight-ntwrk/compact-runtime v0.21.0
- [alignedConcat](https://docs.midnight.network/api-reference/compact-runtime/functions/alignedConcat.md): @midnight-ntwrk/compact-runtime v0.21.0
- [assert](https://docs.midnight.network/api-reference/compact-runtime/functions/assert.md): @midnight-ntwrk/compact-runtime v0.21.0
- [assertDefined](https://docs.midnight.network/api-reference/compact-runtime/functions/assertDefined.md): @midnight-ntwrk/compact-runtime v0.21.0
- [assertIsContractAddress](https://docs.midnight.network/api-reference/compact-runtime/functions/assertIsContractAddress.md): @midnight-ntwrk/compact-runtime v0.21.0
- [assertUndefined](https://docs.midnight.network/api-reference/compact-runtime/functions/assertUndefined.md): @midnight-ntwrk/compact-runtime v0.21.0
- [asVerifierKeyHash](https://docs.midnight.network/api-reference/compact-runtime/functions/asVerifierKeyHash.md): @midnight-ntwrk/compact-runtime v0.21.0
- [bigIntModFr](https://docs.midnight.network/api-reference/compact-runtime/functions/bigIntModFr.md): @midnight-ntwrk/compact-runtime v0.21.0
- [bigIntToValue](https://docs.midnight.network/api-reference/compact-runtime/functions/bigIntToValue.md): @midnight-ntwrk/compact-runtime v0.21.0
- [checkProofData](https://docs.midnight.network/api-reference/compact-runtime/functions/checkProofData.md): @midnight-ntwrk/compact-runtime v0.9.0
- [checkRuntimeVersion](https://docs.midnight.network/api-reference/compact-runtime/functions/checkRuntimeVersion.md): @midnight-ntwrk/compact-runtime v0.21.0
- [coinCommitment](https://docs.midnight.network/api-reference/compact-runtime/functions/coinCommitment.md): @midnight-ntwrk/compact-runtime v0.9.0
- [communicationCommitment](https://docs.midnight.network/api-reference/compact-runtime/functions/communicationCommitment.md): @midnight-ntwrk/compact-runtime v0.21.0
- [communicationCommitmentRandomness](https://docs.midnight.network/api-reference/compact-runtime/functions/communicationCommitmentRandomness.md): @midnight-ntwrk/compact-runtime v0.21.0
- [constructJubjubPoint](https://docs.midnight.network/api-reference/compact-runtime/functions/constructJubjubPoint.md): @midnight-ntwrk/compact-runtime v0.21.0
- [constructorContext](https://docs.midnight.network/api-reference/compact-runtime/functions/constructorContext.md): @midnight-ntwrk/compact-runtime v0.9.0
- [contractDependencies](https://docs.midnight.network/api-reference/compact-runtime/functions/contractDependencies.md): @midnight-ntwrk/compact-runtime v0.19.0
- [convert_bigint_to_Uint8Array](https://docs.midnight.network/api-reference/compact-runtime/functions/convert_bigint_to_Uint8Array.md): @midnight-ntwrk/compact-runtime v0.8.1
- [convert_Uint8Array_to_bigint](https://docs.midnight.network/api-reference/compact-runtime/functions/convert_Uint8Array_to_bigint.md): @midnight-ntwrk/compact-runtime v0.8.1
- [convertBigintToBytes](https://docs.midnight.network/api-reference/compact-runtime/functions/convertBigintToBytes.md): @midnight-ntwrk/compact-runtime v0.21.0
- [convertBytesToField](https://docs.midnight.network/api-reference/compact-runtime/functions/convertBytesToField.md): @midnight-ntwrk/compact-runtime v0.21.0
- [convertBytesToUint](https://docs.midnight.network/api-reference/compact-runtime/functions/convertBytesToUint.md): @midnight-ntwrk/compact-runtime v0.21.0
- [convertFieldToBytes](https://docs.midnight.network/api-reference/compact-runtime/functions/convertFieldToBytes.md): @midnight-ntwrk/compact-runtime v0.16.0
- [convertNumericToJubjubScalar](https://docs.midnight.network/api-reference/compact-runtime/functions/convertNumericToJubjubScalar.md): @midnight-ntwrk/compact-runtime v0.21.0
- [copyCircuitContext](https://docs.midnight.network/api-reference/compact-runtime/functions/copyCircuitContext.md): @midnight-ntwrk/compact-runtime v0.21.0
- [createCallContext](https://docs.midnight.network/api-reference/compact-runtime/functions/createCallContext.md): @midnight-ntwrk/compact-runtime v0.21.0
- [createCircuitContext](https://docs.midnight.network/api-reference/compact-runtime/functions/createCircuitContext.md): @midnight-ntwrk/compact-runtime v0.21.0
- [createConstructorContext](https://docs.midnight.network/api-reference/compact-runtime/functions/createConstructorContext.md): @midnight-ntwrk/compact-runtime v0.21.0
- [createInitialQueryContext](https://docs.midnight.network/api-reference/compact-runtime/functions/createInitialQueryContext.md): @midnight-ntwrk/compact-runtime v0.21.0
- [createWitnessContext](https://docs.midnight.network/api-reference/compact-runtime/functions/createWitnessContext.md): @midnight-ntwrk/compact-runtime v0.21.0
- [createZswapInput](https://docs.midnight.network/api-reference/compact-runtime/functions/createZswapInput.md): @midnight-ntwrk/compact-runtime v0.21.0
- [createZswapOutput](https://docs.midnight.network/api-reference/compact-runtime/functions/createZswapOutput.md): @midnight-ntwrk/compact-runtime v0.21.0
- [crossContractCall](https://docs.midnight.network/api-reference/compact-runtime/functions/crossContractCall.md): @midnight-ntwrk/compact-runtime v0.21.0
- [decodeCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/functions/decodeCoinInfo.md): @midnight-ntwrk/compact-runtime v0.9.0
- [decodeCoinPublicKey](https://docs.midnight.network/api-reference/compact-runtime/functions/decodeCoinPublicKey.md): @midnight-ntwrk/compact-runtime v0.21.0
- [decodeContractAddress](https://docs.midnight.network/api-reference/compact-runtime/functions/decodeContractAddress.md): @midnight-ntwrk/compact-runtime v0.21.0
- [decodeQualifiedCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/functions/decodeQualifiedCoinInfo.md): @midnight-ntwrk/compact-runtime v0.9.0
- [decodeQualifiedShieldedCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/functions/decodeQualifiedShieldedCoinInfo.md): @midnight-ntwrk/compact-runtime v0.21.0
- [decodeRawTokenType](https://docs.midnight.network/api-reference/compact-runtime/functions/decodeRawTokenType.md): @midnight-ntwrk/compact-runtime v0.21.0
- [decodeRecipient](https://docs.midnight.network/api-reference/compact-runtime/functions/decodeRecipient.md): @midnight-ntwrk/compact-runtime v0.21.0
- [decodeShieldedCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/functions/decodeShieldedCoinInfo.md): @midnight-ntwrk/compact-runtime v0.21.0
- [decodeTokenType](https://docs.midnight.network/api-reference/compact-runtime/functions/decodeTokenType.md): @midnight-ntwrk/compact-runtime v0.9.0
- [decodeUserAddress](https://docs.midnight.network/api-reference/compact-runtime/functions/decodeUserAddress.md): @midnight-ntwrk/compact-runtime v0.21.0
- [decodeZswapLocalState](https://docs.midnight.network/api-reference/compact-runtime/functions/decodeZswapLocalState.md): @midnight-ntwrk/compact-runtime v0.21.0
- [degradeToTransient](https://docs.midnight.network/api-reference/compact-runtime/functions/degradeToTransient.md): @midnight-ntwrk/compact-runtime v0.21.0
- [dummyContractAddress](https://docs.midnight.network/api-reference/compact-runtime/functions/dummyContractAddress.md): @midnight-ntwrk/compact-runtime v0.21.0
- [dummyUserAddress](https://docs.midnight.network/api-reference/compact-runtime/functions/dummyUserAddress.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ecAdd](https://docs.midnight.network/api-reference/compact-runtime/functions/ecAdd.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ecMul](https://docs.midnight.network/api-reference/compact-runtime/functions/ecMul.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ecMulGenerator](https://docs.midnight.network/api-reference/compact-runtime/functions/ecMulGenerator.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ecNeg](https://docs.midnight.network/api-reference/compact-runtime/functions/ecNeg.md): @midnight-ntwrk/compact-runtime v0.21.0
- [emptyRunningCost](https://docs.midnight.network/api-reference/compact-runtime/functions/emptyRunningCost.md): @midnight-ntwrk/compact-runtime v0.21.0
- [emptyZswapLocalState](https://docs.midnight.network/api-reference/compact-runtime/functions/emptyZswapLocalState.md): @midnight-ntwrk/compact-runtime v0.21.0
- [encodeCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/functions/encodeCoinInfo.md): @midnight-ntwrk/compact-runtime v0.9.0
- [encodeCoinPublicKey](https://docs.midnight.network/api-reference/compact-runtime/functions/encodeCoinPublicKey.md): @midnight-ntwrk/compact-runtime v0.21.0
- [encodeContractAddress](https://docs.midnight.network/api-reference/compact-runtime/functions/encodeContractAddress.md): @midnight-ntwrk/compact-runtime v0.21.0
- [encodeQualifiedCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/functions/encodeQualifiedCoinInfo.md): @midnight-ntwrk/compact-runtime v0.9.0
- [encodeQualifiedShieldedCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/functions/encodeQualifiedShieldedCoinInfo.md): @midnight-ntwrk/compact-runtime v0.21.0
- [encodeRawTokenType](https://docs.midnight.network/api-reference/compact-runtime/functions/encodeRawTokenType.md): @midnight-ntwrk/compact-runtime v0.21.0
- [encodeRecipient](https://docs.midnight.network/api-reference/compact-runtime/functions/encodeRecipient.md): @midnight-ntwrk/compact-runtime v0.21.0
- [encodeShieldedCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/functions/encodeShieldedCoinInfo.md): @midnight-ntwrk/compact-runtime v0.21.0
- [encodeTokenType](https://docs.midnight.network/api-reference/compact-runtime/functions/encodeTokenType.md): @midnight-ntwrk/compact-runtime v0.9.0
- [encodeUserAddress](https://docs.midnight.network/api-reference/compact-runtime/functions/encodeUserAddress.md): @midnight-ntwrk/compact-runtime v0.21.0
- [encodeZswapLocalState](https://docs.midnight.network/api-reference/compact-runtime/functions/encodeZswapLocalState.md): @midnight-ntwrk/compact-runtime v0.21.0
- [entryPointHash](https://docs.midnight.network/api-reference/compact-runtime/functions/entryPointHash.md): @midnight-ntwrk/compact-runtime v0.21.0
- [finalizeCallProofData](https://docs.midnight.network/api-reference/compact-runtime/functions/finalizeCallProofData.md): @midnight-ntwrk/compact-runtime v0.21.0
- [fromHex](https://docs.midnight.network/api-reference/compact-runtime/functions/fromHex.md): @midnight-ntwrk/compact-runtime v0.21.0
- [hasCoinCommitment](https://docs.midnight.network/api-reference/compact-runtime/functions/hasCoinCommitment.md): @midnight-ntwrk/compact-runtime v0.21.0
- [hashToCurve](https://docs.midnight.network/api-reference/compact-runtime/functions/hashToCurve.md): @midnight-ntwrk/compact-runtime v0.21.0
- [isContractAddress](https://docs.midnight.network/api-reference/compact-runtime/functions/isContractAddress.md): @midnight-ntwrk/compact-runtime v0.21.0
- [isEncodedContractAddress](https://docs.midnight.network/api-reference/compact-runtime/functions/isEncodedContractAddress.md): @midnight-ntwrk/compact-runtime v0.19.0
- [isValidSecp256k1Point](https://docs.midnight.network/api-reference/compact-runtime/functions/isValidSecp256k1Point.md): @midnight-ntwrk/compact-runtime v0.21.0
- [isVerifierKeyHash](https://docs.midnight.network/api-reference/compact-runtime/functions/isVerifierKeyHash.md): @midnight-ntwrk/compact-runtime v0.21.0
- [jubjubPointX](https://docs.midnight.network/api-reference/compact-runtime/functions/jubjubPointX.md): @midnight-ntwrk/compact-runtime v0.21.0
- [jubjubPointY](https://docs.midnight.network/api-reference/compact-runtime/functions/jubjubPointY.md): @midnight-ntwrk/compact-runtime v0.21.0
- [jubjubSampleScalar](https://docs.midnight.network/api-reference/compact-runtime/functions/jubjubSampleScalar.md): @midnight-ntwrk/compact-runtime v0.21.0
- [jubjubSchnorrSign](https://docs.midnight.network/api-reference/compact-runtime/functions/jubjubSchnorrSign.md): @midnight-ntwrk/compact-runtime v0.21.0
- [jubjubSchnorrVerify](https://docs.midnight.network/api-reference/compact-runtime/functions/jubjubSchnorrVerify.md): @midnight-ntwrk/compact-runtime v0.21.0
- [jubjubSchnorrVerifyingKey](https://docs.midnight.network/api-reference/compact-runtime/functions/jubjubSchnorrVerifyingKey.md): @midnight-ntwrk/compact-runtime v0.21.0
- [keccak256](https://docs.midnight.network/api-reference/compact-runtime/functions/keccak256.md): @midnight-ntwrk/compact-runtime v0.21.0
- [leafHash](https://docs.midnight.network/api-reference/compact-runtime/functions/leafHash.md): @midnight-ntwrk/compact-runtime v0.21.0
- [maxAlignedSize](https://docs.midnight.network/api-reference/compact-runtime/functions/maxAlignedSize.md): @midnight-ntwrk/compact-runtime v0.21.0
- [maxField](https://docs.midnight.network/api-reference/compact-runtime/functions/maxField.md): @midnight-ntwrk/compact-runtime v0.21.0
- [mulField](https://docs.midnight.network/api-reference/compact-runtime/functions/mulField.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ownPublicKey](https://docs.midnight.network/api-reference/compact-runtime/functions/ownPublicKey.md): @midnight-ntwrk/compact-runtime v0.21.0
- [persistentCommit](https://docs.midnight.network/api-reference/compact-runtime/functions/persistentCommit.md): @midnight-ntwrk/compact-runtime v0.21.0
- [persistentHash](https://docs.midnight.network/api-reference/compact-runtime/functions/persistentHash.md): @midnight-ntwrk/compact-runtime v0.21.0
- [proofDataIntoSerializedPreimage](https://docs.midnight.network/api-reference/compact-runtime/functions/proofDataIntoSerializedPreimage.md): @midnight-ntwrk/compact-runtime v0.21.0
- [queryLedgerState](https://docs.midnight.network/api-reference/compact-runtime/functions/queryLedgerState.md): @midnight-ntwrk/compact-runtime v0.21.0
- [rawTokenType](https://docs.midnight.network/api-reference/compact-runtime/functions/rawTokenType.md): @midnight-ntwrk/compact-runtime v0.21.0
- [runProgram](https://docs.midnight.network/api-reference/compact-runtime/functions/runProgram.md): @midnight-ntwrk/compact-runtime v0.21.0
- [runtimeCoinCommitment](https://docs.midnight.network/api-reference/compact-runtime/functions/runtimeCoinCommitment.md): @midnight-ntwrk/compact-runtime v0.21.0
- [sampleContractAddress](https://docs.midnight.network/api-reference/compact-runtime/functions/sampleContractAddress.md): @midnight-ntwrk/compact-runtime v0.21.0
- [sampleRawTokenType](https://docs.midnight.network/api-reference/compact-runtime/functions/sampleRawTokenType.md): @midnight-ntwrk/compact-runtime v0.21.0
- [sampleSigningKey](https://docs.midnight.network/api-reference/compact-runtime/functions/sampleSigningKey.md): @midnight-ntwrk/compact-runtime v0.21.0
- [sampleTokenType](https://docs.midnight.network/api-reference/compact-runtime/functions/sampleTokenType.md): @midnight-ntwrk/compact-runtime v0.9.0
- [sampleUserAddress](https://docs.midnight.network/api-reference/compact-runtime/functions/sampleUserAddress.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1Add](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1Add.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1BaseAdd](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1BaseAdd.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1BaseInv](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1BaseInv.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1BaseMul](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1BaseMul.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1BaseNeg](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1BaseNeg.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1BaseSub](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1BaseSub.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1EcdsaRecover](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1EcdsaRecover.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1FromProjective](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1FromProjective.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1Mul](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1Mul.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1MulGenerator](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1MulGenerator.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1PointX](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1PointX.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1PointY](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1PointY.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1ScalarAdd](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1ScalarAdd.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1ScalarInv](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1ScalarInv.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1ScalarMul](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1ScalarMul.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1ScalarNeg](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1ScalarNeg.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1ScalarSub](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1ScalarSub.md): @midnight-ntwrk/compact-runtime v0.21.0
- [secp256k1ToProjective](https://docs.midnight.network/api-reference/compact-runtime/functions/secp256k1ToProjective.md): @midnight-ntwrk/compact-runtime v0.21.0
- [sha512](https://docs.midnight.network/api-reference/compact-runtime/functions/sha512.md): @midnight-ntwrk/compact-runtime v0.21.0
- [signatureTypesEqual](https://docs.midnight.network/api-reference/compact-runtime/functions/signatureTypesEqual.md): @midnight-ntwrk/compact-runtime v0.21.0
- [signatureVerifyingKey](https://docs.midnight.network/api-reference/compact-runtime/functions/signatureVerifyingKey.md): @midnight-ntwrk/compact-runtime v0.21.0
- [signData](https://docs.midnight.network/api-reference/compact-runtime/functions/signData.md): @midnight-ntwrk/compact-runtime v0.21.0
- [signingKeyFromBip340](https://docs.midnight.network/api-reference/compact-runtime/functions/signingKeyFromBip340.md): @midnight-ntwrk/compact-runtime v0.21.0
- [subField](https://docs.midnight.network/api-reference/compact-runtime/functions/subField.md): @midnight-ntwrk/compact-runtime v0.21.0
- [toBinaryRepr](https://docs.midnight.network/api-reference/compact-runtime/functions/toBinaryRepr.md): @midnight-ntwrk/compact-runtime v0.21.0
- [toHex](https://docs.midnight.network/api-reference/compact-runtime/functions/toHex.md): @midnight-ntwrk/compact-runtime v0.21.0
- [tokenType](https://docs.midnight.network/api-reference/compact-runtime/functions/tokenType.md): @midnight-ntwrk/compact-runtime v0.9.0
- [transientCommit](https://docs.midnight.network/api-reference/compact-runtime/functions/transientCommit.md): @midnight-ntwrk/compact-runtime v0.21.0
- [transientHash](https://docs.midnight.network/api-reference/compact-runtime/functions/transientHash.md): @midnight-ntwrk/compact-runtime v0.21.0
- [type_error](https://docs.midnight.network/api-reference/compact-runtime/functions/type_error.md): @midnight-ntwrk/compact-runtime v0.9.0
- [typeError](https://docs.midnight.network/api-reference/compact-runtime/functions/typeError.md): @midnight-ntwrk/compact-runtime v0.21.0
- [upgradeFromTransient](https://docs.midnight.network/api-reference/compact-runtime/functions/upgradeFromTransient.md): @midnight-ntwrk/compact-runtime v0.21.0
- [valueToBigInt](https://docs.midnight.network/api-reference/compact-runtime/functions/valueToBigInt.md): @midnight-ntwrk/compact-runtime v0.21.0
- [verifierKeyHashOf](https://docs.midnight.network/api-reference/compact-runtime/functions/verifierKeyHashOf.md): @midnight-ntwrk/compact-runtime v0.21.0
- [verifySignature](https://docs.midnight.network/api-reference/compact-runtime/functions/verifySignature.md): @midnight-ntwrk/compact-runtime v0.21.0
- [witnessContext](https://docs.midnight.network/api-reference/compact-runtime/functions/witnessContext.md): @midnight-ntwrk/compact-runtime v0.9.0
- [globals](https://docs.midnight.network/api-reference/compact-runtime/globals.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CallProofData](https://docs.midnight.network/api-reference/compact-runtime/interfaces/CallProofData.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CircuitContext](https://docs.midnight.network/api-reference/compact-runtime/interfaces/CircuitContext.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CircuitResults](https://docs.midnight.network/api-reference/compact-runtime/interfaces/CircuitResults.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CommunicationCommitmentData](https://docs.midnight.network/api-reference/compact-runtime/interfaces/CommunicationCommitmentData.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactType](https://docs.midnight.network/api-reference/compact-runtime/interfaces/CompactType.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ConstructorContext](https://docs.midnight.network/api-reference/compact-runtime/interfaces/ConstructorContext.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ConstructorResult](https://docs.midnight.network/api-reference/compact-runtime/interfaces/ConstructorResult.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ContractStateProvider](https://docs.midnight.network/api-reference/compact-runtime/interfaces/ContractStateProvider.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CurvePoint](https://docs.midnight.network/api-reference/compact-runtime/interfaces/CurvePoint.md): @midnight-ntwrk/compact-runtime v0.9.0
- [EncodedCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/interfaces/EncodedCoinInfo.md): @midnight-ntwrk/compact-runtime v0.9.0
- [EncodedCoinPublicKey](https://docs.midnight.network/api-reference/compact-runtime/interfaces/EncodedCoinPublicKey.md): @midnight-ntwrk/compact-runtime v0.21.0
- [EncodedContractAddress](https://docs.midnight.network/api-reference/compact-runtime/interfaces/EncodedContractAddress.md): @midnight-ntwrk/compact-runtime v0.21.0
- [EncodedQualifiedCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/interfaces/EncodedQualifiedCoinInfo.md): @midnight-ntwrk/compact-runtime v0.9.0
- [EncodedQualifiedShieldedCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/interfaces/EncodedQualifiedShieldedCoinInfo.md): @midnight-ntwrk/compact-runtime v0.21.0
- [EncodedRecipient](https://docs.midnight.network/api-reference/compact-runtime/interfaces/EncodedRecipient.md): @midnight-ntwrk/compact-runtime v0.21.0
- [EncodedShieldedCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/interfaces/EncodedShieldedCoinInfo.md): @midnight-ntwrk/compact-runtime v0.21.0
- [EncodedZswapLocalState](https://docs.midnight.network/api-reference/compact-runtime/interfaces/EncodedZswapLocalState.md): @midnight-ntwrk/compact-runtime v0.21.0
- [JubjubPoint](https://docs.midnight.network/api-reference/compact-runtime/interfaces/JubjubPoint.md): @midnight-ntwrk/compact-runtime v0.21.0
- [JubjubSchnorrSignature](https://docs.midnight.network/api-reference/compact-runtime/interfaces/JubjubSchnorrSignature.md): @midnight-ntwrk/compact-runtime v0.21.0
- [MerkleTreeDigest](https://docs.midnight.network/api-reference/compact-runtime/interfaces/MerkleTreeDigest.md): @midnight-ntwrk/compact-runtime v0.21.0
- [MerkleTreePath](https://docs.midnight.network/api-reference/compact-runtime/interfaces/MerkleTreePath.md): @midnight-ntwrk/compact-runtime v0.21.0
- [MerkleTreePathEntry](https://docs.midnight.network/api-reference/compact-runtime/interfaces/MerkleTreePathEntry.md): @midnight-ntwrk/compact-runtime v0.21.0
- [NativePoint](https://docs.midnight.network/api-reference/compact-runtime/interfaces/NativePoint.md): @midnight-ntwrk/compact-runtime v0.14.0
- [PartialProofData](https://docs.midnight.network/api-reference/compact-runtime/interfaces/PartialProofData.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ProofData](https://docs.midnight.network/api-reference/compact-runtime/interfaces/ProofData.md): @midnight-ntwrk/compact-runtime v0.21.0
- [Recipient](https://docs.midnight.network/api-reference/compact-runtime/interfaces/Recipient.md): @midnight-ntwrk/compact-runtime v0.21.0
- [Secp256k1Point](https://docs.midnight.network/api-reference/compact-runtime/interfaces/Secp256k1Point.md): @midnight-ntwrk/compact-runtime v0.21.0
- [WitnessContext](https://docs.midnight.network/api-reference/compact-runtime/interfaces/WitnessContext.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ZswapLocalState](https://docs.midnight.network/api-reference/compact-runtime/interfaces/ZswapLocalState.md): @midnight-ntwrk/compact-runtime v0.21.0
- [AlignedValue](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/AlignedValue.md): @midnight-ntwrk/compact-runtime v0.21.0
- [Alignment](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/Alignment.md): @midnight-ntwrk/compact-runtime v0.21.0
- [AlignmentAtom](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/AlignmentAtom.md): @midnight-ntwrk/compact-runtime v0.21.0
- [AlignmentSegment](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/AlignmentSegment.md): @midnight-ntwrk/compact-runtime v0.21.0
- [BlockContext](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/BlockContext.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CallContext](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/CallContext.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CallProofDataTrace](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/CallProofDataTrace.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CircuitContextOptions](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/CircuitContextOptions.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CircuitId](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/CircuitId.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CircuitSignature](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/CircuitSignature.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CircuitSignatures](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/CircuitSignatures.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CoinCommitment](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/CoinCommitment.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CoinInfo](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/CoinInfo.md): @midnight-ntwrk/compact-runtime v0.9.0
- [CoinPublicKey](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/CoinPublicKey.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CommunicationCommitment](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/CommunicationCommitment.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CommunicationCommitmentRand](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/CommunicationCommitmentRand.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ContractAddress](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/ContractAddress.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ContractCtor](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/ContractCtor.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ContractInstance](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/ContractInstance.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ContractReferenceLocations](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/ContractReferenceLocations.md): @midnight-ntwrk/compact-runtime v0.19.0
- [CrossContractCallOptions](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/CrossContractCallOptions.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CrossContractInputs](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/CrossContractInputs.md): @midnight-ntwrk/compact-runtime v0.21.0
- [DomainSeparator](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/DomainSeparator.md): @midnight-ntwrk/compact-runtime v0.21.0
- [DomainSeperator](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/DomainSeperator.md): @midnight-ntwrk/compact-runtime v0.9.0
- [DustTokenType](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/DustTokenType.md): @midnight-ntwrk/compact-runtime v0.21.0
- [Effects](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/Effects.md): @midnight-ntwrk/compact-runtime v0.21.0
- [EmptyPublicLedger](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/EmptyPublicLedger.md): @midnight-ntwrk/compact-runtime v0.19.0
- [EncodedStateValue](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/EncodedStateValue.md): @midnight-ntwrk/compact-runtime v0.21.0
- [Fr](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/Fr.md): @midnight-ntwrk/compact-runtime v0.21.0
- [GatherResult](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/GatherResult.md): @midnight-ntwrk/compact-runtime v0.21.0
- [InterfaceCircuitDeclaration](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/InterfaceCircuitDeclaration.md): @midnight-ntwrk/compact-runtime v0.21.0
- [InterfaceDescriptor](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/InterfaceDescriptor.md): @midnight-ntwrk/compact-runtime v0.21.0
- [Key](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/Key.md): @midnight-ntwrk/compact-runtime v0.21.0
- [LogEvent](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/LogEvent.md): @midnight-ntwrk/compact-runtime v0.21.0
- [NamedSignatureType](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/NamedSignatureType.md): @midnight-ntwrk/compact-runtime v0.21.0
- [Nonce](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/Nonce.md): @midnight-ntwrk/compact-runtime v0.21.0
- [Nullifier](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/Nullifier.md): @midnight-ntwrk/compact-runtime v0.21.0
- [Op](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/Op.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ProvableCircuit](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/ProvableCircuit.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ProvableCircuits](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/ProvableCircuits.md): @midnight-ntwrk/compact-runtime v0.21.0
- [PublicAddress](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/PublicAddress.md): @midnight-ntwrk/compact-runtime v0.21.0
- [PublicLedgerSegments](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/PublicLedgerSegments.md): @midnight-ntwrk/compact-runtime v0.19.0
- [PureCircuit](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/PureCircuit.md): @midnight-ntwrk/compact-runtime v0.20.0
- [PureCircuits](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/PureCircuits.md): @midnight-ntwrk/compact-runtime v0.20.0
- [QualifiedCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/QualifiedCoinInfo.md): @midnight-ntwrk/compact-runtime v0.9.0
- [QualifiedShieldedCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/QualifiedShieldedCoinInfo.md): @midnight-ntwrk/compact-runtime v0.21.0
- [RawTokenType](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/RawTokenType.md): @midnight-ntwrk/compact-runtime v0.21.0
- [RunningCost](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/RunningCost.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ShieldedCoinInfo](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/ShieldedCoinInfo.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ShieldedTokenType](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/ShieldedTokenType.md): @midnight-ntwrk/compact-runtime v0.21.0
- [Signature](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/Signature.md): @midnight-ntwrk/compact-runtime v0.21.0
- [SignatureType](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/SignatureType.md): @midnight-ntwrk/compact-runtime v0.21.0
- [SignatureVerifyingKey](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/SignatureVerifyingKey.md): @midnight-ntwrk/compact-runtime v0.21.0
- [SigningKey](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/SigningKey.md): @midnight-ntwrk/compact-runtime v0.21.0
- [SparseCompactADT](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/SparseCompactADT.md): @midnight-ntwrk/compact-runtime v0.19.0
- [SparseCompactArrayLikeADT](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/SparseCompactArrayLikeADT.md): @midnight-ntwrk/compact-runtime v0.19.0
- [SparseCompactCellADT](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/SparseCompactCellADT.md): @midnight-ntwrk/compact-runtime v0.19.0
- [SparseCompactContractAddress](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/SparseCompactContractAddress.md): @midnight-ntwrk/compact-runtime v0.19.0
- [SparseCompactListADT](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/SparseCompactListADT.md): @midnight-ntwrk/compact-runtime v0.19.0
- [SparseCompactMapADT](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/SparseCompactMapADT.md): @midnight-ntwrk/compact-runtime v0.19.0
- [SparseCompactSetADT](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/SparseCompactSetADT.md): @midnight-ntwrk/compact-runtime v0.19.0
- [SparseCompactStruct](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/SparseCompactStruct.md): @midnight-ntwrk/compact-runtime v0.19.0
- [SparseCompactType](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/SparseCompactType.md): @midnight-ntwrk/compact-runtime v0.19.0
- [SparseCompactValue](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/SparseCompactValue.md): @midnight-ntwrk/compact-runtime v0.19.0
- [SparseCompactVector](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/SparseCompactVector.md): @midnight-ntwrk/compact-runtime v0.19.0
- [TokenType](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/TokenType.md): @midnight-ntwrk/compact-runtime v0.21.0
- [Transcript](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/Transcript.md): @midnight-ntwrk/compact-runtime v0.21.0
- [UnreadableSignature](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/UnreadableSignature.md): @midnight-ntwrk/compact-runtime v0.21.0
- [UnshieldedTokenType](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/UnshieldedTokenType.md): @midnight-ntwrk/compact-runtime v0.21.0
- [UserAddress](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/UserAddress.md): @midnight-ntwrk/compact-runtime v0.21.0
- [Value](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/Value.md): @midnight-ntwrk/compact-runtime v0.21.0
- [VerifierKeyHash](https://docs.midnight.network/api-reference/compact-runtime/type-aliases/VerifierKeyHash.md): @midnight-ntwrk/compact-runtime v0.21.0
- [BooleanDescriptor](https://docs.midnight.network/api-reference/compact-runtime/variables/BooleanDescriptor.md): @midnight-ntwrk/compact-runtime v0.9.0
- [Bytes32Descriptor](https://docs.midnight.network/api-reference/compact-runtime/variables/Bytes32Descriptor.md): @midnight-ntwrk/compact-runtime v0.16.0
- [CoinInfoDescriptor](https://docs.midnight.network/api-reference/compact-runtime/variables/CoinInfoDescriptor.md): @midnight-ntwrk/compact-runtime v0.9.0
- [CoinRecipientDescriptor](https://docs.midnight.network/api-reference/compact-runtime/variables/CoinRecipientDescriptor.md): @midnight-ntwrk/compact-runtime v0.9.0
- [CompactTypeBoolean](https://docs.midnight.network/api-reference/compact-runtime/variables/CompactTypeBoolean.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeField](https://docs.midnight.network/api-reference/compact-runtime/variables/CompactTypeField.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeJubjubPoint](https://docs.midnight.network/api-reference/compact-runtime/variables/CompactTypeJubjubPoint.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeJubjubScalar](https://docs.midnight.network/api-reference/compact-runtime/variables/CompactTypeJubjubScalar.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeMerkleTreeDigest](https://docs.midnight.network/api-reference/compact-runtime/variables/CompactTypeMerkleTreeDigest.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeMerkleTreePathEntry](https://docs.midnight.network/api-reference/compact-runtime/variables/CompactTypeMerkleTreePathEntry.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeNativePoint](https://docs.midnight.network/api-reference/compact-runtime/variables/CompactTypeNativePoint.md): @midnight-ntwrk/compact-runtime v0.14.0
- [CompactTypeOpaqueString](https://docs.midnight.network/api-reference/compact-runtime/variables/CompactTypeOpaqueString.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeOpaqueUint8Array](https://docs.midnight.network/api-reference/compact-runtime/variables/CompactTypeOpaqueUint8Array.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeSecp256k1Base](https://docs.midnight.network/api-reference/compact-runtime/variables/CompactTypeSecp256k1Base.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeSecp256k1Point](https://docs.midnight.network/api-reference/compact-runtime/variables/CompactTypeSecp256k1Point.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CompactTypeSecp256k1Scalar](https://docs.midnight.network/api-reference/compact-runtime/variables/CompactTypeSecp256k1Scalar.md): @midnight-ntwrk/compact-runtime v0.21.0
- [CONTRACT_ADDRESS_BYTE_LENGTH](https://docs.midnight.network/api-reference/compact-runtime/variables/CONTRACT_ADDRESS_BYTE_LENGTH.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ContractAddressDescriptor](https://docs.midnight.network/api-reference/compact-runtime/variables/ContractAddressDescriptor.md): @midnight-ntwrk/compact-runtime v0.16.0
- [DUMMY_ADDRESS](https://docs.midnight.network/api-reference/compact-runtime/variables/DUMMY_ADDRESS.md): @midnight-ntwrk/compact-runtime v0.21.0
- [FIELD_MODULUS](https://docs.midnight.network/api-reference/compact-runtime/variables/FIELD_MODULUS.md): @midnight-ntwrk/compact-runtime v0.21.0
- [HEX_REGEX_NO_PREFIX](https://docs.midnight.network/api-reference/compact-runtime/variables/HEX_REGEX_NO_PREFIX.md): @midnight-ntwrk/compact-runtime v0.21.0
- [JUBJUB_SCALAR_MODULUS](https://docs.midnight.network/api-reference/compact-runtime/variables/JUBJUB_SCALAR_MODULUS.md): @midnight-ntwrk/compact-runtime v0.21.0
- [MAX_FIELD](https://docs.midnight.network/api-reference/compact-runtime/variables/MAX_FIELD.md): @midnight-ntwrk/compact-runtime v0.21.0
- [MAX_JUBJUB_SCALAR](https://docs.midnight.network/api-reference/compact-runtime/variables/MAX_JUBJUB_SCALAR.md): @midnight-ntwrk/compact-runtime v0.21.0
- [MAX_SECP256K1_BASE](https://docs.midnight.network/api-reference/compact-runtime/variables/MAX_SECP256K1_BASE.md): @midnight-ntwrk/compact-runtime v0.21.0
- [MAX_SECP256K1_SCALAR](https://docs.midnight.network/api-reference/compact-runtime/variables/MAX_SECP256K1_SCALAR.md): @midnight-ntwrk/compact-runtime v0.21.0
- [MaxUint8Descriptor](https://docs.midnight.network/api-reference/compact-runtime/variables/MaxUint8Descriptor.md): @midnight-ntwrk/compact-runtime v0.16.0
- [sampleJubjubSchnorrSk](https://docs.midnight.network/api-reference/compact-runtime/variables/sampleJubjubSchnorrSk.md): @midnight-ntwrk/compact-runtime v0.21.0
- [SECP256K1_BASE_MODULUS](https://docs.midnight.network/api-reference/compact-runtime/variables/SECP256K1_BASE_MODULUS.md): @midnight-ntwrk/compact-runtime v0.21.0
- [SECP256K1_SCALAR_MODULUS](https://docs.midnight.network/api-reference/compact-runtime/variables/SECP256K1_SCALAR_MODULUS.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ShieldedCoinInfoDescriptor](https://docs.midnight.network/api-reference/compact-runtime/variables/ShieldedCoinInfoDescriptor.md): @midnight-ntwrk/compact-runtime v0.16.0
- [ShieldedCoinRecipientDescriptor](https://docs.midnight.network/api-reference/compact-runtime/variables/ShieldedCoinRecipientDescriptor.md): @midnight-ntwrk/compact-runtime v0.16.0
- [versionString](https://docs.midnight.network/api-reference/compact-runtime/variables/versionString.md): @midnight-ntwrk/compact-runtime v0.21.0
- [ZswapCoinPublicKeyDescriptor](https://docs.midnight.network/api-reference/compact-runtime/variables/ZswapCoinPublicKeyDescriptor.md): @midnight-ntwrk/compact-runtime v0.16.0

### dapp-connector

@midnight-ntwrk/dapp-connector-api v4.0.1

- [Midnight DApp connector API](https://docs.midnight.network/api-reference/dapp-connector.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [globals](https://docs.midnight.network/api-reference/dapp-connector/globals.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [APIError](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/APIError.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [Configuration](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/Configuration.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [ConnectedAPI](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/ConnectedAPI.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [ConnectionStatus](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/ConnectionStatus.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [DesiredInput](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/DesiredInput.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [DesiredOutput](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/DesiredOutput.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [ErrorCode](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/ErrorCode.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [ExecutionStatus](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/ExecutionStatus.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [HintUsage](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/HintUsage.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [HistoryEntry](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/HistoryEntry.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [InitialAPI](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/InitialAPI.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [KeyMaterialProvider](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/KeyMaterialProvider.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [ProvingProvider](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/ProvingProvider.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [Signature](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/Signature.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [SignDataOptions](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/SignDataOptions.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [TokenType](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/TokenType.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [TxStatus](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/TxStatus.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [WalletConnectedAPI](https://docs.midnight.network/api-reference/dapp-connector/type-aliases/WalletConnectedAPI.md): @midnight-ntwrk/dapp-connector-api v4.0.1
- [ErrorCodes](https://docs.midnight.network/api-reference/dapp-connector/variables/ErrorCodes.md): @midnight-ntwrk/dapp-connector-api v4.0.1

### error-reference

- [DApp Connector API errors](https://docs.midnight.network/api-reference/error-reference/dapp-connector-errors.md): Error codes returned by the Midnight DApp Connector API when a DApp interacts with a wallet.
- [Midnight indexer error codes](https://docs.midnight.network/api-reference/error-reference/indexer-errors.md): Error codes and messages returned by the Midnight Indexer API and infrastructure services.
- [Ledger error reference](https://docs.midnight.network/api-reference/error-reference/ledger-errors.md): Rust-level error types from the Midnight ledger crate covering transaction validation, execution, and state management failures.
- [Proof server errors](https://docs.midnight.network/api-reference/error-reference/proof-server-errors.md): Error codes returned by the Midnight proof server during zero-knowledge proof generation.

### ledger

@midnight/ledger v8.1.3

- [Ledger API](https://docs.midnight.network/api-reference/ledger.md): @midnight/ledger v8.1.3
- [Binding](https://docs.midnight.network/api-reference/ledger/classes/Binding.md): @midnight/ledger v8.1.3
- [ChargedState](https://docs.midnight.network/api-reference/ledger/classes/ChargedState.md): @midnight/ledger v8.1.3
- [ClaimRewardsTransaction](https://docs.midnight.network/api-reference/ledger/classes/ClaimRewardsTransaction.md): @midnight/ledger v8.1.3
- [CoinSecretKey](https://docs.midnight.network/api-reference/ledger/classes/CoinSecretKey.md): @midnight/ledger v8.1.3
- [ContractCall](https://docs.midnight.network/api-reference/ledger/classes/ContractCall.md): @midnight/ledger v8.1.3
- [ContractCallPrototype](https://docs.midnight.network/api-reference/ledger/classes/ContractCallPrototype.md): @midnight/ledger v8.1.3
- [ContractDeploy](https://docs.midnight.network/api-reference/ledger/classes/ContractDeploy.md): @midnight/ledger v8.1.3
- [ContractMaintenanceAuthority](https://docs.midnight.network/api-reference/ledger/classes/ContractMaintenanceAuthority.md): @midnight/ledger v8.1.3
- [ContractOperation](https://docs.midnight.network/api-reference/ledger/classes/ContractOperation.md): @midnight/ledger v8.1.3
- [ContractOperationVersion](https://docs.midnight.network/api-reference/ledger/classes/ContractOperationVersion.md): @midnight/ledger v8.1.3
- [ContractOperationVersionedVerifierKey](https://docs.midnight.network/api-reference/ledger/classes/ContractOperationVersionedVerifierKey.md): @midnight/ledger v8.1.3
- [ContractState](https://docs.midnight.network/api-reference/ledger/classes/ContractState.md): @midnight/ledger v8.1.3
- [CostModel](https://docs.midnight.network/api-reference/ledger/classes/CostModel.md): @midnight/ledger v8.1.3
- [DustActions](https://docs.midnight.network/api-reference/ledger/classes/DustActions.md): @midnight/ledger v8.1.3
- [DustGenerationState](https://docs.midnight.network/api-reference/ledger/classes/DustGenerationState.md): @midnight/ledger v8.1.3
- [DustLocalState](https://docs.midnight.network/api-reference/ledger/classes/DustLocalState.md): @midnight/ledger v8.1.3
- [DustLocalStateWithChanges](https://docs.midnight.network/api-reference/ledger/classes/DustLocalStateWithChanges.md): @midnight/ledger v8.1.3
- [DustParameters](https://docs.midnight.network/api-reference/ledger/classes/DustParameters.md): @midnight/ledger v8.1.3
- [DustRegistration](https://docs.midnight.network/api-reference/ledger/classes/DustRegistration.md): @midnight/ledger v8.1.3
- [DustSecretKey](https://docs.midnight.network/api-reference/ledger/classes/DustSecretKey.md): @midnight/ledger v8.1.3
- [DustSpend](https://docs.midnight.network/api-reference/ledger/classes/DustSpend.md): @midnight/ledger v8.1.3
- [DustState](https://docs.midnight.network/api-reference/ledger/classes/DustState.md): @midnight/ledger v8.1.3
- [DustStateChanges](https://docs.midnight.network/api-reference/ledger/classes/DustStateChanges.md): @midnight/ledger v8.1.3
- [DustStateMerkleTreeCollapsedUpdate](https://docs.midnight.network/api-reference/ledger/classes/DustStateMerkleTreeCollapsedUpdate.md): @midnight/ledger v8.1.3
- [DustUtxoState](https://docs.midnight.network/api-reference/ledger/classes/DustUtxoState.md): @midnight/ledger v8.1.3
- [EncryptionSecretKey](https://docs.midnight.network/api-reference/ledger/classes/EncryptionSecretKey.md): @midnight/ledger v8.1.3
- [Event](https://docs.midnight.network/api-reference/ledger/classes/Event.md): @midnight/ledger v8.1.3
- [Intent](https://docs.midnight.network/api-reference/ledger/classes/Intent.md): @midnight/ledger v8.1.3
- [LedgerParameters](https://docs.midnight.network/api-reference/ledger/classes/LedgerParameters.md): @midnight/ledger v8.1.3
- [LedgerState](https://docs.midnight.network/api-reference/ledger/classes/LedgerState.md): @midnight/ledger v8.1.3
- [MaintenanceUpdate](https://docs.midnight.network/api-reference/ledger/classes/MaintenanceUpdate.md): @midnight/ledger v8.1.3
- [MerkleTreeCollapsedUpdate](https://docs.midnight.network/api-reference/ledger/classes/MerkleTreeCollapsedUpdate.md): @midnight/ledger v8.1.3
- [NoBinding](https://docs.midnight.network/api-reference/ledger/classes/NoBinding.md): @midnight/ledger v8.1.3
- [NoProof](https://docs.midnight.network/api-reference/ledger/classes/NoProof.md): @midnight/ledger v8.1.3
- [PreBinding](https://docs.midnight.network/api-reference/ledger/classes/PreBinding.md): @midnight/ledger v8.1.3
- [PrePartitionContractCall](https://docs.midnight.network/api-reference/ledger/classes/PrePartitionContractCall.md): @midnight/ledger v8.1.3
- [PreProof](https://docs.midnight.network/api-reference/ledger/classes/PreProof.md): @midnight/ledger v8.1.3
- [PreTranscript](https://docs.midnight.network/api-reference/ledger/classes/PreTranscript.md): @midnight/ledger v8.1.3
- [Proof](https://docs.midnight.network/api-reference/ledger/classes/Proof.md): @midnight/ledger v8.1.3
- [QueryContext](https://docs.midnight.network/api-reference/ledger/classes/QueryContext.md): @midnight/ledger v8.1.3
- [QueryResults](https://docs.midnight.network/api-reference/ledger/classes/QueryResults.md): @midnight/ledger v8.1.3
- [ReplaceAuthority](https://docs.midnight.network/api-reference/ledger/classes/ReplaceAuthority.md): @midnight/ledger v8.1.3
- [SignatureEnabled](https://docs.midnight.network/api-reference/ledger/classes/SignatureEnabled.md): @midnight/ledger v8.1.3
- [SignatureErased](https://docs.midnight.network/api-reference/ledger/classes/SignatureErased.md): @midnight/ledger v8.1.3
- [StateBoundedMerkleTree](https://docs.midnight.network/api-reference/ledger/classes/StateBoundedMerkleTree.md): @midnight/ledger v8.1.3
- [StateMap](https://docs.midnight.network/api-reference/ledger/classes/StateMap.md): @midnight/ledger v8.1.3
- [StateValue](https://docs.midnight.network/api-reference/ledger/classes/StateValue.md): @midnight/ledger v8.1.3
- [SystemTransaction](https://docs.midnight.network/api-reference/ledger/classes/SystemTransaction.md): @midnight/ledger v8.1.3
- [Transaction](https://docs.midnight.network/api-reference/ledger/classes/Transaction.md): @midnight/ledger v8.1.3
- [TransactionContext](https://docs.midnight.network/api-reference/ledger/classes/TransactionContext.md): @midnight/ledger v8.1.3
- [TransactionCostModel](https://docs.midnight.network/api-reference/ledger/classes/TransactionCostModel.md): @midnight/ledger v8.1.3
- [TransactionResult](https://docs.midnight.network/api-reference/ledger/classes/TransactionResult.md): @midnight/ledger v8.1.3
- [UnshieldedOffer](https://docs.midnight.network/api-reference/ledger/classes/UnshieldedOffer.md): @midnight/ledger v8.1.3
- [UtxoMeta](https://docs.midnight.network/api-reference/ledger/classes/UtxoMeta.md): @midnight/ledger v8.1.3
- [UtxoState](https://docs.midnight.network/api-reference/ledger/classes/UtxoState.md): @midnight/ledger v8.1.3
- [VerifiedTransaction](https://docs.midnight.network/api-reference/ledger/classes/VerifiedTransaction.md): @midnight/ledger v8.1.3
- [VerifierKeyInsert](https://docs.midnight.network/api-reference/ledger/classes/VerifierKeyInsert.md): @midnight/ledger v8.1.3
- [VerifierKeyRemove](https://docs.midnight.network/api-reference/ledger/classes/VerifierKeyRemove.md): @midnight/ledger v8.1.3
- [VmResults](https://docs.midnight.network/api-reference/ledger/classes/VmResults.md): @midnight/ledger v8.1.3
- [VmStack](https://docs.midnight.network/api-reference/ledger/classes/VmStack.md): @midnight/ledger v8.1.3
- [WellFormedStrictness](https://docs.midnight.network/api-reference/ledger/classes/WellFormedStrictness.md): @midnight/ledger v8.1.3
- [ZswapChainState](https://docs.midnight.network/api-reference/ledger/classes/ZswapChainState.md): @midnight/ledger v8.1.3
- [ZswapInput](https://docs.midnight.network/api-reference/ledger/classes/ZswapInput.md): @midnight/ledger v8.1.3
- [ZswapLocalState](https://docs.midnight.network/api-reference/ledger/classes/ZswapLocalState.md): @midnight/ledger v8.1.3
- [ZswapLocalStateWithChanges](https://docs.midnight.network/api-reference/ledger/classes/ZswapLocalStateWithChanges.md): @midnight/ledger v8.1.3
- [ZswapOffer](https://docs.midnight.network/api-reference/ledger/classes/ZswapOffer.md): @midnight/ledger v8.1.3
- [ZswapOutput](https://docs.midnight.network/api-reference/ledger/classes/ZswapOutput.md): @midnight/ledger v8.1.3
- [ZswapSecretKeys](https://docs.midnight.network/api-reference/ledger/classes/ZswapSecretKeys.md): @midnight/ledger v8.1.3
- [ZswapStateChanges](https://docs.midnight.network/api-reference/ledger/classes/ZswapStateChanges.md): @midnight/ledger v8.1.3
- [ZswapTransient](https://docs.midnight.network/api-reference/ledger/classes/ZswapTransient.md): @midnight/ledger v8.1.3
- [addressFromKey](https://docs.midnight.network/api-reference/ledger/functions/addressFromKey.md): @midnight/ledger v8.1.3
- [bigIntModFr](https://docs.midnight.network/api-reference/ledger/functions/bigIntModFr.md): @midnight/ledger v8.1.3
- [bigIntToValue](https://docs.midnight.network/api-reference/ledger/functions/bigIntToValue.md): @midnight/ledger v8.1.3
- [coinCommitment](https://docs.midnight.network/api-reference/ledger/functions/coinCommitment.md): @midnight/ledger v8.1.3
- [coinNullifier](https://docs.midnight.network/api-reference/ledger/functions/coinNullifier.md): @midnight/ledger v8.1.3
- [communicationCommitment](https://docs.midnight.network/api-reference/ledger/functions/communicationCommitment.md): @midnight/ledger v8.1.3
- [communicationCommitmentRandomness](https://docs.midnight.network/api-reference/ledger/functions/communicationCommitmentRandomness.md): @midnight/ledger v8.1.3
- [createCheckPayload](https://docs.midnight.network/api-reference/ledger/functions/createCheckPayload.md): @midnight/ledger v8.1.3
- [createProvingPayload](https://docs.midnight.network/api-reference/ledger/functions/createProvingPayload.md): @midnight/ledger v8.1.3
- [createProvingTransactionPayload](https://docs.midnight.network/api-reference/ledger/functions/createProvingTransactionPayload.md): @midnight/ledger v8.1.3
- [createShieldedCoinInfo](https://docs.midnight.network/api-reference/ledger/functions/createShieldedCoinInfo.md): @midnight/ledger v8.1.3
- [decodeCoinPublicKey](https://docs.midnight.network/api-reference/ledger/functions/decodeCoinPublicKey.md): @midnight/ledger v8.1.3
- [decodeContractAddress](https://docs.midnight.network/api-reference/ledger/functions/decodeContractAddress.md): @midnight/ledger v8.1.3
- [decodeQualifiedShieldedCoinInfo](https://docs.midnight.network/api-reference/ledger/functions/decodeQualifiedShieldedCoinInfo.md): @midnight/ledger v8.1.3
- [decodeRawTokenType](https://docs.midnight.network/api-reference/ledger/functions/decodeRawTokenType.md): @midnight/ledger v8.1.3
- [decodeShieldedCoinInfo](https://docs.midnight.network/api-reference/ledger/functions/decodeShieldedCoinInfo.md): @midnight/ledger v8.1.3
- [decodeUserAddress](https://docs.midnight.network/api-reference/ledger/functions/decodeUserAddress.md): @midnight/ledger v8.1.3
- [degradeToTransient](https://docs.midnight.network/api-reference/ledger/functions/degradeToTransient.md): @midnight/ledger v8.1.3
- [dummyContractAddress](https://docs.midnight.network/api-reference/ledger/functions/dummyContractAddress.md): @midnight/ledger v8.1.3
- [dummyUserAddress](https://docs.midnight.network/api-reference/ledger/functions/dummyUserAddress.md): @midnight/ledger v8.1.3
- [dustCommitment](https://docs.midnight.network/api-reference/ledger/functions/dustCommitment.md): @midnight/ledger v8.1.3
- [dustInitialNonce](https://docs.midnight.network/api-reference/ledger/functions/dustInitialNonce.md): @midnight/ledger v8.1.3
- [dustNonce](https://docs.midnight.network/api-reference/ledger/functions/dustNonce.md): @midnight/ledger v8.1.3
- [dustNullifier](https://docs.midnight.network/api-reference/ledger/functions/dustNullifier.md): @midnight/ledger v8.1.3
- [ecAdd](https://docs.midnight.network/api-reference/ledger/functions/ecAdd.md): @midnight/ledger v8.1.3
- [ecMul](https://docs.midnight.network/api-reference/ledger/functions/ecMul.md): @midnight/ledger v8.1.3
- [ecMulGenerator](https://docs.midnight.network/api-reference/ledger/functions/ecMulGenerator.md): @midnight/ledger v8.1.3
- [encodeCoinPublicKey](https://docs.midnight.network/api-reference/ledger/functions/encodeCoinPublicKey.md): @midnight/ledger v8.1.3
- [encodeContractAddress](https://docs.midnight.network/api-reference/ledger/functions/encodeContractAddress.md): @midnight/ledger v8.1.3
- [encodeQualifiedShieldedCoinInfo](https://docs.midnight.network/api-reference/ledger/functions/encodeQualifiedShieldedCoinInfo.md): @midnight/ledger v8.1.3
- [encodeRawTokenType](https://docs.midnight.network/api-reference/ledger/functions/encodeRawTokenType.md): @midnight/ledger v8.1.3
- [encodeShieldedCoinInfo](https://docs.midnight.network/api-reference/ledger/functions/encodeShieldedCoinInfo.md): @midnight/ledger v8.1.3
- [encodeUserAddress](https://docs.midnight.network/api-reference/ledger/functions/encodeUserAddress.md): @midnight/ledger v8.1.3
- [entryPointHash](https://docs.midnight.network/api-reference/ledger/functions/entryPointHash.md): @midnight/ledger v8.1.3
- [feeToken](https://docs.midnight.network/api-reference/ledger/functions/feeToken.md): @midnight/ledger v8.1.3
- [hashToCurve](https://docs.midnight.network/api-reference/ledger/functions/hashToCurve.md): @midnight/ledger v8.1.3
- [leafHash](https://docs.midnight.network/api-reference/ledger/functions/leafHash.md): @midnight/ledger v8.1.3
- [maxAlignedSize](https://docs.midnight.network/api-reference/ledger/functions/maxAlignedSize.md): @midnight/ledger v8.1.3
- [maxField](https://docs.midnight.network/api-reference/ledger/functions/maxField.md): @midnight/ledger v8.1.3
- [nativeToken](https://docs.midnight.network/api-reference/ledger/functions/nativeToken.md): @midnight/ledger v8.1.3
- [parseCheckResult](https://docs.midnight.network/api-reference/ledger/functions/parseCheckResult.md): @midnight/ledger v8.1.3
- [partitionTranscripts](https://docs.midnight.network/api-reference/ledger/functions/partitionTranscripts.md): @midnight/ledger v8.1.3
- [persistentCommit](https://docs.midnight.network/api-reference/ledger/functions/persistentCommit.md): @midnight/ledger v8.1.3
- [persistentHash](https://docs.midnight.network/api-reference/ledger/functions/persistentHash.md): @midnight/ledger v8.1.3
- [proofDataIntoSerializedPreimage](https://docs.midnight.network/api-reference/ledger/functions/proofDataIntoSerializedPreimage.md): @midnight/ledger v8.1.3
- [rawTokenType](https://docs.midnight.network/api-reference/ledger/functions/rawTokenType.md): @midnight/ledger v8.1.3
- [runProgram](https://docs.midnight.network/api-reference/ledger/functions/runProgram.md): @midnight/ledger v8.1.3
- [runtimeCoinCommitment](https://docs.midnight.network/api-reference/ledger/functions/runtimeCoinCommitment.md): @midnight/ledger v8.1.3
- [runtimeCoinNullifier](https://docs.midnight.network/api-reference/ledger/functions/runtimeCoinNullifier.md): @midnight/ledger v8.1.3
- [sampleCoinPublicKey](https://docs.midnight.network/api-reference/ledger/functions/sampleCoinPublicKey.md): @midnight/ledger v8.1.3
- [sampleContractAddress](https://docs.midnight.network/api-reference/ledger/functions/sampleContractAddress.md): @midnight/ledger v8.1.3
- [sampleDustSecretKey](https://docs.midnight.network/api-reference/ledger/functions/sampleDustSecretKey.md): @midnight/ledger v8.1.3
- [sampleEncryptionPublicKey](https://docs.midnight.network/api-reference/ledger/functions/sampleEncryptionPublicKey.md): @midnight/ledger v8.1.3
- [sampleIntentHash](https://docs.midnight.network/api-reference/ledger/functions/sampleIntentHash.md): @midnight/ledger v8.1.3
- [sampleRawTokenType](https://docs.midnight.network/api-reference/ledger/functions/sampleRawTokenType.md): @midnight/ledger v8.1.3
- [sampleSigningKey](https://docs.midnight.network/api-reference/ledger/functions/sampleSigningKey.md): @midnight/ledger v8.1.3
- [sampleUserAddress](https://docs.midnight.network/api-reference/ledger/functions/sampleUserAddress.md): @midnight/ledger v8.1.3
- [shieldedToken](https://docs.midnight.network/api-reference/ledger/functions/shieldedToken.md): @midnight/ledger v8.1.3
- [signatureVerifyingKey](https://docs.midnight.network/api-reference/ledger/functions/signatureVerifyingKey.md): @midnight/ledger v8.1.3
- [signData](https://docs.midnight.network/api-reference/ledger/functions/signData.md): @midnight/ledger v8.1.3
- [signingKeyFromBip340](https://docs.midnight.network/api-reference/ledger/functions/signingKeyFromBip340.md): @midnight/ledger v8.1.3
- [transientCommit](https://docs.midnight.network/api-reference/ledger/functions/transientCommit.md): @midnight/ledger v8.1.3
- [transientHash](https://docs.midnight.network/api-reference/ledger/functions/transientHash.md): @midnight/ledger v8.1.3
- [unshieldedToken](https://docs.midnight.network/api-reference/ledger/functions/unshieldedToken.md): @midnight/ledger v8.1.3
- [updatedValue](https://docs.midnight.network/api-reference/ledger/functions/updatedValue.md): @midnight/ledger v8.1.3
- [upgradeFromTransient](https://docs.midnight.network/api-reference/ledger/functions/upgradeFromTransient.md): @midnight/ledger v8.1.3
- [valueToBigInt](https://docs.midnight.network/api-reference/ledger/functions/valueToBigInt.md): @midnight/ledger v8.1.3
- [verifySignature](https://docs.midnight.network/api-reference/ledger/functions/verifySignature.md): @midnight/ledger v8.1.3
- [globals](https://docs.midnight.network/api-reference/ledger/globals.md): @midnight/ledger v8.1.3
- [AlignedValue](https://docs.midnight.network/api-reference/ledger/type-aliases/AlignedValue.md): @midnight/ledger v8.1.3
- [Alignment](https://docs.midnight.network/api-reference/ledger/type-aliases/Alignment.md): @midnight/ledger v8.1.3
- [AlignmentAtom](https://docs.midnight.network/api-reference/ledger/type-aliases/AlignmentAtom.md): @midnight/ledger v8.1.3
- [AlignmentSegment](https://docs.midnight.network/api-reference/ledger/type-aliases/AlignmentSegment.md): @midnight/ledger v8.1.3
- [Bindingish](https://docs.midnight.network/api-reference/ledger/type-aliases/Bindingish.md): @midnight/ledger v8.1.3
- [BlockContext](https://docs.midnight.network/api-reference/ledger/type-aliases/BlockContext.md): @midnight/ledger v8.1.3
- [CallContext](https://docs.midnight.network/api-reference/ledger/type-aliases/CallContext.md): @midnight/ledger v8.1.3
- [ClaimKind](https://docs.midnight.network/api-reference/ledger/type-aliases/ClaimKind.md): @midnight/ledger v8.1.3
- [CoinCommitment](https://docs.midnight.network/api-reference/ledger/type-aliases/CoinCommitment.md): @midnight/ledger v8.1.3
- [CoinPublicKey](https://docs.midnight.network/api-reference/ledger/type-aliases/CoinPublicKey.md): @midnight/ledger v8.1.3
- [CommunicationCommitment](https://docs.midnight.network/api-reference/ledger/type-aliases/CommunicationCommitment.md): @midnight/ledger v8.1.3
- [CommunicationCommitmentRand](https://docs.midnight.network/api-reference/ledger/type-aliases/CommunicationCommitmentRand.md): @midnight/ledger v8.1.3
- [ContractAction](https://docs.midnight.network/api-reference/ledger/type-aliases/ContractAction.md): @midnight/ledger v8.1.3
- [ContractAddress](https://docs.midnight.network/api-reference/ledger/type-aliases/ContractAddress.md): @midnight/ledger v8.1.3
- [DomainSeparator](https://docs.midnight.network/api-reference/ledger/type-aliases/DomainSeparator.md): @midnight/ledger v8.1.3
- [DustCommitment](https://docs.midnight.network/api-reference/ledger/type-aliases/DustCommitment.md): @midnight/ledger v8.1.3
- [DustGenerationInfo](https://docs.midnight.network/api-reference/ledger/type-aliases/DustGenerationInfo.md): @midnight/ledger v8.1.3
- [DustGenerationUniquenessInfo](https://docs.midnight.network/api-reference/ledger/type-aliases/DustGenerationUniquenessInfo.md): @midnight/ledger v8.1.3
- [DustInitialNonce](https://docs.midnight.network/api-reference/ledger/type-aliases/DustInitialNonce.md): @midnight/ledger v8.1.3
- [DustNonce](https://docs.midnight.network/api-reference/ledger/type-aliases/DustNonce.md): @midnight/ledger v8.1.3
- [DustNullifier](https://docs.midnight.network/api-reference/ledger/type-aliases/DustNullifier.md): @midnight/ledger v8.1.3
- [DustOutput](https://docs.midnight.network/api-reference/ledger/type-aliases/DustOutput.md): @midnight/ledger v8.1.3
- [DustPublicKey](https://docs.midnight.network/api-reference/ledger/type-aliases/DustPublicKey.md): @midnight/ledger v8.1.3
- [DustTokenType](https://docs.midnight.network/api-reference/ledger/type-aliases/DustTokenType.md): @midnight/ledger v8.1.3
- [Effects](https://docs.midnight.network/api-reference/ledger/type-aliases/Effects.md): @midnight/ledger v8.1.3
- [EncodedStateValue](https://docs.midnight.network/api-reference/ledger/type-aliases/EncodedStateValue.md): @midnight/ledger v8.1.3
- [EncPublicKey](https://docs.midnight.network/api-reference/ledger/type-aliases/EncPublicKey.md): @midnight/ledger v8.1.3
- [ErasedTransactionResult](https://docs.midnight.network/api-reference/ledger/type-aliases/ErasedTransactionResult.md): @midnight/ledger v8.1.3
- [EventDetails](https://docs.midnight.network/api-reference/ledger/type-aliases/EventDetails.md): @midnight/ledger v8.1.3
- [EventSource](https://docs.midnight.network/api-reference/ledger/type-aliases/EventSource.md): @midnight/ledger v8.1.3
- [FeePrices](https://docs.midnight.network/api-reference/ledger/type-aliases/FeePrices.md): @midnight/ledger v8.1.3
- [FinalizedTransaction](https://docs.midnight.network/api-reference/ledger/type-aliases/FinalizedTransaction.md): @midnight/ledger v8.1.3
- [Fr](https://docs.midnight.network/api-reference/ledger/type-aliases/Fr.md): @midnight/ledger v8.1.3
- [GatherResult](https://docs.midnight.network/api-reference/ledger/type-aliases/GatherResult.md): @midnight/ledger v8.1.3
- [IntentHash](https://docs.midnight.network/api-reference/ledger/type-aliases/IntentHash.md): @midnight/ledger v8.1.3
- [Key](https://docs.midnight.network/api-reference/ledger/type-aliases/Key.md): @midnight/ledger v8.1.3
- [Nonce](https://docs.midnight.network/api-reference/ledger/type-aliases/Nonce.md): @midnight/ledger v8.1.3
- [NormalizedCost](https://docs.midnight.network/api-reference/ledger/type-aliases/NormalizedCost.md): @midnight/ledger v8.1.3
- [Nullifier](https://docs.midnight.network/api-reference/ledger/type-aliases/Nullifier.md): @midnight/ledger v8.1.3
- [Op](https://docs.midnight.network/api-reference/ledger/type-aliases/Op.md): @midnight/ledger v8.1.3
- [PartitionedTranscript](https://docs.midnight.network/api-reference/ledger/type-aliases/PartitionedTranscript.md): @midnight/ledger v8.1.3
- [ProofErasedTransaction](https://docs.midnight.network/api-reference/ledger/type-aliases/ProofErasedTransaction.md): @midnight/ledger v8.1.3
- [Proofish](https://docs.midnight.network/api-reference/ledger/type-aliases/Proofish.md): @midnight/ledger v8.1.3
- [ProvingKeyMaterial](https://docs.midnight.network/api-reference/ledger/type-aliases/ProvingKeyMaterial.md): @midnight/ledger v8.1.3
- [ProvingProvider](https://docs.midnight.network/api-reference/ledger/type-aliases/ProvingProvider.md): @midnight/ledger v8.1.3
- [PublicAddress](https://docs.midnight.network/api-reference/ledger/type-aliases/PublicAddress.md): @midnight/ledger v8.1.3
- [QualifiedDustOutput](https://docs.midnight.network/api-reference/ledger/type-aliases/QualifiedDustOutput.md): @midnight/ledger v8.1.3
- [QualifiedShieldedCoinInfo](https://docs.midnight.network/api-reference/ledger/type-aliases/QualifiedShieldedCoinInfo.md): @midnight/ledger v8.1.3
- [RawTokenType](https://docs.midnight.network/api-reference/ledger/type-aliases/RawTokenType.md): @midnight/ledger v8.1.3
- [RunningCost](https://docs.midnight.network/api-reference/ledger/type-aliases/RunningCost.md): @midnight/ledger v8.1.3
- [SegmentSpecifier](https://docs.midnight.network/api-reference/ledger/type-aliases/SegmentSpecifier.md): @midnight/ledger v8.1.3
- [ShieldedCoinInfo](https://docs.midnight.network/api-reference/ledger/type-aliases/ShieldedCoinInfo.md): @midnight/ledger v8.1.3
- [ShieldedTokenType](https://docs.midnight.network/api-reference/ledger/type-aliases/ShieldedTokenType.md): @midnight/ledger v8.1.3
- [Signature](https://docs.midnight.network/api-reference/ledger/type-aliases/Signature.md): @midnight/ledger v8.1.3
- [SignatureVerifyingKey](https://docs.midnight.network/api-reference/ledger/type-aliases/SignatureVerifyingKey.md): @midnight/ledger v8.1.3
- [Signaturish](https://docs.midnight.network/api-reference/ledger/type-aliases/Signaturish.md): @midnight/ledger v8.1.3
- [SigningKey](https://docs.midnight.network/api-reference/ledger/type-aliases/SigningKey.md): @midnight/ledger v8.1.3
- [SingleUpdate](https://docs.midnight.network/api-reference/ledger/type-aliases/SingleUpdate.md): @midnight/ledger v8.1.3
- [SyntheticCost](https://docs.midnight.network/api-reference/ledger/type-aliases/SyntheticCost.md): @midnight/ledger v8.1.3
- [TokenType](https://docs.midnight.network/api-reference/ledger/type-aliases/TokenType.md): @midnight/ledger v8.1.3
- [TransactionHash](https://docs.midnight.network/api-reference/ledger/type-aliases/TransactionHash.md): @midnight/ledger v8.1.3
- [TransactionId](https://docs.midnight.network/api-reference/ledger/type-aliases/TransactionId.md): @midnight/ledger v8.1.3
- [Transcript](https://docs.midnight.network/api-reference/ledger/type-aliases/Transcript.md): @midnight/ledger v8.1.3
- [TreeInsertionPath](https://docs.midnight.network/api-reference/ledger/type-aliases/TreeInsertionPath.md): @midnight/ledger v8.1.3
- [TreeInsertionPathEntry](https://docs.midnight.network/api-reference/ledger/type-aliases/TreeInsertionPathEntry.md): @midnight/ledger v8.1.3
- [UnprovenInput](https://docs.midnight.network/api-reference/ledger/type-aliases/UnprovenInput.md): @midnight/ledger v8.1.3
- [UnprovenIntent](https://docs.midnight.network/api-reference/ledger/type-aliases/UnprovenIntent.md): @midnight/ledger v8.1.3
- [UnprovenOffer](https://docs.midnight.network/api-reference/ledger/type-aliases/UnprovenOffer.md): @midnight/ledger v8.1.3
- [UnprovenOutput](https://docs.midnight.network/api-reference/ledger/type-aliases/UnprovenOutput.md): @midnight/ledger v8.1.3
- [UnprovenTransaction](https://docs.midnight.network/api-reference/ledger/type-aliases/UnprovenTransaction.md): @midnight/ledger v8.1.3
- [UnprovenTransient](https://docs.midnight.network/api-reference/ledger/type-aliases/UnprovenTransient.md): @midnight/ledger v8.1.3
- [UnshieldedTokenType](https://docs.midnight.network/api-reference/ledger/type-aliases/UnshieldedTokenType.md): @midnight/ledger v8.1.3
- [UserAddress](https://docs.midnight.network/api-reference/ledger/type-aliases/UserAddress.md): @midnight/ledger v8.1.3
- [Utxo](https://docs.midnight.network/api-reference/ledger/type-aliases/Utxo.md): @midnight/ledger v8.1.3
- [UtxoOutput](https://docs.midnight.network/api-reference/ledger/type-aliases/UtxoOutput.md): @midnight/ledger v8.1.3
- [UtxoSpend](https://docs.midnight.network/api-reference/ledger/type-aliases/UtxoSpend.md): @midnight/ledger v8.1.3
- [Value](https://docs.midnight.network/api-reference/ledger/type-aliases/Value.md): @midnight/ledger v8.1.3

### midnight-indexer

The Midnight Indexer API exposes a GraphQL API that enables clients to query and subscribe to blockchain data—blocks, transactions, contracts, DUST generation, and shielded/unshielded transaction events, indexed from the Midnight blockchain. These capabilities facilitate both historical lookups and real-time monitoring.

- [Midnight Indexer API v4](https://docs.midnight.network/api-reference/midnight-indexer.md): The Midnight Indexer API exposes a GraphQL API that enables clients to query and subscribe to blockchain data—blocks, transactions, contracts, DUST generation, and shielded/unshielded transaction events, indexed from the Midnight blockchain. These capabilities facilitate both historical lookups and real-time monitoring.
- [include](https://docs.midnight.network/api-reference/midnight-indexer/operations/directives/include.md): Directs the executor to include this field or fragment only when the if argument is true.
- [skip](https://docs.midnight.network/api-reference/midnight-indexer/operations/directives/skip.md): Directs the executor to skip this field or fragment when the if argument is true.
- [connect](https://docs.midnight.network/api-reference/midnight-indexer/operations/mutations/connect.md): Connect the wallet with the given viewing key and return a session ID.
- [disconnect](https://docs.midnight.network/api-reference/midnight-indexer/operations/mutations/disconnect.md): Disconnect the wallet with the given session ID.
- [block](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/block.md): Find a block for the given optional offset; if not present, the latest block is returned.
- [committee](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/committee.md): Get committee membership for an epoch.
- [contractAction](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/contract-action.md): Find a contract action for the given address and optional offset.
- [currentEpochInfo](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/current-epoch-info.md): Get current epoch information.
- [dParameterHistory](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/d-parameter-history.md): Get the full history of D-parameter changes for governance auditability.
- [dustCommitmentMerkleTreeUpdate](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/dust-commitment-merkle-tree-update.md): Get a collapsed Merkle tree update for the dust commitment tree.
- [dustGenerationMerkleTreeUpdate](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/dust-generation-merkle-tree-update.md): Get a collapsed Merkle tree update for the dust generation tree.
- [dustGenerationStatus](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/dust-generation-status.md): Get DUST generation status for specific Cardano reward addresses.
- [dustGenerations](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/dust-generations.md): Get all active DUST registrations and aggregated generation stats for Cardano reward
- [epochPerformance](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/epoch-performance.md): Get epoch performance for all SPOs.
- [epochUtilization](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/epoch-utilization.md): Get epoch utilization (produced/expected ratio).
- [poolMetadata](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/pool-metadata.md): Get pool metadata by pool ID.
- [poolMetadataList](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/pool-metadata-list.md): List pool metadata with pagination.
- [registeredFirstValidEpochs](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/registered-first-valid-epochs.md): Get first valid epoch for each SPO identity.
- [registeredPresence](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/registered-presence.md): Get raw presence events for an epoch range.
- [registeredSpoSeries](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/registered-spo-series.md): Get registration statistics for an epoch range.
- [registeredTotalsSeries](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/registered-totals-series.md): Get cumulative registration totals for an epoch range.
- [spoByPoolId](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/spo-by-pool-id.md): Get SPO with metadata by pool ID.
- [spoCompositeByPoolId](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/spo-composite-by-pool-id.md): Get composite SPO data (identity + metadata + performance).
- [spoCount](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/spo-count.md): Get total count of SPOs.
- [spoIdentities](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/spo-identities.md): List SPO identities with pagination.
- [spoIdentityByPoolId](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/spo-identity-by-pool-id.md): Get SPO identity by pool ID.
- [spoList](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/spo-list.md): List SPOs with optional search.
- [spoPerformanceBySpoSk](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/spo-performance-by-spo-sk.md): Get SPO performance by SPO key.
- [spoPerformanceLatest](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/spo-performance-latest.md): Get latest SPO performance entries.
- [stakeDistribution](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/stake-distribution.md): Get stake distribution with search and ordering.
- [stakePoolOperators](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/stake-pool-operators.md): Get SPO identifiers ordered by performance.
- [termsAndConditionsHistory](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/terms-and-conditions-history.md): Get the full history of Terms and Conditions changes for governance auditability.
- [transactions](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/transactions.md): Find transactions for the given offset.
- [zswapMerkleTreeCollapsedUpdate](https://docs.midnight.network/api-reference/midnight-indexer/operations/queries/zswap-merkle-tree-collapsed-update.md): Get a Merkle tree collapsed update for the given zswap state index range.
- [blocks](https://docs.midnight.network/api-reference/midnight-indexer/operations/subscriptions/blocks.md): Subscribe to blocks starting at the given offset or at the latest block if the offset is
- [contractActions](https://docs.midnight.network/api-reference/midnight-indexer/operations/subscriptions/contract-actions.md): Subscribe to contract actions with the given address starting at the given offset or at the
- [dustGenerations](https://docs.midnight.network/api-reference/midnight-indexer/operations/subscriptions/dust-generations.md): Subscribe to dust generation entries for a dust address in [startindex, endindex]
- [dustLedgerEvents](https://docs.midnight.network/api-reference/midnight-indexer/operations/subscriptions/dust-ledger-events.md): Subscribe to dust ledger events starting at the given ID or at the very start if omitted.
- [dustNullifierTransactions](https://docs.midnight.network/api-reference/midnight-indexer/operations/subscriptions/dust-nullifier-transactions.md): Subscribe to transactions containing dust nullifiers whose 32-byte little-endian form
- [shieldedNullifierTransactions](https://docs.midnight.network/api-reference/midnight-indexer/operations/subscriptions/shielded-nullifier-transactions.md): Subscribe to transactions containing shielded (zswap) nullifiers matching the provided
- [shieldedTransactions](https://docs.midnight.network/api-reference/midnight-indexer/operations/subscriptions/shielded-transactions.md): Subscribe to shielded transaction events for the given session ID starting at the given
- [unshieldedTransactions](https://docs.midnight.network/api-reference/midnight-indexer/operations/subscriptions/unshielded-transactions.md): Subscribe unshielded transaction events for the given address and the given transaction ID
- [zswapLedgerEvents](https://docs.midnight.network/api-reference/midnight-indexer/operations/subscriptions/zswap-ledger-events.md): Subscribe to zswap ledger events starting at the given ID or at the very start if omitted.
- [beta](https://docs.midnight.network/api-reference/midnight-indexer/types/directives/beta.md): Marks a schema field or type as in-flight / unstable. Consumers should
- [deprecated](https://docs.midnight.network/api-reference/midnight-indexer/types/directives/deprecated.md): Marks an element of a GraphQL schema as no longer supported.
- [oneOf](https://docs.midnight.network/api-reference/midnight-indexer/types/directives/one-of.md): Indicates that an Input Object is a OneOf Input Object (and thus requires exactly one of its field be provided)
- [specifiedBy](https://docs.midnight.network/api-reference/midnight-indexer/types/directives/specified-by.md): Exposes a URL that specifies the behavior of this scalar.
- [TransactionResultStatus](https://docs.midnight.network/api-reference/midnight-indexer/types/enums/transaction-result-status.md): The status of the transaction result: success, partial success or failure.
- [BlockOffset](https://docs.midnight.network/api-reference/midnight-indexer/types/inputs/block-offset.md): Either a block hash or a block height.
- [ConnectOptions](https://docs.midnight.network/api-reference/midnight-indexer/types/inputs/connect-options.md): Options for the connect mutation.
- [ContractActionOffset](https://docs.midnight.network/api-reference/midnight-indexer/types/inputs/contract-action-offset.md): Either a block offset or a transaction offset.
- [TransactionOffset](https://docs.midnight.network/api-reference/midnight-indexer/types/inputs/transaction-offset.md): Either a transaction hash or a transaction identifier.
- [ContractAction](https://docs.midnight.network/api-reference/midnight-indexer/types/interfaces/contract-action.md): A contract action.
- [DustLedgerEvent](https://docs.midnight.network/api-reference/midnight-indexer/types/interfaces/dust-ledger-event.md): A dust related ledger event.
- [Transaction](https://docs.midnight.network/api-reference/midnight-indexer/types/interfaces/transaction.md): A Midnight transaction.
- [Block](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/block.md): A block with its relevant data.
- [CollapsedMerkleTree](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/collapsed-merkle-tree.md): A Merkle tree collapsed update between two indices.
- [CommitteeMember](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/committee-member.md): Committee member for an epoch.
- [ContractBalance](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/contract-balance.md): Represents a token balance held by a contract.
- [ContractCall](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/contract-call.md): A contract call.
- [ContractDeploy](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/contract-deploy.md): A contract deployment.
- [ContractUpdate](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/contract-update.md): A contract update.
- [DParameter](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/dparameter.md): The D-parameter controlling validator committee composition.
- [DParameterChange](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/dparameter-change.md): D-parameter change record for history queries.
- [DustGenerationDtimeUpdate](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/dust-generation-dtime-update.md): No description
- [DustGenerationDtimeUpdateItem](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/dust-generation-dtime-update-item.md): A dust generation dtime update emitted when the backing Night UTXO is
- [DustGenerationStatus](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/dust-generation-status.md): DUST generation status for a specific Cardano reward address.
- [DustGenerations](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/dust-generations.md): Dust generations for a Cardano reward address.
- [DustGenerationsItem](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/dust-generations-item.md): A dust generations item with optional collapsed Merkle tree update.
- [DustGenerationsProgress](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/dust-generations-progress.md): Progress indicator for dust generations subscription (includes final collapsed update).
- [DustInitialUtxo](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/dust-initial-utxo.md): No description
- [DustNullifierTransaction](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/dust-nullifier-transaction.md): A transaction containing a dust nullifier match with block context.
- [DustOutput](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/dust-output.md): A dust output.
- [DustRegistration](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/dust-registration.md): A single dust registration with aggregated generation stats.
- [DustSpendProcessed](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/dust-spend-processed.md): No description
- [EpochInfo](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/epoch-info.md): Current epoch information.
- [EpochPerf](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/epoch-perf.md): SPO performance for an epoch.
- [FirstValidEpoch](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/first-valid-epoch.md): First valid epoch for an SPO identity.
- [MerkleTreeCollapsedUpdate](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/merkle-tree-collapsed-update.md): A Merkle tree collapsed update between two indices.
- [ParamChange](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/param-change.md): No description
- [PoolMetadata](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/pool-metadata.md): Pool metadata from Cardano.
- [PresenceEvent](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/presence-event.md): Presence event for an SPO in an epoch.
- [RegisteredStat](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/registered-stat.md): Registration statistics for an epoch.
- [RegisteredTotals](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/registered-totals.md): Cumulative registration totals for an epoch.
- [RegularTransaction](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/regular-transaction.md): A regular Midnight transaction.
- [RelevantTransaction](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/relevant-transaction.md): A transaction relevant for the subscribing wallet and an optional zswap state Merkle tree
- [Segment](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/segment.md): One of many segments for a partially successful transaction result showing success for some
- [ShieldedNullifierTransaction](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/shielded-nullifier-transaction.md): A transaction containing a shielded (zswap) nullifier match with block context.
- [ShieldedTransactionsProgress](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/shielded-transactions-progress.md): Information about the shielded transactions indexing progress.
- [Spo](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/spo.md): SPO with optional metadata.
- [SpoComposite](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/spo-composite.md): Composite SPO data (identity + metadata + performance).
- [SpoIdentity](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/spo-identity.md): SPO identity information.
- [StakeShare](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/stake-share.md): Stake share information for an SPO.
- [SystemParameters](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/system-parameters.md): System parameters at a specific block height.
- [SystemTransaction](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/system-transaction.md): A system Midnight transaction.
- [TermsAndConditions](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/terms-and-conditions.md): Terms and Conditions agreement.
- [TermsAndConditionsChange](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/terms-and-conditions-change.md): Terms and Conditions change record for history queries.
- [TransactionFees](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/transaction-fees.md): Fees information for a transaction.
- [TransactionResult](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/transaction-result.md): The result of applying a transaction to the ledger state. In case of a partial success (status),
- [UnshieldedTransaction](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/unshielded-transaction.md): A transaction that created and/or spent UTXOs alongside these and other information.
- [UnshieldedTransactionsProgress](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/unshielded-transactions-progress.md): Information about the unshielded indexing progress.
- [UnshieldedUtxo](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/unshielded-utxo.md): Represents an unshielded UTXO.
- [ZswapLedgerEvent](https://docs.midnight.network/api-reference/midnight-indexer/types/objects/zswap-ledger-event.md): A zswap related ledger event.
- [Boolean](https://docs.midnight.network/api-reference/midnight-indexer/types/scalars/boolean.md): The Boolean scalar type represents true or false.
- [CardanoRewardAddress](https://docs.midnight.network/api-reference/midnight-indexer/types/scalars/cardano-reward-address.md): No description
- [DustAddress](https://docs.midnight.network/api-reference/midnight-indexer/types/scalars/dust-address.md): No description
- [Float](https://docs.midnight.network/api-reference/midnight-indexer/types/scalars/float.md): The Float scalar type represents signed double-precision fractional values as specified by IEEE 754.
- [HexEncoded](https://docs.midnight.network/api-reference/midnight-indexer/types/scalars/hex-encoded.md): No description
- [Int](https://docs.midnight.network/api-reference/midnight-indexer/types/scalars/int.md): The Int scalar type represents non-fractional signed whole numeric values. Int can represent values between -(2^31) and 2^31 - 1.
- [String](https://docs.midnight.network/api-reference/midnight-indexer/types/scalars/string.md): The String scalar type represents textual data, represented as UTF-8 character sequences. The String type is most often used by GraphQL to represent free-form human-readable text.
- [Unit](https://docs.midnight.network/api-reference/midnight-indexer/types/scalars/unit.md): No description
- [UnshieldedAddress](https://docs.midnight.network/api-reference/midnight-indexer/types/scalars/unshielded-address.md): No description
- [ViewingKey](https://docs.midnight.network/api-reference/midnight-indexer/types/scalars/viewing-key.md): No description
- [DustGenerationsEvent](https://docs.midnight.network/api-reference/midnight-indexer/types/unions/dust-generations-event.md): An event of the dust generations subscription.
- [ShieldedTransactionsEvent](https://docs.midnight.network/api-reference/midnight-indexer/types/unions/shielded-transactions-event.md): An event of the shielded transactions subscription.
- [UnshieldedTransactionsEvent](https://docs.midnight.network/api-reference/midnight-indexer/types/unions/unshielded-transactions-event.md): An event of the unshielded transactions subscription.

### midnight-js

Midnight.js API Reference v4.1.0

- [Midnight JS API](https://docs.midnight.network/api-reference/midnight-js.md): Midnight.js API Reference v4.1.0
- [midnight-js](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js.md): Midnight.js API Reference v4.1.0
- [midnight-js-contracts](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md): Midnight.js API Reference v4.1.0
- [CallTxFailedError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/CallTxFailedError.md): Midnight.js API Reference v4.1.0
- [ContractTypeError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/ContractTypeError.md): Midnight.js API Reference v4.1.0
- [DeployTxFailedError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/DeployTxFailedError.md): Midnight.js API Reference v4.1.0
- [IncompleteCallTxPrivateStateConfig](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/IncompleteCallTxPrivateStateConfig.md): Midnight.js API Reference v4.1.0
- [IncompleteFindContractPrivateStateConfig](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/IncompleteFindContractPrivateStateConfig.md): Midnight.js API Reference v4.1.0
- [InsertVerifierKeyTxFailedError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/InsertVerifierKeyTxFailedError.md): Midnight.js API Reference v4.1.0
- [RemoveVerifierKeyTxFailedError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/RemoveVerifierKeyTxFailedError.md): Midnight.js API Reference v4.1.0
- [ReplaceMaintenanceAuthorityTxFailedError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/ReplaceMaintenanceAuthorityTxFailedError.md): Midnight.js API Reference v4.1.0
- [TxFailedError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/TxFailedError.md): Midnight.js API Reference v4.1.0
- [createCallTxOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/createCallTxOptions.md): Midnight.js API Reference v4.1.0
- [createCircuitCallTxInterface](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/createCircuitCallTxInterface.md): Midnight.js API Reference v4.1.0
- [createCircuitMaintenanceTxInterface](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/createCircuitMaintenanceTxInterface.md): Midnight.js API Reference v4.1.0
- [createCircuitMaintenanceTxInterfaces](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/createCircuitMaintenanceTxInterfaces.md): Midnight.js API Reference v4.1.0
- [createContractMaintenanceTxInterface](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/createContractMaintenanceTxInterface.md): Midnight.js API Reference v4.1.0
- [createUnprovenCallTx](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/createUnprovenCallTx.md): Midnight.js API Reference v4.1.0
- [createUnprovenCallTxFromInitialStates](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/createUnprovenCallTxFromInitialStates.md): Midnight.js API Reference v4.1.0
- [createUnprovenDeployTx](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/createUnprovenDeployTx.md): Midnight.js API Reference v4.1.0
- [createUnprovenDeployTxFromVerifierKeys](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/createUnprovenDeployTxFromVerifierKeys.md): Midnight.js API Reference v4.1.0
- [deployContract](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/deployContract.md): Midnight.js API Reference v4.1.0
- [findDeployedContract](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/findDeployedContract.md): Midnight.js API Reference v4.1.0
- [getPublicStates](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/getPublicStates.md): Midnight.js API Reference v4.1.0
- [getStates](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/getStates.md): Midnight.js API Reference v4.1.0
- [getUnshieldedBalances](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/getUnshieldedBalances.md): Midnight.js API Reference v4.1.0
- [submitCallTx](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/submitCallTx.md): Midnight.js API Reference v4.1.0
- [submitCallTxAsync](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/submitCallTxAsync.md): Midnight.js API Reference v4.1.0
- [submitDeployTx](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/submitDeployTx.md): Midnight.js API Reference v4.1.0
- [submitInsertVerifierKeyTx](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/submitInsertVerifierKeyTx.md): Midnight.js API Reference v4.1.0
- [submitRemoveVerifierKeyTx](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/submitRemoveVerifierKeyTx.md): Midnight.js API Reference v4.1.0
- [submitReplaceAuthorityTx](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/submitReplaceAuthorityTx.md): Midnight.js API Reference v4.1.0
- [submitTx](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/submitTx.md): Midnight.js API Reference v4.1.0
- [submitTxAsync](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/submitTxAsync.md): Midnight.js API Reference v4.1.0
- [verifierKeysEqual](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/verifierKeysEqual.md): Midnight.js API Reference v4.1.0
- [verifyContractState](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/verifyContractState.md): Midnight.js API Reference v4.1.0
- [ContractMaintenanceTxInterface](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/interfaces/ContractMaintenanceTxInterface.md): Midnight.js API Reference v4.1.0
- [TransactionContext](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/interfaces/TransactionContext.md): Midnight.js API Reference v4.1.0
- [CallOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallOptions.md): Midnight.js API Reference v4.1.0
- [CallOptionsBase](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallOptionsBase.md): Midnight.js API Reference v4.1.0
- [CallOptionsProviderDataDependencies](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallOptionsProviderDataDependencies.md): Midnight.js API Reference v4.1.0
- [CallOptionsWithArguments](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallOptionsWithArguments.md): Midnight.js API Reference v4.1.0
- [CallOptionsWithPrivateState](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallOptionsWithPrivateState.md): Midnight.js API Reference v4.1.0
- [CallOptionsWithProviderDataDependencies](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallOptionsWithProviderDataDependencies.md): Midnight.js API Reference v4.1.0
- [CallResult](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallResult.md): Midnight.js API Reference v4.1.0
- [CallResultPrivate](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallResultPrivate.md): Midnight.js API Reference v4.1.0
- [CallResultPublic](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallResultPublic.md): Midnight.js API Reference v4.1.0
- [CallTxOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallTxOptions.md): Midnight.js API Reference v4.1.0
- [CallTxOptionsBase](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallTxOptionsBase.md): Midnight.js API Reference v4.1.0
- [CallTxOptionsWithPrivateStateId](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallTxOptionsWithPrivateStateId.md): Midnight.js API Reference v4.1.0
- [CircuitCallTxInterface](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CircuitCallTxInterface.md): Midnight.js API Reference v4.1.0
- [CircuitMaintenanceTxInterface](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CircuitMaintenanceTxInterface.md): Midnight.js API Reference v4.1.0
- [CircuitMaintenanceTxInterfaces](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CircuitMaintenanceTxInterfaces.md): Midnight.js API Reference v4.1.0
- [ContractConstructorOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractConstructorOptions.md): Midnight.js API Reference v4.1.0
- [ContractConstructorOptionsBase](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractConstructorOptionsBase.md): Midnight.js API Reference v4.1.0
- [ContractConstructorOptionsProviderDataDependencies](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractConstructorOptionsProviderDataDependencies.md): Midnight.js API Reference v4.1.0
- [ContractConstructorOptionsWithArguments](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractConstructorOptionsWithArguments.md): Midnight.js API Reference v4.1.0
- [ContractConstructorOptionsWithPrivateState](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractConstructorOptionsWithPrivateState.md): Midnight.js API Reference v4.1.0
- [ContractConstructorOptionsWithProviderDataDependencies](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractConstructorOptionsWithProviderDataDependencies.md): Midnight.js API Reference v4.1.0
- [ContractConstructorResult](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractConstructorResult.md): Midnight.js API Reference v4.1.0
- [ContractProviders](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md): Midnight.js API Reference v4.1.0
- [ContractStates](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractStates.md): Midnight.js API Reference v4.1.0
- [DeployContractOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployContractOptions.md): Midnight.js API Reference v4.1.0
- [DeployContractOptionsBase](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployContractOptionsBase.md): Midnight.js API Reference v4.1.0
- [DeployContractOptionsWithPrivateState](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployContractOptionsWithPrivateState.md): Midnight.js API Reference v4.1.0
- [DeployedContract](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployedContract.md): Midnight.js API Reference v4.1.0
- [DeployTxOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployTxOptions.md): Midnight.js API Reference v4.1.0
- [DeployTxOptionsBase](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployTxOptionsBase.md): Midnight.js API Reference v4.1.0
- [DeployTxOptionsWithPrivateState](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployTxOptionsWithPrivateState.md): Midnight.js API Reference v4.1.0
- [DeployTxOptionsWithPrivateStateId](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployTxOptionsWithPrivateStateId.md): Midnight.js API Reference v4.1.0
- [FinalizedCallTxData](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedCallTxData.md): Midnight.js API Reference v4.1.0
- [FinalizedDeployTxData](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedDeployTxData.md): Midnight.js API Reference v4.1.0
- [FinalizedDeployTxDataBase](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedDeployTxDataBase.md): Midnight.js API Reference v4.1.0
- [FindDeployedContractOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FindDeployedContractOptions.md): Midnight.js API Reference v4.1.0
- [FindDeployedContractOptionsBase](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FindDeployedContractOptionsBase.md): Midnight.js API Reference v4.1.0
- [FindDeployedContractOptionsExistingPrivateState](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FindDeployedContractOptionsExistingPrivateState.md): Midnight.js API Reference v4.1.0
- [FindDeployedContractOptionsStorePrivateState](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FindDeployedContractOptionsStorePrivateState.md): Midnight.js API Reference v4.1.0
- [FoundContract](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FoundContract.md): Midnight.js API Reference v4.1.0
- [PublicContractStates](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/PublicContractStates.md): Midnight.js API Reference v4.1.0
- [ScopedTransactionOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ScopedTransactionOptions.md): Midnight.js API Reference v4.1.0
- [SubmittedCallTx](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/SubmittedCallTx.md): Midnight.js API Reference v4.1.0
- [SubmitTxOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/SubmitTxOptions.md): Midnight.js API Reference v4.1.0
- [SubmitTxProviders](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/SubmitTxProviders.md): Midnight.js API Reference v4.1.0
- [UnprovenCallTxProvidersBase](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnprovenCallTxProvidersBase.md): Midnight.js API Reference v4.1.0
- [UnprovenCallTxProvidersWithPrivateState](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnprovenCallTxProvidersWithPrivateState.md): Midnight.js API Reference v4.1.0
- [UnprovenDeployTxOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnprovenDeployTxOptions.md): Midnight.js API Reference v4.1.0
- [UnprovenDeployTxProviders](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnprovenDeployTxProviders.md): Midnight.js API Reference v4.1.0
- [UnsubmittedCallTxData](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md): Midnight.js API Reference v4.1.0
- [UnsubmittedDeployTxData](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxData.md): Midnight.js API Reference v4.1.0
- [UnsubmittedDeployTxDataBase](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxDataBase.md): Midnight.js API Reference v4.1.0
- [UnsubmittedDeployTxPrivateData](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxPrivateData.md): Midnight.js API Reference v4.1.0
- [UnsubmittedDeployTxPublicData](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxPublicData.md): Midnight.js API Reference v4.1.0
- [UnsubmittedTxData](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedTxData.md): Midnight.js API Reference v4.1.0
- [withContractScopedTransaction](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/variables/withContractScopedTransaction.md): Midnight.js API Reference v4.1.0
- [midnight-js-dapp-connector-proof-provider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-dapp-connector-proof-provider.md): Midnight.js API Reference v4.1.0
- [dappConnectorProofProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-dapp-connector-proof-provider/functions/dappConnectorProofProvider.md): Midnight.js API Reference v4.1.0
- [dappConnectorProvingProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-dapp-connector-proof-provider/functions/dappConnectorProvingProvider.md): Midnight.js API Reference v4.1.0
- [DAppConnectorProvingAPI](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-dapp-connector-proof-provider/type-aliases/DAppConnectorProvingAPI.md): Midnight.js API Reference v4.1.0
- [midnight-js-fetch-zk-config-provider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-fetch-zk-config-provider.md): Midnight.js API Reference v4.1.0
- [FetchZkConfigProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-fetch-zk-config-provider/classes/FetchZkConfigProvider.md): Midnight.js API Reference v4.1.0
- [midnight-js-http-client-proof-provider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-http-client-proof-provider.md): Midnight.js API Reference v4.1.0
- [httpClientProofProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-http-client-proof-provider/functions/httpClientProofProvider.md): Midnight.js API Reference v4.1.0
- [httpClientProvingProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-http-client-proof-provider/functions/httpClientProvingProvider.md): Midnight.js API Reference v4.1.0
- [ProvingProviderConfig](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-http-client-proof-provider/interfaces/ProvingProviderConfig.md): Midnight.js API Reference v4.1.0
- [DEFAULT_CONFIG](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-http-client-proof-provider/variables/DEFAULT_CONFIG.md): Midnight.js API Reference v4.1.0
- [DEFAULT_TIMEOUT](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-http-client-proof-provider/variables/DEFAULT_TIMEOUT.md): Midnight.js API Reference v4.1.0
- [midnight-js-indexer-public-data-provider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider.md): Midnight.js API Reference v4.1.0
- [IndexerDataError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/classes/IndexerDataError.md): Midnight.js API Reference v4.1.0
- [IndexerError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/classes/IndexerError.md): Midnight.js API Reference v4.1.0
- [IndexerFormattedError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/classes/IndexerFormattedError.md): Midnight.js API Reference v4.1.0
- [IndexerProviderConfigError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/classes/IndexerProviderConfigError.md): Midnight.js API Reference v4.1.0
- [IndexerQueryError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/classes/IndexerQueryError.md): Midnight.js API Reference v4.1.0
- [IndexerSubscriptionDataError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/classes/IndexerSubscriptionDataError.md): Midnight.js API Reference v4.1.0
- [correlateDeployTxId](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/functions/correlateDeployTxId.md): Midnight.js API Reference v4.1.0
- [indexerPublicDataProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/functions/indexerPublicDataProvider.md): Midnight.js API Reference v4.1.0
- [isRegularTransaction](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/functions/isRegularTransaction.md): Midnight.js API Reference v4.1.0
- [toSegmentStatus](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/functions/toSegmentStatus.md): Midnight.js API Reference v4.1.0
- [toSegmentStatusMap](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/functions/toSegmentStatusMap.md): Midnight.js API Reference v4.1.0
- [toTxStatus](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/functions/toTxStatus.md): Midnight.js API Reference v4.1.0
- [toUnshieldedBalances](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/functions/toUnshieldedBalances.md): Midnight.js API Reference v4.1.0
- [toUnshieldedUtxos](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/functions/toUnshieldedUtxos.md): Midnight.js API Reference v4.1.0
- [IndexerDataErrorContext](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/type-aliases/IndexerDataErrorContext.md): Midnight.js API Reference v4.1.0
- [IndexerSubscriptionField](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/type-aliases/IndexerSubscriptionField.md): Midnight.js API Reference v4.1.0
- [IndexerUtxo](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider/type-aliases/IndexerUtxo.md): Midnight.js API Reference v4.1.0
- [midnight-js-level-private-state-provider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider.md): Midnight.js API Reference v4.1.0
- [StorageEncryption](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/classes/StorageEncryption.md): Midnight.js API Reference v4.1.0
- [decryptValue](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/functions/decryptValue.md): Midnight.js API Reference v4.1.0
- [levelPrivateStateProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/functions/levelPrivateStateProvider.md): Midnight.js API Reference v4.1.0
- [migrateToAccountScoped](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/functions/migrateToAccountScoped.md): Midnight.js API Reference v4.1.0
- [CryptoBackend](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/interfaces/CryptoBackend.md): Midnight.js API Reference v4.1.0
- [LevelPrivateStateProviderConfig](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/interfaces/LevelPrivateStateProviderConfig.md): Midnight.js API Reference v4.1.0
- [MigrationResult](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/interfaces/MigrationResult.md): Midnight.js API Reference v4.1.0
- [PasswordRotationOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/interfaces/PasswordRotationOptions.md): Midnight.js API Reference v4.1.0
- [PasswordRotationResult](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/interfaces/PasswordRotationResult.md): Midnight.js API Reference v4.1.0
- [StorageEncryptionOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/interfaces/StorageEncryptionOptions.md): Midnight.js API Reference v4.1.0
- [CryptoBackendType](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/type-aliases/CryptoBackendType.md): Midnight.js API Reference v4.1.0
- [DatabaseLevel](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/type-aliases/DatabaseLevel.md): Midnight.js API Reference v4.1.0
- [LevelFactory](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/type-aliases/LevelFactory.md): Midnight.js API Reference v4.1.0
- [PrivateStoragePasswordProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/type-aliases/PrivateStoragePasswordProvider.md): Midnight.js API Reference v4.1.0
- [DEFAULT_CONFIG](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/variables/DEFAULT_CONFIG.md): Midnight.js API Reference v4.1.0
- [midnight-js-logger-provider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-logger-provider.md): Midnight.js API Reference v4.1.0
- [LoggerProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-logger-provider/classes/LoggerProvider.md): Midnight.js API Reference v4.1.0
- [midnight-js-network-id](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-network-id.md): Midnight.js API Reference v4.1.0
- [getNetworkId](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-network-id/functions/getNetworkId.md): Midnight.js API Reference v4.1.0
- [setNetworkId](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-network-id/functions/setNetworkId.md): Midnight.js API Reference v4.1.0
- [NetworkId](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-network-id/type-aliases/NetworkId.md): Midnight.js API Reference v4.1.0
- [midnight-js-node-zk-config-provider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-node-zk-config-provider.md): Midnight.js API Reference v4.1.0
- [NodeZkConfigProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-node-zk-config-provider/classes/NodeZkConfigProvider.md): Midnight.js API Reference v4.1.0
- [midnight-js-protocol](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-protocol.md): Midnight.js API Reference v4.1.0
- [midnight-js-types](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md): Midnight.js API Reference v4.1.0
- [ExportDecryptionError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/ExportDecryptionError.md): Midnight.js API Reference v4.1.0
- [ImportConflictError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/ImportConflictError.md): Midnight.js API Reference v4.1.0
- [InvalidExportFormatError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/InvalidExportFormatError.md): Midnight.js API Reference v4.1.0
- [InvalidProtocolSchemeError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/InvalidProtocolSchemeError.md): Midnight.js API Reference v4.1.0
- [PrivateStateExportError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/PrivateStateExportError.md): Midnight.js API Reference v4.1.0
- [PrivateStateImportError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/PrivateStateImportError.md): Midnight.js API Reference v4.1.0
- [SigningKeyExportError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/SigningKeyExportError.md): Midnight.js API Reference v4.1.0
- [ZKConfigProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/ZKConfigProvider.md): Midnight.js API Reference v4.1.0
- [LogLevel](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/enumerations/LogLevel.md): Midnight.js API Reference v4.1.0
- [asContractAddress](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/functions/asContractAddress.md): Midnight.js API Reference v4.1.0
- [asEffectOption](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/functions/asEffectOption.md): Midnight.js API Reference v4.1.0
- [createProofProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/functions/createProofProvider.md): Midnight.js API Reference v4.1.0
- [createProverKey](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/functions/createProverKey.md): Midnight.js API Reference v4.1.0
- [createVerifierKey](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/functions/createVerifierKey.md): Midnight.js API Reference v4.1.0
- [createZKIR](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/functions/createZKIR.md): Midnight.js API Reference v4.1.0
- [zkConfigToProvingKeyMaterial](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/functions/zkConfigToProvingKeyMaterial.md): Midnight.js API Reference v4.1.0
- [ExportPrivateStatesOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ExportPrivateStatesOptions.md): Midnight.js API Reference v4.1.0
- [ExportSigningKeysOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ExportSigningKeysOptions.md): Midnight.js API Reference v4.1.0
- [FinalizedTxData](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/FinalizedTxData.md): Midnight.js API Reference v4.1.0
- [ImportPrivateStatesOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ImportPrivateStatesOptions.md): Midnight.js API Reference v4.1.0
- [ImportPrivateStatesResult](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ImportPrivateStatesResult.md): Midnight.js API Reference v4.1.0
- [ImportSigningKeysOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ImportSigningKeysOptions.md): Midnight.js API Reference v4.1.0
- [ImportSigningKeysResult](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ImportSigningKeysResult.md): Midnight.js API Reference v4.1.0
- [LoggerProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/LoggerProvider.md): Midnight.js API Reference v4.1.0
- [MidnightProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/MidnightProvider.md): Midnight.js API Reference v4.1.0
- [MidnightProviders](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/MidnightProviders.md): Midnight.js API Reference v4.1.0
- [PrivateStateExport](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/PrivateStateExport.md): Midnight.js API Reference v4.1.0
- [PrivateStateProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/PrivateStateProvider.md): Midnight.js API Reference v4.1.0
- [ProofProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ProofProvider.md): Midnight.js API Reference v4.1.0
- [ProveTxConfig](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ProveTxConfig.md): Midnight.js API Reference v4.1.0
- [PublicDataProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/PublicDataProvider.md): Midnight.js API Reference v4.1.0
- [SigningKeyExport](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/SigningKeyExport.md): Midnight.js API Reference v4.1.0
- [WalletProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/WalletProvider.md): Midnight.js API Reference v4.1.0
- [ZKConfig](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ZKConfig.md): Midnight.js API Reference v4.1.0
- [All](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/All.md): Midnight.js API Reference v4.1.0
- [AnyPrivateState](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/AnyPrivateState.md): Midnight.js API Reference v4.1.0
- [AnyProvableCircuitId](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/AnyProvableCircuitId.md): Midnight.js API Reference v4.1.0
- [BlockHash](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/BlockHash.md): Midnight.js API Reference v4.1.0
- [BlockHashConfig](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/BlockHashConfig.md): Midnight.js API Reference v4.1.0
- [BlockHeightConfig](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/BlockHeightConfig.md): Midnight.js API Reference v4.1.0
- [ContractExecutableRuntimeOptions](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ContractExecutableRuntimeOptions.md): Midnight.js API Reference v4.1.0
- [ContractStateObservableConfig](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ContractStateObservableConfig.md): Midnight.js API Reference v4.1.0
- [Fees](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/Fees.md): Midnight.js API Reference v4.1.0
- [KeyMaterialProvider](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/KeyMaterialProvider.md): Midnight.js API Reference v4.1.0
- [Latest](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/Latest.md): Midnight.js API Reference v4.1.0
- [PrivateStateId](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/PrivateStateId.md): Midnight.js API Reference v4.1.0
- [PrivateStateImportErrorCause](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/PrivateStateImportErrorCause.md): Midnight.js API Reference v4.1.0
- [ProverKey](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ProverKey.md): Midnight.js API Reference v4.1.0
- [SegmentStatus](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/SegmentStatus.md): Midnight.js API Reference v4.1.0
- [TxIdConfig](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/TxIdConfig.md): Midnight.js API Reference v4.1.0
- [TxStatus](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/TxStatus.md): Midnight.js API Reference v4.1.0
- [UnboundTransaction](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnboundTransaction.md): Midnight.js API Reference v4.1.0
- [UnshieldedBalance](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnshieldedBalance.md): Midnight.js API Reference v4.1.0
- [UnshieldedBalances](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnshieldedBalances.md): Midnight.js API Reference v4.1.0
- [UnshieldedUtxo](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnshieldedUtxo.md): Midnight.js API Reference v4.1.0
- [UnshieldedUtxos](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnshieldedUtxos.md): Midnight.js API Reference v4.1.0
- [VerifierKey](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/VerifierKey.md): Midnight.js API Reference v4.1.0
- [ZKIR](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ZKIR.md): Midnight.js API Reference v4.1.0
- [exitResultOrError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/variables/exitResultOrError.md): Midnight.js API Reference v4.1.0
- [FailEntirely](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/variables/FailEntirely.md): Midnight.js API Reference v4.1.0
- [FailFallible](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/variables/FailFallible.md): Midnight.js API Reference v4.1.0
- [makeContractExecutableRuntime](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/variables/makeContractExecutableRuntime.md): Midnight.js API Reference v4.1.0
- [MAX_EXPORT_SIGNING_KEYS](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/variables/MAX_EXPORT_SIGNING_KEYS.md): Midnight.js API Reference v4.1.0
- [MAX_EXPORT_STATES](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/variables/MAX_EXPORT_STATES.md): Midnight.js API Reference v4.1.0
- [SegmentFail](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/variables/SegmentFail.md): Midnight.js API Reference v4.1.0
- [SegmentSuccess](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/variables/SegmentSuccess.md): Midnight.js API Reference v4.1.0
- [SucceedEntirely](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/variables/SucceedEntirely.md): Midnight.js API Reference v4.1.0
- [midnight-js-utils](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils.md): Midnight.js API Reference v4.1.0
- [PasswordValidationError](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/classes/PasswordValidationError.md): Midnight.js API Reference v4.1.0
- [assertDefined](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/assertDefined.md): Midnight.js API Reference v4.1.0
- [assertIsContractAddress](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/assertIsContractAddress.md): Midnight.js API Reference v4.1.0
- [assertIsHex](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/assertIsHex.md): Midnight.js API Reference v4.1.0
- [assertSafeName](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/assertSafeName.md): Midnight.js API Reference v4.1.0
- [assertSemVer](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/assertSemVer.md): Midnight.js API Reference v4.1.0
- [assertUndefined](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/assertUndefined.md): Midnight.js API Reference v4.1.0
- [fromHex](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/fromHex.md): Midnight.js API Reference v4.1.0
- [isHex](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/isHex.md): Midnight.js API Reference v4.1.0
- [parseCoinPublicKeyToHex](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/parseCoinPublicKeyToHex.md): Midnight.js API Reference v4.1.0
- [parseEncPublicKeyToHex](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/parseEncPublicKeyToHex.md): Midnight.js API Reference v4.1.0
- [parseHex](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/parseHex.md): Midnight.js API Reference v4.1.0
- [toHex](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/toHex.md): Midnight.js API Reference v4.1.0
- [ttlOneHour](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/ttlOneHour.md): Midnight.js API Reference v4.1.0
- [validatePassword](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/validatePassword.md): Midnight.js API Reference v4.1.0
- [warnIfInsecureRemoteUrl](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/functions/warnIfInsecureRemoteUrl.md): Midnight.js API Reference v4.1.0
- [ParsedHexString](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/type-aliases/ParsedHexString.md): Midnight.js API Reference v4.1.0
- [PasswordValidationFailure](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/type-aliases/PasswordValidationFailure.md): Midnight.js API Reference v4.1.0
- [MAX_CONSECUTIVE_REPEATED](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/variables/MAX_CONSECUTIVE_REPEATED.md): Midnight.js API Reference v4.1.0
- [MAX_SAFE_NAME_LENGTH](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/variables/MAX_SAFE_NAME_LENGTH.md): Midnight.js API Reference v4.1.0
- [MIN_CHARACTER_CLASSES](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/variables/MIN_CHARACTER_CLASSES.md): Midnight.js API Reference v4.1.0
- [MIN_PASSWORD_LENGTH](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/variables/MIN_PASSWORD_LENGTH.md): Midnight.js API Reference v4.1.0
- [MIN_SEQUENTIAL_LENGTH](https://docs.midnight.network/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/variables/MIN_SEQUENTIAL_LENGTH.md): Midnight.js API Reference v4.1.0
- [packages](https://docs.midnight.network/api-reference/midnight-js/packages.md): Midnight.js API Reference v4.1.0

### onchain-runtime

@midnight-ntwrk/onchain-runtime v3.1.2

- [Onchain Runtime API](https://docs.midnight.network/api-reference/onchain-runtime.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [ChargedState](https://docs.midnight.network/api-reference/onchain-runtime/classes/ChargedState.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [CoinSecretKey](https://docs.midnight.network/api-reference/onchain-runtime/classes/CoinSecretKey.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [ContractMaintenanceAuthority](https://docs.midnight.network/api-reference/onchain-runtime/classes/ContractMaintenanceAuthority.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [ContractOperation](https://docs.midnight.network/api-reference/onchain-runtime/classes/ContractOperation.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [ContractState](https://docs.midnight.network/api-reference/onchain-runtime/classes/ContractState.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [CostModel](https://docs.midnight.network/api-reference/onchain-runtime/classes/CostModel.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [QueryContext](https://docs.midnight.network/api-reference/onchain-runtime/classes/QueryContext.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [QueryResults](https://docs.midnight.network/api-reference/onchain-runtime/classes/QueryResults.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [StateBoundedMerkleTree](https://docs.midnight.network/api-reference/onchain-runtime/classes/StateBoundedMerkleTree.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [StateMap](https://docs.midnight.network/api-reference/onchain-runtime/classes/StateMap.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [StateValue](https://docs.midnight.network/api-reference/onchain-runtime/classes/StateValue.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [VmResults](https://docs.midnight.network/api-reference/onchain-runtime/classes/VmResults.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [VmStack](https://docs.midnight.network/api-reference/onchain-runtime/classes/VmStack.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [bigIntModFr](https://docs.midnight.network/api-reference/onchain-runtime/functions/bigIntModFr.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [bigIntToValue](https://docs.midnight.network/api-reference/onchain-runtime/functions/bigIntToValue.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [communicationCommitment](https://docs.midnight.network/api-reference/onchain-runtime/functions/communicationCommitment.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [communicationCommitmentRandomness](https://docs.midnight.network/api-reference/onchain-runtime/functions/communicationCommitmentRandomness.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [decodeCoinPublicKey](https://docs.midnight.network/api-reference/onchain-runtime/functions/decodeCoinPublicKey.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [decodeContractAddress](https://docs.midnight.network/api-reference/onchain-runtime/functions/decodeContractAddress.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [decodeQualifiedShieldedCoinInfo](https://docs.midnight.network/api-reference/onchain-runtime/functions/decodeQualifiedShieldedCoinInfo.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [decodeRawTokenType](https://docs.midnight.network/api-reference/onchain-runtime/functions/decodeRawTokenType.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [decodeShieldedCoinInfo](https://docs.midnight.network/api-reference/onchain-runtime/functions/decodeShieldedCoinInfo.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [decodeUserAddress](https://docs.midnight.network/api-reference/onchain-runtime/functions/decodeUserAddress.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [degradeToTransient](https://docs.midnight.network/api-reference/onchain-runtime/functions/degradeToTransient.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [dummyContractAddress](https://docs.midnight.network/api-reference/onchain-runtime/functions/dummyContractAddress.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [dummyUserAddress](https://docs.midnight.network/api-reference/onchain-runtime/functions/dummyUserAddress.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [ecAdd](https://docs.midnight.network/api-reference/onchain-runtime/functions/ecAdd.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [ecMul](https://docs.midnight.network/api-reference/onchain-runtime/functions/ecMul.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [ecMulGenerator](https://docs.midnight.network/api-reference/onchain-runtime/functions/ecMulGenerator.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [encodeCoinPublicKey](https://docs.midnight.network/api-reference/onchain-runtime/functions/encodeCoinPublicKey.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [encodeContractAddress](https://docs.midnight.network/api-reference/onchain-runtime/functions/encodeContractAddress.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [encodeQualifiedShieldedCoinInfo](https://docs.midnight.network/api-reference/onchain-runtime/functions/encodeQualifiedShieldedCoinInfo.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [encodeRawTokenType](https://docs.midnight.network/api-reference/onchain-runtime/functions/encodeRawTokenType.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [encodeShieldedCoinInfo](https://docs.midnight.network/api-reference/onchain-runtime/functions/encodeShieldedCoinInfo.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [encodeUserAddress](https://docs.midnight.network/api-reference/onchain-runtime/functions/encodeUserAddress.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [entryPointHash](https://docs.midnight.network/api-reference/onchain-runtime/functions/entryPointHash.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [hashToCurve](https://docs.midnight.network/api-reference/onchain-runtime/functions/hashToCurve.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [leafHash](https://docs.midnight.network/api-reference/onchain-runtime/functions/leafHash.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [maxAlignedSize](https://docs.midnight.network/api-reference/onchain-runtime/functions/maxAlignedSize.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [maxField](https://docs.midnight.network/api-reference/onchain-runtime/functions/maxField.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [persistentCommit](https://docs.midnight.network/api-reference/onchain-runtime/functions/persistentCommit.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [persistentHash](https://docs.midnight.network/api-reference/onchain-runtime/functions/persistentHash.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [proofDataIntoSerializedPreimage](https://docs.midnight.network/api-reference/onchain-runtime/functions/proofDataIntoSerializedPreimage.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [rawTokenType](https://docs.midnight.network/api-reference/onchain-runtime/functions/rawTokenType.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [runProgram](https://docs.midnight.network/api-reference/onchain-runtime/functions/runProgram.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [runtimeCoinCommitment](https://docs.midnight.network/api-reference/onchain-runtime/functions/runtimeCoinCommitment.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [runtimeCoinNullifier](https://docs.midnight.network/api-reference/onchain-runtime/functions/runtimeCoinNullifier.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [sampleContractAddress](https://docs.midnight.network/api-reference/onchain-runtime/functions/sampleContractAddress.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [sampleRawTokenType](https://docs.midnight.network/api-reference/onchain-runtime/functions/sampleRawTokenType.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [sampleSigningKey](https://docs.midnight.network/api-reference/onchain-runtime/functions/sampleSigningKey.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [sampleUserAddress](https://docs.midnight.network/api-reference/onchain-runtime/functions/sampleUserAddress.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [signatureVerifyingKey](https://docs.midnight.network/api-reference/onchain-runtime/functions/signatureVerifyingKey.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [signData](https://docs.midnight.network/api-reference/onchain-runtime/functions/signData.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [signingKeyFromBip340](https://docs.midnight.network/api-reference/onchain-runtime/functions/signingKeyFromBip340.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [transientCommit](https://docs.midnight.network/api-reference/onchain-runtime/functions/transientCommit.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [transientHash](https://docs.midnight.network/api-reference/onchain-runtime/functions/transientHash.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [upgradeFromTransient](https://docs.midnight.network/api-reference/onchain-runtime/functions/upgradeFromTransient.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [valueToBigInt](https://docs.midnight.network/api-reference/onchain-runtime/functions/valueToBigInt.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [verifySignature](https://docs.midnight.network/api-reference/onchain-runtime/functions/verifySignature.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [globals](https://docs.midnight.network/api-reference/onchain-runtime/globals.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [AlignedValue](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/AlignedValue.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [Alignment](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/Alignment.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [AlignmentAtom](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/AlignmentAtom.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [AlignmentSegment](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/AlignmentSegment.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [BlockContext](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/BlockContext.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [CallContext](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/CallContext.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [CoinCommitment](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/CoinCommitment.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [CoinPublicKey](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/CoinPublicKey.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [CommunicationCommitment](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/CommunicationCommitment.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [CommunicationCommitmentRand](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/CommunicationCommitmentRand.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [ContractAddress](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/ContractAddress.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [DomainSeparator](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/DomainSeparator.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [DustTokenType](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/DustTokenType.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [Effects](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/Effects.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [EncodedStateValue](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/EncodedStateValue.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [FeePrices](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/FeePrices.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [Fr](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/Fr.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [GatherResult](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/GatherResult.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [Key](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/Key.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [Nonce](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/Nonce.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [Nullifier](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/Nullifier.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [Op](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/Op.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [PublicAddress](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/PublicAddress.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [QualifiedShieldedCoinInfo](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/QualifiedShieldedCoinInfo.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [RawTokenType](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/RawTokenType.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [RunningCost](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/RunningCost.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [ShieldedCoinInfo](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/ShieldedCoinInfo.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [ShieldedTokenType](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/ShieldedTokenType.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [Signature](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/Signature.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [SignatureVerifyingKey](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/SignatureVerifyingKey.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [SigningKey](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/SigningKey.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [TokenType](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/TokenType.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [Transcript](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/Transcript.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [UnshieldedTokenType](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/UnshieldedTokenType.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [UserAddress](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/UserAddress.md): @midnight-ntwrk/onchain-runtime v3.1.2
- [Value](https://docs.midnight.network/api-reference/onchain-runtime/type-aliases/Value.md): @midnight-ntwrk/onchain-runtime v3.1.2

### overview

- [Authentication](https://docs.midnight.network/api-reference/overview/authentication.md): Learn about the authentication methods for deploying and interacting with smart contracts in Midnight.
- [Usage limits](https://docs.midnight.network/api-reference/overview/usage-limits.md): Learn about the usage limits for transactions and other API calls.

### testkit-js

@midnight-ntwrk/testkit-js v4.1.0

- [Testkit JS API](https://docs.midnight.network/api-reference/testkit-js.md): @midnight-ntwrk/testkit-js v4.1.0
- [DAppConnectorInitialAPI](https://docs.midnight.network/api-reference/testkit-js/classes/DAppConnectorInitialAPI.md): @midnight-ntwrk/testkit-js v4.1.0
- [DAppConnectorWalletAdapter](https://docs.midnight.network/api-reference/testkit-js/classes/DAppConnectorWalletAdapter.md): @midnight-ntwrk/testkit-js v4.1.0
- [DynamicProofServerContainer](https://docs.midnight.network/api-reference/testkit-js/classes/DynamicProofServerContainer.md): @midnight-ntwrk/testkit-js v4.1.0
- [EnvVarRemoteTestEnvironment](https://docs.midnight.network/api-reference/testkit-js/classes/EnvVarRemoteTestEnvironment.md): @midnight-ntwrk/testkit-js v4.1.0
- [FaucetClient](https://docs.midnight.network/api-reference/testkit-js/classes/FaucetClient.md): @midnight-ntwrk/testkit-js v4.1.0
- [FluentWalletBuilder](https://docs.midnight.network/api-reference/testkit-js/classes/FluentWalletBuilder.md): @midnight-ntwrk/testkit-js v4.1.0
- [GzipFile](https://docs.midnight.network/api-reference/testkit-js/classes/GzipFile.md): @midnight-ntwrk/testkit-js v4.1.0
- [IndexerClient](https://docs.midnight.network/api-reference/testkit-js/classes/IndexerClient.md): @midnight-ntwrk/testkit-js v4.1.0
- [LocalTestConfiguration](https://docs.midnight.network/api-reference/testkit-js/classes/LocalTestConfiguration.md): @midnight-ntwrk/testkit-js v4.1.0
- [LocalTestEnvironment](https://docs.midnight.network/api-reference/testkit-js/classes/LocalTestEnvironment.md): @midnight-ntwrk/testkit-js v4.1.0
- [MidnightWalletProvider](https://docs.midnight.network/api-reference/testkit-js/classes/MidnightWalletProvider.md): @midnight-ntwrk/testkit-js v4.1.0
- [NodeClient](https://docs.midnight.network/api-reference/testkit-js/classes/NodeClient.md): @midnight-ntwrk/testkit-js v4.1.0
- [PreprodTestEnvironment](https://docs.midnight.network/api-reference/testkit-js/classes/PreprodTestEnvironment.md): @midnight-ntwrk/testkit-js v4.1.0
- [PreviewTestEnvironment](https://docs.midnight.network/api-reference/testkit-js/classes/PreviewTestEnvironment.md): @midnight-ntwrk/testkit-js v4.1.0
- [ProofServerClient](https://docs.midnight.network/api-reference/testkit-js/classes/ProofServerClient.md): @midnight-ntwrk/testkit-js v4.1.0
- [QanetTestEnvironment](https://docs.midnight.network/api-reference/testkit-js/classes/QanetTestEnvironment.md): @midnight-ntwrk/testkit-js v4.1.0
- [RemoteTestEnvironment](https://docs.midnight.network/api-reference/testkit-js/classes/RemoteTestEnvironment.md): @midnight-ntwrk/testkit-js v4.1.0
- [StaticProofServerContainer](https://docs.midnight.network/api-reference/testkit-js/classes/StaticProofServerContainer.md): @midnight-ntwrk/testkit-js v4.1.0
- [TestEnvironment](https://docs.midnight.network/api-reference/testkit-js/classes/TestEnvironment.md): @midnight-ntwrk/testkit-js v4.1.0
- [WalletFactory](https://docs.midnight.network/api-reference/testkit-js/classes/WalletFactory.md): @midnight-ntwrk/testkit-js v4.1.0
- [WalletSaveStateProvider](https://docs.midnight.network/api-reference/testkit-js/classes/WalletSaveStateProvider.md): @midnight-ntwrk/testkit-js v4.1.0
- [WalletSeeds](https://docs.midnight.network/api-reference/testkit-js/classes/WalletSeeds.md): @midnight-ntwrk/testkit-js v4.1.0
- [buildUrlWithPath](https://docs.midnight.network/api-reference/testkit-js/functions/buildUrlWithPath.md): @midnight-ntwrk/testkit-js v4.1.0
- [createDefaultTestLogger](https://docs.midnight.network/api-reference/testkit-js/functions/createDefaultTestLogger.md): @midnight-ntwrk/testkit-js v4.1.0
- [createLogger](https://docs.midnight.network/api-reference/testkit-js/functions/createLogger.md): @midnight-ntwrk/testkit-js v4.1.0
- [delay](https://docs.midnight.network/api-reference/testkit-js/functions/delay.md): @midnight-ntwrk/testkit-js v4.1.0
- [expectFoundAndDeployedStatesEqual](https://docs.midnight.network/api-reference/testkit-js/functions/expectFoundAndDeployedStatesEqual.md): @midnight-ntwrk/testkit-js v4.1.0
- [expectFoundAndDeployedTxDataEqual](https://docs.midnight.network/api-reference/testkit-js/functions/expectFoundAndDeployedTxDataEqual.md): @midnight-ntwrk/testkit-js v4.1.0
- [expectFoundAndDeployedTxPrivateDataEqual](https://docs.midnight.network/api-reference/testkit-js/functions/expectFoundAndDeployedTxPrivateDataEqual.md): @midnight-ntwrk/testkit-js v4.1.0
- [expectFoundAndDeployedTxPublicDataEqual](https://docs.midnight.network/api-reference/testkit-js/functions/expectFoundAndDeployedTxPublicDataEqual.md): @midnight-ntwrk/testkit-js v4.1.0
- [expectSuccessfulCallTx](https://docs.midnight.network/api-reference/testkit-js/functions/expectSuccessfulCallTx.md): @midnight-ntwrk/testkit-js v4.1.0
- [expectSuccessfulDeployTx](https://docs.midnight.network/api-reference/testkit-js/functions/expectSuccessfulDeployTx.md): @midnight-ntwrk/testkit-js v4.1.0
- [expectSuccessfulTxData](https://docs.midnight.network/api-reference/testkit-js/functions/expectSuccessfulTxData.md): @midnight-ntwrk/testkit-js v4.1.0
- [getContainersConfiguration](https://docs.midnight.network/api-reference/testkit-js/functions/getContainersConfiguration.md): @midnight-ntwrk/testkit-js v4.1.0
- [getDustSeed](https://docs.midnight.network/api-reference/testkit-js/functions/getDustSeed.md): @midnight-ntwrk/testkit-js v4.1.0
- [getInitialShieldedState](https://docs.midnight.network/api-reference/testkit-js/functions/getInitialShieldedState.md): @midnight-ntwrk/testkit-js v4.1.0
- [getInitialState](https://docs.midnight.network/api-reference/testkit-js/functions/getInitialState.md): @midnight-ntwrk/testkit-js v4.1.0
- [getInitialUnshieldedState](https://docs.midnight.network/api-reference/testkit-js/functions/getInitialUnshieldedState.md): @midnight-ntwrk/testkit-js v4.1.0
- [getShieldedSeed](https://docs.midnight.network/api-reference/testkit-js/functions/getShieldedSeed.md): @midnight-ntwrk/testkit-js v4.1.0
- [getTestEnvironment](https://docs.midnight.network/api-reference/testkit-js/functions/getTestEnvironment.md): @midnight-ntwrk/testkit-js v4.1.0
- [getUnshieldedSeed](https://docs.midnight.network/api-reference/testkit-js/functions/getUnshieldedSeed.md): @midnight-ntwrk/testkit-js v4.1.0
- [getWalletStateFilename](https://docs.midnight.network/api-reference/testkit-js/functions/getWalletStateFilename.md): @midnight-ntwrk/testkit-js v4.1.0
- [initializeMidnightProviders](https://docs.midnight.network/api-reference/testkit-js/functions/initializeMidnightProviders.md): @midnight-ntwrk/testkit-js v4.1.0
- [inMemoryPrivateStateProvider](https://docs.midnight.network/api-reference/testkit-js/functions/inMemoryPrivateStateProvider.md): @midnight-ntwrk/testkit-js v4.1.0
- [setContainersConfiguration](https://docs.midnight.network/api-reference/testkit-js/functions/setContainersConfiguration.md): @midnight-ntwrk/testkit-js v4.1.0
- [stateValueEqual](https://docs.midnight.network/api-reference/testkit-js/functions/stateValueEqual.md): @midnight-ntwrk/testkit-js v4.1.0
- [syncWallet](https://docs.midnight.network/api-reference/testkit-js/functions/syncWallet.md): @midnight-ntwrk/testkit-js v4.1.0
- [txsEqual](https://docs.midnight.network/api-reference/testkit-js/functions/txsEqual.md): @midnight-ntwrk/testkit-js v4.1.0
- [waitForFunds](https://docs.midnight.network/api-reference/testkit-js/functions/waitForFunds.md): @midnight-ntwrk/testkit-js v4.1.0
- [globals](https://docs.midnight.network/api-reference/testkit-js/globals.md): @midnight-ntwrk/testkit-js v4.1.0
- [ContainerEndpoints](https://docs.midnight.network/api-reference/testkit-js/interfaces/ContainerEndpoints.md): @midnight-ntwrk/testkit-js v4.1.0
- [ContainersConfiguration](https://docs.midnight.network/api-reference/testkit-js/interfaces/ContainersConfiguration.md): @midnight-ntwrk/testkit-js v4.1.0
- [ContractConfiguration](https://docs.midnight.network/api-reference/testkit-js/interfaces/ContractConfiguration.md): @midnight-ntwrk/testkit-js v4.1.0
- [DustWalletOptions](https://docs.midnight.network/api-reference/testkit-js/interfaces/DustWalletOptions.md): @midnight-ntwrk/testkit-js v4.1.0
- [EnvironmentConfiguration](https://docs.midnight.network/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md): @midnight-ntwrk/testkit-js v4.1.0
- [LogConfiguration](https://docs.midnight.network/api-reference/testkit-js/interfaces/LogConfiguration.md): @midnight-ntwrk/testkit-js v4.1.0
- [ProofServerContainer](https://docs.midnight.network/api-reference/testkit-js/interfaces/ProofServerContainer.md): @midnight-ntwrk/testkit-js v4.1.0
- [ProofServerContainerConfiguration](https://docs.midnight.network/api-reference/testkit-js/interfaces/ProofServerContainerConfiguration.md): @midnight-ntwrk/testkit-js v4.1.0
- [StandaloneContainerNames](https://docs.midnight.network/api-reference/testkit-js/interfaces/StandaloneContainerNames.md): @midnight-ntwrk/testkit-js v4.1.0
- [StandaloneContainersConfiguration](https://docs.midnight.network/api-reference/testkit-js/interfaces/StandaloneContainersConfiguration.md): @midnight-ntwrk/testkit-js v4.1.0
- [ComponentPortsConfiguration](https://docs.midnight.network/api-reference/testkit-js/type-aliases/ComponentPortsConfiguration.md): @midnight-ntwrk/testkit-js v4.1.0
- [DEFAULT_DUST_OPTIONS](https://docs.midnight.network/api-reference/testkit-js/variables/DEFAULT_DUST_OPTIONS.md): @midnight-ntwrk/testkit-js v4.1.0
- [DEFAULT_WALLET_STATE_DIRECTORY](https://docs.midnight.network/api-reference/testkit-js/variables/DEFAULT_WALLET_STATE_DIRECTORY.md): @midnight-ntwrk/testkit-js v4.1.0
- [defaultContainersConfiguration](https://docs.midnight.network/api-reference/testkit-js/variables/defaultContainersConfiguration.md): @midnight-ntwrk/testkit-js v4.1.0
- [latestContainersConfiguration](https://docs.midnight.network/api-reference/testkit-js/variables/latestContainersConfiguration.md): @midnight-ntwrk/testkit-js v4.1.0
- [logger](https://docs.midnight.network/api-reference/testkit-js/variables/logger.md): @midnight-ntwrk/testkit-js v4.1.0
- [MINUTE](https://docs.midnight.network/api-reference/testkit-js/variables/MINUTE.md): @midnight-ntwrk/testkit-js v4.1.0
- [TEST_MNEMONIC](https://docs.midnight.network/api-reference/testkit-js/variables/TEST_MNEMONIC.md): @midnight-ntwrk/testkit-js v4.1.0

### wallet-sdk

API reference for the Midnight Wallet SDK covering the facade, HD key derivation, address encoding, transfers, DUST management, and atomic swaps.

- [Wallet SDK API Reference](https://docs.midnight.network/api-reference/wallet-sdk.md): API reference for the Midnight Wallet SDK covering the facade, HD key derivation, address encoding, transfers, DUST management, and atomic swaps.
