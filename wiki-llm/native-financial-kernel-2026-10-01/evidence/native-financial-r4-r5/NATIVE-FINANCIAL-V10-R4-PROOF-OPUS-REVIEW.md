# NATIVE-FINANCIAL-V10-R4 proof packet: independent full candidate audit (Opus)

- Requested auditor: `claude-opus-5-5`, effort `high` (user routing `AUDIT-ROUTING-20261001.json` a415f7d6…f9b4). This is a fresh independent review session. The root captures the returned runtime identity separately; this report's SHA is not a model identity.
- Role: review only. I hold no author vote. I did not read counterpart V10-R4 reports, solicit other reviewers or dispatch subagents.
- Date: 2026-10-01. Evidence root B = `/home/charl/research/moriarty-signed-intent-2026-10-01`.
- Startup: I read the checked-in develop skill (no Skill tool was exposed) and AGENTS.md, then ran guarded `status`. It reports SP01.6 historical accounting blocked (stale binding/candidate inputs, missing current accounting, resource live state unavailable), `next` = `sp01-loan-report`, and no financial transaction evidence. No public tx is pending, so I have no notification lines to post. Status is not dispatch authority.

## Method and mutation disclosure

I read in full: the R4 wrapper (321 lines), the R4 helper (60 lines), the R4 handoff, the R4 source freeze and the six unchanged Rust modules (`main.rs`, `funding.rs`, `provider.rs`, `verify.rs`, `keygen.rs`, `artifacts.rs`). I also read the complete R3 interrupted freeze contents (receipt including all 64 disk samples, attempt, config, log, the 14 outputs as hashed, root authorization, live admission, resource decision and review disposition), the new cleanup plan and record, the launch smoke record, the old V10 disk-stop receipt, the original cache cleanup, the V10-R2 actual freeze and its preflight and funding-preflight results, and the history-case spec.

Checks run (all read-only):

- Python streaming SHA-256 of every pinned path.
- `ast.parse` of the wrapper and helper, with no import or execution of either.
- A reverse-normalized `diff` of R3 against R4.
- Read-only `systemctl --user show-environment/show/list-units`, `loginctl`, `/proc` and `df` queries.

I ran no native, Cargo, build, fetch, keygen, prove, verify, network, service, unit, Git or cache operation.

One disclosure: I briefly wrote a temporary hashing script and its output to `/tmp/opus_r4_rehash.py` and `/tmp/opus_r4_rehash.out`, then deleted both. Nothing under B other than this report was written.

## Exact identities independently recomputed

| Item | SHA-256 | Status |
|---|---|---|
| `NATIVE-FINANCIAL-V10-R4-PROOF-SOURCE-FREEZE.json` | 312c0d1d0e3dd581d0ec9ac6a554dc33ab908a8b983a58db5a496cbaa6cca13f | match |
| wrapper `run-native-financial-v10-r4-proof-bounded.py` | 839f6fa09ca7ccc77b0ede6486f3bb66e3a315d1ec41079adcb697c30fb62d93 | match |
| helper `native-financial-v10-r4-verify-controls.py` | 4dc92714db7aee473e5de8944f0d48217ab96bf30ced963b054bf7074ea98aca | match |
| input `NATIVE-FINANCIAL-V10-R4-PROOF-INPUT-FREEZE.json` | 5e8e11b40def9a86b1e621d5916055b1aa8cb5750cabb49928356a575921c641 | match |
| handoff `NATIVE-FINANCIAL-V10-R4-PROOF-RESOURCE-HANDOFF.md` | 94ace0ff08d4ebde7792527cd0a0be3a33b740a5e767152263424d400c7e6d24 | match |
| `AUDIT-ROUTING-20261001.json` / `…-OPUS55-HIGH-CANARY.json` | a415f7d6…f9b4 / 1160eb48…cfb | match |
| parent R3 source freeze / wrapper / helper / input | b6d71b41…1553 / c5dfb533…ff19 / d26eefed…cd4d / 80c00ddf…b335 | match, immutable |
| R3 interrupted freeze | de08a1a6855495f4b6a57eeb99fb9a9893904a2c0f28a72be4589e9e22ec518d | match, 35 entries |
| new cleanup record / plan | 5a324f21…8c2d8c / b1429ed0…c7b4d5 | match |
| launch smoke | b06e562cf0ad6971525d21d06d67f61399f38ddde3413a8df18a1465a173aea6 | match |
| candidate `SOURCE-HASHES.json` (12 entries, six Rust modules) | f7e7eaf472aa2bc24c313d05a69e4790d6c63ec71c2e9ed012ea37a02f91e9a7 | match |
| ELF `target/debug/beta-native-ledger-consumer` | ae7e8dab21c4cf09143fa7d58741c0e18ec5f525018de1b8b4099f6fe5a18239, 698019744 bytes | match |

