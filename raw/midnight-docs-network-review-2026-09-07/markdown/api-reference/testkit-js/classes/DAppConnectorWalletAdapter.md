# DAppConnectorWalletAdapter

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

## Implements[​](#implements "Direct link to Implements")

* `ConnectedAPI`

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new DAppConnectorWalletAdapter**(`walletProvider`, `environmentConfiguration`): `DAppConnectorWalletAdapter`

#### Parameters[​](#parameters "Direct link to Parameters")

##### walletProvider[​](#walletprovider "Direct link to walletProvider")

`Pick`<[`MidnightWalletProvider`](/api-reference/testkit-js/classes/MidnightWalletProvider.md), `"wallet"` | `"unshieldedKeystore"` | `"zswapSecretKeys"` | `"dustSecretKey"`>

##### environmentConfiguration[​](#environmentconfiguration "Direct link to environmentConfiguration")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

#### Returns[​](#returns "Direct link to Returns")

`DAppConnectorWalletAdapter`

## Methods[​](#methods "Direct link to Methods")

### balanceSealedTransaction()[​](#balancesealedtransaction "Direct link to balanceSealedTransaction()")

> **balanceSealedTransaction**(`tx`, `options?`): `Promise`<{ `tx`: `string`; }>

Take sealed transaction (with proofs, signatures and cryptographically bound), pay fees, add necessary inputs and outputs to remove imbalances from it, returning a transaction ready for submission

This method is mainly expected to be used by DApps when they operate on transactions created by the wallet or when the DApp wants to be sure that wallet performs balancing in a separate intent. In such case, it is important to remember that some contracts might make use of fallible sections, in which case wallet won't be able to properly balance the transaction. In such cases, the DApp should use [balanceUnsealedTransaction](#balanceunsealedtransaction) instead.

In relation to Ledger API (`@midnight-ntwrk/ledger-v<N>`), this method expects a serialized transaction of type `Transaction<SignatureEnabled, Proof, Binding>` Options: `payFees` - whether wallet should pay fees for the issued transaction or not, true by default

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### tx[​](#tx "Direct link to tx")

`string`

##### options?[​](#options "Direct link to options?")

###### payFees?[​](#payfees "Direct link to payFees?")

`boolean`

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<{ `tx`: `string`; }>

#### Implementation of[​](#implementation-of "Direct link to Implementation of")

`ConnectedAPI.balanceSealedTransaction`

***

### balanceUnsealedTransaction()[​](#balanceunsealedtransaction "Direct link to balanceUnsealedTransaction()")

> **balanceUnsealedTransaction**(`tx`, `options?`): `Promise`<{ `tx`: `string`; }>

Take unsealed transaction (with proofs, with no signatures and with preimage data for cryptographic binding), pay fees, add necessary inputs and outputs to remove imbalances from it, returning a transaction ready for submission

This method is expected to be used by DApps when interacting with contracts - in many cases when contracts interact with native tokens, where wallet may need to add inputs and outputs to an existing intent to properly balance the transaction.

In relation to Ledger API (`@midnight-ntwrk/ledger-v<N>`), this method expects a serialized transaction of type `Transaction<SignatureEnabled, Proof, PreBinding>` Options: `payFees` - whether wallet should pay fees for the issued transaction or not, true by default

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### tx[​](#tx-1 "Direct link to tx")

`string`

##### options?[​](#options-1 "Direct link to options?")

###### payFees?[​](#payfees-1 "Direct link to payFees?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<{ `tx`: `string`; }>

#### Implementation of[​](#implementation-of-1 "Direct link to Implementation of")

`ConnectedAPI.balanceUnsealedTransaction`

***

### getConfiguration()[​](#getconfiguration "Direct link to getConfiguration()")

> **getConfiguration**(): `Promise`<`Configuration`>

Get the configuration of the services used by the wallet.

It is important for DApps to make use of those services whenever possible, as the wallet user might have some preferences in this regard, which e.g. improve privacy or performance.

#### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<`Configuration`>

#### Implementation of[​](#implementation-of-2 "Direct link to Implementation of")

`ConnectedAPI.getConfiguration`

***

### getConnectionStatus()[​](#getconnectionstatus "Direct link to getConnectionStatus()")

> **getConnectionStatus**(): `Promise`<`ConnectionStatus`>

Status of an existing connection to wallet

DApps can use this method to check if the connection is still valid.

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<`ConnectionStatus`>

#### Implementation of[​](#implementation-of-3 "Direct link to Implementation of")

`ConnectedAPI.getConnectionStatus`

***

### getDustAddress()[​](#getdustaddress "Direct link to getDustAddress()")

> **getDustAddress**(): `Promise`<{ `dustAddress`: `string`; }>

Get the Dust address of the wallet. It is provided in Bech32m format.

#### Returns[​](#returns-5 "Direct link to Returns")

