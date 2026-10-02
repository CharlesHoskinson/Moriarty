# NATIVE-FINANCIAL-V10-R5 verify-only packet: independent Opus audit

## Identity and scope

- Role: independent auditor. I am not the source author. The author (Grok 4.6 high) has no vote.
- Requested model: `claude-opus-5-5`, effort `high`. The model serving this session identifies as Claude Opus 5.5, model ID `claude-opus-5-5`. Root must record the runtime-returned identity from the terminal receipt. This report's SHA is not a model identity.
- Routing: `AUDIT-ROUTING-20261001.json` (a415f7d6…f9b4), which selects gpt-6.1-sol high, claude-opus-5-5 high and grok-4.7 xhigh. No substitution was made.
- Prompt: `NATIVE-FINANCIAL-V10-R5-VERIFY-OPUS-PROMPT.md` (868b635c…f49b).
- Date: 2026-10-01. Evidence root B = `/home/charl/research/moriarty-signed-intent-2026-10-01`.
- Subject: frozen source `NATIVE-FINANCIAL-V10-R5-VERIFY-SOURCE-FREEZE.json`, SHA-256 `ca65299f7f728ff974df6e37113ce8a74bac70d167ea4d502753fed1f47b9dda`, with 8 pinned paths.

### Boundaries kept

This audit was read-only. I did not:

- run any native process, or import or run the candidate or helper;
- run Cargo, build, fetch, keygen, prove or verify;
- mutate systemd units, git, caches, sources, artifacts or receipts;
- acquire anything over the network or use subagents.

Python files were checked with `ast.parse` only.

I did not read any counterpart R5 report and did not contact other reviewers. Only historical R4 material was read, including my own R4 Opus review and CPU clarification, to describe the preserved dissent accurately.

The only file written is this report, created at an exclusive new path.

### Startup

- I loaded the installed develop skill (`…/moriarty-dev/0.1.0+codex.20260912173932/skills/develop/SKILL.md`) and read `AGENTS.md` in worktree `moriarty-beta-20260930`.
- Worktree HEAD is `f9e73c7a0e56ea74db727c527fbc1336919e9468`, and the tree is clean.
- Read-only guarded `status --json` reports:
  - capability SP01.6 loan-swap-subset;
  - next action `sp01-loan-report`;
  - historical accounting blocked: `current-accounting-missing`, stale SP01 binding/candidate inputs, and `resource-live-state-unavailable`;
  - `pendingTransactions: []`.

This is not dispatch authority for this offline fixture experiment, and it does not change it.

## Evidence read in full

**R5 packet:**
- wrapper `run-native-financial-v10-r5-verify-bounded.py` (334 lines);
- helper `native-financial-v10-r5-verify-controls.py` (60 lines);
- `NATIVE-FINANCIAL-V10-R5-HISTORY-CASES.json`;
- `NATIVE-FINANCIAL-V10-R5-VERIFY-RESOURCE-HANDOFF.md`;
- `NATIVE-FINANCIAL-V10-R5-VERIFY-INDEPENDENT-EXPECTATIONS.md`;
- the input-freeze structure and all 1130 entries (hashed);
- the routing record and the Opus 5.5 availability canary. The canary is availability evidence only, not a vote.

**Native source:** all twelve entries of `native-ledger-reward-funded-v10-r2-candidate/SOURCE-HASHES.json`, including the six Rust modules in full:
- `artifacts.rs` (26 lines), `funding.rs` (135), `verify.rs` (66), `main.rs` (309), `provider.rs` (70), `keygen.rs` (77);
- `Cargo.toml` and the `midnight-serialize` entry of `Cargo.lock`, pinned to midnight-ledger rev `9a8777c4d035fc7f38ae286bcf5f8656668efd9f`;
- `README.md`.

The production Beta/Core handoff is the pinned v5 keyless handoff and runtime in the prove config. It was checked through the frozen identity closure, not re-derived.

