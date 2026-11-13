# E3 results: the full `L` interface carried through `L_Fr`

Plan: `PLAN.md` beside this file (decisions D-5 to D-9, increments J1 to J6).

## 1. Status

**E3 gate: PASS (J1 to J5, tree 2f09caf); the owner accepted the all-entries case in place of all-non-`NOP` (ruling 2026-11-12 22:10).** At the `L_Fr` set (`lfr_params`) the all-entries case (253 entries, every value slot used, no `NOP` before the padding) and the forced-overflow case (551 operations to tier E) prove and the stock verifier accepts with 2 commitments; the missing-`R` case is refused by the prover. D3 at the measured key: 101.1435 bits (equal to E2). An entry table with no `NOP` row at all is impossible at `B = V = 256` (DERIVED, section 5), so all-entries is the strongest case that exists. "Statement words equal the native ones" is the plain-Rust decoder check of D-9 on the exposed `L_Fr` words, not a constraint of the wrap.

**Cases proved at `lfr_params` (MEASURED, read from the J5 logs `~/.cache/e3-iface/logs/j5-final.log`, `j8-j5-final.log` and `j9-j5.log`).** No run proved ten cases at the `L_Fr` set. Each J5 run proves at most four, under one key: on 2f09caf, all-entries and overflow accepted, missing-`R` refused by the prover; on f725b15 (J8) and on 95ed47b (J9), overflow, all-entries and chunk 2 of 3 accepted and the forged `DIGEST` refused by the prover (`LogupZerocheck(NonZeroRootSum)`). The ten cases of the fixture (full-V, full-P, overflow, all-entries, chunks 1, 2 and 3 of 3, missing-`R`, anchor tag swapped, forged `DIGEST`) run at J4, on the tape and the decoder, not as `L_Fr` proofs: full-V, full-P, chunk 1 of 3, chunk 3 of 3 and anchor tag swapped were never proved at `lfr_params`, and missing-`R` was proved and refused only on the 2f09caf key.

| # | Increment | Status | Evidence |
|---|---|---|---|
| J1 | `Entry::Public`, D-6 carrier in `L` and `L_Fr`, small `L_iface` | PASS | section 3 |
| J2 | native reference `iface.rs` | PASS (unit tests; used by J1) | section 4 |
| J3 | `L_iface` fixture at the OD-19 `L` set | PASS | section 5 |
| J4 | `R` check (D-7), the full tape of every case, op counts | PASS | section 6 |
| J5 | gate at `lfr_params`, D3 | PASS | section 7 |
| J6 | optional replay of every entry | not built: does not fit the key (DERIVED) | section 8 |
| J7 | ENC-1/ENC-2, `R` with `claim_pub` and chunk anchors, replay of `R` | PASS (with J8, full rerun) | sections 9, 10 |
| J8 | the log onion in `L`, tied to `log_d`, `log_n` (bench form: one segment, no node tie) | PASS: fixture, J4, J5 at the E2 shape; D3 101.1435 | section 10 |
| J9 | `j`, `k` range rule in the `R` check and the reader; native `TAG_NODE` merge; Gemini Quint lanes on `OnionAir` and the `R` replay | PASS: words-level test, J4, J5 (D3 101.1435); lanes rounds 1 and 2 done, no open defect | sections 12, 13 |

## 2. Code

| Repository (not pushed) | Branch | Commits |
|---|---|---|
| openvm worktree `~/.cache/m7-wrap/src/openvm-e3` | `e3-iface` (from `e2-lfr` c4bd312) | 96acfa3 J1 and J2; 71bf0f1 J3 and J4; 2f09caf `PublicAir` row binding, all-entries case; 6d70a45 J7 |
| openvm worktree `~/.cache/m7-wrap/src/openvm-e3-onion` | `e3-iface-onion` (from 6d70a45); `e3-iface` fast-forwarded to it | ab0e88a J8; f725b15 J4 shape guard, bus check, small tape delta |
| openvm worktree `~/.cache/m7-wrap/src/openvm-e3` | `e3-iface` | 95ed47b J9 `j`, `k` range rule, `log_merge`; 52a538c ENC-1 vector test; 099d6d8 lane map `DIGEST` or `SPONGE(3)`; 9539418 `apply_lane_map` |
| Moriarty `~/Projects/Moriarty-wt/e3-iface` | `e3-iface` (from `e2-lfr` fbba858f1) | 8bcb491b9 plan and J1; this file |

Runner `~/.cache/e3-iface/run.sh <stage> <mem> <dir> cmd...` (heavy-slot, `CARGO_TARGET_DIR=~/.cache/e3-iface/target`, logs `~/.cache/e3-iface/logs/`).

**2a. Files (crate `crates/sherlock-lfr`).**

