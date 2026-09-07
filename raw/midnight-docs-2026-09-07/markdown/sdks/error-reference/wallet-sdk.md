> For the complete documentation index, see [llms.txt](/llms.txt)

# Wallet SDK error reference

All errors in the Midnight wallet SDK are Effect `Data.TaggedError` instances unless noted otherwise.

To catch errors, use `Effect.catchTag` with the `_tag` field shown for each error. For union types, use `Effect.catchTags`.

This reference highlights the error classes with details on how to catch and fix them.

## Node client (`@midnight-ntwrk/wallet-sdk-node-client`)[​](#node-client-midnight-ntwrkwallet-sdk-node-client "Direct link to node-client-midnight-ntwrkwallet-sdk-node-client")

These seven errors form the `NodeClientError` union type.

### `SubmissionError`[​](#submissionerror "Direct link to submissionerror")

An error indicating that transaction submission to the node failed.

| Field  | Value                                                   |
| ------ | ------------------------------------------------------- |
| `_tag` | `'SubmissionError'`                                     |
| Fields | `message: string`, `txData: unknown`, `cause?: unknown` |

**Fix**: Inspect `message` and `cause` for the underlying reason. Check node connectivity and that the transaction format is correct.

```
Effect.catchTag('SubmissionError', (e) => ...)
```

### `ConnectionError`[​](#connectionerror "Direct link to connectionerror")

An error indicating a failed node connection through a WebSocket.

| Field  | Value               |
| ------ | ------------------- |
| `_tag` | `'ConnectionError'` |

**Known messages**:

* `"Could not connect within specified time range (5s)"`: Node is unreachable or slow to respond
* `"Failed to retrieve genesis transactions"`: Connected but genesis data unavailable

**Fix**: Verify the node WebSocket URL is correct and the node is running. Increase connection timeout if the node is on a slow network.

```
Effect.catchTag('ConnectionError', (e) => ...)
```

### `TransactionProgressError`[​](#transactionprogresserror "Direct link to transactionprogresserror")

An error indicating that a submitted transaction did not reach the desired lifecycle stage within the expected time.

| Field  | Value                        |
| ------ | ---------------------------- |
| `_tag` | `'TransactionProgressError'` |

**Known messages**: `"Transaction did not reach finality within expected time"`

**Fix**: Check network congestion and node health. The transaction may still be in the mempool — query its status before resubmitting.

```
Effect.catchTag('TransactionProgressError', (e) => ...)
```

### `ParseError`[​](#parseerror "Direct link to parseerror")

An error indicating that the SDK could not parse a result returned by the node.

| Field  | Value          |
| ------ | -------------- |
| `_tag` | `'ParseError'` |

**Fix**: Usually indicates a protocol version mismatch between the SDK and the node. Check that SDK and node versions are compatible.

```
Effect.catchTag('ParseError', (e) => ...)
```

### `TransactionUsurpedError`[​](#transactionusurpederror "Direct link to transactionusurpederror")

An error indicating that another transaction replaced a transaction with the same discriminators.

| Field  | Value                       |
| ------ | --------------------------- |
| `_tag` | `'TransactionUsurpedError'` |

**Fix**: The original transaction is no longer relevant. If an unexpected replacement occurred, then investigate which process submitted a conflicting transaction.

```
Effect.catchTag('TransactionUsurpedError', (e) => ...)
```

### `TransactionDroppedError`[​](#transactiondroppederror "Direct link to transactiondroppederror")

An error indicating that the node dropped a transaction, most likely because the mempool is full.

| Field  | Value                       |
| ------ | --------------------------- |
| `_tag` | `'TransactionDroppedError'` |

**Fix**: Wait for the mempool to drain and resubmit. Consider increasing the transaction fee if the network supports fee prioritization.

```
Effect.catchTag('TransactionDroppedError', (e) => ...)
```

### `TransactionInvalidError`[​](#transactioninvaliderror "Direct link to transactioninvaliderror")

An error indicating that the node rejected a transaction as invalid.

| Field  | Value                       |
| ------ | --------------------------- |
| `_tag` | `'TransactionInvalidError'` |

