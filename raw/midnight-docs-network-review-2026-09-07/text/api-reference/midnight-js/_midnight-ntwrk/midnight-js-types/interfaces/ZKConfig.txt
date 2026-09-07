# ZKConfig

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / ZKConfig

# Interface: ZKConfig\<K>

Contains all information required by the [ProofProvider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ProofProvider.md)

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### K[​](#k "Direct link to K")

`K` *extends* `string`

The type of the circuit ID.

## Properties[​](#properties "Direct link to Properties")

### circuitId[​](#circuitid "Direct link to circuitId")

> `readonly` **circuitId**: `K`

A circuit identifier.

***

### proverKey[​](#proverkey "Direct link to proverKey")

> `readonly` **proverKey**: [`ProverKey`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ProverKey.md)

The prover key corresponding to [ZKConfig.circuitId](#circuitid).

***

### verifierKey[​](#verifierkey "Direct link to verifierKey")

> `readonly` **verifierKey**: [`VerifierKey`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/VerifierKey.md)

The verifier key corresponding to [ZKConfig.circuitId](#circuitid).

***

### zkir[​](#zkir "Direct link to zkir")

> `readonly` **zkir**: [`ZKIR`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ZKIR.md)

The zero-knowledge intermediate representation corresponding to [ZKConfig.circuitId](#circuitid).
