# T10m results: T10 merged into `tested` (T8b), rekeyed and rerun

Row T10m of `research/sherlock-tested-design-20261113/LOOP.md` (lead-created 2026-11-14). Trees, both on branch `t10m` / `t10mm`: Moriarty `~/Projects/Moriarty-wt/t10m` (base `tested` = `b1ff56ba1`, T8b; merges `t10-bound` at `30567a640`), openvm `~/.cache/m7-wrap/src/openvm-t10mm` (base `tested` = `89bee35`, T8b; merges `t10-bound` at `33371a6`). stark-backend unchanged: the openvm `[patch]` points at `../stark-backend-t8b` (`deeeb0a`, the same commit as `../stark-backend-t7c`). Run state, logs and outputs: `~/.cache/t10mm/`.

Labels: MEASURED (read from a run record), DERIVED (computed from MEASURED numbers or source by a stated rule), ESTIMATE.

## 0. For the lead

**Merge: done.** `t10m` and `t10mm` hold T8b and T10 with both rows' behavior. Three defects found and fixed by the run: the path C DAG leaf index (`KEYDAG_IDX` 2 to 3), the `super::core` import that the merge resolution dropped (path C became two shards), and `t8_gp`'s N1 without T10's `h_hi` table (section 1, 2).

**Bounds hold (MEASURED):** `L` **1,995 of 2,000 columns**, `StmtAir` **99 of 100 interactions**, G1 accepted; wrap **1,685,845 rows at k = 21** (gate 1,782,573); wrap VK **1,325 bytes** (bound 50,000). No bound and no hard rule changed.

**Every T10 and T8b gate passes on one merged tree** (section 3): ledger 9.1.0.1 `Ok` with every refusal; devnet `acceptRun` included with R1 to R4 refused and the privacy scans passing; check-cases 15, V.9 70 cases, V.10 A and B, D3 on every key (weakest leaf shift 101.0110; MERGE-WORK, NW, NW2 shift 101.1643), path C 25 of 25, SG-6 (GPU 0.1500 s per proved step), gp2 48 of 48, x7 22 of 22, `lfr_e3 0` and `i7_decode`.

**The two T10 leftovers: fixed** (section 2). **Preview: not run** (owner-gated). **No push.**

**Rekey:** `key_root` and `keys_d` change (section 3.1), so the T10 Preview receipts and any T10-era key hashes are stale for this tree; Preview, if the owner runs it, uses `chain.sh preview` with `T10_HOME=~/.cache/t10mm` and the `t10m`/`t10mm` trees.

## 1. The merge

