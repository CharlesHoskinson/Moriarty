# CallOptionsProviderDataDependencies

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / CallOptionsProviderDataDependencies

# Type Alias: CallOptionsProviderDataDependencies

> **CallOptionsProviderDataDependencies** = `object`

Data retrieved via providers that should be included in the call options.

## Properties[​](#properties "Direct link to Properties")

### coinPublicKey[​](#coinpublickey "Direct link to coinPublicKey")

> `readonly` **coinPublicKey**: `CoinPublicKey`

The Zswap public key of the current user.

***

### initialContractState[​](#initialcontractstate "Direct link to initialContractState")

> `readonly` **initialContractState**: `ContractState`

The initial public state of the contract to run the circuit against.

***

### initialZswapChainState[​](#initialzswapchainstate "Direct link to initialZswapChainState")

> `readonly` **initialZswapChainState**: `ZswapChainState`

The initial public Zswap state of the contract to run the circuit against.

***

### ledgerParameters[​](#ledgerparameters "Direct link to ledgerParameters")

> `readonly` **ledgerParameters**: `LedgerParameters`

The ledger parameters to use when executing the circuit.
