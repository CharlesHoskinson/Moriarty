# Dissent and recorded conditions

Status: CONSENSUS v2.1 (council vote 6/6 ACCEPT at 2026-11-07 21:29Z; text unchanged from DRAFT v2.1; amended 2026-11-08 (3) by the owner decisions in `dissent.md` "Resolved 2026-11-08", "Resolved 2026-11-08 (2)" and "Resolved 2026-11-08 (3)"). Unresolved disagreements are listed first; conditions that resolved an issue are kept with the member's words so the resolution can be checked. Votes and any OBJECT are appended in Phase 4.

## Unresolved at DRAFT v1

None at v1 issue stage; see 'Phase 4 votes' for the objection and revision record. Every ISSUE 1-10 reply received by 21:05Z was ACCEPT or ACCEPT-with-conditions; the conditions are written into the requirements named below.

## Conditions recorded verbatim

### ISSUE 4 (dispatch receipt coverage) -> SAS-013, SAS-014

astra-1: "the proposed tuple is not yet a demonstrated dispatch-equivalence relation. Derive its complete dependencies from the actual guards and paired premises. PROG, referenced values, bounds and canonicality checks can affect acceptance beyond control tags. Show why states grouped together preserve each dispatch predicate under StepInv, or label the grouping a coverage heuristic. Add directed cases for every admitted rule and priority boundary absent from P1–P3. Do not call a reached-class sample full-rule coverage."

astra-2: "support partitioned receipts if the partition uses complete dispatch predicates, adds directed cases for every admitted rule/priority boundary absent from P1-P3 (or restricts admission), tests every competing rule per class, includes coordinated/raw-field mutations, and labels finite preprocessing scope."

astra-3: "equal_differ reads UInt/Bool/type/constructor values and flips EqualDifferent vs EqualPair while listed tags/phase/presence can stay fixed (c6_dispatch_step.rs:750-806,1189-1200). Fix: complete guard dependencies and StepInv premises, directed every-rule/priority cases, explicit finite scope. I accept bounded partition receipts with that fix, not only the old exhaustive test."

### ISSUE 6 (leaf shape, S) -> SRC-013, SRC-014

astra-1: "Require S=1 as the initial qualified baseline. Treat S=8 as one measured candidate, not a promised hour-long proof or mandatory final shape."

astra-3: "S=8 and the one-hour estimate are experiments, not measured complete-path results."

astra-2: "Family F needs authenticated in-circuit key membership and compatible interpretation."

### ISSUE 2 (keys at re-pin) -> SRC-009, SMN-002

astra-3: "Disagree with assuming all current keys invalid: Cargo already uses ledger-v9; release notes preserve older verification and identify specific changed circuits. Test the exact candidate, then regenerate the affected closure."

astra-1: "A historical stagenet result remains historical evidence rather than becoming false."

### ISSUE 1 (mainnet scope) -> SWF-008, SWF-009

fable-3: "This was my stated fallback; I withdraw 'successor change'." astra-2: "Mainnet workflow stays in the requested design scope; implementation additions may be explicit future tasks."

### ISSUE 3 (stagenet) -> SWF-011

fable-3: "existing rows A8/A9/A10/B4/B8 cannot be 'frozen' without lying - predicates.yaml holds no results, and status.json will keep A8 as firstFailing until a call lands on 2391d62f." astra-1: "retire superseded stagenet predicate definitions from the active catalog while preserving their historical runs/receipts and crosswalk, or name the exact scoped-next/check addition needed." Resolved by E10 `retired:`.

### ISSUE 9 (throughput) -> SRC-011

astra-1: "A reported shortfall can satisfy a measurement task, but cannot claim that the throughput target was achieved." astra-3: "shortfall reporting closes measurement only, not a promised performance requirement."

### Genesis accumulator finding -> SRC-005

astra-2: "This review did not establish a practical cancellation attack or a forged accepted history. Please preserve that qualification when describing the current code." fable-2: "Whether this is exploitable depends on how `verifier.accumulate` combines its inputs ... the safe fix is one line."

### 70-lane decider record -> SAS-004

astra-1: "do not automatically apply the qa_mutation panic-accounting defect to the historical 70-lane native verifier result ... It does not call qa_mutation." fable-2: "I withdraw 'ASR-003 applies retroactively to the M4 record'; the record needs provenance ... and the label 'existing-proof public-input binding, not malicious-witness soundness'."

### ZKIR version wording -> SRC-008

astra-3: "Compact 0.36 release notes explicitly say its bundled zkir-v3 binary comes from zkir-3.2.0-rc.2, while the compiler continues to generate ZKIR 3.1." The manifest records the emitted header, compiler version and zkir binary version as three fields.

### Deployed-source gap -> SMN-004

