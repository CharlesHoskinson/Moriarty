# D7 frame protocol: cached or uncached

Design point D7 of `openspec/changes/sherlock-vm-consensus-20261107/design.md` 2.7.8 is open:
the chip leaf must use one frame protocol, uncached or cached. This note gives the evidence and
a recommendation. It does not change `design.md`; section 8 has the replacement text.

Models (all in `models/sherlock/quint/`):

- `chipLeaf.qnt`, `chipLeaf_test.qnt`: the existing uncached model. Not changed.
- `chipLeafChain.qnt` (new): two chained leaves of the uncached model. It imports `chipLeaf.qnt`
  unchanged and copies only the body of `genLeaf` so that leaf 2 starts from leaf 1's statement.
- `chipLeafCached.qnt` (new): a copy of `chipLeaf.qnt` with the cached frame rules, a `top`
  register in the statement, a cached reference machine and two chained leaves.
- `chipLeafCached_test.qnt` (new): deterministic scenarios for both protocols and every D7 mutant.

Gemini lane: `qa/gemini-quint/chipleaf-d7/` (section 6).

## 1. Answer

Use the **cached** protocol. Both protocols are sound with D1 to D8 and the constraints of
section 3; the frame-stack refinement invariant holds for both over two chained leaves, and every
mutant of either protocol is caught. The choice is on cost and on consistency with the rest of
the design:

1. A rewrite of the top frame costs nothing on the bus in the cached protocol. In the uncached
   protocol it costs one MEM append (a `mem_root` update and one MEM row of the fixed height)
   and one read, which is a PAGE opening (200 to 500 rows, ESTIMATE) when the top frame was
   written by an earlier leaf. Every rule that rewrites the top frame or task pays this
   (`study.md` 2.4).
2. PUSH and POP cost the same in both: one append, one read.
3. The cached protocol adds the top frame to the statement. `design.md` 2.7.1 and `study.md` 1.3
   already list "top frame, top task" in the register file, so the cached protocol is the one the
   rest of 2.7 assumes. The cost is about 3 words per side of the statement, hashed once per leaf
   (ESTIMATE), against at least one append per rewrite step for the uncached protocol.
4. The dispatch class (`study.md` 2.2) contains the frame tag. In the cached protocol it is a
   register. In the uncached protocol each step that dispatches on the frame tag needs a read of
   `mem[fp]` before DEC (not modelled; it follows from the protocol).

The cached protocol needs two more stated constraints than the uncached one: `top` is carried in
`pre_d`/`post_d`, and POP sets `top` from the loaded entry. It also needs a sentinel at depth 0.
The model has a counterexample for each when it is missing (section 4).

## 2. The two protocols

Registers: `fp`, `depth`, `len` in both; `top` in the cached protocol only. Cells are
`(value, link)`; NIL = -1.

| Op | Uncached (`chipLeaf.qnt`) | Cached (`chipLeafCached.qnt`) |
|---|---|---|
| meaning of `fp` | address of the top entry | address of the spill that holds the frame below the top |
| PUSH v | append `(v, fp)` at `len`; `fp := len`; `len + 1`; `depth + 1` | append `(top, fp)` at `len`; `fp := len`; `len + 1`; `top := v`; `depth + 1` |
| POP (depth >= 1) | read `mem[fp] = (a, b)`; result `a`; `fp := b`; `depth - 1` | result `top`; read `mem[fp] = (a, b)`; `top := a`; `fp := b`; `depth - 1` |
| rewrite v (depth >= 1) | read `mem[fp] = (a, b)`; append `(v, b)` at `len`; `fp := len`; `len + 1` | `top := v`; no bus request |
| depth 0 | `fp = NIL` | `top = 0`, `fp = NIL`; the first PUSH spills the sentinel `(0, NIL)` and the last POP reloads it |
| statement | `fp`, `depth`, `len`, memory root | the same plus `top` |

