# NATIVE-FINANCIAL-V10-R4 proof review

Requested model: `grok-4.7`. Requested effort: `xhigh`.
Returned provider identity is captured by root from the host receipt. This report does not invent a `modelUsage` record and does not allocate a phase.

Read-only guarded status at review start: capability `SP01.6 loan-swap-subset`, next action `sp01-loan-report`, blocked on stale SP01 admission inputs and missing current accounting. `pendingTransactions` is empty. No public transaction is pending. That historical campaign stays blocked. This review is the separate offline candidate audit and grants no dispatch.

Votes, kept separate. None of them is executed native acceptance, a finalized proof, or Preview settlement.

| Scope | Vote |
|---|---|
| A SOURCE | APPROVE_SCOPED |
| B ACTUAL RESULT | APPROVE_SCOPED |
| C RESOURCES | APPROVE_SCOPED |

## Identities checked in this review

| Object | SHA-256 | Bytes |
|---|---|---|
| `NATIVE-FINANCIAL-V10-R4-PROOF-SOURCE-FREEZE.json` | `312c0d1d0e3dd581d0ec9ac6a554dc33ab908a8b983a58db5a496cbaa6cca13f` | 885 |
| `run-native-financial-v10-r4-proof-bounded.py` | `839f6fa09ca7ccc77b0ede6486f3bb66e3a315d1ec41079adcb697c30fb62d93` | 29754 |
| `native-financial-v10-r4-verify-controls.py` | `4dc92714db7aee473e5de8944f0d48217ab96bf30ced963b054bf7074ea98aca` | 5745 |
| `NATIVE-FINANCIAL-V10-R4-PROOF-INPUT-FREEZE.json` | `5e8e11b40def9a86b1e621d5916055b1aa8cb5750cabb49928356a575921c641` | 197669 |
| `NATIVE-FINANCIAL-V10-R4-PROOF-RESOURCE-HANDOFF.md` | `94ace0ff08d4ebde7792527cd0a0be3a33b740a5e767152263424d400c7e6d24` | 10705 |
| `AUDIT-ROUTING-20261001.json` | `a415f7d6bdcf2381f2eb18253c2e96096b4abe99d4c104bb76db5e02fa5df9b4` | 1531 |
| `AUDIT-ROUTING-20261001-OPUS55-HIGH-CANARY.json` | `1160eb48804746ec6dafb196714806bd064ad74878b6ba75d9b2ffceed7a6cfb` | 2619 |
| R3 source freeze parent | `b6d71b415dd5680f257bfb19206271fbf951a7f69051a558bb178fc1d4d11553` | 837 |
| R3 wrapper / helper / input | `c5dfb533cf4a10cd9ebf58c49313c21eaef633a227930e080fdd283c1260ff19` / `d26eefed22bdafc4ef9a1da217c86ec9a89248afaa1b312be367b75bf18ccd4d` / `80c00ddf01abdbdac062c96b925743ac4959eb85d4f2819289512701fb67b335` | |
| Interrupted result freeze | `de08a1a6855495f4b6a57eeb99fb9a9893904a2c0f28a72be4589e9e22ec518d` | 4955 |
| `SOURCE-HASHES.json` (six Rust modules plus six companion source files) | `f7e7eaf472aa2bc24c313d05a69e4790d6c63ec71c2e9ed012ea37a02f91e9a7` | 1423 |
| Live ELF `.../target/debug/beta-native-ledger-consumer` | `ae7e8dab21c4cf09143fa7d58741c0e18ec5f525018de1b8b4099f6fe5a18239` | 698019744 |
| Trash-build cleanup / plan | `5a324f21d3bdb76e2b8a8244e860946befdfeaed654916de5d635f4c6b8e2d8c` / `b1429ed03769e97aac9237cd3850c73b4f48342a315a48ab92a4ae49adb7c4d5` | |
| Old disk-stop freeze / cache cleanup / funded R2-67 | `0d15cbbebd3a41659bdb4a6d27ad2848dd9178d97e1dbd8ef0eda58731fbd97b` / `50edf5acfc1a8218a86a5699ef47f695201ac209307a5a3c9b394b52c95915db` / `c14f6594fe33b0fc48486381dce4ce1087c85472b0fca032c52d9f67e75588eb` | |
| Launch smoke | `b06e562cf0ad6971525d21d06d67f61399f38ddde3413a8df18a1465a173aea6` | 1090 |