Stream rehash of all 1039 input-freeze entries returned 0 mismatches (3,298,681,719 bytes). The six source-freeze entries and all eight `prove_config` artifacts also match, and each `prove_config` hash equals its own input-map entry. Both scripts AST-parse; the wrapper contains 127 `assert` statements and the helper 22. The root's claimed rehash of current 6 + 1039 is consistent with my independent result.

---

## A: SOURCE. Vote: **APPROVE_SCOPED**

### Minimal-successor verification

Reverse-normalized diff (`r3→R` in R3 vs `r4→R` in R4):

- **Wrapper.** The only changes are:
  - `V8_INPUT_PIN` → 5e8e11b4 (line 9).
  - The closed authorization key set gains `interrupted_result_freeze_sha256` (line 26).
  - `INTERRUPTED_PIN` de08a1a6… is defined and asserted exactly (lines 31–32).
  - `verify_helper_pin` → 4dc92714 (line 36).
  - A new `verify()` block (lines 68–73) loads the interrupted freeze pinned, asserts 35 entries, rehashes all 35 and checks the actual R3 terminal receipt: `exit_code==-9`, `stop_reason=='supervisor exception or signal'`, `supervisor_error=={'type':'KeyboardInterrupt','message':'supervisor signal 15'}`, source and binary `True`, `native_postconditions_passed False`, `produced_binary_sha256==binary_pin`, 14 outputs, and no `native-financial-v10-r3-proof/{proof.raw,transaction.tagged}`.
  - The receipt dict copies `interrupted_result_freeze_sha256` (line 314).

  Raw diffstat is +27/−19, which matches the handoff's claim (line 7). Every other difference is the r3→r4 virgin basename substitution.
- **Helper.** The only change is the first-good log basename `native-financial-v10-r4-verify-good-first.log` (line 29).
- **`verify()` timing.** It runs before reservation (line 154) and again after child termination (line 280), so the interrupted 35 set is rehashed both pre- and post-execution. The unpinned R3 receipt read at line 71 is covered because that receipt (ec14b715…) is one of the 35 rehashed on line 70.
- **Input freeze.** All 1003 R3 tuples are present with identical hashes and identical insertion order. Nothing was removed or changed. The 36 added entries are exactly the 35 interrupted entries (including the R3 source freeze, input freeze, wrapper, helper, handoff, three R3 reports and receipts, R3 authorization, live admission, resource decision and review disposition, attempt, config, log, receipt, 14 outputs, new cleanup plan and record) plus the interrupted freeze itself. `binary`, `prove_config` and `v7_result_freeze` are byte-equal to R3. No R4 path appears in the map, so there is no self-hash or future hash.
- **Native closure unchanged.** The source manifest f7e7eaf4, ELF ae7e8dab, keys, SRS, IR c20c6e73, history cases 103fa0e3 and v5 projection are unchanged. Config pin 96684666 is reproduced because the `prove_config` bytes are identical (wrapper lines 157–168, 196). Distinct exclusive R4 output, reservation, config, log and receipt paths are enforced at line 173; I confirmed they are all absent. The R3 reservation, receipt and outputs cannot be adopted: prove requires `proof_result_freeze is None` (line 175) and writes only under `native-financial-v10-r4-proof`.
- **Three-provider families.** Both families still require exactly `{gpt-6.1-sol, claude-opus-5-5, grok-4.7}` with closed `{provider,path,sha256,approved}` entries and `approved is True` (lines 41–45). Both are rehashed again after execution (lines 152–153).

### Whole-source behaviour I re-audited (unchanged, not diff-only)

