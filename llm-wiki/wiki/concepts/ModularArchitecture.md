---
title: "Modular architecture: module kits, seams and build order"
type: concept
tags: [architecture, module-kits, final-design, lead-decision]
sources: [sherlock-architecture, arch-audit-astra, arch-audit-response, final-design-decisions, rule-maxima-experiment, c1-dense-memory-experiment, astra-area1-processor-draft, astra-area2-memory-draft, astra-area3-recursion-draft, gemini-area1-processor-draft, gemini-area2-memory-draft, gemini-area3-recursion-draft]
last_updated: 2026-11-11
---

# Modular architecture: module kits, seams and build order

How the Sherlock leaf and recursion are split so that one agent builds and checks one module against a frozen interface. Status: **proposed design, revision B2, lead decisions pending owner review** ([[sherlock-architecture]], [[final-design-decisions]] A-1). Nothing in it is built, modelled or measured beyond the landed C1 parts ([[c1-dense-memory-experiment]]). It merges three Astra and three Gemini drafts ([[astra-area1-processor-draft]], [[astra-area2-memory-draft]], [[astra-area3-recursion-draft]], [[gemini-area1-processor-draft]], [[gemini-area2-memory-draft]], [[gemini-area3-recursion-draft]]).

## Module kits

A kit is the unit of work: nine items (purpose, interface, invariants, row budget, verification, Lean obligations, code skeleton, what the agent reads and must not need to read, dependencies). An agent stops and reports an interface gap rather than read another module's internals ([[sherlock-architecture]]). There are 23 kits:

| Area | Kits | State today |
|---|---|---|
| Leaf, level 0 to 1 | BUS, RANGE, HASH, MEM window | dynamic bus, 128-bit RANGE, HASH with `sponge_iv` and MEM window landed; tagged table, ports manifest, domain registry new |
| Leaf, level 2 to 4 | ALU, DEC, STACKS, MEMORY BACKEND dense, VALUE DIGEST, PAGE/ROM block cache, CORE (framework plus one sub-kit per rule family), BOUNDARY COMMITMENT, INIT | dense backend landed (C1); ALU ADD and LT landed; DEC a stub; CORE a 32-word spike; the rest not built |
| Benchmark only | MEMORY BACKEND variant ZmLogUp (Profile B) | not built before M4 |
| Recursion | NODE INTEGRATION, DECIDER, CONTINUATION, GP-2 hook, GP-3 hook, GP-4 hook, GP-3 general verifier (five subkits) | linear node and run decider landed for the spike; the rest not built |
| Host | REFERENCE BRIDGE, HOST CHECKPOINT STORE | reference only for the dedicated grammar; store not built |

## Seams

- Two channels between modules only: a bus tuple `(chip, key, a, b, c)` and a named copy-constrained port. A module never reads another module's columns. CORE is the only bus requester; VALUE DIGEST and INIT get records only as `AuthRec` and `AuthRom` tokens of CORE's read port ([[sherlock-architecture]], [[arch-audit-response]] finding 4).
- Two swappable seams: MEMORY BACKEND (the landed `MemBackend` trait with `persist`, `cold_read` and a new `open_line`; token opaque) and BOUNDARY COMMITMENT. Profile A, dense Merkle with the Poseidon header and body commitment, is the built path; Profile B, ZmLogUp with Horner, is benchmark-only ([[DenseTailMemory]], [[BlockCache]]).
- Composition rests on two orders: bus tuple levels (tagged table rows, then ALU, HASH and ROM, then MEM by address) and the port dependency order (the build order read as a DAG). The proved C3 theorem covers bus tuples only; ports need the open obligation O42 and memory history O43 ([[arch-audit-astra]] finding 7).

## Build order

Group F (interface freeze by the lead), then 0 BUS; R REFERENCE BRIDGE beside groups 0 to 4; 1 RANGE, HASH, MEM window; 2 ALU, DEC, STACKS, dense backend, VALUE DIGEST; 3 PAGE/ROM, CORE framework; 4 CORE rule families, BOUNDARY COMMITMENT, then INIT; L leaf integration with the whole-leaf harness at zero coverage gaps; 5 NODE INTEGRATION; 6 DECIDER, HOST CHECKPOINT STORE then CONTINUATION, GP-3 hook, GP-4 hook; 7 GP-2 hook; 8 GP-3 general verifier ([[sherlock-architecture]] section 3.1).

