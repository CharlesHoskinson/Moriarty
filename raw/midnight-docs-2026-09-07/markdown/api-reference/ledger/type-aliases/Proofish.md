# Proofish

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / Proofish

# Type Alias: Proofish

```
type Proofish = 

  | Proof

  | PreProof

  | NoProof;
```

How proofs are currently being represented, between:

* Actual zero-knowledge proofs, as should be transmitted to the network
* The data required to *produce* proofs, for constructing and preparing transactions.
* Proofs not being provided, largely for testing use or replaying already validated transactions.
