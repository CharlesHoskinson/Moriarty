# WalletConnectedAPI

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/dapp-connector-api v4.0.1**](/api-reference/dapp-connector.md)

***

[@midnight-ntwrk/dapp-connector-api](/api-reference/dapp-connector/globals.md) / WalletConnectedAPI

# Type Alias: WalletConnectedAPI

> **WalletConnectedAPI** = `object`

Wallet connected API. It is a subset of the Connected API defining all wallet-relevant methods. Full Connected API also implements [HintUsage](/api-reference/dapp-connector/type-aliases/HintUsage.md). The operations provided cover all necessary functionality for a DApp to interact with the wallet:

* getting balances and addresses
* submitting transactions
* creating and balancing transactions
* initializing intents (for swaps)
* signing data

## Methods[​](#methods "Direct link to Methods")

### balanceSealedTransaction()[​](#balancesealedtransaction "Direct link to balanceSealedTransaction()")

> **balanceSealedTransaction**(`tx`, `options?`): `Promise`<{ `tx`: `string`; }>

Take sealed transaction (with proofs, signatures and cryptographically bound), pay fees, add necessary inputs and outputs to remove imbalances from it, returning a transaction ready for submission

This method is mainly expected to be used by DApps when they operate on transactions created by the wallet or when the DApp wants to be sure that wallet performs balancing in a separate intent. In such case, it is important to remember that some contracts might make use of fallible sections, in which case wallet won't be able to properly balance the transaction. In such cases, the DApp should use [balanceUnsealedTransaction](#balanceunsealedtransaction) instead.

In relation to Ledger API (`@midnight-ntwrk/ledger-v<N>`), this method expects a serialized transaction of type `Transaction<SignatureEnabled, Proof, Binding>` Options: `payFees` - whether wallet should pay fees for the issued transaction or not, true by default

#### Parameters[​](#parameters "Direct link to Parameters")

##### tx[​](#tx "Direct link to tx")

`string`

##### options?[​](#options "Direct link to options?")

###### payFees?[​](#payfees "Direct link to payFees?")

`boolean`

#### Returns[​](#returns "Direct link to Returns")

`Promise`<{ `tx`: `string`; }>

***

### balanceUnsealedTransaction()[​](#balanceunsealedtransaction "Direct link to balanceUnsealedTransaction()")

> **balanceUnsealedTransaction**(`tx`, `options?`): `Promise`<{ `tx`: `string`; }>

Take unsealed transaction (with proofs, with no signatures and with preimage data for cryptographic binding), pay fees, add necessary inputs and outputs to remove imbalances from it, returning a transaction ready for submission

This method is expected to be used by DApps when interacting with contracts - in many cases when contracts interact with native tokens, where wallet may need to add inputs and outputs to an existing intent to properly balance the transaction.

In relation to Ledger API (`@midnight-ntwrk/ledger-v<N>`), this method expects a serialized transaction of type `Transaction<SignatureEnabled, Proof, PreBinding>` Options: `payFees` - whether wallet should pay fees for the issued transaction or not, true by default

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### tx[​](#tx-1 "Direct link to tx")

`string`

##### options?[​](#options-1 "Direct link to options?")

###### payFees?[​](#payfees-1 "Direct link to payFees?")

`boolean`

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<{ `tx`: `string`; }>

***

### getConfiguration()[​](#getconfiguration "Direct link to getConfiguration()")

> **getConfiguration**(): `Promise`<[`Configuration`](/api-reference/dapp-connector/type-aliases/Configuration.md)>

Get the configuration of the services used by the wallet.

It is important for DApps to make use of those services whenever possible, as the wallet user might have some preferences in this regard, which e.g. improve privacy or performance.

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<[`Configuration`](/api-reference/dapp-connector/type-aliases/Configuration.md)>

***

### getConnectionStatus()[​](#getconnectionstatus "Direct link to getConnectionStatus()")

> **getConnectionStatus**(): `Promise`<[`ConnectionStatus`](/api-reference/dapp-connector/type-aliases/ConnectionStatus.md)>

Status of an existing connection to wallet

DApps can use this method to check if the connection is still valid.

#### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<[`ConnectionStatus`](/api-reference/dapp-connector/type-aliases/ConnectionStatus.md)>

***

### getDustAddress()[​](#getdustaddress "Direct link to getDustAddress()")

> **getDustAddress**(): `Promise`<{ `dustAddress`: `string`; }>

Get the Dust address of the wallet. It is provided in Bech32m format.

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<{ `dustAddress`: `string`; }>

***

### getDustBalance()[​](#getdustbalance "Direct link to getDustBalance()")

> **getDustBalance**(): `Promise`<{ `balance`: `bigint`; `cap`: `bigint`; }>

Get the balance of Dust of the wallet. It reports both:

* the current balance (which may change over time due to generation mechanics)
* the cap (the maximum amount of Dust that can be generated from the current Night balance).

#### Returns[​](#returns-5 "Direct link to Returns")

`Promise`<{ `balance`: `bigint`; `cap`: `bigint`; }>

***

### getProvingProvider()[​](#getprovingprovider "Direct link to getProvingProvider()")

> **getProvingProvider**(`keyMaterialProvider`): `Promise`<[`ProvingProvider`](/api-reference/dapp-connector/type-aliases/ProvingProvider.md)>

Obtain the proving provider from the wallet to delegate proving to the wallet.

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### keyMaterialProvider[​](#keymaterialprovider "Direct link to keyMaterialProvider")

[`KeyMaterialProvider`](/api-reference/dapp-connector/type-aliases/KeyMaterialProvider.md)

object resolving prover and verifier keys, as well as the ZKIR representation of the circuit; `KeyMaterialProvider` is almost identical to the one in Midnight.js's `ZKConfigProvider` (<https://github.com/midnightntwrk/midnight-js/blob/main/packages/types/src/zk-config-provider.ts#L25>)

#### Returns[​](#returns-6 "Direct link to Returns")

`Promise`<[`ProvingProvider`](/api-reference/dapp-connector/type-aliases/ProvingProvider.md)>

A `ProvingProvider` instance, compatible with Ledger's ProvingProvider (<https://github.com/midnightntwrk/midnight-ledger/blob/main/ledger-wasm/ledger-v6.template.d.ts#L992>)

***

### getShieldedAddresses()[​](#getshieldedaddresses "Direct link to getShieldedAddresses()")

> **getShieldedAddresses**(): `Promise`<{ `shieldedAddress`: `string`; `shieldedCoinPublicKey`: `string`; `shieldedEncryptionPublicKey`: `string`; }>

Get the shielded addresses of the wallet. For convenience it also returns the coin public key and encryption public key. All of them are provided in Bech32m format.

#### Returns[​](#returns-7 "Direct link to Returns")

`Promise`<{ `shieldedAddress`: `string`; `shieldedCoinPublicKey`: `string`; `shieldedEncryptionPublicKey`: `string`; }>

***

### getShieldedBalances()[​](#getshieldedbalances "Direct link to getShieldedBalances()")

> **getShieldedBalances**(): `Promise`<`Record`<`string`, `bigint`>>

Get the balances of shielded tokens of the wallet. They are represented as a record, whose keys are token types.

#### Returns[​](#returns-8 "Direct link to Returns")

`Promise`<`Record`<`string`, `bigint`>>

***

### getTxHistory()[​](#gettxhistory "Direct link to getTxHistory()")

> **getTxHistory**(`pageNumber`, `pageSize`): `Promise`<[`HistoryEntry`](/api-reference/dapp-connector/type-aliases/HistoryEntry.md)\[]>

Get the history of transactions of the wallet. Each history entry is a simplistic record of the fact that a transaction is relevant to the wallet.

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### pageNumber[​](#pagenumber "Direct link to pageNumber")

`number`

##### pageSize[​](#pagesize "Direct link to pageSize")

`number`

#### Returns[​](#returns-9 "Direct link to Returns")

`Promise`<[`HistoryEntry`](/api-reference/dapp-connector/type-aliases/HistoryEntry.md)\[]>

***

### getUnshieldedAddress()[​](#getunshieldedaddress "Direct link to getUnshieldedAddress()")

> **getUnshieldedAddress**(): `Promise`<{ `unshieldedAddress`: `string`; }>