Streaming SHA-256 of all 1039 input-freeze paths: 3,298,681,719 bytes, 0 mismatches, 4.026 s on a warm page cache. The live ELF is not a key in that map. It was hashed separately in 0.696 s and matches the `binary` object. All 12 files inside `SOURCE-HASHES.json` match, including:

| Module | SHA-256 |
|---|---|
| `src/main.rs` | `55063ab8e1da422391c7290c7d2e58d093ddb27f35439eb93cafbf0eabe3c52c` |
| `src/funding.rs` | `d584c06c53a0e6ab5fea2ebb7b42c82defa2df727735f7b137ee9258eba59dc5` |
| `src/verify.rs` | `4b7d39feb594aa8026a47c0c54d7803dce059366a69057e6a4cda0541e32cdd4` |
| `src/provider.rs` | `1bf45f54594e1741214635251dd98ed3cac13651c57d1df30c491f808aaf2c6f` |
| `src/keygen.rs` | `9f97af1657ad1e23b099e184ea8fb57d91a96976149d7163541f8218ee915f5b` |
| `src/artifacts.rs` | `4e7be84ede3fb11dc90a58fd166fb749bc53ec07247f86266cacb6bf6ee622d7` |

`binary`, `prove_config`, and `v7_result_freeze` are byte-identical to the R3 input freeze. The input text contains no `v10-r4` path, result-freeze name, or host-log name.

Author stdout `native-financial-v10-r4-source-author-grok-stdout.json` was read as evidence. Its `modelUsage` is `grok-4.6-build`. It records no audit vote. The hash table in that stdout matches the table above. This review does not adopt the author text as a vote.

## A — SOURCE: APPROVE_SCOPED

The R4 packet is a minimal successor of immutable R3 source freeze `b6d71b41…`. After reversing R4 basenames to R3 basenames and reversing the wrapper, helper, and input pins, the wrapper diff that remains is only:

- closed authorization key `interrupted_result_freeze_sha256` (`run-native-financial-v10-r4-proof-bounded.py:26`)
- exact pin `de08a1a6855495f4b6a57eeb99fb9a9893904a2c0f28a72be4589e9e22ec518d` (`:31-32`)
- inside `verify()`, rehash of the interrupted 35 and the exact R3 terminal receipt predicates (`:68-73`)
- the same pin copied onto the future phase receipt (`:314`)

Raw diffstat is wrapper +27/−19 and helper +1/−1. The helper’s normalized diff is empty. Its only raw change is the virgin log basename `native-financial-v10-r4-verify-good-first.log` (`native-financial-v10-r4-verify-controls.py:29`). Python `ast.parse` succeeded for the R4 and R3 wrapper and helper. They were not imported.

`verify()` runs before the reservation (`:154`) and again after the child (`:280`), so the interrupted rehash and receipt asserts are both pre and post. The R3 receipt predicates in source match the file read below: exit `-9`, stop `supervisor exception or signal`, `KeyboardInterrupt` / `supervisor signal 15`, source and binary true, native postconditions false, produced ELF `ae7e8dab…`, 14 outputs, and absence of `proof.raw` and `transaction.tagged`.

The input map keeps all 1003 R3 tuples, in order, with equal hashes. The 36 added entries are the interrupted freeze’s 35 paths, rewritten as absolute paths with equal hashes, plus the freeze file itself. Those 35 include the cleanup plan and actual, the current R3 parent packet (source, wrapper, helper, input, handoff, three reviews, three terminal receipts, authorization, live admission, root decision, disposition), the consumed attempt `f697cb4d…`, config `96684666…`, receipt `ec14b715…`, log `9b6289ce…`, and the 14 preparation outputs. No R4 self-hash or future receipt hash is in the map.

