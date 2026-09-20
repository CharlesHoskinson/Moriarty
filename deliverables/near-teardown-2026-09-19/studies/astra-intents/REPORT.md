# Astra review: NEAR Intents accounting and conditional settlement

Independent panel review, 2026-09-19. Assigned model: GPT-6 Astra, medium reasoning. Pinned source: `near/intents@32a7836f825e8c984c26149f4456793ec7e3d49a`. This review validates the preparatory kernel study, makes three wording corrections and one durability qualification, and adds the adjacent escrow-swap implementation. Exact source citations and document capture provenance are in [claims.json](claims.json); review decisions and limits are in [REVIEW.json](REVIEW.json).

## Explanation: the acceptance boundary and its limits

The preparatory study's central distinction is sound. The verifier accepts signed operations against an internal asset ledger. Its acceptance checks authenticate authority, domain, deadlines and nonce freshness; its balance machinery checks quantities and conservation. These checks do not constitute a general proof that an arbitrary developer program executed correctly. `AuthCall` extends authenticated dispatch to another contract, but it does not synchronously incorporate that contract's arbitrary predicate into verifier acceptance. [IK01–IK04, IK11]

At `execute_intents`, no caller role allowlist appears; pause policy remains. The engine iterates signed payloads and verifies each envelope, extracts its application message, checks the current verifier account, deadline and current key association, checks versioned nonce salt/deadline where applicable, commits that account's nonce, and dispatches the typed operations. An empty operation list still consumes the nonce on successful execution. The nonce mutation is inside the execution boundary: it is not a separately durable transaction before the remaining operations. Conversely, successful verifier execution followed by a failed later withdrawal does not restore the nonce. [IK01, IK08]

The supported wallet envelopes and the finite Rust intent enum should not be mistaken for a general program-and-proof interface. The inspected NEP-413 acceptance domain is the verifier account, not a commitment to arbitrary code, a particular semantics version, predecessor/history or a ZK verification key. The source exposes privileged upgrade, full-access-key and force-withdraw capabilities. This establishes questions for a deployment audit; it does not establish present role holders or actual deployed WASM. [IK02, IK07]

`TokenDiff` means exact signed deltas. Eligible negative deltas incur rounded-up protocol fees, which credit the collector and participate in closure. NFTs and specified one-unit MT/IMT changes are exempt. A balanced batch can be economically unattractive while remaining authorized. Exact amounts plus conservation neither choose the best route nor prove external backing. The sample solver's optional service partner filter is separate from verifier caller access. [IK03, IK08–IK09]

The captured official documentation independently supports the simulation and ordering limits: simulation excludes external asynchronous effects, and cross-contract calls need not finish in intent order. Its broad introductory wording about custody and eventual destination delivery should be read alongside the concrete administrative and withdrawal paths, not promoted into a stronger source-level guarantee. [AI17]

## Explanation: the stronger escrow precedent

Graph navigation revealed `contracts/escrow-swap`, outside the preparatory kernel's selected files. It supplies a genuine funded waiting state. Maker funding increments `maker_src_remaining`; each interaction supplies parameters whose hash must match the stored commitment. A taker later transfers the specified destination token, and the contract checks the live/closed boundary, deadline, optional taker restriction and price. Partial fills are explicitly optional. These are application conditions evaluated against committed terms and persisted funding. [AI13]

This is closer to the user's requested **conditional settlement with composable evidence requirements** than a detached callback alone. The full requirement remains broader: submit a transaction to the destination, then allow settlement only when the signed combination of additional signatures, recipient signing, documents, proofs or other conditions is satisfied. Funded/locked programmable escrow is one specialization. A language must also specify whether an unfunded pending commitment reserves anything; submission alone cannot imply asset availability.

The inspected escrow implements specific funding/fill conditions. It does not supply arbitrary evidence accumulation, document attestation semantics or a universal predicate/proof interpreter. Its authenticated entrypoint checks a configured forwarding contract and currently dispatches only `Close`. No claim is made that broader evidence-based escrow is impossible elsewhere in the ecosystem. [AI13, AI15]

Escrow also exposes a significant residual-duty counterexample. On fill, it decreases remaining maker inventory and creates maker and taker payouts. Maker payout uses resolve metadata and persistent maker-loss accounting. Taker payout has no resolve metadata; state explicitly excludes lost-and-found for takers and fee collectors. The README warns about this limitation, and code confirms it. A taker payout failure cannot be represented as an outstanding taker recovery right by this particular contract. This is a documented design boundary, not an executed exploit or proof of a live loss. [AI14]

Close and cleanup are stage-sensitive. After expiry anyone may close; before expiry the maker may close with zero remaining source inventory, or the sole whitelisted taker may close. Cleanup additionally requires no tracked callbacks, no tracked maker balances, closed state and expiry. “No tracked obligations remain” is therefore weaker than “all parties have been paid.” [AI16]

## Reference: exact boundaries

