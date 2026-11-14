# T7c results: path C, the one-leaf Compact call

Row T7c of `research/sherlock-tested-design-20261113/LOOP.md`. Trees, all on branch `t7c-pathc`: Moriarty `~/Projects/Moriarty-wt/t7c-pathc` (base `316b17682`), openvm `~/.cache/m7-wrap/src/openvm-t7c` (base `662235f`; the stark-backend `[patch]` points at `../stark-backend-t7c`, base `deeeb0a`, no change there). State, binaries and logs under `~/.cache/t7c/`.

Labels: MEASURED (read from a run record), DERIVED (computed from MEASURED numbers or source by a stated rule), ESTIMATE.

## 0. For the lead

Done. Path C proves one leaf of `entry x;` through the leaf layer at the internal set, `L` and `L_Fr`, on the same universal `L` and `L_Fr` keys as the full tree. Every T7 check still passes, and D3 passes on every key.

1. **One universal `L` key (MEASURED).** The `L` key built over N3 is byte-identical to the `L` key built over the path C leaf layer of chip set 0 and of chip set 1. The pre-hash is the same in the chain run and the pathc run (`~/.cache/t7c/logs/pathc.log`, `check-chain.log`). The verifier sub-circuit runs in child-meta mode (section 1.1). `L_Fr` is one key over both paths: the `L` proofs of the two paths have equal per-AIR heights and stacked widths (fixed heights, case `pathc_and_full_tree_L_same_shape`).
2. **`L` columns (MEASURED, `~/.cache/t7c/out/shape-pc1/shape.json`).** Slim plus path C gives 47 AIRs and 1,925 columns, under the unchanged 2,000 bound with 75 free. T10's 70-column onion AIR fits (1,995), so there is no fallback bound and nothing awaits owner confirmation at T15. `StmtAir` uses 98 of its 100 interactions (T10's `TIE_BUS` receive makes 99). The slim commit `2080b5f` stands alone for T10.
3. **`X` binding: ACCEPTED by the lead 2026-11-14.** `L_Fr` checks `X` against the `L` cached commitment it reads, so the wrap needs no `X` check and no `Canon` (section 1.3). ARCHITECTURE section 1.6, section 2.4 (consumed by `L`), I-17 and section 9 are edited to match. `L_Fr` public words stay at 1,522 (MEASURED).
4. **I-17 code check.** The child `h_pre` is not bound by `X`, so it joins the DAG key leaf `P2(DOM_KEYDAG, entry, PATH_C, enc(X), h_pre, pre_app, cc_app)`.
5. **Wrap rows (DERIVED): unchanged.** The `L_Fr` proof has the same shape as at T7 (MEASURED: max log height 19, stacked widths [33, 10], 1,522 public words, vk 45,528 bytes), and the wrap gains no `X` check. So the wrap rows equal the T10a value of 1,680,729, under 1,782,573 at k = 21.
6. **D10 on path C: done.** The `L` heights are fixed at the maximum over the full-tree child and the path C children (`l_fixed_heights` in `pathc.json`): `L` max log height 16 (bound 20), stacked widths [17, 1] on both paths, `L_Fr` max log height 19 (bound 20). D3 at these shapes: every term is at least 100 and every grindable term at least 101 on every key (`~/.cache/t7c/out/pathc/d3.json`; weakest: `L` vk `mu` 101.0730, `L_Fr` `query` 101.1435, path C leaf `shift` 101.1643).
7. **Keys below `L` unchanged.** The app, leaf-layer, N1, N2, N3 and FV keys are byte-identical to T7 at `3607e95` (MEASURED by `t7_chain shape` keygen against the T7 `chain.json` pre-hashes). The proof runs ran at `acb741e`, where those keys differed from T7 by one equivalent symbolic form; `3607e95` restores the stock form. The `L` and `L_Fr` keys are the T7c ones.
8. **For T10 at merge.** T10 makes the N3 log words key constants in `root_map` and adds `TIE_BUS` to the root statement. The path C branch reads the leaf `log_d` and `log_n` from `root_map` columns (`lw` in `eval_root`), so the merge must keep those words as columns on path C, or the path C log must take the same tie.

