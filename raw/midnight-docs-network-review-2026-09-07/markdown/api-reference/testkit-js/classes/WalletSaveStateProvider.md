# WalletSaveStateProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

Provider class for saving and loading wallet state to/from compressed files

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new WalletSaveStateProvider**(`logger`, `seed`, `directoryPath?`, `filename?`): `WalletSaveStateProvider`

Creates a new WalletSaveStateProvider instance

#### Parameters[​](#parameters "Direct link to Parameters")

##### logger[​](#logger "Direct link to logger")

`Logger`

Logger instance for recording operations

##### seed[​](#seed "Direct link to seed")

`string`

##### directoryPath?[​](#directorypath "Direct link to directoryPath?")

`string` = `DEFAULT_WALLET_STATE_DIRECTORY`

Directory path for wallet state files

##### filename?[​](#filename "Direct link to filename?")

`string` = `...`

Filename for the wallet state file

#### Returns[​](#returns "Direct link to Returns")

`WalletSaveStateProvider`

## Properties[​](#properties "Direct link to Properties")

### directoryPath[​](#directorypath-1 "Direct link to directoryPath")

> **directoryPath**: `string`

Absolute path to the directory containing wallet state files

***

### filePath[​](#filepath "Direct link to filePath")

> **filePath**: `string`

Full path including filename for the wallet state file

***

### logger[​](#logger-1 "Direct link to logger")

> **logger**: `Logger`

Logger instance for recording operations

## Methods[​](#methods "Direct link to Methods")

### load()[​](#load "Direct link to load()")

> **load**(): `Promise`<`string`>

Loads and decompresses the wallet state from a file

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`string`>

A promise that resolves with the decompressed wallet state as a string

#### Throws[​](#throws "Direct link to Throws")

If there is an error reading or decompressing the file

***

### save()[​](#save "Direct link to save()")

> **save**(`wallet`): `Promise`<`void`>

Saves the wallet state to a compressed file

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### wallet[​](#wallet "Direct link to wallet")

`ShieldedWalletAPI` | `UnshieldedWalletAPI`

The wallet instance to save state from

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<`void`>

A promise that resolves when the save is complete
