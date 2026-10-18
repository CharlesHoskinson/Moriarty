# Fresh independent actual-result review: repaired four-way M2/O2 ordinary run T2

**Reviewer:** canonical Claude Opus 5.5, in a fresh context. I authored none of the candidate, operator, controller or run. I made no tool calls and executed nothing. Every check below is arithmetic or textual comparison over the inline packet. This review grants no execution authority.

## 1. Verdict

**ACTUAL_PREDICATE_CLOSED** for this narrow finite predicate:

> Under AUTH `1a35701d…3f06`, one ordinary run of O2 (`a916a452`) against M2 (`54e0fc56`, 14,506 B) did the following:
> - invoked all 15 steps once, in the fixed order, each exiting 0;
> - matched all 147 frozen expectations of `EXPECTATIONS 80cf1601` (125 `shard4_test` with 9 `fail`, plus 22 cost benches) on name, membership, fail flag and status;
> - passed the hash-chain falsifiers;
> - kept every applied script at or under the 14,336 B working target;
> - held PS4-1..7, PC4-1..4 and OP-P1..P5;
> - preserved RANGE-vs1 = FALSE (informative) as the only miss;
> - passed tree hygiene at three points and final integrity;
> - closed the GREEN condition with OPX7f.exit = 0.

`allCommandsExit0 = true` and `allPredictionsHeld = false` stay distinct. GREEN here means source-measured only. It is not proof, ledger, deployability, fee, ExUnits or settlement acceptance.

## 2. Raw cross-checks

### 2.1 Controller, exits, integrity

- **Receipt and AUTH binding.** `OPERATOR-RECEIPT.json` has `authorizationSha256 = 1a35701d…`, which equals the AUTH row. Status is `terminal-all-15-invoked`, retries 0, elapsed `328.42336079699453` s.
- **Wrapper exits.** All 15 `wrapperExit` values are 0.
  - `recordedExit` is `"0"` for the 13 log-writing steps.
  - OPX0 is `null`, as designed: OPX0 writes nothing. Its exit is in `OPX0-stdout.txt` (`OPX0 end … exit 0`).
  - OPX7f is `"0"`, read from `T/OPX7f.exit`. The bytes `9a271f2a…` equal `"0\n"`, the same sha as every other `.exit` row in the result manifest.
- **Integrity checks.** There are 16 entries (before each of the 15 steps, plus after the final one), all `files 34, manifestMatch true, match true`.
- **Controller binding.**
  - `run-reviewed-ordinary.py` is 3,046 B, sha `ab77250c`. Both values equal `AUTH.rootRunner`.
  - The runner asserts its own sha against AUTH and takes the AUTH sha as `argv[1]`. There is no self-reference cycle.
  - `ROOT-CONTROLLER-DIFF.patch` shows exactly the three asserted replacements from the parent `c5407a02`:
    1. `import …,sys`;
    2. the hard-coded parent AUTH sha replaced by the `argv` check plus the self-pin;
    3. `step['argv'][1]` replaced by `step['scriptPath']`.
  - Change 3 is needed because the new argv starts with `/usr/bin/env -i …`, so `argv[1]` would no longer be the script path. Without it the parent controller would have hashed `-i`.
- **Elapsed-time sum.** The per-step `elapsedSeconds` add up to about 328.36 s. The remaining ~0.06 s is integrity overhead, consistent with 328.42.

### 2.2 OPX0 / preflight (closes OF-1)

`preflight pre`: `ok true`, `problems []`, and the note `P5 module names in package: 31; filter-selected: ['pvc2/shard4_cost_test','pvc2/shard4_test']`.

P3 calls `file_is` on every manifest row's `original`. That includes the harness original at the doubled `…/candidate/candidate/aiken/lib/pvc2/shard4_harness.ak`. With `ok: true`, that path existed with sha `0df2a7a7`. **OF-1 is closed by actual observation**, matching AUTH `rootConditionsResolved`.

The remaining checks in OPX0 stdout all report `OK`:
- the 10 m-docs, 32 package files, 9 parent-evidence files, 3 stdlib pins and 38 stdlib files;
- the DATA-ONLY binary, tarball and manifest.

### 2.3 Staging, placement, type check, build

