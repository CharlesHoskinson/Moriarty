# Design: modular Sherlock leaf and recursion

Status: S2, specified only. This file is the authoritative short form of the design. The full architecture, with the nine items of every kit, the conflicts and the traceability, is `research/sherlock-final-design-20261111/ARCHITECTURE.md` (revision B2, 3,536 lines), below `FD/ARCHITECTURE.md`. Where the two differ, `FD/ARCHITECTURE.md` is the source of truth, and the text of this change is corrected to it.

Inputs: `FD/DECISIONS.md` (R-1..R-10, E-1..E-3, L-1..L-10, M-1..M-4, A-1..A-6), `FD/ARCH-AUDIT-RESPONSE.md` (20 of 20 findings fixed as design text), experiments C1 (`FD/EXPERIMENTS.md`), C3 (`FD/EXPERIMENTS-C3.md`), X1 and X2 (`FD/EXPERIMENTS-MAXIMA.md`), and the C1 Quint verdict `qa/gemini-quint/c1-dense/verdict.md`. Governing change: `openspec/changes/sherlock-vm-consensus-20261107/` (below `O/`).

## 1. Goals and non-goals

Goal: one agent builds and checks one kit against its frozen interface, and the memory backend and the boundary commitment each change behind one seam, while GP-1 to GP-4 (`O/general-purpose.md`) stay unchanged.

Non-goals: Profile B as a built path, continuation mode 2, semantic profile `/2`, time and throughput gates, and any change of GP text. A narrowing option is never an acceptance path (A-6).

## 2. Module graph

```text
                 published run statement [prog_d, input_d, outcome_d, n_steps]
                                       ^
                                  DECIDER (opens the sealed SET head)   LEDGER CONTRACT (one ledger)
                                       ^
  GP-2 hook (claims) ---> NODE INTEGRATION <--- GP-3 hook, GP-3 general verifier
                          (verify children, link heights, pre_d/post_d,
                           the only seal; ADMIT-WORK, VIEW-WORK)
                                       ^  <--- GP-4 hook (shape, MERGE-WORK, JOIN)
                                       ^  <--- CONTINUATION + HOST CHECKPOINT STORE (host)
        leaf public lanes [binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]
  ======================== one LEAF circuit, one key ===========================
   CORE --state cells--> BOUNDARY COMMITMENT (seam 2) <-- token -- MEMORY BACKEND (seam 1)
    |  \--stack steps--> STACKS --stack cells--^              ^         ^
    |   \--append chain--> MEM window --Window port-----------+    PAGE/ROM block cache
    |   INIT (gated by h_pre == 0) between BOUNDARY COMMITMENT and CORE
    +== BUS (2 request lanes, one provider table) <== ALU, HASH, MEM window, heap lines, ROM lines
    +== tagged fixed table (RANGE | DEC)              VALUE DIGEST -> HASH whole-sponge slots
  Host only: REFERENCE BRIDGE (semantic reference, event map, coverage gate)
```

Rules of the graph:

- CORE is the only bus requester. VALUE DIGEST and INIT take records only as `AuthRec` and `AuthRom` tokens of the read port of CORE.
- A module never reads the columns of another module. The only channels are a bus tuple `(chip, key, a, b, c)` and a named copy-constrained port.
- Bus tuples have levels: tagged-table rows 0, ALU, HASH and ROM 1, MEM 2 with the address as sub-level, CORE requests above. Ports follow the build-order DAG. A port guarantee travels as a port certificate (Lean O42, open).
- The bus is a membership lookup (`LookupSound`). No argument uses `BusBalanced` for the landed bus.
- MEM validity is the history invariant `MemHist`: INIT sets it at genesis, and each leaf proves it for `post` from `pre` (Lean O43, open).

## 3. Frozen bus schema and ports

| chip id | Name | Tuple | Provider | Status |
|---:|---|---|---|---|
| 0 | disabled | `(0, 0, 0, 0, 0)` | none | landed |
| 1 | ALU | `(1, op + 8 flag, x, y, z)` | ALU | ADD and LT landed, SUB, MUL and EQ new |
| 2 | MEM | `(2, 8 addr + kind, a, b, c)`, kind `0..=4` | MEM window (`addr >= 8 q_pre`), heap lines (`addr < 8 q_pre`) | landed (C1) |
| 3 | HASH | `(3, BUS_KEY, x, y, H([x, y]))` | HASH | landed |
| 4 | ROM | `(4, tbl 2^D_row + row, a, b, c)` | PAGE/ROM ROM lines | new (id proposed) |
| 5+ | reserved | unsatisfiable | none | |

