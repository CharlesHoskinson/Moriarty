# Native financial V10 R5: actual-result review (Opus)

## Verdict

**ACTUAL RESULT: APPROVE_SCOPED**

The completed R5 verify-only run and the unchanged R4 financial proof meet every requested actual predicate within the stated conditional scope: an explicit trusted local public genesis, a fixed financial kernel and the exact production Beta/Core handoff. I found no blocker. The limitations listed under "Findings" and "Open qualifications" are part of this approval.

| Item | Value |
|---|---|
| Actual packet | `NATIVE-FINANCIAL-V10-R5-ACTUAL-RESULT-FREEZE.json`, SHA256 `32305c38817649e1d6bca2d51a93360e8dc065584a5bf7c8fe8bf2c764fd3c26` (recomputed) |
| Requested model / effort | `claude-opus-5-5` / high |
| Routing | `AUDIT-ROUTING-20261001.json` `a415f7d6…` (recomputed): gpt-6.1-sol high, claude-opus-5-5 high, grok-4.7 xhigh; no substitution |
| Role | Routed auditor. I did not author anything. I read no counterpart R5 actual reports and used no subagents. |

## Method

- I loaded the `moriarty-dev:develop` skill (installed copy) and read `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930/AGENTS.md`. I also ran the guarded `status --json`. It reports SP01 accounting/admission blocked, no pending public transactions, and `nextAction` `sp01-loan-report`. That is unrelated to this review, so I did not act on it.
- All checks were read-only: hashing, JSON/log parsing, and source reading of the frozen wrapper, helper, cases, handoff and the six Rust modules. I ran no native binary, helper import, Cargo, build, fetch, keygen, prove, verify, unshare, network or git command, and I changed no cache or artifact. I used one `systemctl --user show` query to confirm the unit's state. The only file written is this report.
- The root actual inspection (`16ef6179…`) and the earlier source reviews were treated as evidence only. I recomputed every predicate below from the raw files.

## Evidence read

- **Wrapper and launch:** `run-native-financial-v10-r5-verify-bounded.py` (all 334 lines), `native-financial-v10-r5-verify-controls.py` (all 60 lines), `NATIVE-FINANCIAL-V10-R5-HISTORY-CASES.json`, the R5 source freeze, the authorization, both live admissions, the root resource decision and review disposition, the launch record, the terminal capture and the attempt reservation.
- **Receipts and outputs:** the R5 supervisor receipt, the combined phase log, the good-first log, the host log, every fault config, log and artifact, `history-control-results.json`, the good-again outputs, and the R4 prove receipt and its 22 outputs.
- **Native source:** `native-ledger-reward-funded-v10-r2-candidate/src/{verify,main,funding,artifacts}.rs` (the `verify::run`, `accept`, `construct`, `verify_history`, `decode` and `read_pinned` paths), plus the `SOURCE-HASHES.json` manifest that covers all six modules.
- **Producer chain:** `native-beta-handoff/README.md`, `handoff.ts` (artifact and config export lines) and the `native-ledger-v5-keyless-handoff` receipt and config.
- **Product contract and history:** `docs/MORIARTY-PRODUCT-CONTRACT.md` (acceptance relation, lines 39–76). For history I read the R4 verify receipt, root cause and log, the cleanup records, and the disk-stop, R3-interrupted and V10-R2 freezes.

## Hash checks (all recomputed; 0 mismatches, 0 missing)

| Set | Entries | Result |
|---|---|---|
| Actual packet `sha256` map (all absolute paths) | 165 | 165/165 |
| Produced ELF `ae7e8dab…`, 698,019,744 B | 1 | hash and size match |
| R5 source freeze `ca65299f…` | 8 | 8/8 |
| R5 input freeze `8e988c0a…` | 1130 | 1130/1130 |
| R4 actual freeze `ad34703a…` (old actual, immutable) | 88 | 88/88 |
| R4 proof-result gate `e02f9372…` | 67 | 67/67 |
| Native source manifest `f7e7eaf4…` (six `.rs` files, Cargo, patches) | 12 | 12/12 |
| Disk-stop `0d15cbbe…` / R3 interrupted `de08a1a6…` / V10-R2 actual `c14f6594…` | 35/35/67 | all match |
| R5 output set vs. receipt vs. packet | 30 files, 122,158 B | the on-disk set, receipt map and packet entries are equal; every hash and size matches |
| R4 proof output set vs. R4 receipt | 22 files, 98,863 B | set equal; every hash and size matches |
| R4 failed verify output set | 13 files | set equal to packet; no suite marker or good-again log |

