# DESIGN: four-piece VS for the sharded settlement

Author: `claude-opus-5-5` (runtime-stated), fresh context, not an author of the parent candidate. Status: **source proposal, uncompiled, unrun, unreviewed**. The user chose four pieces; that choice is not reopened here. Consequential sub-choices are listed in `DESIGN-VOTES.md` with my first positions only.

## 1. Problem

The parent run of C′ (raw, review pending) compiled everything and matched 472/472 expected test outcomes. Applied sizes: C2 2,770 B, VI 12,784 B, **VS 20,503 B**, S2′ 12,356 B. PS-S4 (VS ≤ 14,336 B) is false. The 16,384 B provider value is an unauthenticated capture, not a qualified node parameter, but VS exceeds even that, so it cannot be deployed as a reference script in one transaction on that assumption.

VS carried 15 per-thread W1 components plus global allocation, the VI binding and its own typed decode of every covered datum and redeemer. VI already carries the other 3 components and the signatures.

## 2. Architecture

```
C2 (no parameter, unchanged)
 └─> VI(c2_hash)                         unchanged bytes
      └─> VS3(c2_hash, vi_hash)          identity piece
VS1 ()                                    structure piece
VS2 ()                                    accounting piece
VS4 ()                                    custody piece
S2'(c2_hash, vi_hash, vs1_hash, vs2_hash, vs3_hash, vs4_hash)
```

The chain stays acyclic. VS1, VS2 and VS4 read nothing that depends on C2 or VI, so they take no parameter. VS3 needs `c2_hash` for `claim_script` and `vi_hash` to find VI's redeemer. S2′'s hash is still the signed-intent anchor (`settle_hash`) and names every piece through its parameters.

All five shards are withdraw-zero scripts with only `withdraw` and `else fail`. A settle transaction carries five zero withdrawals (VI, VS1–VS4), each with exactly one Withdraw redeemer.

### 2.1 Call graph (production path)

```
validators/s2p.ak  spend, Settle ──> shard4.delegated
                      ├─ identities_distinct(c2, vi, [vs1..vs4], own_hash)
                      └─ piece_lists(k, vs_k) for k = 1..4 ──> shard.shard_redeemer
                   mint, Genesis ──> shard4.genesis
                      ├─ thread.genesis (unchanged)
                      ├─ identities_distinct
                      └─ shard4.credentials_free(cat, [vi, vs1..vs4])
                   spend, Revoke/Close ──> thread.revoke / thread.close (unchanged)
validators/vi.ak   withdraw ──> shard.authorize (unchanged)
validators/vs1.ak  withdraw ──> shard4.vs1_structure
                      ├─ coverage(1) ──> shard.covered
                      ├─ thread.global_outputs_distinct
                      └─ structure_thread ──> thread.settle_base, settle_shape_structure,
                                              settle_shape_cells, settle_own_mint,
                                              settle_topup, settle_reserve_floor
validators/vs2.ak  withdraw ──> shard4.vs2_accounting
                      └─ accounting_thread ──> settle_base, settle_effects, settle_recomputed,
                                               settle_bounds, settle_snapshot, settle_intent_effects
validators/vs3.ak  withdraw ──> shard4.vs3_identity
                      ├─ shard.shard_redeemer(vi_hash) -> AuthShardV1
                      └─ identity_thread ──> settle_claim_script, settle_shape_digest_of,
                                             settle_shape_digest, settle_effects, settle_claim_input,
                                             settle_claim, settle_statement, settle_statement_digest
validators/vs4.ak  withdraw ──> shard4.vs4_custody
                      └─ custody_thread ──> settle_custody_pre, settle_effects, settle_snapshot,
                                            claimed_next (settle_claim_input, settle_next),
                                            settle_continuation, settle_payouts
```

Every leaf is an unchanged `pub fn` in `pvc2/thread`. No component was rewritten.

## 3. Interfaces and typed redeemers

| Script | Redeemer | Data shape |
|---|---|---|
| VI | `AuthShardV1 { thread_policy, threads: List<ThreadDigestV1 { outref, intents_digest }> }` | unchanged |
| VS1–VS4 | `VsPieceV1 { piece: Int, thread_policy: ByteArray, threads: List<OutputReference> }` | constr 0, fields positional; `piece` is 1, 2, 3 or 4 |

`VsPieceV1` replaces `StateShardV1` and adds the tag. Each piece accepts only its own tag. S2′ requires tag k in the redeemer under `Withdraw(Script(vs_k_hash))`.

**Output identities.** No piece creates or reads a new output. The continuation, payouts and reserve output are the W1 outputs, checked by VS4 and allocated once by VS1. No receipt, status datum or reference input carries provenance.

## 4. Partition (each W1 component exactly once)

