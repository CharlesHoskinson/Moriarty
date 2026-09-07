# deprecated

> For the complete documentation index, see [llms.txt](/llms.txt)

Marks an element of a GraphQL schema as no longer supported.

```
directive @deprecated(

  reason: String = "No longer supported"

) on FIELD_DEFINITION | ARGUMENT_DEFINITION | INPUT_FIELD_DEFINITION | ENUM_VALUE
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`deprecated.reason`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#deprecatedreasonstring- "Direct link to deprecatedreasonstring-")

Explains why this element was deprecated, usually also including a suggestion for how to access supported similar data. Formatted using the Markdown syntax, as specified by [CommonMark](https://commonmark.org/).
