# DAppConnectorInitialAPI

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

## Implements[​](#implements "Direct link to Implements")

* `InitialAPI`

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new DAppConnectorInitialAPI**(`connectedWallet`, `networkId`, `options?`): `DAppConnectorInitialAPI`

#### Parameters[​](#parameters "Direct link to Parameters")

##### connectedWallet[​](#connectedwallet "Direct link to connectedWallet")

`ConnectedAPI`

##### networkId[​](#networkid "Direct link to networkId")

`string`

##### options?[​](#options "Direct link to options?")

###### apiVersion?[​](#apiversion "Direct link to apiVersion?")

`string`

###### icon?[​](#icon "Direct link to icon?")

`string`

###### name?[​](#name "Direct link to name?")

`string`

###### rdns?[​](#rdns "Direct link to rdns?")

`string`

#### Returns[​](#returns "Direct link to Returns")

`DAppConnectorInitialAPI`

## Properties[​](#properties "Direct link to Properties")

### apiVersion[​](#apiversion-1 "Direct link to apiVersion")

> `readonly` **apiVersion**: `string`

Version of the API implemented by this instance of the API, string containing a version of the API package @midnight-ntwrk/dapp-connector-api that was used in implementation E.g. wallet implementing version 3.1.5 provides apiVersion with value '3.1.5' This value lets DApps to differentiate between different versions of the API and implement appropriate logic for each version or not use some versions at all

#### Implementation of[​](#implementation-of "Direct link to Implementation of")

`InitialAPI.apiVersion`

***

### icon[​](#icon-1 "Direct link to icon")

> `readonly` **icon**: `string`

Wallet icon, as an URL, either reference to a hosted resource, or a base64 encoded data URL. It is expected to be displayed to the user. Because of this, DApps need to display the icon in a secure fashion to prevent XSS. For example, displaying the icon using an `img` tag.

#### Implementation of[​](#implementation-of-1 "Direct link to Implementation of")

`InitialAPI.icon`

***

### name[​](#name-1 "Direct link to name")

> `readonly` **name**: `string`

Wallet name, expected to be displayed to the user. As such, DApps need to sanitize the name to prevent XSS when displaying it to the user. An example of sanitization is displaying the name using a text node.

#### Implementation of[​](#implementation-of-2 "Direct link to Implementation of")

`InitialAPI.name`

***

### rdns[​](#rdns-1 "Direct link to rdns")

> `readonly` **rdns**: `string`

Wallet identifier, in a reverse DNS notation (e.g. `com.example.wallet`). Wallets should keep this identifier stable throughout the lifecycle of the product. DApps can use this property to identify the wallet, but should be prepared to handle values that are unknown, invalid, or potentially misleading, similar to handling user agent strings in web browsers.

#### Implementation of[​](#implementation-of-3 "Direct link to Implementation of")

`InitialAPI.rdns`

## Methods[​](#methods "Direct link to Methods")

### connect()[​](#connect "Direct link to connect()")

> **connect**(`networkId`): `Promise`<`ConnectedAPI`>

Connect to wallet, hinting desired network id; Use 'mainnet' for mainnet.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### networkId[​](#networkid-1 "Direct link to networkId")

`string`

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`ConnectedAPI`>

#### Implementation of[​](#implementation-of-4 "Direct link to Implementation of")

`InitialAPI.connect`
