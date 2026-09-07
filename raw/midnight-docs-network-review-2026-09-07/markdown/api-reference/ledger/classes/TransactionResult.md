# TransactionResult

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / TransactionResult

# Class: TransactionResult

The result status of applying a transaction. Includes an error message if the transaction failed, or partially failed.

## Properties[​](#properties "Direct link to Properties")

### error?[​](#error "Direct link to error?")

```
readonly optional error: string;
```

***

### events[​](#events "Direct link to events")

```
readonly events: Event[];
```

***

### successfulSegments?[​](#successfulsegments "Direct link to successfulSegments?")

```
readonly optional successfulSegments: Map<number, boolean>;
```

***

### type[​](#type "Direct link to type")

```
readonly type: "success" | "partialSuccess" | "failure";
```

## Methods[​](#methods "Direct link to Methods")

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns "Direct link to Returns")

`string`
