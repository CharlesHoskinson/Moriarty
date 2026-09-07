# MC02 public ledger interface intake

Status: source-inspected candidate design only. This 600-second preparation implements or executes no contract. MC01 acceptance remains a prerequisite to downstream implementation/compilation/execution. No wallet data, key material or live network was accessed. Source hashes and installed package versions are retained in `ledger-interface-intake.json`.

## Source pins and compatibility

Repository observation: the installed stack is ledger-v8 **8.1.0**, onchain-runtime-v3 **3.0.0**, compact-runtime **0.16.0**, and midnight-js contracts/types/indexer/protocol **4.1.1**. The compiler is a separate versioned component, **Compact 0.31.1**, with language **0.23.0**; calling the SDK “0.31.1” conflates these components. The retained unshielded-token tutorial explicitly targets compiler 0.31.1 and explains its range-check fix. This intake reads versions/docs only; it does not recompile or establish compatibility of the new combined wrapper.

Primary local source locations:

- `ledger-v8/ledger-v8.d.ts`: Transcript/Effects 255–381; ContractState 765–809; Utxo/UtxoOutput/UtxoSpend/addressFromKey 1769–1840; ContractCall 1885–1915; Intent/UnshieldedOffer 1984–2089; Transaction serialization/hash/identifiers/imbalances/fees 2399–2480; DustSpend/DustActions 1485–1514.
- `midnight-js-types/dist/index.d.ts`: FinalizedTxData 186 onward; PublicDataProvider 916–1000.
- `midnight-js-indexer-public-data-provider/dist/index.mjs`: transaction query 205 onward; raw deserializer 572; UTXO summary conversion 637 onward; queryContractState 834 onward; watchForTxData 963 onward.
- Retained docs `tokens/unshielded-token.md`, `examples/contracts/token-transfers.md`, and `compact/reference/compact-reference.md` imports 817–888 and sealed ledger fields 1316 onward.
- Repository `experiments/moriarty-midnight-network/preview-call.mjs` and `preview-verify.mjs` supply transaction retention and canonical-finality patterns. They were read as source; their wallet-loading/network code was not run.

## Combined wrapper and six submissions

Design: one newly deployed combined contract imports two namespaced generic kernels and keeps their numeric state/lifecycles separate. The existing kernels declare conflicting top-level names (`KernelState`, `transition0`, arithmetic helpers); two raw includes are not a valid composition. Package each in a named module, retaining exact generated-source and source-map lineage, then use the documented syntax:

```compact
import LoanKernel prefix Loan$;
import SwapKernel prefix Swap$;
export ledger loanState: Loan$KernelState;
export ledger swapState: Swap$KernelState;
```

This is a syntax/design sketch, not a compiled wrapper. Module-scoped library/helper imports and exported struct identities must be tested after the gate. Do not silently rename source Core or regenerate its identity. Persist initial semantic program/bounds/source bindings, role-address bindings, initialization flag, token colors and per-instance revision/remaining/horizon. Constructor-set bindings may use sealed ledger fields; token colors assigned by the later initialize action cannot be sealed constructor-only fields. Initialize must be authorized and one-shot. Subsequent calls use persisted state as kernel input; caller-supplied “before” snapshots are not authoritative.

The existing Preview wallet will own the borrower and trader positions. They are roles in distinct semantic instances, so this does not require duplicate actor mappings within one genesis. Lender and provider must have distinct recoverable ownership identities, distinct from the spending wallet; choose and bind them later. No identity was generated or inspected in this intake. A Text-table actor ID or `arg.actor` equality does not authenticate an owner. The retained token tutorial explicitly warns that `ownPublicKey()` is prover-claimed and unsuitable alone for authorization. The combined wrapper needs a real authorization predicate, for example a recoverable capability commitment or supported signature verification bound to each exact action/context; source-only actor guards are insufficient, especially for provider closure where there is no provider UTXO input.

