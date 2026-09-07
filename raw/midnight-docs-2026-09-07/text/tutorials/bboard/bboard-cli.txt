> For the complete documentation index, see [llms.txt](/llms.txt)

# Bulletin board CLI

This tutorial explains how to build a command-line interface that interacts with the bulletin board smart contract created in the [bulletin board contract](/tutorials/bboard/smart-contract.md) tutorial.

## Prerequisites[​](#prerequisites "Direct link to Prerequisites")

Before you begin, ensure that you have:

* Completed the [bulletin board contract](/tutorials/bboard/smart-contract.md) tutorial with the contract compiled in `contract/src/managed/bboard/`
* Docker Desktop installed and running
* Node.js version 24+

## Architecture overview[​](#architecture-overview "Direct link to Architecture overview")

The bulletin board CLI uses a monorepo structure with three packages:

* **contract**: The Compact smart contract (from the previous tutorial)
* **api**: A reusable abstraction layer for contract interactions
* **bboard-cli**: The command-line application

The API package provides a high-level interface (`BBoardAPI`) that handles contract deployment, state management, and transaction submission while exposing a reactive state observable for real-time updates. The CLI package implements wallet management, user interaction, and the main application logic.

## Set up the root package[​](#set-up-the-root-package "Direct link to Set up the root package")

From the `example-bboard` root directory, create or update `package.json`:

```
{

  "name": "@midnight-ntwrk/example-bboard",

  "version": "0.1.0",

  "author": "IOG",

  "license": "MIT",

  "private": true,

  "type": "module",

  "engines": {

    "node": ">=24.11.1"

  },

  "workspaces": {

    "packages": [

      "bboard-cli",

      "api",

      "contract"

    ]

  },

  "dependencies": {

    "@midnight-ntwrk/dapp-connector-api": "4.0.1",

    "@midnight-ntwrk/midnight-js-contracts": "4.1.1",

    "@midnight-ntwrk/midnight-js-fetch-zk-config-provider": "4.1.1",

    "@midnight-ntwrk/midnight-js-http-client-proof-provider": "4.1.1",

    "@midnight-ntwrk/midnight-js-indexer-public-data-provider": "4.1.1",

    "@midnight-ntwrk/midnight-js-level-private-state-provider": "4.1.1",

    "@midnight-ntwrk/midnight-js-network-id": "4.1.1",

    "@midnight-ntwrk/midnight-js-node-zk-config-provider": "4.1.1",

    "@midnight-ntwrk/midnight-js-protocol": "4.1.1",

    "@midnight-ntwrk/midnight-js-types": "4.1.1",

    "@midnight-ntwrk/midnight-js-utils": "4.1.1",

    "@midnight-ntwrk/testkit-js": "4.1.1",

    "@midnight-ntwrk/wallet-sdk": "1.0.0",

    "axios": "^1.17.0",

    "buffer": "^6.0.3",

    "fp-ts": "^2.16.11",

    "pino": "^10.3.1",

    "pino-pretty": "^13.1.3",

    "rxjs": "^7.8.2",

    "semver": "^7.8.4",

    "testcontainers": "^12.0.1",

    "ws": "^8.21.0"

  },

  "devDependencies": {

    "@eslint/js": "^9.39.2",

    "@originjs/vite-plugin-commonjs": "^1.0.3",

    "@types/babel__core": "^7.20.5",

    "@types/semver": "^7.7.1",

    "@types/ws": "^8.18.1",

    "@typescript-eslint/eslint-plugin": "^8.61.0",

    "@typescript-eslint/parser": "^8.61.0",

    "eslint": "^9.39.2",

    "eslint-config-prettier": "^10.1.8",

    "eslint-plugin-prettier": "^5.5.6",

    "eslint-plugin-react": "^7.37.5",

    "http-server": "^14.1.1",

    "prettier": "^3.8.4",

    "ts-node": "^10.9.2",

    "typescript": "^5.9.3",

    "typescript-eslint": "^8.61.0",

    "vite": "^8.0.14",

    "vite-plugin-top-level-await": "^1.6.0",

    "vite-plugin-wasm": "^3.6.0"

  }

}
```