**Official upstream source (checkout HEAD `9a8777c4…efd9f`):**
- `serialize/src/deserializable.rs`, lines 1-140;
- `serialize/src/tagged.rs` (`Tagged for Vec<T>` gives `vec(T)`);
- `ledger/src/structure.rs` tags: `transaction[v12]`, `signature[v2]`, `proof`, `ledger-state[v18]`;
- `transient-crypto/src/commitment.rs` tag `pedersen-schnorr[v1]`.

**R4 parents:**

*Prove:*
- receipt (all 56 disk samples), attempt, config, log and host log;
- all 22 proof outputs, including `receipt.json`, `application-result.txt`, `replay-refusal.txt`, `construction-diagnostics.json`, `preparation.json`, `envelope-preflight.json`, `preliminary-fees.json`, `funding-history.json`, `partition-costs-effects.txt`, and the raw headers of `transaction.tagged`, `funding-history.tagged` and `funding-claim.tagged`.

*Verify:*
- receipt (all 6 samples), attempt, config, the full 1061-line log, the good-first log and the host log;
- all 13 partial outputs, including the alternate-funding outputs and the history-suffix config and log.

*Root records:*
- R4 prove and verify authorizations, live admissions and service launch/terminal records;
- R4 root resource decisions (prove and verify), the R4 root review disposition and the R4 verify root cause (7da98368…f244);
- the independent launch smoke record, both cleanup records and the trash-cleanup plan.

**Older consumed or failed parents:**
- disk-stop freeze (35);
- cache cleanup;
- R3 interrupted freeze (35);
- V10-R2 actual freeze (67) and its build, funding-preflight and preflight receipts and output sets;
- v7 freeze (44), v7 stage receipts, the SRS acquisition record and keygen success;
- v5 freeze (34);
- inherited consumer manifest (39 + 260);
- runtime manifest (240 + 240 plus symlinks);
- v6 retention receipt and archive.

## Independent hash and structure checks (all streamed, all pass)

### R5 source and freezes

| Check | Result |
|---|---|
| Source freeze | `ca65299f…9dda`; all 8 entries match: wrapper `367606608b…02c6`, helper `cd21da34…a086`, input `8e988c0a…0a20c`, cases `b88c0529…cc1f`, handoff `453d74c6…87fbe`, expectations `5ad4977f…bfc2`, routing `a415f7d6…f9b4`, canary `1160eb48…cfb` |
| Candidate `SOURCE-HASHES.json` | `f7e7eaf4…a9e7`; all 12 entries match, including six `.rs` files, Cargo.toml/lock and patches |
| Input freeze | 1130 entries, all absolute paths, all match (3,299,423,613 bytes streamed) |
| R4 input freeze `5e8e11b4…c641` vs R5 input | 1039 R4 tuples retained; 0 changed, 0 removed |
| `binary`, `prove_config`, `v7_result_freeze` | Unchanged from R4 |
| Additions | Exactly 91 = all 88 R4-actual paths + `NATIVE-FINANCIAL-V10-R4-ACTUAL-RESULT-FREEZE.json` + `NATIVE-FINANCIAL-V10-R5-HISTORY-CASES.json` + `NATIVE-FINANCIAL-V10-R5-VERIFY-INDEPENDENT-EXPECTATIONS.md` |
| Self-hashes and future paths | R5 input freeze, source freeze, wrapper, helper and handoff are absent from the input freeze. All R5 future paths are absent from disk |
| ELF | `ae7e8dab…8239`, 698,019,744 bytes |
| Retained prior financial ELF | 696,902,872 bytes, match |
| Diagnostic ELF | 277,301,184 bytes, match |
| v7 retained ELF and v6 archive | Match |
| Keys | `pk`/`vk`/`ir` match keygen-success |
| SRS | `4a9ef6c7…3b74`, 25,166,212 bytes |
| Runtime | 240 + 240 files; exact file sets and symlinks match |

The `prove_config` fixture is the frozen trusted public development fixture with `night_value` 1e12 and `fee_allowance` 1e20.

### R4 actual-result freeze

- `NATIVE-FINANCIAL-V10-R4-ACTUAL-RESULT-FREEZE.json` is `ad34703a…c29f`.
- It has 88 absolute paths, all match, and `produced_binary` equals the input binary.

