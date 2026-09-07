# WalletFactory

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new WalletFactory**(): `WalletFactory`

#### Returns[​](#returns "Direct link to Returns")

`WalletFactory`

## Methods[​](#methods "Direct link to Methods")

### createDustWallet()[​](#createdustwallet "Direct link to createDustWallet()")

> `static` **createDustWallet**(`config`, `seed`, `dustOptions?`): `DustWalletAPI`

#### Parameters[​](#parameters "Direct link to Parameters")

##### config[​](#config "Direct link to config")

`DefaultV1Configuration`

##### seed[​](#seed "Direct link to seed")

`Uint8Array`

##### dustOptions?[​](#dustoptions "Direct link to dustOptions?")

[`DustWalletOptions`](/api-reference/testkit-js/interfaces/DustWalletOptions.md) = `DEFAULT_DUST_OPTIONS`

#### Returns[​](#returns-1 "Direct link to Returns")

`DustWalletAPI`

***

### createShieldedWallet()[​](#createshieldedwallet "Direct link to createShieldedWallet()")

> `static` **createShieldedWallet**(`config`, `seed`): `ShieldedWalletAPI`

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### config[​](#config-1 "Direct link to config")

`DefaultV1Configuration`

##### seed[​](#seed-1 "Direct link to seed")

`Uint8Array`

#### Returns[​](#returns-2 "Direct link to Returns")

`ShieldedWalletAPI`

***

### createUnshieldedWallet()[​](#createunshieldedwallet "Direct link to createUnshieldedWallet()")

> `static` **createUnshieldedWallet**(`config`, `unshieldedKeystore`): `UnshieldedWalletAPI`

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### config[​](#config-2 "Direct link to config")

`DefaultV1Configuration`

##### unshieldedKeystore[​](#unshieldedkeystore "Direct link to unshieldedKeystore")

`UnshieldedKeystore`

#### Returns[​](#returns-3 "Direct link to Returns")

`UnshieldedWalletAPI`

***

### createWalletFacade()[​](#createwalletfacade "Direct link to createWalletFacade()")

> `static` **createWalletFacade**(`config`, `shieldedWallet`, `unshieldedWallet`, `dustWallet`): `Promise`<`WalletFacade`>

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### config[​](#config-3 "Direct link to config")

`DefaultConfiguration`

##### shieldedWallet[​](#shieldedwallet "Direct link to shieldedWallet")

`ShieldedWalletAPI`

##### unshieldedWallet[​](#unshieldedwallet "Direct link to unshieldedWallet")

`UnshieldedWalletAPI`

##### dustWallet[​](#dustwallet "Direct link to dustWallet")

`DustWalletAPI`

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<`WalletFacade`>

***

### restoreShieldedWallet()[​](#restoreshieldedwallet "Direct link to restoreShieldedWallet()")

> `static` **restoreShieldedWallet**(`config`, `serializedState`): `Promise`<`ShieldedWallet`>

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### config[​](#config-4 "Direct link to config")

`DefaultV1Configuration`

##### serializedState[​](#serializedstate "Direct link to serializedState")

`string`

#### Returns[​](#returns-5 "Direct link to Returns")

`Promise`<`ShieldedWallet`>

***

### startWalletFacade()[​](#startwalletfacade "Direct link to startWalletFacade()")

> `static` **startWalletFacade**(`wallet`, `shieldedSeed`, `dustSeed`): `Promise`<`WalletFacade`>

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### wallet[​](#wallet "Direct link to wallet")

`WalletFacade`

##### shieldedSeed[​](#shieldedseed "Direct link to shieldedSeed")

`Uint8Array`

##### dustSeed[​](#dustseed "Direct link to dustSeed")

`Uint8Array`

#### Returns[​](#returns-6 "Direct link to Returns")

`Promise`<`WalletFacade`>