**Conflicts and how each was resolved** (both rows' behavior kept):

| file | resolution |
|---|---|
| openvm `Cargo.toml` | the T8b stark-backend copy `../stark-backend-t8b` (T10 named `../stark-backend-t7c`; both are `deeeb0a` with no change) |
| openvm `crates/sherlock-leaf/src/leaf.rs` | T8b's imports (`VExtra`, `DecVAir`, `R_CNULL`, `R_PUBV`) with T7c's `use super::core::{...}` form; see the defect below |
| openvm `crates/sherlock-wrap-export/examples/t7_chain.rs` | `Keys` holds `nw`, `nw2` (T8b) and `pc` (T7c, T10) |
| Moriarty `research/sherlock-stark-code-20261112/README.md` | both the `openvm-t8b/` and the `openvm-t10/` rows and paragraphs |

**Semantic fixes the merge needed** (not textual conflicts):

- **`KEYDAG_IDX` 2 to 3** (openvm `9f3b521`). T8b puts three chip-set leaves (sets 0, 1, 2) in `key_root`, so the path C DAG key leaf is leaf 3. The key tree pads to 4 leaves either way, so the path depth stays 2.
- **`leaf.rs` must import `super::core`** (openvm `d26453c`). The first resolution took T8b's `crate::core` import. The S = 8 copy of the file (`crate::s8::leaf`, T7c) then used the S = 4 core, `entry x;` became two shards on path C, and `key-A` refused at `t7_chain.rs:482` ("path C: one leaf", MEASURED, first attempt of the run). With `super::core` the S = 8 bucket is one shard (`t7_chain s8app`: 1 shard, proved and verified). The kernel uses of `crate::core` (split CORE, `FrameAir`) stay as T8 wrote them.

**What the merged tree holds:** T8b's `keys_d` with 7 node rows (N2 over N1, N3 over N2, N3 over FV, N3 over N3, NW over MERGE-WORK, NW2 over NW, N3 over NW2), the kernel CORE step words as columns, chip set 2 (the GP-2 value set) in `key_root`; T10's real `anchor_d2(r_L, prog_d)`, `gen_ops_check` in the `L_Fr` tape, the `L` log tie (onion AIR, `TIE_BUS`), the N1 `h_hi` range table, slim `L` and path C.

## 2. The two T10 leftovers

- **`t8_gp` uses the current `L` key** (openvm `c987b71`). `t8_gp chain|cases|omit` now run the production `L` of `t7_chain` (T7c path C, T10 log tie and onion AIR); the T7-era `L` copy and its `T8_GP_OLD_L` gate are removed (643 lines). `t8_gp keys()` puts the path C DAG key leaf after the chip-set leaves, so `gp2`, `x7` and `ladder` run under the same `key_root` as `t7_chain`, `L`, `L_Fr` and the wrap; before, `t8_gp`'s `key_root` had no DAG leaf and differed from the universal one. `t7_chain` exposes `path_c()` (the path C keys, shared), and `chain`, `cases_mode`, `omit`, `PathC` and `KEYDAG_IDX` are `pub` for `t8_gp`.
- **`t8_gp` N1 key** (openvm `5b7ada8`, found by this run). `t8_gp`'s own `layer()` built N1 without T10's `HhiRangeAir`, while the First statement AIR looks up the `h_hi` limbs on bus 963: `gp2` refused its honest First proof (`t8_gp.rs:273`, `LogupZerocheck(NonZeroRootSum)`, MEASURED) and the `t8_gp` N1 key was not the universal one. The layer now pushes `HhiRangeAir` after `PermSpongeAir` with its trace, as `t7_chain` does, and `keys()` prints `key_root`, `keys_d` and the N1 pre-hash (`T8GPKEYS`), which equal `t7_chain`'s (MEASURED).
- **`lfr_e3` panic** (openvm `be62bb8`). Cause (MEASURED): the J8 bench `L` proof in `~/.cache/e3-iface/fixture-j8` has 12,164 public words; `Dims::FULL` has 12,166 since T9's FP-3 (5 lane pairs, owner ruling O7), so `gen_r_check` failed its length assertion at `lverify.rs:1648`. `lfr_e3` now reads `~/.cache/e3-iface/fixture-t10m`, written by `sherlock-lfr` `iface_fixture` on this tree, and checks the word count before the tape, with the regeneration command in the message. The new fixture runs (section 3.5). With it the stock verdict of `wrap_x_not_cached_commitment` is `ok`, so the override T10 put in `i7_decode.rs:89` is no longer needed; this row leaves the test unchanged (follow-up: drop the override and point the default `E5_FIXTURES` at the new decode file).

## 3. Measurements (openvm `d26453c`, Moriarty `38290a7a6`; MEASURED unless marked)

### 3.1 Keys (`~/.cache/t10mm/key/keys.json`)

| key | G1 | AIRs | columns | max interactions |
|---|---|---|---|---|
| `L` over N3 (one key over N3 and both path C leaf layers) | accepted | 48 | **1,995** (bound 2,000) | **99** (`StmtAir`; bound 100) |
| `L_Fr` | accepted | 15 | 561 | 46 |
| MERGE-WORK over N3 | accepted (`merge_work`) | 49 | 5,284 | 183 |
| NW over MERGE-WORK | accepted | 41 | 2,285 | 182 |
| NW2 over NW, N3 over NW2, N2, N3, FV | accepted | 41 | 2,284 | 182 |
| app chip set 0, 1, 2 | accepted | 7, 8, 7 | 2,361, 2,363, 2,423 | 145, 145, 147 |

N3 over NW2 equals N3 and N3 over N3 equals N3 (`T7KEY` lines). `L` over each path C leaf layer is the same key as `L` over N3. `L` max log height 16 (bound 20); `L_Fr` max log height 19, widths [34, 10], 1,522 public words. `L` cached meta rows: N3 26,303, path C 19,880 and 19,888, height 32,768 (`key-A.log`).

**The rekey.** `key_root` `0fade1a4...8051` and `keys_d` `15ed5153...a738` (T10: `543f5320...f038`, `1cf4f508...03ae`); the `L` and `L_Fr` pre-hashes change with them (`key-A.log` `T10CALL`). The `L` shape equals T10's (1,995 columns, 99 interactions): `keys_d` enters the root statement as one digest, so the three T8b node rows add no `L` column.

### 3.2 Wrap (`~/.cache/t10mm/logs/wrap-cost.log`, `wrap-key.log`, `wrap-B.log`)

| item | value |
|---|---|
| wrap rows at k = 21 | **1,685,845** (gate 1,782,573), within gate |
| wrap key (Midnight vk) | **1,325 bytes** (bound 50,000) |
| proof | 2,912 bytes, 1,026 public inputs; MockProver honest ok, changed public input and changed proof value refused; library verify ok |
| program B | same wrap key (asserted byte-equal), 1,685,845 rows |

### 3.3 Ledger 9.1.0.1 (stock crates, rev `a2b30128`; `ledger-A.log`, `ledger-B.log`)

Program A: `Ok(())`. Refused: 7 changed public inputs, 7 changed named lanes (lane 0, anchor lane 10, `claim_pub` lanes 20 and 34, `outcome_pub` lane 39, the middle and the last lane), 3 flipped proof bytes. Program B's proof on A's anchor (lane 10 differs) and on A's statement (lanes 10, 20, 34 differ): refused. `L_Fr` tape of program B against A's ledger anchor: refused with exactly one failed assertion (`A_0 = C_P`, `b-on-a.log`, exit 3).

### 3.4 Local devnet (`~/.cache/t10mm/devnet/t10a-state.json`, `devnet-call.log`)

| item | value |
|---|---|
| deploy | `SherlockLedger` at `d515978b...caeae`, block 22,586, included; anchor = `C_P` of A |
| `acceptRun` of A | included, tx id 45, block 22,728, `SUCCESS`; `claim_pub` in `runs`, `runs` size 1, anchor and keys unchanged, the included program equals the proven one |
| refusals | R1 changed outcome, R2 one proof bit flipped, R4 program B against A's anchor, R3 replay after A: all refused by the node (R3 at the proof stage: `failed to verify proof`) |
| privacy | the scan of the call vector, the included transcript and the ledger state for `prog_d`, `pack(prog_d)`, `pack(r_P)` and the `R` operands passes, and fires on its controls; the small-word scan (`n_steps`, `h_hi` limbs, `r_P` words) passes; verdict `pass: true` |

The lane map and `PUB` entries of the bound run are asserted equal to the call vector inside `t10_run` (T10: 4 pairs, 4 entries); that assertion held in `key-A` and `devnet-call` (DERIVED from the passing run).

### 3.5 Gates

| gate | result | record |
|---|---|---|
| check-cases (T7 binding cases and T10 cases) | PASS, 15 cases | `check-cases.log` |
| V.9 omit | PASS, 70 cases, every AIR required in app, leaf, N1, N2, N3, FV, `L` (incl. N1 `HhiRangeAir`, `L` `OnionAir`); app shape 7 AIRs, 2,361 columns, 145 interactions | `check-omit.log` |
| V.10 chain A and B | PASS: 2 shards, 5 steps, fixed heights, G1 accepted, 10 cases each (incl. the `L` log tie cases), N3 fixed point, `L_Fr` 1,522 words; two programs, one `key_root`, one `keys_d` | `check-chain.log` |
| D3 on the 8 chain keys, both programs | PASS: every term at least 100, every grindable term at least 101; weakest leaf shift 101.0110, app shift 101.0891, `L` vk and bounds mu 101.0730, `L_Fr` query 101.1435, nodes shift 101.1643 | `check-chain/chain-{A,B}/d3.json` |
| T7c path C (`t7_chain pathc`) | PASS, 25 of 25 cases (one leaf, leaf layer, `L`, `L_Fr` on the universal `L` key); D3 PASS on the path C key set | `pathc.log`, `pathc-d3.log`, `out/pathc/cases.json` |
| SG-6 | PASS: GPU 0.1500 s per proved step, CPU 1.9855 s (5 steps, 8 slots); the stock CPU verifier accepts every shard proof; shared host, not a timing | `sg6.log`, `out/sg6/sg6.json` |
| T8b gp2 (`t8_gp gp2`, openvm `5b7ada8`) | PASS, 48 of 48 on the stock verifier: the Halted producer sealed, SEAL-WITH-DEPS of the consumer, the twins refused; `t8_gp` prints `key_root` `0fade1a4...` and `keys_d` `15ed5153...`, equal to `t7_chain`'s | `gp2.log`, `out/gp2/gp2.json` |
| T8b x7 (`t8_gp x7`, openvm `5b7ada8`) | PASS, 22 of 22: NW, NW2 and N3 over the honest MERGE-WORK proof accepted; forged NW output, wrong key rows, forged step-word column refused | `x7.log`, `out/x7/x7.json` |
| D3 on MERGE-WORK, NW, NW2 (with the chain A keys) | PASS: MERGE-WORK (`merge_work`, 5,284 columns) weakest shift 101.1643 at vk and bounds (query 101.1643, mu 101.6304, LogUp 102.7205, fold 102.6276); NW and NW2 shift 101.1643; no measured-shape row for these three (as in T8b) | `d3work.log`, `check-chain/d3work/d3.json` |
| `iface_fixture` (`L_iface` at `Dims::FULL`, 11 cases) | written to `~/.cache/e3-iface/fixture-t10m`, each proof verified by the stock verifier | `iface-fixture.log` |
| `lfr_e3 0` (decode cases) | PASS, no panic: 46 decode cases written (`~/.cache/t10mm/e5fx/lfr-e3-decode.json`); `wrap_x_not_cached_commitment` now has the stock verdict `ok` (the decoder keeps only range and `Canon` of `X`) | `lfr-e3.log` |
| `i7_decode` (wrap decoder on every case) | PASS, 2 of 2 tests with `E5_FIXTURES=~/.cache/t10mm/e5fx`; honest decode ok, changed statement refused, `wrap_x_not_cached_commitment` accepted | `i7-decode.log` |

## 4. Not run, and open

- **Preview**: not run (owner-gated in this row). `chain.sh` `preview` stage is unchanged except its declared memory (64G).
- **`chain.sh` memory**: `devnet-call` and `preview-call` now declare 64G in place of 80G (heavy.slice `MemoryHigh` is 64G; the lead raised it for part of this run, see the log). No bound or hard rule changed.
- **Run history**: units t10mm-run to t10mm-run7 and t10mm-gp2. run stopped at `key-A` (the `super::core` defect, section 1); run2 to run6 resumed from the stage state after a lead restart for the heavy-slot budget and three host reboots (11:39, 11:49, 13:49; the first two from another session's GPU confidential-compute query); run6 stopped at `gp2` (the `t8_gp` defect, section 2); gp2 ran alone as t10mm-gp2 and run7 finished the rest. The chain, gates, path C and SG-6 records come from openvm `d26453c`; gp2, x7, the D3 of MERGE-WORK, NW, NW2 and the `lfr_e3` stages from `5b7ada8`, which changes `t8_gp.rs` only.
- The T10 and T8b open items stand: the interim child log rule `(D_EMPTY, 0)`, one contract for all programs (SRC-007), `L` over a SET root (GP-2 to the ledger), the X8 in-circuit Compact verifier, the STACKS AIR.

## 5. Circuit changes for a Carbon Quint lane

The merge changes no constraint. The only code change that reaches a circuit is the `KEYDAG_IDX` leaf index (a witness position in `key_root`; the `KeyPathAir` depth stays 2) and the `super::core` import (restores the T7c S = 8 core; no constraint differs from T7c and T8b). The `t8_gp` change makes its N1 the `t7_chain` N1; it adds no constraint. The Carbon lanes of T10 (T10 RESULTS section 3) and T8b (Q1 to Q7) cover the merged circuits.

## 6. Commits

openvm `t10mm` (series `research/sherlock-stark-code-20261112/openvm-t10m/`, first-parent, round trip checked): `9f3b521` (merge), `c987b71` (`t8_gp`), `be62bb8` (`lfr_e3`), `d26453c` (`super::core`), `5b7ada8` (`t8_gp` `h_hi` table). Moriarty `t10m`: `91079c7ce` (merge), `38290a7a6` (runner, `chain.sh` memory), and the commit of this report with the patch series.