- **Prove (main.rs 241–266, provider.rs 40–66).**
  - SRS identity is enforced.
  - Construction is redone with the real VK.
  - The one-call budget is irrevocable (`attempts==0` and checked skips equal; incremented before proving).
  - A finalized nonzero binding is required.
  - The proof is verified twice: with the SRS-derived verifier params and with the embedded `PARAMS_VERIFIER`.
  - Exactly one record is required, and its PIs must equal the finalized ledger call PIs with binding as the first PI.
  - The statement must survive sign and seal.
- **Accept (main.rs 268–302).**
  - Default `WellFormedStrictness` and full `TransactionResult::Success`.
  - Escrow 8990 and storage equal to the expected contract.
  - The funding input is consumed; A1 outputs are exactly `{recipient 1000, fee 10}`; exactly one NIGHT output to the payer equals `night_value`.
  - The NIGHT supply invariant holds on input, output and replay.
  - Exactly one `DustInitialUtxo`, with consumed fee `0 < d−initial ≤ allowance` and non-negative fee balance.
  - Identical replay is `Failure` with an unchanged state hash.
- **Wrapper postconditions (lines 291–311).** They re-check `Success`, `proof_count==1`, strictness string, 293 ops / 47 reads, `0<fee≤allow≤available`, `fee+remainder==available`, and pre ≠ post. They require the exact 22-name output set, which matches the 14 construct outputs plus 8 prove outputs written by `main.rs`. Seven preparation artifacts must be byte-identical to the R2 registered preflight. The statement, prestate and poststate hashes must equal the artifacts, and `proof.raw` must be non-empty.
- **Verify (verify.rs 10–66, wrapper 177–195, 283–287, 305–310).** Verify depends on success:
  - The proof receipt must be `exit 0` with postconditions `True`, input and v7 pins equal, and the binary equal.
  - The whole proof output set must be root-frozen at the exact absolute-path `{sha256}` map `NATIVE-FINANCIAL-V10-R4-PROOF-RESULT-FREEZE.json`, which must include the prove receipt.
  - That set is rechecked after verification.

  The native verifier then checks:
  - Canonical re-encoding of statement, transaction and genesis.
  - Independent funding predecessor reconstruction (funding.rs 129–135).
  - Genesis reconstruction with the registered v3 VK.
  - Call, proof and statement equality, with binding0 refused.
  - Dual verification.
  - Tamper controls: per-PI +1, missing and extra PI, address, entry point, communication commitment, gas, effects, proof bit flip, truncation, suffix EOF, VK, statement, transaction and genesis suffix decoders, and missing ownership signature.
  - A fresh full `accept`.

  The independent receipt must equal the original byte-for-byte as JSON (wrapper line 306).
- **Helper controls.** The helper runs, in order:
  1. A first good verify.
  2. An alternate public-fixture `funding-preflight` producer (`night_value+1`, `night_creation_seconds+1`, keyless v5 prepare config).
  3. Six matched-SHA fault invocations: four strict-decoder cases plus canonical-alternate history and claim, each requiring exit 1, no output dir, and no host-identity or SHA-format error.
  4. A good-again run whose receipt bytes equal the first.

  The spec qualification correctly labels the alternates as canonical changed history and claim (value, time and keyless registration). They are not isolated field tests and do not provide authenticated-history or PCD closure.

### Source findings (none blocking)

- **A-1 (low; inherited from R3; root discipline).** Review and resource vote entries are bound only by provider label, path, SHA and `approved` (lines 41–45). Nothing in the wrapper prevents a root from supplying the R3 report paths and SHAs, which are themselves pinned R4 inputs, as R4 votes. The rule against substituting old votes is enforced procedurally, as the handoff says on line 9, but not mechanically. Root must use the fresh R4 report files. A future successor could refuse any vote SHA already present in `inputs['sha256']`.
- **A-2 (low; inherited).** Every gate is an `assert`, and nothing checks `sys.flags.optimize` in-script. The interpreter is not isolated: `/usr/bin/python3` without `-I` puts B first on `sys.path`, honours `PYTHON*` variables and runs user-site `.pth` hooks. One such hook exists: `~/.local/lib/python3.14/site-packages/a1_coverage.pth` runs `coverage.process_startup` when `COVERAGE_PROCESS_START` or `COVERAGE_PROCESS_CONFIG` is set.

  What I observed:
  - `/usr/bin/python3` resolves to Python 3.14.4 with optimize 0.
  - The current user-manager environment keys are `HOME LANG LOGNAME PATH SHELL USER XDG_RUNTIME_DIR QT_ACCESSIBILITY XDG_DATA_DIRS DBUS_SESSION_BUS_ADDRESS GPG_AGENT_INFO SSH_AUTH_SOCK`, with no `PYTHON*` or `COVERAGE*` variables.
  - B has no stdlib-shadowing modules.

  Under systemd the relevant environment is the user manager's, not the root shell's. The root's "Python opt0" check must therefore read `systemctl --user show-environment` immediately before launch.
