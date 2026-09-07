# HintUsage

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/dapp-connector-api v4.0.1**](/api-reference/dapp-connector.md)

***

[@midnight-ntwrk/dapp-connector-api](/api-reference/dapp-connector/globals.md) / HintUsage

# Type Alias: HintUsage

> **HintUsage** = `object`

## Methods[​](#methods "Direct link to Methods")

### hintUsage()[​](#hintusage "Direct link to hintUsage()")

> **hintUsage**(`methodNames`): `Promise`<`void`>

Hint usage of methods to the wallet.

DApps should use this method to hint to the wallet what methods are expected to be used in a certain context (be it a whole session, single view, or a user flow - it is up to DApp). The wallet can use these calls as an opportunity to ask user for permissions and in such case - resolve the promise only after the user has granted the permissions.

#### Parameters[​](#parameters "Direct link to Parameters")

##### methodNames[​](#methodnames "Direct link to methodNames")

keyof [`WalletConnectedAPI`](/api-reference/dapp-connector/type-aliases/WalletConnectedAPI.md)\[]

#### Returns[​](#returns "Direct link to Returns")

`Promise`<`void`>
