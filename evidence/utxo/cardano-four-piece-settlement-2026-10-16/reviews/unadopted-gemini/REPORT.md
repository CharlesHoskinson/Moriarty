# Independent Full Candidate & Operator Source-and-Effect Review (`M2` + `O2`)

## 1. Executive Verdict & Independence Statement

- **Review Token**: `current-cardano-four-way-vs-harness-repair-independent-gemini-full-candidate-effect-review-20261016`
- **Reviewer**: Fresh independent `gemini-4-argon-high` (`HIGH` thinking) context authorized via `USER-ROUTING-ADDENDUM-20261016.json` (`fdacaaa7f14875b823ebfe634a2037e3e4f05d9deb7196584dea42b937aecb22`) after two Opus tasks terminated with provider errors.
- **Independence**: This reviewer did not author or execute candidate `M1`, candidate `M2`, operator `O1`, operator `O2`, the Muse diagnosis/repair proposals, or any prior review. All 41 rows of `M2` (`ROOT-SOURCE-MANIFEST.json` SHA256 `54e0fc5629068371d2e1c2ff77945abe4394c8cd2632dbfcf5be0ac2dde07a47`, 14,506 B) and all 34 files of `O2` (`ROOT-SOURCE-MANIFEST.json` SHA256 `a916a452553a00dc4d814caff3e00d600b6e2bf67a89a15b1cbafd800fc49ff5`, 11,831 B) were inspected directly from the read-only public mirror. No shell commands, builds, tests, RNG, setup, proving, verification, wallet, ledger, or subagents were executed.
- **Falsifiable Predicate Verdict**: **YES (`CONDITIONAL_SOURCE_AND_OPERATOR_READY_FOR_ONE_ORDINARY_MEASUREMENT`, `prerequisiteClosed: true`)** — Current candidate `M2` (containing the test-only single-constant fix `vs2_t`: `0xc2`×28 -> `0xd2`×28 in `candidate/aiken/lib/pvc2/shard4_harness.ak`) paired with the concrete 34-file operator `O2` and `PRE-LAUNCH.json` (`74b248ed2f84cd3296b34be2904b3ae4765199732043e850d15b6f3efa162aef`, `targetAbsent: true`) is source-complete, logically sound within its bounded local working-target scope, and ready for **one bounded ordinary 15-step, 147-case local measurement run**, subject to separate root `AUTHORIZATION` and subsequent post-run independent actual-result review.

---

## 2. Candidate `M2` Composition & Single-Constant Harness Repair

### 2.1 Manifest & Diff Verification (`CC-M2-COMPOSITION-AND-SINGLE-CONSTANT-DIFF`)
- **Candidate Root (`M2`)**: `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-materialized-20261016` (`ROOT-SOURCE-MANIFEST.json` SHA256 `54e0fc5629068371d2e1c2ff77945abe4394c8cd2632dbfcf5be0ac2dde07a47`, 14,506 B, 41 rows: 32 package files under `candidate/aiken/` [31 `.ak` + `aiken.toml`] and 9 doc rows).
- **Exact Single-Constant Change**: Per `COMPOSITION.json` (`8be56bc97a8785d7fa857209c39c4c451c32753b1820e2d70261e952104a2deb`) and `EXACT-HARNESS-DIFF.patch` (`22c3a8922bb8363ca6a469928b7e5bf446cde4c1553b2227e403869540c49191`), `M2` is a 40-file byte-for-byte copy of parent `M1` (`dcdf544a47e5eb15e0ab2b7debb7789d7da4e9f52b771ac90ed104068c744ca5`) plus a 1-line constant change in `candidate/aiken/lib/pvc2/shard4_harness.ak` (7,967 B, SHA256 `dd635e53b0bad05146a6c65a0f56768b97f09cccbeff771df6a8b84c9725ebb9` -> `0df2a7a74ca2c79984a36fdf4e1b3f10739d18d8a10375aaea96e9b8fce497da`):
  - Lines 25-26: `pub const vs2_t: ByteArray = #"d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2"` (replacing `#"c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2"`).
