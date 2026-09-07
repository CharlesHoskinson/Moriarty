# StorageEncryption

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-level-private-state-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider.md) / StorageEncryption

# Class: StorageEncryption

## Methods[​](#methods "Direct link to Methods")

### decrypt()[​](#decrypt "Direct link to decrypt()")

> **decrypt**(`encryptedData`): `Promise`<`string`>

#### Parameters[​](#parameters "Direct link to Parameters")

##### encryptedData[​](#encrypteddata "Direct link to encryptedData")

`string`

#### Returns[​](#returns "Direct link to Returns")

`Promise`<`string`>

***

### decryptWithPassword()[​](#decryptwithpassword "Direct link to decryptWithPassword()")

> **decryptWithPassword**(`encryptedData`, `password`): `Promise`<`string`>

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### encryptedData[​](#encrypteddata-1 "Direct link to encryptedData")

`string`

##### password[​](#password "Direct link to password")

`string`

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`string`>

***

### encrypt()[​](#encrypt "Direct link to encrypt()")

> **encrypt**(`data`): `Promise`<`string`>

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### data[​](#data "Direct link to data")

`string`

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<`string`>

***

### getSalt()[​](#getsalt "Direct link to getSalt()")

> **getSalt**(): `Buffer`

#### Returns[​](#returns-3 "Direct link to Returns")

`Buffer`

***

### verifyPassword()[​](#verifypassword "Direct link to verifyPassword()")

> **verifyPassword**(`password`): `Promise`<`boolean`>

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### password[​](#password-1 "Direct link to password")

`string`

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<`boolean`>

***

### create()[​](#create "Direct link to create()")

> `static` **create**(`password`, `options?`): `Promise`<`StorageEncryption`>

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### password[​](#password-2 "Direct link to password")

`string`

##### options?[​](#options "Direct link to options?")

[`StorageEncryptionOptions`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/interfaces/StorageEncryptionOptions.md)

#### Returns[​](#returns-5 "Direct link to Returns")

`Promise`<`StorageEncryption`>

***

### getVersion()[​](#getversion "Direct link to getVersion()")

> `static` **getVersion**(`encryptedData`): `number`

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### encryptedData[​](#encrypteddata-2 "Direct link to encryptedData")

`string`

#### Returns[​](#returns-6 "Direct link to Returns")

`number`

***

### isEncrypted()[​](#isencrypted "Direct link to isEncrypted()")

> `static` **isEncrypted**(`data`): `boolean`

#### Parameters[​](#parameters-6 "Direct link to Parameters")

##### data[​](#data-1 "Direct link to data")

`string`

#### Returns[​](#returns-7 "Direct link to Returns")

`boolean`