Commits, openvm `t7c-pathc`: `517a86b`, `2080b5f` (slim), `638776f` (child-meta), `e7cbc09` (S = 8 bucket), `acb741e` (path C), `3607e95` (stock form). Moriarty `t7c-pathc`: this report, the ARCHITECTURE edits and the patch series `research/sherlock-stark-code-20261112/openvm-t7c/`.

## 1. What was built

openvm commits on `t7c-pathc` (patch series `research/sherlock-stark-code-20261112/openvm-t7c/`, `git format-patch --binary 662235f..t7c-pathc`):

| commit | what |
|---|---|
| `517a86b` | stark-backend `[patch]` to `../stark-backend-t7c`; `t7_chain shape` mode |
| `2080b5f` | slim `L`: the `Stmt` and `Call` interface AIRs of the production `L` are one zero column each (they had 0 interactions and no constraint read their cells); bench and `lfr_call` paths unchanged. For T10 to cherry-pick |
| `638776f` | recursion child-meta mode (`VerifierConfig::child_meta`) |
| `e7cbc09` | `sherlock_leaf::s8`, the S = 8 shard bucket; the CORE `rules_d` send in 8-word blocks |
| `acb741e` | path C in the root statement of `L`, `KeyPathAir`, the seal's `prog_d` opening, the universal `L` and `L_Fr` in `t7_chain`, the `pathc` mode |

### 1.1 One universal `L` key (child-meta mode)

The stock verifier sub-circuit writes the per-AIR constants of the child key (AIR count, which AIRs are required, widths, interaction counts and the other proof-shape constants) into the constraints of `ProofShapeAir`. So an `L` built over N3 and an `L` built over a leaf layer differ in that AIR (MEASURED earlier in this row: the per-AIR vk diff shows only `StmtAir` and `ProofShapeAir` differ). Child-meta mode moves those constants into data. They become rows of the cached constraint-DAG trace (`SymbolicExpressionAir`, fixed height `h`), and the meta bus carries them to `ProofShapeAir`. The `L` key then depends only on the child parameter set. The cached trace, and so the `L` cached commitment `X`, carries the child DAG and its shape constants.

- `crates/recursion/src/system/mod.rs:71` (`child_meta`), `:634` (wiring: the meta bus, `ProofShape::set_meta`, the batch-constraint meta height)
- `crates/recursion/src/proof_shape/proof_shape/air.rs:178-218, 268, 343, 385, 526-531` (`meta` columns, the meta receives, `is_first_idx` and `num_airs` from the meta columns)
- `crates/recursion/src/proof_shape/proof_shape/trace.rs` (meta rows), `crates/recursion/src/proof_shape/mod.rs:240` (`set_meta`)
- `crates/recursion/src/batch_constraint/mod.rs:87-112` (`commit_child_vk_meta`, `meta_cached_rows`), `.../symbolic_expression/air.rs:123, 231` (meta sends), `.../symbolic_expression/trace.rs:696` (meta rows of the cached trace)

The meta height is the next power of two over every admitted child: N3 22,694 rows, the path C leaf layer 19,881 (chip set 0) and 19,889 (chip set 1), height 32,768 (MEASURED).

### 1.2 Path C in `L`

`L` takes one child. A full-tree child is an N3 proof (`pc` = 0), and the statement is the T7 statement unchanged. A path C child is a leaf-layer proof at the internal set (`pc` = 1):

