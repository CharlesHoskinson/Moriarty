# Design: modular Sherlock leaf and recursion

Status: S2, specified only. This file is the authoritative short form of the design. The full architecture, with the nine items of every kit, the conflicts and the traceability, is `research/sherlock-final-design-20261111/ARCHITECTURE.md` (revision B2), below `FD/ARCHITECTURE.md`. Where the two differ, `FD/ARCHITECTURE.md` is the source of truth, and the text of this change is corrected to it. The exceptions are the owner rulings of 2026-11-12 and 2026-11-13 that section 9 cites; they postdate the shape-bound table of `FD/ARCHITECTURE.md` section 1.9 and override it.

Refreshed 2026-11-13 to the STARK design (row T6a of `research/sherlock-tested-design-20261113/LOOP.md`, from commit `bcae0f185`). The kits are OpenVM AIRs over BabyBear under stark-backend. The halo2 gates (MockProver, `k_leaf <= 16`, the node and decider row gates) are history (DG-007 of `sherlock-processor-final-20261111`). The STARK kit gate of section 9 replaces them. The semantics, the interfaces and the Lean obligations of every kit do not change.

Inputs: `FD/DECISIONS.md` (R-1..R-10, E-1..E-3, L-1..L-10, M-1..M-4, A-1..A-6), `FD/ARCH-AUDIT-RESPONSE.md` (20 of 20 findings fixed as design text), experiments C1 (`FD/EXPERIMENTS.md`), C3 (`FD/EXPERIMENTS-C3.md`), X1 and X2 (`FD/EXPERIMENTS-MAXIMA.md`), the C1 Quint verdict `qa/gemini-quint/c1-dense/verdict.md`, and the owner rulings of `research/sherlock-processor-final-20261111/DECISIONS-ASK.md` (below `DA`). Governing change: `openspec/changes/sherlock-vm-consensus-20261107/` (below `O/`).

## 1. Goals and non-goals

Goal: one agent builds and checks one kit against its frozen interface, and the memory backend and the boundary commitment each change behind one seam, while GP-1 to GP-4 (`O/general-purpose.md`) stay unchanged.

Non-goals: Profile B as a built path, continuation mode 2, semantic profile `/2`, time and throughput gates, and any change of GP text. A narrowing option is never an acceptance path (A-6).

## 2. Module graph

```text
                 call vector of 2 + L_TX words (run statement as effect words)
                                       ^
                                  L AND WRAP (change sherlock-processor-final-20261111)   LEDGER CONTRACT (one ledger)
                                       ^
  GP-2 hook (claims) ---> NODE INTEGRATION <--- GP-3 hook, GP-3 general verifier
                          (verify children, link heights, pre_d/post_d,
                           the only seal; ADMIT-WORK, VIEW-WORK)
                                       ^  <--- GP-4 hook (shape, MERGE-WORK, JOIN)
                                       ^  <--- CONTINUATION + HOST CHECKPOINT STORE (host)
        leaf public lanes [binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]
  ============ one LEAF shard: OpenVM AIRs, fixed shape from the plan menu ========
   CORE --state cells--> BOUNDARY COMMITMENT (seam 2) <-- token -- MEMORY BACKEND (seam 1)
    |  \--stack steps--> STACKS --stack cells--^              ^         ^
    |   \--append chain--> MEM window --Window port-----------+    PAGE/ROM block cache
    |   INIT (gated by h_pre == 0) between BOUNDARY COMMITMENT and CORE
    +== LogUp buses ALU, MEM, HASH, ROM <== ALU, HASH, MEM window, heap lines, ROM lines
    +== LogUp buses RANGE, DEC (fixed tables)          VALUE DIGEST -> HASH whole-sponge slots
  Host only: REFERENCE BRIDGE (semantic reference, event map, coverage gate)
```

Rules of the graph:

- CORE is the only requester of buses 1 to 4 and 6. Every kit may query the RANGE bus 5. VALUE DIGEST and INIT take records only as `AuthRec` and `AuthRom` tokens of the read port of CORE.
- A module never reads the columns of another module. The only channels are a bus message on one LogUp bus and a named port (FP-1 of `FD/ARCHITECTURE.md` section 1.11: shared columns of one row inside one AIR, or a `PermutationCheckBus` between two AIRs).
- Bus tuples have levels: tagged-table rows 0, ALU, HASH and ROM 1, MEM 2 with the address as sub-level, CORE requests above. Ports follow the build-order DAG. A port guarantee travels as a port certificate through the port's multiset equality (Lean O42, open).
- The bus is a membership lookup (`LookupSound`) under the LogUp bounds of FP-1 (in place of BUS-007). No argument uses `BusBalanced` for the landed bus.
- MEM validity is the history invariant `MemHist`: INIT sets it at genesis, and each leaf proves it for `post` from `pre` (Lean O43, open).

## 3. Frozen bus schema and ports

The bus and the ports freeze in the BabyBear form of `FD/ARCHITECTURE.md` section 1.11, FP-1 (FPR-001 to FPR-004 of `sherlock-processor-final-20261111`). Each chip id is one stark-backend LogUp bus with index `BUS_BASE + id`. The bus index is part of the message, so the `chip` cell of the prototype tuple is not a lane, and a disabled request has count 0 in place of the zero tuple. Each field is its own typed lane in 16-bit little-endian limbs; no word packs two parts.

| id | Bus | Message (FP-1) | Provider | Requesters |
|---:|---|---|---|---|
| 1 | ALU | `op: tag(4)`, `flag: bit`, `x, y, z: u128` | ALU | CORE |
| 2 | MEM | `kind: tag(3)` in `1..=4`, `addr`, `rec` (`W_rec` lanes, waits T5) | MEM window (`addr >= 8 q_pre`), heap lines (`addr < 8 q_pre`), MEM-004 | CORE |
| 3 | HASH | `tag: u16`, `x, y, h: digest`, `h = P2(tag, x, y)` | HASH | CORE |
| 4 | ROM | `tbl: tag(D_tbl)`, `row: u32`, `rec` (`W_rom` lanes, waits T5) | PAGE/ROM ROM lines | CORE |
| 5 | RANGE | `x`, `w: tag(5)`, `0 <= w <= 16` | stock `VariableRangeCheckerAir` at 16 bits, 2^17 rows (LD-S10x-3) | every kit |
| 6 | DEC | `dec_key`, `dec_val` in `u16` parts (AD-9, waits T5) | DEC table with the idle row | CORE |
| 7 to 15 | reserved | none | no AIR | a request is unsatisfiable |

The MEM provider split uses the MEM-004 form with `q_pre` a block index; FP-1 writes `addr >= q_pre`, a discrepancy left for row T2. The halo2 prototype tuple `(chip, key, a, b, c)` over `Fq` with packed keys (`8 addr + kind`, `tbl 2^D_row + row`, `op + 8 flag`) is history and is the form of the landed `crates/mori-chip` tests only. Provider rules 1 to 6 of `FD/ARCHITECTURE.md` section 1.2 bind every provider kit. Provider rule 7 is a halo2 rule: over stark-backend an AIR reads its local row and its next row only, stays inside the shape bounds of its layer, and draws no challenge of its own. The append transport is unchanged: the append-chain port (A1 to A3) and one MEM bus request per append (A4).

Named ports (section 1.3, in the FP-1 form): `Window`, append chain with `len_at`, `AuthRec`, `AuthRom`, memory token `[root, tail_d]`, `ColdRead`, state cells, stack cells, state header cells, whole-sponge slot, RANGE port, DEC port, leaf public values. The leaf public values are the 54 words of FP-1: `binding`, `prog_d`, `pre_d`, `post_d`, `rules_d` (SRC-001, computed by CORE), `h_pre`, `steps`, `log_d`, `log_n`.

## 4. The two seams

| MEMORY BACKEND (seam 1) | BOUNDARY COMMITMENT (seam 2) | Status |
|---|---|---|
| dense tail-buffered Merkle with heap and ROM lines | Poseidon header and body | Profile A, the built path |
| ZmLogUp | Horner | Profile B, benchmark-only, not before M4 |
| dense Merkle | Horner | not offered |
| ZmLogUp | Poseidon | not offered |

Seam 1 is the trait `MemBackend` (`persist`, `cold_read`, one block-line method) with an opaque token. Seam 2 is one trait over the serialized boundary cells: a capacity-tagged body digest and a six-input pinned header over `outcome_tag`, `outcome_payload`, `input_d`, `deps_d`, `body_d`. The NODE seal modes read the header with `open_state_header` (X10 measures its cost). Kit L AND WRAP opens the sealed SET head and reads no header. A swap changes only the backend files, the token width and, for Profile B, the leaf statement, node, seal and continuation. Two profiles never share a verifying key.