The workspaces configuration tells npm to manage the contract, API, and bboard-cli as linked packages. Dependencies defined at the root level are shared across all workspaces, reducing duplication and ensuring version consistency.

Wallet SDK consolidation

Midnight.js 4.1.x consumes the wallet stack through the bundled `@midnight-ntwrk/wallet-sdk` package and the new `@midnight-ntwrk/midnight-js-protocol` umbrella, which provides subpath imports such as `@midnight-ntwrk/midnight-js-protocol/ledger` and `@midnight-ntwrk/midnight-js-protocol/compact-runtime`. Earlier examples used the standalone `@midnight-ntwrk/wallet-sdk-*` and `@midnight-ntwrk/ledger-v8` packages directly.

Version compatibility

Always refer to the [release compatibility matrix](/relnotes/support-matrix.md) to ensure you are using compatible versions.

Install all dependencies from the root:

```
npm install
```

This command installs dependencies for the root package and all workspace packages, creating symlinks between them for local development.

## Implementation guides[​](#implementation-guides "Direct link to Implementation guides")

To build the bulletin board CLI, follow these implementation guides in order:

1. **[API implementation](/tutorials/bboard/bboard-api-implementation.md)**: Set up the API package with the `BBoardAPI` class that handles contract deployment, state management, and transaction submission.

2. **[CLI implementation](/tutorials/bboard/bboard-cli-implementation.md)**: Set up the CLI package with wallet management, DUST generation, and the interactive command-line interface.

## Project structure[​](#project-structure "Direct link to Project structure")

The complete example-bboard project uses a monorepo structure with npm workspaces:

```
example-bboard/

├── package.json                 # Root package with workspaces

├── contract/                    # Compact contract

│   ├── src/

│   │   ├── bboard.compact

│   │   ├── managed/

│   │   ├── witnesses.ts

│   │   └── index.ts

│   └── package.json

├── api/                         # Shared API layer

│   ├── src/

│   │   ├── index.ts            # BBoardAPI implementation

│   │   ├── common-types.ts     # Type definitions

│   │   └── utils/

│   │       └── index.ts        # Utility functions

│   └── package.json

└── bboard-cli/                  # Bulletin board CLI

    ├── src/

    │   ├── config.ts           # Network configuration

    │   ├── index.ts            # Main CLI logic

    │   ├── logger-utils.ts     # Logging setup

    │   ├── wallet-utils.ts     # Wallet synchronization

    │   ├── generate-dust.ts    # DUST generation

    │   ├── midnight-wallet-provider.ts  # Wallet provider

    │   └── launcher/

    │       ├── preprod.ts      # Preprod entry point

    │       ├── preview.ts      # Preview entry point

    │       └── standalone.ts   # Standalone entry point

    ├── proof-server-local.yml  # Docker compose for local proof server

    ├── proof-server.yml        # Docker compose for testcontainers

    └── package.json
```

The monorepo structure enables code sharing between packages through workspace references.

## Run the CLI[​](#run-the-cli "Direct link to Run the CLI")

After completing both implementation guides, start a local proof server and then run the CLI from the `bboard-cli` directory:

```
docker compose -f proof-server-local.yml up -d
```

* preprod
* preview
* standalone

```
npm run preprod-remote
```

```
npm run preview-remote
```

```
npm run standalone
```

The `preprod-remote` and `preview-remote` commands start the CLI and connect to the corresponding Midnight remote testnet, while the `standalone` command runs against a local Midnight network using the genesis-mint wallet seed.

You should see the following output:

```
You can do one of the following:

  1. Build a fresh wallet

  2. Build wallet from a seed

  3. Exit

Which would you like to do?
```

From here, you can choose to build a fresh wallet or build a wallet from a seed. You can also exit the CLI by entering `3`. For more information on running the CLI, see the [Bulletin board DApp](/examples/dapps/bboard.md) example.

## Next steps[​](#next-steps "Direct link to Next steps")

Now that you have built a complete bulletin board CLI with privacy-preserving message posting:

* **Build the UI**: Create a browser-based interface for the bulletin board using the [DApp connector API](/api-reference/dapp-connector.md).
* **Extend functionality**: Add features like message history, multiple boards, or time-limited posts.
