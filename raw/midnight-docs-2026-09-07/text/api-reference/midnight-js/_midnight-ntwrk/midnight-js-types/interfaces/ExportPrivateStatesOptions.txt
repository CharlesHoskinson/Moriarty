# ExportPrivateStatesOptions

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / ExportPrivateStatesOptions

# Interface: ExportPrivateStatesOptions

Options for exporting private states.

## Properties[​](#properties "Direct link to Properties")

### maxStates?[​](#maxstates "Direct link to maxStates?")

> `readonly` `optional` **maxStates?**: `number`

Maximum number of states to export. Defaults to MAX\_EXPORT\_STATES (10000). Set to a lower value to limit memory usage.

***

### password?[​](#password "Direct link to password?")

> `readonly` `optional` **password?**: `string`

Password used to encrypt the export. Must be at least 16 characters. If not provided, uses the storage password.
