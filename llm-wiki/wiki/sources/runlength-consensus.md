---
title: "Run-length consensus (sherlock-runlength-consensus-20261111)"
type: source
tags: [run-length, gp-1, memory, continuations, consensus]
sources: [runlength-consensus, runlength-consensus-audit, runlength-consensus-brief]
last_updated: 2026-11-10
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-runlength-consensus-20261111/CONSENSUS.md
source_url: local
---

# Run-length consensus (sherlock-runlength-consensus-20261111)

## Summary

Final report of the five Gemini 4 Carbon High lanes (A to E, two rounds) on the run-length bound, the D3 memory ceiling and continuations, asked for in [[runlength-consensus-brief]]. It answers Q1 to Q5. It was audited by Fable 5.2 in [[runlength-consensus-audit]], which says "adopt with changes" and corrects several of its claims. Where the audit contradicts this report, the lead adopted the audit's correction ([[final-design-decisions]], R-2 to R-10). The study ran no build, test, model check or proof; every figure beyond committed receipts is labelled `ESTIMATE` by the report itself (section 5).

## Key Claims

- **Q1, the meaning of 2^64.** 2^64 is the SRC-004 range guard on `prev_h`, `steps` and `h = prev_h + steps` (`ivc.rs`, `H_BITS = 64`). It stops `h` from wrapping modulo the BLS12-381 scalar prime back to 0 (genesis). No circuit has 2^64 rows: the leaf is `k_leaf = 16`, the node and the decider are `k = 20`. Proving 2^64 steps on this host takes ESTIMATE about 2.5 x 10^12 years. Unanimous (5 of 5) on substance. The audit confirms this answer.
- **Q1, proposed GP-1 and GP-001 text.** The report proposes replacement text that adds three counter bounds (steps, heap cells, claims, each below 2^64) and the words "every admitted program ... every valid input". The audit finds that this text narrows GP-1 and GP-4 and replaces it (see Contradictions).
- **Q2, memory.** All lanes reject keeping D3 and chaining runs through GP-2 (option a), widening D3 while keeping leaf-close padding (b), and the non-per-leaf variants (c1 to c5), including the D-1 backward cons-linked slices. All lanes adopt "M3-Tail64": dense 8-cell segment memory at fixed `TOP = 61`, `len_post = len_pre + used` with no leaf-close padding, the partial tail carried across leaves, a flush when `floor(len_post / 8) > floor(len_pre / 8)` into an empty slot (`old_leaf == EMPTY_SEG`); plus register-cached top frame and task with Poseidon hash-chain stacks for FRAMES and TASKS; plus slot-0 zeroing of scratch `v0..v2`. See [[DenseTailMemory]].
- **Q2, dissent D-Q2.1.** One domain-tagged arena with one flush unit (lanes C, D, E) against four arenas DATA, FIELDS, ENV, SLOTS with two flush units (lanes A, B). Kept side by side. D-Q2.2 (4 to 1): a 1-word `tail_d` per arena against Lane B's 28-word record-digest carry.
- **Q3, continuations.** Tier 1: segment recursion over a deterministic `state_d(h)` for any single-session run. Tier 2: cross-session epochs under a disjoint tag `SHK_CONT_V1`, rejected by RunDecider and acceptRun, carrying the unmodified `orig_input_d`, cut at `quantum_steps` (never through `fuel`), joined by `CONT-FOLD` in the node back to `h_lo == 0`, sealed once into one RUN proof. Two checkpoint modes: mode 1 retained-arena `canon_state_d(h)` (lanes A to D), mode 2 address-free `sherlock-ckpt/1` with lazy range-checked `UNFOLD` (all lanes; Lane E uses only mode 2, dissent D-Q3.1). See [[Continuations]].
- **Q4, prior art.** A 17-system matrix (RISC Zero, SP1, OpenVM, Ziren, Pico, Jolt, Nexus, Valida, Cairo, zkWasm, Miden, Airbender and others): production zkVMs decouple small segment traces from wide counters and do not publish memory roots or segment counts. Seven 2024 to 2026 advisories are mapped to the guards of Q2 and Q3. Dissent D-Q4.1 on SP1 v6 `main` against `dev`.
- **Q5, host throughput.** At `S = 16`, sequential linear IVC is ESTIMATE about 4.0 to 4.5 s per step (about 19,000 to 21,000 steps per day); a 3-worker binary tree is ESTIMATE about 2.74 to 3.15 s per step. A four-tier test schedule: G-B smoke, G-C with an unseen multi-segment program `P3`, an M5 endurance and 2-epoch continuation soak of at least 4,096 steps, and a three-partition replay plus a MockProver `2^64` boundary suite. Dissents D-Q5.1 (keep `S = 16` against Lane D's `S = 256`) and D-Q5.2 (the `P3` step floor: 512 against 1,024). The audit (finding 12) says these throughput figures assume full 16-step leaves and are lower bounds; several are unmarked in the report and the audit asks for them to be marked ESTIMATE. Per [[final-design-decisions]] R-10, they are not quoted as results.
- **What nobody checked (section 5).** No builds, `cost_model`, MockProver, model checkers or proofs; the 108-rule leaf is unbuilt; `R`, `CONT-FOLD` and the Lean and Quint extensions are unbuilt.

## Key Quotes

- "No circuit ever has `2^64` rows" (section 1, Q1).
- "Every unmeasured projection is marked `ESTIMATE`" (header).
- On D3: "Hard `131,071`-leaf (`2^17 - 1`) single-run wall" (section 1.2.1).

## Connections

- Brief: [[runlength-consensus-brief]]. Audit: [[runlength-consensus-audit]]. Lead decisions: [[final-design-decisions]].
- Concepts: [[RunLengthCeiling]], [[DenseTailMemory]], [[Continuations]], [[GeneralPurposeRecursion]].
- D3 and D-1: [[dissent-c6-section]], [[design-2-7-chip-leaf]], [[ChipLeaf]]. Claim counter: [[gp-architecture]].
- The three Astra drafts predate this report: [[astra-area1-processor-draft]], [[astra-area2-memory-draft]], [[astra-area3-recursion-draft]].

## Contradictions

The audit ([[runlength-consensus-audit]]) contradicts this report on these points. The lead adopted the audit's correction in each case where an R-item is cited ([[final-design-decisions]]):

- GP-001 text says "every admitted program ... every valid input" (from Lane E only); the current text says "every program ... every input". Audit blocker 1. Lead: R-2, use the audit's text.
- The claim counter `n_claims < 2^64` narrows GP-4 at a reachable DAG size (depth 64, about 128 proofs). Audit blocker 2. Lead: R-5 (OWNER).
- The heap-cell bound with `TOP = 61` can bind before the height field when a step appends up to 3 cells. Audit blocker 3. Lead: R-4 (OWNER), TOP >= 63 recommended.
- "All five lanes adopt" one memory design, but the arena count stays 3 against 2. Audit finding 4. Lead: R-6, arenas open.
- Mode 2 continuations "agreed by all 5/5" and "closing SRC-021 [OPEN]"; tier 2 composes "via GP-002". Audit finding 5: mode 2 not ready; GP-002 forbids that use. Lead: R-7, tier 1 plus mode 1 only.
- Slot-0 zeroing replaces write-before-read proofs. Audit finding 9: wrong. Lead: R-8.
- The D-1 linked slices rejected 5 of 5 from a code reading. Audit finding 13: D-1 needs the measurement the record requires. Lead: R-9, D-1 stays open.
- "+4..+12 node rows" and "+0 decider rows" as estimates of this design. Audit finding 11: they are the E2 fit for leaf k alone. Lead: R-10.
- D3 "FALSIFIES" SRC-023 and the Lean theorem. Audit finding 10: overstated; D3 makes the `det` premise impossible to discharge at digest level. No separate lead decision is recorded.
- [[astra-area3-recursion-draft]] uses "Tier 1" for a physical checkpoint and "Tier 2" for logical relocation; this report uses Tier 1 for intra-run segment recursion and Tier 2 for cross-session epochs. See [[Continuations]].
- This report rejects offline LogUp on the pinned stack (option c5); [[astra-area2-memory-draft]] rejects that categorical claim and keeps a two-pass committed-instance LogUp backend as a measured alternative.
- Other audit findings with no separate lead decision recorded: per-step maximum of reads and appends (6), overstated "5/5" counts (7; D-Q1.1 is 2 against 3), invariants of the flush design (8, taken into R-3), a wrong path for `verifier_gadget.rs` (14), an unsourced 73-row figure (15), line references (16), "2^64" against "`2^64 - 1`" cells (17), rows A33b and A34c that do not exist (18), the CVE-2026-62333 date (19), `k_leaf = 16..17` against the gate `k_leaf <= 16` (20).
