# T4: wrap row budget at k = 21

Row T4 of `LOOP.md` in this folder. The hard rule is never k = 22, so the wrap must stay at or below the k = 21 gate of **1,782,573 rows** (85 percent of the usable 2^21 - 7 rows, `~/Projects/Moriarty-wt/fp8/research/sherlock-final-design-20261111/ARCHITECTURE.md:2359`) and keep every check the design requires. "ARCHITECTURE" below is that file.

## Verdict

- **Every required check fits.** With all the missing checks added, the wrap over the bound `L_Fr` of FP8 round 2 (1,528 public words) is **1,676,222 rows** (MEASURED, cost model at k = 21, `~/.cache/t4-budget/logs/cost4.log:34`). That is 94.03 percent of the gate, with **106,351 rows of headroom** (DERIVED).
- **D-11 (owner item O4): full replay fits.** The native replay of every `PERM`, `H2`, `SPONGE`, `DIGEST` **and `PATH`** entry is inside that total. The `R`-only replay is not needed. It is not a lever either, because it drops a required check (ARCHITECTURE:2349, "native replay of every log entry").
- No lever is needed. Section 4 lists reserve levers for later growth of the `L_Fr` shape.

## Method and labels

- **Source.** I used the E5 round 3 decoder at Moriarty `e5-wrap` commit 10fe24968 (T0's commit of the tree that measured 1,668,323 rows, `~/.cache/e5-wrap/logs/r3-i6-excl.log:11`). I copied it to `~/.cache/t4-budget/sherlock-wrap` and added the missing checks there behind an env switch `T4`, one name per check (`x`, `layout`, `popeq`, `aout`, `lm01`, `tpl`, `openr`, `kw`, `path`; `head` is the first five, `all` is every one). Patch: `~/.cache/t4-budget/t4-probe.diff`. Runner: `~/.cache/t4-budget/run.sh`. Logs: `~/.cache/t4-budget/logs/cost3.log` and `cost4.log` (cost models), `mock1.log` (MockProver). Every run went through `heavy-slot --mem 32G`, never exclusive, with `RUSTUP_AUTO_INSTALL=0` and the stable toolchain.
- **Build check.** The build logs show `Finished` and a new `release/wrap` timestamp. With `T4=base` the copy gives exactly 1,668,323 rows, the E5 figure (`cost3.log:4`, `cost4.log:4`). The combined runs equal the sum of the single deltas: 15 + 430 + 7,421 = 7,866 on `lfr-e3` (`cost3.log:16`), and 11 + 4 = 15 for `head` (`cost4.log:28`).
- **What MEASURED (probe) means.** It is a midnight `cost_model` row count of the probe gadgets at k = 21, with placeholder key constants (rows do not depend on constant values). The native verdict assert in `WrapRelation::instance` is bypassed for the probes, because the honest fixtures have random `X` and `layout_d` words. The probes measure rows; they do not test soundness. One MockProver run is on record: `T4=path` on `lfr-e3` accepts the honest proof and refuses a changed public input and a changed proof value (46.4 s, `mock1.log:5`). That fixture has no `PATH` entry, so the run shows only that the new gates leave the honest replay of the other operations intact.
- **Zero-row checks.** An equality against a key constant or between two existing cells adds 0 rows in the cost model (`layout`, `popeq`, `lm01`, `tpl`: `cost4.log:10,13,19,22`). The midnight layouter places these as copy or fixed constraints.
- **DERIVED** is arithmetic on MEASURED numbers, with the arithmetic shown. **ESTIMATE** is a figure that is not derived from a measurement of this circuit.

## 1. Every required wrap check and its state

Requirements: ARCHITECTURE:2334-2353 (interface and invariants of kit L AND WRAP), the FP-3 index table (ARCHITECTURE:428-438), the call-vector layout (ARCHITECTURE:250-256) and the reservation `R` (ARCHITECTURE:247). File:line is `~/Projects/Moriarty-wt/e5-wrap/tools/sherlock-wrap/src/` at 10fe24968.

| # | Required check | State | Where |
|---|---|---|---|
| 1 | Verify `L_Fr` natively in Fr under the frozen `L_Fr` key, u = 7 transcript, shape pinned as constants | in circuit | `wrap.rs:99-118`; `verifier.rs:286-330` (preamble), `stacked.rs`, `whir.rs`; shape pin `verifier.rs:103,156` |
| 2 | `L_Fr` verifies `L` under `sched`, with bounded-quotient and canonical-squeeze checks | in `L_Fr`, no wrap rows | ARCHITECTURE:2348; the wrap covers it through item 1 |
| 3 | `L` vk pre-hash pinned (the wrap key commits to one `L` key) | in circuit | `decode.rs:271` |
| 4 | `pv` absorbed natively and bound | in circuit: the `L_Fr` public-value cells are observed in the preamble (`verifier.rs:320-321`) and are the cells the decoder unpacks (`wrap.rs:117`, `decode.rs:276-295`) | |
| 5 | Range and `Canon` on every logged value | in circuit | limbs `< 2^30` `decode.rs:285-286`; `dec` with `Canon` `decode.rs:224-258`; values `304-308` |
| 6 | Range and `Canon` on every lane | in circuit | `decode.rs:299-303` |
| 7 | Range and `Canon` on the limbs of `X` | **missing**: words 97 to 105 are only `< P` | `decode.rs:285-288` (else branch); E5 RESULTS 6j finding 4 |
| 8 | `X` equals the cached-trace commitment of `L` (the DAG key check) | **missing** | ARCHITECTURE:2352; FP8 RESULTS 15.1 (`L_Fr` pins that commitment by `fr_const`, so the wrap compares `X` with a key constant) |
| 9 | Op admitted, `param` bounds | in circuit | `decode.rs:318-338` |
| 10 | `NOP` rule | in circuit | `decode.rs:339-343` |
| 11 | Running sums of the value and permutation offsets, capacity | in circuit | `decode.rs:344-383` |
| 12 | Zero padding after `off_n` | in circuit | `decode.rs:384-399` |
| 13 | Every entry of `R` present, with its tags | **partial**: the run `R` (ordinary call and chunk) is in circuit. The opening-proof `R` (`DIGEST(SHK_SET)` as a fifth statement entry, ARCHITECTURE:247) is not: entry 4 is fixed as the `a_in` anchor. `openClaim` is an entry of the one universal wrap key (ARCHITECTURE:426), so the one wrap must accept both forms. `iface.rs` builds no opening-proof `R` either (only the `SHK_SET` tag, `openvm-e5 .../iface.rs:59`) | `decode.rs:401-445` |
| 14 | Anchor tag rule (`a_in`: PROGC when ORD or `j = 1`; `a_out`: PROGC when ORD or `j = k`; LOCK otherwise) | in circuit | `decode.rs:403-438` |
| 15 | Anchor entry words `e_in`, `e_out`; `claim_pub` inputs | in circuit | `decode.rs:439-445` |
| 16 | `a_in` and `a_out` are outputs of `DIGEST` entries | in circuit for the entry form: entries 4 and 5 are `DIGEST` and are replayed in step 7; their outputs sit at the static cells `vs[11]` and `vs[13]` | `decode.rs:431-438`, `512-652` |
| 17 | The `popeq` lane equals `a_in` | **missing**: in the E3 fixture the lane is set only if the prover's lane map names it (`openvm-e5 .../iface.rs:733,862`) | ARCHITECTURE:2350 |
| 18 | `a_out` reaches the call vector: for a chunk, the anchor-write effect lane (`ins anchor := a_out`) equals `a_out` | **missing** | ARCHITECTURE:3113-3118 |
| 19 | Anchor template words: the other anchor-prefix lanes (the uncached `idx` of cell `(0)`) are key constants | **missing** | ARCHITECTURE:255, 2342 |
| 20 | Lane 0 equals the E-1 `bind` word; lane 1 equals `comm` | **partial**. FP-3 fact 1 puts lane 1 = `comm` and the effect lanes in `L` (ARCHITECTURE:444). ARCHITECTURE:3109 says the wrap requires lane 0 = `bind`, and ARCHITECTURE:3133 gives E-1 placement V "0 wrap rows" (ESTIMATE), so `bind` reaches the wrap only through the call-vector words of `L`. The bench `L` sets lane 0 to a random value (`openvm-e5 .../iface.rs:764`): the `L`-side binding of lane 0 is not built. The wrap-side part, keeping the lane map off lanes 0 and 1, is missing too | `decode.rs:299-303, 455-499` |
| 21 | `layout_d` equals the key constant (V+) | **missing**: words 89 to 96 are only `< P` | ARCHITECTURE:2351, 3109 |
| 22 | Other head words: `root_stmt` (0 to 73), `run_tag` (81 to 88) | range only. Range only is correct for these words: ARCHITECTURE:2345-2353 gives the wrap no reader for them. `L` opens `root_stmt` (the SET head, ARCHITECTURE:423) and uses `run_tag` inside the lock operand (ARCHITECTURE:3111), and both are forbidden public inputs (ARCHITECTURE:2343) | `decode.rs:31-32, 285-288` |
| 23 | Lane map rules (stock order, `DIGEST` or `SPONGE(3)`, at most once, u16) | in circuit | `decode.rs:446-499` |
| 24 | `PUB(i)` values equal their lanes | in circuit | `decode.rs:501-510, 693-723` |
| 25 | Native replay of `PERM`, `H2`, `SPONGE`, `DIGEST` | in circuit | `decode.rs:512-652, 724-742` |
| 26 | Native replay of `PATH`, with wrap-private siblings | **missing** | `decode.rs:512-513` (comment) |
| 27 | The 6 FP8 key words (the bound statement, 1,528 words) are read and bound | **missing**: the decoder asserts 1,522 words (`decode.rs:270`, `wrap.rs:53`) | FP8 RESULTS:144, 179, 208, 222 |
| 28 | Public inputs are exactly the call vector (no `X`, `mode`, `run_tag`, `prog_d`, ...) | in circuit | `wrap.rs:121-123` |
| 29 | Fresh blinders on every proof | in the prover, 0 rows | `bin/wrap.rs:70` (`OsRng`) |

## 2. Rows of each missing check

Base: 1,668,323 rows on `lfr-e3` (MEASURED, `r3-i6-excl.log:11`; repeated at `cost4.log:4`).

| Check (item) | Form built in the probe | Rows | Label and source |
|---|---|---:|---|
| `Canon` on `X` and `X` pinned (7, 8) | `X` limbs move from `< P` (6 rows each) to `< 2^30` (3 rows each), then `dec` with the `Canon` borrow chain, then `assert_eq_c` | **+11** | MEASURED (probe), `cost4.log:7` |
| `layout_d` pinned (21) | 8 `assert_eq_c` | **0** | MEASURED (probe), `cost4.log:10` |
| `popeq` lane = `a_in` (17) | `s[9] == vs[11]` | **0** | MEASURED (probe), `cost4.log:13` |
| Anchor-write lane = `a_out` for a chunk (18) | `a_out = vs[11] + mode (vs[13] - vs[11])`; `mode (s[10] - a_out) == 0` | **+4** | MEASURED (probe), `cost4.log:16` |
| Lane map kept off lanes 0 and 1 (20, wrap part) | `h[0] = h[1] = 0` in each of the 4 pairs | **0** | MEASURED (probe), `cost4.log:19` |
| Lane 0 = `bind`, lane 1 = `comm` (20, `L` part) | in `L`, not the wrap | 0 wrap rows | ESTIMATE, ARCHITECTURE:3133 |
| Anchor template lanes pinned (19) | lanes 2 to 8 equal key constants | **0** | MEASURED (probe), `cost4.log:22` |
| Opening-proof `R` in the one wrap (13) | a statement bit `op`; entry 4 is `DIGEST(SHK_SET)` or `a_in`; entries 5 and 6 take the anchors; `e_in`, `e_out` and the `a_in`, `a_out` cells move by `op`. Added on top of the run-`R` checks, so an upper bound | **+21** | MEASURED (probe), `cost4.log:25` |
| `PATH` replay (26) | A `PATH` bit in the slot tuple. In each of the 256 slots: a private sibling and an index bit, the order select of `H2(l, r)`, gated permutation inputs `(l, r, cap(0, 2))`, the leaf read at `vs[o]` on the first block, the running index sum `sum b_m 2^m` against `vs[o + 1]` and the root against `vs[o + 2]` on the last block. The existing reads are reused, so there is no new lookup query | **+7,421** (29.0 per slot) | MEASURED (probe), `cost3.log:13` |
| Bound `L_Fr` statement: 6 more public values in the preamble and the DAG (27) | none: the bound `L_Fr` shape itself | **+78** | MEASURED: 1,668,401 on `~/.cache/fp8/out/kb/chain/lfr-fp8.json` (`cost3.log:19`) against 1,668,323 |
| Key-word reader, interim `KeyBind` form (27) | unpack each of the 6 Fr into 8 BabyBear words `< P`; a u = 0 sponge of the 6 words (3 permutations) equal to a statement-side value (placeholder: lane 2) | **+364** | MEASURED (probe): 1,668,765 (`cost3.log:22`) - 1,668,401 |
| Key-word reader that must recompute BabyBear `P2` in the wrap (key triple, then the `DOM_PROGC` operand) | about 10 `P2` permutations at 2,000 to 4,000 rows each (ARCHITECTURE:423; `DECISIONS.md:131`) | 20,000 to 40,000 | ESTIMATE |
| Key words, recommended final form (FP8 RESULTS:144, 252: the leaf carries `prog_d` and the first node opens the key triple from `key_root`, so the 6 words leave the statement) | none | 0, and the +78 goes away | DERIVED |

`PATH` arithmetic: 7,421 / 256 slots = 29.0 rows per slot (DERIVED), against 228 rows per slot for the round 3 replay (58,285 / 256; 58,285 = 223,453 - 165,168, the decoder alone in E5 RESULTS 6m, DERIVED).

## 3. Total against 1,782,573

| Case | Rows | Percent of gate | Headroom | Label and source |
|---|---:|---:|---:|---|
| E5 round 3 base (`lfr-e3`) | 1,668,323 | 93.59 | 114,250 | MEASURED, `r3-i6-excl.log:11` |
| Audit checks on `lfr-e3` (head, key-word stand-in, `PATH`) | 1,676,189 | 94.03 | 106,384 | MEASURED (probe), `cost3.log:16` |
| Audit checks on the bound `L_Fr` | 1,676,201 | 94.03 | 106,372 | MEASURED (probe), `cost3.log:25` |
| **Every missing check on the bound `L_Fr` (`T4=all`): the T4 total** | **1,676,222** | **94.03** | **106,351** | MEASURED (probe), `cost4.log:34` |
| The same with a `P2`-form key-word reader (upper end) | 1,676,222 - 364 + 40,000 = 1,715,858 | 96.26 | 66,715 | DERIVED on the ESTIMATE |
| The same with the final key-word form (no key words) | 1,676,222 - 364 - 78 = 1,675,780 | 94.01 | 106,793 | DERIVED |

All three cases fit k = 21. The table rows stay 1,261,315 in every run (MEASURED, `cost3.log`, `cost4.log`), also below the gate.

**The shape the total holds for.** The bound `L_Fr` has the same stacked layouts as `lfr-e3`: 2 commitments, widths 33 + 10, log stacked height 20, 15 AIRs, 561 columns (MEASURED by reading `layouts` and `derived` in both fixtures; FP8 RESULTS 15.3). Both fixtures already carry the full interface (`Dims::FULL`, 12,164 `L` words in 1,522 Fr, plus 6 key words in the bound one), so the statement does not grow further. Growth would come from the `L_Fr` stacked width and height, which follow the `L` key and the `L` heights.

**How much width growth the headroom absorbs.** Each extra stacked column costs, per query, 16 opened values times (1 witness + 1 leaf addition + 1 batching `mul_add`) plus 16 x 0.5 leaf permutations x 19.4 rows. That is about 16 x 3 + 16 x 9.7 = 203 rows per query, so 98 queries give about **19,900 rows per extra stacked column** (DERIVED from E5 RESULTS 6h, unit costs at RESULTS:92). The 106,351 rows of headroom absorb about **5 extra stacked columns** of `L_Fr` (DERIVED).

## 4. Reserve levers (not needed now; each keeps every check and k <= 21)

Ranked by saving per unit of risk. None drops a check or adds a BLS12-381 layer. `R`-only replay is not on the list: it drops item 26 and part of item 25.

| Rank | Lever | Saving | Label | Risk |
|---:|---|---:|---|---|
| 1 | Four range columns (`nr_pow2range_cols` 4): a range `< 2^30` goes from 3 to 2 rows and `< P` from 6 to 5 (E5 RESULTS 6i unit costs). The decoder alone holds about 23,000 range checks below `2^30`: (1,026 lanes + 256 values) x 9 limbs = 11,538 limbs, each with its limb range and its `Canon` difference range | at least 23,000 | DERIVED (count from `decode.rs:224-308`); the verifier core's own range checks come on top, unmeasured | medium: it changes `ZkStdLibArch`, so the ledger 9.1.0.1 check of the key must be run again (E5 left the lever out for that reason; ARCHITECTURE:2359 allows at most 4 lookups) |
| 2 | Fewer preprocessed columns in `L_Fr` (width 10 today) | about 19,900 per column | DERIVED (section 3) | medium: an `L_Fr` key change, so D3 and the FP8 key set are regenerated |
| 3 | Shared `Canon` chain: compare the limbs high to low with one equality flag, in place of 9 difference ranges per value | about 11,000 (half of the `Canon` ranges) | ESTIMATE | medium: a new gadget, so the Quint decoder lane and the ENC `Canon` vectors (E5 I9 part A) run again |
| 4 | Fewer gated reads in the replay (each read costs one lookup term and one observed value) | a few thousand | ESTIMATE | low to medium: a decoder change, Quint lane again |
| 5 | Smaller `k_whir` at round 0, or other `L_Fr` WHIR parameters | not measured | ESTIMATE | high: the `L_Fr` set is qualified again against the 100-bit gate |

Not a lever under the current rules: a smaller capacity `P`. Each 64 slots is about 64 x (228 + 29) = 16,450 rows (DERIVED), but `B = V = P = 256` is an owner decision (OD-6, ARCHITECTURE:233), so it would need a ruling.

## 5. Assumptions in the probe forms (for T10, which builds the real checks)

- The `PATH` node hash is the `iface.rs` placeholder `H2(l, r)`, with the index bit choosing the order (`~/.cache/m7-wrap/src/openvm-e5/crates/sherlock-lfr/src/iface.rs:16, 212-222`). If the real node hash takes more than one permutation per level, the `PATH` rows scale with it.
- The `popeq` lane is 9 (`iface.rs:733`, a LEAD pick, test data). The anchor-write lane (10) and the template lanes (2 to 8) are my placeholders: the design leaves their places to the compiled contract and the N-11 prologue (ARCHITECTURE:445). The row cost does not depend on the lane numbers.
- `X` and `layout_d` are compared with key constants. `X` is the `N3` DAG commitment that `L_Fr` pins (FP8 RESULTS 15.1), and `layout_d` is the V+ constant (ARCHITECTURE:3109).
- Lane 0 = `bind` is an `L`-side check (ARCHITECTURE:3109, 3133) that the bench `L` does not have (`iface.rs:764`). If T10 needs it in the wrap, the wrap has no `bind` word to compare with; `L` must expose it, and the comparison is then 0 rows by the same copy-constraint rule.
- The opening-proof form needs a statement bit that tells the two `R` forms apart. The probe witnesses it; T10 takes it from a head word or the entry label (LEAD F-8, ARCHITECTURE:426).
- The interim key-word reader compares a sponge of the 6 words with one statement value. The design gives no target for it yet (FP8 RESULTS:222: "that reader is not built"). The `P2` case and the final form bound it from both sides (section 2).
- The probe siblings and index bits are witnessed without host values, so a fixture with `PATH` entries would not pass MockProver as written. T10 must wire the siblings from the `L_Fr` witness and run the named mutants of ARCHITECTURE:2376 (`wrap_popeq_lane_not_a_in`, `wrap_lane0_not_bind`, `wrap_layout_d_other`, `wrap_x_not_cached_commitment`, and a wrong `PATH` root or order).
