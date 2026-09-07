# Transaction

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / Transaction

# Class: Transaction\<S, P, B>

A Midnight transaction, consisting a section of [ContractAction](/api-reference/ledger/type-aliases/ContractAction.md)s, and a guaranteed and fallible [ZswapOffer](/api-reference/ledger/classes/ZswapOffer.md).

The guaranteed section are run first, and fee payment is taken during this part. If it succeeds, the fallible section is also run, and atomically rolled back if it fails.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### S[​](#s "Direct link to S")

`S` *extends* [`Signaturish`](/api-reference/ledger/type-aliases/Signaturish.md)

### P[​](#p "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

### B[​](#b "Direct link to B")

`B` *extends* [`Bindingish`](/api-reference/ledger/type-aliases/Bindingish.md)

## Properties[​](#properties "Direct link to Properties")

### bindingRandomness[​](#bindingrandomness "Direct link to bindingRandomness")

```
readonly bindingRandomness: bigint;
```

The binding randomness associated with this transaction

***

### fallibleOffer[​](#fallibleoffer "Direct link to fallibleOffer")

```
fallibleOffer: undefined | Map<number, ZswapOffer<P>>;
```

The fallible Zswap offer

Note that writing to this re-computes binding information if and only if this transaction is unbound *and* unproven. If this is not the case, creating or removing offer components will lead to a binding error down the line.

#### Throws[​](#throws "Direct link to Throws")

On writing if `B` is [Binding](/api-reference/ledger/classes/Binding.md) or this is not a standard transaction

***

### guaranteedOffer[​](#guaranteedoffer "Direct link to guaranteedOffer")

```
guaranteedOffer: undefined | ZswapOffer<P>;
```

The guaranteed Zswap offer

Note that writing to this re-computes binding information if and only if this transaction is unbound *and* unproven. If this is not the case, creating or removing offer components will lead to a binding error down the line.

#### Throws[​](#throws-1 "Direct link to Throws")

On writing if `B` is [Binding](/api-reference/ledger/classes/Binding.md) or this is not a standard transaction

***

### intents[​](#intents "Direct link to intents")

```
intents: undefined | Map<number, Intent<S, P, B>>;
```

The intents contained in this transaction

Note that writing to this re-computes binding information if and only if this transaction is unbound *and* unproven. If this is not the case, creating or removing intents will lead to a binding error down the line, but modifying existing intents will succeed.

#### Throws[​](#throws-2 "Direct link to Throws")

On writing if `B` is [Binding](/api-reference/ledger/classes/Binding.md) or this is not a standard transaction

***

### rewards[​](#rewards "Direct link to rewards")

```
readonly rewards: 

  | undefined

| ClaimRewardsTransaction<S>;
```

The rewards this transaction represents, if applicable

## Methods[​](#methods "Direct link to Methods")

### addCalls()[​](#addcalls "Direct link to addCalls()")

```
addCalls(

   segment, 

   calls, 

   params, 

   ttl, 

   zswapInputs?, 

   zswapOutputs?, 

zswapTransient?): Transaction<S, P, B>;
```

Adds a set of new calls to the transaction.

