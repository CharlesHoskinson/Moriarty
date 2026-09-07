# Bindingish

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / Bindingish

# Type Alias: Bindingish

```
type Bindingish = 

  | Binding

  | PreBinding

  | NoBinding;
```

Whether an intent has binding cryptography applied or not. An intent's content can no longer be modified after it is [Binding](/api-reference/ledger/classes/Binding.md).