| Step | Evidence |
|---|---|
| OPX1t | Binary `4257cc8b` matches; staged mode 600 then 555 (`bin-stat-staged`/`bin-stat` receipts); `/proc/net/dev` shows only `lo:`; `aiken v1.1.24+bacbeb3`; 5 help files sha-equal to attempt-2 help |
| OPX1p | 32 + 32 `OK`; `packages.toml` `42ca2389`; stdlib 38 `OK`; `tree placed: 71 files, 19 dirs, ok=True` |
| OPX2a | `Summary 0 errors, 0 warnings` (8.15 s wall, peak RSS 69 MB) |
| OPX3 | Blueprint `6327327b`, read-only copy in `a2/` |
| OPX3p / OPX2p / OPX7 | Tree check after build, after tests and final: each 74 files, 19 dirs, `ok=True`, `extraDirectories []`. `aiken.lock` `ec5d6a58` equals attempt 2 (OP-P5). All four `tmp-*.txt` listings are empty (sha `e3b0c442…`) |

OPX2a's log contains `Resolving dependencies`. Under the net namespace (only `lo`), with an unchanged lock, a single stdlib entry and no fetch-category tree problem, I read this as offline resolution against the placed packages, not a fetch.

### 2.4 Hash chain and parameter DAG (tool-derived; M-HASH unperformed)

`chain.json` reports `ok true`, `problems []`. The DAG stages appear in the parsed blueprints exactly as declared:

| Stage | What changes |
|---|---|
| U | `vi` has 1 parameter; `vs3` has 2; `s2p` has 6. `vi` hash `032582a0`, `vs3` hash `81b786f2` (still unapplied) |
| B1 | `vi` reaches 0 parameters, hash `fc226622`. Code goes from 25,498 to 25,568 hex chars (12,749 to 12,784 B) |
| B2 | `vs3` drops to `[vi_hash]`, hash `3701d92b` |
| B3 | `vs3` reaches 0 parameters, hash `8759332d`, 21,664 hex chars (10,832 B) |
| B4–B9 | `s2p` parameter count goes 5, 4, 3, 2, 1, 0. Code grows +70/+68 hex chars per step, from 25,622 to 26,036 (13,018 B), ending at hash `e315b315` |

Other checks on the chain:

- **Non-target stability.** Within every stage, the code sha256 of each non-target validator is constant (`compiledCodeSha256` rows).
- **Parameter-file consistency.** The three C2 parameter files share sha `7c1c2d26`. The two VI parameter files share `cbc7adde`. This matches the single-source rule.
- **Distinct identities.** The seven tool hashes are pairwise distinct. None equals any test identifier (`b1/b2/b3/c1/c2/c3/c4/cc/d2 ×28`).
- **Production guard on the genuine chain.** The real deployment tuple `(6a4b…, fc22…, e315…, a063…, 6b56…, 8759…, 0af4…)` would pass `identities_distinct` itself. The production guard therefore does not self-refuse on the actual chain. This is source plus tool-hash reasoning; no compiled run checks it.
- **Determinism.** `receipts/chain.json` here is byte-identical (`c3a3d600…`, 8,427 B) to the first four-piece run's `chain.json`. U is also identical (`6327327b`).

This is strong evidence that the one-constant harness change, in a test-only module no validator imports, left every compiled validator unchanged.

### 2.5 Sizes

Bytes are `utf8Bytes / 2` of the B9 compiledCode, matching `sizes.json`:

| Script | Hex chars | Bytes |
|---|---|---|
| c2 | 5,540 | 2,770 |
| vi | 25,568 | 12,784 |
| vs1 | 13,718 | 6,859 |
| vs2 | 19,880 | 9,940 |
| vs3 | 21,664 | 10,832 |
| vs4 | 21,650 | 10,825 |
| s2p | 26,036 | 13,018 |

Totals:
- Pieces: 6,859 + 9,940 + 10,832 + 10,825 = **38,456**.
- All seven: 38,456 + 2,770 + 12,784 + 13,018 = **67,028**.
- Attempt 2: 2,770 + 12,784 + 20,503 + 12,356 = **48,413**, so the increase is **+18,615**.

`sizes.json` is now `6b29d8e5` (3,205 B), against the old `d9fdfd6f` (3,175 B). The 30 B difference is exactly two blueprint path strings, each 15 characters longer (`harness-repair-`). Every value is equal.

