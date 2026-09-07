> For the complete documentation index, see [llms.txt](/llms.txt)

# Midnight.js

Midnight.js provides tools for deploying and interacting with smart contracts, managing encrypted private state, generating zero-knowledge proofs, and submitting transactions to the Midnight network.

This guide provides a comprehensive overview of the Midnight.js SDK, including its architecture, packages, and how to get started.

## Prerequisites[​](#prerequisites "Direct link to Prerequisites")

Before using Midnight.js, ensure that you have:

* [Node.js](https://nodejs.org/) version 22.x or higher installed.
* [Docker](https://www.docker.com/) installed and running (required for the [proof server](/guides/run-proof-server.md)).

## Packages[​](#packages "Direct link to Packages")

Midnight.js uses a modular architecture, with each package providing a specific functionality.

### Core[​](#core "Direct link to Core")

The core packages provide the foundational functionality for the SDK.

| Package                                  | Purpose                                                           |
| ---------------------------------------- | ----------------------------------------------------------------- |
| `@midnight-ntwrk/midnight-js-types`      | Shared types, interfaces, and provider contracts                  |
| `@midnight-ntwrk/midnight-js-contracts`  | Contract deployment, circuit calls, and transaction submission    |
| `@midnight-ntwrk/midnight-js-network-id` | Network identifier configuration for runtime and ledger WASM APIs |
| `@midnight-ntwrk/midnight-js-utils`      | Shared utilities (hex encoding, bech32m, assertions)              |

### Providers[​](#providers "Direct link to Providers")

The provider packages includes functionality for proof generation, private state management, and public data queries.

| Package                                                    | Purpose                                                                                                                               |
| ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `@midnight-ntwrk/midnight-js-indexer-public-data-provider` | GraphQL-based blockchain data provider (queries and subscriptions)                                                                    |
| `@midnight-ntwrk/midnight-js-level-private-state-provider` | AES-256-GCM encrypted persistent state storage via [LevelDB](https://github.com/Level/level)                                          |
| `@midnight-ntwrk/midnight-js-http-client-proof-provider`   | HTTP client for the Midnight proof server                                                                                             |
| `@midnight-ntwrk/midnight-js-fetch-zk-config-provider`     | Browser-compatible zero-knowledge artifact provider using the [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) |
| `@midnight-ntwrk/midnight-js-node-zk-config-provider`      | Node.js filesystem-based ZK artifact provider                                                                                         |
| `@midnight-ntwrk/midnight-js-logger-provider`              | Application-specific [Pino](https://github.com/pinojs/pino) logger configuration                                                      |

### Tooling[​](#tooling "Direct link to Tooling")

The tooling packages provide tools for compiling Compact smart contracts.

| Package                               | Purpose                                           |
| ------------------------------------- | ------------------------------------------------- |
| `@midnight-ntwrk/midnight-js-compact` | Compact compiler manager for contract compilation |

## Installation[​](#installation "Direct link to Installation")

To install the barrel package that provides a single entry point for the SDK.

* npm
* yarn

```
npm install @midnight-ntwrk/midnight-js
```

```
yarn add @midnight-ntwrk/midnight-js
```

To install the individual packages:

* npm
* yarn

```
npm install @midnight-ntwrk/midnight-js-types

npm install @midnight-ntwrk/midnight-js-contracts

npm install @midnight-ntwrk/midnight-js-network-id

npm install @midnight-ntwrk/midnight-js-utils

npm install @midnight-ntwrk/midnight-js-indexer-public-data-provider

npm install @midnight-ntwrk/midnight-js-level-private-state-provider

npm install @midnight-ntwrk/midnight-js-http-client-proof-provider

npm install @midnight-ntwrk/midnight-js-fetch-zk-config-provider

npm install @midnight-ntwrk/midnight-js-node-zk-config-provider

npm install @midnight-ntwrk/midnight-js-logger-provider

npm install @midnight-ntwrk/ledger-v8
```

```
yarn add @midnight-ntwrk/midnight-js-types

yarn add @midnight-ntwrk/midnight-js-contracts

yarn add @midnight-ntwrk/midnight-js-network-id

yarn add @midnight-ntwrk/midnight-js-utils

yarn add @midnight-ntwrk/midnight-js-indexer-public-data-provider

yarn add @midnight-ntwrk/midnight-js-level-private-state-provider

yarn add @midnight-ntwrk/midnight-js-http-client-proof-provider

yarn add @midnight-ntwrk/midnight-js-fetch-zk-config-provider

yarn add @midnight-ntwrk/midnight-js-node-zk-config-provider

yarn add @midnight-ntwrk/midnight-js-logger-provider

yarn add @midnight-ntwrk/ledger-v8
```

important

Always refers to the [Compatibility matrix](/relnotes/support-matrix.md) to know which version of the SDK works with other components of the Midnight Network.

## Configure the network[​](#configure-the-network "Direct link to Configure the network")

The network ID sets the network identifier for the SDK. To perform operations on the Midnight Network, you need to set the appropriate network ID. Other Midnight.js libraries and components use this network ID to execute operations on the network.

* preprod
* undeployed

```
import { setNetworkId } from '@midnight-ntwrk/midnight-js-network-id';



setNetworkId('preprod');



export const CONFIG = {

  indexer: 'https://indexer.preprod.midnight.network/api/v4/graphql',

  indexerWS: 'wss://indexer.preprod.midnight.network/api/v4/graphql/ws',

  node: 'https://rpc.preprod.midnight.network',

  proofServer: 'http://127.0.0.1:6300',

};
```

```
import { setNetworkId } from '@midnight-ntwrk/midnight-js-network-id';



setNetworkId('undeployed');



export const CONFIG = {

  indexer: 'http://localhost:8088/api/v4/graphql',

  indexerWS: 'ws://localhost:8088/api/v4/graphql/ws',

  node: 'ws://localhost:9944',

  proofServer: 'http://localhost:6300',

};
```

note

The undeployed network is a local network that is not deployed to the blockchain. It is used for testing and development purposes. For more information, see [Running a local network](/guides/networks-and-environments.md#running-a-local-network).

Available networks include:

| Network    | Network ID   |
| ---------- | ------------ |
| Mainnet    | `mainnet`    |
| Preview    | `preview`    |
| Preprod    | `preprod`    |
| Undeployed | `undeployed` |

For Node.js environments, enable WebSocket for GraphQL subscriptions:

```
import { WebSocket } from 'ws';



globalThis.WebSocket = WebSocket;
```

## Configure providers[​](#configure-providers "Direct link to Configure providers")

Providers are modular, pluggable components that each handle a specific capability required for transaction construction and submission to the Midnight blockchain.

```
import { levelPrivateStateProvider } from '@midnight-ntwrk/midnight-js-level-private-state-provider';

import { indexerPublicDataProvider } from '@midnight-ntwrk/midnight-js-indexer-public-data-provider';

import { httpClientProofProvider } from '@midnight-ntwrk/midnight-js-http-client-proof-provider';

import { FetchZkConfigProvider } from '@midnight-ntwrk/midnight-js-fetch-zk-config-provider';

import * as ledger from '@midnight-ntwrk/ledger-v8';



const zkConfigProvider = new FetchZkConfigProvider(zkArtifactsUrl);



const providers: MidnightProviders = {

  privateStateProvider: levelPrivateStateProvider({

    privateStoragePasswordProvider: () => password,

    accountId: walletAddress,

  }),

  publicDataProvider: indexerPublicDataProvider(queryUrl, subscriptionUrl),

  zkConfigProvider,

  proofProvider: httpClientProofProvider(proofServerUrl, zkConfigProvider),

  walletProvider,    // from @midnight-ntwrk/wallet-sdk-facade

  midnightProvider,  // from @midnight-ntwrk/wallet-sdk-facade

};
```

info

The `walletProvider` and `midnightProvider` handle transaction balancing and submission to the blockchain. The Wallet SDK package provides them. For more information, see the [Wallet SDK](/sdks/official/wallet-developer-guide.md) guide.

You can either use the `FetchZkConfigProvider` or the `NodeZkConfigProvider` to provide the ZK configuration for the proof provider.

### Fetch ZK configuration provider[​](#fetch-zk-configuration-provider "Direct link to Fetch ZK configuration provider")

ZK configuration provider that retrieves proving keys, verifier keys, and ZK intermediate representation over HTTP/HTTPS.

Here is an example using the `FetchZkConfigProvider` method:

```
import { FetchZkConfigProvider } from '@midnight-ntwrk/midnight-js-fetch-zk-config-provider';



const zkConfigProvider = new FetchZkConfigProvider('https://example.com/zk-artifacts');



const proverKey = await zkConfigProvider.getProverKey('myCircuit');

const verifierKey = await zkConfigProvider.getVerifierKey('myCircuit');

const zkir = await zkConfigProvider.getZKIR('myCircuit');
```

### Node ZK configuration provider[​](#node-zk-configuration-provider "Direct link to Node ZK configuration provider")

ZK configuration provider that retrieves proving keys, verifier keys, and ZK intermediate representation from the file system.

Here is an example using the `NodeZkConfigProvider` method:

```
import { NodeZkConfigProvider } from '@midnight-ntwrk/midnight-js-node-zk-config-provider';



const zkConfigProvider = new NodeZkConfigProvider('/path/to/zk-artifacts');



const proverKey = await zkConfigProvider.getProverKey('myCircuit');

const verifierKey = await zkConfigProvider.getVerifierKey('myCircuit');

const zkir = await zkConfigProvider.getZKIR('myCircuit');
```

### DApp Connector proof provider[​](#dapp-connector-proof-provider "Direct link to DApp Connector proof provider")

Proof provider implementation that delegates zero-knowledge proof generation to a DApp Connector wallet. Use this when your DApp runs in a browser and the user's wallet handles proving.

Install the `@midnight-ntwrk/dapp-connector-proof-provider` package:

```
npm install @midnight-ntwrk/dapp-connector-proof-provider
```

Use the `dappConnectorProofProvider` function to create a proof provider:

```
import { dappConnectorProofProvider } from '@midnight-ntwrk/midnight-js-dapp-connector-proof-provider';



const proofProvider = await dappConnectorProofProvider(

  walletConnectedAPI,

  zkConfigProvider,

  costModel

);



const provenTx = await proofProvider.proveTx(unprovenTx);
```

The `dappConnectorProofProvider` function takes the following *required* parameters:

* `walletConnectedAPI`: DApp Connector wallet API with proving capabilities
* `zkConfigProvider`: Provider for ZK configuration artifacts
* `costModel`: Cost model for transaction proving

note

The DApp Connector API handles the wallet connection and provides the `walletConnectedAPI` parameter. For more information, see the [DApp Connector](/api-reference/dapp-connector.md) API reference.

## Deploy a smart contract[​](#deploy-a-smart-contract "Direct link to Deploy a smart contract")

The `deployContract` method deploys a smart contract to the blockchain. It takes the following parameters:

* `providers`: The providers object.
* `compiledContract`: The compiled contract.
* `privateStateId`: The private state ID.
* `initialPrivateState`: The initial private state.

```
import { deployContract } from '@midnight-ntwrk/midnight-js-contracts';



const deployed = await deployContract(providers, {

  compiledContract,

  privateStateId: 'my-state',

  initialPrivateState: { counter: 0n },

});
```

The `deployContact` method returns a `deployedContract` promise. This object contains the contract address and the private state ID.

```
const contractAddress = deployed.deployTxData.public.contractAddress;

console.log(`Contract Address: ${contractAddress}`);
```

## Interact with the contract[​](#interact-with-the-contract "Direct link to Interact with the contract")

To interact with the contract, use the `findDeployedContract` method to get the deployed contract object.

```
import { findDeployedContract } from '@midnight-ntwrk/midnight-js-contracts';



const contract = await findDeployedContract(providers, {

  contractAddress: contractAddress,

  compiledContract,

  privateStateId: 'my-state',

  initialPrivateState: {},

});
```

This method takes the following parameters:

* `providers`: The providers object.

* Options containing the following properties:

  <!-- -->

  * `contractAddress`: The contract address
  * `compiledContract`: The compiled contract
  * `privateStateId`: The private state ID
  * `initialPrivateState`: The initial private state

After initializing the contract object, you can call circuits from your smart contract using the `callTx` property.

```
const transaction = await contract.callTx.someCircuit(circuitArguments);
```

This operation creates a new transaction and submits it to the blockchain. To access the transaction details:

```
const transaction = await contract.callTx.someCircuit(circuitArguments);

console.log('Transaction ID:', transaction.public.txId);

console.log(`Block: ${transaction.public.blockHeight}\n`);
```

## Transaction submission[​](#transaction-submission "Direct link to Transaction submission")

When you already have transaction payloads or options objects prepared, these helpers submit call, deploy, or generic transactions through the configured providers. This is useful for explicit submission flows outside the `contract.callTx.*` convenience methods.

```
import { submitCallTx, submitDeployTx, submitTx } from '@midnight-ntwrk/midnight-js-contracts';



// Submit a call transaction

await submitCallTx(providers, callOptions);



// Submit a deploy transaction

await submitDeployTx(providers, deployOptions);



// Generic transaction submission

await submitTx(providers, txData);
```

## Query state[​](#query-state "Direct link to Query state")

After deploying a smart contract, you can query the private and public states of the contract using the `getStates` and `getPublicStates` methods.

```
import { getStates, getPublicStates } from '@midnight-ntwrk/midnight-js-contracts';



const states = await getStates(providers, contractAddress, privateStateId);

const publicStates = await getPublicStates(providers, contractAddress);
```

The `getStates` method retrieves the Zswap, ledger, and private states of the deployed smart contract corresponding to the given identifier using the given providers.

It accepts the following parameters:

* `providers`: The providers object.
* `contractAddress`: The contract address.
* `privateStateId`: The private state ID.

The `getPublicStates` method fetches only the public visible (Zswap and ledger) states of a deployed smart contract. It accepts the following parameters:

* `providers`: The providers object.
* `contractAddress`: The contract address.

To get the unshielded balance of the contract, use the `getUnshieldedBalances` method passing the providers and contract address as parameters.

```
const balances = await getUnshieldedBalances(providers, contractAddress);

console.log('Unshielded Balances:', balances);
```

## Key concepts[​](#key-concepts "Direct link to Key concepts")

These concepts are important to understand the Midnight.js SDK and how it works.

### Contract model[​](#contract-model "Direct link to Contract model")

The contract model uses the following terminology:

* **Circuit**: Smart contract function that executes locally and generates a zero-knowledge proof.
* **Witness**: Private computation that runs on the end-user's device.
* **Private state**: User-local state updated by circuits — never stored on-chain.
* **Ledger state**: On-chain public contract state.

### ZK artifacts[​](#zk-artifacts "Direct link to ZK artifacts")

ZK artifacts are the files generated by the Compact compiler to create zero-knowledge proofs. It includes the following files:

* **Prover key**: Binary used to create zero-knowledge proofs.
* **Verifier key**: Binary used for on-chain proof verification.
* **ZKIR**: Zero-Knowledge Intermediate Representation of compiled contracts.

### Transaction flow[​](#transaction-flow "Direct link to Transaction flow")

Submitting a contract interaction runs locally first (producing an unproven transaction), then uses the typed providers to prove, balance, and broadcast the transaction on the network.

The diagram below summarizes that sequence.

<!-- -->

### Transaction status[​](#transaction-status "Direct link to Transaction status")

Transactions can have the following statuses:

* `SucceedEntirely`: All transaction segments succeeded.
* `FailFallible`: Guaranteed portion succeeded, fallible portion failed.
* `FailEntirely`: Transaction is invalid.
