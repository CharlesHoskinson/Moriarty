# ErrorCodes

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/dapp-connector-api v4.0.1**](/api-reference/dapp-connector.md)

***

[@midnight-ntwrk/dapp-connector-api](/api-reference/dapp-connector/globals.md) / ErrorCodes

# Variable: ErrorCodes

> `const` **ErrorCodes**: `object`

All possible error codes gathered in a single object.

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### Disconnected[​](#disconnected "Direct link to Disconnected")

> `readonly` **Disconnected**: `"Disconnected"` = `'Disconnected'`

The connection to the wallet was lost

### InternalError[​](#internalerror "Direct link to InternalError")

> `readonly` **InternalError**: `"InternalError"` = `'InternalError'`

The dapp connector wasn't able to process the request

### InvalidRequest[​](#invalidrequest "Direct link to InvalidRequest")

> `readonly` **InvalidRequest**: `"InvalidRequest"` = `'InvalidRequest'`

Can be thrown in various circumstances, e.g. one being a malformed transaction

### PermissionRejected[​](#permissionrejected "Direct link to PermissionRejected")

> `readonly` **PermissionRejected**: `"PermissionRejected"` = `'PermissionRejected'`

Permission to perform action was rejected.

### Rejected[​](#rejected "Direct link to Rejected")

> `readonly` **Rejected**: `"Rejected"` = `'Rejected'`

The user rejected the request