| Submission | Action | Real financial effects |
|---|---|---|
| 1 | deploy | Store fixed bindings and initial semantic state; no test-token movement. |
| 2 | initialize | Mint USD 20,000,000,000, A 1,100,000 and B 2,000,000 to self; send USD 20,000,000,000 to borrower and A 100,000 to trader; retain pool A 1,000,000 and B 2,000,000. |
| 3 | accrue | No token movement; create PR 500,000,000 and IP 33,972,602 semantic dues; retain notional 4,500,000,000. |
| 4 | settle | Receive USD 533,972,602 from borrower funding; send that exact amount to lender; contract USD ends at zero. |
| 5 | swap | Receive A 10,000; send B 19,743 to trader; contract holds A 1,010,000 and B 1,980,257. |
| 6 | close | Send A 1,010,000 and B 1,980,257 to provider; all three contract test-token balances end at zero. |

All three contract-origin colors use distinct fixed 32-byte domain separators. `mintUnshieldedToken(domain, amount:Uint<64>, left<ContractAddress,UserAddress>(kernel.self()))` returns the color and directly credits contract custody. All planned supplies fit Uint64. `tokenType(domain,kernel.self())` binds each color to this deployment; colors from another contract or NIGHT are not substitutes. Store the actual colors and bind the source asset symbols to them with quantum exactly one.

The retained source examples specify these operations (uncompiled sketch):

```compact
receiveUnshielded(usdColor, 533972602);
sendUnshielded(usdColor, 533972602,
  right<ContractAddress, UserAddress>(lender));
receiveUnshielded(aColor, 10000);
sendUnshielded(bColor, 19743,
  right<ContractAddress, UserAddress>(trader));
```

Actual wrapper amounts must come from the checked kernel effect operands and be equated to the fixed profile expectations, not from unrelated hard-coded final-state counters. `receiveUnshielded` constrains incoming value but does not by itself identify the economic payer. The finalized offer's signed UTXO inputs must establish the authorized borrower/trader funding. Transfer recipients must equal persistent authenticated role addresses. `unshieldedBalance`/`unshieldedBalanceGte` and externally decoded `ContractState.balance` are real custody checks; numeric `borrower_cash`, reserves or snapshot cells alone do not prove token ownership or settlement. The exact intra-call balance behavior for receive-then-send must be checked in the generated wrapper before public execution.

## Decoder and transaction binding

Design: retain the bytes returned by wallet `finalizeRecipe()` before submission, as the existing Preview call module does; retain their raw-byte digest, `transactionHash()` and all `identifiers()`. On inclusion obtain `FinalizedTxData.tx` plus raw indexer bytes, status, segment statuses, block identity, fees and protocolVersion. The installed provider deserializes raw data using:

```js
const tx = Transaction.deserialize('signature', 'proof', 'binding', rawBytes);
for (const [segmentId, intent] of tx.intents ?? []) {
  for (const phase of ['guaranteedUnshieldedOffer', 'fallibleUnshieldedOffer']) {
    const offer = intent[phase];
    // Preserve every input and output, including original array index.
    // input owner is a verifying key: addressFromKey(input.owner) is its address.
  }
  for (const [actionIndex, action] of intent.actions.entries()) {
    // Classify deploy/call/maintenance; reject unexpected action kinds/addresses.
    // Calls carry guaranteedTranscript and fallibleTranscript.
  }
}
```

This is API-shaped design code, not an implemented or executed decoder. Never use `toString()` as the financial schema or discard fields through JSON Map serialization. Encode every TokenType and PublicAddress as a tagged canonical record; sort map entries by encoded key, retaining both tags. Map keys are JS objects/tuples, so object identity or iteration order cannot be treated as economic equality. Preserve transcript `program`, `gas` and every `effects` member:

- `unshieldedMints`: domain separator → Uint64 amount.
- `unshieldedInputs`, `unshieldedOutputs`: TokenType → amount.
- `claimedUnshieldedSpends`: `[TokenType, PublicAddress]` → expected output amount. Despite its name this is the contract's expected UTXO **output** claim, not an external input-spend list.
- All nullifiers, shielded receives/spends/mints and claimed contract calls. The initial unshielded profile expects these empty; unexpected effects reject rather than disappear from the report.

Inspect both guaranteed and fallible sections for every segment and action. Require whole-transaction success and all relevant segment success; reject partial success for these atomic financial actions. Determine and test the exact standard-library mint/receive/send section placement; do not infer gross movement solely from the final contract balance. Settlement has zero net USD custody change but gross incoming/outgoing USD 533,972,602, both of which must be evidenced. Swap and close require the exact destination claims and real destination outputs as well as reserve balances.

