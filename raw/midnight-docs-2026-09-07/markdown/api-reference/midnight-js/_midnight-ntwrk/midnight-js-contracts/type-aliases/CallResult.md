# CallResult

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / CallResult

# Type Alias: CallResult\<C, PCK>

> **CallResult**<`C`, `PCK`> = `object`

Contains all information resulting from circuit execution.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `Contract.ProvableCircuitId`<`C`>

## Properties[​](#properties "Direct link to Properties")

### private[​](#private "Direct link to private")

> `readonly` **private**: [`CallResultPrivate`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallResultPrivate.md)<`C`, `PCK`>

The private/sensitive data produced by the circuit execution.

***

### public[​](#public "Direct link to public")

> `readonly` **public**: [`CallResultPublic`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallResultPublic.md)

The public/non-sensitive data produced by the circuit execution.
