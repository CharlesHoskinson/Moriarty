# Brief: the Sherlock pipeline animation

Read `WORKFLOW.md` first. This file holds the canonical facts and the technical contract. Every lane uses these facts. If a source file disagrees with this sheet, this sheet wins for the animation, and the lane notes the disagreement in its own output.

## 1. Register

- The page presents the system as built and running. Use the present tense. Never use "designed", "planned", "proposed", "estimate", "open", "pending", "roadmap" or any status label on the page.
- Give no account of how the system was made: no agents, no council, no audits, no experiments, no lanes, no decision ids (R-4, OD-1 and so on), no file paths.
- Explain the mechanism. Each beat says what moves, why it moves and what it proves. The owner wants exhaustive detail. A viewer who watches all eight scenes can explain how a program becomes an accepted claim on Midnight.
- Never invent a number. If the facts sheet below gives no value for a quantity, show the quantity without a number (for example, "a fixed number of cache lines per leaf"). Never label it.

## 2. Resolved choices (the page shows only these)

| Topic | What the page shows |
|---|---|
| Tree depth | `TOP = 63`: the memory tree has depth 63 over segments of 8 cells, so 2^66 cells. One logical record is one cell. |
| Arenas | One arena. |
| Appends per step | At most 3 cells per step. 3 x 2^64 < 2^66, so memory never runs out before the step counter does. |
| Flush | One flush unit per leaf (`w_flush = 1`): when the 8-cell tail fills, it becomes a segment and is hashed into the tree. |
| Old reads | A scalar cold read (a Merkle opening against the pre-state root) or an 8-record cache line opened against the pre-state root and served on the bus. A leaf has a fixed number of each. Do not show a number. |
| Boundary commitment | A Poseidon digest in header-and-body form. The header holds `outcome_tag`, `outcome_payload`, `input_d`, `deps_d`, `body_d`. The body digests the registers, the stack hashes and `(len, root, tail_d)`. |
| Memory backend | Dense tail-buffered Poseidon Merkle tree. Do not show the alternative backend or the Horner boundary. |
| Stacks | FRAMES and TASKS are Poseidon hash chains. A push hashes the old top into the chain. A pop opens it. |
| Input admission | By digest. The genesis leaf proves `Init(prog_d, input_d)` at height 0 with an empty genesis memory. |
| Claim counter | None. The public statement carries no count of claims. |
| GP-2 | A run consumes earlier claims at the seal (SEAL-WITH-DEPS mode, dependency list `deps_d`). |
| GP-3 | Foreign proofs enter through a registry of verifier keys (`reg_root`), as FOREIGN children of a node. |
| GP-4 | Any shape: segments merge left to right (MERGE), whole runs join into sets (JOIN, `set_root = H(SHK_SET_JOIN, l, r)`), in any tree or DAG. |
| One contract | The contract takes `prog_d` as call data. One verifier key set and one contract serve every program and every input (GP-1). |
| Semantics | The reference machine `sherlock-k-a/1` defines every step. |

## 3. Facts per step

Use the section numbers of `research/sherlock-final-design-20261111/ARCHITECTURE.md` (written `A §`) for detail.