## 5. Leaf shape

`LeafShape { s, w_flush, k_page, k_rom, n_cold }` is owned by CORE. Working values for the first build: `s = 16` (SRC-014, AD-6), `w_flush = 1` (M-3), one arena (M-1), `TOP = 63` (AD-1, OWNER, valid only with one physical cell per record, AD-11). `k_page + n_cold >= 4` (X1). `k_rom` waits for X15. Nothing in the shape is frozen until X15 and X5.

A leaf shard is a fixed shape from the plan's shape menu (`FD/ARCHITECTURE.md` section 1.6, the shard plan). The shape fixes the chip set, the AIR list in order and the trace height of every AIR. No height is chosen from the witness: a leaf with fewer steps pads to the height of its shape, with disabled slots (CORE-001). Every leaf shard of one run has the height of its menu shape. The heights are PENDING measurement (task V.9 and X5). The only fixed height in the sources is the 2^17-row RANGE table. The cost of a shape is seconds per proved step on the GPU (DG-006).

## 6. Trust boundaries and failure handling

- In circuit: every kit of the leaf, NODE INTEGRATION, the GP hooks and the general verifier. The bridge layer `L` and the wrap are kit L AND WRAP of `sherlock-processor-final-20261111`, which supersedes kit DECIDER. On chain: LEDGER CONTRACT (one contract, one ledger state). Their guarantees rest on the constraint tests, the Quint models and the Lean obligations.
- Host, trusted diagnostics only: REFERENCE BRIDGE. It finds divergences and proves nothing.
- Host, not a soundness dependency: HOST CHECKPOINT STORE. A corrupted store can stop the prover. It cannot make the prover prove a false statement, because each leaf constrains its reads against `pre_d`.
- Hypotheses that stay hypotheses: `digest_inj`, `stuck`, the hash injectivity premises (O7), lookup soundness (O2), and the soundness of the STARK recursion (premise A4 of `bridge_sound`).
- Failure classes are kept apart: a constraint refusal (the only rejection credit), a configuration failure, a parser or decoder refusal, a host integrity failure. A panic or synthesis error earns no rejection credit.

## 7. Semantic guarantees against feasibility

Semantic guarantees are the requirements of `specs/`. Feasibility is the derived gates DG-001 to DG-007 of `sherlock-processor-final-20261111` (`FD/ARCHITECTURE.md` section 3.3): no BLS12-381 circuit above k = 21, the wrap at 85 percent of `2^k - 7` usable rows at the smallest k of 19, 20 and 21, fixed STARK shard shapes inside the shape bounds of their layer (section 9), and STARK leaf and node cost in seconds per proved step. A STARK trace has a height, never a circuit k. The machine-level kits map onto the AIRs of the leaf shards by the STARK kit gate of section 9 and the term map of section 12. Their row figures in `FD/ARCHITECTURE.md` section 2 are halo2 prototype figures (MEASURED in C1 or ESTIMATE), including the figures of AD-3 and AD-4, and are not gates. A failed gate never narrows GP-1 to GP-4. It leaves the GP target open and goes to the owner (DG-010).

Superseded 2026-11-11: the halo2 row gates `k_leaf <= 16` at S = 16, node at most 950,000 rows and decider at most 1,000,000 rows with no new range-table tag (text at commit `0ab8cd79d`). Superseded 2026-11-13: MockProver as the gate of a kit. MockProver stays only for the halo2 prototypes in `crates/mori-chip` and for the wrap circuit (DG-008).

## 8. Build order

