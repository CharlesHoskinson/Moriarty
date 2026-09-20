# NEAR transaction patterns as Moriarty staged transactions and contingent settlement

Status: bounded independent pass over supplied primary excerpts only. No repository, page or network was read beyond the excerpts quoted in the prompt. No code was run. The three preparatory drafts were treated as untrusted; every proposition below is marked as verified from a supplied primary excerpt, taken from documentation text only, draft-only, or a design inference. The controlling document is /home/charl/Moriarty-aeon-study/docs/MORIARTY-PRODUCT-CONTRACT.md lines 5, 11, 33-39 and 64-68.

## Explanation: what NEAR actually gives a staged transaction

NEAR has no global transaction. It has a signed transaction that is converted into one action receipt, and receipts that spawn further receipts. The verified excerpts establish the shape precisely.

A transaction passes config checks and a signature check over the transaction hash (transaction.rs L327-343). The ordinary access-key path then checks nonce against the effective nonce, available balance against total cost, allowance, storage stake and, for function-call keys, the single-action/zero-deposit/receiver/method restriction (verifier.rs L406-497, L298-344). None of this states anything about economic outcome. It is native authorization and resource validity, and Moriarty's table at product-contract lines 45-52 classifies exactly these things as owner authorization and Midnight execution conditions, not as intent refinement.

A successful verdict creates a receipt and reports SuccessReceiptId (runtime/lib.rs L2234-2283). The signer's amount, nonce and key are committed under TransactionProcessing (L2325-2354) before any action executes. So authority and fee consumption are committed at a boundary that precedes effects. That is the first pattern Moriarty must carry: consumed authority and nonce are a phase-1 fact that survives later failure.

Receipt execution consumes input data dependencies, removing them from state and mapping Some/None to PromiseResult::Successful/Failed (L860-898), commits pending updates before running actions (L902-904), then either commits or rolls back prospective state for that receipt only (L1077-1087). TrieUpdate::rollback clears only the prospective map and pending deploys (update.rs L225-227). Gas burnt is computed on the failure path too (L1089-1094), and a failed function call still adds burnt gas and logs while withholding new receipts (function_call.rs L101-152, L226-227). This is receipt-local atomicity with retained cost, which is the same shape as Midnight's guaranteed/fallible phase split described at product-contract line 39.

Callbacks are data dependencies, not control flow. create_action_receipt appends output_data_receivers to the receipts it waits on and records input_data_ids on the new receipt (receipt_manager.rs L112-137; receipt.rs L590-606). promise_then is sugar over promise_batch_then plus a function call (host.rs L2432-2458). The VM context separately exposes signer, predecessor, refund_to, promise_results, attached deposit and prepaid gas (context.rs L11-64). The docs state that between call and callback any public method may run (reentrancy.mdx L6-8), that no automatic rollback exists (crosscontract L35, L449-459), that batches to one contract act as a unit while parallel calls to different contracts are independent (L465-483), and that a callback that panics after removing a pending record reverts its own cleanup and leaves the obligation stuck (callback-panics.mdx L28-35).

Liveness is a separate axis. Cross-shard receipts are buffered when outgoing gas or bandwidth limits are hit (congestion_control.rs L49-74, L98-117, L159-175). Gas is split into burnt and reserved-for-promises with two limits (gas_counter.rs L126-149). Chunk validation replays recorded storage and checks receipt-proof shard IDs, Merkle root, transaction root and encoded root (chunk_validation.rs L532-566, L764-790); NEP-509 assumes at most one third corrupted stake, never rolls back state, and puts ZK integration out of scope (nep-0509.md L30, L36, L104-109). Doomslug readiness needs strictly more than two thirds of stake (doomslug.rs L267-283). A state witness is validator replay evidence, not a proof of a user's signed outcome.

The Intents layer adds an application acceptance kernel on top. From documentation only: intents in a batch execute in order but their cross-contract calls need not complete in order (intent-types L22-41); nonces carry a rotating salt (L119-131); a withdrawal storage_deposit is not refunded on failure (L597-598); simulation excludes external asynchronous effects (simulating-intents L481-487); the solver relay's guaranteed delivery is at-least-once with acknowledge-then-process and dedupe by seq (guaranteed-delivery L275-348). The engine source and the ft_resolve_withdraw branches described in the intents-kernel draft were not supplied as primary excerpts and remain unverified here.

## Reference: Moriarty types for staged, contingent execution

These are design proposals for Moriarty semantics, not implemented features. Names are provisional.

**Stage.** A transition bound to program identity and semantics version, stage identifier, authenticated predecessor, the signed intent, authority in and authority out, complete effects, duties in and duties out, cost bound and the pinned Midnight phase layout. A stage is the unit that a proof statement binds (product contract line 31).

**Receipt.** An authenticated data dependency produced by a stage and consumed by exactly one continuation. Its payload is a typed sum: Success(value), Failed(reason), Unknown(observation set). Success and Failed correspond to NEAR's Some/None input data (lib.rs L893-894). Unknown has no NEAR native analogue for same-chain callbacks, because the docs say the callback always runs if the origin succeeded (crosscontract L425-432); it is required for external chains, bridges and Midnight fallible phases where the observation may never arrive.

**Continuation.** {stage_id, awaits: receipt set, join: All | Any | Quorum(k), resume_token, expiry policy, allowed_next transitions}. The resume token is one-shot: consuming it is part of the stage's effects, mirroring the removal of ReceivedData from state (lib.rs L882-885). Resuming twice is a predecessor mismatch and must fail to prove.