Freeze prerequisites: X1 (done for inventory, appends, stack operations and heap reads; ROM open as X15) and X2 (done) from [[rule-maxima-experiment]]; the B2 audit answered; INIT specified (input part waits for OD-17 and X11); the boundary opening specified (X10 before the NODE spec); the GP-2 protocol specified (X12 before the GP-2 spec); reference coverage (X13, before group L); record layouts (OD-11); ROM depth and `K_rom` (OD-12, X15). SRC-023 at the window's capacity (`FitsWindow`) blocks reporting SRC-023 as proved, not the freeze.

Gates per kit: MockProver positives and named mutants with weakened controls; a Quint verdict file (invariants, bounds, seeds, mutant results); Lean theorems with no `sorryAx`; the repository Rust checks (the workspace clippy baseline is red today and is reported as red). Lean tally: 14 obligations proved in C3, 53 open; P9 and P14 hold under `FitsLeaf` (at most 8 appends in a leaf) only ([[sherlock-architecture]]).

## Kits added by revision B2

- **INIT (genesis and input admission).** The one owner of genesis: when `h_pre = 0` (selector `is_zero(h_pre)`, not a witness flag) it proves the first pre state is `Init(prog_d, input_d)` field by field, opens `prog_d` (ties `prog_root`) and `input_d = Poseidon(SHK_IN, val_d, fuel_lo, fuel_hi, stack_limit)` with each limb below 2^64, sets `deps_d = EMPTY_DEPS`, both stacks empty and the genesis memory canonical (condition C4). Input admission contract I1 to I6: no scan of the input, no new input-size bound, every used input node authenticated against `val_d`. The form is OD-17 (OWNER; recommended by-digest admission with an empty genesis memory) and the one-arena SRC-021 text is OD-18 (OWNER). ESTIMATE about 500 to 700 rows in every leaf ([[sherlock-architecture]]; [[ValueDigest]], [[DenseTailMemory]]).
- **REFERENCE BRIDGE (semantic reference and projection).** Host-side; adds no constraint. Today's `reference.rs` has no constructor values and the projection returns `UnsupportedGeneric` for nominal constructs, planned matches and constructor state, so it cannot run the GP-001 corpus (a constructor of more than 8 fields across a leaf boundary). Deliverables R1 full-language evaluator, R2 explicit event map (`n_steps` counts enabled owner events), R3 total projection, R4 coverage report where a gap fails the gate. Which artifact is normative is OD-21 (OWNER; recommended a separate evaluator over `moriarty-sherlock-nominal/1`); X13 sizes the work. Trusted host diagnostics, not a proof ([[sherlock-architecture]], [[arch-audit-astra]] finding 14).
- **HOST CHECKPOINT STORE.** The host store and versioned codec `sherlock-ckpt/1` that CONTINUATION needs; see [[Continuations]].
- **GP-3 general verifier (five subkits).** Bounded-work foreign verification; see [[GeneralPurposeRecursion]].

## Not narrowing

Narrowing options are not acceptance paths of this change: a memory bound below the run scope (OD-1 c), a small or per-program ROM depth (V-1 b, c), two successive nodes for GP-4 "plus new work", adapter-only GP-3. Failing to fit leaves a GP target incomplete; it does not narrow it ([[final-design-decisions]] A-6, [[arch-audit-astra]], [[OwnerRules]]).

## Contradictions

- The Gemini drafts propose split chip ids (2 to 6), other memory keys, a two-column tagged table, four requests per step, a 2-leaf `RunLift` and Profile B as co-equal or default; the Astra drafts propose four arenas and a depth-61 tree. The merge rejects each against a decided input (L-4 to L-10, R-4) ([[sherlock-architecture]] section 6).
- The merge kept the drafts' kit count of 19; the audit required four more, and the B2 text has 23 ([[arch-audit-astra]], [[arch-audit-response]]).
- Section 7.3 of the architecture still calls the rule inventory, the maxima and non-interleaving unknown; X1, X2 and M-1 to M-4 settle them ([[rule-maxima-experiment]]).

See also [[ChipLeaf]], [[MoriChip]], [[IvcNode]], [[RunDecider]], [[OpenOwnerQuestions]].
