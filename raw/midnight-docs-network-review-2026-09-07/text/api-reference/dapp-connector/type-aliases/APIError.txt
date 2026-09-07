# APIError

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/dapp-connector-api v4.0.1**](/api-reference/dapp-connector.md)

***

[@midnight-ntwrk/dapp-connector-api](/api-reference/dapp-connector/globals.md) / APIError

# Type Alias: APIError

> **APIError** = `Error` & `object`

Declaration of the error type thrown by the DApp Connector.

It is not a class extending the base `Error` type, because it would make it difficult to implement in a way where `instanceof APIError` would work. Instead a check like `error.type === 'DAppConnectorAPIError'` should be used.

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### code[​](#code "Direct link to code")

> **code**: [`ErrorCode`](/api-reference/dapp-connector/type-aliases/ErrorCode.md)

The code of the error that's thrown

### reason[​](#reason "Direct link to reason")

> **reason**: `string`

The reason the error is thrown

### type[​](#type "Direct link to type")

> **type**: `"DAppConnectorAPIError"`

indication it is a DApp Connector Error