- **Unchanged Production & Test Assertions**: All 7 validators (`c2.ak`, `vi.ak`, `s2p.ak`, `vs1.ak`..`vs4.ak`), all production library modules (`shard4.ak`, `shard.ak`, `core.ak`, `claim.ak`, `catalogue.ak`, `intent.ak`, `thread.ak`, `groth16.ak`, `bytes.ak`, `diag.ak`, `types.ak`), all 12 test modules (`shard4_test.ak`, `shard4_cost_test.ak`, `shard_test.ak`, `shard_cost_test.ak`, `settle_r1_test.ak`, `unit_r1_test.ak`, `unit_test.ak`, `w0_cost_test.ak`, `w1_gate_test.ak`, and helpers `shard_harness.ak`, `test_ctx.ak`, `fixtures_r1.ak`), and `EXPECTATIONS.json` (`80cf1601480be0c415649ac1b071326b0097f89c2132279b3636a38c15df8393`) are 100% unmodified.

### 2.2 Diagnosis Verification: 18 Failing Tests & 7 Negative Alias Tests (`CC-IDENTITIES-DISTINCT-18-FAILURES-AND-7-NEGATIVE-ALIAS-TESTS`)
- **Production Guard Unweakened**: `shard4.identities_distinct(vs1, vs2, vs3, vs4, c2, vi, thread_policy)` in `shard4.ak:230-242` checks all 21 pairwise inequalities across the 7 script hashes, including `vs2 != c2` at line 235.
- **Root Cause of Prior 18 Failures**: In `fixtures_r1.ak:379-380`, `c2_hash = 0xc2*28`. In parent `shard4_harness.ak`, `vs2_t` was also `0xc2*28`, causing `pieces_t()` (`shard4_harness.ak:29-36`) to fail `vs2 != c2` whenever `identities_distinct`, `delegated`, or `genesis` was called with `fx.c2_hash`.
- **Distinctness of Repaired Test Identifiers**: With `vs2_t = 0xd2*28`, the test identifiers `vs1_t = 0xc1*28`, `vs2_t = 0xd2*28`, `vs3_t = 0xc3*28`, `vs4_t = 0xc4*28` (`shard4_harness.ak:22-32`) are pairwise distinct from `fx.c2_hash = 0xc2*28`, `fx.c2_hash_foreign = 0xcc*28` (`fixtures_r1.ak:379-382`), `h.vi_t = 0xb1*28`, `h.vs_t = 0xb2*28`, `h.foreign = 0xb3*28` (`shard_harness.ak:20-26`), and fixture thread policies `0xa2*28` and `0xe2*28`.
- **All 18 Previously Failing Tests**:
  1. `q_p01_accept`..`q_p07_accept` and `q_p14_accept`..`q_p17_accept` (11 tests, `shard4_test.ak:86-150`, via `accept_case` lines 48-82): call `shard4.piece_1..4` and `shard4.delegated(..., fx.c2_hash, h.vi_t, q.pieces_t(), ...)`. Previously failed only on `vs2_t == fx.c2_hash` in `delegated`; now consume the repaired fixture without collision.
  2. `q_p08_genesis` and `q_p09_genesis` (2 tests, `shard4_test.ak:153-169`): call `shard4.genesis(..., fx.c2_hash, h.vi_t, q.pieces_t(), ...)`. Previously failed `identities_distinct` at `shard4.ak:279`; now consume the repaired fixture.
  3. `q_distinct_fixture_accepts` (1 test, `shard4_test.ak:525-532`): directly checks `shard4.identities_distinct(q.vs1_t, q.vs2_t, q.vs3_t, q.vs4_t, fx.c2_hash, h.vi_t, fx.p01_policy)`.
  4. `s4c_p01_s2p`, `s4c_p04_s2p`, `s4c_p07_s2p_0`, `s4c_p07_s2p_1` (4 cost tests, `shard4_cost_test.ak:43-50, 79-86, 115-132`): benchmark `shard4.delegated` with `fx.c2_hash`, `h.vi_t`, and `q.pieces_t()`, enabling `OPX2x` to measure true accepting `S2'` ExUnits rather than early-refusal cost.