| Group | Kits (parallel inside a group) | Entry condition |
|---|---|---|
| F | interface freeze by the lead | every freeze prerequisite |
| V | vertical slice: BUS, RANGE, HASH, CORE with a small rule subset, MEM window, BOUNDARY COMMITMENT, INIT, as STARK AIRs in one fixed-shape leaf shard; one Sherlock program run proves leaf, N1, N2, N3, `L`, `L_Fr` (the T6 done-when) | the F.1 parts that the slice uses, T3 (every AIR required), T5 rows I-5, I-9, I-15 (section 13) |
| 0 | BUS | F, V |
| R | REFERENCE BRIDGE (host, beside groups 0 to 4) | F, AD-21, X13 |
| 1 | RANGE, HASH, MEM window (landed, conformance) | 0 |
| 2 | ALU, DEC, STACKS, MEMORY BACKEND dense (landed, conformance and seam test), VALUE DIGEST | 1 |
| 3 | PAGE/ROM block cache, CORE framework | 2 |
| 4 | CORE rule families, BOUNDARY COMMITMENT, then INIT | 3, X1, X15 for ROM, R for the ADT families |
| L | leaf integration in the STARK leaf shard, whole-leaf harness with zero coverage gaps, leaf shapes and cost in seconds per proved step (X5) | 4, R complete |
| 5 | NODE INTEGRATION (with `open_state_header`, X10) | L |
| 6 | L AND WRAP (change `sherlock-processor-final-20261111`), HOST CHECKPOINT STORE then CONTINUATION, GP-3 hook, GP-4 hook | 5 |
| 7 | GP-2 hook; LEDGER CONTRACT | 6, X12; N-11 and the template recapture for LEDGER CONTRACT |
| 8 | GP-3 general verifier: FV-SCHED first, then FV-PARSE, FV-TRANSCRIPT, FV-ARITH, FV-FINAL | 6, X14 |

Group V comes first because no kit gate of section 9 can pass before a real Sherlock leaf shard proves through the recursion layers. The slice builds each listed kit only as far as one chosen program needs it. Its rule subset is the rules of that program, and DEC holds only the rows of the subset. The program is chosen to keep the other kits at their genesis or stub values: no stack push or pop (the FRAMES and TASKS heads stay at their genesis constants), fewer than 8 records allocated (no segment flush, no heap line, no scalar cold read) and no ROM row beyond what INIT opens. If the smallest such program needs a stack step, a flush or a ROM row, the STACKS step (STK-001 to STK-003), the flush (MBK-003) or the ROM line (PAGE-004) over those rows enters the slice, and task V.1 records why. Every program reads its entry expression row `expressions[e0]`, so the ROM line for the rows of the V.1 program is in the slice (task V.8). A slice task ticks no kit task of groups 0 to 4 and no GP target. The kit tasks then extend the slice AIRs to the full kit. They do not replace them.

Profile B kits are in no group. Agents can start a later group early against mocks once group F is done.

Freeze prerequisites of group F: X1 (done except ROM rows), X2 (done), B2 audit answered (done), the Init relation (specified, input part waits for AD-17 and X11), the boundary opening interface (specified, X10 measures it), the record layouts and physical cells per record (AD-11, waits T5), ROM depth and `K_rom` (AD-12, X15, waits T5). The dependency discharge protocol (X12) and reference coverage (X13) gate later groups, not the leaf freeze. SRC-023 at the window capacity (O14, O47) does not block the freeze. It blocks reporting SRC-023 as proved for the circuit rule.

## 9. Gates

**STARK kit gate.** Every kit whose relation is proved in a STARK layer meets SG-1 to SG-6. These are the leaf kits BUS, RANGE, ALU, DEC, HASH, CORE, STACKS, MEM window, MEMORY BACKEND, PAGE/ROM, VALUE DIGEST, BOUNDARY COMMITMENT and INIT, and the node-side kits NODE INTEGRATION, CONTINUATION (its node part), GP-2 hook, GP-3 hook, GP-4 hook and the GP-3 general verifier. The host kits REFERENCE BRIDGE and HOST CHECKPOINT STORE have host tests only. LEDGER CONTRACT has Compact contract tests. DECIDER is superseded by kit L AND WRAP, whose gates are in `sherlock-processor-final-20261111`. Profile B (ZmLogUp) is not built, and its halo2 text stays as history.