| File | Content |
|---|---|
| `src/carrier.rs` | D-6: the public-value carrier (selectors `a[h1]`, `b[h2]`, `w` value cells per row; uniqueness argument in the header) |
| `src/iface.rs` | J2: index table (`Dims`, 12,164 words at `Dims::FULL`), log operations, native values with Midnight Poseidon (u = 0 and u = 4), OD-17 planner, `Run::words`, `Run::statement`, the decoder `decode` (D-9), the cases of J3 |
| `src/iface_l.rs` | D-5: the `L`-side interface AIRs `Stmt`, `LogEntry`, `LogValue`, `Call` and their traces |
| `src/lverify.rs` | `Entry::Public` (part 2 reads the per-AIR public addresses of part 1); `gen_verifier_full`; `gen_r_check` (D-7) |
| `src/machine.rs`, `src/tape.rs` | `PublicAir` as a carrier, 16 words per row (`PUBLIC_W`), program row `[addr_0..addr_15, active, idx]` with `active (idx - row) = 0`; `Tape::expose`; `Op::PublicPad` (the zero read by the empty cells of the last row) |
| `src/record.rs` | `stock_log_at` (the stock record at any `L` parameter set) |
| `examples/iface_fixture.rs` | J3 fixture |
| `tests/j1_public.rs`, `tests/j5_gate.rs` | the gates of J1, J4 and J5 |

## 3. J1: public values of `L` through `L_Fr` (MEASURED)

Numbers of this section are from the 2f09caf tree (`l_tx` 14); from J7 on J1 runs at `l_tx` 62 (passes in the J8 small stage).

Test `j1_public_values_through_lfr` (log `~/.cache/e3-iface/logs/j1-final.log`, tree of 2f09caf). Small `L_iface`: the 4 interface AIRs at `Dims { b: 32, v: 32, p: 32, l_tx: 14 }` (610 public words), `L` config, `small_params_n(4, 8)`, case full-V (14 entries, 32 values).

| Check | Result |
|---|---|
| Stock verifier on the small `L_iface` | accepts; 4 AIRs at log heights 2, 5, 5, 2; 3,337 transcript events |
| Tape (whole `L` verifier with `Entry::Public`) | 59,566 ops; every one of the 3,337 events equals the stock record; no assertion fails |
| `L_Fr` public words | 78 = 1 + ceil(610 / 8); word 0 is the `L` vk pre-hash; `decode` gives the native statement (all 16 lanes) |
| `L_Fr` proof at `small_params_n(3, 14)` | accepted; 2 commitments (stacked widths 80 and 20); 4,415,810 bytes; prove 55.1 s (shared slot, not a clean timing) |
| Changed `L` public value (the last call-vector word + 1) | stock `L` verifier refuses; tape: 142 assertions fail, the tape (so the key) is unchanged; `L_Fr` prover refuses (`LogupZerocheck(NonZeroRootSum)`) |
| Carrier controls (same witness, changed `L_Fr` public values): first word, middle word, last word, two words swapped | each refused by the stock verifier (`BatchConstraintError(SumClaimMismatch)`) |
| Row binding control: `PublicAir` rows 0 and 1 swapped together with their program cells and values, and public blocks 0 and 1 swapped (program bus and memory bus still balance) | refused by the stock verifier (`SumClaimMismatch`) |

**Soundness fix found in review (2f09caf).** The program bus fixes only the multiset of program rows. In 96acfa3 a `PublicAir` program row could sit at any trace row, so a prover could permute 16-word blocks of the public words (DERIVED; the first swap control kept the honest trace and so tested only the carrier). Each program row now carries its row index `idx` (18 cells, exactly `PROG_W`), and `active (idx - (sum_i i a_i + h1 sum_j j b_j)) = 0` ties it to the carrier row. The row binding control above is refused on 2f09caf. `cargo clippy -p sherlock-lfr --all-targets -D warnings` is clean on 2f09caf.

Also pass on the same tree: `i10_public_air_small` (the E2 test, now with `Tape::expose`) and the `iface` unit tests (layout 12,164 words, `W_pv` 2,816; limb round trip; `Canon` refuses `r` and a limb at `2^30`).

## 4. J2: the native reference (CODE)

- Layout of ARCHITECTURE 1.11 FP-3: words 0 to 113 the statement head (`mode` at 74, lane map at 106 to 113), the entry table at 114, the value table at 626, the call vector at 2,930; 12,164 words (DERIVED and asserted).
- Entries as ARCHITECTURE 1.8: values and slots per operation; bounds `SPONGE 1..V-1`, `PATH 1..32`, `PUB 0..1+L_TX`, `DIGEST` 4 tags; `ECMUL`, `ECMULG` never admitted (`E = 0`).
- Native values: `PERM` the Midnight permutation; `H2`, `SPONGE(n)` the Midnight fixed-length sponge at u = 0 (capacity `l`); `PATH(d)` `H2` per level with the index bit choosing the order (placeholder, D-8); `DIGEST` the sponge at u = 4 of `(tag, pack(d2))` with tag `Fr(k + 1)` (placeholder, D-8).
- Planner (OD-17): `R` first (refused if it does not fit), then each discretionary operation if its bound and all four limits still hold, else tier E.
- Decoder (D-9): unpack the `L_Fr` words, range and `Canon` of every lane and value, bounds, `NOP` padding, capacity, zero value rows after `off_n`, the lane map (each mapped entry a `DIGEST`, its lane zero in `pv`), every `PUB(i)` against lane `i`; output the statement (the call vector with the lane-map lanes filled).

