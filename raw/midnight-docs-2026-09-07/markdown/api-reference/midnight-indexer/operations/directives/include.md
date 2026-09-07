# include

> For the complete documentation index, see [llms.txt](/llms.txt)

Directs the executor to include this field or fragment only when the `if` argument is true.

```
directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`include.if`](#) ● [`Boolean!`](/api-reference/midnight-indexer/types/scalars/boolean.md) non-null scalar[​](#includeifboolean-- "Direct link to includeifboolean--")
