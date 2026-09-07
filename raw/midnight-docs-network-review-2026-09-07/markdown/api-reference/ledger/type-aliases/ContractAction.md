# ContractAction

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ContractAction

# Type Alias: ContractAction\<P>

```
type ContractAction<P> = 

  | ContractCall<P>

  | ContractDeploy

  | MaintenanceUpdate;
```

An interactions with a contract

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### P[​](#p "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)