## 5. J3: the `L_iface` fixture (MEASURED)

Example `iface_fixture` (unit `e3-j3b`, tree 2f09caf, wall 1,558 s, log `~/.cache/e3-iface/logs/j3-final.log`, directory `~/.cache/e3-iface/fixture/`, file `iface_fixture.json`). The bench chain of E2 I0 (two W1 proofs, the node at the OD-19 internal set, 90.2 s), then `L_iface` at the OD-19 `L` set (`l_ud(48)`, BabyBear base with the Midnight Poseidon Fr hash): the 39 AIRs of `L_cached` and the 4 interface AIRs at `Dims::FULL`. One key; the stock verifier accepts each proof, also after a re-read from disk.

| Quantity | Value |
|---|---|
| AIRs | 43 (39 bench, 4 interface) |
| Public words of `L` | 12,164 (Stmt 114, LogEntry 512, LogValue 2,304, Call 9,234) |
| Log heights of the interface AIRs | Stmt 2^2, LogEntry 2^8, LogValue 2^8, Call 2^8; the 39 bench log heights equal `l_log_heights` of the E2 `l_fixture.json` (compared) |
| `L` vk | 238,345 bytes (bitcode; bench `L` 80,439) |
| Proof | 487,514 bytes in every case (bench `L` 430,073) |
| Prove (16 threads, shared slot, not a clean timing) | full-V 188.2 s, full-P 156.0 s, overflow 165.8 s, missing-`R` 247.0 s, all-entries 258.7 s |

| Case | Log | Planner |
|---|---|---|
| full-V | 189 entries (`R`, 8 `PERM`, 8 `H2`, `SPONGE(5)`, `PATH(8)`, `DIGEST(SHK_LOCK)`, 167 `PUB`), 256 values, 31 slots | `V` filled exactly; 67 entry rows `NOP` |
| full-P | 11 entries (`R`, 7 `PATH(32)`, `PATH(29)`), 30 values, 256 slots | `P` filled exactly |
| overflow | 46 entries, 256 values, 76 slots | `R` and 594 discretionary operations (64 `PERM`, `SPONGE(256)` over its bound, 16 `PATH(32)`, `PATH(33)` over its bound, 512 `PUB`); 551 go to tier E |
| missing-`R` | full-V with the `SHK_INPUT` entry removed: 188 entries, 254 values | built without the planner (negative) |
| all-entries | 253 entries (`R` and 250 `PUB`), 256 values, 3 slots | `V` filled exactly; every entry before the 3 padding rows non-`NOP` |

Every case gives the same `L` shape (asserted in J4), so one `L_Fr` key serves all of them.

All-non-`NOP` (DERIVED): with `B = V = 256`, every non-`NOP` entry logs at least one value and the 3 `DIGEST` entries of `R` log 2 each, so at most 253 entries are used and an entry table with no `NOP` row is impossible. The all-entries case reaches this bound: 253 non-`NOP` entries, all 256 values, `R` real.

## 6. J4: the full tape on every case (MEASURED)

Test `j4_tapes_on_l_iface` (log `~/.cache/e3-iface/logs/j4-final.log`, tree 2f09caf). Per case: the stock verifier accepts the `L_iface` proof (33,120 transcript events); the shape and the tape are the same in every case; the tape is the whole `L` verifier, the `R` check of D-7 and the exposure of 1,522 words.

| Case | Failed tape assertions | Decoder (D-9) |
|---|---|---|
| full-V | 0 | statement equals the native one (1,026 lanes) |
| full-P | 0 | equal |
| overflow | 0 | equal (551 operations in tier E do not change the statement) |
| missing-`R` | 3 (the `R` check: the tag of entry 1, and the op and tag of entry 2) | refused (lane-map pair 2 names entry 2, now a `PERM`) |
| all-entries | 0 | equal |

**Added `L_Fr` rows** (MEASURED op counts; bench values from E2 I10):

| Op kind | Bench `L` (E2 I10) | `L_iface` | Added |
|---|---:|---:|---:|
| ALU | 347,887 | 383,493 | 35,606 |
| Input | 34,773 | 47,481 | 12,708 |
| Pack | 11,187 | 14,502 | 3,315 |
| Perm | 20,563 | 21,460 | 897 |
| Decompose | 6,936 | 7,480 | 544 |
| Public | 1 | 1,522 | 1,521 |
| AssertEq | 658 | 665 | 7 |
| all ops | 442,806 | 497,410 | 54,604 |

