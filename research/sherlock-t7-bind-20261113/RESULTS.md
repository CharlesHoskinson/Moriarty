# T7 results: `prog_d` and the general key route

Row T7 of `research/sherlock-tested-design-20261113/LOOP.md`, with the open T6 items A1 to A4. Change `openspec/changes/sherlock-modular-leaf-20261111/` (`ML/` below). Trees, all on branch `t7-bind`: Moriarty `~/Projects/Moriarty-wt/t7-bind` (base `8e00930b8`), openvm `~/.cache/m7-wrap/src/openvm-t7` (base `8672651`, the `tested` merge of `t6-leaf`), stark-backend `~/.cache/m7-wrap/src/stark-backend-t7` (no change; the openvm `[patch]` points at `../stark-backend-t6`). State, logs and outputs under `~/.cache/t7/`.

Labels: MEASURED (read from a run record), DERIVED (computed from MEASURED numbers or source by a stated rule), ESTIMATE.

## 0. For the lead

**Done-when.** Every case of the T7 done-when passes on real proofs of `entry x;` (program A) and `entry y;` (program B) on the full tree (sections 1 and 4). Path C is scoped and not built: the `L` key is at 1,986 of its 2,000 columns, and path C adds about 75 columns to `L` (section 6). That needs a ruling on the `L` bound or an `L` slimming step. D10 on path C sits with it.

**Readings and picks to accept or overrule:**

1. **X10 (I-12, I-35) is in `L`, not in a SEAL node.** `RootSealAir` (openvm `crates/sherlock-lfr/src/stmt.rs:1150`) opens the 41-word header against `post_d` by `P2(SHK_SD2)`, requires a terminal `outcome_tag` in {1, 2, 3} and `deps_d = EMPTY_DEPS`, and exposes the three `R` operand digests. SEG and `h_lo = 0` are key constants of the root statement. The NODE SEAL mode, NODE-002, NODE-010 and the SET head stay with group 5.
2. **Outcome kind = `outcome_tag - 1`** in `OPO = P2(DOM_OUTHIDE, input_d, outcome_d)`.
3. **The key leaf holds the cached-commitment words.** Key leaf `P2(DOM_KEYTRIPLE 8, set_id, pre_app, pre_leaf, pre_ifl, cc_app, cc_leaf)`; path of up to `D_KEY = 16` levels by `P2(DOM_KEYNODE 10, l, r)`.
4. **Node statement is 90 words:** the 74-word FP-2 envelope plus the own `vkc` (cached commitment 8 words, pre-hash 8 words). A parent compares the `vkc` words of a child with the commitment and pre-hash that its verifier sub-circuit receives.
5. **Universal keys by a table, not by `key_root`.** `keys_d = P2(DOM_UKEYS 32, c_node, c_foreign)` over 4 node rows and 4 foreign rows of `(cc, pre)`. A node or FV unit picks its own `vkc` row by a one-hot selector and requires every child `keys_d` to equal its own. `L` pins `keys_d` and the N3 pre-hash as key constants. Leaf key triples go into `key_root` (they are per program and per chip set, lead ruling). Spec edit: `ML/specs/sherlock-node-integration/spec.md` NODE-004 (strict validate passes).
6. **The internal-recursive key is at a fixed point.** N3 over N3 has the pre-hash of N3 over N2 (MEASURED, `T7KEY "N3 fixed point" n3_over_n3_pre_equals_n3_pre=true`). The table holds N2 over N1, N3 over N2, N3 over the FV unit and N3 over N3, so a tree of any depth uses the same four node rows.
7. **`A_0 = C_P` in `L_Fr` (LEAD pick, affects T10).** `lverify::gen_ops_check` (openvm `crates/sherlock-lfr/src/lverify.rs:1835`) requires, in mode 0, that the `a_in` anchor operand equals the operand of the `DIGEST(SHK_PROGC)` entry. So the ledger anchor `C_L = DIGEST(SHK_PROGC, anchor_d2)` fixes `prog_d`. The check is called by the `t7_chain` `L_Fr` generator; `lfr_call` (T10a) does not call it, so its run is not changed. T10 must call `gen_ops_check` and replace the `ANCHOR_D2` constant by `iface::anchor_d2(r_L, prog_d_ledger)` (seam `crates/sherlock-wrap-export/examples/lfr_call.rs:122`, comment added).
8. **The `L` log tie is open (A2 in `L`).** The slice leaf log is the neutral `(D_EMPTY, 0)`. The node SEG merge does the FP-2 log merge with is-zero gadgets (A2 in the tree). The `L` log holds the `R` entries of the run, and the root statement log words are not yet tied to the onion. This needs the T8/T9 log route; it is not a T7 hard rule.
9. **`shape_d = 0`** in every statement, and the `h_hi` limbs are not range checked (16-bit limbs with boolean carries; a range check is one lookup per limb). Both are listed for group 5.
10. **`L_Fr` refusals are at the tape level**, plus one mutant `L_Fr` proof of the failing tape refused by the stock `L_Fr` verifier (`wrong_prog_d_anchor_refused_by_stock_L_Fr_verifier`, program B run).
11. **`r_P` is a test constant** (`[1000..1007]`) and `r_L = r_P`. Production `r_P` is the run blinding of FP-3.
12. **The FV unit is a stand-in**: the N2 AIR shape with the `Fv` statement (tag 5, `fv_state_d`). The FV work AIR set is O1/O2 work and is not in T7.
13. **"A changed outcome word at the leaf is refused at `L`" reading.** The case changes one outcome payload word of the header that `L` opens (mutant `L` prover, stock `L` verifier). The header words are not in any leaf public value; `L` opens them against the `post_d` that the tree carries from the leaf. A change of `post_d` at the leaf is refused by the leaf layer (the app public values), and a change of a header word is refused at `L` (ZeroCheckFailed). Both are covered.
14. **Gemini Carbon Quint lanes not run** (lead runs them). Every circuit change is in section 7 with file:line.