- **SG-1 Keygen with every AIR required.** stark-backend keygen keys the kit's AIRs in their menu shape with every AIR required, never optional (row T3 of `research/sherlock-tested-design-20261113/LOOP.md`). A proof that omits a kit AIR is refused at N1, N3 and `L` (mutant `<kit>_air_omitted`). Keygen refuses a key outside the shape bounds of SG-4 and a key whose per-round query vector differs from the frozen one (SQG-002). The FP8 round 2 record found every AIR optional in the bound keys (`out/kb/univ.json`, commit `bcae0f185`). No kit passes SG-1 before T3 passes.
- **SG-2 Positive proofs.** Each positive is a real proof of the witness by the stark-backend prover at the parameter set of its layer. The stock stark-backend verifier, unmodified, accepts it. The debug constraint checker also passes. MockProver is not a gate of a STARK kit.
- **SG-3 Negative proofs.** Each named mutant starts from a valid baseline with a fresh malicious witness. The debug constraint checker names the failing constraint, LogUp bus or permutation bus. A proof made from the witness with the debug checker off is refused by the stock verifier. That refusal is the rejection credit. The weakened control AIR (the constraint under test removed) gives a proof of the same witness that the stock verifier accepts. A prover panic, a tracegen error or a keygen error earns no rejection credit (DG-008). Keygen and shape-check refusals are recorded apart.
- **SG-4 Fixed shape and layer budgets.** Every AIR has the fixed height of its menu shape (section 5). The shape stays inside the bounds of its layer, and keygen refuses every other shape (DG-004):
  - app set (leaf shards): at most 5,000 constraints and 1,000 interactions per AIR, 100 AIRs and 30,000 columns (`FD/ARCHITECTURE.md` section 1.9), with the LogUp bounds 1 to 3 of FP-1 (message at most 128 words, the per-bus and total count-times-height sums below `p`);
  - internal set (every node, the leaf kits inside the inline kernel of SHP-006, the GP hooks): height at most 2^19, at most 1,000 constraints and 200 interactions per AIR (owner ruling 2026-11-12 20:35, X14 option 2, `DA` line 82), at most 64 AIRs and 3,000 columns (same ruling, X7 and N-6 option 2, `DA` line 81). These values override the 100 interactions, 50 AIRs and 2,000 columns of the `FD/ARCHITECTURE.md` section 1.9 table;
  - FV work units only: at most 3,300 columns, with the 17-bit range checker (owner rulings O1 and O2 of 2026-11-13, `DA` line 92). Every other kit uses the 16-bit RANGE table of LD-S10x-3.
- **SG-5 D3 at the measured shapes.** The stock D3 calculator runs on the frozen parameter set of the layer at the measured shape of the keyed shard (AIR count, columns, interactions per AIR, trace heights). It prints every term at 100 bits or more and every grindable term at 101 bits or more (margin rule OD-4). D3 runs at task V.10, at task L.1 and after every change of a kit's AIR list, column count or interaction count. A failed term goes to the owner (DG-010).
- **SG-6 Cost.** The receipt records the cost of the shape in seconds per proved step on the GPU, with the run record and the host load (DG-006). No row count is a gate.

Per kit, all of these, recorded in one `scripts/doctorwatson check <ID>` receipt:

1. The STARK kit gate SG-1 to SG-6 (a host kit: its host tests and refusals; LEDGER CONTRACT: its contract tests). The halo2 prototypes and the wrap circuit keep MockProver (DG-008).
2. Quint: a Gemini lane through `argon-run` with the quint-llm-kit skills writes or extends the model under `models/sherlock/quint/`, the lead re-runs it, and a verdict file in the form of `qa/gemini-quint/c1-dense/verdict.md` records invariants, commands, bounds, seeds, mutant results, assumptions and model-to-code differences.
3. Lean: `lake build SherlockSpec` green, an axiom-report companion per new module, and every obligation that the kit requires is a proved theorem whose `#print axioms` shows only `propext`, `Quot.sound` and `Classical.choice`. A `sorry` keeps the obligation open.
4. Rust: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`. Until the clippy baseline task passes, a kit records the clippy result of its own files and the workspace gate is reported red.
5. T0 on every commit. Heavy jobs only through `scripts/doctorwatson heavy` and `heavy-slot`.

Hard rules that every gate keeps: never k = 22; every proof is a fixed-size shard; per-program, witness-independent preprocessing bound to `prog_d` through `key_root`; zero knowledge end to end, with no helper that sees the plain witness and seeded proofs in tests only; GP-1 to GP-4 never narrowed.

Integration: the whole-leaf harness `crates/mori-chip/tests/diff_leaf_vs_sherlock_ka1.rs` under three partitions on every module change, on real proofs of the STARK leaf shard; the REFERENCE BRIDGE coverage report at zero gaps; the derived gates of section 7; the node height edge; SRC-010 and SRC-020 real-proof checks; and the GP acceptance matrix of `FD/ARCHITECTURE.md` section 3.3. A GP target is complete only when every line of its matrix has passing evidence. A prototype or slice milestone never ticks a GP target.

## 10. Lean obligations

The consolidated table of `FD/ARCHITECTURE.md` section 2 has 69 rows: P1 to P14 proved in C3, O1 to O55 open. P7, P9, P10 and P14 hold only for leaves of at most 8 appends. P2 and P3 cover bus tuples only. P1 is not evidence for the landed bus. O21, O22 and O30 belong to Profile B and are not scheduled. Each spec scenario names the obligation id and its status.

## 11. Open decisions

OWNER decisions that gate parts of this change are listed in `proposal.md`. The other open decisions (AD-3, AD-8, AD-9, AD-11, AD-12, AD-15, N-1, N-4, N-5, N-8, N-13, V-1 to V-10) are taken as recommended in `FD/ARCHITECTURE.md` section 4 and are decided by the experiment that each names. The experiments X3 to X15 that are still open are tasks in `tasks.md`.

## 12. Term map from the halo2 text

Some scenario text of `specs/` keeps halo2 words. In every STARK kit (section 9) they mean:

| Text in a requirement | Meaning for a STARK kit |
|---|---|
| "the constraint checker accepts" X | SG-2: a real proof of X verifies on the stock verifier |
| "the correct circuit refuses the witness", "the node (unit, lift, leaf) is unsatisfiable" | SG-3: the stock verifier refuses the negative proof |
| "control circuit", "control port", "control test" | the weakened control AIR of SG-3 |
| "configuration fails", "refused at synthesis", "refused before key generation" | a keygen or shape-check refusal, recorded apart |
| "copy-constrained", "copy-linked", "copied", "copy equality" | an FP-1 port: shared columns of one row, or one `PermutationCheckBus` message `(seq, cells)` |
| "fixed column", "fixed selector", "fixed table" | a preprocessed trace column bound in the key under `key_root`; the RANGE table is the exception, with 4 main columns, no preprocessed trace and its row set fixed by constraints (FP-1) |
| "advice cell" | a main trace column (witness) |
| "request lane", "the zero tuple", "chip id 0" | one interaction of the requester AIR; a disabled request has count 0 |
| "packed key" | separate typed lanes (FP-1) |
| "the only bus requester", "issues no bus request" | of buses 1 to 4 and 6; every kit may query the RANGE bus 5 |
| "public lane" | a leaf public value (FP-1, 54 words) |
| `configure`, `assign`, `Region`, `Layouter` | `KitAir` and `KitTrace` of FP-1 |

Mutant and test names keep their names. Each name now denotes the STARK witness of the same attack. A mutant whose attack the FP-1 encoding removes (a packed key that wraps the field) becomes a control of FP-1 (`bus_key_packed_alias`).

## 13. Fields that wait on T5

The leaf schemas are drafted in `research/sherlock-tested-design-20261113/T5-SCHEMAS.md` (DRAFT for owner sign-off, ruling O12). The fields below stay open until their row of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN. No value of the draft is a requirement of this change.

| Field | 1.12 row | Requirements that use it |
|---|---|---|
| AD-11 record layouts: `W_rec`, the lane types per kind, the physical cells per record, the `AuthRec` cells | I-5, I-9 | BUS-001, MEM-003, MEM-007, MBK-004, CORE-004, VDG-004, PAGE-001 |
| State schema body words | I-15 | BND-001 |
| AD-12 ROM tables and rows: `D_tbl`, `D_row`, `W_rom`, the `AuthRom` cells | I-6, I-9 | PAGE-004, CORE-004, VDG-003 |
| AD-9 guard list and control word: `dec_key`, `dec_val` | I-7 | DEC-001, DEC-003, DEC-004, BUS-004 |
| `D_view`, the view form, the `AuthIn` cells | I-10 | CORE-013, INIT-006, NODE-012, NODE-013, VDG-005 |
| `ty_d` normal form (AD-14) | I-14 | NODE-011, INIT-006, PAGE-004 |
| SRC-021 Init field list (and the SRC-022 form it names) | I-15 | INIT-001, INIT-002 |