**Fix**: A malformed transaction or a validation-rule violation caused this error. Review the transaction construction logic. Do not resubmit without changes.

```
Effect.catchTag('TransactionInvalidError', (e) => ...)
```

## Shielded wallet (`@midnight-ntwrk/wallet-sdk-shielded`)[​](#shielded-wallet-midnight-ntwrkwallet-sdk-shielded "Direct link to shielded-wallet-midnight-ntwrkwallet-sdk-shielded")

The shielded wallet exposes 8 error types:

### `OtherWalletError`[​](#otherwalleterror "Direct link to otherwalleterror")

Catch-all for wallet errors that do not fit a more specific category.

| Field  | Value            |
| ------ | ---------------- |
| `_tag` | `'Wallet.Other'` |

**Fix**: Inspect the error details. If this surfaces in production, then consider opening an issue with a reproduction case.

```
Effect.catchTag('Wallet.Other', (e) => ...)
```

### `SyncWalletError`[​](#syncwalleterror "Direct link to syncwalleterror")

An error indicating that a wallet sync with the Midnight blockchain failed.

| Field  | Value           |
| ------ | --------------- |
| `_tag` | `'Wallet.Sync'` |

**Fix**: Check connectivity to the node. Retry the sync operation. Persistent failures may indicate a corrupted local state.

```
Effect.catchTag('Wallet.Sync', (e) => ...)
```

### `SubmissionWalletError`[​](#submissionwalleterror "Direct link to submissionwalleterror")

A wrapper around submission errors that occur at the wallet layer (distinct from the node-client `SubmissionError`).

| Field  | Value                            |
| ------ | -------------------------------- |
| `_tag` | `'Wallet.SubmissionWalletError'` |

**Fix**: Unwrap and inspect the underlying cause. Usually delegates to the node-client submission path.

```
Effect.catchTag('Wallet.SubmissionWalletError', (e) => ...)
```

### `InsufficientFundsError`[​](#insufficientfundserror "Direct link to insufficientfundserror")

An error indicating that the wallet does not hold enough tokens of the specified type to complete the operation.

| Field  | Value                                 |
| ------ | ------------------------------------- |
| `_tag` | `'Wallet.InsufficientFunds'`          |
| Fields | `tokenType: string`, `amount: bigint` |

**Fix**: Check the wallet balance for `tokenType` before constructing the transaction. Request a top-up or reduce the transfer amount.

```
Effect.catchTag('Wallet.InsufficientFunds', (e) => {

  console.log(`Need more ${e.tokenType}, shortfall: ${e.amount}`)

})
```

### `AddressError`[​](#addresserror "Direct link to addresserror")

An error indicating that the provided address is invalid.

| Field  | Value                     |
| ------ | ------------------------- |
| `_tag` | `'Wallet.Address'`        |
| Fields | `originalAddress: string` |

