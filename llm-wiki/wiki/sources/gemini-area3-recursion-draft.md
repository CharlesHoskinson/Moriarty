---
title: "Draft: area 3 recursion and system architecture (Gemini)"
type: source
tags: [draft, final-design, recursion, gp-2, gp-3, gp-4, gemini]
sources: [gemini-area3-recursion-draft]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-final-design-20261111/drafts/gemini-area3.md
source_url: local
---

# Draft: area 3 recursion and system architecture (Gemini)

## Summary

Draft, not adopted. The Gemini lane's area 3 draft for run `sherlock-final-design-20261111` (listed in the lane's `GEMINI-DONE` file; the draft does not name the model) specifies four module kits: M10 NODE INTEGRATION, M11 DECIDER, M9 CONTINUATION and M12 HARNESS. It adds a system dependency graph, a wave build order, a canonical module kit template, a blueprint for a new OpenSpec 1.17.0 change (`sherlock-modular-chip-gp-20261112`), five prototype experiments P1 to P5, six open decisions (D-A3-1 to D-A3-6) and a conflict table. It was merged with the sister draft [[astra-area3-recursion-draft]] into `ARCHITECTURE.md` of the same run. Nothing was built or run: "no builds, `cargo test` runs, `cost_model` executions, key generations, Quint runs, or Lean builds were executed". It did not read the area 1 and area 2 drafts. GP-2, GP-3 and GP-4 all live inside M10 rather than in separate modules.

## Key Claims

- M10 defines four node-class relations at `k = 20`: `RunLift` R (`SHKLFT01`, verifies 1 or 2 [[ChipLeaf]] proofs and emits `SEG`), `ForeignLift` F (`SHKFOR01`, GP-3, adapter proof `A_c` under a witnessed VK plus two registry Merkle paths, emits a 1-claim `SET`), a polymorphic binary `Node` N (`SHKNOD01`) and `ClosingProof` `C_close` (`SHKCLS01`), which resolves the `{N, R, F}` fixed bases before the decider. In the M4 single-run fast path the decider resolves `{Node}` directly without `C_close`.
- N has four modes: `M: SEG + SEG -> SEG`, unary seal `C: SEG -> SET` (requires `h_lo == 0`, `term_tag in {1,2,3}`, `deps_d == 0`), `Resolve: SEG + SET -> SET` for GP-2 (`SEG.deps_d_post == slot_1.set_root`) and `U_set: SET + SET -> SET` for GP-4. `U_set` carries no `n_claims` counter and no `< 2^64` range check; settlement is once per distinct `claim_d` in `openClaim`.
- One `LeafBoundaryContract` trait with two swappable profiles: Profile A (dense page backend, Poseidon boundary, `Instance(0)` is the identity, 12 header scalars) and Profile B (ZmLogUp, Horner boundary, a witnessed KZG commitment that must use the subgroup-checked `assign`, 18 scalars, two-pass prover).
- Every node and decider range check uses `assigned_to_le_bytes(..., Some(8))`, so the decider's `Pow2Range` table stays at 1,032,467 rows: the draft derives this from the queried tag set `{0, 1, 4, 8, 14..19}` and gives 16,103 rows of headroom against 1,048,570 usable rows at `k = 20`. The measured decider is 929,166 circuit rows and 1,032,467 table rows.
- Row figures, all ESTIMATE: 2-leaf R 994,820 to 1,022,000 under Profile A and 1,013,000 to 1,062,400 under Profile B (straddles `k = 20`); 1-leaf `ChipNode` 794,416 to 809,903 (Profile A); N 760,000 to 855,000; F 550,000 to 750,000; `C_close` 700,000 to 950,000; `SHKRUND2` decider 927,500 to 930,500; `ClaimDecider` 810,000 to 860,000. Measured anchors: E8 1-leaf node 823,466 rows, self plus self probe 730,243 rows.
- Throughput: one `k = 20` node proof takes 61 to 67 s solo at 20 threads, 16.6 to 17.6 GB RSS (measured, cited from the GP study); about 95 to 115 s per worker under `3x6t` (ESTIMATE); about 2.2 s per step at full occupancy `S_eff = 16` and about 35 s per step at `S_eff = 1` (ESTIMATE); a 303-step `list123` run in about 12 to 15 min (ESTIMATE).
- M11 `RunDecider` becomes `SHKRUND2` with exactly four public scalars `[prog_d, input_d, outcome_d, n_steps]`; `claim_d = Poseidon("SHK_RUN", 1, prog_d, input_d, outcome_d)` with `n_steps` stored beside it, not hashed in. A new `ClaimDecider` (`SHKCLMD1`) exposes `[reg_root, set_root, shape_d]` for a `SherlockClaims` contract with `acceptRoot`, `openClaim` (up to 64 levels per call, deeper via sub-roots) and `admitKey`. See [[C6OutcomeOnlyPublication]], [[RunDecider]], [[AcceptRunContract]].
- Ledger boundary on [[MidnightLedger9]]: VK 2,189 B raw (measured), proof about 6,320 to 6,432 B (unlabeled), about 60 to 70 public inputs (unlabeled). The workspace proves with `midnight-proofs 0.8.3` but the ledger resolves 0.8.2; experiment P5 and gate SRC-020 must check this before any network call.
- M9 CONTINUATION: tier 1 is a streaming binary segment tree on `P = 3` workers of 6 threads plus 2 cores for the witness stream, with at most 64 open `SEG` proofs in RAM (under 768 KiB, ESTIMATE) and peak aggregate RSS about 50 to 60 GiB (ESTIMATE) under `MemoryMax=100G`. Profile B needs two passes because the challenges bind every leaf commitment.
- Mode 1 (production default) checkpoint `SherlockSegCheckpointV1` (`SHKSCK01`) stores a proven `SEG` prefix plus the retained segment store and resumes with 0 loader leaves strictly through `SEG + SEG -> SEG`; it must never be sealed, decided or consumed under GP-2. Mode 2 (`sherlock-ckpt/1` plus lazy `UNFOLD`) is quarantined behind a disabled feature flag because three problems are open: slices with `m > 8`, `n_steps` invariance and 39-field root coverage. See [[Continuations]].
- M12 HARNESS diffs per step against `sherlock-k-a/1` through an address-erased `LogicalStepObservation`, over 8 corpus vectors and four partition schedules (uniform 16, early close `[1, 3, 7, 16]`, single step, Mode 1 stop and resume) for both profiles. Tier A runs on every PR in about 25 to 55 s (ESTIMATE); tier B is the heavy real-KZG gate. A mutant earns credit only when the baseline passes, a weakened-constraint control passes, and the production circuit fails at the declared stage.
- Lean obligations: L-NODE-1 to L-NODE-3, L-DEC-1, L-CONT-1 and L-HARN-1, with new files `ModularRefinement.lean` and `ClaimDag.lean` and new premise classes `NamedAccAlgebra` and `ForeignPremises`.
- Build order: waves 0 to 4 with up to 11 parallel module lanes in wave 1 and freeze gates FG-0 to FG-4. P1 picks Profile B as the M4 default only if it is at least 20% faster end to end, 2-leaf R stays under 1,048,570 rows and Lean obligation 1 is complete; otherwise Profile A. P3 freezes 2-leaf R only if `R.rows <= 1,035,000`.
- Open decisions and recommendations: D-A3-1 2-leaf R under Profile A; D-A3-2 build the full `{U, R, F, N, C_close, D}` key set from day 1; D-A3-3 two-slot N, "surfaced explicitly to the owner"; D-A3-4 keep both `acceptRun` and `SherlockClaims`; D-A3-5 expose `shape_d` publicly; D-A3-6 an append-only `reg_root` history with a revoked set so long runs survive key admissions.

