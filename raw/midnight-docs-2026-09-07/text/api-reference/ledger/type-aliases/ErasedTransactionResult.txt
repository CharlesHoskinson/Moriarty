# ErasedTransactionResult

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ErasedTransactionResult

# Type Alias: ErasedTransactionResult

```
type ErasedTransactionResult = {

  successfulSegments?: Map<number, boolean>;

  type: "success" | "partialSuccess" | "failure";

};
```

The result status of applying a transaction, without error message

## Properties[​](#properties "Direct link to Properties")

### successfulSegments?[​](#successfulsegments "Direct link to successfulSegments?")

```
optional successfulSegments: Map<number, boolean>;
```

***

### type[​](#type "Direct link to type")

```
type: "success" | "partialSuccess" | "failure";
```