### R4 proof gate

- `NATIVE-FINANCIAL-V10-R4-PROOF-RESULT-FREEZE.json` is `e02f9372…cefa0`.
- Its keys are exactly `{sha256}`. It has 67 paths, all match.

### Overlap between the two freezes

I checked the two freezes independently:
- 61 paths are common, and all agree.
- 6 paths appear only in the 67-path gate: both trash-cleanup files, the launch smoke, the disk-stop freeze, the R3 interrupted freeze and the V10-R2 actual freeze. Each is also pinned and checked elsewhere.
- 27 paths appear only in the 88-path freeze: the R4 verify records, the proof gate itself and the official decoder.

### Older consumed or failed parents

All match:
- disk-stop (35);
- R3 interrupted (35);
- V10-R2 actual (67), plus three receipt/output-set closures;
- v7 (44);
- v5 (34);
- inherited consumer (39 + 260);
- Cargo.toml/lock equal to the inherited consumer.

Cleanup `50edf5ac…15db` has its protected hashes matched, all commands exited 0 and no new native allocation. The trash-build cleanup record is `5a324f21…d8c`:
- it covers only `.lake/build` directories in two trashed repositories, deleted 2026-08-14;
- the observed free increase was 14,917,431,296 B, qualified for concurrency and hardlinks.

### Consumed reservations

All are preserved and none is rebound:

| Attempt | Exit | Stop / error | Notes |
|---|---|---|---|
| V10 prove | -9 | disk stop | No `proof.raw` |
| R3 | -9 | `KeyboardInterrupt` signal 15 | No `proof.raw` |
| R4 verify | 1 | Attempt file and outputs intact | |

### Target and cache

Target apparent size is 9,342,748,478 B and cache is 2,933,611,498 B, both identical to the R4 receipts.

### Receipt hashes

- R4 prove receipt `d70ae58b…3d8d3396` and R4 verify receipt `02eb0725…c777` match the root terminal records.
- Host logs `a1ad5a1f…` and `b57b9b1b…` match.
- Authorizations `92156900…` (prove) and `65875ef4…` (verify) match the receipts.

## A. SOURCE: **APPROVE_SCOPED**

### Minimum-change verification

**History cases.** The raw diff from the immutable R4 spec (`103fa0e3…fa47`) to the R5 spec changes exactly two lines, the `expected_error` strings of `claim-suffix` and `history-suffix`.
- Every case path, sha, byte count, field and classification is identical.
- The base pins, alternate deltas and qualification are also identical.
- The case artifacts are unchanged (`a0f1dd4e`, `b06b4a5b`, `33eb5125`, `0daaa62d`).
- I confirmed by byte comparison:
  - the suffix cases equal the base plus one `0x00` (history 20933 to 20934; claim 221 to 222);
  - the truncations are exact prefixes of the base (10466 and 110 bytes).

**Helper.** The diff from the R4 helper (`4dc92714…aca`) is exactly:
- the spec path changed to `NATIVE-FINANCIAL-V10-R5-HISTORY-CASES.json`;
- its pin changed to `b88c0529…`;
- the first-good log changed to the virgin `native-financial-v10-r5-verify-good-first.log`.

Nothing else changed: the commands, per-case assertions, case order, alternate producer, good-again, receipt equality, post-rehash and completion marker are all the same.

**Wrapper.** The diff from the R4 wrapper (`839f6fa0…2d93`) is scoped. It makes these changes:
- **Phase:** phase must be `verify`, so prove is unreachable.
- **Pins:** `R4_INPUT_PIN` is kept for the parent receipt and `V8_INPUT_PIN` becomes the new 8e98 map. A new `R4_ACTUAL_PIN` is added, and the closed authorization adds `r4_actual_result_freeze_sha256`, still requiring three exact provider labels per family.
- **New `verify()` checks**, run before launch and again after:
  - the actual-88 and proof-67 freezes;
  - the R4 prove receipt (exit 0, no stop or error, identities true, postconditions true, input pin 5e8, exactly 22 outputs, exact output set and digests);
  - the R4 verify receipt (exit 1, no stop or error, identities true, postconditions false, exactly 13 outputs, exact set and digests);
  - absence of `history-control-results.json` and `good-again.log` in R4;
  - R4 independent receipt byte-equal to the R4 proof receipt.