## Key Quotes

- "`SET` commits strictly to `set_root` (content-addressed Poseidon DAG digest), and settlement linearity is enforced per distinct `claim_d` nullifier in `openClaim`"
- "Reject `Option B` on modularity grounds (`MODULARITY.md:7-10`: duplicating VM rules inside `N` destroys leaf/node separation)."
- "Area 1 and Area 2 drafts in `research/sherlock-final-design-20261111/drafts/` were strictly not read"

## Connections

- Sister draft: [[astra-area3-recursion-draft]]. Merged result: [[sherlock-architecture]].
- Inputs it cites: [[gp-architecture]], [[runlength-consensus]], [[runlength-consensus-audit]], [[lookup-cache-consensus]], [[e2-e4-experiments]].
- Concepts and entities: [[GeneralPurposeRecursion]], [[Continuations]], [[ChipLeaf]], [[MoriChip]], [[DenseTailMemory]], [[ValueDigest]], [[RunLengthCeiling]], [[C6OutcomeOnlyPublication]], [[IvcNode]], [[RunDecider]], [[AcceptRunContract]], [[MidnightLedger9]].
- Lead decisions: [[final-design-decisions]]. Questions and status: [[OpenOwnerQuestions]], [[WorkstreamStatus]].

## Contradictions

- GP-4 "plus new work": Gemini recommends two-slot N, where combining two prior nodes plus new work takes two `k = 20` nodes, and asks the owner. Astra says "Two nodes do not satisfy the literal one-node requirement" and that failure of the one-node experiment leaves GP-4 incomplete ([[astra-area3-recursion-draft]], [[GeneralPurposeRecursion]]).
- Module split: Astra has separate CLAIM CONSUMPTION, FOREIGN ADMISSION and GRAPH COMPOSITION modules; Gemini folds all three into M10 and adds M12 HARNESS as a module.
- Tier names: Gemini follows [[runlength-consensus]] (tier 1 segment tree, Mode 1 retained store, Mode 2 portable); Astra's Tier 1 is a physical checkpoint and Tier 2 a logical relocation.
- Foreign proofs: Gemini's GP-3 path is an adapter `A_c` (Stage-1 Poseidon, Stage-2 Blake2b per P4); Astra says native adapters do not cover full GP-3 and a general verifier path is required.
- `shape_d`: Gemini recommends exposing it (D-A3-5); Astra keeps disclosure an open decision under D-6 ([[dissent-c6-section]], [[OpenOwnerQuestions]]).
- 2-leaf `RunLift`: Gemini recommends it (D-A3-1) and builds the throughput figures on it; lead decision L-10 says the 2-leaf `RunLift` is unbuilt and not part of the design until measured ([[final-design-decisions]]).
- Profile B: Gemini lets P1 make Profile B the M4 default and runs both profiles in every harness tier; L-5 makes Profile B benchmark-only and not built before M4 ([[final-design-decisions]]).
- Memory depth: P1 describes Profile A as a "depth-61/paged Merkle" backend; R-4 recommends TOP >= 63, and E-1 measured TOP = 63 as cheap ([[final-design-decisions]], [[RunLengthCeiling]]).
- Agreements: no `n_claims` counter matches R-5 and Astra's claim-count advice; Mode 1 composing through `SEG + SEG -> SEG` and never through GP-2 matches R-7. R-7 also names a checkpoint domain tag, which the draft does not mention.
- Internal: section 0 speaks of a "10-module / 13-unit" graph while sections 5 and 7 say 12 modules.
