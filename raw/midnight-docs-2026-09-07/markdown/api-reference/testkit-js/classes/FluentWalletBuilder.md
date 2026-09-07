# FluentWalletBuilder

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

## Methods[​](#methods "Direct link to Methods")

### build()[​](#build "Direct link to build()")

> **build**(): `Promise`<`WalletFacade`>

#### Returns[​](#returns "Direct link to Returns")

`Promise`<`WalletFacade`>

***

### buildWithoutStarting()[​](#buildwithoutstarting "Direct link to buildWithoutStarting()")

> **buildWithoutStarting**(): `Promise`<{ `keystore`: `UnshieldedKeystore`; `seeds`: [`WalletSeeds`](/api-reference/testkit-js/classes/WalletSeeds.md); `wallet`: `WalletFacade`; }>

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<{ `keystore`: `UnshieldedKeystore`; `seeds`: [`WalletSeeds`](/api-reference/testkit-js/classes/WalletSeeds.md); `wallet`: `WalletFacade`; }>

***

### withDustOptions()[​](#withdustoptions "Direct link to withDustOptions()")

> **withDustOptions**(`options`): `FluentWalletBuilder`

#### Parameters[​](#parameters "Direct link to Parameters")

##### options[​](#options "Direct link to options")

[`DustWalletOptions`](/api-reference/testkit-js/interfaces/DustWalletOptions.md)

#### Returns[​](#returns-2 "Direct link to Returns")

`FluentWalletBuilder`

***

### withMnemonic()[​](#withmnemonic "Direct link to withMnemonic()")

> **withMnemonic**(`mnemonic`): `FluentWalletBuilder`

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### mnemonic[​](#mnemonic "Direct link to mnemonic")

`string`

#### Returns[​](#returns-3 "Direct link to Returns")

`FluentWalletBuilder`

***

### withRandomSeed()[​](#withrandomseed "Direct link to withRandomSeed()")

> **withRandomSeed**(): `FluentWalletBuilder`

#### Returns[​](#returns-4 "Direct link to Returns")

`FluentWalletBuilder`

***

### withSeed()[​](#withseed "Direct link to withSeed()")

> **withSeed**(`seed`): `FluentWalletBuilder`

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### seed[​](#seed "Direct link to seed")

`string`

#### Returns[​](#returns-5 "Direct link to Returns")

`FluentWalletBuilder`

***

### withTestWallet()[​](#withtestwallet "Direct link to withTestWallet()")

> **withTestWallet**(): `FluentWalletBuilder`

#### Returns[​](#returns-6 "Direct link to Returns")

`FluentWalletBuilder`

***

### forEnvironment()[​](#forenvironment "Direct link to forEnvironment()")

> `static` **forEnvironment**(`envConfig`): `FluentWalletBuilder`

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### envConfig[​](#envconfig "Direct link to envConfig")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

#### Returns[​](#returns-7 "Direct link to Returns")

`FluentWalletBuilder`