- **Proof gate:** the proof gate pin is fixed to `e02f…`. `proof_dir` stays `native-financial-v10-r4-proof`.
- **R5 paths:** all R5 paths (receipt, attempt, config, log, verification directory) are new. Pre-launch exclusivity is asserted on receipt, reservation, config and output directory.
- **Unchanged:** limits are 120 / 240 / 2 GiB, plus disk, output and Rayon limits. Native acceptance postconditions are unchanged, now applied to the verify path only.

I rebuilt the closed verify config bytes independently, without importing the wrapper. They are byte-identical to the R4 verify config (`e1959fa2…9fa8`), so the native input to good-first and good-again is exactly the R4 input.

### Root-cause correctness, from primary source

- `artifacts.rs:21-24` `decode` calls `tagged_deserialize(&mut cursor)?` before its local cursor-position guard.
- Official `deserializable.rs` (`d8c026b0…7e85`):
  - lines 29-30: `tagged_deserialize` calls `tagged_deserialize_inner(reader, true)`;
  - lines 85-132: it checks the header against `tag_expected = "midnight:" + T::tag() + ":"`, deserializes, counts the remaining bytes, and returns `InvalidData` "Not all bytes read deserializing '{tag_expected}'; {count} bytes remaining".
- The `?` operator propagates that error, so the local "outer tagged decoder trailing bytes" guard is unreachable for trailing data. The R4 spec literal could never match.

**History string:** observed in actual R4 `fault-controls/history-suffix.log` as `Error: Custom { kind: InvalidData, error: "Not all bytes read deserializing 'midnight:vec(ledger-state[v18]):'; 1 bytes remaining" }`. It agrees with:
- the raw header of the base history;
- `Tagged for Vec<T>`, which gives `vec(...)`;
- `#[tag = "ledger-state[v18]"]`.

The R5 string is an exact substring of the observed line, and Debug formatting escapes none of its characters.

**Claim string:** source-derived. The raw claim header is `midnight:transaction[v12](signature[v2],proof,pedersen-schnorr[v1]):`. `SignedTransaction = Transaction<Signature, ProofMarker, PureGeneratorPedersen, DB>`, and the official tags are `transaction[v12]`, `signature[v2]`, `proof` and `pedersen-schnorr[v1]`. The final `transaction.tagged` has the identical header.

The pristine claim decodes under `ensure_consumed` in the successful R4 good-first run. Therefore the header check passes for the suffix variant and exactly one byte remains. The 1-byte count follows from the deterministic, self-delimiting deserialization of the same prefix.

This is a specific typed EOF and count, not generic acceptance. **It has not been executed** for the claim case. If the prediction is wrong, the helper fails honestly with exit 1 and `native_postconditions_passed=false`. It cannot falsely accept.

### Refusal path and order

- `verify.rs` runs VK, proof, statement, SRS, tx and genesis decoding and the invariant first.
- Then `funding::verify_history` decodes history, then claim, before the canonicality and reconstruction checks (`funding.rs:129-134`).
- `fs::create_dir(out)` comes only afterwards (`verify.rs` line 60 of 66).

So every decoder or reconstruction fault exits with code 1 and creates no output directory. The helper enforces all of the following:
- exactly one changed config field, with matching path, SHA and size;
- `code==1`;
- no output directory;
- neither "host artifact identity mismatch" nor "invalid supervisor SHA256" in the output.

Truncations keep `expected_error=null` and the generic `Error:` gate, unchanged. Both canonical alternates require the unchanged exact message "supervisor-pinned funding predecessor history differs from independent native reconstruction" (`funding.rs:133`).

The alternate producer is the unchanged public keyless-prepare fixture with `night_value+1` and `night_creation_seconds+1`. Because it runs `genesis(…, None)`, the operation is also registered keyless. The cases spec carries the qualification that this is not an isolated-field test and not authenticated PCD or history evidence.