**Fix**: Validate the address format before use. To understand the address format, see the [Address encoding](/sdks/official/wallet-developer-guide.md#address-encoding) section of the wallet SDK guide. The `originalAddress` field contains the rejected input.

```
Effect.catchTag('Wallet.Address', (e) => ...)
```

### `InvalidCoinHashesError`[​](#invalidcoinhasheserror "Direct link to invalidcoinhasheserror")

An error indicating that one or more coins are missing their required nonce hashes.

| Field  | Value                        |
| ------ | ---------------------------- |
| `_tag` | `'Wallet.InvalidCoinHashes'` |
| Fields | `missingNonces: unknown[]`   |

**Fix**: Ensure coins are fully synced before spending. The `missingNonces` field identifies the affected coins.

```
Effect.catchTag('Wallet.InvalidCoinHashes', (e) => ...)
```

### `TransactingError`[​](#transactingerror "Direct link to transactingerror")

Error during transaction construction or fee balancing.

| Field  | Value                  |
| ------ | ---------------------- |
| `_tag` | `'Wallet.Transacting'` |

**Fix**: Check that inputs are valid and that the fee token balance is sufficient. Review transaction parameters.

```
Effect.catchTag('Wallet.Transacting', (e) => ...)
```

### `TransactionHistoryError`[​](#transactionhistoryerror "Direct link to transactionhistoryerror")

Error reading or writing the transaction history store.

| Field  | Value                         |
| ------ | ----------------------------- |
| `_tag` | `'Wallet.TransactionHistory'` |

**Fix**: Check local storage availability and permissions. A corrupt store history might be causing the error. Clearing and resyncing is a recovery option.

```
Effect.catchTag('Wallet.TransactionHistory', (e) => ...)
```

## Unshielded wallet (`@midnight-ntwrk/wallet-sdk-unshielded-wallet`)[​](#unshielded-wallet-midnight-ntwrkwallet-sdk-unshielded-wallet "Direct link to unshielded-wallet-midnight-ntwrkwallet-sdk-unshielded-wallet")

The unshielded wallet exposes eleven error types: all eight from the shielded wallet (same `_tag` values) plus five additional types below.

### `SignError`[​](#signerror "Direct link to signerror")

Failed to sign a transaction.

| Field  | Value           |
| ------ | --------------- |
| `_tag` | `'Wallet.Sign'` |

**Fix**: Ensure the signing key is available and not locked. Check hardware wallet connectivity if applicable.

```
Effect.catchTag('Wallet.Sign', (e) => ...)
```

### `ApplyTransactionError`[​](#applytransactionerror "Direct link to applytransactionerror")

Failed to apply a transaction to the local UTXO set.

| Field  | Value                       |
| ------ | --------------------------- |
| `_tag` | `'Wallet.ApplyTransaction'` |

**Fix**: Usually follows a submission error. Verify that the node accepted the transaction before attempting to apply it locally.

```
Effect.catchTag('Wallet.ApplyTransaction', (e) => ...)
```

### `RollbackUtxoError`[​](#rollbackutxoerror "Direct link to rollbackutxoerror")

Failed to roll back a UTXO during a chain reorganisation.

| Field  | Value                   |
| ------ | ----------------------- |
| `_tag` | `'Wallet.RollbackUtxo'` |

**Fix**: If rollback fails repeatedly, then the local UTXO state might be inconsistent. If that happens, then you might need a full resync.

```
Effect.catchTag('Wallet.RollbackUtxo', (e) => ...)
```

### `SpendUtxoError`[​](#spendutxoerror "Direct link to spendutxoerror")

Failed to mark a UTXO as spent.

| Field  | Value                |
| ------ | -------------------- |
| `_tag` | `'Wallet.SpendUtxo'` |

**Fix**: The UTXO may already be spent or not present in the local set. To reconcile state, resync the wallet.

```
Effect.catchTag('Wallet.SpendUtxo', (e) => ...)
```

### `UtxoNotFoundError`[​](#utxonotfounderror "Direct link to utxonotfounderror")

A referenced UTXO could not be found in the local set.

| Field  | Value                 |
| ------ | --------------------- |
| `_tag` | `'UtxoNotFoundError'` |

**Fix**: Ensure the wallet is fully synced. The UTXO might have already been spent or the sync might be behind the chain tip.

```
Effect.catchTag('UtxoNotFoundError', (e) => ...)
```

## DUST wallet (`@midnight-ntwrk/wallet-sdk-dust-wallet`)[​](#dust-wallet-midnight-ntwrkwallet-sdk-dust-wallet "Direct link to dust-wallet-midnight-ntwrkwallet-sdk-dust-wallet")

The DUST wallet exposes four error types, all shared with the shielded wallet:

| Error                    | `_tag`                       |
| ------------------------ | ---------------------------- |
| `OtherWalletError`       | `'Wallet.Other'`             |
| `SyncWalletError`        | `'Wallet.Sync'`              |
| `TransactingError`       | `'Wallet.Transacting'`       |
| `InsufficientFundsError` | `'Wallet.InsufficientFunds'` |

For field details and fixes, see the [Shielded wallet](#shielded-wallet-midnight-ntwrkwallet-sdk-shielded) section.

## Capabilities (`@midnight-ntwrk/wallet-sdk-capabilities`)[​](#capabilities-midnight-ntwrkwallet-sdk-capabilities "Direct link to capabilities-midnight-ntwrkwallet-sdk-capabilities")

The capabilities package exposes three error types.

### `ProvingError`[​](#provingerror "Direct link to provingerror")

Wraps errors from the proving provider.

| Field  | Value              |
| ------ | ------------------ |
| `_tag` | `'Wallet.Proving'` |

**Fix**: Check proof server connectivity and that you are loading the correct circuit keys. Inspect the wrapped cause for the underlying provider error.

```
Effect.catchTag('Wallet.Proving', (e) => ...)
```

### `SubmissionError` (Capabilities)[​](#submissionerror-capabilities "Direct link to submissionerror-capabilities")

Submission error raised at the capabilities/service layer.

| Field  | Value               |
| ------ | ------------------- |
| `_tag` | `'SubmissionError'` |

**Fix**: Same as node-client `SubmissionError`. Check node connectivity and transaction validity.

```
Effect.catchTag('SubmissionError', (e) => ...)
```

### `InsufficientFundsError` (Capabilities — native `Error`)[​](#insufficientfundserror-capabilities--native-error "Direct link to insufficientfundserror-capabilities--native-error")

This is a plain JavaScript `Error`, *not* a `Data.TaggedError`. It is thrown (not yielded as an Effect failure) during coin selection.

**Known messages**: `"Insufficient Funds: could not balance <tokenType>"`

Where `<tokenType>` is the token identifier.

**Fix**: Catch with standard `try/catch` or `Effect.tryPromise`. Ensure the wallet has enough of the specified token type before invoking coin selection.

## Utilities (`@midnight-ntwrk/wallet-sdk-utilities`)[​](#utilities-midnight-ntwrkwallet-sdk-utilities "Direct link to utilities-midnight-ntwrkwallet-sdk-utilities")

The utilities package exposes five error types.

### `LedgerError`[​](#ledgererror "Direct link to ledgererror")

Wraps exceptions thrown by the ledger WASM module.

| Field  | Value           |
| ------ | --------------- |
| `_tag` | `'LedgerError'` |

**Fix**: Inspect the wrapped cause. Usually indicates invalid state passed to the ledger. Check that ledger inputs are well-formed.

```
Effect.catchTag('LedgerError', (e) => ...)
```

### `LeftError<L>`[​](#lefterrorl "Direct link to lefterrorl")

Raised when you encounter an `Either.Left` value where you expected a `Right`.

| Field  | Value         |
| ------ | ------------- |
| `_tag` | `'LeftError'` |

**Fix**: Check the logic that produces the `Either` value. A `Left` here indicates an unhandled error branch.

```
Effect.catchTag('LeftError', (e) => ...)
```

### `InvalidProtocolSchemeError`[​](#invalidprotocolschemeerror "Direct link to invalidprotocolschemeerror")

This error occurs when one or more of the network URLs provided uses an unsupported protocol scheme.

| Field  | Value                          |
| ------ | ------------------------------ |
| `_tag` | `'InvalidProtocolSchemeError'` |

**Fix**: Ensure URLs use the expected scheme (for example, `ws://` or `wss://` for WebSocket connections, `http://` or `https://` for HTTP).

```
Effect.catchTag('InvalidProtocolSchemeError', (e) => ...)
```

### `FailedToDeriveWebSocketUrlError`[​](#failedtoderivewebsocketurlerror "Direct link to failedtoderivewebsocketurlerror")

This error occurs when the system cannot derive a WebSocket URL from the provided input.

| Field  | Value                               |
| ------ | ----------------------------------- |
| `_tag` | `'FailedToDeriveWebSocketUrlError'` |

**Fix**: Check that the input URL is well-formed and that the derivation logic covers the provided scheme/host combination.

```
Effect.catchTag('FailedToDeriveWebSocketUrlError', (e) => ...)
```

### `ClientError`[​](#clienterror "Direct link to clienterror")

A client-side networking error, corresponding to HTTP 400–499 status codes.

| Field  | Value           |
| ------ | --------------- |
| `_tag` | `'ClientError'` |

**Fix**: These indicate a problem with the request, such as bad input, authentication failure, or not found. Do not retry without changing the request. Inspect the error for the specific status code.

```
Effect.catchTag('ClientError', (e) => ...)
```

### `ServerError`[​](#servererror "Direct link to servererror")

A server-side error, corresponding to HTTP 500+ status codes.

| Field  | Value           |
| ------ | --------------- |
| `_tag` | `'ServerError'` |

**Fix**: For persistent 500 errors, check server logs. The SDK automatically retries on transient 502–504 errors (gateway/service unavailable errors).

```
Effect.catchTag('ServerError', (e) => ...)
```

## Runtime (`@midnight-ntwrk/wallet-sdk-runtime`)[​](#runtime-midnight-ntwrkwallet-sdk-runtime "Direct link to runtime-midnight-ntwrkwallet-sdk-runtime")

The runtime package exposes one error type.

### `WalletRuntimeError`[​](#walletruntimeerror "Direct link to walletruntimeerror")

A configuration error in the wallet runtime.

| Field  | Value                  |
| ------ | ---------------------- |
| `_tag` | `'WalletRuntimeError'` |

**Known messages**:

* `"No variant to init"`: No wallet variant was provided during initialisation
* `"Empty variants list"`: The variants list supplied to the runtime is empty

**Fix**: Ensure that you configure at least one wallet variant before you initialise the runtime.

```
Effect.catchTag('WalletRuntimeError', (e) => ...)
```

## Address format (`@midnight-ntwrk/wallet-sdk-address-format`)[​](#address-format-midnight-ntwrkwallet-sdk-address-format "Direct link to address-format-midnight-ntwrkwallet-sdk-address-format")

The address format package exposes five error types.

These are plain JavaScript `Error` throws, not `Data.TaggedError` instances. Catch with standard `try/catch`.

| Message                                          | Cause                                                  |
| ------------------------------------------------ | ------------------------------------------------------ |
| `"Expected prefix mn"`                           | Address does not start with the `mn` prefix            |
| `"Segment contains disallowed characters"`       | Address segment has characters outside the allowed set |
| `"Expected type <expected>, got <actual>"`       | Address type byte does not match the expected type     |
| `"Coin public key needs to be 32 bytes long"`    | Public key component is the wrong length               |
| `"Unshielded address needs to be 32 bytes long"` | Unshielded address payload is the wrong length         |
| `"Dust address is too large"`                    | DUST address exceeds the maximum allowed size          |

**Fix**: Validate address strings before passing them to the address-format API. Use the shielded wallet's `AddressError` (`'Wallet.Address'`) for higher-level address validation.

## Complete tag registry[​](#complete-tag-registry "Direct link to Complete tag registry")

All twenty-nine `_tag` values, alphabetically sorted

| `_tag`                              | Package                                                                                                                         | Error type                        |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| `'ClientError'`                     | `@midnight-ntwrk/wallet-sdk-utilities`                                                                                          | `ClientError`                     |
| `'ConnectionError'`                 | `@midnight-ntwrk/wallet-sdk-node-client`                                                                                        | `ConnectionError`                 |
| `'FailedToDeriveWebSocketUrlError'` | `@midnight-ntwrk/wallet-sdk-utilities`                                                                                          | `FailedToDeriveWebSocketUrlError` |
| `'InvalidProtocolSchemeError'`      | `@midnight-ntwrk/wallet-sdk-utilities`                                                                                          | `InvalidProtocolSchemeError`      |
| `'LeftError'`                       | `@midnight-ntwrk/wallet-sdk-utilities`                                                                                          | `LeftError<L>`                    |
| `'LedgerError'`                     | `@midnight-ntwrk/wallet-sdk-utilities`                                                                                          | `LedgerError`                     |
| `'ParseError'`                      | `@midnight-ntwrk/wallet-sdk-node-client`                                                                                        | `ParseError`                      |
| `'ServerError'`                     | `@midnight-ntwrk/wallet-sdk-utilities`                                                                                          | `ServerError`                     |
| `'SubmissionError'`                 | `@midnight-ntwrk/wallet-sdk-node-client`, `@midnight-ntwrk/wallet-sdk-capabilities`                                             | `SubmissionError`                 |
| `'TransactionDroppedError'`         | `@midnight-ntwrk/wallet-sdk-node-client`                                                                                        | `TransactionDroppedError`         |
| `'TransactionInvalidError'`         | `@midnight-ntwrk/wallet-sdk-node-client`                                                                                        | `TransactionInvalidError`         |
| `'TransactionProgressError'`        | `@midnight-ntwrk/wallet-sdk-node-client`                                                                                        | `TransactionProgressError`        |
| `'TransactionUsurpedError'`         | `@midnight-ntwrk/wallet-sdk-node-client`                                                                                        | `TransactionUsurpedError`         |
| `'UtxoNotFoundError'`               | `@midnight-ntwrk/wallet-sdk-unshielded-wallet`                                                                                  | `UtxoNotFoundError`               |
| `'Wallet.Address'`                  | `@midnight-ntwrk/wallet-sdk-shielded`, `@midnight-ntwrk/wallet-sdk-unshielded-wallet`                                           | `AddressError`                    |
| `'Wallet.ApplyTransaction'`         | `@midnight-ntwrk/wallet-sdk-unshielded-wallet`                                                                                  | `ApplyTransactionError`           |
| `'Wallet.InsufficientFunds'`        | `@midnight-ntwrk/wallet-sdk-shielded`, `@midnight-ntwrk/wallet-sdk-unshielded-wallet`, `@midnight-ntwrk/wallet-sdk-dust-wallet` | `InsufficientFundsError`          |
| `'Wallet.InvalidCoinHashes'`        | `@midnight-ntwrk/wallet-sdk-shielded`, `@midnight-ntwrk/wallet-sdk-unshielded-wallet`                                           | `InvalidCoinHashesError`          |
| `'Wallet.Other'`                    | `@midnight-ntwrk/wallet-sdk-shielded`, `@midnight-ntwrk/wallet-sdk-unshielded-wallet`, `@midnight-ntwrk/wallet-sdk-dust-wallet` | `OtherWalletError`                |
| `'Wallet.Proving'`                  | `@midnight-ntwrk/wallet-sdk-capabilities`                                                                                       | `ProvingError`                    |
| `'Wallet.RollbackUtxo'`             | `@midnight-ntwrk/wallet-sdk-unshielded-wallet`                                                                                  | `RollbackUtxoError`               |
| `'Wallet.Sign'`                     | `@midnight-ntwrk/wallet-sdk-unshielded-wallet`                                                                                  | `SignError`                       |
| `'Wallet.SpendUtxo'`                | `@midnight-ntwrk/wallet-sdk-unshielded-wallet`                                                                                  | `SpendUtxoError`                  |
| `'Wallet.SubmissionWalletError'`    | `@midnight-ntwrk/wallet-sdk-shielded`, `@midnight-ntwrk/wallet-sdk-unshielded-wallet`                                           | `SubmissionError` (wallet)        |
| `'Wallet.Sync'`                     | `@midnight-ntwrk/wallet-sdk-shielded`, `@midnight-ntwrk/wallet-sdk-unshielded-wallet`, `@midnight-ntwrk/wallet-sdk-dust-wallet` | `SyncWalletError`                 |
| `'Wallet.Transacting'`              | `@midnight-ntwrk/wallet-sdk-shielded`, `@midnight-ntwrk/wallet-sdk-unshielded-wallet`, `@midnight-ntwrk/wallet-sdk-dust-wallet` | `TransactingError`                |
| `'Wallet.TransactionHistory'`       | `@midnight-ntwrk/wallet-sdk-shielded`, `@midnight-ntwrk/wallet-sdk-unshielded-wallet`                                           | `TransactionHistoryError`         |
| `'WalletRuntimeError'`              | `@midnight-ntwrk/wallet-sdk-runtime`                                                                                            | `WalletRuntimeError`              |