In contrast to [Intent.addCall](/api-reference/ledger/classes/Intent.md#addcall), this takes calls *before* transcript partitioning ([partitionTranscripts](/api-reference/ledger/functions/partitionTranscripts.md)), will create the target intent where needed, and will ensure that relevant Zswap parts are placed in the same section as contract interactions with them.

#### Parameters[​](#parameters "Direct link to Parameters")

##### segment[​](#segment "Direct link to segment")

[`SegmentSpecifier`](/api-reference/ledger/type-aliases/SegmentSpecifier.md)

##### calls[​](#calls "Direct link to calls")

[`PrePartitionContractCall`](/api-reference/ledger/classes/PrePartitionContractCall.md)\[]

##### params[​](#params "Direct link to params")

[`LedgerParameters`](/api-reference/ledger/classes/LedgerParameters.md)

##### ttl[​](#ttl "Direct link to ttl")

`Date`

##### zswapInputs?[​](#zswapinputs "Direct link to zswapInputs?")

[`ZswapInput`](/api-reference/ledger/classes/ZswapInput.md)<[`PreProof`](/api-reference/ledger/classes/PreProof.md)>\[]

##### zswapOutputs?[​](#zswapoutputs "Direct link to zswapOutputs?")

[`ZswapOutput`](/api-reference/ledger/classes/ZswapOutput.md)<[`PreProof`](/api-reference/ledger/classes/PreProof.md)>\[]

##### zswapTransient?[​](#zswaptransient "Direct link to zswapTransient?")

[`ZswapTransient`](/api-reference/ledger/classes/ZswapTransient.md)<[`PreProof`](/api-reference/ledger/classes/PreProof.md)>\[]

#### Returns[​](#returns "Direct link to Returns")

`Transaction`<`S`, `P`, `B`>

#### Throws[​](#throws-3 "Direct link to Throws")

If called on bound, proven, or proof-erased transactions.

***

### bind()[​](#bind "Direct link to bind()")

```
bind(): Transaction<S, P, Binding>;
```

Enforces binding for this transaction. This is irreversible.

#### Returns[​](#returns-1 "Direct link to Returns")

`Transaction`<`S`, `P`, [`Binding`](/api-reference/ledger/classes/Binding.md)>

***

### cost()[​](#cost "Direct link to cost()")

```
cost(params, enforceTimeToDismiss?): SyntheticCost;
```

The underlying resource cost of this transaction.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### params[​](#params-1 "Direct link to params")

[`LedgerParameters`](/api-reference/ledger/classes/LedgerParameters.md)

##### enforceTimeToDismiss?[​](#enforcetimetodismiss "Direct link to enforceTimeToDismiss?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

[`SyntheticCost`](/api-reference/ledger/type-aliases/SyntheticCost.md)

***

### eraseProofs()[​](#eraseproofs "Direct link to eraseProofs()")

```
eraseProofs(): Transaction<S, NoProof, NoBinding>;
```

Erases the proofs contained in this transaction

#### Returns[​](#returns-3 "Direct link to Returns")

`Transaction`<`S`, [`NoProof`](/api-reference/ledger/classes/NoProof.md), [`NoBinding`](/api-reference/ledger/classes/NoBinding.md)>

***

### eraseSignatures()[​](#erasesignatures "Direct link to eraseSignatures()")

```
eraseSignatures(): Transaction<SignatureErased, P, B>;
```

Removes signatures from this transaction.

#### Returns[​](#returns-4 "Direct link to Returns")

`Transaction`<[`SignatureErased`](/api-reference/ledger/classes/SignatureErased.md), `P`, `B`>

***

### fees()[​](#fees "Direct link to fees()")

```
fees(params, enforceTimeToDismiss?): bigint;
```

The cost of this transaction, in SPECKs.

Note that this is *only* accurate when called with proven transactions.

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### params[​](#params-2 "Direct link to params")

[`LedgerParameters`](/api-reference/ledger/classes/LedgerParameters.md)

##### enforceTimeToDismiss?[​](#enforcetimetodismiss-1 "Direct link to enforceTimeToDismiss?")

`boolean`

#### Returns[​](#returns-5 "Direct link to Returns")

`bigint`

***

### feesWithMargin()[​](#feeswithmargin "Direct link to feesWithMargin()")

```
feesWithMargin(params, margin): bigint;
```

The cost of this transaction, in SPECKs, with a safety margin of `n` blocks applied.

As with [fees](#fees), this is only accurate for proven transactions.

Warning: `n` must be a non-negative integer, and it is an exponent, it is very easy to get a completely unreasonable margin here!

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### params[​](#params-3 "Direct link to params")

[`LedgerParameters`](/api-reference/ledger/classes/LedgerParameters.md)

##### margin[​](#margin "Direct link to margin")

`number`

#### Returns[​](#returns-6 "Direct link to Returns")

`bigint`

***

### identifiers()[​](#identifiers "Direct link to identifiers()")

```
identifiers(): string[];
```

Returns the set of identifiers contained within this transaction. Any of these *may* be used to watch for a specific transaction.

#### Returns[​](#returns-7 "Direct link to Returns")

`string`\[]

***

### imbalances()[​](#imbalances "Direct link to imbalances()")

```
imbalances(segment, fees?): Map<TokenType, bigint>;
```

For given fees, and a given section (guaranteed/fallible), what the surplus or deficit of this transaction in any token type is.

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### segment[​](#segment-1 "Direct link to segment")

`number`

##### fees?[​](#fees-1 "Direct link to fees?")

`bigint`

#### Returns[​](#returns-8 "Direct link to Returns")

`Map`<[`TokenType`](/api-reference/ledger/type-aliases/TokenType.md), `bigint`>

#### Throws[​](#throws-4 "Direct link to Throws")

If `segment` is not a valid segment ID

***

### merge()[​](#merge "Direct link to merge()")

```
merge(other): Transaction<S, P, B>;
```

Merges this transaction with another

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### other[​](#other "Direct link to other")

`Transaction`<`S`, `P`, `B`>

#### Returns[​](#returns-9 "Direct link to Returns")

`Transaction`<`S`, `P`, `B`>

#### Throws[​](#throws-5 "Direct link to Throws")

If both transactions have contract interactions, or they spend the same coins

***

### mockProve()[​](#mockprove "Direct link to mockProve()")

```
mockProve(): Transaction<S, Proof, Binding>;
```

Mocks proving, producing a 'proven' transaction that, while it will *not* verify, is accurate for fee computation purposes.

Due to the variability in proof sizes, this *only* works for transactions that do not contain unproven contract calls.

#### Returns[​](#returns-10 "Direct link to Returns")

`Transaction`<`S`, [`Proof`](/api-reference/ledger/classes/Proof.md), [`Binding`](/api-reference/ledger/classes/Binding.md)>

#### Throws[​](#throws-6 "Direct link to Throws")

If called on bound, proven, or proof-erased transactions, or if the transaction contains unproven contract calls.

***

### prove()[​](#prove "Direct link to prove()")

```
prove(provider, cost_model): Promise<Transaction<S, Proof, B>>;
```

Proves the transaction, with access to a low-level proving provider. This may *only* be called for `P = PreProof`.

#### Parameters[​](#parameters-6 "Direct link to Parameters")

##### provider[​](#provider "Direct link to provider")

[`ProvingProvider`](/api-reference/ledger/type-aliases/ProvingProvider.md)

##### cost\_model[​](#cost_model "Direct link to cost_model")

[`CostModel`](/api-reference/ledger/classes/CostModel.md)

#### Returns[​](#returns-11 "Direct link to Returns")

`Promise`<`Transaction`<`S`, [`Proof`](/api-reference/ledger/classes/Proof.md), `B`>>

#### Throws[​](#throws-7 "Direct link to Throws")

If called on bound, proven, or proof-erased transactions.

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns-12 "Direct link to Returns")

`Uint8Array`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-7 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-13 "Direct link to Returns")