### Completion requirements, unchanged

The wrapper requires all of:
- exit 0 and no stop;
- an independent receipt equal to the R4 `receipt.json`;
- `Success`, `proof_count` 1, `default Real`, 293 ops and 47 reads;
- `0 < fee <= allowance <= available` and `fee + remainder = available`;
- `prestate != poststate`;
- `history-control-results.json` with `all_six_refusals`, `pristine_good_again` and exactly the 8 named records;
- the five log markers.

The helper adds a good-again receipt byte-equal to first-good, plus a post-suite rehash of every config input. The full proof, PI (1035 indices), address, entry, communication, gas, effects, proof corruption/truncation/suffix, four `STRICT_DECODER` suffix and absent-owner-signature refusals are unchanged native code. The dual native crypto verify (`pv` and `PARAMS_VERIFIER`) and default-Real financial WF, apply and replay are also unchanged native code.

### Blockers

None.

### Prerequisites for any launch (A side)

- A-P1. Three fresh full reports, one from each routed model, with valid terminal receipts, read by root before authorization. The closed authorization must carry exactly the 15 keys, including `r4_actual_result_freeze_sha256 = ad34703a…`, `proof_result_freeze = {path to R4 PROOF-RESULT-FREEZE, e02f…}` and `source_freeze_sha256 = ca65299f…`.
- A-P2. The child Python must run without optimization: no `PYTHONOPTIMIZE` and no `PYTHON*` or `COVERAGE*` variables in the user-manager environment. The helper's `all_six_refusals: True` record and all wrapper gates are `assert`-based; under `-O` they would be fabricated.
- A-P3. Root must check, before launch, that `native-financial-v10-r5-verify.log`, `native-financial-v10-r5-verify-good-first.log` and the host log path are virgin, and that the unit name is absent. The wrapper opens the phase log with `x`, but only after the reservation is fsynced. The helper checks the good-first log only inside the child. A pre-existing file would therefore consume the attempt rather than refuse cleanly.

### Recommendations (not prerequisites)

- The handoff's input-closure sentence ("+ successful R4 proof gate + … + pinned official decoder + new spec cases as absent") is imprecise. The proof gate and the decoder are members of the 88-path freeze, and the R5 cases file is present in the input freeze. The arithmetic 1039 + 88 + 3 = 1130 is correct.
- A future successor could move the phase-log and good-first-log exclusivity check before the reservation. This is not needed now, given A-P3.

## B. ACTUAL PARENT RESULTS: **APPROVE_SCOPED**

### R4 prove

Terminal fields:
- exit 0, `stop_reason` null, `supervisor_error` null;
- source and binary identity true, native postconditions true;
- elapsed 287.614241269 s, peak sampled CPU 321.87 s, peak RSS 2,905,444,352 B, under the 600 / 1200 / 4 GiB limits;
- journal: 6 min 22 s CPU over 4 min 52 s, 2.6 G peak.

Outputs:
- 22 outputs totalling 98,863 B;
- `proof.raw` 6336 B, `32a8aed3…36a3`;
- native receipt `3ffc8ee3…9c65`: `Success`, `proof_count` 1, `native default Real; proof-verifying enabled`, 293 / 47.

Fee arithmetic:
- fee 1,117,490,000,000,001 > 0;
- fee <= allowance 100,000,000,000,000,000,000 <= available 5,000,000,000,000,000,000,000;
- remainder 4,999,998,882,509,999,999,999, and fee + remainder = available (verified).

Other results:
- One `DustInitialUtxo` event with `initial_value` equal to the remainder.
- Replay refused with `Failure(ReplayProtectionViolation(IntentAlreadyExists))`.
- Construction diagnostics: `preimage_equal` and all `CallContext` fields equal.
- `partition-costs-effects.txt` shows 1010 / 1000 / 10.

Escrow 8990, A1 conservation, NIGHT conservation, the expected contract storage, a positive DUST fee and replay with unchanged state are enforced by `accept()` before the receipt is written. The receipt therefore attests them.

Scope: conditional trusted-genesis local financial acceptance only.