Both `terminal_disk_stop_result_reviews` and `resource_votes` still require exactly the three provider labels `{gpt-6.1-sol, claude-opus-5-5, grok-4.7}`, each `{provider, path, sha256, approved: true}` (`:41-45`). Routing `a415f7d6…` selects Sol high, Opus 5.5 high, and Grok 4.7 xhigh. Canary `1160eb48…` says `purpose: availability only; no audit verdict` and `result: READY`. The R3 authorization object has no `interrupted_result_freeze_sha256`, so it cannot satisfy the R4 closed set. `proof_result_freeze` must stay null for prove (`:175`). Verify accepts only a new root freeze whose path is exactly `NATIVE-FINANCIAL-V10-R4-PROOF-RESULT-FREEZE.json`, whose parent is `native-financial-v10-r4-prove-receipt.json`, with exit 0, preserved identities, `native_postconditions_passed` true, and the whole proof-directory file set (`:178-194`). That freeze is absent now.

Prove and verify stay distinct: 600 s / 1200 group CPU / 4 GiB, then 120 s / 240 CPU / 2 GiB (`:176`, `:195`). Reservation, config, log, and receipt use `O_EXCL` (`:219`, `:227`, `:229`, `:315`). The source contains no deletion or reset of a reservation. R4 receipt, attempt, config, log, host log, proof directory, verification directory, and result freeze are absent. The loaded user unit `moriarty-native-financial-v10-r4-prove.service` is `LoadState=not-found` with an empty fragment path.

The six Rust modules are unchanged and still contain the required financial gates:

- one irreversible proof attempt and a nonzero finalized binding (`src/provider.rs:40-57`), then exactly one record (`src/main.rs:252-254`)
- dual verification, SRS-derived verifier parameters and `PARAMS_VERIFIER` (`src/provider.rs:61-64`, `src/verify.rs:31-32`)
- `WellFormedStrictness::default()` and full `TransactionResult::Success` before the success JSON (`src/main.rs:277-282`, `:301`)
- manual A1 escrow 10000 (`src/main.rs:67`), VM effects 1000+10 (`:123-128`), post-state escrow 8990 plus those two outputs (`:285-290`)
- NIGHT conservation of `night_value` to the fixture owner (`:290`) and `check_night_balance_invariant` (`src/funding.rs:8-9`)
- actual positive DUST fee `consumed = availability - initial_value`, with `0 < consumed <= fee_allowance` (`src/main.rs:291-295`)
- replay must be `Failure` and must preserve `state_hash` (`:297-299`)
- public-input, address, entry, communication, gas, effects, proof-bit, truncation, and EOF refusals, plus absent-owner `well_formed` refusal (`src/verify.rs:33-59`)
- reconstructed funding predecessor (`src/verify.rs:22`, `src/funding.rs:109-134`)

The public helper still does first good (`native-financial-v10-r4-verify-controls.py:29-32`), keyless `funding-preflight` with `night_value += 1` and `night_creation_seconds += 1` (`:35-40`), six verify refusals (`:45-53`), and pristine good-again (`:55-57`). The four spec cases are suffix and truncation of history and claim (`NATIVE-FINANCIAL-V10-HISTORY-CASES.json`). The two canonical cases replace `funding_history` and `funding_claim` with files the alternate producer writes. `genesis` registers `pay` with the optional VK (`src/main.rs:68`); funding preflight calls `genesis(..., None)` (`src/funding.rs:101`). Value, time, and keyless registration therefore move together. This is a different canonical history, not a one-field splice of the original tagged bytes, and the helper text leaves generic history and PCD open (`:59`).

Scoped limitations of A:

- `runtime_ops: 293` and `runtime_reads: 47` are checked dynamically on the retained transcript (`src/main.rs:168`) and then repeated as literals in the success JSON (`:301`). The wrapper rechecks those literals (`run-native-financial-v10-r4-proof-bounded.py:294`).
- The local source calls `WellFormedStrictness::default()` and labels it `native default Real`. Cargo enables `proof-verifying` (`Cargo.toml:7`). This review did not fetch or reopen the upstream enum.
- Provider checks bind a label, a path, and a hash. They do not require three distinct paths, and a report hash is not a runtime model identity. Root still has to read the fresh reviews.
- Financial asserts are Python `assert` statements. This interpreter reports `sys.flags.optimize == 0`, and the handoff command is `/usr/bin/python3` with no `-O`. The wrapper does not test the flag itself.
- The exclusive-path assert omits the phase log (`:173`). The reservation is created at `:218-219`, and the log is created with `open('x')` only at `:227`. A pre-existing log therefore consumes the attempt. The R4 log is absent now. Root’s exclusive-path check is what keeps that from happening on the first launch.
- Stable `rustc 1.98.1` rejects `-Z parse-only`, and no nightly toolchain is installed. Rust was read in full and not typechecked or built. Python AST parse is the parse that ran.

