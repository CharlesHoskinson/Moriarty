# ImportSigningKeysResult

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / ImportSigningKeysResult

# Interface: ImportSigningKeysResult

Result of a signing key import operation.

## Properties[​](#properties "Direct link to Properties")

### imported[​](#imported "Direct link to imported")

> `readonly` **imported**: `number`

Number of keys successfully imported.

***

### overwritten[​](#overwritten "Direct link to overwritten")

> `readonly` **overwritten**: `number`

Number of keys that overwrote existing keys (when conflictStrategy is 'overwrite').

***

### skipped[​](#skipped "Direct link to skipped")

> `readonly` **skipped**: `number`

Number of keys skipped due to conflicts (when conflictStrategy is 'skip').
