# MidnightProviders

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / MidnightProviders

# Interface: MidnightProviders\<PCK, PSI, PS>

Set of providers needed for transaction construction and submission.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* [`AnyProvableCircuitId`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/AnyProvableCircuitId.md) = [`AnyProvableCircuitId`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/AnyProvableCircuitId.md)

A union of string literal types representing the callable circuits.

### PSI[​](#psi "Direct link to PSI")

`PSI` *extends* [`PrivateStateId`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/PrivateStateId.md) = [`PrivateStateId`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/PrivateStateId.md)

Parameter indicating the private state ID, sometimes a union of string literals.

### PS[​](#ps "Direct link to PS")

`PS` = `any`

Parameter indicating the private state type stored, sometimes a union of private state types.

## Properties[​](#properties "Direct link to Properties")

### loggerProvider?[​](#loggerprovider "Direct link to loggerProvider?")

> `readonly` `optional` **loggerProvider?**: [`LoggerProvider`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/LoggerProvider.md)

An optional logger that provides utilities for logging at given levels.

***

### midnightProvider[​](#midnightprovider "Direct link to midnightProvider")

> `readonly` **midnightProvider**: [`MidnightProvider`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/MidnightProvider.md)

Submits proven, balanced transactions to the network.

***

### privateStateProvider[​](#privatestateprovider "Direct link to privateStateProvider")

> `readonly` **privateStateProvider**: [`PrivateStateProvider`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/PrivateStateProvider.md)<`PSI`, `PS`>

Manages the private state of a contract.

***

### proofProvider[​](#proofprovider "Direct link to proofProvider")

> `readonly` **proofProvider**: [`ProofProvider`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ProofProvider.md)

Creates proven, unbalanced transactions.

***

### publicDataProvider[​](#publicdataprovider "Direct link to publicDataProvider")

> `readonly` **publicDataProvider**: [`PublicDataProvider`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/PublicDataProvider.md)

Retrieves public data from the blockchain.

***

### walletProvider[​](#walletprovider "Direct link to walletProvider")

> `readonly` **walletProvider**: [`WalletProvider`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/WalletProvider.md)

Creates proven, balanced transactions.

***

### zkConfigProvider[​](#zkconfigprovider "Direct link to zkConfigProvider")

> `readonly` **zkConfigProvider**: [`ZKConfigProvider`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/ZKConfigProvider.md)<`PCK`>

Retrieves the ZK artifacts of a contract needed to create proofs.