## B — ACTUAL RESULT: APPROVE_SCOPED

This vote accepts the consumed interruption and the already recorded parents as evidence. It does not accept a financial proof.

R3 prove receipt `ec14b715571f984b8339296368b559655ea21eede4e3f5c3023066686ede44af`, attempt `f697cb4d…` at Unix 1790891591.3223357 (`2026-10-01T21:53:11Z`), config `96684666…`:

- command is `unshare -Urn -- env RAYON_NUM_THREADS=2` plus ELF `ae7e8dab…`, `prove`, that config, and `native-financial-v10-r3-proof`
- `exit_code` `-9`, `stop_reason` `supervisor exception or signal`, `supervisor_error` exactly `{type: KeyboardInterrupt, message: supervisor signal 15}`
- `elapsed_seconds` `334.87774517497746`, peak sampled RSS `925040640`, peak sampled CPU `155.69`
- source and binary identities true, `native_postconditions_passed` false, `child_launched` true
- limits recorded as wall 600, group CPU 1200, group RSS and address space `4294967296`
- 64 disk samples, every sample target `9342748478` and cache `2933611498`
- initial free `27071590400`, final free `27479089152`, sample minimum `26802946048` (24.962 GiB), sample maximum `27926892544`
- every initial, final, and sample free value is above the 10 GiB floor `10737418240`
- last sample at 318.496 s has RSS `904593408` and CPU `154.46`; the higher peaks are from the 0.1 s sampler after that 5 s disk sample

The log `9b6289ce…` is 12 lines: `ir_parse`, `k_model`, `replay`, `add_calls`, `envelope`, and `ir_check`, each begin and end. It ends at `ir_check_end`. There is no native error line. `construct` writes that marker at `src/main.rs:219-222` and returns; `prove` then calls `unproven.prove` at `:249`. The process was still in that prove call when the supervisor observed signal 15. The child wait status `-9` is the supervisor `killpg` in `finally` after the catchable signal handler. The receipt does not name the signal sender. Free space moved during the run, including a step from `26802946048` at 60.596 s to `27671056384` at 65.682 s, while target and cache stayed constant. Concurrent disk activity is visible. Who sent SIGTERM is not established. This is a consumed host-signal stop, not a native financial refusal, not a success, and not a disk-floor stop.

Fourteen preparation outputs total 66,373 bytes. `proof.raw`, `statement.tagged`, `transaction.tagged`, `receipt.json`, `poststate.tagged`, `application-result.txt`, and `replay-refusal.txt` are absent. All 14 hashes match `native-reward-funded-v10-r2-preflight-output` byte for byte. `preparation.json` records `native_check` `successful actual Zkir::check`, `preimage_equal` true, `registered_vk_present` true, `proof_invocations` 0. `native-check-result.txt` is `Ok([...])`. `construction-diagnostics.json` shows context match, escrow balance 10000, `runtime_ops` 293, `runtime_reads` 47, and scope `construction only`. `envelope-preflight.json` has guaranteed 1 input / 1 output and fallible 0 / 2, with `ledger_applied` false and `ledger_accepted` false. `preliminary-fees.json` is an unproven estimate, `1128519108356650` within the allowance, `unproven` true. `funding-history.json` records `authority` null and `funding_ctime` 0. Tagged files are `midnight:` documents and were hashed, not treated as a proof.

The attempt file still says `reserved before child launch; never delete to retry`. The 14 outputs and the reservation are present.

