# LedgerState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / LedgerState

# Class: LedgerState

The state of the Midnight ledger

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new LedgerState(network_id, zswap): LedgerState;
```

Intializes from a Zswap state, with an empty contract set

#### Parameters[​](#parameters "Direct link to Parameters")

##### network\_id[​](#network_id "Direct link to network_id")

`string`

##### zswap[​](#zswap "Direct link to zswap")

[`ZswapChainState`](/api-reference/ledger/classes/ZswapChainState.md)

#### Returns[​](#returns "Direct link to Returns")

`LedgerState`

## Properties[​](#properties "Direct link to Properties")

### blockRewardPool[​](#blockrewardpool "Direct link to blockRewardPool")

```
readonly blockRewardPool: bigint;
```

The remaining unrewarded supply of native tokens.

***

### dust[​](#dust "Direct link to dust")

```
readonly dust: DustState;
```

The dust subsystem state

***

### lockedPool[​](#lockedpool "Direct link to lockedPool")

```
readonly lockedPool: bigint;
```

The remaining size of the locked Night pool.

***

### parameters[​](#parameters-1 "Direct link to parameters")

```
parameters: LedgerParameters;
```

The parameters of the ledger

***

### reservePool[​](#reservepool "Direct link to reservePool")

```
readonly reservePool: bigint;
```

The size of the reserve Night pool

***

### utxo[​](#utxo "Direct link to utxo")

```
readonly utxo: UtxoState;
```

The unshielded utxos present

***

### zswap[​](#zswap-1 "Direct link to zswap")

```
readonly zswap: ZswapChainState;
```

The Zswap part of the ledger state

## Methods[​](#methods "Direct link to Methods")

### apply()[​](#apply "Direct link to apply()")

```
apply(transaction, context): [LedgerState, TransactionResult];
```

Applies a [Transaction](/api-reference/ledger/classes/Transaction.md)

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### transaction[​](#transaction "Direct link to transaction")

[`VerifiedTransaction`](/api-reference/ledger/classes/VerifiedTransaction.md)

##### context[​](#context "Direct link to context")

[`TransactionContext`](/api-reference/ledger/classes/TransactionContext.md)

#### Returns[​](#returns-1 "Direct link to Returns")

\[`LedgerState`, [`TransactionResult`](/api-reference/ledger/classes/TransactionResult.md)]

***

### applySystemTx()[​](#applysystemtx "Direct link to applySystemTx()")

```
applySystemTx(transaction, tblock): [LedgerState, Event[]];
```

Applies a system transaction to this ledger state.

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### transaction[​](#transaction-1 "Direct link to transaction")

[`SystemTransaction`](/api-reference/ledger/classes/SystemTransaction.md)

##### tblock[​](#tblock "Direct link to tblock")

`Date`

#### Returns[​](#returns-2 "Direct link to Returns")

\[`LedgerState`, [`Event`](/api-reference/ledger/classes/Event.md)\[]]

***

### bridgeReceiving()[​](#bridgereceiving "Direct link to bridgeReceiving()")

#### Call Signature[​](#call-signature "Direct link to Call Signature")

```
bridgeReceiving(recipient): bigint;
```

How much in bridged night a recipient is owed and can claim.

##### Parameters[​](#parameters-4 "Direct link to Parameters")

###### recipient[​](#recipient "Direct link to recipient")

`string`

##### Returns[​](#returns-3 "Direct link to Returns")

`bigint`

#### Call Signature[​](#call-signature-1 "Direct link to Call Signature")

```
bridgeReceiving(recipient): bigint;
```

How much in bridged night a recipient is owed and can claim.

##### Parameters[​](#parameters-5 "Direct link to Parameters")

###### recipient[​](#recipient-1 "Direct link to recipient")

`string`

##### Returns[​](#returns-4 "Direct link to Returns")

`bigint`

***

### index()[​](#index "Direct link to index()")

```
index(address): undefined | ContractState;
```

Indexes into the contract state map with a given contract address

#### Parameters[​](#parameters-6 "Direct link to Parameters")

##### address[​](#address "Direct link to address")

`string`

#### Returns[​](#returns-5 "Direct link to Returns")

`undefined` | [`ContractState`](/api-reference/ledger/classes/ContractState.md)

***

### postBlockUpdate()[​](#postblockupdate "Direct link to postBlockUpdate()")

```
postBlockUpdate(

   tblock, 

   detailedBlockFullness?, 

   overallBlockFullness?): LedgerState;
