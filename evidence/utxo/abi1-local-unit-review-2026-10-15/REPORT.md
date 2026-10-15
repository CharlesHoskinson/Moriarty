# REPORT: independent actual-result audit of the combined ABI1 R/G run and the C2 Aiken A run

**Bottom line.**
- Both runs ran once, in order. Every authoritative exit file reads `0`: R/G 11/11, A 15/15.
- Every frozen independent expectation was observed exactly: V8 for R/G, V9 for A (I-1…I-9 and the named test sets).
- I re-derived each comparison with my own stdlib parsers from the raw logs and output files. I did not rely on the operators' summaries.
- I found no mismatch and no source defect that affects these results.
- Adoption is limited to **local UNIT evidence** for the exact legacy ABI1 bytes and toolchains. It carries no product, ledger, proof or authority acceptance.

## 0. Identity, independence and method

| Item | Statement |
|---|---|
| Reviewer | Claude Opus 5.5 (`claude-opus-5-5`, as my runtime reports). Requested effort "high" (`PROCESS.json`); I cannot observe it. No subagents. |
| Independence | I authored no candidate, request, expectation, vote, run or earlier review. Every party (V6–V10 and both operators) is the same model family, so independence is at the context level only. I make no model or vendor independence claim. |
| Startup | Read in full: the installed `moriarty-dev/0.1.1` `SKILL.md` and `references/execution-focus.md`; in `utxo-pr54-full-integration-20261015`, `AGENTS.md`, `docs/MORIARTY-PRODUCT-CONTRACT.md` and `docs/audits/2026-10-14/SECURITY-ROUTING.md`. The root refreshed guarded status. SP01.6 history is not authority here. Native/SDK authority stays 0. The public developer path stays permissionless: this maintainer review adds no compile, prove or deploy prerequisite. |
| Executed | Only my own `python3 -I` stdlib scripts in `tools/` (hash, stat, JSON, raw-log parsing, text comparison), plus one-liners that print my own outputs. I imported or executed no candidate, operator or reviewer helper. No cargo, rustc, aiken, VM, SDK, K, Quint, key, proof, ledger, network or Git action. The aiken binaries were hashed and stat'ed as data only. |
| Private material | I did not open, hash or stat any `provider-result.private.json`, `process.stderr`, credential, key, PK, SRS, cache or wallet file. Disclosure: my first action in my own folder was an `ls`, which printed six file names, including those two. Names only; I made no further listings. |
| Harness | Two calls were denied and **not performed**: a pipe into `sort`/`awk`, and a trailing `; echo rc=$?`. I re-ran both in plain `python3 -I` form. No conclusion depends on them. |
| Inventory freeze | Before forming any conclusion I froze `mech/inv-1.json`. All 121 `DISPATCH-FREEZE.json` paths and all 6 full-hash prompt pins **MATCH**. The three unpinned frozen files hash as follows: `EXPECTED-INDEPENDENT-RG.json` `3eb019ea…a7fc7` (= the pin in R/G `CONTROLLER-RECORD`), `EXPECTED-INDEPENDENT-A.json` `95d87e83…a710` (= `ROOT-SCOPE.json`), `VERDICT-AND-VOTES-V9.md` `6cc56cbc…d8d7` (= the A operator preflight). |

## 1. Verdicts (separate axes)