## 1. Status

| Done-when item | Status | Evidence (MEASURED) |
|---|---|---|
| `wrong_prog_d` refused | PASS | N1: `wrong_prog_d_merge_refused_at_N1`, `wrong_prog_d_opening_refused_at_N1`; `L_Fr`: `wrong_prog_d_anchor_refused_at_L_Fr` (A and B), `wrong_prog_d_anchor_refused_by_stock_L_Fr_verifier` (B) |
| changed outcome word refused at `L` | PASS | `outcome_word_changed_refused_at_L`, `non_terminal_outcome_refused_at_L` (A and B), `forged_outcome_operand_refused_at_L_Fr` |
| mixed chip sets under one `key_root` accepted | PASS | `mixed_chip_sets_one_key_root_N2_accepts`, `mixed_chip_sets_N3_accepts`; refusals `key_triple_outside_key_root_refused_at_N1`, `chip_set_swap_refused_at_N1` |
| FV unit under N3 accepted | PASS | `fv_unit_proves`, `fv_unit_under_N3_accepted`; refusals `fv_row_missing_refused_at_N3`, `fv_row_missing_refused_at_N3_true_table`, `keys_d_mismatch_refused_at_FV_unit` |
| V.10 chain, D3 >= 100 every term, >= 101 every grindable term | PASS | chains A and B, `fp8_d3` all gates PASS on 8 keys each (section 4.3) |
| wrap fits if the `L_Fr` words change | not needed | `L_Fr` words 1,522 and shape unchanged from T10a (section 5) |
| `anchor_d2` tested at the `lfr_call` seam | PASS | `iface::anchor_d2` (openvm `crates/sherlock-lfr/src/iface.rs:1582`); honest anchor accepted, other program refused (section 4.2) |
| A1 FP-2 node statement | done | sections 3, 4 |
| A2 multi-segment `TAG_NODE` merge | done in the tree; `L` log tie open (pick 8) | `mixed_chip_sets_one_key_root_N2_accepts` is a binary SEG merge |
| A3 X10 (I-12, I-35) | done in `L` (pick 1) | `non_terminal_outcome_refused_at_L` |
| A4 `plan_d` in the ENC-5 42-word form | done | `sherlock-leaf/src/leaf.rs:59` `slice_plan`, `:89` `slice_plan_d` |
| B path C (I-17 `h_pre`, D10) | scoped, not built | section 6 |

