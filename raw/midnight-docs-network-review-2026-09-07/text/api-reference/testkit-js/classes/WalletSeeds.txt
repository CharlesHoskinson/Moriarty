# WalletSeeds

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

## Properties[​](#properties "Direct link to Properties")

### dust[​](#dust "Direct link to dust")

> `readonly` **dust**: `Uint8Array`

***

### masterSeed[​](#masterseed "Direct link to masterSeed")

> `readonly` **masterSeed**: `string`

***

### shielded[​](#shielded "Direct link to shielded")

> `readonly` **shielded**: `Uint8Array`

***

### unshielded[​](#unshielded "Direct link to unshielded")

> `readonly` **unshielded**: `Uint8Array`

## Methods[​](#methods "Direct link to Methods")

### fromMasterSeed()[​](#frommasterseed "Direct link to fromMasterSeed()")

> `static` **fromMasterSeed**(`seed`): `WalletSeeds`

#### Parameters[​](#parameters "Direct link to Parameters")

##### seed[​](#seed "Direct link to seed")

`string`

#### Returns[​](#returns "Direct link to Returns")

`WalletSeeds`

***

### fromMnemonic()[​](#frommnemonic "Direct link to fromMnemonic()")

> `static` **fromMnemonic**(`mnemonic`): `WalletSeeds`

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### mnemonic[​](#mnemonic "Direct link to mnemonic")

`string`

#### Returns[​](#returns-1 "Direct link to Returns")

`WalletSeeds`

***

### generateRandom()[​](#generaterandom "Direct link to generateRandom()")

> `static` **generateRandom**(): `WalletSeeds`

#### Returns[​](#returns-2 "Direct link to Returns")

`WalletSeeds`

***

### testWallet()[​](#testwallet "Direct link to testWallet()")

> `static` **testWallet**(): `WalletSeeds`

#### Returns[​](#returns-3 "Direct link to Returns")

`WalletSeeds`