| Axis | Verdict |
|---|---|
| **SOURCE** | **Unchanged and re-verified; no new source verdict.** R/G candidate `d08c200d…8124`: 76/76 rows, 79 files, sidecar = rows, `main.rs` `263865c5…20b1`. Both `#[path]` includes resolve from `source/x7/src` to the frozen `gadgets.rs` `a7cf1c0a…` / `circuit.rs` `624dadc2…`. Lock = frozen + exactly ` "libc",` (line 364; my difflib agrees with `lock.diff`). 69 lock packages: 68 registry + `mrg16`. A candidate `99bf694f…6296`: 76/76 rows, equal path set, exactly one changed row, `tools/aiken_place_v1r1.py` `5a4e1643…b97f`. All 16 Aiken rows and all 8 `source/x7/` rows are identical to R/G. The prior source verdicts (V8 `ACCEPT_SCOPED_FOR_BOUNDED_UNIT`, V9 `ACCEPT_SCOPED_FOR_EXACT_A_ONLY`) stand. I did no new generic compiler or P3 source audit. |
| **RESULT** | **Callable predicate HOLDS for both runs.** The actual compile, unit, ABI, regeneration, Aiken check and build observations match the frozen independent outcomes and the exact reviewed sources and effects. Qualifications: pairing reach is inferred from model cost, not traced (§5.3). Hygiene holds only for the locations checked (RF-6). R/G build-script and proc-macro execution counts were not itemized in the log. |
| **ADOPTION / UNIT-EVIDENCE** | **ADOPT_AS_SCOPED_LOCAL_UNIT_EVIDENCE**, for these exact bytes on this host: R/G with rustc/cargo 1.98.1 and the 135-file closure; A with Aiken `v1.1.24+bacbeb3` (bin `4257cc8b…7677`) and stdlib v2.2.0. Covers the fixed legacy ABI1 relation and the 16 legacy Aiken files only. |
| **PRODUCT ACCEPTANCE** | **None.** See §7. |

## 2. R/G actual results versus frozen V8 (`EXPECTED-INDEPENDENT-RG.json`)

Source of truth: `claim-utxo-abi-v1-combined-offline-check-1/logs/*.{log,exit}`, `receipts/`, `out/`. Parsed by `tools/rg_check.py` → `mech/rg-check.json`.