### R4 verify

Terminal fields:
- exit 1, `stop_reason` null, `supervisor_error` null;
- source and binary true, native postconditions false;
- 33.827428 s, 27.03 CPU, 716,685,312 B peak RSS;
- 13 partial outputs, 30,781 B.

Results:
- The good-first log is a byte prefix of the full log.
- It shows 1035 `public field i+1` refusals (indices 0-1034), then missing/extra field, address, entrypoint, communication, gas, effects, proof corruption/truncation/suffix-EOF, `STRICT_DECODER` VK/statement/transaction/genesis suffix, absent ownership signature, and `INDEPENDENT_CONDITIONAL_NATIVE_LEDGER_GOOD_AGAIN_OK`.
- `independent-ledger-receipt.json`, `application-result.txt`, `poststate.tagged` and `replay-refusal.txt` are byte-identical to the R4 proof outputs.
- The alternate public funding preflight produced its 5 outputs and passed.
- The first history-suffix case: native exit 1, no output, the typed EOF line above.
- The helper then raised `AssertionError` at line 51 against the stale literal.

The remaining five faults and the helper's final good-again never ran. There is no `history-control-results.json`. The partial suite is correctly classified as FAIL, and the consumed R4 verify attempt is preserved.

The root-cause record `7da98368…f244` and the independent expectations agree with my reading of the primary source.

**This source repair establishes no new verification success.** Approval here covers the actual parent facts and their use as R5 inputs only.

### Older parents

- Disk-stop, R3-interrupted, V10-R2 funded actual and v7 actual are all preserved with consumed reservations and no rebind.
- The trash cleanup was user-authorized and touched only generated `.lake/build` directories. The target, cache and every ELF are unchanged.

### Blockers

None.

### Still open (outside this evidence)

Generic compiler correspondence, source ownership, native payer, authenticated funding, deployment, generic contract/intent/transition/history, PCD and Preview settlement all remain open. SP01 historical campaign admission and accounting remains blocked, and there is no pending public transaction.

## C. RESOURCES: **APPROVE_SCOPED** (one verify-only attempt; separate root live admission still required)

### Envelope

- One offline verify-only attempt: 120 s wall, 240 s group CPU, 2 GiB sampled group RSS and per-process `RLIMIT_AS`.
- Target <= 10 GiB, target +2 GiB, cache +1 GiB, free floor >= 10 GiB.
- Retained R4 proof plus new R5 verify output <= 512 MiB; 2 GiB per file.
- `RAYON_NUM_THREADS=2`, which is not a global hard cap.
- `unshare -Urn` with no network.
- No build, prove, keygen, fetch, wallet or chain.

The envelope is unchanged from R4 verify.

### Launch form

The launch is a one-shot user transient service: `--collect --service-type=exec Restart=no RuntimeMaxSec=900s KillMode=control-group`, with no `--wait` or `--pipe`.

- The native 120 s limit is binding. The outer 900 s only initiates a stop, with the default 90 s stop grace, and covers supervisor hashing outside the child budget.
- It requires a live user manager and session (`Linger=no` observed; no re-login authority). Suspend is not monotonic. The collected unit is lost after exit, so the durable receipt, host log and journal are the record of truth.
- The R4 prove and verify launches were observed to work. The 3 s smoke was not a daemon-restart test.
- Inherited limitations remain: 0.1 s CPU/RSS and 5 s disk sampling can race or overshoot, and SIGKILL or power loss cannot run cleanup. The helper buffers the 698 MB ELF for hashing (R4 peak 716.7 MB, journal 691.6 M), within 2 GiB.

### Fit evidence from actual R4 verify

File mtimes show these timings:
- good-first native run about 25.7 s;
- alternate producer about 0.35 s;
- one fault run about 0.9 s;
- each helper ELF rehash about 0.85 s.

Projected full R5 suite: good-first 25.7 + alternate about 1.2 + six faults about 6 x 1.75 + good-again about 26.5, roughly **64 s wall (about 53% of 120 s)**. CPU is about 60 s (about 25% of 240), and RSS is about 0.7 GiB (about 35% of 2 GiB).