astra-2 / astra-3: "local source and live runtime provenance remain distinct while deployed source mapping is unavailable"; "preserve source/binary reproduction gaps and recovery limits."

### Early draft corrections applied before DRAFT v1 (21:02-21:11Z)

astra-2: "design 2.2 uses post==pre under 1-sel[r]. If inactive fragments constrain the shared post this way, they block the selected fragment's updates. Use per-rule candidate outputs or selected-rule preservation masks" -> §2.2(4). "leaf derives rules_d/steps, node binds h_pre" -> §2.2(6), §3.2(3). "Preview exit must pass all mandatory security/assurance gates for its claimed profile" -> proposal exit. "Unsafe-key retirement must be observed on chain" -> SWF-010, §7. "O2/O3 universal ... cannot be closed by finite hostile/differential tests" -> assurance §1.

astra-3: "enable=0 must accept constrained identity padding with zero count; unsupported enable=1 must refuse" -> SVM-004, §2.2(5). "one application producer must not become one oracle" -> SVM-007, §2.5. "option F ... State that F initially uses S=1" -> §3.4. E10 sweeps -> SWF-011.

fable-2: "leaf stand-in k=13" -> "C6-shaped digest leaf (M2)"; re-key closure includes the leaf; VK-change classes informative only; genesis scaling is irrelevance not refusal -> §3.1, §3.2(2), §5.2(2); SRC-007 nonzero-low-byte withdrawn.

astra-1: "Freeze the supported semantic profile, admitted rule inventory, layout/caps and relation/key manifest before P3 is authored" -> §2.6, §3.4, SVM-008. "SRC-007 must not require a nonzero low result byte" -> SRC-007. "SVM-006 must permit constrained public digests" -> SVM-006. "SAS-013 needs justified equivalence, not merely mechanical origin" -> SAS-013.

### Late corrections applied as DRAFT v1.1 (21:11-21:20Z)

astra-3: SWF-005 commit order (receipt commit, then `state-commit`, `evalloop.py:39-46`); freshness by dependency closure, HEAD flag interim (SAS-002, E7); retirement applies to sweeps; S=8 after the S=1 profile (task 4.8). SAS-011 scenario is a manifest mismatch, not a V2 ban; SRC-004 scenario must reach the constraint boundary; SAS-013: a partition known to omit a guard dependency does not satisfy the receipt.

fable-3 W1-W5: retire B5/B9 too and land E10 before `retired:` fields; E12 evaluator-owned network parametrisation of the rejection suite and U023 holdout; E13 network-parametrised SDK scripts (M6 commands created in 6.1); `validate` ignores unknown keys today; numbering.

astra-2: §3.2(2) genesis text replaced (transcript observation is not a cancellation proof); §2.2 composition is a required check with initialization and shared-carrier premises; SVM-002 table and scenario synchronized (refuse or checked unreachability; a bad pre-state is not a full-run exploit).

## Phase 4 votes

### DRAFT v1 (21:15Z): fable-2 ACCEPT; astra-1 OBJECT; astra-2 OBJECT; astra-3 OBJECT; fable-3 pending at 21:20Z.

Objections (soundness/assurance items marked *): astra-1 1* (finite release evidence column, premises in O1-O4, composition as a required check), 2* (genesis inference removed), 3* (IR preprocessing class for M3 receipts; SAS-013 refusal; SRC-004 boundary), 4 (task order: inventory R, P3 after freeze, S=8 after profile, SRC-014 conditional, M4 naming), 5 (receipt commit then state-commit; dependency-closure freshness; E12/E13; B5/B9), 6 (SAS-011 mismatch; SWF-010 merge; SMN-007 immutable permitted). astra-2 1*, 2* (same as above plus an initialization task), 3 (workflow runnability), nonblocking (label-forgery and nonmember-key tests; S=8 order). astra-3 1*, 2*, 3, 4, 5* (SAS-008/009 applicable-when-triggered; SRC-014 conditional), 6. fable-2 editorial 1-4 (heading IDs, SRC-004 wording, stand-in wording, 5.2 negatives) and support for the four recursion-lens fixes.

### DRAFT v1.1 delta votes (21:19Z): fable-2 ACCEPT; fable-3 OBJECT (workflow runnability: receipt commit order, E7 freshness, E12, E13, B5/B9); astra-1 OBJECT pending synchronization; astra-2 OBJECT narrowed; astra-3 OBJECT with four synchronization fixes plus the optional-B gate note.

### Revision round 1 -> DRAFT v2 (21:30Z)