`Promise`<{ `dustAddress`: `string`; }>

#### Implementation of[​](#implementation-of-4 "Direct link to Implementation of")

`ConnectedAPI.getDustAddress`

***

### getDustBalance()[​](#getdustbalance "Direct link to getDustBalance()")

> **getDustBalance**(): `Promise`<{ `balance`: `bigint`; `cap`: `bigint`; }>

Get the balance of Dust of the wallet. It reports both:

* the current balance (which may change over time due to generation mechanics)
* the cap (the maximum amount of Dust that can be generated from the current Night balance).

#### Returns[​](#returns-6 "Direct link to Returns")

`Promise`<{ `balance`: `bigint`; `cap`: `bigint`; }>

#### Implementation of[​](#implementation-of-5 "Direct link to Implementation of")

`ConnectedAPI.getDustBalance`

***

### getProvingProvider()[​](#getprovingprovider "Direct link to getProvingProvider()")

> **getProvingProvider**(`keyMaterialProvider`): `Promise`<`ProvingProvider`>

Obtain the proving provider from the wallet to delegate proving to the wallet.

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### keyMaterialProvider[​](#keymaterialprovider "Direct link to keyMaterialProvider")

`KeyMaterialProvider`

object resolving prover and verifier keys, as well as the ZKIR representation of the circuit; `KeyMaterialProvider` is almost identical to the one in Midnight.js's `ZKConfigProvider` (<https://github.com/midnightntwrk/midnight-js/blob/main/packages/types/src/zk-config-provider.ts#L25>)

#### Returns[​](#returns-7 "Direct link to Returns")

`Promise`<`ProvingProvider`>

A `ProvingProvider` instance, compatible with Ledger's ProvingProvider (<https://github.com/midnightntwrk/midnight-ledger/blob/main/ledger-wasm/ledger-v6.template.d.ts#L992>)

#### Implementation of[​](#implementation-of-6 "Direct link to Implementation of")

`ConnectedAPI.getProvingProvider`

***

### getShieldedAddresses()[​](#getshieldedaddresses "Direct link to getShieldedAddresses()")

> **getShieldedAddresses**(): `Promise`<{ `shieldedAddress`: `string`; `shieldedCoinPublicKey`: `string`; `shieldedEncryptionPublicKey`: `string`; }>

Get the shielded addresses of the wallet. For convenience it also returns the coin public key and encryption public key. All of them are provided in Bech32m format.

#### Returns[​](#returns-8 "Direct link to Returns")

`Promise`<{ `shieldedAddress`: `string`; `shieldedCoinPublicKey`: `string`; `shieldedEncryptionPublicKey`: `string`; }>

#### Implementation of[​](#implementation-of-7 "Direct link to Implementation of")

`ConnectedAPI.getShieldedAddresses`

***

### getShieldedBalances()[​](#getshieldedbalances "Direct link to getShieldedBalances()")

> **getShieldedBalances**(): `Promise`<`Record`<`string`, `bigint`>>

Get the balances of shielded tokens of the wallet. They are represented as a record, whose keys are token types.

#### Returns[​](#returns-9 "Direct link to Returns")

`Promise`<`Record`<`string`, `bigint`>>

#### Implementation of[​](#implementation-of-8 "Direct link to Implementation of")

`ConnectedAPI.getShieldedBalances`

***

### getTxHistory()[​](#gettxhistory "Direct link to getTxHistory()")

> **getTxHistory**(`_pageNumber`, `_pageSize`): `Promise`<`HistoryEntry`\[]>

Get the history of transactions of the wallet. Each history entry is a simplistic record of the fact that a transaction is relevant to the wallet.

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### \_pageNumber[​](#_pagenumber "Direct link to _pageNumber")

`number`

##### \_pageSize[​](#_pagesize "Direct link to _pageSize")

`number`

#### Returns[​](#returns-10 "Direct link to Returns")

`Promise`<`HistoryEntry`\[]>

#### Implementation of[​](#implementation-of-9 "Direct link to Implementation of")

`ConnectedAPI.getTxHistory`

***

### getUnshieldedAddress()[​](#getunshieldedaddress "Direct link to getUnshieldedAddress()")

> **getUnshieldedAddress**(): `Promise`<{ `unshieldedAddress`: `string`; }>

Get the unshielded address of the wallet. It is provided in Bech32m format.

#### Returns[​](#returns-11 "Direct link to Returns")

`Promise`<{ `unshieldedAddress`: `string`; }>

#### Implementation of[​](#implementation-of-10 "Direct link to Implementation of")

`ConnectedAPI.getUnshieldedAddress`

***

### getUnshieldedBalances()[​](#getunshieldedbalances "Direct link to getUnshieldedBalances()")

> **getUnshieldedBalances**(): `Promise`<`Record`<`string`, `bigint`>>

Get the balances of unshielded tokens (potentially including Night) of the wallet. They are represented as a record, whose keys are token types.

