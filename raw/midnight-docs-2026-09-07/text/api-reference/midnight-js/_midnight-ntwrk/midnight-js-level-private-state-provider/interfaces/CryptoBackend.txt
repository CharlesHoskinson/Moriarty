# CryptoBackend

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-level-private-state-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider.md) / CryptoBackend

# Interface: CryptoBackend

## Methods[​](#methods "Direct link to Methods")

### aesGcmDecrypt()[​](#aesgcmdecrypt "Direct link to aesGcmDecrypt()")

> **aesGcmDecrypt**(`key`, `iv`, `ciphertext`, `authTag`): `Promise`<`Uint8Array`<`ArrayBufferLike`>>

#### Parameters[​](#parameters "Direct link to Parameters")

##### key[​](#key "Direct link to key")

`Uint8Array`

##### iv[​](#iv "Direct link to iv")

`Uint8Array`

##### ciphertext[​](#ciphertext "Direct link to ciphertext")

`Uint8Array`

##### authTag[​](#authtag "Direct link to authTag")

`Uint8Array`

#### Returns[​](#returns "Direct link to Returns")

`Promise`<`Uint8Array`<`ArrayBufferLike`>>

***

### aesGcmEncrypt()[​](#aesgcmencrypt "Direct link to aesGcmEncrypt()")

> **aesGcmEncrypt**(`key`, `iv`, `plaintext`): `Promise`<{ `authTag`: `Uint8Array`; `ciphertext`: `Uint8Array`; }>

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### key[​](#key-1 "Direct link to key")

`Uint8Array`

##### iv[​](#iv-1 "Direct link to iv")

`Uint8Array`

##### plaintext[​](#plaintext "Direct link to plaintext")

`Uint8Array`

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<{ `authTag`: `Uint8Array`; `ciphertext`: `Uint8Array`; }>

***

### pbkdf2()[​](#pbkdf2 "Direct link to pbkdf2()")

> **pbkdf2**(`password`, `salt`, `iterations`, `keyLength`): `Promise`<`Uint8Array`<`ArrayBufferLike`>>

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### password[​](#password "Direct link to password")

`Uint8Array`

##### salt[​](#salt "Direct link to salt")

`Uint8Array`

##### iterations[​](#iterations "Direct link to iterations")

`number`

##### keyLength[​](#keylength "Direct link to keyLength")

`number`

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<`Uint8Array`<`ArrayBufferLike`>>

***

### randomBytes()[​](#randombytes "Direct link to randomBytes()")

> **randomBytes**(`length`): `Uint8Array`

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### length[​](#length "Direct link to length")

`number`

#### Returns[​](#returns-3 "Direct link to Returns")

`Uint8Array`

***

### sha256()[​](#sha256 "Direct link to sha256()")

> **sha256**(`data`): `Promise`<`Uint8Array`<`ArrayBufferLike`>>

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### data[​](#data "Direct link to data")

`Uint8Array`

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<`Uint8Array`<`ArrayBufferLike`>>
