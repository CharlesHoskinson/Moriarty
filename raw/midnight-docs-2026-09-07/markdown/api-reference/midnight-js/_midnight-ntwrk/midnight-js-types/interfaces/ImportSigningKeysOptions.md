# ImportSigningKeysOptions

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / ImportSigningKeysOptions

# Interface: ImportSigningKeysOptions

Options for importing signing keys.

## Properties[​](#properties "Direct link to Properties")

### conflictStrategy?[​](#conflictstrategy "Direct link to conflictStrategy?")

> `readonly` `optional` **conflictStrategy?**: `"error"` | `"skip"` | `"overwrite"`

How to handle conflicts when a signing key already exists for an address.

* 'skip': Keep existing key, ignore imported key
* 'overwrite': Replace existing key with imported key
* 'error': Throw an error if any conflict is detected Default: 'error'

***

### maxKeys?[​](#maxkeys "Direct link to maxKeys?")

> `readonly` `optional` **maxKeys?**: `number`

Maximum number of keys to import. Defaults to MAX\_EXPORT\_SIGNING\_KEYS (10000). Set to a lower value to limit memory usage.

***

### password?[​](#password "Direct link to password?")

> `readonly` `optional` **password?**: `string`

Password used to decrypt the import. Must match the password used during export. If not provided, uses the storage password.