| Step | Frozen | Observed (raw) | Match |
|---|---|---|---|
| R1 | exit 0 | `0\n` (no wrapper, so no log, as the request specifies) | yes |
| R0 | 3 self-digests, 76 rows, 79 files, 8 frozen pins, lock diff, 4 reviewer/closure pins | 91 `OK` lines = 3 + 76 + 8 + 2 + 2. `cand-files-before` = manifest paths + 3, C-sorted. `lock.diff` = `363a364\n>  "libc",\n` | yes |
| G1 | `a1c132f8…bbf7`, cmp equal, 10 tests | Regenerated bytes **identical** to candidate `v0_rewrap_test.ak`; generator printed `"tests": 10`; my census counts 10. V0OUT L1 `2108993b…`, M1 `4b9c407c…` | yes |
| R2 | 10 digests, rustc/cargo 1.98.1, umask 0022 | All 10 digests equal in order; `rustc 1.98.1 (48a229cea 2026-09-01)`, `cargo 1.98.1 (797e8a9bc 2026-08-05)`, LLVM 22.1.8; `0022`; controller drift `[]` | yes |
| R2c | 135 OK src/dest; 68 crates; `problems []` | 135/135 and 135/135 `OK`, 68 `.crate` lines; `packages 69 registry 68 local ['mrg16'] missing-checksum []`; `checked 68 crates; problems []` | yes |
| R3 | exit 0; 62 deps + mrg16; libc; 6 proc-macros; serde/serde_derive/zerocopy-derive not built | 63 `Compiling` lines (62 deps, all in the lock, then `mrg16`); libc 0.2.190 present; all 6 proc-macros present; 0 error lines; one warning (`Liability` never constructed, RF-2); x7 `c9efbe85…8239` | yes |
| R4 | 46 = abi 22 / cli 6 / fsx 8 / pkio 10; names = `TEST-NAMES-R1.json` `c64b38b6…` | `running 46 tests`, all `ok`; `46 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`; per module 22/6/8/10; name set equal (0 missing, 0 unexpected) | yes |
| R5 | 25 lines AS-EXPECTED; last line exact; L1 264/412635, M1 761/307706 | 25 `[AS-EXPECTED]` lines; log ids = `results.json` ids in order; last line byte-equal to the frozen text; `{"Ok": 307706}` with an **integer** value; statement lengths recomputed from hex: 264 and 761 | yes |
| R6 | `mismatches []` | `{"compare": …, "mismatches": []}`; author fixtures and pay hex equal the observed values | yes |
| R6r | `{"mismatches": []}`, comparator `21cbc8c5…` unchanged | Last line `{"mismatches": []}`; comparator hash re-verified. Its printed L1/M1 `predicateDigest` and `statementHex` equal the observed values. The 12 frozen first-unsatisfied indices (#412633 ×3, #307704 ×2, #1781, #1652, #753, #2039, #882, #5144, #260184) appear verbatim in R5 | yes |
| R7 | 76 rows, file set unchanged, no in-tree target, empty HOME/tmp-test | 76 `OK`; before = after (`50e9fec5…`); residue file empty; POSTCHECK post-state agrees | yes |

- **Ordering.** Every `logs/*.exit` mtime falls inside its `CONTROLLER-RECORD` window, and every start follows the previous end (`mech/timing-lock.json`).
- **Controller.** `tools/controller.py` (`32a59b3a…`, read only) takes the request bodies byte for byte (request sha checked) and runs the recorded argv with the recorded env, cwd `/` and stdin `/dev/null`. It writes `.exit` from the real wait status and stops at the first non-zero exit. The tee path is the literal TASK path rather than `"$1/$2.log"`, which is equivalent.
- **Votes.** The literal second votes (`PRE-EXECUTION-REVIEW-VOTES.json` `65ffea7b…`, V9 R/G `REPORT.md` `746fa241…`) give Q-3…Q-8 and placement 2 votes each. The earlier pre-R1 permission stop is preserved as written.

## 3. A actual results versus frozen V9 (`EXPECTED-INDEPENDENT-A.json`)

Source of truth: `claim-utxo-abi-v1-c2-aiken-check-1/logs/*.exit` and `A12.exit` (authoritative; the scripts' own trailing exit is `times`, not step status), plus `logs/*.log`, `receipts/` and `run/plutus.json`. Parsed by `tools/a_check.py` → `mech/a-check.json` and `tools/cmp_operator.py` → `mech/cmp-operator.json`.

**Step copies.**
- All 15 `steps/*.sh` hash to `STEP-COPY.json`, and their V9 sources hash to `sourceSha256`.
- Each differs from its source at **line 14 only**: `J=<review>/journal/X.txt` → `J=<r2>/journal/X.txt`.
- Lines 5–13 equal proposal lines 10–18 verbatim. Each reviewed step line and its comment occur verbatim in the proposal `ed21dd72…`, and the line hash matches `reviewedLineSha256`.
- All 15 proposal step lines are covered, and `commands[]` equals the expected literal `env -i …` strings.
- Journals and exit-file mtimes show a strict sequence, with each exit written inside its window.

| Step | Frozen | Observed (raw) | Match |
|---|---|---|---|
| A1 | exit 0 | `0\n`; no log (no wrapper) | yes |
| A0 | exit 0; pins | 78 candidate `OK` lines (2 self + 76 rows), 3 stdlib pin, 2 DATA-ONLY, 1 published-checksum line | yes |
| A2 | sha `4257cc8b…`; `600 38299808 regular file` | exact | yes |
| A2m | 600 → 555 on the copy | `555 38299808 regular file 5509762` | yes |
| A3 | only `lo:` | Only `lo:` with zero counters | yes |
| A4 | contains v1.1.24 | `aiken v1.1.24+bacbeb3` | yes |
| A4p | allHold; 0555; 5 empty | exact | yes |
| A5 | 16 / 38 | `projectFiles 16, stdlibFiles 38`. The 16 placed files equal the candidate rows; the 38 stdlib files equal the pin manifest; the only other placed file is `packages.toml` | yes |
| A6 | 55 files | 55 | yes |
| A7 | 150 / 150 pass, per-module counts | JSON report: total 150, passed 150, failed 0, unit 150, property 0, seed 2795378180; trailer `Summary 150 checks, 0 errors, 0 warnings` | yes |
| A8 | allHold; single-package lock, no etag; packages only stdlib + toml | exact. My read of `run/aiken.lock` (`ec5d6a58…`): one `aiken-lang/stdlib v2.2.0`, empty `[etags]`. `unexpectedTopLevel []` | yes |
| A9 | titles only c0/c1/settle0/settle1; one hash per module; 4 distinct | 12 entries (`mint`/`spend`/`else` ×4), one hash **and** one compiled-code body per module, 4 distinct hashes; receipt = blueprint; preamble compiler `v1.1.24+bacbeb3`, plutusVersion v3; blueprint `48620ab8…f779` | yes |
| A10 | allHold; plutusJsonPresent | exact; run tree +3 files (`aiken.lock`, `build/aiken-compile.lock`, `plutus.json`), none removed or changed | yes |
| A11 | candidate unchanged; DATA-ONLY 0644 unchanged; pin docs unchanged | 76 `OK`, `cmp` equal, DATA-ONLY binary and manifest `OK`. My own stat now: DATA-ONLY 0644 `4257cc8b…`; staged copy 0555 `4257cc8b…` | yes |
| A12 | exit 0 | `A12.exit` `0\n`; all 39 `task-final.json` entries re-hash | yes |

**I-1 and the named sets.** These are my parse of the A7 JSON plus my own regex census of the 16 Aiken files.
- 150 unique names; census = observed; 0 missing, 0 unexpected, 0 duplicates. Per module 36/30/24/48/10/1/1.
- Exactly 26 tests have `on_failure: succeed_eventually`. They equal both the frozen `failTests` and my census of `fail`-marked tests.
- All 8 positive decoder/handler probes pass. All 6 negative decoder `fail` tests pass.
- The operator's 150-row table equals my parse field for field: status, on_failure, cpu, mem, module.

## 4. Pairing, cost and decoder observations (raw numbers, not just pass)

| Set | Test | CPU (model) | Status |
|---|---|---|---|
| pairing-true | `x2_regression_verifies` | 2,142,406,732 | pass |
| | `fi_v1_verify_x2_regression` | 2,156,446,038 | pass |
| | `l1_v0_proof_verifies_under_v0` | 2,152,057,818 | pass |
| | `m1_v0_proof_verifies_under_v0` | 2,175,161,256 | pass |
| | `m1_c0_issue_rewrapped_refund_key_accepted` | 2,348,645,595 | pass |
| pairing-false | `x2_regression_statement_bit_flip_rejected` | 2,142,562,781 | pass |
| (I-2 subset ★) | ★`fi_v1_verify_x2_statement_flip_rejected` | 2,156,602,087 | pass |
| | ★`l1_v0_proof_rejected_under_v1_statement` | 2,171,300,643 | pass |
| | ★`m1_v0_proof_rejected_under_v1_statement` | 2,194,716,844 | pass |
| | ★`l1_c1_issue_with_v0_proof_refused` | 2,298,432,008 | pass |
| | ★`m1_c1_issue_with_v0_proof_refused` | 2,363,638,491 | pass |
| no-pairing | `fi_v1_verify_gamma_identity_false` | 278,071,919 | pass |
| | `c1_issue_identity_gamma_vk_refused` | 637,045,938 | pass |

- **I-2.** All five lie in the request's 2.1–2.4·10⁹ band.
- **I-3.** Relation: fi/x2 = 1.00655 (+14,039,306), so it holds. The inherited constant 2,142,407,438 is **706 units higher** than observed (RF-5). The no-pairing pair is below 10⁹, as frozen. The value 6.4·10⁸ is "below", not "≪". It is still the cheapest of the five C1 issue refusals (next lowest 704,669,732) and about 1.7·10⁹ below the refused C1 issues that reach the pairing.
- **I-8.** `decoder_rejects_extra_transition_field_after_option_data` passes (113,672,774). Its failure trace is `expect env: EnvV0 = d`, the same locus as the plain extra-field test, so it is still **non-evidence**. The `probe_*_order_*` tests are the discriminating checks, and all pass.

### 4.1 Failure loci of all 26 `fail` tests (from the A7 JSON `traces`)

Each fails at its intended check, not incidentally:
- The four v0/v1 decoder tests and `m1_v0_envelope_not_decodable_as_v1` fail at `expect env: EnvV0/EnvV1 = …`.
- `v1_statement_rejects_short_claimant` fails at `expect abi.len(env.claimant_vkh) == 28`.
- `u16_overflow_fails` fails at the range expectation.
- The two x2 tamper tests fail at the IC-count and proof-length guards.
- **Both junk-VK tests fail at `blst error BLST_BAD_ENCODING`**, which is point decoding.
- `s1_v0_claim_at_claim_hash_fails_closed` and `c1_issue_v0_envelope_fails_decoding` fail at `expect env: EnvV1 = raw`.
- Five settle tests fail at `abi.match_slots`.
- The settle token, datum and two-thread tests fail at their own guards.
- The genesis tests fail at the test's own `expect settle.genesis(...)`, so they are boolean refusals rather than crashes.
- The issue tests fail at the single-claim-output and holder guards.
- `settle_universal_claim_refused` fails at `expect env.claim_kind == 0`.

## 5. Security judgments

### 5.1 The result is genuine for its exact scope

These points are observed facts, not inference:
- The run used the reviewed bytes: candidate rows before and after, step-copy diffs, request pins.
- It ran in the reviewed order once: journals, mtimes, single TASK/ATASK.
- It used the pinned compilers: R2 digests; aiken sha and mode at five gates and again by me.
- It produced the frozen outcomes.

I found no trace of a retry, refit, edited comparator or substituted tool.

### 5.2 The ≥10⁹ CPU proxy is coarse (operator observation 2 confirmed)

Eleven tests that run no proof verification still cost at least 10⁹:
- the issue paths, 1.10–1.22·10⁹;
- the two-slot settles, about 2.04·10⁹, which run no verify at all.

So the threshold alone does not prove pairing reach.

### 5.3 Pairing reach: stronger differential support, still inference

These are my comparisons of raw model costs:
- **Flip twins.** Each flip test costs exactly +156,049 over its passing twin (`x2_*` and `fi_*`). The rejecting run therefore did the same work as the accepting run, plus a constant flip-construction cost. An early `False` before the Miller loop would sit roughly 10⁹ lower.
- **v0 proof under the v1 statement.** l1: +19,242,825; m1: +19,555,588 over the v0-accept twins. That is consistent with hashing a 28-byte-longer statement and then a full verify.
- **C1 issue with a v0 proof.** `claim1.issue` (`claim1.ak:37-46`) is a short-circuit `and` that runs `groth16_verify` **last**. The test builds a well-formed v1 envelope whose `vk_digest` equals the v0 VK, so every cheaper conjunct holds.
  - Observed: l1_c1 − l1_rej = 127,131,365; m1_c1 − m1_rej = 168,921,647.
  - An accepted C0 issue, which completes the pairing, has overhead m1_c0 − m1_v0 = 173,484,339.
  - All C1 issue refusals that do not reach the pairing cost ≤ 1.2·10⁹.

Conclusion: the costs are consistent with all five I-2 tests completing the pairing and returning `False` at the final equality. This is **model-budget inference, not a trace**. No ExUnits, fee or transaction validity follows. Test budgets measure library calls on placeholder transactions, not the A9 validators under a ledger ScriptContext.

### 5.4 Known hazards confirmed live in the compiled legacy artifacts

- `m1_c0_issue_rewrapped_refund_key_accepted` **passes**. The v0 R-1 hazard is present in the compiled C0 logic: a published v0 proof issues a second claim for another refund key. The blueprint includes `c0` `e3007711…` and `settle0` `d03b5ff5…`. **These must never be funded or adopted** (RP-2).
- The old public fixtures (x2, X7 v0 L1/M1) are vanilla Groth16 **v0** data. They were used only as UNIT inputs. **No ABI1 proof exists**, so the C1 **issue-acceptance** path has never been exercised. C1 evidence covers refusals plus release/settle on constructed claims.
- **SD-1 remains open.** Claimant binding stops copy-use by another party (`c1_copy_with_victim_claimant_needs_victim_signature` passes). It is not thread or debit authority. Anyone holding a publishable PK and the public M1 witness can re-prove under their own claimant and advance a VK-pinned thread. I do **not** call public-PK thread advances safe. There is no signed intent and no all-account authority.

## 6. Findings (none blocks adoption as scoped UNIT evidence)

| Id | Sev. | Finding |
|---|---|---|
| RF-1 | Info | `x7v1_expect.py:124` truncates its printed expectations at 4,000 characters. `out/x7v1_expect.json` is therefore a cut JSON body (ending `"fir`) followed by `{"mismatches": []}`. The comparison uses the full dict and is unaffected. Proposal RP-4. |
| RF-2 | Info | R3/R4 warning: `Effect::Liability` (`abi.rs:478`) is never constructed, in either the binary or the test build. The liability effect encoding arm (`abi.rs:579`) is unexercised. `liability_presence_encoding` covers snapshot presence only. Liability execution stays open. |
| RF-3 | Info | V8's R3 `notBuilt` list (3 crates) is not exhaustive. Six locked registry packages were not compiled: rustc_version, semver, serde, serde_derive, syn 3.0.9, zerocopy-derive. The frozen count, 62 compiled, matches exactly. |
| RF-4 | Low (interpretive) | The ≥10⁹ pairing proxy is non-discriminating (§5.2). Differential support is in §5.3. Proposal RP-1. |
| RF-5 | Info | The I-3 inherited constant is not reproduced: the observed value is 706 units lower. Model budgets depend on compiler and build; this reinforces the rule against ExUnits inference. |
| RF-6 | Low (premise) | A sets no `TMPDIR` and post-checks do not inspect `/tmp` or other host paths; namespaces do not restrict filesystem writes. "No writes outside ATASK" is established only for the checked locations. R/G set `TMPDIR`/`CARGO_HOME`/`CARGO_TARGET_DIR` inside TASK, and R7 checked the residue. Proposal RP-3. No stray write was observed. |
| RF-7 | Info | No ABI1 proof exists, so the C1 issue-acceptance path is unobserved (§5.4). |
| RF-8 | Security (known, confirmed) | The v0 R-1 hazard is live in the compiled C0; the C0/settle0 blueprint entries must never be funded (RP-2). |
| RF-9 | Info | The blueprint validators carry no parameters. Cross-script identity (C1 policy ↔ settle1 hash) is carried in datum content. Deployment identity binding is a U/P3 matter and was not evaluated here. |

The earlier frozen and historical records stay as written:
- the origin R3 2×E0277 failure and the consumed scratch `claim-utxo-abi-v1-offline-check-1` (not touched);
- the R/G pre-R1 permission stop;
- the A first-invocation denial with no attempt.

## 7. Product acceptance and what this does not establish

This establishes, for the exact bytes, local UNIT behaviour only:
- R/G compile under the frozen lock + libc;
- 46 Rust unit tests on public nonsense bytes;
- unchanged `x7 abi` encodings, constraint counts and refusal indices;
- byte-identical v0-rewrap regeneration;
- 150 Aiken model-level unit tests;
- one blueprint.

It does **not** establish any of the following:
- the current generic Rust Core / `moriarty/1` compiler, or ABI2;
- setup, keys, a real PK lifecycle, any ABI1 proof, a VK, or PCD;
- the compiled ScriptContext wrapper (U), ExUnits, fees, ledger validity or Preprod;
- thread or debit authority (SD-1 / TA-1-a), signed intent, all-account authority, or P3 physical settlement;
- genesis, private or multi-parent composition, or all-20 / general coverage;
- any U0–U7, M1–M17 or MC/SP/G exit.

Source or result acceptance here cannot authorize key generation, proving or ledger actions. The old persistence, no-PK and research boundaries remain.

## 8. First unproven boundary toward a genuine current-language Preprod prototype

**Current-language compilation into a proof relation (P4).** No program in the current language (`moriarty/1`, Rust Core) has been compiled, by the current generic compiler, into this or any claim relation or ABI. The only relation is M1/L1, hand-lowered from the frozen legacy oracle.

Everything downstream stays unproven until P4 holds:
1. keys and a real ABI1/ABI2 proof (native authority 0);
2. the C1 issue-acceptance path;
3. the ScriptContext wrapper (U);
4. the SD-1 / TA-1-a authority design and signed intent;
5. P3 physical settlement;
6. any Preprod transaction.

On the legacy-only track, the next missing observation is a real ABI1 proof, which is blocked by native authority 0.

## 9. Deliverables (this folder only)

- `REPORT.md` (this file).
- `REVIEW-MANIFEST.json`.
- `COVERAGE.json`: full, partial and unread coverage, plus the harness notes.
- `proposals/PROPOSALS.md`: RP-1…RP-4, unreviewed, no votes.
- `tools/`: `rinv.py`, `freeze_allow.py`, `rg_check.py`, `a_check.py`, `cmp_operator.py`, `timing_lock.py`, `bin_stdlib.py`, `make_review_manifest.py`.
- `mech/`: `prompt-pins.txt`, `allow-1.txt`, `inv-1.json` (the frozen inventory), `rg-check.json`, `a-check.json`, `cmp-operator.json`, `timing-lock.json`, `bin-stdlib.json`.

I did not mutate any candidate, expectation, raw record, operator record or counter. I requested no retry and no authority. No new consequential change is proposed for decision here, so no vote is cast.
