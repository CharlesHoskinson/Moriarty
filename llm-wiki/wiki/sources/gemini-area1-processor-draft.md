---
title: "Draft: area 1 processor modules (Gemini)"
type: source
tags: [draft, final-design, processor, bus, chip-leaf]
sources: [gemini-area1-processor-draft]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-final-design-20261111/drafts/gemini-area1.md
source_url: local
---

# Draft: area 1 processor modules (Gemini)

## Summary

Draft, not adopted. The Gemini lane's area 1 draft for run `sherlock-final-design-20261111` (the file does not name the model; `GEMINI-DONE` lists it with the area 2 and area 3 drafts). It defines the processor modules BUS, CORE, DEC, ALU + RANGE and HASH, each with a frozen interface, invariants, an ESTIMATE row budget, Quint, MockProver and mutant tests, Lean obligations, a Rust skeleton and an OpenSpec capability spec. It was merged into `ARCHITECTURE.md` of the same run ([[sherlock-architecture]]); the sister draft for the same area is [[astra-area1-processor-draft]]. Nothing was compiled, tested, proved or model checked: "All row deltas off E8 ... are linear-fit calculations marked `ESTIMATE`". The sibling drafts were "intentionally not read".

## Key Claims

- Recursive verifier envelope (from `VerifierGadget`): single-phase advice only, no `Challenge` in gates, rotations only in {-1, 0, +1}. Every module must keep to it.
- Baseline (measured, E8, cited from design.md): 1-leaf node 823,466 rows, decider 929,166 circuit rows and 1,032,467 `Pow2Range` rows. Node model `570,957 + 7,298 N_adv + 8,189 N_eq + 6,004 N_fix + 22,600 N_lookup` (ESTIMATE for deltas).
- BUS: two dynamic 5-tuple `lookup_any` channels plus copy-constrained ports (`ArchStatePort`, `WindowBoundaryPort`, `HashFixedPort`). Chip ids: 0 NOP, 1 reserved (unsatisfiable), 2 ALU, 3 MEM_ALLOC, 4 MEM_READ (split between MEM and MEMORY BACKEND at `base`), 5 ROM, 6 HASH. Memory key `(arena << 67) | (addr << 3) | kind`, `arena in 0..=3`.
- 4 bus requests per step on two adjacent rows (+5, +6) of a 7-row `Arch::Narrow` slot, at "+0 recursive `ChipNode` rows" (unlabeled).
- DEC and RANGE leave the dynamic bus for one merged 2-column `TaggedTableConfig` (`t_dom_key = (tag << 16) + key`, `t_val`): leaf lookups 4 to 3 and table columns 3 to 2, ESTIMATE -29,062 1-leaf node rows and ESTIMATE -51,204 to -51,662 2-leaf `RunLift` rows.
- CORE: `ArchState` of 41 cells, all hashed into `pre_d` / `post_d` (`DOM_STATE`). Monotone `is_pad`, slot 0 non-empty, `h < 2^64`, allocation counters advance only by real allocations, top-of-stack Frame and Task caches with non-zero empty-stack sentinels, scratch `v0..v2` write-before-read or zero per slot with no cross-slot link. `LeafShape { s_max }` with S = 16 for M4 and S = 64 after. ESTIMATE 114 to 130 advice rows at S = 16.
- DEC: `RuleIdC` has 134 variants; 7 disabled and 4 `/2` extension get no rows; 121 step rules plus 2 startup rules = 123 enabled. 15-bit `packed_key`, ESTIMATE 1,180 to 1,850 table rows (cap 2,048). 54-bit `ctrl_word` with a bound on every unpacked field to close `Exploit C1` (modular wrap forgery).
- ALU: nine ops (ADD, SUB, MUL, DIV_MOD_U, DIV_MOD_S, LT_U, LT_S, EQ, BITOPS), `op_key = op | (ovf << 4)`. ALU range-checks `x, y, w < 2^128` itself (`Exploit D1`), uses 64-bit schoolbook MUL with a boolean `c1_bit` and an exact overflow flag, and extracts sign bits for signed ops (`Exploit D2`). Jolt/Lasso 2D subword tables are rejected.
- RANGE: parameterized 12-bit or 14-bit limbs; recommends 12-bit for M4 (OD-1).
- HASH: slim 5-column `PoseidonCfg`, 68 rounds (8 full, 60 partial), 70 rows per block, all rounds fixed at keygen (`Exploit E3`). Twelve-domain capacity-IV registry `st[2] = absorb_len + (((dom << 16) | sub) << 32)`, from `DOM_REC = 1` to `DOM_CLAIM = 12`; legacy `HashChipCfg` deprecated.
- Leaf HASH totals (ESTIMATE): about 14,140 rows at S = 16 with `ZM-LogUp`; about 40,980 at S = 16 and 103,740 at S = 64 with a dense depth-61 Merkle backend.
- Recommends `Arch::Narrow` over `Arch::Wide` (Wide ESTIMATE +123,896 to +154,870 node rows) and full 123-rule coverage over the 108-rule C6 subset, which "Fails `GP-001`".
- Gating experiments P1 (node fit, pass at 1-leaf node <= 900,000 rows), P3 (S and limb width) and P4 (`ZM-LogUp` against `DensePageBackend`) before a new OpenSpec change `sherlock-modular-vm-20261115`.
- Lists nine conflicts C-1 to C-9, among them the legacy spike chip ids, ALU's caller-ranged assumption in design.md, design.md's E8 column counts, the "120 + 2" rule count in the lookup-cache consensus (it undercounted `XAdtHelperPlan*` by 1), and the "63-round" Poseidon typo there.

