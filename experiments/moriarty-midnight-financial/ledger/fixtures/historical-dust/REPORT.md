# Positive historical native DUST fixture

Result: public retrieval and native deserialization PASS; current financial receipt fee-equality assumption is contradicted. Independent GPT-6 Astra source-evidence task, 2026-09-09. No new transaction, wallet restoration, private state/seed access, prover callback, proof verification or native execution occurred. Existing frozen modules were not modified.

One public GraphQL read used Scrapling 0.4.15 Fetcher.post, timeout20 seconds, retries0. It returned HTTP200 in0.898 seconds at 2026-09-09T20:52:25Z. Exact query, response and retrieval receipt are retained beside this report. The request used the known historical call identifier from settlement-2026-09-07T03-46-27-714Z.json. No cookies were retained, no login or bypass used, and the existing API/GraphQL skill pattern sufficed. No dependency install/upgrade or new acquisition pattern was needed.

Observed positive native fixture:

- Transaction/raw SHA-256 `f79580fe0075dc26ae3f97f10557706f340cdf7a3b3118cd65a72b4fd870110a`, 6598 raw bytes; deserialize(signature,proof,binding) and canonical reserialization agree using ledger-v8 package8.1.0.
- One actual native DustSpend in segment1; vFee=`300000000000001`, positive. No registrations. Existing hello-world call action is storeMessage at the historical contract address.
- Indexer protocolVersion=`1000000`; status SUCCESS; paidFees=`1`, estimatedFees=`1`. Package version is not protocol version.
- Native transaction hash, both identifiers and block755889/hash match the original historical receipt exactly. That receipt records canonical-node hash and finality coverage through height755891. historical-finality.json binds its exact bytes. No fresh finality RPC was necessary or performed; this is explicitly reused historical finality evidence.
- Existing decodeNativeFinancialTransaction successfully decodes these real public bytes and returns the positive native dust sum. The known fee comparison in observeFinalizedStage would reject this record at DUST_FEE_MISMATCH once its preceding identity/finality checks pass. That failure is a source inference from the exact observed values, not a claimed new end-to-end observer execution.

Exact capture hashes:

- response.json: `e011c09bf5dcd0d44b7fe738e3bd7b4f33bbf458239f8d41c4a6271baccf0222`
- transaction.bin: `f79580fe0075dc26ae3f97f10557706f340cdf7a3b3118cd65a72b4fd870110a`
- native-observation.json: `64935eca501593874b046ad89448b883d164418d51772eea8d778cb5fe8937e4`
- retrieval.json: `5c97c2eae4938aabb28889663d8a3154f627f55b358916691cf8565309c78b88`

## What the sources establish about fees

`source-hashes.json` pins the locally inspected primary SDK files and archived official documentation bytes. No fee computation or transaction validation API was executed.

1. ledger-v8/ledger-v8.d.ts:2444–2458 explicitly documents transaction fees and feesWithMargin in SPECKs. Its DustSpend.vFee getter and DustLocalState.spend(...,vFee,...) signature are at1490 and1602.
2. wallet-sdk-dust-wallet/dist/v1/Transacting.js:222–224 calculates the selected fee as feesWithMargin plus additionalFeeOverhead. Lines248–275 choose recipe inputs for that amount; CoreWallet.js:94–98 passes each selected takeFee into native state.spend and retains its resulting DustSpend. The native quantity therefore represents the fee resource amount selected for consumption from that DUST input, not merely an indexer estimate.
3. The retained hello-world src/wallet.ts:116 configures additionalFeeOverhead=`300000000000000` and feeBlocksMargin=5. The observed native debit is exactly that overhead plus1. This arithmetic and the source path strongly support wallet safety overhead as the cause of the larger native spend. They do not establish the deployed indexer's internal fee algorithm.
4. Pinned midnight-js-indexer-public-data-provider/dist/index.mjs:988–992 simply forwards indexer paidFees and estimatedFees strings; it applies no conversion or native-spend reconciliation.
5. The archived official TransactionFees documentation, https://docs.midnight.network/api-reference/midnight-indexer/types/objects/transaction-fees.md (retrieved2026-09-07; SHA25631e74799d586eb9f1a5146a770969da858e3b1762d6216b74870a08a6bea6ea2), describes paidFees as actual transaction fees “in DUST” and estimatedFees as an estimate. It does not specify atomic encoding, rounding, safety-overhead inclusion or the deployed indexer computation. This label is insufficient to equate its string with vFee or determine a scale conversion.
6. The archived official DUST architecture page, https://docs.midnight.network/concepts/dust-architecture.md (SHA256481fe2164f3716d1941004b95e6cd7638f373c41f521b7cdc67b69e40e8af010), describes each spend's output as updated input minus gas consumed and identifies SPECK as the atomic unit, with10^15 SPECK per DUST. This supports native fee-debit interpretation. It does not reconcile the indexer field for this deployment; multiplying or dividing the observed1 by10^15 does not explain300000000000001.

Recommendation: keep summing distinct native DustSpend.vFee values across intents as the native fee-debit budget, including the wallet-selected overhead. For this registration-free finalized historical transaction, the sum is the observed transaction-declared native amount spent; ledger acceptance/finality comes from the explicitly bound external evidence, not this deserialization. Do not replace it with the smaller indexer string, subtract overhead from authorization, or claim an independently measured wallet balance delta. Preserve indexer paid/estimated strings separately with documented source label DUST and unresolved exact encoding/relationship. Remove the unsupported cross-field equality from the production acceptance predicate until deployed primary evidence defines it. Retain this positive real fixture and a control that demonstrates unequal fields are not automatically a failed transaction.

Limits: this positive native DUST fixture closes the absence of a real positive getter example, not financial contract settlement, all DUST-family accounting, ownership proof, protocol universality or SP05 acceptance. Original inspect.mjs hit a BigInt JSON serialization error after successfully writing transaction.bin; it is preserved. inspect-v2.mjs only corrected evidence serialization and checked the existing raw bytes, without another request or proof operation. Native nullifier is bigint and is represented as decimal text in the JSON receipt.
