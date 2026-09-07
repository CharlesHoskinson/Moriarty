# DesiredOutput

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/dapp-connector-api v4.0.1**](/api-reference/dapp-connector.md)

***

[@midnight-ntwrk/dapp-connector-api](/api-reference/dapp-connector/globals.md) / DesiredOutput

# Type Alias: DesiredOutput

> **DesiredOutput** = `object`

Desired output from a transaction or intent. It specifies the type of the output, the amount and the recipient. Recipient needs to be a properly formatted Bech32m address matching the kind of the token and network id the wallet is connected to.

## Properties[​](#properties "Direct link to Properties")

### kind[​](#kind "Direct link to kind")

> **kind**: `"shielded"` | `"unshielded"`

***

### recipient[​](#recipient "Direct link to recipient")

> **recipient**: `string`

***

### type[​](#type "Direct link to type")

> **type**: [`TokenType`](/api-reference/dapp-connector/type-aliases/TokenType.md)

***

### value[​](#value "Direct link to value")

> **value**: `bigint`
