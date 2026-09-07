# verifyContractState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / verifyContractState

# Function: verifyContractState()

> **verifyContractState**(`verifierKeys`, `contractState`): `void`

Checks that the given `contractState` contains the given `verifierKeys`.

## Parameters[​](#parameters "Direct link to Parameters")

### verifierKeys[​](#verifierkeys "Direct link to verifierKeys")

\[`string`, `VerifierKey`]\[]

The verifier keys the client has for the deployed contract we're checking.

### contractState[​](#contractstate "Direct link to contractState")

`ContractState`

The (typically already deployed) contract state containing verifier keys.

## Returns[​](#returns "Direct link to Returns")

`void`

## Throws[​](#throws "Direct link to Throws")

ContractTypeError When one or more of the local and deployed verifier keys do not match.