R4 verify ran at load 13.6 / 18.1 / 20.3 on 6 CPUs and still accrued about 0.96 CPU-s per wall-second. **This full-suite timing is a projection and has not been measured.**

### CPU classification (explicit)

**Feasible finite bounded attempt on the currently busy host. Quiet host RECOMMENDED. A numerical CPU threshold is NOT a mandatory prerequisite for this verify-only allocation.**

My R4 `REFUSE_CURRENT_LIVE_ADMISSION` (prove; C-1: load1 <= N-2, load5 <= N, idle >= 2 CPU, no heavy competing jobs) is preserved unchanged as historical dissent on the R4 prove admission. It is not retroactively satisfied, and the R4 Success does not erase it.

My current vote differs for a stated reason. The R4 concern was a foreseeable 600 s wall stop of a multi-minute, multi-threaded prove at about 4x oversubscription. For this mostly single-threaded verify suite, actual R4 verify evidence under comparable load shows roughly 1.9x wall margin.

A wall, CPU or RSS stop would be a valid consumed resource stop. It would never be a verification result and would never be retried automatically.

### Current read-only observation

At 2026-10-01T23:36:47Z:
- nproc 6;
- load 23.77 / 22.90 / 20.91;
- 5 s idle+iowait 0.04 CPU-equivalents;
- `MemAvailable` 19.57 GB, `SwapFree` 5.04 GB;
- free 22,196,994,048 B, against 10 GiB + 4,342,506,048 = 15,079,924,288 B;
- no `beta-native-ledger-consumer`, cargo, rustc, lean or lake process;
- unit `moriarty-native-financial-v10-r5-verify` not found;
- `Linger=no`.

The load is somewhat above the R4 verify admission, so the margin is smaller than projected but still positive under the observed per-task CPU share. This observation is not admission.

### Mandatory prerequisites (root live admission, fresh, immediately before launch)

- C-P1. All three routed full reports have valid terminal receipts and model identities, and root has read them (see A-P1).
- C-P2. A fresh rehash of the source freeze (8), the input closure (1130), the ELF, the actual-88, the proof-67 and the parent closures. This is effectively done by `verify()`, but root confirms before authorization.
- C-P3. Memory available >= 4 GiB, free bytes >= 10 GiB floor + 4,342,506,048 B, target <= 10 GiB.
- C-P4. Python `-O0`, with no `PYTHON*` or `COVERAGE*` keys in the user manager (A-P2); `unshare -Urn` smoke exits 0.
- C-P5. No competing native consumer process.
- C-P6. Exclusive R5 paths and unit name (A-P3). Root pre-creates the virgin host log exclusively and records unit, InvocationID, MainPID, properties and the authority SHA. A `systemd-run` exit of 0 means launch only.

### Recommendations (not prerequisites)

- Prefer a quieter moment, for example load not materially above the R4 verify admission (about 13-20 on 6 CPUs), to protect the about 1.9x margin.
- Do not stop unrelated user jobs, raise any limit or retry automatically.

### Decision mechanics

Resource allocation is decided by the user's majority rule: two substantive agreeing votes, with all dissent preserved. My C vote is an independent current R5 vote and inherits no R4 authority.

## Votes

- **A SOURCE: APPROVE_SCOPED.** No blockers. Prerequisites A-P1 to A-P3.
- **B ACTUAL PARENT RESULTS: APPROVE_SCOPED.** R4 prove is a scoped conditional financial Success. R4 verify is a correctly failed partial suite with good-first and the typed-EOF refusal established. There is no verification-success claim.
- **C RESOURCES: APPROVE_SCOPED** for exactly one offline verify-only 120 / 240 / 2 GiB attempt under prerequisites C-P1 to C-P6.
  - CPU: feasible on a busy host; quiet host recommended, not required.
  - The R4 prove CPU dissent is preserved as historical.

### Limitations

- The claim-suffix message and both truncation messages have never been observed at runtime.
- The full-suite timing is projected, not measured.
- These approvals are review authority only, not native financial acceptance, and they do not complete the goal.