For each raw external input retain value, type, owner verifying key, derived user address, parent intentHash and outputNo. For each raw output retain value, type, owner and original output index; combine with its intent's `intentHash(segmentId)` to reconstruct UTXO identity. Cross-check indexer created/spent output summaries. The installed SDK summaries **omit outputNo**, so they cannot replace the raw transaction's exact identity inventory.

Retain all balancing change and unrelated-token effects, including zero entries if present. A borrower spending a single 20-billion USD UTXO to pay 533,972,602 will normally have 19,466,027,398 returned as balancing change; exact fragmentation is an observed wallet result, not a prescribed number of outputs. Source gross debit is the economic contract-directed movement, not the entire consumed funding UTXO. Classify authenticated same-owner change explicitly and use contract transcript gross receive/send claims to prevent genuine counterparty refunds from hiding over-spending. No unexplained recipient or color may be silently treated as change. Final test-token ownership totals must be borrower USD 19,466,027,398; lender USD 533,972,602; trader A 90,000/B 19,743; provider A 1,010,000/B 1,980,257; contract zero. Global test-token supply must stay USD 20 billion, A 1.1 million, B 2 million, with minting only during initialize.

Retain raw finalized/included bytes separately. Ledger source warns that merging can change `transactionHash()`, so use identifiers to locate inclusion, then reconcile every included intent/action/effect against the submitted transaction; hash inequality is neither automatic equivalence nor grounds to drop extra effects. For this bounded profile, reject an unexplained merge or explicitly prove the allowed unchanged action/intent subset plus absence of unauthorized additions.

## Public state, finality and DUST

Repository observation: `queryContractState(address,{type:'blockHash',blockHash})` returns decoded `ContractState`, whose `.data` is consumed by generated `ledger(...)` and whose `.balance` is actual contract custody. Save both serialized state bytes and these decoded views before/after each call. The installed `queryUnshieldedBalances` GraphQL ContractCall branch reads `deploy.unshieldedBalances`; its freshness must be tested independently against fixed-block `.balance` and raw action data. Do not silently accept deployment balances as current reserves. Block-level readback may include a later same-block action: pin exact contractAction state/index or establish there was no intervening action before using it as the immediate post-state.

The API calls its return `FinalizedTxData`, but inspected `watchForTxData` polls indexer inclusion and maps its status; it performs no node canonical-finality check. Reuse the retained Preview verifier protocol: check `system_chain == 'Midnight Preview'`, obtain `chain_getFinalizedHead` and its header height, require each inclusion height ≤ finalized height, and require `chain_getBlockHash(inclusionHeight)` equal the indexer's inclusion block hash. Record exact response bytes and normalize only the optional `0x` prefix. Bind included transaction bytes/identifiers and the fixed-block readback to that canonical block; SDK inclusion alone is insufficient. Use application-level bounded waits; provider watches are specified to wait indefinitely.

For every intent separately retain DUST action ctime and registrations; each spend exposes `vFee`, `oldNullifier`, `newCommitment` and proof presence. Retain indexed paid/estimated fees and relevant ledger parameters; ledger `fees(params)` is documented accurate for proven transactions. DUST is a distinct token type and must not enter the three test-token gross/net calculations. Do not register freshly minted contract tokens for DUST: only NIGHT is eligible. The existing Preview recovery history requires preservation of wallet identity/state and retained failed transaction evidence, not blind resubmission. This preparation performs no wallet operation.

## Remaining gates

Open: MC01 acceptance; compiled namespaced wrapper and exact stdlib effect partition; real recoverable role authorization; raw address/color codecs (ledger comments mention 35-byte strings while Compact uses Bytes32, so use pinned encode/decode APIs rather than width guesses); exact finalized-byte/UTXO/contract-action decoder; actual custody and recipient balance tests; proof/ledger correspondence; mandatory Moriarty PCD/history/refinement acceptance. Financial token settlement alone does not discharge semantic outstanding notional or establish these proof claims. No downstream execution is authorized by this source-only receipt.
