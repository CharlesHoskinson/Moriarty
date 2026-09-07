# FailFallible

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / FailFallible

# Variable: FailFallible

> `const` **FailFallible**: `"FailFallible"`

Indicates that the transaction is valid but the portion of the transcript that is allowed to fail (the portion after a checkpoint) did fail. All effects from the guaranteed part of the transaction are kept but the effects from the fallible part of the transaction are discarded.