Receipts: RECEIPTS (section 8).

## 2. Commits

openvm `t7-bind` (base `8672651`):
- `95a1f01` FP-2 statement AIR, slice `plan_d` in ENC-5, chip set 1 (`ExtAir`), `anchor_d2` seam.
- `a133602` root statement and seal of `L`, bound interface run, `t7_chain` example.
- `662235f` `t7_chain`: `omit` mode (V.9), the FV-row negatives, the mutant `L_Fr` stock-verifier case, the N3 fixed-point row; the `lfr_call` seam comment.

Moriarty `t7-bind`: `e2cd326a9` (NODE-004; `scripts/qa/check-chain.sh`, `check-v9.sh`, `check-v10.sh` on `t7_chain`), then the report commit (this file, `runs/`, patch series `research/sherlock-stark-code-20261112/openvm-t7/`).

stark-backend: no change.

## 3. Design as built

**Every recursion layer**: `StmtAir` at AIR 0 (`stmt.rs:697`); the verifier sub-circuit AIRs follow (`CE_AIR_ID = 1` is the cached child-verifier trace); `PermSpongeAir` last (`stmt.rs:423`, buses 960 and 961; its permutations go to the recursion permute bus through `VerifierExternalData::poseidon2_permute_inputs`). Sponge heights: First 128, Node and FV 32, Root 16.

**Leaf layer** (kind Leaf): 62 public values, the 54 app public values plus `pre_app`.

**First node N1** (`stmt.rs:853`): opens `prog_d = P2(DOM_PROG 5, image_d, plan_d, key_root, mv)`; the key leaf of pick 3 and its path into `key_root`; `h_hi = h_pre + steps` in 16-bit limbs; own `vkc = (cc_leaf, pre_ifl)`. Binary SEG merge of two shards (`stmt.rs:952`): both SEG; equal `prog_d`, `key_root`, `reg_root`, `shape_d`, `keys_d`; `post_0 = pre_1`; `h_hi_0 = h_lo_1`; FP-2 log merge (`stmt.rs:979`). A unary child passes through.

**Node and FV** (`stmt.rs:911`): `keys_d` table of pick 5; the child `keys_d` equals the own; the child `vkc` words equal the received `(cc, pre)` (`stmt.rs:817`). FV output: tag 5 with `fv_state_d` (`stmt.rs:1023`). Node and FV share one column layout.

**`L`** (`Kind::Root`, `stmt.rs:1088`): one N3 child. TAG, `h_lo`, `keys_d` and the child `vkc` pre-hash are key constants (`root_map`, `stmt.rs:1049`); the columns are `valid` plus 69 child words. It receives the 90 child words, the cached commitment and the pre-hash; it exposes the 74 `root_stmt` words in the FP-3 interface (`iface_l` `skip` field, `iface_l.rs:73`, `:95`) and sends `(prog_d, post_d)` on `SEAL_BUS` 962 to `RootSealAir` (pick 1). `RootSealAir`: 74 columns, 24 public values `[OPC, input_d, OPO]` with `OPC = P2(DOM_PROGC 6, r_P, prog_d)`.

**Universal key set** (`t7_chain.rs` `keys()`): app per chip set (set 0: the T6 slice AIRs; set 1 adds `ExtAir`, `sherlock-leaf/src/ext.rs:18`); leaf layer per set; N1 per set (one pre-hash for both sets); N2 over N1; FV unit over N1; N3 over N2 (and over N3, fixed point); `key_root` over the 2 key triples; `L` over N3 pinning `keys_d` and the N3 pre-hash; `L_Fr` over `L`.

**Chain**: 2 shards, leaf x2, N1 binary, N2 unary, N3 unary, `L`, `L_Fr`.

## 4. Runs

### 4.1 Chains (MEASURED; `runs/chain-A.json`, `runs/chain-B.json`; logs `~/.cache/t7/logs/chain-a3.log`, `chain-b1.log`)

