> For the complete documentation index, see [llms.txt](/llms.txt)

# Decode 1010 transaction rejection errors

Resolve `1010 Invalid Transaction` responses by decoding the inner `Custom error: N` value into a Midnight ledger error variant.

## Understand the cause of the error[​](#understand-the-cause-of-the-error "Direct link to Understand the cause of the error")

When you submit a transaction to a Midnight node, the JSON-RPC layer may reject it with code `1010`:

```
{

  "code": 1010,

  "message": "Invalid Transaction",

  "data": "Custom error: 155"

}
```

Code `1010` itself is a [Substrate](https://github.com/paritytech/polkadot-sdk/tree/master/substrate) envelope, not a Midnight-specific error. It signals that the node's transaction pool rejected the extrinsic. The actionable signal is the `N` in `Custom error: N`, which is a `u8` (0–255) corresponding to a `LedgerApiError` variant defined by the Midnight node.

Two layers, one error

The `1010` is inherited from Substrate. The inner `u8` is Midnight's. To diagnose the problem, you must decode the `u8`.

## Extract the inner u8[​](#extract-the-inner-u8 "Direct link to Extract the inner u8")

The inner `u8` is embedded in the `data` field of the RPC error response. Use the steps below to extract it from common client environments.

1

### Capture the full RPC response[​](#capture-the-full-rpc-response "Direct link to Capture the full RPC response")

Capture the complete error response, not just the message string. Many wallet libraries and CLI wrappers truncate the `data` field by default.

* JavaScript
* cURL

```
try {

  await api.tx.someCall().signAndSend(account);

} catch (err) {

  console.error(JSON.stringify(err, null, 2));

}
```

```
curl -H "Content-Type: application/json" \

  -d '{"jsonrpc":"2.0","id":1,"method":"author_submitExtrinsic","params":["0x..."]}' \

  http://localhost:9944
```

2

### Read the trailing integer in the data field[​](#read-the-trailing-integer-in-the-data-field "Direct link to Read the trailing integer in the data field")

The `data` field ends with `Custom error: N` where `N` is the variant code:

```
"data": "Custom error: 155"
```

In this example, `N` is `155`.

3

### Look up the variant[​](#look-up-the-variant "Direct link to Look up the variant")

Use the [variant tables](#look-up-the-variant) below to map the `u8` to its ledger error name and category.

Variant codes change between releases

Variant numbering is defined by the Midnight node and may change across releases. For example, `168 FeeCalculation` was retired and replaced by `155 FeeCalculationError`. Always cross-reference the variant against the [release notes](/relnotes/node.md) for the node version your network is running.

## Look up the variant[​](#look-up-the-variant-1 "Direct link to Look up the variant")

The tables below list every `LedgerApiError` variant currently emitted by the Midnight node, grouped by category. Each row shows the `u8`, the variant name, and the meaning. Use the `Custom error: N` value extracted in the previous step to find the corresponding row.

### Deserialization errors (0–11)[​](#deserialization-errors-011 "Direct link to Deserialization errors (0–11)")

The node could not decode an input from its serialized form. The transaction is rejected before any ledger logic runs.

**Show full table**

| Code | Variant                       |
| ---- | ----------------------------- |
| 0    | NetworkId                     |
| 1    | Transaction                   |
| 2    | LedgerState                   |
| 3    | ContractAddress               |
| 4    | PublicKey                     |
| 5    | VersionedArenaKey             |
| 6    | UserAddress                   |
| 7    | TypedArenaKey                 |
| 8    | SystemTransaction             |
| 9    | DustPublicKey                 |
| 10   | CNightGeneratesDustActionType |
| 11   | CNightGeneratesDustEvent      |

### Serialization errors (50–63)[​](#serialization-errors-5063 "Direct link to Serialization errors (50–63)")

The node could not encode a value to its serialized form. These typically indicate an internal inconsistency rather than a client error.

**Show full table**

| Code | Variant                  |
| ---- | ------------------------ |
| 50   | TransactionIdentifier    |
| 51   | LedgerState              |
| 52   | LedgerParameters         |
| 53   | ContractAddress          |
| 54   | ContractState            |
| 55   | ContractStateToJson      |
| 56   | ZswapState               |
| 57   | UnknownType              |
| 58   | MerkleTreeDigest         |
| 59   | VersionedArenaKey        |
| 60   | TypedArenaKey            |
| 61   | CNightGeneratesDustEvent |
| 62   | SystemTransaction        |
| 63   | ArenaHash                |

### Invalid transactions (100–109, 193–200, 239–250)[​](#invalid-transactions-100109-193200-239250 "Direct link to Invalid transactions (100–109, 193–200, 239–250)")

The transaction is structurally valid but conflicts with current ledger state. These are the most common rejections in production traffic.

**Show full table**

| Code    | Variant                                           |
| ------- | ------------------------------------------------- |
| 100     | EffectsMismatch                                   |
| 101     | ContractAlreadyDeployed                           |
| 102     | ContractNotPresent                                |
| 103     | Zswap                                             |
| 104     | Transcript                                        |
| 105     | InsufficientClaimable                             |
| 106     | VerifierKeyNotFound                               |
| 107     | VerifierKeyAlreadyPresent                         |
| 108     | ReplayCounterMismatch                             |
| 109     | UnknownError                                      |
| ~~193~~ | ~~ReplayProtectionViolation~~ *(Retired)*         |
| 194     | BalanceCheckOutOfBounds                           |
| 195     | InputNotInUtxos                                   |
| 196     | DustDoubleSpend                                   |
| 197     | DustDeregistrationNotRegistered                   |
| 198     | GenerationInfoAlreadyPresent                      |
| 199     | InvariantViolation                                |
| 200     | RewardTooSmall                                    |
| 239     | Zswap.Invalid.NullifierAlreadyPresent             |
| 240     | Zswap.Invalid.CommitmentAlreadyPresent            |
| 241     | Zswap.Invalid.UnknownMerkleRoot                   |
| 242     | ReplayProtectionViolation.IntentTtlExpired        |
| 243     | ReplayProtectionViolation.IntentTtlTooFarInFuture |
| 244     | ReplayProtectionViolation.IntentAlreadyExists     |
| 248     | DivideByZero                                      |
| 249     | Invalid.MerkleTreeError                           |
| 250     | Zswap.Invalid.MerkleTreeError                     |

### Malformed transactions (110–139, 166–192, 212–238)[​](#malformed-transactions-110139-166192-212238 "Direct link to Malformed transactions (110–139, 166–192, 212–238)")

The transaction failed structural or cryptographic validation. The submitting client should treat these as bugs in transaction construction or signing.

**Show full table**

| Code    | Variant                                               |
| ------- | ----------------------------------------------------- |
| 110     | VerifierKeyNotSet                                     |
| 111     | TransactionTooLarge                                   |
| 112     | VerifierKeyTooLarge                                   |
| 113     | VerifierKeyNotPresent                                 |
| 114     | ContractNotPresent                                    |
| 115     | InvalidProof                                          |
| 116     | BindingCommitmentOpeningInvalid                       |
| 117     | NotNormalized                                         |
| 118     | FallibleWithoutCheckpoint                             |
| 119     | ClaimReceiveFailed                                    |
| 120     | ClaimSpendFailed                                      |
| 121     | ClaimNullifierFailed                                  |
| 122     | ClaimCallFailed                                       |
| 123     | InvalidSchnorrProof                                   |
| 124     | UnclaimedCoinCom                                      |
| 125     | UnclaimedNullifier                                    |
| 126     | Unbalanced                                            |
| 127     | Zswap                                                 |
| 128     | BuiltinDecode                                         |
| 129     | GuaranteedLimit                                       |
| 130     | MergingContracts                                      |
| 131     | CantMergeTypes                                        |
| 132     | ClaimOverflow                                         |
| 133     | ClaimCoinMismatch                                     |
| 134     | KeyNotInCommittee                                     |
| 135     | InvalidCommitteeSignature                             |
| 136     | ThresholdMissed                                       |
| 137     | TooManyZswapEntries                                   |
| 138     | BalanceCheckOverspend                                 |
| 139     | UnknownError                                          |
| 166     | InvalidNetworkId                                      |
| 167     | IllegallyDeclaredGuaranteed                           |
| ~~168~~ | ~~FeeCalculation~~ *(Retired)*                        |
| 169     | InvalidDustRegistrationSignature                      |
| 170     | InvalidDustSpendProof                                 |
| 171     | OutOfDustValidityWindow                               |
| 172     | MultipleDustRegistrationsForKey                       |
| 173     | InsufficientDustForRegistrationFee                    |
| 174     | MalformedContractDeploy                               |
| 175     | IntentSignatureVerificationFailure                    |
| 176     | IntentSignatureKeyMismatch                            |
| 177     | IntentSegmentIdCollision                              |
| 178     | IntentAtGuaranteedSegmentId                           |
| 179     | UnsupportedProofVersion                               |
| 180     | GuaranteedTranscriptVersion                           |
| 181     | FallibleTranscriptVersion                             |
| ~~182~~ | ~~TransactionApplicationError~~ *(Retired)*           |
| 183     | BalanceCheckOutOfBounds                               |
| 184     | BalanceCheckConversionFailure                         |
| 185     | PedersenCheckFailure                                  |
| ~~186~~ | ~~EffectsCheckFailure~~ *(Retired)*                   |
| ~~187~~ | ~~DisjointCheckFailure~~ *(Retired)*                  |
| ~~188~~ | ~~SequencingCheckFailure~~ *(Retired)*                |
| 189     | InputsNotSorted                                       |
| 190     | OutputsNotSorted                                      |
| 191     | DuplicateInputs                                       |
| 192     | InputsSignaturesLengthMismatch                        |
| 212     | EffectsCheck.RealCallsSubsetCheckFailure              |
| 213     | EffectsCheck.AllCommitmentsSubsetCheckFailure         |
| 214     | EffectsCheck.RealUnshieldedSpendsSubsetCheckFailure   |
| 215     | EffectsCheck.ClaimedUnshieldedSpendsUniquenessFailure |
| 216     | EffectsCheck.ClaimedCallsUniquenessFailure            |
| 217     | EffectsCheck.NullifiersNeqClaimedNullifiers           |
| 218     | EffectsCheck.CommitmentsNeqClaimedShieldedReceives    |
| 219     | SequencingCheck.CallSequencingViolation               |
| 220     | SequencingCheck.SequencingCorrelationViolation        |
| 221     | SequencingCheck.GuaranteedInFallibleContextViolation  |
| 222     | SequencingCheck.FallibleInGuaranteedContextViolation  |
| 223     | SequencingCheck.CausalityConstraintViolation          |
| 224     | SequencingCheck.CallHasEmptyTranscripts               |
| 225     | DisjointCheck.ShieldedInputsDisjointFailure           |
| 226     | DisjointCheck.ShieldedOutputsDisjointFailure          |
| 227     | DisjointCheck.UnshieldedInputsDisjointFailure         |
| 228     | TransactionApplication.IntentTtlExpired               |
| 229     | TransactionApplication.IntentTtlTooFarInFuture        |
| 230     | TransactionApplication.IntentAlreadyExists            |
| 231     | FeeCalculation.OutsideTimeToDismiss                   |
| 232     | FeeCalculation.BlockLimitExceeded                     |
| 233     | MalformedContractDeploy.NonZeroBalance                |
| 234     | MalformedContractDeploy.IncorrectChargedState         |
| 235     | Zswap.Malformed.InvalidProof                          |
| 236     | Zswap.Malformed.ContractSentCiphertext                |
| 237     | Zswap.Malformed.NonDisjointCoinMerge                  |
| 238     | Zswap.Malformed.NotNormalized                         |

### Infrastructure errors (150–157, 165)[​](#infrastructure-errors-150157-165 "Direct link to Infrastructure errors (150–157, 165)")

The node could not complete an internal operation, such as a ledger lookup or fee calculation. These are not caused by the submitted transaction.

**Show full table**

| Code | Variant                       |
| ---- | ----------------------------- |
| 150  | LedgerCacheError              |
| 151  | NoLedgerState                 |
| 152  | LedgerStateScaleDecodingError |
| 153  | ContractCallCostError         |
| 154  | BlockLimitExceededError       |
| 155  | FeeCalculationError           |
| 156  | ContractNotPresent            |
| 157  | BeneficiaryNotFound           |
| 165  | GetTransactionContextError    |

### System transactions (201–211, 245–247)[​](#system-transactions-201211-245247 "Direct link to System transactions (201–211, 245–247)")

The node rejected a system-level transaction such as a payout or treasury operation. These are typically only seen by validators or block producers.

**Show full table**

| Code    | Variant                                                           |
| ------- | ----------------------------------------------------------------- |
| 201     | IllegalPayout                                                     |
| 202     | InsufficientTreasuryFunds                                         |
| 203     | CommitmentAlreadyPresent                                          |
| 204     | UnknownError                                                      |
| ~~205~~ | ~~ReplayProtectionFailure~~ *(Retired)*                           |
| 206     | IllegalReserveDistribution                                        |
| 207     | GenerationInfoAlreadyPresent                                      |
| 208     | InvalidBasisPoints                                                |
| 209     | InvariantViolation                                                |
| 210     | TreasuryDisabled                                                  |
| 211     | MerkleTreeError                                                   |
| 245     | SystemTransaction.ReplayProtectionFailure.IntentTtlExpired        |
| 246     | SystemTransaction.ReplayProtectionFailure.IntentTtlTooFarInFuture |
| 247     | SystemTransaction.ReplayProtectionFailure.IntentAlreadyExists     |

### Host API error (255)[​](#host-api-error-255 "Direct link to Host API error (255)")

A reserved variant indicating that the node's host API surfaced an error not covered by the categories above.

**Show full table**

| Code | Variant      |
| ---- | ------------ |
| 255  | HostApiError |

## Common causes and fixes[​](#common-causes-and-fixes "Direct link to Common causes and fixes")

The following variants account for the majority of `1010` errors.

**155 FeeCalculationError** — fee calculation failed for the submitted transaction

Most often caused by stale fee estimates or an outdated runtime client. Refresh the client's view of network fees and resubmit. If the error persists, verify that your runtime packages match the values in the [release compatibility matrix](/relnotes/support-matrix.md).

**154 BlockLimitExceededError** — the transaction would exceed the block resource limits

Reduce the number of intents, calls, or similar in the transaction. For batched operations, split the work across multiple transactions.

**108 ReplayCounterMismatch / 193 ReplayProtectionViolation** — replay protection rejected the transaction

Either the replay counter is stale or the intent's TTL window has elapsed. Re-fetch the current counter and rebuild the transaction. Variants `242`–`244` and `228`–`230` give finer-grained reasons (TTL expired, TTL too far in the future, intent already exists).

**115 InvalidProof / 235 Zswap.Malformed.InvalidProof** — a zero-knowledge proof failed verification

Confirm that the proof server is running a version compatible with the node and SDK. See [run a proof server](/guides/run-proof-server.md) and the [release compatibility matrix](/relnotes/support-matrix.md).

**166 InvalidNetworkId** — the transaction was constructed for a different network

Verify that the SDK and wallet are configured for the same network ID as the node you are submitting to (for example, `Preprod` vs `TestNet02`).

**126 Unbalanced / 138 BalanceCheckOverspend** — the transaction's inputs and outputs do not balance

The wallet attempted to spend more than the available coin set permits, or change outputs were miscalculated. Re-sync the wallet and rebuild the transaction.

## When 1010 has no inner u8[​](#when-1010-has-no-inner-u8 "Direct link to When 1010 has no inner u8")

If the response contains `code: 1010` but no `Custom error: N` substring, the rejection happened in upstream Substrate validation rather than Midnight ledger logic. The most common causes are:

* **Bad signature**: the signing key does not match the sender, or the payload was altered after signing.
* **Stale era**: the mortal era window has passed. Re-fetch the current block hash and rebuild the transaction.
* **Wrong nonce**: the account nonce on the node does not match the value used when signing. Query the current nonce and resubmit.

These rejections are not represented in the `LedgerApiError` tables because they never reach the ledger.

## Verify the fix[​](#verify-the-fix "Direct link to Verify the fix")

After identifying and addressing the variant, resubmit the transaction and confirm it is accepted:

```
# Re-submit and observe the response

curl -H "Content-Type: application/json" \

  -d '{"jsonrpc":"2.0","id":1,"method":"author_submitExtrinsic","params":["0x..."]}' \

  http://localhost:9944
```

A successful submission returns a transaction hash rather than an error envelope.

## Get help[​](#get-help "Direct link to Get help")

If you cannot identify the variant or the rejection persists after applying the fixes above:

* **Confirm the node version**: Variant numbering changes across releases. Check the [node release notes](/relnotes/node.md) for the version your network is running.
* **Check the compatibility matrix**: Verify the proof server, runtime, and SDK match the supported set in the [release compatibility matrix](/relnotes/support-matrix.md).
