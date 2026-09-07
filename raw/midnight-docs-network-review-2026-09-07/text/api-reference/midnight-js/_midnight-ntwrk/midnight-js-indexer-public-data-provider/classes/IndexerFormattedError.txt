# IndexerFormattedError

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-indexer-public-data-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider.md) / IndexerFormattedError

# Class: IndexerFormattedError

An error describing the causes of error that occurred during server-side execution of a query against the Indexer.

## Extends[​](#extends "Direct link to Extends")

* `Error`

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new IndexerFormattedError**(`cause`): `IndexerFormattedError`

#### Parameters[​](#parameters "Direct link to Parameters")

##### cause[​](#cause "Direct link to cause")

readonly `GraphQLFormattedError`\[]

An array of GraphQL errors that occurred during the server-side execution.

#### Returns[​](#returns "Direct link to Returns")

`IndexerFormattedError`

#### Overrides[​](#overrides "Direct link to Overrides")

`Error.constructor`

## Properties[​](#properties "Direct link to Properties")

### cause[​](#cause-1 "Direct link to cause")

> `readonly` **cause**: readonly `GraphQLFormattedError`\[]

An array of GraphQL errors that occurred during the server-side execution.

#### Inherited from[​](#inherited-from "Direct link to Inherited from")

`Error.cause`