PS4-1/2/3/4/5/6 are TRUE. RANGE-vs1 is FALSE: 6,859 < 7,000. The other five informative ranges are TRUE. The verdict is FIT against the working target only.

### 2.6 Tests against EXPECTATIONS 80cf1601 and the `.ak` declarations

- **Selection.** `--match-tests pvc2/shard4_test --match-tests pvc2/shard4_cost_test` was printed as `Collecting … *pvc2/shard4_test*` and `*pvc2/shard4_cost_test*`. That is module glob selection, and only those two modules match (preflight P5).
- **Report summary.** `total 147, passed 147, failed 0, unit 147, property 0`; per module 22/22 and 125/125.
- **Comparison tool.** `OPX2x`: membership missing/unexpected/duplicates/otherModules all empty, `matchFilterAnomaly false`, `PREDICTIONS_HOLD`.
- **Independent re-count.** I counted the TSV in order against EXPECTATIONS: 17 positive/shared/fin, 44 splits, 6 pieces_refuse, 6 genesis, then the 52 hostile/guard rows. That totals 125, in identical name order.
- **Fail flags against source.** In `shard4_test.ak`, exactly 9 declarations carry `fail`:
  - `q_h14`, `q_h33`, `q_h40`, `q_h41`, `q_h64`, `q_h68` `_pieces_refuse`;
  - `q_s2p_piece_redeemer_wrong_type`;
  - `q_claim_reference_input_vs3_refuses` and `…_vs4_refuses`.

  These are exactly the 9 `succeed_eventually` rows. All 22 cost benches and the other 116 rows are `fail_immediately`.

**What the fail annotation means.** For these 9 tests, status `pass` means the body refused, by halting or returning False. That is the expected refusal outcome, not an ignored failure. It is refusal-only evidence, as EXPECTATIONS states; it does not isolate which conjunct refused.

### 2.7 Positive and genesis paths are now meaningful

- **Positive paths.** The 10 `accepts1` tests and `q_p07_accept` pass. Each conjunct (coverage, `vi_count == n`, VS1..VS4, and `s2p_on` through `shard4.delegated`) therefore held on the honest five-shard transaction, for P01–P06, P14–P17 and both P07 threads.
- **Genesis.** `q_p08` and `q_p09` pass. So `thread.genesis ∧ identities_distinct ∧ credentials_free([b1, c1, d2, c3, c4])` holds on those catalogues.
- **A source-review residual is resolved.** `q_guard_accepts_fixture_catalogue` also passes. This resolves the opaque-fixture residual the prior source review raised: no P08 catalogue native or reserve-owner credential is `Script(d2×28)`.
- **Cost supports the accepting path.**
  - `s4c_p01_s2p` minus `decode` = 26,695,359 − 26,192,938 = **502,421** mem, and 7,022,591,031 − 6,786,228,972 = **236,362,059** cpu.
  - P04 gives 465,021 / 230,378,059.
  - The old 33,496 mem came from the alias-refused path and was never a settlement cost.

**Qualifiers.**
- "Accept" here means every non-signature check: VI uses `authorize_gate`, not `authorize`, and the placeholder signatures would still refuse.
- These are library predicates over contexts decoded by `test_ctx`, not runs of the compiled `withdraw`/`spend` wrappers.

### 2.8 The seven negative aliases each isolate their guard

The control `q_distinct_fixture_accepts` passes. It uses the same `fx.c2_hash`, `h.vi_t` and P01 policy, with `[c1, d2, c3, c4]`. Each alias test differs from that control in exactly one list position, or by dropping one element:

| Test | Difference from control |
|---|---|
| `vs1_vs2` | duplicate `c1` |
| `vs3_vi` | `b1` in the VS3 slot (the old `vs2 = c2` collision no longer co-occurs) |
| `vs4_c2` | single `c2` alias |
| `vs2_policy` | policy in the VS2 slot |
| `three_pieces` | length 3 |
| `alias_s2p` | `delegated` on the honest `p01_t` with `[c1, c1, c3, c4]` |
| `alias_genesis` | `genesis4` on P08 with `[c1, c1, c3, c4]` |

Since the control passes and each test refuses, each refusal is attributable to that single substitution.

