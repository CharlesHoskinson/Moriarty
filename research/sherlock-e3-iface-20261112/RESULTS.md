# E3 results: the full `L` interface carried through `L_Fr`

Plan: `PLAN.md` beside this file (decisions D-5 to D-9, increments J1 to J6).

## 1. Status

**E3 gate: NOT REACHED.** J1 and J2 pass. The `L_iface` fixture (J3) is proving; J4 and J5 run next.

| # | Increment | Status | Evidence |
|---|---|---|---|
| J1 | `Entry::Public`, D-6 carrier in `L` and `L_Fr`, small `L_iface` | PASS | section 3 |
| J2 | native reference `iface.rs` | PASS (unit tests; used by J1) | section 4 |
| J3 | `L_iface` fixture at the OD-19 `L` set | example written | section 5 |
| J4 | `R` check (D-7), op counts | written | section 6 |
| J5 | gate at `lfr_params`, D3 | written | section 7 |
| J6 | optional replay | not started | |

## 2. Code

| Repository (not pushed) | Branch | Commits |
|---|---|---|
| openvm worktree `~/.cache/m7-wrap/src/openvm-e3` | `e3-iface` (from `e2-lfr` c4bd312) | 96acfa3 J1 and J2 |
| Moriarty `~/Projects/Moriarty-wt/e3-iface` | `e3-iface` (from `e2-lfr` fbba858f1) | none yet |

Runner `~/.cache/e3-iface/run.sh <stage> <mem> <dir> cmd...` (heavy-slot, `CARGO_TARGET_DIR=~/.cache/e3-iface/target`, logs `~/.cache/e3-iface/logs/`).

**2a. Files (crate `crates/sherlock-lfr`).**

| File | Content |
|---|---|
| `src/carrier.rs` | D-6: the public-value carrier (selectors `a[h1]`, `b[h2]`, `w` value cells per row; uniqueness argument in the header) |
| `src/iface.rs` | J2: index table (`Dims`, 12,164 words at `Dims::FULL`), log operations, native values with Midnight Poseidon (u = 0 and u = 4), OD-17 planner, `Run::words`, `Run::statement`, the decoder `decode` (D-9), the cases of J3 |
| `src/iface_l.rs` | D-5: the `L`-side interface AIRs `Stmt`, `LogEntry`, `LogValue`, `Call` and their traces |
| `src/lverify.rs` | `Entry::Public` (part 2 reads the per-AIR public addresses of part 1); `gen_verifier_full`; `gen_r_check` (D-7) |
| `src/machine.rs`, `src/tape.rs` | `PublicAir` as a carrier, 16 words per row (`PUBLIC_W`); `Tape::expose`; `Op::PublicPad` (the zero read by the empty cells of the last row) |
| `src/record.rs` | `stock_log_at` (the stock record at any `L` parameter set) |
| `examples/iface_fixture.rs` | J3 fixture |
| `tests/j1_public.rs`, `tests/j5_gate.rs` | the gates of J1, J4 and J5 |

## 3. J1: public values of `L` through `L_Fr` (MEASURED)

Test `j1_public_values_through_lfr` (log `~/.cache/e3-iface/logs/j1-final.log`). Small `L_iface`: the 4 interface AIRs at `Dims { b: 32, v: 32, p: 32, l_tx: 14 }` (610 public words), `L` config, `small_params_n(4, 8)`, case full-V (14 entries, 32 values).

| Check | Result |
|---|---|
| Stock verifier on the small `L_iface` | accepts; 4 AIRs at log heights 2, 5, 5, 2; 3,337 transcript events |
| Tape (whole `L` verifier with `Entry::Public`) | 59,566 ops; every one of the 3,337 events equals the stock record; no assertion fails |
| `L_Fr` public words | 78 = 1 + ceil(610 / 8); word 0 is the `L` vk pre-hash; `decode` gives the native statement (all 16 lanes) |
| `L_Fr` proof at `small_params_n(3, 14)` | accepted; 2 commitments (stacked widths 80 and 20); 4,415,746 bytes; prove 62.3 s |
| Changed `L` public value (the last call-vector word + 1) | stock `L` verifier refuses; tape: 142 assertions fail, the tape (so the key) is unchanged; `L_Fr` prover refuses (`LogupZerocheck(NonZeroRootSum)`) |
| Carrier controls (same witness, changed `L_Fr` public values): first word, middle word, last word, two words swapped | each refused by the stock verifier (`BatchConstraintError(SumClaimMismatch)`) |

Also pass on the same tree: `i10_public_air_small` (the E2 test, now with `Tape::expose`) and the `iface` unit tests (layout 12,164 words, `W_pv` 2,816; limb round trip; `Canon` refuses `r` and a limb at `2^30`).

## 4. J2: the native reference (CODE)

- Layout of ARCHITECTURE 1.11 FP-3: words 0 to 113 the statement head (`mode` at 74, lane map at 106 to 113), the entry table at 114, the value table at 626, the call vector at 2,930; 12,164 words (DERIVED and asserted).
- Entries as ARCHITECTURE 1.8: values and slots per operation; bounds `SPONGE 1..V-1`, `PATH 1..32`, `PUB 0..1+L_TX`, `DIGEST` 4 tags; `ECMUL`, `ECMULG` never admitted (`E = 0`).
- Native values: `PERM` the Midnight permutation; `H2`, `SPONGE(n)` the Midnight fixed-length sponge at u = 0 (capacity `l`); `PATH(d)` `H2` per level with the index bit choosing the order (placeholder, D-8); `DIGEST` the sponge at u = 4 of `(tag, pack(d2))` with tag `Fr(k + 1)` (placeholder, D-8).
- Planner (OD-17): `R` first (refused if it does not fit), then each discretionary operation if its bound and all four limits still hold, else tier E.
- Decoder (D-9): unpack the `L_Fr` words, range and `Canon` of every lane and value, bounds, `NOP` padding, capacity, zero value rows after `off_n`, the lane map (each mapped entry a `DIGEST`, its lane zero in `pv`), every `PUB(i)` against lane `i`; output the statement (the call vector with the lane-map lanes filled).