In both, the stored link is the core's `fp`, the read address is the core's `fp`, and POP and the
rewrite at depth 0 are errors. The sentinel makes PUSH and POP one rule each at every depth (no
depth-0 or depth-1 case).

## 3. The invariant

The same definitions are in `chipLeafCached.qnt` and `chipLeafChain.qnt`. A chain is two leaves:
leaf 1 starts from a reference state after 0 to 4 steps; leaf 2 is built from leaf 1's post
registers and committed memory (`memOut`), not from the reference. `chainAccepts` holds when both
leaves are accepted, they share the program, `leaf2.pre == leaf1.post` and
`leaf2.oldMem == leaf1.memOut`. Each leaf may carry one tamper (bus lie, table row, register,
slot shape); the cached chain may also replace leaf 2's `pre.top` (`lie`).

- `invPops` (frame-stack refinement). If the chain is accepted, the POP results of leaf 1 and
  leaf 2 in order, followed by the results of the POPs that would drain the final statement, equal
  the reference stack's POP results over the same steps followed by the reference's final stack,
  top first. The drain reads the committed statement: uncached, the chain from `fp`; cached, `top`
  and then the chain from `fp`. The drain is part of the invariant because the sampled programs are
  8 rows long: a wrong link written in leaf 1 often shows only in a POP after leaf 2.
- `invPopsExec`: the same without the drain (only POPs that the leaves executed).
- `invChainRefine`: leaf 2 refines the reference from where leaf 1 ended (registers including
  `top`, committed memory, frame stack).
- `invChainTamper`, `invChainComplete`: in the base model a tampered or lied chain is refused and
  the honest chain is accepted.
- `invAll` (per leaf, as in `chipLeaf.qnt`): `invRefine`, `invFrames`, `invTerminal`, `invBus`,
  `invMem`, `invCapacity`, `invShape`, `invFunctional`, `invTamperRefused`, `invComplete`.

The cached reference machine has the cached memory layout (PUSH appends the spill, a rewrite
appends nothing). So a mutant is not caught only because its memory layout differs from the
reference; section 4 lists which invariant fires.

## 4. Results (own re-run, 2026-11-10)

All runs: `quint` 0.32.0, from `models/sherlock/quint/`, wrapped in `heavy-run --mem 4G`.
`quint run` uses `--max-samples 20000 --max-steps 1 --seed 20261110`; each trace samples a fresh
chain (as in `chipLeaf.qnt`). Exit 0 is `[ok]`; exit 1 is `[violation] Found an issue` (the
output line was checked for each run, so no exit 1 below is a parse or runtime error).

Baseline, before any change: all 15 test modules of `chipLeaf_test.qnt` pass (4 tests each, exit
0). `chipLeaf.qnt` and `chipLeaf_test.qnt` are not changed by this work. The 24-module `quint run`
sweep of `quint-check.md` section 4 on `chipLeaf.qnt` was not re-run; `chipLeafBase invAll` was
(below).

Typecheck, exit 0 for each: `chipLeafCached.qnt`, `chipLeafChain.qnt`, `chipLeafCached_test.qnt`.

Deterministic tests (`quint test chipLeafCached_test.qnt --main <module>`), all exit 0:

| Module | Tests | What it shows |
|---|---|---|
| `cachedTest` | 3 passing | reference POP results; honest chains PF and PL accepted and sound; leaf 2 reads leaf 1's spill at address 3 through PAGE; the `top` lie is refused at the boundary |
| `cachedTest_rewrite_appends` | 3 passing | chain accepted, executed POP results `[4, 4, 2]`, reference `[4, 2, 1]` |
| `cachedTest_push_link_free` | 3 passing | program PL: `[3, 2, 0]`, reference `[3, 2, 1]` |
| `cachedTest_pop_no_reload` | 3 passing | `[4, 4, 4]` |
| `cachedTest_pop_no_fp_reload` | 3 passing | `[4, 2, 2]` |
| `cachedTest_top_dropped` | 3 passing | leaf 2 starts with `top = 0`: `[0, 2, 1]` |
| `uncachedTest` | 2 passing | the same programs on the uncached protocol: `[4, 2, 1]`, `[3, 2, 1]`; the rewrite appended (`len` 5) |
| `uncachedTest_rewrite_no_move` | 2 passing | `[3, 2, 1]` |
| `uncachedTest_push_link_free` | 2 passing | program PL: `[3, 1, 1]` |