For the conjunct inside `delegated`/`genesis`, attribution rests on source short-circuit order (`identities_distinct` precedes `piece_lists`; `thread.genesis` precedes `identities_distinct`), not on traces. Traces are silent. The costs fit early stops without proving them:

| Test | Mem | Comparison |
|---|---|---|
| `alias_s2p` | 26,241,188 | below the accepting 26,695,359 |
| `alias_genesis` | 9,918,908 | below `q_p08` at 10,479,000 |

**Production guard unchanged.** `shard4.ak` is still `7bac10be` in every run-tree freeze. Compiled VS/S2′ code is byte-identical to the prior run.

### 2.9 PC4 recomputation from the TSV

**P01, pieces minus decode:**

| | vs1 | vs2 | vs3 | vs4 | Sum | Parent VS |
|---|---|---|---|---|---|---|
| mem | 2,428,646 | 2,859,115 | 3,385,856 | 2,730,141 | 11,403,758 | 7,265,669 |
| cpu | 778,552,326 | 963,141,418 | 1,364,397,759 | 949,103,690 | 4,055,195,193 | 2,602,535,681 |

- **PC4-1 TRUE:** the four-piece sum exceeds the parent VS on both axes.
- **PC4-2 TRUE:** each piece alone is below the parent VS on both axes.

**P07:** the piece deltas sum to 24,267,833 mem and 8,371,074,775 cpu, against parent 15,268,800 and 5,370,985,007. **PC4-3 TRUE.**

**PC4-4 TRUE** per §2.7.

These piece values are identical to the prior run's, as expected: the piece benches never call `delegated`.

**Limits.**
- Parent `shc_*` values come only through the pinned `OP2-tests.tsv` (`2771106b`), whose body I did not read.
- They are host-CEK costs with an approximate decode subtraction. They are not ledger ExUnits, wrapper costs or a budget fit.
- Per-piece cost is lower than monolithic VS, but aggregate cost is higher.

### 2.10 Labels, freeze, OPX7f

- **Labels.** PS4-7 HOLDS on the applied B9, with the positive control at 47 hits. One context-only word, `state`, appears at the same offset in VI unapplied and applied. It is non-gating and outside the R1 label set.
- **OPX7.** Preflight `post` is `ok`. The T top level is exactly `a2 bin empty-* help journal logs probe-cwd receipts run tmp`.
- **OPX7f.** `OPX7f.out` reads `evaluate 0 freeze 0 chmod 0`, and `OPX7f.exit = 0`. That satisfies outcome.json's own condition: "GREEN … conditional on OPX7f.exit = 0". **The original pending final-hygiene condition is closed.**
- **task-final.json.** 89 files: a2 10 + help 5 + logs 25 + receipts 49, excluding itself.
- **RESULT-MANIFEST row count.** 114 rows = logs 25 + receipts 50 + a2 10 + journal 28 + `OPX7f.exit` 1.

## 3. Resource and counter honesty

| Item | Value |
|---|---|
| Controller wall | 328.42 s |
| Charged | 416 s = 75 fixed (OPX0 70 + OPX1 5) + 341 (13 elapsed lines, OPX1t..OPX7f, including OPX7f 1) |
| Budget journal arithmetic | OPX2 `used 102 wall 1392 kill 60 reserve 246` (1800 − 102 − 246 − 60 = 1392); OPX2p `used 412`; OPX7 `used 414`. All consistent |
| OPX2 | 5:08.73 wall, user+sys 1,103.75 s, 357 % CPU, peak RSS 484 MB, against the 32 GiB address-space limit. The cap was never approached |
| Outcome record | Shows 415 charged before OPX7f; the final total is 416. This is documented, not a discrepancy |
| Lineage | Prior 43 calls / 1,470 s preserved separately. Cumulative 58 / 1,886. Nothing reset |
| This review | Zero calls, zero execution |

The charge is conservative: 416 charged against 328 actual wall. Nothing was under-reported.

## 4. Findings

None is material to the closed predicate, and **no source fix is required**.