- **A-3 (info).** The launch smoke record is not pinned in the source or input freeze. The handoff does not cite its SHA, so the root launch record should pin b06e562c…. The unconsumed `NATIVE-FINANCIAL-V10-R3-PROVE-ADMISSION-REFUSAL.json` and the R3 reviewer stdout and prompt files are also outside the 35 and 1039 sets. None is a consumed-attempt artifact, so I do not treat this as a closure defect.
- **A-4 (info).** After the monitor's `finally` (line 261), SIGINT and SIGTERM stay blocked for the rest of the run. Post-interrupt verification and receipt writing therefore cannot be re-interrupted by a catchable signal; only SIGKILL or power loss can stop them. A SIGTERM between handler install (line 217) and the `try` (line 226) can leave a reservation without a receipt. This window is a few milliseconds and inherited.

---

## B: ACTUAL RESULT. Vote: **APPROVE_SCOPED** (consumed catchable-signal stop; no financial result)

This verdict accepts the classification only. It is not executed native success.

- **Receipt.** `native-financial-v10-r3-prove-receipt.json` (ec14b715…) records:
  - `exit_code -9`, `stop_reason "supervisor exception or signal"`, `supervisor_error {KeyboardInterrupt, "supervisor signal 15"}`.
  - `elapsed_seconds 334.87774517497746`, `peak_sampled_group_cpu_seconds 155.69`, `peak_sampled_group_rss_bytes 925040640`. These are below the 1200 s CPU and 4 GiB limits, and the 600 s wall limit was not reached.
  - Source and binary identity `true`, `produced_binary_sha256` ae7e8dab, `native_postconditions_passed false`, `child_launched true`.
  - Exactly 14 outputs totalling 66,373 bytes. They rehash, and the directory set equals the receipt set.
  - No `proof.raw`, `statement.tagged`, `transaction.tagged`, `receipt.json`, application, poststate or replay output.
- **Attempt and config.** The attempt (f697cb4d…) is a durable reservation. The config is 96684666…, the same closed ProveConfig as the old V10 attempt.
- **Log.** The log (9b6289ce…) has exactly 12 `NATIVE_PUBLIC_STAGE` begin/end markers from `ir_parse` through `ir_check`. File mtimes place `ir_check_end` about 117 s after launch (config 15:53:11.35, log 15:55:08.71 local).
- **Disk.**
  - All 64 samples show target 9342748478 and cache 2933611498 unchanged.
  - Free space is 27071590400 initially and 27479089152 at the end; first sample 26906836992, minimum sample 26802946048, last sample 27479105536 at 318.496 s.
  - Every sample is above the 10 GiB floor, so this is not a disk-floor stop.
  - The R3 funding history and claim equal the spec base (1672df97 / 1185a81d), confirming deterministic reconstruction.
- **Process interpretation.**
  - The native child ran in its own session (`start_new_session`). A signal aimed at the agent's process group would therefore reach only the supervisor.
  - The child's −9 is consistent with the supervisor's own `killpg(SIGKILL)` in `finally`.
  - `elapsed_seconds` includes post-interrupt cleanup and re-verification. The signal arrived after the 318.5 s sample and before the next one would have been due (about 323.5 s), so post-processing took roughly 11–16 s.
  - Source, ELF and the 1003-input closure were re-verified after the stop.
- **Qualifications.**
  - No artifact in the 35 records the sender of SIGTERM. "Host daemon restart" is the root's account. What the evidence establishes is a catchable SIGTERM delivered to the supervisor.
  - Nothing about proving progress can be inferred from CPU, RSS or stage markers.
  - Writer or other host activity behind the free-space changes is not established.
  - This is not a native financial refusal or success, not a disk fault, and not an identity failure.
  - The reservation and its outputs must stay consumed forever, with no deletion, rebinding or reset.
