# T3 results: every AIR required in every recursion layer

Row T3 of `research/sherlock-tested-design-20261113/LOOP.md`. Inputs: the optional-AIR finding of FP8 (`research/sherlock-fp8-20261113/RESULTS.md` section 15.6b), `research/sherlock-e-path-20261112/AUDIT-opus.md` (FP8 row (e), risk 6) and `AUDIT-codex.md` (risk 4). Trees, all on branch `tested`: openvm `~/.cache/m7-wrap/src/openvm-tested`, stark-backend `~/.cache/m7-wrap/src/stark-backend-tested` (no change), Moriarty `~/Projects/Moriarty-wt/tested`. Runner `~/.cache/t3/run.sh` (the T1 runner with T1's target dirs and env, its own state, logs and fixtures under `~/.cache/t3/`).

Labels: MEASURED (read from a run log), DERIVED (computed from MEASURED numbers by a stated rule), ESTIMATE.

## 1. Status

**T3: DONE (INTEGRATED-PASS on `tested`).** The done-when of LOOP row T3 holds:
- Real proofs of N1, N3 and `L` with `KeyBind` left out, and with each of two proof-shape AIRs (`ProofShapeAir`, `PublicValuesAir`) left out, are refused: 30 of 30 refusals and 4 of 4 honest acceptances, in two runs (MEASURED, section 3).
- The three T1 failures pass: `e4_profile_lfr_prove` (845 s), the `lfr_real` export (`exp-lfr-real`, keygen 200.1 s, prove 645.7 s) and the wrap test `i5_lfr_real_native` (MEASURED, section 5).
- Every suite that passed in T1 passes again on the T3 tree (section 5).

The wrap stays at k = 21 and loses 140 rows; the wrap key stays 1,325 bytes. The STARK vks of the recursion layers grow by about 400 bytes each (section 4).

## 2. What changed

Every key of the leaf aggregation, internal (N1, N2, N3), `L` and `L_Fr` sets is now made with every AIR required, as upstream `continuations::prover::keygen_all_required` does. Only the app set keeps optional AIRs, because an app key may leave chips unused (GP-1 to GP-4 stay as they are). All Sherlock keygen goes through `sherlock_lfr::frozen::keygen_checked`, so the change has one place.

| Commit | Repo | What |
|---|---|---|
| `d86f56c` | openvm `tested` | `e4_profile_lfr_prove` and the `lfr_real` export call `Tape::expose` (the E3 rule; T1 panicked at `machine.rs:1189`) |
| `b3fbaa5` | openvm `tested` | required AIRs in `frozen`, the `L_Fr` tape generator refusal, the `fp8_keyset omit` negatives |
| `a4ae673c6`, and the record fix after it | Moriarty `tested` | the wrap shape refusal, this report, the `openvm-tested` patch series |

stark-backend: no change. The stock verifier already refuses a proof without a required AIR (`verifier/proof_shape.rs:332`, `RequiredAirNoVData`). The recursion proof-shape AIR already constrains `is_present = 1` for a required child AIR (`crates/recursion/src/proof_shape/proof_shape/air.rs:331-332`). T3 turns both checks on by keying the layers with required AIRs.

### 2.1 Circuit changes for the Carbon Quint lane

Each item below changes a key or a circuit relation.

1. `crates/sherlock-lfr/src/frozen.rs:211` `FrozenSet::all_required`: true for every set but `APP`.
2. `crates/sherlock-lfr/src/frozen.rs:294` `keygen_required` (every AIR through `add_required_air`); `frozen.rs:308` `keygen_set`; `frozen.rs:453` `keygen_checked` now calls `keygen_set`. Effect: every recursion-layer vk has `is_required = true` on every AIR. In the parent, the proof-shape AIR over such a child gets `when_current.assert_one(local.is_present)` per child AIR (recursion `proof_shape/air.rs:331-332`), and the presence-bit transcript receive (`air.rs:458-470`) gets multiplicity 0 for those AIRs. In the stock verifier and in the `L_Fr` tape, the presence bit is no longer observed (`stark-backend verifier/mod.rs:137`, `lverify.rs:296`, wrap `verifier.rs:306`).
3. `crates/sherlock-lfr/src/frozen.rs:432-433`: G1 (`check_vk_bounds`) refuses a key of a required set with an optional AIR (`G1Error::Optional`).
4. `crates/sherlock-lfr/src/frozen.rs:279-285` `keygen_any`: at non-frozen test parameters it is `keygen_required` (was plain keygen).
5. `crates/sherlock-lfr/src/lverify.rs:259-266` (`gen_part1`): the `L_Fr` tape generator refuses a shape whose AIR count differs from the `L` key (`ShapeError::Lengths("AIRs")`) or that leaves out a required AIR (`ShapeError::RequiredAbsent(i)`, `lverify.rs:245`). No tape, so no `L_Fr` key, exists for such a shape.
6. `tools/sherlock-wrap/src/verifier.rs:124-128` (Moriarty, `Shape::from_wrapin`): the wrap relation refuses to build from a key fixture in which a required AIR has no trace.
7. `crates/sherlock-wrap-export/examples/fp8_keyset.rs:272`: `check_layout` asserts that a bound layer has no optional AIR (was an `eprintln`); `fp8_keyset.rs:136` the keys-mode fallback keys through `frozen::keygen_set`.
8. Test code only: `crates/sherlock-lfr/tests/e4_profile.rs:72` (`Tape::expose`), `:92` (`keygen_required`); `crates/sherlock-wrap-export/examples/lfr_real.rs:31` (`Tape::expose`); the new `omit` mode, `fp8_keyset.rs` from `fn air_idx` to the end of `fn omit`.

## 3. Negatives on real proofs (`fp8_keyset omit`)

Reading of "refused at N1, N3 and `L`": a real proof of the layer N1, N3 or `L` with one AIR left out is refused (a) when the layer is proved without that AIR's trace, (b) by the stock verifier of the layer, and (c) one layer up: N2 over the N1 proof, N3 over the N3 proof, and the `L_Fr` tape generator over the `L` proof. The chain is app W1 (2^14 rows) -> leaf layer -> N1 -> N2 -> N3 -> N3 over N3 -> `L` (`L_iface` with `KeyBind`, `Dims::FULL`, case `AllEntries`, seed 14), all under the frozen sets. Omitted AIRs: `KeyBindAir`, and the proof-shape module AIRs `ProofShapeAir` and `PublicValuesAir`.

Result (MEASURED): **34 of 34 cases behave as expected** in both runs. Run 1 ran on the uncommitted tree (`~/.cache/t3/logs/omit-run1.summary`, `~/.cache/t3/out/omit/omit-run1.json`, wall 15.7 min), and run 2 ran on openvm `b3fbaa5` with the proof-shape constraint count below (`~/.cache/t3/logs/omit-run2.summary`, `~/.cache/t3/out/omit/omit.json`, wall 16.9 min). There are 4 honest acceptances and 30 refusals. The keys of leaf, N1, N2, N3 and `L` have every AIR required (`all_required` all true).

| Layer | Omitted AIR (index) | (a) prover path | (b) stock verifier | (c) one layer up |
|---|---|---|---|---|
| N1 | `KeyBindAir` (0) | `NonZeroRootSum` | `RequiredAirNoVData { air_idx: 0 }` | N2 trace generation panics: child KeyBind public values 0 of 82 |
| N1 | `ProofShapeAir` (17) | `NonZeroRootSum` | `RequiredAirNoVData { air_idx: 17 }` | N2 trace generation panics (index out of bounds) |
| N1 | `PublicValuesAir` (18) | `NonZeroRootSum` | `RequiredAirNoVData { air_idx: 18 }` | N2 trace generation panics (index out of bounds) |
| N3 | `KeyBindAir` (0) | `NonZeroRootSum` | `RequiredAirNoVData { air_idx: 0 }` | N3-over-N3 trace generation panics: child KeyBind public values 0 of 82 |
| N3 | `ProofShapeAir` (17) | `NonZeroRootSum` | `RequiredAirNoVData { air_idx: 17 }` | N3-over-N3 trace generation panics (index out of bounds) |
| N3 | `PublicValuesAir` (18) | `NonZeroRootSum` | `RequiredAirNoVData { air_idx: 18 }` | N3-over-N3 trace generation panics (index out of bounds) |
| `L` | `KeyBindAir` (43) | `NonZeroRootSum` | `RequiredAirNoVData { air_idx: 43 }` | `L_Fr` tape: `RequiredAbsent(43)`; with the pinned honest shape `Heights` |
| `L` | `ProofShapeAir` (16) | `NonZeroRootSum` | `RequiredAirNoVData { air_idx: 16 }` | `L_Fr` tape: `RequiredAbsent(16)`; with the pinned honest shape `Heights` |
| `L` | `PublicValuesAir` (17) | `NonZeroRootSum` | `RequiredAirNoVData { air_idx: 17 }` | `L_Fr` tape: `RequiredAbsent(17)`; with the pinned honest shape `Heights` |

Honest cases accepted: N2 over N1, N3 over N3, `L` over N3, the `L_Fr` tape over that `L` proof, plus every honest layer proof of the chain (each `bound_prove` verifies).

Which check fires:
- (b) is the check that T3 turns on: `RequiredAirNoVData` exists only for required AIRs.
- (a) is the LogUp balance. It fires with optional keys too (control below), so it is not evidence for T3 by itself.
- (c) at the nodes is a refusal by honest trace generation, not by a constraint; an adversarial prover can write any trace. The constraint that refuses such a child in the parent circuit is `assert_one(local.is_present)` for each required child AIR (`proof_shape/air.rs:331-332`), which the parent key now carries: the `ProofShapeAir` of N2 has 125 constraints over the T3 N1 key and 85 over the optional N1 key, 40 more, one per N1 AIR (MEASURED, `omit-run2.summary`, `T3CTL n2_proof_shape_air`). Its interaction count is 78 in both: the presence-bit transcript receive stays, with multiplicity `not(is_required) * is_valid`, which is 0 for every required AIR.
- (c) at `L` is the new tape-generator refusal (`RequiredAbsent`); the honest-shape run shows that the pinned `L_Fr` tape also refuses (`Heights`).

Control (no verdict, MEASURED): with N1 keyed by plain keygen (every AIR optional, the pre-T3 keys), the same three omissions are refused too: the prover path by `NonZeroRootSum` and the stock verifier by `InvalidNumeratorTerms { expected: 39, actual: 40 }`. So on these cases the bus-balance argument of FP8 15.6b holds (now MEASURED, was DERIVED), and T3 adds a second, direct check. T3 removes the dependence on the bus argument for every AIR: an AIR whose omission would leave every bus balanced (for example one with no interactions) is now refused as well (DERIVED from the `RequiredAirNoVData` check, which does not depend on buses).

## 4. Key and wrap sizes

| Item | Before (optional AIRs) | After (T3) | Label |
|---|---|---|---|
| bound leaf layer vk bitcode (before: FP8 `neg` run on the fp8 tree, `~/.cache/fp8/out/kb/neg/neg.json`) | 79,525 B | 79,525 B (pre-hash changed) | MEASURED (FP8 `neg.json`, T3 `omit-run1.json`) |
| bound N1 vk | 85,515 B | 85,918 B (+403) | MEASURED |
| bound N2 vk | 85,457 B | 85,860 B (+403) | MEASURED |
| bound N3 vk | 86,496 B | 86,899 B (+403) | MEASURED |
| bound `L` vk | 243,725 B | 244,127 B (+402) | MEASURED |
| E2 fixture `L` vk (39 AIRs) | 80,439 B | 80,833 B (+394) | MEASURED (`l_fixture.json`) |
| E3 fixture `L_iface` vk (44 AIRs) | 241,251 B | 241,646 B (+395) | MEASURED (`iface_fixture.json`) |
| `L_Fr` AIRs required (lfr-e3, lfr-real) | 0 of 15 | 15 of 15 | MEASURED (wrapin `vk`) |
| wrap rows at k = 21, lfr-e3 | 1,668,323 | 1,668,183 (-140) | MEASURED (`wrap ... 21 cost`) |
| wrap rows at k = 21, lfr-real | (no fixture in T1) | 1,438,971 | MEASURED |
| wrap columns (advice, fixed, lookups) | 8, 27, 1 | 8, 27, 1 | MEASURED |
| wrap `MidnightVK` bytes, lfr-e3 (`wrap lfr-e3 ... 21 0`) | 1,325 B | 1,325 B (other bytes: sha256 `82eb9f8b...` before, `599da624...` after) | MEASURED (`~/.cache/t3/logs/wrap-key-{before,after}.log`) |
| wrap keygen, prove, library verify, lfr-e3 | 90.7 s, 136.7 s, Ok | 88.6 s, 138.5 s, Ok | MEASURED (shared host, not `--exclusive`) |

The STARK vks grow by about 400 bytes per layer whose child is required (one `assert_one` per child AIR in the proof-shape AIR, DERIVED from the table). These vks are not the ledger key; the ledger key is the wrap `MidnightVK`, bound at 50,000 bytes. The wrap loses 140 rows (MEASURED); the likely cause is that the `L_Fr` transcript no longer absorbs 15 presence bits (DERIVED, not isolated). k stays 21; headroom to the gate of 1,782,573 rows is 114,390 rows for lfr-e3 (DERIVED).

## 5. Tests on the T3 tree

Same commands as T1 (`~/.cache/t1/run.sh`), run by `~/.cache/t3/run.sh` on openvm `b3fbaa5`. The stages up to j5 ran on `d86f56c` plus the then uncommitted T3 diff (`runs.log` shows `ovm d86f56c ovm-dirty 4`). `b3fbaa5` commits that diff; the `sherlock-lfr` and export sources were not edited after the builds of those stages, and only `fp8_keyset.rs` gained the control counted in run 2 (`git diff b3fbaa5` is empty at handback). The stages from m7-bls on ran on `b3fbaa5` itself. Every stage used stark-backend `0004791` and Moriarty `52b71e250` with the `verifier.rs` change (committed in `a4ae673c6`). The fixtures were made again (`l_fixture`, `iface_fixture`, every export), because T3 changes every recursion-layer key. Logs: `~/.cache/t3/logs/<stage>.log`, times in `~/.cache/t3/logs/runs.log`.

| Stage | T1 | T3 (MEASURED) |
|---|---|---|
| ovm build, export build | pass | pass |
| `fix-l`, `fix-iface` | pass | pass (made again) |
| sherlock-lfr suite (lib and tests, i0, i5_l, i9 included) | 34 passed, 1 failed (`e4_profile_lfr_prove`) | 35 passed, 0 failed |
| `exp-fixtures`, `exp-enc`, `exp-lfr-e3-dec`, `exp-lfr-e3` | pass | pass |
| `exp-lfr-real` | panic at `machine.rs:1189` | pass |
| i10 gate at the `L_Fr` set | pass | pass (2,287 s) |
| j5 gate at the `L_Fr` set | pass | pass (3,244 s) |
| m7-bls | 6 passed | 6 passed |
| lfr-gpu | 3 passed, 1 ignored | 3 passed, 1 ignored |
| wrap lock check | pass | pass |
| wrap suite (all ignored included) | 24 passed, 1 failed (`i5_lfr_real_native`) | 25 passed, 0 failed |
| `fp8_keyset omit` (new) | none | 34 of 34, twice |
| sdk-node | 140 of 144 pass, 3 fail | 140 of 144 pass, 3 fail (same 3) |

sdk-node note: in both runs `rollup-anchor`, `rollup-keys` and `settle-harness` fail at their first assertion, `SHERLOCK_SDK_ROOT must identify the pinned SDK package directory` (the runner does not set it). The T1 runner logs `rc=0` for this stage because it records the status of `echo`, not of `node`. This is outside T3 and unchanged by it; the T1 record "sdk-node pass" is wrong and needs a rerun with `SHERLOCK_SDK_ROOT` set.

stark-backend suites (sb-nextest, sb-fr, sb-bb-bytes) were not rerun: T3 changes no stark-backend file, and those suites do not build sherlock-lfr.

## 6. Hard rules

- k = 21 for the wrap (both fixtures under the k = 21 gate); no circuit above k = 21.
- Parameters of every frozen set unchanged (G1 `check_params` passes in every keygen); D3 depends on the parameters and the measured shapes, and the shapes are unchanged (same AIRs, same heights in the fixtures), so the 100-bit analysis is not affected (DERIVED; D3 not rerun). Wrap key 1,325 bytes, at most 50,000 (MEASURED).
- One universal key set: the required flag is a property of each AIR of a layer and adds no program data to N1, N2, N3, `L` or `L_Fr`, so the program independence that FP8 measured (`univ.json`) is unchanged (DERIVED; `univ` mode not rerun).
- Ledger 9.1.0.1 verifier: unchanged; no ledger run in T3.
- GP-1 to GP-4: the app set keeps optional AIRs, so no app shape is refused.
- `KeyBind` is withdrawn as the general key route (O10); T3 requires it, and every other AIR that exists today. T7's `prog_d` route inherits the required-AIR keygen from `frozen`.

## 7. Open

- The parent-circuit refusal of an omitting child is shown by the constraint in the parent key, not by an adversarial trace that passes every other constraint (section 3).
- No test covers `G1Error::Optional` (`frozen.rs:432-433`), `ShapeError::Lengths("AIRs")` (`lverify.rs:260-262`) or the wrap `from_wrapin` assert (`verifier.rs:124-128`); every key in the runs is all-required, so these checks never fired. The Quint lane should take them.
- No case shows optional keys accepting what required keys refuse: on the three AIRs tested, the optional-key control refuses too (section 3).
- The `fixtures` mixtures (`mix-small`, `mix-lfr`) keep optional AIRs on purpose: they test the wrap's presence-bit path, which a required-only key set no longer uses.
