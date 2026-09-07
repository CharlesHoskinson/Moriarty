# WellFormedStrictness

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / WellFormedStrictness

# Class: WellFormedStrictness

Strictness criteria for evaluating transaction well-formedness, used for disabling parts of transaction validation for testing.

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new WellFormedStrictness(): WellFormedStrictness;
```

#### Returns[​](#returns "Direct link to Returns")

`WellFormedStrictness`

## Properties[​](#properties "Direct link to Properties")

### enforceBalancing[​](#enforcebalancing "Direct link to enforceBalancing")

```
enforceBalancing: boolean;
```

Whether to require the transaction to have a non-negative balance

***

### enforceLimits[​](#enforcelimits "Direct link to enforceLimits")

```
enforceLimits: boolean;
```

Whether to enforce the transaction byte limit

***

### verifyContractProofs[​](#verifycontractproofs "Direct link to verifyContractProofs")

```
verifyContractProofs: boolean;
```

Whether to validate contract proofs in the transaction

***

### verifyNativeProofs[​](#verifynativeproofs "Direct link to verifyNativeProofs")

```
verifyNativeProofs: boolean;
```

Whether to validate Midnight-native (non-contract) proofs in the transaction

***

### verifySignatures[​](#verifysignatures "Direct link to verifySignatures")

```
verifySignatures: boolean;
```

Whether to enforce the signature verification