- **All 7 Negative Alias Tests (`shard4_test.ak:534-606`)**: `q_alias_vs1_vs2` (`vs1==vs2`), `q_alias_vs3_vi` (`vs3==vi`), `q_alias_vs4_c2` (`vs4==c2`), `q_alias_vs2_policy` (`vs2==thread_policy`), `q_alias_three_pieces` (`vs3==vs4`), `q_alias_s2p_refuses` (`vs4==vs1` in `delegated`), and `q_alias_genesis_refuses` (`vs2==fx.c2_hash` in `genesis`) now each isolate their intended single-alias refusal.

---

## 3. Whole-Candidate Logical Body Audit (`M2`: 31 `.ak` Files + `aiken.toml`)

### 3.1 Four-Way Piece Partition & All 16 Claim IDs (`CC-FOUR-WAY-16-CLAIM-PARTITION-AND-HASH-CHAIN-DAG`)
Inspection of all 7 validators and 24 `lib/pvc2/` modules confirms:
- **`VS1` (`vs1.ak` -> `shard4.piece_1`, `shard4.ak:44-89`, `core.check_transition_claims_1`, `core.ak:1126-1212`)**: Enforces `threads == covered_refs(thread_policy, tx)` (`RR-06`), transaction-wide `core.tx_output_guards(thread_policy, tx)` (`core.ak:1084-1124`: no output reference scripts, `RR-16` `count_policy_outputs(tx.outputs, thread_policy, 0) == list.length(threads)`), per-thread `core.thread_pre_checks`, Groth16 proof verification (`groth16.ak`), and `catalogue.verify_claims_1` (`catalogue.ak:448-518`) for **5 claim IDs**: `{0, 1, 2, 4, 5}`.
- **`VS2` (`vs2.ak` -> `shard4.piece_2`, `shard4.ak:93-138`, `core.check_transition_claims_2`, `core.ak:1214-1301`)**: Enforces `threads == covered_refs(thread_policy, tx)`, `core.thread_pre_checks`, and `catalogue.verify_claims_2` (`catalogue.ak:520-586`) for **4 claim IDs**: `{3, 6, 7, 8}`.
- **`VS3` (`vs3.ak` -> `shard4.piece_3(c2_hash, vi_hash, ...)`, `shard4.ak:142-189`, `core.check_transition_claims_3`, `core.ak:1303-1402`)**: Parameterized by `(c2_hash, vi_hash)`. Enforces `threads == covered_refs(thread_policy, tx)`, `shard.vi_withdrawn(vi_hash, threads, tx.redeemers)`, `core.thread_pre_checks`, and `catalogue.verify_claims_3` (`catalogue.ak:588-654`) with `claim_script = c2_hash` for **4 claim IDs**: `{9, 10, 11, 12}` (including external `C2` claim verification via `claim.ak`).
- **`VS4` (`vs4.ak` -> `shard4.piece_4`, `shard4.ak:193-224`, `core.check_transition_claims_4`, `core.ak:1404-1482`)**: Enforces `threads == covered_refs(thread_policy, tx)`, `core.thread_pre_checks`, and `catalogue.verify_claims_4` (`catalogue.ak:656-717`) for **3 claim IDs**: `{13, 14, 15}`.
- **Exact Partition & Range Guard**: $\{0,1,2,4,5\} \cup \{3,6,7,8\} \cup \{9,10,11,12\} \cup \{13,14,15\} = \{0..15\}$ ($5+4+4+3=16$ claim IDs, pairwise disjoint, with `catalogue.ids_in_range` rejecting any ID outside `0..15`).
- **`S2'` Delegation & Hash Chain DAG (`VI <- C2`, `VS3 <- C2 + VI`, `S2' <- C2 + VI + VS1..VS4`)**: `s2p.ak:18-37` resolves `own_policy = shard.own_policy_id(own_ref, self.inputs)` and calls `shard4.delegated` (checking `identities_distinct`, `piece_lists` equality `t1 == t2 == t3 == t4` under `own_policy` across all 4 piece withdrawals, and `contains_ref(t1, own_ref)`). Carry-forward conditions `FV-01` (`S2'` delegates `VI` withdrawal checking to `VS3` and must be parameterized via the exact 4-step hash chain from the same `C2`, verified offline by `OPX4`) and `FV-02` (shared unparameterized `VS1`/`VS2`/`VS4` hashes prevent two distinct `S2'` deployments from settling in one transaction) remain non-blocking.