**Authority.** Affine spending authority derived from signed intent: exact asset identity or explicit substitution predicate, recipients, gross debit, fee and liability bounds, minimum net outcome, validity, replay and revocation rules, allowed partial completion (product contract line 33). Authority only decreases across stages. It is distinct from a NEAR function-call key, which restricts invocation (verifier.rs L298-344) and says nothing about balances or debt.

**ResidualDuty.** A persistent liability: owed_to, asset, amount, kind (Refund | Settle | Deliver | Compensate), discharge conditions. Duties obey opening plus created minus explicitly discharged equals closing (product contract line 35). A stage that fails cannot delete a duty; only a discharge transition can. This is the formal version of the callback-panics rule.

**Outcome state.** prepared, authorized, pending, partial(filled), settled, failed, unknown, compensating, recovered. Timeout moves pending to unknown or to expired-with-late-arrival-policy. It never moves to failed on its own.

**Join semantics.** All: resume when every awaited receipt exists; the continuation sees each result separately and inherits the union of surviving duties, matching NEAR's P1.and(P2).then(P3) (crosscontract L226-235). Any/Quorum: resume on the first k arrivals, but every late receipt must still be consumed by a declared absorb transition that either discharges or re-records its duty. No NEAR native any-join is documented; independent parallel calls simply proceed (L478-483), so a late branch is a real resource that must not be dropped silently.

**Partial fills.** The intent declares allowed partial completion and minimum net outcome. Each fill receipt strictly reduces remaining authority and increases filled amount; the proof of every fill stage must show cumulative fills stay within gross debit and fee bounds and that residual authority plus refund duty equals the original reservation.

**Refund versus compensation.** Refund discharges a duty by returning an asset that was reserved and never released. Compensation is a new transition that creates a credit or a new external effect after an external release already occurred; it can itself be pending or fail, and it does not rewind history. Both are permitted transitions only if the signed intent admits them. The NEAR docs' fallback of crediting a withdrawable balance (callback-panics L53) is the compensation form.

**Native check versus proof-carrying intention.** Native, checked by the ledger: signature, nonce, revocation, balance, fee affordability, storage, phase validity, verifier key identity. Proof-carrying, established by the bound relation: intent refinement, transition validity, authority monotonicity, duty conservation, history linkage, complete-effect accounting. A valid signature or a state witness discharges only the first column.

## How-to: translate a NEAR pattern into Moriarty stages

1. Identify the NEAR commit boundaries you rely on: transaction processing commit (lib.rs L2352-2354), receipt start commit (L902-904), receipt end commit or rollback (L1077-1087). Map each to a Moriarty stage and a Midnight phase in the pinned layout.
2. For each cross-contract call, replace the implicit promise with an explicit pending transition that records reserved authority and a Settle-or-Recover duty before the effect leaves the stage. Do not rely on a later callback to create the duty; NEAR shows the callback may revert its own work.
3. Type every callback as a continuation with a one-shot resume token and an authenticated receipt. Re-check current state and remaining authority at resume; reentrancy between call and callback is documented behaviour.
4. Enumerate failure outcomes: Failed, Unknown, gas exhausted, phase partially applied. Each gets an explicit transition that retains fees and consumed nonces, following the burnt-gas-on-failure path (function_call.rs L141-148).
5. For any-joins, write the absorb transition for late branches before writing the happy path.
6. Bind cumulative fees, recipients, custody and liability evolution in the complete effects of the last stage, not in an off-chain simulation.
7. Keep deployment permissionless: the inspected deploy action requires actor equal to account (actions.rs L755-775) and code/storage accounting (L290-327) with no allowlist in the excerpt. Moriarty may require certified semantics and proof validity, never reviewer identity.

## Tutorial: reserve, contingent release, timeout

Alice signs an intent: debit at most 100 A plus fee bound 1 A, release A to Solver only against accepted evidence of 200 B delivery, partial fills allowed to a minimum net of 195 B, refund after expiry E under policy P.

Stage 1 (authorized to pending): consumes nonce, reserves 100 A in custody, records duty D1 = Refund-or-Settle 100 A owed to Alice, emits Receipt R1. Fees charged here are bound in the intent and survive any later failure.

Stage 2a (pending to partial): receipt R2 carries accepted evidence of 120 B delivered. Proof shows 60 A released to Solver, remaining authority 40 A, D1 reduced to 40 A, filled 120 B, cumulative fee within bound.

Stage 2b (partial to unknown): expiry E passes with no further receipt. State becomes unknown for the remaining 40 A. No refund yet. D1 stays live.

Stage 3 (unknown to recovered or settled): either a late receipt shows 80 B delivered and 40 A is released, or policy P's bound condition is met and 40 A is refunded, discharging D1. If evidence later shows an external release already happened for the same 40 A, the only admissible path is a Compensate duty, not a second refund.

Adversarial checks the pipeline must reject: a second R2 with the same resume token; a resume against a predecessor whose custody was altered by an interleaved stage; a callback that records failure but whose fallible phase reverted the duty update; a solver-supplied success flag with no bound observation; a lowerer that inserts a fee-on-failure policy not present in the intent; a refund transition issued while D1 is unknown; a fill that substitutes asset identity without a signed substitution predicate.

## Limits

Nightshade and Doomslug papers, the intents engine source, the MPC contract and the omni-bridge were not supplied as primary excerpts. Live protocol activation, deployed code hashes and governance holders are unknown. All Moriarty types above are proposals and impose objective proof obligations that no current artifact discharges.