- **Root records.**
  - The R3 authorization (09f31ec6) pins the three R3 reports (Sol 1479d07c, Opus 1dfbec0d, Grok 86860173) in both families.
  - The review disposition records the three R3 audits as source, actual and resource `APPROVE_SCOPED` for the R3 source packet and the then-current parents. They approved R3 source, not any executed result.
  - The resource decision pins the three terminal receipts and the new cleanup 5a324f21.
  - Live admission at 21:52:14Z recorded free 27927265280 and headroom 17.19 GB against 4.34 GB required.
- **Additional cleanup.**
  - The plan and record share UTC 21:50:24.30Z; file mtimes are 15:50:24 and 15:50:30 local, before R3 admission and launch.
  - Targets were only the 18 `.lake/build` directories under the two trashed 2026-08-14 repositories.
  - Read-only spot checks: both trashed `.git` directories exist, both `.trashinfo` files are intact, both trees still hold 9632 `.lean` files and all package source directories, and no `.lake/build` remains.
  - The record shows free 13010419712 → 27927851008 (an observed increase of 14917431296), qualified by concurrent activity and hardlink/block accounting. All 1003 then-current inputs rehash today.
  - No whole source tree or package cache was destroyed.
- **Older parents.**
  - Old disk stop 0d15cbbe (35 entries): wall 224.196 s, CPU 224.68, RSS 2755014656, free 13700079616 → 10316431360, target and cache unchanged, 14 preparation outputs totalling 66,373 bytes, no accepted financial result.
  - Original cleanup 50edf5ac: pip, uv and npm caches only; protected hashes matched.
  - V10-R2 actual 67 (c14f6594): build, funding-preflight and preflight all terminal Success with postconditions `True`. Preparation shows a successful actual `Zkir::check`, `preimage_equal`, a registered VK and 0 proofs. Funding preflight shows Success, default WF, claim applied, invariants held, originals unchanged and good-again equal, with no proof or financial acceptance.
  - Three retained ELFs, keys, SRS, runtime (240+240), symlinks and the production consumer (39+260) are pinned and rehashed.

---

## C: RESOURCES and launch. Vote: **APPROVE_SCOPED** (the frozen envelope and launch form; not live admission)

The native budgets and limits are unchanged from R3:

- Prove: one attempt at 600 s monotonic wall, 1200 s group CPU, 4 GiB sampled RSS and per-process address space (lines 176, 214).
- Verify: allowed only after actual R4 Success plus a root whole-output freeze, at 120 s / 240 s / 2 GiB (line 195).
- Shared limits: free floor 10 GiB, target ≤ 10 GiB, target +2 GiB, cache +1 GiB, combined output ≤ 512 MiB, 2 GiB per file, `RAYON_NUM_THREADS=2` (not a global hard cap), network-namespaced `unshare -Urn`, exclusive fsynced reservation, config, log and receipt.

No retry, reset, cache cleanup, keygen, SRS, Compact, wallet, chain or new framework is involved. The systemd wrapper only adds an outer 900 s cap around the unchanged supervisor, so it does not grow any budget.

### Launch semantics checked

- **Command form.** `systemd-run --user --unit=… --collect --service-type=exec --property=Restart=no --property=RuntimeMaxSec=900s --property=KillMode=control-group --property=StandardOutput=append:B/native-financial-v10-r4-prove-host.log --property=StandardError=inherit --working-directory=B /usr/bin/python3 B/run-…-r4-proof-bounded.py prove <auth> <sha>`. There is no `--wait` or `--pipe`.
- **Isolation from the agent.** The agent runs in `0::/init.scope`, and the user manager is `user@1000.service`. A SIGTERM sent to the agent's command group therefore cannot reach the service. This addresses the R3 failure mode.
- **PATH.** The user-manager PATH includes `/usr/bin`, so `unshare` resolves to `/usr/bin/unshare`. The verify-phase `env … python3` resolves to `/usr/bin/python3` (3.14.4) in the service.
- **Unit and log paths.** No `moriarty*` unit is currently loaded or installed. All R4 host-log, receipt, attempt, config, log, proof, verification and result-freeze paths are absent.
- **What happens if `RuntimeMaxSec` expires.** systemd sends SIGTERM to the whole control group, including the native child, so the recorded child exit may be −15 rather than −9. The supervisor's `finally` then records a consumed stop. After `DefaultTimeoutStopUSec` (observed `1min 30s`), SIGKILL follows. R3's post-interrupt processing took about 11–16 s, so a receipt would normally be written well before then. If post-processing ever exceeded 90 s, the reservation would be left without a receipt; that case remains a consumed failure.