| ID | Severity | Finding |
|---|---|---|
| AR-1 | LOW | `journal/` is excluded from `task-final.json` and not made read-only by OPX7f. Its integrity rests only on the root RESULT-MANIFEST, created at 08:41:58, about 3.7 min after the run. This is an accepted root-trust residual; future successors could freeze the journal under a root-side post-receipt. |
| AR-2 | LOW | Stale identifier text persists: `hash_chain4.TEST_IDS` and `notGenuine` (now inside the byte-identical `chain.json`), and EXPECTATIONS `testIdentifiers` (`vs2_t "#c2"`). This is non-gating, and no actual hash equals `d2×28`. Per the authority clauses, it must not be refit. |
| AR-3 | INFO | Historical labels stay historical: outcome lineage "attempt2 … review pending", RAW-OBSERVATION `actualResultIndependentlyReviewed:false`, and archival PRELAUNCH `targetAbsent`. None describes the current state. |
| AR-4 | INFO | OPX2a's `Resolving dependencies` line is benign (see §2.3). |
| AR-5 | INFO | The context-only `state` hit in VI is outside PS4-7. |

The prior source-review findings OF-2/OF-3/OF-4 are carried unchanged. OF-1 is closed (§2.2).

## 5. Read and unread coverage

**Read in full (inline):**
- M2 manifest and every candidate `.ak` body, except fixture literals over 2,000 B (byte count and sha only);
- EXPECTATIONS, DESIGN, requirements and the other M2 documents;
- all 34 O2 files;
- PRE-LAUNCH, EXACT-OPERATOR-CLAUSES, EXACT-HARNESS-DIFF, OPERATOR-COMPOSITION, GEMINI discrepancies, AUTH, VOTE-TALLY, controller diff, runner, receipt, RESULT-MANIFEST and RAW-OBSERVATION;
- all 15 step stdout files;
- every RESULT-MANIFEST row, as text or as a lossless parsed view;
- the parent AUTH/PRE-LAUNCH/runner, the prior run's sizes/outcome/chain, and the prior Opus, Muse and Gemini reports (as evidence).

**Unread or opaque:**
- every compiledCode string (only UTF-8 length and sha; no UPLC decode, no script-hash recomputation, i.e. M-HASH not performed);
- fixture CBOR and datum literals (no curve or CBOR claim);
- the RENDER-PLAN body and COMPOSITION.json;
- parent O1 bodies, the parent `OP2-tests.tsv` body, stdlib and compiler sources, help-file bodies (sha only) and the `bin/aiken` body.

I widen no scope on the basis of anything omitted.

## 6. Scoped adoption recommendation

**Adopt narrowly**, as an independently reviewed raw local measurement covering exactly the §1 predicate:
- M2 compiles;
- the four pieces and S2′ (after the harness fix) satisfy all 147 frozen library-level expectations through `test_ctx`;
- the seven-script tool hash chain is consistent;
- every applied script is ≤ 14,336 B;
- the aggregate is 67,028 B (+18,615 B).

Any publication must carry these qualifiers:
- this is local CEK of library predicates, not compiled-wrapper runs;
- no VI signature verification and no C2 Groth16 pairing was executed;
- hashes are tool-derived;
- the size figure is a working target, not a transaction or ledger fit;
- RANGE-vs1 is FALSE;
- carried conditions FV-01 (S2′ must be bound to the exact parameter DAG) and FV-02 (VS1, VS2 and VS4 are shared across deployments, which is a liveness limit) still apply.

Adoption confers no new operator, proof, native, ledger or Preprod authority. Source approval, this local result, proof, and ledger qualification remain four separate things.

## 7. First unproved boundary

The first unproved boundary is **execution of the compiled, applied B9 scripts** (`vs1`..`vs4` and `vi` via Withdraw, `s2p` via Spend and Mint) on ledger-shaped ScriptContexts carrying the genuine hashes (X1/PX04, M-EU). That has never been done. Independent recomputation of the seven hashes (M-HASH) is still owed alongside it.

After that come:
- FX1 signature positives;
- a concrete Conway settle transaction with its size, fees and ExUnits (PB-02/PB-10);
- registration (M-REG);
- SR-06 and the other open SR and PB items.

**Still open:** all 20 UTxO families; ACTUS 277/18/32/DS01-07; DeFi 72/DA01-24, held-outs, intents and regressions; general obligations; recursion/PCD; private handoff; bounded multi-parent composition; universal certificates; real Preprod. No compatibility percentage is claimed.

current-cardano-four-way-vs-harness-repair-independent-opus-actual-review-20261016-FINAL