Provider rules 1 to 7 of `FD/ARCHITECTURE.md` section 1.2 bind every provider kit. The append transport is the landed hybrid: a copy-linked address chain (A1 to A3) and one MEM bus request per append (A4). No challenge, no second phase, rotations `-1, 0, +1`, degree at most 6.

Named ports (section 1.3): `Window`, append chain with `len_at`, `AuthRec`, `AuthRom`, memory token `[root, tail_d]`, `ColdRead`, state cells, stack cells, state header cells, whole-sponge slot, RANGE port, DEC port, leaf public lanes. Lane 5 is `rules_d` (SRC-001), computed by CORE.

## 4. The two seams

| MEMORY BACKEND (seam 1) | BOUNDARY COMMITMENT (seam 2) | Status |
|---|---|---|
| dense tail-buffered Merkle with heap and ROM lines | Poseidon header and body | Profile A, the built path |
| ZmLogUp | Horner | Profile B, benchmark-only, not before M4 |
| dense Merkle | Horner | not offered |
| ZmLogUp | Poseidon | not offered |

Seam 1 is the trait `MemBackend` (`persist`, `cold_read`, one block-line method) with an opaque token. Seam 2 is one trait over the serialized boundary cells: a capacity-tagged body digest and a six-input pinned header over `outcome_tag`, `outcome_payload`, `input_d`, `deps_d`, `body_d`. The NODE seal modes read the header with `open_state_header` (X10 measures its cost); the run decider opens the sealed SET head and reads no header. A swap changes only the backend files, the token width and, for Profile B, the leaf statement, node, seal and continuation. Two profiles never share a verifying key.

## 5. Leaf shape

`LeafShape { s, w_flush, k_page, k_rom, n_cold }` is owned by CORE. Working values for the first build: `s = 16` (SRC-014, OD-6), `w_flush = 1` (M-3), one arena (M-1), `TOP = 63` (OD-1, OWNER, valid only with one physical cell per record, OD-11). `k_page + n_cold >= 4` (X1). `k_rom` waits for X15. Nothing in the shape is frozen until X15 and X5.

## 6. Trust boundaries and failure handling

- In circuit: every kit of the leaf, NODE INTEGRATION, DECIDER, the GP hooks and the general verifier. On chain: LEDGER CONTRACT (one contract, one ledger state). Their guarantees rest on the constraint tests, the Quint models and the Lean obligations.
- Host, trusted diagnostics only: REFERENCE BRIDGE. It finds divergences and proves nothing.
- Host, not a soundness dependency: HOST CHECKPOINT STORE. A corrupted store can stop the prover. It cannot make the prover prove a false statement, because each leaf constrains its reads against `pre_d`.
- Hypotheses that stay hypotheses: `digest_inj`, `stuck`, the hash injectivity premises (O7), lookup soundness (O2), accumulation and pairing soundness.
- Failure classes are kept apart: a constraint refusal (the only rejection credit), a configuration failure, a parser or decoder refusal, a host integrity failure. A panic or synthesis error earns no rejection credit.

## 7. Semantic guarantees against feasibility

Semantic guarantees are the requirements of `specs/`. Feasibility is the row gates: `k_leaf <= 16` at S = 16, node <= 950,000 rows, decider <= 1,000,000 rows with no new range-table tag, with table rows and domain headroom recorded. Every row figure in `FD/ARCHITECTURE.md` is ESTIMATE unless C1 measured it. A failed row gate never narrows GP-1 to GP-4. It leaves the GP target open and goes to the owner.

## 8. Build order

