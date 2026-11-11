# Proposal: modular Sherlock leaf and recursion (`sherlock-modular-leaf-20261111`)

Status: S2 (written design, specified only). Night-plan steps D1 and D2 of `research/sherlock-final-design-20261111/NIGHT-PLAN.md`, 2026-11-11. Nothing in this change is built, keyed or proved by this change. The full architecture is `research/sherlock-final-design-20261111/ARCHITECTURE.md` (revision B2), below called `FD/ARCHITECTURE.md`. The decisions taken while the owner was AFK are in `FD/DECISIONS.md`. The audit answers are in `FD/ARCH-AUDIT-RESPONSE.md`.

## Why

The leaf of `openspec/changes/sherlock-vm-consensus-20261107/design.md` section 2.7 is one circuit description. One agent cannot build or check it in one session, and the memory backend and the boundary commitment cannot change without a change to every other part. This change specifies the leaf and the recursion as 23 module kits with frozen interfaces (`FD/ARCHITECTURE.md` sections 1 and 2). One agent builds and checks one kit against its interface. The memory backend and the boundary commitment each sit behind one seam.

The change can move the terminal evidence-gated decision of `sherlock-vm-consensus-20261107` because it turns the GP-1 to GP-4 acceptance lines into per-kit evidence (MockProver tests and named mutants, Quint verdict files, Lean theorems with axiom reports) and a build order with gates. A kit or a GP target is reported complete only on that evidence.

## What Changes

- One capability spec per module kit (23 kits, list below), each with requirement IDs per kit.
- The frozen bus schema `(chip, key, a, b, c)` with two request lanes and one provider table, the chip registry, the tagged fixed table for RANGE and DEC, and the named copy-constrained ports (`FD/ARCHITECTURE.md` sections 1.2 and 1.3).
- Profile A as the built path: dense tail-buffered Merkle memory with 8-record PAGE and ROM cache lines, and a Poseidon header-and-body boundary digest (decisions L-1, M-1, E-1).
- Hash-chain FRAMES and TASKS stacks with the compound stack step (R-3, revision B2 finding 2).
- `sherlock-value/1` unchanged (L-3).
- Continuations: tier 1 segment tree and the retained-memory checkpoint mode 1 only (R-7).
- The GP-2, GP-3 and GP-4 hooks, the GP-3 general verifier, INIT, the REFERENCE BRIDGE and the HOST CHECKPOINT STORE (revision B2).
- The Lean obligations per kit (consolidated table of `FD/ARCHITECTURE.md` section 2: 14 proved in C3, 53 open).
- Replacement of D2, D3, D6 and D7 of `openspec/changes/sherlock-vm-consensus-20261107/design.md:95-100` by the dense memory and hash-stack rules. This edit is made as a task of this change in the governing change, not silently here.

## Scope

In scope: the 23 kits of `FD/ARCHITECTURE.md` section 2.

| Spec folder | Kit | ID prefix |
|---|---|---|
| `sherlock-chip-bus` | BUS | BUS |
| `sherlock-chip-range` | RANGE | RNG |
| `sherlock-chip-alu` | ALU | ALU |
| `sherlock-chip-dec` | DEC (rule decoder) | DEC |
| `sherlock-chip-hash` | HASH | HSH |
| `sherlock-chip-core` | CORE framework and the rule-family contract | CORE |
| `sherlock-chip-stacks` | STACKS (FRAMES and TASKS) | STK |
| `sherlock-chip-mem` | MEM window | MEM |
| `sherlock-chip-membackend` | MEMORY BACKEND (dense Merkle) | MBK |
| `sherlock-chip-page-cache` | PAGE/ROM block cache | PAGE |
| `sherlock-chip-zmlogup` | MEMORY BACKEND variant ZmLogUp (Profile B) | ZML |
| `sherlock-value-digest` | VALUE DIGEST | VDG |
| `sherlock-chip-boundary` | BOUNDARY COMMITMENT | BND |
| `sherlock-init-admission` | INIT (genesis and input admission) | INIT |
| `sherlock-reference-bridge` | REFERENCE BRIDGE | RFB |
| `sherlock-node-integration` | NODE INTEGRATION | NODE |
| `sherlock-decider` | DECIDER | DCD |
| `sherlock-checkpoint-store` | HOST CHECKPOINT STORE | CKPT |
| `sherlock-continuation` | CONTINUATION | CONT |
| `sherlock-claim-consumption` | GP-2 hook | CLM |
| `sherlock-foreign-registry` | GP-3 hook | FREG |
| `sherlock-proof-shape` | GP-4 hook | SHP |
| `sherlock-foreign-verifier` | GP-3 general verifier (five subkits) | FVR |

