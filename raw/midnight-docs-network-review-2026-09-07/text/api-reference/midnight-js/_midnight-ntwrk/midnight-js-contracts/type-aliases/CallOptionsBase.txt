# CallOptionsBase

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / CallOptionsBase

# Type Alias: CallOptionsBase\<C, PCK>

> **CallOptionsBase**<`C`, `PCK`> = `object`

Describes the target of a circuit invocation.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `Contract.ProvableCircuitId`<`C`>

## Properties[​](#properties "Direct link to Properties")

### additionalCoinEncPublicKeyMappings?[​](#additionalcoinencpublickeymappings "Direct link to additionalCoinEncPublicKeyMappings?")

> `readonly` `optional` **additionalCoinEncPublicKeyMappings?**: `ReadonlyMap`<`CoinPublicKey`, `EncPublicKey`>

An optional mapping of CoinPublicKey to EncPublicKey that can be used to resolve encryption keys for coins created during circuit execution.

***

### circuitId[​](#circuitid "Direct link to circuitId")

> `readonly` **circuitId**: `PCK`

The identifier of the circuit to call.

***

### compiledContract[​](#compiledcontract "Direct link to compiledContract")

> `readonly` **compiledContract**: `CompiledContract.CompiledContract`<`C`, `any`>

The contract defining the circuit to call.

***

### contractAddress[​](#contractaddress "Direct link to contractAddress")

> `readonly` **contractAddress**: [`ContractAddress`](#)

The address of the contract being executed.