## Key Quotes

- "No Area 1 module bakes program AST constants, `Bounds`, rule subsets, leaf indices, or step-indexed memory offsets into verifying keys (`GP-001`)."
- "Narrowing `/1` by dropping `XAdtHelper*` directly violates `GP-001`"
- "`cargo check`, `cargo test`, `MockProver`, real Halo2/KZG `ChipNode`/`Decider` keygen, `quint`, and `lake -d clean/fork build` were **not** executed in this drafting session."

## Connections

- Merged into [[sherlock-architecture]]. Sister draft: [[astra-area1-processor-draft]]; Astra's other areas: [[astra-area2-memory-draft]], [[astra-area3-recursion-draft]].
- Concepts and entities: [[ChipLeaf]], [[MoriChip]], [[DenseTailMemory]], [[BlockCache]], [[ValueDigest]], [[GeneralPurposeRecursion]], [[C6OutcomeOnlyPublication]], [[IvcNode]], [[RunDecider]].
- Inputs it cites: [[lookup-cache-consensus]], [[runlength-consensus-audit]]. Lead decisions: [[final-design-decisions]]. Questions: [[OpenOwnerQuestions]]. Status: [[WorkstreamStatus]].

## Contradictions

- Chip ids differ from [[astra-area1-processor-draft]]: Astra has 1 ALU, 2 CELL, 3 HASH2, 4 PROG and reserves 5 and 6 for DEC and RANGE on one bus schema; this draft has 2 ALU, 3 MEM_ALLOC, 4 MEM_READ, 5 ROM, 6 HASH and takes DEC and RANGE off the dynamic bus.
- Memory key differs from Astra (`index + 2^64 * arena + 2^67 * chunk`) and from lead decision L-4 (key `8 addr + kind`); its 2-bit arena field also keeps four arenas against L-9's one arena ([[final-design-decisions]]).
- DEC + RANGE lookup: this draft uses the two-column tagged form; L-6 adopts the three-column form and says "the two-column form is not adopted". Astra's DEC is a two-column ROM of its own.
- 4 bus requests per step: this draft claims it at +0 node rows; L-8 says the row 6 promotion to 4 requests per step "needs a new layout plan; not adopted until planned and measured". Astra keeps the measured two request lanes.
- Uses 2-leaf `RunLift` figures and plans `specs/quint/` files; L-10 says the 2-leaf `RunLift` is unbuilt and not part of the design until measured and drops `specs/quint/` as invented.
- P4 would select `ZM-LogUp` if it fits; L-5 keeps Profile B (ZmLogUp) benchmark-only behind the seam, not built before M4.
- `claim_d = H(DOM_CLAIM, 2, [prog_d, input_d, outcome_d, n_steps])` puts `n_steps` inside the claim digest; [[C6OutcomeOnlyPublication]] keeps the claim as `(version, prog_d, input_d, outcome_d)` with `n_steps` published beside it.
- ALU: this draft has nine ops including division and bit ops; Astra proposes ADD, LT, SUB, MUL, EQ, NOT. Both keep an overflow flag, as L-7 requires.
- Agreements: per-slot scratch write-before-read matches Astra, audit finding 9 and R-8; domain tags in the sponge capacity match E-2; RANGE 12-bit limbs match Astra's default.