- The root statement AIR receives the 62 leaf-layer words. The words at the N3 key-constant positions 0 and 25 to 28 come from 5 extra columns, and no word past 61 is received. It also receives the cached commitment (`cc_app`, the app DAG in the leaf layer) and the child pre-hash (8 columns `hp`). It requires `h_pre = 0` (one leaf from step 0), and its output is the SEG of the leaf (`seg_of_leaf`: `h_lo = 0`, `h_hi = steps`, `reg_root` and `shape_d` 0, `keys_d` the key constant). `key_root` comes from the seal (`crates/sherlock-lfr/src/stmt.rs:1164`, `eval_root`).
- It sends the DAG key leaf `P2(DOM_KEYDAG, entry, PATH_C, enc(X), h_pre, pre_app, cc_app)` (5 blocks; `stmt.rs:1121`, `dag_leaf_words`). **I-17 code check:** the stock recursion takes the child pre-hash as data on the pre-hash bus and `X` does not hold it, so on path C the child `h_pre` is bound only if the leaf holds it. It joins the leaf. `pre_app` and `cc_app` (the app key that the leaf layer verified) join the leaf too, as in the full-tree key triple.
- `KeyPathAir` (20 columns, 32 rows; `stmt.rs:1297, 1314`): row `r` receives the node of level `r` on the sponge output port (row 0: the DAG key leaf), sends `P2(DOM_KEYNODE, l, r)` while enabled, and the root row sends `key_root` on `KR_BUS` 970 (`stmt.rs:96`).
- `RootSealAir` (`stmt.rs:1406`) receives `(prog_d, post_d, pc, key_root)`. On path C it opens `prog_d = P2(DOM_PROG, image_d, plan_d, key_root, machine_version)` and receives `key_root` from `KeyPathAir`. On the full tree the first node already did this.
- On the full tree, `X` is the key constant `x_n3` (the `L` cached commitment over N3).
- `X` is now a root statement word: the root statement AIR exposes FP-3 words 0 to 105 (`ROOT_PVS`, `stmt.rs:104`), and the `Stmt` interface AIR keeps words 106 to 115. The word order of `L` is unchanged.

### 1.3 `X` binding: `L_Fr` checks `X` against the `L` commitment (LEAD pick)

`L` cannot see its own cached commitment. `L_Fr` now reads it as a tape input (`lverify::Gen::cached_in`, `crates/sherlock-lfr/src/lverify.rs:73, 328`; no pin, so one `L_Fr` key covers every child of `L`) and asserts `X = sum_t limb_t 2^(30 t)` against it (`t7_chain.rs:740-770`, `lfr_gen_any`). That closes the DAG key check inside `L_Fr`. The wrap needs no `X` check, and the `L_Fr` public words stay at 1,522. Limb range and `Canon` of `X` are not needed for this binding: on path C the limbs are fixed by a `key_root` leaf that `sherlock plan` writes canonical, and on the full tree `X` is a key constant of `L`. ARCHITECTURE section 1.6 assigns the check to the wrap (about 120 rows, ESTIMATE). Doing it in `L_Fr` costs no wrap row.

Wrap scope (for T10, not edited here): `tools/sherlock-wrap/src/decode.rs:366-368` checks `X` against one pinned constant `sh.x`. With path C, `X` differs per program, so that equality must go: `L_Fr` now binds `X`. The Rust reference decoder has the same check (`crates/sherlock-lfr/src/iface.rs:747-750`, `decode_with`). The T7c runs pass the run's `X` as the decoder key (`dec_keys`).

### 1.4 The S = 8 shard bucket

`entry x;` is 5 steps; at S = 4 it is 2 shards. `sherlock_leaf::s8` compiles `core.rs` and `leaf.rs` a second time with S = 8 (CORE height 16; `crates/sherlock-leaf/src/s8/mod.rs`, `lib.rs` `slots`). Its plan has CORE height 16, so `plan_d` and `prog_d` differ from the S = 4 program. The CORE `rules_d` sponge send now goes in 8-word blocks (`core.rs:297`). At S = 4 that is one block with the same expressions. At S = 8 it is two blocks: the first S = 8 proof failed LogUp on bus 80 before this fix (MEASURED). The leaf OpenSpec already has a shape menu with several shapes (design section 3), so no requirement changed.