The scenarios (`chipLeafCached_test.qnt` header): PF pushes three frames (one before the chain,
two in leaf 1), rewrites the top in leaf 1 and pops all three in leaf 2. PL pushes three frames in
leaf 1 from an empty memory, so a forged link 0 points at a frame entry and the bad POP itself
still has a valid read.

Sampled runs, base models (must hold):

| Command (`quint run <file> --main <module> --invariant <inv>`) | Exit | Result |
|---|---|---|
| `chipLeafCached.qnt cachedBase invAll` (with witnesses) | 0 | ok |
| `chipLeafCached.qnt cachedBase invChain` | 0 | ok |
| `chipLeafCached.qnt cachedS4 invAll` (S = 4, N_MEM = 3) | 0 | ok |
| `chipLeafCached.qnt cachedS4 invChain` | 0 | ok |
| `chipLeafChain.qnt chainBase invChain` (with witnesses) | 0 | ok |
| `chipLeaf.qnt chipLeafBase invAll` (unchanged model) | 0 | ok |

`invChain` = `invPops`, `invChainRefine`, `invChainTamper`, `invChainComplete`.

Witnesses, 20,000 traces each. Cached (`cachedBase`): `reachPopAcross` (a POP in leaf 2 reads a
spill written in leaf 1) 695; `reachTwoPopsLeaf2` 26; `reachLieRefused` 6,764;
`reachChainTamperRefused` 6,871; `reachPopAfterRewrite` 269; `reachPopOldFrame` 2,726;
`reachFull` 6,008; `reachPublishHalt` 190. Uncached (`chainBase`): `reachPopAcross` 1,321;
`reachTwoPopsLeaf2` 39; `reachChainTamperRefused` 6,436. None is 0.

Sampled runs, mutants (each must be caught). V = violation (exit 1), ok = no violation (exit 0):

| Module | `invPops` | `invPopsExec` | `invChainRefine` | `invRefine` | `invFrames` | `invMem` |
|---|---|---|---|---|---|---|
| `cached_rewrite_appends` (F12) | V | ok | V | V | V | V |
| `cached_push_link_free` (F13) | V | ok | V | V | V | V |
| `cached_pop_no_reload` | V | V | V | V | V | ok |
| `cached_pop_no_fp_reload` (Gemini d7 F7) | V | ok | V | V | V | V |
| `cached_top_dropped` | V | V | V | ok | ok | ok |
| `chain_rewrite_no_move` (uncached) | V | V | V | - | - | - |
| `chain_push_link_free` (uncached) | V | V | V | - | - | - |
| `cached_table_ungated` (D1, regression) | `invAll`: V | | | | | |
| `cached_page_no_base_bound` (D2, regression) | `invAll`: V | | | | | |

Reading the table:

- Every mutant is caught by `invPops`, the frame-stack refinement invariant, in the sampled runs,
  and by `invPopsExec` in the deterministic tests.
- `invPopsExec` misses three cached mutants in the sampled runs: with 8 program rows and S = 3,
  the bad POP rarely runs inside the two leaves. The drain in `invPops` and the longer test
  programs close this gap; it is a limit of the sampler, not of the protocol.
- `cached_top_dropped` passes every single-leaf invariant and fails only on the chain. The `top`
  in the statement is load-bearing only across a leaf boundary, so a single-leaf model cannot see
  it.
- `cached_pop_no_reload` passes `invMem`: every read is correct; only the register is wrong.
- `cached_rewrite_appends` also fails `invRefine` because its memory layout differs from the
  cached reference (`len` grows). It fails `invPops` and `invFrames` for the frame reason: the
  POP after the rewrite loads the rewritten entry, so the frame below the top is lost (test
  result `[4, 4, 2]`).

