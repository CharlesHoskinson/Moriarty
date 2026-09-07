# Effects

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / Effects

# Type Alias: Effects

```
type Effects: {

  claimedContractCalls: [bigint, ContractAddress, string, Fr][];

  claimedNullifiers: Nullifier[];

  claimedShieldedReceives: CoinCommitment[];

  claimedShieldedSpends: CoinCommitment[];

  claimedUnshieldedSpends: Map<[TokenType, PublicAddress], bigint>;

  shieldedMints: Map<string, bigint>;

  unshieldedInputs: Map<TokenType, bigint>;

  unshieldedMints: Map<string, bigint>;

  unshieldedOutputs: Map<TokenType, bigint>;

};
```

The contract-external effects of a transcript.

## Type declaration[​](#type-declaration "Direct link to Type declaration")

### claimedContractCalls[​](#claimedcontractcalls "Direct link to claimedContractCalls")

```
claimedContractCalls: [bigint, ContractAddress, string, Fr][];
```

The contracts called from this contract. The values are, in order:

* The sequence number of this call
* The contract being called
* The entry point being called
* The communications commitment

### claimedNullifiers[​](#claimednullifiers "Direct link to claimedNullifiers")

```
claimedNullifiers: Nullifier[];
```

The nullifiers (spends) this contract call requires

### claimedShieldedReceives[​](#claimedshieldedreceives "Direct link to claimedShieldedReceives")

```
claimedShieldedReceives: CoinCommitment[];
```

The coin commitments (outputs) this contract call requires, as coins received

### claimedShieldedSpends[​](#claimedshieldedspends "Direct link to claimedShieldedSpends")

```
claimedShieldedSpends: CoinCommitment[];
```

The coin commitments (outputs) this contract call requires, as coins sent

### claimedUnshieldedSpends[​](#claimedunshieldedspends "Direct link to claimedUnshieldedSpends")

```
claimedUnshieldedSpends: Map<[TokenType, PublicAddress], bigint>;
```

The unshielded UTXO outputs this contract expects to be present.

### shieldedMints[​](#shieldedmints "Direct link to shieldedMints")

```
shieldedMints: Map<string, bigint>;
```

The shielded tokens minted in this call, as a map from hex-encoded 256-bit domain separators to unsigned 64-bit integers.

### unshieldedInputs[​](#unshieldedinputs "Direct link to unshieldedInputs")

```
unshieldedInputs: Map<TokenType, bigint>;
```

The unshielded inputs this contract expects.

### unshieldedMints[​](#unshieldedmints "Direct link to unshieldedMints")

```
unshieldedMints: Map<string, bigint>;
```

The unshielded tokens minted in this call, as a map from hex-encoded 256-bit domain separators to unsigned 64-bit integers.

### unshieldedOutputs[​](#unshieldedoutputs "Direct link to unshieldedOutputs")

```
unshieldedOutputs: Map<TokenType, bigint>;
```

The unshielded outputs this contract authorizes.