Trash-build cleanup `5a324f21…` records user authorization `Find some free space and then execute`, 18 `.lake/build` directories under two Trash copies, selected bytes `14917447680`, before free `13010419712`, after free `27927851008`, observed increase `14917431296`. The 16,384-byte gap between selected size and observed increase is the record’s own hardlink and concurrent-host qualification. All 18 build directories are absent now. Both `.git` directories remain, top-level `.lean` files remain (12 under the first mechanization tree), and both `.trashinfo` files remain. The plan `b1429ed0…` carries the same target list and the same UTC `2026-10-01T21:50:24.301801+00:00` without the after-image. The 1003 older input tuples are unchanged. This cleanup is outside the native target and Cargo cache.

Older parents, read from their receipts rather than from the handoff summary:

- V10 disk-stop receipt: exit `-9`, stop `total target/free disk limit`, `supervisor_error` null, elapsed `224.19636944599915`, CPU `224.68`, RSS `2755014656`, target and cache unchanged at the same two constants, free `13700079616` to `10316431360`, 44 samples, first below-floor sample at 217.391 s with free `10385440768`, 14 outputs, 66,373 bytes, no proof. Wrapper pins `0d15cbbe…` and still requires `native-financial-v10-proof/proof.raw` and `transaction.tagged` to be absent; both are absent.
- Cache cleanup `50edf5ac…`: three commands exited 0, protected hashes match, native target and Cargo cache preserved, attempt consumed, `new_native_allocation` false, protected ELF `ae7e8dab…`, recorded free `22305591296`.
- Funded R2 freeze `c14f6594…` has 67 paths and pins the live ELF. Build, funding-preflight, and preflight receipts are exit 0, identities true, native postconditions true, same ELF and source manifest. Funding-preflight JSON is local `Success` with `proof_produced` false and `financial_ledger_accepted` false. Preflight preparation matches the interrupted 14 outputs and claims no final proof. Retained prior financial ELF `7430db3b…` / 696902872 and diagnostic `e50d5a26…` / 277301184 are inside that freeze. The wrapper also still pins retained v7 ELF `9e1b153a…` / 696706768 and v6 archive `034ed49b…` / 624060592. Those paths were inside the 1039 rehash.
- Counts used by `verify()` match the files: v7 map 44, v5 map 34, consumer manifest 39 + 260, runtime files 240 and kernel files 240. `kernel-prototype/node_modules` is a symlink to `compiler-probe/runtime/node_modules`, as `NATIVE-LEDGER-RUNTIME-INPUTS-V5.json` says. SRS `4a9ef6c7…` is 25,166,212 bytes. Keys `pk.tagged` `28fbadfa…` and `vk.tagged` `6152e754…` are the prove-config pins.
- The v7 result JSON still names the live target path for hash `9e1b153a…`. The wrapper checks the retained v7 file, not that live path. The live path is the later ELF `ae7e8dab…`.

Pinned R3 disposition `5e630ca6…` records three `APPROVE_SCOPED` source, actual-result, and resource votes from `gpt-6.1-sol`, `claude-opus-5-5`, and `grok-4.7` on packet `b6d71b41…`. Terminal receipts `31a803d6…`, `53b5e980…`, and `8313b705…` are the Sol, Opus, and Grok host receipts. Sol’s host context says `gpt-6.1-sol` high. Opus `modelUsage` is canonical `claude-opus-5-5`. Grok `modelUsage` is `grok-4.7-build` for requested `grok-4.7` xhigh. R3 authorization `proof_result_freeze` is null. Those votes do not approve these R4 bytes and do not approve an executed R3 or R4 proof.

## C — RESOURCES: APPROVE_SCOPED

The native bounds in the wrapper are unchanged and are not raised: one prove at 600 s wall, 1200 group CPU, 4 GiB sampled group RSS and per-process address space; a separately authorized verify at 120 / 240 / 2 GiB only after a new R4 success freeze. Shared target at most 10 GiB, incremental target at most 2 GiB, incremental cache at most 1 GiB, free floor 10 GiB, combined proof/verify output 512 MiB, per-file 2 GiB, `RAYON_NUM_THREADS=2`. No build, fetch, keygen, SRS, Compact, wallet, or chain mode is added. Hash walks and disk walks run in the supervisor, outside the child rlimits. Samples are 0.1 s for CPU/RSS and 5 s for disk. `Rayon 2` is not a global thread cap. Helper `sha()` uses `read_bytes` (`native-financial-v10-r4-verify-controls.py:7`), so the 698,019,744-byte ELF is a transient allocation inside the 2 GiB verify address-space cap. Full verify-suite wall time is unmeasured because no successful R4 proof exists.