**Heights of `L_Fr`** (rows / main width): BbAlu 524,288/47, ProgramTable 524,288/1 (preprocessed 20), Input 65,536/30, Range16 65,536/2, PoseidonFr 32,768/185, Pack 16,384/27, FrInput 8,192/5, Reshape 8,192/16, Halve 8,192/9, CSwap 8,192/13, FrAdd 2,048/8, AssertEq 1,024/7, Split 512/64, Bits 512/69, **Public 128/58** (E2: 2/3; 57 in 71bf0f1, plus the `idx` cell). Every height except `Public` equals E2 I10; the table holds 497,410 ops plus the nop rows and stays at 2^19 (about 27,000 rows of headroom, DERIVED). The 2-commitment shape is unchanged (one `ProgramTable`).

## 7. J5: the gate at the `L_Fr` set, D3 (MEASURED)

Test `j5_gate_at_lfr_params` (unit `e3-j5b`, tree 2f09caf, log `~/.cache/e3-iface/logs/j5-final.log`, 20 threads, shared slot: the times are not clean timings). One `L_Fr` key from the tape of J4 (497,410 ops, 15 AIRs, 561 columns, main cells 34,218,752 = 32.63 columns of 2^20, preprocessed 10 columns, one preprocessed AIR `ProgramTable`). Keygen 394.2 s.

| Case | Result |
|---|---|
| all-entries | accepted; 2 commitments (stacked widths 33 and 10); 2,568,818 bytes; prove 860.8 s; verify 1.92 s |
| overflow | accepted; 2 commitments (33, 10); 2,568,818 bytes; prove 2,589.0 s (with other heavy jobs); verify 3.53 s |
| missing-`R` | prover refuses (`LogupZerocheck(NonZeroRootSum)`; the 3 failed `R` assertions of J4) |

Full-V and full-P use the same tape and key; their plain-Rust runs (0 failed assertions, decode equal) are in J4.

**D3 at the measured key** (`w_stack` 48, log stacked height 20, max 171 constraints and 46 interactions per AIR): logup 215.95, GKR sumcheck 253.27, GKR batching 254.86, zerocheck 250.95, constraint batching 247.40, stacked reduction 244.73, WHIR 101.1435 (mu 226.30, fold 232.86, query 101.1435, shift 101.1435, gamma 248.23, OOD 239.86); **total 101.1435 bits**, equal to E2 I10 (the query term, set by the parameters, is the weakest; the added `Public` rows do not change it). D3 weakest term 101.1435 bits; proven 100-bit security is not qualified until G3 and G4 pass (ARCHITECTURE 1.9).

**Added `L_Fr` rows** (section 6): 54,604 ops (+12.3 %) all fit in the existing heights; only `Public` grows (2 to 128 rows, width 3 to 58). The 2-commitment shape with one shared `ProgramTable` is kept.

## 8. J6: replay of every entry in `L_Fr` (DERIVED; not built)

The tape is fixed per key, while the values of entry `i` start at the data-dependent offset `off_i`. A replay of every entry therefore needs either routing of each permutation slot to its value rows (about 256 by 256 selector pairs on a fixed tape, ESTIMATE) or a new lookup AIR. The entry of a value alone costs 257 ops (`Pack` per limb and `FrAdd` doubling, MEASURED in J7), so 256 values cost about 65,800 ops against about 27,000 rows of `ProgramTable` headroom (DERIVED). Limb ranges would add about 4,600 ops (2,304 limbs, 2 ops each). So J6 does not fit the 2^19 key and is not built (owner rule: build only if it fits). The replay of `R`, at static positions, fits and is J7. The reader (D-9) replays every entry whose values are all in `pv` (`PERM`, `H2`, `SPONGE`, `DIGEST`; `PATH` needs the wrap-private siblings).

## 9. J7: ENC-1/ENC-2, `R` after FP-3, chunk anchors, replay of `R` (MEASURED on the words)

Code: openvm `e3-iface` 6d70a45 (`iface.rs`, `lverify.rs::gen_r_check`, `tests/j5_gate.rs`). Decisions D-8', D-10 (confirmed by owner ruling O4, 2026-11-13) and D-11 (resolved 2026-11-13 under O4 by the T4 wrap budget: the wrap replays every log entry, `PATH` included; `L_Fr` keeps the replay of `R` only, `research/sherlock-tested-design-20261113/T4-WRAP-BUDGET.md`) of the plan.

