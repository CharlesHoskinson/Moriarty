# Architecture of the Sherlock co-processor

Sherlock is a universal, elastic proving co-processor for Midnight. A user runs any program on the Sherlock machine, proves the run as a tree of fixed-size small-field STARK shards on the user's own GPU, compresses the tree into one last STARK layer `L` hashed over BLS12-381 Fr, and wraps `L` in one blinded midnight-proofs PLONK proof that the unchanged ledger 9.1.0.1 verifier accepts.

This file is the design. Its source is the merged proposal `research/sherlock-processor-final-20261111/PROPOSAL.md` (cited as `PR` with a section number); the co-processor decisions OD-1 to OD-18 and the machine-level decisions AD-1 to AD-23 are in `FD/DECISIONS.md`, the experiments in `FD/EXPERIMENTS.md`, the terms in `FD/TERMINOLOGY.md`. Section 1 gives the layer stack and the frozen co-processor interfaces. Section 2 gives the kits: the machine-level kits of the leaf (CORE to INIT), the recursion and claim kits, the bridge layer and wrap (kit L AND WRAP), the ledger contract, and the kits ZERO KNOWLEDGE and COMPACT FRONTEND.

Hard rules. The design keeps each of them by construction except the proven 100-bit rule, which holds only once the security qualification gate of section 1.9 passes; until then every bit count in this file is a calculator figure (MEASURED by D3, or ESTIMATE), not a proven bound, and no universal key freezes. The rules: never k = 22; every proof is a fixed-size shard; per-program, witness-independent preprocessing bound to `prog_d`; zero knowledge end to end, with every STARK layer on the user's machine and no helper that sees the plain witness; proven 100-bit security with no unproved conjecture; GP-1 to GP-4 never narrowed; one universal key set; the ledger 9.1.0.1 verifier unchanged and a verifier key of at most 50,000 bytes.

Conventions:

- Paths are relative to the repository root. `FD/` is `research/sherlock-final-design-20261111/`. `PR` is `research/sherlock-processor-final-20261111/PROPOSAL.md`. `O/` is `openspec/changes/sherlock-vm-consensus-20261107/`. `RL/` is `research/sherlock-runlength-consensus-20261111/`. `LC/` is `research/sherlock-lookup-cache-20261111/`. `CO/` is `research/sherlock-compact-open-problems-20261111/`. `CT/` is `research/sherlock-call-time-opt-20261111/`. `chip/` is `crates/mori-chip/src/`. `lean/` is `clean/fork/Clean/Sherlock/`. `qnt/` is `models/sherlock/quint/`. Draft citations `A:` and `G:` are `FD/drafts/astra-area*.md` and `FD/drafts/gemini-area*.md`.
- MEASURED: a number with its record. ESTIMATE: a derived number with named inputs. PENDING: a measurement that has not run (`FD/EXPERIMENTS.md`). LEAD: a constant that a co-processor decision sets, binding only after the owner confirms it. DERIVED (in `FD/EXPERIMENTS.md` and D3): an ESTIMATE computed by a stated rule from MEASURED numbers; it is not a measurement.
- The machine-level kits were prototyped as halo2 chips in `crates/mori-chip` and `crates/mori-recursion`. Their invariants, Lean obligations, Quint models and named mutants are statements about the machine; how each kit maps onto the STARK AIRs of the leaf shards is open work of the specs. Their row figures are halo2 prototype figures (MEASURED in experiment C1 or ESTIMATE); the STARK leaf and node have no fixed row gate, and their cost is measured in seconds per proved step on the GPU (section 1.6).
- "Proved in C3": the theorem is named in `FD/EXPERIMENTS-C3.md`. Every other Lean obligation is open. A "proved in C3" row states the premises of the Lean statement; a proved theorem is not credited with a claim that its statement does not make.
- `chip/dense.rs` line numbers are of commit `f36eb714d` unless a citation names commit `b85b00213`.
- "B2 finding N" names finding N of the architecture audit `FD/ARCH-AUDIT-astra.md`; `FD/ARCH-AUDIT-RESPONSE.md` gives the check for each. "The B1 text" is the first merged form of this file that the audit read.
- Mutant protocol (one wording for the whole file): a named negative mutant is a malicious witness that the correct circuit refuses at a constraint, lookup or permutation boundary. Its control is a deliberately weakened circuit (the constraint under test removed) that accepts the same witness. "Refused" never means that a weakened circuit is refused.
- GP-1 to GP-4 (`O/general-purpose.md`) are not narrowed by anything in this file. Where a design constant could narrow them (tree depth, cache-line counts, ROM depth, claim counter, log capacity, `L_TX`) the item is a decision with an owner mark, and the design keeps a path with no bound (the spill rule of section 1.8, chunked calls of kit COMPACT FRONTEND).

## 1. Overview

### 1.1 Module graph

```text
          Midnight, ledger 9.1.0.1: VerifierKey::verify on the call vector of 2 + L_TX words
                                              ^
                                    +--------------------+
                                    |  WRAP (G4 shape)   |  blinded PLONK-KZG over BLS12-381, k in {19, 20, 21};
                                    +--------------------+  replays the deferred field log natively
                                              ^  proof of L as private witness
                                    +--------------------+
                                    |  L (bridge layer)  |  verifies the node root; exposes the statement,
                                    +--------------------+  the anchor statement, pv (W_pv words) and X
                                              ^   run statement [C_P, input_pub, outcome_pub] as effect words
          GP-2 hook ---->  +--------------------------------------+  <---- GP-3 hook (foreign keys, registry)
          (claims)         |           NODE INTEGRATION           |  <---- GP-4 hook (any shape)
                           |  verifies leaf / node proofs, links   |
                           |  heights and pre_d / post_d           |  <---- CONTINUATION (segment tree,
                           +--------------------------------------+        retained-memory checkpoint)
                                              ^
                 leaf public lanes [binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]
                                              ^
  ====================== one LEAF shard (per chip set, per-program key) ======================
                                +---------------------+
              named cells       | BOUNDARY COMMITMENT |   seam 2: Poseidon digest (default) | Horner
        +---------------------> |  pre_d, post_d      |
        |                       +---------------------+
        |                          ^         ^      ^
        |   registers, counters    |         |      |  token (root, tail_d): opaque cells; len from MEM
  +-----------+   stack hashes +--------+    |   +----------------+   seam 1
  |   CORE    |--------------->| STACKS |    |   | MEMORY BACKEND |   dense Merkle (default) | ZmLogUp
  | slots,    |                | FRAMES |    |   | persist,       |
  | rules,    |   Window cells | TASKS  |    |   | cold_read,     |
  | append    |--------+       +--------+    |   | block lines    |<--- PAGE/ROM block cache
  | chain     |        v                     |   +----------------+     (heap lines, ROM lines)
  +-----------+   +------------+  Window     |        ^   |
     |  |  |      | MEM window |-------------+--------+   |
     |  |  |      +------------+                          |
     |  |  |            | provides (MEM, 8 addr + kind, a, b, c)   provides MEM (old records), ROM
     |  |  |            v                                 v
     |  |  +=====================  BUS  (2 request lanes, one provider table)  ==================+
     |  |                 ^                  ^                                                   |
     |  |           +-----------+      +-----------+        +--------------+                     |
     |  |           |    ALU    |      |   HASH    |<-------| VALUE DIGEST |  whole-sponge slots  |
     |  |           +-----------+      +-----------+        +--------------+  (copied cells)     |
     |  |                 |  limb checks      ^ whole-sponge slots: STACKS, MEMORY BACKEND,       |
     |  v                 v                   | BOUNDARY COMMITMENT, VALUE DIGEST                 |
     |  +--------->  tagged fixed table (tag, key, val):  tag RANGE  |  tag DEC  <----------------+
     +--- DEC lookup (one row per enabled slot)
```

Reading rules for the graph:

- An arrow into BUS is a provider; CORE is the only bus requester. VALUE DIGEST issues no bus request: it receives authenticated record and ROM cells from CORE's read port (section 1.3, `AuthRec`, `AuthRom`; B2 finding 4). A module never reads another module's columns (`FD/MODULARITY.md:8`). The only two channels are a bus tuple `(chip, key, a, b, c)` and a named copy-constrained port.
- Two orders make the composition well founded (B2 finding 7). They are different orders and neither replaces the other.
  1. Bus tuples have levels, as `Contract.ordered` in `lean/ModularComposition.lean:63-65` requires: level 0 the tagged table rows (RANGE, DEC); level 1 ALU, HASH (two-input provider) and ROM tuples; level 2 MEM tuples; CORE requests are above all of them. A MEM tuple at address `a` has the sub-level `a`: a read is below the running length, so a record depends only on records at lower addresses (the history invariant below).
  2. Ports have a dependency order, which is the build order of section 3.1 read as a DAG: HASH slots -> STACKS, MEMORY BACKEND, PAGE/ROM, VALUE DIGEST, BOUNDARY COMMITMENT, INIT; RANGE -> ALU, DEC, CORE, INIT, VALUE DIGEST; MEM window -> MEMORY BACKEND; MEMORY BACKEND token, STACKS cells and CORE state cells -> BOUNDARY COMMITMENT -> INIT. A guarantee that travels through a port is a port certificate: the producer's relation over the port cells, used by the consumer through the copy equalities. The C3 composition theorem (`provided_valid`, `leaf_refines`) covers bus tuples only; the port form is the open obligation O42 (kit BUS).
- History invariant of memory (B2 finding 7). The validity of a MEM tuple is not a fact of one leaf. It is `MemHist`: the token `pre` commits to the append log of the run so far (INIT establishes it at genesis; each leaf proves it for `post` from `pre` and its append events). The logical append events of a leaf are its new active window rows; a CORE append request is a check that the rule's record equals that row (conditions A1 to A4 of kit CORE). A stored `vd` at address `a` is computed from records at addresses below `a`, so the order is the address order.
- The bus is a lookup: it gives membership (`LookupSound`), not a permutation between request and provider lists. Unused provider rows and repeated requests are legal (`chip/bus.rs:50-61`). No argument in this file may use `BusBalanced` for the landed bus.
- Not drawn: INIT (genesis and input admission) sits between BOUNDARY COMMITMENT and CORE and is gated by `h_pre == 0`; it is a kit of section 2. The host-side kits REFERENCE BRIDGE and HOST CHECKPOINT STORE and the GP-3 general verifier kits are outside the leaf circuit.
- Everything below the double line is one leaf shard of one chip set, proved under that chip set's per-program app key, which `key_root` binds into `prog_d` (section 1.6). NODE INTEGRATION, CONTINUATION and the GP hooks are the recursion and claim kits; kit L AND WRAP holds the bridge layer `L` and the wrap; kits ZERO KNOWLEDGE and COMPACT FRONTEND are not drawn.

### 1.2 Bus tuple schema (halo2 prototype form)

Freeze status: this section is the halo2 prototype form over `Fq`. The frozen form is the BabyBear encoding of section 1.11, FP-1: one lane for each typed field, 16-bit limbs, one LogUp bus index for each chip id. The packing bounds below (for example `8 addr + kind` far below about 2^255, 128-bit ALU operands in one cell) hold over `Fq` only and are not part of the frozen interface.

One schema `(chip, key, a, b, c)` over the proof field `Fq` (`chip/lib.rs`), five cells, two request lanes that look up one provider table (`chip/bus.rs:40-68`). "One bus" means one schema and one provider namespace; the leaf has three lookup arguments: two bus lanes and one tagged fixed table (the spike has four: two bus lanes, DEC, RANGE; L-6 merges the last two).

| chip id | Name | Tuple | Key space | Provider | Requesters | Status |
|---:|---|---|---|---|---|---|
| 0 | disabled | `(0, 0, 0, 0, 0)` only | none | none (any non-provider row evaluates to it) | every unused lane | landed (`chip/bus.rs:15-17`) |
| 1 | ALU | `(1, op + 8 flag, x, y, z)` | `op` in the op list of kit ALU, `flag` in `{0, 1}` | ALU | CORE | landed for ADD, LT (`chip/alu.rs:23-25`); L-7 |
| 2 | MEM | `(2, 8 addr + kind, a, b, c)` | `addr < 2^(TOP+3)`, `kind` in `0..=4` (`KIND_DATA = 1`, `KIND_FIELD = 2`, `KIND_ENV = 3`, `KIND_SLOT = 4`, `chip/membackend.rs:50-58`) | MEM window (`addr >= 8 q_pre`); heap lines of PAGE/ROM (`addr < 8 q_pre`). The scalar `cold_read` is not a bus provider: CORE's read port passes `ColdRead` cells and the backend ties them to `pre.root` by a gated equality (`chip/dense.rs:725-762`) | CORE only (record reads and append requests; VALUE DIGEST receives `AuthRec` cells from CORE's read port) | landed (C1); L-4 |
| 3 | HASH | `(3, BUS_KEY, x, y, H([x, y]))` | one key: the two-input pinned sponge (`chip/poseidon.rs:53,602`) | HASH | CORE (only where the operation is exactly a two-input hash) | landed |
| 4 | ROM | `(4, rom_key, a, b, c)`, `rom_key` holds table id and row | table id and row bits come from the path bits (kit PAGE/ROM) | PAGE/ROM block cache | CORE only (VALUE DIGEST receives `AuthRom` cells from CORE's read port) | new; id proposed here, not in code |
| 5 and above | reserved | unsatisfiable until a versioned schema assigns them | | | | |

Fields and who range-checks them:

| Field | Range | Checked by |
|---|---|---|
| `chip` on a provider row | the provider's fixed non-zero constant | the provider gate (`chip/bus.rs:18-20` rule; mutants `pad_row_on_bus`, `bus_chip_id_unpinned`) |
| `chip` on a request lane | 0, or the id the rule fixes | CORE; an unused lane is zero in all five cells (LC AUDIT item 13) |
| ALU `x`, `y`, `z`, internal witness | below 2^128 | ALU, inside the chip (LC AUDIT item 21). The landed chip leaves the operands to the requester (`chip/alu.rs:8-9`); this changes |
| ALU `op`, `flag` | op list; boolean | ALU |
| MEM `addr` | below `len_cur` at the read point; `len < 2^(TOP+3)` | CORE's read port checks `addr < len_cur` on every read, for every consumer, with `len_cur` the running length of the append chain at that read point (item 11; Quint lane condition C3); MEM decomposes `len_pre`, `len_post` into `TOP + 3` boolean bits (`chip/membackend.rs:329-357`) |
| packed keys | MEM `8 addr + kind < 2^(TOP+6)`; ROM `tbl 2^D_row + row < 2^(D_tbl+D_row)`; ALU `op + 8 flag < 16`. Each bound is far below the field size (about `2^255`), so no key wraps and each packing is injective on its ranged parts | the owner of each part (MEM, the ROM read port and PAGE/ROM, ALU); the field-injectivity bound is a stated lemma of the kit (B2 finding 9) |
| MEM `kind` | `0..=4` on provider rows; the kind the rule expects on a request | MEM (window rows), MEMORY BACKEND (hashed into the record); CORE fixes the kind in the request key |
| MEM `a`, `b`, `c` | field elements; an integer meaning is established by the writer | the appending rule (through ALU or RANGE) at the time of the append; readers inherit it because memory is write-once and authenticated |
| HASH `x`, `y`, output | field elements | none (no integer range) |
| ROM table id, row | table id in the frozen table list; `row < 2^D_row`, a physical row index that includes the sub-row bits of a wide row | the ROM read port of CORE (`read_rom`): the table id is a constant of the call site and the row argument is accepted only as a RANGE handle of width `D_row`. PAGE/ROM bounds only the provider side (`blk_r < 2^D_rom` by its path bits). One owner per side; the request side is never PAGE/ROM (B2 finding 4) |
| ROM `a`, `b`, `c` | as the program normal form defines | trusted only through the PROG canonicality premise (`O/design.md:84`, open) |
| tagged table `tag` | a fixed selector value on every query row (`TAG_RANGE` on a RANGE row, `TAG_DEC` on a DEC row, 0 on any other row); never a witness and never multiplied by a witness | the circuit (L-6). A disabled DEC slot keeps `TAG_DEC` and queries the DEC idle row `(TAG_DEC, 0, 0)` (kit BUS; B2 finding 10) |
| RANGE limb | below 2^12 | the table |
| DEC key parts, control word parts | each part bounded by its width | DEC (LC AUDIT item 20) |

Provider rules (every provider kit must state how it meets each one; they keep D1, D2, D4 and D8 of `O/design.md:94-101`):

1. An active provider row pins a fixed non-zero chip id.
2. A padding provider row is the zero tuple in all five cells, or is off the provider selector.
3. A provider is a function of its identity. The identity is `(op, x, y)` for ALU, `(x, y)` for HASH, the key for MEM and ROM (within the leaf's authenticated pre-state and the program root). The key field alone is not the identity for ALU and HASH (`chip/alu.rs:1-9`).
4. Two provider classes that share a chip id have disjoint key sets by a local rule (MEM window `addr >= 8 q_pre`; heap lines `addr < 8 q_pre`; the scalar cold read is off the bus and is routed by CORE's range-checked `addr < base`). Inside one class, two rows may hold the same key only with the same payload: two heap lines that open the same block are legal and serve identical tuples, because both reach `pre.root` (PAGE/ROM item 8). Disjointness is a rule between classes, not between rows (B2 finding 20).
5. Every provider row satisfies its relation whether or not a request uses it. No provider row has an unconstrained output.
6. Set inclusion does not prove that an append happened once. The append transport is the landed hybrid and no other (B2 finding 11; Quint lane condition C1): (i) the address chain of the appends is copy-linked (`len_pre` -> each append address -> `len_post`, A1 to A3 of kit CORE); (ii) the value of each append is one MEM bus request `(MEM, 8 addr + kind, a, b, c)` with the append's own enable, which the new window row at that address serves (A4). The one-to-one relation between append events and active non-imported window rows follows from (i), (ii) and key injectivity; it is never concluded from the lookup alone. Every append takes one bus request slot and is counted in every capacity figure.
7. No challenge, no second phase, rotations `-1, 0, +1` only, degree at most 6 including the packed selector (`midnight-circuits` 7.3.0 `verifier/verifier_gadget.rs:310,604-611`; `chip/vsel.rs`).

### 1.3 Named copy-constrained ports

| Port | From | To | Cells |
|---|---|---|---|
| `Window` (`chip/membackend.rs:75`) | CORE append chain and MEM | MEMORY BACKEND | window rows `(enable/act, imp, addr, kind, a, b, c)`, `len_pre`, `len_post` |
| append chain | CORE | MEM | per append `(act, addr, addr_next)`; `addr_0 = len_pre`, `addr_next = addr + 1` when `act = 1`, last `addr_next = len_post`; the cells `len_at[i]` (the running length after append `i`) are exported to the read port |
| `AuthRec` (new, B2 finding 4) | CORE read port | CORE rules, VALUE DIGEST, INIT | `(addr, kind, a, b, c)` with the port certificate: `addr` is copy-linked to the requesting register or micro-operation cell, `kind` equals the constant the call site expects, `addr < len_cur` holds for the `len_at` cell of that read point, and the record was served by the window, a heap line or the scalar cold read. The type has no public constructor outside `core/ports.rs` |
| `AuthRom` (new, B2 finding 4) | CORE read port | CORE rules, DEC inputs, VALUE DIGEST | `(tbl, row, a, b, c)` with the port certificate: `tbl` is a call-site constant, `row` is a RANGE handle of width `D_row`, and the tuple was served by a ROM line under `prog_root` |
| `AuthIn` (new; input admission form (b), AD-17) | CORE read port | CORE rules, VALUE DIGEST, INIT | `(idx, tag, ty_d, c, m, vd, child_pos)` of one input record, with the port certificate: the record was opened by a Merkle path of fixed depth `D_view` against the `view_root` body word, and `idx` is 0 (the input root) or the record index of field `k < m` of an earlier `AuthIn` token. 0 MEM appends and no cap charge |
| memory token (`pre`, post: `Vec<Cell>`) | MEMORY BACKEND | BOUNDARY COMMITMENT | dense: `[root, tail_d]` (`chip/dense.rs:637`); `len` is a MEM frontier cell (`Window.len_pre`, `len_post`), not part of the token; the token is opaque to MEM and CORE (`chip/membackend.rs:93-94`) |
| `ColdRead` (`chip/membackend.rs:87`) | CORE read port | MEMORY BACKEND | `enable`, `addr`, `rec[4] = (kind, a, b, c)` |
| state cells | CORE | BOUNDARY COMMITMENT | every persistent register in schema order |
| stack cells | STACKS | CORE, BOUNDARY COMMITMENT, INIT | `frame_count`, `task_count`, `frames_below`, `tasks_below`, cached tops |
| state header cells (new, B2 finding 5) | BOUNDARY COMMITMENT | INIT (in the leaf); the NODE INTEGRATION seal modes (by re-hashing the header of the public lane `post_d`; `L` opens the sealed SET head (section 1.11, FP-3), not the header) | `outcome_tag`, `outcome_payload`, `input_d`, `deps_d`, `view_root`, `body_d` (section 1.11, FP-5) |
| whole-sponge slot | VALUE DIGEST, STACKS, MEMORY BACKEND, BOUNDARY COMMITMENT | HASH | ordered input cells (copied), one digest cell; fixed length and fixed capacity word per call site |
| RANGE port | any module | RANGE | a source cell and a static width; returns the same cell, range-checked |
| DEC port | CORE | DEC | key parts in, bounded control parts out |
| leaf public lanes | BOUNDARY COMMITMENT, CORE | NODE INTEGRATION | `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]` (`crates/mori-recursion/src/ivc.rs:70-74`, where lane 5 is named `rule`). Lane 5 is `rules_d` of SRC-001, computed by CORE (kit CORE, B2 finding 12) |

### 1.4 The two swappable seams

Seam 1, MEMORY BACKEND, is the landed trait `MemBackend` (`chip/membackend.rs:95-128`): `persist` (authenticate the imported tail against `pre`, persist completed segments, return the post token) and `cold_read` (prove that a record is at an address under `pre`), plus one new method for 8-record block lines (LC AUDIT item 24). The token stays opaque.

- Default (Profile A, L-1): dense tail-buffered Merkle tree, `chip/dense.rs`, with heap and ROM cache lines. One pass, no challenge, no committed instance.
- Alternative (Profile B, L-5): ZmLogUp, run-wide write-once LogUp. Benchmark-only, not built before M4. It is not a two-trait swap: it adds a committed instance column, changes the leaf statement, the node, the seal and the continuation module, and replaces the `digest_inj` premise by a probabilistic one. It cannot stop and resume without reloading live memory, so it is not a production candidate under GP-001.

Seam 2, BOUNDARY COMMITMENT, is one trait over the serialized boundary cells.

- Default: Poseidon digest over the registers, the stack hashes and `(len, root, tail_d)` (LC AUDIT item 25), in a header and body form: a capacity-tagged body digest inside the leaf and a short pinned-sponge header that NODE INTEGRATION and L AND WRAP can open (kit BOUNDARY COMMITMENT; B2 finding 5).
- Alternative: Horner fingerprint. Benchmark-only, and offered only together with the ZmLogUp backend (Profile B). A Horner boundary needs the two-pass committed-instance protocol whatever the memory is, so pairing it with the dense backend would bring the whole Profile B protocol for no benefit; that pairing is not offered.

Profile compatibility (the one matrix; B2 finding 20):

| MEMORY BACKEND | BOUNDARY COMMITMENT | Status |
|---|---|---|
| dense Merkle | Poseidon header and body | Profile A, the built path |
| ZmLogUp | Horner | Profile B, benchmark-only, not before M4 |
| dense Merkle | Horner | not offered |
| ZmLogUp | Poseidon | not offered (no memory root to commit; the run-wide sum still needs the two-pass protocol) |

The matrix is enforced when the code is compiled, not by convention (taken from the late text of `gemini-area2.md`, sections 5 and 6.1): the Horner boundary is generic over a backend that implements a sealed marker trait for "commits its words in the committed instance before any challenge" (`PrecommitsInstance0` in that draft), only the ZmLogUp backend implements it, and a compile-fail test (`compile_fail_horner_without_precommit`) shows that Horner with the dense backend does not build.

What a swap may change: the backend's own files, the token width, and (Profile B only) the modules listed in L-5. What a swap must not change: CORE, DEC, ALU, RANGE, HASH, STACKS, VALUE DIGEST, the MEM window and the bus schema. A swap gives a different keyed relation; two profiles never share a verifying key.

### 1.5 Leaf shape parameters

`LeafShape { s, w_flush, k_page, k_rom, n_cold }` is the one definition of the leaf shape in this file (kit CORE owns the type; B2 finding 20). `s`, `w_flush`, `k_page` and `k_rom` are the four fields of LC AUDIT item 28; `n_cold` is the number of scalar cold-read units, which L-2 keeps. `TOP` and the arena count are parameters of the MEM and backend configuration. None of them is a constant of any kit. Working values for the first build: `s = 16` (SRC-014), `w_flush = 1`, one arena (L-9), `TOP = 63` (R-4, OWNER). `w_flush = 1` is derived from the per-rule table (X1, M-3). `k_page`, `k_rom` and `n_cold` are not frozen: X1 gives `k_page + n_cold >= 4`, and `k_rom` waits for the ROM maxima (X15; L-2, section 4).

### 1.6 Layer stack, keys and the shard tree

| # | Layer | Field and hash | Size | Runs on | Hiding |
|---|---|---|---|---|---|
| 0 | Per-program plan (`sherlock plan`), once per program, no witness | n/a | output bound to `prog_d` | user or build host | n/a |
| 1 | Leaf shards: the program's steps, emulated Fr work (tier E), log writes | BabyBear, `EF = BabyBear^4`, Poseidon2 over BabyBear | fixed shapes from the plan's menu | user's GPU | none needed |
| 2 | Nodes: verify children, merge log digests, check leaf keys against `key_root` | same | fixed arity | user's GPU | none needed |
| 3 | `L`: verifies the node root, exposes the statement, the log and the call vector as public values | BabyBear; Merkle tree and transcript in Midnight Poseidon over Fr | one shape, height at most 2^20 | user's GPU | none needed |
| 4 | Wrap: verifies `L` as a private witness, replays the Fr log natively | midnight-proofs PLONK with KZG over BLS12-381, Blake2b transcript | one key, k = 19, 20 or 21 | user's GPU, CPU fallback | yes: the only published proof |
| 5 | Ledger 9 `VerifierKey::verify` | n/a | key at most 50,000 bytes | Midnight | n/a |

Source: `PR` sections 1 and 3.1.

**STARK engine.** OpenVM stark-backend v2.1.0 (`a6b4761`) with its CUDA prover (LogUp-GKR, batched zerocheck, stacked commitment, WHIR) proves layers 1 to 3. Plonky3 0.9.0-rc.2 with Plonky3-recursion is the second choice. The recursion circuit reads the child's constraint DAG as a cached trace, so the internal-recursive key and the root key are the same for every app key: that is the mechanism for per-program leaf chips under one universal key set (`PR` section 2). Production settings: `zerocheck_save_memory = false` per AIR set (the GPU default costs about 34.5 times on W1 at 2^16, MEASURED), and a program's key lists only the AIRs its leaves use (a generic child key with absent preprocessed AIRs gives a node proof the verifier rejects, `SumClaimMismatch`, MEASURED).

**Measured cells** (OpenVM, 20 host threads, `PR` section 2): W1 leaf 2^20 on the GPU 5.829 s, on the CPU 43.959 s; node of arity 2 over 2^20 children 1.809 s, arity 8 over 2^16 children 4.682 s on the GPU; Fr multiply (194 columns) 2^20 on the GPU 9.126 s. A BabyBear trace of height 2^22 (18.378 s on the GPU) is a STARK scaling point, not a circuit k. Per leaf with its node share, a 2^20 leaf under arity-8 nodes costs 6.50 s, ESTIMATE. Node cost is nearly flat in child height, so leaves are tall.

**The tree.** A run is a tree of fixed-size proofs. Leaves of fixed shapes prove the steps; nodes of fixed arity verify their children; one `L` verifies the node root; one wrap verifies `L`. Work grows by adding shards, never by growing a circuit. No key constant bounds run length or Fr work. The one-leaf Compact call takes path C: one leaf, one OpenVM leaf layer at internal parameters, no internal nodes, then `L` and the wrap. Longer runs, GP-2 consumption and GP-4 trees use the full tree.

**Keys.**

- Per program: one app key, one OpenVM leaf-layer key and one internal-for-leaf key per chip set the program uses. Their commitments, and per entry and tree path the Fr commitment `X` of the child DAG that `L` loads, form a Merkle tree with root `key_root` inside `prog_d`.
- Universal: the internal-recursive key, the `L` key and the wrap key, the same for every program. The wrap key, the `L` key and the log capacity do not depend on the plan.

**Binding the plan.**

```text
prog_d = H2(DOM_PROG, image_d, plan_d, key_root, machine_version)      (sherlock-value/2)
C_P    = H(SHK_PROGC, r_P, prog_d)                                      (anchor cell, one r_P per deployment)
```

On the full tree the first universal node checks each child's key commitment by a Poseidon2 inclusion path into `key_root` and forwards the root. On path C, which has no universal node, `L` proves the inclusion of the leaf `P2(DOM_KEYDAG, entry, path, enc(X))` with one more public value `X`, and the wrap checks `Canon` on the limbs of `X` and equates `X` with the cached-trace commitment of `L` that it already reads, about 120 rows, ESTIMATE (the DAG key check, `PR` section 4.3). The wrap sees `key_root` only inside the statement of `L`; OpenVM's plain `app_exe_commit` and `app_vm_commit` never reach the ledger. A public program publishes `(prog_d, r_P)` in its manifest; a private program keeps both secret.

**The shard plan.** `sherlock plan` runs once per program, with no witness, and produces: the straight-line instruction table of each entry as a preprocessed trace; the chip set and leaf shape (chip set, area bucket) of each leaf, with three chip sets for Compact (base: CPU, ALU, range, Poseidon2, SHA2, log writer; base plus FIELD-EMU and POSEIDON-FR-EMU; base plus JUBJUB-EMU); the cut points and tree, padded to a public shard count `N_pub`; the log layout with the reservation `R` first; the placement of discharge shards and of tier E and GP-3 chips; and the per-program keys under `key_root` (`PR` section 4.2).

**What freezes with the universal keys** (`PR` section 4.4): the log operation list, the capacity `(B, V, P, E)` with the encoding of `pv` and its per-operation bounds, the 9 by 30-bit limb encoding, `L_TX` and the public-table layout, the anchor template words with the anchor statement, the hash domains of section 1.8, the per-layer parameter sets and shape bounds of section 1.9, the public value `X` of `L`, and the internal layer's constant fields (AIR count and order, cached and preprocessed layout, interactions per row, public-value layout). They freeze only after the security qualification gate passes. The freeze is in two steps: the interface shapes (bus tuple, ports, public statement, `claim_d`) freeze at loop step S11 in the form of section 1.11, which closes FP-1, FP-2 and FP-4 to FP-7 of section 3.1 and gives the open facts of FP-3; the universal key set freezes only after G1 to G5 pass and the owner confirms OD-1 to OD-4, OD-17 and OD-18 (FP-8).

### 1.7 Rule inventory and machine version

The interpreted rule inventory is the 120 enabled step rules of decision M-4 (X1). It stays for general programs (GP-1). Compact Stage 1 adds the straight-line family `R_SL_*`, about 14 rules (ALU, FIELD, SELECT, RANGE, LT, EQ, DIVMOD2, TAPE, EFF, SPONGE, SHA256, EC, ASSERT, ENTRY): `pc` indexes the per-program instruction table, operands are register records and results are appends, one CPU row plus 2 to 3 interactions per step, 42 to 340 times fewer steps than the interpreted path, ESTIMATE (`PR` sections 6.1 and 7.1; `CO/FRONTEND.md` section 2.4). The arithmetic and hash families of Compact become opcodes of chip AIRs (FIELD-EMU, POSEIDON-FR-EMU, SHA2, JUBJUB-EMU, the log writer) that the CPU row reaches over the bus. Superinstructions may bind all their arguments in one row; a fused rule advances the height by the events it replaces.

On the STARK engine a rule costs 0 to 1 selector column, or an AIR of height 0 when unused, so the rule count is not a cost driver. New rules and chips change the per-program app, leaf-layer and internal-for-leaf keys that `sherlock plan` commits under `prog_d`; they change no universal key. The `R_SL_*` family and the chips define the machine version `sherlock-k-a/2`. `prog_d` binds `machine_version`, so the keys of old programs stay valid when the rule family grows.

### 1.8 Deferred field log, call vector, hash domains

**Log entries.** `e = (op, inputs, outputs)`, each Fr value as 9 limbs of 30 bits with `dec(x) < r`. The frozen operation list is `PERM`, `H2`, `SPONGE(n)`, `PATH(d)`, `ECMUL`, `ECMULG`, `PUB(i)`, `DIGEST`, `NOP`. `FADD`, `FMUL`, `FINV` and `ECADD` are never deferred. The digest is a tagged onion in Poseidon2 over BabyBear carried as `(d, n)` in the boundary state: `d_0 = P2(TAG_EMPTY)`, `d_(i+1) = P2(tagStep, d_i, P2(tagEnt, enc(e_i)))`, so it chains through leaves, continuations, suspension and resume. `L` takes the flattened log and its tree shape as private witness, recomputes `d`, and binds both tables of `pv` to their parts of the public-value vector, each by its own multilinear extension; the Fr transcript of `L` absorbs `pv` before the first challenge. The wrap checks range and `Canon` on every logged value, a bounded-quotient range check on every lazy reduction, a canonical check on every squeeze recomposition, absorbs `pv` natively, evaluates one `eq`, and replays every entry natively (`PR` section 3.2).

**Capacity and encoding (OD-6, OD-17).** Four wrap key constants: `B` entries, `V` logged values, `P` permutation slots, `E` Jubjub slots; the lead decision is `B = V = P = 256`, `E = 0`. `pv` is an entry table of `B` rows `(op, param)` and a value table of `V` rows of 9 limbs, `W_pv = 2B + 9V` = 2,816 words. Entry `i` owns the value rows from the running sum `off_i`; `L` and the wrap check the running sums, so offsets are not words of `pv`. Unused entry rows are `NOP` with `param` 0, no used entry follows a `NOP`, unused value rows are zero, and the wrap checks both, so the used counts stay private and the encoding of a log is unique.

| `op` (code) | `param` | Values | `P` slots | `E` slots | Bound |
|---|---|---:|---:|---:|---|
| `NOP` (0) | 0 | 0 | 0 | 0 | padding only |
| `PERM` (1) | 0 | 6 | 1 | 0 | |
| `H2` (2) | 0 | 3 | 1 | 0 | |
| `SPONGE(n)` (3) | `n` | `n + 1` | `ceil(n/2)` | 0 | `1 <= n <= V - 1` |
| `PATH(d)` (4) | `d` | 3 | `d` | 0 | `1 <= d <= 32` |
| `ECMUL` (5) | 0 | 5 | 0 | 1 | none admitted while `E = 0` |
| `ECMULG` (6) | 0 | 3 | 0 | 1 | none admitted while `E = 0` |
| `PUB(i)` (7) | `i` | 1 | 0 | 0 | `0 <= i < 2 + L_TX` |
| `DIGEST` (8) | tag index | 2 | 1 | 0 | tag from the frozen list below |

**Reservation and the capacity rule.** The reservation `R` is the anchor reservation `R_A` (1 entry, 2 values, 1 permutation for an ordinary call; 2 entries, 4 values, 2 permutations for a chunk) plus the public-digest `DIGEST` entries of the run's statement. `R` depends only on the mode and the statement, so it is static and the same in the planner, in `L` and in the wrap. A run's deferred set is `R` plus a discretionary set `S`, admissible only if all four limits hold together for `R` and `S`. The planner places `R` first and adds a discretionary operation only if every limit and its per-operation bound still hold; otherwise the operation goes to tier E. Keygen refuses a capacity that does not hold the largest `R` (4 entries, 8 values, 4 permutations). A discretionary operation over a bound is proved inline by tier E and never refused, so GP-1 keeps no bound on Fr work. The largest checked entry, `zerocash spend` at 111 values and 39 permutations with the mandatory entries counted, fits with margins of 2.31 times on values and 6.56 times on permutations, DERIVED; as a chunk, 113 and 40, 2.27 and 6.40 times. When the slots that `R` leaves are full, every further discretionary Fr operation is proved inline by tier E chips; a claim that another run consumes is closed by a discharge shard, a fixed leaf shape. Lean proves that any split of the discretionary operations gives the same claim (`spill_equiv`, `discharge_sound`).

**Call vector (OD-7, OD-15).** One fixed layout of `2 + L_TX` words for every program, Compact and general runs alike, `L_TX` = 1,024:

| Lanes | Content |
|---|---|
| 0 | `binding_input` (E-1, placement V) |
| 1 | `comm` |
| `2 .. 2+A` | anchor prefix: uncached `idx` of cell `(0)`, `popeq a_in` |
| `2+A .. 2+A+L_e` | effect words |
| rest | constrained zero |

A general run is a Sherlock program whose effect words write its run statement `[C_P, input_pub, outcome_pub]` (section 1.11, FP-3 and FP-6), so the general path and the Compact path share the one wrap key.

**Hash domains (OD-18).** Fr side, Midnight Poseidon permutation, width 3, rate 2: the initial capacity is `cap(u, l) = l + u 2^66` for a fixed length `l` and `2^64 + u 2^66` for the transcript. Use codes: `u` = 0 program operations (`PERM`, `H2`, `SPONGE(n)`, `PATH(d)`, unchanged native Midnight definitions); `u` = 1 Merkle leaf of `L` (BabyBear words packed 8 per Fr element); `u` = 2 Merkle node of `L`; `u` = 3 the Fr transcript of `L`; `u` = 4 public digests `(tag, pack(d2))`. Public digest tags: `SHK_PROGC` (`C_P`), `SHK_LOCK` (lock values `A_j`), `SHK_INPUT` (`input_d`), `SHK_OUTCOME` (`outcome_d`). The fixed-length hashes use Midnight's fixed-length sponge, which replaces the stock template's leaf hasher and truncated-permutation compression for `u` = 1 and 2. The transcript of `L` is the stark-backend multi-field overwrite duplex over Fr with Midnight's permutation, stock in every transition, with one change: the initial capacity cell `s_2 = 2^64 + 3 * 2^66`. The key schedule `sched` fixes the count of words between any two flushes, and the wrap's verifier of `L` follows `sched`, not the proof. The wrap implements the duplex on its 19-row permutation layout and exposes the bare permutation. BabyBear side: every Sherlock-defined Poseidon2 hash absorbs its tag word first, with the total length in the first capacity word; tags `TAG_EMPTY`, `tagStep`, `tagEnt`, `TAG_NODE` (log onion and tree), `DOM_PROG`, `DOM_PROGC`, `DOM_LOCK`, `DOM_KEYTRIPLE`, `DOM_KEYDAG`, `DOM_KEYNODE`, `DOM_LAYOUT`; each new digest takes a new tag. The leaf, leaf-layer and internal layers keep the stock OpenVM hashes and challenger (premise A4). Full tables: `PR` section 3.6.

**GP-1 to GP-4.** GP-1: programs and inputs are data, fixed leaf shapes and a fixed wrap, the spill rule removes every bound on Fr work, `(d, n)` in the boundary state keeps stop and resume. GP-2: a claim from the same machine joins the log tree by `TAG_NODE` or closes in a discharge shard; a claim from another machine arrives as that run's wrap proof and is verified in a GP-3 shard. GP-3: STARK shards with OpenVM's algebra and pairing extensions; the wrap holds no pairing and no foreign verifier. GP-4: nodes merge tagged digests; a claim consumed twice is replayed twice; nodes never reject fanout.

### 1.9 Security statement and the qualification gate

**Target.** Every layer on the chain of a call (leaf, OpenVM leaf layer, internal layers, `L`, wrap) has at least 100 bits of round-by-round soundness proven from theorems as stated, with no unproved conjecture. **Status: not qualified.** The target is a gate, not a result. Every bit count below is a calculator figure (MEASURED by D3, `FD/EXPERIMENTS.md`, or ESTIMATE), not a proven bound, and no universal key freezes before the gate passes.

**Parameter sets** (LEAD, OD-1 to OD-4; `PR` section 11.2). Each layer has one parameter set and one set of shape bounds, frozen with the universal keys; keygen refuses a key outside its bounds and a key whose per-round query vector differs from the frozen one.

| Layer | Log blowup, `l_skip`, `n_stack`, `w_stack` | Proximity | PoW: folding, `mu`, query | OOD per round | Queries per round | Shape bounds |
|---|---|---|---|---|---|---|
| App (leaf shards) | 1, 4, at most 20, 2,048 | unique decoding | 5, 15, 21 | 1 | 193/88/81/81 | 5,000 constraints and 1,000 interactions per AIR, 100 AIRs, 30,000 columns |
| OpenVM leaf layer | 2, 4, 17, 2,048 | unique decoding | 4, 13, 21 | 1 | 118/84/81 | height at most 2^21; 1,000 constraints and 100 interactions per AIR, 50 AIRs, 2,000 columns |
| Internal | 3, 2, 17, 512 | list decoding at radius (4/3) sqrt(rho), m = 3 | 19, 22, 21; LogUp 20 (the code rule gives 19) | 1 | 75/31/20 | height at most 2^19; same per-AIR bounds |
| `L` | 4, 2, 18, at most 48 | list decoding at radius (4/3) sqrt(rho), m = 3 | per fold: round 0: 15, 14, 13, 12; round 1: 19, 18, 17, 16; round 2: 22, 21, 20, 19; `mu` 22; query 20; LogUp 21 | 2 | 52/27/18 | height at most 2^20; same per-AIR bounds |

`L` also fixes: base field BabyBear `p = 2^31 - 2^27 + 1`, `EF` of degree 4; leaf and transcript hash Midnight Poseidon over Fr, 8 full and 60 partial rounds, `x^5`; packing 8 BabyBear elements per Fr element; 5 base-`p` digits per squeeze with bias at most 2^-100; WHIR with `k_whir` = 4, 3 rounds, final polynomial 2^8. D3 prints its weakest term at 101.1727 bits (folding, round 2) at `w_stack` = 16 and 101.0869 bits at 18, MEASURED; LogUp PoW 21 is required, because the code rule's 20 gives 100.9126 bits; the OOD term is 205.6404 bits with two samples and 98.0129 with one. Only the `mu` term moves with `w_stack`: with `mu` PoW 20 the weakest term is 100.6508, 100.2202, 100.0451 and 99.6198 bits at `w_stack` 24, 32, 36 and 48, MEASURED (D3 section 8, `research/sherlock-processor-final-20261111/D3-SOUNDNESS.md` section 8 on branch `d3-soundness`, commit `81abb8f7a`). The keyed structure of `L` points to `w_stack` about 36. The `L` key bound is `w_stack` at most 48 with `mu` PoW 22, weakest term 101.1727 bits at 48, MEASURED (LEAD, OD-3, owner to confirm). `mu` PoW 21 holds only to `w_stack` 37, DERIVED, too close to the measured proxy. The internal set keeps its own bound 512. D10 (the trace heights of `L`) is the measurement that may later tighten the bound. The keyed structure of `L` is 39 AIRs, 1,537 trace columns, 487 interactions and 2,210 constraints, MEASURED (`research/sherlock-stark-framework-20261111/L-STRUCTURE.md` on branch `l-structure`, commit `3957ddf7c`), inside the shape bounds; its trace heights are not measured. Two OOD samples per round follow one transcript order: observe the round commitment; sample `z_(i,1)` then `z_(i,2)`; observe `y_(i,1)` then `y_(i,2)`; the query PoW, query indices and `gamma` as stock; the batched claim `y_(i,1) gamma + y_(i,2) gamma^2 + sum_j v_(i,j) gamma^(j+3)`. The OD-2 per-fold vector differs from the owner's earlier wording of 21 bits on every fold: one fold needs 22 bits.

**Margin rule (OD-4).** Every term of every layer is at least 100 bits. Grindable terms (folding, `mu`, query, LogUp) take at least 1 bit of margin, set by D3 through their PoW constants. Terms that no grinding moves need 100 bits and no margin. D3 prints the internal OOD term at 100.0129 bits with one sample, MEASURED: it meets the rule with 0.013 bits of slack. Two internal OOD samples per round (OD-4 (b)) give that term 208.6404 bits if more slack is wanted; they change the recursion verifier AIRs and the internal key and stay one universal key set. D3 prints the app and leaf-aggregation sets at 101.0891 and 101.0110 bits with query PoW 21, and the internal set at 100.0129 bits with LogUp PoW 20; at the code rule's internal LogUp PoW 19 the LogUp term is 100.4129 bits, below the margin.

**Premises** of the bridge theorem `bridge_sound` (`PR` section 11.3): A1 `wrapKnowledgeSound`, A2 `lastLayerSound`, A2b `frSpongeInj`, A3 `rootBinds`, A4 `recSound`, A5 `p2Inj`, A6 `oracleElim`, A7 `wrapZK`, and the static check `vkL_ok`. `bridge_sound` and `wrap_view_only_claim` build at commit `bdb92973a` of branch `bridge-lean` (BUILD PASS, 112 theorems, 0 sorry, the standard axioms only; lemma 23 proved; the transcript of `L` is a program in the key, so `pvFirstT` and the PoW part of `ProvenRegime` are derived from it), but take the soundness of `L` and of the recursion as premises and check an older parameter gate (`ProvenRegime` still requires 21-bit folding and `mu` PoW), so the build does not establish the bit count of the recommended set. The restatement after OD-1 to OD-3 gives `ProvenRegime` the per-fold PoW vector, separate radius and bound multiplicities, two OOD samples and the `w_stack` bound; proves the two-OOD bound against the transcript order above; follows the two-table encoding of `pv`; adds the DAG key check and the anchor statement checks to `wrap_refines`; and names the two Fr sponge algorithms on the one permutation.

**Security qualification gate.** Status: not passed. D3 passes the calculator part of G2 for the sets above, with internal LogUp PoW 20; G1, the rest of G2, G3, G4 and G5 are open. The design claims proven 100-bit security only when all five checks pass; a failed check blocks the claim, whichever it is.

| # | Check | Pass condition |
|---|---|---|
| G1 | Parameters frozen | the owner confirms OD-1 to OD-4, OD-17 and OD-18; every set and bound above, with the explicit per-round query vectors and query PoW 21 for the app, leaf-aggregation and internal sets, is fixed in the key-generation code, and keygen refuses a key whose query vector differs |
| G2 | D3 on the frozen sets | the stock calculator, after build step 4, prints every term of every layer at its shape bounds at 100 bits or more and every grindable term at 101 or more, on the frozen query vectors; for `L` at its key bound `w_stack` 48 with `mu` PoW 22, or at a tighter bound that D10 sets. Calculator part: passed for the sets above with internal LogUp PoW 20, `L` at `w_stack` 16, 18, 24, 32, 36 and 48 (at 48 with `mu` PoW 22). Open: the frozen sets of G1, a tighter `w_stack` from D10, and the calculator change in the prover and verifier |
| G3 | Two-OOD argument and transcript | the prover, the stark-backend verifier, the calculator and the wrap's verifier of `L` follow the two-OOD order and the transcript algorithm of section 1.8; the transcript vectors T-FS give equal outputs on the CPU prover, the GPU prover, the stark-backend verifier and the wrap's verifier of `L`; and the Lean proof of the two-OOD bound builds |
| G4 | Lean | the restated theorem is committed with a build log: 0 sorry, standard axioms only, and the concrete `L` key passes `ProvenRegime` by `decide` |
| G5 | Theorem-to-configuration mapping | WHIR (ACFY24, ePrint 2024/1586) and the mutual correlated agreement theorem that SWIRL cites (BCGM26 Theorem 9.2) are fetched and read in full; a written mapping, one row per hypothesis of WHIR Theorem 5.2 and of BCGM26 Theorem 9.2, shows each hypothesis holds for each layer's frozen set, the two-OOD order and the `gamma` batching of `q_i + 2` claims included. A hypothesis the protocol does not meet is closed by the Lean two-OOD proof or by a parameter change, never by a conjecture; the conjecture-based capacity regime of the WHIR reference code is excluded |

### 1.10 Midnight side

The ledger checks each `ContractCall` with `midnight_transient_crypto::proofs::VerifierKey::verify` on the key installed under the entry label. The wrap is a midnight-proofs PLONK circuit with KZG over BLS12-381 and the Blake2b transcript, the same proof system as today's k = 21 decider, so ledger 9.1.0.1 needs no change. Every exported Sherlock entry holds the same universal wrap key. Limits: verifier key at most 50,000 bytes (today's decider key 2,217 bytes, MEASURED); at most 65,536 public inputs per call (the wrap has 1,026); `VERIFIER_MAX_DEGREE` 14 bounds the public inputs, not the circuit k; transaction at most 1,048,576 bytes. The path through the unchanged ledger is MEASURED for today's k = 21 decider: a local devnet call (block 237, submit to inclusion 3.79 s) and a Preview deploy and call at blocks 71100 and 71431, with refusals for a changed result, a flipped proof bit and a proof for another program digest (`PR` section 9). The Preview node reports build `2.1.101-bbd616b1`, and the Preview pin records the same build. The same local ledger accepts the cross-entry attack E-1: a decider proof of one entry's transcript, in a call labelled with a second entry that holds the same key, is included at block 1,085 with the first entry's write, MEASURED (`PR` section 7.2). The wrap's verifier-key bytes, proof bytes (8 to 9 kB, ESTIMATE), verify time and fee at `2 + L_TX` public inputs are PENDING.

**Call time** (`PR` section 6.3, all ESTIMATE). The prove stages of one warm Compact call on the full GPU path take 9.45 s (6.74 to 18.34 s); the whole warm call with witness fill, transaction prebuild, the balance and DUST proof and submit to inclusion is 19.6 s (11.5 to 47.2 s), with the wrap at k = 19, a GPU Fr Poseidon kernel for `L` and a rewritten MSM kernel, neither of which has a prototype. The wrap part of paths 2 and 3 is ESTIMATE conditional on the fused quotient kernel and the MSM rewrite; the MEASURED baseline with today's kernels is the GPU wrap table of kit L AND WRAP (155.2 s on the GPU against 441.7 s on the CPU at k = 21 under load). The CPU-only path is 174.7 s mid (155.9 to 207.9 s) with the measured CPU `L` rate under load (D1). Below about 20 s the floor is fixed per-call work: the balance and DUST proof (3.05 s, DERIVED), submit to inclusion (3.92 s mid, 3.79 s MEASURED), trace generation (1.95 s) and the BabyBear part of `L` (1.81 s).

### 1.11 Interface freeze over BabyBear (FP-1 to FP-7)

This section gives the BabyBear form of the interfaces that freeze at loop step S11: the bus tuple and the ports (FP-1), the node statement envelope (FP-2), the public statement codec (FP-3), the one digest table (FP-4), the seal read of `view_root` (FP-5), the public `n_steps` rule (FP-6) and the chunk and E-1 bindings (FP-7). Where it differs from the halo2 prototype text of sections 1.2 and 1.3 and of the kits of section 2, this section applies. A rule that the sources leave open is marked LEAD and is one of the owner items F-1 to F-13 of `FD/DECISIONS.md`. A Lean name marked "open" is proposed here and is not proved. Hash outputs are not written here; each digest is pinned by its preimage word list and a named differential vector.

#### FP-1 Bus tuple and ports

**Word types** (LEAD F-1). Every lane of a bus message, a port message, a public value and a digest preimage has one type of the table below. A word is an element of BabyBear, `p = 2^31 - 2^27 + 1`, in its canonical representative in `[0, p)`. A multi-limb integer is little-endian: limb 0 is the least significant.

| Type | Words | Encoding | Range check and owner |
|---|---:|---|---|
| `bit` | 1 | 0 or 1 | the writing AIR, `b (b - 1) = 0` |
| `tag(w)`, `w <= 16` (op, flag, kind, table id, form, outcome kind, `stmt_tag`, mode) | 1 | the integer | the writer, by RANGE `(x, w)` or by a gate |
| `u16` | 1 | the integer | RANGE `(x, 16)` |
| `u32` | 2 | 16-bit limbs | RANGE on each limb |
| `u64` | 4 | 16-bit limbs | RANGE on each limb |
| `addr`, below `2^(TOP+3)` | `n_a = ceil((TOP + 3) / 16)`, 5 at TOP = 63 | 16-bit limbs; the top limb is below `2^(TOP + 3 - 16 (n_a - 1))`, 4 at TOP = 63 | MEM on the frontier; CORE's read port for `addr < len_cur` |
| `u128` | 8 | 16-bit limbs | ALU, on operands and results |
| `digest` | 8 | one Poseidon2 output over BabyBear (`DIGEST_SIZE` = 8, `SB/crates/stark-sdk/src/config/baby_bear_poseidon2.rs:24-25`) | none: every word is canonical by representation |
| `fr` | 9 | 30-bit limbs with `Canon` (section 1.8) | the leaf at the log write; the wrap |

Every 16-bit limb is below `2^16 < p`, so no limb wraps. No word holds two parts: the packed keys of section 1.2 (`8 addr + kind`, `tbl 2^D_row + row`, `op + 8 flag`) and the 128-bit operand in one cell are halo2 prototype forms only, and over BabyBear each part is its own lane.

**Bus registry** (LEAD F-2). Each chip id of section 1.2 is one LogUp bus index of stark-backend (`BusIndex = u16`, `SB/crates/stark-backend/src/interaction/mod.rs:13`). The bus index is part of the LogUp message, so tuples on two buses never alias, and the `chip` cell of the prototype tuple is not a lane. A disabled request has count 0, which replaces the zero tuple of chip id 0. The index of chip id `c` is `BUS_BASE + c`. `BUS_BASE` is a constant of the chip-set configuration above every bus index that a stock OpenVM chip of the same chip set uses (the `sha2` chip, the Poseidon2 chip). Keygen refuses a chip set in which one bus index carries two message layouts.

| id | Bus | Message, in order | Words at TOP = 63 | Provider | Requesters |
|---:|---|---|---:|---|---|
| 1 | ALU | `op: tag(4)`, `flag: bit`, `x: u128`, `y: u128`, `z: u128` | 26 | ALU | CORE |
| 2 | MEM | `kind: tag(3)` with `kind` in `1..=4`, `addr: addr`, `rec`: the `W_rec` typed lanes of the record layout of the kind (AD-11) | `6 + W_rec` | MEM window for `addr >= q_pre`; heap lines of PAGE/ROM for `addr < q_pre` | CORE |
| 3 | HASH | `tag: u16`, `x: digest`, `y: digest`, `h: digest` with `h = P2(tag, x, y)` | 25 | HASH | CORE, where the operation is exactly one two-input hash |
| 4 | ROM | `tbl: tag(D_tbl)`, `row: u32`, `rec`: the `W_rom` typed lanes of the row layout of `tbl` | `3 + W_rom` | PAGE/ROM block cache | CORE |
| 5 | RANGE | `x`, `w: tag(5)` with `0 <= w <= 16` | 2 | the RANGE table: one preprocessed row `(x, w)` for each `w` in `0..=16` and `x < 2^w`, `2^17 - 1` rows | every kit |
| 6 | DEC | `dec_key`, `dec_val`, in `u16` parts (AD-9) | per AD-9 | the DEC table with the idle row `(0, 0)` | CORE, one query per step slot |
| 7 to 15 | reserved | none | | no AIR | a request is unsatisfiable (BUS-001) |

The record and ROM-row layouts (`W_rec`, `W_rom` and the type of each lane) are the separate prerequisite AD-11 of section 3.1; every layout keeps `1 + message words <= 128`. The tagged table of L-6 is two bus indices over BabyBear: the tag is the bus index, a constant of the interaction, never a witness. Every DEC slot, enabled or not, queries the DEC bus with count 1 from a fixed selector, and a disabled slot queries the idle row, so a full leaf and a partial leaf prove under one key (kit BUS).

**Ports** (LEAD F-2). A port of section 1.3 whose producer and consumer are columns of one AIR is a set of shared columns of one row, with no interaction. A port between two AIRs is one stark-backend `PermutationCheckBus` of index `BUS_BASE + 16 + j`, with `j` the position of the port in the ports manifest (`chip/ports.rs`). The producer sends `(seq, cells)` once with count `act`; the consumer receives the same message once with count `act`; `seq: u32` is the leaf-local event index of the instance. The LogUp multiset equality gives one consumer instance for each producer instance. A port certificate (section 1.1) is the producer's relation over the message cells, and the consumer uses it through this equality in place of the halo2 copy equalities.

**Leaf public values** (the lanes of `sherlock-leaf-stmt/1` over BabyBear, then the log commitment of FP-2): `binding: digest` at word 0, `prog_d: digest` at 8, `pre_d: digest` at 16, `post_d: digest` at 24, `rules_d: digest` at 32, `h_pre: u64` at 40, `steps: u16` at 44 with `1 <= steps <= S_MAX < 2^16`, `log_d: digest` at 45, `log_n: u16` at 53. Total 54 words.

**LogUp bounds, in place of BUS-007.** The bus keeps membership semantics. Each bus of ids 1 to 6 is a stark-backend `LookupBus`: a request uses `lookup_key` with a boolean count and `count_weight` 1, and a provider row uses `add_key_with_lookups` with a witness multiplicity and `count_weight` 0 (`SB/crates/stark-backend/src/interaction/mod.rs:91-125`). The enabled request multiset is then a subset of the provider keys, if these bounds hold:

1. Every message with its bus index has at most `2^7 = 128` words (`log_max_message_length = 7`, `SB/crates/stark-sdk/src/config/log_up_params.rs:25`); keygen refuses a longer interaction (`SB/crates/stark-backend/src/keygen/mod.rs:141-152`).
2. For each bus index, the sum over the AIRs of (the `count_weight` sum of the AIR's interactions on that bus) times (the AIR's height) is below `p`. Keygen writes this as a trace-height constraint, and the verifier checks it (`SB/crates/stark-backend/src/keygen/mod.rs:132-186`).
3. The sum over the AIRs of (the AIR's interaction count) times (its height) is below `max_interaction_count = p` (`SB/crates/stark-sdk/src/config/log_up_params.rs:24`).
4. The AIR stays inside the app shape bounds of section 1.9 (1,000 interactions per AIR, 100 AIRs). The plan chooses leaf shapes that meet bounds 2 and 3; keygen and the verifier refuse every other shape.
5. A request count is a boolean that the requester constrains. A provider multiplicity is a witness. Unused provider rows and repeated requests are legal (section 1.1).

The append transport of provider rule 6 does not change: the copy-linked address chain is the append-chain port, and each append is one MEM request. Provider rule 7 is a halo2 rule; over stark-backend an AIR reads its local row and its next row only, stays inside the shape bounds of its layer, and draws no challenge of its own (the LogUp challenges are those of stark-backend).

**AIR interfaces, in place of the halo2 `Layouter<F>` signatures.** The kit skeletons of section 2 (`configure`, `assign`, `Region`, `Layouter`) are the halo2 prototype. Each kit spec maps them onto the two traits below.

```rust
pub struct BusRegistry {
    pub alu: LookupBus, pub mem: LookupBus, pub hash: LookupBus, pub rom: LookupBus,
    pub range: LookupBus, pub dec: LookupBus, pub ports: Vec<PermutationCheckBus>,
}
/// One kit AIR. It owns its columns and talks only through the buses of `BusRegistry`.
/// Every kit also implements `Air<AB>` for `AB: InteractionBuilder`.
pub trait KitAir<F: Field>: BaseAir<F> + BaseAirWithPublicValues<F> + PartitionedBaseAir<F> {
    fn buses(&self) -> &BusRegistry;
}
/// Requires: `enable` is boolean (the caller constrains it). Ensures: if enable = 1, msg is a provider key of `bus`.
pub fn request<AB: InteractionBuilder>(bus: LookupBus, b: &mut AB, msg: impl IntoIterator<Item = AB::Expr>, enable: AB::Expr);
/// Ensures: msg is offered `mult` times; the provider relation holds on the row whether or not mult = 0.
pub fn provide<AB: InteractionBuilder>(bus: LookupBus, b: &mut AB, msg: impl IntoIterator<Item = AB::Expr>, mult: AB::Var);
/// Requires: `act` is boolean. Ensures: exactly one `port_recv` with the same (seq, cells) when act = 1.
pub fn port_send<AB: InteractionBuilder>(port: PermutationCheckBus, b: &mut AB, seq: [AB::Expr; 2], cells: impl IntoIterator<Item = AB::Expr>, act: AB::Expr);
pub fn port_recv<AB: InteractionBuilder>(port: PermutationCheckBus, b: &mut AB, seq: [AB::Expr; 2], cells: impl IntoIterator<Item = AB::Expr>, act: AB::Expr);
/// Witness generation: one trace for each AIR from the leaf's event records. It holds no bus logic.
pub trait KitTrace<F: Field> { type Records; fn generate_trace(&self, r: &Self::Records) -> RowMajorMatrix<F>; }
```

**Invariant and Lean.** The encoding is injective on each bus: two messages that are equal on one bus have equal typed fields. Lemma `bus_enc_inj` (new `lean/BusEncoding.lean`; open): for one message layout (a list of the types above) whose limbs are in range, the map from typed fields to words is injective. The proof reduces each multi-limb field to `horner_inj` (`FrLog.lean:93` at `bdb92973a`, proved) with base `2^16` and the bound `2^16 < pBB`. The LogUp soundness of the bus under bounds 1 to 3 is part of premise A4 (section 1.9), not a Sherlock lemma.

**Checks.** Vector `bus_enc_vectors`: `u64` `2^64 - 1` gives `[65535, 65535, 65535, 65535]`; `u64` `0x0001000200030004` gives `[4, 3, 2, 1]`; `addr` `2^65 + 1` at TOP = 63 gives `[1, 0, 0, 0, 2]`; `u128` `2^127` gives `[0, 0, 0, 0, 0, 0, 0, 32768]`. Mutants: `bus_limb_over_range` (a limb equal to `2^16`), `bus_addr_top_limb_over` (top limb 4 at TOP = 63), `bus_key_packed_alias` (control: an AIR with the packed key `8 addr + kind` in one word accepts two `(addr, kind)` pairs with one key word), `bus_port_received_twice` (a second `port_recv` of one producer instance). Keygen and verifier refusals: `bus_message_too_long` (128 words and the bus index), `bus_height_constraint_over_p` (a proof whose heights break bound 2).

#### FP-2 Node statement envelope

**Layout** (LEAD F-3). The public values of every node proof, in order (74 words):

| Word | Field | Type |
|---:|---|---|
| 0 | `stmt_tag`: SEG 1, SET 2, SUB 3, DL 4, FV 5, AW 6, VW 7, LG 8 | `tag(4)` |
| 1 | payload, 32 words, unused words 0 | see below |
| 33 | `key_root` | `digest` |
| 41 | `reg_root` | `digest` |
| 49 | `shape_d` | `digest` |
| 57 | `log_d` | `digest` |
| 65 | `log_n` | `u16` |
| 66 | `keys_d`: the commitment `P2(DOM_UKEYS, c_node, c_foreign)` of the universal node and foreign-lift keys; it replaces `keys_repr` | `digest` |

Payloads (word offsets inside the payload): SEG `prog_d` 0, `pre_d` 8, `post_d` 16, `h_lo: u64` 24, `h_hi: u64` 28; SET `set_root` 0; SUB `set_root` 0, `sub_root` 8; DL `chain_d` 0, `cover_root` 8, `in_link` 16, `prem_root` 24; FV `fv_state_d` 0; AW `adm_state_d` 0; VW `view_state_d` 0; LG `dis_d` 0, `dis_n: u16` 8. A LEAF child counts as the SEG `(prog_d, pre_d, post_d, h_pre, h_pre + steps)` with its leaf log `(log_d, log_n)`.

**Log update rule, every mode.** Let `D_EMPTY = P2(TAG_EMPTY)` (section 1.8). The merge of two logs is `merge((d_a, n_a), (d_b, n_b)) = (d_b, n_b)` if `n_a = 0`, `(d_a, n_a)` if `n_b = 0`, and `(P2(TAG_NODE, d_a, d_b), n_a + n_b)` otherwise; the node range-checks the output `n` below `2^16`. A statement with `n = 0` has `d = D_EMPTY`: a leaf with no entry has the empty onion, and the merge keeps it. So the tree shape that `L` takes as witness has at most `log_n` segments, and an empty shard adds no work to `L`. The output log of a node is `merge(merge(c_0, c_1), w)`, with `c_j` the log of slot `j` (`(D_EMPTY, 0)` for a disabled slot) and `w` the log of the inline work (`(D_EMPTY, 0)` with no inline work).

| Mode | Output log |
|---|---|
| MERGE, MERGE-WORK | the rule above; MERGE-WORK adds the onion of its inline segment as `w` |
| SEAL, SEAL-WITH-DEPS | `merge(c_0, c_1)`: the SET carries the run's log, and the premise SET adds its own log (empty for `ADM` and `VIEW`) |
| JOIN, DL-STEP, DESCEND | the merge of the enabled children; DESCEND keeps the log of its SET child, so a claim consumed twice merges its log twice (GP-4) |
| FV-WORK, ADMIT-WORK, VIEW-WORK | `c_0`: an administrative unit writes no log entry |
| DISCHARGE (new mode, LEAD F-4) | `(D_EMPTY, 0)`, after the check below |

DISCHARGE: slot 0 holds any statement `S`, and slot 1 a LEAF child of the discharge shape with `stmt_tag` LG. The node requires `dis_d = S.log_d` and `dis_n = S.log_n`, and outputs `S` with the log `(D_EMPTY, 0)`. The discharge shape is a fixed leaf shape of the plan (`PR` section 4.2 item 5): it takes the entries and the tree shape as witness, recomputes `(dis_d, dis_n)` and proves `Valid(e)` for each entry with tier E chips (`discharge_sound`, `FrLog.lean:1125`, proved). The node relation table gains the row DISCHARGE (slot 0 any, slot 1 LG, no inline work, output the type of slot 0).

**Key rule** (LEAD F-5). For a LEAF child, the node opens `prog_d = P2(DOM_PROG, image_d, plan_d, key_root, machine_version)` from witnessed words (one hash), requires the opened `key_root` to equal the output `key_root`, and proves an inclusion path of fixed depth `D_key = 16` from the child's key leaf `P2(DOM_KEYTRIPLE, set_id: u16, c_app, c_leaf, c_ifl)` to that root, with internal nodes `P2(DOM_KEYNODE, l, r)`. An enabled NODE child with a SEG statement has the output `key_root`. A SET, SUB, DL, FV, AW, VW or LG output takes the `key_root` of slot 0; a sealed claim binds `prog_d`, and so `key_root`, through `claim_d`, and the forwarded root serves only the DAG key check of `L`. With no enabled child, the node takes `key_root` from the `prog_d` opening of its inline work, or the zero digest when that work has no `prog_d` (VIEW-WORK, FV-WORK). Every enabled NODE child has the output `keys_d`. `reg_root` keeps the rule of kit NODE INTEGRATION.

**Consumed by `L`.** `L` verifies the root proof under the internal key, requires `keys_d` to equal its key constant, requires `log_n <= B`, recomputes `log_d` from the flattened log and the tree shape (private witness), binds the log to `pv` (section 1.8), and proves the DAG key leaf `P2(DOM_KEYDAG, entry: u32, path: tag(2), enc(X): fr)` in the root `key_root`. On path C, `L` opens `prog_d` to `key_root` itself from the leaf-layer statement.

**Lean.** `node_log_merge` (`BinaryNode.lean`; open): the flattened output log of every mode is the left-to-right concatenation of the flattened logs of the enabled children and of the inline work, and `digest_inj` (`FrLog.lean:1056`, proved) with `onion_snoc` (`FrLog.lean:977`, proved) gives one flattened log per output `log_d`. `node_key_route` (`BinaryNode.lean`; open; the STARK form of NODE-L3): every enabled LEAF child's key leaf is in the `key_root` opened from its `prog_d`, and every enabled NODE child has the parent's `keys_d`.

**Checks.** Positives `pos_node_log_unary_passthrough` (the vector `merge((D_EMPTY, 0), (d, n)) = (d, n)`), `pos_node_log_two_children`, `pos_node_discharge`. Mutants `node_log_child_dropped`, `node_log_slots_swapped`, `node_log_count_over` (`n = 2^16`), `node_log_empty_not_neutral` (an empty child that changes `d`), `node_key_root_other_prog`, `node_leaf_key_outside_root`, `node_keys_d_other`, `node_discharge_other_log`.

#### FP-3 Public statement codec

- **The layer that opens the SET head is `L`** (LEAD F-6, decision C-2). The SET forms are Poseidon2 hashes over BabyBear, and `L` is a BabyBear STARK; the wrap has no native Poseidon2 over BabyBear and would pay 2,000 to 4,000 rows for each value (`B/CANDIDATE.md:252`). For a run root, `L` requires `stmt_tag` SET, opens `set_root` with one hash as `LEAF(claim_d, n_steps)` or `DEP(claim_d, n_steps, cover_root, in_link)` (every other form fails the equality under premise A5), recomputes `claim_d` from its witnessed preimage `(prog_d, input_d, outcome_d)` and checks no seal predicate (LW-008).
- **Public form of the program identity: `C_P`** (LEAD F-7, owner item N-11). Plain `prog_d` is never call data (ZK-002).
- **Public run statement:** `[C_P, input_pub, outcome_pub]`, three Fr words, each the output of a mandatory `DIGEST` entry of the reservation `R` (section 1.8), with the forms of FP-4. `n_steps` is not public (FP-6).
- **Discriminator** (LEAD F-8): the entry label. The run statement and the collection statement are the call vectors of two entries of the LEDGER CONTRACT (`acceptRun`, `acceptCollection`), and lane 0 binds the label of the call (E-1), so a statement of one kind cannot be accepted by the entry of the other.
- **Public values of `L`, index table** (LEAD F-8):

| Word | Field | Words | Encoding |
|---:|---|---:|---|
| 0 | `root_stmt`: the node statement of FP-2 (on path C, the leaf-layer statement in the same layout with `stmt_tag` SEG) | 74 | FP-2 |
| 74 | anchor statement: `mode: tag(1)` (ORD 0, CHUNK 1), `j: u32`, `k: u32`, `e_in: u16`, `e_out: u16` (the entry rows of the `DIGEST` entries whose outputs are `a_in` and `a_out`) | 7 | FP-1 types |
| 81 | `run_tag` | 8 | `digest` |
| 89 | `layout_d` | 8 | `digest` |
| 97 | `X` | 9 | `fr` |
| 106 | lane map: 4 pairs `(lane: u16, entry: u16)`; the wrap requires lane `lane` of the call vector to equal the output of `DIGEST` entry `entry`; an unused pair is `(0, 0)` and is ignored | 8 | `u16` |
| 114 | `pv` | `2B + 9V` = 2,816 | section 1.8 |
| 2,930 | call vector, lanes 0 to `1 + L_TX` | `9 (2 + L_TX)` = 9,234 | `fr`; a lane in the lane map holds zero limbs here |

Total 12,164 words at `B = V = 256` and `L_TX` = 1,024.

Open, the exact missing facts (FP-3 stays open):
1. The carrier of the effect region `E` and of `comm` from the terminal state of a run of more than one shard to the call-vector table of `L`. `CO/FRONTEND.md` section 1.4 and its merge relation name only "the root's public values" of a one-root execution; the node statement of FP-2 carries no effect word, and no source defines a carrier.
2. The call-vector lanes of the run statement and of the collection statement. They follow from the compiled transcript of the LEDGER CONTRACT entries `acceptRun` and `acceptCollection` (`abi.json` of `sherlock compile`), and that contract is not built.
3. The public and contract-side form of a collection. `acceptCollection`, `descend` and `openClaim` (LDG-003 to LDG-005) recompute SET openings in a Compact contract, and under `sherlock-value/2` the SET grammar is Poseidon2 over BabyBear, which a Compact contract does not compute natively.

#### FP-4 One digest table

Internal digests are `P2(tag, words)`: Poseidon2 over BabyBear, width 16, rate 8, the tag word first, the total length `1 + l` in the first capacity word, the last rate block padded with zeros, output one `digest` (section 1.8, `PR` section 3.6). Public digests are `PoseidonFr_4(tag, pack(d2))`, the Midnight fixed-length sponge with capacity `cap(4, 2)` (section 1.8). The ledger contract uses Compact `transientHash`, the native Midnight Poseidon of use code `u` = 0.

**BabyBear tag codes** (LEAD F-9, under OD-18): 1 `TAG_EMPTY`, 2 `tagStep`, 3 `TAG_NODE`, 4 `tagEnt` (the codes of `FrLog.lean:952-955`); 5 `DOM_PROG`; 6 `DOM_PROGC`; 7 `DOM_LOCK`; 8 `DOM_KEYTRIPLE`; 9 `DOM_KEYDAG`; 10 `DOM_KEYNODE`; 11 `DOM_LAYOUT`; 12 `SHK_IMAGE`; 13 `SHK_VALUE_V2`; 14 `SHK_OUTCOME_V2`; 15 `SHK_IN`; 16 `SHK_RUN`; 17 `SHK_CLAIM`; 18 `SHK_SET_LEAF`; 19 `SHK_SET_DEP`; 20 `SHK_SET_JOIN`; 21 `SHK_SET_FOREIGN`; 22 `SHK_SET_ADM`; 23 `SHK_SET_VIEW`; 24 `SHK_ADM`; 25 `SHK_ADM_STATE`; 26 `SHK_VIEW`; 27 `SHK_VIEW_STATE`; 28 `SHK_DEPS`; 29 `SHK_SD2`; 30 `DOM_STATE_BODY`; 31 `DOM_OUTHIDE`; 32 `DOM_UKEYS`. The memory, stack and ROM domains of the HASH registry (HSH-002) and every later Sherlock digest take the next free codes, in registry order; a used code is never reused.

**Constants** (LEAD F-9). Each is the hash of its tag alone, so its preimage length (1) differs from that of every digest with the same tag, and under premise A5 it equals no such digest: `D_EMPTY = P2(TAG_EMPTY)`, `EMPTY_DEPS = P2(SHK_DEPS)`, `NO_INPUT_LINK = P2(SHK_RUN)`, `NO_PREMISE = P2(SHK_SET_ADM)`, `VIEW_NONE = P2(SHK_VIEW)`, and the absent-input constant `VAL_NONE = P2(SHK_VALUE_V2)`.

**The table** (the `sherlock-value/2` forms; the forms of kits VALUE DIGEST, HASH and INIT over `Fq` are the halo2 prototype):

| Digest | Field and sponge | Tag | Preimage after the tag, in order (types of FP-1) | Words with the tag | Salt |
|---|---|---|---|---:|---|
| `image_d` | BabyBear, `P2` | `SHK_IMAGE` | `prog_root: digest`, `layout_version: u16`, `profile_id: u16`, `entry_kind: tag(1)` (0 `Expr`, 1 `Main`), `entry_a: u32` (`e0` or `in_fn`), `in_ty: digest` (`ty_d` for `Main`, zero for `Expr`) | 22 | none |
| `prog_d` | BabyBear, `P2` | `DOM_PROG` | `image_d: digest`, `plan_d: digest`, `key_root: digest`, `machine_version: u16` | 26 | none in the STARK; public only as `C_P` |
| `vd` | BabyBear, `P2` | `SHK_VALUE_V2` | `2: u16` (version), `form: tag(3)`, `arity: u32`, then the fields: `UInt n` (0, 0, `n: u128`); `Bool b` (1, 0, `b: bit`); `Nil` (2, 0); `Cons(h, t)` (3, 2, `vd(h)`, `vd(t)`); `Leaf(x)` (4, 1, `vd(x)`); `Branch(l, r)` (5, 2, `vd(l)`, `vd(r)`); constructor field step (7, 2, `acc_k`, `vd(f_k)`) with `acc_0` the zero digest; constructor completion (6, `m`, `ty_d: digest`, `c: u32`, `acc_m`) | 13 to 29 | none |
| `val_d` | as `vd` | | the `vd` of the input value, or `VAL_NONE` | | none of its own |
| `input_d` | BabyBear, `P2` | `SHK_IN` | `salt: 8 words`, `val_d: digest`, `fuel_lo: u64`, `fuel_hi: u64`, `stack_limit: u64` | 29 | `salt`: 8 uniform words from the user's CSPRNG, fixed for the run at INIT |
| `outcome_d` | BabyBear, `P2` | `SHK_OUTCOME_V2` | `2: u16`, `kind: tag(2)`, `payload: digest`: Halted (0, `vd(v)`), Rejected (1, `[code, 0, 0, 0, 0, 0, 0, 0]`), Exhausted (2, `[kind, 0, 0, 0, 0, 0, 0, 0]`) | 11 | none; hidden in public by `DOM_OUTHIDE` |
| `claim_d` | BabyBear, `P2` | `SHK_RUN` | `1: u16`, `prog_d`, `input_d`, `outcome_d` | 26 | through `input_d` |
| `fclaim_d` | BabyBear, `P2` | `SHK_CLAIM` | `KIND_FOREIGN: u16` = 2, `vk_id_inner: digest`, `adapter_id: u16`, `out_d: digest` | 19 | none |
| `adm_d`, `vw_d` | BabyBear, `P2` | `SHK_ADM`, `SHK_VIEW` | `1: u16`, `prog_d`; `1: u16`, `val_d`, `view_root` | 10, 18 | none |
| SET forms | BabyBear, `P2` | `SHK_SET_*` | the forms of the GP-2 hook, item 1, with `n_steps: u64` | 13 to 37 | through `claim_d` |
| `deps_d` step | BabyBear, `P2` | `SHK_DEPS` | `deps_d`, `claim_d` | 17 | none |
| `state_d`, `body_d` | BabyBear, `P2` | `SHK_SD2`, `DOM_STATE_BODY` | FP-5 | 42, per schema | none |
| `C_P` | Fr, `PoseidonFr_4` | `SHK_PROGC` | `pack(P2(DOM_PROGC, r_P, prog_d))`, `r_P`: 8 words | 2 | `r_P`, one per deployment |
| `input_pub` | Fr, `PoseidonFr_4` | `SHK_INPUT` | `pack(input_d)` | 2 | the salt inside `input_d` |
| `outcome_pub` | Fr, `PoseidonFr_4` | `SHK_OUTCOME` | `pack(P2(DOM_OUTHIDE, input_d, outcome_d))` | 2 | `input_d`, which holds the salt and is never public (LEAD F-10) |
| `claim_pub` | Fr, Compact `transientHash` | none | `[C_P, input_pub, outcome_pub]` | 3 | through `input_pub` (LEAD F-10) |

`claim_pub` is the replay key of the ledger contract (LDG-002, LW-010). Every partition of one run has the same salt, because INIT fixes it in `input_d`, so the second partition of a run gives the same `claim_pub` and is refused. A new salt gives another input and another claim.

**Restated kits.** VALUE DIGEST computes the `vd` rows above in place of the `sherlock-value/1` rows of its table, with the same forms, the same field order and the version word 2. HASH computes `P2` with the tag word first; the pinned sponge and the capacity-tagged sponge of the prototype (HASH families 1 and 2) are one form over BabyBear. INIT opens `prog_d` to `image_d` and `image_d` to `prog_root` and the entry metadata (two hashes), opens `input_d` (one hash), range-checks `fuel_lo`, `fuel_hi` and `stack_limit` as `u64` limbs, and leaves the salt words unconstrained.

**Lean.** `digest_table_inj` (new `lean/DigestTable.lean`; open): under `P2Inj` (premise A5, `FrLog.lean:1006`), two digests of the table are equal only if their tags, lengths and preimages are equal; so each constant differs from every digest of its tag. The public forms use `packed_hash_inj` and `public_digest_binds` (`Bridge.lean:290,419` at `bdb92973a`, proved) with `pack_injective` (`FrLog.lean:218`, proved).

**Checks.** `digest_table_vectors`: one differential vector for each row, with the preimage word list of the row, computed by the Rust `P2` against the stark-sdk Poseidon2 (`SB/crates/stark-sdk/src/config/baby_bear_poseidon2.rs`) and against the AIR, on the CPU and the GPU. Hand-computable preimage check: `input_d` with salt `[1, 2, 3, 4, 5, 6, 7, 8]`, `val_d = VAL_NONE`, fuel `2^64` and stack limit `2^16` has the limbs `fuel_lo = [0, 0, 0, 0]`, `fuel_hi = [1, 0, 0, 0]`, `stack_limit = [0, 1, 0, 0]`. Mutants: `digest_tag_swapped` (a `claim_d` computed with `SHK_CLAIM`), `digest_constant_as_claim` (`NO_INPUT_LINK` offered as a claim), `input_salt_changed_midrun`, `outcome_pub_unsalted` (the control: an `outcome_pub` over `outcome_d` alone).

#### FP-5 Seal read of `view_root`

`view_root` is a header word of the state commitment (LEAD F-11; AD-17 stays OWNER):

```text
state_d = P2(SHK_SD2, outcome_tag: tag(2), outcome_payload: digest, input_d: digest,
             deps_d: digest, view_root: digest, body_d: digest)                     42 words, 6 permutations
body_d  = P2(DOM_STATE_BODY, schema_id: u16, body words in keyed order)
```

`outcome_payload` is the `vd`, or the code or kind in word 0 with zeros after it. `view_root` leaves the body. Under form (a) or (c) of AD-17, and for an absent input, the header word is `VIEW_NONE`. INIT sets it at genesis (from the witness under form (b)); no rule writes it (mutant `core_unlisted_register_changed`). `read_input` reads the header cell inside the leaf. `open_state_header` returns `StateOpeningCells { outcome_tag, outcome_payload, input_d, deps_d, view_root, body_d }`, and SEAL and SEAL-WITH-DEPS compare the opened `view_root` with the `view_root` of the `VIEW` claim. The node never re-hashes the body. Reason: `body_d` is a flat sponge with no selective opening (kit BOUNDARY COMMITMENT), so a body word has no defined opening in the node, and the header is the one typed opening that exists. X10 measures the six-word header opening.

**Lean.** `header_open_sound` (`BinaryNode.lean`; open; the "Header opening" row of kit BOUNDARY COMMITMENT): if `P2(SHK_SD2, w) = post_d` for the authenticated lane `post_d`, then `w` is the header of the committed state (premise A5).

**Checks.** `boundary_header_leaf_vs_std`: the header digest of the leaf AIR equals the stark-sdk Poseidon2 on the same 42 words, zero words included. `pos_node_header_opening_all_outcomes` covers the `view_root` word. Mutants `seal_view_root_forged` (an opened `view_root` that is not the committed header word), `view_root_written_by_rule`.

#### FP-6 Public `n_steps`

`n_steps` is not public (LEAD F-12; the owner's `n_steps` rule). It is not in the run statement, the call vector, `acceptRun` or the `runs` map. It stays authenticated private metadata: `n_steps = h_hi` of the sealed segment, range-checked by NODE, inside the SET forms `LEAF` and `DEP` (NODE-003). Reasons: the exact height leaks private control flow (`K21/BRIEF.md:53`), so the exact public height breaks zero knowledge end to end; no source defines a padding rule for a public `n_steps`; removal is the decision of the recalibration proposals that decide it (`K21/proposals/opus.md:486-496`; `K21/proposals/gemini.md:570-576`). A collection publishes `set_root` only through FP-3, and `set_root` hides `n_steps` because `claim_d` holds the salted `input_d`. The leaf count and the shard shapes follow the public padding `N_pub` of the plan (section 1.6). The GP-1 acceptance "the same `n_steps` under three partitions" is checked on the private value by the integration harness (section 3.2). Owner amendments: SRC-006 (the run entry), SRC-007 (`acceptRun`), LDG-001 (the `runs` map).

**Lean.** `run_view_no_nsteps` (`Bridge.lean`; open): the published view of a run call is a function of the call vector, which holds no `n_steps`; it follows from `wrap_view_only_claim` (`Bridge.lean:392` at `bdb92973a`, proved) once the call vector of FP-3 is fixed.

**Checks.** Keygen refusal `decider_nsteps_public` (a wrap key whose call vector carries `n_steps`; forbidden-input check of LW-003). Positive `pos_run_nsteps_private` (the harness reads the same private `n_steps` from the SET leaf under three partitions).

#### FP-7 Chunk and E-1 bindings

Under placement V, `bind_label` runs inside each chunk's own run and takes the private tape bytes `m` with the V+ layout parameters `(|domain|, domain, offset)` (LEAD F-13):

- `bind = le31(SHA256(m))`, written to the outcome; the wrap requires lane 0 to equal it (CF-008).
- The claim address `addr_c = m[|domain| .. |domain| + 32)`: the 32-byte contract address follows the domain bytes in the binding preimage (`ML/ledger/src/verify.rs:2151-2156`). `bind_label` requires `offset = |domain| + 32`, so `addr_c` is the address of the called contract.
- The claim `ep_hash = SHA256(EP_OPEN || lab)`, with `EP_OPEN` the 32 bytes `"midnight:entry-point"` and 12 zero bytes, and `lab` the raw label bytes: `enc(e)` of the pinned window without its SCALE length prefix (`ML/onchain-state/src/state.rs:913-918`; `ML/base-crypto/src/hash.rs:98-102`; `ML/base-crypto/src/repr.rs:66-69`). It is computed on the `sha2` chip from the window, so the claim targets the same entry label.
- For a chunk `j < k`, the run writes `(addr_c, ep_hash, comm_(j+1))` into the effect words of its `claimContractCall` at the static indices of the entry template; the slice shard fixes `comm_(j+1)`, `(j, k)` and the slice bounds (`CO/ATOMIC-CHUNKS.md:120-127`). This replaces the binding shard of placement B (`CO/ATOMIC-CHUNKS.md:272`).

E-1 window under a variable-length preimage. The right alignment is a property of the midnight-circuits `sha2_256_varlen` gadget (`E1:129`), which only placement S uses. Under placement V, `m` is a tape byte array of the per-entry length `M_B(e)` with the true length `len_m: u32` as a tape word and zero bytes after it; the window offsets are absolute byte positions in `m`; `bind_label` runs exactly `N_blk(e) = ceil((M_B(e) + 9) / 64)` SHA-256 compressions over the standard SHA-256 padding of the first `len_m` bytes and selects the chaining value after block `ceil((len_m + 9) / 64)` as the digest, so the step count `N_e` stays static (CF-004). `M_B(e)` is a constant of `abi.json`: the largest binding preimage of the entry's static transcript layout. This is the form of the binding shard of `CO/FRONTEND.md` section 1.9 (`m` padded to `M_B` with the true length as witness and standard SHA padding), moved into the run.

**Lean.** `bind_label_sound` (new `clean/fork/Clean/Sherlock/BindLabel.lean`; open): with the windows pinned, `bind = le31(SHA256(m))` and `addr_c`, `ep_hash` computed from the windows, a call whose label or address differs gives `m' != m`, or a collision of SHA-256 truncated to 248 bits. CF-L2 (open) uses it for the claim chain.

**Checks.** T0 with the window at offset 58 (MEASURED, `PR` section 7.2). `ep_hash_vs_ledger`: `ep_hash` for the labels `acceptRun` and `acceptRunB` and for labels of 1, 55, 56 and 64 bytes against `EntryPointBuf::ep_hash`. `bind_len_select`: `len_m` in {119, 131, 183, 247, 503, 1,015, 2,039} bytes (2, 3, 3, 4, 8, 16 and 32 blocks, the preimage lengths of `E1` section 5) against `persistent_hash`, at one `M_B` of 2,039 bytes. Mutants `chunk_claim_addr_free` (`CO/ATOMIC-CHUNKS.md:357`), `chunk_ep_hash_other_label`, `bind_len_select_wrong_block`. PENDING evidence, not design: T0 on calls with a fallible transcript or non-empty effects, the remaining T2 mutants, T3 to T6.

## 2. Module kits

Every kit has the same nine items: purpose; interface; invariants; row budget; verification; Lean obligations; code skeleton; what the implementing agent reads and must not need to read; dependencies. Signatures are sketches and were not compiled. A kit is implemented by one agent in one session against its interface only.

Four kits follow the GP-4 hook: INIT (genesis and input admission), REFERENCE BRIDGE, HOST CHECKPOINT STORE, and the GP-3 general verifier (five subkits). LEDGER CONTRACT (a Compact contract, after HOST CHECKPOINT STORE) owns the contract-state properties. There are 26 kits: 19 from the merge, the four of revision B2, LEDGER CONTRACT, and the co-processor kits ZERO KNOWLEDGE and COMPACT FRONTEND.

### Kit: BUS

#### Purpose

BUS connects request lanes to provider rows through the tuple schema of section 1.2, and owns the tagged fixed table that serves RANGE and DEC. It gives membership only; each provider kit gives the relation and the uniqueness that make membership enough.

#### Interface

- Dynamic bus, as landed (`chip/bus.rs:40-126`): `BusConfig::configure(meta, cols)`, `request`, `request_pair`, `provide`. Two request lanes, one provider table. The chip id registry is section 1.2. A request is active when its chip id is non-zero; an unused lane is the zero tuple.
- Tagged fixed table (new, L-6, three-column form): one lookup over three table columns `(tag, key, val)`. `TAG_RANGE = 1`, `TAG_DEC = 2` (proposed; 0 is the tag of a row that makes no query). Table rows: the zero row `(0, 0, 0)`; `(TAG_RANGE, limb, 0)` for `limb < 2^12`; the DEC idle row `(TAG_DEC, 0, 0)`; `(TAG_DEC, dec_key, dec_val)` with `dec_key != 0`, one row per DEC key.
- Exact query expressions (B2 finding 10). Let `q_rng` and `q_dec` be fixed selector columns, each 0 or 1, never both 1 on a row.
  - Query tuple of a row: `(q_rng * TAG_RANGE + q_dec * TAG_DEC, (q_rng + q_dec) * key, (q_rng + q_dec) * val)`, with `key` and `val` advice cells of that row. Every multiplier is a fixed selector, as on the landed bus (`chip/bus.rs:50-61`, `qr * r`); no component is multiplied by a witness.
  - RANGE row (`q_rng = 1`): `key = limb`, `val` is constrained to 0 by the RANGE gate.
  - DEC row (`q_dec = 1`), one per step slot, enabled or not: `key = dec_key`, `val = dec_val`. For a disabled slot DEC forces `dec_key = 0` (the class sum of kit DEC is `sum selected = enable = 0`), and the only table row with tag `TAG_DEC` and key 0 is the idle row, so `dec_val = 0`. The slot enable is a witness under the key, and a full and a partial leaf use the same key: the tag does not move with it.
  - Any other row (`q_rng = q_dec = 0`): the query is `(0, 0, 0)` whatever the shared bank columns hold, and it matches the zero row. The lookup puts no condition on a row that makes no query.
  - There is no advice tag and no `enable * tag` product. A DEC query cannot take the tag `TAG_RANGE` or 0, and a RANGE query cannot take the tag `TAG_DEC`.
- Row-role allocator (from `astra-area1.md:198-218`): BUS owns the shared 12-column bank and the packed selector plan (`chip/vsel.rs`). A module asks for row roles; it does not allocate its own copy of the columns. The plan checks row disjointness and `selector_degree + body_degree <= 6`.
- Ports manifest: for each named port of section 1.3, its name, lane order, range, producer, consumer and activation condition, in one file (`ports.rs`).

Requests and provides no tuple of its own.

#### Invariants

Guaranteed:

1. Every active request equals an active provider tuple. Request rows cannot serve requests (`chip/bus.rs` tests `requests_do_not_serve_each_other`, `lane_one_is_checked`).
2. Only the zero tuple has chip id 0 (`zero_chip_with_payload_rejected`).
3. A RANGE query can match only a RANGE row and a DEC query only a DEC row, with no condition on the witness, because the tag is fixed (LC AUDIT finding 6). A disabled DEC slot matches only the DEC idle row.
4. No challenge, one phase, rotation 0 on the lookups.

Assumed from providers: provider rules 1 to 5 of section 1.2. Assumed from CORE: unused lanes are zero in all five cells.

#### Row budget

- Dynamic bus: 0 rows of its own. A pair of requests takes one request row unless it sits on an existing CORE row; a provider tuple sits on the provider's operation row.
- Tagged table: `2^12 + N_dec + 2` fixed rows (the zero row and the DEC idle row), overlapping the advice region in the domain. The interpreted rule inventory is known (120 rules, X1; section 1.7); `N_dec` is the number of dispatch classes, which is not known until the guard list of AD-9 is fixed (ESTIMATE below 2,048, from `gemini-area1.md:541`).
- Node effect of the merge of DEC and RANGE: one lookup argument less, about -22,600 node rows (ESTIMATE, LC AUDIT finding 6; the three-column form keeps three table columns, so the -29,062 figure of `gemini-area1.md:149` does not apply).
- A third bus lane would cost about +22,600 node rows (ESTIMATE, `O/design.md:134` fit). More requests per step take more rows, not more lookup arguments.

#### Verification

- Quint: refine the bus part of `qnt/chipLeaf.qnt`; new `qnt/busContract.qnt` with provider roles, the chip registry, key disjointness of the two MEM providers and the tagged table. Invariants: `noZeroChipProvider`, `unassignedChipUnserved` (no provider row holds a chip id outside the registry, so a request with an unassigned non-zero id is never served; B2 finding 19), `memProviderClassesDisjoint` (between the window class and the line class; identical duplicate lines allowed), `taggedNoAlias`, `decIdleOnlyWhenDisabled`, `levelOrder`, `portOrderAcyclic`.
- MockProver positive: the five landed tests of `chip/bus.rs:183-250`; `bus_both_lanes_all_chips` (ALU, MEM, HASH, ROM in one region, the same numeric key on two chips); `tagged_range_and_dec_coexist`; `tagged_full_and_partial_leaf_same_key` (a leaf with every slot enabled and a leaf with a disabled suffix prove under one key; the disabled slots query the DEC idle row); `bus_unused_provider_and_repeated_request` (membership semantics: legal).
- Named negative mutants: `bus_provider_gate_removed`, `bus_request_serves_itself`, `bus_second_lane_unchecked`, `bus_chip_id_unpinned`, `bus_zero_chip_payload`, `bus_inactive_slot_leaks_payload`, `bus_unassigned_chip` (a request with chip id 5), `bus_key_namespace_alias`, `tagged_dec_query_matches_range_row`, `tagged_range_query_matches_dec_row`, `tagged_tag_from_advice` (the control circuit: with the tag moved to an advice cell the previous two witnesses are accepted), `tagged_disabled_dec_slot_nonzero_val` (a disabled slot with `dec_key = 0` and a non-zero control word), `tagged_enabled_dec_slot_idle_row` (an enabled slot with `dec_key = 0`; refused by `sum selected = enable` of kit DEC), `tagged_dec_query_zero_tag` (a DEC row whose witness tries to reach the zero row; impossible with a fixed tag, the control is the `enable * TAG_DEC` form), `vsel_overlapping_roles`, `vsel_degree_overflow` (the last two are configuration failures, not constraint failures).

#### Lean obligations

| Obligation | File | Premises | Contract discharged | Status |
|---|---|---|---|---|
| A balanced bus is lookup-sound (`balanced_lookup`) | `lean/ModularComposition.lean:55` | none | nothing for the landed bus: the landed lookup is membership, not a permutation, so `BusBalanced` is not its property (B2 finding 7). Kept for the Profile B LogUp form only | proved in C3 |
| If every module meets its contract, every tuple on the bus of an accepted leaf is valid (`provided_valid`, `requested_valid`) | `lean/ModularComposition.lean:68-88` | module contracts over bus tuples, tuple level order, `LookupSound` (inside `LeafRel`, `:60-61`) | modular composition (ARCH-BRIEF obligation 3), generic form, bus tuples only. It says nothing about a guarantee that travels through copied cells | proved in C3 |
| The Sherlock tuple type, the tuple level function of section 1.1 (with the address sub-level of MEM tuples) and the chip registry instantiate `Module` and `Contract` with no cycle | new `lean/BusContract.lean` | none | BUS level order for the real module list | open |
| The lookup argument gives `LookupSound` (every enabled request tuple is in the set of enabled provider tuples) up to the lookup error | not stated (the library has no field theory) | proof-system premise | BUS guarantee 1 | open (premise) |
| Port composition (new, B2 finding 7): modules with typed port certificates. A `PortModule` has, besides bus requests and provides, in-ports and out-ports, each a tuple of cells with a predicate. If the port dependency relation is well founded, every module guarantees its out-port predicates and its provided tuples from its relation, the validity of its requests and the predicates of its in-ports, and each consumer's in-port cells equal the producer's out-port cells (the copy equalities), then every port predicate and every tuple of an accepted leaf is valid and the CORE goal follows (`leaf_refines_ports`). A non-vacuous instance with a copied port is required: HASH slot -> `Tree8` of the backend -> an imported window row served as a MEM tuple -> a CORE read (`toy_ports`), with a counterexample when the copy equality is dropped | new `lean/PortComposition.lean` | none beyond the module contracts | the composition of STACKS, MEMORY BACKEND, PAGE/ROM, VALUE DIGEST, BOUNDARY COMMITMENT, INIT, whole-sponge HASH calls and RANGE and DEC outputs with CORE | open |
| Memory history: `MemHist(pre)` and the leaf relation give `MemHist(post)`, and every MEM tuple served in the leaf is the record of the log at its address; the order is the address order | new `lean/MemContract.lean` | backend contract, hash premises, INIT at genesis | the validity predicate of MEM tuples across leaves | open |

#### Code skeleton

Files: `chip/bus.rs` (owned, extend), `chip/vsel.rs` (owned), new `chip/tagged.rs`, new `chip/ports.rs` (the manifest; the `AuthRec` and `AuthRom` types are declared here and constructed only in `core/ports.rs`), new `chip/layout.rs`.

```rust
pub const CHIP_ALU: u64 = 1;   // landed
pub const CHIP_MEM: u64 = 2;   // landed
pub const CHIP_HASH: u64 = 3;  // landed in poseidon.rs:53 and hashchip.rs:32; move the one constant here
pub const CHIP_ROM: u64 = 4;   // new

pub struct TaggedTable { /* tag, key, val table columns; one fixed tag selector */ }
impl TaggedTable {
    /// One lookup over (tag, key, val); the tag of a query row is a fixed selector value.
    pub fn configure(meta: &mut ConstraintSystem<F>, cols: Cols) -> Self;
    /// Load the zero row, 2^12 RANGE rows, the DEC idle row (TAG_DEC, 0, 0) and the DEC rows;
    /// refuse a duplicate DEC key and a DEC row with key 0.
    pub fn load(&self, l: &mut impl Layouter<F>, dec_rows: &[(F, F)]) -> Result<(), Error>;
    /// Query (TAG_RANGE, limb, 0) on this row: sets the fixed q_rng.
    pub fn range_query(&self, r: &mut Region<'_, F>, row: usize, limb: &Cell) -> Result<(), Error>;
    /// Query (TAG_DEC, key, val) on this row: sets the fixed q_dec. Used for every slot, enabled or
    /// not; the tag never depends on a witness.
    pub fn dec_query(&self, r: &mut Region<'_, F>, row: usize, key: &Cell, val: &Cell) -> Result<(), Error>;
}

impl SelectorPlan {
    /// Check role disjointness and composed degree before assigning codes.
    pub fn freeze(roles: &[RoleSpec], max_degree: usize) -> Result<Self, ConfigError>;
}
```

#### Agent reads / must not need to read

Reads: this kit, section 1.2 and 1.3, `chip/bus.rs`, `chip/vsel.rs`, `chip/lib.rs`, D1 to D8 (`O/design.md:94-101`), `midnight-circuits` 7.3.0 `verifier/verifier_gadget.rs:308-345,604-611`. Must not need: CORE rule bodies, `chip/dense.rs`, `chip/poseidon.rs` internals, `crates/mori-recursion`.

#### Dependencies

None. First kit to build.

### Kit: RANGE

RANGE is split from ALU (the brief lists them separately): it is the level-0 provider that ALU, DEC, CORE, MEM and VALUE DIGEST all use, and it can be built and proved before ALU exists.

#### Purpose

RANGE proves that a cell holds an integer below `2^B` for a static width `B`. It is the only source of integer bounds in the leaf; a host integer type proves nothing.

#### Interface

- Named port: `check(source: &Cell, bits: B) -> RangedCell<B>`; the returned handle is the same cell.
- Shapes: 128 bits (landed: 11 limbs of 12 bits with a top-limb pad, running-sum mode 12 rows, `chip/range.rs:30-34,93,207`); 64 bits (new, needed by ALU multiply, LC AUDIT finding 5); small widths for DEC parts and counters (one limb with a shift: `limb * 2^(12-t)` also in the table).
- Requests: tagged-table queries `(TAG_RANGE, limb, 0)`. Provides: no bus tuple. Its guarantee leaves through the port: `RangedCell<B>` is a port certificate (`source < 2^B` as an integer) in the sense of section 1.1.

#### Invariants

Guaranteed: the source cell equals the reconstruction of its limbs; every limb is in the table; the top limb is bounded to the exact width; the reconstruction is copy-connected to the source. Assumed: tagged-table soundness (BUS).

#### Row budget

12 rows for a 128-bit value in running-sum mode (from `chip/range.rs:12-19`, landed layout); about 7 rows for 64 bits (ESTIMATE). Table: 4,096 fixed rows. A 14-bit limb (16,384 rows, fewer limb rows) is a later measurement (`gemini-area1.md:745-749`), not part of the first build.

#### Verification

- Quint: none of its own (arithmetic); covered by `qnt/aluArith.qnt` at a reduced width.
- MockProver positive: `range_128_boundaries` (0, 1, 2^64, 2^128 - 1), `range_64_boundaries`, `range_small_widths`.
- Named mutants: `range_limb_out_of_table`, `range_top_pad_unlinked`, `range_source_copy_removed`, `range_running_sum_broken`, `range_value_2_128` (2^128 refused), `range_field_wrap` (`p - 1` refused).

#### Lean obligations

| Obligation | File | Premises | Contract discharged | Status |
|---|---|---|---|---|
| RANGE as a `Module` instance: limbs in the table and the reconstruction gate imply `source < 2^B`, with no wrap in the field | new `lean/AluRange.lean` (written directly in Clean, as `toyAlu` is, `FD/EXPERIMENTS-C3.md` part 2) | table soundness | RANGE contract | open |

#### Code skeleton

File: `chip/range.rs` (owned).

```rust
impl RangeConfig {
    /// Constrain `source < 2^bits` with 12-bit limbs through the tagged table; return the same cell.
    pub fn check(&self, r: &mut Region<'_, F>, o: &mut usize, source: &Cell, bits: u32) -> Result<RangedCell, Error>;
    /// 64-bit halves of a checked 128-bit value, for the ALU multiply.
    pub fn split64(&self, r: &mut Region<'_, F>, o: &mut usize, v: &RangedCell) -> Result<[RangedCell; 2], Error>;
}
```

#### Agent reads / must not need to read

Reads: this kit, kit BUS (tagged table), `chip/range.rs`. Must not need: anything else.

#### Dependencies

BUS.

### Kit: ALU

#### Purpose

ALU proves exact unsigned 128-bit operations and their overflow flag, so that an overflow (`Rejected(10)`) is provable and not a missing proof (L-7).

#### Interface

Provides `(ALU, op + 8 flag, x, y, z)` (`chip/alu.rs:23-25,62`). Requests RANGE checks. Op list: only operations of the reference machine. `BinaryOp` is Add, Sub, Mul, Eq, Lt, Cons, Branch and `UnaryOp` is Leaf, Not (`crates/mori-core/src/sherlock/helper_machine/shared.rs:45-58`).

| op | Name | `flag = 0` | `flag = 1` | Status |
|---:|---|---|---|---|
| 1 | ADD | `x + y = z` | `x + y = z + 2^128` (carry) | landed |
| 2 | LT | `z = [x < y]`, boolean | not allowed | landed |
| 3 | SUB | `x - y = z` | `x - y = z - 2^128` (borrow) | new |
| 4 | MUL | `x y = z < 2^128` | `z = x y mod 2^128` and the high part is non-zero | new |
| 5 | EQ | `z = [x = y]`, boolean | not allowed | new |

`(z, flag)` is unique for every `(op, x, y)`, including the overflow case, so the chip is a function of its identity (LC AUDIT item 21). `x`, `y`, `z` and every internal witness are range-checked below 2^128 inside the chip. CORE maps `flag = 1` to the machine outcome; ALU does not choose outcomes. Boolean negation (`UnaryOp::Not`) is a CORE gate `z = 1 - x` on a cell that CORE constrains to be boolean, not an ALU op (`astra-area1.md:775` proposes op 6 `NOT_BOOL`; not needed on the bus). Division, signed and bitwise operations are not in the list (L-7); they are added only if the frozen rule inventory shows a rule that needs them.

Multiply (lane C form, LC AUDIT finding 5): with 64-bit limbs, `x0 y0 = c0 2^64 + r0`; `x0 y1 + x1 y0 + c0 = c1 2^64 + r1`; `z = r0 + 2^64 r1`; `flag = 1` iff `x1 y1 + c1 != 0`. Every limb is range-checked and `c1 < 2^65` (`c1 = c1_lo + 2^64 c1_bit`, `c1_lo < 2^64`, `c1_bit` boolean, `gemini-area1.md:728-737`). All intermediate integers are below 2^129, so no equation wraps in the field. A single field equation `x y = z` is unsound (`astra-area1.md:820`).

#### Invariants

Guaranteed: the table above; an unused provider slot is the zero tuple. Assumed: RANGE soundness. Not assumed: any range from the requester.

#### Row budget

ESTIMATE (the landed running-range ALU is 12 rows per op with operand ranges left to the requester, `chip/alu.rs:131`): ADD, SUB, LT with two fresh 128-bit operands 36 to 40 rows; EQ 1 to 4 rows plus operand checks; MUL 64 to 128 rows. Two more 128-bit decompositions per operation are 24 more rows in running-sum mode (LC AUDIT finding 5). The number of ALU slots per leaf comes from the per-rule table.

#### Verification

- Quint: new `qnt/aluArith.qnt` at a reduced width (exact results, carry, borrow, overflow, uniqueness of `(z, flag)`).
- MockProver positive: `alu_boundary_vectors` (0, 1, 2^64 - 1, 2^64, 2^128 - 1 for every op), `alu_mul_cross_terms_only`, `alu_mul_low_zero_high_nonzero`, `alu_mul_c1_needs_65_bits`, `alu_sub_underflow_flag`, `alu_unused_slot_zero`.
- Named mutants: `alu_operand_range_missing` (`x = p - 5`; control: with the in-chip range check removed the witness passes), `alu_lt_field_wrap` (`x = p - 1`, `y = 2`, claims `x < y`), `alu_output_range_missing`, `alu_carry_forged`, `alu_borrow_forged`, `alu_lt_boolean_removed`, `alu_mul_field_residue`, `alu_mul_high_half_dropped` (overflowing product with `flag = 0`), `alu_mul_c1_overflow`, `alu_overflow_unprovable` (an honest overflowing ADD and MUL with `flag = 1` must be accepted; listed here because losing it narrows GP-1), `alu_eq_reverse_implication_removed`, `alu_second_result_for_same_inputs`.

#### Lean obligations

| Obligation | File | Premises | Contract discharged | Status |
|---|---|---|---|---|
| ALU as a `Module` instance: for each op the gates and the range checks imply the integer relation of the table and a unique `(z, flag)`; the multiply has no field wrap | new `lean/AluRange.lean` | RANGE contract | ALU contract | open |
| A non-vacuous CORE + ALU composition (`toy_contracts`, `toy_refines`, `toy_needs_bus`) | `lean/ModularComposition.lean` | none | shows the pattern only | proved in C3 |

ALU and RANGE are the two chips small enough to write directly in Clean (`FD/EXPERIMENTS-C3.md`, last line of part 2). `clean/fork` imports only `Init`, so the proofs are over `Nat` and `Fin` without Mathlib (`gemini-area1.md:793-795`).

#### Code skeleton

Files: `chip/alu.rs` (owned), optional `chip/alu/mul.rs`.

```rust
pub const OP_ADD: u64 = 1; pub const OP_LT: u64 = 2;            // landed
pub const OP_SUB: u64 = 3; pub const OP_MUL: u64 = 4; pub const OP_EQ: u64 = 5;
pub const FLAG_SHIFT: u64 = 8;                                    // landed

impl AluConfig {
    /// Provider gates for the op list; operands, result and witnesses ranged inside the chip.
    pub fn configure(meta: &mut ConstraintSystem<F>, bus: BusConfig, range: RangeConfig) -> Self;
    /// Assign `capacity` provider slots; unused slots are the zero tuple.
    pub fn assign(&self, l: &mut impl Layouter<F>, capacity: usize, ops: &[AluOp]) -> Result<(), Error>;
}
impl AluOp {
    /// The unique (z, flag) of the reference semantics for (op, x, y).
    pub fn eval(op: u64, x: u128, y: u128) -> Self;
}
```

#### Agent reads / must not need to read

Reads: this kit, kits BUS and RANGE, `chip/alu.rs`, `chip/range.rs`, the scalar arithmetic contract `crates/mori-backend/src/sherlock_machine/c6_scalar_arithmetic_step.rs:6-53`. Must not need: dispatch, stacks, memory, recursion.

#### Dependencies

BUS, RANGE.

### Kit: DEC

#### Purpose

DEC maps the dispatch facts of an enabled slot to exactly one rule of the reference machine and returns that rule's fixed metadata (micro-operation plan, write set, flags, rule lane). It is the only place where the rule inventory enters the circuit as data.

#### Interface

- Named port in: `DispatchInputs` (the enable bit; the canonical pre-state control, outcome, helper, result, top-frame and top-task tags; authenticated instruction cells; comparator results from ALU or is-zero gates). Every guard bit is linked to its source cells; a supplied boolean is not a fact until its producer proves the predicate (`astra-area1.md:612`).
- Named port out: `DecodeCells` (`rule_ordinal`, `microplan_id`, `write_set_id`, `flags`, `rule_lane`), each bounded.
- Requests: one tagged-table query `(TAG_DEC, dec_key, dec_val)` per step slot, with the fixed tag `TAG_DEC` whether the slot is enabled or not (kit BUS, exact query expressions). A disabled slot has `dec_key = 0` and so matches only the DEC idle row `(TAG_DEC, 0, 0)`: every control part is zero and `rule_lane = 0`, which is the reserved rule id of a disabled slot in SRC-001 (`O/requirements.md:26`). No rule has key 0 or lane 0. RANGE or boolean checks on every packed part. Provides: no bus tuple.
- Key and value:
  - `dec_key` is the dispatch class. Two constructions were drafted (open decision AD-9): (a) a priority network over the guard facts that yields a finite class, `class = sum selected[r] (ordinal(r) + 1)`, `sum selected = enable` (`astra-area1.md:614-623`), so `class = 0` exactly when the slot is disabled; (b) a packed 15-bit key of normalized guard fields (`gemini-area1.md:528-541`). The first build uses (a): the priority order P0, P1, P2, `owner_select` is the documented reference behaviour (`crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:4-38`), and the field list of (b) is not checked against the rule inventory. If (b) is ever taken, key 0 stays reserved for the idle row.
  - `dec_val` packs the control parts. Every unpacked part is bounded by its width (boolean bits or a RANGE check), so the linear reconstruction cannot wrap and unpacking is a bijection (`gemini-area1.md:563`; LC AUDIT item 20). The exact layout is frozen only with the rule inventory and the per-rule table. The layout of `gemini-area1.md:547-561` is not adopted as written: it carries four request-lane chip ids, ALU ops 1 to 9 and chip ids 2 to 6, which L-8, L-7 and L-4 reject.
- The ordered rule inventory is explicit and versioned; it is not derived from enum discriminants. Disabled rules and `/2` extension rules have no row, so their pre-states are unsatisfiable with `enable = 1` (SVM-004, SVM-009).

#### Invariants

Guaranteed: reference precedence; exactly one selected rule when enabled and none when disabled; one DEC row per key (checked at table load); the idle row `(TAG_DEC, 0, 0)` present and the only row with key 0; a disabled slot returns the zero control word and `rule_lane = 0`; every rule's `rule_lane` is non-zero and distinct (`validate_rom`); metadata equal to the fixed row of the selected rule; an unsupported dispatch path makes an enabled slot unsatisfiable. Assumed: canonical record ports and authenticated program reads from CORE; tagged-table soundness. Program admission is not proved by a DEC row.

DEC enforces the active profile: an outcome tag that profile `/1` excludes (exhaust kind 7, `O/dissent.md:226`) is not admitted because the encoding can represent it.

#### Row budget

ESTIMATE: one table row per dispatch class plus the DEC idle row; the query sits on an existing CORE row; unpacking 1 to 2 RANGE rows per step (`gemini-area1.md:581`); the priority network needs serial rows set by the guard inventory (not counted; the six-row spike ROM is not a cost model, `astra-area1.md:679`).

#### Verification

- Quint: new `qnt/decDispatch.qnt`: facts and selection as separate contracts; enumerate abstract guard combinations by dispatch group against the reference priority; partition of the rule enum into enabled, disabled and extension; injectivity of the control packing.
- MockProver positive: `dec_every_enabled_rule_roundtrip`, `dec_publication_with_zero_fuel` (P1 with zero remaining fuel), `dec_helper_vs_result_present`, `dec_comparator_boundaries`, `dec_disabled_slot_idle_row`, `dec_partial_leaf_same_key` (with kit BUS).
- Named mutants: `dec_priority_swapped`, `dec_guard_unlinked`, `dec_wrong_instruction_source`, `dec_two_rules_selected`, `dec_enabled_without_rule`, `dec_metadata_forged`, `dec_write_set_swapped`, `dec_rule_lane_zero`, `dec_unknown_class_accepted`, `dec_wrap_forgery` (a control part changed and compensated in another part modulo the field; control test: with the part bounds removed it passes), `dec_disabled_rule_query`, `dec_extension_rule_query`, `dec_disabled_slot_nonzero_lane` (a disabled slot with a non-zero `rule_lane`), `dec_range_row_alias` (a DEC query shaped as a RANGE row), `dec_duplicate_key_at_load` and `dec_rule_with_key_zero_at_load` (configuration failures).

#### Lean obligations

| Obligation | File | Premises | Contract discharged | Status |
|---|---|---|---|---|
| Packing of bounded control parts is injective in the field, so the unpacked parts are the parts of the table row | new `lean/DecDispatch.lean` | field size above the packed width | DEC "metadata equal to the fixed row" | open |
| DEC as a `Module` instance: a valid key yields the dispatch of the reference machine; disabled and extension rules have no row | new `lean/DecDispatch.lean` | tagged-table soundness, the frozen inventory | DEC contract | open |

#### Code skeleton

Files: `chip/dec.rs` (owned; replaces the stub), new `chip/dec/facts.rs`, `chip/dec/priority.rs`, `chip/dec/rom.rs`.

```rust
pub struct DecodeCells { pub rule_ordinal: Cell, pub microplan_id: Cell, pub write_set_id: Cell, pub flags: Cell, pub rule_lane: Cell }

impl DecConfig {
    /// One fixed ROM and the priority logic from the frozen inventory.
    pub fn configure(meta: &mut ConstraintSystem<F>, tagged: &TaggedTable, inv: &RuleInventory) -> Result<Self, ConfigError>;
    /// Derive every guard from named, authenticated input cells.
    pub fn constrain_facts(&self, ctx: &mut RegionCtx<'_>, inputs: &DispatchInputs) -> Result<DispatchFacts, Error>;
    /// Enforce reference precedence, query the table, return bounded metadata cells.
    pub fn decode(&self, ctx: &mut RegionCtx<'_>, facts: &DispatchFacts) -> Result<DecodeCells, Error>;
}
/// Refuse duplicate keys, zero or colliding rule lanes, missing plans, rows for disabled or extension rules.
pub fn validate_rom(inv: &RuleInventory, rows: &[DecodeRow]) -> Result<(), ConfigError>;
```

#### Agent reads / must not need to read

Reads: this kit, kit BUS (tagged table), kit RANGE, `chip/dec.rs`, the rule enum `RuleIdC` (`crates/mori-core/src/sherlock/helper_machine/shared.rs:400-534`, 133 variants) and the disabled and extension lists (`shared/owner_runtime.rs:70-90`, 7 and 4 variants; 122 rules pass `check_enabled`; `supported_rules()` counts 108, `O/tasks.md:67`; experiment X1 reconciled the counts: 120 enabled step rules, the 108 plus the 12 ADT Build and Plan rules, with the 2 `KAdminStartup*` rules no step; `FD/maxima/rule_maxima.md` is the per-rule table), `crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs`. Must not need: CORE's register layout, memory, ALU internals, recursion.

#### Dependencies

BUS, RANGE. The rule inventory (experiment X1, done: 120 enabled step rules, section 5.4) and the guard list of those rules (AD-9, not part of X1).

### Kit: HASH

Merged from `astra-area1.md:924-1075`, `gemini-area1.md:922-1116`, the landed `chip/poseidon.rs` and decisions E-2 and L-3. From Gemini the kit takes the capacity-word registry idea, the rule that every allocated permutation block runs all rounds through key-fixed selectors, the non-zero check of the sentinel constants at key generation, and the mutants `hash_round_skipped` and `hash_domain_swapped`. It does not take the dynamic `CHIP_HASH = 6` pool with a `(dom, sub, absorb_len)` bus key, the `SHK_VAL` capacity domain for value digests, or the `DOM_CLAIM` form of `claim_d` (section 6.1).

#### Purpose

HASH implements the pinned Poseidon permutation and the fixed-length sponge. It returns digests whose inputs are copy-connected to the caller's cells; the caller owns the meaning of the preimage.

#### Interface

- Bus: provides `(HASH, BUS_KEY, x, y, H([x, y]))`, the pinned two-input sponge, `BUS_KEY = 2` is the input count; a provider row serves only two-input hashes (`chip/poseidon.rs:21-26,53,61,312`). Used only where the operation is exactly that hash.
- Whole-sponge slot (named port, LC AUDIT item 23): `sponge(region, offset, inputs: &[HIn], provide, tamper, perm)` and `sponge_iv(.., iv, ..)`, each returning the digest cell and the next offset (`chip/poseidon.rs:283-307`). Every variable input is a copied cell (`HIn::Cell`); the input count and the capacity word are fixed per call site. A slot is not on the bus; it is 70 rows per permutation in the caller's region (`chip/poseidon.rs:59,123-125`).
- Two families of call sites, never mixed:
  1. Pinned hashes: every digest that the specification defines or that another circuit recomputes. These are the semantic public digests `sherlock-value/1`, `input_d`, `outcome_d`, `claim_d`, `prog_d` and `rules_d` (SRC-001), and the header of the state commitment `pre_d` / `post_d`, which NODE INTEGRATION and L AND WRAP re-hash with the library sponge (kit BOUNDARY COMMITMENT). They use `sponge` (capacity word equal to the input length, as `midnight-circuits` 7.3.0 `hash/poseidon/poseidon_cpu.rs:131-133`), bit for bit (L-3), with the domain word as the first input where the specification puts it there.
  2. Internal hashes: digests that only this leaf circuit computes and that reach the outside only as an opaque word. These are memory records, pad, empty segment, tree nodes (`DOM_REC = 1`, `DOM_PAD = 2`, `DOM_EMPTY = 3`, `DOM_NODE = 4` with the height; `iv(dom, sub) = ((dom << 16) | sub) << 32`, `chip/dense.rs:54-62`), stack chains, the body digest `body_d` of the state commitment, ROM leaves and nodes. They use `sponge_iv` with a domain constant in the capacity word (E-2). The capacity word is fixed by the circuit and differs per domain, height and input length. A domain registry (`hash_domains.rs`) lists every domain once; a new domain is a schema change. Layout of the capacity word (landed, `chip/dense.rs:60-62`; stated in full by the late text of `gemini-area1.md`, section 5.2 B): bits 0 to 31 the input count, bits 32 to 47 `sub` (a height, a table id, a schema version), bits 48 to 63 `dom`; `dom` is never 0, so the word differs from the pinned sponge's bare length. Proposed numbers for the domains that are not landed (from the same text, kept where they do not conflict with a decided input): 5 for a ROM node with `sub = (tbl << 8) | h`, 7 for a ROM row leaf, 8 for the state body (`sub` = the schema version), 9 for the frame chain, 10 for the task chain. Not taken from that table: domain 6 `DOM_TAIL8` (the landed tail uses the node domain with heights 1 to 3, and the Quint lane found no collision, `REPORT-dense.md`, finding F11), domain 11 `SHK_VAL` and domain 12 `DOM_CLAIM` (value, outcome, claim and input digests are pinned hashes, L-3), and the 24-input ROM leaf (the ROM leaf is a row hash under `Tree8`, as the heap line is).
- The circuit sponge refuses fewer than two inputs; the native helper accepts one (`chip/poseidon.rs:108,311`; the one-input sponge would leave `in1` free). The port stays at two or more inputs.

#### Invariants

Guaranteed: correct initial state with the exact length or domain word; absorption order; every round with the pinned constants; chain and odd-tail absorption; the output copied from the final state; provider chip id and key pinned; no internal state accepted as a digest; no provider row with an unconstrained output; a disabled permutation is off the bus (LC AUDIT item 22); every allocated block runs all 68 rounds whether or not a caller uses it (`gemini-area1.md:939`); the capacity words of two distinct registry entries differ and are non-zero, and `PAD`, `EMPTY_SEG` and the two empty-stack constants are checked non-zero at key generation (`gemini-area1.md:965,971`). Assumed from callers: the domain, the canonical encoding of integers and tags, the authentication of any child digest. HASH cannot tell a child digest from an address; VALUE DIGEST owns that.

Collision resistance of the pinned permutation is a premise, not a theorem.

#### Row budget

`rows_for(N) = 70 ceil(N / 2)` for `N >= 2` (`chip/poseidon.rs:123-125`, landed layout). Measured in C1 as parts of larger units: a `Tree8` is 7 compressions (490 rows), a record hash 2 permutations (140 rows). Callers count their slots; HASH has no budget of its own beyond the bus providers CORE needs (per-rule table).

Tags as a first hash input instead of the capacity word would double every tree node (ESTIMATE, flush unit 17,080 rows, cold read 9,100 rows at TOP = 61, `FD/EXPERIMENTS.md` section 5, C1 deviations); not used.

#### Verification

- Quint: new `qnt/hashProtocol.qnt`: call framing, input order, domain separation, copy connections; the hash is an injective abstraction inside the model, stated as a premise.
- MockProver positive: differential vectors against the pinned stdlib for even and odd lengths (landed tests stay); `sponge_iv` against `sponge_native_iv`; zero, repeated and maximal inputs; several calls in one region; a shared two-input provider.
- Named mutants: `hash_state_tampered`, `hash_round_constant_changed`, `hash_round_skipped`, `hash_absorb_skipped`, `hash_length_changed`, `hash_odd_tail_absorbs_padding`, `hash_chip_id_forged`, `hash_key_forged`, `hash_output_copy_removed`, `hash_input_copy_removed`, `hash_domain_swapped` (a `DOM_NODE` call given the `DOM_REC` capacity word), `hash_value_digest_given_capacity_tag` (a `sherlock-value/1` call site must fail the differential vector if it uses `sponge_iv`).

#### Lean obligations

| Obligation | File | Premises | Contract discharged | Status |
|---|---|---|---|---|
| Injectivity of the tagged hashes on distinct `(domain, height, length, inputs)` is the hypothesis every digest theorem uses; it is stated once as a structure of premises, with no `axiom`. The arithmetic part (the capacity word is injective in `(dom, sub, len)` and non-zero, `gemini-area1.md:1011`) is a small provable lemma in the same file | new `lean/HashPremises.lean` (extends the `digest_inj` premise of `lean/Aggregation.lean:201`) | collision resistance | the HASH assumption of MEM, STACKS, VALUE DIGEST, BOUNDARY COMMITMENT | open (premise statement) |

Gates, round constants and layout stay with MockProver and the differential vectors, not Lean.

#### Code skeleton

File: `chip/poseidon.rs` (owned); `chip/hashchip.rs` stays as a calibration reference; new `chip/hash_domains.rs`.

```rust
impl PoseidonCfg {
    /// Pinned sponge over `inputs` (N >= 2); every `HIn::Cell` is copy-constrained; `provide` puts a
    /// two-input hash on the bus.                                                    (landed, :283-291)
    pub fn sponge(&self, region: &mut Region<'_, F>, offset: usize, inputs: &[HIn], provide: bool,
                  tamper: &Tamper, perm: &mut usize) -> Result<(AssignedCell<F, F>, usize), Error>;
    /// The same with a domain constant `iv` added to the capacity word.              (landed in C1, :298-307)
    pub fn sponge_iv(&self, region: &mut Region<'_, F>, offset: usize, inputs: &[HIn], iv: F, provide: bool,
                     tamper: &Tamper, perm: &mut usize) -> Result<(AssignedCell<F, F>, usize), Error>;
}
/// Rows of an N-input sponge.                                                        (landed, :123-125)
pub fn rows_for(n: usize) -> usize;
/// The one list of internal hash domains and their capacity words.
pub enum HashDomain { MemRec, MemPad, MemEmpty, MemNode(u8), FrameChain, TaskChain, StateBody, RomLeaf, RomNode(u8) }
pub fn iv(dom: HashDomain) -> F;
```

#### Agent reads / must not need to read

Reads: this kit, kit BUS, `chip/poseidon.rs`, `chip/dense.rs:54-98` (the landed domain constants), the pinned parameters. Must not need: value traversal, Merkle walks, CORE rules, node code.

#### Dependencies

BUS.

### Kit: CORE

#### Purpose

CORE proves that each enabled step slot is the transition of the reference machine `sherlock-k-a/1` that DEC selected. It carries the persistent registers from slot to slot, issues the typed requests, owns the append chain and the read obligations toward memory, and exports the boundary cells.

#### Interface

State (named port to BOUNDARY COMMITMENT). The full record schemas of `O/design.md:220-267` are used, not the 32-word spike (`chip/rf.rs`): control and origin; outcome; fuel (initial, remaining, consumed work); logical counters (helper work, visits, cursor, allocation counters); `stack_limit`; `frame_count`, `task_count`; environment reference; the cached top frame (13 lanes), top task (7 lanes), helper (23 lanes), result (4 lanes) (`astra-area1.md:332-352`, citing `O/design.md:241-244`); `frames_below`, `tasks_below` (from STACKS); the memory length `len` (copied from the MEM frontier); `prog_d`, the program root `prog_root`, `input_d`; the dependency commitment `deps_d` (GP-2 hook). `prog_d`, `prog_root` and `input_d` are in no rule's write set; INIT fixes them at genesis. The expansion into lanes is frozen in `core/state_schema.rs`. The 41-cell state with a 6-cell packed helper of `gemini-area1.md:313-322` is not adopted; packing is a later layout optimisation of the same schema. The lane count is not known until the schema is frozen (section 7).

Requests on the bus (two lanes per request row, as landed; a rule that needs more requests uses more rows). CORE is the only bus requester of the leaf. The request inventory is complete: a rule's `EffectBounds` counts every item of this list, and no other request exists (B2 finding 11).

- `(ALU, op + 8 flag, x, y, z)` for arithmetic, comparisons, fuel and counter updates, `frame_count < stack_limit`.
- MEM read: `(MEM, 8 addr + kind, a, b, c)` for a record read that the window or a heap line serves, issued only by the read port below. A read served by the scalar `cold_read` uses the `ColdRead` cells, not a bus request.
- MEM append: `(MEM, 8 addr + kind, a, b, c)`, one request per append event, with the request enable equal to the append's `act` (condition A4 below). It is served by the new window row at that address. An append therefore takes one request slot of a bus lane, exactly as a read does.
- `(ROM, rom_key, a, b, c)` for program, type, constructor and constant rows, issued only by the read port (`read_rom`).
- `(HASH, BUS_KEY, x, y, h)` where a rule needs exactly a two-input hash.

Read port (one shared authenticated-read interface for every consumer of a record; B2 finding 4; Quint lane condition C3, `qa/gemini-quint/c1-dense/REPORT-dense.md:90-98`). `core/ports.rs` owns it. CORE rules, VALUE DIGEST and INIT obtain record and ROM cells only as its return values `AuthRec` and `AuthRom` (section 1.3); a module that needs a child record asks CORE's rule emitter for the token and never issues a request of its own.

`read_record(addr_src, kind_expected, at)` constrains, for every read:

1. `cold` is boolean.
2. `diff = cold (base - 1 - addr) + (1 - cold) (addr - base)` is decomposed into `TOP + 3` bits, with `base = 8 q_pre` copied from the `Window`. So `cold = 1` exactly when `addr < base` (mutant `cold_route_forged`).
3. `d2 = len_cur - 1 - addr` is decomposed into `TOP + 3` bits, so `addr < len_cur`. `len_cur` is the cell `len_at[at]` of the append chain: the running length after the appends that precede this read point in the rule's micro-operation order, and `len_pre` plus the appends of the earlier slots. It is never `len_post` by default. The C1 stand-in offered only `len_pre` or `len_post` (`chip/dense/tests.rs`, `at_post`); the real port takes the chain position (REPORT C3 item 7; `FD/EXPERIMENTS.md`, "Not covered and open"; mutants `future_record_read`, `mem_read_between_two_appends`).
4. `kind == kind_expected`, a constant of the call site in `1..=4`; `key = 8 addr + kind`.
5. `addr` is copy-linked to `addr_src`, which is a register lane or a cell that an earlier micro-operation of the same slot constrained. It is never a free provider payload (mutants `core_read_address_free`, `vd_child_wrong_reference`).
6. Route of an old read: `line` is boolean and `line (1 - cold) = 0`. `line = 1` means that a heap line of this leaf serves the record on the bus; `line = 0` with `cold = 1` means the scalar cold read. `cold` keeps its meaning (`addr < base`) in both cases.
7. Bus request: enabled by `req = (1 - cold) + cold line`, a boolean; the request is `req (MEM, key, a, b, c)`. It is served by a window row when `cold = 0` and by a heap line row when `cold = line = 1` (the two provider classes are disjoint by `addr >= base` against `blk < q_pre`).
8. Scalar cold read: `ColdRead` cells with `enable` copied from `cold (1 - line)`, `pre` the token of the leaf's pre state, and `[addr, kind, a, b, c]` copied from the port (`chip/dense.rs:725-762`).
9. Exactly one of the three routes authenticates every read: `req + cold (1 - line) = 1` follows from items 1 and 6, so no read is left without a provider or an opening (mutant `core_read_old_record_unrouted`).

Items 1 to 5, 7 and 8 are condition C3 of the Quint lane; the C1 stand-in had no lines, so there `line = 0` and items 7 and 8 are its warm and cold paths.

`read_input(src, k)` (input admission form (b), AD-17) returns an `AuthIn` token: the record at index 0, or the record that field `k` of the earlier token `src` points to. It opens the child-vd leaf `src.child_pos + k` and, if the child is entered, the child record, each by one Merkle path of depth `D_view` against the `view_root` body word, and requires `k < src.m` (RANGE). It adds no MEM append and charges no DataNodes or DataFields cap. If X11 shows that the reference charges `Literal(x)` cells for an input read, the charge is a constant per read and the rule contract states it (mutant `in_read_outside_view`: a record that is not under `view_root`, or an index not reached from the root).

`read_rom(tbl, row: RangedCell<D_row>)` issues `(ROM, tbl 2^D_row + row, a, b, c)`; `tbl` is a constant of the call site, and the row is accepted only with its RANGE handle, so `(tbl, 2^D_row)` cannot alias `(tbl + 1, 0)` (PAGE/ROM item C-2; mutants `rom_row_overflow_alias`, `vd_rom_row_overflow`).

Named ports:

- DEC port (kit DEC). RANGE port for every new or updated integer cell.
- Append chain to MEM (the landed hybrid transport; Quint lane condition C1, `REPORT-dense.md:79-85`; LC AUDIT item 12). Per leaf, the ordered append events `(act, addr, kind, a, b, c)` of all slots, in slot order and inside a slot in the apply order of the reference (`owner_apply_validated`: data fields, data node, data field, constructor, env, slot; `FD/EXPERIMENTS-MAXIMA.md`, premise S1). The four conditions:
  - A1 (start): with no append in the leaf, `len_post` is copy-equal to `len_pre`; otherwise the address cell of the first append is copy-equal to `len_pre`.
  - A2 (unit step): the address of append `j + 1` is the `WindowCfg::inc` output of append `j` (`next = cur + 1`, `chip/membackend.rs:360-368`). An inactive append position passes the address through.
  - A3 (end): the `inc` output of the last active append is copy-equal to `len_post`.
  - A4 (value and kind): each active append issues the MEM append request of the list above with `kind` equal to the constant of the rule's record kind, in `1..=4`.
  - Consequence (not a separate gate): the active non-imported window rows are exactly the addresses `len_pre .. len_post - 1`, one row per address, and A1 to A4 give one request at each of these addresses; the key is injective, so append events and new window rows are in bijection, and each row holds the appended record (mutants `close_advances_len`, `append_value_mismatch`, `core_append_without_window_row`, `core_append_order_reversed`). Closing a leaf advances nothing.
  - Across leaves, `(len_post, root_post, tail_post)` of one leaf equals `(len_pre, root_pre, tail_pre)` of the next through `post_d == pre_d` (BOUNDARY COMMITMENT and NODE INTEGRATION), which is the stitch that C1 requires.
- Read port to MEMORY BACKEND: the `ColdRead` cells of the read port above.
- STACKS port: one `StackStep` per stack per slot: an optional rewrite of the top, then one of none, push, pop, in that order, as one owner event (kit STACKS; B2 finding 2). The reference applies `task_rewrite_top` before `TaskOp::Push` or `Pop` (`crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:3566-3577`), and `BHelperValidateField` and `BHelperEqualField` set both in one step (`families/structural.rs:97-102`, `:188-194`). A slot is never split in two to express this; the step count is the owner-event count.
- VALUE DIGEST port: when a rule constructs or publishes a value. The rule passes `AuthRec` and `AuthRom` tokens for every child record, type row and constant row that the digest uses.
- Public cells: `steps = sum enable`, `h_pre`, and `rules_d`.

`rules_d` (B2 finding 12; SRC-001 and SRC-003, `O/requirements.md:26,28`; `O/design.md:42`). CORE owns the computation and the public equality. It is not a naming question.

- `rid_j = sum_r sel_j[r] lane(r)` is the `rule_lane` cell that DEC returns for slot `j`. A disabled slot has `rid_j = 0` (the DEC idle row), and 0 is the lane of no rule.
- `rules_d = Poseidon(dom("SHK_RULES"), rid_0, ..., rid_(S-1))` over exactly S slot lanes in slot order, in one whole-sponge HASH slot of the pinned family (`S + 1` inputs, each `rid_j` a copied cell). There is no special case for S = 1 or for one enabled slot.
- Leaf public lane 5 is copy-equal to the digest cell. The landed spike exports a single `rule` lane (`crates/mori-recursion/src/ivc.rs:70-74`); the leaf of this design exports `rules_d`, and the rename of the lane in `ivc.rs` is part of the NODE INTEGRATION task. The lane count and order of `sherlock-leaf-stmt/1` do not change.
- Rows: `70 ceil((S + 1) / 2)`, that is 630 rows at S = 16 and 70 at S = 1 (ESTIMATE from the landed 70 rows per permutation).

Provides no bus tuple.

#### Invariants

Guaranteed, per slot (`astra-area1.md:384-403`, `gemini-area1.md:362-376`):

1. `enable` is boolean; enables form a prefix; the first slot is enabled (`1 <= steps <= S`, SRC-014); every micro-operation of a slot uses the same enable.
2. With `enable = 1` DEC selects exactly one rule; every register in the rule's write set has its specified next value; every other persistent register is unchanged; the slot's post cells are copied to the next slot's pre cells.
3. A disabled slot is the identity on persistent state, has every request lane zero, queries the DEC idle row (so its `rid` is 0) and adds no step.
4. No enabled step starts from a terminal outcome; a publishing step is the last enabled step.
5. One enabled slot adds one to the step count; a micro-operation is a scheduling unit and adds nothing. `h_post = h_pre + steps`, both below 2^64 (SRC-004, `crates/mori-recursion/src/ivc.rs:79,363-376`).
6. Each rule's exact charge behaviour. The spike's "one fuel per enabled step" is not copied (`astra-area1.md:401`).
7. Closing a leaf changes no counter, fuel, outcome or length. A step that does not fit the leaf's remaining capacity is not enabled; this never produces `Rejected` or `Exhausted` (SVM-010).
8. Scratch `v0..v2` (R-8, RL AUDIT finding 9, B2 finding 1). The property is independence from incoming scratch: for every slot, the persistent post state, every request and every port cell are a function of the persistent pre state and the authenticated inputs, and do not depend on the value a scratch lane has when the slot begins. It is enforced by ordered production and consumption, in the style of single static assignment:
   - A scratch value is produced by a named micro-operation of the rule and consumed only by later micro-operations of the same slot. Each consumer's source cell is copy-linked to (or is the same cell as) its producer's output cell.
   - No constraint of any rule, other than the next-state constraint of the scratch lane itself, reads the incoming cell `Pre(v_k)` of a slot. This is the landed check: `m3_scratch_registers_are_write_only` walks every core constraint and fails if one reads `Pre(v_k)` outside `next v_k` (`chip/rfcore.rs:1731-1759`). It is stronger than "the selected rule writes `v_k` somewhere in the slot": a rule that copies `Pre(v0)` into a persistent register and writes `v0` afterwards passes the weaker form and fails this one.
   - `SelectorPlan::freeze` and `check_definite_assignment` refuse, before key generation, a rule plan in which a scratch use precedes its definition in the micro-operation order, or in which a use is not linked to a definition of the same slot.
   - No constraint links scratch across slots or across leaves, and scratch is outside the boundary schema. Zeroing scratch at slot 0, at a leaf boundary, or when the selected rule does not write it is not the enforcement and is not accepted as a substitute; INIT still sets `v0..v2 = 0` at genesis because SRC-021 lists field 22, and that value is never read.
9. Unused request lanes are zero in all five cells; an active request fixes a non-zero chip id (item 13).
10. `rules_d` is the SRC-001 hash of the S slot lanes in slot order; a disabled slot contributes 0.
11. Every record or ROM row that a rule, VALUE DIGEST or INIT uses was returned by the read port; every append satisfies A1 to A4.

Assumed: BUS soundness; the DEC contract; valid ALU, MEM, ROM and HASH tuples; the STACKS and VALUE DIGEST port certificates; kit INIT establishes the canonical state and the range invariants at `h_pre = 0` (an earlier leaf's `post_d` does so otherwise); BOUNDARY COMMITMENT binds the first pre cells and the last post cells.

Capacity rule (GP-1): every rule publishes its maximum bus requests (ALU, MEM reads, MEM appends, ROM, HASH, each counted), appends in physical cells, distinct old segments read, distinct ROM blocks per table, stack hashes, range checks and whole-sponge hashes in one step. Two obligations are kept apart (B2 findings 8 and 9):

- Single-step fit: the leaf's capacities fit the largest single step of every rule, over every branch of the rule, at one step per leaf and at every tail occupancy `r_pre` in `0..=7`. For appends this is `max appends per step <= 8 W`, because the window holds `8 W + 7 - r_pre` new records. Early close cannot repair a step that does not fit an empty leaf (RL AUDIT finding 6, LC AUDIT blocker 1).
- Cumulative leaf fit: the appends of all enabled slots of a leaf are at most `8 W + 7 - r_pre`, and likewise for every other capacity. The window gate enforces the append bound (the last window row is inactive, `chip/membackend.rs:23-26`); a prover closes the leaf early when the next step would exceed a capacity. Early close is a prover rule, not a gate, and changes no counter (invariant 7).

Status of the table (experiment X1, `FD/EXPERIMENTS-MAXIMA.md`, `FD/maxima/rule_maxima.md`, decisions M-1 to M-4): 120 enabled step rules; at most 3 appended cells per step (STATIC = OBSERVED, `BHelperScalarExit`), so single-step fit holds at `W = 1`; at most one frame operation, or one task push or pop with one rewrite of the top, and never a frame and a task operation in one step (STATIC); at most 4 distinct heap records read per step (STATIC for 116 rules, code reading for 4). The maxima are derived from the rule source over all branches and validated by the corpus run; an observed corpus maximum alone is never a bound. Not yet bounded: ROM rows and ROM blocks per step (`K_rom`), ALU and range requests per rule, and the physical cells of a logical record (AD-11).

#### Row budget

The landed spike slot is 13 rows for 32 words on 12 advice columns (`chip/rfcore.rs:215`); the E8 leaf without memory is 421 rows at S = 16, 26.3 rows per step (`FD/EXPERIMENTS.md` section 5, C1 leaf rows, composed). ESTIMATE for the full schema: `2 ceil(words / 6) + 1` rows per slot plus operand routing, request rows and rule-local rows; a state of 128 to 256 words gives 45 to 87 base rows per slot (`astra-area1.md:452-460`). The 7-row slot with four requests of `gemini-area1.md:326-340` is not adopted (L-8); it remains a layout experiment with the acceptance "no new column, all rules placed" (LC AUDIT finding 11). ESTIMATE: the `rules_d` sponge, 630 rows per leaf at S = 16; the read port, 134 rows per read at TOP = 63 (MEASURED for the C1 stand-in, `FD/EXPERIMENTS.md` section 5, C1 measured rows); one request slot per append (up to 3 per step), which is half a request row each on the landed two-lane layout.

#### Verification

- Quint: new `qnt/processorCore.qnt`, reusing the slot obligations of `qnt/chipLeaf.qnt` and the stack obligations of `qnt/chipLeafCached.qnt`, with rule-family transition contracts in place of the stand-in instruction set. Properties: each enabled slot projects to one owner transition; closure projects to the identity; a change of partition preserves the logical trace, the accounting and the outcome; every append event is one reference allocation; the post state of a slot is independent of its incoming scratch (two runs that differ only in incoming scratch agree on every persistent register and request); a compound stack step is one owner event; `rules_d` is a function of the selected rules in slot order.
- MockProver positive: one instance of every enabled rule and every branch of its charge behaviour; full and partial leaves; a zero-allocation loop (no address consumed); halted, rejected and exhausted runs; the same trace under full, one-step and mixed partitions with equal post state and `n_steps`; `core_height_edge` (`h_pre = 2^64 - 2`, one step); `core_scratch_incoming_free` (a free value in every scratch lane at every slot entry changes neither acceptance nor any public lane; the landed form is the second half of `m3_scratch_registers_are_write_only`); `core_rewrite_then_push_one_slot` (`BHelperValidateField` and `BHelperEqualField` on the non-tail branch); `core_read_between_two_appends` (a read at the running length); `core_rules_d_reference_vectors` (S = 16 full, S = 16 with a disabled suffix, and an S = 1 key shape, against a host reference written from SRC-001).
- Named mutants: `core_selector_forged`, `core_enable_split_across_microops`, `core_enable_nonprefix`, `core_zero_step_leaf`, `core_post_pre_copy_removed`, `core_unlisted_register_changed`, `core_disabled_state_changed`, `core_terminal_followed_by_step`, `core_steps_counts_microops`, `core_close_charges_fuel`, `core_close_advances_length`, `core_height_wrap_2_64`, `core_future_read`, `core_read_kind_unconstrained`, `core_read_address_free`, `core_append_address_free`, `core_append_order_reversed`, `core_append_without_window_row`, `core_unused_lane_nonzero`, `core_value_digest_host_supplied`; for `rules_d`: `core_rules_d_forged` (a public lane 5 that is not the hash), `core_rules_d_lanes_reordered`, `core_rules_d_disabled_lane_nonzero`, `core_rules_d_s1_shortcut` (at S = 1 the single lane published without the hash); for the read port: `core_read_len_post_instead_of_running`, `core_read_cold_bit_forged`, `core_read_old_record_unrouted` (an old read with the bus request and the scalar cold read both off; the control is a port whose `ColdRead.enable` is a free bit), `core_read_line_bit_on_window_address` (`line = 1` with `cold = 0`); for appends: `core_append_request_disabled` (an active append with its bus request off), `core_append_kind_free`.
- Scratch, stated in the mutant protocol of the conventions. The witness: a slot whose incoming scratch `v0` holds a value the prover chose. In the correct circuit this witness is accepted and changes nothing (`core_scratch_incoming_free`), so there is no refusal to observe there; the refusals are at configuration time: `core_plan_scratch_use_before_def` (a rule plan that reads `v0` and writes `v0` later in the same slot; `check_definite_assignment` refuses it) and `core_plan_reads_pre_scratch` (the constraint walk finds a read of `Pre(v_k)`). The control circuits, each of which must show the attack: `ctl_scratch_read_then_write` (a rule that copies `Pre(v0)` into a persistent register and then writes `v0`: two witnesses that differ only in incoming scratch give different `post_d`, both accepted) and `ctl_scratch_cross_slot_link` (`v0` carried across a slot boundary: the same). The audit's mutant "read scratch, then write that scratch later in the same slot" is `ctl_scratch_read_then_write` with `core_plan_scratch_use_before_def`.

#### Lean obligations

| Obligation | File | Premises | Contract discharged | Status |
|---|---|---|---|---|
| If CORE's contract gives the goal from valid requests, an accepted leaf satisfies the goal (`leaf_refines`, `leaf_refines_balanced`) | `lean/ModularComposition.lean` | module contracts, bus soundness | modular composition, generic | proved in C3 |
| If CORE's decoded rows follow the reference step function, an accepted leaf is a `Run` of exactly `rows.length` steps (`leaf_refines_step`, `follows_run`, `fun_det`) | `lean/ModularComposition.lean` | the hypothesis that rows follow the step | shape of `leaf_sound` | proved in C3 |
| CORE's concrete contract: with valid requests, the enabled slots follow the reference step over the full rule set, a disabled slot is the identity, and at least one slot is enabled | new `lean/CoreRefinement.lean` | contracts of DEC, ALU, MEM, ROM, STACKS, VALUE DIGEST; hash premises | the hypothesis of `leaf_refines_step` | open |
| `leaf_sound` of `Premises` derived from the composition theorem and the concrete contracts | new `lean/LeafSound.lean` | all module contracts, bus soundness, `digest_inj` | `Premises.leaf_sound` (`lean/Aggregation.lean`) | open |
| Independence from incoming scratch: for a slot relation built from an ordered micro-operation plan that passes definite assignment, two witnesses with equal persistent pre state and equal authenticated inputs have equal persistent post state, requests and port cells, whatever their incoming scratch (stated as non-interference, not as "depends on the full pre-state") | new `lean/ScratchIndependence.lean` (the C3 model has no scratch registers) | none | invariant 8; the reason scratch may stay outside `pre_d` | open |
| `rules_d` binds the selected rules: equal `rules_d` implies equal slot lane lists | `lean/CoreRefinement.lean` | hash premises (pinned sponge), distinct non-zero rule lanes | SRC-001, SRC-003 | open |
| Append bijection: A1 to A4, the window relation and `LookupSound` imply that the append events of a leaf and its new window rows are in bijection with equal records | new `lean/MemContract.lean` | key injectivity (`kind` in `0..=4`) | LC AUDIT item 12; Quint lane condition C1 | open |

#### Code skeleton

Files: `chip/rf.rs`, `chip/rfcore.rs` (owned, refactor); new `chip/core/mod.rs`, `core/state_schema.rs`, `core/slots.rs`, `core/ports.rs`, `core/rules/<family>.rs`.

```rust
/// The one leaf shape of the design (section 1.5).
pub struct LeafShape { pub s: usize, pub w_flush: usize, pub k_page: usize, pub k_rom: usize, pub n_cold: usize }

pub struct RuleContract {
    pub rule: RuleIdC,
    pub reads: &'static [RegisterId],
    pub writes: &'static [RegisterId],
    pub plan: &'static [MicroOp],   // ordered; each scratch use names its earlier definition
    pub effects: EffectBounds,      // max ALU, MEM-read, MEM-append, ROM and HASH requests; appended physical
                                    // cells; old segments; ROM blocks per table; stack hashes; over all branches
}
/// Refuse a plan with a scratch use before its definition in the same slot, or a use with no definition.
pub fn check_definite_assignment(plan: &[MicroOp]) -> Result<(), ConfigError>;
/// The only constructors of AuthRec and AuthRom (section 1.3).
pub fn read_record(ctx: &mut RuleCtx<'_>, addr_src: &Cell, kind_expected: u64, at: ChainPos) -> Result<AuthRec, Error>;
pub fn read_rom(ctx: &mut RuleCtx<'_>, tbl: u64, row: &RangedCell) -> Result<AuthRom, Error>;
/// The only constructor of AuthIn (input admission form (b), AD-17): the input root, or field k of `src`,
/// opened against the view_root body word. Ensures: no MEM append, no cap charge.
pub fn read_input(ctx: &mut RuleCtx<'_>, src: Option<&AuthIn>, k: &RangedCell) -> Result<AuthIn, Error>;
/// One append event: address from the chain (A1 to A3) and the MEM append request (A4).
pub fn append_record(ctx: &mut RuleCtx<'_>, kind: u64, rec: [&Cell; 3]) -> Result<ChainPos, Error>;
/// rules_d = Poseidon(dom("SHK_RULES"), rid_0, ..., rid_(S-1)); returns the lane-5 cell.
pub fn rules_digest(ctx: &mut RegionCtx<'_>, pc: &PoseidonCfg, rids: &[Cell]) -> Result<Cell, Error>;
pub trait RuleEmitter {
    /// Constrain one reference rule using only declared ports and write lanes.
    fn constrain(&self, ctx: &mut RuleCtx<'_>, selected: &Cell, pre: &CoreStateCells, post: &CoreStateCells) -> Result<(), Error>;
}
pub struct CorePorts {
    pub pre: CoreStateCells, pub post: CoreStateCells,
    pub steps: Cell, pub h_pre: Cell, pub rule_lanes: Vec<Cell>, pub rules_d: Cell,
    pub appends: Vec<AppendCells>, pub len_at: Vec<Cell>, pub cold_reads: Vec<ColdRead>,
    pub stack_steps: Vec<StackStepCells>,
}
impl CoreConfig {
    /// Fixed slot geometry and the complete enabled inventory.
    pub fn configure(meta: &mut ConstraintSystem<F>, shape: LeafShape, inv: &RuleInventory, ports: &SharedPorts) -> Result<Self, ConfigError>;
    /// Assign linked slots, all effects and the boundary copies.
    pub fn assign(&self, l: &mut impl Layouter<F>, w: Value<CoreWitness>) -> Result<CorePorts, Error>;
}
/// The complete write set and equality of all other registers.
pub fn constrain_frame_condition(ctx: &mut RuleCtx<'_>, c: &RuleContract, selected: &Cell, pre: &CoreStateCells, post: &CoreStateCells) -> Result<(), Error>;
```

#### Agent reads / must not need to read

The CORE framework agent reads: this kit, sections 1.2 and 1.3, the state schema, `chip/rf.rs`, `chip/rfcore.rs`, `chip/vsel.rs`, kit DEC's `DecodeCells`. A rule-family agent reads in addition its own reference family in `crates/mori-core/src/sherlock/helper_machine/`, the existing fragment in `crates/mori-backend/src/sherlock_machine/` and its tests. Neither needs: Merkle walks, `chip/dense.rs`, LogUp equations, boundary hashing internals, `chip/poseidon.rs` gates, `crates/mori-recursion`.

CORE is the one kit that is more than one session: the framework is one kit, and each rule family is a sub-kit with the same nine items generated from its `RuleContract` (read set, write set, ordered plan, charge behaviour, effect bounds). The families are those of the reference (`family_of_rule`; the publications, the nine helper groups Scalar to Match, the legacy sites, the entry rules, and the ADT Build and Plan groups; `FD/EXPERIMENTS-MAXIMA.md`, "Rule universe"); section 5.2 gives the spec outline of a family sub-kit and section 5.3 one task per family. A family sub-kit is complete only against the REFERENCE BRIDGE kit, which supplies the semantic reference for its rules.

#### Dependencies

BUS, RANGE, DEC, ALU, HASH (interfaces only); STACKS, MEM window, PAGE/ROM, VALUE DIGEST, INIT (ports). The frozen rule inventory and the per-rule table (X1, done for appends, stack operations and heap reads; ROM open). REFERENCE BRIDGE for the rule families.

### Kit: STACKS (FRAMES and TASKS)

Split from CORE because the hash-chain protocol has its own model, mutants and theorem, and it replaces a protocol (D7) that the spec models differently.

#### Purpose

STACKS keeps the frame stack and the task stack as a cached top record in registers plus one Poseidon hash chain for the records below it (R-3). Stack records never enter random-access memory.

#### Interface

Named port from CORE, per stack and slot: one compound step `StackStep { rw, rw_top, op in {none, push, pop}, new_top }` (B2 finding 2). It is one owner event with two ordered parts:

1. If `rw = 1`, the top is replaced by `rw_top` (a rewrite of the pre-image top).
2. Then `op` is applied to the stack that part 1 left.

This is the order of the reference: `task_rewrite_top` is applied before `TaskOp::Push` or `TaskOp::Pop` (`owner_runtime.rs:3566-3577`). The legal combinations are `rw` alone, `op` alone, and `rw` then `push` or `pop`. The reference uses rewrite-then-push on the task stack (`BHelperValidateField`, `BHelperEqualField`: the parent task's cursor advances and the child task is pushed, `families/structural.rs:86-106`, `:169-198`); rewrite-or-pop are alternative branches of `BHelperValidateOccurrence` and `BHelperEqualPair`. The frame stack has one operation per step (`FrameWrite::{Push, RewriteTop, Pop}`, `shared.rs:1768-1772`), which is the same port with `rw` and `op` never both set. No rule has a frame and a task operation in one step (`FD/EXPERIMENTS-MAXIMA.md`, STATIC). The port is the same for both stacks; the per-rule table says which combinations a rule uses.

Named cells to CORE, BOUNDARY COMMITMENT and INIT: `count`, `below`, `top[..]` (13 lanes for a frame, 7 for a task). Requests: at most one whole-sponge slot of HASH per stack per step (one spill or one restore; a compound step never needs two); RANGE for `count`. Provides no bus tuple.

Chain: `below' = H_chain(count, below, top[0], ..., top[n-1])` on a push onto a non-empty stack, with a capacity domain word per stack (`FrameChain`, `TaskChain`); the depth `count` is hashed, so a restored record is bound to the depth at which it was spilled (`astra-area1.md:425-444`). Astra puts the domain in the first input and Gemini uses a two-input bus hash over a packed top (`gemini-area1.md:372`); the merge uses the capacity word (E-2 form for internal hashes) and a whole-sponge slot, because a 13-lane frame is not a two-input hash and packing it into one field element is not shown to be injective.

Protocol:

Let `top*` be the effective top after part 1: `rw_top` if `rw = 1` and `count >= 1`, else `top`.

- `count = 0`: the cached top is zero and `below = EMPTY_F` (or `EMPTY_T`), a fixed non-zero constant per stack.
- Push at `count = 0`: install the new top; no hash. Push at `count >= 1`: spill `top*` into `below`, that is `below' = H_chain(count, below, top*[0], ..., top*[n-1])`, and install the new top. With `rw = 1` the spilled record is the rewritten parent, so the cursor update is kept and the child is pushed in the same step.
- Pop at `count = 1`: clear the top; `below` stays `EMPTY`. Pop at `count >= 2`: open `below` to the next top, the predecessor hash and `count - 1`. A rewrite before a pop is discarded with the popped top.
- `rw` with `op = none` on a non-empty stack changes only the top; no hash.
- At `count = 0` the relation is the list function of the reference: a rewrite and a pop of an empty stack change nothing (`owner_runtime.rs:3567-3575`: `last_mut()` and `Vec::pop` on an empty list). Whether a rule can reach these cases, and what outcome it then publishes, is decided by the rule's guards in CORE and DEC; STACKS does not make a slot unsatisfiable for them and does not choose an outcome.
- `frame_count <= stack_limit` is a CORE check through ALU.

The cached-top abstraction (defined here because the Lean theorem of C3 is about another object). A physical stack is `(count, top, below)`. Its abstraction is the list `[]` when `count = 0`, and `top :: rest` when `count >= 1`, where `rest` is the list of `count - 1` records whose depth-indexed chain is `below`: `chainD([]) = EMPTY`, `chainD(x :: l) = H_chain(|l| + 1, chainD(l), x)`. So `below` is the hash of the suffix under the cached top, not of the whole stack, and the canonical empty suffix `below = EMPTY` belongs to both `count = 0` and `count = 1`.

#### Invariants

Guaranteed: the count is the rule. `count = 0` implies `below = EMPTY` and a zero top. `count = 1` implies `below = EMPTY`, by construction (a push at `count = 0` does not hash, and a pop at `count = 2` restores the predecessor, which is `EMPTY`), with no collision assumption. The converse, `below = EMPTY` implies `count <= 1`, holds only under the premise that `EMPTY` is outside the range of `H_chain`; it is not enforced by a gate (RL AUDIT finding 8). The statement "`below = EMPTY` only at `count = 0`" of the B1 text was false and is withdrawn (B2 finding 2). A compound step equals the reference's rewrite-then-operation on the abstract list, in one slot and one owner event. A restored record equals the record that was spilled at that depth, under the injectivity premise. Frame and task chains cannot be confused (different domains). Every restored integer lane gets its canonical record checks again. This protocol is not D7: D7 links frames by an address in memory (`O/design.md:100`), and only that form has a Quint model; a new model is a condition of adoption (RL AUDIT finding 8).

Assumed: HASH; CORE supplies one `StackStep` per stack per slot with the combination its rule declares; INIT sets both stacks empty at genesis.

#### Row budget

ESTIMATE, 70 rows per permutation: frame spill or restore 15 inputs, 8 permutations, 560 rows; task spill or restore 9 inputs, 5 permutations, 350 rows; rewrite 0; rewrite-then-push one spill (of the rewritten top), so the compound step costs what a push costs. Per step at most one stack hash in total: one frame operation, or one task push or pop with one rewrite, never both stacks (X1, `FD/EXPERIMENTS-MAXIMA.md`). The number of spill units per leaf is at most S. Hash-chain stacks are not built (`FD/EXPERIMENTS.md`, "Not covered and open"); the C1 leaf tables exclude them.

#### Verification

- Quint: new `qnt/hashStacks.qnt` (push, pop, rewrite, rewrite-then-push and rewrite-then-pop as one event, the empty rule, the `count = 1` suffix rule, depth binding, refinement of the list stack of the reference machine, `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:3566-3591`). Invariants: `cachedTopRefinesList`, `compoundStepIsOneEvent`, `belowEmptyAtCountLe1`. It replaces the memory-backed stack reference of `qnt/chipLeafCached.qnt` for this protocol.
- MockProver positive: nested push, rewrite and pop on both stacks; push and pop across a leaf boundary; `count` 0 to 1 to 0 with no hash; `stack_rewrite_then_push_one_step` at `count = 1` and at `count >= 2` (the restored parent after the child's pop carries the advanced cursor); `stack_rewrite_then_pop_one_step`; `stack_count_one_below_empty`; rewrite and pop at `count = 0` leave the stack empty, as the reference list function does.
- Named mutants: `stack_compound_spills_unrewritten_top` (rewrite-then-push whose spill hashes the old top, so the cursor update is lost), `stack_compound_child_not_installed` (rewrite-then-push that keeps the rewritten parent as the top and drops the child), `stack_compound_count_plus_two`, `stack_count_one_below_nonempty` (`count = 1` with `below != EMPTY`), `stack_empty_nonzero_top`, `stack_empty_below_not_empty_const` (`count = 0`, `below = 0`), `stack_predecessor_forged`, `stack_restored_record_forged`, `stack_count_unchecked` (a record restored at the wrong depth), `stack_frame_task_domain_swapped`, `stack_rewrite_spills`, `stack_push_without_spill`, `stack_count_wrap`.

#### Lean obligations

| Obligation | File | Premises | Contract discharged | Status |
|---|---|---|---|---|
| The hash of a complete list determines the list if `H` is injective and `EMPTY` is not in its range (`chainHash_inj`) | `lean/PartitionIndependence.lean:251-261` | the two collision premises | a lemma only. It is about `chainHash` of the whole list with no depth word; the circuit hashes the suffix below a cached top with the depth. It does not by itself give "restored record equals spilled record" (B2 finding 2) | proved in C3 |
| Partition independence with `frames` and `tasks` in the logical state (`partition_independent`), for an `Effect` with exactly one `StackOp` per stack (`keep`, `push`, `pop`, `rewrite`; `lean/PartitionIndependence.lean:228-240`, `:261-265`) | `lean/PartitionIndependence.lean:404` | none | SRC-023 for a machine with one stack operation per step. The reference's rewrite-then-push is not one `StackOp`, so this theorem does not yet cover the task stack of the real machine | proved in C3 (for the single-operation model) |
| Compound stack effect: `Effect` carries, per stack, an optional rewrite followed by an operation; `partition_independent` and `leaf_tail_commutes` for that effect | `lean/PartitionIndependence.lean` (extend) | none | SRC-023 for the stack part of the real machine | open |
| The cached-top protocol refines the list: for the abstraction `absCached(count, top, below)` of this kit, every `StackStep` (rewrite, operation, and their ordered composition) maps to the reference list function; `count = 1` implies `below = EMPTY`; a restore returns the record spilled at that depth | new `lean/HashStacks.lean` | depth-indexed chain injective and `EMPTY` outside its range | STACKS contract toward CORE | open |

#### Code skeleton

File: new `chip/core/stacks.rs` (owned).

```rust
pub struct StackCells { pub count: Cell, pub below: Cell, pub top: Vec<Cell> }
/// One owner event: optional rewrite of the top (`rw`, `rw_top`), then `op` in {none, push, pop}.
pub struct StackStepCells { pub rw: Cell, pub rw_top: Vec<Cell>, pub op: Cell, pub new_top: Vec<Cell> }
impl StackCfg {
    /// Gates of one stack with `lanes` top lanes and the given chain domain.
    pub fn configure(meta: &mut ConstraintSystem<F>, ports: &SharedPorts, lanes: usize, dom: HashDomain) -> Self;
    /// One compound step: the rewrite, then the operation on the rewritten top; the count rule, the
    /// empty rule, and at most one spill or restore through one HASH slot. A push spills the rewritten top.
    pub fn step(&self, pc: &PoseidonCfg, r: &mut Region<'_, F>, o: &mut usize, st: &StackStepCells, pre: &StackCells) -> Result<StackCells, Error>;
}
```

#### Agent reads / must not need to read

Reads: this kit, kit HASH (slot port), `qnt/chipLeafCached.qnt`, `owner_runtime.rs:3566-3591`, `shared.rs:1768-1778` (`FrameWrite`, `TaskOp`), the stack columns of `FD/maxima/rule_maxima.md`. Must not need: memory, DEC, rule bodies, recursion.

#### Dependencies

BUS, RANGE, HASH.

### Kit: MEM window

The landed C1 interface is the MEM interface (L-4). This kit describes what is in `chip/membackend.rs` and what is added; it does not redefine it.

#### Purpose

MEM is the in-leaf part of dense tail-buffered memory (R-3): it decomposes the two lengths, lays out the window of imported tail records and new records, serves them on the bus, and hands the window to the backend. It does not know how old segments are stored.

#### Interface

The memory state is `(len, root, tail_d)` (`chip/dense.rs:5`). MEM owns `len` (`len_pre`, `len_post`); the backend token holds `[root, tail_d]` and MEM never reads it. One arena with kind-tagged records (L-9).

- Provides `(MEM, 8 addr + kind, a, b, c)` for every active window row (`chip/membackend.rs:11,56-58`). `kind` is in `0..=4` (a degree-5 gate) and equals the fixed `kfix` when `kfix != 0` (`chip/membackend.rs:28-30,155`); with one arena `kfix = 0` on window rows.
- Window: `8 (W + 1)` provider rows at addresses `8 q_pre + i`, `q_pre = floor(len_pre / 8)` (`chip/membackend.rs:7-8`). `imp` (imported tail) and `act` (active) are boolean prefixes with `imp <= act`; the `imp` prefix ends at `len_pre` and the `act` prefix at `len_post`; the last row is inactive, so a leaf holds at most `8 W + 7 - r_pre` new records, with `r_pre = len_pre mod 8`, and crosses at most `W` segment boundaries (`chip/membackend.rs:23-26`). At `W = 1` this is `15 - r_pre`: up to 15 new records when the tail is empty and 8 when it holds 7. The capacity depends on the state; it is not "8 per leaf" (B2 finding 8). A padding row has zero kind and data and serves the zero tuple.
- Frontier: `len_pre` and `len_post` are each decomposed into `TOP + 3` boolean bits (`WindowCfg::bits`, `chip/membackend.rs:329-357`), so both are integers below `2^(TOP+3)`.
- Named cells out: `Window { len_pre, len_post, q_pre, base, rows: Vec<WinRow> }` with `WinRow { addr, act, imp, rec: [kind, a, b, c] }` (`chip/membackend.rs:65-82`), given to `MemBackend::persist`.
- Named cells in: the append chain of CORE and the `len` cells of the boundary. The append transport is the hybrid of provider rule 6 and of conditions A1 to A4 of kit CORE: the addresses are copy-linked from `len_pre` to `len_post`, and each append's value is a MEM bus request of CORE that the new window row at that address serves (mutant `append_value_mismatch`). The window does not receive append values as copied cells.
- Requests no bus tuple. Uses no hash: record hashes, the tail opening and `Tree8` are in the backend (`chip/membackend.rs:12-16`, `chip/dense.rs:644-671`).

A logical record wider than three words (`a, b, c`) takes consecutive addresses, and each address is one physical cell: every bound of this kit (window capacity, the address bound, the per-rule maximum of appends) counts physical cells, not logical records (B2 finding 9). The record layouts per kind (which words hold the tag, the payload and the child `vd`) are a schema owned by CORE and VALUE DIGEST (`core/state_schema.rs`, `value/records.rs`); they are not frozen here (section 4, AD-11). The six-word, two-lane records of `astra-area2.md:57-88` are not adopted (they change the key).

#### Invariants

Guaranteed (these are the MEM contract items of RL AUDIT finding 8 and LC AUDIT items 11 to 13 that MEM owns; the others are in the backend and CORE kits):

1. Dense: active rows have consecutive addresses from `8 q_pre`; a new row's address is the current length; `len_post = len_pre + (number of new rows)`; `len` advances only by real appends; closing a leaf advances nothing (mutants `address_reuse_window_row`, `close_advances_len`).
2. Both lengths are below `2^(TOP+3)`; `len = 2^(TOP+3) - 1` is reachable and `2^(TOP+3)` is not (mutants `counter_wrap_2_64`, `counter_field_wrap`).
3. A leaf that appends nothing has `len_post = len_pre` and an unchanged window (pass-through, with the backend's rule 2).
4. The key `8 addr + kind` is injective for `kind` in `0..=4` at any TOP (mutant `kind_out_of_range`: kind 9 at address 16 would alias address 17 kind 1).
5. Window rows serve only addresses at or above `8 q_pre`; the backend serves only addresses below it (mutant `cold_route_forged`).
6. A padding row is off the bus (mutant `pad_row_on_bus`).
7. An active window row with kind 0 is allowed: no reader can request kind 0 and its key is still injective (`FD/EXPERIMENTS.md`, deviations). A gate `act * kind != 0` would need an inverse witness and is not added.

8. Capacity is enforced by the window, cumulatively for the leaf: more than `8 W + 7 - r_pre` appends in one leaf is unsatisfiable (`"mem win last"` forces the last row inactive). Single-step fit (`max appends per step <= 8 W`) is a configuration check against the per-rule table (`check_shape`), and holds at `W = 1` with the measured maximum of 3 (M-2, M-3).

Assumed (the conditions of the C1 Quint lane that MEM cannot establish locally; `qa/gemini-quint/c1-dense/REPORT-dense.md:77-101`, verdict `qa/gemini-quint/c1-dense/verdict.md:9`):

- C1, from CORE: the append chain conditions A1 to A4 and the stitch of `(len, root, tail_d)` across leaves. The bus lookup is set inclusion (`chip/bus.rs:50-61`), so the one-to-one relation of invariant 1 holds only with them.
- C3, from CORE's read port: on every read, items 1 to 9 of the read port (boolean `cold`, the two `TOP + 3`-bit decompositions, the kind pin, the route bits, the bus request, the cold cells, and `len_cur` bound to the running length of the read point). The window cannot see the step of a read; mutants `read_at_len`, `read_beyond_len`, `future_record_read` are refused by the port's decomposition (`"mem bits last"`).
- C4, from INIT: the genesis memory state (see kit MEMORY BACKEND, assumed).
- The backend authenticates the imported rows against the pre token (C2 is the backend's own condition); BOUNDARY COMMITMENT binds `len`.

#### Row budget

MEASURED (C1, run `20261111T065836Z-c1-rows`, `FD/EXPERIMENTS.md` section 5, C1 measured rows): frontier (two length decompositions) 128 rows at TOP = 61 and 132 at TOP = 63; one append in the CORE stand-in 3 rows; the window rows themselves are inside the per-slot figure "one window slot 203 rows" which also holds the backend's record hash, mask and 1/8 of `Tree8`. The circuit pays the window capacity (fixed by W) whether a slot is used or not. No new advice or equality column and no lookup; MEM adds the fixed column `kfix` and 3 packed code columns (`FD/EXPERIMENTS.md` section 5, C1 node rows).

#### Verification

- Quint: the C1 lane model `qa/gemini-quint/c1-dense/dense.qnt` with `dense_test.qnt` (final verdict `qa/gemini-quint/c1-dense/verdict.md`; it is not under `qnt/`, and moving it to `qnt/denseMemory.qnt` is a task of this kit). Result as re-run by the lead: `quint test` 34 passing; `dense_sound` holds the invariant `invRefine`; all 23 mutant modules violate `invRefine` with an accepted tampered witness, and the honest witness is accepted under each mutant; 30 of 30 lines of `rerun.sh` match (`quint run`, `--max-samples 4000 --max-steps 3 --seed 0x20261111`). Model bounds: `SEGC = 4`, `TOP = 2`, 16 cells, a small hash field (`REPORT-dense.md:253-261`); the Rust tests cover `SEGC = 8` and TOP 5, 61, 63. Questions 1 to 8 of the lane: state is a function of the append sequence under any partition (sound with C1 and C2); flush iff a boundary is crossed (sound with C2); canonical `tail_d` and pass-through (sound); key injectivity (sound); window and cold providers disjoint and every read below `len_cur` with the expected kind (sound with C3); the `2^(TOP+3) - 1` edge (sound); appends and new rows one to one (sound with C1); capacity tags equivalent to first-input tags (sound). The lane found one code gap, at `TOP == 1`, fixed in `f36eb714d` (kit MEMORY BACKEND).
- MockProver positive (landed, `chip/dense/tests.rs:438-539`): `c1_pass_through_and_every_tail_occupancy`, `c1_cross_boundary_reads_and_chain`, `c1_two_flush_units`, `c1_last_segment_two_units`, `c1_partition_independence`, `c1_top61_and_top63_positive`, `c1_counter_edge_2_64_minus_1`, `c1_four_arenas`; added with the `TOP == 1` fix: `c1_top1_positive`.
- Named mutants owned by MEM (landed, `chip/dense/tests.rs:640-798`): `address_reuse_window_row`, `read_at_len`, `read_beyond_len`, `future_record_read`, `wrong_kind_warm`, `cold_route_forged`, `counter_wrap_2_64`, `counter_field_wrap`, `pad_row_on_bus`, `kind_out_of_range`, `arena_kind_pinned`, `close_advances_len`, `append_value_mismatch`.
- To add when MEM is wired to the real CORE: `mem_read_between_two_appends` (a read between two appends of one leaf needs the step's own `len` from CORE; the C1 stand-in reads after all appends or at `len_pre`), `mem_wide_record_split_across_leaves`.
- To add for the capacity statements (B2 finding 8): positive `mem_w1_leaf_9_to_15_appends` (one `W = 1` leaf with 9, 12 and 15 appends at `r_pre = 0`, and with `15 - r_pre` appends at each `r_pre`; equal post state to the same appends cut into smaller leaves); mutant `mem_leaf_sum_exceeds_window` (steps that each fit, whose sum is `16 - r_pre` appends in one `W = 1` leaf: unsatisfiable; the honest prover closes the leaf one step earlier); configuration refusal `mem_shape_step_exceeds_window` (a `LeafShape` whose `8 W` is below the per-rule maximum).

#### Lean obligations

| Obligation | File | Premises | Contract discharged | Status |
|---|---|---|---|---|
| `len` advances exactly by the real appends (`appendCells_len`, `canon_len`) | `lean/PartitionIndependence.lean` | none | invariant 1 | proved in C3 |
| For every cut of the steps into leaves the physical state is the abstraction of the logical state, per-cell rule (`partition_independent`) | `lean/PartitionIndependence.lean:404` | none | SRC-023 for memory (ARCH-BRIEF obligation 2) | proved in C3 |
| Two valid cuts with the same sum give the same physical state and digest (`cuts_agree`) | `lean/PartitionIndependence.lean:423-430` | `ValidCut S` for both cuts, equal sums, and `FitsLeaf` for both cuts: every leaf of each cut holds at most 8 appends in total (`:397-400`) | SRC-023 for cuts whose leaves hold at most 8 appends. The window accepts up to `15 - r_pre` at `W = 1`, so this is not yet the theorem of the circuit's capacity (B2 finding 8; the B1 text listed no premise) | proved in C3 |
| State-dependent window capacity: replace `FitsLeaf` by `FitsWindow W`: for every leaf of the cut, `r_pre + appends <= 8 W + 7`. Prove `partition_independent_flush` and `cuts_agree` under it, first at `W = 1` (the lower lemma `leafFlush_eq` already holds while `r_pre + used < 16`, `:198`), then for W flush units | `lean/PartitionIndependence.lean` (extend) | none | SRC-023 for every cut the circuit accepts | open; a condition of reporting SRC-023 as proved for the circuit rule (section 3.1, freeze prerequisites, last row) |
| The `det` premise holds for the physical step relation; `result_unique` with `det` discharged (`pstep_det`, `premises_det`, `result_unique_tail`) | `lean/PartitionIndependence.lean:471,480` | `abs` injective; `stuck` stays a hypothesis | link to `lean/Aggregation.lean` `result_unique` | proved in C3 |
| D3 padding makes the physical state partition dependent, and no deterministic step relation makes every padded leaf a `Run` (`d3_partition_dependent`, `d3_no_det_run`) | `lean/PartitionIndependence.lean:511,537` | none | reason to replace D3 | proved in C3 |
| The address bound, over physical cells and with the initial length: the model's lists are unbounded; add `len < 2^(TOP+3)` and show that the bound is never reached before 2^64 steps when the genesis length is `len_0`, a step appends at most `a` logical records, a record takes at most `e` physical cells, and `len_0 + a e 2^64 <= 2^(TOP+3) - 1`. With `a = 3` (M-2) and `e = 1`, TOP = 63 leaves room for `len_0 < 2^64`; TOP = 61 does not hold even with `len_0 = 0`; `e = 2` would need TOP = 64 | `lean/PartitionIndependence.lean` (extend) | the per-rule maximum `a` (X1, done: 3), the record expansion `e` (AD-11), `len_0` (kit INIT, AD-17) | invariant 2, R-4 | open |
| MEM window as a `Module` instance: valid append requests imply that each provided tuple is the record at its address | new `lean/MemContract.lean` | backend contract | MEM contract for `leaf_refines` | open |

#### Code skeleton

File: `chip/membackend.rs` (owned; landed).

```rust
pub const KIND_DATA: u64 = 1; pub const KIND_FIELD: u64 = 2; pub const KIND_ENV: u64 = 3; pub const KIND_SLOT: u64 = 4;
/// Bus key of a record: 8 addr + kind.                                                (landed, :56)
pub fn bus_key(addr: F, kind: F) -> F;
impl WindowCfg {
    pub fn configure(meta: &mut ConstraintSystem<F>, bus: BusConfig) -> Self;         // landed, :236
    /// Decompose `v` into `nb` boolean bits. Returns the `acc_in` cells, not the bit cells:
    /// `[0]` is `v`, `[i]` is `floor(v / 2^i)`.                                      (landed, :329-357)
    pub fn bits(&self, r: &mut Region<'_, F>, o: &mut usize, v: &Src, nb: usize) -> Result<Vec<Cell>, Error>;
    /// x + 1.                                                                          (landed, :360)
    pub fn inc(&self, r: &mut Region<'_, F>, o: &mut usize, x: &Cell) -> Result<Cell, Error>;
    /// Frontier decomposition and the 8 (W + 1) window rows; returns the cells for the backend. (landed, :375)
    pub fn window(&self, r: &mut Region<'_, F>, o: &mut usize, len_pre: &Src, len_post: &Src, top: usize,
                  kfix: u64, w: &WinW, names: &mut Names, tag: &str) -> Result<Window, Error>;
}
```

To add: nothing to the trait side. The wiring into `RfLeaf` in place of M3 (`chip/m3.rs`) is a NODE-independent task of this kit and is the open item of C1.

#### Agent reads / must not need to read

Reads: this kit, `chip/membackend.rs`, `chip/bus.rs`, `chip/vsel.rs`, the "Design as built" paragraph of `FD/EXPERIMENTS.md`, the harness CORE stand-in in `chip/dense/tests.rs`. Must not need: `chip/dense.rs` gate bodies, `chip/poseidon.rs` gates, CORE rule bodies, `crates/mori-recursion`.

#### Dependencies

BUS. CORE (append chain, read port) by interface.

### Kit: MEMORY BACKEND (dense Merkle, default)

#### Purpose

The backend authenticates what MEM cannot establish locally (the imported tail and records of old segments) and carries memory across leaves in an opaque token. The default is the dense tail-buffered Merkle tree as landed in `chip/dense.rs`.

#### Interface

The seam is the landed trait (`chip/membackend.rs:95-129`):

```rust
pub trait MemBackend {
    type LeafW;   // per-leaf auxiliary witness (dense: walk siblings)
    type ReadW;   // per-cold-read auxiliary witness (dense: the opening path)
    /// Authenticate the imported tail of `win` against `pre`, persist the completed segments,
    /// and return the post token.
    fn persist(&self, pc: &PoseidonCfg, r: &mut Region<'_, F>, o: &mut usize, win: &Window, pre: &[Cell],
               w: &Self::LeafW, names: &mut Names) -> Result<Vec<Cell>, Error>;
    /// When `rd.enable = 1`, prove that `rd.rec` is the record at `rd.addr` under `pre`.
    fn cold_read(&self, pc: &PoseidonCfg, r: &mut Region<'_, F>, o: &mut usize, pre: &[Cell], rd: &ColdRead,
                 w: &Self::ReadW, names: &mut Names) -> Result<(), Error>;
    // New (LC AUDIT item 24, kit PAGE/ROM): one block-line method. `persist` and the opaque token stay.
}
```

- Token: `pre`, post: `Vec<Cell>`; for dense `[root, tail_d]` (`pre[0]`, `pre[1]`, `chip/dense.rs:637`); `len` reaches the backend through the `Window` cells. MEM and CORE never read inside it (`chip/membackend.rs:93-94`). The number of flush units `W` is a parameter.
- Provides on the bus: nothing in `persist` (window rows are MEM's). The scalar cold read does not put a provider row on the bus either: CORE's read port passes the `ColdRead` cells, and the read is tied to the pre root by a gated equality. Heap lines (kit PAGE/ROM) do provide `(MEM, 8 addr + kind, a, b, c)` rows for `addr < 8 q_pre`.
- Requests: whole-sponge slots of HASH with capacity tags: record `H_REC(kind, a, b, c)` (2 permutations), pad record `PAD = H_PAD(0, 0)`, `EMPTY_SEG = H_EMPTY(0, 0)` (non-zero, differs from `PAD`), node `H_NODE_h(x, y)` (heights 1 to 3 inside a segment, 4 to `TOP + 3` in the top tree) (`chip/dense.rs:54-98`).
- The scalar `cold_read` stays next to the lines (L-2): an isolated old read is 4,761 rows against about 6,020 (ESTIMATE) for a line.

#### Invariants

Guaranteed (RL AUDIT finding 8 rules, as built and tested in C1):

1. `tail_d` is canonical: `Tree8(act ? rec : PAD)` over the eight slots of the tail segment; positions at or above `r` are the fixed pad record; `r = 0` gives one constant (`chip/dense.rs:113-120`; mutant `non_canonical_tail`).
2. Every leaf opens the tail: `Tree8(imp ? rec : PAD)` over window rows 0..7 is copy-constrained to `pre.tail_d`. A leaf with no append passes `(len, root, tail_d)` through unchanged (mutant `forged_tail_across_leaf_boundary`; test `c1_pass_through_and_every_tail_occupancy` checks `pre == post`).
3. Flush unit `j` is on iff `f_j = act(row 8 j + 7)`, that is iff `floor(len_post / 8) > q_pre + j`. Its old walk starts at `EMPTY_SEG` by a gate and must reach the running root; the new walk from `T_j` copies the old walk's siblings, bits and accumulators; `root' = root + f_j (new - root)` (mutants `flush_skipped`, `flush_spurious`, `address_reuse_occupied_slot`, `empty_seg_forgery`). The `EMPTY_SEG` gate holds at every `TOP >= 1` (condition C2 of the Quint lane, closed by the fix; see "The `TOP == 1` fix" below).
4. The walk index is `idx_0 = q_pre`, `idx_j = idx_(j-1) + f_j`, decomposed into TOP bits by the walk accumulator with the last accumulator zero. So every walk index is below `2^TOP` and the last segment is provable at any W (test `c1_last_segment_two_units`).
5. Cold read: record hash, then `TOP + 3` levels whose bits are the bits of `addr`, result equal to `pre.root` when `enable = 1`. The kind is hashed into the record, so a read constrains the kind (mutants `wrong_kind_cold`, `cold_value_forged`).
6. A record of segment `q_pre` that the leaf flushes is served by the window, not by the backend. Old reads open against `pre.root`, never the running root.
7. Every hash carries a domain tag in the sponge capacity (E-2), fixed by the circuit, different per domain, height and input length. The audit asked for first-input tags; the capacity gives the same separation at no extra permutation.
8. `EMPTY_SEG` differs from every `Tree8` of record hashes and `PAD` from every record hash only as a collision assumption (RL AUDIT finding 8); the height and domain tags make the inputs distinct.
9. With one arena, the kind is not pinned per arena; reads constrain the kind through the key and the record hash. With four arenas the kind is pinned by `kfix` (mutant `arena_kind_pinned`, test `c1_four_arenas`); four arenas are not the default (L-9).

Assumed: MEM's window invariants; from CORE, conditions C1 (append chain A1 to A4 and the boundary stitch) and C3 (the read port: route `addr < base` by a range-checked difference, `addr < len_cur` at the running length, the kind pin, `enable`, `pre` and the record cells copied from the port) of the Quint lane.

Condition C4 (genesis), assumed from kit INIT, which is its one provider: at `h_pre = 0` the memory state is `(len = 0, root = empty_at(TOP + 3), tail_d = tail_d([]))` (`chip/dense.rs:86-119,144-152`). If the input-admission decision (AD-17) ever puts records into the genesis memory, INIT must prove that each has `kind` in `1..=4` and that `(len, root, tail_d)` is the canonical state of those records as `DenseModel` builds it by appends. "Stored kinds are in range by induction" (LC AUDIT item 7) rests on this and on the window's kind gate.

The `TOP == 1` fix (condition C2; commit `f36eb714d`, `qa/gemini-quint/c1-dense/verdict.md:34-39`). Before the fix `DenseCfg::walk` enabled `WALK_LAST` before `WALK_EMPTY`. At `TOP == 1` the only level of the old walk is also its last, so `x0 == EMPTY_SEG` was not constrained and a prover could flush into an occupied slot; `persist` also did not check the sibling count. The fix enforces rather than rejects: `WALK_EMPTY` takes precedence at `l == 0` for the old walk (`chip/dense.rs:593-601`); at `n == 1` the `acc_out == 0` check that this row loses is enforced by the new walk of the same unit, which copies `acc_out` and whose only row is `WALK_LAST`; `persist` asserts `TOP >= 1` (`:679`) and `u.sibs.len() == w.top` for every flush unit (`:685`). No gate, column or row was added: the 20 `C1-ROWS` lines at TOP = 61 and 63 are byte-identical before and after. Tests: `c1_top1_positive` and `c1_gap_top1_empty_seg_forgery` (accepted before the fix, refused by `"dense walk empty"` after; 33 dense tests pass, run `20261111T101433Z-c1-top1-fix`). The contract item for implementers: the backend is sound for every `TOP >= 1`, the production values are 61 and 63, and a change of the walk selector order must keep `c1_gap_top1_empty_seg_forgery` refused.

Not a soundness condition: `q_post = q_pre + sum f_j` is not a separate gate; it follows from the `act` prefix, the address chain and the decomposition of `len_post` (`FD/EXPERIMENTS.md`, deviations).

#### Row budget

MEASURED (C1, `FD/EXPERIMENTS.md` section 5, C1 measured rows; harness key shape 12 advice, 13 equality, 28 fixed, 2 lookups, degree 6):

| Item | TOP = 61 | TOP = 63 |
|---|---:|---:|
| One flush unit (old and new walk, flush row) | 8,541 | 8,821 |
| One flush with the segment's `Tree8` | 9,031 | 9,311 |
| One cold read | 4,621 | 4,761 |
| Read port of the CORE stand-in | 130 | 134 |
| One window slot | 203 | 203 |
| Tail open | 498 | 498 |
| `persist` at W = 1 / 2 / 3 | 12,137 / 22,298 / 32,459 | 12,417 / 22,858 / 33,299 |
| Harness, W = 1, 8 appends, 0 / 4 cold reads | 12,305 (k 14) / 31,309 (k 15) | 12,589 (k 14) / 32,169 (k 15) |
| Harness, 4 arenas, W = 1 each | 49,172 (k 16) | 50,308 (k 16) |

`persist(W) = 12,417 + 10,441 (W - 1)` at TOP = 63. TOP = 63 costs 280 rows per flush unit and 140 per cold read and changes no k (E-1).

ESTIMATE, composed from the measured parts (`FD/EXPERIMENTS.md` section 5, C1 leaf rows): leaf at S = 16, W = 1, TOP = 63: 15,226 rows with no cold read, 34,806 with 4, 54,386 with 8 (all `k_leaf <= 16`); S = 64, W = 3: 37,387 with none; `k_leaf <= 16` holds at S = 16 up to about 10 cold reads. With 4 heap lines and 2 ROM lines, value digest, stack spills and bound checks the one-arena leaf is about 50,700 to 51,800 rows (ESTIMATE, LC AUDIT finding 7). Node: net +4 fixed columns, about +24,016 rows, from 823,466 to about 847,494 (ESTIMATE, gate 950,000). The backend is not wired into `RfLeaf`; no leaf or node figure with it is measured. The tables exclude hash-stack spills, PROG and Init openings, TRAP and the range table rows.

#### Verification

- Quint: `qa/gemini-quint/c1-dense/dense.qnt` (shared with MEM; final verdict as stated in kit MEM). Its mutant modules owned by the backend: `forged_tail_across_leaf_boundary`, `non_canonical_tail`, `flush_skipped`, `flush_spurious`, `address_reuse_occupied_slot`, `empty_seg_forgery`, `new_walk_unshared_sibs`, `domain_tag_dropped`, `top1_walk_empty_shadowed` (the regression mutant of the pre-fix selector order; the model has `TOP = 2`, so it drops the `EMPTY_SEG` check directly).
- MockProver positive: the landed tests listed in kit MEM, with `c1_top1_positive`.
- Named mutants owned by the backend (landed): `forged_tail_across_leaf_boundary`, `non_canonical_tail`, `flush_skipped`, `flush_spurious`, `address_reuse_occupied_slot`, `empty_seg_forgery`, `c1_gap_top1_empty_seg_forgery` (`TOP = 1`), `wrong_kind_cold`, `cold_value_forged`.
- To add: `persist_wrong_sibling_count` (a synthesis-time refusal by the new assert, recorded apart from constraint refusals), `cold_read_uses_running_root` (a read opened against the root after a flush), `walk_sibling_differs_old_new`, `walk_bit_nonboolean`, `domain_tag_swapped_node_height`, `tail_open_skipped_when_no_append` (a leaf with no append and a changed `tail_d`).
- The seam test: the same MEM harness compiled against a second `MemBackend` (a test backend that authenticates nothing) with no change to `WindowCfg` or the CORE stand-in.

#### Lean obligations

| Obligation | File | Premises | Contract discharged | Status |
|---|---|---|---|---|
| A well-formed `(segs, tail)` is determined by its cells: canonical `tail_d` (`Mem.wf_unique`) | `lean/PartitionIndependence.lean:160` | none | invariant 1 | proved in C3 |
| A segment is flushed iff `floor(len / 8)` grows (`pushCell_flush_iff`, `leafFlush_iff`) | `lean/PartitionIndependence.lean` | none | invariant 3 | proved in C3 |
| The one-flush circuit rule equals the per-cell rule while `r_pre + used < 16` (`leafFlush_eq`); partition independence for every cut whose leaves hold at most 8 appends in total (`leaf_flush_commutes`, `partition_independent_flush`, `ones_fit`) | `lean/PartitionIndependence.lean:198,413-420` | `FitsLeaf` (`:397-400`): at most 8 appends over the whole leaf, a cumulative bound, not a per-rule bound | SRC-023 for the circuit rule at W = 1, for leaves of at most 8 appends only. A `W = 1` leaf with 9 to `15 - r_pre` appends is accepted by the circuit and is outside this theorem (B2 finding 8) | proved in C3 |
| The state-dependent capacity `FitsWindow W` (`r_pre + appends <= 8 W + 7` per leaf), at `W = 1` and then for W flush units | `lean/PartitionIndependence.lean` (extend); the row of kit MEM | none | SRC-023 for every accepted cut, also at S = 64 (W = 2 to 5); today the MockProver tests `c1_partition_independence` and `c1_two_flush_units` and the Quint lane (4 partitions, small bounds) | open |
| Cumulative leaf fit is a property of every accepted leaf: the window relation implies `FitsWindow W` for the leaf (the last row is inactive) | `lean/MemContract.lean` | the window relation | discharges the `FitsWindow` hypothesis for accepted leaves; no scheduler is trusted | open |
| Single-step fit: every rule appends at most `8 W` physical cells in one step, so one step fits a leaf at every `r_pre` (completeness, GP-1) | `lean/PartitionIndependence.lean` with the per-rule table | the per-rule maximum (3, M-2) and the record expansion `e` | every run has a valid cut; with `ones_fit` the one-step cut is always valid | open |
| Read soundness: a cold read or a line that reaches `pre.root` returns the record appended at that address | new `lean/MemContract.lean` | hash injectivity premises | backend contract for `leaf_refines` | open |

#### Code skeleton

Files: `chip/dense.rs`, `chip/dense/tests.rs` (owned; landed), `chip/poseidon.rs` `sponge_iv` (shared with HASH).

```rust
pub fn iv(dom: u64, sub: u64) -> F;                 // capacity word of a domain            (landed, :60)
pub fn rec_d(r: &Rec) -> F;  pub fn pad_d() -> F;  pub fn empty_seg() -> F;               // (landed, :67-79)
pub fn node(h: usize, x: F, y: F) -> F;  pub fn tree8(l: &[F]) -> F;  pub fn tail_d(recs: &[Rec]) -> F;
impl DenseModel {                                   // native model, sparse top tree        (landed, :161-246)
    pub fn new(top: usize) -> Self;
    pub fn append(&mut self, r: Rec);
    pub fn state(&self) -> State;                   // (len, root, tail)
    pub fn sibs_cold(&self, addr: u128) -> Vec<F>;
    /// Witness of one leaf that appends `recs` with `w` flush units.
    pub fn leaf(&mut self, recs: &[Rec], w: usize) -> (WinW, DenseLeafW);
}
impl DenseCfg { pub fn configure(meta: &mut ConstraintSystem<F>, cols: Cols) -> Self; }   // (landed, :345)
impl MemBackend for DenseCfg { type LeafW = DenseLeafW; type ReadW = DenseReadW; /* persist :624, cold_read :716 */ }
```

#### Agent reads / must not need to read

Reads: this kit, kit MEM, kit HASH (slot port and domains), `chip/membackend.rs:1-130`, `chip/dense.rs`, `FD/EXPERIMENTS.md`, `qa/gemini-quint/c1-dense/verdict.md` and sections 1 and 2 of `REPORT-dense.md` (section 4 of the report describes the pre-fix code and its test spec is wrong for `SEGC = 8`, `verdict.md:44`). Must not need: CORE rules, DEC, ALU, `crates/mori-recursion`, the ZmLogUp variant.

#### Dependencies

BUS, HASH, MEM window. BOUNDARY COMMITMENT consumes the token. INIT provides the genesis state (C4). The host side of memory (the append log, sibling and line witnesses, restore) is the HOST CHECKPOINT STORE kit, not this trait: `persist` assigns constraints and returns boundary cells and writes no durable store (`chip/membackend.rs:95-128`).

### Lead notes on the kits that follow

The next ten kits were digested by helper agents from the drafts named in the provenance note, under the same decided inputs. The lead did not read these kits line by line: they were checked by search for rejected terms, for the nine kit items and for status claims, and patched where the lead notes below say so. Three points join them to the kits above:

1. Leaf lanes and the boundary schema (LC AUDIT item 27; N-2 and N-14 in section 4.2; V-6 in section 4.3). The leaf keeps the seven landed lanes. `input_d`, the dependency commitment `deps_d`, the terminal kind and the terminal payload are the words of a short header of the state commitment, so the seal (NODE SEAL and SEAL-WITH-DEPS, the only seal path; `L` opens the sealed SET head (section 1.11, FP-3)) reads them by re-hashing that header against `post_d` with one fixed-length sponge (as `crates/mori-recursion/src/ivc.rs:81-84` and `run_decider.rs:285-288` do for the outcome tag and value today, where the header is `[dom, tag, value, lanes_d]`). The B1 text said "top-level words of a flat sponge at fixed positions"; a flat sponge gives no selective opening, and the design uses the header and body form of kit BOUNDARY COMMITMENT (B2 finding 5). CORE never writes `input_d` after INIT (it is in no rule's write set; mutant `core_unlisted_register_changed`). If measurement (X10) shows that this opening does not fit the node, the fallback is a new versioned leaf statement, which needs an SRC-001 amendment.
2. ROM chip id 4 is the id of section 1.2. No `CHIP_*` constant other than ALU 1, MEM 2 and HASH 3 exists in `chip/` (checked).
3. The block-line method is named `open_line` in the PAGE/ROM kit. It is the one new method of `MemBackend` (LC AUDIT item 24).

`LC-AUDIT` in the helper kits is `LC/AUDIT.md`. `A:` and `G:` in the recursion kits are `FD/drafts/astra-area3.md` and `FD/drafts/gemini-area3.md`.

### Kit: PAGE/ROM block cache

#### Purpose

This is the Profile A cache line of L-1. Each leaf has a small, fixed number of 8-record lines, and each line has two parts:

- **Heap line.** It opens one completed segment of the one arena (L-9) against the leaf's PRE-state root. It serves the 8 records on the bus as MEM providers. It is one new method on the landed `MemBackend` trait (LC-AUDIT item 24). `persist` and the scalar `cold_read` stay (L-2): an isolated old read costs 4,761 rows with the scalar walk (MEASURED, TOP = 63) against about 6,020 with a line (ESTIMATE).
- **ROM line.** It opens 8 rows of a program table (program, type and constant tables) against `prog_root`. It serves them as providers of a new chip, ROM. ROM is not a `MemBackend` method, because its root is `prog_root` and not the memory token.

`K_page` (heap lines per leaf) and `K_rom` (ROM lines per leaf) are not frozen (L-2, M-3, LC-AUDIT item 28). They are fields of `LeafShape { s, w_flush, k_page, k_rom, n_cold }` (section 1.5). Status after experiment X1 (`FD/EXPERIMENTS-MAXIMA.md`, "What this means for leaf sizing"): one step reads at most 4 distinct heap records, in up to 4 distinct old segments in one arena (a code-reading bound; 2 observed), so a step fits only if `k_page + n_cold >= 4`, for example `K_page >= 4` with line caching or 4 scalar cold reads; this is a bound from the rules, not yet a measured leaf. `K_rom` has no bound: the ROM rows and blocks of one step are not bounded by X1 (at most 6 ADT view row sites in 4 tables by a static count of one site class, 3 logged entries in 3 tables observed as a lower bound, plus 2 selection expression rows by code reading). Experiment X15 closes it. The ROM tree depth is also open (LC-AUDIT finding 12; see section 4.3, V-1).

#### Interface

**Heap line, method `open_line` on `MemBackend` (dense implementation in `dense.rs`).**

Inputs (copy-constrained cells):
- `pre[0] = root` and `pre[1] = tail_d`: the backend's pre token, the same cells that `persist` receives. MEM never reads inside them (`membackend.rs:93-94`).
- `q_pre`: the cell of `Window` (`membackend.rs:78`). MEM decomposes it as the high part of `len_pre`, below `2^TOP`.
- `en`: the line enable. It is boolean (contract item 5).
- `blk`: the segment index of the line.
- `rec[j] = [kind_j, a_j, b_j, c_j]` for `j = 0..7`: the provider cells.
- Witness: TOP sibling values for the walk.

Provided bus tuples: 8 provider rows, row `j` fixed by the layout:
`(MEM, 8 (8 blk + j) + kind_j, a_j, b_j, c_j)` with `MEM = 2`. When `en = 0`, all five cells are zero.

The key is the landed `bus_key(addr, kind) = 8 addr + kind` (`membackend.rs:55-58`). It stays injective at every TOP (LC-AUDIT finding 3).

Requested: HASH permutations through `PoseidonCfg` sponge slots inside the backend region, with copy-constrained inputs, as `persist` does (`dense.rs:650-651`). The line uses no bus request.

Range checks and their owners:
- `blk < 2^TOP`: the backend. The walk bits are the TOP-bit decomposition of `blk`, and the last accumulator is zero.
- `blk < q_pre`: the backend, with a new gated comparison gate (for example, a TOP-bit decomposition of `q_pre - 1 - blk` when `en = 1`).
- `kind_j` in `0..=4`: by induction from `persist`, because `WindowCfg` checks it on every stored record (`membackend.rs:28-30`). The base of the induction is condition C4, which kit INIT provides: the genesis memory is empty (or, if AD-17 puts input records there, INIT proves their kinds). The first build also repeats the kind gate on line rows (V-3 option b) until INIT is built and tested.
- `addr < len_cur` and the expected kind: CORE's read port, for every consumer including VALUE DIGEST (LC-AUDIT item 11; kit CORE, read port). The line does not check them. Membership in the window or in a line is not enough for a read: the window also holds records appended later in the leaf.

Window and line providers serve the same chip id. A request does not name its provider.

**ROM line (new provider; chip `ROM`, id 4 proposed; no other use of id 4 exists in `crates/mori-chip/src`).**

Inputs:
- `prog_root`: a cell copied from the persistent register `prog_root` of CORE's state. Kit INIT is the one owner of the `prog_d` opening: at `h_pre = 0` it opens `prog_d` and ties the register to it (`O/design.md:174`); the register is in no rule's write set, so every later leaf carries the same value through `pre_d`.
- `en`: boolean.
- `blk_r`: the ROM block index.
- `row[j] = [a_j, b_j, c_j]` for `j = 0..7`.
- Witness: `D_rom` sibling values, where `D_rom` is the number of tree levels above the 8-row block.

Provided: 8 provider rows `(ROM, 8 blk_r + j, a_j, b_j, c_j)`. When `en = 0`, all five cells are zero.

Key layout:
- The path index is `blk_r = tbl * 2^(D_row - 3) + rowblk`. So the key is `8 blk_r + j = tbl * 2^D_row + row`.
- The table id `tbl` takes the top `D_tbl` path bits, and the row takes the low `D_row` bits. All bits come from the walk decomposition (item 15).
- `D_rom + 3 = D_tbl + D_row`.

Who range-checks (one owner per side; the same assignment as the table of section 1.2):
- Provider side, `blk_r < 2^D_rom`: the ROM line, by its walk decomposition.
- Request side, `row < 2^D_row` and the table id: CORE's ROM read port `read_rom`, the only issuer of ROM requests. It takes `tbl` as a call-site constant and `row` only as a RANGE handle of width `D_row`. VALUE DIGEST and DEC inputs receive `AuthRom` cells from it and issue nothing. Without the range, `(tbl, 2^D_row)` aliases `(tbl + 1, 0)`. This is contract item C-2; it is an obligation of the read port, not of each consumer.

A table row wider than three words needs a row layout. One way is several sub-rows, with the sub-row index in the low key bits. That choice is open (section 4.3, V-2). Whatever the layout, `row` is a physical row index: `D_row` must cover the logical index width of the widest table plus the sub-row bits, `D_row >= index_width + ceil(log2(max sub-rows per logical row))`, and `D_tbl + D_row` is far below the field size, so the key does not wrap (B2 finding 9).

Domains: a ROM row leaf is `H_ROMROW(a, b, c)`, and a ROM node is `H_ROMNODE_h(x, y)`. Each has its own capacity tag (`poseidon::sponge_native_iv`, `dense.rs:59-62`). Both tags differ from `DOM_REC = 1`, `DOM_PAD = 2`, `DOM_EMPTY = 3` and `DOM_NODE = 4` (`dense.rs:54-57`). Node tags carry heights `1..D_rom + 3`. The numeric values are not assigned yet.

**Shape parameter.** `LeafShape { s, w_flush, k_page, k_rom, n_cold }` (section 1.5). `k_page`, `k_rom` and `n_cold` stay parameters until the ROM part of the per-rule table (X15) and the locality census (X4) exist. The non-interleaving check is done (X2, M-1: holds for all 120 rules), and `w_flush = 1` is derived (M-3).

#### Invariants

Numbered contract items. LC-AUDIT section 5.1 items 1-10 and section 5.3 items 15-19 keep their numbers. C-1 and C-2 are additions of this kit. Each item names its mutant or test; landed names are marked (landed, `FD/EXPERIMENTS.md`), and every other name is new.

| # | Contract item (guaranteed by this kit unless "assumed") | Mutant or test |
|---|---|---|
| 1 | A heap line is opened against the leaf's `pre.root`, never against the running root. The window serves the records of segments flushed in this leaf. | `line_uses_post_root` (the line authenticates a segment flushed in this leaf against the running root; refused by the root equality against `pre[0]` and by item 6) |
| 2 | The 8 leaves of a line are `H_REC(kind, a, b, c)` with the capacity tag `iv(DOM_REC, 0)` (`dense.rs:67-69`). `Tree8` uses heights 1 to 3, and the walk uses heights 4 to TOP + 3, as in `persist`. | `line_leaf_not_hrec` (Tree8 over raw words or over an untagged record hash); `line_height_splice` (a height-5 node used at height 4) |
| 3 | The provider cells are the hashed cells (copy constraints). Row `j` of a line serves `(MEM, 8 (8 blk + j) + kind_j, a, b, c)`, and the row fixes `j`. | `line_provider_not_hashed` (the served `a` differs from the hashed `a`); `line_row_index_swapped` (row 2 serves key `8 (8 blk + 3) + kind`) |
| 4 | The walk bits are the TOP-bit decomposition of `blk` (last accumulator zero). | `line_walk_bits_not_blk` (the bits decompose `blk + 1`, or the last accumulator is nonzero) |
| 5 | `en` is boolean. With `en = 0`, all five provider cells are zero and the root equality is off (as the gated equality at `dense.rs:743-749`). | `line_disabled_nonzero` (en = 0 and the tuple is nonzero); `line_en_nonboolean` (en = 2) |
| 6 | `blk < q_pre`. The domain separation of `EMPTY_SEG` already prevents opening an unflushed slot. The check stays, so that window and cache are disjoint by a local rule. | `line_blk_ge_qpre` (blk = q_pre, the tail segment) |
| 7 | `kind` in `0..=4` holds for stored records by induction from `persist`. The induction needs a genesis root that is the empty tree or is proved by kit INIT (condition C4). Otherwise the line repeats the kind gate on its rows. If a four-arena variant is ever chosen (not the default, L-9), a line names its arena root and pins the kind. | `line_kind_out_of_range` (a pre root with a kind-9 record and no Init proof; refused by the Init gate or by the repeated kind gate, depending on V-3). Landed analogue: `kind_out_of_range` (landed) |
| 8 | Deduplication and ordering of lines are not soundness conditions. The lookup is set inclusion, and a provider is a function of its key (`bus.rs:18-20`). The kit adds no constraint for them. | Positive test `ok_duplicate_and_unordered_lines`. Mutant on the property it relies on: `line_dup_block_two_values` (two lines for one block serve different records; one of them fails the root equality) |
| 9 | A line changes no boundary value. `(len, root, tail_d)` and the public statement do not depend on which lines a leaf opens (SRC-023). | Differential test `line_boundary_invariance` (equal post token and equal `pre_d`/`post_d` with 0, 1 and `K_page` lines). Mutant `line_cell_in_post_token` (a line output copied into the post token; the differential fails) |
| 10 | Capacity: `K_page + n_cold`, `K_rom` and the lane count are at least the per-step maxima of the per-rule table, derived from every branch of every rule (4 old heap records per step by X1; ROM open, X15). This is completeness, not soundness: a step that needs more lines than the leaf has cannot be proved even at S = 1 (GP-1). A corpus maximum is validation, never the bound. | Completeness check `leafshape_below_rule_max_refused` (building a `LeafShape` below the per-rule table maximum fails); test `ok_max_rule_step_at_s1` (`BHelperEqualField` with four old records in four segments) |
| 15 | ROM has its own chip id (ROM = 4, proposed). Its key holds the table id and the row, and the row and table bits come from the path bits. | `rom_key_not_from_path` (key = path index + 1); `rom_tbl_row_alias` (a row of table 1 served under the key of table 14) |
| 16 | The ROM leaf hash and node tags differ from the heap domains and carry heights. | `rom_heap_domain_splice` (a heap `H_NODE` used inside the ROM walk); `rom_height_splice` |
| 17 | The padding of unused rows and of the last line is canonical and is part of the `prog_d` normal form. | `rom_pad_noncanonical` (a nonzero pad row under the same `prog_root`; refused by the padding check of ADMIT-WORK, mutant `admit_pad_noncanonical`, and the line differs from the canonical root) |
| 18 | `ty_d` and constant `vd` in ROM are trusted only through the PROG canonicality premise (`O/design.md:84`), which the leaf takes as a premise and the seal discharges with the `ADM` claim of ADMIT-WORK (kit NODE INTEGRATION). | ADMIT-WORK mutants `admit_const_vd_forged`, `admit_ty_d_forged`, `admit_ctor_arity_forged`; seal mutants `node_seal_unadmitted_prog`, `node_seal_admission_other_prog`. Host differential `rom_ty_d_matches_admission` |
| 19 | The depth and the program-size bound are stated over physical rows (logical index width plus sub-row bits). A depth below that, or a depth keyed per program, is not an acceptance path of this change: if the required depth does not fit, GP-1 is incomplete, not narrowed (LC-AUDIT finding 12; B2 finding 9). | Completeness check `rom_depth_covers_table_index` (admission refuses no program whose table indices fit the language's index width, with wide rows); V-1 |
| C-1 | ROM lines have no pre/post: `prog_root` is fixed for the run. A ROM line changes no boundary value. | Covered by `line_boundary_invariance` with ROM lines |
| C-2 (assumed from CORE's ROM read port, the one requester) | The read port takes `row` as a RANGE handle of width `D_row` and `tbl` as a call-site constant. | `rom_row_overflow_alias` (request `(tbl, 2^D_row)`, served as `(tbl + 1, 0)`; refused by the port's range; the control is a port that accepts an unranged row) |

Assumed from other modules:
- MEM: `q_pre` is the high part of `len_pre` and is below `2^TOP`; window rows have addresses `8 q_pre + i` (`membackend.rs:18-30`).
- CORE (LC-AUDIT items 11-14): every read goes through the read port, which takes its address from a register, constrains its kind and checks `addr < len_cur` at the running length of that read point, for CORE rules and for VALUE DIGEST alike. Unused lanes are zero in all five cells. The landed mutants `future_record_read`, `read_at_len` and `wrong_kind_warm` (landed) cover this. They now also cover reads served by lines.
- INIT: the genesis memory is empty, or its records are proved by INIT (item 7, condition C4), and `prog_root` is the root opened from `prog_d`. The form in which an input enters memory is AD-17 (`O/design.md:194,257`).
- HASH: tags live in the capacity (E-2). There is no provider row with an unconstrained output.

Why the five attacks of LC-AUDIT 5.1 fail rests on items 1-7. Item 8 is the reason that duplicate lines are harmless.

#### Row budget

- Scalar `cold_read`: 4,761 rows at TOP = 63 (MEASURED); 4,895 with the CORE read port (MEASURED parts).
- Heap line: 16 record-hash permutations + 7 `Tree8` + TOP = 63 walk levels = 86 permutations, so 6,020 rows (ESTIMATE, LC-AUDIT finding 7). Add the `blk < q_pre` comparison, about 66 to 132 rows (ESTIMATE, by analogy with the 132-row frontier of two decompositions, MEASURED), and 8 provider rows.
- ROM line: `70 x (16 + 7 + D_rom)` rows (ESTIMATE; 3-word rows hash in 2 permutations). At `D_rom = 32` this is 3,850 rows (ESTIMATE). The depth is open (V-1).
- Composed leaf at S = 16, W = 1, TOP = 63, with `K_page = 4` and `K_rom = 2` as one example only: about 50,700 to 51,800 rows (ESTIMATE, LC-AUDIT finding 7; inputs 15,226 composed leaf, 4 x 6,020, 2 x 3,850, value digest, spills, ALU ranges, bound checks). The example does not freeze K.
- Node: lines and ROM add code columns. At about 6,004 rows per fixed column (ESTIMATE fit, `FD/EXPERIMENTS.md` "Node rows"), on the node base of about 847,494 (ESTIMATE).
- Unused lines still run their permutations.

#### Verification

- Quint: new `models/sherlock/quint/denseLineCache.qnt`, built on the C1 lane model `qa/gemini-quint/c1-dense/dense.qnt` (final verdict `qa/gemini-quint/c1-dense/verdict.md`; the model is not under `models/sherlock/quint/`). Invariants: `LineMatchesPreRoot`, `WindowLineDisjoint` (`addr >= 8 q_pre` against `addr < 8 q_pre`), `KeyInjective` (kind in `0..=4`), `BoundaryIndependentOfLines`, `RomImmutable`, `RomKeyInjective` (with the requester range).
- MockProver positives:
  - `ok_line_all_8_records`
  - `ok_line_and_scalar_cold_read_same_leaf`
  - `ok_duplicate_and_unordered_lines`
  - `ok_line_top63`
  - `ok_rom_line_each_table`
  - `ok_rom_last_partial_line`
  - `ok_max_rule_step_at_s1`
  - `line_boundary_invariance`
- Negative mutants: the table above. These landed mutants must still pass with lines enabled: `future_record_read`, `read_at_len`, `wrong_kind_warm`, `wrong_kind_cold`, `cold_value_forged`, `cold_route_forged`, `kind_out_of_range`, `empty_seg_forgery` (landed).

#### Lean obligations

| Obligation | File | Premises/axioms | Contract | Status |
|---|---|---|---|---|
| A line opening is free of boundary effects: the physical state after a leaf does not depend on the set of lines it opens. | `clean/fork/Clean/Sherlock/PageCache.lean` (new) | builds on `partition_independent_flush` and `cuts_agree` (`FitsLeaf`) | item 9, C-1 | open |
| Line soundness: if `en = 1`, `blk < q_pre` and the line reaches `pre.root`, each served tuple is the cell at `8 blk + j` of logical memory. | `PageCache.lean` (new) | collision resistance of the tagged hashes (a `digest_inj`-style premise); `Mem.wf_unique` | items 1-7 | open |
| The line is a `Module` whose `Contract` guarantees that every provided MEM tuple is valid. | `PageCache.lean` instance over `ModularComposition.lean` | `provided_valid`, `leaf_refines` (proved in C3) | items 1-8 | open (instance); composition theorem proved in C3 |
| A ROM line serves the table row of `prog_d` at `(tbl, row)`. | `clean/fork/Clean/Sherlock/RomLine.lean` (new) | PROG canonicality premise (`O/design.md:84`), collision resistance | items 15-18, C-2 | open |

#### Code skeleton

```rust
// crates/mori-chip/src/membackend.rs (add; persist and cold_read unchanged)
pub struct Line { pub enable: Cell, pub blk: Cell, pub recs: [[Cell; 4]; SEGC] }
pub trait MemBackend {
    type LeafW; type ReadW;
    /// Per-line witness (dense: TOP siblings).
    type LineW;
    /// Capabilities, known at configuration time. A backend without lines sets `lines = false`,
    /// and `check_shape` then refuses `k_page > 0`. `open_line` never fails at synthesis time
    /// because of a missing capability (B2 finding 17).
    const CAPS: BackendCaps;   // BackendCaps { lines: bool }
    // persist, cold_read: as landed (membackend.rs:104-127)
    /// When line.enable = 1: line.recs are the 8 records of segment line.blk under pre,
    /// line.blk < q_pre, and the 8 provider rows serve (MEM, 8(8 blk + j) + kind_j, a, b, c);
    /// when 0, the rows serve the zero tuple. Changes no token cell.
    fn open_line(&self, pc: &PoseidonCfg, r: &mut Region<'_, F>, o: &mut usize,
                 pre: &[Cell], q_pre: &Cell, line: &Line, w: &Self::LineW,
                 names: &mut Names) -> Result<(), Error>;
}
// crates/mori-chip/src/dense.rs: impl of open_line for DenseCfg (same walk and iv as persist).

// crates/mori-chip/src/rom.rs (new)
pub const CHIP_ROM: u64 = 4; // proposed; unused in the crate today
pub struct RomShape { pub d_tbl: usize, pub d_row: usize } // open (V-1)
pub struct RomLine { pub enable: Cell, pub blk: Cell, pub rows: [[Cell; 3]; SEGC] }
/// key = tbl * 2^d_row + row; requires row < 2^d_row.
pub fn rom_key(shape: &RomShape, tbl: u64, row: u64) -> F;
/// Native canonical root of the normalised tables (padding rows canonical).
pub fn rom_root(shape: &RomShape, tables: &[Vec<[F; 3]>]) -> F;
pub struct RomCfg { /* private */ }
impl RomCfg {
    /// When enable = 1: rows are block blk under prog_root, served as (ROM, 8 blk + j, a, b, c).
    pub fn open(&self, pc: &PoseidonCfg, r: &mut Region<'_, F>, o: &mut usize,
                prog_root: &Cell, line: &RomLine, sibs: &[F], names: &mut Names) -> Result<(), Error>;
}
// crates/mori-chip/src/leaf.rs: uses the LeafShape of kit CORE (five fields, section 1.5); no second definition.
/// Refuses a shape below the per-rule maxima table (appends against 8 w_flush; old heap records
/// against k_page + n_cold; ROM blocks against k_rom), and k_page > 0 with a backend that has no lines.
pub fn check_shape<B: MemBackend>(shape: &LeafShape, maxima: &RuleMaxima) -> Result<(), ShapeError>;
```

Tests: `crates/mori-chip/src/dense/tests.rs` (extend) and `crates/mori-chip/src/rom/tests.rs` (new).

#### Agent reads / must not need to read

Reads:
- `crates/mori-chip/src/{membackend.rs, dense.rs, bus.rs, poseidon.rs}`
- `FD/EXPERIMENTS.md` (C1 tests, mutants, rows)
- LC-AUDIT section 5.1 and 5.3 and findings 1, 3, 4, 12
- `O/design.md:174` (the `prog_d` opening)

Must not need: CORE rule bodies (`rfcore.rs` rules), ALU, DEC, `crates/mori-recursion/`, the ZmLogUp kit, VALUE DIGEST internals.

#### Dependencies

MEM (`WindowCfg`), HASH (`PoseidonCfg`), CORE (the read port and request contract items 11-14), INIT (`prog_root` from the `prog_d` opening; the genesis memory, C4), BOUNDARY COMMITMENT (the token passes through unchanged), and the per-rule maxima table for `LeafShape` (X1 for heap records, X15 for ROM).

---

### Kit: MEMORY BACKEND variant ZmLogUp (Profile B, benchmark-only)

#### Purpose

This is a run-wide write-once LogUp memory backend behind the landed `MemBackend` seam. It is benchmark-only (L-5) and is not built before milestone M4. It is not a two-trait swap: it changes the leaf statement, node, seal and continuation modules, and it replaces the `digest_inj` premise with a probabilistic one (LC-AUDIT finding 9). It has no stop/resume: across a stop there is no memory commitment, so resuming needs a reload of all live memory or the address-free checkpoint (mode 2), which R-7 does not adopt. It is not suited as the production backend.

#### Interface

What it needs from the proof system:
- **Committed instance column.** It already exists in the spike: `committed: Column<Instance>` (`crates/mori-chip/src/rfcore.rs:605,833`). The `committed-instances` feature is on (LC-AUDIT finding 9). Per leaf it holds `P_i`:
  - the write entries `(act, key, a, b, c, m)`, one per appended record lane, with `key = 8 addr + kind`;
  - the read entries `(act, key, a, b, c)`, one per authenticated old-record entry (deduplication inside a leaf is allowed);
  - the imported-tail entries, which are read entries: one per imported window row (see the mapping below);
  - the boundary words, if the Horner boundary is used;
  - canonical zero padding.

Mapping from cells to LogUp entries (B2 finding 17). It is complete: every cell that can serve a record to a request is on exactly one side of the balance.

| Cell class | Entry | Side | Multiplicity |
|---|---|---|---|
| New window row (`act = 1`, `imp = 0`): a record appended in this leaf | write entry `(key, a, b, c, m)` | credit `m / (alpha - E)` | `m` = the number of read entries of the whole run that name this record (old-record reads and imported-tail entries of later leaves); committed in pass 1; `m = 0` is legal |
| Imported window row (`imp = 1`): a tail record appended by an earlier leaf | read entry `(key, a, b, c)`, always, whether or not a request uses the row | debit `1 / (alpha - E)` | 1 per imported row per leaf |
| Old-record provider row (the Profile B form of `cold_read`, and of a line if lines are configured) | read entry `(key, a, b, c)` | debit | 1 per provider row; one provider row may serve many bus requests of the leaf (that is the deduplication), and each provider row is one debit |
| Initial record at genesis (only if AD-17 puts input records into the genesis memory) | write entry emitted by the genesis leaf, with the record authenticated by INIT | credit | as a new row |
| Padding row, inactive entry | none | contribution 0 | `m = 0`, `inv = 0` |

An imported row is never trusted because it is committed: committing a fabricated record before the challenges are drawn does not show that it was written. It is debited, so the run sum is zero only if an earlier leaf's write entry with the same `(key, a, b, c)` credits it. The window serves imported rows on the bus exactly as in Profile A; without the debit, a read served from a forged imported row would escape the read-entry path. Bus requests are not entries: a request is tied to a provider row by the lookup, and the provider row carries the debit.

  `C_i = commit(P_i)`. Every term of the sums is a function of cells bound cell by cell to the committed instance column (LC-AUDIT item 26).
- **Challenges as public inputs.** One pair `(gamma, alpha)` in the base field by default; a second pair is optional and costs rows (LC-AUDIT finding 9). They enter as scalar instance values of the leaf, never as `Expression::Challenge`. This keeps the verifier's single-phase assertion (`verifier_gadget.rs:310`) untouched.
- **Two passes.**
  - Pass 1 executes the whole run, fixes the multiplicities and commits every `P_i`.
  - `d_root` is the hash over all `C_i` in order.
  - The challenges are the hash of `d_root` (with the profile, `prog_d` and `input_d` as domain inputs).
  - Pass 2 proves the leaves under the fixed challenges.

Per-leaf relation, for each challenge pair:
- `E(key, a, b, c) = key + gamma a + gamma^2 b + gamma^3 c` (compression staged over rows to keep the degree at most 6).
- An active entry has `(alpha - E) inv = 1`.
- An inactive entry has tuple = 0, `m = 0`, `inv = 0` and contribution 0.
- `sum_i = Σ m_w inv_w - Σ inv_r - Σ inv_imp`, with one `inv_imp` term per imported window row of the leaf.

Bus: reads are served to CORE as `(MEM, 8 addr + kind, a, b, c)` with the same key and the same chip id. CORE keeps `addr < len_cur` on every read (item 11).

Token (opaque to MEM, `membackend.rs:12-16`): no memory root and no tail digest, so the token `Vec<Cell>` is empty and nothing of the token authenticates the imported tail; the imported-tail debit above does. `len` stays a MEM `Window` cell. The leaf sum (one per challenge pair) is a leaf output that the node adds; it is not a stitched token cell. Capability: `CAPS.lines` is fixed at configuration time (kit PAGE/ROM skeleton); if Profile B is configured without lines, `check_shape` refuses `k_page > 0`, and no call returns `Error` at synthesis time for a missing capability. The challenge cells reach the backend through its config, assigned from the scalar instance column, so the landed `persist`/`cold_read` signatures stay. That this is enough is not checked.

Node (Area 3):
- witness `C_i` with the subgroup-checked `assign` (`weierstrass_chip.rs:343-360`);
- `C_i` enters `d_root`;
- the sums add;
- the children carry equal challenges.

Seal:
- the challenges equal the hash of `d_root`;
- the total sum is zero;
- the initial records (input) are authenticated by Init.

Range checks: `len` and the addresses by MEM. A range check on `m_w` is not needed: the characteristic is far above the counts (LC-AUDIT finding 9, hardening 3).

Modules it changes (L-5): the leaf statement (a new versioned statement that carries `C_i` binding, the challenges and the sums; not appended silently to the seven lanes), NODE INTEGRATION (variable committed point, subgroup check, `d_root`), the seal/L AND WRAP, and CONTINUATION. It also replaces the `digest_inj` premise (`Aggregation.lean:201`) for memory.

#### Invariants

Soundness holds only if all of the following hold (LC-AUDIT finding 9 and item 26):
1. Every term of the sums (and of the Horner fingerprint, if used) is a function of cells bound to the committed instance column.
2. All leaf commitments enter `d_root`.
3. The seal checks that the challenges equal the hash of `d_root` and that the sum is zero.
4. Each address is written once (dense counter from MEM).
5. Every read keeps `addr < len_cur` (CORE).
6. Active denominators are nonzero, and inactive terms are zero.
7. Commitment points are subgroup-checked.
8. Initial memory has an authenticated source. A host-generated table is insufficient (`astra-area2.md:573`).
9. Coverage: every imported window row and every old-record provider row is a read entry (the mapping table). No cell that can serve a record is outside the balance.

Soundness bound (LC-AUDIT finding 9): Schwartz-Zippel on the joint identity in the challenge pair gives about `(W + R) x arity / r`, near `2^-187` at `2^64` accesses. One challenge pair in the base field meets 128 bits. A factor for Fiat-Shamir re-derivation attempts is not in the audit figure (section 4.3, not checked).

Not claimed:
- stop and resume of a run;
- proving a prefix before the run ends;
- resetting sums at a checkpoint (unsound when later leaves read earlier writes).

Neither of these is a contract item; LC-AUDIT item 26 lists neither:
- **A trailing-zero gate on unused committed rows** (gemini-area2.md:274). Rows that no term reads do not change the sums, and invariant 1 already binds every row that a term reads.
- **A blinding scalar in `P_i`** (gemini-area2.md:274). It is a privacy question for any exposed precommitment, not a soundness question (astra-area2.md:1220). It stays open for the Profile B design.

Assumed: MEM unique append addresses; CORE read causality; KZG binding of `C_i` before the challenges.

#### Row budget

All ESTIMATE, unmeasured; nothing is built (`FD/EXPERIMENTS.md` "Not covered").
- Leaf: about 4 to 8 rows per entry lane for one compression/inverse/sum track. This is half of the 8 to 16 rows for two tracks in `astra-area2.md:597`.
- Up to `|P_i|` rows of committed-instance binding.
- 100 to 300 rows of counters and glue.
- No heap Merkle walks.
- Node: one subgroup check per `C_i`. `gemini-area2.md:297` gives +9,500 to 20,200 rows; LC-AUDIT section 4 lists the subgroup-check figures as not verified.

#### Verification

- Quint: new `models/sherlock/quint/writeOnceLogup.qnt`. The precommit phase is separate from the challenge phase. A small-field model can show counterexamples but not the production bound.
- MockProver positives: `ok_logup_two_and_three_leaf_cancel`, `ok_logup_unused_write_m0`, `ok_logup_repeated_cold_reads`, `ok_logup_input_records`.
- Negative mutants:
  - `multiplicity_uncommitted`
  - `activity_uncommitted`
  - `tuple_uncommitted`
  - `inverse_unconstrained`
  - `inactive_term_nonzero`
  - `zero_denominator_active`
  - `read_entry_not_charged`
  - `zm_imported_tail_forged` (two leaves: leaf 2 imports a tail record that differs from the record leaf 1 appended; every commitment binding of invariants 1 to 3 holds, because leaf 2 commits the forged record before the challenges; refused at the seal because the run sum is not zero. Control: the circuit without the imported-tail debit accepts it and serves the forged record to a read)
  - `zm_imported_row_not_debited_when_unused` (an imported row that no request uses, left out of the sum; the committed-entry binding refuses it)
  - `read_before_write_balanced_later` (a read at `addr >= len_cur` balanced by a later write; refused by `addr < len_cur`)
  - `address_written_twice`
  - `challenge_not_derived`
  - `commitment_substituted_in_node`
  - `commitment_not_subgroup_checked`
  - `seal_sum_unchecked`
  - `checkpoint_resets_debt`
- Real-proof tests (not MockProver): committed-instance binding, and the `C_i` link to `d_root` in the node.

#### Lean obligations

| Obligation | File | Premises | Contract | Status |
|---|---|---|---|---|
| Write-once LogUp: if the run sum is zero under challenges derived after all `C_i`, every read returns the value written at its address, except with the stated error. | `clean/fork/Clean/Sherlock/WriteOnceLogUp.lean` (new) | KZG binding, Fiat-Shamir, Schwartz-Zippel; the library has no field or polynomial theory (`FD/EXPERIMENTS-C3.md:92`) | invariants 1-6 | open |
| `BusBalanced` gives `LookupSound` (`balanced_lookup`) | `ModularComposition.lean` | none beyond C3 | bus step | proved in C3 |
| The statement-level SRC-023 under Profile B: the public `[prog_d, input_d, outcome_d, n_steps]` is independent of the cut. Intermediate values are not. | `WriteOnceLogUp.lean` (new) | the premise above, `result_unique_tail` | SRC-023 | open |

#### Code skeleton

```rust
// crates/mori-chip/src/zmlogup.rs (new; cargo feature "profile-b"; not built before M4)
pub struct ZmLogUpCfg { /* private: committed column, scalar column, challenge cells */ }
pub struct RunChallenges { pub gamma: F, pub alpha: F }       // one pair default
/// Canonical P_i: field order, lengths, zero padding.
pub fn encode_precommit(leaf: &LeafEntries) -> Vec<F>;
/// Hash of the ordered commitments, then the challenges.
pub fn derive_challenges(d_root: F, prog_d: F, input_d: F) -> RunChallenges;
impl MemBackend for ZmLogUpCfg {
    // persist: emits write entries bound to committed cells; returns an empty token; the leaf sum is a separate output cell.
    // cold_read: emits a read entry bound to committed cells; serves the record.
    // const CAPS: BackendCaps { lines: false } unless M4 decides to build lines (then 8 read entries
    // per line). open_line is never reached when lines = false: check_shape refuses k_page > 0.
}
```

The node and seal changes live in `crates/mori-recursion/` (Area 3).

#### Agent reads / must not need to read

Reads:
- this kit
- `crates/mori-chip/src/{membackend.rs, bus.rs, rfcore.rs:600-610,830-844}`
- `midnight-proofs` `plonk/prover.rs:32-49`
- `midnight-circuits` `verifier/verifier_gadget.rs:286-316,604-638`
- `ecc/foreign/weierstrass_chip.rs:343-360`
- LC-AUDIT finding 9

Must not need: CORE rules, `dense.rs` internals, VALUE DIGEST.

#### Dependencies

MEM, CORE (item 11), BOUNDARY COMMITMENT (Horner variant), NODE INTEGRATION, L AND WRAP/seal, CONTINUATION (Area 3), Init (initial records).

---

### Kit: VALUE DIGEST

#### Purpose

VALUE DIGEST computes exactly `sherlock-value/1` (`O/design.md:175-191`), bit for bit (L-3). It binds each `vd` to the write-once record that the creating step writes (`O/design.md:193`). It gives `outcome_d` at halt and `val_d = vd(x)` for `input_d` (`O/design.md:192`).

#### Interface

Hash: zk-stdlib Poseidon. The input length is in the initial capacity (`O/design.md:175`; `poseidon.rs:7-10,107-120`), and words are absorbed two per permutation from the first word. The capacity tag is `iv = 0`: E-2 tags are for memory hashes only (L-3). `V = dom("SHK_VALUE_V1")`, `O = dom("SHK_OUTCOME_V1")`.

The seven forms are those of `O/design.md:179-185`. A reference (`:186`) has no own preimage: it takes the `vd` of its target. Tag 7 is the constructor fold step, not a form.

| Form (`O/design.md` line) | Preimage | Words | Constant chunks (folded) | Runtime permutations (with the HASH constant-start mode) |
|---|---|---|---|---|
| `UInt n` (:179) | `[V,1,0,0,n]` | 5 | `(V,1)`, `(0,0)` | 1 (tail chunk `n`) |
| `Bool b` (:180) | `[V,1,1,0,b]` | 5 | all | 0: `VD_FALSE` and `VD_TRUE` constants, selected by `b` |
| `Nil` (:181) | `[V,1,2,0]` | 4 | all | 0: `VD_NIL` constant |
| `Cons(h,t)` (:182) | `[V,1,3,2,vd(h),vd(t)]` | 6 | `(V,1)`, `(3,2)` | 1 |
| `Leaf(x)` (:183) | `[V,1,4,1,vd(x)]` | 5 | `(V,1)`, `(4,1)` | 1 (tail) |
| `Branch(l,r)` (:184) | `[V,1,5,2,vd(l),vd(r)]` | 6 | `(V,1)`, `(5,2)` | 1 |
| `Constructor` (:185), field step | `acc_(k+1) = H(V,1,7,2,acc_k,vd(f_k))`, `acc_0 = 0` | 6 | `(V,1)`, `(7,2)` | 1 per field |
| `Constructor` (:185), completion | `[V,1,6,m,ty_d(T),c,acc_m]` | 7 | `(V,1)` only (`m` varies) | 3 (`(6,m)`, `(ty_d,c)`, tail `acc_m`) |
| Outcome (:191) | `H(O,1,0,vd(v))`, `H(O,1,1,code)`, `H(O,1,2,kind)` | 4 | `(O,1)` | 1 |

The constants are indexed by preimage length and form (LC-AUDIT item 23), never one universal prefix (L-3).

**New requirement on HASH.** HASH must be able to start a sponge from a fixed 3-word state `K` (the state after the constant chunks) instead of `(in0, in1, len)`. A proposal: fold `K` into the absorb row's three round-constant columns, as `len` is folded today (`poseidon.rs:22-24`). This is not checked. Without it, the costs are those of `O/design.md:200-204`.

**Whole-sponge slots.** VALUE DIGEST runs each sponge in `PoseidonCfg` slots with copy-constrained inputs (as `dense.rs:650-651`). It never uses a chain of 2-input bus requests: a HASH provider serves only 2-input hashes (`poseidon.rs:24-26`, `:602`), and a chain that passes one output word is a different hash (LC-AUDIT finding 2b).

Inputs and requests (B2 finding 4). VALUE DIGEST issues no bus request of its own. Every record and every ROM row it uses arrives as an `AuthRec` or `AuthRom` token of CORE's read port (section 1.3). The token is the complete read contract: its address is copy-linked to the reference cell that the rule holds, its kind is the kind the call site expects, `addr < len_cur` holds at the running length of that read point (so a record appended later in the same leaf cannot be read as a child), and for a ROM row the row index is ranged. The skeleton's "authenticated" parameter types are these tokens and nothing weaker.
- Child records: `AuthRec` for each child, with the kind that the rule constrains; the child `vd` lane is a cell of that token.
- ROM rows: `AuthRom` with `tbl = tbl_types` for `ty_d`, `tbl_ctors` for the ordinal and arity, `tbl_consts` for the constant `vd`. All are under the PROG canonicality premise (provider: the `ADM` claim of ADMIT-WORK, checked at the seal by NODE INTEGRATION; AD-14 fixes only the `ty_d` normal form inside it). Table ids are open (V-2).
- RANGE: `n < 2^128` (128-bit shape, `range.rs:1-10`), unless `n` is copied from an ALU output that ALU ranges (LC-AUDIT item 21); `code` below 2^8; `kind` in `0..7`; `m` and the cursor. The 8-bit and `0..7` shapes are new requirements on RANGE (only the 128-bit shape exists, `range.rs:1-10`); a small bound can also be a gate or a 128-bit check of `bound - 1 - x`.
- HASH: whole-sponge slots.

Named copy-constrained cells:
- `vd`: copied into the record's digest word. Where the digest is carried is open (`O/design.md:193`).
- `b`: boolean before the `VD_TRUE`/`VD_FALSE` selection.
- Fold cells `(cursor, arity m, acc, ty_d, ordinal c)`: CORE boundary cells (`astra-area2.md:794`) that cross leaf boundaries.
- `outcome_d`: copied to the terminal opening.
- `val_d`: the value-digest word of the `input_d` preimage. Kit INIT owns the opening of `input_d = Poseidon(SHK_IN, val_d, fuel_lo, fuel_hi, stack_limit)` and the authentication of the input value behind `val_d`; VALUE DIGEST supplies the digest relation it uses.

Bounded value opening (B2 finding 6; used by INIT's input admission and by any consumer that inspects a value it holds only by digest). `open_value_node(vd, form)` proves, in one call of bounded work, that a digest `vd` is the digest of one node of the given form with the stated scalar or the stated child digests: one sponge for `UInt`, `Cons`, `Leaf`, `Branch`; a constant comparison for `Bool` and `Nil`. A constructor of arity `m` is opened by the fold, one field per call, with the same fold cells `(cursor, m, acc, ty_d, c)` that construction uses; the fields are bound to `vd` only when the completion hash equals `vd`, and a rule may use a field only after that completion (the fold cells carry an `opened` flag that the consuming rule requires). No call scans a whole value, and no bound on the size of a value is introduced. A consumer that needs only equality of two values compares their digests and opens nothing. The bottom-up fold stays the rule for construction. For an input under admission form (b) (AD-17), a rule reads a field through an `AuthIn` token of the read port under the certified `view_root` and does not fold: the `VIEW` claim has already checked every record against `sherlock-value/1`.

#### Invariants

Guaranteed:
1. Every digest uses the exact tag, version, arity, field order and preimage length of `O/design.md:179-191`.
2. The stored `vd` is computed from the tag and the `vd` lanes of the children (LC-AUDIT finding 2).
3. Each child `vd` is read from a record whose kind the rule constrains.
4. `b` is boolean before the selection.
5. The constructor fold advances by one field position, and completion requires `cursor = m`.
6. A constant and dynamic data of the same value have equal `vd`. Shared and unfolded structure have equal `vd`.
7. Halt copies the result's `vd`. `Running` has no `outcome_d`.
8. Padding, a dangling reference, an out-of-range scalar and a wrong arity are not values.

Assumed:
- CORE's read port returns only tokens that meet its certificate (address provenance, expected kind, `addr < len_cur` at the read point, ranged ROM row); MEM and PAGE/ROM serve records by key and kind.
- ROM rows are the `prog_d` rows (INIT ties `prog_root` to `prog_d`).
- `val_d` and the genesis state are authenticated by INIT.
- `ty_d` and constant digests are canonical (PROG premise, `O/design.md:84`, discharged at the seal by the `ADM` claim of ADMIT-WORK; `decl_schema_d` normal form open, dissent D-2).
- HASH computes the zk-stdlib sponge.
- CORE keeps the fold cells across leaves and writes scratch before it reads it (R-8).

Exhaust kind 7 (`LogicalCap`) is in the encoding range (`O/design.md:191`). Profile `/1` rejecting it is a recorded conflict (`astra-area2.md:780`); the encoding does not authorize it.

#### Row budget

Without the constant-start mode: `O/design.md:200-204` (ESTIMATE)
- 210 rows for UInt, Bool and Leaf
- 140 rows for Nil
- 210 rows for Cons and Branch
- 210 rows per constructor field step
- 280 rows at completion
- 140 rows per outcome

With the constant-start mode (ESTIMATE, 70 rows per permutation, unmeasured):
- UInt, Leaf, Cons, Branch: 70 rows each
- Bool, Nil: 0
- field step: 70 rows
- completion: 210 rows
- outcome: 70 rows

Leaf: `O/design.md:205` gives about +1,680 rows for 8 appends without folding (ESTIMATE). Node: 0 new columns if `K` is folded into the existing round-constant columns (ESTIMATE, not checked).

#### Verification

- Quint: new `models/sherlock/quint/valueDigest.qnt`, a structural value model with separate physical representations. Invariants: `VdRelocationInvariant`, `VdConstDataEqual`, `VdSharingEqual`, `VdFormsDisjoint` (distinct `(length, tag, arity)`).
- Differential test `vd_differential_every_form`: every form and every outcome against a reference written from the text of `O/design.md:175-191`, including UInt 0 and `2^128 - 1`, and constructors with 0, 1 and 3 fields. No reference file exists today (L-10).
- MockProver positives:
  - every builtin form
  - a constructor fold split across leaves
  - constant and data representations of the same value
  - shared and duplicated structure
  - input and consumed-output digest parity
- Negative mutants:
  - `host_supplied_vd`
  - `wrong_child_digest`
  - `child_kind_unconstrained`
  - `wrong_value_tag`
  - `wrong_arity`
  - `field_order_swapped`
  - `fold_cursor_skipped`
  - `fold_accumulator_reset`
  - `raw_type_id_in_digest`
  - `constructor_ordinal_unbound`
  - `scalar_out_of_range`
  - `bool_nonboolean_selection`
  - `digest_not_copied_to_record`
  - `running_has_outcome`
  - `padding_as_value`
  - `vd_single_prefix_constant` (one start state for all lengths)
  - `vd_bus_rehash_chain` (2-input bus chain in place of one sponge)
  - `vd_capacity_tag_added` (`iv != 0`)
  - `vd_child_future_record` (the child record is at an address at or above the running length of the read point: a record appended later in the same leaf; refused by the read port. Control: a port that checks membership in the window only)
  - `vd_child_wrong_reference` (the child record is read at an address other than the reference the rule holds; refused by the copy link of the port)
  - `vd_rom_row_overflow` (a `ty_d` or constant row requested at `row = 2^D_row`, which aliases row 0 of the next table; refused by the ranged row of `read_rom`)
  - `vd_open_field_used_before_completion` (a constructor field used by a rule before the opening fold reached the digest)
  - `vd_open_node_wrong_child` (an opened node whose stated child digest differs from the hashed one)

#### Lean obligations

| Obligation | File | Premises | Contract | Status |
|---|---|---|---|---|
| Encoding injectivity: distinct values (and distinct outcomes) give distinct preimage sequences of the stated lengths. | `clean/fork/Clean/Sherlock/ValueDigest.lean` (new) | none (structural); `n < 2^128 < r` | invariant 1 | open |
| Equal digests imply equal values. | `ValueDigest.lean` | collision resistance of H (a `digest_inj`-style premise) | invariants 1, 6 | open |
| Layout independence: the `vd` of a shared/relocated structure equals the `vd` of its unfolded tree. | `ValueDigest.lean` | none | invariants 2, 3, 6 | open |
| Constant start: a sponge from the precomputed state `K(len, form)` equals the full sponge over the preimage. | `ValueDigest.lean` | an abstract permutation | constant table | open |
| VALUE DIGEST is a `Module` with a `Contract` (every provided `vd` is the digest of the decoded value). | instance over `ModularComposition.lean` | `provided_valid` (proved in C3) | invariants 1-8 | open (instance) |

#### Code skeleton

```rust
// crates/mori-chip/src/value/{mod.rs, encoding.rs, digest.rs} (new)
pub struct ValueDigestConfig { /* private */ }
pub struct ValueDigestCells { pub vd: Cell }
pub struct ConstructorFoldCells { /* cursor, arity, acc, ty_d, ordinal */ }
/// Fixed start state after the constant chunks of a form; length-indexed.
pub fn start_state(len: usize, head: &[F]) -> [F; 3];
pub fn vd_nil() -> F; pub fn vd_bool(b: bool) -> F; // constants, 0 permutations
impl ValueDigestConfig {
    /// UInt/Bool/Nil/Cons/Leaf/Branch digest from authenticated fields and ranges.
    pub fn constrain_builtin(&self, l: &mut impl Layouter<F>, v: &AuthenticatedBuiltinCells) -> Result<ValueDigestCells, Error>;
    /// Exactly one ordered field extends the fold.
    pub fn fold_field(&self, l: &mut impl Layouter<F>, pre: &ConstructorFoldCells, f: &AuthenticatedFieldCells) -> Result<ConstructorFoldCells, Error>;
    /// Completion at cursor = arity; binds vd to the appended record.
    pub fn finish_constructor(&self, l: &mut impl Layouter<F>, fold: &ConstructorFoldCells, rec: &[Cell; 4]) -> Result<ValueDigestCells, Error>;
    /// outcome_d of a proved terminal outcome.
    pub fn constrain_outcome(&self, l: &mut impl Layouter<F>, o: &TerminalOutcomeCells) -> Result<Cell, Error>;
    /// Bounded opening of one node held by digest: vd is the digest of `node` (scalar or child digests).
    pub fn open_value_node(&self, l: &mut impl Layouter<F>, vd: &Cell, node: &OpenedNodeCells) -> Result<(), Error>;
    /// One field of a constructor opening; fields become usable when the fold completes at vd.
    pub fn open_field(&self, l: &mut impl Layouter<F>, pre: &ConstructorFoldCells, field_vd: &Cell) -> Result<ConstructorFoldCells, Error>;
}
// AuthenticatedBuiltinCells and AuthenticatedFieldCells hold AuthRec / AuthRom tokens (section 1.3);
// they cannot be built from bare cells.
/// Canonical preimage from the specification text, for host/circuit differentials.
pub fn encode_value_preimage(v: &CanonicalValueNode) -> Vec<F>;
// crates/mori-chip/src/poseidon.rs (HASH kit): sponge slot starting from a fixed state.
```

#### Agent reads / must not need to read

Reads:
- `O/design.md:174-212`
- `crates/mori-chip/src/poseidon.rs`
- this kit's encoding table
- the `AuthRec` and `AuthRom` port types of section 1.3 (not the MEM or ROM request formats: it issues none)
- the CORE fold-cell contract

Must not need: Merkle walks, LogUp, recursion keys, ledger code.

#### Dependencies

HASH (the constant-start mode), RANGE, ALU (ranged outputs), CORE (the read port for every record and ROM row; fold cells, R-8), BOUNDARY COMMITMENT (fold cells in the schema), INIT (`val_d`). MEM and PAGE/ROM only through CORE's read port.

---

### Kit: BOUNDARY COMMITMENT

#### Purpose

BOUNDARY COMMITMENT binds every state cell that is needed to continue execution, at leaf entry (`pre_d`) and at leaf exit (`post_d`), through one interface with two implementations:
- **Poseidon digest: the default, Profile A** (LC-AUDIT item 25).
- **Horner fingerprint: benchmark-only, Profile B only**, paired with ZmLogUp (L-5).

#### Interface

`BoundarySchema`: a fixed, keyed, ordered list of words, in two parts: six header words (item 5) and the body words (items 1 to 4):
1. The CORE persistent registers, every one that is readable in the future. Area 1 gives the full schema. The spike's 29 words are not the final count.
2. The constructor fold cells.
3. The FRAMES and TASKS hash-chain heads (R-3) and their counts.
4. The memory state of the one arena: `(len, root, tail_d)`, with `len` from MEM and `[root, tail_d]` from the dense token. With four arenas (not the default, L-9) the triple appears per arena. Under Profile B there is no `root` or `tail_d`: the memory words are `len` and the imported tail records (or their committed entries), and memory correctness rests on the run-wide sum at the seal.

5. Header words (lead merge decision, section 2 lead notes; N-2, N-14, V-6; revised by B2 finding 5): the terminal kind `outcome_tag`, the terminal payload `outcome_payload` (the `vd`, the code or the kind that `outcome_d` wraps), `input_d`, the dependency commitment `deps_d`, and `view_root` (section 1.11, FP-5). They are CORE registers; this item fixes that they are the header of the commitment and are never packed, nested or moved into the body. `prog_d` and `prog_root` are body words; `prog_d` is also leaf lane 1, and the leaf constrains the lane to equal the body word in every leaf. `input_d`, `prog_d` and `prog_root` are in no rule's write set. `view_root` is the root of the certified input view under input admission form (b) (AD-17, kit INIT) and `VIEW_NONE` under forms (a) and (c) or for an absent input; INIT sets it at genesis, it is in no rule's write set, and the read port `read_input` reads it inside the leaf.

Scratch `v0..v2` are outside the schema only under the independence-from-incoming-scratch obligation of CORE (invariant 8). Zeroing at a boundary or at slot 0 does not replace it.

**Poseidon (default): a header and body commitment.** A flat sponge over all words gives no selective opening: to read one word a verifier must re-hash every word. The landed decider opens the outcome with one short hash only because the landed digest is `Poseidon(dom("SHK_SD"), outcome_tag, outcome_value, lanes_d)` with an opaque `lanes_d` (`crates/mori-recursion/src/ivc.rs:81-84`, `run_decider.rs:284-288`). The design keeps that structure and widens the header:

- `body_d = H_iv(DOM_STATE_BODY, schema_id; body words)`: the capacity-tagged internal sponge (HASH family 2) over the body words in keyed order. The tag is a capacity word, so it costs 0 permutations; its value differs from every memory, stack and ROM tag (the `StateBody` entry of the domain registry). `schema_id` identifies the schema and the profile.
- `state_d = Poseidon(dom("SHK_SD2"), outcome_tag, outcome_payload, input_d, deps_d, view_root, body_d)`: the halo2 prototype form of the header. The frozen BabyBear form is `P2(SHK_SD2, outcome_tag, outcome_payload, input_d, deps_d, view_root, body_d)`, 42 words, 6 permutations (section 1.11, FP-5). `pre_d` and `post_d` are `state_d` of the first pre state and of the last post state. The domain word `SHK_SD2` is new and differs from the landed `SHK_SD`, because the header has more words (name proposed here).
- Equality constraints: each header word and each body word is a copied cell of its owner (CORE, STACKS, MEM, the backend token); `body_d` is the copied output of the body sponge; the lane cells `pre_d` and `post_d` are the copied outputs of the header sponges.
- Why the header is pinned and not capacity-tagged: NODE INTEGRATION (SEAL and SEAL-WITH-DEPS, the only seal path) recomputes it with the library sponge (`std.poseidon`, as `run_decider.rs:287` does), which has no capacity-tag input. LC AUDIT item 25 asks for a capacity-tagged boundary digest; the body keeps that form and the header deviates from it for this reason (`FD/ARCH-AUDIT-RESPONSE.md`).
- Inputs are copied cells. The input lengths are fixed by the schema.
- One pass, no challenges, the committed instance column unused.
- Outputs: `pre_d` and `post_d` go to the leaf lanes `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]` (`crates/mori-recursion/src/ivc.rs:70-74`), lanes 2 and 3. The genesis gate is `h_pre == 0`: kit INIT constrains the header and body cells of the first pre state to `Init(prog_d, input_d)` (`O/design.md:172`).

Typed opening interface (the three uses the B1 text named without a signature):

| Use | Where | What it does | Cost |
|---|---|---|---|
| INIT | in the leaf | no opening: INIT reads the header and body cells directly, because the leaf computes `pre_d` from those cells (`BoundaryCells`, returned by `commit`) | 0 |
| Terminal sealing | NODE INTEGRATION (SEAL, SEAL-WITH-DEPS) only; `L` opens a SET head (section 1.11, FP-3), not a state header | `open_state_header(state_d, witness) -> StateOpeningCells { outcome_tag, outcome_payload, input_d, deps_d, view_root, body_d }`: assigns the six header words, computes `P2(SHK_SD2, header words)` and asserts equality with the authenticated lane `post_d` | one 42-word Poseidon2 sponge over BabyBear, 6 permutations (section 1.11, FP-5); ESTIMATE, measured by X10 |
| Continuation | host only (mode 1) | no circuit opening: the suffix leaf's `pre_d` lane equals the prefix `post_d` by NODE MERGE; the host recomputes `body_d` and `state_d` when it validates a restored checkpoint (kit HOST CHECKPOINT STORE) | 0 circuit rows |

The bridge between leaf cells and recursion cells is the public lane: the recursive verifier authenticates the leaf's `post_d` lane, and the recursion re-hashes the header with the library sponge over the same field and the same pinned parameters as the leaf's `sponge`. The test of the bridge is a differential vector `boundary_header_leaf_vs_std` (the leaf chip's header digest equals `std.poseidon` on the same six words, including zero words). A full re-hash of the body in the node is never needed and is not budgeted.

**Horner (Profile B only).**
- The schema words of both endpoints are copied into the committed instance column before any challenge is drawn.
- `fp = Σ words[i] beta^i`, with `beta` derived from `d_root` under its own domain label.
- Leaf cells are equal to the committed words.
- It needs the Profile B leaf statement (L-5).
- It is not offered with the dense backend.

Bus: BOUNDARY COMMITMENT requests and provides no bus tuple. It uses HASH sponge slots with copied inputs.

Range checks: `len` by MEM; register ranges by CORE/DEC. The boundary checks no ranges.

#### Invariants

Guaranteed:
- Each output is derived from the exact serialization; every schema field occurs once, in keyed order.
- Poseidon equality implies state equality under collision resistance. The digest instantiates the `digest_inj` premise (`Aggregation.lean:201`); it does not discharge it (LC-AUDIT minor 18).
- Horner equality implies state equality only with the precommitment, a correct challenge derivation and the polynomial error. Over `N` words, one boundary has error `(N - 1) / r`, and the union over all boundaries must be included (ESTIMATE model; the bound model is to be derived).

Assumed: CORE, MEM and the backend expose their true boundary cells and omit nothing. Lines change no boundary value (PAGE/ROM item 9).

**SRC-023 partition independence for each implementation.**
- **Poseidon with the dense backend.** The physical state `(len, root, tail_d)` after any cut of the steps into leaves is the abstraction of the logical state. Proved in C3 for cuts whose leaves hold at most 8 appends in total: `partition_independent_flush` and `cuts_agree` under `FitsLeaf`, and `Mem.wf_unique` for the canonical `tail_d`. The circuit accepts a `W = 1` leaf with up to `15 - r_pre` appends, and `W > 1` leaves; for those cuts the evidence today is the MockProver tests (`c1_partition_independence`, `c1_two_flush_units`) and the Quint lane, and the theorem is the open `FitsWindow` obligation of kit MEM (B2 finding 8). Registers and stack heads are a function of the step count (`pstep_det`, `chainHash_inj`, proved in C3). Lines open nothing into the boundary (item 9). So `pre_d`/`post_d` at every common cut point are equal across partitions, and `result_unique_tail` (proved in C3, with the `digest_inj` and `stuck` hypotheses) gives one public statement `[prog_d, input_d, outcome_d, n_steps]`.
- **Horner with ZmLogUp.** `fp` depends on `beta`, which depends on all `C_i`, which depend on the partition. So intermediate fingerprints differ between cuts. Only the public statement is independent, on the probabilistic premise (KZG binding, Schwartz-Zippel, Fiat-Shamir) that replaces `digest_inj`. This needs a new proof (open). A Horner fingerprint is not a portable checkpoint identifier.

#### Row budget

- Poseidon: `2 x 70 x (ceil(N_body / 2) + 3)` rows per leaf (ESTIMATE): the body sponge and the three-permutation header, at both endpoints. Examples:
  - `N_body = 31` (29 spike registers + `root` + `tail_d`): 2,240 rows for the bodies (`FD/EXPERIMENTS.md` composed ESTIMATE) plus 420 for the two headers;
  - `N_body = 34` (adding `len` and two stack heads): 2,380 + 420 rows (ESTIMATE).

  The final `N_body` comes from the Area 1 schema.
- Opening in NODE INTEGRATION (SEAL and SEAL-WITH-DEPS): one 42-word header sponge per opened state (section 1.11, FP-5; ESTIMATE). `L` opens no state header; `L` opens the sealed SET head (section 1.11, FP-3). The landed decider's four-input opening is inside the measured 929,166 rows and is removed. Measured by X10 inside the node.
- Horner: up to `4N + O(1)` rows (ESTIMATE, `astra-area2.md:1000`), plus the committed binding.
- Node: Poseidon adds nothing; Horner is part of the Profile B node changes.

#### Verification

- Quint: new `models/sherlock/quint/boundaryCommitment.qnt`. Serialization is separate from commitment. The existing `aggregation.qnt` takes an injective digest as a premise; Horner uses a conditional premise in its place.
- MockProver positives:
  - `ok_boundary_poseidon_two_leaf_stitch`
  - every terminal outcome
  - a constructor fold and stack heads across a boundary
  - `ok_boundary_partition_three_cuts` (full leaves, `S = 1`, mixed cuts; compare `pre_d`/`post_d` at common cuts and the public statement)
- Negative mutants:
  - `boundary_word_omitted`
  - `boundary_word_reordered`
  - `boundary_tail_omitted`
  - `boundary_len_omitted`
  - `stack_head_omitted`
  - `post_uses_pre_cell`
  - `memory_token_unbound`
  - `schema_id_unbound`
  - `boundary_tag_equals_memory_tag`
  - `scratch_in_digest` (a scratch lane hashed into `body_d`: the digest then depends on a value outside the state; with CORE's `core_scratch_incoming_free`)
  - `terminal_opening_unbound`
  - `header_word_moved_to_body` (a header word hashed only inside `body_d`, so the opening returns a free word)
  - `header_body_d_unlinked` (`body_d` in the header not copied from the body sponge)
  - `header_domain_landed_sd` (the header hashed under the landed `SHK_SD` domain)
  - `prog_d_lane_differs_from_body` (leaf lane 1 not equal to the `prog_d` body word)
  - positive differential `boundary_header_leaf_vs_std`
  - `boundary_commitment_substituted`
  - Profile B: `horner_words_uncommitted`, `horner_beta_free`, `horner_beta_mismatch`, `horner_two_word_compensation` (two words changed with the fingerprint kept; the committed implementation must refuse)

#### Lean obligations

| Obligation | File | Premises | Contract | Status |
|---|---|---|---|---|
| The serialization is injective and complete on canonical states (fixed length, fixed order, every carrier). | `clean/fork/Clean/Sherlock/BoundaryCommitment.lean` (new) | none | the schema | open |
| A Poseidon boundary instantiates `digest_inj` for the physical state: equal `state_d` gives equal header words and equal `body_d`, and equal `body_d` gives equal body words. | `BoundaryCommitment.lean` | collision resistance of both sponges, distinct domains | Poseidon equality | open |
| Header opening: an accepted `open_state_header` against `state_d` returns the header words of the state that `state_d` commits. | `BoundaryCommitment.lean` | collision resistance of the pinned sponge | the SEAL and SEAL-WITH-DEPS reads of `outcome_tag`, `outcome_payload`, `input_d`, `deps_d` | open |
| Equal physical state at common cuts | `PartitionIndependence.lean` (`partition_independent_flush`, `cuts_agree`, `Mem.wf_unique`) | `FitsLeaf` (at most 8 appends per leaf); the circuit's capacity needs `FitsWindow` (open, kit MEM) | SRC-023, memory part | proved in C3 under `FitsLeaf` |
| A unique public result over partitions | `PartitionIndependence.lean` (`result_unique_tail`) | `digest_inj`, `stuck` (hypotheses) | SRC-023 | proved in C3 |
| A Horner boundary is sound under the precommitment. | `BoundaryCommitment.lean` (new) | KZG binding, Fiat-Shamir; needs field theory (C3:92) | Horner equality | open |

#### Code skeleton

```rust
// crates/mori-chip/src/boundary/{mod.rs, schema.rs, poseidon.rs, horner.rs} (new)
pub struct BoundarySchema { /* fixed ordered field descriptors, schema_id */ }
pub struct BoundaryCells { /* named state cells: regs, fold cells, stack heads, len, token */ }
pub struct HeaderCells { pub outcome_tag: Cell, pub outcome_payload: Cell, pub input_d: Cell, pub deps_d: Cell }
pub struct SerializedBoundaryCells { pub header: HeaderCells, pub body: Vec<Cell> }
pub struct StateDigestCells { pub state_d: Cell, pub header: HeaderCells, pub body_d: Cell }
/// Every schema field once, in keyed order; the four header words apart from the body.
pub fn serialize_boundary(s: &BoundarySchema, st: &BoundaryCells) -> Result<SerializedBoundaryCells, LayoutError>;
pub trait BoundaryCommitment {
    /// The output binds the serialized, constrained boundary. Poseidon: body sponge, then header sponge.
    fn commit(&self, l: &mut impl Layouter<F>, words: &SerializedBoundaryCells) -> Result<StateDigestCells, Error>;
}
/// Native header digest, shared by the leaf witness generator, the recursion and the host store.
pub fn state_digest_native(header: [F; 5], body_d: F) -> F;

// crates/mori-recursion/src/state_open.rs (new): the recursion side of the opening.
pub struct StateOpeningCells { pub outcome_tag: AssignedNative<F>, pub outcome_payload: AssignedNative<F>,
                               pub input_d: AssignedNative<F>, pub deps_d: AssignedNative<F>, pub view_root: AssignedNative<F>,
                               pub body_d: AssignedNative<F> }
/// Assign the six header words, hash [dom("SHK_SD2"), words] with the library sponge, assert equality with state_d.
pub fn open_state_header(cx: &mut RecursionContext<'_>, state_d: &AssignedNative<F>, w: &HeaderWitness) -> Result<StateOpeningCells, Error>;
pub struct PoseidonBoundaryConfig { /* private */ }        // Profile A
/// Sealed marker: the backend copies its boundary words into the committed instance before any challenge.
/// Only ZmLogUpCfg implements it, so Horner with the dense backend does not compile (section 1.4).
#[cfg(feature = "profile-b")] pub trait PrecommitsInstance0: sealed::Sealed {}
#[cfg(feature = "profile-b")] pub struct HornerBoundaryConfig<B: MemBackend + PrecommitsInstance0> { /* committed column, beta cell */ }
```

#### Agent reads / must not need to read

Reads:
- this kit
- `crates/mori-chip/src/{membackend.rs, dense.rs:112-127, poseidon.rs}`
- the Area 1 register schema
- `crates/mori-recursion/src/ivc.rs:60-80`
- `O/design.md:166-172`
- `clean/fork/Clean/Sherlock/Aggregation.lean:185-215`

Must not need: rule bodies, Merkle path internals, LogUp internals (Poseidon agent), ledger code.

#### Dependencies

CORE (register schema, R-8), MEM (`len`), the dense backend token, hash-chain stacks, VALUE DIGEST (fold cells), INIT (reads the cells of the first pre state), NODE INTEGRATION and L AND WRAP (the header opening, the lanes), and ZmLogUp (Horner only).

#### Conventions for the recursion kits (NODE INTEGRATION to GP-4 hook)

- Area 3 kits use no leaf bus tuple. They talk through named cells inside one circuit and through public-instance lanes across proof boundaries. The recursive verifier authenticates a child's instance lanes; the parent copy-constrains them into its own cells (A:51-55).
- Leaf statement: the seven lanes `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]` of `sherlock-leaf-stmt/1`, genesis gate `h_pre == 0`. SRC-001 defines the fifth lane as the hash `rules_d` (`openspec/changes/sherlock-vm-consensus-20261107/requirements.md:26`); the spike code carries a lane named `rule` (`crates/mori-recursion/src/ivc.rs:70-74`). CORE computes `rules_d` (kit CORE), and NODE INTEGRATION treats lane 5 as that hash and renames the lane in `ivc.rs`. Both drafts add lanes (A:80-93, G:42-53); that conflicts with the landed lanes and is open (section 4.2, N-2). The kits below use only the seven lanes. Values that a seal needs and that are not lanes (`input_d`, the dependency commitment, the terminal kind and payload) are read with `open_state_header` on the final state digest `post_d` (kit BOUNDARY COMMITMENT; recommendation N-2).
- Segment statement `SEG = (prog_d, pre_d, post_d, h_lo, h_hi)`: the `Seg` of `clean/fork/Clean/Sherlock/Aggregation.lean:27`. Sealed statement `SET = (set_root)`, with the typed grammar of the GP-2 hook. Two more statement types serve dependency discharge (GP-2 hook): `SUB = (set_root, sub_root)` and `DL = (chain_d, cover_root, in_link)`. The GP-3 general verifier adds `FV = (fv_state_d)`; its registry root is the node's `reg_root` lane. Program admission adds `AW = (adm_state_d)` (mode ADMIT-WORK of NODE INTEGRATION). The input view adds `VW = (view_state_d)` (mode VIEW-WORK, input admission form (b), AD-17). Run statement `[prog_d, input_d, outcome_d, n_steps]`; `claim_d = H(SHK_RUN, 1, prog_d, input_d, outcome_d)` (SRC-007, `requirements.md:32`).
- Claim count (R-5, merge decision): the public statement carries NO claim count. `set_root` binds the claim DAG. If a count is wanted for telemetry, it is a count of distinct claims, it is not range-checked and it carries no soundness (OWNER to confirm).
- Profile B (ZmLogUp memory + Horner boundary) is benchmark-only (L-5), not built before M4. It appears only in the extension point marked "PROFILE B EXTENSION POINT (benchmark-only)" in NODE INTEGRATION. It has no stop/resume, so CONTINUATION does not support it.
- Row figures for this design are ESTIMATE. The spike figures (node 823,466 rows, decider 929,166 rows; `openspec/changes/sherlock-vm-consensus-20261107/design.md:129-131`) were measured on the spike key, not on this design, and are used only as ESTIMATE inputs. Only C1 rows in `FD/EXPERIMENTS.md` are MEASURED (R-10).
- Lean: no area 3 obligation is proved. C3 theorems (`FD/EXPERIMENTS-C3.md`) appear only as premises. Pre-C3 theorems of `Aggregation.lean` (for example `linkAll_append`, `bracket_independent`, `decider_pinning`, `result_unique`) exist but are not "proved in C3".


### Kit: NODE INTEGRATION

#### Purpose
Verify one or two child proofs in a universal STARK node of fixed arity, link execution segments, merge the children's deferred field log digests by `TAG_NODE`, check each child's key commitment against `key_root`, and seal a terminal segment into a claim. It hides leaf internals from `L` and the hooks, and it is the only place where child keys are routed (A:177; G:29; `PR` sections 3.1 and 4.3).

#### Interface
No bus tuples. Named cells and instance lanes:

Requests (per child slot `j` in {0, 1}):
- `en_j`: bit. `key_sel_j`: one-hot over {LEAF, NODE, FOREIGN}, summing to `en_j` (G:64; A:194 with the 2-leaf lift kind removed, L-10). NODE INTEGRATION constrains booleanity and the sum. The selector also fixes the permitted `stmt_tag` of the child: `LEAF -> {SEG}`, `FOREIGN -> {SET}`, `NODE -> {SEG, SET, SUB, DL, FV, AW, VW}`.
- The child's instance lanes. A LEAF child gives the seven landed lanes. A NODE child gives the node statement below. A FOREIGN child gives the GP-3 hook's output statement.
- Whether a LEAF child enters a NODE slot directly or through a one-leaf lift relation (the landed `SHKIVC06` shape) is open (section 4.2, N-1). The two-leaf lift is not part of the design (L-10).
- From BOUNDARY COMMITMENT (seal modes only): `open_state_header(post_d, witness)` returns `StateOpeningCells { outcome_tag, outcome_payload, input_d, deps_d, view_root, body_d }` by one header sponge (section 1.11, FP-5) asserted equal to the authenticated `post_d` (kit BOUNDARY COMMITMENT, typed opening interface). NODE uses the returned cells. The signature exists in `crates/mori-recursion/src/state_open.rs`; the B1 text referred to an opening signature that no kit defined (B2 finding 5).
- From VALUE DIGEST (seal mode only): the `outcome_d` formula of SRC-022 (`requirements.md:47`) applied to the opened kind and payload. Interface only.

Provides (node statement, ordered; final order frozen before keygen):
- `binding`: full field, transcript-bound.
- `keys_repr = [node_repr, leaf_or_lift_repr, foreign_repr]`: full field; equal in every child and the parent (G:85).
- `reg_root`: full field; consumed by the GP-3 hook. It is equal in every enabled NODE or FOREIGN child and in the parent. A LEAF child carries no `reg_root`. With no enabled NODE or FOREIGN child, the parent takes `reg_root` from its witness. The constraint is `en_j * (1 - is_leaf_j) * (child_j.reg_root - parent.reg_root) == 0` for `j` in {0, 1}, with `is_leaf_j` the LEAF entry of `key_sel_j`. So a FOREIGN child that proved membership under one root cannot sit beside a child or a parent that publishes another root.
- `stmt_tag` in {SEG = 1, SET = 2, SUB = 3, DL = 4, FV = 5, AW = 6, VW = 7}: NODE constrains. Only SEG, SET, AW and VW exist before the GP-2 hook is built; SUB and DL are GP-2 statements, FV is the GP-3 general verifier's, AW is the in-progress statement of ADMIT-WORK and VW the in-progress statement of VIEW-WORK.
- payload: SEG `(prog_d, pre_d, post_d, h_lo, h_hi)`, SET `(set_root)`, SUB `(set_root, sub_root)`, DL `(chain_d, cover_root, in_link, prem_root)`, FV `(fv_state_d)`, AW `(adm_state_d)` or VW `(view_state_d)` (the registry root of an FV statement is the node's `reg_root` lane, which NODE constrains across slots); the payload lanes have one fixed width (five words), unused words zero. `h_lo`, `h_hi` in `[0, 2^64)`: NODE range-checks (SRC-004, `requirements.md:29`). For a LEAF child, NODE checks `1 <= steps <= S_MAX` and `h_pre + steps < 2^64`.
- `shape_d`: full field; built by the GP-4 hook.

Modes (selected by booleans, one active):
- MERGE `SEG + SEG -> SEG`: equal `prog_d`; `L.post_d == R.pre_d`; `L.h_hi == R.h_lo`; result `(prog_d, L.pre_d, R.post_d, L.h_lo, R.h_hi)`; `R.h_hi < 2^64`. A unary MERGE (one slot off) passes the child through. MERGE sees no terminal tag (seven lanes); "no segment after a terminal state" is an assumed invariant (below), not a MERGE check (G:67 checks `L.term_tag == 0` as a lane).
- SEAL `SEG + SET or SUB -> SET`: the sealed `prog_d` is admitted: slot 1 (the seal-premise slot) holds a verified SET or SUB child whose root (or `sub_root`) `prem_root` opens with one hash as `ADM(adm_d)` with `adm_d == H(SHK_ADM, 1, prog_d)` (program admission, below); under input admission form (b) (AD-17) and a present input, `prem_root` opens instead as `JOIN(a, v)` with `a = ADM(adm_d)` and `v = VIEW(vw_d)`, `vw_d == H(SHK_VIEW, 1, val_d, view_root)`, `val_d` from a witnessed `input_d` preimage re-hashed equal to the opened `input_d`, and `view_root` the run's body word (input view, below). The seal tells the two cases apart by the re-hashed `val_d`: only the absent-input constant (an entry with no input; open in the spec, `O/design.md:214`) allows the `ADM`-only premise, and for it INIT sets `view_root = VIEW_NONE`, which `read_input` refuses; every other `val_d` requires `JOIN(ADM, VIEW)`; `h_lo == 0` (the genesis leaf proved `Init(prog_d, input_d)` at `h_pre == 0` through kit INIT, SRC-021, `requirements.md:46`); `open_state_header(post_d)`; the opened terminal kind is not Running (SRC-006, `requirements.md:31`); the opened `deps_d` equals the constant `EMPTY_DEPS` (a run with consumed claims seals only through SEAL-WITH-DEPS); `outcome_d` computed from the opened kind and payload; `claim_d = H(SHK_RUN, 1, prog_d, input_d, outcome_d)` with the opened `input_d`; `set_root = H(SHK_SET_LEAF, claim_d, n_steps)` with `n_steps = h_hi`, which NODE has range-checked below 2^64 as the height of the sealed interval, so `n_steps` is authenticated metadata of the SET leaf and stays outside `claim_d` (form proposed here: G:68 hashes `claim_d` only and carries `n_steps` in a side header; A:163-166 keeps `n_steps` in a receipt beside the claim; both keep it outside `claim_d`). No `pre_d`, `post_d`, memory root, tail digest or address enters the SET payload or `shape_d`.
- JOIN `SET + SET -> SET`: provided by the GP-4 hook (`set_root = H(SHK_SET_JOIN, l, r)`, no count).
- SEAL-WITH-DEPS `SEG + DL -> SET`, DL-STEP and DESCEND: provided by the GP-2 hook. SEAL-WITH-DEPS opens `DL.prem_root` with the same premise check (admission, and the input view under form (b)) as SEAL.
- ADMIT-WORK `AW or off -> AW`, or `-> SET` in the `ADM` form at completion: program admission, below.
- VIEW-WORK `VW or off -> VW`, or `-> SET` in the `VIEW` form at completion: input view, below (form (b) of AD-17).

The node relation, complete (B2 finding 16). Each row is one legal combination of the two child slots and the inline work port; every other combination of `stmt_tag`s, enables and work kind is unsatisfiable (`graph_unsupported_combination`). A LEAF child counts as a SEG `(prog_d, pre_d, post_d, h_pre, h_pre + steps)`.

| Mode | Slot 0 | Slot 1 | Inline work | Output | Checks |
|---|---|---|---|---|---|
| MERGE | SEG | SEG or off | Disabled | SEG | equal `prog_d`; `L.post_d == R.pre_d`; `L.h_hi == R.h_lo`; output `(prog_d, L.pre_d, R.post_d, L.h_lo, R.h_hi)`; unary passes the child through |
| MERGE-WORK | SEG or off | SEG or off | ExecuteOwnerWork, `k` steps, `1 <= k <= S_inline` | SEG | the MERGE checks on the enabled children, then the inline segment `W` is linked at the right end: `W.pre_d == post_d` of the last enabled child, `W.h_pre == h_hi` of that child, `W.prog_d` equal; output `(prog_d, first pre_d, W.post_d, first h_lo, W.h_pre + k)`, `W.h_pre + k < 2^64`. With both slots off the node is a leaf: output `(prog_d, W.pre_d, W.post_d, W.h_pre, W.h_pre + k)`, and INIT's gate applies inside the inline kernel when `W.h_pre == 0` |
| SEAL | SEG | SET or SUB (seal premise) | Disabled | SET, `LEAF` form | `prem_root` opens as `ADM(adm_d)` with `adm_d == H(SHK_ADM, 1, prog_d)`, or under form (b), for every `val_d` other than the absent-input constant, as `JOIN(ADM(adm_d), VIEW(vw_d))` with `vw_d == H(SHK_VIEW, 1, val_d, view_root)`; `h_lo == 0`; opened `outcome_tag` terminal; opened `deps_d == EMPTY_DEPS` |
| SEAL-WITH-DEPS | SEG | DL | Disabled | SET, `DEP` form | the SEAL checks without the empty-dependency check, with `prem_root = DL.prem_root`; `DL.chain_d ==` opened `deps_d`; the input link if `DL.in_link` is set (GP-2 hook) |
| DL-STEP | DL or off | SET or SUB | Disabled | DL | head-claim opening of the SET or of `sub_root`; chain, input-link or premise update (the premise element sets `prem_root` only; GP-2 hook) |
| DESCEND | SET or SUB | off | Disabled | SUB | one hash opening of a `JOIN` or `DEP` node of the grammar (GP-2 hook) |
| JOIN | SET | SET | Disabled | SET, `JOIN` form | none beyond the hash (GP-4 hook) |
| FV-WORK | FV or off | off | ForeignVerifyWork | FV, or SET in the `FOREIGN` form at completion | GP-3 general verifier kit |
| ADMIT-WORK | AW or off | off | AdmitWork | AW, or SET in the `ADM` form at completion | program admission, below |
| VIEW-WORK | VW or off | off | ViewWork | VW, or SET in the `VIEW` form at completion | input view, below (form (b) of AD-17) |

A FOREIGN child (the lift relation of the GP-3 hook) publishes a SET in the `FOREIGN` form and enters slots typed SET. Inline owner work is legal only in MERGE-WORK; it is real owner work, bound to the adjacent segment state and height as the row states, and it exports the same seven-lane contract as a leaf in cells (GP-4 hook). Administrative modes (SEAL, DL-STEP, DESCEND, JOIN, FV-WORK, ADMIT-WORK, VIEW-WORK) execute no owner step and never change a height.

Program admission (ADMIT-WORK). PROG canonicality (`O/design.md:84`) is a premise of the leaf on `prog_root`; it is discharged at the seal by an admission claim per `prog_d`.
- Claim and SET form: `adm_d = H(SHK_ADM, 1, prog_d)`; `ADM(adm_d) = H(SHK_SET_ADM, adm_d)`, a form of the SET grammar (GP-2 hook) with head claim `adm_d`. `SHK_ADM` and `SHK_SET_ADM` differ from every other domain word.
- In-progress statement: `AW = (adm_state_d)`, `adm_state_d = H(SHK_ADM_STATE, prog_d, prog_root, tbl, row)`, pinned sponge, in this order. `tbl` is the table in the fixed table order of the schedule and `row` is the next row to check.
- First unit (no AW child): open `prog_d` to its SVM-005 preimage as INIT does, take `prog_root` and the table bounds from it, and start at the first row of the first table.
- One unit: check, in order, at most the unit budget of rows, each opened under `prog_root` by a ROM line (kit PAGE/ROM). For every row: (1) a constant row has the `vd` recomputed from its payload and from the `vd` of the rows it references, with the formulas of VALUE DIGEST; (2) a type row has the `ty_d` of the `decl_schema_d` normal form (AD-14); (3) a constructor row has an ordinal and an arity that agree with its type row; (4) every reference index is in range, and a row reads only rows that the schedule checks before it (where the language admits a cycle, such as a recursive type, the `ty_d` normal form of AD-14 defines the check); (5) the padding of unused rows and of the last line is canonical (PAGE/ROM item 17).
- Completion: after the last row of the last table the unit emits the SET `ADM(adm_d)`. No unit emits it earlier.
- One admission chain per distinct `prog_d`; every run of that program reuses it. A unit is administrative work: it changes no height and counts no owner step.

Input view (VIEW-WORK; input admission form (b) of AD-17, which stays OWNER). It gives the leaf a certified read-only view of the input, so that a step reads field `k` of any input node with one Merkle opening and no fold.
- Claim and SET form: `vw_d = H(SHK_VIEW, 1, val_d, view_root)`; `VIEW(vw_d) = H(SHK_SET_VIEW, vw_d)`, a form of the SET grammar with head claim `vw_d`.
- The view: `view_root` is the root of a Merkle tree of fixed depth `D_view` with two kinds of leaves. A record `(idx, tag, ty_d, c, m, vd, child_pos)` describes one node of the value: `tag` and `m` as in `sherlock-value/1`, `c` the constructor ordinal or the scalar of a `UInt` or `Bool`, `ty_d` for a constructor and 0 otherwise. A child-vd leaf `(child_pos + k, vd_k, idx_k)` gives field `k`: its digest and the index of its record, or `NOT_ENTERED`. Record 0 is the input root.
- In-progress statement: `VW = (view_state_d)`, `view_state_d = H(SHK_VIEW_STATE, val_d, view_root, cursor, n_rec, fold_d)`, pinned sponge, in this order. `cursor` is the next record to check, `n_rec` the number of records, `fold_d` the constructor fold cells `(k, acc)` of the record at `cursor` while its fold spans units.
- One unit: for records in index order, at most the unit budget: open the record and its `m` child-vd leaves against `view_root`; recompute its `vd` from `tag`, `c`, `ty_d` and the child `vd_k` by the formulas of VALUE DIGEST (a constructor of `m` fields folds over units in `fold_d`); require `vd == val_d` for record 0; for an entered child require the child record's `vd == vd_k`; require that `child_pos` is the running count of child-vd leaves (so no two records share a leaf) and that every index is below `n_rec`.
- Completion: after record `n_rec - 1` the unit emits the SET `VIEW(vw_d)`. No unit emits it earlier.
- A record is entered only when VIEW-WORK reaches it, so the work grows with the entered part of the input. The full view (every node entered, in breadth-first order) is canonical per `val_d`. A lazy view enters only the nodes that the run reads; it is sound for the same reason, but its `view_root` depends on the prover's choice, so the boundary values at a common cut are equal across partitions only when every partition uses the same view; the public statement does not depend on it. Whether both forms are allowed, and the depth `D_view` (it must not bound the input, I2), are open points of AD-17.
- Seal read path: the seal compares the `VIEW` claim's `view_root` with the `view_root` header word that `open_state_header` returns (section 1.11, FP-5). The node does not re-hash the body.

PROFILE B EXTENSION POINT (benchmark-only, L-5): under Profile B the leaf gains a committed-instance column and challenge lanes. MERGE would require equal challenges and add the two LogUp partial sums; SEAL would require both sums zero and the challenges equal the hash of the commitment root; a witnessed committed-instance point must use the subgroup-checked assign (G:44, G:53, G:67-68, G:83; lookup AUDIT finding 9). This changes the leaf statement, node, seal and continuation and replaces the `digest_inj` premise. It is not built before M4 and no other kit depends on it.

#### Invariants
Guaranteed:
- A child instance cannot be changed without changing the verified proof (A:202).
- Every SEG covers `[h_lo, h_hi)` with `0 <= h_lo < h_hi < 2^64`, one `prog_d` and an unbroken `post == pre` chain (G:81; A:203-205).
- A disabled slot uses the canonical neutral statement encoding; genesis carry is neutral (SRC-005, `requirements.md:30`; landed in `ivc.rs:360-420` per A:220, G:84).
- Key routing: one selector binds the child's key representation, its permitted `stmt_tag` and its fixed-base coefficient destinations (A:194; G:85).
- Registry root: every enabled NODE or FOREIGN child has the parent's `reg_root`, so every FOREIGN claim under one root of the tree was lifted under that root (test `node_reg_root_mixed`; Quint invariant `RegRootEqualAcrossSlots` in `nodeIntegration.qnt`).
- SEAL refuses Running and `h_lo != 0`; the SET carries no physical state field (G:82).
- Program admission: a SET in the `ADM` form for `prog_d` exists only after ADMIT-WORK has checked every ROM row under the `prog_root` of `prog_d`, and SEAL and SEAL-WITH-DEPS require it, so no run of an unadmitted program is sealed (tests `node_seal_unadmitted_prog`, `node_seal_admission_other_prog`, the `admit_*` mutants; Lean NODE-L4).
- Input view (form (b) of AD-17): a SET in the `VIEW` form for `(val_d, view_root)` exists only after VIEW-WORK has checked every record and child-vd leaf of the view against `sherlock-value/1` with record 0 equal to `val_d`, and the seal requires it for the sealed `input_d`, so every `AuthIn` read of the run returns a node of the input (tests `seal_view_other_input`, `view_record_forged`, `view_claim_before_done`, `in_read_outside_view`; Lean NODE-L5).
- One seal path: SEAL and SEAL-WITH-DEPS are the only place where seal predicates are checked. `L` opens the sealed SET head (section 1.11, FP-3) and repeats no seal predicate; on the M4 linear path one SEAL node runs before the decider (kit L AND WRAP).
- Every enabled child's proof is verified in the node by the recursion verifier, which reads the child's constraint DAG as a cached trace; each child's key commitment has an inclusion path into `key_root`; the children's log digests `(d, n)` merge by `TAG_NODE` (`PR` sections 2 and 3.2).
- The relation serializes every keyed capacity, including `S_MAX`. Today `write_relation` writes only magic, column count, k and leaf key (`ivc.rs:571-576`; checked); SRC-014 requires `S_MAX` in the relation bytes (`requirements.md:39`; checked).
Assumed:
- The leaf proves `steps` reference steps from `pre_d` to `post_d` (`Premises.leaf_sound`, `Aggregation.lean:211-213`).
- CORE: a terminal state has no successor step and every leaf has `steps >= 1` (SRC-004), so no valid leaf starts from a terminal state (`stuck` hypothesis in Lean).
- BOUNDARY COMMITMENT: the state digest is collision resistant on canonical states (`digest_inj`), and the header opening is sound (its Lean row "Header opening").
- INIT: a leaf with `h_pre == 0` has the pre state `Init(prog_d, input_d)`.
- The recursion is sound (premise A4 of section 1.9).

#### Row budget
A STARK node has no fixed row gate; its cost is seconds per proof on the GPU. MEASURED (OpenVM, 20 host threads, `PR` section 2): arity 2 over 2^20 children 1.809 s; arity 4 over 2^16 children 2.595 s; arity 8 over 2^16 children 4.682 s. Node cost is nearly flat in child height, so leaves are tall (2^20 rows or more), and a 2^20 leaf under arity-8 nodes costs 6.50 s with its node share, ESTIMATE. The node's shape bounds are those of the internal set of section 1.9 (height at most 2^19; 1,000 constraints and 100 interactions per AIR, 50 AIRs, 2,000 columns); keygen refuses a node outside them. The key-root inclusion check is a small-field Poseidon2 path per child; its cost and byte-equal internal-recursive keys across two chip sets are PENDING (frontend M2a). The seal glue (outcome wrapper, `claim_d`, `set_root`, the admission and view checks), ADMIT-WORK and VIEW-WORK units, and MERGE-WORK with one real owner step are unmeasured on the STARK node; X7 and X10 measure them.

#### Verification
Quint: existing `models/sherlock/quint/aggregation.qnt` (abstracts verification and accumulation, older slot shape; A:258-260). New `models/sherlock/quint/nodeIntegration.qnt` (A:260; G:116 calls it `node_pcd_dag.qnt`): typed slots, key routing, MERGE/SEAL interval coverage, seal requires genesis and terminal, seal erases physical fields.
Quint invariants (G:118, Profile B one dropped): `SegCoversExactInterval`, `SealRequiresGenesisAndTerminal`, `SealErasesPhysicalAddresses`, `RegRootEqualAcrossSlots`, `SealRequiresAdmittedProgram`, `SealRequiresCertifiedInputView`.
MockProver positives: `pos_node_header_opening_all_outcomes` (the opened words equal the leaf's header cells for Halted, Rejected, Exhausted), `pos_node_merge_work_two_children_one_step`, `pos_node_merge_work_no_children` (the node as a leaf, with the INIT gate at `h = 0`), `pos_node_genesis_neutral_slots`, `pos_node_unary_merge`, `pos_node_merge_adjacent`, `pos_node_merge_bracketing_same_claim` (G:121), `pos_node_key_kinds_each_slot`, `pos_node_seal_all_outcomes` (Halted, Rejected, Exhausted), `pos_node_height_2p64_minus_1` (GP-001 acceptance, `requirements.md:56`).
Named mutants (fresh malicious witness, rejection at the constraint boundary): `node_unbound_child_instance`, `node_drop_child_carry`, `node_genesis_non_neutral`, `node_disabled_slot_garbage`, `node_wrong_key_destination`, `node_selector_repr_mismatch`, `node_reg_root_mixed` (a FOREIGN or NODE child whose `reg_root` differs from the parent's, in JOIN, DL-STEP or SEAL-WITH-DEPS), `node_height_wrap` (`h = 2^64`), `node_steps_zero`, `node_steps_gt_smax`, `node_overlap_interval`, `node_gap_interval`, `node_broken_boundary` (`L.post_d != R.pre_d`), `node_cross_program_merge`, `node_seal_running`, `node_seal_nonzero_hlo`, `node_seal_state_digest_in_set`, `node_seal_header_word_forged` (an opened `input_d` or payload that differs from the committed one), `node_seal_nonempty_deps` (a SEAL of a state whose `deps_d` is not `EMPTY_DEPS`), `node_seal_unadmitted_prog` (a seal whose premise slot holds no `ADM` head), `node_seal_admission_other_prog` (an `ADM` head of another `prog_d`), `admit_const_vd_forged`, `admit_ty_d_forged`, `admit_ctor_arity_forged`, `admit_pad_noncanonical`, `admit_claim_before_done` (an `ADM` SET emitted before the last row), `seal_view_other_input` (a `VIEW` claim whose `val_d` is not the opened `input_d` preimage word, or whose `view_root` is not the run's body word), `seal_view_omitted` (form (b): a seal of a run with a present input whose premise is `ADM` alone), `view_record_forged` (a record or child-vd leaf whose `vd` does not follow `sherlock-value/1` from its children), `view_claim_before_done` (a `VIEW` SET emitted before record `n_rec - 1`), `node_work_not_adjacent` (inline work whose `pre_d` or height is not the right end of the children), `node_work_height_not_advanced`, `node_admin_mode_changes_height`, `graph_unsupported_combination` (for example SEG + SET in MERGE, or inline work in JOIN). Host/parser refusal, not a constraint mutant: `node_relation_missing_smax` (deserialization of a relation without `S_MAX` refused; SRC-014: `S_MAX` SHALL be a serialized field of the node relation, `requirements.md:39`). Dropped: A:282 `node_continue_after_terminal` (nothing to reject at MERGE; covered by the CORE assumption).
Parser refusals are recorded apart from constraint refusals (A:285).

#### Lean obligations
- NODE-L1. Every binary SEG certificate (MERGE over LEAF children) gives a run of exactly `h_hi - h_lo` reference steps from any canonical state opening to `pre_d` to one opening to `post_d`, and the SEG is the same for every bracketing of the same leaves. File: new `clean/fork/Clean/Sherlock/BinaryNode.lean` (name proposed here), importing `Aggregation.lean` and reusing `Seg`, `link`, `linkAll_append`, `bracket_independent`, `sound_linkAll`. Premises: `Premises.leaf_sound`, `digest_inj`, `progDigest_inj`; `det` as hypothesis, discharged for dense memory by `premises_det` (C3). Contract: MERGE soundness, SRC-004, SRC-023 at segment level. Status: open.
- NODE-L2. SEAL soundness: a SET leaf `claim_d` from SEAL implies a run from `Init(prog_d, input_d)` to a terminal state with that outcome digest in `n_steps` steps. File: `BinaryNode.lean`. Premises: NODE-L1, `decider_pinning` shape, `stuck` hypothesis, `digest_inj`, outcome digest injectivity (VALUE DIGEST obligation 4). Contract: SEAL. Status: open.
- NODE-L3 (halo2 prototype statement; the STARK node folds no accumulator, so its STARK form is `node_key_route` of section 1.11, FP-2). Named-key routing: with one-hot selectors and per-key coefficient routing, the folded accumulator is valid iff every enabled child's proof verifies under its selected key. File: new `clean/fork/Clean/Sherlock/NamedAccumulator.lean` (name proposed here), extending `AccAlgebra` (`Aggregation.lean:1230`). Premise: accumulation soundness as a hypothesis structure (no `axiom`). Contract: key routing, SRC-005 neutral slots. Status: open.
- NODE-L4. Program admission: if ADMIT-WORK units chain from the first unit to the `ADM(adm_d)` SET for `prog_d`, then every ROM row under the `prog_root` of `prog_d` satisfies the PROG canonicality premise `ProgCanonical(prog_root)`. File: `BinaryNode.lean`. Premises: hash injectivity for `SHK_ADM`, `SHK_ADM_STATE` and the ROM domains; the `ty_d` normal form of AD-14 as a hypothesis. Contract: the seal admission check; it discharges `ProgCanonical` for NODE-L2. Status: open.
- NODE-L5. Input view (form (b) of AD-17): if VIEW-WORK units chain to the `VIEW(vw_d)` SET, then every record reached from record 0 through child-vd leaves under `view_root` is a node of the value whose digest is `val_d`, with that node's tag, fields and `vd`. File: `BinaryNode.lean`. Premises: VALUE DIGEST obligation 4 (injectivity), hash injectivity for `SHK_VIEW`, `SHK_VIEW_STATE` and the view tree. Contract: the seal view check and the `AuthIn` read port; it discharges I3 of kit INIT under form (b). Status: open.

#### Code skeleton
Files: `crates/mori-recursion/src/node.rs`, `node_slot.rs`, `seal.rs` (new; G:150 and A:291-296 differ on layout, flat files follow the existing crate). `ivc.rs` stays as the M4 linear path until parity; its `write_relation` gains `S_MAX`.

This skeleton is the halo2 prototype form (`AssignedNative<F>`, `PinnedKeyReprs`, accumulator cells). The STARK node verifies children with no accumulator; its statement envelope (the deferred-log `(d, n)`, the forwarded `key_root`, `keys_d` and the update rule of every mode) is section 1.11, FP-2.

```rust
pub enum KeyKind { Leaf, Node, Foreign }            // Leaf = direct leaf or one-leaf lift (N-1)
pub enum NodeMode { Merge, MergeWork, Seal, SealWithDeps, DlStep, Descend, Join, FvWork, AdmitWork, ViewWork }   // the relation table
pub enum StmtTag { Seg = 1, Set = 2, Sub = 3, Dl = 4, Fv = 5, Aw = 6, Vw = 7 }
pub struct SegCells { pub prog_d: AssignedNative<F>, pub pre_d: AssignedNative<F>, pub post_d: AssignedNative<F>, pub h_lo: AssignedNative<F>, pub h_hi: AssignedNative<F> }
pub struct SetCells { pub set_root: AssignedNative<F> }
pub struct NodeRelation { pub child_k: u32, pub s_max: u64, pub keys: PinnedKeyReprs }
/// Authenticate the enabled child's exact instance; return its obligation and typed payload.
pub fn verify_slot(cx: &mut RecursionContext<'_>, slot: SlotId, w: &SlotWitness) -> Result<VerifiedChild, Error>;
/// One selector routes key repr, permitted stmt_tag and fixed-base coefficients.
pub fn route_child_key(cx: &mut RecursionContext<'_>, c: &VerifiedChild, keys: &PinnedKeyReprs) -> Result<RoutedChild, Error>;
/// Range-check heights (8-bit bytes only) and steps; link adjacent segments.
pub fn merge_segments(cx: &mut RecursionContext<'_>, l: &SegCells, r: &SegCells) -> Result<SegCells, Error>;
/// Require h_lo == 0, terminal and deps_d == EMPTY_DEPS on the opened header (open_state_header of
/// state_open.rs); compute outcome_d, claim_d and set_root = H(SHK_SET_LEAF, claim_d, h_hi); drop physical fields.
/// Requires: prem_root (slot 1) opens as ADM(adm_d) with adm_d = H(SHK_ADM, 1, seg.prog_d), or under form (b)
/// with an input as JOIN(ADM(adm_d), VIEW(vw_d)) with vw_d = H(SHK_VIEW, 1, val_d, view_root).
pub fn seal(cx: &mut RecursionContext<'_>, seg: &SegCells, open: &StateOpeningCells, prem: &SubCells) -> Result<SetCells, Error>;
/// ADMIT-WORK: continue `prev` (or open prog_d at the first unit), check up to the unit budget of ROM rows,
/// emit the next AW state, or the SET ADM(adm_d) after the last row.
pub fn admit_work(cx: &mut RecursionContext<'_>, prev: Option<&AwCells>, w: &AdmitUnitWitness) -> Result<AdmitOut, Error>;
/// VIEW-WORK (form (b) of AD-17): continue `prev`, check up to the unit budget of view records against
/// sherlock-value/1, emit the next VW state, or the SET VIEW(vw_d) after record n_rec - 1.
pub fn view_work(cx: &mut RecursionContext<'_>, prev: Option<&VwCells>, w: &ViewUnitWitness) -> Result<ViewOut, Error>;
/// MERGE-WORK: link the inline segment at the right end of the enabled children; add its k steps to the height.
pub fn merge_with_work(cx: &mut RecursionContext<'_>, ch: [Option<&SegCells>; 2], work: &SegCells) -> Result<SegCells, Error>;
/// Fold all enabled obligations and inherited carries in canonical order; neutral for off slots.
pub fn accumulate_children(cx: &mut RecursionContext<'_>, ch: &[RoutedChild; 2], local: &LocalObligations) -> Result<AccumulatorCells, Error>;
impl Relation for NodeRelation { /* write_relation serializes MAGIC, cols, k, s_max, key reprs */ }
```

#### Agent reads / must not need to read
Reads: this kit; `crates/mori-recursion/src/ivc.rs:60-90,340-590`; `inner.rs` verification interfaces; `Aggregation.lean:27-70,176-218,1230-1290`; named-key conditions in `research/sherlock-gp-architecture-20261110/architecture.md:426-437` (A:223); from BOUNDARY COMMITMENT only `open_state_header` and `StateOpeningCells` (`state_open.rs`) and the header word order.
Must not need: CORE rules, MEM or backend columns, Poseidon rounds of the leaf HASH chip, VALUE DIGEST internals, Compact templates, foreign verifier internals.

#### Dependencies
BOUNDARY COMMITMENT (`open_state_header`), VALUE DIGEST (outcome formula interface; the constant and constructor formulas for ADMIT-WORK), PAGE/ROM (ROM line openings for ADMIT-WORK), INIT (the genesis guarantee; the `prog_d` preimage), the integrated leaf key (all leaf kits). GP-2, GP-3 and GP-4 hooks plug into it through the rows of the node relation table.

---

### Kit: L AND WRAP

#### Purpose
Compress the shard tree into the bridge layer `L` and wrap `L` in the one blinded PLONK-KZG proof that the ledger 9.1.0.1 verifier checks. It is the only place where the field changes from BabyBear to BLS12-381 Fr, and the wrap is the only proof that leaves the user's machine (`PR` sections 3, 5 and 8).

#### Interface
No bus tuples. Inputs and outputs across proof boundaries:
- `L` takes: the node root proof (full tree) or the leaf-layer proof (path C); the flattened deferred field log and its tree shape as private witness; the `key_root` inclusion path of the leaf `P2(DOM_KEYDAG, entry, path, enc(X))`.
- `L` exposes as public values the index table of section 1.11, FP-3: the root statement (74 words, FP-2), the anchor statement `(mode, j, k, e_in, e_out)`, `run_tag`, `layout_d`, `X`, the lane map, `pv` (section 1.8) and the call vector of `2 + L_TX` lanes. `L` opens the sealed SET head of a general run as `LEAF(claim_d, n_steps)` or `DEP(claim_d, n_steps, cover_root, in_link)` and refuses a SEG, `JOIN`, `FOREIGN`, `ADM` or `VIEW` root (LW-008). A general run is a Sherlock program whose effect words write its run statement `[C_P, input_pub, outcome_pub]` (OD-15, FP-4, FP-6). Open (FP-3): the carrier of the effect region and `comm` from a multi-shard terminal state to the call vector, the call-vector lanes of the run and collection statements, and the contract-side form of a collection. The test names `decider_seg_root_as_run`, `decider_set_head_forged`, `pos_decider_all_outcomes` and `pos_decider_collection_three_runs` and obligation DEC-L1 belong to this kit.
- The wrap takes the proof of `L` with its Merkle roots, PoW witnesses, query positions, the log, `pv` and the onion digests as private witness, and has exactly the call vector as public inputs.
- Wrap key constants: `B`, `V`, `P`, `E`; `L_TX`; the anchor template words; `layout_d`; the hash domains; the `L` key schedule `sched`.
- Forbidden public inputs: `X`, `mode`, `j`, `k`, `run_tag`, `key_root`, plain `prog_d`, `pre_d`, `post_d`, any memory root, tail digest, length or address, and anything derived from the transcript of `L`.

#### Invariants
Guaranteed by the wrap:
- It verifies `L` under the frozen `L` key, following `sched`, the transcript algorithm and the two-OOD order of sections 1.8 and 1.9.
- Range and `Canon` on every logged value and on the limbs of `X`; a bounded-quotient range check on every lazy reduction; a canonical check on every squeeze recomposition; `pv` absorbed natively and bound by one `eq` evaluation; native replay of every log entry; the running sums of the value and permutation offsets; the `NOP` and zero padding rules; every entry of the reservation `R` present.
- Anchor checks for both modes in the one wrap: the `popeq` lane equals `a_in`; `a_in` and `a_out` are outputs of `DIGEST` entries of the log; the tag of the `a_in` entry is `SHK_PROGC` when `mode` is ORD or `j = 1` and `SHK_LOCK` otherwise; the tag of the `a_out` entry is `SHK_PROGC` when `mode` is ORD or `j = k` and `SHK_LOCK` otherwise.
- Lane 0 equals the `bind` word of E-1, lane 1 equals `comm`, and `layout_d` equals the key constant (V+).
- `X` equals the cached-trace commitment of `L` that the wrap reads (the DAG key check).
- Blinding: fresh CSPRNG blinders on every proof; seeded proofs are a test mode only (kit ZERO KNOWLEDGE).
Assumed: premises A1 to A7 of section 1.9; the leaf, node and leaf-layer proofs are sound under their keys (A4); NODE INTEGRATION holds every seal predicate.

#### Row budget
Usable wrap rows are `2^k - 7`: 2,097,145 at k = 21 and 1,048,569 at k = 20, MEASURED, and 524,281 at k = 19, DERIVED. The gate is 85 percent of usable rows. Every gate and lookup has degree at most 5, so the extended domain is 4n; one degree-6 gate doubles the quotient work. One fixed 2^16-row range table with at most 4 lookups of input degree 2.

| Shape | Low | Mid | High | Status |
|---|---:|---:|---:|---|
| Single lane, 70-row Poseidon (degree 6), k = 21 | 2,064,403 | 2,440,107 | 2,652,569 | ESTIMATE on the keyed `L`; fails the k = 21 gate in all three |
| G4, 19-row Poseidon, 80 percent packing | 393,581 | 434,692 | 454,423 | ESTIMATE on the keyed `L`; fits k = 20 in all three; under the k = 19 gate at low and mid (mid at 97.5 percent of it), so k = 19 is not a safe target |
| G4, no limb overlap and 60 percent packing | 882,760 | 939,160 | 966,651 | ESTIMATE on the keyed `L`; over the k = 20 gate in all three, fits k = 21 |
| G4 at `w_stack` 36, the bench-node proxy area | 431,803 | 468,675 | 488,406 | ESTIMATE on the keyed `L`; fits k = 20 |
| G4 with the wrap-rows bundle and radius 4/3 | | about 364,000 | | ESTIMATE on the model structure before keygen; 82 percent of the k = 19 gate |

The keyed rows rest on the MEASURED keygen structure of `L` (39 AIRs, 1,537 trace columns, 487 interactions), the S4 query constants 57/28/19 with 21-bit folding and `mu` PoW, and the heights of the bench node as a proxy for the heights of `L` (`PR` section 3.5). The section 3.4 set and the heights of D10 replace both before M7.

The key's k is the smallest of 19, 20 and 21 at which the measured G4 rows fit the gate; M7 (D15) measures them. The wrap-rows bundle assumes OD-1, OD-3 and OD-6. G4: 4 identical lanes of 8 advice columns over about 14 shared fixed columns, 18 permutation columns, rotations within {-1, 0, +1}, no foreign-field, pairing or SHA chip; one lane takes one query of Merkle work, one extension-field coefficient and one log slot. The anchor lanes as key constants and one `L_TX` save about 780 rows (`PR` section 6.3). Added rows: `layout_d` at most 120; the DAG key check about 120; the two OOD samples about 12,000 (all ESTIMATE). `L`: 2.0 x 10^7 Fr permutations of hashing plus 1.5 x 10^7 of grinding, ESTIMATE; the model puts one child at about 2^23 main cells, below the 2^24 that `w_stack` = 16 would allow and far below the key bound of 48; the bench-node heights, used as a proxy, give 37.5 M cells, which would need `w_stack` about 36. Keygen fixes no heights, so D10 reads them from `L` tracegen over one node proof.

GPU wrap kernels in build order: K0 Fr field header shared with the Fr Poseidon kernel of `L`; K1 NTT over Fr; K2 MSM on G1; K3 coset-tiled quotient numerator; K4 grand products; K5 evaluations and multiopen; K6 lookup sort. ESTIMATE: whole wrap 3 to 7.5 s at k = 19 with today's residual, 1.9 s (1.44 to 2.34 s) with the full-device pipeline and the rewritten MSM; memory about 5 GB at k = 19; proof about 8 to 9 kB. MEASURED on today's k = 20 decider, a stand-in for the G4 wrap (`PR` section 8): seven CPU and GPU configs give one proof SHA-256 under a test-only chip seed, each accepted by ledger 9 `VerifierKey::verify`; the GPU quotient numerator matches the CPU in all 4,194,304 extended-domain values. In one run at load average 16 to 24 the CPU path proves in 243.3 s and the full GPU path in 95.0 s (2.6 times), the GPU path without the GPU MSM in 131.6 s; the quotient numerator falls from 101.6 s to 8.4 s; a quiet host proves the decider on the CPU in 68.1 s, so the ratio holds for the same run only. The MSM skew mask `MSM(s) = MSM(s + r) - MSM(r)`, witness-independent and on the user's GPU, takes the 45 MSM calls from 622 to 647 s to 23.2 s with unchanged proof bytes. Final table (`PR` section 8; `GW` at commit `52ad95069`): full GPU 34.8 / 95.0 / 155.2 s against CPU 118.8 / 243.3 / 441.7 s at k = 19 (decider-shaped proxy), 20 and 21, one proof per cell under load, all byte-identical and accepted by ledger 9, MEASURED; on a quiet host the k = 21 GPU path is about 1.35 times faster than the CPU path with the GPU MSM and about 1.45 times without, ESTIMATE. The 3 to 7.5 s and 1.9 s wrap figures above are ESTIMATE conditional on the fused quotient kernel and the MSM rewrite, both not built. A quiet-host repeat is PENDING. The MSM stays split between CPU and GPU while the GPU MSM is above 0.2 s per 2^20. Fallback triggers for an Fr-terminal layer between `L` and the wrap: rows over budget after M7, a measured GPU wrap above 8 s at the chosen k after K1 to K5, or an owner request for a larger log capacity; form A of that layer is not built (OD-11).

#### Verification
Wrap circuit: MockProver positives for an ordinary call, chunk 1, chunk `j` and chunk `k`, and the forced-overflow cases of frontend M4 with `R` reserved; named mutants `wrap_log_value_noncanon`, `wrap_quotient_unbounded`, `wrap_squeeze_noncanon`, `wrap_pv_unbound`, `wrap_nop_then_used`, `wrap_value_padding_nonzero`, `wrap_anchor_entry_removed` (one anchor entry removed from the log: refused), `wrap_anchor_tag_swapped`, `wrap_popeq_lane_not_a_in`, `wrap_lane0_not_bind`, `wrap_layout_d_other`, `wrap_x_not_cached_commitment`, `wrap_l_transcript_from_proof` (a verifier of `L` that follows the proof instead of `sched`) (names proposed here). Transcript vectors T-FS (`FD/EXPERIMENTS.md` section 2). Frontend M2b: one wrap proof that ledger 9 `VerifierKey::verify` accepts with `2 + L_TX` public inputs, negative controls, and EQ-ZK on two wraps. Real-proof checks: each public word flipped and refused by `validate_call`. Quint: an anchor model of ordinary calls and chunks over the anchor statement table (kit COMPACT FRONTEND).

#### Lean obligations
- BR-L1. `bridge_sound` and `wrap_view_only_claim` at commit `bdb92973a` (built; 0 sorry, standard axioms only, 112 theorems), with premises A1 to A7 and `vkL_ok`.
- BR-L2. The restatement of section 1.9 (gate check G4): `ProvenRegime` with the per-fold vector, separate multiplicities, two OOD samples and the `w_stack` bound; the two-OOD bound against the transcript order; `encode` on the two-table `pv`; `wrap_refines` with the DAG key check and the anchor statement checks; the two Fr sponge algorithms named. Status: open.
- BR-L3. Lemma 23 (`pv_binding_count`, over BabyBear `Z/q`), the transcript of `L` as a program in the key with `pvFirstT` and the PoW part of `ProvenRegime` derived from it, `d_0 = P2(TAG_EMPTY)` and the fixed `pv` length per key: `PvBinding.lean`, `Bridge.lean`, `BridgeAxioms.lean`, `FrLog.lean` at commit `bdb92973a` (built). Status: proved.
- BR-L4. A signed lazy reduction in the wrap needs `ReduceRel` in place of the unsigned `ArithGadget` of the model. Status: open.

#### Code skeleton
Files (proposed): `crates/mori-recursion/src/wrap/` (G4 regions, verifier of `L`, log replay), stark-backend parameter code (per-fold `folding_pow_bits`, separate radius and bound multiplicities in `ProximityRegime`, explicit per-round query vectors, `NUM_OOD_SAMPLES = 2` for `L`, the `L` Merkle hash and transcript of section 1.8).
```rust
pub struct WrapKeyConsts { pub b: u32, pub v: u32, pub p: u32, pub e: u32, pub l_tx: u32, pub layout_d: Fr, pub anchor_words: AnchorTemplate, pub sched: TranscriptSchedule }
/// Requires: proof_l verifies under vk_l following `sched`. Ensures: pv bound, every entry replayed natively,
/// R present, padding canonical, anchor and lane checks hold. Public inputs: the call vector only.
pub fn wrap_prove(pk: &WrapPk, c: &WrapKeyConsts, proof_l: &LProof, log: &FlatLog, call: &[Fr], rng: &mut impl CryptoRngCore) -> Result<WrapProof, Error>;
/// Keygen refuses: a capacity that does not hold the largest R; an L key whose query vector or shape bounds differ from the frozen set.
pub fn wrap_keygen(params: &Params, c: &WrapKeyConsts, vk_l: &LVerifyingKey) -> Result<(WrapPk, WrapVk), Error>;
```

#### Agent reads / must not need to read
Reads: this kit; sections 1.6 to 1.10; `PR` sections 3, 5, 8, 9 and 11; `B/CANDIDATE.md` of the bridge round; the stark-backend verifier and transcript files that section 1.8 names; `crates/mori-recursion/src/run_decider.rs` for the ledger template capture.
Must not need: CORE rules, memory internals, the leaf AIRs, Compact compiler passes.

#### Dependencies
NODE INTEGRATION (root statement and the one seal path), the per-program keys of the shard plan (`key_root`), kit COMPACT FRONTEND (call vector layout, anchor template, E-1 placement V), kit ZERO KNOWLEDGE (blinding and hiding rules), GP-2 hook (the `LEAF` and `DEP` forms of the SET grammar). The universal keys freeze only after the security qualification gate of section 1.9 passes.

---

### Kit: CONTINUATION

#### Purpose
Prove runs longer than one leaf through the tier 1 segment tree, and let the proving of one run stop and resume under the same keys in the retained-memory checkpoint mode (mode 1). The resumed run gives the same `prog_d`, `input_d`, `outcome_d`, `n_steps` and `claim_d` as the uninterrupted run (GP-001, `requirements.md:56`; R-7).

#### Interface
No bus tuples and no new circuit cells in mode 1 (G:658-663). It uses NODE INTEGRATION's MERGE and SEAL only. Everything that is persisted goes through the HOST CHECKPOINT STORE kit; this kit is the scheduler.
- Tier 1 (host): leaves arrive in order; a carry stack of proven SEGs by rank; two adjacent SEGs of equal rank are merged by NODE MERGE (G:614-615). The fold order is a prover choice and does not change the statement (`bracket_independent`, `Aggregation.lean:428`).
- Mode 1 checkpoint record (host artifact, persisted): key-set manifest hash (SRC-008); `prog_d`; `input_d`; `h` (u64, `1 <= h < 2^64`, the `h_hi` of the proven prefix, which starts at `h_lo = 0`); the prefix SEG proof(s) covering `[0, h)` with their accumulators; the post-state `post_d`; and every witness resource needed to prove the suffix, in the versioned codec `sherlock-ckpt/1` of the HOST CHECKPOINT STORE kit: the explicit register state (header and body words), the memory append log with the raw tail records, the stack preimages below both cached tops, the program image and input preimage, the proof-prefix carry stack, and the SRC-019 bundle prefix hash (G:622-630). `MemBackend::persist` is not that store: it assigns circuit constraints and returns boundary cells and writes nothing durable, and no `StoreHandle` exists in the code (`crates/mori-chip/src/membackend.rs:95-128`; B2 finding 13). The record digest uses a checkpoint domain tag (R-7; name `SHK_CKPT` proposed here) distinct from `SHK_RUN` and the claim domains, so a record can never parse as a claim. Whether the tag is also needed in circuit is open (section 4.2, N-4).
- Resume: the first suffix leaf has `h_pre = h` and `pre_d` = the record's `post_d`; NODE MERGE joins prefix and suffix; one SEAL at the end (G:632-633; run-length AUDIT finding 5: same heights, same `pre_d`/`post_d` link, same Init at `h_lo = 0`).
- Mode 2 (address-free checkpoint, lazy UNFOLD): open research, not part of this kit (R-7). Gaps (run-length AUDIT finding 5): (1) a slice of `m > 8` cells materialised in one step does not fit the window and breaks the appends-per-step argument; extra steps would change `n_steps` and break SRC-023; (2) after a mode 2 resume the addresses differ, so the state digest at height `h` is not a function of `(prog_d, input_d, h)` across modes; (3) the checkpoint work is O(live roots + stack depth), not one step, and no proof is named to carry it while every leaf must carry at least one step (SRC-004, SRC-014); (4) the continuation claim formula has no mode field; (5) the cut length (`quantum_steps`) is bound nowhere and must stay a prover choice, not a keyed constant; (6) the study said continuations compose "via GP-002", which GP-002 forbids. The drafts' mode 2 parts (A:535-573 "Tier 2", G:636-642) are not specified here.

#### Invariants
Guaranteed: pause and resume execute no machine event; fuel and logical counters are unchanged; `input_d` is unchanged; a suffix joins only to the authenticated prefix state; a prefix is a SEG with Running state and is never sealed, sent to `L` or consumed as a claim (GP-001, GP-002); the full run still starts at genesis (A:560-565; G:647-650). Exhausted is terminal and never a checkpoint trigger (A:587).
Assumed: NODE MERGE is associative in effect (NODE-L1); the HOST CHECKPOINT STORE restores a state that it has validated against the `post_d` of the verified prefix; BOUNDARY COMMITMENT digest is canonical (`Mem.wf_unique`, C3).

Two kinds of failure are kept apart. A constraint failure: a suffix leaf whose witness does not satisfy the leaf relation (for example a cold read whose record does not reach the root in `pre_d`); no accepted proof exists. A host integrity failure: the restored data does not reproduce the committed digests; the restore refuses before any proving. Soundness never depends on the store: a corrupted store can only stop the prover. A corrupted store cell that no later leaf reads is not detected by the next leaf, and could surface as a failed proof much later, so the restore procedure validates the whole record (kit HOST CHECKPOINT STORE), not the leaves.
Profile B: not supported (L-5; lookup AUDIT finding 9). Mode 1 requires Profile A.

#### Row budget
ESTIMATE: 0 added circuit rows in mode 1 (uses MERGE; G:662). At most one extra NODE MERGE proof per pause when the stack is flushed mid-tree (ESTIMATE, G:664). Host carry stack at most about 64 SEG entries (ESTIMATE, `ceil(log2 L)` for `L < 2^64` leaves). No throughput figure is quoted (R-10).

#### Verification
Quint: new `models/sherlock/quint/continuation.qnt` (A:603; G:671 calls it `continuation_scheduler.qnt`): out-of-order completion keeps the interval, any pause cuts `0 < h1 < h2 < N` give the same `claim_d` and `n_steps`, checkpoint refused by SEAL-as-claim and by GP-2, machine steps and administrative steps kept apart.
Quint invariants (G:673): `MaxStackDepthLeLogLPlus2`, `ResumeProducesIdenticalClaimD`, `CheckpointRejectedBySealAndGP2`, `OutOfOrderWorkerCompletionPreservesSegInterval`.
Positives: `pos_mode1_stop_resume_allocating_run` (the GP-001 acceptance run: more than one leaf, allocates more than one leaf's memory, builds a constructor of more than 8 fields across a leaf boundary; uninterrupted and with one stop and resume, same four values and `claim_d`; resume at a power-of-two leaf boundary and at an odd leaf, G:676); `pos_mode1_resume_nonempty_tail`; `pos_mode1_resume_nested_frames_tasks` (A:611-614); `pos_tier1_stack_bound` with randomised worker latency (G:675).
Named mutants (constraint boundary): `cont_refresh_fuel`, `cont_change_input`, `cont_skip_predecessor`, `cont_reset_height`, `cont_publish_as_run`, `cont_ckpt_as_gp2_claim`, `cont_tampered_read_cell_unvalidated` (restore validation switched off and a memory record that the first suffix leaf reads is changed: the leaf is unsatisfiable at the root equality of `cold_read`) (A:619-626; G:678-679). Host/preflight refusals, recorded apart: `cont_mismatched_key_manifest`, `cont_bundle_missing_proof` (G:680-681), and the restore refusals of the HOST CHECKPOINT STORE kit (`ckpt_tampered_unused_cell` among them: a changed record that no leaf reads is refused by the restore validation, not by a constraint).

#### Lean obligations
- CONT-L1 (claim chaining for continuations). For leaves `ls = prefix ++ suffix`, merging the prefix SEG and the suffix SEG gives the same SEG as one fold of `ls`, so after SEAL the same `outcome_d`, `n_steps` and `claim_d`. File: `clean/fork/Clean/Sherlock/ClaimDag.lean` (G:687; extends `Aggregation.lean` via `linkAll_append`, `same_leaves_same_stmt`). Premises: NODE-L1, `digest_inj`, `leaf_sound`, `det` via `premises_det`. Contract: GP-001 stop and resume. Status: open.
- CONT-L2. No SEG with a Running state has a path to an accepted claim without SEAL. File: `ClaimDag.lean`. Premises: SEAL definition. Contract: GP-002 "a segment statement or a state digest SHALL NOT be accepted as a claim". Status: open.

#### Code skeleton
Files: `crates/mori-recursion/src/continuation.rs` (scheduler), `checkpoint.rs` (new).
```rust
pub struct ProvenSeg { pub rank: u32, pub h_lo: u64, pub h_hi: u64, pub shape_d: F, pub proof_path: PathBuf, pub stmt_and_acc: Vec<u8> }
pub struct SegmentTreeScheduler { /* carry stack, bundle dir */ }
impl SegmentTreeScheduler {
    /// Requires: next leaf has h_pre == current h_hi. Ensures: merges equal-rank SEGs, flushes to the bundle.
    pub fn push_leaf(&mut self, leaf: LeafProof) -> Result<(), SchedError>;
    /// Requires: run still Running. Ensures: persists a mode 1 record through the store kit; never seals.
    pub fn pause(self, run: &dyn RunStore) -> Result<CheckpointV1, SchedError>;
    /// Requires: last leaf terminal, h_lo == 0. Ensures: final merges and one SEAL.
    pub fn finish(self, deps: Option<ProvenSet>) -> Result<SealedBundle, SchedError>;
}
// CheckpointV1, RunStore and the codec are defined in the HOST CHECKPOINT STORE kit (checkpoint.rs, run_store.rs).
impl CheckpointV1 {
    /// Requires: manifest equal, prefix verifies (SRC-010), restore validation passed.
    /// Ensures: scheduler seeded at h with the carry stack; the run store rebuilt.
    pub fn resume(self, expected_manifest: &[u8; 32]) -> Result<(SegmentTreeScheduler, Box<dyn RunStore>), CkptError>;
}
```

#### Agent reads / must not need to read
Reads: this kit; NODE INTEGRATION's MERGE and SEAL signatures; the `RunStore` and `CheckpointV1` interfaces of the HOST CHECKPOINT STORE kit; SRC-004, SRC-010, SRC-014, SRC-019, GP-001, GP-002 rows; run-length `AUDIT.md` finding 5.
Must not need: verifier gadget internals, CORE gates, Merkle walk gates, value digest internals, Compact code.

#### Dependencies
NODE INTEGRATION (MERGE, SEAL), HOST CHECKPOINT STORE (persist, validate and restore), BOUNDARY COMMITMENT (state digest).

---

### Kit: GP-2 hook (consuming claims)

#### Purpose
Let a run use the sealed claim `(prog_d', input_d', outcome_d')` of another verified run, checked in circuit, and let independent runs be combined into one proof whose statement commits to the set of claims (GP-002, `requirements.md:57`). Only sealed claims enter; a segment statement or a state digest never does.

#### Interface
No bus tuples in area 3. Named cells and lanes:
The B1 text recommended "the dependency commitment uses the SET hash itself". That is not a protocol: a consumed claim can sit inside a JOIN or inside a DEP that also holds its own ancestors, so the root of the producer's verified SET differs from anything the consumer accumulates, and empty dependencies, order, repetition, claim extraction and the `n_steps` metadata were not defined (B2 finding 6). It is replaced by the protocol below, in which the grammar, the accumulation and the discharge are defined together. The protocol is in neither draft; it has no model yet, and X12 (Quint model, then rows) must pass before the GP-2 spec freezes it.

1. The SET grammar. A `set_root` is a hash of exactly one of the typed forms below; their domain words are distinct and differ from `SHK_RUN`, `SHK_DEPS` and every state and memory domain. All are pinned-sponge hashes computed in the recursion.
   - `LEAF(claim_d, n_steps) = H(SHK_SET_LEAF, claim_d, n_steps)`: a sealed run with no dependency.
   - `DEP(claim_d, n_steps, cover_root, in_link) = H(SHK_SET_DEP, claim_d, n_steps, cover_root, in_link)`: a sealed run with discharged dependencies. `cover_root` is a `set_root`; `in_link` is the claim digest of the producer whose Halted value is this run's input, or the constant `NO_INPUT_LINK`.
   - `JOIN(l, r) = H(SHK_SET_JOIN, l, r)`: a batch of two SETs.
   - `FOREIGN(fclaim_d) = H(SHK_SET_FOREIGN, fclaim_d)`: one foreign claim (GP-3 hook).
   - `ADM(adm_d) = H(SHK_SET_ADM, adm_d)`: one program admission claim `adm_d = H(SHK_ADM, 1, prog_d)` (ADMIT-WORK of NODE INTEGRATION). It is a seal premise, never a run claim or a dependency.
   - `VIEW(vw_d) = H(SHK_SET_VIEW, vw_d)`: one input view claim `vw_d = H(SHK_VIEW, 1, val_d, view_root)` (VIEW-WORK of NODE INTEGRATION, input admission form (b) of AD-17). It is a seal premise, never a run claim or a dependency.
   - `n_steps` is the height `h_hi` of the sealed segment, range-checked by NODE; it is inside the SET leaf and outside `claim_d` (SRC-007).
   - Head claim: `LEAF`, `DEP`, `FOREIGN`, `ADM` and `VIEW` have a head claim (`claim_d`, `fclaim_d`, `adm_d` or `vw_d`), opened with one hash from the root. `JOIN` has none.
   - Members: `members(LEAF) = {claim_d}`; `members(DEP) = {claim_d} + members(cover_root)`; `members(JOIN(l, r)) = members(l) + members(r)`; `members(FOREIGN) = {fclaim_d}`; `members(ADM) = {adm_d}`; `members(VIEW) = {vw_d}`. "Each claim can be opened from `set_root`" (GP-002 acceptance b) means a path of such one-hash openings from the root to a node whose head claim is the claim.
2. Accumulation in the consumer. The state word `deps_d` (header of the state commitment) is a chain over the claims the run consumed, in consume order, with repetition: `deps_d = EMPTY_DEPS` at INIT (`O/design.md:262`, field 39; `EMPTY_DEPS` is a fixed non-zero constant), and each consume event sets `deps_d' = H(SHK_DEPS, deps_d, claim_d')`. The chain holds only direct dependencies; a producer's own ancestors never enter it.
3. Authenticating one claim inside a verified SET, in bounded work per node.
   - A verified SET child with root `R` gives `SUB(R, R)` for free.
   - DESCEND: from `SUB(R, x)` (or SET `R` with `x = R`) and a one-hash opening of `x` as `JOIN(l, r)` or as `DEP(c, n, cov, link)`, emit `SUB(R, l)`, `SUB(R, r)` or `SUB(R, cov)`. One hash per node; a path of depth `d` takes `d` unary nodes. Every sub-root reached this way is the root of a SET that was verified as part of `R` (Lean GP2-L2).
   - Head opening: from `SUB(R, x)` with `x` a `LEAF`, `DEP` or `FOREIGN` form, one hash gives the head claim `c` and, for a run claim, its `n_steps`.
   - So a claim is discharged from the producer's own sealed proof (its root has the head claim: zero DESCEND steps) or from any batch or DAG proof that contains it (some DESCEND steps). No batching shape of the producer is required.
4. Discharge list. `DL = (chain_d, cover_root, in_link, prem_root)` states: "`chain_d` is the `SHK_DEPS` chain over the head claims of verified SETs, in order; `cover_root` joins the roots they were verified under; `in_link` is the claim marked as the input link, or `NO_INPUT_LINK`; `prem_root` is a sub-root of a verified SET that holds the seal premises, or `NO_PREMISE`". A chain element and the input-link element take only a `LEAF`, `DEP` or `FOREIGN` head; an `ADM` or `VIEW` head enters only through the premise element.
   - DL-STEP, chain element: from an optional `DL(ch, cov, link)` (absent: `ch = EMPTY_DEPS`, `cov` absent, `link = NO_INPUT_LINK`) and `SUB(R, x)` with head claim `c`: emit `DL(H(SHK_DEPS, ch, c), cov', link)` with `cov' = R` if `cov` is absent, else `JOIN(cov, R)`.
   - DL-STEP, premise element (exactly one per list that a seal uses, and before any other element): from an optional DL and `SUB(R, x)`, emit the DL with `prem_root = x`; `chain_d`, `cover_root` and `in_link` are unchanged, so a premise never enters the cover or the chain.
   - DL-STEP, input-link element (at most one per list, and only before any chain element): the same with `ch` unchanged and `link = c`.
   - A claim consumed twice appears twice in the chain and takes two DL-STEPs; the same producer proof may serve both. One producer proof may serve any number of consumers.
5. SEAL-WITH-DEPS `SEG + DL -> SET`: the SEAL checks (`h_lo == 0`, terminal header, the premise check on `DL.prem_root`) and `DL.chain_d ==` the opened `deps_d`. If `DL.in_link = c_A` is set: the consumer's opened `input_d` is re-hashed from a witnessed `(val_d, fuel_lo, fuel_hi, stack_limit)`, `c_A` is re-hashed from a witnessed `(prog_d', input_d', outcome_d')`, and `outcome_d' == H(O, 1, 0, val_d)`: the producer Halted and the consumer's input value digest is the producer's value digest. Output `set_root = DEP(claim_d, n_steps, DL.cover_root, DL.in_link)`.
6. Empty dependencies: a run with `deps_d = EMPTY_DEPS` and no input link seals with SEAL to a `LEAF`. SEAL refuses any other `deps_d`. A run with `deps_d = EMPTY_DEPS` and an input link seals with SEAL-WITH-DEPS from a DL that has only the premise element and the input-link element. SEAL takes its premise in slot 1 and needs no DL.
7. The consume transition against the owner machine, and its accounting.
   - Input consumption (GP-002 acceptance a: "a run of program B whose input is the verified result of a run of program A"). There is no owner-machine transition: the reference machine has no consume rule (`grep` finds no `deps` or consume rule in `crates/`; the only source is field 39 of the Init table), the run of B is the ordinary run of B on its input, `deps_d` stays `EMPTY_DEPS`, and no step, fuel or counter is charged for consumption. The check is the input link of item 5, at the seal. The value is passed by digest equality (`val_d` of B equals the `vd` behind `outcome_d'` of A); no structured value is opened in the node. Inside B's run the input value is reached through INIT's input admission (kit INIT): under form (b) of AD-17 a step reads it through `AuthIn` under the certified `view_root`, so a wide constructor needs no fold in a step.
   - Open reading, for the owner (AD-20): SRC-021 says that when the first execution segment begins the proof authenticates "the input value, the dependency claims and every resource-policy parameter" (`O/requirements.md:46`). This protocol authenticates the resource parameters and `deps_d = EMPTY_DEPS` at genesis (kit INIT) and the claim behind the input at the seal (the input link). If the owner requires the producer claim to be bound at genesis, the input-link element moves into the state at INIT (a header word `in_link` set at genesis and compared at the seal); the node relation does not change.
   - Mid-run consumption (a step that consumes a claim while the run executes) needs an owner-machine rule that `sherlock-k-a/1` does not have. Items 2 to 6 are its circuit side. Its transition contract, for the spec amendment that adds it: one owner event; it reads a claim digest and, if it uses the value, binds the Halted wrapper to a value reference by digest; it sets `deps_d' = H(SHK_DEPS, deps_d, claim_d')`; it charges fuel and counters as the amendment states; its appends are within the per-rule maximum. Adding the rule is an amendment of the governing VM specification: OWNER (section 4, AD-20). Until then `deps_d` is constant in every run and the chain part of DL is exercised only by tests of the node relation.
- When a consumer uses the value behind `outcome_d'`, it opens the Halted wrapper (GP-002; A:714). A Rejected or Exhausted claim can be consumed as a claim (a chain element), never as an input link and never relabelled as a value (A:706).
- Consumer input: `input_d = H(SHK_IN, val_d, fuel_lo, fuel_hi, stack_limit)`, fuel limbs and stack limit in `[0, 2^64)`, range-checked by INIT at genesis (A:711-714; G:47).
- Batching: JOIN `SET + SET -> SET` (GP-4 hook); the collection statement `[reg_root, set_root, shape_d]` (L AND WRAP), each member claim openable from `set_root` by the path openings of item 1.
- No claim count anywhere (R-5 merge decision).

#### Invariants
Guaranteed: only head claims of verified SETs are discharged; the linked input value digest equals the producer's Halted value digest; no element of the consumer's `deps_d` chain is dropped, reordered or added before the consumer is sealed (chain equality at SEAL-WITH-DEPS); SEAL refuses a non-empty chain; one claim may feed many consumers and may be consumed twice; the producer's batching shape and its own ancestors do not matter; ledger replay is keyed by `claim_d` alone (A:720-725).
Assumed: VALUE DIGEST defines canonical value encodings; NODE SEAL is sound (NODE-L2); INIT sets `deps_d = EMPTY_DEPS` and authenticates `input_d`; if a mid-run consume rule is added, CORE writes `deps_d` as item 2 states.

#### Row budget
ESTIMATE, in library-sponge hashes per node (rows not measured; X12): DESCEND one hash; DL-STEP one head opening, one chain hash and at most one JOIN hash; SEAL-WITH-DEPS the SEAL hashes plus, for an input link, one `input_d` re-hash, one claim re-hash and one Halted wrapper. Every mode is a node of the fixed node circuit, so its real cost is one node proof per DESCEND or DL-STEP. A:739 gives about 22 rows per two-input Poseidon level (citing `research/sherlock-processor-analogy-20261110/experiments-2.md:157-164`; not checked). Input admission is charged to INIT and VALUE DIGEST.

#### Verification
Quint: new `models/sherlock/quint/claimConsumption.qnt` (A:745): the SET grammar, DESCEND, DL-STEP and both seals as actions over an abstract injective hash; a consumer and producers built under different batchings.
Quint invariants: `DepsDischargedIffVerifiedSet` (G:118 (d)), `SegmentNeverConsumed` (new), `DischargeIndependentOfProducerShape` (the same consumer seals whether a producer's claim is offered as its own sealed proof, inside a JOIN of either bracketing, or inside a DEP cover), `ChainOrderAndRepetitionExact`, `EmptyDepsOnlyBySeal`, `InputLinkHaltedOnly`.
Positives: `pos_gp2_b_consumes_a_halted` (GP-002 acceptance (a), by input link); `pos_gp2_claim_from_join_left_and_right_bracketing` (DESCEND paths of depth 2 in both bracketings); `pos_gp2_claim_from_dep_cover` (a nested dependency); `pos_gp2_repeated_consumption` (chain with one claim twice, node-relation test); `pos_gp2_empty_deps_plain_seal`; `pos_gp2_struct_value_across_programs`; `pos_gp2_three_runs_collection` (acceptance (b)); `pos_gp2_one_producer_two_consumers`; `pos_gp2_rejected_as_metadata`.
Named mutants: `claim_consume_unsealed_segment`, `claim_wrong_set_root`, `claim_swap_outcome`, `claim_skip_value_opening`, `claim_rejected_as_value`, `claim_drop_dependency`, `claim_seal_undischarged_deps`, `claim_address_in_identity`, `claim_steps_in_identity` (A:757-765; G:131), `claim_chain_reordered`, `claim_chain_dedup_one_step_for_two` (one DL-STEP offered for a claim that the chain holds twice), `claim_descend_wrong_domain` (a `LEAF` hash opened as a `JOIN`), `claim_head_of_join` (a JOIN root offered as if it had a head claim), `claim_sub_root_unrelated` (a `SUB` whose `sub_root` was not reached from the verified root), `claim_input_link_rejected_producer`, `claim_input_link_value_mismatch`, `claim_nsteps_forged_in_set_leaf`, `claim_plain_seal_with_nonempty_chain`, `claim_premise_in_chain` (an `ADM` or `VIEW` head appended to `chain_d` or used as an input link).

#### Lean obligations
- GP2-L1 (claim chaining). Every member of a SET built by SEAL, SEAL-WITH-DEPS, JOIN and the foreign lift is the claim of a run from Init to a terminal state (or a registered foreign claim), and every element of that run's `deps_d` chain and its input link are members of its `cover_root` (induction on the certificate). File: `clean/fork/Clean/Sherlock/ClaimDag.lean` (G:146), extending the aggregation theorems. Premises: NODE-L2, hash injectivity over the SET domains (hypothesis), VALUE DIGEST obligation 4. Contract: GP-002 consumption. Status: open.
- GP2-L2 (sub-root soundness). If `SUB(R, x)` is derived by DESCEND steps from a verified SET `R`, then `x` is a sub-term of the unique grammar term of `R`, so `members(x)` is a subset of `members(R)`; the discharge relation does not depend on the producer's bracketing. File: `ClaimDag.lean`. Premises: injectivity and domain separation of the SET form hashes (`LEAF`, `DEP`, `JOIN`, `FOREIGN`, `ADM`, `VIEW`). Contract: item 3 of the protocol. Status: open.
- GP2-L3 (chain exactness). `DL.chain_d == deps_d` implies that the list of discharged head claims equals the list of consumed claims, with order and repetition. File: `ClaimDag.lean`. Premises: `chainHash_inj`-style premises for `SHK_DEPS` (`lean/PartitionIndependence.lean:251`, reused), `EMPTY_DEPS` outside the range. Contract: items 2, 4 and 5. Status: open.

#### Code skeleton
Files: `crates/mori-recursion/src/claims.rs`, `consume.rs` (new).
```rust
pub struct ClaimCells { pub claim_d: AssignedNative<F>, pub prog_d: AssignedNative<F>, pub input_d: AssignedNative<F>, pub outcome_d: AssignedNative<F> }
pub struct SubCells { pub set_root: AssignedNative<F>, pub sub_root: AssignedNative<F> }
pub struct DlCells { pub chain_d: AssignedNative<F>, pub cover_root: AssignedNative<F>, pub in_link: AssignedNative<F>, pub prem_root: AssignedNative<F> }
/// DESCEND: open sub_root as JOIN(l, r) or DEP(c, n, cov, link) with one hash; return SUB to the chosen child.
pub fn descend(cx: &mut RecursionContext<'_>, sub: &SubCells, w: &DescendWitness) -> Result<SubCells, Error>;
/// Head opening: sub_root as LEAF, DEP or FOREIGN; returns the head claim digest and, for a run
/// claim, the authenticated n_steps of the SET leaf.
pub fn open_head_claim(cx: &mut RecursionContext<'_>, sub: &SubCells, w: &HeadWitness) -> Result<(AssignedNative<F>, AssignedNative<F>), Error>;
/// DL-STEP: append the head claim to the chain (or mark it as the input link, or set prem_root) and join the cover
/// (the premise element leaves the cover unchanged).
pub fn dl_step(cx: &mut RecursionContext<'_>, dl: Option<&DlCells>, sub: &SubCells, w: &DlStepWitness) -> Result<DlCells, Error>;
/// Input link: re-hash input_d and the producer claim; require outcome_d' == H(O, 1, 0, val_d).
pub fn link_input(cx: &mut RecursionContext<'_>, open: &StateOpeningCells, link: &AssignedNative<F>, w: &InputLinkWitness) -> Result<(), Error>;
/// SEAL-WITH-DEPS: the SEAL checks, dl.chain_d == opened deps_d, the input link if set; emit the DEP form.
pub fn seal_with_deps(cx: &mut RecursionContext<'_>, seg: &SegCells, open: &StateOpeningCells, dl: &DlCells) -> Result<SetCells, Error>;
```

#### Agent reads / must not need to read
Reads: this kit; SRC-021, SRC-022, GP-002 rows; NODE INTEGRATION's SEAL signature and relation table; `open_state_header`; the outcome wrapper formula of VALUE DIGEST (the node opens no structured value; `open_value_node` is used in the leaf by INIT).
Must not need: producer traces, physical memory, verifier gadget internals.

#### Dependencies
NODE INTEGRATION, L AND WRAP (collection statement), VALUE DIGEST (outcome wrapper), INIT (`EMPTY_DEPS`, `input_d`), GP-4 hook (JOIN). CORE only if the mid-run consume rule is added (AD-20).

---

### Kit: GP-3 hook (foreign keys through a registry)

#### Purpose
Admit proofs of circuits other than the Sherlock leaf (Compact contract circuits, other PLONK/halo2 circuits on the same curve and commitment scheme) by a verifying key whose identity is a member of an authenticated registry proved in circuit, and map their public inputs to the common claim form through an authenticated adapter (GP-003, `requirements.md:58`).

#### Interface
No bus tuples. Named cells and lanes:
- `reg_root`: full field. NODE INTEGRATION requires it equal in every enabled NODE or FOREIGN child and in the parent (kit NODE INTEGRATION, `node_reg_root_mixed`). How the collection statement carries it to the contract under the one call-vector layout is an open item of the interface freeze.
- Registry entry preimage: binds the full verification interpretation, not only a key digest: proof system, curve, commitment scheme, SRS id, verifying-key digest, transcript id, public-input schema and exact count, adapter digest, format version, policy (A:817-832). Membership proof against `reg_root` at a fixed depth that is part of the registry schema (A:889).
- Foreign lift relation (FOREIGN key kind of NODE): verifies one adapter-class proof under a witnessed key, proves membership of both the adapter key and the inner foreign key in `reg_root`, requires the adapter verification and the inner verification to complete in circuit (FV-FINAL) with no accumulator folded, and emits a one-claim SET over a foreign claim `H(SHK_CLAIM, KIND_FOREIGN, vk_id_inner, adapter_id, out_d)` (G:62). Foreign claim domain disjoint from `SHK_RUN` (A:171).
- The Sherlock run's key identity is also checked against the registry where the aggregate needs both claims checked (A:879; G:68; GP-003 acceptance).
- Two paths (A:860-865), two separate tasks with separate completion gates (B2 finding 15). The native adapter path (this kit, probe X8) is a milestone. The general verification path in bounded work units is required for the full GP-3 scope (`O/general-purpose.md:33`: Compact contract circuits and other PLONK/halo2 circuits on the same curve and commitment scheme); it is specified as the kit "GP-3 general verifier" below with five subkits, and it is in the build order and the spec outline. Passing the native path never ticks GP-3.
- Registry entry encoding (frozen here; version `sherlock-reg-entry/1`): `entry_d = H(SHK_REG_ENTRY, version, proof_system_id, curve_id, pcs_id, srs_id, vk_d, transcript_id, pi_schema_d, pi_count, adapter_d, policy)`, eleven words after the domain, in this order, pinned sponge; `pi_count` is range-checked below 2^32; a membership path of the fixed schema depth opens `entry_d` under `reg_root`.
- Adapter statement (frozen here): `(vk_id_inner, adapter_id, pi_d, out_d)`, where `pi_d` is the pinned-sponge digest of the verified public inputs in order with their count, and `out_d` is the common claim form the adapter derives; the foreign claim is `fclaim_d = H(SHK_CLAIM, KIND_FOREIGN, vk_id_inner, adapter_id, out_d)`.
- No supported proof is folded into an accumulator, under any SRS: the STARK node folds no accumulator (NODE-004, NODE-007). Its whole verification, final pairing or opening check included, is done in circuit by the general verifier's FV-FINAL subkit, in bounded work units; nothing of it is deferred to `L` or the wrap. The alternative (a second deferred accumulator per SRS, paired by the decider) would put one constant per SRS into the decider key and is not taken (section 4, AD-22).

#### Invariants
Guaranteed: membership authenticates both verifier and adapter; the adapter consumes the verified public inputs; proof parsing refuses trailing or missing data; point encodings get their checks; a foreign field becomes a value digest only through an authenticated adapter relation (GP-003); an unfinished verification emits no claim; no foreign proof is folded into an accumulator, under any SRS (A:867, A:873-879; the STARK node has none).
Assumed: NODE INTEGRATION routes the FOREIGN key kind and keeps `reg_root` equal across the enabled NODE and FOREIGN children and the parent; the registry root in the transcript is the contract's root (L AND WRAP).

#### Row budget
ESTIMATE: a 255-level binary membership path about 255 x 22 = 5,610 hash rows per path before entry hashing (A:887); the foreign lift relation 550,000 to 750,000 rows (G:106, input `architecture.md:218-220`, not checked). Foreign verification and adapter execution are unmeasured.

#### Verification
Quint: new `models/sherlock/quint/foreignRegistry.qnt` (A:898): claim requires dual registry membership; stale or foreign registry root refused.
Quint invariants: `ForeignClaimRequiresDualRegistryMembership` (G:118 (e)), `StaleOrForeignRegRootRefused` (G:456).
Positives: `pos_gp3_sherlock_plus_compact` (GP-003 acceptance); `pos_gp3_two_keys_one_class`; `pos_gp3_distinct_adapters`.
Named mutants: `foreign_wrong_registry_root`, `foreign_key_digest_only`, `foreign_swap_transcript`, `foreign_swap_srs`, `foreign_wrong_input_count`, `foreign_unbound_committed_instance`, `foreign_unregistered_adapter`, `foreign_unregistered_inner_vk`, `foreign_adapter_ignores_input`, `foreign_value_relabel`, `foreign_drop_inner_obligation`, `foreign_trailing_proof_bytes` (A:912-923; G:136).

#### Lean obligations
- GP3-L1. A foreign claim in a valid SET comes from a proof accepted by a verifier whose full entry is in `reg_root`, under an adapter in `reg_root`, applied to that proof's public inputs. File: `clean/fork/Clean/Sherlock/ClaimDag.lean` (G:146). Premises: a `ForeignPremises` hypothesis structure (verifier soundness per registered entry), registry hash injectivity (hypothesis, no `axiom`). Contract: GP-003. Status: open.

#### Code skeleton
Files: `crates/mori-recursion/src/registry.rs`, `foreign_lift.rs` (new).
```rust
pub struct RegistryEntry { /* fields of A:817-832, canonical order */ }
/// Authenticate the full verifier interpretation under reg_root (fixed depth).
pub fn open_registry_entry(cx: &mut RecursionContext<'_>, root: &AssignedNative<F>, w: &RegistryOpeningWitness) -> Result<AdmittedVerifierCells, Error>;
/// Verify the adapter proof under its witnessed key; return inner obligation and public inputs.
pub fn verify_adapter(cx: &mut RecursionContext<'_>, a: &AdmittedVerifierCells, w: &AdapterProofWitness) -> Result<VerifiedForeignInputs, Error>;
/// Apply the registered adapter; emit the one-claim foreign SET.
pub fn adapt_foreign_statement(cx: &mut RecursionContext<'_>, a: &AdmittedVerifierCells, i: &VerifiedForeignInputs) -> Result<SetCells, Error>;
```

#### Agent reads / must not need to read
Reads: this kit; registry entry schema and test vectors; the pinned verifier API for the adapter class (`midnight-circuits` verifier gadget); NODE INTEGRATION's key routing signature.
Must not need: CORE, MEM, continuation internals. The general verifier path is the next kit (parsing, transcript, field and group arithmetic, final equation, scheduling; A:977).

#### Dependencies
NODE INTEGRATION (FOREIGN key kind); L AND WRAP (`reg_root` in transcript; interface-only edge, so it builds in parallel with L AND WRAP). The GP-3 general verifier kit shares the registry entry and the adapter statement.

---

### Kit: GP-4 hook (any shape)

#### Purpose
Let one key set prove chains, trees and DAGs of any depth with node arity at least 2 plus new work, bind the shape into the statement and keep the final proof size and verification cost independent of depth and claim count (GP-004, `requirements.md:59`).

#### Interface
No bus tuples. Named cells:
- Two child slots of NODE INTEGRATION plus one inline work port with a distinct tag per variant: `Disabled` (zero payload), `ExecuteOwnerWork` (A:990-998, without the mode 2 variants), `ForeignVerifyWork` (GP-3 general verifier; administrative) `AdmitWork` (program admission of NODE INTEGRATION; administrative) or `ViewWork` (input view of NODE INTEGRATION; administrative). Administrative work never claims owner-step progress. The legal combinations of child types and work, and the output of each, are the node relation table of kit NODE INTEGRATION; this kit owns the rows MERGE-WORK and JOIN and the shape commitment.
- MERGE-WORK (the "arity at least 2 plus new work" of GP-004, `O/general-purpose.md:46`): two enabled prior SEG proofs plus `k >= 1` real owner steps in one node. The inline kernel constrains the same reference steps as the leaf and exports `(prog_d, pre_d, post_d, rules_d, h_pre, steps)` as cells. Binding: `W.pre_d` equals the `post_d` of the right-most enabled child, `W.h_pre` equals its `h_hi`, and the output height is `W.h_pre + k`, range-checked. Two successive nodes (one that merges, one that works) are a different relation and do not satisfy this requirement, in the design or in the measurement.
- JOIN `SET + SET -> SET`: `set_root = H(SHK_SET_JOIN, l.set_root, r.set_root)`, total at any depth, no counter, no range check (G:70; R-5 merge decision).
- Shape commitment, with its base cases and field order (B2 finding 16). All are pinned-sponge hashes in the recursion; `SHK_SHAPE` is one domain word and the second word is the node type.
  - Node: `shape_d = H(SHK_SHAPE, T_NODE, mode, slot_bits, key_sel_0, key_sel_1, shape_0, shape_1, work_kind, work_steps, out_a, out_b)`, in exactly this order. `mode` is the `NodeMode` number; `slot_bits = en_0 + 2 en_1`; `key_sel_j` is the key kind number of slot `j` (0 when off); `work_kind` is 0, 1 (`ExecuteOwnerWork`), 2 (`ForeignVerifyWork`), 3 (`AdmitWork`) or 4 (`ViewWork`); `work_steps` is `k` or 0; `(out_a, out_b)` is `(h_lo, h_hi)` for a SEG output, `(set_root, 0)` for a SET, `(chain_d, cover_root)` for a DL, `(set_root, sub_root)` for a SUB, and `(0, 0)` for an FV, AW or VW output. A FOREIGN SET that FV-WORK emits at completion, and an `ADM` or `VIEW` SET at the completion of ADMIT-WORK or VIEW-WORK, use the SET rule `(set_root, 0)`. `fv_state_d` commits transcript state and never enters `shape_d`.
  - LEAF child: a leaf proof has no `shape_d` lane. The parent computes `shape_j = H(SHK_SHAPE, T_LEAF, h_pre, steps)` from the leaf's authenticated lanes.
  - FOREIGN child: the foreign lift relation publishes `shape_d = H(SHK_SHAPE, T_FOREIGN, fclaim_d)` in its node statement, and the parent forwards it.
  - Disabled slot: the constant `SHAPE_OFF = H(SHK_SHAPE, T_OFF, 0)`, fixed and non-zero; the parent constrains `shape_j = SHAPE_OFF` when `en_j = 0`.
  - No field is `pre_d`, `post_d`, `body_d`, a memory root, a tail digest, a challenge or an address (A:1035-1045; G:87; GP-004). Heights and claim digests are allowed by GP-004. `shape_d` is in the collection statement; acceptance, replay and consumption never use it.
- Inline work: two enabled prior proofs plus new owner work in one node (A:1024-1028). The inline kernel is built from the same kit `configure` and `assign` functions as the leaf (CORE, DEC, ALU, RANGE, HASH, STACKS, MEM, the memory backend, VALUE DIGEST, INIT), instantiated in the node's constraint system. There is one encoding of the machine relation; no separate node emitter exists, and agreement on differential vectors is not evidence of equal constraints. The kits are generic over the host constraint system through their ports and buses (section 1). Whether this fits the node gate is the deciding experiment; if it does not fit, the result goes to the owner under N-6 and GP-4 is not narrowed (OWNER, section 4.2, N-6).

#### Invariants
Guaranteed: the inline kernel and the leaf share one constraint encoding of the machine relation (the same kit `configure` and `assign` code; test `graph_inline_kernel_separate_encoding`, a build check that refuses an inline kernel that does not instantiate the leaf kits); the shape is bound and address-free, with canonical encodings for leaf, foreign and disabled children; a claim may be reused in two branches; JOIN never overflows or caps; the same claims under different shapes give equal acceptance; inline owner work is linked to the adjacent segment state and height and adds exactly its steps; a combination outside the node relation table is unsatisfiable (A:1020).
Assumed: NODE INTEGRATION's slot verification and routing; CORE's execution contract for inline work.

#### Row budget
ESTIMATE: 1,000 to 5,000 rows for relation selection, `shape_d` hashing (one twelve-input library sponge per node, plus one small sponge per LEAF child) and interval checks, excluding inline work and child verification (A:1057). Inline work gets the measured remainder under 950,000; no result shows that two slots plus one real step fit (A:1059). X7 measures exactly that node: two enabled prior proofs and one real owner step with its memory, stack and value-digest work.

#### Verification
Quint: new `models/sherlock/quint/pcdGraph.qnt` (A:1065), refining `aggregation.qnt` with claim reuse; a symbolic deep diamond DAG accepted with no counter (G:118).
Quint invariant: `DiamondDagDepth64NoOverflow` (G:118 (f)); plus `ShapeNotUsedForAcceptance` (new).
Positives: `pos_gp4_chain_tree_dag`; `pos_gp4_two_prior_plus_step`; `pos_gp4_diamond_reuse` (G:125); `pos_gp4_bracketing_same_claim`. Depth >= 8 for each shape with equal final proof size is a real-proof acceptance test at integration (GP-004).
Named mutants: `graph_leaf_shape_free` (a LEAF child's shape word not derived from its lanes), `graph_disabled_slot_shape_not_off`, `graph_shape_field_order_swapped`, `graph_work_not_bound_to_child_state` (inline work whose `pre_d` is not the child's `post_d`), `graph_work_steps_not_in_height`, `graph_two_nodes_for_merge_work` (not a constraint mutant: the acceptance check refuses a run of two successive nodes as evidence for MERGE-WORK), `graph_omit_relation_tag`, `graph_unbound_slot_bits`, `graph_swap_children`, `graph_state_root_in_shape`, `shape_fv_state_in_shape` (an FV output whose `(out_a, out_b)` is not `(0, 0)`, for example `fv_state_d` in `out_a`), `graph_shape_as_replay_key`, `graph_two_children_disable_work`, `graph_duplicate_execution_interval`, `graph_reject_valid_claim_fanout` (completeness), `graph_admin_work_changes_steps` (A:1077-1086 without `graph_count_wrap`). Build check, not a constraint mutant: `graph_inline_kernel_separate_encoding` (an inline kernel whose constraints do not come from the leaf kits' `configure` and `assign` functions is refused at build).

#### Lean obligations
- GP4-L1. JOIN is total at every depth with no counter; every claim reachable in the joined root is reachable in one child root; acceptance does not depend on `shape_d`. File: `clean/fork/Clean/Sherlock/ClaimDag.lean` (G:146). Premises: hash injectivity over SET domains (hypothesis). Contract: GP-004, R-5. Status: open.
- GP4-L2. A node with two prior SEGs and inline work of `k` steps gives a SEG of the summed interval. File: `BinaryNode.lean`. Premises: NODE-L1, CORE contract via `leaf_refines_step` (C3, `ModularComposition.lean:131`). Contract: "arity >= 2 plus new work". Status: open.

#### Code skeleton
Files: `crates/mori-recursion/src/shape.rs`, `inline_work.rs` (new).
```rust
pub enum InlineWork { Disabled, ExecuteOwnerWork, ForeignVerifyWork, AdmitWork, ViewWork }
/// Hash the frozen address-free descriptor in the fixed field order.
pub fn bind_shape(cx: &mut RecursionContext<'_>, rel: &ConstrainedRelationCells, ch: &[AssignedNative<F>; 2]) -> Result<AssignedNative<F>, Error>;
/// Base cases: H(SHK_SHAPE, T_LEAF, h_pre, steps) from a leaf's lanes; the constant SHAPE_OFF for a disabled slot.
pub fn leaf_shape(cx: &mut RecursionContext<'_>, h_pre: &AssignedNative<F>, steps: &AssignedNative<F>) -> Result<AssignedNative<F>, Error>;
pub fn shape_off() -> F;
/// JOIN: set_root = H(SHK_SET_JOIN, l, r); no count.
pub fn join_sets(cx: &mut RecursionContext<'_>, l: &SetCells, r: &SetCells) -> Result<SetCells, Error>;
/// Requires: the implementation instantiates the leaf kits' configure and assign functions in the node's
/// constraint system (one encoding of the machine relation); a separate emitter is not an implementation.
pub trait InlineExecutionKernel { type Witness;
    /// Constrain real owner work; export the same execution contract as a leaf.
    fn execute(&self, cx: &mut RecursionContext<'_>, input: &SegCells, w: &Self::Witness) -> Result<SegCells, Error>; }
```

#### Agent reads / must not need to read
Reads: this kit; GP-004 row and acceptance; NODE INTEGRATION slot and mode signatures; CORE's exported execution contract (for inline work only).
Must not need: verifier arithmetic, checkpoint formats, memory columns, foreign transcripts.

#### Dependencies
NODE INTEGRATION; CORE (inline work); L AND WRAP (collection statement; interface-only edge, so it builds in parallel with L AND WRAP).

---

### Kit: GP-3 general verifier (five subkits)

B2 finding 15. The GP-3 hook above is the native adapter path. This kit is the general path that the full GP-3 scope needs; it is several sessions and is split into five subkits, each with one agent. Nothing in it is built or measured.

#### Purpose
Verify, in circuit and in bounded work units spread over several nodes, a proof of any registered PLONK/halo2 circuit on the same curve and commitment scheme, under the verification interpretation of its registry entry, and emit its foreign claim only when the whole verification has completed.

#### Interface
No bus tuples. One statement type `FV = (fv_state_d)`, whose registry root is the node's `reg_root` lane (kit NODE INTEGRATION constrains it across slots), and the inline work variant `ForeignVerifyWork` of the node (row FV-WORK of the node relation table).

- Authenticated bounded-work state: `fv_state_d = H(SHK_FV, entry_d, phase, cursor, transcript_d, acc_d, pi_d, consumed_d, work_root)`, pinned sponge, in this order. `entry_d` is the registry entry (GP-3 hook encoding) opened once at the first unit; `phase` is the phase number of the schedule; `cursor` counts consumed proof elements; `transcript_d` commits the Fiat-Shamir sponge state; `acc_d` commits the partial sums of the arithmetic phase; `pi_d` is the running digest of the public inputs; `consumed_d` is the running digest of every proof element consumed so far, so all units read one proof; `work_root` is the root of an indexed Merkle store of typed verifier records `(index, kind, value)`, empty at the first unit.
- Operand store: cells do not survive a unit boundary, and a running digest can be checked only by a replay that the consume-once rule forbids. So every value that a later unit uses is a record under `work_root`. FV-PARSE writes each parsed element at index `cursor`; FV-TRANSCRIPT writes each challenge at the fixed index that the schedule assigns to it; FV-ARITH and FV-FINAL read elements and challenges only through openings against `work_root`, never from a witness. A write is an update of `work_root` at an index that the schedule marks empty; a record is never overwritten.
- One work unit: FV-WORK takes an optional `FV` child (absent only for the first unit, which opens the registry entry), performs at most the unit budget of operations of the current phase, and emits the next `FV`, or, when `phase = DONE`, a SET in the `FOREIGN` form with `fclaim_d` from the adapter statement. A unit is administrative work: it changes no height and counts no owner step.
- Subkits and what each owns:

| Subkit | Owns | In | Out |
|---|---|---|---|
| FV-PARSE | the proof element stream: decoding points and scalars from witness elements in the registered format, encoding checks (on-curve, subgroup where required, canonical scalars), `cursor` and `consumed_d`, refusal of missing and trailing elements at `DONE`, the write of each element at index `cursor` of `work_root` | registry entry format fields | typed element cells and their `work_root` records |
| FV-TRANSCRIPT | the registered transcript: absorb and squeeze in the order of the registry's `transcript_id`, `transcript_d`, challenge cells, the write of each challenge at its schema index of `work_root` | typed elements | challenges and their `work_root` records |
| FV-ARITH | scalar-field and group arithmetic of the verifier equation in unit-sized steps: evaluation of the gate and permutation expressions at the challenge point, the multi-scalar multiplication of the opening equation as a resumable sum (`acc_d`) | elements and challenges, each read by an opening against `work_root` | the final opening claim |
| FV-FINAL | the final check, for every SRS: the pairing or final opening check is executed in circuit, in work units, with that SRS's verifier parameters taken from the registry entry; nothing is deferred and no accumulator is folded (the STARK node has none) | the opening claim, `srs_id`; any element or challenge by an opening against `work_root` | accept, or an accumulator |
| FV-SCHED | the phase schedule, the operation inventory with a cost per operation, the unit budget, the state digest, the index schema of `work_root` per `transcript_id` (which index holds which element or challenge), its initialization (the empty root at the first unit), the completion gate (`DONE` only after every phase and with every scheduled record written), and the adapter application at `DONE` | everything above | `FV`, or the foreign SET |

- The operation inventory (which operations exist, their row cost, and how many fit one unit beside the node's own child verification) is not known; X14 decides it. The `work_root` cost is one more word in the state sponge per unit (about one permutation) and one Merkle opening per cross-unit read or write, inside the unit budget that X14 sizes (ESTIMATE; the depth of the index schema is fixed per `transcript_id`).

#### Invariants
Guaranteed: an unfinished verification emits no claim; every unit continues exactly the state its predecessor committed; all units consume one proof stream in order, with no element skipped, repeated or left over; every element or challenge that a later phase uses is the value absorbed or squeezed earlier, read from `work_root` at its schema index; the registry entry opened at the first unit governs every later unit; every foreign proof, under any SRS, is fully checked in circuit and folds no accumulator; the adapter is applied to the verified public inputs (`pi_d`) only.
Assumed: NODE INTEGRATION verifies the `FV` child and routes its key; the registry root is the contract's (L AND WRAP); soundness of each registered proof system (`ForeignPremises`).

#### Row budget
Unknown. No figure is given: there is no operation inventory. The constraint is the node shape: one work unit plus the verification of one `FV` child must fit a fixed STARK node shape inside the bounds of the internal set (DG-004, DG-006; the halo2 gate of 950,000 rows is history, DG-007) (X14). If no unit size fits, GP-3 stays incomplete; the general path is not dropped and the scope is not reduced to adapter classes.

#### Verification
Quint: `models/sherlock/quint/foreignRegistry.qnt` extended with the phase schedule: invariants `NoClaimBeforeDone`, `UnitsContinueCommittedState`, `ProofStreamConsumedExactlyOnce`, `LaterPhaseReadsAbsorbedValue`.
Positives: `pos_fv_plonk_proof_same_srs`, `pos_fv_plonk_proof_other_srs`, `pos_fv_unit_boundaries_every_phase`, each against a host verifier of the same registry entry (differential).
Named mutants: `fv_claim_at_unfinished_phase`, `fv_state_forked` (a unit that continues a state other than its child's), `fv_skip_proof_element`, `fv_trailing_element_ignored`, `fv_challenge_not_from_transcript`, `fv_point_not_on_curve`, `fv_other_srs_folded_into_core_acc`, `fv_entry_swapped_mid_run`, `fv_adapter_on_unverified_inputs`, `fv_operand_substituted` (an MSM term uses a commitment other than the one absorbed), `fv_challenge_index_swapped` (a challenge read from another schema index).

#### Lean obligations
- GP3-L2. If FV-WORK units chain from the first unit to `DONE` and emit `fclaim_d`, then the registered verifier of `entry_d` accepts the consumed proof stream on the public inputs behind `pi_d`. File: new `clean/fork/Clean/Sherlock/ForeignVerify.lean`. Premises: hash injectivity for `SHK_FV` and for the `work_root` store, the per-phase step relations as a hypothesis structure (the arithmetic is checked by MockProver and differential tests, not Lean). Contract: completion gate and stream integrity. Status: open.

#### Code skeleton
Files (new): `crates/mori-recursion/src/fv/{mod.rs, parse.rs, transcript.rs, arith.rs, final_check.rs, sched.rs}`.
```rust
pub struct FvCells { pub fv_state_d: AssignedNative<F> }   // reg_root is the node lane, not FV payload
/// One typed verifier record of the operand store; opened or written against work_root at a schema index.
pub struct FvRecord { pub index: u64, pub kind: FvRecordKind, pub value: Vec<F> }
pub enum FvPhase { Open, Parse, Transcript, Arith, Final, Done }
/// The frozen operation inventory with a cost per operation and the unit budget (from X14).
pub struct FvSchedule { /* phases, ops, unit_budget */ }
/// One bounded unit: continue `prev` (or open the registry entry), run up to the unit budget, emit the next
/// state, or the foreign SET at Done.
pub fn fv_work(cx: &mut RecursionContext<'_>, prev: Option<&FvCells>, w: &FvUnitWitness, s: &FvSchedule) -> Result<FvOut, Error>;
pub enum FvOut { Next(FvCells), Done(SetCells) }
```

#### Agent reads / must not need to read
Each subkit agent reads: this kit; the registry entry and adapter statement of the GP-3 hook; the pinned verifier of the target proof system as a host reference; NODE INTEGRATION's relation table row FV-WORK. Must not need: CORE, MEM, the leaf circuit, continuation, GP-2 internals.

#### Dependencies
NODE INTEGRATION (FV-WORK row, key routing), GP-3 hook (registry, adapter, foreign SET form), L AND WRAP (`reg_root`). X14 before any subkit starts.

---

### Kit: INIT (genesis and input admission)

B2 finding 3. In the B1 text CORE assumed a canonical Init, the memory kits assumed an empty or proved genesis root, PAGE/ROM delegated the `prog_d` opening to an unnamed owner, VALUE DIGEST assumed an authenticated `val_d`, and NODE treated genesis as proved. This kit is the one owner of all of these. It also provides condition C4 of the C1 Quint lane.

#### Purpose
When `h_pre = 0`, prove in the leaf that the first pre state is `Init(prog_d, input_d)` field by field (SRC-021, `O/requirements.md:46`; `O/design.md:172-262`), with the program and the run input authenticated by opening `prog_d` and `input_d`. Equality with a supplied digest is not that check.

#### Interface
The genesis selector is `g = is_zero(h_pre)`, computed from the public lane `h_pre` by an is-zero gate. It is not a witness flag (`O/design.md:172`). Every constraint below is multiplied by `g`; at `h_pre != 0` INIT constrains nothing, and the pre state is bound by the previous leaf's `post_d`.

Inputs (copied cells): `h_pre` and `prog_d` (leaf lanes, from CORE and BOUNDARY COMMITMENT); the `BoundaryCells` of the first pre state (every header and body word, from BOUNDARY COMMITMENT's `commit`); the memory frontier `len_pre` and the token `pre = [root, tail_d]`; the stack cells of both stacks.

Openings (pinned sponge, whole-sponge HASH slots, inputs copied):
- `prog_d`: the SVM-005 preimage (`O/design.md` section 2.4; `:90`, `:174`): `prog_root`, the layout version and the profile identity, and the entry metadata, which is `Expr(e0)` or `Main(in_fn, in_ty)`. The opened `prog_root` is constrained equal to the `prog_root` body word; ROM lines of every leaf use that register.
- `input_d = Poseidon(SHK_IN, val_d, fuel_lo, fuel_hi, stack_limit)`. `fuel_lo`, `fuel_hi` and `stack_limit` are each range-checked below 2^64 (RANGE), so the u128 fuel has one encoding. The opened digest is constrained equal to the `input_d` header word.

The Init relation, for one arena (every initialized field; the numbers are the fields of the table at `O/design.md:222-262`):

| Field(s) | State word(s) | Init value |
|---|---|---|
| 1-4 control | control tag, expression, rest, origin | `Eval` at `e0` for `Expr(e0)`, at the reserved `ENTRY` for `Main`; rest and origin 0 |
| 5 outcome | `outcome_tag`, `outcome_payload` (header) | `Running`, 0 |
| 6-8 fuel | initial fuel, remaining fuel, consumed work | `fuel_lo + 2^64 fuel_hi` in both fuel registers; 0 |
| 9-11 | helper work, helper visits, reference cursor | 0 |
| 12 | `stack_limit` | from the `input_d` opening |
| 13-14 | `frame_count`, `task_count` | 0 |
| 15 | environment pointer | `None` |
| 16-17 `fp`, `tp` | `frames_below`, `tasks_below` | the constants `EMPTY_F`, `EMPTY_T` (the `NIL` of the table is the empty-stack constant of a hash-chain stack, `O/design.md:265`) |
| 18-21 | cached top frame (13 lanes), top task (7), helper (23), result (4) | 0 |
| 22 | scratch `v0..v2` | 0 (kept because SRC-021 lists it; the value is never read, CORE invariant 8) |
| 23-25 | allocation counters (data nodes, data fields, environment cells) | `n_d`, `n_f`, 0; both are 0 under the recommended input form |
| 26-29 | per-kind active lengths, where the machine keeps them as registers | `n_d`, `n_f`, 0, 0; 0 under the recommended input form |
| 30-32 | segment counters | 0 |
| 33 | `prog_root`, `prog_d` (body words) | the opened `prog_root`; the lane `prog_d` |
| 34-37 four table roots | one arena: `len`, token `[root, tail_d]` | the genesis memory `(len_0, root_0, tail_0)`. Under the recommended input form: `(0, empty_at(TOP + 3), tail_d([]))`, two circuit constants (`chip/dense.rs:86-119,144-152`). This is condition C4 |
| 38 `fin` | (pure profile) | absent, 0 |
| 39 `deps` | `deps_d` (header) | the constant `EMPTY_DEPS` |
| (new) | `input_d` (header) | the opened `input_d` |
| (new) | constructor fold cells | 0, not opened |
| (new) | input reference | the form of AD-17; under the recommendation `(present, val_d, in_ty)` |
| (new) | `view_root` (body word, form (b) of AD-17 only) | from the witness; the constant `VIEW_NONE` when `val_d` is the absent-input constant (INIT constrains this from the `input_d` opening, and `read_input` refuses `VIEW_NONE`). The seal ties any other `view_root` to `val_d` by a `VIEW` claim (kit NODE INTEGRATION) |

The one-arena column replaces the four `(root, length)` pairs of SRC-021 and the note "its root SHALL open to these four pairs" (`O/design.md:266`). That is an edit of the governing specification: section 4, AD-18 (OWNER).

SRC-021 also requires that at the first segment the proof authenticates "the input value, the dependency claims and every resource-policy parameter that can affect the outcome" (`O/requirements.md:46`). Here: the resource parameters are the fuel and the stack limit of the `input_d` opening; the input value is the input-admission contract below; the dependency claims are `deps_d = EMPTY_DEPS` at genesis, with the producer of an input authenticated at the seal by the input link of the GP-2 hook (the reading that AD-20 puts to the owner).

Admissibility (`O/design.md:214`): `Expr(e0)` takes no input (`val_d` is the absent-input constant, open in the spec); `Main(in_fn, in_ty)` requires a present input whose type is `in_ty`; under form (b) the type is checked on the inspected nodes of the input only (I7). A statement outside these two cases has no Init and the gate is unsatisfiable.

Input admission: how the value behind `val_d` becomes readable by the run. The governing specification leaves this open (`O/design.md:194`: a loading prefix before `Eval(ENTRY)`, or a lazy unfold; the Init values of fields 23, 24, 26, 27, 34 and 35 depend on it), and the owner keeps the final call on SRC-021 (owner question 1 of `O/dissent.md`). INIT does not decide it. INIT fixes the contract that any form must meet, and the kit is built against that contract:

- I1. No constraint or step scans the whole input; the work of the genesis leaf does not grow with the input (the kill criterion of `O/design.md:172`).
- I2. No bound on the size of an input is introduced, in the circuit or in admission.
- I3. Every input node that a step uses is authenticated against `val_d` before it is used, in bounded work per step, by `open_value_node` of VALUE DIGEST or by an equivalent digest check.
- I4. The run of `prog` on `x` has the outcome of the reference run of `apply(prog, x)` (`O/design.md:216`), and `n_steps` is defined by the owner-event count of that run under the chosen form, the same for every partition.
- I5. The genesis memory `(len_0, root_0, tail_0)` is a function of `(prog_d, input_d)` and is canonical; every record in it has `kind` in `1..=4` (condition C4).
- I6. A structured value and a relocated or shared copy of it have one `val_d` (SRC-023).
- I7 (form (b)). `in_ty` admissibility is checked on inspected nodes only: a node that a step reads through `AuthIn` has the type that the step's rule requires; an uninspected subtree is carried by digest, and the claim is relative to `val_d`. The alternative, a VIEW-WORK check of `ty(x) == in_ty` over the whole input, is an owner choice under AD-17.

Candidate forms (section 4, AD-17, OWNER; X11 measures them): (a) a loading prefix that writes `enc(x)` into memory before `Eval(ENTRY)`: it meets I5 only if INIT proves the root of `enc(x)` from `val_d`, which is a scan (fails I1) unless the loading is done by counted steps, which changes `n_steps` against the reference (I4); (b) by-digest admission with a certified read-only input view: the genesis memory is empty, the input is held as a reference by digest, and a step that inspects an input node reads it through the read port `read_input` (an `AuthIn` token) under the `view_root` body word, which the seal ties to `val_d` by a `VIEW(val_d, view_root)` claim of the node mode VIEW-WORK (kit NODE INTEGRATION). Field `k` of any node, of any arity, is one Merkle opening of fixed depth for its digest, plus one more for the child's record when the step inspects the child, so a wide constructor needs no fold in a step, and a read adds no MEM append. It meets I1, I2, I3, I5, I6 and I7; its open points are the depth `D_view`, the full or lazy view (below) and the seal read path of `view_root` (AD-17); (c) an input constant table under a committed root with a digest check on every row read, which is (b) with the prover's table as a hint. Recommendation: (b), with the empty genesis memory. Until the owner decides, the kit is built with the genesis memory as a parameter `GenesisMem` and tested with the empty one.

Requests: RANGE (three 64-bit checks); HASH (two pinned whole-sponge slots); no bus tuple of its own. Under form (b): the `view_root` body word from the witness. Records, if the chosen form reads any at genesis, come through CORE's read port.

#### Invariants
Guaranteed, when `h_pre = 0`: the pre state is the unique `Init(prog_d, input_d)`; `prog_root` is the root opened from `prog_d`; `input_d` in the header is the hash of the ranged fuel limbs, the stack limit and `val_d`; both stacks are empty with their constants; `deps_d = EMPTY_DEPS`; the genesis memory is the canonical `GenesisMem` (C4); the entry form and the input presence agree. The gate is `h_pre == 0` itself, so a segment chain whose first leaf has `h_pre = 0` cannot start from another state, and SEAL requires `h_lo == 0`.
Assumed: BOUNDARY COMMITMENT returns the true cells of the first pre state; HASH and RANGE; VALUE DIGEST for the digest relation of an opened input node. PROG canonicality is not proved here: it is the named premise `ProgCanonical(prog_root)` of the leaf (Lean) and the host differential `rom_ty_d_matches_admission`. Its discharge point is the seal: SEAL and SEAL-WITH-DEPS require the `ADM` claim of ADMIT-WORK for the sealed `prog_d` (kit NODE INTEGRATION, NODE-L4). AD-14 (OWNER) fixes the `ty_d` normal form inside that check.
Not provided here: a proof that the program is well typed; admission is checked at the seal, not in the leaf; the absent-input constant under `sherlock-value/1` (open in the spec, `O/design.md:214`).

#### Row budget
ESTIMATE (nothing built): the `prog_d` sponge (its word count is fixed by SVM-005; 6 to 8 words is 210 to 280 rows); the `input_d` sponge (5 inputs, 210 rows); three 64-bit range checks (about 21 rows); the is-zero gate (1 to 2 rows); the gated equalities, which sit on rows that already hold the first pre state or take about one row per 6 words. About 500 to 700 rows in every leaf, because the circuit is uniform. The spike measured 27,736 rows for the Init check of the C6 digest leaf with four table roots (`O/design.md:172`); that figure is not of this design. X5 measures INIT inside the integrated leaf.

#### Verification
- Quint: the existing `models/sherlock/quint/init.qnt`, extended to the one-arena field list and the genesis selector. Invariants: `GenesisStateIsInit`, `NonGenesisUnconstrainedByInit`, `InitFieldsCoverSchema` (every schema word has an Init value), `GenesisMemCanonical`.
- MockProver positive: `init_expr_entry_no_input`, `init_main_entry_with_input`, `init_fuel_limbs_boundaries` (0, 2^64 - 1 in each limb), `init_non_genesis_leaf_any_state` (at `h_pre != 0` the gate is off), `init_schema_coverage` (a host check that every word of the frozen schema appears in the Init table).
- Named mutants: `init_wrong_control`, `init_fuel_differs_from_input_d`, `init_fuel_limb_2_64`, `init_stack_limit_unbound`, `init_outcome_not_running`, `init_counter_nonzero`, `init_stack_head_not_empty_const`, `init_cached_top_nonzero`, `init_memory_not_genesis` (a non-empty tree or a non-canonical tail at `h_pre = 0`), `init_genesis_record_kind_out_of_range` (only if AD-17 gives genesis records), `init_deps_not_empty`, `init_prog_root_not_opened` (a `prog_root` register that is not the root inside `prog_d`), `init_input_d_not_opened`, `init_input_d_swapped_opening` (the opening of another `input_d`; SRC-021 acceptance), `init_entry_input_mismatch` (`Main` with no input, or an input of the wrong shape or `in_ty`), `init_gate_at_nonzero_height` (completeness: Init wrongly forced at `h_pre = 5`), `init_input_node_unauthenticated` (a step uses an input node that was not opened against `val_d`), `in_read_outside_view` (form (b): an `AuthIn` record that is not under `view_root`, or an index not reached from the input root). Control circuits: `ctl_init_gate_from_witness_flag` (the selector taken from an advice bit: a leaf at `h_pre = 0` with the flag off starts from any state) and `ctl_init_supplied_digest_only` (equality with a supplied `init_d` and no field relation).

#### Lean obligations
| Obligation | File | Premises | Contract discharged | Status |
|---|---|---|---|---|
| A leaf accepted with `h_pre = 0` has the pre state `Init(prog_d, input_d)`; Init is a function of `(prog_d, input_d)` | new `lean/InitAdmission.lean` | hash premises for the two openings, RANGE contract | the carrier-state premise of `leaf_sound` and `decider_pinning` (`lean/Aggregation.lean:204-213,1110-1120`); SRC-021 | open |
| The genesis memory is well formed and canonical, and every stored kind is in range for every reachable state (base case of the induction) | `lean/InitAdmission.lean` with `Mem.wf_unique` | the window kind gate | condition C4; LC AUDIT item 7 | open |

#### Code skeleton
Files (new): `chip/init.rs`, `chip/init/tests.rs`.
```rust
pub struct ProgOpening { pub prog_root: Cell, pub layout_profile: Cell, pub entry: EntryCells }
pub struct InputOpening { pub val_d: Cell, pub fuel_lo: RangedCell, pub fuel_hi: RangedCell, pub stack_limit: RangedCell }
/// The genesis memory of the chosen input form; the empty arena for the recommended form.
pub struct GenesisMem { pub len: F, pub root: F, pub tail_d: F }
impl InitCfg {
    pub fn configure(meta: &mut ConstraintSystem<F>, ports: &SharedPorts, schema: &BoundarySchema) -> Self;
    /// g = is_zero(h_pre). When g = 1: open prog_d and input_d and constrain every schema word of `pre`,
    /// the memory frontier and token, and both stacks to Init(prog_d, input_d). When g = 0: nothing.
    pub fn constrain(&self, pc: &PoseidonCfg, r: &mut Region<'_, F>, o: &mut usize, h_pre: &Cell, prog_d: &Cell,
                     pre: &BoundaryCells, mem: (&Cell, &[Cell]), w: &InitWitness) -> Result<(ProgOpening, InputOpening), Error>;
}
/// Native Init, shared with the witness generator, the reference bridge and the checkpoint store.
pub fn init_state(prog: &ProgOpeningNative, input: &InputNative, g: &GenesisMem) -> NativeState;
```

#### Agent reads / must not need to read
Reads: this kit; SRC-021 (`O/requirements.md:46`); `O/design.md:166-266` (Init table, openings, admissibility, where the input enters); the frozen `BoundarySchema`; kits HASH and RANGE; `chip/dense.rs:86-152` (empty constants); `models/sherlock/quint/init.qnt`. Must not need: rule bodies, Merkle walks, the node, the decider.

#### Dependencies
BOUNDARY COMMITMENT (cells of the first pre state), HASH, RANGE, CORE (state schema; the read port if genesis records exist), STACKS (empty constants), MEM and MEMORY BACKEND (frontier and token cells), VALUE DIGEST (`open_value_node`), CORE (`read_input` under form (b)), NODE INTEGRATION (the `VIEW` claim at the seal under form (b)). The owner decisions AD-17 and AD-18 and experiment X11 before the input part freezes; the rest of the kit does not wait for them.

---

### Kit: REFERENCE BRIDGE (semantic reference and projection)

B2 finding 14. The whole-leaf harness of section 3.2 compares the circuit with a reference. The reference it nominated cannot represent the corpus it requires, and no kit closed the gap.

#### Purpose
Supply a semantic reference for every program of the target language and the map from circuit events to reference steps, so that the whole-leaf differential harness is an executable acceptance gate. It is a host-side kit: it adds no constraint.

#### Interface
What exists (checked): the reference evaluator `Machine` of `crates/mori-core/src/sherlock/reference.rs` works on `Value::{UInt, Bool, List, Tree}` and on expressions without constructors (`reference_types.rs:21-26`, `:69-78`); the projection from owner-machine observations to that reference returns `UnsupportedGeneric` for generic nominal constructs, planned matches and constructor state (`helper_machine/reference_projection.rs:10-11`, `:46-50`). The GP-001 acceptance run "a constructor of more than 8 fields across a leaf boundary" (`O/general-purpose.md:16`) is therefore outside both today. The owner machine itself runs these programs (the 12 `XAdtHelperBuild*` and `XAdtHelperPlan*` rules; a 10-field constructor is in the corpus of `FD/EXPERIMENTS-MAXIMA.md`).

Deliverables:
- R1. A full-language semantic evaluator: nominal types, constructors of any arity, planned matches, structured inputs, every outcome. Which artifact is normative for these forms is section 4, AD-21 (OWNER). Recommendation: an evaluator over the source language `moriarty-sherlock-nominal/1` (`crates/mori-core/src/sherlock/nominal_source.rs`), written from the language definition and independent of the owner machine's rule code, leaving the dedicated `sherlock-k-a/1` grammar of `reference.rs` unchanged.
- R2. The event map, explicit and total on the corpus: helper events to owner events; owner events to source steps; the free terminal publication; `n_steps` as the count of enabled owner events, publication included and padding excluded (SRC-003, `O/requirements.md:28`). A helper-event count is never equated with a source-step count without this map.
- R3. A total projection: owner-machine state at an owner-event boundary to reference state, for every form of R1. `ProjectionGap::UnsupportedGeneric` and `UnmappedRule` do not occur on the corpus.
- R4. A coverage report per run: rules fired, forms projected, gaps. A gap fails the gate; it is never skipped and the language is never cut to the supported subset.

#### Invariants
Guaranteed: for every corpus case, the projected owner-machine run and the R1 evaluation agree on the outcome, the value digest of the result and, through the event map, on the step at which each compared quantity is taken; the coverage report lists zero gaps before the harness is used as an acceptance gate.
Assumed: the language definition that R1 is written from (AD-21); `sherlock-value/1` for comparing structured values.
Not a proof: this kit is trusted host diagnostics, as `reference_projection.rs:5` says of itself. The harness finds divergences; soundness rests on the constraint tests, the models and the Lean obligations.

#### Row budget
None (host). Runtime of the harness is measured after the first complete leaf (section 3.2).

#### Verification
- X13 first: a census of `UnsupportedGeneric`, `UnmappedRule` and the other gap kinds over the corpus, which sizes the work.
- Tests: `bridge_corpus_zero_gaps`; `bridge_wide_constructor_across_leaf` (more than 8 fields); `bridge_structured_input`; `bridge_planned_match`; `bridge_every_outcome`; `bridge_step_count_matches_owner_events`; `bridge_rules_fired_cover_inventory` (all 120 rules, with a constructed case for each of the 18 rules the X1 corpus did not fire, or a recorded reason why a source program cannot reach it).
- Negative controls: a seeded divergence in the owner machine (a wrong field order in a constructor, a dropped publication step) must be reported by the harness.

#### Lean obligations
None of its own. The step function that `leaf_refines_step` and O8 refer to is the owner-event step; this kit's event map is the host-side statement of the same correspondence and is checked by differential tests, not in Lean.

#### Code skeleton
Files (proposed here): `crates/mori-core/src/sherlock/reference_full.rs` (R1), `crates/mori-core/src/sherlock/helper_machine/reference_projection.rs` (extend, R3), `crates/mori-core/src/sherlock/helper_machine/event_map.rs` (R2), `crates/mori-chip/tests/support/coverage.rs` (R4).
```rust
/// Evaluate a full-language program on an input; outcome, result value and source-step trace.
pub fn eval_full(program: &ExtProgram, input: Option<&ExtValue>, fuel: u128, stack_limit: usize) -> FullTrace;
/// Owner events of one run mapped to source steps; total or an explicit gap.
pub fn map_events(run: &OwnerRun) -> Result<EventMap, ProjectionGap>;
/// Fails if any gap kind occurs.
pub fn coverage(run: &OwnerRun, map: &EventMap) -> CoverageReport;
```

#### Agent reads / must not need to read
Reads: this kit; `reference.rs`, `reference_types.rs`, `reference_projection.rs`, `nominal_source.rs`, the owner runtime's public step interface, SRC-003, section 3.2. Must not need: any circuit code, `crates/mori-recursion`.

#### Dependencies
None on circuit kits. It is a prerequisite of leaf integration (group L) and of every CORE rule family that handles ADT rules. AD-21 and X13 first.

---

### Kit: HOST CHECKPOINT STORE

B2 finding 13. CONTINUATION referred to a persistence interface of `MemBackend` that the code does not have.

#### Purpose
Hold, outside the circuit, every witness resource a prover needs to continue a run, write it as a versioned checkpoint, and restore it only after checking it against the committed state. It is the host counterpart of the MEM, MEMORY BACKEND, STACKS and BOUNDARY COMMITMENT kits.

#### Interface
`RunStore` (live, in memory or on disk) supplies the witness generators:
- the memory append log: every record `[0, len)` in order, from which the native `DenseModel` (`chip/dense.rs:161-246`) gives segment digests, the tail, cold-read siblings and line witnesses;
- the two stacks as full lists (the preimages of `frames_below` and `tasks_below`, which the circuit needs for every future pop);
- the register state (every header and body word of the schema);
- the program image (for ROM line witnesses) and the input preimage `(x, fuel, stack_limit)`.

`CheckpointV1`, codec `sherlock-ckpt/1` (a versioned, length-prefixed, canonical byte format; an unknown version is refused):

| Section | Content |
|---|---|
| header | codec version; key-set manifest hash (SRC-008); profile and schema id; `TOP`, `LeafShape` |
| identity | `prog_d`, `input_d`, `h` (`1 <= h < 2^64`), `post_d` at `h` |
| state | the header words and all body words at `h`, explicit |
| memory | `len`, the append log (all records, so the raw tail records are included), or a reference to a content-addressed log file with its digest |
| stacks | the frame list and the task list, bottom to top, with the cached tops |
| program and input | the program image or its content address; the input preimage |
| proofs | the tier 1 carry stack: every proven SEG `(rank, h_lo, h_hi, statement, accumulator, proof)` that covers `[0, h)`; the SRC-019 bundle prefix hash |
| digest | the record digest under the domain `SHK_CKPT`, disjoint from `SHK_RUN` and the SET domains |

Restore validation, in this order; any failure is a host integrity failure and stops before proving:
1. codec version and manifest equal to the expected key set;
2. every SEG proof of the carry stack verifies (SRC-010), the intervals are adjacent and cover `[0, h)`, and the last `post_d` equals the record's `post_d`;
3. the memory log rebuilds a `DenseModel` whose `(len, root, tail_d)` equal the body words;
4. each stack list rebuilds the chain head and count of the body words;
5. `body_d` and `state_d` recomputed from the state section equal `post_d` (`state_digest_native`);
6. the program image hashes to the `prog_root` inside `prog_d`, and the input preimage to `input_d`.

After validation every later witness is derived from data that reproduces the committed digests, so a changed record is found here whether or not a later leaf reads it.

#### Invariants
Guaranteed: a restored `RunStore` reproduces `post_d` exactly; a checkpoint with any changed, missing or extra section is refused at restore; a checkpoint is never a claim (its digest domain is disjoint, and it holds a Running state); writing and restoring execute no machine event.
Assumed: collision resistance of the hashes used by the validation; the verifier of SRC-010.
Not a soundness dependency: an attacker who controls the store can stop the prover and cannot make it prove a false statement, because every leaf's reads are constrained against `pre_d`.

#### Row budget
None (host). Size: the log is linear in the memory of the run; no bound is introduced. Restore time is linear in `len` and the stack depths (ESTIMATE); measured with the GP-001 acceptance run.

#### Verification
- Tests: `ckpt_roundtrip_allocating_run`, `ckpt_resume_nonempty_tail`, `ckpt_resume_nested_stacks` (pops below the checkpoint depth succeed after resume), `ckpt_carry_stack_of_several_segs`, `ckpt_codec_canonical` (one byte string per record).
- Host refusals (not constraint mutants): `ckpt_tampered_unused_cell`, `ckpt_tampered_tail_record`, `ckpt_stack_preimage_changed`, `ckpt_register_changed`, `ckpt_missing_stack_section`, `ckpt_wrong_manifest`, `ckpt_unknown_version`, `ckpt_prefix_proof_invalid`, `ckpt_trailing_bytes`.
- Constraint check with the validation switched off: CONTINUATION's `cont_tampered_read_cell_unvalidated`.
- Quint: `continuation.qnt` models the store as data that may be corrupted; invariant `CorruptStoreNeverYieldsAcceptedWrongClaim`.

#### Lean obligations
None of its own: restore correctness is O28 and O29 (the state is determined by its digest) applied by the host. CONT-L1 does not depend on the store.

#### Code skeleton
Files (new): `crates/mori-recursion/src/checkpoint.rs`, `crates/mori-recursion/src/run_store.rs`.
```rust
pub trait RunStore {
    fn records(&self) -> &dyn RecordLog;                 // the append log [0, len)
    fn dense(&self) -> &DenseModel;                      // siblings, tail, line witnesses
    fn frames(&self) -> &[FrameRow]; fn tasks(&self) -> &[TaskRow];
    fn state(&self) -> &NativeState;                     // header and body words
    fn program(&self) -> &ProgramImage; fn input(&self) -> &InputNative;
}
pub struct CheckpointV1 { /* the sections of the table */ }
impl CheckpointV1 {
    pub fn write(run: &dyn RunStore, carry: &[ProvenSeg], manifest: &[u8; 32]) -> Result<Self, CkptError>;
    pub fn encode(&self) -> Vec<u8>;  pub fn decode(bytes: &[u8]) -> Result<Self, CkptError>;
    /// Digest under SHK_CKPT; disjoint from SHK_RUN and the SET domains.
    pub fn record_digest(&self) -> F;
    /// The six validation steps; returns the rebuilt store or CkptError::Integrity(which).
    pub fn validate(&self, expected_manifest: &[u8; 32]) -> Result<Box<dyn RunStore>, CkptError>;
}
```

#### Agent reads / must not need to read
Reads: this kit; `chip/dense.rs:161-246` (`DenseModel`); `state_digest_native` and the frozen `BoundarySchema`; kit STACKS (chain definition); SRC-008, SRC-010, SRC-019; kit CONTINUATION. Must not need: gates of any chip, verifier gadget internals, rule bodies.

#### Dependencies
BOUNDARY COMMITMENT (schema, native digest), MEMORY BACKEND (native model), STACKS (native chain), INIT (native Init and openings). CONTINUATION uses it.

---

### Kit: LEDGER CONTRACT  (interface version 1)

A Compact contract outside the recursion crates. It owns the contract-state properties that no circuit kit can enforce: at-most-once settlement of a claim, root acceptance of collections, and claim opening in bounded steps.

#### Purpose
Accept run statements and collection statements on chain, and settle each claim at most once in one shared ledger state, whichever entry point inserts it. Open a claim from a verified collection root in steps that fit the per-call operation cap, so a DAG of any depth (GP-004) can be settled.

#### Interface
One contract, one shared state. No bus tuples and no circuit cells; the inputs are the public values of the L AND WRAP entries and ledger call data.
- State:
  - `runs: Map<claim_pub, outcome_pub>`: settled run claims, keyed by `claim_pub` (section 1.11, FP-4); no `n_steps` (FP-6).
  - `foreign: Map<fclaim_d, ()>`: settled foreign claims.
  - `verified_roots: Set<Field>`: collection roots and sub-roots reached from a verified root.
  - the registry root `reg_root`, with the history and revocation rule of N-8 (open).
- `acceptRun(C_P, input_pub, outcome_pub)`: verifies the wrap proof whose effect words carry the three values (section 1.11, FP-3), computes `claim_pub = transientHash([C_P, input_pub, outcome_pub])` and refuses a `claim_pub` already in `runs`; else inserts it. Plain `prog_d` and `n_steps` are not call data (ZK-002, FP-6; amendments of SRC-007 and LDG-001 are owner item F-12; the public program identity `C_P` is N-11, OWNER).
- `acceptCollection(reg_root, set_root, shape_d)`: verifies the wrap proof of the collection statement, checks `reg_root` against the registry state, and inserts `set_root` into `verified_roots`. It makes no per-claim write and no write keyed by `shape_d`.
- `descend(root, openings)`: requires `root` in `verified_roots`; checks up to `K_open` one-hash openings of `JOIN` or `DEP` nodes of the SET grammar (GP-2 hook) along one path, and inserts the reached sub-root into `verified_roots`. `K_open` is a constant sized so that one call stays within `MAX_CONTRACT_OPERATIONS = 256` (`O/tasks.md:44`); a deeper path takes several calls.
- `openClaim(sub_root, head)`: requires `sub_root` in `verified_roots`; checks one head hash of the `LEAF`, `DEP` or `FOREIGN` form; inserts the claim into `runs`, or `fclaim_d` into `foreign` (the contract-side form of these openings is open: section 1.11, FP-3, fact 3). It is idempotent: a key already present is not written again and the call changes nothing. It refuses an `ADM` or `VIEW` head (seal premises, never claims) and a `JOIN` root (no head claim).
- Domains: the SET domain words of the GP-2 hook, `SHK_RUN` and `SHK_CLAIM`, computed with the same pinned Poseidon as the recursion.

#### Invariants
Guaranteed:
- A `claim_d` is in `runs` at most once, whichever entry point inserted it (test `ledger_double_settle_two_entries`; Quint invariant `ClaimSettledOnceAcrossEntries`).
- A diamond-shared claim with many paths in `set_root` settles at most once (Quint invariant `DiamondAncestorOpenedAtMostOnce`, moved from L AND WRAP; Lean LDG-L1).
- Every claim that is a member of a verified `set_root` under the SET grammar has an opening path of `descend` and `openClaim` calls, each within the operation cap (Quint invariant `EveryJoinedClaimOpens`, moved from L AND WRAP; test `ledger_open_depth_beyond_one_call`).
- Collection acceptance has no per-claim side effect keyed by `set_root` or `shape_d` (test `ledger_collection_writes_no_claim`).
- A sub-root enters `verified_roots` only from a verified root by checked openings (test `ledger_descend_unverified_root`).
- A stale or foreign registry root is refused (Quint invariant `StaleOrForeignRegRootRefused`; the history rule is N-8).
Assumed: the L AND WRAP proofs are sound for their public values (DEC-L1); the SET grammar hashes are injective (GP2-L2); the Compact runtime enforces the per-call operation cap.

#### Row budget
No circuit rows. On chain: one Poseidon per path level opened plus one head hash per claim (ESTIMATE), paid only by the caller that needs a claim settled. `K_open` is set by the recapture of the operation cost per Poseidon against the 256-operation cap (ESTIMATE until the recapture of task F.10).

#### Verification
Quint: new `models/sherlock/quint/ledgerContract.qnt`: the two entry points over one `runs` map, `descend` and `openClaim`. Invariants: `ClaimSettledOnceAcrossEntries`, `DiamondAncestorOpenedAtMostOnce`, `EveryJoinedClaimOpens`, `StaleOrForeignRegRootRefused`.
Positives (local contract tests): `ledger_accept_run_then_open_same_claim_is_noop`, `ledger_open_from_three_run_collection` (GP-002 acceptance b), `ledger_open_depth_beyond_one_call`, `ledger_foreign_claim_open`.
Named refusals (contract tests with a weakened control): `ledger_double_settle_two_entries` (control: two maps), `ledger_descend_unverified_root`, `ledger_open_join_as_claim`, `ledger_open_premise_as_claim` (an `ADM` or `VIEW` head offered to `openClaim`), `ledger_collection_writes_no_claim` (control: a per-claim write at `acceptCollection`), `ledger_descend_over_op_cap` (a call with more than `K_open` openings).

#### Lean obligations
- LDG-L1 (was DEC-L2). `claim_d` settles at most once in the contract state even when it has many paths in `set_root`, and whichever entry point inserts it. File: `clean/fork/Clean/Sherlock/ClaimDag.lean`. Premises: hash injectivity over the SET domains (hypothesis). Contract: the replay rule, GP-002 "accepted once on-chain", GP-004. Status: open.

#### Code skeleton
Files: `tools/sherlock-sdk/sherlock-ledger.compact` (new; one contract), beside the existing `tools/sherlock-sdk/sherlock-run.compact` (the C6 per-program milestone contract, kept until N-11 is answered).
```
// Compact sketch; not compiled.
ledger runs: Map<Field, Field>;             // claim_pub -> outcome_pub (section 1.11, FP-4, FP-6)
ledger foreign: Map<Field, Boolean>;        // fclaim_d
ledger verified_roots: Set<Field>;
ledger reg_root: Field;                     // history per N-8
// Requires: the wrap proof verifies with the three values in its effect words. Ensures: claim_pub inserted once, or refused.
export circuit acceptRun(c_p: Field, input_pub: Field, outcome_pub: Field): [];   // section 1.11, FP-3, FP-4, FP-6
// Requires: the wrap proof of the collection statement verifies; reg_root is current. Ensures: set_root in verified_roots; no claim write.
export circuit acceptCollection(reg_root: Field, set_root: Field, shape_d: Field): [];
// Requires: root in verified_roots; at most K_open JOIN or DEP openings. Ensures: the reached sub-root in verified_roots.
export circuit descend(root: Field, openings: Vector<K_open, Opening>): [];
// Requires: sub_root in verified_roots; a LEAF, DEP or FOREIGN head. Ensures: idempotent insert into runs or foreign.
export circuit openClaim(sub_root: Field, head: HeadPreimage): [];
```

#### Agent reads / must not need to read
Reads: this kit; the L AND WRAP public-value layouts (run entry and collection entry); the SET grammar of the GP-2 hook (forms, domains, head claims); SRC-007, SRC-020 and GP-002 rows; `tools/sherlock-sdk/sherlock-run.compact` and the captured `acceptRun` template (`O/tasks.md:44`).
Must not need: any circuit kit internals, the node relation, CORE rules, memory.

#### Dependencies
L AND WRAP (public-value layouts; interface-only edge), GP-2 hook (SET grammar), GP-3 hook (`reg_root` and foreign claims). Owner decisions N-11 (contract surface and the sealed `program_d` question of SRC-007) and N-8 (registry root history).

#### Owned files
`tools/sherlock-sdk/sherlock-ledger.compact`; tests `tools/sherlock-sdk/sherlock-ledger.test.mjs` (paths proposed here).

#### Completion evidence
The local contract tests and named refusals above against the compiled contract; the Quint invariants above with a verdict file; LDG-L1 proved or listed open; the measured operation count per Poseidon and the chosen `K_open`; the remaining assumption that the deciders are sound (DEC-L1, open).

### Kit: ZERO KNOWLEDGE

#### Purpose
Keep the witness, the program and the run shape private end to end. The co-processor is zero knowledge by placement and by one blinded proof: every STARK layer runs on the user's machine, and the wrap is the only proof that leaves it (`PR` section 5).

#### Interface
No bus tuples. Rules that bind the other kits:
- Placement: leaves, nodes, `L` and the wrap prover run on the user's machine. Local devices and GPUs on hosts the user owns count as the user's machine (OD-9). STARK proofs and prover checkpoints are secrets like the witness.
- The wrap: blinded midnight-proofs PLONK with fresh CSPRNG blinders; private witness as in kit L AND WRAP; public inputs the call vector only. Nothing derived from the transcript of `L` becomes a public input or a constant of the wrap.
- Padding: the log is padded with `NOP` to its fixed capacity, so the count of used entries and the deferred-or-emulated split stay hidden; the call vector is padded with constrained zeros to `L_TX`.
- The anchor lane carries `C_P` or a lock value `A_j`, both salted digests. Plain `prog_d` never reaches the ledger.
- `X`, `mode`, `j`, `k` and `run_tag` are public values of `L` only; the wrap takes them as private witness.
- Statement digests: the salt of `input_d` comes from the user's CSPRNG (8 BabyBear elements in the STARK, a full Fr element in public). The hiding commitment in `outcome_d` and the salted `prog_d` of a private program are inputs of the small-field digest `d2` inside `PoseidonFr(tag, pack(d2))`; the salts are private words of `DIGEST` entries and private witness of the wrap. The shard plan binds to the unsalted program digest inside the STARK layers only.
- Shape: per-AIR heights and the set of AIRs present stay inside the user's trust boundary. `n_steps` is never public (section 1.11, FP-6); leaf counts follow the public padding `N_pub` of the plan. A chunked call's chunk count is a function of the public transcript length; where a secret branch changes the length, filler chunks pad it to the entry bound.
- Never allowed: a helper that proves a STARK layer or `L`; a remote unmasked MSM, NTT, quotient or Merkle hash; the full sponge state on a helper; reuse of a blinding slot; a helper-made slot.
- Aggregation: no aggregator receives STARK proofs. One wrap settles one `ContractCall`. A user's own batch (OD-12, one batch digest per `acceptRun`, opt-in) keeps every witness local.
- Prover daemon: the resident Sherlock prover (OD-13) listens on a local socket and holds a pool of one-use blinding slots.

#### Invariants
Guaranteed: the published view of a call is the call vector and one blinded wrap proof (`wrap_view_only_claim`, premise A7 `wrapZK`); the used log count, the deferred-or-emulated split, the shard plan, the AIR set and heights, and plain `prog_d` are not in that view; two wraps of the same statement with fresh blinders are indistinguishable to the ledger (EQ-ZK).
Assumed: the blinded PLONK-KZG wrap is zero knowledge (A7); the user's CSPRNG is sound; the user's machine is trusted.

#### Row budget
No added wrap rows beyond the constrained zero padding of the call vector and the `NOP` padding of the log, which the wrap row figures already count. Masking of intermediate STARK layers is not needed while they stay local, and the design keeps them local.

#### Verification
EQ-ZK at frontend M2b: two wraps of one statement with fresh blinders, compared byte for byte (different) and both accepted. A seeded-proof guard: a build without the test feature refuses a seeded blinder. A daemon test: a reused blinding slot is refused. Byte-identity tests of the GPU wrap use a test-only seed patch active only under `GW_DET=1`. Quint: the "what lives where" model of the animation's beat 8.9 as a property: no STARK proof, checkpoint or plan leaves the user's machine.

#### Lean obligations
- ZK-L1. `wrap_view_only_claim` (built at `bdb92973a`, premise A7): the ledger's view of a call is a function of the call vector alone.
- ZK-L2. Padding: the encoding of a log in `pv` is unique and does not depend on the used count beyond what the call vector shows (the `NOP` and zero padding rules of section 1.8). Status: open.

#### Code skeleton
Files (proposed): `crates/sherlock-prover/src/daemon.rs`, `slots.rs`.
```rust
/// One-use blinding slot; consumed by exactly one wrap_prove. Ensures: a second use is refused.
pub struct BlindSlot { /* fresh CSPRNG blinders, never serialized off the machine */ }
pub fn take_slot(pool: &mut SlotPool) -> Result<BlindSlot, DaemonError>;
/// Requires: the peer is a local socket of the same user. Ensures: no STARK proof or checkpoint leaves the process.
pub fn serve_local(sock: &UnixListener, pool: SlotPool) -> Result<(), DaemonError>;
```

#### Agent reads / must not need to read
Reads: this kit; kit L AND WRAP (public inputs and private witness); `PR` section 5; the salt rules of VALUE DIGEST and INIT for `input_d` and `outcome_d`.
Must not need: CORE rules, memory internals, GPU kernels.

#### Dependencies
L AND WRAP (blinding, public inputs), COMPACT FRONTEND (call-vector padding, filler chunks), VALUE DIGEST and INIT (salted digests), HOST CHECKPOINT STORE (checkpoints stay local).

---

### Kit: COMPACT FRONTEND

#### Purpose
Compile a Compact contract to the Sherlock machine so that its entries run as Sherlock programs under the one universal wrap key, with an effect vector equal to the native public-input vector word for word. It also owns the E-1 entry-label binding (placement V), atomic chunked calls (modes 1F and 1G) and the anchor prologue of native-routed entries. These are the first build items (`PR` section 7, with the native prologue of section 9).

#### Interface
- Tap: the frontend reads `Lflattened` after `optimize-circuit2` (`CC/compiler/passes.ss:138`), the exact input of `reduce-to-zkir`. The manifest and the TypeScript runtime come from `analyzed-ir` unchanged. One new analysis pass, `add-sherlock-anchor`, follows `determine-ledger-paths`. The frontend replaces `reduce-to-zkir`, the `zkir-v3 compile-many` key step and the per-circuit key fingerprint.
- Commands: `sherlock compile` (images, `abi.json`), `sherlock plan` (witness-free plan, per-program keys, `prog_d`), `sherlock simulate` (reference machine, no proof), `sherlock deploy` (anchor cell `C_P`, every entry holds the wrap key), `sherlock prove` (STARK layers on the GPU), `sherlock call` (host order, ending in submit).
- `abi.json` per entry: the straight-line step count `N_e`, the effect words `L_e`, the tape length, the counts per builtin (FMUL, FADD, FINV, sponge permutations with input lengths, SHA blocks, ECADD, ECMUL, ECMULG).
- Lowering: Stage 1 in straight-line mode (`R_SL_*`, section 1.7). Field arithmetic on FIELD-EMU (OD-14: Compact `Field` stays emulated); `transientHash` as `SPONGE(n)` in the log or POSEIDON-FR-EMU inline; `persistentHash` on the OpenVM `sha2` chip; `ecMul` and `ecMulGenerator` as log entries while `E` slots last, otherwise JUBJUB-EMU. A native call with no kit is a compile error that names the kit.
- Tape: `Cons(salt, Cons(selector, args ++ comm_rand ++ witness, read and callee results in program order))`, static in length and layout per entry. Ledger reads are `popeq` values from the tape, which the ledger replays and refuses when stale. Writes are effect words at static indices; a guarded-off impact writes zeros, and the host inserts the same `Noop` merges as the native transaction builder.
- Call vector: the layout of section 1.8. An entry with `L_e > L_TX - A` is a compile error that names the entry and its `L_e` until the frontend's chunked mode exists; then it runs as a chunked call.
- E-1, placement V (OD-5): the ledger computes `binding_input = le31(SHA256(m))` from public transaction bytes `m`; the label window starts at byte offset 58 for every call. `bind_label(e)` reads `m` from the private tape, pins `m[0 .. 26) = "midnight:binding-input[v1]"` and `m[58 .. 58 + |enc(e)|) = enc(e)`, computes SHA-256 on the `sha2` chip and writes `bind = le31(digest)` into the outcome; the wrap requires lane 0 equal to `bind`. V+: `bind_label` reads the domain bytes, their length and the label offset from the tape, uses exactly those for the window checks, and writes `layout_d = P2(DOM_LAYOUT, |domain|, domain, offset)` as one statement word, which the wrap pins to a key constant. A ledger layout change then replaces one wrap key constant and no `prog_d`.
- Chunked calls: one logical call runs as `k` contract calls in one intent of one transaction on the unmodified ledger, with a forward claim chain (chunk `j` claims chunk `j+1`) and an anchor lock. Mode 1F (default): all chunks fallible-only, in one segment; a middle failure leaves contract state unchanged and the transaction is included with its fee paid. Mode 1G: all chunks guaranteed-only, for a callee of a guaranteed caller; a failure makes the transaction invalid. Mixed chains are refused. Rules: R-phase (one phase for all chunks), R-cut (cut only where the native run has stack depth exactly 3, at a boundary the run certifies; the slice shard checks the cut), R-ctx (hoist every read of context slot 0 and effects slot 1 into a prologue in chunk 1), R-outcome (1F all or none applied with fee paid; 1G all applied or invalid).
- Anchor values (OD-16): `A_0 = A_k = C_P`; `A_j = PoseidonFr_4(SHK_LOCK, pack(P2(DOM_LOCK, r_P, prog_d, run_tag, j)))` for `1 <= j < k`, with `run_tag` a fresh salted value per run. The anchor statement per case:

| Case | `mode` | `a_in` | `a_out` | Anchor write |
|---|---|---|---|---|
| Ordinary call | ORD | `C_P` | `C_P` | none; user code cannot address cell `(0)` |
| Chunk 1 of `k` | CHUNK | `C_P` | `A_1` | `ins anchor := a_out` in the effect region |
| Chunk `j`, `1 < j < k` | CHUNK | `A_(j-1)` | `A_j` | same |
| Chunk `k` | CHUNK | `A_(k-1)` | `C_P` | same |

- Hybrid contracts (OD-8): small entries route native under their own keys at compile time. The pass gives a hybrid contract a second cell `anchor_home` beside the anchor; deploy writes `C_P` into both; no entry can write `anchor_home`. A native entry starts with two uncached reads, `popeq anchor` and `popeq anchor_home`, and its circuit asserts that the two values are equal, so during a run (`anchor = A_j`, `anchor_home = C_P`) no valid native call exists. The prologue needs no deploy-time constant in the native circuit. Sherlock entries keep the one-cell anchor prefix: their proof fixes `a_in`.

#### Invariants
Guaranteed:
- `effect_vector(preimage)` equals the native public-input vector word for word for every entry (golden vectors).
- `N_e` is the same for any two tapes of one entry.
- The anchor cell holds `C_P` or a salted lock value, never plain `prog_d`; the last chunk writes `C_P` back.
- Atomicity of a chunked call: no dropped suffix (`RealCallsSubsetCheckFailure`), no dropped prefix (the `popeq` of `A_(j-1)` fails), no interleaving by other intents, no interleaving inside the intent (the anchor lock, with the prologue in every Sherlock entry and in every native-routed entry), replay bound through `binding_input`, fees checked for the whole transaction before application.
- The claim address, the claim `ep_hash`, `comm_(j+1)`, `pi[1] = comm_j`, the anchor statement and the slice bounds are fixed by the proof. The source of the claim address and `ep_hash` per chunk under placement V is an open item of the chunk specification.
- E-1: if the real label `e'` differs from `e`, then `m' != m`, and `(m, m')` is a collision of SHA-256 truncated to 248 bits, about 2^124 hash calls.
Assumed: the Compact compiler's `Lflattened` is the input of `reduce-to-zkir`; the ledger applies a segment's fallible actions as one unit.

#### Row budget
Leaf: `spend` is 104 to 550 straight-line steps, ESTIMATE; FMUL is one row of 194 main columns, MEASURED. E-1 placement V: 3 to 32 SHA blocks, 51 to 544 leaf rows, under 0.1 s, and 0 wrap rows, ESTIMATE; V+ adds at most 120 wrap rows. A midnight-proofs prototype of the bind statement costs 5,749 rows for 3 SHA blocks and 61,726 for 32, about 1,928 rows per block, MEASURED; that is the placement S figure, not a leaf row count. `L_TX` = 1,024: the largest of 27 checked entries needs 474 words, STATIC COUNT; M7 measures the binding rows per word on G4. Chunks: about 30 words of overhead per chunk, about 990 effect words per chunk, 5 to 30 chunks per transaction with the code defaults, about `k` times the fee of one call; proving is one run plus `k` times (slice shard, binding, `L`, wrap), ESTIMATE. Chunked calls are the capability for one logical call above `L_TX`, not a call-time lever. A rebuild under placement V re-proves the whole run, about 20 s on the full GPU path; every byte of `m` is fixed when the call intent is built, so the triggers left are TTL expiry and a cost-model change, both avoided by a TTL of at least 10 times the prove time.

#### Verification
Milestones: M0 golden vectors (effect vector equal to native word for word for every example entry; four mutants caught); M1 the reference machine over the `R_SL_*` family (100 random preimages per entry equal native; `N_e` equal for two tapes; `spend` at most 550 steps); M2a a verified leaf and root on the GPU under the stock OpenVM verifier, with byte-equal internal-recursive keys across two chip sets; M2b one wrap proof that ledger 9 accepts with `2 + L_TX` public inputs, negative controls and EQ-ZK; M3 deploy and call on the local devnet with refusals for a tampered bit, another `C_P` and a stale read; M4 acceptance sets 1 and 2 end to end with the forced-overflow cases (`R` reserved); M5 Jubjub contracts, one GP-2 consumption, one GP-3 aggregation. E-1 tests T0 (rebuild `m` from public transaction bytes and compare with `public_inputs[0]`; it gates the design) to T6. MEASURED on a local ledger 9.1.0.1 devnet (`PR` section 7.2): T0 passes on the XC-9 call and three devnet calls, with the window at offset 58; T1 passes, so E-1 is real on the current ledger; the bind prototype refuses the attack in MockProver and in a real prove and verify at k = 13. Open: the remaining T2 mutants and the `label_d` link, T3 to T6, T0 on calls with a fallible transcript or non-empty effects, and the window position under a variable-length SHA-256 (a length-selected window or one fixed-length bind per length class). Chunk tests T1 (ledger level, 13 cases, no proofs), T2 (XC-9 devnet, `chunky.compact`) with the native-routed controls T2-N and T2-N0, T3 with universal-key chunks after M5, including chunk 1, chunk `j` and chunk `k` runs whose discretionary operations fill the log. Chunk results: PENDING (`FD/EXPERIMENTS.md`).

#### Lean obligations
- CF-L1. Effect equivalence: for every entry and preimage, the Sherlock run's effect vector equals the native public-input vector. File (proposed): `clean/fork/Clean/Sherlock/CompactEffect.lean`. Status: open.
- CF-L2. Anchor atomicity: under the anchor table and the prologue in every entry, no call of the contract other than chunk `j+1` of the same run is valid while the anchor holds `A_j`. File (proposed): `ClaimDag.lean`. Status: open.

#### Code skeleton
Files (proposed): `tools/sherlock-compact/` (frontend passes), `crates/mori-core/src/sherlock/sl/` (the `R_SL_*` reference semantics), `bind_label` in the Sherlock standard library.
```rust
/// Requires: Lflattened of one contract. Ensures: per-entry images, abi.json, the effect template with holes.
pub fn compile(lf: &Lflattened, out: &Path) -> Result<Abi, CompileError>;
/// Ensures: R reserved first; each discretionary Fr op to the log or tier E; plan_d and key_root; prog_d.
pub fn plan(abi: &Abi, images: &Images, caps: &LogCaps) -> Result<Plan, PlanError>;
/// E-1 V+: windows from the tape's layout parameters; returns (bind, layout_d).
pub fn bind_label(m: &TapeBytes, layout: &LayoutParams, e: &EntryLabel) -> (u256, BbDigest);
```

#### Agent reads / must not need to read
Reads: this kit; sections 1.7 and 1.8; `CO/FRONTEND.md`, `CO/E1-entry-label.md`, `CO/ATOMIC-CHUNKS.md`; the Compact compiler passes named above.
Must not need: the wrap circuit, GPU kernels, the recursion verifier.

#### Dependencies
CORE and DEC (the `R_SL_*` family under `machine_version`), the chip AIRs of the shard plan, L AND WRAP (call-vector lanes, anchor checks, `layout_d` constant), ZERO KNOWLEDGE (padding and filler chunks), LEDGER CONTRACT (the `acceptRun` entry for general runs).

---

### Lean obligations: consolidated table and tally

One row per obligation. An obligation that several kits use is counted once. "Proved in C3" means the theorem is named in `FD/EXPERIMENTS-C3.md` (52 theorems in `lean/PartitionIndependence.lean` and `lean/ModularComposition.lean`, no `sorry`, no `axiom`). Every other row is open; every file marked new does not exist.

| Id | Kit | Obligation | File | Status |
|---|---|---|---|---|
| P1 | BUS | a balanced bus is lookup-sound (`balanced_lookup`); not a property of the landed membership bus, used only for Profile B | `ModularComposition.lean:55` | proved in C3 |
| P2 | BUS | every bus tuple of an accepted leaf is valid if every module meets its contract, under `LookupSound`; bus tuples only, no copied port (`provided_valid`, `requested_valid`) | `ModularComposition.lean:68-88` | proved in C3 |
| P3 | CORE | composition: `leaf_refines`, `leaf_refines_balanced` | `ModularComposition.lean:91` | proved in C3 |
| P4 | CORE | an accepted leaf is a `Run` of its rows if the rows follow the step (`leaf_refines_step`, `follows_run`, `fun_det`) | `ModularComposition.lean:131` | proved in C3 |
| P5 | ALU | non-vacuous CORE + ALU instance (`toy_contracts`, `toy_refines`, `toy_needs_bus`) | `ModularComposition.lean:157` | proved in C3 |
| P6 | STACKS | the chain hash of a whole list determines the list (`chainHash_inj`); a lemma, not the cached-top protocol | `PartitionIndependence.lean:251` | proved in C3 |
| P7 | MEM, STACKS | partition independence, per-cell rule, for an effect with one `StackOp` per stack (`partition_independent`, `leaf_tail_commutes`) | `PartitionIndependence.lean:404` | proved in C3 |
| P8 | MEM | `len` advances by the real appends (`appendCells_len`, `canon_len`) | `PartitionIndependence.lean` | proved in C3 |
| P9 | MEM, BOUNDARY | two valid cuts give the same state and digest (`cuts_agree`), under `FitsLeaf` for both cuts (at most 8 appends per leaf) | `PartitionIndependence.lean:423` | proved in C3 |
| P10 | MEM, BOUNDARY | `det` discharged; unique result (`pstep_det`, `leaf_run`, `premises_det`, `result_unique_tail`) | `PartitionIndependence.lean:471,480` | proved in C3 |
| P11 | MEM | D3 is partition dependent and admits no deterministic `Run` (`d3_partition_dependent`, `d3_no_det_run`, `tail_det_run`) | `PartitionIndependence.lean:511,537` | proved in C3 |
| P12 | MEMORY BACKEND | canonical tail (`Mem.wf_unique`) | `PartitionIndependence.lean:160` | proved in C3 |
| P13 | MEMORY BACKEND | flush iff a segment boundary is crossed (`pushCell_flush_iff`, `leafFlush_iff`) | `PartitionIndependence.lean` | proved in C3 |
| P14 | MEMORY BACKEND | one-flush circuit rule: `leafFlush_eq`, `leaf_flush_commutes`, `partition_independent_flush`, `ones_fit` (under `FitsLeaf`: at most 8 appends in the whole leaf) | `PartitionIndependence.lean:198,413` | proved in C3 |
| O1 | BUS | the Sherlock tuple type, levels and chip registry instantiate `Module` and `Contract` | new `BusContract.lean` | open |
| O2 | BUS | the lookup argument gives `LookupSound` (membership) up to the lookup error | not stated (no field theory in the library) | open (premise) |
| O3 | RANGE | RANGE as a `Module` instance | new `AluRange.lean` | open |
| O4 | ALU | ALU as a `Module` instance, unique `(z, flag)`, no field wrap in the multiply | new `AluRange.lean` | open |
| O5 | DEC | packing of bounded control parts is injective | new `DecDispatch.lean` | open |
| O6 | DEC | DEC as a `Module` instance over the frozen inventory | new `DecDispatch.lean` | open |
| O7 | HASH | the hash premises as one structure of hypotheses | new `HashPremises.lean` | open (premise statement) |
| O8 | CORE | CORE's concrete contract over the full rule set | new `CoreRefinement.lean` | open |
| O9 | CORE, NODE | `Premises.leaf_sound` derived from the composition theorem and the concrete contracts (the helper's LEAF-REF row) | new `LeafSound.lean` | open |
| O10 | CORE | independence from incoming scratch (non-interference of a slot under definite assignment) | new `ScratchIndependence.lean` | open |
| O11 | STACKS | the cached-top protocol, with the compound step, refines the list function; `count = 1` implies `below = EMPTY` | new `HashStacks.lean` | open |
| O12 | MEM | address bound over physical cells with the genesis length: `len_0 + a e 2^64 <= 2^(TOP+3) - 1` | extend `PartitionIndependence.lean` | open |
| O13 | MEM | MEM window as a `Module` instance | new `MemContract.lean` | open |
| O14 | MEM, MEMORY BACKEND | partition independence under the state-dependent capacity `FitsWindow W` (`r_pre + appends <= 8 W + 7`), at `W = 1` and for W flush units | extend `PartitionIndependence.lean` | open |
| O15 | MEMORY BACKEND | single-step fit from the per-rule table (at most `8 W` physical cells per step), so the one-step cut is always valid | extend `PartitionIndependence.lean` | open |
| O16 | MEMORY BACKEND | read soundness of a cold read | new `MemContract.lean` | open |
| O17 | PAGE/ROM | the physical state does not depend on the lines a leaf opens | new `PageCache.lean` | open |
| O18 | PAGE/ROM | an enabled heap line serves exactly the records of its segment | new `PageCache.lean` | open |
| O19 | PAGE/ROM | the line as a `Module` instance | new `PageCache.lean` | open |
| O20 | PAGE/ROM | an enabled ROM line serves the normalised row; key injective | new `RomLine.lean` | open |
| O21 | ZmLogUp | write-once memory soundness with the Schwartz-Zippel bound, including the coverage condition that every imported-tail row and every old-record provider row is a debit (ARCH-BRIEF obligation 1; the helper's NODE-PB row is the same obligation) | new `WriteOnceLogUp.lean`, not before M4 | open (not scheduled) |
| O22 | ZmLogUp | the public statement is the same for every cut under Profile B | new `WriteOnceLogUp.lean` | open (not scheduled) |
| O23 | VALUE DIGEST | distinct values give distinct preimage sequences | new `ValueDigest.lean` | open |
| O24 | VALUE DIGEST | equal `vd` implies equal values, under collision resistance | new `ValueDigest.lean` | open |
| O25 | VALUE DIGEST | `vd` is independent of sharing and layout | new `ValueDigest.lean` | open |
| O26 | VALUE DIGEST | a sponge started from the per-length constant equals the full sponge | new `ValueDigest.lean` | open |
| O27 | VALUE DIGEST | VALUE DIGEST as a `Module` instance | new `ValueDigest.lean` | open |
| O28 | BOUNDARY | the boundary serialization is injective and complete | new `BoundaryCommitment.lean` | open |
| O29 | BOUNDARY | the Poseidon digest instantiates `digest_inj` for the physical state | new `BoundaryCommitment.lean` | open |
| O30 | BOUNDARY | Horner boundary soundness (Profile B) | new `BoundaryCommitment.lean` | open (not scheduled) |
| O31 | NODE | NODE-L1 binary segment merge, bracket independent | new `BinaryNode.lean` | open |
| O32 | NODE | NODE-L2 seal | new `BinaryNode.lean` | open |
| O33 | NODE | NODE-L3 named-key routing | new `NamedAccumulator.lean` | open |
| O34 | L AND WRAP | DEC-L1 outcome-only statement | new `OutcomeStatement.lean` | open |
| O35 | LEDGER CONTRACT | LDG-L1 (was DEC-L2) at-most-once settlement across both entry points | new `ClaimDag.lean` | open |
| O36 | CONTINUATION | CONT-L1 checkpoint resume proves the same claim as one run (ARCH-BRIEF obligation 5) | new `ClaimDag.lean` | open |
| O37 | CONTINUATION | CONT-L2 a Running segment never reaches an accepted claim | new `ClaimDag.lean` | open |
| O38 | GP-2 hook | GP2-L1 claim chaining across runs (ARCH-BRIEF obligation 5) | new `ClaimDag.lean` | open |
| O39 | GP-3 hook | GP3-L1 foreign claims come from registered verifiers | new `ClaimDag.lean` | open |
| O40 | GP-4 hook | GP4-L1 join total at every depth, no counter | new `ClaimDag.lean` | open |
| O41 | GP-4 hook | GP4-L2 two prior segments plus inline work (the MERGE-WORK row) | new `BinaryNode.lean` | open |
| O42 | BUS, all port kits | port composition with typed port certificates and copy equalities (`leaf_refines_ports`), with the copied-port instance `toy_ports` | new `PortComposition.lean` | open |
| O43 | MEM, BUS | memory history `MemHist`: served MEM tuples are the records of the log, across leaves, in address order | new `MemContract.lean` | open |
| O44 | CORE | `rules_d` binds the slot lane list (SRC-001, SRC-003) | new `CoreRefinement.lean` | open |
| O45 | CORE, MEM | append bijection from A1 to A4, the window relation and `LookupSound` | new `MemContract.lean` | open |
| O46 | STACKS | partition independence for the compound stack effect (rewrite, then operation) | extend `PartitionIndependence.lean` | open |
| O47 | MEM | every accepted leaf satisfies `FitsWindow W` (cumulative fit from the window relation) | new `MemContract.lean` | open |
| O48 | BOUNDARY | header opening returns the committed header words | new `BoundaryCommitment.lean` | open |
| O49 | INIT | a leaf with `h_pre = 0` starts from `Init(prog_d, input_d)` | new `InitAdmission.lean` | open |
| O50 | INIT, MEMORY BACKEND | the genesis memory is canonical and stored kinds are in range (condition C4, base case) | new `InitAdmission.lean` | open |
| O51 | GP-2 hook | GP2-L2 sub-root soundness of DESCEND | new `ClaimDag.lean` | open |
| O52 | GP-2 hook | GP2-L3 chain exactness of the discharge list | new `ClaimDag.lean` | open |
| O53 | GP-3 general verifier | GP3-L2 completion gate and stream integrity of the work units | new `ForeignVerify.lean` | open |
| O54 | NODE | NODE-L4 program admission discharges `ProgCanonical(prog_root)` | new `BinaryNode.lean` | open |
| O55 | NODE | NODE-L5 input view (form (b) of AD-17) | new `BinaryNode.lean` | open |

Tally of the machine and recursion obligations (O54 and O55 cover program admission and the input view): 14 obligations proved in C3 (P1 to P14), 55 open (O1 to O55), 69 in total. The co-processor obligations are listed in their kits and counted apart: BR-L1, BR-L3 and ZK-L1 built at commit `bdb92973a` (the same build, with premises A1 to A7), BR-L2, BR-L4, ZK-L2, CF-L1 and CF-L2 open; 8 in total. O42 to O53 restated O2, O10, O11, O12, O14, O15 and O21; they change no status, and they narrow what P1, P2, P6, P7, P9 and P14 are credited with to the premises of their Lean statements. Of the open rows, O2 and O7 are premise statements, and O21, O22 and O30 belong to Profile B and are not scheduled before M4.

The five obligations of the ARCH-BRIEF addendum: (1) write-once memory soundness: O21, open, only if Profile B is built; (2) partition independence: P7, P9, P10, P14 proved for leaves of at most 8 appends and one stack operation per step; O12, O14, O15, O46, O47 open; (3) modular composition: P2, P3, P4 proved in generic form for bus tuples; O42 (ports), O43, O9 and the module instances open; (4) value digest canonicity: O23 to O25 open; (5) claim chaining: O36, O38, O51, O52 open.

The C3 hypotheses that stay hypotheses: `digest_inj` and `stuck` (`lean/Aggregation.lean:201`), the hash injectivity premises, and the proof-system premises (lookup soundness, accumulation and pairing soundness).

## 3. Build order and integration harness

### 3.0 Co-processor build order

Each step is useful whatever later experiments show. Heavy jobs run one at a time under `heavy-run` (`PR` section 10).

**Phase 1: the first build items, no heavy job.**

1. Compact frontend M0 (golden vectors) and M1 (reference machine, differential against native).
2. E-1 in placement V: `bind_label` on the `sha2` chip and the V+ `layout_d` word. T0 and T1 pass. First D4 and T0 on calls with a fallible transcript or non-empty effects; then the bind-circuit tests T2, with every mutant and the `label_d` link, and T3.
3. Atomic chunked calls: T1 at ledger level with anchor values in the `C_P` and `A_j` form, then the frontend's chunked mode (R-phase, R-cut, R-ctx, the anchor statement) and the native anchor prologue.
4. stark-backend parameter code: a per-fold `folding_pow_bits` vector; separate radius and bound multiplicities in `ProximityRegime`; an explicit per-round query vector with the keygen check, and query PoW 21 for the app, leaf-aggregation and internal sets; `NUM_OOD_SAMPLES = 2` with the transcript order and `gamma` batching of section 1.9 for `L` only; the `L` Merkle hash and transcript of section 1.8, with the transcript vectors T-FS on the CPU and GPU provers and the stark-backend verifier. The calculator change of D3 is the reference for the calculator part of this step. Then D3 reruns on the frozen sets at their shape bounds.
5. Lean: the restatement of section 1.9 on the committed round-2 files (`bdb92973a`).

**Phase 2: kernels and STARK path.**

6. The Fr Poseidon GPU kernel for `L`, starting with the microbench D7.
7. CPU wrap items of a day each: affine SRS, cached twiddles and parallel bit reversal, dynamic chunks, limb sort, multiopen fix.
8. sppark one-batch patch and window from the call size, then D9.
9. Frontend M2a: base chip set leaf AIRs, the planner, `key_root` under `prog_d`, byte-equal internal-recursive keys across two chip sets.
10. Atomic chunks T2 on the XC-9 devnet with T2-N and T2-N0.

**Phase 3: the wrap.**

11. D10: `L` tracegen over one node proof for the trace heights, then `w_stack` and the high row case; the keyed structure of `L` is MEASURED. The bound is `w_stack` at most 48 with `mu` PoW 22 (OD-3); a D10 value can only tighten it, and a tighter bound reruns D3 before the `L` key freezes.
12. M7 (D15): the G4 wrap circuit with the wrap-rows bundle, V+, the anchor statement checks, the DAG key check and the direct writer, then the cost model on the real `L` shape. Freeze k, `L_TX`, the capacity with the encoding of `pv`, the hash domains and the anchor template with the wrap key, and only after the security qualification gate passes.
13. Batched MSM on the MSM skew mask, then the NVRTC numerator.
14. Frontend M2b: one wrap proof that the ledger 9 verifier accepts with `2 + L_TX` public inputs.

**Phase 4: on chain and coverage.**

15. Path C and the resident daemon with its slot pool.
16. Frontend M3 (local devnet), M4 (acceptance sets, forced overflow), M5 (Jubjub, GP-2, GP-3).
17. E-1 T4 to T6 on the real wrap; atomic chunks T3 with universal-key chunks.
18. Preview deploy and call of the wrap key.

The order of the machine-level kits of section 3.1 against these phases is an open item of the specs.

### 3.1 Dependencies and order

An edge means "needs the other kit's interface frozen". During implementation each kit is tested against mock providers of the kits it depends on.

| Kit | Depends on | State today |
|---|---|---|
| BUS | none | dynamic bus landed; tagged table, ports manifest, row-role allocator new |
| RANGE | BUS | 128-bit shape landed; 64-bit shape and tagged query new |
| HASH | BUS | landed with `sponge_iv`; domain registry new |
| ALU | BUS, RANGE | ADD, LT landed; in-chip operand ranges, SUB, MUL, EQ new |
| DEC | BUS, RANGE; rule inventory (X1: 120 rules) | stub landed |
| MEM window | BUS | landed (C1); wiring into `RfLeaf` open |
| MEMORY BACKEND (dense) | BUS, HASH, MEM window | landed (C1) |
| PAGE/ROM block cache | BUS, HASH, MEMORY BACKEND | not built |
| STACKS | BUS, RANGE, HASH | not built |
| VALUE DIGEST | RANGE, HASH; `AuthRec` and `AuthRom` tokens of CORE's read port | not built |
| CORE | all leaf kits above by interface; rule inventory and per-rule table (X1; ROM part X15) | 32-word spike with 6 stand-in rules landed |
| BOUNDARY COMMITMENT | HASH; the state schema of CORE; STACKS cells; the backend token | spike digest over 29 words with a four-word header landed |
| INIT | BOUNDARY COMMITMENT, HASH, RANGE, STACKS, MEM, VALUE DIGEST; AD-17, AD-18, X11 for the input part | not built (the spike measured a four-table Init check, `O/design.md:172`) |
| REFERENCE BRIDGE | none in the circuit; AD-21, X13 | reference evaluator and projection landed for the dedicated grammar only |
| NODE INTEGRATION | the leaf public lanes; BOUNDARY COMMITMENT (`open_state_header`); VALUE DIGEST (outcome formula) | linear node landed (`crates/mori-recursion/src/ivc.rs`) |
| L AND WRAP | NODE INTEGRATION; stark-backend parameter code (build step 4); COMPACT FRONTEND (call vector, anchor) | today's k = 21 run decider landed and measured through ledger 9; `L` and the G4 wrap not built |
| ZERO KNOWLEDGE | L AND WRAP; HOST CHECKPOINT STORE | rules specified; daemon not built |
| COMPACT FRONTEND | CORE and DEC (`R_SL_*`), L AND WRAP (interface) | E-1 T0 and T1 pass on the local devnet and a bind prototype refuses the attack (`e1-test`, commit `4a4abd875`); chunk harness in `atomic-chunks`; frontend not built |
| HOST CHECKPOINT STORE | BOUNDARY COMMITMENT, MEMORY BACKEND and STACKS native models, INIT | not built |
| CONTINUATION | NODE INTEGRATION, HOST CHECKPOINT STORE | not built |
| GP-3 hook, GP-4 hook | NODE INTEGRATION (GP-4 also CORE for inline work) | not built |
| GP-3 general verifier (five subkits) | NODE INTEGRATION, GP-3 hook; X14 | not built |
| GP-2 hook | L AND WRAP, GP-4 hook, INIT, VALUE DIGEST (outcome wrapper); X12 | not built |
| LEDGER CONTRACT | L AND WRAP (public-value layouts), GP-2 hook (SET grammar), GP-3 hook (`reg_root`); N-11, N-8 | per-program `sherlock-run.compact` landed for the C6 milestone; the shared ledger contract not built |

Build order. One implementing agent per kit; kits in one group are built in parallel; a group is integrated only after the earlier groups pass their gates.

| Group | Kits (parallel) | Entry condition |
|---|---|---|
| F | interface freeze by the lead: section 1.2 and 1.3, the ports manifest (with `AuthRec`, `AuthRom`, the append chain and the state header), the hash domain registry, the state schema with its header and body split, the record layouts, the rule inventory, the node statement types and the node relation table | every freeze prerequisite of the list below |
| 0 | BUS | F |
| R | REFERENCE BRIDGE (host; runs beside groups 0 to 4) | F; AD-21; X13 |
| 1 | RANGE, HASH, MEM window (landed; conformance to the frozen ports only) | 0 |
| 2 | ALU, DEC, STACKS, MEMORY BACKEND dense (landed; conformance and the seam test), VALUE DIGEST | 1 |
| 3 | PAGE/ROM block cache; CORE framework (slots, ports, frame condition, append chain, read port, `rules_d`, definite assignment) | 2 |
| 4 | CORE rule families (one agent per family, in parallel); BOUNDARY COMMITMENT; then INIT | 3; per-rule table (X1; X15 for ROM); R for the ADT families |
| L | leaf integration by the lead: all kits in `RfLeaf`, whole-leaf harness green with zero coverage gaps, leaf rows and k measured (X5) | 4; R complete |
| 5 | NODE INTEGRATION (with `open_state_header`, X10) | L |
| 6 | L AND WRAP, HOST CHECKPOINT STORE then CONTINUATION, GP-3 hook, GP-4 hook | 5 |
| C | COMPACT FRONTEND (M0 and M1, E-1, chunked calls; phase 1 of section 3.0), then ZERO KNOWLEDGE with L AND WRAP | F for the frontend; ZERO KNOWLEDGE with group 6 |
| 7 | GP-2 hook; LEDGER CONTRACT | 6; X12 (GP-2 hook); N-11 and the template recapture (LEDGER CONTRACT) |
| 8 | GP-3 general verifier: FV-SCHED first, then FV-PARSE, FV-TRANSCRIPT, FV-ARITH and FV-FINAL in parallel | 6 (GP-3 hook); X14 |

Freeze prerequisites (one list; B2 finding 18). Group F starts only when each line has its evidence:

| Prerequisite | Evidence | State |
|---|---|---|
| X1 rule inventory and per-rule maxima | `FD/EXPERIMENTS-MAXIMA.md`, `FD/maxima/rule_maxima.md` | done for the inventory (120 rules), appends, stack operations and heap reads; ROM rows per step open (X15) |
| X2 one-arena non-interleaving | `FD/maxima/noninterleave.json`, decision M-1 | done (premises mechanical, the induction a code reading) |
| B2 audit of this file answered | `FD/ARCH-AUDIT-RESPONSE.md` | done |
| Init relation and its owner | kit INIT; AD-18 for the one-arena field list | specified; the input part waits for AD-17 and X11 |
| Boundary commitment structure and opening interface | kit BOUNDARY COMMITMENT (`open_state_header`) | specified; X10 measures it before the NODE spec freezes |
| Dependency discharge protocol | GP-2 hook, items 1 to 7 | specified; X12 (model) before the GP-2 spec freezes; not a prerequisite of the leaf freeze |
| Reference coverage | kit REFERENCE BRIDGE; X13 | open; a prerequisite of group L, not of the leaf interface freeze |
| Record layouts and physical cells per record | AD-11 | open |
| ROM depth over physical rows and `K_rom` | AD-12, X15 | open |
| FP-1 Bus tuple and ports over BabyBear | the canonical limb encoding of every key and payload, the injectivity lemma over the limbs, LogUp interaction bounds in place of BUS-007, AIR interfaces in place of the halo2 `Layouter<F>` signatures (S9 council: all three lanes) | closed: section 1.11, FP-1; Lean `bus_enc_inj` open; LEAD F-1, F-2 |
| FP-2 Recursive statement envelope of the STARK node | the deferred-log commitment `(d, n)` and the forwarded `key_root` in the node statement, with the update rule of every mode (disabled slot, SEAL, JOIN, discharge), consumed exactly by `L` (S9 council) | closed: section 1.11, FP-2; Lean `node_log_merge`, `node_key_route` open; LEAD F-3 to F-5 |
| FP-3 Public statement codec | which layer (`L` or the wrap) opens the sealed SET head (LW-008), the run and collection effect-word indices and discriminator in the `2 + L_TX` call vector, the index table of the public values of `L`, and the public form of `prog_d` (owner item N-11; ZK-002 keeps plain `prog_d` off the ledger) | open: section 1.11, FP-3 closes the opening layer (`L`), `C_P`, the discriminator and the index table of `L` (LEAD F-6 to F-8); missing facts: the carrier of the effect region and `comm` from a multi-shard terminal state to the call vector, the call-vector lanes of `acceptRun` and `acceptCollection` (contract not built), the contract-side form of a collection under `sherlock-value/2`; N-11 is an owner item |
| FP-4 One digest table | one definition (field, sponge, tag, preimage order, salt) of `input_d`, `outcome_d`, `prog_d` through `image_d`, `claim_d` and `fclaim_d` under `sherlock-value/2`; the tags `SHK_RUN`, `SHK_CLAIM`, `SHK_SET_LEAF`, `SHK_ADM`, `SHK_VIEW` registered under OD-18; VALUE DIGEST, HASH and INIT restated from their `sherlock-value/1` prototype forms (S9 council: all three lanes) | closed: section 1.11, FP-4; Lean `digest_table_inj` open; LEAD F-9, F-10 |
| FP-5 Seal read of `view_root` | a typed authenticated opening of `view_root` from the state (header word or body opening) for both seal modes under AD-17 form (b) | closed: section 1.11, FP-5 (header word); Lean `header_open_sound` open; LEAD F-11; AD-17 stays an owner item |
| FP-6 Public `n_steps` | exact height (NODE-003) or a padded value (ZK-008) | closed: section 1.11, FP-6 (not public); Lean `run_view_no_nsteps` open; LEAD F-12, the `n_steps` rule is an owner item |
| FP-7 Chunk and E-1 bindings | the claim address and `ep_hash` per chunk under placement V (ANC-010); the window offset of E-1 under a variable-length SHA-256 (CF-010) | closed: section 1.11, FP-7; Lean `bind_label_sound` and CF-L2 open; LEAD F-13 |
| FP-8 Universal key set | G1 to G5 passed; owner confirmation of OD-1 to OD-4, OD-17 and OD-18, with the OD-2 choice between 22-bit and 21-bit PoW | open; gate not passed |
| SRC-023 at the window's capacity (`FitsWindow`, O14 and O47) | the Lean theorems, no `sorryAx` | open. It does not block the interface freeze; it blocks reporting SRC-023 as proved for the circuit rule. Until then the claim is stated with its evidence: C3 for leaves of at most 8 appends, MockProver and the Quint lane beyond |

Agents may start a later group early against mocks once group F is done. The helper's merged order (from `astra-area3.md:1192-1203` and `gemini-area3.md:1055-1056`) put MEM window after MEMORY BACKEND; the landed code has the opposite edge (`persist` takes the `Window`), and the table above follows the code. Profile B kits are in no group (L-5).

### 3.2 Integration harness

One whole-leaf differential test against the reference machine `sherlock-k-a/1`, run on every module change (MODULARITY.md:12).

- Where: `crates/mori-chip/tests/diff_leaf_vs_sherlock_ka1.rs` with support code in `crates/mori-chip/tests/support/` (G:889 test name; A:1219-1221 support layout). Whole-run composition tests live in `crates/mori-recursion/tests/` (A:1222; G:905).
- Reference: the REFERENCE BRIDGE kit. `crates/mori-core/src/sherlock/reference.rs`, `Machine` (`:9`), executes at most one `sherlock-k-a/1` transition or free terminal publication per `step` (`:39-43`), but only for the dedicated grammar: its values are `UInt`, `Bool`, `List`, `Tree` and its expressions have no constructor (`reference_types.rs:21-26`, `:69-78`), and the projection returns `UnsupportedGeneric` for generic nominal constructs, planned matches and constructor state (`helper_machine/reference_projection.rs:10-11`). The corpus below needs constructors of more than 8 fields and structured inputs, so this harness is an acceptance gate only after the REFERENCE BRIDGE kit is complete (deliverables R1 to R4, zero coverage gaps). Until then it runs on the dedicated-grammar subset as a development check and its result is not a completion gate. A projection gap is a failing coverage item, never a silent skip, and the language is never reduced to the supported subset (A:1270; B2 finding 14).
- What it compares, per case (program, input):
  1. Per enabled step: rule id, control tag, fuel, stack depth, the value digest of a created or returned value, the consumed-dependency digest and the terminal outcome digest (G:839-846). A maps chip events to helper events and helper intervals to reference steps explicitly; helper-event count is not equated with reference step count without that map (A:1229-1232). `n_steps` counts enabled owner events, free terminal publication included, padding excluded (SRC-003, `requirements.md:28`).
  2. At every leaf close: the decoded logical state from the leaf's `post_d`, decoded independently of the witness generator (A:1243), equals the reference state; leaf close leaves fuel and counters unchanged.
  3. At the end: `prog_d`, `input_d`, `outcome_d`, `n_steps` and `claim_d`.
  4. Every leaf passes MockProver.
- Partitions (SRC-023 acceptance, `requirements.md:48`): all leaves full, one step per leaf, and one mixed partition. "All leaves full" respects every capacity; a step that needs more memory work than a leaf holds closes the leaf early (A:1251). The four values and `claim_d` must be equal across the three; a shared and a duplicated structure give one `vd`.
- Stop and resume (GP-001) and the replay refusal of the second partition's proof are separate integration gates in `crates/mori-recursion/tests/` (CONTINUATION kit), run through `scripts/doctorwatson heavy`, not on every change.
- Profile: Profile A only. Profile B is benchmark-only (L-5) and is not in the every-change harness (G:856 puts it there).
- Corpus (merged A:1257-1268 and G:849): arithmetic and zero-allocation loops; nested calls with frame and task spills; a constructor of more than 8 fields built across a leaf boundary; non-empty memory tails at leaf boundaries; shared and duplicated equal subvalues; structured input; Halted, Rejected (all codes) and Exhausted (fuel and stack) runs; raw height and counter boundary cases. GP-2, GP-3 and GP-4 fixtures are in the recursion tests.
- Hostile witnesses: injected after honest witness generation through a raw assignment path that bypasses host checks (A:1249; G:874). A panic, synthesis error or decoder refusal earns no rejection credit (SAS-003 as cited by G:862); each mutant has a weakened-constraint control that passes when the gate under test is switched off (SAS-004 as cited by G:863). The SAS rows were not read for this merge. Each mutant is one record of the harness (the `MutantSpec` of `gemini-area3.md`, section 6 of that draft): the raw-witness override, the control circuit, and the stage at which the refusal is expected, one of gate constraint, lookup, permutation, or pairing and transcript for a real proof. A refusal at another stage than the recorded one fails the mutant. The harness is the test infrastructure of every kit; its spec is in section 5.2.3.
- Runtime: G gives under 60 s for the MockProver tier (ESTIMATE, G:882); A says it must be measured after the first complete leaf (A:1301). Measure it.

### 3.3 Gates

Per kit:
- Constraint tests (the stark-backend debug constraint checker for STARK AIRs, MockProver for the wrap circuit and the halo2 prototypes): every positive test passes; every named mutant is refused at the constraint, lookup or permutation boundary from a valid baseline with a fresh malicious witness; parser refusals recorded apart (A:285). Each mutant has its weakened control (G:863).
- Quint lane: a Gemini lane run through `argon-run` with the quint-llm-kit skills writes or extends the kit's model under `models/sherlock/quint/` (existing models use camelCase names, for example `aggregation.qnt`, `chipLeaf.qnt`). The lead re-runs the model and each seeded model mutant before the kit is accepted. `quint typecheck` and `quint test` passing is not the gate by itself. The gate is a verdict file, in the form of `qa/gemini-quint/c1-dense/verdict.md`, that records: the named invariants that were explored; for each, the command, the bounds (`--max-samples`, `--max-steps`, the model constants such as `SEGC` and `TOP`) and the seed; the result of every mutant module (the invariant violated with an accepted tampered witness, and the honest witness still accepted under the mutant); the conditions the model assumes of other kits; and every difference between the model and the code.
- Lean: `lake build SherlockSpec` green (`clean/fork/lakefile`, `lean_lib SherlockSpec`); each new module has an axiom-report companion `<Name>Axioms.lean` (the C3 pattern, `FD/EXPERIMENTS-C3.md:5-14`); `scripts/qa/check-lean-aggregation.sh` stays green. A kit is complete only when every obligation that its kit lists as required for the kit is a proved concrete theorem, and the `#print axioms` report of each such theorem holds only `propext`, `Quot.sound` and `Classical.choice`: no `sorryAx` anywhere in its dependency closure. A statement with `sorry` in an obligations file is an open obligation; it lets the build pass and never lets a kit or the change be reported complete. Premise structures (hash injectivity, proof-system soundness) are hypotheses of the theorems and are listed in the receipt.
- Rust: the repository's mandatory checks, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings` and `cargo test --workspace --locked` (`AGENTS.md:28-30`). Known state: clippy fails today under the pinned toolchain on files outside this design (`qa/gemini-quint/c1-dense/verdict.md:43`; `FD/EXPERIMENTS-MAXIMA.md`, runs `maxima-clippy`: 271 errors in `mori-core` lib). Clearing that baseline is a prerequisite task of the change (section 5.3); until it is done a kit records the clippy result of its own files with `--no-deps`, and the workspace gate stays red and is reported as red.
- T0: the pre-commit hook runs T0 on every commit (`AGENTS.md:19`).
- Heavy jobs (keygen, real proofs, `cost_model` of node and decider, `lake build`): only through `scripts/doctorwatson heavy --id <id> [--budget KEY] -- <cmd>`.
- Rows: leaf, node and decider row and table figures recorded with `cost_model`; no figure is reported MEASURED without a run id.

Integration:
- Whole-leaf harness green on every module change (three partitions), with the REFERENCE BRIDGE coverage report at zero gaps.
- Derived gates: no BLS12-381 circuit above k = 21; the wrap fits 85 percent of `2^k - 7` usable rows at the smallest k of 19, 20 and 21, with every gate and lookup of degree at most 5 and one fixed 2^16-row range table; the wrap verifier key at most 50,000 bytes; every STARK shard has a fixed shape from the plan's menu and keygen refuses a key outside the shape bounds of its layer (section 1.9); `L` height at most 2^20 and `w_stack` at most 48 with `mu` PoW 22 (OD-3), with a D3 rerun when D10 tightens the bound. STARK leaf and node cost is recorded in seconds per proved step.
- Node boundary: `h = 2^64 - 1` accepted, `h = 2^64` refused (GP-001 acceptance).
- Real-proof checks: SRC-010 independent verifier, SRC-020 `validate_call` with the five mutations refused.

The GP acceptance matrix (complete; `O/general-purpose.md:16,26,36,48`, `O/requirements.md:56-59`). A hook or the change is reported complete for a GP target only when every line of that target has passing evidence. Prototype milestones (M4 linear path, the native GP-3 probe, a node measured without inline work) may pass while a GP target stays open, and are reported as milestones.

| Target | Line | Evidence |
|---|---|---|
| GP-001 | one key set and one deployed verifier contract accept proofs of at least three programs that the key set was not built from | real-proof integration test, one contract address |
| GP-001 | among them: one program larger than one leaf's carriers, one run with input, one run of each outcome kind (`Halted`, `Rejected`, `Exhausted`) | the same test |
| GP-001 | one run of more than one leaf that allocates more memory than one leaf holds and builds a constructor of more than 8 fields across a leaf boundary | the same test; needs the REFERENCE BRIDGE |
| GP-001 | its final proof has the same byte length as a one-leaf proof | byte lengths recorded |
| GP-001 | the same allocating run under three leaf partitions, and with one stop and resume, gives the same `prog_d`, `input_d`, `outcome_d`, `n_steps`; the second submission is refused by `claim_d` | CONTINUATION and HOST CHECKPOINT STORE tests; SRC-023 |
| GP-001 | at the constraint boundary a height of `2^64 - 1` is accepted and `2^64` refused | `pos_node_height_2p64_minus_1`, `node_height_wrap` |
| GP-002 | (a) a run of B whose input is the verified result of a run of A, accepted once on chain | `pos_gp2_b_consumes_a_halted` as a real proof |
| GP-002 | (b) one proof that aggregates at least three runs of different programs, accepted once on chain, from which each claim can be opened | `pos_decider_collection_three_runs` as a real proof, with the claim openings |
| GP-003 | one proof that aggregates a Sherlock run and at least one Compact contract circuit proof, both checked in circuit against the registry, accepted on chain | `pos_gp3_sherlock_plus_compact` (native path). The full scope of GP-3 also needs the general verifier kit: `pos_fv_plonk_proof_same_srs`, `pos_fv_plonk_proof_other_srs` |
| GP-004 | one key set proves a chain, a balanced tree and a DAG in which one claim feeds two consumers, each at depth >= 8, with equal final proof size | real-proof acceptance test |
| GP-004 | a node with two prior proofs plus new work | `pos_gp4_two_prior_plus_step` as a real proof of one MERGE-WORK node; two successive nodes do not count |

### 3.4 Module kit template

Both drafts give one (A:1307-1355; G:1067-1127). Merged: the nine kit headings of section 2 are the template, plus three sections taken from the drafts.

```
### Kit: <NAME>  (interface version <v>)
#### Purpose            two sentences
#### Interface          every tuple requested/provided, named cells, field ranges, range-check owner,
                        domains and tags, canonical inactive encodings
#### Invariants         guaranteed (each with its test and model property); assumed (each names its provider)
#### Row budget         ESTIMATE with inputs named; shared costs not counted twice
#### Verification       Quint model (existing or new); MockProver positives; named mutants with weakened controls
#### Lean obligations   plain words; file; premises; contract; status
#### Code skeleton      owned files; public types and signatures with Requires/Ensures; no bodies
#### Agent reads / must not need to read
#### Dependencies
#### Owned files        exact production and test paths (A:1313; G:1071)
#### Completion evidence  positive and negative results at their real boundaries; measured costs where
                        required; remaining assumptions (A:1349-1354; G:1121-1127)
```

Rules from the drafts kept: an agent stops and reports an interface gap rather than reading another module's internals (G:1072); it never infers another module's invariant from a test fixture or host helper (A:1357); large modules (general foreign verifier) are split into several kits (A:1362). Dropped from G's template: the fixed proof-system rules that assume Profile B committed instances (G:1077) and the "<= 1,500 lines" reading ceiling (no source).

## 4. Open decisions

Each entry has its options, the recommendation, what decides it, and an OWNER mark where the owner must decide. The co-processor decisions OD-1 to OD-18 are in `FD/DECISIONS.md`, each a lead decision with the owner to confirm. This section holds the machine-level decisions AD-1 to AD-23 and the kit decisions N-1 to N-14 and V-1 to V-10; the owner can reverse any of them.

### 4.1 Decisions from the decided inputs and the leaf kits

| Id | Decision | Options | Recommendation | Decided by | Owner |
|---|---|---|---|---|---|
| AD-1 (R-4, E-1) | Tree depth TOP | (a) TOP = 61: 2^64 - 1 cells; with up to 3 appends per step (measured by X1, M-2) the address space binds before 2^64 steps, which narrows GP-1 formally. (b) TOP >= 63: cells >= appends-per-step x 2^64. (c) accept a cell bound below the step bound | (b) TOP = 63. Measured cost: +280 rows per flush unit, +140 per cold read, no new column, no change of k (C1, E-1). X1 measured at most 3 appended cells per step (M-2), so 2^64 steps can need `3 x 2^64` cells: TOP = 61 is too small and TOP = 63 (`2^66` cells) is enough, with room for a genesis length below `2^64`. The exact condition is over physical cells: `len_0 + a e 2^64 <= 2^(TOP+3) - 1` with `a = 3`; it holds at TOP = 63 only if a logical record is one physical cell (`e = 1`, AD-11); `e = 2` needs TOP = 64. Option (c) is not an acceptance path of this change: a memory bound below the run scope leaves GP-1 incomplete, it does not authorize a narrower GP-1 (B2 finding 9) | X1 done (`a = 3`); AD-11 fixes `e`; C1 has measured the cost | **OWNER**: confirm TOP >= 63 (R-4, M-2). Option (c) could be taken only by an explicit owner decision that amends GP-1, recorded in `O/dissent.md`; this file never takes it |
| AD-2 (R-5) | Claim counter | (a) `n_claims < 2^64` range-checked occurrence count (as `research/sherlock-gp-architecture-20261110/architecture.md:100`): refuses an honest diamond DAG at depth 64 with about 128 proofs, narrows GP-4. (b) a count of distinct claims. (c) no count in the public statement | (c) for the statement: `set_root` binds the claim DAG and carries the soundness; no count is range-checked and no count carries soundness. If telemetry wants a number, it is a distinct-claim count outside the soundness argument. This is the mechanism of kits GP-2 and GP-4 hooks | A read of every consumer of `n_claims` (GPR-010 and the collection public values `[reg_root, set_root, n_claims, shape_d]`, `architecture.md:103`) to confirm nothing depends on it; a Quint check of the diamond DAG at depth above 64 | **OWNER**: confirm |
| AD-3 (L-2, M-3) | Cache-line counts `K_page`, `K_rom`, the number of scalar cold-read units `n_cold`, bus requests per step, and the per-rule maximum table | The table is a prerequisite, not an option: per rule, over every branch, the maximum bus requests (ALU, MEM reads, MEM appends, ROM, HASH), distinct old-memory segments, distinct ROM blocks per table, appended physical cells, stack hashes, range checks and hashes in one step. Then choose the smallest `(K_page, K_rom, n_cold)` that fits every rule at one step per leaf | Status after X1 (M-2, M-3): `w_flush = 1` is derived (3 consecutive cells cross at most one boundary); `k_page + n_cold >= 4` is required (4 old heap records in up to 4 segments, a code-reading bound; 2 observed), so the prototype value `K_page = 4` is now the bound from the rules and not a guess; `K_rom` has no bound and stays open, and `K_rom = 2` is known to be too small for a rule that reads 3 or 4 tables (`XAdtHelperPlanSwitch`: 4 tables by the static site count). Nothing is frozen until X15 and X5. A step must never need more lines or reads than a leaf has (GP-1) | X1 (done except ROM), X15 (ROM rows and blocks per step), a per-path count that makes the heap bound of 4 mechanical for the 4 rules where it is a code reading, then X5 (leaf rows) | no, unless the table shows that no setting fits `k_leaf <= 16`; then the gate change is **OWNER** |
| AD-4 (L-9, R-6, E-3, M-1) | Arenas | (a) one arena with kind-tagged records: 12,281 backend rows (MEASURED). (b) four arenas: 49,172 rows (MEASURED); with 4 + 2 cache lines about 67,500 to 85,100 rows (ESTIMATE), above `k_leaf <= 16` | (a) one arena: decided (M-1). X2 found the non-interleaving property to hold for all 120 enabled rules with no counterexample: premises S1 to S4 are checked mechanically (static over all rules, dynamic on 6,668 steps and 9,031 slices), and the induction from them to slice contiguity is a code reading. The MEM interface still hides the count (`kfix`) | done (X2). Remaining hardening, not a blocker: mechanise the induction (a Quint model or an assertion in `owner_apply_validated`) | no |
| AD-5 (R-9) | Dissent D-1, SI-12 to SI-14: linked slices | (a) Carbon's linked-slice form (the candidate to measure first, `O/dissent.md:198-203`). (b) Astra's mapping from logical identifiers to addresses. (c) the form this design implies: contiguous slices in dense memory, addresses as references, no padding | Keep D-1 open (`gemini-area2.md:877-879` recommends (c) and rejects the other two from a reading, which R-9 does not accept as closing it). Propose (c) as a new candidate and rewrite task 4.18 to measure it against (a) with the scenario of the C6 review. The status change in `O/dissent.md` is made by the lead or the owner, not by this file | X6 (task 4.18 measurement: a constructor of more than 8 fields built across a leaf boundary, rows and steps for each form) | **OWNER** for the status change of D-1 |
| AD-6 | S (steps per leaf) | 16 (SRC-014, `S_MAX = 16`, `crates/mori-recursion/src/ivc.rs:77`); 64; 128 | 16 for the first build. S is a `LeafShape` parameter; every kit is written for any S. ESTIMATE leaf rows at TOP = 63 with no cold read: 15,226 (S = 16, W = 1), 37,387 (S = 64, W = 3), 59,969 (S = 128, W = 5). A larger S changes the per-program app and leaf-layer keys only, never a universal key (section 1.6), and needs an SRC-014 amendment and the multi-flush theorem | X5 (measured leaf and node rows with the backend wired into `RfLeaf`) and the early-close rate from X1 | **OWNER** if S changes (spec amendment) |
| AD-7 (L-5) | Profile B (ZmLogUp + Horner) | build now; benchmark-only after M4; drop | benchmark-only behind the seam, not before M4. It cannot stop and resume without reloading live memory | a benchmark after M4, only if Profile A misses a throughput target | no |
| AD-8 (L-8) | Requests per step and slot layout | two lanes, more rows per request (landed); row-6 promotion to four requests on a shorter slot | landed layout. Promotion is a layout experiment with the acceptance "no new column, all rules placed, rows 5 and 7 still reach their lane copies" | the bus requests per rule (X1 measured appends, heap reads and stack operations per rule; ALU, range and ROM requests per rule are not in it: X15 for ROM, the rule-family contracts for the rest), then a layout spike with `cost_model` rows | no |
| AD-9 | DEC key construction and control word layout | (a) priority network to a finite class; (b) packed key of normalized guard fields | (a) for the first build; the control word layout is frozen with the guard list of the 120 rules | the guard list (the inventory is done, X1; the guards per rule are not part of X1) | no |
| AD-10 (M-4) | Rule inventory | `RuleIdC` has 133 variants (`shared.rs:400-534`); 7 disabled and 4 extension leave 122 (`owner_runtime.rs:70-90`); 2 of those are `KAdminStartup*` rules that are no step; `supported_rules()` counts 108 (`O/tasks.md:67`); drafts said 108, 123 and "120 + 2" | Resolved by X1 (M-4): the inventory is the 120 enabled step rules, which are the 108 of `supported_rules()` plus the 12 `XAdtHelperBuild*` and `XAdtHelperPlan*` rules; the test asserts that the 13 other variants are exactly the 7 disabled, the 4 extension and the 2 startup rules. The frozen interpreted list is these 120, explicit, versioned and ordered; Compact Stage 1 adds the `R_SL_*` family under `machine_version`, which changes per-program keys only (section 1.7). GP-1 says every program of the language: a rule that is enabled in the reference machine and unsupported in the circuit is a gap to close, never a language restriction (`astra-area1.md:1130`). 18 of the 108 did not fire on the X1 corpus (16 failure or refusal paths); each needs a constructed case (kit REFERENCE BRIDGE). That the backend's `supported_rules()` list equals the Core groups is a code reading | done (X1); the 18 unfired rules by the REFERENCE BRIDGE corpus | no; **OWNER** only if a rule is to stay out |
| AD-11 | Record layouts per kind and the state schema lane count | three-word records at consecutive addresses (landed key); wider records by a second lane in the key (Astra) | three-word records, consecutive addresses; schema file frozen before CORE rule families start | a schema review against `O/design.md:220-267` and the value forms | no |
| AD-12 | ROM depth and the program-size bound | a fixed depth is a circuit constant that bounds the program tables (16, 29 and 32 were drafted) | state the depth over physical rows and show that `D_row` is at least the index width of the language's program tables plus the sub-row bits of a wide row. A smaller depth and a per-program depth key are rejected alternatives under this change: if the required depth does not fit, GP-1 is incomplete (LC AUDIT finding 12; B2 finding 9) | a reading of the program table index types and the wide-row layout (V-2); kit PAGE/ROM | no for this change (the narrowing options are not offered); an amendment of GP-1 would be the owner's own decision |
| AD-13 | Leaf public lanes | 7 lanes as landed `[binding, prog_d, pre_d, post_d, rule, h_pre, steps]` (`ivc.rs:70-74`); 8 lanes `[binding, prog_d, input_d, pre_d, post_d, h_pre, steps, deps_d]` (`architecture.md:97`) | keep the seven lanes, with lane 5 the SRC-001 hash `rules_d`; `input_d`, `deps_d`, the terminal kind and payload are the header words of the state commitment and are read from `post_d` at the seal by `open_state_header` (kit BOUNDARY COMMITMENT item 5; N-2 option a; AD-19). Fallback: a new versioned statement with an SRC-001 amendment, never a silent change of `sherlock-leaf-stmt/1` | X10 (does the seal-time header opening fit the node) | no; **OWNER** only if the fallback is needed (spec amendment) |
| AD-14 | `decl_schema_d`: the normal form of constructor type identity, and the form of the PROG canonicality premise | program-local index; program-independent `ty_d` | keep program-independent `ty_d`; define the normal form before key freeze (`O/dissent.md:204-207`, `O/design.md:84`). The discharge point of the premise is fixed: the seal requires the `ADM` claim of ADMIT-WORK (kit NODE INTEGRATION). This decision fixes only the `ty_d` normal form that ADMIT-WORK checks | a spec task in `sherlock-vm-consensus-20261107`, not an experiment | **OWNER** (it is an open item of the existing spec) |
| AD-15 (R-7) | Continuation mode 2 (address-free checkpoint) | adopt; open research | open research; tier 1 plus retained-memory mode 1 only | a written definition that closes the gaps of RL AUDIT finding 5 | no |
| AD-16 | Exhaust kind 7 against profile `/1` | the encoding lists it; SVM-009 refuses it (`O/dissent.md:226`) | DEC enforces the profile; the spec conflict stays recorded | the existing spec change | **OWNER** (existing open item) |
| AD-17 (B2 finding 3) | Input admission: how the value behind `val_d` becomes readable by the run (`O/design.md:194`, open since the C6 review; SRC-021, owner question 1 of `O/dissent.md`) | (a) a loading prefix that writes `enc(x)` into memory before `Eval(ENTRY)`; (b) by-digest admission with an empty genesis memory and a certified read-only input view: the input is a reference by digest, a step reads an input node through `AuthIn` under the `view_root` body word, and the seal requires the `VIEW(val_d, view_root)` claim of VIEW-WORK (kit NODE INTEGRATION); (c) an input constant table under a committed root with a digest check per row read | (b). It is the only candidate that meets the contract I1 to I7 of kit INIT without a scan of the input and without a new input-size limit; it makes the genesis memory the two empty constants, which is the simplest form of condition C4. (a) needs a scan at genesis or counted loading steps that change `n_steps`. Under (b) the mechanism for wide constructors and input reads is VIEW-WORK with the `AuthIn` read port (no fold in a step, no MEM append per read). Open points of (b): the depth `D_view`; whether a lazy view is allowed beside the full view; how the seal reads the `view_root` body word (the header opening returns `body_d` only); and the owner question of I7: `in_ty` checked on inspected nodes only (recommended), or a VIEW-WORK check of `ty(x) == in_ty` over the whole input | X11 (the cost of `read_input` per step and of VIEW-WORK per entered record; whether the reference charges `Literal(x)` cells for an input read; step count against the reference run of `apply(prog, x)`; a constructor input of more than 8 fields) | **OWNER** (it settles an open item of the governing specification) |
| AD-18 (B2 finding 3) | The one-arena Init field list: SRC-021 and `O/design.md:222-266` name 39 fields with four `(root, length)` pairs; L-9 and M-1 decide one arena | (a) amend SRC-021 and the Init table to the one-arena list of kit INIT: one `(len, root, tail_d)`, the two stack constants in place of `NIL` links, `input_d` and `deps_d` as header words; (b) keep four roots, which contradicts L-9 | (a) | a spec edit of `sherlock-vm-consensus-20261107`, after AD-17 | **OWNER** (SRC-021 text; the owner keeps the final call on it) |
| AD-19 (B2 finding 5) | Structure of the state commitment | (a) a flat sponge over all words, re-hashed in full wherever a word is opened; (b) a header and body commitment: a body digest and a seven-input header (section 1.11, FP-5) | (b), specified in kit BOUNDARY COMMITMENT. It keeps the landed opening pattern and costs one small sponge per opening. It deviates from the letter of LC AUDIT item 25 for the header only (`FD/ARCH-AUDIT-RESPONSE.md`) | X10 (the opening measured inside the NODE seal modes; `L` opens a SET head only (section 1.11, FP-3)) | no; **OWNER** only if X10 fails and the fallback leaf statement `sherlock-leaf-stmt/2` is needed (AD-13) |
| AD-20 (B2 finding 6) | Mid-run claim consumption | (a) input consumption only: a consumer takes a producer's Halted value as its input, checked by the input link at the seal; no owner-machine rule; (b) add an owner-machine consume rule to the profile, with the `deps_d` chain | (a) as the built path of this change, with the circuit side of (b) (chain, DL-STEP, SEAL-WITH-DEPS) specified so that (b) needs no new node relation. (a) meets the two acceptance lines of GP-002 as written, and the reference machine has no consume rule. Whether (a) alone is the whole of GP-2 is the lead's reading and is not settled here: field 39 of the Init table (`deps`, "no consumed claim") shows that consumption inside a run was contemplated, and SRC-021 says that the proof authenticates "the dependency claims" when the first execution segment begins (`O/requirements.md:46`), which this design does at the seal (the input link) and not at genesis. GP-1 to GP-4 are never narrowed: if the owner reads GP-2 as requiring (b), or requires the dependency to be bound at genesis, then (b) and that binding are required work of this change and (a) is only the first milestone. Dependency of (b) on the run entry: under (a) an input link is a `SHK_RUN` claim, so no foreign claim reaches a run claim; under (b) a FOREIGN claim can enter the `deps_d` chain and a `DEP` cover, so the run entry of kit L AND WRAP changes with (b) in one of two ways: (i) refuse a `DEP` head whose cover holds a FOREIGN member, or (ii) read `reg_root` into the run-entry transcript from contract state, as the collection entry does, and check it there (the four published run values do not change). Which one is chosen with (b) | X12; the owner's reading of GP-002 and of the SRC-021 sentence | **OWNER** (first among the machine-level owner questions: it decides the scope of GP-2 in this change, and option (b) adds a rule to the governing VM specification) |
| AD-21 (B2 finding 14) | The normative semantic reference for the full language (nominal types, constructors of any arity, planned matches, structured inputs) | (a) extend the dedicated `sherlock-k-a/1` evaluator `reference.rs` and its types; (b) a separate full-language evaluator over the source language `moriarty-sherlock-nominal/1`, with the dedicated grammar unchanged; (c) declare the owner machine its own reference for these forms | (b). (c) makes the differential harness compare the owner machine with itself for exactly the forms GP-001 names | X13 (the census of projection gaps on the corpus) | **OWNER** (it decides what "the reference machine" is for GP-1's "every program of the Sherlock language") |
| AD-22 (B2 finding 15) | Supported foreign proofs that cannot share the core accumulation SRS | (a) complete their final check in circuit, in bounded work units (FV-FINAL); (b) a second deferred accumulator per SRS, paired in the wrap | (a). (b) puts one constant per SRS into the wrap key, so a new SRS would need a new key | X14 (cost of the in-circuit final check per unit) | no; **OWNER** only if (b) is wanted |
| AD-23 | Consumer of `rules_d` after the leaf. Rule selection is constrained inside the leaf by DEC and CORE; no node, shape, `L` or wrap reads lane 5, so the lane adds no guarantee after the leaf | (a) give it a consumer: put `rules_d` into the LEAF base case of `shape_d` (`H(SHK_SHAPE, T_LEAF, h_pre, steps, rules_d)`), one more word per leaf shape; (b) amend SRC-001 so that lane 5 is a reserved zero lane, and drop the CORE `rules_d` sponge and its three mutants `core_rules_d_forged`, `core_rules_d_lanes_reordered`, `core_rules_d_disabled_lane_nonzero` (saves about 9 permutations per leaf at S = 16, about 630 rows, ESTIMATE) | none; until the owner answers, the kits keep the SRC-001 hash as written | the owner (SRC-001 and SRC-003 fix the lane, `O/requirements.md:26,28`) | **OWNER** (low priority) |

### 4.2 Open decisions of the recursion kits (N-1 to N-14)

From the digest of the two area 3 drafts. `A:` is `FD/drafts/astra-area3.md`, `G:` is `FD/drafts/gemini-area3.md`. N-6, N-9 and N-11 carry an OWNER mark; N-7 does if it changes GP-4 text. N-9 is AD-2. N-12 is AD-16. The lead's decision on N-2 and N-14 is in the lead notes of section 2 (seven landed lanes; `input_d`, `deps_d` and the terminal fields as the header words of the state commitment). N-3 is replaced by the protocol of the GP-2 hook, and AD-19, AD-20 and AD-22 of section 4.1 belong to these kits.

N-1. How leaf proofs enter the binary node.
- Options: (a) LEAF is a key kind of the binary node slot, so the node's fixed-base name union includes the leaf key; (b) a one-leaf lift relation (the landed linear node shape) turns each leaf into a SEG and the binary node selects only {lift, NODE, FOREIGN} (G:58-60 without the 2-leaf form). The 2-leaf form is out (L-10).
- Recommendation: (b) if the node measurement shows the leaf names push (a) outside the shape bounds of the internal set (DG-004; the halo2 gate of 950,000 rows is history, DG-007); else (a), which has one relation fewer.
- Decided by: node rows of both shapes over the real leaf key.

N-2. Leaf statement lanes.
- Options: (a) keep the landed seven lanes and carry `input_d`, `deps_d`, terminal kind and payload inside the state committed by `pre_d`/`post_d`, read by opening `post_d` at seal (CORE never writes `input_d` after Init); (b) a new versioned statement `sherlock-leaf-stmt/2` with extra lanes (A:80-93, G:42-53), which needs an SRC-001 amendment.
- Recommendation: (a). It keeps the landed interface (`FD/DECISIONS.md`, lookup AUDIT finding 22) and adds no node column. Proposed here, not by either draft.
- Decided by: X10 (the header opening of section 1.11, FP-5, measured inside the NODE seal modes; kit BOUNDARY COMMITMENT specifies it) and node rows for (b). Spec amendment if (b). Lane 5 is the SRC-001 hash `rules_d`, which CORE computes (kit CORE); the lane named `rule` in `ivc.rs:70` is renamed with that change. This is a required computation, not a naming question (B2 finding 12).

N-3. GP-2 dependency discharge format.
- Options: (a) G: running chain `deps_d` from 0, discharged at seal by equality with a SET root (G:51,69); as written the two hash structures differ; (b) the B1 proposal: the dependency commitment is a SET hash built by a dependency-join in consume order; the audit showed that this is not a protocol (a consumed claim can sit inside a JOIN or a DEP with its own ancestors, and empty dependencies, order, repetition, claim extraction and `n_steps` were undefined; B2 finding 6); (c) A: bind a dependency root before execution with membership openings (A:733), which conflicts with empty dependencies at Init (`requirements.md:46`); (d) the protocol of the GP-2 hook, items 1 to 7: a typed SET grammar, a `SHK_DEPS` chain in the consumer, DESCEND and head opening to authenticate a claim inside any verified SET, a discharge list DL built by DL-STEP, and chain equality at SEAL-WITH-DEPS.
- Decision: (d) replaces (b). It keeps Init empty and deferred discharge, needs no batching shape of the producer, and is bounded per node. It is in neither draft.
- Decided by: X12 (the Quint model `claimConsumption.qnt` with the invariants of the GP-2 hook, then rows of DESCEND, DL-STEP and SEAL-WITH-DEPS). The GP-2 spec does not freeze before X12 passes. AD-20 (OWNER) settles whether a mid-run consume rule is added.

N-4. Checkpoint domain tag in mode 1.
- Options: (a) tag only on the persisted record digest, no new circuit cell (G:663 "0 rows"); (b) an in-circuit continuation envelope (A:511-533 `SHK_CONT_V1`).
- Recommendation: (a). The run-length audit found no replay attack on mode 1 and asks whether a tag is needed at all (finding 5); R-7 names a checkpoint domain tag, which (a) provides for the record.
- Decided by: the `continuation.qnt` model showing a stored SEG cannot be confused with a claim without the tag. Lead to confirm the reading of R-7.

N-5. Closing proof and key set.
- Options: (a) the decider resolves the node's fixed bases directly (today's `run_decider.rs`; M4 path); (b) a separate closing proof resolves the finite union of node, lift and foreign base names before the decider (A:402-404; G:71-73).
- Recommendation: (a) for M4; (b) only if the measured `L` over the multi-key node exceeds its shape bounds (section 1.9) or the wrap exceeds its row gate.
- Decided by: decider rows over the binary node.

N-6. GP-4 "arity >= 2 plus new work". OWNER.
- Options: (a) inline owner work in the two-slot node (A:1024-1028); (b) two nodes, one for the two prior proofs and one for new work (G:1189 option A); (c) three-slot node at k = 21 (closed, `general-purpose.md:52`).
- Recommendation: (a), specified as the MERGE-WORK row of the node relation table, with the inline kernel built from the leaf kits' `configure` and `assign` functions (one encoding of the machine relation; a separate node emitter is not an option). GP-1..GP-4 are never narrowed; (b) is not an acceptance path of this change, as a design or as a measurement: two successive nodes are a different relation.
- Decided by: X7, a node with two enabled prior proofs plus one real owner step with its memory, stack and value-digest work, against the shape bounds of the internal set (DG-004; formerly the halo2 gate of 950,000 rows, DG-007). If it fails, GP-4 stays incomplete and the owner decides; the clause is not removed.

N-7. Bracketing disclosure in `shape_d` (dissent D-6).
- Options: bind execution intervals and bracketing (A:1047); bind only the claim graph.
- Recommendation: bind intervals (GP-004 binds the shape); disclosure stays the open dissent item. OWNER if it changes GP-4 text.
- Decided by: owner on dissent D-6.

N-8. `reg_root` lifecycle.
- Options: latest root only; append-only root history with revocation (G:1192).
- Recommendation: history with revocation, so long runs survive key admissions. NODE INTEGRATION requires `reg_root` equal across child slots (`node_reg_root_mixed`), which is sound for any lifecycle. Only if the history rule lets subtrees lifted under different roots be joined does the FOREIGN form need the root inside it, `FOREIGN(reg_root, fclaim_d)`, with the LEDGER CONTRACT checking the root history at opening.
- Decided by: contract design review; no experiment.

N-9. Claim count (R-5). OWNER.
- Merge decision: no count in the public statement; `set_root` binds the DAG; an optional telemetry count of distinct claims is not range-checked and carries no soundness. Both drafts drop the occurrence count; A:1051 would allow a constrained count (overruled).
- Decided by: owner confirmation.

N-10. General foreign verification path (GP-3).
- Options: native adapter classes only (G); native first plus a general bounded verifier (A:860-865).
- Recommendation: native first as a milestone; the general path is the kit "GP-3 general verifier" with five subkits, in build group 8 and in the spec outline (GP-3 is not narrowed; an adapter-only GP-3 is not an acceptance path). The native probe and the full GP-3 completion are separate tasks.
- Decided by: X8 (the adapter probe for the first class) for the native path; X14 (a costed operation inventory and the unit budget) for the general path.

N-11. Contract surface and program policy. OWNER.
- Options: (a) two templates: `acceptRun(prog_d, input_d, outcome_d, n_steps)` with `prog_d` as call data, and a claims contract with root acceptance and claim opening for batches and DAGs (G:1190); (b) the claims contract only. Separately, SRC-007 keeps a sealed `program_d` per contract while GP-1 needs one contract for all programs (G:1206; A:1417-1418).
- Recommendation: (a) as two entry points of one contract over one `runs` map (kit LEDGER CONTRACT: `acceptRun` with `prog_d` as call data, and `acceptCollection`, `descend` and `openClaim`), so one `claim_d` is accepted once on chain whichever entry inserts it. Two contracts with separate ledgers would let one claim be accepted once in each and are not an option. The C6 per-program contract stays as the milestone path. The sealed `program_d` question of SRC-007 stays with the owner.
- Decided by: owner (it changes SRC-007 and the deployment), then the template recapture (SRC-020).

N-12. Exhaustion kind 7 under profile /1.
- The encoding lists kinds 0 to 7 but profile /1 excludes `LogicalCap = 7` (A:95, A:1420-1421, citing dissent). L AND WRAP's "every admitted exhaustion kind" positive depends on it.
- Recommendation: the profile gate refuses kind 7 under /1 until the spec resolves it (A's rule).
- Decided by: spec owner of SRC-022 and the dissent item.

N-13. Staged or unified key set.
- Options: (a) build the full key set (leaf, one-leaf lift if N-1 picks it, foreign lift, node, optional closing, `L`, wrap) from the start, with the foreign slot pinned to a valid key before any foreign adapter exists (G:1188); (b) ship a linear `{leaf, node, L, wrap}` for M4 and replace the node key for GP-2 to GP-4.
- Recommendation: (a) in code structure, so M4 keys and Lean proofs are not thrown away; M4 may still run the linear path while the binary node is measured.
- Decided by: node and decider rows of the binary node (N-1, N-5).

N-14. Cross-area ask to BOUNDARY COMMITMENT (blocks the SEAL interface).
- With the seven landed lanes, SEAL and SEAL-WITH-DEPS read `input_d`, the dependency commitment `deps_d`, the terminal kind and the terminal payload by opening `post_d`. Answered: BOUNDARY COMMITMENT commits these four words as the header of a header-and-body commitment (as `ivc.rs:81-84` does for the outcome tag and value) and gives the typed opening `open_state_header`, one header sponge (section 1.11, FP-5); CORE never writes `input_d` after INIT. X10 measures the opening. If it does not fit, N-2 option (b) (a versioned leaf statement) is needed.

Note for the spec writer: lane 5 is `rule` in `crates/mori-recursion/src/ivc.rs:70` and `rules_d` in SRC-001 (`requirements.md:26`). SRC-001 defines it as `Poseidon(dom("SHK_RULES"), rid_0, ..., rid_(S-1))`; kit CORE owns the hash, its HASH slot, the public equality and the reserved zero lane.


### 4.3 Open decisions of the PAGE/ROM, ZmLogUp, VALUE DIGEST and BOUNDARY kits (V-1 to V-10)

From the digest of the area 2 drafts. V-1 is AD-12 (OWNER if the depth is below the language's index width). V-4 is AD-3. V-6 is settled by the lead note of section 2 (option a). V-7 is part of AD-11.

**V-1. ROM tree depth and program-size bound** (LC AUDIT finding 12, item 19)
- Options:
  - (a) A fixed `D_tbl + D_row` that is at least the index width of the language's program tables. Every table index type must fit.
  - (b) A smaller depth, such as the 32/33 bits in gemini-area2.md:246. This bounds the program size and narrows GP-1. Rejected under this change.
  - (c) A depth per program: a per-key parameter. Rejected under this change (a new program would need a new key, against GP-001).
- Recommendation: (a), over physical rows: `D_row` covers the widest table index type of the language plus the sub-row bits of the widest row (V-2). If (a) does not fit the leaf, GP-1 is incomplete; (b) and (c) do not become acceptance paths.
- Deciding step: read the table index types in the reference machine and fix the wide-row layout.
- Cost: `70 x (16 + 7 + D_rom)` rows per ROM line (ESTIMATE).

**V-2. ROM rows wider than three words** (constructor rows, constant rows)
- Options:
  - (a) Sub-rows, with the sub-row index in the low key bits.
  - (b) Wider provider tuples. This changes the bus width of 5 (`bus.rs:34`), so it is not recommended.
  - (c) Pre-hash wide rows into three words.
- Recommendation: (a), so that the bus stays as landed.
- Deciding experiment: the PROG normal-form table list (Area 1).

**V-3. Kind gate on heap line rows** (LC AUDIT item 7)
- Options:
  - (a) Rely on the induction from `persist`, which needs Init to prove the genesis DATA/FIELDS root, including the input prefix.
  - (b) Repeat the kind gate on line rows (`kind` in `0..=4`), at a few rows per line (ESTIMATE).
- Recommendation: (b) while the input-loading form is open (O/design.md:194, :257-258). Move to (a) once kit INIT is built and proves the genesis memory (condition C4); under the recommended input form (AD-17 b) the genesis memory is empty and (a) holds at once.
- Deciding experiment: AD-17 and X11; kit INIT.

**V-4. `K_page`, `K_rom` and `S`** (L-2, LC AUDIT finding 1, item 28)
- Options: values from the per-rule maximum table and the locality census.
- State after X1 (M-2, M-3; the same as AD-3 and section 5.4): `w_flush = 1` is derived; `k_page + n_cold >= 4` is the bound from the rules (4 old heap records per step, a code-reading bound for 4 rules); `K_rom` has no bound yet.
- Recommendation: keep `k_page`, `k_rom` and `n_cold` as `LeafShape` parameters until X15 gives the ROM maxima; every value must fit the largest step at S = 1.
- Deciding experiment: X15 (ROM rows and blocks per step), then X4 (a locality census on the G-C corpus). The early leaf close rate depends on it (LC AUDIT minor 19).

**V-5. HASH constant-start mode for VALUE DIGEST**
- Options:
  - (a) Fold the per-form start state into the absorb row's round-constant columns.
  - (b) No folding. Costs as in O/design.md:200-204.
- Recommendation: (a) if it adds no fixed column.
- Deciding experiment: a HASH prototype with a differential test against `sponge_native`, plus a key-shape check (fixed columns).

**V-6. `input_d` and the schema id in the boundary**
- Options:
  - (a) `input_d` as a boundary word, so that the terminal state opens it.
  - (b) The decider recomputes `init_d` from `(prog_d, input_d)`.
  - (c) `input_d` as a node state field.
- Recommendation: leave this to Area 3. Put the schema/profile id in the capacity tag (0 permutations).
- Deciding step: the Area 3 seal design.

**V-7. Where `vd` is carried** (O/design.md:193: "not measured")
- Options:
  - (a) In the DATA record digest word.
  - (b) With every reference.
- Recommendation: (a), so that a child `vd` is read from a kind-constrained record.
- Deciding experiment: task 4.16 (dissent D-4), measured with the record layout.

**V-8. Profile B transcript shape**
- Options:
  - (a) A Merkle tree over the `C_i` (gemini-area2.md:276).
  - (b) An ordered hash chain (astra-area2.md:520).
- Recommendation: decide at M4 with Area 3. Use one challenge pair, plus a separately labelled `beta` for Horner.
- Deciding experiment: the Profile B benchmark after M4.

**V-9. Profile B soundness accounting for Fiat-Shamir re-derivation**
- The LC AUDIT figure (about 2^-187 per pair) does not include a factor for the number of hash queries a prover can spend grinding commitments. At `Q` queries the bound becomes about `Q x 2^-187` (ESTIMATE).
- Recommendation: state the bound model with `Q` when Profile B is designed.
- Deciding step: a written bound model with a review.

**V-10. Boundary word count `N`**
- The spike has 29 register words. The full machine (fold cells, stack heads and counts, `len`) is larger.
- Recommendation: take the schema from Area 1; Poseidon cost `140 x ceil(N / 2)` rows per leaf (ESTIMATE).

The following owner items touch these kits without being settled here: TOP >= 63 (R-4, M-2: confirmed as needed by the measured 3 appends per step; OWNER to confirm), the PROG canonicality premise and the `decl_schema_d` normal form (dissent D-2), and exhaust kind 7 against profile `/1`.


## 5. OpenSpec outline: change `sherlock-modular-leaf-20261111`

The change does not exist yet (`openspec/changes/sherlock-modular-leaf-20261111/` is absent). It is separate from `sherlock-vm-consensus-20261107`, which keeps the requirements (SRC, SVM, GP, SAS) and its open items. OpenSpec 1.17.0; `openspec validate sherlock-modular-leaf-20261111 --strict` must pass with no warning (night-plan D2). `gemini-area3.md:1137` names the change `sherlock-modular-chip-gp-20261112`; the name of the night plan is used.

### 5.1 `proposal.md` scope

- Why: the leaf of `O/design.md` 2.7 is one circuit description. This change specifies it as module kits with frozen interfaces, so that one agent builds and checks one kit, and so that the memory backend and the boundary commitment can be swapped behind one seam each.
- Amendments of the governing specification `sherlock-vm-consensus-20261107` that this change needs. Each is an explicit task of section 5.3 and is made in that change, not silently here: SRC-021 and the Init table to the one-arena field list (AD-18, OWNER); the input-admission open item of `O/design.md:194` (AD-17, OWNER); `O/design.md:80` to "one schema, two lanes"; D2, D3, D6 and D7 of `O/design.md:95-100` replaced by the dense memory and hash-stack rules, with the compound stack step; SRC-014 `S_MAX` in the relation bytes (already required; the code change is NODE's); SRC-007 and the contract templates for universal program acceptance and collection opening (N-11, OWNER); the semantic reference for the full language (AD-21, OWNER); a consume rule only if AD-20 chooses it.
- What changes: one capability spec per kit (26 kits), plus one spec per CORE rule family; the bus schema and ports (frozen in their BabyBear encoding, FP-1); Profile A (dense tail-buffered Merkle memory with 8-record cache lines, Poseidon boundary digest) as the built path; hash-chain FRAMES and TASKS stacks; `sherlock-value/1` unchanged; continuations tier 1 and retained-memory mode 1; the GP-2, GP-3 and GP-4 hooks; the Lean obligations per kit; replacement of D2, D3, D6 and D7 of `O/design.md:95-100` by the dense memory and hash-stack rules.
- Out of scope: Profile B (ZmLogUp, Horner) beyond its seam and a benchmark-only spec; continuation mode 2; semantic profile `/2`; any change of GP-1 to GP-4 text; time and throughput gates (they stay in SRC-011 and SRC-015 rows after measurement).
- `design.md` of the change is this file as decided after the B2 audit and experiment C4, with the measured tables.
- Impact: `crates/mori-chip`, `crates/mori-recursion`, `models/sherlock/quint/`, `clean/fork/Clean/Sherlock/`; the per-program leaf keys change with the leaf AIRs under `key_root`; the universal internal-recursive, `L` and wrap keys do not (section 1.6).

### 5.2 Capability specs, one per kit

Each requirement statement stays under 500 characters; scenarios carry the examples. Each scenario names the MockProver test or mutant, the Quint invariant or the Lean obligation (ids of the consolidated table in section 2) that is its evidence.

Scenario wording follows the mutant protocol of the conventions: a scenario states the malicious witness that the correct circuit refuses, and where a control is named, the weakened circuit that accepts the same witness. No scenario says that removing a constraint "is refused" (B2 finding 19). Each evidence item is the property that the test or invariant actually checks.

`specs/sherlock-chip-bus/spec.md`
- Requirement: One tuple schema and a fixed chip registry.
  - WHEN a request names an unassigned non-zero chip id (5 or above) THEN no provider row matches and the lookup refuses it -> `bus_unassigned_chip`; Quint `unassignedChipUnserved`. (`noZeroChipProvider` is evidence only for chip id 0.)
  - WHEN a request row offers its own tuple THEN it is not served -> `bus_request_serves_itself`, `bus_second_lane_unchecked`.
- Requirement: Provider rows pin a non-zero chip id and padding is the zero tuple.
  - WHEN an inactive provider slot keeps a payload THEN MockProver refuses -> `bus_inactive_slot_leaks_payload`, `bus_zero_chip_payload`.
- Requirement: One tagged fixed table serves RANGE and DEC with a fixed tag.
  - WHEN a DEC query is shaped to match a RANGE row THEN it is refused -> `tagged_dec_query_matches_range_row`; control `tagged_tag_from_advice`; Quint `taggedNoAlias`.
- Requirement: A disabled DEC slot keeps the fixed DEC tag and matches only the idle row.
  - WHEN a full leaf and a leaf with disabled slots are proved under one key THEN both are accepted -> `tagged_full_and_partial_leaf_same_key`; WHEN a disabled slot carries a non-zero control word THEN refused -> `tagged_disabled_dec_slot_nonzero_val`; Quint `decIdleOnlyWhenDisabled`.
- Requirement: Tuple levels and port dependencies have no cycle.
  - WHEN the module list is instantiated THEN every requested tuple has a lower level than every provided tuple, and the port dependency graph is acyclic -> Quint `levelOrder`, `portOrderAcyclic`; Lean P2, O1, O42, O43.

`specs/sherlock-chip-range/spec.md`
- Requirement: A checked cell is below its static width.
  - WHEN a value of 2^128 or `p - 1` is checked at 128 bits THEN refused -> `range_value_2_128`, `range_field_wrap`; Lean O3.
- Requirement: The decomposition is tied to the source cell.
  - WHEN the limbs decompose a value other than the source cell, or the top limb exceeds the exact width, THEN the circuit refuses the witness -> `range_source_copy_removed`, `range_top_pad_unlinked` (each names the constraint whose control circuit accepts the same witness).

`specs/sherlock-chip-alu/spec.md`
- Requirement: The tuple is `(ALU, op + 8 flag, x, y, z)` and `(z, flag)` is unique.
  - WHEN a second result is offered for the same inputs THEN refused -> `alu_second_result_for_same_inputs`; Quint `aluArith`; Lean O4.
- Requirement: Operands and results are ranged inside the chip.
  - WHEN `x = p - 5` is offered to ADD or `x = p - 1` to LT THEN refused -> `alu_operand_range_missing`, `alu_lt_field_wrap`.
- Requirement: Overflow is provable with flag 1.
  - WHEN an overflowing ADD or MUL is proved with `flag = 1` THEN accepted, and with `flag = 0` refused -> `alu_overflow_unprovable`, `alu_mul_high_half_dropped`, `alu_mul_c1_overflow`.
- Requirement: The op list is the reference machine's.
  - WHEN an op outside the list is requested THEN the lookup refuses it -> `bus_key_namespace_alias` (ALU case).

`specs/sherlock-chip-dec/spec.md`
- Requirement: Exactly one enabled rule per enabled slot, in reference priority.
  - WHEN two rules are selected, or none with `enable = 1` THEN refused -> `dec_two_rules_selected`, `dec_enabled_without_rule`, `dec_priority_swapped`; Quint `decDispatch`.
- Requirement: Disabled and extension rules have no row.
  - WHEN a disabled or `/2` rule pre-state is queried THEN refused -> `dec_disabled_rule_query`, `dec_extension_rule_query`; Lean O6.
- Requirement: Every unpacked control part is bounded.
  - WHEN a part is changed and compensated modulo the field THEN refused -> `dec_wrap_forgery`; Lean O5.

`specs/sherlock-chip-hash/spec.md`
- Requirement: The pinned sponge is unchanged for digests that leave the leaf.
  - WHEN the circuit sponge is compared with the pinned stdlib on even and odd lengths THEN equal -> differential vectors; `hash_value_digest_given_capacity_tag`.
- Requirement: Internal hashes carry a capacity domain word from one registry.
  - WHEN a node call is given the record domain THEN refused -> `hash_domain_swapped`; Quint `hashProtocol`; Lean O7.
- Requirement: Inputs and outputs are copy-connected; no provider row has a free output.
  - WHEN a caller's cell differs from the sponge input it is said to feed, or a digest cell differs from the final state, THEN the circuit refuses the witness -> `hash_input_copy_removed`, `hash_output_copy_removed` (controls: the circuits without that copy accept it).

`specs/sherlock-chip-core/spec.md`
- Requirement: Each enabled slot is one reference step; disabled slots are the identity.
  - WHEN a register outside the write set changes, or a disabled slot changes state THEN refused -> `core_unlisted_register_changed`, `core_disabled_state_changed`; Quint `processorCore`; Lean O8, P4.
- Requirement: Enables form a non-empty prefix and heights stay below 2^64.
  - WHEN `steps = 0`, the prefix is broken, or `h` reaches 2^64 THEN refused; WHEN `h = 2^64 - 1` THEN accepted -> `core_zero_step_leaf`, `core_enable_nonprefix`, `core_height_wrap_2_64`, `core_height_edge`.
- Requirement: Every read is below the current length with the expected kind.
  - WHEN a read is at `len_cur`, of a later record, or with a free kind THEN refused -> `core_future_read`, `core_read_kind_unconstrained`, `core_read_address_free`.
- Requirement: Appends are dense and one to one with window rows (conditions A1 to A4).
  - WHEN an append has no window row, its bus request is off, or closing advances the length THEN refused -> `core_append_without_window_row`, `core_append_request_disabled`, `core_close_advances_length`; Lean O45.

`specs/sherlock-chip-core-<family>/spec.md`, one per rule family (the publications, the nine helper groups Scalar to Match, the legacy sites, the entry rules, the ADT Build group, the ADT Plan group). Outline of each, generated from the family's `RuleContract`s:
- Requirement: Each rule of the family is its reference transition.
  - WHEN the rule fires on each branch of its reference code THEN the slot's post state equals the reference post state -> one positive per branch; the REFERENCE BRIDGE differential.
  - WHEN a register outside the rule's write set changes, or a write has another value THEN refused -> one mutant per written register class.
- Requirement: The rule's effect bounds hold on every branch.
  - WHEN the rule runs its maximal branch at one step per leaf THEN it is provable (appends, reads, ROM rows, stack step within the `LeafShape`) -> a per-rule maximal-step positive; the row of `FD/maxima/rule_maxima.md`.
- Requirement: The rule's charge and outcome behaviour is the reference's.
  - WHEN fuel or a counter is charged differently, or a failure branch publishes another outcome THEN refused -> one mutant per charge and per failure branch.
- Requirement: A step is independent of its incoming scratch.
  - WHEN any value is placed in the scratch lanes at a slot entry THEN acceptance and every public lane are unchanged -> `core_scratch_incoming_free`; Lean O10.
  - WHEN a rule plan uses a scratch lane before defining it in the same slot, also if it defines it later, THEN configuration refuses the plan -> `core_plan_scratch_use_before_def`, `core_plan_reads_pre_scratch`; controls `ctl_scratch_read_then_write`, `ctl_scratch_cross_slot_link` accept two witnesses with different `post_d`.
- Requirement: `rules_d` is the SRC-001 hash of the S slot rule lanes.
  - WHEN lane 5 is not that hash, the lanes are reordered, or a disabled slot has a non-zero lane THEN refused -> `core_rules_d_forged`, `core_rules_d_lanes_reordered`, `core_rules_d_disabled_lane_nonzero`; WHEN S = 1 THEN the same hash form holds -> `core_rules_d_reference_vectors`, `core_rules_d_s1_shortcut`; Lean O44.
- Requirement: Every record and ROM row reaches its consumer through the read port.
  - WHEN a read uses `len_post` in place of the running length, or a ROM row is unranged, THEN refused -> `core_read_len_post_instead_of_running`, `rom_row_overflow_alias`.
- Requirement: A compound stack step is one owner event.
  - WHEN a rule rewrites the top task and pushes a child THEN one slot proves it and `steps` counts one -> `core_rewrite_then_push_one_slot`.
- Requirement: Every rule's maxima fit an empty leaf.
  - WHEN a `LeafShape` is below a rule maximum THEN the shape check refuses it before key generation -> `leafshape_below_rule_max_refused`; the maximal step proves at one step per leaf.

`specs/sherlock-chip-stacks/spec.md`
- Requirement: One stack step is an optional rewrite followed by none, push or pop.
  - WHEN a rewrite and a push happen in one step THEN the rewritten top is spilled and the child becomes the top -> `stack_rewrite_then_push_one_step`; refused: `stack_compound_spills_unrewritten_top`, `stack_compound_child_not_installed`; Quint `compoundStepIsOneEvent`; Lean O11, O46.
- Requirement: The count is the rule; an empty stack has the fixed non-zero constant.
  - WHEN `count = 0` with `below = 0` or a non-zero top, or `count = 1` with `below != EMPTY`, THEN refused -> `stack_empty_below_not_empty_const`, `stack_empty_nonzero_top`, `stack_count_one_below_nonempty`; Quint `belowEmptyAtCountLe1`.
- Requirement: A restored record is the record spilled at that depth.
  - WHEN a restored record or its depth is forged THEN refused -> `stack_restored_record_forged`, `stack_count_unchecked`; Lean P6, O11.
- Requirement: Frame and task chains are domain separated.
  - WHEN a task chain value is used as a frame chain THEN refused -> `stack_frame_task_domain_swapped`.

`specs/sherlock-chip-mem/spec.md`
- Requirement: Dense append: `len` advances only by real appends and both lengths are below `2^(TOP+3)`.
  - WHEN a window row reuses an address, closing advances `len`, or `len` wraps THEN refused -> `address_reuse_window_row`, `close_advances_len`, `counter_wrap_2_64`, `counter_field_wrap`; Lean P8, O12.
- Requirement: The key `8 addr + kind` is injective and padding is off the bus.
  - WHEN kind 9 is offered with a consistent key THEN refused -> `kind_out_of_range`, `pad_row_on_bus`; Quint property 4 of `denseMemory`.
- Requirement: Window and old-memory providers are disjoint.
  - WHEN a window address is sent to the cold path THEN refused -> `cold_route_forged`.
- Requirement: The state after a run does not depend on the partition.
  - WHEN 37 appends are cut in four ways THEN `(len, root, tail_d)` is equal -> `c1_partition_independence`; Quint `invRefine` of `dense.qnt` (`src023PartitionIndependenceTest`); Lean P7, P9, P10 for leaves of at most 8 appends, O14 for the window's capacity.
- Requirement: A leaf holds at most `8 W + 7 - r_pre` new records, and every single step fits.
  - WHEN a `W = 1` leaf appends 9 to `15 - r_pre` records THEN accepted; WHEN the steps of one leaf sum to more THEN unsatisfiable -> `mem_w1_leaf_9_to_15_appends`, `mem_leaf_sum_exceeds_window`; Lean O47, O15.

`specs/sherlock-chip-membackend/spec.md`
- Requirement: One seam: `persist`, `cold_read`, `open_line`, opaque token.
  - WHEN the MEM harness is compiled against a second backend THEN `WindowCfg` and CORE are unchanged -> the seam test.
- Requirement: The tail is canonical and every leaf opens it.
  - WHEN a leaf imports a forged or non-canonical tail THEN refused -> `forged_tail_across_leaf_boundary`, `non_canonical_tail`; Lean P12.
- Requirement: A flush happens iff a segment boundary is crossed, into an empty slot, at every `TOP >= 1`.
  - WHEN a flush is skipped, spurious, or onto an occupied slot THEN refused -> `flush_skipped`, `flush_spurious`, `address_reuse_occupied_slot`, `empty_seg_forgery`, `c1_gap_top1_empty_seg_forgery` (TOP = 1); Quint mutant `top1_walk_empty_shadowed`; Lean P13, P14, O14.
- Requirement: An old read opens the record and its kind against the pre-state root.
  - WHEN the value or the kind of an old record is forged THEN refused -> `cold_value_forged`, `wrong_kind_cold`, `cold_read_uses_running_root`; Lean O16.

The specs of the other kits are outlined by the helpers that digested them; they follow in 5.2.1 and 5.2.2. The kits INIT, REFERENCE BRIDGE, HOST CHECKPOINT STORE and the GP-3 general verifier follow in 5.2.3.


#### 5.2.1 PAGE/ROM block cache, ZmLogUp, VALUE DIGEST, BOUNDARY COMMITMENT

`specs/sherlock-chip-page-cache/spec.md`

- Requirement: Heap cache lines open against the pre-state root
A heap line SHALL authenticate its 8 records as `H_REC` leaves of `Tree8` and a TOP-level walk to the leaf's `pre.root`. Its walk bits SHALL be the decomposition of `blk`, and `blk` SHALL be below `q_pre`. It SHALL serve `(MEM, 8 (8 blk + j) + kind_j, a, b, c)` from the hashed cells, with `j` fixed by the row.
  - Scenario: WHEN a line authenticates a segment flushed in the same leaf THEN MockProver refuses (`line_uses_post_root`, `line_blk_ge_qpre`).
  - Scenario: WHEN a served cell differs from the hashed cell, or the walk bits are not `blk` THEN MockProver refuses (`line_provider_not_hashed`, `line_walk_bits_not_blk`); Lean line soundness (open).

- Requirement: Disabled, duplicate and unordered lines
A line with `en = 0` SHALL serve the zero tuple and turn off its root equality. Duplicate or unordered lines SHALL need no extra constraint.
  - Scenario: WHEN `en = 0` and a nonzero tuple is served THEN MockProver refuses (`line_disabled_nonzero`).
  - Scenario: WHEN two lines open one block THEN the leaf is accepted with equal records (`ok_duplicate_and_unordered_lines`) and refused with different records (`line_dup_block_two_values`).

- Requirement: Lines change no boundary value
Opening heap or ROM lines SHALL NOT change `(len, root, tail_d)`, `pre_d`, `post_d` or the public statement.
  - Scenario: WHEN a leaf opens 0, 1 or `K_page` lines THEN the post token and digests are equal (`line_boundary_invariance`); Quint `BoundaryIndependentOfLines`; Lean `PageCache.lean` boundary invariance (open).

- Requirement: ROM lines under prog_root
ROM SHALL have its own chip id. Its key SHALL be `tbl 2^D_row + row`, with bits from the walk. Its leaf and node tags SHALL differ from the heap tags and carry heights. Padding SHALL be canonical. CORE's ROM read port, the one requester, SHALL take `row` only as a ranged cell.
  - Scenario: WHEN a row of one table is served under another table's key, or the requested row overflows `D_row` THEN MockProver refuses (`rom_tbl_row_alias`, `rom_row_overflow_alias`).
  - Scenario: WHEN a heap node is spliced into a ROM walk THEN MockProver refuses (`rom_heap_domain_splice`); Quint `RomKeyInjective`.

- Requirement: Line capacity covers every rule
`K_page + n_cold`, `K_rom` and the lane count SHALL be at least the per-step maxima of the per-rule table, derived over every branch of every rule. A shape below them SHALL be refused before key generation.
  - Scenario: WHEN a `LeafShape` is below a rule maximum THEN the shape check refuses it (`leafshape_below_rule_max_refused`), and the maximal step proves at S = 1 (`ok_max_rule_step_at_s1`).

`specs/sherlock-chip-zmlogup/spec.md`

- Requirement: Profile B is benchmark-only
The ZmLogUp backend SHALL be built only behind a feature, not before M4. It SHALL NOT be described as supporting stop and resume of a run. It SHALL be listed as changing the leaf statement, node, seal and continuation modules.
  - Scenario: WHEN a checkpoint resets the memory sum THEN the seal refuses (`checkpoint_resets_debt`).

- Requirement: Every term is bound to the committed instance
Every write, read and multiplicity cell SHALL be equal to its cell in the committed instance column, and the challenges SHALL be scalar public inputs equal to the hash of `d_root`.
  - Scenario: WHEN a multiplicity, activity bit or tuple is not committed THEN the test refuses (`multiplicity_uncommitted`, `activity_uncommitted`, `tuple_uncommitted`).
  - Scenario: WHEN the challenges are not derived from `d_root` THEN the seal refuses (`challenge_not_derived`); Lean `WriteOnceLogUp.lean` (open).

- Requirement: Rational terms are well formed
An active entry SHALL have a nonzero denominator with `(alpha - E) inv = 1`. An inactive entry SHALL contribute zero.
  - Scenario: WHEN the inverse is free or an inactive term is nonzero THEN MockProver refuses (`inverse_unconstrained`, `inactive_term_nonzero`, `zero_denominator_active`).

- Requirement: Causality and single writes
Every read SHALL keep `addr < len_cur`, and every address SHALL be written once.
  - Scenario: WHEN a read is balanced by a later write, or an address is written twice THEN MockProver refuses (`read_before_write_balanced_later`, `address_written_twice`); Quint `writeOnceLogup.qnt`.

- Requirement: Imported tail rows are debits
Every imported window row and every old-record provider row SHALL be a read entry of the balance. Backend capabilities SHALL be fixed at configuration time.
  - Scenario: WHEN the second of two leaves imports a tail record that the first did not write, with every commitment binding intact THEN the seal refuses (`zm_imported_tail_forged`; the control without the debit accepts it).

`specs/sherlock-value-digest/spec.md`

- Requirement: sherlock-value/1 bit for bit
VALUE DIGEST SHALL compute exactly the preimages of the seven forms, the constructor fold step and the outcome wrappers of the design. It SHALL use the zk-stdlib sponge with the length in the start state and no capacity domain tag.
  - Scenario: WHEN every form and outcome is digested THEN the circuit equals the reference written from the specification text (`vd_differential_every_form`).
  - Scenario: WHEN a capacity tag is added, or one start state is used for all lengths THEN the differential fails (`vd_capacity_tag_added`, `vd_single_prefix_constant`).

- Requirement: Whole-sponge slots with copied inputs
Each digest SHALL be one sponge in HASH slots with copy-constrained inputs. Constant chunks SHALL be precomputed per preimage length and form. No chain of 2-input bus hashes SHALL be used.
  - Scenario: WHEN a 2-input bus chain replaces a sponge THEN the differential fails (`vd_bus_rehash_chain`); Lean constant-start equivalence (open).

- Requirement: Records and ROM rows only through the read port
VALUE DIGEST SHALL issue no bus request. Every child record and ROM row SHALL be an `AuthRec` or `AuthRom` token of CORE's read port.
  - Scenario: WHEN a child record is at or above the running length, at another address than the held reference, or a ROM row index overflows `D_row` THEN refused (`vd_child_future_record`, `vd_child_wrong_reference`, `vd_rom_row_overflow`).

- Requirement: Bounded value opening
A value held by digest SHALL be opened one node per call, and a constructor field SHALL be usable only after its opening fold reaches the digest.
  - Scenario: WHEN a field is used before the fold completes, or an opened child digest differs THEN refused (`vd_open_field_used_before_completion`, `vd_open_node_wrong_child`).

- Requirement: Digests from authenticated children
The stored `vd` SHALL be computed from the tag and the child `vd` lanes of kind-constrained records. `b` SHALL be boolean, UInt SHALL be below 2^128, and the constructor fold SHALL advance one field per step and complete at the arity.
  - Scenario: WHEN a host digest, a wrong child digest or an unconstrained child kind is supplied THEN MockProver refuses (`host_supplied_vd`, `wrong_child_digest`, `child_kind_unconstrained`).
  - Scenario: WHEN the fold cursor skips or the accumulator resets across a leaf THEN MockProver refuses (`fold_cursor_skipped`, `fold_accumulator_reset`).

- Requirement: Layout independence
Constants and data of one value, and shared and unfolded structure, SHALL have equal `vd`.
  - Scenario: WHEN one value is stored at relocated addresses or shared THEN `vd` is equal (Quint `VdRelocationInvariant`, `VdSharingEqual`); Lean layout independence (open).

`specs/sherlock-chip-boundary/spec.md`

- Requirement: Complete keyed boundary schema
The boundary SHALL serialize every future-readable CORE register, the fold cells, the FRAMES and TASKS heads and the arena `(len, root, tail_d)` in fixed keyed order. Scratch is excluded only because a step is independent of its incoming scratch.
  - Scenario: WHEN a word is omitted or reordered between leaves THEN stitching refuses (`boundary_word_omitted`, `boundary_word_reordered`, `boundary_tail_omitted`, `stack_head_omitted`).
  - Scenario: WHEN incoming scratch is set to any value THEN `pre_d`, `post_d` and acceptance are unchanged (`core_scratch_incoming_free`).

- Requirement: Poseidon header and body commitment is the default
`pre_d` and `post_d` SHALL be the header digest of section 1.11, FP-5, over the terminal kind, the terminal payload, `input_d`, `deps_d`, `view_root` and a body digest of the other schema words, in one pass with no challenges. They SHALL be published in the seven leaf lanes with the genesis gate `h_pre == 0`.
  - Scenario: WHEN two leaves stitch THEN `post_d` equals the next `pre_d` iff all words match (`ok_boundary_poseidon_two_leaf_stitch`); Lean O29.
  - Scenario: WHEN a header word is hashed only inside the body, or `body_d` is not the body sponge output THEN refused (`header_word_moved_to_body`, `header_body_d_unlinked`).

- Requirement: Typed header opening for the seal (NODE SEAL and SEAL-WITH-DEPS only)
`open_state_header` SHALL return the four header words and `body_d` of the state that an authenticated `post_d` commits, with one library sponge.
  - Scenario: WHEN the leaf header digest and the library sponge are computed on the same words THEN they are equal (`boundary_header_leaf_vs_std`); WHEN an opened word differs from the committed word THEN refused (`node_seal_header_word_forged`); Lean O48.

- Requirement: Partition independence of boundary values
At every cut shared by two partitions, the Poseidon boundary values and the public statement SHALL be equal.
  - Scenario: WHEN a trace is cut as full leaves, single steps and mixed leaves THEN digests at common cuts and the public statement agree (`ok_boundary_partition_three_cuts`); Lean `partition_independent_flush`, `cuts_agree` (proved in C3 for leaves of at most 8 appends), O14 (open) for the window's capacity.

- Requirement: Horner boundary only with Profile B
A Horner boundary SHALL be offered only with the ZmLogUp backend. Its words SHALL be committed before `beta` is derived.
  - Scenario: WHEN boundary words are not committed, or `beta` is free THEN the test refuses (`horner_words_uncommitted`, `horner_beta_free`, `horner_two_word_compensation`); Lean Horner soundness (open).

#### 5.2.2 NODE INTEGRATION, L AND WRAP, CONTINUATION, GP hooks


Each requirement is to stay under 500 characters; scenarios carry examples.

`specs/sherlock-node-integration/spec.md` (G:333 name)
- Requirement: Segment merge links adjacent segments.
  - Scenario: WHEN two SEGs with equal `prog_d`, `L.post_d == R.pre_d` and `L.h_hi == R.h_lo` are merged THEN the node outputs `(prog_d, L.pre_d, R.post_d, L.h_lo, R.h_hi)` -> `pos_node_merge_adjacent`; Quint `SegCoversExactInterval`; Lean NODE-L1.
  - Scenario: WHEN `L.post_d != R.pre_d` or the intervals gap or overlap THEN the node is unsatisfiable -> `node_broken_boundary`, `node_gap_interval`, `node_overlap_interval`.
- Requirement: Heights and steps are range-checked (SRC-004).
  - Scenario: WHEN `h = 2^64 - 1` THEN accepted (`pos_node_height_2p64_minus_1`); WHEN `h = 2^64`, `steps = 0` or `steps = S_MAX + 1` THEN unsatisfiable -> `node_height_wrap`, `node_steps_zero`, `node_steps_gt_smax`.
- Requirement: Seal requires genesis and a terminal state and erases physical fields.
  - Scenario: WHEN a SEG with `h_lo == 0` and a Halted, Rejected or Exhausted state is sealed THEN the SET leaf binds `claim_d` and `n_steps` and no `pre_d`, `post_d` or memory root -> `pos_node_seal_all_outcomes`; Quint `SealRequiresGenesisAndTerminal`, `SealErasesPhysicalAddresses`; Lean NODE-L2.
  - Scenario: WHEN the state is Running or `h_lo != 0` THEN unsatisfiable -> `node_seal_running`, `node_seal_nonzero_hlo`.
- Requirement: Child keys are routed by one selector; disabled slots are neutral.
  - Scenario: WHEN a child proved under one key is routed to another key's coefficients THEN unsatisfiable -> `node_wrong_key_destination`, `node_selector_repr_mismatch`; Lean NODE-L3.
  - Scenario: WHEN a disabled slot carries a non-neutral accumulator THEN unsatisfiable -> `node_disabled_slot_garbage`, `node_genesis_non_neutral`.
- Requirement: The node relation serializes `S_MAX` and every keyed capacity.
  - Scenario: WHEN a relation file lacks `S_MAX` THEN deserialization is refused -> host refusal test `node_relation_missing_smax`.

`specs/sherlock-decider/spec.md` (G:568 name)
- Requirement: The run decider publishes only `[C_P, input_pub, outcome_pub]` (section 1.11, FP-6).
  - Scenario: WHEN a sealed Halted, Rejected or Exhausted run is decided THEN the public inputs are exactly the four values and no state digest -> `pos_decider_all_outcomes`; Quint `NoStateDigestOnChain`; Lean DEC-L1.
  - Scenario: WHEN a SEG root, Running or terminal, or a forged SET head is offered to the run entry THEN unsatisfiable -> `decider_seg_root_as_run`, `decider_set_head_forged`.
- Requirement: Every carried obligation reaches the pairing.
  - Scenario: WHEN the root carry is dropped or the pairing skipped THEN unsatisfiable -> `decider_drop_root_carry`, `decider_skip_pairing`.
- Requirement: Replay is refused by `claim_d` only.
  - Scenario: WHEN the same run is proved under a second leaf partition and submitted THEN the second call is refused -> integration test (SRC-023, row A38); Quint `PartitionReplayRefusedByClaimD`; Lean DEC-L1.
- Requirement: The collection decider publishes `[reg_root, set_root, shape_d]` and no claim count.
  - Scenario: WHEN three sealed runs are joined and decided THEN each claim opens from `set_root` -> `pos_decider_collection_three_runs`; Quint `EveryJoinedClaimOpens`.
  - Scenario: WHEN one claim has two paths in `set_root` THEN it settles at most once -> kit LEDGER CONTRACT (`specs/sherlock-ledger-contract`), Quint `DiamondAncestorOpenedAtMostOnce`; Lean LDG-L1.
- Requirement: The decider adds no range-table tag and stays under its row gate.
  - Scenario: WHEN `cost_model` runs on both deciders THEN rows <= 1,000,000 and table rows do not grow -> `pos_decider_table_rows`; cost-model refusal `decider_new_range_tag`.

`specs/sherlock-continuation/spec.md` (G:799 name)
- Requirement: Tier 1 folds leaves by segment merge in any bracketing.
  - Scenario: WHEN leaves finish out of order THEN the root SEG interval and the sealed statement are unchanged -> `pos_tier1_stack_bound`; Quint `OutOfOrderWorkerCompletionPreservesSegInterval`; Lean NODE-L1.
- Requirement: Mode 1 stop and resume gives the same claim.
  - Scenario: WHEN the allocating GP-001 acceptance run is stopped once and resumed THEN `prog_d`, `input_d`, `outcome_d`, `n_steps` and `claim_d` equal the uninterrupted run -> `pos_mode1_stop_resume_allocating_run`; Quint `ResumeProducesIdenticalClaimD`; Lean CONT-L1.
  - Scenario: WHEN fuel, input or height is changed in the suffix THEN the suffix cannot join the prefix -> `cont_refresh_fuel`, `cont_change_input`, `cont_reset_height`; WHEN a memory record that the suffix reads is changed with restore validation off THEN the leaf is unsatisfiable -> `cont_tampered_read_cell_unvalidated`.
- Requirement: A checkpoint is never a run or a claim.
  - Scenario: WHEN a checkpoint SEG is passed to the decider or to a consumer THEN refused -> `cont_publish_as_run`, `cont_ckpt_as_gp2_claim`; Quint `CheckpointRejectedBySealAndGP2`; Lean CONT-L2.
- Requirement: The resume refuses a different key set.
  - Scenario: WHEN the checkpoint manifest differs from the key manifest THEN refused before proving -> host refusal test `cont_mismatched_key_manifest`.

`specs/sherlock-claim-consumption/spec.md` (name proposed here)
- Requirement: Only sealed claims are consumed.
  - Scenario: WHEN program B consumes A's Halted claim from a verified SET THEN B seals with dependencies discharged -> `pos_gp2_b_consumes_a_halted`; Quint `DepsDischargedIffVerifiedSet`; Lean GP2-L1.
  - Scenario: WHEN a SEG or a state digest is offered as a claim THEN unsatisfiable -> `claim_consume_unsealed_segment`; Quint `SegmentNeverConsumed`.
- Requirement: A used value is opened under `sherlock-value/1`.
  - Scenario: WHEN the value opening is skipped or a Rejected claim is used as a value THEN unsatisfiable -> `claim_skip_value_opening`, `claim_rejected_as_value`.
- Requirement: A consumer cannot seal with an undischarged dependency.
  - Scenario: WHEN the opened `deps_d` differs from the chain of the discharge list, in content, order or repetition THEN unsatisfiable -> `claim_seal_undischarged_deps`, `claim_drop_dependency`, `claim_chain_reordered`, `claim_chain_dedup_one_step_for_two`; Lean GP2-L3.
- Requirement: A claim is discharged from any verified SET that contains it.
  - Scenario: WHEN the producer's claim is offered as its own sealed proof, inside a JOIN of either bracketing, or inside a DEP cover THEN the consumer seals -> `pos_gp2_claim_from_join_left_and_right_bracketing`, `pos_gp2_claim_from_dep_cover`; Quint `DischargeIndependentOfProducerShape`; Lean GP2-L2.
  - Scenario: WHEN a sub-root not reached from the verified root, or a JOIN root as a head claim, is offered THEN unsatisfiable -> `claim_sub_root_unrelated`, `claim_head_of_join`.
- Requirement: The input link binds the consumer's input to the producer's Halted value.
  - Scenario: WHEN the producer is Rejected, or the value digests differ THEN unsatisfiable -> `claim_input_link_rejected_producer`, `claim_input_link_value_mismatch`.

`specs/sherlock-foreign-registry/spec.md` (name proposed here)
- Requirement: A foreign proof enters only through a registry entry that binds its full verification interpretation.
  - Scenario: WHEN a Sherlock claim and a Compact claim are aggregated against `reg_root` THEN both open from the collection -> `pos_gp3_sherlock_plus_compact`; Quint `ForeignClaimRequiresDualRegistryMembership`; Lean GP3-L1.
  - Scenario: WHEN only the key digest matches, or the transcript or SRS differs THEN unsatisfiable -> `foreign_key_digest_only`, `foreign_swap_transcript`, `foreign_swap_srs`.
- Requirement: Both adapter key and inner key are registered.
  - Scenario: WHEN the inner key is absent from `reg_root` THEN unsatisfiable -> `foreign_unregistered_inner_vk`, `foreign_unregistered_adapter`.
- Requirement: A foreign field becomes a value digest only through the adapter relation.
  - Scenario: WHEN an adapter relabels a foreign field as `vd` THEN unsatisfiable -> `foreign_value_relabel`.

`specs/sherlock-proof-shape/spec.md` (name proposed here)
- Requirement: Join is total at any depth with no claim count.
  - Scenario: WHEN one claim feeds two consumers at depth >= 8 and the branches are joined THEN accepted with the same final proof size as a chain and a tree -> `pos_gp4_diamond_reuse` and the real-proof acceptance test (GP-004, row A36); Quint `DiamondDagDepth64NoOverflow`; Lean GP4-L1.
- Requirement: The shape commitment is address-free and never decides acceptance.
  - Scenario: WHEN a state root enters `shape_d` or `shape_d` is used as a replay key THEN refused -> `graph_state_root_in_shape`, `graph_shape_as_replay_key`; Quint `ShapeNotUsedForAcceptance`.
- Requirement: A node takes two prior proofs plus new work.
  - Scenario: WHEN two enabled prior SEGs and one owner step are proved in one node THEN the node outputs the summed interval -> `pos_gp4_two_prior_plus_step`; `graph_two_children_disable_work`, `graph_work_not_bound_to_child_state`; Lean GP4-L2.
- Requirement: Every child-type and work combination outside the node relation table is unsatisfiable.
  - Scenario: WHEN a SEG and a SET are merged, or owner work runs in a JOIN THEN unsatisfiable -> `graph_unsupported_combination`.
- Requirement: Leaf, foreign and disabled children have canonical shape encodings.
  - Scenario: WHEN a leaf child's shape word is not derived from its lanes, or a disabled slot's is not the constant THEN unsatisfiable -> `graph_leaf_shape_free`, `graph_disabled_slot_shape_not_off`.

#### 5.2.3 Kits INIT, REFERENCE BRIDGE, HOST CHECKPOINT STORE, GP-3 general verifier

`specs/sherlock-init-admission/spec.md`
- Requirement: At `h_pre = 0` the pre state is `Init(prog_d, input_d)`, field by field.
  - Scenario: WHEN any Init field differs (control, fuel, counters, stack heads, memory, `deps_d`) THEN refused -> `init_wrong_control`, `init_fuel_differs_from_input_d`, `init_stack_head_not_empty_const`, `init_memory_not_genesis`, `init_deps_not_empty`; Quint `GenesisStateIsInit`; Lean O49.
- Requirement: The genesis selector is `is_zero(h_pre)`.
  - Scenario: WHEN `h_pre != 0` THEN INIT constrains nothing (`init_non_genesis_leaf_any_state`); controls `ctl_init_gate_from_witness_flag`, `ctl_init_supplied_digest_only` accept a wrong genesis state.
- Requirement: `prog_d` and `input_d` are opened and bound.
  - Scenario: WHEN `prog_root` is not the root inside `prog_d`, or a fuel limb is 2^64 THEN refused -> `init_prog_root_not_opened`, `init_fuel_limb_2_64`, `init_input_d_not_opened`.
- Requirement: The genesis memory is canonical (condition C4).
  - Scenario: WHEN the genesis token is not the canonical state of the genesis records THEN refused -> `init_memory_not_genesis`; Lean O50.
- Requirement: Input admission scans nothing and bounds nothing.
  - Scenario: WHEN a step uses an input node that was not opened against `val_d` THEN refused -> `init_input_node_unauthenticated`; the form is AD-17 (OWNER), measured by X11.

`specs/sherlock-reference-bridge/spec.md`
- Requirement: The semantic reference covers the full language.
  - Scenario: WHEN the corpus runs, including a constructor of more than 8 fields and a structured input THEN the coverage report has zero gaps -> `bridge_corpus_zero_gaps`, `bridge_wide_constructor_across_leaf`, `bridge_structured_input`.
- Requirement: The event map is explicit and total.
  - Scenario: WHEN owner events are mapped to source steps THEN `n_steps` equals the enabled owner-event count -> `bridge_step_count_matches_owner_events`.
- Requirement: A gap fails the gate.
  - Scenario: WHEN a projection gap occurs THEN the harness fails and does not skip the case -> the coverage gate; negative controls with a seeded divergence.

`specs/sherlock-checkpoint-store/spec.md`
- Requirement: A checkpoint holds every witness resource of the suffix, in a versioned codec.
  - Scenario: WHEN a run with a non-empty tail and nested stacks is stopped and resumed THEN later pops and reads succeed -> `ckpt_resume_nonempty_tail`, `ckpt_resume_nested_stacks`, `ckpt_roundtrip_allocating_run`.
- Requirement: Restore validates the whole record against the committed state.
  - Scenario: WHEN any record, stack preimage or register is changed, also one that no later leaf reads THEN restore refuses with a host integrity failure -> `ckpt_tampered_unused_cell`, `ckpt_stack_preimage_changed`, `ckpt_register_changed`.
- Requirement: Host integrity failures are apart from constraint failures, and soundness does not depend on the store.
  - Scenario: WHEN validation is off and a read cell is changed THEN the leaf is unsatisfiable -> `cont_tampered_read_cell_unvalidated`; Quint `CorruptStoreNeverYieldsAcceptedWrongClaim`.

`specs/sherlock-harness/spec.md` (taken from the spec blueprint of `gemini-area3.md`, late text; the harness is test infrastructure, not a kit)
- Requirement: One whole-leaf differential against the semantic reference on every module change.
  - Scenario: WHEN any kit changes THEN the three-partition differential runs with zero coverage gaps -> section 3.2; the REFERENCE BRIDGE coverage gate.
- Requirement: A mutant earns credit only at its recorded rejection stage, with its control.
  - Scenario: WHEN a mutant is refused by a panic, a synthesis error, or at another stage than recorded THEN it fails -> the `MutantSpec` record; WHEN its control circuit refuses the same witness THEN it fails.

`specs/sherlock-foreign-verifier/spec.md`
- Requirement: General foreign verification runs in bounded, authenticated work units.
  - Scenario: WHEN a unit continues a state other than its child's, or skips a proof element THEN unsatisfiable -> `fv_state_forked`, `fv_skip_proof_element`; Quint `UnitsContinueCommittedState`, `ProofStreamConsumedExactlyOnce`.
- Requirement: A claim is emitted only at completion.
  - Scenario: WHEN a claim is taken from an unfinished phase THEN unsatisfiable -> `fv_claim_at_unfinished_phase`; Quint `NoClaimBeforeDone`; Lean GP3-L2.
- Requirement: A proof under another SRS is fully checked in circuit.
  - Scenario: WHEN a foreign proof's obligation, under any SRS, is folded into an accumulator or deferred to `L` or the wrap THEN unsatisfiable -> `fv_other_srs_folded_into_core_acc`; positives `pos_fv_plonk_proof_same_srs` and `pos_fv_plonk_proof_other_srs`.
- Requirement: A later phase reads only the values absorbed earlier.
  - Scenario: WHEN an MSM term uses a commitment other than the one absorbed, or a challenge is read from another schema index THEN unsatisfiable -> `fv_operand_substituted`, `fv_challenge_index_swapped`; Quint `LaterPhaseReadsAbsorbedValue`.

`specs/sherlock-ledger-contract/spec.md` (kit LEDGER CONTRACT)
- Requirement: One contract holds one ledger of settled claims.
  - Scenario: WHEN the same `claim_d` is accepted by `acceptRun` and opened by `openClaim` THEN `runs` holds it once -> `ledger_accept_run_then_open_same_claim_is_noop`; control `ledger_double_settle_two_entries`; Quint `ClaimSettledOnceAcrossEntries`.
- Requirement: A claim is opened in bounded steps from a verified root.
  - Scenario: WHEN a claim sits below more than `K_open` levels THEN successive `descend` calls open it -> `ledger_open_depth_beyond_one_call`; Quint `EveryJoinedClaimOpens`.
- Requirement: `openClaim` inserts a head claim once and refuses non-claims.
  - Scenario: WHEN one claim has two paths THEN it settles once -> Quint `DiamondAncestorOpenedAtMostOnce`; Lean LDG-L1; WHEN an `ADM` or `VIEW` head or a `JOIN` root is offered THEN refused -> `ledger_open_premise_as_claim`, `ledger_open_join_as_claim`.

### 5.3 `tasks.md`

`tasks.md` is the build order of section 3.1: one task per kit (26 kits; the five GP-3 general verifier subkits are five tasks), in groups F, 0 to 4, R, L, 5 to 8, plus:

- one task per freeze prerequisite of section 3.1 that is still open, each with an owner;
- one task per CORE rule family (the families of section 5.2);
- one task per amendment of the governing specification listed in section 5.1;
- one task per prerequisite experiment that is not done (X3 stacks part, X4 to X15);
- the interface freeze; the leaf integration with the whole-leaf harness and the REFERENCE BRIDGE coverage gate;
- the contract and template work, explicit: the run contract template with `prog_d` as call data for universal program acceptance, the claims contract with root acceptance and collection opening, and the template recapture (SRC-020; N-11, OWNER);
- the key cascade (node and decider keys over the new leaf key);
- the Rust baseline task (the workspace clippy gate green under the pinned toolchain);
- the native GP-3 probe and the full GP-3 completion as two tasks;
- one acceptance task per line of the GP acceptance matrix of section 3.3, and the heavy end-to-end qualification.

A task is ticked only on passing evidence: a `scripts/doctorwatson check <ID>` predicate with a receipt that names the positive tests, the mutant count with each control, the Quint verdict file (invariants, bounds, seeds, mutant results), the Lean theorems required for the task with their axiom reports (no `sorryAx`), and the Rust check results. A prototype milestone has its own task and its tick does not tick a GP target. The Profile B spec has one task, "benchmark after M4", which is not part of the completion of the change.

### 5.4 Experiments that must finish first, and what each decides

| Id | Experiment | Decides | Needed before |
|---|---|---|---|
| X1 | Rule inventory and per-rule maximum table over the reference machine, derived from the rule source over every branch (a static scan) and validated by an instrumented corpus run; an observed maximum alone is never a bound. DONE on main (`FD/EXPERIMENTS-MAXIMA.md`, `FD/maxima/`, decisions M-2 to M-4): 120 enabled step rules; at most 3 appends per step; at most one frame operation or one task push or pop with one rewrite, never both stacks; at most 4 distinct heap records read. Not covered: ROM rows and blocks per step (X15), ALU and range requests per rule, the physical cells of a record (AD-11), and a mechanical per-path heap bound for 4 rules | decided: AD-10 (120 rules), AD-1 (`a = 3`, so TOP = 63 with `e = 1`), `w_flush = 1`, `k_page + n_cold >= 4`, single-step fit (O15 input). Still to decide from its remainder: AD-8, AD-9, `K_rom` | the interface freeze (done for what it covers) |
| X2 | Non-interleaving invariant of the one-arena layout. DONE on main (`FD/EXPERIMENTS-MAXIMA.md`, "Non-interleaving check"; `FD/maxima/noninterleave.json`; decision M-1): holds for all 120 rules, no counterexample; premises S1 to S4 mechanical, the induction a code reading, 9,031 slices on 6,668 steps with 0 violations | decided: AD-4 (one arena) | the MEM and MEMORY BACKEND specs (done) |
| X3 | Quint lane for dense memory and for hash stacks. Dense memory: DONE (`qa/gemini-quint/c1-dense/verdict.md`: 34 tests, `invRefine` holds, 23 of 23 mutants caught, one code gap found at `TOP == 1` and fixed in `f36eb714d`; conditions C1 to C4 carried into the kits). Hash stacks with the compound step: open | adoption condition of RL AUDIT finding 8 (new chip models) | the STACKS spec (the MEM and MEMORY BACKEND part is met) |
| X4 | Block-line and ROM-line prototype behind `open_line`: rows per line, a locality census of old reads and program rows on the corpus | L-2 (line against scalar read), the early-close rate, the ROM depth cost (AD-12) | the PAGE/ROM spec |
| X5 | Dense backend wired into `RfLeaf` in place of M3, with INIT, the `rules_d` sponge, the header-and-body boundary, the read port and hash-chain stacks: measured leaf cost at S = 16 (and 64) in seconds per proved step, then node cost and the `L` shape over that leaf key | the shape bounds of section 1.9 and the wrap row gate; AD-6 (S); the +24,016 node rows ESTIMATE; the INIT and `rules_d` estimates | `design.md` of the change |
| X6 | Task 4.18: a constructor of more than 8 fields built across a leaf boundary, measured for linked slices and for contiguous slices in dense memory | AD-5 (D-1) | the CORE rule-family specs that build constructors |
| X7 | Binary node shapes: leaf key direct against a one-leaf lift (N-1); the MERGE-WORK node, two enabled prior proofs plus one real owner step with its memory, stack and value-digest work in one node (N-6; two successive nodes are not this measurement); decider with and without a closing proof (N-5) | N-1, N-5, N-6, N-13 | the NODE INTEGRATION, L AND WRAP and GP-4 specs |
| X8 | Foreign adapter probe: one Compact adapter class and the foreign lift relation | the scope of the GP-3 native path (N-10) | the GP-3 spec |
| X9 | HASH constant-start mode for VALUE DIGEST with a differential test and a key-shape check | V-5 (whether the per-form constants save permutations at no fixed column) | the VALUE DIGEST spec row budget |
| X10 (new, B2 finding 5) | The state header opening inside the NODE seal modes (`L` opens a SET head only (section 1.11, FP-3)): `open_state_header` with the 42-word header sponge of section 1.11, FP-5, against the leaf chip's header digest (`boundary_header_leaf_vs_std`), with node and decider rows and table rows | AD-19 and N-2 (does the seal-time opening fit; else the fallback leaf statement, AD-13) | the BOUNDARY COMMITMENT, NODE INTEGRATION and L AND WRAP specs |
| X11 (new, B2 finding 3) | Input admission forms on the reference machine and a leaf prototype: for by-digest admission, the cost of the `AuthIn` read port per step and of VIEW-WORK per entered record, and whether the reference charges `Literal(x)` cells for an input read; the owner-event count against the reference run of `apply(prog, x)`; a constructor input of more than 8 fields; the loading-prefix form for comparison | AD-17 (the form), then AD-18 (the Init values of fields 23, 24, 26, 27, 34, 35) | the INIT spec (its input part) and the SRC-021 amendment |
| X12 (new, B2 finding 6) | GP-2 discharge: the Quint model `claimConsumption.qnt` of the SET grammar, DESCEND, DL-STEP and both seals with the invariants of the GP-2 hook, run by a Quint lane and re-run by the lead; then rows of each mode in the node | N-3 (the protocol of the GP-2 hook stands or is revised); AD-20 input | the GP-2 spec |
| X13 (new, B2 finding 14) | Reference coverage census: every gap kind of the projection (`UnsupportedGeneric`, `UnmappedRule`, the others) counted over the corpus, and the 18 rules the X1 corpus did not fire | the size and order of the REFERENCE BRIDGE work; AD-21 input | the REFERENCE BRIDGE task; group L |
| X14 (new, B2 finding 15) | General foreign verifier: a costed operation inventory for one PLONK/halo2 verifier (parse, transcript, expression evaluation, MSM steps, final check with the core SRS and with another SRS), and the largest unit that fits the node beside one child verification | the unit budget and schedule of FV-SCHED; AD-22; whether the general path fits the node gate | the GP-3 general verifier spec and its five tasks |
| X15 (new, decision M-3) | ROM rows and ROM blocks per step: log every `view.<table>` row read and every ROM read of selection and value reads, add those sites to the static bound, assert dynamic <= static | `K_rom`; the ROM part of `EffectBounds`; with V-1 the ROM line cost | the PAGE/ROM spec and the `LeafShape` freeze |

State of the freeze blockers: X1 and X2 are done; the remaining freeze prerequisites are the list of section 3.1 (X15 and AD-11 for the leaf shape; the others gate later groups). X3, X4 and X5 are the "memory backend measurement, leaf rows, node rows" that the ARCH-BRIEF addendum names. A proof made with `midnight-proofs` 0.8.3 verified under the ledger's lockfile version is a qualification check that `gemini-area3.md:1179` asks for; it decides whether the prover pin changes before key generation and is not a design input.

## 6. Traceability of the decided inputs, and what was not checked

### 6.1 LC AUDIT section 5: the 28 interface items

| Item | Short form | Kit |
|---:|---|---|
| 1 | heap line opens against `pre.root`, never the running root | PAGE/ROM block cache; MEMORY BACKEND invariant 6 |
| 2 | line leaves are `H_REC(kind, a, b, c)` with capacity tags; heights as in `persist` | PAGE/ROM block cache; HASH domains |
| 3 | provider cells are the hashed cells; row `j` serves `(MEM, 8 (8 blk + j) + kind_j, a, b, c)` | PAGE/ROM block cache |
| 4 | walk bits are the TOP-bit decomposition of `blk` | PAGE/ROM block cache |
| 5 | `en` boolean; disabled line serves zero and its root equality is off | PAGE/ROM block cache |
| 6 | `blk < q_pre` | PAGE/ROM block cache; MEM invariant 5 |
| 7 | stored kinds in range by induction from `persist`; genesis root empty or proved by Init | PAGE/ROM block cache; MEMORY BACKEND assumptions (condition C4); kit INIT (the provider) |
| 8 | no constraint for deduplication or ordering of lines | PAGE/ROM block cache |
| 9 | a line changes no boundary value (SRC-023) | PAGE/ROM block cache; BOUNDARY COMMITMENT |
| 10 | capacity at least the per-step maxima | PAGE/ROM block cache; CORE capacity rule; AD-3 |
| 11 | read address from a register, expected kind, `addr < len_cur` at that step | CORE read port (items 1 to 9), for every consumer including VALUE DIGEST and INIT; mutants `core_future_read`, `core_read_kind_unconstrained`, `core_read_address_free`, `vd_child_future_record` |
| 12 | appends and new window rows one to one; `len` advances only by real appends | CORE append chain (A1 to A4, the hybrid transport); MEM invariant 1; Lean O45 |
| 13 | unused lanes zero in all five cells; an active request fixes a non-zero chip id | CORE invariant 9; BUS |
| 14 | scratch write before read (R-8) | CORE invariant 8 (independence from incoming scratch); Lean O10 |
| 15 | ROM has its own chip id; key holds table id and row from the path bits | section 1.2 (id 4); PAGE/ROM block cache |
| 16 | ROM leaf and node tags differ from heap domains and carry heights | PAGE/ROM block cache; HASH domain registry |
| 17 | canonical padding of unused rows and the last line, part of the `prog_d` normal form | PAGE/ROM block cache |
| 18 | `ty_d` and constant `vd` in ROM trusted only through the PROG canonicality premise | PAGE/ROM block cache; VALUE DIGEST; AD-14 |
| 19 | ROM depth and program-size bound stated; no narrowing without the owner | PAGE/ROM block cache; AD-12 |
| 20 | tagged table: fixed tag (three columns), zero row, one DEC row per key, range checks on every packed part | BUS (tagged table); DEC |
| 21 | ALU tuple `(ALU, op + 8 flag, x, y, z)`, ranges inside the chip, unique `(z, flag)`, 64-bit limbs for multiply | ALU; RANGE (64-bit shape) |
| 22 | HASH: capacity tags; no unconstrained provider output; disabled permutation off the bus | HASH |
| 23 | VALUE DIGEST: `sherlock-value/1` unchanged; constants per preimage length; whole-sponge slots | VALUE DIGEST; HASH |
| 24 | `MemBackend` keeps `persist`, gains one block-line method; token opaque; W a parameter | MEMORY BACKEND; PAGE/ROM block cache |
| 25 | boundary default: Poseidon digest over registers, stack hashes and `(len, root, tail_d)`, capacity-tagged | BOUNDARY COMMITMENT: the body digest is capacity-tagged; the header is pinned (section 1.11, FP-5; deviation in `FD/ARCH-AUDIT-RESPONSE.md`) |
| 26 | Profile B conditions | MEMORY BACKEND variant ZmLogUp; NODE INTEGRATION and L AND WRAP extension points |
| 27 | leaf public lanes reconciled with `ivc.rs:70-74` and the `h_pre == 0` genesis gate | NODE INTEGRATION; BOUNDARY COMMITMENT; AD-13 |
| 28 | `LeafShape { s, w_flush, k_page, k_rom }` and the arena count stay parameters | section 1.5 (one `LeafShape`, with `n_cold` as a fifth field); CORE skeleton; AD-3, AD-4 |

Conditions of the C1 Quint lane (`qa/gemini-quint/c1-dense/REPORT-dense.md:77-101`; `verdict.md:9`) and where each is a contract item:

| Condition | Short form | Kit |
|---|---|---|
| C1 | CORE append chain A1 to A4 and the boundary stitch; the lookup is set inclusion | CORE (append chain, guaranteed); MEM (assumed); provider rule 6; BOUNDARY COMMITMENT and NODE (stitch); Lean O45 |
| C2 | `TOP >= 2`, or the walk fix, and `sibs.len() == top` | MEMORY BACKEND: fixed in `f36eb714d` for every `TOP >= 1` ("The `TOP == 1` fix"); tests `c1_top1_positive`, `c1_gap_top1_empty_seg_forgery` |
| C3 | read port: boolean `cold`, the two `TOP + 3`-bit decompositions, kind pin, warm request, cold cells, `len_cur` bound to the running length | CORE read port (items 1 to 9, guaranteed); MEM and MEMORY BACKEND (assumed); VALUE DIGEST and INIT through the port |
| C4 | genesis: empty arena constants, or canonical records with kinds in `1..=4` | INIT (guaranteed); MEMORY BACKEND and PAGE/ROM item 7 (assumed); Lean O50 |

### 6.2 RL AUDIT finding 8: invariants that become MEM contract items (R-3)

| Rule of finding 8 | Where |
|---|---|
| `tail_d` canonical; `r = 0` gives one constant | MEMORY BACKEND invariant 1; `Mem.wf_unique` (proved in C3) |
| a leaf that reads or extends the tail opens it; otherwise passes `tail_d` through | MEMORY BACKEND invariant 2 (as built, every leaf opens the tail, which is stronger) |
| `len_pre` and `len_post` both decomposed with boolean bits and `r < 8` | MEM frontier (`TOP + 3` bits each) |
| every read constrains the record kind | CORE read request; MEMORY BACKEND invariant 5; mutants `wrong_kind_warm`, `wrong_kind_cold` |
| the one-flush argument is per leaf (`r_pre + used <= 15`) | MEM window bound `8 W + 7 - r_pre`; `leafFlush_eq` (proved in C3) |
| every hash carries a domain tag | MEMORY BACKEND invariant 7, in the capacity word (E-2), not the first input |
| `count == 0 <-> hash == EMPTY` enforceable in one direction; `EMPTY_SEG` distinct only as a collision assumption | STACKS invariants; MEMORY BACKEND invariant 8; `chainHash_inj` premises |
| the design replaces D2, D3, D6; hash stacks are not D7; new chip models are a condition of adoption | MEM and STACKS verification items: the dense model exists and passed (`qa/gemini-quint/c1-dense/dense.qnt`, final verdict in the same folder); `qnt/hashStacks.qnt` is new and open |

### 6.3 Not checked

- The B2 audit response made no build, test, proof or model run either. It read the cited code and Lean lines (listed per finding in `FD/ARCH-AUDIT-RESPONSE.md`) and quotes the C1 Quint verdict and the X1 and X2 results from their files. `scripts/sherlock-next` was not run: the step is a document revision with two output files.
- In no draft or decided input: the header-and-body state commitment and the domain `SHK_SD2`; `AuthRec`, `AuthRom` and the read-port signatures; the compound `StackStep`; the DEC idle row; the GP-2 protocol (SET grammar, `SHK_DEPS`, `SUB`, `DL`, DESCEND, DL-STEP, input link); the node relation table and the shape base cases; the registry entry encoding and the `FV` state; the kits INIT, REFERENCE BRIDGE, HOST CHECKPOINT STORE and GP-3 general verifier with their file names; the codec name `sherlock-ckpt/1`; every new mutant, invariant and theorem name. None is built, modelled or measured. Every row figure that revision B2 added is ESTIMATE.
- Not read by the B2 audit response: `O/design.md` section 2.4 (the exact preimage of `prog_d`; kit INIT refers to it and gives no word count), the SAS rows, `O/dissent.md` in full line by line (they were searched for the phrases that revision B2 changed).
- The step semantics of a rewrite or a pop on an empty stack is read from `owner_runtime.rs:3566-3577` only; whether a rule can reach it was not checked.
- No build, test, proof or model run was made for this merge. The MEASURED rows are quoted from `FD/EXPERIMENTS.md`; the "proved in C3" status is quoted from `FD/EXPERIMENTS-C3.md`. Neither was re-run.
- The two study `CONSENSUS.md` files were not read in full. Their content enters only through the two audits and `FD/DECISIONS.md`, which override them.
- `gemini-area2.md:245-779` and both area 3 drafts were read by helper agents, not by the lead. Line citations into those ranges come from the helpers' digests. The Gemini area 2 file was still growing while its helper read it (final length 900 lines); the helper's line numbers for that file may be off against the final file and were not re-checked. `gemini-area3.md` was final (02:47) before its helper started (02:50).
- The working tree shows the three Gemini drafts, `LC/CONSENSUS.md` and `qa/sherlock/status.json` as modified against HEAD. This merge did not write them; it read the working-tree versions.
- Known since experiments X1 and X2 (`FD/EXPERIMENTS-MAXIMA.md`, decisions M-1 to M-4): the rule inventory (120 enabled step rules), the per-rule maxima of appends (3), stack operations and heap reads (4), and the one-arena non-interleaving property (holds; the final induction is a code reading). Still not known: ROM rows and blocks per step (X15), ALU and range requests per rule, the physical cells of a record (AD-11), the full state schema lane count, and the ROM table index widths. Every budget that depends on these is ESTIMATE.
- No leaf or node row count exists for the dense backend wired into `RfLeaf`, for hash-chain stacks, for cache lines, for VALUE DIGEST or for the full CORE. The decider table-row headroom (16,103 of 1,048,570, LC AUDIT section 3) was not re-derived for any new leaf key.
- Signatures in the code skeletons were not compiled. New file names, new Quint model names, new Lean file names and the chip id 4 for ROM are proposals.
- The lead did not read the spec files (`O/design.md` 2.7, `O/requirements.md`, `O/general-purpose.md`, `O/dissent.md`) in full; their content entered through the audits, the drafts and the helpers' line checks (helper facts file, items 9 to 11).
- Items that come from no draft and no decided input, proposed by the merge or its helpers: ROM chip id 4; the tagged-table tag values; the method name `open_line`; the SET leaf form `H(SHK_SET_LEAF, claim_d, n_steps)` and the dependency-join discharge (N-3); the LogUp tuple encoding in the ZmLogUp kit; the new Lean file names, Quint model names and spec folder names; the boundary schema item 5.
- OWNER marks: `FD/DECISIONS.md` carries them on R-4 and R-5 only. The other OWNER marks in section 4 were added by the merge where a choice would change spec text or a gate.
- External facts in the drafts (prior art, advisories, the pinned verifier beyond the lines the audits verified) were not re-checked.
- `scripts/sherlock-next` (the factory loop of `AGENTS.md`) was not run: this step is a read-only document merge with one output file.
- The wiki ingest (`llm-wiki/`) and the B2 audit are left to the lead.