`string`

***

### transactionHash()[​](#transactionhash "Direct link to transactionHash()")

```
transactionHash(): string;
```

Returns the hash associated with this transaction. Due to the ability to merge transactions, this should not be used to watch for a specific transaction.

#### Returns[​](#returns-14 "Direct link to Returns")

`string`

***

### wellFormed()[​](#wellformed "Direct link to wellFormed()")

```
wellFormed(

   ref_state, 

   strictness, 

   tblock): VerifiedTransaction;
```

Tests well-formedness criteria, optionally including transaction balancing

#### Parameters[​](#parameters-8 "Direct link to Parameters")

##### ref\_state[​](#ref_state "Direct link to ref_state")

[`LedgerState`](/api-reference/ledger/classes/LedgerState.md)

##### strictness[​](#strictness "Direct link to strictness")

[`WellFormedStrictness`](/api-reference/ledger/classes/WellFormedStrictness.md)

##### tblock[​](#tblock "Direct link to tblock")

`Date`

#### Returns[​](#returns-15 "Direct link to Returns")

[`VerifiedTransaction`](/api-reference/ledger/classes/VerifiedTransaction.md)

#### Throws[​](#throws-8 "Direct link to Throws")

If the transaction is not well-formed for any reason

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize<S, P, B>(

   markerS, 

   markerP, 

   markerB, 

raw): Transaction<S, P, B>;
```

#### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

##### S[​](#s-1 "Direct link to S")

`S` *extends* [`Signaturish`](/api-reference/ledger/type-aliases/Signaturish.md)

##### P[​](#p-1 "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

##### B[​](#b-1 "Direct link to B")

`B` *extends* [`Bindingish`](/api-reference/ledger/type-aliases/Bindingish.md)

#### Parameters[​](#parameters-9 "Direct link to Parameters")

##### markerS[​](#markers "Direct link to markerS")

`S`\[`"instance"`]

##### markerP[​](#markerp "Direct link to markerP")

`P`\[`"instance"`]

##### markerB[​](#markerb "Direct link to markerB")

`B`\[`"instance"`]

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-16 "Direct link to Returns")

`Transaction`<`S`, `P`, `B`>

***

### fromParts()[​](#fromparts "Direct link to fromParts()")

```
static fromParts(

   network_id, 

   guaranteed?, 

   fallible?, 

   intent?): UnprovenTransaction;