- ENC-1 values: `tagFr(SHK_PROGC)` = `0x43474f52505f4b4853` and `tagFr(SHK_OUTCOME)` equal the ENC table; `PoseidonFr_4(tagFr(SHK_INPUT), pack(input_d))` equals the pinned `enc1.input_pub` (unit test `enc1_tags_and_input_pub_vector`).
- Cases (one key): full-V, full-P, overflow, all-entries (now 249 entries: `R` of 5 entries and 12 values, then 244 `PUB`), chunk 1 of 3, chunk 2 of 3, chunk 3 of 3; negatives missing-`R`, anchor tag swapped (`wrap_anchor_tag_swapped`: chunk 2 with `a_in` under `SHK_PROGC`, its value recomputed), forged `DIGEST` (the `C_P` output changed, `claim_pub` recomputed to match).
- The `R` check and replay on each case's 12,164 words as tape inputs (`j7_r_check_on_words`): 2,943 ops in every case (Alu 28, AssertEq 56, CSwap 3, Const 6, FrAdd 2,729, FrConst 7, Input 8 advice, Pack 99, Perm 7); failed assertions 0 on the 7 positives, 36 on missing-`R`, 2 on anchor tag swapped (the tag and the replay), 1 on forged `DIGEST` (the replay). The decoder accepts the positives with the native statement and refuses the 3 negatives (`cases_decode_or_refuse`).
- 2,943 ops is about 11 % of the headroom (DERIVED); `FrAdd` grows from 1,568 to about 4,300 rows (2,048 to 8,192, DERIVED).

## 10. J8: the log onion in `L` (MEASURED)

Code: openvm `e3-iface-onion` ab0e88a and f725b15 (`iface.rs` native `P2`, `enc`, `onion`; `iface_l.rs` `OnionAir`, `airs_onion`, `traces_onion`; `carrier.rs::row_ind`; `tests/j8_onion.rs`; the fixture with the bench Poseidon2 external inputs). Decision D-12 (confirmed by owner ruling O4, 2026-11-13).

- Native: `P2(TAG_EMPTY)` equals `enc2.D_EMPTY` and the onion of the 6-entry ENC-2 log equals `enc2.onion6` (unit test `enc2_onion_vectors`).
- Small `L_iface` (`Dims { 32, 64, 32, 62 }`, `small_params_n(4, 8)`, a stand-in key table on the permute bus, onion height 256): accepted full-V, all-entries, chunk 2 of 3, missing-`R` (the onion binds whatever log is in `pv`); refused by the prover (`LogupZerocheck(NonZeroRootSum)`): `log_d` changed, one value changed with `log_d` kept, `log_n` changed.
- Small-set figure (ESTIMATE that did not carry over): on the small `L_iface` the onion set (onion AIR 70 columns, 143 constraints, 11 interactions; `LogValue` 9 and `LogEntry` 1 more interactions; the key table) adds 151,357 tape ops at `small_params_n(4, 8)`.

**Full rerun with J7 and J8** (unit `e3-j8`, tree f725b15, logs `~/.cache/e3-iface/logs/j8-{small,fixture,j4,j5}-final.log`, fixture `~/.cache/e3-iface/fixture-j8/`; shared slots, times are not clean timings):

| Stage | Result |
|---|---|
| small | lib (5 unit tests), J1 (`l_tx` 62), I10 small, J7 on the words, J8 small: pass |
| fixture | 44 AIRs of `L` (39 bench, 4 interface, onion), vk 241,251 bytes (2f09caf: 238,345); the 39 bench heights equal E2 in all 10 cases (no pinning needed); onion 1,006 permutations on full-V as external inputs of the bench `Poseidon2Air`; `L` proof 489,806 bytes in every case (2f09caf: 487,514); prove 149 to 167 s |
| J4 | 33,690 transcript events per case; one tape of 502,877 ops (2f09caf: 497,410; +5,467, DERIVED split: about 2,943 for the new `R` check and replay, which replaces the 14 ops of D-7, and about 2,524 for the onion set); 7 positives with 0 failed assertions and the decoder equal to the native statement; missing-`R` 36, anchor tag swapped 2, forged `DIGEST` 1 failed assertions, each refused by the decoder (`R entry 1`, `R entry 4`, `entry 0: native replay`) |
| `L_Fr` shape | `BbAlu` and `ProgramTable` 2^19 (guard passed), `FrAdd` 8,192 rows (was 2,048), every other height as in section 6; main cells 34,267,904 (32.68 columns of 2^20) |
| J5 | keygen 189.0 s; overflow, all-entries, chunk 2 of 3: accepted, 2 commitments (widths 33 and 10), 2,571,250 bytes, prove 658 to 680 s, verify 1.9 to 2.0 s; forged `DIGEST`: prover refuses (`LogupZerocheck(NonZeroRootSum)`) |
| D3 | 101.1435 bits at the measured key, every term as in section 7 |

Headroom left in the 2^19 table: about 21,000 rows (DERIVED).

## 11. Open items and next step

