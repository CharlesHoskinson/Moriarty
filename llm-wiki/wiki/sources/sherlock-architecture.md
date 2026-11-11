---
title: "Architecture of the modular Sherlock leaf and recursion (revision B2)"
type: source
tags: [design, final-design, architecture, module-kits, lead, pending-owner]
sources: [sherlock-architecture]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-final-design-20261111/ARCHITECTURE.md
source_url: local
---

# Architecture of the modular Sherlock leaf and recursion (revision B2)

## Summary

`ARCHITECTURE.md` of run `sherlock-final-design-20261111`, night-plan step B1, revised as "revision B2" after the Astra audit ([[arch-audit-astra]], [[arch-audit-response]]). Fable 5.2 merged six drafts ([[astra-area1-processor-draft]], [[astra-area2-memory-draft]], [[astra-area3-recursion-draft]], [[gemini-area1-processor-draft]], [[gemini-area2-memory-draft]], [[gemini-area3-recursion-draft]]) with the decided inputs of [[final-design-decisions]], experiment C1 ([[c1-dense-memory-experiment]]), experiment C3 (Lean) and the rule-maxima experiment ([[rule-maxima-experiment]]). Where a draft conflicts with a decided input, the decided input wins and the conflict is recorded in section 6. It is a proposed design: "Revision B2 made no build, test, proof or model run". Signatures were not compiled. The decisions in it are lead decisions taken while the owner was asleep; OWNER marks name what the owner must confirm. The concept page is [[ModularArchitecture]].

## Key Claims

- 23 module kits: 19 from the merge plus INIT, REFERENCE BRIDGE, HOST CHECKPOINT STORE and the GP-3 general verifier (five subkits), added by revision B2. Every kit has nine items: purpose, interface, invariants, row budget, verification, Lean obligations, code skeleton, what the agent reads and must not need to read, dependencies. One agent builds one kit against its interface only.
- Leaf kits: BUS, RANGE, ALU, DEC, HASH, CORE, STACKS, MEM window, MEMORY BACKEND (dense, default), PAGE/ROM block cache, MEMORY BACKEND variant ZmLogUp (Profile B, benchmark-only), VALUE DIGEST, BOUNDARY COMMITMENT, INIT. Recursion kits in `crates/mori-recursion`: NODE INTEGRATION, DECIDER, CONTINUATION, GP-2 hook, GP-3 hook, GP-4 hook, GP-3 general verifier. Host kits: REFERENCE BRIDGE, HOST CHECKPOINT STORE.
- Frozen bus tuple `(chip, key, a, b, c)`: two request lanes over one provider table plus one tagged fixed table, so three lookup arguments. Chip ids 0 disabled, 1 ALU, 2 MEM (key `8 addr + kind`, kind `0..=4`), 3 HASH (two-input sponge), 4 ROM (new, proposed); 5 and above unsatisfiable. CORE is the only bus requester. The bus gives membership (`LookupSound`), not a permutation.
- Named copy-constrained ports: `Window`, the append chain, `AuthRec` and `AuthRom` (new; the only way CORE rules, VALUE DIGEST and INIT get records and ROM rows), the memory token `[root, tail_d]`, `ColdRead`, state cells, stack cells, state header cells, whole-sponge slot, RANGE and DEC ports, the seven leaf public lanes `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`.
- Two swappable seams. Seam 1 MEMORY BACKEND: the landed `MemBackend` trait (`persist`, `cold_read`) plus one new method `open_line`. Seam 2 BOUNDARY COMMITMENT. Profile A (dense Merkle plus Poseidon header and body) is the built path; Profile B (ZmLogUp plus Horner) is benchmark-only, not before M4; the mixed pairs are not offered.
- `LeafShape { s, w_flush, k_page, k_rom, n_cold }`, one definition. Working values: `s = 16`, `w_flush = 1`, one arena, `TOP = 63` (OWNER). `k_page`, `k_rom`, `n_cold` not frozen.
- CORE owns the append chain (conditions A1 to A4, the landed hybrid: copy-linked addresses plus one MEM append request per append), the read port (`read_record` items 1 to 9 with `addr < len_cur` at the running length, `read_rom` with a ranged row), `rules_d = Poseidon(dom("SHK_RULES"), rid_0, ..., rid_(S-1))` as lane 5 (630 rows at S = 16, ESTIMATE), and scratch independence from incoming values (invariant 8; zeroing is not the enforcement).
- STACKS: cached top plus a depth-indexed Poseidon chain; one compound `StackStep` (optional rewrite, then none, push or pop) per stack per slot. ESTIMATE 560 rows per frame spill or restore, 350 per task.
- MEM window capacity is state dependent: `8 W + 7 - r_pre` new records per leaf, so 15 at W = 1 with an empty tail; single-step fit and cumulative leaf fit are separate obligations.
- MEASURED (C1, TOP = 63): one flush unit 8,821 rows, cold read 4,761, read port 134, `persist` at W = 1 12,417. ESTIMATE: leaf at S = 16, W = 1 about 15,226 rows with no cold read; one-arena leaf with 4 heap and 2 ROM lines about 50,700 to 51,800; node about 847,494 against the 950,000 gate.
- BOUNDARY COMMITMENT: `body_d` capacity tagged over the body words, header `state_d = Poseidon(dom("SHK_SD2"), outcome_tag, outcome_payload, input_d, deps_d, body_d)`; NODE and DECIDER read the header with `open_state_header` (one six-input library sponge; X10 measures it).
- Recursion: node statement types SEG, SET, SUB, DL, FV; an eight-row node relation table (MERGE, MERGE-WORK, SEAL, SEAL-WITH-DEPS, DL-STEP, DESCEND, JOIN, FV-WORK). `set_root` has a typed grammar (`LEAF`, `DEP`, `JOIN`, `FOREIGN`). No claim count anywhere (R-5 merge decision, OWNER to confirm). GP-2 discharge is a new protocol (`SHK_DEPS` chain, DESCEND, DL-STEP, SEAL-WITH-DEPS, input link), in no draft, gated by X12.
- Build order: group F (interface freeze by the lead), 0 BUS, R REFERENCE BRIDGE beside groups 0 to 4, 1 RANGE, HASH, MEM window, 2 ALU, DEC, STACKS, dense backend, VALUE DIGEST, 3 PAGE/ROM and CORE framework, 4 CORE rule families, BOUNDARY COMMITMENT then INIT, L leaf integration (whole-leaf harness at zero coverage gaps), 5 NODE, 6 DECIDER, HOST CHECKPOINT STORE then CONTINUATION, GP-3 hook, GP-4 hook, 7 GP-2 hook, 8 GP-3 general verifier. Profile B kits are in no group.
- Experiments X1 to X15: X1 (rule inventory and per-rule maxima) and X2 (one-arena non-interleaving) done; X3 done for dense memory, open for hash stacks; X4 to X15 open, with X10 to X15 new in B2 (header opening, input admission, GP-2 discharge model, reference coverage census, general foreign verifier inventory, ROM rows per step).
- Open decisions OD-1 to OD-22, plus N-1 to N-14 (recursion) and V-1 to V-10 (PAGE/ROM, ZmLogUp, VALUE DIGEST, BOUNDARY). OWNER marks: OD-1 (confirm TOP >= 63), OD-2 (claim counter), OD-5 (D-1 status), OD-6 (only if S changes), OD-10 (only if a rule stays out), OD-13 (only on fallback), OD-14, OD-16, OD-17, OD-18, OD-19 (only on fallback), OD-20, OD-21, OD-22 (only if option b); N-6, N-7 (if it changes GP-4 text), N-9, N-11.
- Lean tally: 14 obligations proved in C3 (P1 to P14), 53 open (O1 to O53), 67 in total. P9 and P14 hold only under `FitsLeaf` (at most 8 appends in a leaf); the window's capacity needs the open `FitsWindow` rows O14 and O47.
- GP-1 to GP-4 are not narrowed by anything in the file; narrowing options (OD-1 c, V-1 b and c, two-node GP-4, adapter-only GP-3) are "not an acceptance path of this change".
- OpenSpec outline for a new change `sherlock-modular-leaf-20261111` (created 2026-11-11, see [[openspec-modular-leaf]]): one capability spec per kit, one per CORE rule family, explicit amendments of `sherlock-vm-consensus-20261107`.