## 2. Measurements

All on openvm `acb741e` unless stated otherwise.

| item | value | label | record |
|---|---|---|---|
| `L` AIRs and columns | 47, 1,925 (bound 2,000) | MEASURED | `~/.cache/t7c/out/shape-pc1/shape.json` |
| `StmtAir` of `L` | 85 columns, 98 interactions, 106 public values | MEASURED | same |
| `RootSealAir`, `KeyPathAir`, `ProofShapeAir` | 100 / 25 int.; 20 / 5 int.; 52 / 82 int. | MEASURED | same |
| bare `Stmt` and `Call` | 1 column each (were 51 and 80) | MEASURED | same |
| meta cached rows: N3, path C set 0, set 1, height | 22,694; 19,881; 19,889; 32,768 | MEASURED | `pathc.json` `l_meta` |
| path C app (S = 8) | 7 AIRs, 2,373 columns, 1 shard, 5 steps, `h_pre` 0, fixed heights | MEASURED | `pathc.json`, `shape.json` |
| path C leaf layer (internal set) | 1,597 columns, max log height 17 (bound 19) | MEASURED | `pathc.json` |
| `L` on path C and on the full tree | max log height 16 (bound 20), stacked widths [17, 1], 12,190 public values | MEASURED | `pathc.json` |
| `L` vk | 126,117 bytes | MEASURED | `pathc.log` T7KEY |
| `L_Fr` | max log height 19 (bound 20), stacked widths [33, 10], 1,522 public words, vk 45,528 bytes | MEASURED | `pathc.json` |
| wrap rows | 1,680,729 (T10a), unchanged | DERIVED | section 0 item 5 |
| times (shared host, not exclusive) | path C `L` 68.5 s, full-tree `L` 64.6 s, `L_Fr` keygen 57.9 s, `L_Fr` prove 205.8 s | MEASURED | `pathc.log` |
| D3, path C key set | all gates PASS | MEASURED | `~/.cache/t7c/out/pathc/d3.json`, `~/.cache/t7c/logs/pathc-d3.log` |

T7 checks with the new `L` (`scripts/qa/check-chain.sh` with `SHERLOCK_SDK_ROOT` at openvm-t7c `acb741e`, logs `~/.cache/t7c/logs/check-{chain,cases,omit}.log`, MEASURED):
- chain: V.10 A PASS and V.10 B PASS (2 shards, 5 steps, fixed heights, G1 accepted, D3 PASS on 8 keys, 8 cases each with `full_tree_wrong_x_refused_at_L`, N3 fixed point, `L_Fr` 1,522 words); two programs, one `key_root`, one `keys_d`.
- cases: T7 cases PASS, 12 cases.
- omit: V.9 PASS, 63 cases (the `L` omit set now includes `KeyPathAir`); every AIR required.

The first pathc run at a 26G cap was killed during `L_Fr` keygen (exit 137, `~/.cache/t7c/logs/pathc-oom26g.log`; every case before the kill passed). The rerun at 56G passed.

## 3. Cases

`t7_chain pathc` (`~/.cache/t7c/out/pathc/cases.json`, all 25 PASS, MEASURED). Every refusal at `L` uses a mutant prover and the stock `L` verifier.