- The production tie of the root statement to the verified node (the bench node is not FP-2), more than one log segment in `L` (`TAG_NODE`; native `log_merge` is built and tested in J9, the in-circuit form needs the witness tree form of the log, a design choice), check (i) on the operands of `R` (an `L` obligation), the opening-proof `R` with `DIGEST(SHK_SET)`, the `popeq` lane check in the wrap, in-circuit limb range and `Canon`, and the `PATH` node order.
- The `a_out` anchor of a chunk: its `DIGEST` output is bound to no lane (the 4 lane-map pairs are `a_in`, `claim_pub` twice, `outcome_pub`), while the spec writes `ins anchor := a_out` in the effect region: the same 4 against 5 pair question as `openClaim` (ENC section 10).
- Closed in J9 (section 12): the range of the anchor words `j`, `k` and `1 <= j <= k` for a chunk. Open, for the owner: whether an ordinary call must also have `j = k = 0` (J9 checks only the 16-bit range there).
- Negative coverage: at the `L_Fr` set on f725b15 only the forged `DIGEST` was proved and refused; missing-`R` and anchor tag swapped are refused by the tape and the decoder (missing-`R` also by the prover on 2f09caf); the onion forgeries are refused at small scale with a stand-in key table; no wrong-permutation negative.
- Gemini Quint lanes: rounds 1 and 2 on `OnionAir` and on the `R` replay done (section 12), no open defect; the lane-map lane of section 13 done (2 reader defects fixed, 2 items for the owner).
- Owner ruling O4 (2026-11-13): D-10 (`R` order and anchor rule, section 9) and D-12 (onion form in the bench `L`, section 10) confirmed; D-11 resolved by the T4 wrap budget: the wrap replays every log entry, `PATH` included (1,676,222 rows with every check, MEASURED cost model), and `L_Fr` keeps the replay of `R` only (section 9).
- Negative coverage of J9: the 5 `j`, `k` negatives are refused by the tape (failed assertions) and by the reader only; none was proved and refused at the `L_Fr` set (the proved negative of J5 is still the forged `DIGEST`).

**Owner items (one list):**
1. D-10 and D-12 confirmed and D-11 resolved (owner ruling O4, 2026-11-13). Of the AFK changes of section 13, the ENC-1 tag rule is the owner-confirmed pick P-1; the `SPONGE(3)` lane map and `apply_lane_map` stay for owner confirmation.
2. The `a_out` lane-map pair: owner ruling O7 (2026-11-13), the lane map has 5 pairs (the table edit lands in T9).
3. Whether an ordinary call must have `j = k = 0` (J9 checks only the 16-bit range there).
4. Whether lanes 0 and 1 (binding, `comm`) may be lane-map targets (lane `e3-lanemap` finding 2b).
5. Whether the lane map is a wrap key constant or tied to the `R` positions (entry 3 for `claim_pub`, entry 4 for `a_in`; finding 4).
6. The node obligation that an `n = 0` statement has `d = D_EMPTY` (for the node that implements `log_merge`).
7. The witness form of the log tree in `L` (in-circuit `TAG_NODE` merges; native `log_merge` only in E3).

## 12. J9: the `j`, `k` rule, the `TAG_NODE` merge, the Gemini Quint lanes

Code: openvm `e3-iface` 95ed47b (`lverify.rs::gen_r_check`, `iface.rs::{anchor_tags, log_merge, seg_log}`, `tests/j5_gate.rs::j9_jk_range_on_words`). No owner choice was needed for these items.

**`j`, `k` rule.** In both modes each of the 4 limbs passes `LowBits(x, 16) == x` (the `BitsAir` decomposition, so `x < 2^16`). For a chunk (`mode` 1): `j >= 1` by a zero test of `j_lo + j_hi` (below 2^17 < p, so 0 only for `j = 0`), masked by `mode`; `j <= k` by an advice borrow bit `c` with `mode (k_lo - j_lo + 2^16 c)` and `mode (k_hi - j_hi - c)` both 16-bit (each an integer in (-2^17, 2^17), so the 16-bit test is the integer test, and `k - j` is their base-2^16 value). The reader `anchor_tags` refuses the same words.

| Check | Result (MEASURED) |
|---|---|
| `iface` unit tests (`anchor_jk_range`: 13 head words; `enc2_node_merge`) | pass |
| `j9_jk_range_on_words` (full words of each case through `gen_r_check`) | 2,977 ops in every case (2,943 + 34); positives chunk 1 of 1, `j = 0xffff, k = 0x10000`, `j = 0x10000, k = 0x10001`, `j = 2, k = 0x20000`, `j = k = 2^32 - 1`: 0 failed assertions; negatives (from chunk 2 of 3, anchor tags unchanged, so only the new rule can refuse) `j = 0`, `j > k` in the low limb, `j > k` in the high limb, `j_lo = 2^16` (the aliasing case of the lane), `k_hi = 2^16`: 1, 1, 1, 1, 2 failed assertions, each also refused by the reader |
| J4 on fixture-j8 (L unchanged) | pass; 33,690 events; tape 502,910 ops (f725b15: 502,877; +33); `LowBits` 274, `AssertEq` 724; `BbAlu` and `ProgramTable` 2^19; all 10 cases as in section 10 (log `~/.cache/e3-iface/logs/j9-j4.log`) |
| J5 at `lfr_params` (new key, the tape changed; unit `e3-j9-j5`, log `~/.cache/e3-iface/logs/j9-j5.log`; shared slots, times are not clean timings) | pass: keygen 199.4 s; overflow, all-entries, chunk 2 of 3 accepted, 2 commitments (widths 33 and 10), 2,571,250 bytes, prove 684 to 744 s, verify 1.9 to 4.2 s, 0 failed tape assertions; forged `DIGEST`: prover refuses (`LogupZerocheck(NonZeroRootSum)`); D3 101.1435 bits (unchanged); shape as in section 10 |

