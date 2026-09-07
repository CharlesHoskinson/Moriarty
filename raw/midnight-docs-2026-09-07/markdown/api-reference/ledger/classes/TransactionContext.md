# TransactionContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / TransactionContext

# Class: TransactionContext

The context against which a transaction is run.

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new TransactionContext(

   ref_state, 

   block_context, 

   whitelist?): TransactionContext;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### ref\_state[​](#ref_state "Direct link to ref_state")

[`LedgerState`](/api-reference/ledger/classes/LedgerState.md)

A past ledger state that is used as a reference point for 'static' data.

##### block\_context[​](#block_context "Direct link to block_context")

[`BlockContext`](/api-reference/ledger/type-aliases/BlockContext.md)

Information about the block this transaction is, or will be, contained in.

##### whitelist?[​](#whitelist "Direct link to whitelist?")

`Set`<`string`>

A list of contracts that are being tracked, or `undefined` to track all contracts.

#### Returns[​](#returns "Direct link to Returns")

`TransactionContext`

## Methods[​](#methods "Direct link to Methods")

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-1 "Direct link to Returns")

`string`