---

## 4. Concrete 34-File Operator `O2` & Exact 8 `HR-*` Clauses Audit

- **Operator Root (`O2`)**: `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-operator-materialized-20261016` (`ROOT-SOURCE-MANIFEST.json` SHA256 `a916a452553a00dc4d814caff3e00d600b6e2bf67a89a15b1cbafd800fc49ff5`, 11,831 B). All 34 files (15 step scripts `OPX0.sh`..`OPX7f.sh`, 8 tools in `tools/`, 4 pin tables in `data/`, 7 docs) are materialized inside `O2` with zero runtime `CARRY` or reference to `O1`.
- **Host Capability Envelope**: Aiken `v1.1.21+42b308e` (`38,299,808` B, SHA256 `4257cc8be37cbde5bf7b5dd111e43a1277f45352fb2433ad7a7e647209aa7677`); all 6 Aiken steps run under `unshare --user --map-root-user --net`, `prlimit --as=34359738368 --core=0 --nproc=512 --nofile=1024`, `nice -n 10`, and `timeout --signal=TERM --kill-after=10s <step_seconds>` (aggregate wall budget `1800`s, 1 attempt, 0 retries; no mount-namespace isolation, so abstract UNIX sockets are barred while path-based UNIX sockets on visible host paths remain reachable; no `RLIMIT_FSIZE`; no CPU affinity pinning).
- **Pre-Launch Target State (`PRE-LAUNCH.json`)**: SHA256 `74b248ed2f84cd3296b34be2904b3ae4765199732043e850d15b6f3efa162aef` confirms `targetAbsent: true` for `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-local-measurement-20261016` and `authorizationPresent: false`.
- **All 8 Exact `HR-*` Clauses (`EXACT-OPERATOR-CLAUSES.json`, SHA256 `4b5e9729979c6dfcee59f9209e41fc0e9e9ad36a963c25b737ff2e6331229760`)**: Independently verified and **AGREED** (`HR-ORD`, `HR-FILT`, `HR-PIN`, `HR-EXP`, `HR-HYG`, `HR-CLASS`, `HR-CAP`, `HR-TOOLS`).

---

## 5. Size, Prediction, and Benchmark Hygiene & Open Boundaries

- **Sizes & `RANGE-vs1` Prediction**: Because `M2` modifies only test-only `shard4_harness.ak`, compiled validator sizes are unchanged from the first four-piece run (`vs1=6,859` B, `vs2=9,940` B, `vs3=10,832` B, `vs4=10,825` B, `s2p=13,018` B, `c2=13,989` B, `vi=1,565` B; 4-piece sum `38,456` B; 7-validator total `67,028` B, `+18,615` B over attempt 2's `48,413` B). Informative prediction `RANGE-vs1` (`[7000, 10500]` B in frozen `EXPECTATIONS.json` `80cf1601...`, alongside `T-147` and `PC4-4`) will remain `FALSE` (`allPredictionsHeld: false`) and is never promised all-green.
- **Scope Separation**:
  1. **Source & Operator Gate (`CLOSED`)**: `M2` + `O2` are ready for one bounded 15-step, 147-case local measurement run.
  2. **Execution Authority (`BLOCKED` pending root `AUTHORIZATION`)**: This review grants zero execution authority.
  3. **Actual Measurement Evidence (`UNRUN`)**: Requires authorized `O2` execution into `T` followed by separate independent actual-result review.
  4. **Broader Families (`OPEN`)**: Full Cardano ledger/tx/fee/reference-script/ExUnits fit, native Groth16 pairing verification, all 20 UTXO families, ACTUS/DeFi/general obligations, recursion, private handoff, bounded multi-parent PCD, universal certificate catalogs, and real Preprod execution remain open with no percentage of compatibility claimed.
