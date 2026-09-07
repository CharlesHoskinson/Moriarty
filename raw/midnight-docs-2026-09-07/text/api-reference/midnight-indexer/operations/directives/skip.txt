# skip

> For the complete documentation index, see [llms.txt](/llms.txt)

Directs the executor to skip this field or fragment when the `if` argument is true.

```
directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`skip.if`](#) ● [`Boolean!`](/api-reference/midnight-indexer/types/scalars/boolean.md) non-null scalar[​](#skipifboolean-- "Direct link to skipifboolean--")