| | Program A `entry x;` | Program B `entry y;` |
|---|---|---|
| `prog_d` | `46650ca1…9180` | `0a8fda7b…6dd0` |
| `key_root` | `5b796143…f5f0` | same |
| `keys_d` | `1993533b…8d10` | same |
| shards, steps | 2, 5 | 2, 5 |
| fixed shard heights | yes | yes |
| wall (s), shared slot | 833 | 1,094 |
| app prove (s) | 13.8 | 13.2 |
| leaf, N1, N2, N3 prove (s) | 8.4, 8.1, 5.8, 4.1 | 8.2, 8.0, 5.2, 4.2 |
| `L` prove (s) | 93.8 | 89.6 |
| `L_Fr` keygen, prove (s) | 73.1, 261.9 | 74.9, 246.0 |
| `L` public values | 12,190 | 12,190 |
| `L_Fr` public words | 1,522 | 1,522 |

Times ran in a shared heavy slot (not `--exclusive`), so they are MEASURED under load, not timing gates.

Max log heights (bound): app 17 (24), leaf 17 (21), N1 19 (19, at the bound), N2 16 (19), N3 16 (19), `L` 16 (20), `L_Fr` 19 (20). G1 accepted on every layer.

Keys (MEASURED, `T7KEY` lines and `vk_json`):

| Key | AIRs | Columns | Max interactions | VK bytes |
|---|---:|---:|---:|---:|
| app set 0 | 7 | 2,361 | 145 | 70,564 |
| app set 1 | 8 | 2,363 | 145 | 70,610 |
| leaf set 0 / 1 | 40 | 1,597 | 78 | 81,473 / 81,513 |
| N1 (both sets, one pre-hash) | 41 | 2,435 | 144 | 104,524 |
| N2 | 41 | 1,885 | 120 | 93,031 |
| FV unit | 41 | 1,885 | 120 | 91,804 |
| N3 | 41 | 1,885 | 120 | 93,026 |
| `L` | 46 | 1,986 | 93 | 243,121 |
| `L_Fr` | 15 | 561 | 46 | 45,528 |

T6 for comparison: leaf 40/1,675, N1 to N3 40/1,637/84, `L_iface` 44/1,893, `L_Fr` 15/561 at 1,528 words. The first `L` design went over the bounds (`Columns { set: "L", got: 2047, bound: 2000 }`, then `Interactions { got: 110, bound: 100 }`); the key constants of `root_map` and the separate `RootSealAir` bring it to 1,986 columns and 93 interactions.

### 4.2 Cases (MEASURED; `runs/cases.json`, `runs/chain-A-cases.json`, `runs/chain-B-cases.json`)

Chain cases, programs A and B: `chain_{A,B}_L_honest` accept; `outcome_word_changed_refused_at_L` and `non_terminal_outcome_refused_at_L` refuse (ZeroCheckFailed, mutant `L` prover, stock `L` verifier); `lfr_tape_honest` accept; `wrong_prog_d_anchor_refused_at_L_Fr` and `forged_outcome_operand_refused_at_L_Fr` refuse (1 tape assertion each); B also `wrong_prog_d_anchor_refused_by_stock_L_Fr_verifier` refuse. The honest anchor is `iface::anchor_d2(r_L, prog_d)` of the program; the wrong one is that of the other program.

Binding cases (`t7_chain cases`, 12 cases, all PASS):

| Case | Expect | Result |
|---|---|---|
| `mixed_chip_sets_one_key_root_N2_accepts` (shard 0 set 0, shard 1 set 1, N1 unary each, N2 binary merge) | accept | Ok |
| `mixed_chip_sets_N3_accepts` | accept | Ok |
| `wrong_prog_d_merge_refused_at_N1` | refuse | SumClaimMismatch |
| `key_triple_outside_key_root_refused_at_N1` | refuse | SumClaimMismatch |
| `chip_set_swap_refused_at_N1` | refuse | SumClaimMismatch |
| `wrong_prog_d_opening_refused_at_N1` | refuse | ZeroCheckFailed |
| `fv_unit_proves` | accept | Ok |
| `fv_unit_under_N3_accepted` | accept | Ok |
| `fv_unit_on_table_without_its_row_proves` (the FV unit alone cannot see its row) | accept | Ok |
| `fv_row_missing_refused_at_N3` (subtree and N3 on a table without the FV row) | refuse | SumClaimMismatch |
| `fv_row_missing_refused_at_N3_true_table` | refuse | SumClaimMismatch |
| `keys_d_mismatch_refused_at_FV_unit` | refuse | SumClaimMismatch |