| Group | Kits (parallel inside a group) | Entry condition |
|---|---|---|
| F | interface freeze by the lead | every freeze prerequisite |
| 0 | BUS | F |
| R | REFERENCE BRIDGE (host, beside groups 0 to 4) | F, OD-21, X13 |
| 1 | RANGE, HASH, MEM window (landed, conformance) | 0 |
| 2 | ALU, DEC, STACKS, MEMORY BACKEND dense (landed, conformance and seam test), VALUE DIGEST | 1 |
| 3 | PAGE/ROM block cache, CORE framework | 2 |
| 4 | CORE rule families, BOUNDARY COMMITMENT, then INIT | 3, X1, X15 for ROM, R for the ADT families |
| L | leaf integration in `RfLeaf`, whole-leaf harness with zero coverage gaps, leaf rows and k (X5) | 4, R complete |
| 5 | NODE INTEGRATION (with `open_state_header`, X10) | L |
| 6 | DECIDER, HOST CHECKPOINT STORE then CONTINUATION, GP-3 hook, GP-4 hook | 5 |
| 7 | GP-2 hook; LEDGER CONTRACT | 6, X12; N-11 and the template recapture for LEDGER CONTRACT |
| 8 | GP-3 general verifier: FV-SCHED first, then FV-PARSE, FV-TRANSCRIPT, FV-ARITH, FV-FINAL | 6, X14 |

Profile B kits are in no group. Agents can start a later group early against mocks once group F is done.

Freeze prerequisites of group F: X1 (done except ROM rows), X2 (done), B2 audit answered (done), the Init relation (specified, input part waits for OD-17 and X11), the boundary opening interface (specified, X10 measures it), the record layouts and physical cells per record (OD-11, open), ROM depth and `K_rom` (OD-12, X15, open). The dependency discharge protocol (X12) and reference coverage (X13) gate later groups, not the leaf freeze. SRC-023 at the window capacity (O14, O47) does not block the freeze. It blocks reporting SRC-023 as proved for the circuit rule.

## 9. Gates

Per kit, all of these, recorded in one `scripts/doctorwatson check <ID>` receipt:

1. MockProver: every positive passes, every named mutant is refused at a constraint, lookup or permutation boundary from a valid baseline with a fresh malicious witness, and each mutant has its weakened control that accepts the same witness.
2. Quint: a Gemini lane through `argon-run` with the quint-llm-kit skills writes or extends the model under `models/sherlock/quint/`, the lead re-runs it, and a verdict file in the form of `qa/gemini-quint/c1-dense/verdict.md` records invariants, commands, bounds, seeds, mutant results, assumptions and model-to-code differences.
3. Lean: `lake build SherlockSpec` green, an axiom-report companion per new module, and every obligation that the kit requires is a proved theorem whose `#print axioms` shows only `propext`, `Quot.sound` and `Classical.choice`. A `sorry` keeps the obligation open.
4. Rust: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`. Until the clippy baseline task passes, a kit records the clippy result of its own files and the workspace gate is reported red.
5. T0 on every commit. Heavy jobs only through `scripts/doctorwatson heavy`.

Integration: the whole-leaf harness `crates/mori-chip/tests/diff_leaf_vs_sherlock_ka1.rs` under three partitions on every module change, the REFERENCE BRIDGE coverage report at zero gaps, the row gates of section 7, the node height edge, SRC-010 and SRC-020 real-proof checks, and the GP acceptance matrix of `FD/ARCHITECTURE.md` section 3.3. A GP target is complete only when every line of its matrix has passing evidence. A prototype milestone never ticks a GP target.

## 10. Lean obligations

The consolidated table of `FD/ARCHITECTURE.md` section 2 has 69 rows: P1 to P14 proved in C3, O1 to O55 open. P7, P9, P10 and P14 hold only for leaves of at most 8 appends. P2 and P3 cover bus tuples only. P1 is not evidence for the landed bus. O21, O22 and O30 belong to Profile B and are not scheduled. Each spec scenario names the obligation id and its status.

## 11. Open decisions

OWNER decisions that gate parts of this change are listed in `proposal.md`. The other open decisions (OD-3, OD-8, OD-9, OD-11, OD-12, OD-15, N-1, N-4, N-5, N-8, N-13, V-1 to V-10) are taken as recommended in `FD/ARCHITECTURE.md` section 4 and are decided by the experiment that each names. The experiments X3 to X15 that are still open are tasks in `tasks.md`.
