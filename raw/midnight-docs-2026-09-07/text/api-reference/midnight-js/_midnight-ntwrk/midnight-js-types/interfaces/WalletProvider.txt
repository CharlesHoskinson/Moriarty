# WalletProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / WalletProvider

# Interface: WalletProvider

Interface representing a WalletProvider that handles operations such as transaction balancing and finalization, and provides access to cryptographic secret keys.

## Methods[​](#methods "Direct link to Methods")

### balanceTx()[​](#balancetx "Direct link to balanceTx()")

> **balanceTx**(`tx`, `ttl?`): `Promise`<`FinalizedTransaction`>

Balances a transaction

#### Parameters[​](#parameters "Direct link to Parameters")

##### tx[​](#tx "Direct link to tx")

[`UnboundTransaction`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnboundTransaction.md)

The transaction to balance.

##### ttl?[​](#ttl "Direct link to ttl?")

`Date`

#### Returns[​](#returns "Direct link to Returns")

`Promise`<`FinalizedTransaction`>

***

### getCoinPublicKey()[​](#getcoinpublickey "Direct link to getCoinPublicKey()")

> **getCoinPublicKey**(): `string`

#### Returns[​](#returns-1 "Direct link to Returns")

`string`

***

### getEncryptionPublicKey()[​](#getencryptionpublickey "Direct link to getEncryptionPublicKey()")

> **getEncryptionPublicKey**(): `string`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`