Also applied from the v1.1 delta reviews: SRC-005 relabelled consistently in requirements, spec, tasks 2.3 and 5.2 ((a) neutral encoding, (b) zero/ignore with justification; unscaled retained carry needs its own argument); SAS-002 spec and workflow E7 rewritten to dependency-closure freshness with a state-only-commit scenario and no blanket directory exclusion (astra-2); gate rule and task header no longer say "current head"; SMN-007 "immutable, recovery mode stated; disposable only without active state"; new SWF-012 / E14 `optional: true` rows so unimplemented B rows and the S>1 experiment cannot block `next` or `t3.sh` (astra-3). All items from the v1 list above applied: assurance §1 table recolumned with premises and "finite release evidence"; new evidence class "IR preprocessing / witness execution" used for M3; design §2.2 composition as a required check, §3.2(2) genesis text per astra-2, headings corrected; SRC-004/SAS-011/SAS-013 scenarios corrected; SRC-013 names M4 and adds the nonmember-key scenario; SRC-014 conditional on S>1; proposal exit treats SAS-008/009 as applicable-when-triggered; tasks 1.0/1.1 (E10 first, B5/B9 retired), 1.4 (E12 evaluator), 2.3 (genesis realization recorded before keygen), 2.4 (no stand-in), 4.4 (inventory R), 4.7/6.4 (P3 parity after authorship), 4.8 (S=8 after 5.3, conditional), 4.9 (initialization), 5.2 (label forgery, nonmember key), 6.1 (E13); SWF-005/SAS-002/E7 commit order and dependency-closure freshness; SWF-010 headings merged with observed on-chain state; SMN-007 immutable permitted.

### v2 cleanup (21:30Z, within round 1)

astra-1: assurance O5-O7 cells realigned to the five-column header; SMN-005 fee/parameter/DUST evidence attached to the mandatory `A8p`/`A10p` receipt, B8p stays optional; SWF-012 added to the proposal's mandatory list; evidence-only commits keep eligibility only while every declared dependency hash is unchanged. astra-3: every retained B row (B1-B3, B6, B7 and the new preview rows) is `optional: true`; the E14 regression covers failing existing B6/B7; `t3.sh:64`. astra-2: adopting a conditionally guarded feature (S>1, optimization, formal claim) removes `optional:` from its rows and makes SRC-014/SAS-008/SAS-009 mandatory. fable-3: E1 accepts retired rows' historical checks and requires `source:` on optional rows.

### DRAFT v2 votes: fable-3 ACCEPT (21:25Z); fable-2 ACCEPT (21:25Z).

### DRAFT v2.1 final votes (21:27-21:29Z)

| Member | Vote | Reference |
|---|---|---|
| fable-1 (chair) | ACCEPT | this file |
| fable-2 | ACCEPT | `research/sherlock-vm-council-20261107/fable-2/vote-v2.md`, bus 21:27Z |
| fable-3 | ACCEPT | `fable-3/vote-v2.md`, bus 21:25Z (v2; no change requested for v2.1) |
| astra-1 | ACCEPT | `astra-1/vote-v2.1.md`, snapshot `astra-1/review-v2.1.sha256` (13 files matched the live bundle; 56 IDs in specs and traceability) |
| astra-2 | ACCEPT | `astra-2/vote-v2.1.md`, `astra-2/review-v2.1.sha256` |
| astra-3 | ACCEPT | `astra-3/vote.md`, `astra-3/draft-v2.1.sha256` (13 files matched) |

Tally: 6 of 6 ACCEPT, 0 OBJECT. Consensus per BRIEF.md (at least 5 of 6 ACCEPT and no OBJECT on a soundness or security issue) is reached on DRAFT v2.1 after one revision round plus cleanup.

## Unresolved dissent at consensus

None. The concerns every member asked to carry forward are recorded as implementation limits, not dissent: the genesis realization must be re-justified at the re-pinned `midnight-circuits 7.3.0` before node keygen (task 2.3; fable-2 prefers realization (b)); the universal soundness/completeness obligations O1-O4 stay open until a theorem exists; the universal leaf k, prover memory for S>1 and complete-path cost are unmeasured; the deployed node source for spec 2001100 is not public (SMN-004 limit). A council vote is not proof execution, deployment acceptance or a release.

## Owner decisions carried out of the council

1. Amend `openspec/MORIARTY-COMPLETION-PROGRAM.md:269` (mainnet) or keep M7 blocked.
2. Whether B (Fenrir rollup) rows stay in the factory as non-gating Preview work or are dropped (ISSUE 7: all six accept non-gating).
3. Whether a stagenet diagnostic call is ever worth the wallet and proof-server cost.
4. Whether `SherlockRun` is deployed with a maintenance authority (SMN-007).

### Resolved 2026-11-08

The owner's words (2026-11-08, verbatim): "resume the work, merge, amend the program for mainnet, fenrir should test on preview and then mainnet, not testing on old stagenet, use your judgement on sherlock run".