```

Creates a transaction from its parts.

#### Parameters[​](#parameters-10 "Direct link to Parameters")

##### network\_id[​](#network_id "Direct link to network_id")

`string`

##### guaranteed?[​](#guaranteed "Direct link to guaranteed?")

[`UnprovenOffer`](/api-reference/ledger/type-aliases/UnprovenOffer.md)

##### fallible?[​](#fallible "Direct link to fallible?")

[`UnprovenOffer`](/api-reference/ledger/type-aliases/UnprovenOffer.md)

##### intent?[​](#intent "Direct link to intent?")

[`UnprovenIntent`](/api-reference/ledger/type-aliases/UnprovenIntent.md)

#### Returns[​](#returns-17 "Direct link to Returns")

[`UnprovenTransaction`](/api-reference/ledger/type-aliases/UnprovenTransaction.md)

***

### fromPartsRandomized()[​](#frompartsrandomized "Direct link to fromPartsRandomized()")

```
static fromPartsRandomized(

   network_id, 

   guaranteed?, 

   fallible?, 

   intent?): UnprovenTransaction;
```

Creates a transaction from its parts, randomizing the segment ID to better allow merging.

#### Parameters[​](#parameters-11 "Direct link to Parameters")

##### network\_id[​](#network_id-1 "Direct link to network_id")

`string`

##### guaranteed?[​](#guaranteed-1 "Direct link to guaranteed?")

[`UnprovenOffer`](/api-reference/ledger/type-aliases/UnprovenOffer.md)

##### fallible?[​](#fallible-1 "Direct link to fallible?")

[`UnprovenOffer`](/api-reference/ledger/type-aliases/UnprovenOffer.md)

##### intent?[​](#intent-1 "Direct link to intent?")

[`UnprovenIntent`](/api-reference/ledger/type-aliases/UnprovenIntent.md)

#### Returns[​](#returns-18 "Direct link to Returns")

[`UnprovenTransaction`](/api-reference/ledger/type-aliases/UnprovenTransaction.md)

***

### fromRewards()[​](#fromrewards "Direct link to fromRewards()")

```
static fromRewards<S>(rewards): Transaction<S, PreProof, Binding>;
```

Creates a rewards claim transaction, the funds claimed must have been legitimately rewarded previously.

#### Type Parameters[​](#type-parameters-2 "Direct link to Type Parameters")

##### S[​](#s-2 "Direct link to S")

`S` *extends* [`Signaturish`](/api-reference/ledger/type-aliases/Signaturish.md)

#### Parameters[​](#parameters-12 "Direct link to Parameters")

##### rewards[​](#rewards-1 "Direct link to rewards")

[`ClaimRewardsTransaction`](/api-reference/ledger/classes/ClaimRewardsTransaction.md)<`S`>

#### Returns[​](#returns-19 "Direct link to Returns")

`Transaction`<`S`, [`PreProof`](/api-reference/ledger/classes/PreProof.md), [`Binding`](/api-reference/ledger/classes/Binding.md)>