The handoff-only launch is:

```text
systemd-run --user --unit=moriarty-native-financial-v10-r4-prove --collect --service-type=exec --property=Restart=no --property=RuntimeMaxSec=900s --property=KillMode=control-group --property=StandardOutput=append:/home/charl/research/moriarty-signed-intent-2026-10-01/native-financial-v10-r4-prove-host.log --property=StandardError=inherit --working-directory=/home/charl/research/moriarty-signed-intent-2026-10-01 /usr/bin/python3 /home/charl/research/moriarty-signed-intent-2026-10-01/run-native-financial-v10-r4-proof-bounded.py prove <newphaseauthority> <actualauthoritySHA>
```

No `--wait` and no `--pipe`. `unshare` and `env` are `/usr/bin/unshare` and `/usr/bin/env`, which are on the default user-manager PATH. The wrapper then keeps the absolute ELF, `cwd` at the candidate directory, user and network namespaces, and the child rlimits. `Restart=no` does not grow the budget. `RuntimeMaxSec=900s` is an outer kill of the supervisor. It does not change the inner 600 s native wall. Pre-verify happens before the reservation, so an outer kill during the pin walk does not consume the attempt. An outer kill after reservation does, and the receipt must then record failure. `KillMode=control-group` covers the `unshare` descendant because `unshare -Urn` does not leave the service cgroup; `start_new_session` does not either. A catchable stop still runs the wrapper `killpg`. A systemd SIGKILL or host power loss skips `finally`; the reservation remains and success is not written.

`StandardOutput=append:` keeps supervisor `print` output in the precreated host log. The native child’s stdout and stderr are already redirected to the exclusive phase log. `StandardError=inherit` does not add a second durable file in this packet. `--collect` unloads the transient unit after it becomes inactive or failed. Root has to record the invocation id and MainPID before collection. `systemctl --user show` on a missing unit can still display `Result=success` with `LoadState=not-found`; that combination is not a prove result. The R4 unit is not loaded, and the host log is absent.

Smoke `b06e562c…` launched `python3 -c "import time; time.sleep(3)"` as unit `moriarty-host-independent-launch-smoke-20261001.service`, `Type=exec`, `Restart=no`, `RuntimeMaxSec=20s`, `KillMode=control-group`, launch exit 0, invocation `45bf94b8965449e9ae98fdc67c39f333`, then `LoadState=not-found` and `Result=success`. Its own qualification says daemon-restart survival was not tested. It is not a prove, not an `unshare` grandchild test, and not the R4 unit.

This review’s 1039-file rehash plus the ELF finished in about 4.7 s on a warm cache. That does not measure a cold post-prove walk while the prover holds RAM. It also does not show that the proof will finish inside 600 s. At review time `/proc/loadavg` was `14.94 20.82 23.18` on 6 CPUs, and free space on the target filesystem was 26,163,056,640 bytes (24.366 GiB). Both are stale context. They are not live admission. Root still has to read all three fresh reviews, check identities, resources, exclusive paths, and unit absence, and admit the phase before any launch. Verify stays unallocated until a new successful R4 proof and a new whole-output freeze.

## Open gates

Local success would still be a trusted-genesis, manually specialized A1 fixture, fixed kernel, and this source’s native payer. It would not by itself be generic compiler correspondence, authenticated chain state, contract-intent transition history, PCD, or Preview settlement. No public transaction is pending. The historical SP01 campaign remains blocked and is not this allocation.

## Review bounds

No native binary, helper import, Cargo build, fetch, keygen, prove, or verify was run. No cache, Git, source, artifact, or receipt bytes were modified except this report. Current counterpart V10-R4 reviews were not read. Historical R3 receipts and the disposition were used only as parent evidence.