Get the unshielded address of the wallet. It is provided in Bech32m format.

#### Returns[​](#returns-10 "Direct link to Returns")

`Promise`<{ `unshieldedAddress`: `string`; }>

***

### getUnshieldedBalances()[​](#getunshieldedbalances "Direct link to getUnshieldedBalances()")

> **getUnshieldedBalances**(): `Promise`<`Record`<`string`, `bigint`>>

Get the balances of unshielded tokens (potentially including Night) of the wallet. They are represented as a record, whose keys are token types.

#### Returns[​](#returns-11 "Direct link to Returns")

`Promise`<`Record`<`string`, `bigint`>>

***

### makeIntent()[​](#makeintent "Direct link to makeIntent()")

> **makeIntent**(`desiredInputs`, `desiredOutputs`, `options`): `Promise`<{ `tx`: `string`; }>

Initialize a transaction with unbalanced intent containing desired inputs and outputs. Primary use-case for this method is to create a transaction, which inits a swap Options: `intentId` - what id use for created intent: use 1 to ensure no transaction merging will result in actions executed before created intent in the same transaction use specific number within ledger limitations to make the intent have that segment id assigned use "random" to allow wallet to pick one in random (e.g. when creating intent for swap purposes) `payFees` - whether wallet should pay fees for the issued transaction or not

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### desiredInputs[​](#desiredinputs "Direct link to desiredInputs")

[`DesiredInput`](/api-reference/dapp-connector/type-aliases/DesiredInput.md)\[]

##### desiredOutputs[​](#desiredoutputs "Direct link to desiredOutputs")

[`DesiredOutput`](/api-reference/dapp-connector/type-aliases/DesiredOutput.md)\[]

##### options[​](#options-2 "Direct link to options")

###### intentId[​](#intentid "Direct link to intentId")

`number` | `"random"`

###### payFees[​](#payfees-2 "Direct link to payFees")

`boolean`

#### Returns[​](#returns-12 "Direct link to Returns")

`Promise`<{ `tx`: `string`; }>

***

### makeTransfer()[​](#maketransfer "Direct link to makeTransfer()")

> **makeTransfer**(`desiredOutputs`, `options?`): `Promise`<{ `tx`: `string`; }>

Initialize a transfer transaction with desired outputs

Options: `payFees` - whether wallet should pay fees for the issued transaction or not, true by default

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### desiredOutputs[​](#desiredoutputs-1 "Direct link to desiredOutputs")

[`DesiredOutput`](/api-reference/dapp-connector/type-aliases/DesiredOutput.md)\[]

##### options?[​](#options-3 "Direct link to options?")

###### payFees?[​](#payfees-3 "Direct link to payFees?")

`boolean`

#### Returns[​](#returns-13 "Direct link to Returns")

`Promise`<{ `tx`: `string`; }>

***

### signData()[​](#signdata "Direct link to signData()")

> **signData**(`data`, `options`): `Promise`<[`Signature`](/api-reference/dapp-connector/type-aliases/Signature.md)>

Sign provided data using key and format specified in the options, data to sign will be prepended with right prefix

#### Parameters[​](#parameters-6 "Direct link to Parameters")

##### data[​](#data "Direct link to data")

`string`

##### options[​](#options-4 "Direct link to options")

[`SignDataOptions`](/api-reference/dapp-connector/type-aliases/SignDataOptions.md)

#### Returns[​](#returns-14 "Direct link to Returns")

`Promise`<[`Signature`](/api-reference/dapp-connector/type-aliases/Signature.md)>

***

### submitTransaction()[​](#submittransaction "Direct link to submitTransaction()")

> **submitTransaction**(`tx`): `Promise`<`void`>

Submit a transaction to the network, effectively using wallet as a relayer.

The transaction received is expected to be balanced and "sealed" - it means it contains proofs, signatures and cryptographically bound (`Transaction<SignatureEnabled, Proof, Binding>` type from `@midnight-ntwrk/ledger`)

#### Parameters[​](#parameters-7 "Direct link to Parameters")

##### tx[​](#tx-2 "Direct link to tx")

`string`

#### Returns[​](#returns-15 "Direct link to Returns")

`Promise`<`void`>