**`TAG_NODE` merge (native only).** `log_merge` is the ARCHITECTURE 1.11 rule (either empty side returns the other, else `(P2(TAG_NODE, d_a, d_b), n_a + n_b)`, `None` unless the output `n < 2^16`). `merge(seg(es[..2]), seg(es[2..]))` of the 6-entry ENC-2 log equals `enc2.node(2,4)` with `n = 6`; the empty identities hold; segment order and tree shape change `d`; `n = 2^16 - 1` is accepted and `2^16` refused. The in-circuit merge in `L` is not built: `L` takes the tree shape as witness (ARCHITECTURE 1.11), and the form of that witness is a design choice.

**Gemini Quint lanes** (owner rule; `~/.cache/gemini-lanes/e3-onion`, `e3-rreplay`; reports in `qa/gemini-quint/e3-onion/` and `qa/gemini-quint/e3-rreplay/` of this worktree). Lead setup defect in round 1: `.lane-skills/` held symlinks to a path outside the checkout, so neither lane could read the skills; round 2 has real copies. The lead ran every quint command (the lanes cannot).

| Lane | Verdicts (round 1) | Lead runs |
|---|---|---|
| `e3-onion` (`OnionAir`, f725b15) | Q1 row grammar, Q2 absorption, Q3 values and order, Q4 chaining, Q6 wrap-around: SOUND; Q5: COMPLETE | after typecheck fixes and a non-overflowing ideal permutation: 13 tests pass (TypeScript backend; the Rust evaluator crashed); `SOUND_AND_COMPLETE` no violation; lead tamper adversary (random one-cell changes, 3,000 samples) no violation; each of the 7 mutants (`mut_no_off_chain`, `mut_fa_reads_w0`, `mut_no_last_count`, `mut_fin_on_b1`, `mut_cap_not_chained`, `mut_cnt_no_b2`, `mut_d_not_chained`) violated, so caught |
| `e3-rreplay` (`gen_r_check`, f725b15) | Q1 zero tests and mode bit, Q2 entries and offsets, Q3 replay and the entry-5 `CSwap`: SOUND; Q4 limbs: SOUND with the reader (the tape alone takes limbs mod r; D-9); Q5: COMPLETE; Q6: UNSOUND without a `j`, `k` rule (`j = 0`, `j > k`, limb aliasing such as `j = (2^16, 0)`, `k = (0, 1)`) | 13 tests pass; `inv_sound_and_complete` and `inv_jk_range_strengthened` no violation; the Q6 property on the old check violated (`j = 6 > k = 2` at base 4); all 6 mutants caught |

Defect found: Q6 of `e3-rreplay`, the missing `j`, `k` rule; fixed in 95ed47b (above). No defect in `OnionAir`.

**Round 2** (skills readable; reports in `qa/gemini-quint/<area>/r2/`). The lead ran every command.

| Lane | Verdicts | Lead runs |
|---|---|---|
| `e3-rreplay` r2 (the built `j`, `k` rule, 95ed47b; `log_merge`) | Q1 `r16` range, Q2 order and wrap mod p (both `c`), Q3 tape equals reader, Q5 `log_merge` equals the spec rule, Q6 no regression: SOUND; Q4: COMPLETE (every `1 <= j <= k < 2^32`, ORD) | 10 tests pass (exhaustive limb and borrow enumeration); `inv_sound_and_complete` 5,000 samples no violation; 5 mutants (`mut_no_r16_jhi`, `mut_borrow_not_bit`, `mut_order_unmasked`, `mut_j_ge_1_dropped`, `mut_plus_c_in_hi`) caught |
| `e3-onion` r2 (audit of the round-1 model against `eval`, an adversarial trace generator not derived from honest traces) | all questions SOUND, COMPLETE; round 1's stronger-than-Rust `val_mult >= 0` side condition removed with no attack found; `x[O_E] * y[O_OFF] = 0` shown redundant (kept as defence in depth) | 21 tests pass (TypeScript backend; the Rust evaluator crashed); `SOUND_AND_COMPLETE` and `TAMPER_SOUND` no violation; the non-vacuity witness violated as expected (the generator reaches accepted adversarial traces); 12 mutants caught |

