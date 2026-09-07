# httpClientProofProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-http-client-proof-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-http-client-proof-provider.md) / httpClientProofProvider

# Function: httpClientProofProvider()

> **httpClientProofProvider**<`K`>(`url`, `zkConfigProvider`, `config?`): [`ProofProvider`](#)

Creates a high-level [ProofProvider](#) that implements transaction-level proving using the low-level circuit-by-circuit [ProvingProvider](#) as its foundation.

This adapter bridges the gap between:

* High-level ProofProvider interface (works with complete transactions)
* Low-level ProvingProvider interface (works with individual circuits)

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### K[​](#k "Direct link to K")

`K` *extends* `string`

## Parameters[​](#parameters "Direct link to Parameters")

### url[​](#url "Direct link to url")

`string`

The URL of the proof server

### zkConfigProvider[​](#zkconfigprovider "Direct link to zkConfigProvider")

[`ZKConfigProvider`](#)<`K`>

Provider for zero-knowledge configuration artifacts

### config?[​](#config "Direct link to config?")

[`ProvingProviderConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-http-client-proof-provider/interfaces/ProvingProviderConfig.md)

Optional configuration for the underlying ProvingProvider

## Returns[​](#returns "Direct link to Returns")

[`ProofProvider`](#)

A ProofProvider instance that uses ProvingProvider internally

## Remarks[​](#remarks "Direct link to Remarks")

**Architecture:**

```
ProofProvider (Transaction-level)

    ↓ (adapter)

ProvingProvider (Circuit-level)

    ↓ (HTTP client)

Proof Server (/check, /prove endpoints)
```

**Note:** The /prove-tx endpoint is NOT used. All proving is done through individual circuit operations using /check and /prove endpoints.