Commands:

```
cd models/sherlock/quint
heavy-run --mem 4G --name qf -- quint typecheck chipLeafCached.qnt   # also chipLeafChain.qnt, chipLeafCached_test.qnt
heavy-run --mem 4G --name qf -- quint test chipLeafCached_test.qnt --main <module>
heavy-run --mem 4G --name qf -- quint run chipLeafCached.qnt --main <module> --invariant <inv> \
  --max-samples 20000 --max-steps 1 --seed 20261110
heavy-run --mem 4G --name qf -- quint run chipLeafCached.qnt --main cachedBase --invariant invAll \
  --max-samples 20000 --max-steps 1 --seed 20261110 --witnesses reachPopAcross reachTwoPopsLeaf2 \
  reachLieRefused reachChainTamperRefused reachPopAfterRewrite reachPopOldFrame reachFull reachPublishHalt
heavy-run --mem 4G --name qf -- quint run chipLeafChain.qnt --main <module> --invariant <inv> \
  --max-samples 20000 --max-steps 1 --seed 20261110
```

## 5. Constraints the cached protocol needs

Each has a counterexample when it is missing (section 4 and the Gemini lane):

1. A rewrite of the top changes only `top`; no append, `fp` and `len` unchanged
   (`rewrite_appends`, Gemini F12).
2. The spill link is the core's `fp` (`push_link_free`, Gemini F13). Same as for uncached.
3. POP sets `top` from the loaded entry (`pop_no_reload`) and `fp` from its link
   (`pop_no_fp_reload`, Gemini d7 F7). Without the second, set inclusion lets later POPs reuse one
   PAGE or MEM row.
4. `top` is in `pre_d` and `post_d` (`top_dropped`). It is already listed in the register file
   (2.7.1) and in the `Init` table (`design.md` 3.2 item 5, row 18, `top` = 0).
5. Depth 0 is `top = 0`, `fp = NIL` (Gemini d7 F5, `sentinel_unconstrained`; no mutant in this
   model). The `Init` table already sets `fp = NIL` (row 16) and `top = 0` (row 18); this model
   takes the pre state from the reference, so the obligation is on `Init` and on `StepInv`, not
   on a gate.
6. The frame count (`depth` in the model, `frame_count` in `Init` row 13) changes by exactly one
   on PUSH and POP; POP and a rewrite at count 0 are errors (Gemini d7 F6,
   `depth_unconstrained`; no mutant in this model). Same as for uncached.

D1 to D6 and D8 apply unchanged; the two regression mutants above confirm that the copy still
needs D1 and D2.

## 6. Gemini lane

`argon-run` with `gemini-4-argon-high` and the quint-modeling and quint-lang skills (conversation
`c6462ddc-d111-4f6a-a457-758a4b995303`, 3 turns, ended `DONE`), in a detached worktree under
`~/.cache/gemini-lanes/chipleaf-d7/`. It got the cached protocol as text (the same micro-ops as
section 2), was told not to read `chipLeaf*.qnt`, and wrote `qa/gemini-quint/chipleaf-d7/`
(`cached.qnt`, `cached_test.qnt`, `REPORT.md`). Its report is input; the evidence is the runs
below.

One repair before its model ran: the parameter `item` of `putCell` is a built-in name (QNT101);
renamed to `cellVal` (lines 189-190). No other change.

Runs on the lane's model, from `qa/gemini-quint/chipleaf-d7/`, same flags as section 4:

| Command | Exit | Result |
|---|---|---|
| `quint typecheck cached.qnt`, `quint typecheck cached_test.qnt` | 0, 0 | after the rename |
| `quint test cached_test.qnt --main cached_test` | 0 | 13 passing |
| `quint run cached.qnt --main cachedSound --invariant` `invFrameStackRefinement`, `invSound`, `invTamperRefused`, `invPopSeq`, `invFinalDepth`, `invFinalTop` | 0 each | ok |
| same module, `reachPushInLeaf1PopInLeaf2`, `reachRewriteAndPopAcrossLeaves`, `reachNonEmptyFinalStack`, `reachIntraAndInterLeafPop` (negated witnesses) | 1 each | violation, so each shape is reached |
| `--main cached_<m> --invariant invFrameStackRefinement` for `rewrite_appends`, `link_from_bus`, `pop_no_reload`, `top_dropped`, `sentinel_unconstrained`, `depth_unconstrained`, `pop_no_fp_reload`, `pop_page_unauthenticated` | 1 each | violation: all 8 mutants caught |

Limit: as in the first chip leaf lane (`quint-check.md` section 6), its sampler picks among
hand-built witnesses (4 programs and 8 counterexamples), so its passes say nothing beyond that
list. Its mutant catches agree with this model's.

Its verdict: the cached protocol is sound with D1 to D8 and its findings F1 to F8; it recommends
the cached protocol for the same reasons as section 1 (rewrite free, dispatch on the top frame
tag from a register, statement cost paid once per leaf). Reconciliation:

| Lane | Finding | This work |
|---|---|---|
| F1 | rewrite appends | `rewrite_appends` caught |
| F2 | link from the bus | `push_link_free` caught |
| F3 | POP does not reload `top` | `pop_no_reload` caught |
| F4 | `top` not in the statement | `top_dropped` caught (chain only) |
| F5 | depth-0 sentinel not fixed at genesis | not a mutant here (pre comes from the reference); already in the Init table, rows 16 and 18; section 5 item 5 |
| F6 | `depth` unconstrained | not a mutant here; same obligation as uncached; section 5 item 6 |
| F7 | POP does not reload `fp` | added as `pop_no_fp_reload`, caught |
| F8 | PAGE row not authenticated | `page_unauth` in `chipLeaf.qnt` (confirmed load-bearing, `quint-check.md` section 7) |

## 7. Cost comparison

Counts per operation are the `reqs` lists of `coreStep` in the two models. Row prices are from
`study.md` 2.3, 2.4 and 4 and are all ESTIMATE.

| | Uncached | Cached |
|---|---|---|
| PUSH: MEM appends | 1 | 1 |
| PUSH: MEM or PAGE reads | 0 | 0 |
| POP: MEM appends | 0 | 0 |
| POP: MEM or PAGE reads | 1 | 1 |
| Rewrite of the top: MEM appends | 1 | 0 |
| Rewrite of the top: MEM or PAGE reads | 1 (for the link) | 0 |
| Rewrite of the top: bus requests | 2 | 0 |
| Step that dispatches on the frame tag (not modelled) | 1 read of `mem[fp]` before DEC | 0 (register) |
| Registers added to the statement, model | none | `top` (1 of 9 registers) |
| Registers added to the statement, Sherlock (`study.md` 1.3) | none | `top` 13 lanes; with `task` (7 lanes) 5 of the 32 words |

Prices (ESTIMATE, `study.md`):

- One append: 1 MEM row plus its share of the `mem_root` update, 200 to 750 HASH rows per step
  for 6 to 12 appended tuples, so about 17 to 125 rows per append. Gemini F14 (`quint-check.md`
  section 6) found this update undercounted for a depth-20 Merkle tree; E4 measures it.
- One read served by MEM (a cell appended in this leaf): no rows beyond the request.
- One read served by PAGE (a cell from an earlier leaf): 1 row plus 200 (depth 8) to 500
  (depth 20) HASH rows, once per distinct cell per leaf.
- `top` and `task` in the statement: 5 words more in each of `pre_d` and `post_d`. The boundary
  digests are 32 permutations x 25 = 800 rows for about 64 words (`study.md` 4.2), so 10 words
  are about 5 permutations, 125 rows per leaf; `top` alone (3 words per side, 6 words) about 3
  permutations, 75 rows per leaf.