1. **The program.** Moriarty source compiles to the Sherlock machine language (a program image: code tables in ROM). `prog_d` is the Poseidon digest of the program image. `input_d = Poseidon(SHK_IN, val_d, fuel_lo, fuel_hi, stack_limit)`: it binds the input value (its digest `val_d` under the canonical value digest `sherlock-value/1`) together with the run's resource policy, the 128-bit fuel as two 64-bit halves and the stack limit, each range-checked below 2^64. One key set (leaf, node, decider) serves every program: the program is data, not circuit.
2. **The run.** `mori simulate` runs a CEK-style machine. State: control, environment, frame stack, task stack, memory, fuel counters. Each step fires exactly one of 120 enabled rules. The run ends in one of three outcomes: Halted (with a value), Rejected (with a code, for example code 10 for arithmetic overflow), Exhausted (with a kind). The run records the witness stream: per step, the rule id and the state before and after. Steps are counted by a 64-bit counter, so a run can have up to 2^64 - 1 steps. 2^64 is the counter's range, not a circuit size.
3. **The leaf.** One leaf proves a slice of `S = 16` steps. It is one circuit with one key. Modules (A §1.1): CORE (registers, the rule slots, the append chain), DEC (decodes the rule control word from a fixed table), ALU (ADD, LT and the other machine ops, with an overflow flag), RANGE (12-bit limb checks), HASH (Poseidon), BUS (one lookup bus: tuples `(chip, key, a, b, c)`, chip ids 1 ALU, 2 MEM, 3 HASH, 4 ROM; two request lanes, CORE is the only requester), MEM window (this leaf's new records), MEMORY BACKEND (dense tree: persist, cold read, block lines), PAGE/ROM block cache, STACKS, VALUE DIGEST, BOUNDARY COMMITMENT, INIT (only when `h_pre = 0`). The leaf publishes seven lanes: `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`. Partition independence: the memory state at the end is the same for every cut of the run into leaves, so the final claim does not depend on where the leaves were cut. Measured costs at depth 63: a flush is 8,821 rows, an old-memory read is 4,761 rows. The leaf fits `k <= 16` (2^16 rows).
4. **The node.** One self-recursive node circuit at `k = 20`. MERGE checks: same `prog_d`, `L.post_d == R.pre_d`, `L.h_hi == R.h_lo`. It outputs `(prog_d, L.pre_d, R.post_d, L.h_lo, R.h_hi)` with heights below 2^64. It carries one accumulator: every deferred pairing check folds into it, so a node of any height has the same size. Work is linear in the number of steps. Measured on the reference host: one node proof takes about 65 s at 20 threads and 17 GB of memory; three provers in parallel give 1.73x throughput. Trees of nodes (GP-4) let many machines prove leaves and merges in parallel.
5. **The decider.** SEAL mode: `h_lo == 0` (the genesis leaf ran INIT), open the state header of the final `post_d`, require a terminal outcome (Running cannot seal), compute `outcome_d` from the outcome kind and payload under `sherlock-value/1`, and `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`. The decider verifies the root proof under pinned keys, pairs the accumulator once, and publishes exactly `[prog_d, input_d, outcome_d, n_steps]`. `n_steps` is range-checked by an 8-byte decomposition. No state digest, memory root, tail digest, length or address is public. `outcome_d` hides addresses and sharing: two runs that compute the same value publish the same `outcome_d`. The decider fits in 2^20 rows.
6. **The transaction.** The prover wraps the decider proof and the four public values in a Midnight transaction that calls `SherlockRun.acceptRun`. The call transcript binds the four values, so changing any of them invalidates the proof. Fees are paid in DUST. The proof is about 6.4 KB.
7. **The contract.** `acceptRun` verifies the proof against its verifier key, recomputes `claim_d`, refuses a `claim_d` that is already in `runs` (also when the same run is proved again under another leaf partition), and stores `outcome_d` and `n_steps` under `claim_d`. It stores no state digest. A later run can consume this claim at its seal (GP-2). A proof from another system enters through the registry (GP-3).
8. **The network.** Midnight nodes on ledger 9.1.0.1 verify the transaction with the same verifier key for every program. The claim is now a fact on chain: any contract or later proof can rely on "program `prog_d` on input `input_d` produced `outcome_d` in `n_steps` steps". Collections of runs settle as one statement `[reg_root, set_root, shape_d]`.

## 4. Technical contract of the page

- Page: `visualization/pipeline.html`. Modules: `visualization/src/pipeline/`. Shell files: `shell.js` (timeline, controls, narration panel, camera rig), `theme.js` (palette and materials, reuse `visualization/src/designers/theme.js` where it fits), `labels.js`. Scenes: `scenes/scene1.js` to `scenes/scene8.js`, one file per builder.
- Renderer: `THREE.WebGPURenderer` from the vendored build with the import map of `visualization/index.html`. The page accepts `?webgl` and then passes `forceWebGL: true`. Headless capture uses `?webgl`: a WebGPU canvas does not appear in headless screenshots on this host.
- Deterministic seek. Every scene exports `{ id, title, beats: [{ id, duration, narration, labels }], build(ctx), seek(t), dispose() }`. `seek(t)` sets the whole scene to time `t` (seconds from the start of the scene) with no hidden state from earlier frames. The URL `pipeline.html?scene=N&beat=B&t=T&webgl` renders scene N at beat B, time T into the beat, paused, then sets `window.__ready = true`.
- Narration: the panel shows the beat's narration text in full, as readable paragraphs. On-screen labels are short.
- Accessibility: `prefers-reduced-motion` gives a stepped mode (cuts between beats, no camera flights). Text in the panel is real DOM text.
- Capture tool for builders: `/home/hoskinson/.local/share/uv/tools/scrapling/bin/python /home/hoskinson/.cache/pipeline-anim/tools/shot.py "URL" OUT.png 1600 900`. The site server runs at `http://127.0.0.1:4377/` and serves the build worktree's `visualization/` (see `WORKFLOW.md`).

## 5. Sources

- `research/sherlock-final-design-20261111/ARCHITECTURE.md` (kits; sections 1, 2), `DECISIONS.md`, `EXPERIMENTS.md`, `EXPERIMENTS-MAXIMA.md`, `EXPERIMENTS-C3.md`.
- `openspec/changes/sherlock-modular-leaf-20261111/` (specs per kit).
- `openspec/changes/sherlock-vm-consensus-20261107/requirements.md` (SRC-001 to SRC-023), `general-purpose.md` (GP-1 to GP-4), `design.md`.
- Existing site: `visualization/index.html`, `visualization/src/leaf/leafChip.js` (the earlier leaf chip animation: base for scene 3), `visualization/src/designers/theme.js`.

## 6. Design system: Google Material Design 3

Owner instruction, 2026-11-11: the website uses Google Material Design. The page chrome is Material 3: color roles from one seed color, dark scheme only (owner decision 2026-11-11: no light scheme, no toggle) (`visualization/src/pipeline/m3.css`), the M3 type, shape, elevation and motion tokens, Roboto Flex and Roboto Mono, Material Symbols Rounded icons, and M3 components (top app bar, navigation rail or bottom navigation, large FAB for play and pause, slider for the scrubber, cards for narration, chips, state layers, focus rings, 48px touch targets). The 3D scene uses the same tonal palettes through `theme.js`: scenes take M3 roles, never raw hex. `visualization/src/pipeline/SCENE-API.md` holds the mapping.

## 7. Impeccable

The site follows the `impeccable` design skill (4.8.2). Product truth is in `visualization/PRODUCT.md`; the design system as built is in `visualization/DESIGN.md`. Every builder loads the skill and reads `reference/craft-floor.md` before UI edits, and runs `impeccable detect --json` on its files before it returns (zero findings). The landing pass runs impeccable `critique`, `audit` and `polish` on the finished page.

## 8. Scope rulings for the scripts (lead, 2026-11-11)

- Exhaustive scope wins over the scene targets. MERGE-WORK (inline work in a node), the continuation checkpoint and resume, and the full GP-3 foreign-proof path (registry fields, parser, transcript, arithmetic, work units, setup parameters) stay in, presented as built. The architecture sections are the source: kit NODE INTEGRATION, kit CONTINUATION, kit GP-3 hook and kit GP-3 general verifier.
- A collection settles as `[reg_root, set_root, shape_d]` under the same fixed decider key, so its proof size and verification cost do not depend on the shape, depth or number of joined claims. The narration may state this.
- Terms: "request lanes" (two of them), never "request channels". Powers are written with superscripts on the page (2<sup>64</sup>); scripts may write `2^64`.
- The design folder `research/sherlock-final-design-20261111/` is present in this worktree.