## Key Quotes

- "GP-1 to GP-4 (`O/general-purpose.md`) are not narrowed by anything in this file."
- "Option (c) is not an acceptance path of this change: a memory bound below the run scope leaves GP-1 incomplete, it does not authorize a narrower GP-1."
- "Soundness never depends on the store: a corrupted store can only stop the prover."

## Connections

- Concept: [[ModularArchitecture]]. Mechanisms: [[DenseTailMemory]], [[BlockCache]], [[ValueDigest]], [[Continuations]], [[GeneralPurposeRecursion]], [[ChipLeaf]], [[RunLengthCeiling]].
- Entities: [[MoriChip]], [[IvcNode]], [[RunDecider]], [[AcceptRunContract]].
- Inputs: [[final-design-decisions]], [[c1-dense-memory-experiment]], [[rule-maxima-experiment]], [[lookup-cache-audit]], [[runlength-consensus-audit]], [[quint-chipleaf-final]].
- Audit and response: [[arch-audit-astra]], [[arch-audit-response]]. Owner questions: [[OpenOwnerQuestions]].

## Contradictions

- Stale B1 text inside the file: section 7.3 still says "The per-rule maxima ..., the rule inventory, the non-interleaving claim ... are not known", and section 6.3 and 4.3 still say the non-interleaving check "is still required". Section 5.4 (X1, X2 done) and decisions M-1 to M-4 ([[rule-maxima-experiment]]) supersede these lines.
- OWNER marks: `DECISIONS.md` carried them on R-4 and R-5 only; section 7.3 says the merge added the other OWNER marks of section 4. They are lead labels, not owner words ([[final-design-decisions]]).
- Lead R-4 offers "accept a memory bound below the step bound" as an owner option; OD-1 and A-6 say that option is not an acceptance path of this change ([[final-design-decisions]]).
- Drafts against decided inputs (chip ids, memory keys, arena count, Profile B as co-equal, 2-leaf `RunLift`, value forms, four requests per step): every Gemini and Astra divergence is resolved in favour of the decided input in sections 6.1 to 6.4 ([[gemini-area1-processor-draft]], [[gemini-area2-memory-draft]], [[gemini-area3-recursion-draft]]).
- BOUNDARY header is pinned, not capacity tagged; this deviates from the letter of LC AUDIT item 25 and is recorded in section 6.6 ([[lookup-cache-audit]]).