| Surface | Source-established behavior | Boundary that must remain explicit |
|---|---|---|
| `execute_intents` | Signed typed operations, sequential engine checks, per-batch finalization | No durable partial-evidence workflow in this entrypoint |
| Nonce | Account-scoped use plus legacy/versioned handling | Intent nonce, obligation identity, continuation stage and settlement right are different concepts |
| Fees | Rounded-up fees on eligible debits; collector included in accounting | Signed economics and per-fill rounding need explicit policy |
| `simulate_intents` | Cached local effects and report; unmatched-delta error can be returned separately | No reservation, future inclusion or external-call execution |
| FT deposit | Token identity from predecessor; balance/supply credit | Token contract authenticity remains a dependency |
| Deposit plus execution | `refund_if_fails` invokes inline; other branch detaches later execution | Deposit may persist after detached execution fails |
| Ordinary FT withdrawal | Debit, promise chain, failed transfer result yields zero used and attempted internal refund | Refund requires resolver execution; earlier swap is not undone |
| FT transfer-call withdrawal | Failed promise result counts entire amount used; malformed successful result counts zero | Accounting policy is not evidence of actual recipient usage |
| `AuthCall` | Authenticated deferred `on_auth`; optional initialization and attached funds | No internal attached-wNEAR refund on failure; no arbitrary correctness proof |
| Escrow funding/fill | Persisted maker inventory and committed fill conditions; optional partial fills | Specific escrow protocol rather than universal evidence conditions |
| Escrow recovery | Maker loss tracked; taker/collector loss not tracked | Recovery symmetry must not be assumed |

Source claim IDs: IK01–IK12 and AI13–AI17. “Concurrent promises” in the preparatory report is read as absence of a guaranteed completion order, not a promise that the runtime executes every call simultaneously.

## Tutorial: distinguish three forms of incompleteness

Consider three independent conceptual traces. These are reasoning examples, not executed tests.

1. **Unmatched proposed exchange.** Alice signs an exact token diff that lacks counterparties to close it. Submitting that incomplete batch to the verifier yields failed acceptance; the contract does not preserve it as a pending obligation waiting for Bob's signature. A solver may gather signatures off-chain before submitting a valid batch. [IK01, IK03]
2. **Funded escrow awaiting its fill condition.** Alice funds a committed escrow with 100 A. The contract holds source inventory while awaiting an allowed taker's B transfer at the agreed price before the deadline. If partial fills are enabled, a valid smaller fill leaves residual A inventory. The waiting state is in the escrow protocol. [AI13]
3. **General evidence-conditioned destination transaction.** Alice submits an obligation whose release requires Bob's acceptance over the same obligation and terms, a committed document certified by a chosen issuer, and a proof about the allowed effects. Each evidence item may arrive separately. Funds release only once the signed combination is satisfied; otherwise the signed cancellation, expiry or recovery rules apply. This is a proposed Moriarty behavior, not a capability established for NEAR's verifier or the inspected escrow.

A fourth state can follow either a successful verifier withdrawal or escrow fill: payout has been scheduled but its outcome is not yet established. That is asynchronous effect completion. It is separate from intentionally withholding settlement while waiting for further authorization or evidence. Partial fills divide quantities; they are not automatically partial evidence protocols.

## How-to: carry the lessons into Moriarty requirements

Preserve the product contract: a permissionless language for **all Midnight DeFi developers**, genuinely compiled to pinned **ZKIRv3**, carrying proofs bound to a **signed formal intention**. Neither a curated template set nor project admission registry satisfies this. Application programs may still select recipients, counterparties, attesters and acceptable proof verifiers through signed policy.

Specify an obligation's identity, committed terms, stage and evidence requirements before selecting syntax or a theory. Define separately admission, funding/reservation, evidence acceptance, settlement eligibility, settlement consumption and recovery. A candidate need not force every application through every stage. Bind each proof to the selected program semantics, signed terms, predecessor/history, evidence commitments, effects and bounds. Signature verification authenticates authorization; document signatures authenticate an issuer's statement; neither establishes an arbitrary real-world assertion without an explicit trust assumption.

Make stage timing explicit. Admission authorization expiry need not revoke a valid admitted obligation or prohibit required recovery. Decide which evidence can be revoked and when; specify the signed policy for races among final evidence, cancellation, fills and expiry. A timeout means absence of an established outcome only when reliable contrary evidence is absent. A failed receipt can establish that local failure while leaving other effects unresolved.

Track residual duties for every affected participant. Keep token quantities, liabilities, refund claims and permission capabilities dimensionally distinct; relate them by typed invariants rather than adding them into one balance equation. Distinguish compensation from rollback. Preserve a right when payout evidence is missing or failure is known, with explicit rules preventing both double release and double refund.

Treat simulations and proofs as claims with a declared scope. Local acceptance evidence is not an external payment receipt. Proof soundness, compiler correctness and ledger correspondence remain separate obligations; this source review establishes none of them for Moriarty.

Six stable-ID-ready proposals in [MPLR-candidates.json](MPLR-candidates.json) record behavioral needs, evidence, open theory choices, adversarial scenarios and future PL-theory/experimental/deployed-language research. They are proposals for later reviewed filing, not canonical theory-log entries or novelty claims.

## Reference: audit trail and limitations

The AST graph was inspected before source and supplied the escrow navigation lead. All 25 preparatory selected-file hashes match their manifest. The principal acceptance, accounting, simulation, AuthCall, deposit/withdrawal, administrative and solver ranges were independently re-read. Three official documentation captures were hash-checked and compared with source. The escrow state, fund, fill, close, cleanup, authentication and recovery paths were inspected additionally.

The semantic fragment retains preparatory IDs and provenance. Three generic-interface relations were downgraded from extracted concrete calls/data relations to inferred implementation relations; escrow nodes and supported edges were added. No measured token accounting was available, so inherited zero placeholders are explicitly not evidence of zero usage.

No source repository or vault was edited. No financial transaction, deployment, exploit execution, build or runtime test was performed. Deployment configuration, actual privileges, build features and source-to-WASM correspondence remain unverified. Detailed NFT/MT verifier recovery and arbitrary downstream contract correctness remain outside this review.
