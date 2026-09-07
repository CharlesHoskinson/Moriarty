> For the complete documentation index, see [llms.txt](/llms.txt)

# midnight-js

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / @midnight-ntwrk/midnight-js

# Midnight.js

Barrel package that provides a single entry point to the core components of Midnight.js. Import all core modules from one package instead of installing them individually.

## Installation[​](#installation "Direct link to Installation")

```
yarn add @midnight-ntwrk/midnight-js
```

## Quick Start[​](#quick-start "Direct link to Quick Start")

```
import { contracts, networkId, types, utils } from '@midnight-ntwrk/midnight-js';



networkId.setNetworkId('testnet');



const deployed = await contracts.deployContract(providers, {

  compiledContract: myContract,

  privateStateId: 'my-state',

  initialPrivateState: { counter: 0n }

});
```

## Modules[​](#modules "Direct link to Modules")

| Module      | Package                                  | Description                                      |
| ----------- | ---------------------------------------- | ------------------------------------------------ |
| `contracts` | `@midnight-ntwrk/midnight-js-contracts`  | Contract deployment and interaction utilities    |
| `networkId` | `@midnight-ntwrk/midnight-js-network-id` | Network identifier management                    |
| `protocol`  | `@midnight-ntwrk/midnight-js-protocol`   | Version-agnostic protocol type re-exports        |
| `types`     | `@midnight-ntwrk/midnight-js-types`      | Shared types, interfaces, and provider contracts |
| `utils`     | `@midnight-ntwrk/midnight-js-utils`      | Hex encoding, address validation, and utilities  |

## Sub-path Imports[​](#sub-path-imports "Direct link to Sub-path Imports")

Each module is also available as a sub-path import for tree-shaking:

```
import { deployContract, findDeployedContract } from '@midnight-ntwrk/midnight-js/contracts';

import { setNetworkId, getNetworkId } from '@midnight-ntwrk/midnight-js/network-id';

import { type ProofProvider, type WalletProvider } from '@midnight-ntwrk/midnight-js/types';

import { toHex, fromHex } from '@midnight-ntwrk/midnight-js/utils';
```

## Exports[​](#exports "Direct link to Exports")

```
// Namespace imports (all modules)

import { contracts, networkId, types, utils } from '@midnight-ntwrk/midnight-js';



// Sub-path imports (individual modules)

import { ... } from '@midnight-ntwrk/midnight-js/contracts';

import { ... } from '@midnight-ntwrk/midnight-js/network-id';

import { ... } from '@midnight-ntwrk/midnight-js/types';

import { ... } from '@midnight-ntwrk/midnight-js/utils';
```

## Resources[​](#resources "Direct link to Resources")

* [Midnight Network](https://midnight.network)
* [Developer Hub](https://midnight.network/developer-hub)

## Terms & License[​](#terms--license "Direct link to Terms & License")

By using this package, you agree to [Midnight's Terms and Conditions](https://midnight.network/static/terms.pdf) and [Privacy Policy](https://midnight.network/static/privacy-policy.pdf).

Licensed under [Apache License 2.0](http://www.apache.org/licenses/LICENSE-2.0).
