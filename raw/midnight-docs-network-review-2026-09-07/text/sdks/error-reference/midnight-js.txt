> For the complete documentation index, see [llms.txt](/llms.txt)

# Midnight.js error reference

The Midnight.js API exposes structured error classes so you can classify failures when working with contracts and transactions.

This reference highlights the error classes and their properties.

## Quick reference[​](#quick-reference "Direct link to Quick reference")

| Class                                      | Kind             | Trigger                                                             |
| ------------------------------------------ | ---------------- | ------------------------------------------------------------------- |
| `TxFailedError`                            | Transaction      | Consensus rejected or failed to apply a transaction                 |
| `DeployTxFailedError`                      | Transaction      | Deploy transaction failed                                           |
| `CallTxFailedError`                        | Transaction      | Call transaction failed                                             |
| `ReplaceMaintenanceAuthorityTxFailedError` | Transaction      | Maintenance authority replacement failed                            |
| `RemoveVerifierKeyTxFailedError`           | Transaction      | Verifier key removal failed                                         |
| `InsertVerifierKeyTxFailedError`           | Transaction      | Verifier key insertion failed                                       |
| `ContractTypeError`                        | Contract         | Deployed contract type or verifier keys do not match expectations   |
| `IncompleteCallTxPrivateStateConfig`       | Configuration    | `privateStateId` set without `privateStateProvider` on a call       |
| `IncompleteFindContractPrivateStateConfig` | Configuration    | `initialPrivateState` set without `privateStateId` on a find        |
| `ScopedTransactionIdentityMismatchError`   | State / identity | Scoped batch reused cache for a different contract or private state |

## Transaction errors[​](#transaction-errors "Direct link to Transaction errors")

These errors occur when a submitted transaction fails during execution or validation.

### Transaction failed (`TxFailedError`)[​](#transaction-failed-txfailederror "Direct link to transaction-failed-txfailederror")

An error indicating that a transaction submitted to a consensus node failed.

#### Properties[​](#properties "Direct link to Properties")

* `finalizedTxData`: Finalization data of the failed transaction
* `circuitId` (optional): Circuit(s) used to construct the transaction

### Deploy transaction failed (`DeployTxFailedError`)[​](#deploy-transaction-failed-deploytxfailederror "Direct link to deploy-transaction-failed-deploytxfailederror")

An error indicating that a contract deployment transaction failed.

#### Properties[​](#properties-1 "Direct link to Properties")

`finalizedTxData`: Finalization data of the failed deployment transaction.

### Call transaction failed (`CallTxFailedError`)[​](#call-transaction-failed-calltxfailederror "Direct link to call-transaction-failed-calltxfailederror")

An error indicating that a contract call transaction failed.

#### Properties[​](#properties-2 "Direct link to Properties")

* `finalizedTxData`: Finalization data of the failed call transaction
* `circuitId`: Circuit(s) used to build the transaction

### Replace maintenance authority transaction failed (`ReplaceMaintenanceAuthorityTxFailedError`)[​](#replace-maintenance-authority-transaction-failed-replacemaintenanceauthoritytxfailederror "Direct link to replace-maintenance-authority-transaction-failed-replacemaintenanceauthoritytxfailederror")

An error indicating that a maintenance authority replacement transaction failed.

#### Properties[​](#properties-3 "Direct link to Properties")

`finalizedTxData`: Finalization data of the failed maintenance authority replacement transaction.

### Remove verifier key transaction failed (`RemoveVerifierKeyTxFailedError`)[​](#remove-verifier-key-transaction-failed-removeverifierkeytxfailederror "Direct link to remove-verifier-key-transaction-failed-removeverifierkeytxfailederror")

An error indicating that a verifier key removal transaction failed.

#### Properties[​](#properties-4 "Direct link to Properties")

`finalizedTxData`: Finalization data of the failed verifier key removal transaction.

### Insert verifier key transaction failed (`InsertVerifierKeyTxFailedError`)[​](#insert-verifier-key-transaction-failed-insertverifierkeytxfailederror "Direct link to insert-verifier-key-transaction-failed-insertverifierkeytxfailederror")

An error indicating that a verifier key insertion transaction failed.

#### Properties[​](#properties-5 "Direct link to Properties")

`finalizedTxData`: Finalization data of the failed verifier key insertion transaction.

## Contract errors[​](#contract-errors "Direct link to Contract errors")

These errors occur when interacting with deployed contracts.

### Contract type error (`ContractTypeError`)[​](#contract-type-error-contracttypeerror "Direct link to contract-type-error-contracttypeerror")

An error indicating that the expected contract type does not match the deployed contract state.

#### Properties[​](#properties-6 "Direct link to Properties")

* `contractState`: The deployed contract state
* `circuitIds`: Undefined circuits or circuits with mismatched verifier keys

## Configuration errors[​](#configuration-errors "Direct link to Configuration errors")

These errors indicate invalid or incomplete configuration when building transactions or querying contracts.

### Incomplete call transaction private state config (`IncompleteCallTxPrivateStateConfig`)[​](#incomplete-call-transaction-private-state-config-incompletecalltxprivatestateconfig "Direct link to incomplete-call-transaction-private-state-config-incompletecalltxprivatestateconfig")

An error indicating that a call transaction specifies a `privateStateId` but does not provide a `privateStateProvider`.

#### Properties[​](#properties-7 "Direct link to Properties")

* `privateStateId`: The private state ID specified in the call transaction
* `privateStateProvider`: The private state provider specified in the call transaction

### Incomplete find contract private state config (`IncompleteFindContractPrivateStateConfig`)[​](#incomplete-find-contract-private-state-config-incompletefindcontractprivatestateconfig "Direct link to incomplete-find-contract-private-state-config-incompletefindcontractprivatestateconfig")

An error indicating that a contract lookup specifies an `initialPrivateState` but does not include a `privateStateId`.

#### Properties[​](#properties-8 "Direct link to Properties")

* `initialPrivateState`: The initial private state specified in the contract lookup
* `privateStateId`: The private state ID specified in the contract lookup

#### Message[​](#message "Direct link to Message")

Here is the exact runtime message the client throws when this error occurs:

```
'initialPrivateState' was defined for contract find while 'privateStateId' was undefined
```

## State and identity errors[​](#state-and-identity-errors "Direct link to State and identity errors")

These errors help prevent subtle bugs related to contract identity and cached state.

### Scoped transaction identity mismatch (`ScopedTransactionIdentityMismatchError`)[​](#scoped-transaction-identity-mismatch-scopedtransactionidentitymismatcherror "Direct link to scoped-transaction-identity-mismatch-scopedtransactionidentitymismatcherror")

An error indicating that a scoped transaction attempts to reuse cached state with a different contract address or private state ID.

#### Properties[​](#properties-9 "Direct link to Properties")

* `cached`: The cached contract address and private state ID
* `requested`: The requested contract address and private state ID

## Error handling[​](#error-handling "Direct link to Error handling")

Use JavaScript's `try/catch` syntax and the `instanceof` operator to handle errors:

```
import { CallTxFailedError } from '@midnight-ntwrk/midnight-js-contracts';



try {

  await client.call(/* ... */);

} catch (error) {

  if (error instanceof CallTxFailedError) {

    console.error('Transaction failed:', error.message);

  } else {

    console.error('Unexpected error:', error);

  }

}
```
