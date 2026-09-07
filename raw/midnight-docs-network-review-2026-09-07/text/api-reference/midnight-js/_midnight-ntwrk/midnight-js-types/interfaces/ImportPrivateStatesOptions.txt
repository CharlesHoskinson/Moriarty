# ImportPrivateStatesOptions

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / ImportPrivateStatesOptions

# Interface: ImportPrivateStatesOptions

Options for importing private states.

## Properties[​](#properties "Direct link to Properties")

### conflictStrategy?[​](#conflictstrategy "Direct link to conflictStrategy?")

> `readonly` `optional` **conflictStrategy?**: `"error"` | `"skip"` | `"overwrite"`

How to handle conflicts when a private state ID already exists.

* 'skip': Keep existing state, ignore imported state
* 'overwrite': Replace existing state with imported state
* 'error': Throw an error if any conflict is detected Default: 'error'

***

### maxStates?[​](#maxstates "Direct link to maxStates?")

> `readonly` `optional` **maxStates?**: `number`

Maximum number of states to import. Defaults to MAX\_EXPORT\_STATES (10000). Set to a lower value to limit memory usage.

***

### password?[​](#password "Direct link to password?")

> `readonly` `optional` **password?**: `string`

Password used to decrypt the import. Must match the password used during export. If not provided, uses the storage password.
