# T10 results: the bound wrap and the design contract

Row T10 of `research/sherlock-tested-design-20261113/LOOP.md`. Final trees, on branch `t10-bound` in both repos: Moriarty `~/Projects/Moriarty-wt/t10-bound` (base `316b17682`; merges `t7c-pathc` at `c070c2f57` and `tested` at `07af1c450`), openvm `~/.cache/m7-wrap/src/openvm-t10` at `33371a6` (base `662235f`, the T7 tip; merges `t7c-pathc` at `3607e95` and `tested` at `a645de3`, T8). stark-backend unchanged: the openvm `[patch]` points at `../stark-backend-t7c` (`deeeb0a`). The final run's state, logs and outputs are under `~/.cache/t10m/`; the run on the pre-merge tree is under `~/.cache/t10/`.

Labels: MEASURED (read from a run record), DERIVED (computed from MEASURED numbers or source by a stated rule), ESTIMATE.

## 0. For the lead

The final keys cover T7, T7c, T8 and T10 together. Every done-when check passes on the merged tree (MEASURED, section 2), except the Midnight Preview step, which the lead runs (item 6).

1. **The `L` log tie (T7 pick 8): built.** The J8 onion AIR (`iface_l::OnionAir`, 70 columns) is in the production `L`. `LogEntry` and `LogValue` send their words on the onion buses, and the root statement AIR receives the onion's `(d, n)` on `TIE_BUS` (903) as the exposed `log_d`, `log_n`. So the exposed log words are the onion of the `L` entry and value tables, and a changed `log_d` word or a changed logged value is refused at `L` (cases `exposed_log_d_changed_refused_at_L`, `log_value_changed_refused_at_L`). The `L` key: G1 accepted at **1,995 columns** (bound 2,000), 48 AIRs, `StmtAir` at **99 of 100 interactions** (MEASURED, `~/.cache/t10m/key/keys.json`).
2. **Interim rule in the tie, for lead confirmation.** The leaves log nothing yet, so the child log of `L` (N3 on the full tree, the leaf on path C) must be `(D_EMPTY, 0)`, checked in the root statement AIR on both paths; the child log words stay `root_map` columns as T7c asks. The `L` tables then carry the run's log (the reservation `R`, the `PUB` entries) and the tie binds them to the exposed words. When the leaves get a log section, the child log replaces the `(D_EMPTY, 0)` check by a log-tree merge with the onion (follow-up, not built).
3. **`h_hi` range check (T7 pick 9): built.** N1 has `HhiRangeAir` (2 columns, 2^16 rows, bus 963), and the First statement AIR looks up each of the four `h_hi` limbs of every enabled row. A limb of 2^16 is refused at N1 (case `h_hi_limb_over_16_bits_refused_at_N1`).
4. **The `X` check is in `L_Fr` only (lead accepted at T7c).** The wrap decoder (`tools/sherlock-wrap/src/decode.rs:366-368`) and the Rust reference decoder (`crates/sherlock-lfr/src/iface.rs:747-750`) drop the equality with a pinned `X` and keep the range and `Canon` of `X`. The wrap test `i7_decode` now expects `wrap_x_not_cached_commitment` to decode; its T9 fixture verdict for that case is overridden in the test, because `lfr_e3 0` (the fixture generator) panics on the old J8 bench `L` proof at `lverify.rs:1648` (not rerun here).
5. **N-11, I-23 and I-24: AFK pick, for owner confirmation at T15.** One contract `SherlockLedger` (`acceptRun`, `acceptCollection`, `openClaim` over one `runs` map, N-11 option (a)). Its anchor is a deploy-time constant at lane 10; T10 sets it to the program's `C_P = anchor_d2(r_L, prog_d)`, and `L_Fr` asserts `A_0 = C_P`. So one deployed ledger accepts the runs of one program (the sealed `program_d` of SRC-007), and one universal key set serves every deployment. One contract for all programs (GP-1) needs the anchor as call data checked against a program registry, which is the owner's SRC-007 decision. ARCHITECTURE: I-23 and I-24 FROZEN with this pick, N-11 carries the pick.
6. **Preview: the lead runs it.** The agent permission check refused the Preview deploy from this lane, and the owner gave the approval to the lead. Command (after this row's final run, which it reuses):

   ```
   ~/Projects/Moriarty-wt/t10-bound/research/sherlock-t10-bound-20261114/preview.sh
   ```

   It runs `chain.sh preview` with `T10_HOME=~/.cache/t10m` and the final trees: `t10a-ledger-run.mjs deploy`, then `call` with `T10_NET=preview` (pinned Preview genesis `0x3c096de2...3796`; the OP-PV-WALLET wallet, read in-process from the wallet directory as `preview-session.mjs` reads it, address asserted, nothing printed). The wallet held 8,583,829,478,692,841,287 DUST and 5,000,000,000 NIGHT at tip 104,980 (MEASURED, `preview/wallet-status-before.json`), so no faucet step. Expected, as on the devnet run of section 2.4: deploy `included` with the three entry keys equal to the framed wrap key (1,353 bytes, SHA-256 of the final key); `acceptRun` of program A `included` with `runs[claim_pub] = outcome_pub`; R1 (changed outcome), R2 (one proof bit flipped), R4 (program B's arguments and proof against A's anchor) and R3 (replay after A) refused (`failed to verify proof`; R3 also `claim already settled` in the local circuit); verdict `pass: true`; exit 0. Wall time ESTIMATE 20 to 60 min (wallet sync up to 30 min, one wrap proof about 7 min). Receipts: `research/sherlock-t10-bound-20261114/preview/receipts.json` (an allow-list: chain, key hashes, deploy address and tx, the `acceptRun` tx and readback, the refusal verdicts; the scan's private values are dropped). Logs: `~/.cache/t10m/logs/preview-deploy.log`, `preview-call.log`.
7. **Carbon Quint lanes (lead runs them):** section 3.

## 1. What was built

openvm (`t10-bound`, patch series `research/sherlock-stark-code-20261112/openvm-t10/`):

| commit | what |
|---|---|
| `89e57cc` | `HhiRangeAir` and the First `h_hi` lookups; the bound `lfr_call` path (`t7_chain t10`: the ORD `acceptRun` call bound to the N3 envelope, the real operands, the ledger anchor `iface::anchor_d2(r_L, prog_d)` in place of the `ANCHOR_D2` seam, `gen_ops_check` in the `L_Fr` tape); the T10 cases |
| `1f0c243` | T7c `2080b5f` cherry-picked (slim `L`) |
| `0c4f05e` | the `L` log tie |
| `466d704` | V.9 omit cases for `HhiRangeAir` (N1) and `OnionAir` (`L`) |
| `d72ad32` | merge of `t7c-pathc`: the tie on both paths (child log words stay columns), `OnionAir` after `KeyPathAir`, the T10 path on the universal `L` key with fixed heights; the reference decoder without the pinned `X` |
| `33371a6` | merge of `tested` (T8: GP-2/GP-4 node modes, MERGE-WORK key in `keys_d`); `t8_gp`'s T7-era `L` copy refuses and points at `t7_chain` |

Moriarty (`t10-bound`): `28c037eb8` (runner, ledger refusals, Preview mode, T10 cases), `f9a8f94e3` (B against A needs exactly one failed tape assertion; the small-word scan covers the ledger state), `301c27ca1` (check-chain requires the T10 omit and tie cases), the merges of `t7c-pathc` and `tested`, `4a24eb6f3` (wrap decoder without the pinned `X`; `chain.sh pathc`), `f1d88dd11` (`chain.sh preview`), and this report with `preview.sh`, the ARCHITECTURE edits and the patch series.

One command reruns every number: `research/sherlock-t10-bound-20261114/chain.sh all` (stages `ovm`, `wrap`, `devnet`, `gates`, `pathc`, `sg6`; resumable state under `$T10_HOME/state`).

## 2. Measurements (final tree, MEASURED unless marked)

### 2.1 Keys

| key | G1 | AIRs | columns | max interactions | vk bytes | record |
|---|---|---|---|---|---|---|
| `L over N3` (one key over N3 and both path C leaf layers) | accepted | 48 | 1,995 (bound 2,000) | 99 | 129,055 | `~/.cache/t10m/key/keys.json` |
| `L_Fr` | accepted | 15 | 561 | 46 | 45,528 | same |
| `MERGE-WORK<2> over N3` (T8) | accepted | 49 | 5,128 | 171 | 183,279 | same |

`L` max log height 16 (bound 20); `L_Fr` max log height 19 (bound 20), stacked widths [34, 10], 1,522 public words; `L` meta cached rows: N3 25,732, path C 19,880 and 19,888, height 32,768 (`~/.cache/t10m/logs/key-A.log`).

### 2.2 Wrap

| item | value | record |
|---|---|---|
| wrap rows at k = 21 | 1,685,845 (gate 1,782,573) | `~/.cache/t10m/logs/wrap-cost.log` |
| wrap key (Midnight vk) | 1,325 bytes (bound 50,000) | `wrap-key.log` |
| proof | 2,912 bytes, 1,026 public inputs; prove 52.2 s (A), 60.1 s (B), shared host | `wrap-key.log`, `wrap-B.log` |

### 2.3 Ledger 9.1.0.1 (stock crates, rev `a2b30128`)

Program A's proof: `Ok(())`; 7 changed public inputs and 7 changed named lanes (lane 0, anchor lane 10, `claim_pub` lanes 20 and 34, `outcome_pub` lane 39, the middle and the last lane) refused; 3 flipped proof bytes refused. Program B's proof on A's anchor (lane 10 differs) refused; on A's statement (lanes 10, 20, 34 differ) refused (`~/.cache/t10m/logs/ledger-A.log`, `ledger-B.log`).

`L_Fr` tape, program B against program A's ledger anchor: refused with exactly one failed assertion (`A_0 = C_P`) (`b-on-a.log`).

### 2.4 Local devnet (`chain.sh devnet`)

| item | value |
|---|---|
| deploy | `SherlockLedger` at `fa7534b4...28b4`, block 19,748, `SUCCESS`; anchor = `C_P` of A |
| `acceptRun` of A | included, tx id 43, block 19,855; `runs[claim_pub] = outcome_pub`, `runs` size 1, anchor and keys unchanged |
| call vector | 1,026 words; anchor at lane 10, `claim_pub` at 20 and 34, `outcome_pub` at 39; the bound run has the 4-pair lane map and 4 `PUB` entries (`t10_run`, decoded and asserted equal to the call vector) |
| refusals | R1 changed outcome, R2 flipped proof bit, R4 program B against A's anchor, R3 replay: all `refused-by-node` (`failed to verify proof`); R3 also `claim already settled` in the local circuit |
| privacy | no `pack(prog_d)`, `prog_d` word, `pack(r_P)` or `R` operand on the call vector, the included transcript or the ledger state; the small words (`n_steps`, `h_hi` limbs, `r_P` words) on no lane off the template and not in the state; the scanner fires on its controls |

Record: `~/.cache/t10m/devnet/t10a-state.json`, `~/.cache/t10m/logs/devnet-call.log`.

### 2.5 Gates on the final tree

| gate | result | record |
|---|---|---|
| check-cases (T7 binding cases and the T10 cases: changed leaf `post_d` refused at the leaf, `h_hi` limb of 2^16 refused at N1) | 15 PASS | `~/.cache/t10m/logs/check-cases.log` |
| V.9 omit (every AIR required; the N1 `h_hi` table and the `L` onion AIR omitted are refused) | PASS, 70 cases | `check-omit.log` |
| V.10 chain A and B (app to `L_Fr` on one universal key set; the `L` log tie cases; B against A's anchor at the `L_Fr` tape) | PASS, 10 cases each, two programs with one `key_root` and one `keys_d` | `check-chain.log` |
| D3 on every key | PASS on 8 keys per program; every term at least 100 and every grindable term at least 101; weakest `L` vk `mu` 101.0730, `L_Fr` `query` 101.1435, node `shift` 101.1643 | `check-chain.log` |
| T7c path C (`t7_chain pathc`) | PASS, 25 cases; `L` max log height 16 on both paths, `L` stacked widths [17, 1], `L_Fr` [34, 10]; D3 PASS on the path C key set | `pathc.log`, `pathc-d3.log`, `~/.cache/t10m/out/pathc/cases.json` |
| SG-6 (GPU seconds per proved step, stock CPU verifier on each proof) | PASS: GPU 0.1616 s per proved step, CPU 1.8410 s (5 steps, 8 slots), the stock CPU verifier accepts every shard proof; shared host, not exclusive (`runs.log` sg6 start 09:48:15, end 10:03:11, rc 0; unit t10-sg6b) | `sg6.log`, `~/.cache/t10m/out/sg6/sg6.json` |

The path C run closes T7c's open item: `3607e95` (keygen-checked only at T7c) is now proved and verified through the path C and full-tree cases.

### 2.6 The run on the pre-merge tree

openvm `466d704` (T7 plus the T10 commits, before T7c and T8), `~/.cache/t10/`: `L` 47 AIRs, 1,918 columns, G1 accepted; wrap 1,680,729 rows, key 1,325 bytes; ledger and B-against-A refusals as above; devnet deploy at block 18,876, `acceptRun` at block 18,946 with R1 to R4 refused; check-cases 15 PASS, V.9 PASS (66 cases), V.10 A and B PASS with D3 PASS on 8 keys each (weakest `L` vk `mu` 101.0730, `L_Fr` `query` 101.1435). SG-6 did not build there (the systemd unit's `PATH` lacked `/opt/cuda/bin`; `chain.sh` now puts `/opt/cuda/bin` on its `PATH`).

## 3. Circuit changes for Carbon Quint lanes

openvm `33371a6`:

1. `h_hi` range: `crates/sherlock-lfr/src/stmt.rs:109` (`HHI_RANGE_BUS`), `:965` (First lookups, multiplicity `valid`), `:1959-1990` (`HhiRangeAir`, `hhi_range_trace`). Claim: every enabled First row's four `h_hi` limbs are below 2^16.
2. The `L` log tie at the root statement: `stmt.rs:1343-1364` (`eval_root`: the exposed log words leave the child-equality loop; the child log `(D_EMPTY, 0)` on `pc = 0` and `pc = 1`; the `TIE_BUS` receive with multiplicity `-valid`), `:1610-1614` (trace check). Claim: the exposed `(log_d, log_n)` equal the onion's final `(d, n)`, and the child logs nothing.
3. The onion wiring of the production `L`: `crates/sherlock-lfr/src/iface_l.rs:412` (`LogEntry` and `LogValue` send on `ENT_BUS` and `VAL_BUS`; `Stmt` stays bare), `iface_l.rs:450` (`OnionAir`, unchanged from J8), `crates/sherlock-wrap-export/examples/t7_chain.rs:569, 625` (`OnionAir` in the `L` set and its trace and permute inputs). Claim: the onion reads exactly the used entries and values.
4. Run words: `crates/sherlock-lfr/src/iface.rs:1124` (bound runs carry the onion of their log). Not a circuit; the prover's words.
5. Decoders without the pinned `X`: `iface.rs:749`, Moriarty `tools/sherlock-wrap/src/decode.rs:366-368`. Claim: `X` is bound by `L_Fr` (T7c lane 7), so the decoders need only range and `Canon`.
6. `L_Fr` tape: `t7_chain.rs:768-787` (`lfr_gen_any`: `gen_r_check`, `gen_ops_check` with `A_0 = C_P`, the T7c `X` check). Claim: a run whose ledger anchor is not its own `C_P` fails exactly one assertion.

## 4. Open

- The child log tie (section 0 item 2) is the interim `(D_EMPTY, 0)` rule until the leaves log.
- One contract for all programs (section 0 item 5) waits on the owner (SRC-007).
- `lfr_e3 0` panics on the J8 bench `L` proof (`lverify.rs:1648`), so the `i7_decode` fixture was not regenerated.
- `t8_gp chain|cases|omit` refuses at its `L` step (the `L` layer is `t7_chain`'s).
- Preview: section 0 item 6.