1. **Resolved 2026-11-08: amend.** `MORIARTY-COMPLETION-PROGRAM.md:270-283` supersedes the `:269` mainnet ban, and the old sentence stays visible struck through. Mainnet submissions are permitted only as T4 operations after local Docker at 9.1.0.1 has passed, the Preview qualification receipts and the Preview rollback rehearsal receipt are committed, and the pre-deploy checklist has passed. Each operation still needs explicit owner authorization when it runs, under an owner-set per-operation spend ceiling with no automatic retry. The amendment authorizes no specific spend (SWF-009, E9, task 7.1).
2. **Resolved 2026-11-08: keep, and make mandatory** (refined by "Resolved 2026-11-08 (2)" below; the Fenrir reading is superseded). The B rows are the required Fenrir path, Preview first and then Mainnet (new rows B4m, B5m, B8m, B9m). They stay outside the Sherlock U023/C4 dependency chain, so Sherlock qualification does not wait on Fenrir. The Fenrir gates (tasks 6.6 and 7.4) depend on them (SWF-013, D14). This replaces the council's "non-gating" outcome for ISSUE 7 by owner decision.
3. **Resolved 2026-11-08: no.** No stagenet diagnostic call will be made, for Sherlock or for Fenrir. The stagenet rows stay retired as history (design §5.2(6), workflow §6).
4. **Resolved 2026-11-08: deploy with a maintenance authority on Preview and on Mainnet.** The owner delegated this decision ("use your judgement on sherlock run"), and the lead made it. Reason: the rollback path (`workflow.md` §7, SWF-010) depends on maintenance key replacement, and the mainnet rollback must be rehearsed on Preview. This is a trust assumption: whoever holds the authority can install a verifier key, so the authority can change what the contract accepts. The owner holds the authority; its public identity is published in `target-pins.json`. Every maintenance update is preceded by the A11 keygen regression and produces a committed receipt of the observed on-chain key state; midnight-node #2375 is checked before any key removal. A later change may freeze or renounce the authority once the keys are stable (SMN-007, design §5.2(5), `tasks.md` "Later changes").

### Resolved 2026-11-08 (2)

The owner's words (2026-11-08, verbatim): "Sherlock should not depend at all on Fenrir, We have a synthetic rollup design, we use that".

The owner's words on the C4 closure (2026-11-08, verbatim): "keep them out of C4".

This refines decision 2 above ("fenrir should test on preview and then mainnet"), which commit 927f12956 applied by making the Fenrir B rows required, Preview then Mainnet.

1. **No Fenrir dependency.** Moriarty and Sherlock carry no dependency on Fenrir: no Fenrir-owned gate, row, operation, file, repository or live Fenrir system (SWF-014, D14). Fenrir's own testing on Preview and Mainnet belongs to the Fenrir project, not to Moriarty's predicates.
2. **The B rows are Sherlock's synthetic rollup.** The B rows keep their IDs and become Sherlock's synthetic-rollup qualification on data that FSYN generates in this repository (`openspec/changes/sherlock-synthetic-rollup-data/`, SSRD-029, SSRD-030): local first (B1, B2, B3, B6, B7), then Preview (B4p, B5p, B8p, B9p; OP-PV-SYNTH-ROLLUP), then Mainnet (B4m, B5m, B8m, B9m; OP-MN-SYNTH-ROLLUP). OP-PV-FENRIR-BATCH and OP-MN-FENRIR-BATCH are renamed accordingly (SWF-013, tasks 6.6 and 7.4).
3. **Outside C4, by owner decision.** The earlier reason for keeping the B rows out of the C4 closure ("Sherlock should not wait on Fenrir") no longer applies, because the rows are Sherlock's own work. The owner decided to keep them out of C4 ("keep them out of C4"). C4 stays the Sherlock M6 gate for representation and arithmetic on P1-P3; the synthetic-rollup gates (tasks 6.6, 7.4) depend on the B rows.
4. **Stagenet stays retired.** B4, B5, B8 and B9 stay retired; decision 3 above is unchanged.
5. **Code decoupling is separate.** B1, B2 and B3 (and through B3, B6 and B7) still read a Fenrir checkout today; their rows record this. Task 1.9 replaces every such read with FSYN-generated in-repository fixtures and adds the T0 guard F17. The synthetic rollup change is amended so that FSYN is self-contained, its design-cut constants are restated as Moriarty-owned parameters, and sync with Fenrir is optional, offline and manual (its proposal, "Owner amendment 2026-11-08").

### Resolved 2026-11-08 (3): the speed re-plan

The owner's words (2026-11-08, verbatim), in the order they were given:

- On the measured recursion (293 node steps at about 95 s each, 7.7 h for fact(3)): "way too slow".
- On the proposal to re-plan: "yes, re-plan it, and reopen tree aggregation and allow the planning agents to search online for ideas".
- On the design method: "whatever approach you choose have some agents explore it with lean and quint to reason about its design. It has to be general purpose recursion".
- On memory: "yes, 48 GB per process, 100 GB aggregate".
- On tooling: "use the tools in the workflowskills repo".
- On installs: the Lean toolchain (v4.33.1; `clean/fork` SherlockSpec builds) and CUDA 13.4.3 (`/opt/cuda/bin/nvcc`) were approved and installed on 2026-11-08.
- On the GPU, after the planning agents reported that ICICLE's CUDA backend is closed source and licence-server gated and that sppark supports no ARM64 GPU host: "you'll have to build one in rust. Search online for CUDA rust kernels for these things".

The three planning reports are `research/sherlock-speed-replan-20261108/{tree,prover,order}.md`; the lead's synthesis is this amendment. What was decided:

1. **Throughput is mandatory.** SRC-011 moves from Optional to Ubiquitous with the owner's caps: under 2 h wall, under 48 GB peak RSS per process, under 100 GB aggregate across the concurrent heavy processes of a run; a shortfall is a failing row, never a pass by being recorded. This supersedes the ISSUE 9 reading ("shortfall reporting closes measurement only") in the stronger direction: the measurement tasks still close on a recorded shortfall, but the throughput rows (A18, A19, A19b) then fail and gate G-C blocks OP-PV-DEPLOY. Staged gates SRC-015: node step at k=19 <= 60 s / 16 GB (A16), `entry 7;` under 20 min (A18, G-B), fact(3) and list123 under 2 h (A19, A19b, G-C). Caps are seeded requirement caps (`fixed: true`), not measured-then-doubled.
2. **Memory safety first (SRC-016, task 2.0, E11 required).** The 112-117 GB `mori_backend` test binary of 2026-11-08 ran with no memory guard (`order.md` §1 item 1). Every heavy process runs under a kernel cgroup cap of at most 48 GiB with swap off; a declared concurrent run sits in a 100 GiB parent scope; preflight checks `MemAvailable` and swap; the aggregate admission rule is enforced by the queue; heavy tests are `#[ignore]` in plain `cargo test` and `RUST_TEST_THREADS <= 4` for the prover crates. E11 (heavy concurrency) moves from "named, not planned" to task 2.0 of this change. No heavy run before F18.
3. **Primary path: S-batched linear IVC with parallel leaf workers.** Leaves are independent, so 2-3 leaf workers prove ahead of the node chain under the 100 GB aggregate; the node is trimmed to k=19 inside task 2.3 so that 2.4 keys once; the thread sweep is lever 0; S is a keyed constant chosen once (provisionally at G-A from the node numbers, finally at G-A' from the measured `k_U`) by `k_U + log2 S <= 20`, and the leaf is built with S slots and keyed once (SRC-014 unconditional; SRC-013's rule becomes `k <= 17 + log2 S` at the keyed S, and option F fixes S=1). This supersedes the ISSUE 6 conditions ("S=1 as the initial qualified baseline", "S=8 an experiment"): the measured-first principle behind them is kept (`k_U` is measured before S is fixed, and S=1 remains the one-enabled-slot case of the same relation), but S is no longer an after-the-fact experiment that would cost a second key cascade.
4. **Tree aggregation reopened as a measured option (D13').** The uniform three-slot self-recursive node of `tree.md` F8 (one key, disabled slots the identity, hash-bound manifest) replaces the cut binary tree; the per-level-key fallback of `stagenet-gp-recursion.md:168` is struck because it violates the fixed-key requirement. The tree is decided at G-A and G-B by `tree.md` §7 and `prover.md` §6: it is the lever if `k_U >= 18` or k=19 fails, or if the chain alone misses the target. Its criterion 3 (the owner amends SRC-011 to a 100 GB aggregate and funds E11) is met by this decision.
5. **One general aggregation model (SRC-017).** Segment composition is modeled once, the linear chain being the degenerate tree: a Quint model (`models/sherlock/quint/aggregation.qnt`, constants `TREE` and `S`, F20) and Lean theorems (`clean/fork/Clean/Sherlock/Aggregation.lean` in `SherlockSpec`, F21) cover linkage and associativity, padding and identity neutrality, exactly-once coverage, height bounds, unique genesis, replay, schedule independence, program fixity, shape binding, independence from shape and S, and "the root statement implies a valid execution of the general machine for every program". One fixed key set serves every program and every step count. The tools are the `quint-modeling`, `quint-lang` and `quint-execute-spec` skills and the lean4 plugin of the workflowskills repo. A council review (skill `council`, cross-family lanes; task 2.14) of the design plus the formal results precedes the implementation of the node constraints (2.3), the S-slot leaf (4.2) and any tree node (5.7).
6. **GPU: an open-source Rust MSM/NTT build, phased, optional (SRC-018, design §3.7).** No ICICLE or sppark crate dependency (closed CUDA backend, licence-server gated; no ARM64 GPU host support). Source code borrowed into `crates/mori-gpu` is allowed only under MIT or Apache-2.0 (owner decision 2026-11-08 (4), below). Phases: toolchain smoke on sm_121 (done 2026-11-08: a 64-bit `mulhi` kernel matched the CPU) -> field kernels -> MSM -> NTT -> prover integration behind a default-off `gpu` feature of a patched `midnight-proofs 0.8.3` -> node-step measurement; each phase has a kill criterion and a time box; the track opens past the smoke only when the C0 MSM-plus-NTT share is at least 0.4; proofs stay byte-identical under the stock Midnight verifier. The design source is `research/sherlock-speed-replan-20261108/gpu.md` (`crates/mori-gpu`).
7. **New task order (tasks.md).** 2.1 -> 1.9 -> 2.0 -> the C0 path (2.7, 2.8, 2.9, 2.13) and the formal tasks (2.11, 2.12) in parallel -> 2.14 -> G-A -> 2.3 -> 2.4 -> 2.5 -> 2.6 -> GPU phases if open; M4 chooses S at G-A' before the leaf is keyed; M5 carries the throughput gates G-B and G-C; 6.2 depends on G-C. Each task is marked **[loop]** (unattended), **[lead]** (gate or decision) or **[owner]** (install or spend). Gate decisions are committed JSON under `qa/sherlock/decisions/` (`speed-levers.json`).

No council member dissented from this amendment; it was an owner decision outside the council round, and the measured-first conditions of ISSUE 6 and ISSUE 9 are preserved as described in items 1 and 3.

### Resolved 2026-11-08 (4): borrowed GPU kernel code

- Asked whether CUDA C++ kernel templates adapted from sppark (Apache-2.0) or ICICLE v2.8.0 (MIT) inside the Rust crate satisfy "you'll have to build one in rust", the owner said (verbatim): "I don't care as long as the license is MIT or Apache2".
- Carried out: code borrowed into `crates/mori-gpu` or `tools/gpu-prover/` SHALL be MIT or Apache-2.0 licensed, keep its licence and copyright headers, and be recorded with source URL and commit in the crate's provenance notes. Code under any other licence (MPL-2.0, GPL, no licence, or ICICLE v3+ terms) SHALL NOT be copied. A crate dependency on ICICLE's closed CUDA backend stays excluded. This settles gpu-align finding (a); `gpu.md` §2 already follows it.

### Resolved 2026-11-08 (5): the target is Mainnet, tested on Preview

- The owner said (verbatim): "We don't care about stagenet anymore, Midnight forked, this can work on mainnet. We should test on preview or preprod (whichever is the canonical test infrastructure)."
- Carried out: the goal of this change is Sherlock on Midnight Mainnet with full general-purpose recursion. Stagenet is not a target (decision 3 of "Resolved 2026-11-08" already retired it).
- The test network is Preview. Preview runs ledger 9.1.0.1, the same as Mainnet, as a new chain since 2026-11-07. Preprod still runs ledger 8.1.3, and toolchains 0.34-0.36 cannot deploy to it (`wiki/midnight-ledger-9-hardfork.md`, support matrix). The order stays: local Docker at 9.1.0.1 (`undeployed`), then Preview (M6), then Mainnet (M7). If Preprod moves to ledger 9, a later change may add Preprod rows; this change adds none.

### Owner decisions still open after 2026-11-08 (3)

1. Whether to fund the GPU build phases (2.10b-2.10f; agent time, each time-boxed) if gate G-A opens the track (`m + q >= 0.4`). Without that decision the track closes at the smoke and the GPU rows stay optional.
2. Whether to fund the tree build (task 5.7; `tree.md` ESTIMATE 1-2 weeks) if gate G-A opens it, when the chain at the chosen S is projected to miss 2 h.
3. If `k_U > 17` forces option F (S=1), whether the 2 h target may rest on k=19, concurrency, the GPU and the tree alone, or whether the inventory `R` or the leaf design is revisited first. (Restated 2026-11-10 as question 3 of "Owner questions from the 2026-11-10 design review" below.)
4. Items 1-4 of the earlier lists stay as recorded (stagenet, maintenance authority, mainnet spend); any spend remains an owner authorization per operation.

### Owner questions from the 2026-11-10 design review

Source: `research/sherlock-recursion-design-review-20261110/review.md` §4.2 and §5 (`dissent.md` edits). The owner instruction for the amendment is: "yes and launch a series of experiments to resolve these design issues. Then update the openspecs and EARS" (2026-11-10). Each lead recommendation below is a recommendation, not a decision.

1. **Genesis guarantee (open; review §4.2 dissent 2; design §3.2(5), SRC-021).** Which mechanism establishes that a run starts from the canonical initial state of its program: option G1, an in-circuit first-leaf check selected by `h_pre == 0` or another authenticated mechanism; or option G2, off-chain recomputation of `init_d` by the independent verifier, with O1 restated so that an `acceptRun` is a claim relative to the sealed `init_d`. The editor of the review held G2 acceptable if O1 and task 4.9 are rewritten. The second reviewer (review §4.2 dissent 2) held that G2 is sound only for a verifier that performs the recomputation: any reader or downstream contract that treats an `acceptRun` as proof of a canonical run can be fooled, because the constructor accepts any nonzero digest (`tools/sherlock-sdk/sherlock-run.compact:29-38`), and for an unauthenticated `init_d` the Lean premise may not hold. **Lead recommendation:** G1, the in-circuit or otherwise authenticated mechanism (the second reviewer's position), with its cost measured by experiment E3 (task 4.13) before the choice is final. The choice is needed before 4.2 because it changes `k_U`.
2. **Scope of "general purpose" (answered by the owner 2026-11-10).** The review proposed restating SRC-017 and §3.6 in three narrower scopes and asking which of run input, a registry contract, non-halted outcomes and foreign-circuit aggregation are in scope (review C5). The owner answered: "It needs to be general purpose recursion. Make sure we understand what that means. And universal", and selected all four of universal VM, cross-run composition, foreign proofs and any shape. The definition is `general-purpose.md` (GP-1 to GP-4; requirements GP-001 to GP-004). The review's narrowing is not adopted; the current design is the first milestone toward GP-1, and its gaps are listed in design §3.6. The architecture for GP-2 to GP-4 is task 4.14 (GP architecture study).
3. **No S >= 4 fallback (open; restates item 3 of "Owner decisions still open after 2026-11-08 (3)"; review C3).** If no S >= 4 fits the measured leaf, node and decider caps, what follows. Option F (S=1, about 5 h on the node chain alone), the tree (k=21, F22) and the GPU backend (unbuilt, unfunded, no measured gain) are not available answers today. **Lead recommendation (from review C3):** the leaf design is revisited before any key; S <= 2 is a redesign trigger; G-C stays as written (2 h, 48 GB per process, 100 GB aggregate), and a recorded shortfall is not a pass.

### C6 outcome-only review (2026-11-10)

Source: `research/sherlock-c6-outcome-ledger9-review-20261110/review.md` (members Fable, Astra and Carbon. Each member answered before it saw another answer). The owner decision is: "go with publishing only the outcome for C6 after gemini carbon and fable and astra review that" (2026-11-10). Verdicts: Q1 outcome-only publication, adopt with changes (3 of 3, `review.md:11`); Q2 runs on Midnight ledger 9, yes with changes (3 of 3, `review.md:12`). The conditions of adoption are SRC-006, SRC-007, SRC-021 (amended), SRC-022, SRC-023, SVM-010 to SVM-012 and the additions to GP-001 to GP-004 (`review.md:85-94`, `:144-172`). Design: §2.7.11 and §3.2(5). The decision adopts the publication rule only. It does not adopt the chip leaf or D3.

**D-1 SI-12 to SI-14: dense logical identifiers, addresses as references, or linked slices (open).** (`review.md:51-55`.)
- Astra keeps dense logical identifiers and authenticates their mapping to sparse physical storage. SI-12 then holds on logical field blocks, whose physical records may span segments.
- Fable keeps the address as the reference. SI-13 and SI-14 hold on strictly increasing addresses. SI-12 cannot hold under C = 8, because a field slice longer than 8 cells is never contiguous.
- Carbon keeps addresses and replaces contiguous slices by cons-linked cells `(TAG_SLICE, elem_ref, prev_link)`, with inline child references for arity 2 or less. No rule computes `addr + k`.
- Status: open. No form is measured under C = 8. The candidate to measure first is Carbon's linked-slice form, and Astra's mapping is the alternative (task 4.18, row A42). SI-12 blocks D3 adoption for the C6 machine, not the publication decision. No EARS text for SI-12 is adopted (`review.md:174`).

**D-2 Type identity inside a constructor digest (open).** (`review.md:57-61`.)
- Astra hashes the producer's program identity `p`, its type-table identifier `t` and the ordinal `c`. Cross-program nominal types then need an authenticated mapping.
- Fable and Carbon hash a program-independent type digest `ty_d`. Carbon defines `ty_d(Adt(..)) = H(DOM_TY, 4, decl_schema_d, p, ty_args_d)`. The normal form behind `decl_schema_d` is not given.
- Status: open. The verdict takes the program-independent form, because GP-2 acceptance (a) has program B consume a value of program A. Astra's dissent stays. The normal form of `decl_schema_d` must be fixed before any key is frozen (SRC-022 [OPEN]).

**D-3 Step count in the claim (decided).** (`review.md:63-66`.)
- Astra: the claim is `(encoding_version, prog_d, input_d, outcome_d)`, and the step count is optional metadata if it counts logical events only.
- Fable keeps `n_steps` in `stmt`. The GP architecture study puts `n_steps` inside `out_d` (`research/sherlock-gp-architecture-20261110/architecture.md:98`).
- Status: decided by the owner's GP-2 triple. The owner defines the GP-2 claim as `(prog_d', input_d', outcome_d')` (`general-purpose.md` GP-2). So the claim is `(version, prog_d, input_d, outcome_d)`, the replay key is `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`, and `n_steps` is published beside the claim, not in it (SRC-006, SRC-007). `architecture.md:98` needs the matching edit, `out_d` without `n_steps` (task 4.20).

**D-4 Cost of the cached digest (kept as a measurement task).** (`review.md:68-72`.) Astra holds that a digest at creation can cost more over a run than a traversal at halt. Fable and Carbon prefer the digest at creation. No member measured either mechanism. Task 4.16 measures both.

**D-5 Shape of the constructor digest (resolved).** (`review.md:74-78`.) The verdict takes Carbon's fold, `acc_{k+1} = H(V, 1, 7, 2, acc_k, vd(f_k))`, because the Build helper links one field per step. `UInt` is one word, and `outcome_d` has no top-level type word. Carbon's top-level `ty_d` is the safer choice if a later value form loses its tag.

**D-6 Leaf partition in the GP-4 shape commitment (open, against GPR-016).** (`review.md:45,80-83`.)
- Carbon: bind only the inter-claim DAG publicly. Leaf boundaries and segment digests stay hidden.
- Astra: the shape may differ between proofs of the same claims. It must not be a run identity and must not be built from state roots.
- The GP architecture study binds segment bracketing into the public `shape_d = H(SHAPE, relation tag, slot bits, child shape_d values, local statement)` (`research/sherlock-gp-architecture-20261110/architecture.md:101`).
- Status: open. It is a decision on GPR-016 of `architecture.md`. The GP-004 addition forbids a state commitment as a shape input and forbids a decision of acceptance on the shape. Both positions accept that text. Whether bracketing is public stays with GPR-016. A second point is also open: if the "local statement" hashed into `shape_d` contains `pre_d` or `post_d`, a raw-address digest reaches a public value. The study does not define "local statement" at that line.

**Carbon's SRS dissent in Q2 (recorded as wrong).** (`review.md:233`.) Carbon said that the practical ceiling is k = 20, set by the ceremony prover SRS `bls_midnight_2p20`. Astra found that the ledger's data catalogue lists ceremony files through `bls_midnight_2p25`, and the lead confirmed it (`base-crypto/src/data_provider.rs:182-209` of `repos/midnightntwrk/midnight-ledger` at tag `ledger-9.1.0.1`). The SRS does not exclude k = 21. The project's row gates and prover cost exclude it. Carbon's design conclusion (compute the value digest in the leaf, not in the decider) still stands on the row gate alone. Two smaller Q2 points are recorded: Carbon read `midnight-zk` at `HEAD`, not the pinned crates, and Carbon did not see that the ledger and node tags lock `midnight-proofs` 0.8.2 (`review.md:234-235`; task 2.17).

**Owner question from the C6 review: the D3 run ceiling against GP-1 (open; GP-1 conflict).**
- Fact (`review.md:40`): under D3, `base / 8 < 2^17`, every leaf takes one segment (a leaf with zero appends too), and the node requires at least one step per leaf. So a run has fewer than 2^17 leaves and 2^20 cells. GP-1 states a bound of 2^64 steps for one fixed key set (`general-purpose.md` GP-1; GP-001).
- This is an open conflict with GP-1. The C6 decision does not repair it. The prover can only fail to prove a long run. It cannot change the outcome (`review.md:99`).
- GP-1 to GP-4 are not narrowed. The owner ruled that they must never be narrowed. No requirement of this change states the ceiling as a bound.
- Question to the owner: which change to the memory design removes the ceiling, so that one key set proves runs up to 2^64 steps? The task that records the candidates is 4.19. The lead makes no recommendation in this record.
