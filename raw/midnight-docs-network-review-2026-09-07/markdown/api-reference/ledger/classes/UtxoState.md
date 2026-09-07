# UtxoState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / UtxoState

# Class: UtxoState

The sub-state for unshielded UTXOs

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new UtxoState(): UtxoState;
```

#### Returns[​](#returns "Direct link to Returns")

`UtxoState`

## Properties[​](#properties "Direct link to Properties")

### utxos[​](#utxos "Direct link to utxos")

```
readonly utxos: Set<Utxo>;
```

The set of valid UTXOs

## Methods[​](#methods "Direct link to Methods")

### delta()[​](#delta "Direct link to delta()")

```
delta(prior, filterBy?): [Set<Utxo>, Set<Utxo>];
```

Given a prior UTXO state, produce the set differences `this \ prior`, and `prior \ this`, optionally filtered by a further condition.

Note that this should be more efficient than iterating or manifesting the [utxos](#utxos) value, as the low-level implementation can avoid traversing shared sub-structures.

#### Parameters[​](#parameters "Direct link to Parameters")

##### prior[​](#prior "Direct link to prior")

`UtxoState`

##### filterBy?[​](#filterby "Direct link to filterBy?")

(`utxo`) => `boolean`

#### Returns[​](#returns-1 "Direct link to Returns")

\[`Set`<[`Utxo`](/api-reference/ledger/type-aliases/Utxo.md)>, `Set`<[`Utxo`](/api-reference/ledger/type-aliases/Utxo.md)>]

***

### filter()[​](#filter "Direct link to filter()")

```
filter(addr): Set<Utxo>;
```

Filters out the UTXOs owned by a specific user address

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### addr[​](#addr "Direct link to addr")

`string`

#### Returns[​](#returns-2 "Direct link to Returns")

`Set`<[`Utxo`](/api-reference/ledger/type-aliases/Utxo.md)>

***

### lookupMeta()[​](#lookupmeta "Direct link to lookupMeta()")

```
lookupMeta(utxo): undefined | UtxoMeta;
```

Lookup the metadata for a specific UTXO.

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### utxo[​](#utxo "Direct link to utxo")

[`Utxo`](/api-reference/ledger/type-aliases/Utxo.md)

#### Returns[​](#returns-3 "Direct link to Returns")

`undefined` | [`UtxoMeta`](/api-reference/ledger/classes/UtxoMeta.md)

***

### new()[​](#new "Direct link to new()")

```
static new(utxos): UtxoState;
```

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### utxos[​](#utxos-1 "Direct link to utxos")

`Map`<[`Utxo`](/api-reference/ledger/type-aliases/Utxo.md), [`UtxoMeta`](/api-reference/ledger/classes/UtxoMeta.md)>

#### Returns[​](#returns-4 "Direct link to Returns")

`UtxoState`
