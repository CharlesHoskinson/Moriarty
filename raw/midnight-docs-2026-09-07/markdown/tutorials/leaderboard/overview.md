> For the complete documentation index, see [llms.txt](/llms.txt)

# Build a privacy-preserving leaderboard DApp

This tutorial walks you through building a full-stack DApp on the Midnight Network. You write a Compact smart contract, integrate it with TypeScript, build a browser-based frontend with Lace wallet support, and deploy the finished application to production.

By the end, you have a working arcade-style leaderboard where players submit scores with privacy controls and prove ownership of entries using zero-knowledge proofs.

## What you learn[​](#what-you-learn "Direct link to What you learn")

This tutorial covers four areas of Midnight DApp development.

### Smart contract (Compact)[​](#smart-contract-compact "Direct link to Smart contract (Compact)")

* Define structured on-chain data with `Map`, `Counter`, and custom structs.
* Control what data goes on-chain with `disclose()`.
* Use witnesses to feed private data into ZK circuits on demand.
* Use `assert` to enforce ownership checks.
* Hash with `persistentHash` for anonymous identity.

### TypeScript integration[​](#typescript-integration "Direct link to TypeScript integration")

* Compile Compact contracts and work with the generated TypeScript bindings.
* Build a platform-agnostic API layer with `midnight-js-contracts`.
* Create witness providers that bridge private data into the circuit.

### Browser DApp[​](#browser-dapp "Direct link to Browser DApp")

* Connect to the Lace wallet via the DApp Connector API.
* Initialize providers for proof generation, indexer access, wallet operations, and ZK config.
* Read on-chain state directly from the indexer without a wallet.
* Submit transactions through the browser with ZK proof generation.

### Production deployment[​](#production-deployment "Direct link to Production deployment")

* Deploy the frontend to Vercel.
* Configure environment variables for different networks.

## Prerequisites[​](#prerequisites "Direct link to Prerequisites")

You need the following tools installed before starting.

* [Node.js v22+](https://nodejs.org/) (use `nvm install 22` if needed).
* [Compact toolchain](https://docs.midnight.network/getting-started/installation#install-compact).
* [Docker](https://docs.docker.com/desktop/) (for the local proof server).
* [Lace wallet](https://chromewebstore.google.com/detail/lace/gafhhkghbfjjkeiendhlofajokpaflmk) browser extension.

### Wallet setup[​](#wallet-setup "Direct link to Wallet setup")

1. Install the Lace extension and create a new wallet.
2. Set the network to **Preprod**.
3. Set the proof server to `http://localhost:6300`.
4. Fund your wallet with tNIGHT from the [Preprod faucet](https://midnight-tmnight-preprod.nethermind.dev/).
5. Go to Tokens, click **Generate tDUST**, and confirm the transaction.

> **Note:** tDUST is required to pay transaction fees on Preprod. It is generated from your tNIGHT balance.

## Project structure[​](#project-structure "Direct link to Project structure")

The finished project has three workspace packages and a Docker configuration for the proof server.

```
midnight-leaderboard/

├── contract/              # Compact smart contract + TypeScript bindings

│   ├── leaderboard.compact

│   ├── src/

│   │   ├── index.ts       # Compiled contract exports + witness wiring

│   │   └── witnesses.ts   # Witness provider

│   └── managed/           # Compiler output

├── api/                   # Shared business logic (platform-agnostic)

│   └── src/

│       ├── index.ts       # LeaderboardAPI class

│       ├── common-types.ts

│       └── utils/index.ts

├── leaderboard-ui/        # React + Vite browser DApp

│   └── src/

│       ├── App.tsx         # Game UI + leaderboard + wallet connection

│       ├── main.tsx        # Entry point with Buffer polyfill

│       ├── contexts/       # Lace wallet provider bridge

│       └── hooks/          # Indexer read hook

├── proof-server/           # Docker config for the local proof server

│   └── Dockerfile

├── vercel.json             # Frontend deployment config

└── package.json            # Workspace root
```

## Tutorial sections[​](#tutorial-sections "Direct link to Tutorial sections")

Each part builds on the previous one. Complete them in order.

| Section                                                               | What you build                                                            |
| --------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| [Part 1: Smart contract](/tutorials/leaderboard/smart-contract.md)    | Write the Compact contract with privacy modes and ownership verification. |
| [Part 2: TypeScript integration](/tutorials/leaderboard/api-layer.md) | Set up the contract package, witnesses, and shared API layer.             |
| [Part 3: Browser DApp](/tutorials/leaderboard/browser-dapp.md)        | Build the React frontend with Lace wallet and indexer reads.              |
| [Part 4: Production deployment](/tutorials/leaderboard/deployment.md) | Deploy the frontend to Vercel.                                            |