Notes for the owner from round 2: an ordinary call keeps any 16-bit `j`, `k` words (not forced to 0); the node circuit that implements `log_merge` must take an `n = 0` statement only with `d = D_EMPTY` (a node obligation, not built in E3).

## 13. E5 port: ENC-1 tag rule and the `SPONGE(3)` lane map (AFK changes; the tag rule is the owner-confirmed pick P-1, the lane-map changes are FOR OWNER CONFIRMATION)

Asked by the coordinator after the E5 wrap-side ENC vectors (E5 RESULTS sections 6j and 6k).

1. **ENC-1 tag values (rule P-1).** Already in E3 since openvm 6d70a45 (J7): `tag_fr` is the ASCII name as a little-endian `u128`, and the `DIGEST` parameter is the tag index 0 to 3 in PR 3.6 order (`SHK_PROGC`, `SHK_LOCK`, `SHK_INPUT`, `SHK_OUTCOME`); `L_Fr` (`gen_r_check`) uses these constants, so J5 on f725b15 and 95ed47b already proved with them. The placeholder `k + 1` values and the `DIGEST`-only lane map reading `vs[off + 1]` were those of 2f09caf (checked with `git show 2f09caf:crates/sherlock-lfr/src/iface.rs`), which is what E5 RESULTS section 6j items 1 and 2 describe; E5 should port from 9539418 (the `SPONGE(3)` output is `vs[off + 3]`, `o + vals(op, param) - 1` in general). `N_TAGS` stays 6 (indices 4 `SHK_REG` and 5 `SHK_SET` of ARCHITECTURE line 473); the 4 `R` tags are indices 0 to 3. New test (openvm 52a538c, `enc1_all_vectors`, MEASURED pass): every ENC-1 vector of `research/sherlock-enc-fp3-20261112/VECTORS.txt` that `iface` computes equals the pinned value: the 4 tag values, `digest_1to8` under each of the 4 tags, `d2_progc`, `C_P`, `d2_lock_j1`, `A_1` (plus `input_pub` in the older test).
2. **Lane map with `SPONGE(3)` (D-9).** The reader accepted `DIGEST` or any `SPONGE` since 6d70a45; openvm 099d6d8 narrows it to `DIGEST` or `SPONGE(3)` and refuses every other op, `SPONGE(n)` with `n != 3` included. Test `lane_map_sponge_entries` (MEASURED pass): the two `claim_pub` lanes of a full-V run come from entry 3 and equal its output; pairs pointing at a `PUB`, `PERM`, `H2` or `SPONGE(n != 3)` entry, a lane mapped twice, and a changed `SPONGE(3)` output are refused. The tape of `L_Fr` does not change (the reader is plain Rust), so no re-prove is needed.

3. **Lane `e3-lanemap`** (reader lane map at 099d6d8; report `qa/gemini-quint/e3-lanemap/REPORT.md`; the lead ran the commands: 12 tests pass after 2 wrong expected values in the lane's tests were corrected, `SOUND` and `COMPLETE` 5,000 samples no violation, 5 mutants caught). Verdicts: Q1 acceptance and output index SOUND; Q5 `PUB` interplay SOUND; Q6 COMPLETE; Q2 UNSOUND in two edge cases, Q3 no u16 check, Q4 the lane map is not pinned by the reader. Fixed in openvm 9539418 (`apply_lane_map`): a lane is set at most once by a flag (finding 2a: the old `s[lane] != 0` test let a second pair overwrite a lane whose first mapped output was 0), and lane-map words must be u16 (finding 3); tests pass, J4 on fixture-j8 unchanged (7 positives equal native, 3 negatives refused). Left for the owner: whether the reader refuses lanes 0 and 1 (binding, `comm`) as lane-map targets (finding 2b), and whether the lane map is a wrap key constant or tied to the `R` positions (entry 3 for `claim_pub`, entry 4 for `a_in`; finding 4), with the `a_out` 4 against 5 pair question.

Commits for the E5 port (openvm `e3-iface`, not pushed): 6d70a45 (ENC-1 `tag_fr`, `TAG_NAMES`, DIGEST tag indices, the replay of `R`), 52a538c (the ENC-1 vector test), 099d6d8 (the `SPONGE(3)` lane map), 9539418 (`apply_lane_map`: per-lane flag, u16 words). All are AFK changes: 6d70a45 and 52a538c carry the owner-confirmed rule P-1, and 099d6d8 and 9539418 stay FOR OWNER CONFIRMATION; none changes the `L_Fr` tape (MEASURED: the J4 tape at 9539418, `j10-j4.log`, has the same 502,910 ops and op histogram as at 95ed47b, `j9-j4.log`), so the J5 result of section 12 stands. Small suite at 9539418 (`j10-small.log`): lib 9, `i10_gate` small, `j1_public`, `j5_gate` (J7 and J9 words-level), `j8_onion` (2): pass.
