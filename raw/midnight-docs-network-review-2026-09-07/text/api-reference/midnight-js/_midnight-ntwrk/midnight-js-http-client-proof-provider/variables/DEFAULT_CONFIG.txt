# DEFAULT\_CONFIG

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-http-client-proof-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-http-client-proof-provider.md) / DEFAULT\_CONFIG

# Variable: DEFAULT\_CONFIG

> `const` **DEFAULT\_CONFIG**: `object`

HTTP Client Proof Provider

This package provides two levels of abstraction for interacting with a Midnight proof server:

## High-Level: Transaction Proving (ProofProvider)[​](#high-level-transaction-proving-proofprovider "Direct link to High-Level: Transaction Proving (ProofProvider)")

Use `httpClientProofProvider` for most use cases. It handles complete transactions by using the low-level ProvingProvider internally.

```
import { httpClientProofProvider } from '@midnight-ntwrk/midnight-js-http-client-proof-provider';



const proofProvider = httpClientProofProvider(

  'http://localhost:6300',

  zkConfigProvider

);

const provenTx = await proofProvider.proveTx(unprovenTx, { zkConfig });
```

## Low-Level: Circuit Proving (ProvingProvider)[​](#low-level-circuit-proving-provingprovider "Direct link to Low-Level: Circuit Proving (ProvingProvider)")

Use `httpClientProvingProvider` for advanced scenarios where you need fine-grained control over individual circuit proving operations.

```
import { httpClientProvingProvider } from '@midnight-ntwrk/midnight-js-http-client-proof-provider';



const provingProvider = httpClientProvingProvider(

  'http://localhost:6300',

  zkConfigProvider

);

const checkResult = await provingProvider.check(serializedPreimage, circuitId);

const proof = await provingProvider.prove(serializedPreimage, circuitId);
```

## Architecture[​](#architecture "Direct link to Architecture")

```
ProofProvider (httpClientProofProvider)

    ↓ uses

ProvingProvider (httpClientProvingProvider)

    ↓ calls

Proof Server (/check, /prove)
```

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### timeout[​](#timeout "Direct link to timeout")

> **timeout**: `number` = `300000`

### zkConfig[​](#zkconfig "Direct link to zkConfig")

> **zkConfig**: `undefined` = `undefined`