| # | Component | Parent | Now | Stages it needs |
|---|---|---|---|---|
| 1 | custody_pre | VS | **VS4** | B |
| 2 | claim_script | VS | **VS3** | B |
| 3 | shape_structure | VS | **VS1** | B |
| 4 | shape_cells | VS | **VS1** | B |
| 5 | shape_digest | VS | **VS3** | B, SD |
| 6 | recomputed | VS | **VS2** | B, E |
| 7 | bounds | VS | **VS2** | B, E |
| 8 | claim | VS | **VS3** | B, C |
| 9 | authority_shape | VI | VI | B, E |
| 10 | intent_window | VI | VI | B |
| 11 | intent_effects | VS | **VS2** | B, E, S |
| 12 | intent_native | VI | VI | B, A |
| 13 | statement_digest | VS | **VS3** | B, SD, E, C, ST (VI's M digest) |
| 14 | continuation | VS | **VS4** | B, E, C, S, N |
| 15 | payouts | VS | **VS4** | B, E |
| 16 | own_mint | VS | **VS1** | B |
| 17 | topup | VS | **VS1** | B |
| 18 | reserve_floor | VS | **VS1** | B |
| 19 | global_outputs | VS (once) | **VS1 (once)** | transaction |
| — | signatures | VI | VI | M |

The cut follows code, not counts:

- **VS1 structure** needs no effect view, claim or value arithmetic. It holds the cheap structural checks and the once-per-transaction allocation scan.
- **VS2 accounting** holds the Core arithmetic: recomputation, bounds and `check_effects` (gross, fee, net, liability caps on the closing liabilities).
- **VS3 identity** holds everything that binds the step to a program, predicate, VK, claim and statement: claim script, shape bytes and digest, claim envelope, statement body and digest, and the VI binding.
- **VS4 custody** holds every check on physical value: custody before, the continuation datum and value, and the payouts. These share the `custody_value`/`assets` code.

## 5. Binding and coupling

**Exact coverage, per piece.** Each piece recomputes the covered set from the transaction (Settle-redeemer inputs at `Script(thread_policy)`, input order) and requires its `threads` to equal it, non-empty, with a 28-byte policy and its own tag. All four name the policy S2′ checks, so all four cover the same set. A piece cannot cover a subset, a superset, a permutation, a duplicate or an empty list.

**Non-bypassable at settlement.** Every Settle spend of S2′ requires, for k = 1..4, that `Withdraw(Script(vs_k))` is in the withdrawals with exactly one redeemer, decoding as `VsPieceV1` with `piece == k`, `thread_policy == own_hash` and `own_ref ∈ threads`. A present withdrawal makes the ledger run the script with that redeemer (premise P-L1; SRC at the reviewed ledger pin, deployed node open). VS3 requires VI in the same way, as VS did.

**Aliasing.** `identities_distinct` requires the C2, VI, four piece hashes and the S2′ policy to be seven distinct 28-byte values, at genesis and at every Settle spend. If two piece parameters were equal, one script would satisfy two slots and the other piece's checks would be skipped. Tags close the related mis-slotting case: a genuine VS1 applied in the VS2 slot refuses its own redeemer. A fake always-true script in a slot cannot be detected on-chain. It produces a different S2′ hash, and owners' signed intents name the S2′ hash (identity chain, M-HASH).

**Intermediate values.** No new value is published.

1. VS3 reads VI's per-outref intents digest exactly as VS did. VI re-checks it; VS3 requires equal policy and coverage.
2. VS4 builds the next datum with the **claim envelope's** `statement_digest` (`shard4.claimed_next`) instead of recomputing the statement. VS3 accepts only if that envelope digest equals the recomputed statement digest. Both pieces select the claim with the same deterministic function (`settle_claim_input`: the one claim input at `Script(td.claim_hash)` naming this outref), so under the conjunction VS4's datum is the monolithic one. `q_forged_claim_digest_vs3/vs4` refuse a forged envelope in both pieces.

**Cross-group.** All pieces name the S2′ policy. VS1, VS2 and VS4 have one hash shared by every deployment. A withdrawal credential appears once per transaction, so two S2′ deployments cannot settle in one transaction (liveness limit, unchanged in kind). RR-16 (another S2 version outside global allocation) is unchanged.

**Reference inputs.** Coverage, claim selection and custody read `inputs` only. A thread or claim offered as a reference input is not covered and not consumed (`q_thread_reference_input_not_covered`, `q_claim_reference_input_*`).

**Shard credentials as authority.** `credentials_free` now excludes all five shard credentials from account native co-authority and the reserve owner. This is the parent's guard extended to the new credentials. SR-06 (also excluding `Script(c2_hash)` and the S2′ policy, D-3 replacement) is **not** implemented: it changes acceptance beyond the split.

## 6. Accepted-set argument (source argument, not a proof)

Let T be a transaction whose covered set C under policy P is non-empty.

- **No new acceptance.** If every S2′ Settle spend, VI and VS1–VS4 accept, then for every t ∈ C each of the 19 components holds on stage values equal to the monolithic ones. VS3 computes the shape digest itself and uses VI's digest, which VI checks equals `settle_messages(…).intents_digest`. VS4's next datum uses the envelope digest, which VS3 checks equals the statement digest. The union of stages evaluated is the monolithic set. So monolithic `thread.settle` would accept each t on the same transaction.
- **No lost acceptance**, modulo transaction shape. If the monolithic gate accepts each t, it evaluated every stage without halting. Each piece evaluates a subset of those stages on equal values (VS4's digest substitution is equal there) plus binding conjuncts that hold for an honestly built transaction. Additions change only `withdrawals` and `redeemers`. `global_outputs_distinct` reads Spend purposes only. `native_ok` could change only for a shard credential, and genesis refuses those catalogues.
- **Halts versus False.** Pieces build some stages without the earlier components that guarded them in the monolithic order (for example VS4 builds the snapshot before VS2 checks recomputation). On malformed input a piece can therefore halt where the monolithic gate returned False. Both refuse the transaction. On R1 BOOLEAN cases every stage is known to evaluate without halting, which is why the split predictions are exact except H12's VS4 slot.

## 7. Cost and size (estimates are estimates)

**Size.** Each piece carries its own copy of: the typed `ThreadDatumV2`/`SettleRedeemerV2` decode, `covered`, `spend_redeemer`, `settle_base` and the piece redeemer decode. That shared overhead is the main size risk. My rough attribution of the 20,503 B VS: about 4.5 KB shared decode/coverage/base, 1.5 KB effect view, and the rest component code. On that attribution:

| Script | Estimate (applied) | ≤ 14,336 B confidence |
|---|---|---|
| VS1 | 7.0–10.5 KB | moderate-high |
| VS2 | 8.5–12.0 KB | moderate |
| VS3 | 9.5–13.5 KB | low-moderate |
| VS4 | 9.5–13.5 KB | low-moderate |
| S2′ | 12.5–13.4 KB | moderate-high |
| VI | 12,784 B (unchanged) | observed in parent |
| Sum VS1–VS4 | 35–49 KB (> 20,503 B) | high |

If the shared decode alone is above about 8 KB, VS3 and VS4 miss the cap. The next lever would be a decode change, which is a separate design choice (DESIGN-VOTES N-8), not part of this candidate.

**Work per settle transaction** (n covered threads, logical counts from the call graph): typed decodes of covered datum and redeemer 2n → 5n; settle bases 2n → 5n; effect views 2n → 4n; claim scans n → 2n; snapshots n → 2n; shape bytes and statement n (unchanged); custody values 2n (unchanged); global allocation scan 1 (unchanged); S2′ shard-redeemer scans n → 4n; signatures unchanged. Same-transaction splitting adds ExUnits; it never creates budget. PB-02 stays open and is predicted worse.

**Ledger-side consequences** (unobserved): five zero withdrawals and five extra Withdraw redeemers per settle; five stake credentials to register (tag-0 certificate per D-6 replacement), each deposit locked permanently per the earlier ledger reading; about 65–75 KB of reference scripts referenced by one settle transaction (C2, VI, VS1–VS4, S2′), which is below the Conway per-transaction reference-script limit as I recall it (200 KiB, unqualified) but raises the reference-script fee.

## 8. Chosen deployment path

Each script is deployed as its own reference-script output in its own deploy transaction, and the settle transaction references all of them. The fit criterion is per script: applied bytes ≤ 14,336 B (the existing M-SZ recommendation, leaving deploy-transaction margin under the unauthenticated 16,384 B). Deploy-transaction and settle-transaction sizes are unobserved.

## 9. First actual blocker

No observation of these bytes exists. The first blocker to the stated capability is a compile/build/apply/size run of this exact candidate, which needs a new operator (the parent operator pins the parent's 25 files, three-hash chain and 472-test index), a fresh independent source/effect review, and the root's exact authorization. `EXECUTION-PROPOSAL.json` specifies that run. After it: FX1, X1 Withdraw-purpose contexts, a concrete Conway settle transaction, and deploy facts (node identity, registration, M-HASH, PB-10, PB-12).

## 10. Residuals carried (not fixed)

SR-06, SR-08, SR-09, SR-11 (now tested by `q_p07_shared_transaction`, unrun), SR-12, SR-15, SR-16; the debtor-consent item (open, unvoted); D-7 (not agreed); OBS-SH-01; RR-03, RR-08, RR-16; PB-01..PB-12; FX1, X1/PX04, OEX7, K2, M-HASH. Sharding addresses none of them except the size of the VS code.
