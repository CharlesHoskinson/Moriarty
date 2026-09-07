# MidnightWalletProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

Provider class that implements wallet functionality for the Midnight network. Handles transaction balancing, submission, and wallet state management.

## Implements[​](#implements "Direct link to Implements")

* `MidnightProvider`
* `WalletProvider`

## Properties[​](#properties "Direct link to Properties")

### dustSecretKey[​](#dustsecretkey "Direct link to dustSecretKey")

> `readonly` **dustSecretKey**: `DustSecretKey`

***

### env[​](#env "Direct link to env")

> `readonly` **env**: [`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

***

### logger[​](#logger "Direct link to logger")

> **logger**: `Logger`

***

### unshieldedKeystore[​](#unshieldedkeystore "Direct link to unshieldedKeystore")

> `readonly` **unshieldedKeystore**: `UnshieldedKeystore`

***

### wallet[​](#wallet "Direct link to wallet")

> `readonly` **wallet**: `WalletFacade`

***

### zswapSecretKeys[​](#zswapsecretkeys "Direct link to zswapSecretKeys")

> `readonly` **zswapSecretKeys**: `ZswapSecretKeys`

## Methods[​](#methods "Direct link to Methods")

### balanceTx()[​](#balancetx "Direct link to balanceTx()")

> **balanceTx**(`tx`, `ttl?`): `Promise`<`FinalizedTransaction`>

Balances a transaction

#### Parameters[​](#parameters "Direct link to Parameters")

##### tx[​](#tx "Direct link to tx")

`UnboundTransaction`

The transaction to balance.

##### ttl?[​](#ttl "Direct link to ttl?")

`Date` = `...`

#### Returns[​](#returns "Direct link to Returns")

`Promise`<`FinalizedTransaction`>

#### Implementation of[​](#implementation-of "Direct link to Implementation of")

`WalletProvider.balanceTx`

***

### getCoinPublicKey()[​](#getcoinpublickey "Direct link to getCoinPublicKey()")

> **getCoinPublicKey**(): `string`

#### Returns[​](#returns-1 "Direct link to Returns")

`string`

#### Implementation of[​](#implementation-of-1 "Direct link to Implementation of")

`WalletProvider.getCoinPublicKey`

***

### getEncryptionPublicKey()[​](#getencryptionpublickey "Direct link to getEncryptionPublicKey()")

> **getEncryptionPublicKey**(): `string`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`

#### Implementation of[​](#implementation-of-2 "Direct link to Implementation of")

`WalletProvider.getEncryptionPublicKey`

***

### start()[​](#start "Direct link to start()")

> **start**(`waitForFundsInWallet?`, `tokenType?`): `Promise`<`void`>

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### waitForFundsInWallet?[​](#waitforfundsinwallet "Direct link to waitForFundsInWallet?")

`boolean` = `true`

##### tokenType?[​](#tokentype "Direct link to tokenType?")

`TokenType` = `...`

#### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<`void`>

***

### stop()[​](#stop "Direct link to stop()")

> **stop**(): `Promise`<`void`>

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<`void`>

***

### submitTx()[​](#submittx "Direct link to submitTx()")

> **submitTx**(`tx`): `Promise`<`string`>

Submit a transaction to the network to be consensed upon.

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### tx[​](#tx-1 "Direct link to tx")

`FinalizedTransaction`

The finalized transaction to submit.

#### Returns[​](#returns-5 "Direct link to Returns")

`Promise`<`string`>

The transaction identifier of the submitted transaction.

#### Implementation of[​](#implementation-of-3 "Direct link to Implementation of")

`MidnightProvider.submitTx`

***

### build()[​](#build "Direct link to build()")

> `static` **build**(`logger`, `env`, `seed?`): `Promise`<`MidnightWalletProvider`>

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### logger[​](#logger-1 "Direct link to logger")

`Logger`

##### env[​](#env-1 "Direct link to env")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

##### seed?[​](#seed "Direct link to seed?")

`string`

#### Returns[​](#returns-6 "Direct link to Returns")

`Promise`<`MidnightWalletProvider`>

***

### withWallet()[​](#withwallet "Direct link to withWallet()")

> `static` **withWallet**(`logger`, `env`, `wallet`, `zswapSecretKeys`, `dustSecretKey`, `unshieldedKeystore`): `Promise`<`MidnightWalletProvider`>

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### logger[​](#logger-2 "Direct link to logger")

`Logger`

##### env[​](#env-2 "Direct link to env")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

##### wallet[​](#wallet-1 "Direct link to wallet")

`WalletFacade`

##### zswapSecretKeys[​](#zswapsecretkeys-1 "Direct link to zswapSecretKeys")

`ZswapSecretKeys`

##### dustSecretKey[​](#dustsecretkey-1 "Direct link to dustSecretKey")

`DustSecretKey`

##### unshieldedKeystore[​](#unshieldedkeystore-1 "Direct link to unshieldedKeystore")

`UnshieldedKeystore`

#### Returns[​](#returns-7 "Direct link to Returns")

`Promise`<`MidnightWalletProvider`>