#### Returns[​](#returns-12 "Direct link to Returns")

`Promise`<`Record`<`string`, `bigint`>>

#### Implementation of[​](#implementation-of-11 "Direct link to Implementation of")

`ConnectedAPI.getUnshieldedBalances`

***

### hintUsage()[​](#hintusage "Direct link to hintUsage()")

> **hintUsage**(`_methodNames`): `Promise`<`void`>

Hint usage of methods to the wallet.

DApps should use this method to hint to the wallet what methods are expected to be used in a certain context (be it a whole session, single view, or a user flow - it is up to DApp). The wallet can use these calls as an opportunity to ask user for permissions and in such case - resolve the promise only after the user has granted the permissions.

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### \_methodNames[​](#_methodnames "Direct link to _methodNames")

keyof `WalletConnectedAPI`\[]

#### Returns[​](#returns-13 "Direct link to Returns")

`Promise`<`void`>

#### Implementation of[​](#implementation-of-12 "Direct link to Implementation of")

`ConnectedAPI.hintUsage`

***

### makeIntent()[​](#makeintent "Direct link to makeIntent()")

> **makeIntent**(`_desiredInputs`, `_desiredOutputs`, `_options`): `Promise`<{ `tx`: `string`; }>

Initialize a transaction with unbalanced intent containing desired inputs and outputs. Primary use-case for this method is to create a transaction, which inits a swap Options: `intentId` - what id use for created intent: use 1 to ensure no transaction merging will result in actions executed before created intent in the same transaction use specific number within ledger limitations to make the intent have that segment id assigned use "random" to allow wallet to pick one in random (e.g. when creating intent for swap purposes) `payFees` - whether wallet should pay fees for the issued transaction or not

#### Parameters[​](#parameters-6 "Direct link to Parameters")

##### \_desiredInputs[​](#_desiredinputs "Direct link to _desiredInputs")

`DesiredInput`\[]

##### \_desiredOutputs[​](#_desiredoutputs "Direct link to _desiredOutputs")

`DesiredOutput`\[]

##### \_options[​](#_options "Direct link to _options")

###### intentId[​](#intentid "Direct link to intentId")

`number` | `"random"`

###### payFees[​](#payfees-2 "Direct link to payFees")

`boolean`

#### Returns[​](#returns-14 "Direct link to Returns")

`Promise`<{ `tx`: `string`; }>

#### Implementation of[​](#implementation-of-13 "Direct link to Implementation of")

`ConnectedAPI.makeIntent`

***

### makeTransfer()[​](#maketransfer "Direct link to makeTransfer()")

> **makeTransfer**(`_desiredOutputs`, `_options?`): `Promise`<{ `tx`: `string`; }>

Initialize a transfer transaction with desired outputs

Options: `payFees` - whether wallet should pay fees for the issued transaction or not, true by default

#### Parameters[​](#parameters-7 "Direct link to Parameters")

##### \_desiredOutputs[​](#_desiredoutputs-1 "Direct link to _desiredOutputs")

`DesiredOutput`\[]

##### \_options?[​](#_options-1 "Direct link to _options?")

###### payFees?[​](#payfees-3 "Direct link to payFees?")

`boolean`

#### Returns[​](#returns-15 "Direct link to Returns")

`Promise`<{ `tx`: `string`; }>

#### Implementation of[​](#implementation-of-14 "Direct link to Implementation of")

`ConnectedAPI.makeTransfer`

***

### signData()[​](#signdata "Direct link to signData()")

> **signData**(`data`, `options`): `Promise`<`Signature`>

Sign provided data using key and format specified in the options, data to sign will be prepended with right prefix

#### Parameters[​](#parameters-8 "Direct link to Parameters")

##### data[​](#data "Direct link to data")

`string`

##### options[​](#options-2 "Direct link to options")

`SignDataOptions`

#### Returns[​](#returns-16 "Direct link to Returns")

`Promise`<`Signature`>

#### Implementation of[​](#implementation-of-15 "Direct link to Implementation of")

`ConnectedAPI.signData`

***

### submitTransaction()[​](#submittransaction "Direct link to submitTransaction()")

> **submitTransaction**(`tx`): `Promise`<`void`>

Submit a transaction to the network, effectively using wallet as a relayer.

The transaction received is expected to be balanced and "sealed" - it means it contains proofs, signatures and cryptographically bound (`Transaction<SignatureEnabled, Proof, Binding>` type from `@midnight-ntwrk/ledger`)

#### Parameters[​](#parameters-9 "Direct link to Parameters")

##### tx[​](#tx-2 "Direct link to tx")

`string`

#### Returns[​](#returns-17 "Direct link to Returns")

`Promise`<`void`>

#### Implementation of[​](#implementation-of-16 "Direct link to Implementation of")

`ConnectedAPI.submitTransaction`