```

Carries out a post-block update, which does amortized bookkeeping that only needs to be done once per state change.

Typically, `postBlockUpdate` should be run after any (sequence of) (system)-transaction application(s).

#### Parameters[​](#parameters-7 "Direct link to Parameters")

##### tblock[​](#tblock-1 "Direct link to tblock")

`Date`

##### detailedBlockFullness?[​](#detailedblockfullness "Direct link to detailedBlockFullness?")

[`NormalizedCost`](/api-reference/ledger/type-aliases/NormalizedCost.md)

##### overallBlockFullness?[​](#overallblockfullness "Direct link to overallBlockFullness?")

`number`

#### Returns[​](#returns-6 "Direct link to Returns")

`LedgerState`

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns-7 "Direct link to Returns")

`Uint8Array`

***

### testingDistributeNight()[​](#testingdistributenight "Direct link to testingDistributeNight()")

```
testingDistributeNight(

   recipient, 

   amount, 

   tblock): LedgerState;
```

Allows distributing the specified amount of Night to the recipient's address. Use is for testing purposes only.

#### Parameters[​](#parameters-8 "Direct link to Parameters")

##### recipient[​](#recipient-2 "Direct link to recipient")

`string`

##### amount[​](#amount "Direct link to amount")

`bigint`

##### tblock[​](#tblock-2 "Direct link to tblock")

`Date`

#### Returns[​](#returns-8 "Direct link to Returns")

`LedgerState`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-9 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-9 "Direct link to Returns")

`string`

***

### treasuryBalance()[​](#treasurybalance "Direct link to treasuryBalance()")

```
treasuryBalance(token_type): bigint;
```

Retrieves the balance of the treasury for a specific token type.

#### Parameters[​](#parameters-10 "Direct link to Parameters")

##### token\_type[​](#token_type "Direct link to token_type")

[`TokenType`](/api-reference/ledger/type-aliases/TokenType.md)

#### Returns[​](#returns-10 "Direct link to Returns")

`bigint`

***

### unclaimedBlockRewards()[​](#unclaimedblockrewards "Direct link to unclaimedBlockRewards()")

```
unclaimedBlockRewards(recipient): bigint;
```

How much in block rewards a recipient is owed and can claim.

#### Parameters[​](#parameters-11 "Direct link to Parameters")

##### recipient[​](#recipient-3 "Direct link to recipient")

`string`

#### Returns[​](#returns-11 "Direct link to Returns")

`bigint`

***

### updateIndex()[​](#updateindex "Direct link to updateIndex()")

```
updateIndex(

   address, 

   state, 

   balance): LedgerState;
```

Sets the state of a given contract address from a [ChargedState](/api-reference/ledger/classes/ChargedState.md)

#### Parameters[​](#parameters-12 "Direct link to Parameters")

##### address[​](#address-1 "Direct link to address")

`string`

##### state[​](#state "Direct link to state")

[`ChargedState`](/api-reference/ledger/classes/ChargedState.md)

##### balance[​](#balance "Direct link to balance")

`Map`<[`TokenType`](/api-reference/ledger/type-aliases/TokenType.md), `bigint`>

#### Returns[​](#returns-12 "Direct link to Returns")

`LedgerState`

***

### blank()[​](#blank "Direct link to blank()")

```
static blank(network_id): LedgerState;
```

A fully blank state

#### Parameters[​](#parameters-13 "Direct link to Parameters")

##### network\_id[​](#network_id-1 "Direct link to network_id")

`string`

#### Returns[​](#returns-13 "Direct link to Returns")

`LedgerState`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): LedgerState;
```

#### Parameters[​](#parameters-14 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-14 "Direct link to Returns")

`LedgerState`