The spec folder names are the names of `FD/ARCHITECTURE.md` section 5.2. The ID prefixes do not collide with GP, SRC, SVM, SAS or SWF, or with the Lean labels (for example DEC-L1 is a DECIDER obligation, not a kit DEC requirement).

Profile B (ZmLogUp memory and Horner boundary) is benchmark-only. It is not built before milestone M4, it is in no build group, and its one task is not part of the completion of this change (L-5, OD-7).

Out of scope: continuation mode 2 (OD-15), semantic profile `/2`, any change of the GP-1 to GP-4 text, and time and throughput gates (they stay in the SRC-011 and SRC-015 rows after measurement).

## Relation to `sherlock-vm-consensus-20261107`

This change implements the leaf and recursion design of `sherlock-vm-consensus-20261107`. That change keeps the requirements GP-*, SRC-*, SVM-* and SAS-* and its open items. No requirement of this change replaces, narrows or reinterprets GP-1 to GP-4 (`openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md`). A narrowing option is never an acceptance path of this change: OD-1 option (c), V-1 options (b) and (c), a two-node GP-4 and an adapter-only GP-3 (decision A-6).

The amendments of the governing change that this change needs are tasks of group F in `tasks.md`. Each is made in `sherlock-vm-consensus-20261107`:

- SRC-021 and the Init table to the one-arena field list (OD-18, OWNER).
- The input-admission open item of `design.md:194` (OD-17, OWNER).
- `design.md:80` to "one schema, two lanes".
- D2, D3, D6 and D7 of `design.md:95-100` to the dense memory and hash-stack rules, with the compound stack step.
- SRC-014 `S_MAX` in the relation bytes (already required, the code change is in kit NODE INTEGRATION).
- SRC-007 and the contract templates for universal program acceptance and collection opening (N-11, OWNER).
- The semantic reference for the full language (OD-21, OWNER).
- A mid-run consume rule, only if OD-20 selects it (OWNER).

## OWNER decisions that gate parts of this change

| Id | Decision | What it gates |
|---|---|---|
| OD-1 (R-4, M-2) | Confirm TOP >= 63 (TOP = 63 holds only with one physical cell per record, OD-11) | MEM-002, MBK-003 |
| OD-2 (R-5, N-9) | Confirm: no claim count in the public statement | DCD-004, SHP-001 |
| OD-5 (R-9) | Status of dissent D-1 (linked slices), measured by X6 | CORE-011, task 4.13 |
| OD-6 | S above 16 needs an SRC-014 amendment | CORE-006, NODE-002 |
| OD-13, OD-19 | The fallback leaf statement `sherlock-leaf-stmt/2`, only if X10 fails | BND-003, NODE-003 |
| OD-14 | `decl_schema_d` normal form and the PROG canonicality premise | PAGE-004, INIT-003 |
| OD-16 (N-12) | Exhaust kind 7 against profile `/1` | DEC-002, DCD-001 |
| OD-17 | Input admission form (recommended: by digest, empty genesis memory), measured by X11 | INIT-004, INIT-005, task F.3 |
| OD-18 | SRC-021 one-arena Init field list | INIT-001, task F.2 |
| OD-20 | Whether GP-2 also needs a mid-run consume rule or a genesis-bound dependency | CLM-006 |
| OD-21 | The normative semantic reference for the full language, sized by X13 | RFB-001 |
| OD-22 | A second deferred accumulator per SRS, only if the owner wants it | FVR-003 |
| N-6 | GP-4 "arity at least 2 plus new work" in one node, measured by X7 | SHP-003 |
| N-7 | Bracketing disclosure in `shape_d` (dissent D-6), only if it changes GP-4 text | SHP-002 |
| N-11 | Contract surface: `acceptRun` with `prog_d` as call data, claims contract | tasks F.7, F.10 |

## Impact

- Code: `crates/mori-chip` (leaf kits), `crates/mori-recursion` (node, decider, continuation, checkpoint store, GP hooks, general verifier), `crates/mori-core/src/sherlock/` (REFERENCE BRIDGE).
- Models: `models/sherlock/quint/` (new and extended models per kit, verdict files under `qa/gemini-quint/`).
- Proofs: `clean/fork/Clean/Sherlock/` (new Lean files per kit, axiom-report companions).
- Keys: the node and decider keys change with the leaf key (key cascade task).
- Specs: tasks in group F edit `openspec/changes/sherlock-vm-consensus-20261107/` as listed above.