Identity anchors I recomputed:

| Record | SHA256 prefix |
|---|---|
| Authorization | `0c3e8672` |
| Root disposition | `f726a1af` |
| Supervisor receipt | `2b034520` |
| Terminal capture | `73c283f6` |
| Launch record | `8610801a` (equals the capture's `launch_record_sha256`) |
| Host log | `03f1c6c3` (equals the capture's `host_log_sha256`) |
| R5 config | `e1959fa2` (byte-identical to the R4 verify config) |
| History-control results | `1bbcf289` |
| Live admission R2 | `e1c682df` (the admission bound by the root resource decision) |

The ELF's provenance comes from the frozen V10-R2 build receipt: exit 0, source manifest `f7e7eaf4`, produced `ae7e8dab`. The binary was not rebuilt, because rebuilding is out of scope.

## Full financial predicates (R4 proof, re-established by R5 native verify)

- **R4 prove receipt:** exit 0, `stop_reason` null, `supervisor_error` null, source and binary identity true, native postconditions true, input freeze `5e8e11b4`. `proof.raw` is 6,336 B, `32a8aed3…`. `receipt.json` is `3ffc8ee3…`.
- **Production path, confirmed from source.** The run did not rely on receipt flags alone; `verify::run` performs these steps:
  - Strict tagged decode with canonical re-encode equality for the VK, statement, transaction and genesis.
  - `funding::invariant` (the native NIGHT supply invariant), then `verify_history`, an exact equality check against an independent native reconstruction of the history and claim.
  - Genesis re-derived from the fixture and VK, and compared byte-equal.
  - `single_call`: exactly one intent, one call and no shielded offers.
  - The registered v3 VK must be byte-equal to the config VK, with no v2.
  - Finalized-call public inputs must equal the statement, and the first PI must be nonzero.
  - **Dual `vk.verify`** over the same PI: once with the explicit pinned SRS and once with `PARAMS_VERIFIER`.
- **`accept()` then checks:**
  - Exactly one Dust registration, with allowance 1e20 and no spends.
  - `well_formed(WellFormedStrictness::default())` followed by full `apply`, with the result `Success`.
  - Escrow balance **8990** and storage data equal to the expected contract.
  - The funding input was consumed.
  - Complete UTXO conservation: A1 outputs **{recipient 1000, fee recipient 10}** and a single NIGHT output of **1e12** to the fixture owner, with no other assets.
  - Exactly one `DustInitialUtxo` event, and fee balancing holds.
  - Replay returns `Failure(ReplayProtectionViolation(IntentAlreadyExists))` with an unchanged `state_hash`, and the invariant still holds after replay.
- **Actual numbers** (from `independent-ledger-receipt.json`; recomputed arithmetic):
  - Fee 1,117,490,000,000,001 > 0, ≤ allowance 1e20, ≤ available 5e21.
  - Remainder 4,999,998,882,509,999,999,999, and fee + remainder = available exactly.
  - Prestate `d6aa4010` ≠ poststate `16adcadd`. Statement `50712a16`. `proof_count` 1. Strictness "native default Real; proof-verifying enabled".
- **Byte identity.** In both pristine R5 runs, `application-result.txt`, `independent-ledger-receipt.json` (`3ffc8ee3`), `poststate.tagged` and `replay-refusal.txt` are byte-identical to each other and to the original R4 proof outputs.
- **Producer chain.** The Beta/Core handoff (`handoff.ts`) exports `initial-contract`, `expected-contract` (prestate escrow 10000) and `runtime-native.json`. The R4 `construct` path enforces exactly **293 ops / 47 Popeq reads** (main.rs:168). R5 verifies that R4 transaction and proof against the handoff's `expected-contract` (`716eb7dd`). 10000 − 1000 − 10 = 8990 matches.

## Good-first and good-again runs

- The two logs are **byte-identical**: 58,725 B and 1,051 lines each.
- Each contains exactly 1035 `public field i+1` refusals, in order for i = 0..1034, all `Invalid proof`.
- After those come refusals for the missing PI, extra PI, call address, entry point, communication, declared gas, declared effects, proof corruption, proof truncation and proof suffix (all `Invalid proof`).
- Next are four `STRICT_DECODER_REFUSAL` lines (VK, statement, transaction, genesis), then `NATIVE_LEDGER_WELL_FORMED_REFUSAL absent ownership signature`, then the native marker `INDEPENDENT_CONDITIONAL_NATIVE_LEDGER_GOOD_AGAIN_OK`.
- The combined phase log (118,240 B, 2,119 lines) reconstructs exactly as: good-first, producer, the six fault logs in order, good-again, then the separate helper marker `NATIVE_PUBLIC_HISTORY_CONTROL_SUITE_GOOD_AGAIN_OK`. The helper marker is distinct from the native good marker, and no surrogate flag stands in for either.

## Six faults, producer and repeat (history-control-results: 8 ordered records)

There were nine native CLI calls in total: good-first plus the eight recorded calls. For each record I checked that the command uses the pinned ELF, that the config SHA equals both the record and the command pin, and that the log SHA matches.

| # | Case | Exit | Changed field only | Artifact relation | Output dir | Observed native error |
|---|---|---|---|---|---|---|
| 0 | canonical-native-alternate-producer (`funding-preflight`) | 0 | fixture `night_value` +1, `night_creation_seconds` +1 vs. keyless prepare config | – | 5 exact files | `LOCAL_NATIVE_REWARD_FUNDING_PREFLIGHT_ONLY` |
| 1 | history-suffix | 1 | `funding_history` | base (20,933 B) + `0x00` | absent | `InvalidData "Not all bytes read deserializing 'midnight:vec(ledger-state[v18]):'; 1 bytes remaining"` |
| 2 | history-truncated | 1 | `funding_history` | strict prefix (10,466 B) | absent | `UnexpectedEof "failed to fill whole buffer"` |
| 3 | claim-suffix | 1 | `funding_claim` | base (221 B) + `0x00` | absent | `InvalidData "…'midnight:transaction[v12](signature[v2],proof,pedersen-schnorr[v1]):'; 1 bytes remaining"` |
| 4 | claim-truncated | 1 | `funding_claim` | strict prefix (110 B) | absent | `UnexpectedEof "failed to fill whole buffer"` |
| 5 | canonical-alternate-history | 1 | `funding_history` | producer output; SHA ≠ base | absent | `"supervisor-pinned funding predecessor history differs from independent native reconstruction"` |
| 6 | canonical-alternate-claim | 1 | `funding_claim` | producer output; SHA ≠ base | absent | same reconstruction refusal |
| 7 | pristine-good-again | 0 | none (config bytes equal the R5/R4 config) | – | 4 outputs | full native good marker; receipt `3ffc8ee3` |

Notes on the table:

- Case-file SHAs and sizes match `HISTORY-CASES.json`, and the base pins equal the R4 `funding-history.tagged` and `funding-claim.tagged`.
- No log contains `host artifact identity mismatch`, `invalid supervisor SHA256` or a panic. Every refusal is `Error:` returned from `main`, which exits with code 1, so none is a crash or host-hash refusal disguised as an expected error.
- The claim-suffix and both truncation errors are now **observed**, not merely specified.
- In the producer's `funding-preflight.json`, the result is Success. Default well-formedness, claim application, supply invariants, unchanged originals and pristine good-again are all true. `proof_produced` and `financial_ledger_accepted` are both false.

## Resources and launch identity

- **Admission and launch.** R1 admission (00:00:30Z, `f74a19e5`) was superseded. R2 admission came at 00:02:06.362Z, then the authorization at .589, the root decision (binding R2) at .604 and the launch at .702. The attempt was reserved at 00:02:11.409Z, after the wrapper's pre-launch hash walk. There is one attempt file and no retry or alternate R5 output.
- **R2 admission values:** Python unoptimized, no `PYTHON*`/`COVERAGE*` keys, namespace smoke 0, exclusive paths and unit absent, free 18,542,051,328 B (at least the 10 GiB floor plus 4,342,506,048 B), MemAvailable 18.7 GB, target 9,342,748,478 B (≤ 10 GiB).
- **Unit:** `moriarty-native-financial-v10-r5-verify.service`, invocation `526ab152d36549cbbaa77e550cb603bb`, MainPID 2772251. Properties: `Type=exec`, `Restart=no`, `KillMode=control-group`, `RuntimeMaxSec=900`, stop timeout 90 s. The unit is now `not-found`/inactive and no consumer process is running. Because of `--collect`, the defaults shown for the gone unit are not terminal truth; terminal truth is the receipt, the host log and the journal.
- **Receipt:**
  - Exit 0, no stop, error null, identities true, postconditions true, child launched.
  - Elapsed 67.04 s against a 120 s limit; sampled CPU 58.18 s against 240 s; sampled RSS 716,906,496 B against 2 GiB.
  - 13 disk samples. Minimum free was 18,506,014,720 B. Target 9,342,748,478 and cache 2,933,611,498 stayed constant. Free space went from 18,638,839,808 to 18,608,803,840.
  - The scope caps were unchanged: target 10 GiB, +2 GiB target, +1 GiB cache, 10 GiB free floor, 512 MiB proof+verify output, 2 GiB per file, Rayon 2.
- **Journal:** 80.380 s CPU, 71.829 s wall, 688.2 MB peak. It measures the whole unit, including the wrapper's hash walks before and after the child, so it uses a different instrument from the sampled child group. Every figure is far below its cap.
- **Load.** The host was busy (load about 15 on 6 CPUs, two `lean-lsp-mcp`). A quiet CPU was a recommendation, not a numeric prerequisite, in all three current R5 resource votes.
- **Historical dissent.** The historical R4 Opus prove-CPU refusal and the 2–1 R4 admission are preserved (`c387e212`). R5 does not retroactively satisfy them.
- **Authorization and votes.** All 15 required keys are present. There are three providers with `approved=true`, and their report hashes match the files. The terminal receipts:
  - Opus: `claude-opus-5-5`, end_turn, not error.
  - Sol: actual `gpt-6.1-sol`/high, completed.
  - Grok: `grok-4.7-build` in `modelUsage`, end_turn.
  - All three report A/B/C APPROVE_SCOPED.
  - The source author was Grok 4.6 high and did not vote.

## Historical failure preservation

- The R4 partial verify remains FAIL: exit 1, postconditions false, 13 outputs, no suite marker. The log ends in the helper `AssertionError` on the wrong history-suffix literal. The root cause is recorded, and the R5 source repair did not relabel that attempt.
- The disk stop (−9, total disk limit) and the R3 interruption (−9, SIGTERM) remain as frozen.
- The funded, keygen and historical graph closures still hash correctly.
- The user-authorized Trash cleanup removed only generated `.lake/build` directories. The observed 14,917,431,296 B delta is qualified by concurrent host activity.

## Findings (non-blocking limitations)

1. **F1.** In verify mode, `runtime_ops: 293` and `runtime_reads: 47` are literal constants inside `accept()`'s receipt JSON. The wrapper's assertion on them is therefore tautological for R5. The real 293/47 enforcement is the R4 `construct` path (main.rs:168). It binds to R5 transitively through the byte-identical R4 transaction, proof and receipt. R5 did not re-measure it.
2. **F2.** The truncation cases carry `expected_error: null`. The helper accepted any `Error:` with exit 1 and no output, and did not pin the typed `UnexpectedEof`. The typed EOF is established by this review's reading of the raw logs, not by a helper assertion.
3. **F3.** The fault controls exercise only the funding history and claim decode/reconstruction step, which runs before the cryptographic checks. Cryptographic negative coverage comes from the internal mutation refusals in the two good runs.
4. **F4.** The canonical alternate is a valid local keyless history that also differs in operation registration. Fields 5 and 6 are each isolated at the config level, but the alternate is not a timestamp-only or value-only perturbation, and it is not authenticated history or PCD.
5. **F5.** Resource peaks are sampled at 0.1 s for CPU/RSS and 5 s for disk, so they are lower bounds. Rayon 2 is not a global thread cap. The hard per-process limits came from RLIMIT_CPU, RLIMIT_AS and RLIMIT_FSIZE. The final 716.9 MB RSS sample is consistent with the helper reading the 698 MB ELF into memory to hash it (inference).

## Open qualifications (unchanged; not established by R5)

- The trusted local public genesis (`TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY`, network `undeployed`), the fixed kernel, the manual A1 asset and the call-source mapping are all still open.
- Still open: generic compiler correspondence, source owner and native payer, authenticated funding and deployment, generic contract/intent/transition/history compliance under the product contract's acceptance relation, PCD, and Preview settlement.
- R5 makes no claim of general goal completion and no public Preview transaction claim. There are no pending public transactions, and SP01 campaign accounting/admission remains blocked.
- This approval does not authorize another native attempt, a proof, a build or a key operation.