Per rewrite, the uncached protocol costs 17 to 125 rows for the append, plus 200 to 500 rows the
first time in a leaf that it rewrites a frame written by an earlier leaf, plus one MEM row of the
fixed height. The cached protocol costs about 75 rows per leaf for `top` (125 with `task`). At
S = 16 the cached protocol is cheaper when a leaf has more than 1 to 5 rewrites of the top frame
(75 / 125 to 75 / 17; 1 to 8 with `task`, 125 / 125 to 125 / 17), or one rewrite of a frame
from an earlier leaf. How many of the 108 rules rewrite the top frame or task, and how often they
run, is UNMEASURED. `study.md` 2.4 lists "every rule that rewrites the top frame or task" among
the appending rules; that its 6 to 12 appends per step include these rewrites is an inference of
this note, not a statement of the study. E0 can count them from the per-rule fragments.

## 8. Replacement text for `design.md` 2.7.8, D7

Replace the bullet that starts "- D7 One frame protocol. [OPEN: pick one]" with:

```
- D7 Cached frame protocol. The top frame is the register `top`, carried in `pre_d` and `post_d`; `fp` is the address of the entry that holds the frame below the top. PUSH appends `(top, fp)` at `len`, then sets `fp := len` and `top` to the new frame. POP returns `top`, loads `mem[fp] = (a, b)` with `fp < len`, and sets `top := a` and `fp := b`. A rewrite of the top changes only `top`: no bus request, `fp` and `len` unchanged. The stored link is the core's `fp`, never a bus value. With `frame_count = 0`, `top` is the zero record and `fp = NIL`, as in `Init` (3.2 item 5); `frame_count` changes by exactly one on PUSH and POP, and POP and a rewrite at `frame_count = 0` are errors. The top task (`task`, `tp`) uses the same protocol [not modelled]. (`d7-frames.md`. Mutants in `chipLeafCached.qnt`: `rewrite_appends`, `push_link_free`, `pop_no_reload`, `pop_no_fp_reload`, `top_dropped`. The `frame_count = 0` and count clauses: Gemini lane `qa/gemini-quint/chipleaf-d7`, hand-built witnesses `cex_sentinel_unconstrained`, `cex_depth_unconstrained`.)
```

If this text is adopted, these lines say the opposite and need the same change (not edited here):

- `study.md` 2.4, Constraints: "pop loads `fp` and sets `fp` to the stored link; a rewrite of the
  top appends with the same link", and "every rule that rewrites the top frame or task" in the
  list of appending rules.
- `study.md` section 7, the proposed text: "push appends, pop follows the link, a rewrite of the
  top appends with the same link".
- `quint-check.md` section 7, D7: "[pick one]" and the description of both protocols.
- `models/sherlock/quint/chipLeaf.qnt` stays the uncached reference model; its header says so. The
  cached model is `chipLeafCached.qnt`.

`design.md` 2.7.1 (register file with "top frame, top task") and the `Init` table in 3.2 item 5
(rows 13, 16 to 19: `frame_count` = 0, `fp`, `tp` = NIL, `top`, `task` = 0) already match the
cached protocol. The read address of POP is the core's `fp`; this is the "read address comes from
a register" item of the "Also load-bearing" paragraph, confirmed on the uncached model
(`load_key_free`) and not re-run on the cached one.

## 9. Not covered

- The task stack (`tp`, `task`) is not modelled; the text assumes it uses the same protocol.
- A frame is one cell `(value, link)`. A 13-lane frame packed into the cell fields of a bus tuple
  is not modelled (`study.md` 2.4 counts 750 HASH rows per step if records are not packed).
- HASH rows, Merkle paths and field arithmetic are not modelled, as in `chipLeaf.qnt`.
- Dispatch on the frame tag is not modelled; the extra read of the uncached protocol in section 7
  follows from the protocol, not from a run.
- The sampled programs have 8 rows and the leaves 3 or 4 slots; deeper stacks are covered only by
  the deterministic scenarios PF and PL.