### 4.3 D3 (MEASURED; `runs/d3-A.json`, `runs/d3-B.json`)

`fp8_d3` on each chain plus the FV unit key, at the measured shapes, the vk shape and the bounds: all gates PASS on both. Weakest term per key: app shift 101.0891; leaf shift 101.0110; N1, N2, N3, FV shift 101.1643; `L` measured shift 101.1427, vk and bounds mu 101.0730; `L_Fr` query 101.1435. No term is under 100, and no grindable term is under 101.

### 4.4 V.9 omit cases (MEASURED; `runs/omit.json`)

`t7_chain omit` (log `~/.cache/t7/logs/omit1.log`): 59 cases, all PASS; every AIR is required in the app, leaf, N1, N2, N3, FV and `L` keys.
- App (7 slice AIRs, 2,361 columns, 145 max interactions): each AIR left out is refused on the prover path, by the stock app verifier, and by the leaf layer over the stripped proof (21 cases).
- N1 and N3, each without `StmtAir`, `ProofShapeAir` or `PublicValuesAir`: refused on the prover path, by the stock verifier, and by the parent (N2 over N1, `L` over N3) over the stripped proof (18 cases).
- `L` without `StmtAir`, `ProofShapeAir`, `PublicValuesAir` or `RootSealAir`: refused on the prover path and by the stock `L` verifier; the `L_Fr` tape generator refuses the stripped proof at its own shape and at the pinned honest shape (16 cases).
- Honest: app, leaf, `L`, `L_Fr` tape (4 cases).

## 5. Wrap fit

The `L_Fr` public words are 1,522, the same count as the T10a `lfr_call` run, and the `L_Fr` key shape is the same: 15 AIRs, 561 columns, stacked widths 33 and 10, max log height 19 (MEASURED). The wrap relation depends only on the `L_Fr` key shape and the public word count, so it is unchanged from T10a: 1,680,729 rows at k = 21 (bound 1,782,573) and a 1,325-byte key (bound 50,000) (DERIVED from the T10a MEASURED numbers). The `A_0 = C_P` check is in the `L_Fr` tape, not in the wrap. No wrap run in T7.

## 6. Path C (scoped, not built) and D10

**What path C needs** (ARCHITECTURE section 1.6, I-17, `PR` 4.3; lead ruling 2026-11-13 21:40): one leaf, one leaf layer at internal parameters (ARCHITECTURE parameter table row "Internal (and the leaf layer on path C)"), then `L`, `L_Fr`, wrap. The leaf-layer key is per program and per chip set, so the `L` key must not depend on it: `L` takes the child verifier as a cached-trace commitment `X` (one more public value), proves `P2(DOM_KEYDAG, entry, path, enc(X)) ∈ key_root`, and opens `prog_d` to `key_root` from the leaf-layer statement; the wrap runs `Canon` on the limbs of `X` and equates `X` with the cached commitment of `L`.

**Why the current `L` depends on its child.** The recursion verifier bakes the child per-AIR metadata into the proof-shape AIR as key constants. Seams (openvm):
- `crates/recursion/src/proof_shape/proof_shape/air.rs:299-361`: per-AIR constants selected by the `idx_encoder` flags: `need_rot`, `main_width`, `num_pvs`/`has_pvs`, `num_interactions` and limbs, `is_required`, `min_cached`, preprocessed log height, width and commitment, cached widths; also `:450` and `:885` (`per_air.len()`).
- `crates/recursion/src/proof_shape/mod.rs:45` `AirMetadata`, and the encoder size (number of child AIRs).
- `crates/recursion/src/system/mod.rs:585-600`: `trace_height_constraints` and `max_interaction_count` of the child.
- `crates/recursion/src/batch_constraint/mod.rs:158-159`: `l_skip` and the child `max_constraint_degree` (fix at the set bound 4).
- GKR, stacking and WHIR take the child system parameters, so the child must be at internal parameters (as the architecture says for path C).

