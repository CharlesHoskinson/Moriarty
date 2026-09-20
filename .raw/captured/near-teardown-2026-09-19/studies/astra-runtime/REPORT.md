# Independent NEAR runtime review — GPT-6 Astra, medium

Scope: final-panel runtime member, pinned-source examination on 2026-09-19. This is a bounded independent validation of the preparatory runtime study, not an audit of every acquired repository or a live-network observation. Exact locators and SHA-256 hashes are in [claims.json](claims.json). Every cited source range matched its pinned Git blob. No source repositories or vaults were changed.

## Explanation — the relevant execution contract

A NEAR transaction authenticates a serialized transaction hash, checks account access-key permissions, nonce and charging conditions, and creates an action receipt. Its nonce and account charging commit separately from the receipt's eventual action result. Function-call keys constrain target, method, action count and attached deposit; these checks do not establish signed economic postconditions (AR01–AR05).

The action receipt is the principal application rollback boundary. Its actions execute sequentially and stop on the first error. Storage changes enter prospective account-scoped trie state. Failure clears those changes and newly scheduled receipts; earlier successful receipts remain. The boundary has important exceptions: input dependencies are consumed and committed before action execution; gas, logs and profile survive failure; receiver gas rewards can be committed afterward. Therefore “failed receipt has no effects” is false, even though its action-state changes roll back (AR07–AR10, AR26).

WASM execution is prepared under selected runtime configuration and metered. Guest failure and gas must be deterministic across validators. An internal runner error represents a different category of failure. Generic execution correctness is distinct from satisfying an owner's signed formal economic intention (AR07, AR12, AR31).

Promises create later action receipts, and callbacks receive dependency results. A join gathers dependency identities and waits for their data, including failed results. It does not mean every branch succeeded and creates no global commit boundary. Partial branch success must remain visible to the settlement policy. The callback may execute after unrelated application changes; it must check current state rather than assume its initiating snapshot still holds (AR27, AR30).

Ordinary generated receipts retain original signer context while setting the immediate predecessor to the calling contract. This path does not create another transaction signature or consume another ordinary signer access-key nonce. Consequently a compiler must distinguish transaction replay protection from continuation replay protection. External inputs, relayer submissions, contract-internal continuations and delegated actions cannot be collapsed into one signer field. This review did not fully audit DelegateV2 or every gas-key/bootstrap path (AR25, AR28).

## Reference — adjudicated discrepancies and limits

| Draft topic | Independent finding | Consequence |
|---|---|---|
| GasKeys 85 versus 86 | Source feature mapping is 85; pinned docs explicitly say nearcore 2.13/protocol 86. | Source mapping is settled for this commit. Historical deployment timing remains unobserved. |
| Refunds always succeed | System-refund failure has an explicit burn branch. | Refund creation is not evidence of repayment. |
| First receipt determines status | Code follows `SuccessReceiptId` until a determining outcome; missing outcomes can leave Started. | Returned promise chains matter; unrelated branches need separate inspection. |
| Receipt rollback | Queued receipts and prospective writes are discarded; dependency consumption and gas accounting persist. | Complete effects include failure costs and protocol bookkeeping. |
| Callback panic example | A transfer can be queued before panic but discarded with the failed call/batch. | “Transfer line never runs” is imprecise; “no transfer receipt survives” is the defensible statement. |

Refund calculation also distinguishes unused gas on success from unburnt gas on failure, applies a refund penalty, and contains protocol-dependent price and account-creation adjustments. Deposit refunds follow the balance-refund receiver; gas refunds target the original signer/key path. A single undifferentiated refund field would lose authority and accounting information (AR18, AR19, AR21, AR29, AR33).

`FinalExecutionStatus` is not the same datum as a transaction finality wait level. This review verified status reduction, not every RPC finality predicate. The source's stable protocol constant is 88, minimum supported 84, nightly 157, and Spice binary 200 with feature activation 180. Those are source/configuration facts, not a verified current mainnet protocol observation (AR17).

NEP-509's state witness supplies trie evidence for replay; chunk endorsements and transaction signatures provide other assurance. Its scope explicitly excludes ZK integration. No reviewed path demonstrates proof that complete effects refine a signed formal intent. Proving an arbitrary program executed is insufficient if that program is not bound to the accepted formal relation (AR31).

Ordinary function calls do not require owner equality in the inspected actor-permission function; deployment/key changes do. This bounded fact supports permissionless programming with account authority, rather than maintainer-approved program admission (AR32).

## How-to — requirements for Moriarty

The required product remains a permissionless language for **all Midnight DeFi**, compiling to **ZKIRv3**, with **proof-carrying signed formal intent**. Partial transactions, contingent settlement and NEAR transaction patterns are required language semantics. They are not grounds to narrow the product to swaps, a hosted solver, an allowlist, or a NEAR backend. The following EARS requirements are proposed requirements, not claims of existing implementation:

- **AR-E01:** When an intent is signed, Moriarty shall bind the program and semantics version, complete-effect policy, admissible partial outcomes, observation policy, authority, cumulative fee bound and recovery policy to the signed payload.
- **AR-E02:** When a stage commits an asynchronous effect, Moriarty shall persist its retained effects, reserved assets, liabilities and outstanding duties in the state authenticated by the transition proof.
- **AR-E03:** When a continuation executes, Moriarty shall validate its identity, predecessor or observation authority, prior-stage commitment, current duty state and single-use authorization before applying effects.
- **AR-E04:** When a stage fails, Moriarty shall prove the permitted failure transition including retained earlier effects, consumed authorization, fees, refund obligations and residual liabilities.
- **AR-E05:** When joined dependencies become available, Moriarty shall evaluate the complete dependency-result vector against the signed settlement policy before declaring settlement.
- **AR-E06:** While an external result is unresolved, Moriarty shall retain an unresolved duty and shall prevent timeout alone from being treated as proof of nonexecution.
- **AR-E07:** When refund, retry or compensation is attempted, Moriarty shall authorize and prove it as a separate transition whose failure preserves the remaining duty.
- **AR-E08:** When an outcome is reported as settled, Moriarty shall prove that all effect-relevant branches and duties satisfy the signed terminal policy, including any explicitly permitted residual obligations.
- **AR-E09:** When a stage incurs a fee or changes refund destination, Moriarty shall account for the per-stage payer, beneficiary, cost and cumulative budget in complete effects.
- **AR-E10:** When compiled ZKIRv3 execution is accepted, Moriarty shall bind the proof relation to the signed intent and certified source-to-target semantics, including the actual Midnight ledger phase boundary.
- **AR-E11:** Where a program meets objective language, proof and ledger validity conditions, Moriarty shall permit its deployment and use without maintainer, council, reviewer or hosted-solver approval.

These requirements imply two distinct proof goals: every committed stage preserves authorized safety and duty invariants; terminal settlement discharges or explicitly carries the duties permitted by the signed policy. A pending stage must not be required to satisfy a final proceeds postcondition prematurely. Conversely, calling a workflow partial must not make its outstanding liabilities disappear. Liveness requires separately stated scheduling, availability, gas and authorized-recovery assumptions.

## Tutorial — reproduce and challenge the trace

Read AR01 → AR05 → AR26 to locate authentication, charging and receipt rollback boundaries. Read AR27 → AR30 for join dependencies and durable pending state. Read AR29 → AR19 for refund creation and refund failure. Read AR21 for the status-determining branch. The structural graph navigation is preserved in [graph-navigation.json](graph-navigation.json); its AST edges are navigation evidence, not semantic proofs.

A useful application scenario reserves a user's maximum debit, launches two dependent external effects and settles only after observing both results. Challenge it with one successful branch and one failed branch, then callback panic, duplicate callback, stale state, exhausted callback gas, unknown external execution, refund destination change, successful compensation followed by failed bookkeeping, and retry after an unknown result. The required outcome is an accurately preserved duty or a signed permitted terminal state, never fictitious rollback of the successful branch. These are proposed implementation tests; they were not executed here.

Run `python3 studies/astra-runtime/build_review.py` from this research tree to recheck source blobs and generated ledger/graph integrity. The performed check passed for 23 claims, 34 unique source ranges and closed semantic-graph endpoints. No prebuilt nearcore target directory was present; no nearcore build/test or network transaction was performed. [reading-inventory.json](reading-inventory.json) records bounded source reading. [paper-coverage.json](paper-coverage.json) explicitly claims no independently viewed PDF pages; preparatory PixelRAG coverage is not relabeled as this review's work.

## Dissent and missing evidence

Do not adopt “all actions roll back” as a theorem about all receipt effects; do not equate a successful returned chain with successful contingent settlement; do not require a new original-user signature for every internal continuation when prior signed authority explicitly permits it; do not treat an authenticated external observation as guaranteed availability or inevitable completion.

Missing evidence includes live protocol configuration, deployed code hashes, DelegateV2 and full gas-key/bootstrap behavior, complete finality wait predicates, independent PDF proof review, formal progress proofs, and the actual Midnight/ZKIRv3 correspondence. This study neither proves Moriarty's implementation nor certifies all NEAR runtime behavior. The target's breadth and permissionlessness remain requirements even where implementation evidence is missing.

## MPLR theory-log candidates

[MPLR-candidates.json](MPLR-candidates.json) provides six candidate requirements for the persistent Moriarty Programming Language Requirements theory log: durable duties, stage authority, contingent joins, unresolved observations, complete failure effects and permissionless proof correspondence. Each carries a behavioral requirement, exact NEAR sources, linked EARS requirements, open PL research questions and an adversarial acceptance scenario. Linear types, effect systems, temporal logics and translation validation are research candidates, not selected solutions. Stable MPLR IDs and vault filing belong to the coordinator.