| case | expect | got |
|---|---|---|
| `pathc_L_honest` | accept | accepted |
| `full_tree_L_honest_on_the_same_key` | accept | accepted |
| `pathc_and_full_tree_L_same_shape` | accept | equal heights and stacked widths |
| `pathc_child_key_outside_key_root_refused_at_L` (leaf layer of chip set 1, its DAG leaf not in `key_root`) | refuse | refused (zerocheck) |
| `pathc_wrong_x_refused_at_L` (`X` limb 0 xor 1) | refuse | refused |
| `pathc_x_of_the_full_tree_refused_at_L` | refuse | refused |
| `pathc_wrong_prog_d_opening_refused_at_L` | refuse | refused |
| `pathc_x_not_the_l_commitment_accepted_at_L` (a `key_root` whose leaf holds `X` of chip set 1) | accept | accepted (`L` cannot see its commitment) |
| `pathc_x_not_the_l_commitment_refused_at_L_Fr` | refuse | tape assertion failed |
| `pathc_x_not_the_l_commitment_refused_by_stock_L_Fr_verifier` (mutant `L_Fr` prover) | refuse | refused |
| `slim_{call,stmt}_word_{first,middle,last}_changed_refused_by_stock_L` (6) | refuse | refused (`InvalidLogupPowWitness`) |
| `slim_{call,stmt}_word_{first,middle,last}_changed_refused_by_L_Fr_tape` (6) | refuse | refused |
| `pathc_lfr_tape_honest`, `full_tree_lfr_tape_honest_on_the_same_tape` | accept | accepted |
| `pathc_L_Fr_honest` | accept | accepted |

## 4. Carbon Quint lanes (lead runs them)

Every circuit change, for a Gemini Carbon Quint lane:

1. Child-meta mode of the verifier sub-circuit: `crates/recursion/src/system/mod.rs:634`, `proof_shape/proof_shape/air.rs:182-218, 343-385, 526-531`, `batch_constraint/expr_eval/symbolic_expression/air.rs:231`, `.../trace.rs:696`. Claim: with `child_meta`, the proof-shape constants of the child come from the cached trace rows that `X` commits, so a fixed `X` fixes the child key as the stock constants did.
2. Root statement of `L`, path C branch: `crates/sherlock-lfr/src/stmt.rs:1164` (`eval_root`). Claim: on `pc` = 1 the output is `seg_of_leaf` of the received leaf words with `h_pre = 0`, and the DAG key leaf holds `entry`, `PATH_C`, `X`, the child pre-hash, `pre_app` and `cc_app`; on `pc` = 0 the T7 statement is unchanged and `X = x_n3`.
3. `KeyPathAir`: `stmt.rs:1297-1373`. Claim: the root sent on `KR_BUS` is the fold of the received leaf with the sibling path (depth at least 1, one root row).
4. `RootSealAir` path C opening: `stmt.rs:1406-1453`. Claim: on `pc` = 1, `prog_d = P2(DOM_PROG, image_d, plan_d, key_root, mv)` with `key_root` from `KeyPathAir`.
5. Slim interface AIRs: `crates/sherlock-lfr/src/iface_l.rs:127` (`bare`). Claim: the removed cells were read by no constraint and no interaction.
6. CORE `rules_d` send: `crates/sherlock-leaf/src/core.rs:285-302`. Claim: the blocks are those of `P2(SHK_RULES, rids)` for any S.
7. `L_Fr` `X` check: `crates/sherlock-wrap-export/examples/t7_chain.rs:740-770` (`lfr_gen_any`). Claim: the tape fails unless `X` packs to the `L` cached commitment.

## 5. Open and handoffs

- Wrap and the reference decoder (T10 lane): drop the constant `X` equality at `tools/sherlock-wrap/src/decode.rs:366-368` and `crates/sherlock-lfr/src/iface.rs:747-750` (section 1.3).
- Merge with T10: `root_map` and the root statement interactions (section 0 item 8). If T10's L log tie adds 1 interaction, `StmtAir` is at 99 of 100.
- The checks ran at `acb741e`; `3607e95` changes only the symbolic form of one constraint outside meta mode (keys below `L` back to T7, MEASURED by keygen). It was not rerun through the proof checks.
- `T7_DEBUG` does not work on the node layers: the stark-backend interaction debugger has no `is_first_row` (`interaction/debug.rs:95`), and node statement AIRs use it in interaction multiplicities. This predates T7c.
- Path C admits only programs that fit one S = 8 shard. Other buckets are more `sherlock_leaf` instances, each with its leaf layer at the internal set and its DAG key leaf. Their meta rows must stay at or under 32,768 (this height is a constant of the `L` key).