### Conditions and limitations for root (no implicit budget growth)

- **C-1 (medium; admission timing).** CPU contention can make the 600 s wall limit the binding stop. Evidence:
  - R3 averaged about 0.49 CPU-s per wall-second (154.46 CPU-s by 318.5 s).
  - The old V10 attempt averaged about 1.0 (224.68 CPU-s by 224 s) and reached 2.76 GB RSS without finishing proving.
  - R3 needed about 117 s just to reach `ir_check_end`.
  - The ELF is a debug build.
  - During this review the host load average was 25.32 / 24.87 / 24.19 on 6 CPUs.

  At R3-like contention, a wall-limit stop is a plausible outcome and would irrevocably consume R4. I do not recommend raising any limit. I recommend that root's live admission confirm CPU headroom (load substantially below CPU count, no competing heavy jobs) rather than launching into the currently observed load. A wall, CPU or RSS stop would be a valid consumed resource stop, never a financial result.
- **C-2 (medium; remedy efficacy unestablished).** `loginctl` reports `Linger=no`. The user manager is backed by session c1 (pts/1, leader 1218, class user) and c2 (manager). If the last user session ends or the WSL VM stops, the user manager will stop the service: SIGTERM to the cgroup, recorded as a consumed stop. The smoke test (b06e562c) shows only an independent 3 s launch and collection. It is not a daemon-restart or session-loss survival test. Enabling linger would be a host mutation that needs user authorization and is outside this review.
- **C-3 (low).** A host suspend or VM pause is not counted by `time.monotonic` or by `RuntimeMaxSec`. The R2 preflight receipt shows `elapsed_seconds 62.86`, yet its attempt and receipt mtimes are 10:18:57.65 and 10:25:13.43 (about 376 s apart), with outputs written over that span. This is consistent with a paused host, though the cause is not established. Budgets are therefore monotonic-time budgets.
- **C-4 (low).** `--collect` discards unit state after exit (the smoke shows `LoadState=not-found`). Root must record the actual unit, InvocationID, MainPID and authority SHA at launch, and must treat a `systemd-run` exit of 0 as launch only. Terminal truth is the wrapper receipt plus the host log and journal.
- **C-5 (low).** Immediately before launch, from the user manager rather than the shell, root should re-check:
  - The user-manager environment has no `PYTHON*` or `COVERAGE*` variables (A-2).
  - The namespace smoke still passes.
  - Free space and target/cache sizes are within limits.
  - No competing native process is running.
  - Unit-name and path exclusivity still hold.

  The `systemctl --user` state is currently `degraded` because of failed Chromium scopes, which is unrelated but recorded.
- **C-6 (info).** The verify suite timing is unmeasured. It runs nine native invocations: first good, the alternate funding-preflight, six faults and good-again. Two of these perform a full verify plus `accept`, all inside 120 s / 240 CPU-s and 2 GiB address space per process. The helper also temporarily holds the 698 MB ELF via `read_bytes` hashing within that ceiling. A verify resource stop would be a consumed resource stop and must not be retried automatically.
- **C-7 (info).** Sampling is 0.1 s for CPU and RSS and 5 s for disk, so overshoot races are possible. Hashing and tree walks run outside the child limits. Uncatchable SIGKILL or power loss cannot run cleanup and remains a consumed failure with no acceptance.

---

## Scope reminders

The general financial goal remains open. Even a future local R4 Success with independent verification would be conditional on:

- trusted public development genesis
- manual A1 escrow
- a specialized fixed kernel rather than generic compiler correspondence
- the source-owner versus native-payer mapping

It would leave these unresolved:

- authenticated chain state and history
- contract, intent, transition and history compliance
- PCD
- Preview settlement

No public transaction is pending. The historical SP01 campaign stays blocked. This review concerns only the independently reviewed offline fixture allocation, and no executed acceptance is claimed or implied.

**Votes: A SOURCE = APPROVE_SCOPED; B ACTUAL RESULT = APPROVE_SCOPED; C RESOURCES = APPROVE_SCOPED (conditions C-1 to C-5 for live admission).**