**Work** (ESTIMATE): a cached ChildMeta table AIR keyed by `air_idx` (about 25 to 30 columns by 64 rows, padded to a fixed `MAX_AIRS`), with the proof-shape constants read by lookup; the proof-shape eval and trace rewrite; `X` as an `L` public value; a KEYDAG path in `L` (one row per level, about 45 columns); the `L` First-style `prog_d` opening; a second leaf-layer key per chip set at internal parameters. About 2 to 3 days of circuit work plus a Quint lane, and all of it in the recursion crate and `sherlock-lfr`.

**Why not built tonight.** `L` is at 1,986 of the frozen 2,000 columns (MEASURED; `G1 Columns` is a per-key total). The ChildMeta lookup columns, `X`, the KEYDAG path and the `prog_d` opening add about 75 columns to `L` (ESTIMATE), over the bound. Options for the lead: (a) raise the `L` column bound (a frozen-parameter change: owner); (b) slim `L` first, for example by moving the 69 child word columns of the root statement into `RootSealAir` reads and dropping the duplicates, then add path C. Pick (b): it stays inside every hard rule. It is the remaining path C item.

**Wrap cost** (ESTIMATE, lead number): about 120 rows for `Canon` on the limbs of `X` and the equality, plus 1 public word; the margin is 101,844 rows (DERIVED from T10a 1,680,729 against 1,782,573).

**D10 on path C** stays open with path C: measure `w_stack` of `L` with the leaf-layer child (`L_cached`). The full-tree `L` here has stacked widths 17 and 1 (MEASURED) and passes D3 (section 4.3).

**I-17 `h_pre`** (reading, not tested): the code check of `PR` 4.3 decides whether the child `h_pre` joins the KEYDAG preimage. The pre-hash of a child key commits the whole child verifying key, and the cached commitment `X` commits only the child-verifier trace that `L` reads. If the ChildMeta table carries every per-AIR constant of the child (including the preprocessed commitments), `X` alone fixes what `L` checks and `h_pre` is redundant; otherwise `h_pre` must join. Pick: put `h_pre` in the preimage, `P2(DOM_KEYDAG, entry, path, enc(X), h_pre)` (8 more words in the leaf hash, no column cost in the path rows), and drop it only if the code check shows ChildMeta is complete.

## 7. Circuit changes for the Gemini Carbon Quint lanes (lead runs them)

All in openvm `t7-bind`:
1. `crates/sherlock-lfr/src/stmt.rs:423`, `:436` `PermSpongeAir` (sponge over the recursion permute bus, buses 960/961).
2. `stmt.rs:697`, `:741` `StmtAir` (row rules `:758`, receives `:773`).
3. `stmt.rs:817` NODE child check (child `vkc` = received `(cc, pre)`).
4. `stmt.rs:853` First: `prog_d` opening, key leaf, `key_root` path, `h_hi`.
5. `stmt.rs:911` Node and FV: `keys_d` table, own `vkc` selector.
6. `stmt.rs:952` binary SEG merge; `:979` FP-2 log merge; `:1023` FV output.
7. `stmt.rs:1049` `root_map`, `:1088` `eval_root` (`L` root statement).
8. `stmt.rs:1150`, `:1172` `RootSealAir` (X10, `OPC`, `OPO`).
9. `crates/sherlock-lfr/src/lverify.rs:1835` `gen_ops_check` (operand tie and `A_0 = C_P` in the `L_Fr` tape).
10. `crates/sherlock-lfr/src/iface_l.rs:73`, `:95` `skip` (the `L` statement words left to the root statement AIR).
11. `crates/sherlock-leaf/src/ext.rs:18`, `:28` `ExtAir` (chip set 1).
12. `crates/sherlock-leaf/src/leaf.rs:59` `slice_plan` (ENC-5 42-word `plan_d`).

## 8. Receipts and reproduce

RECEIPTS_SECTION

Reproduce: `scripts/qa/check-chain.sh chain|cases|omit` (defaults: openvm `~/.cache/m7-wrap/src/openvm-t7`, target `~/.cache/t7/target-ovm`, out `~/.cache/t7/check-MODE`; `T7_EXCLUSIVE=1` for `--exclusive`). The run copies in `runs/` are from `~/.cache/t7/out/`. Patch series: `research/sherlock-stark-code-20261112/openvm-t7/` (`git format-patch --binary 8672651..t7-bind`).
