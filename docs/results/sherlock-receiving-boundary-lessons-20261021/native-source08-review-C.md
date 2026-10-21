# NATIVE08 adapter source — fresh independent non-author SECURITY/API review (lane C)

Requested reviewer identity: `claude-opus-5-5-high` through AGY, high effort. This is the requested identity only, not a backend attestation. I am not the source08 author. I did not read the source08 reviewer-B report and did not reuse any of its opinions.

Subject: `astra-1/NATIVE08-PROVISIONAL-SNAPSHOT10/candidate` (8 files), with `run_selected.py` at 121007 B, sha256 `371cf5e3366191f2f1afce0cf946b824c97d460cbf76cfa6e33d715a11e400ca`.

## Verdict

**REVISE. There are 3 mandatory defects (N1, N2, N3).** The candidate is not eligible for integration or native use until they are fixed and the changed bytes get a fresh review.

Most of the work is correct, and the review checked it against the actual receiving APIs. Source07 A-M1 through A-M4 and B-M1 through B-M4 are resolved in source. A-M5 is only partly resolved. One part of the new typing contradicts the actual producer. That defect (N1) would make the one-shot fail at collection after a genuine successful compile. The 170 and 41 mock passes are author-run, not independent. They cannot detect N1 because the fixture invents the process record.

## Method and tools (truthful)

- I read every file with `read_file`.
- I ran three read-only stdlib commands (`python3 -I -S`, json and hashlib only). I imported no subject code and executed no tests, guard, helper, operator, capture, sudo, Git, network or provider:
  1. I recomputed the bytes and sha256 of all 8 candidate files against SNAPSHOT-MAP: all match. I recomputed all 32 FULL-CONTEXT-MAP snapshot rows: all match. I extracted config top-level keys, `limits`, `paths` and `resource_disposition` values.
  2. I hash-verified and read the two actual helper-pinned historical producer receipts, P9 and P10 `F_CACHED_CURRENT_RESULT.json`. Their shas (`e897099d…` and `fad41c7e…`) equal the helper pins at `cached_current_build.py` L130 and L260. I printed only the process argv[0], the argv length and the key sets.
  3. I counted tokens in the candidate, guard and helper text. I also hash-verified OPERATOR-SEQUENCE05 `OPERATOR-COMMANDS.PROPOSED.json` (154e3edc…) and `NATIVE-COMMAND.TEMPLATE.NOTREADY.json` (e1446d96…) against the pins in `context/expectations/SOURCE-AND-CONTEXT-PINS.json`, then read both.
- I wrote only to this folder: NOTES.md (incremental), REPORT.md and RESULT-DETAILS.json.
- The develop skill was read. The `status` CLI was not run because of the tool restriction.

## Mandatory defects

### N1 — The F-launch decoder refuses the actual producer's process record (critical for the one-shot)

- **Candidate code:** `_validate_f_launch_result` (run_selected.py L1384–1389) requires `len(argv) <= 128` and `_is_norm_abs(argv[0])` for every `f_result.processes[]` entry.
- **Actual producer:** helper `cached_current_f` builds `f_result = guard.launch(c,'F',config_sha)` (cached_current_build.py L2523).
  - Guard `launch` records `{'argv':args,...}`, where `args=bwrap_opts+['--',*cmd]` (bounded_sdk_compile.py L1190, L1238).
  - `bwrap_opts` begins with `sandbox_args`'s first element `'--die-with-parent'` (L885).
  - Audit-worker argv carries one `--ro-bind` triple per binding (L893–930), so it is well over 128 elements.
- **Corroboration:** the read-only reads of the pinned P9 and P10 receipts show argv0 `'--die-with-parent'` in every process. The lengths are 230/126/205 for P9 and 236/126/205 for P10.
- **Effect:** after a genuine successful native compile, `collect_result` raises `CollectError`. The run writes `result-collect-failed.json`, returns `EXIT_FAILED` and produces no consumer Result. This happens after ONE_SHOT has been consumed, and policy03 `scopeSingleUse.attemptsAuthorizedIfAdopted = 1`.
- **Why tests miss it:** `fixture_support.f_result_doc` (L374–382) invents short absolute argv (`/usr/bin/python3 …`), even though its docstring claims producer transcription (L4–7). The test case `"f-proc-relative-argv0"` (test_native_adapter07 L388) enshrines the wrong rule.
- **Minimal remedy:**
  - Type `argv` to the actual guard shape: a nonempty list of nonempty strings with a generous bound (for example ≤ 4096), and no absolute-argv0 requirement. Optionally, require the element after the final `'--'` to be `/usr/bin/python3`.
  - Replace the fixture process records with the real bwrap-prefixed shape.
  - Replace the `f-proc-relative-argv0` case with a positive test that uses a real-shape record.

### N2 — The policy remaining-wall arithmetic is not bound in production; floor checks are dead (residual of A-M5 and B-M4)

- **Candidate code:** `_guard_constants` (L413–430) greps the guard for `POST_R3_TOTAL_BUDGET_S`, `F_CARGO_WALL_S`, `DEFAULT_EXPECTED_D_FREEZE_SHA256` and `DEFAULT_EXPECTED_E_RESOLVED_SHA256`. Token counts show none of them exists in the guard or the helper. So the checks at L1197–1205 (remaining ≤ budget − holdback) and L1330–1338 (D/E freeze) never run against the real guard.
- **What production enforces:** only `policy_remaining_total_wall_max_s ≥ 4162` (config total) and `policy_remaining_f_wall_max_s ≥ 352` (config F), plus `f ≤ total`. A forged packet with total 9999 decodes successfully.
- **What the tests claim:** the fixture `guard_stub` (fixture_support L465–475) fabricates those constants, so `"start-total-wall-floor-exceeded"` and `"end-f-wall-floor-exceeded"` (test L375–376) test branches that do not exist in production.
- **Values not checked:** cumulative and effective-prior values are not bound to the voted policy03 `resourceDispositionStateReturnsExactly`, which specifies 2827637656/323864287/2503773362, 2832194513/328421144/2503773362, 113/10248/4162/352. The actual guard computes those values in `resource_disposition_state` (L447–472).
- **Minimal remedy:**
  - Bind the policy `current_disposition` to values derived from the pinned config `resource_disposition`, which `_config_facts` already loads: cumulative = adopted_prior + native21 delta; effective = cumulative + reserve_ab on E/S; P = cumulative; total/F = config `remaining_*_max` (4162/352) with exact equality; holdback 113; allowance 10248.
  - Delete the nonexistent-constant branches, or source them from constants that really exist (guard `POLICY_TOTAL_WALL_MAX_S`/`POLICY_F_WALL_MAX_S` L99–100, or helper `PRESERVED_PACKET_PINS` L94–101).
  - Stop inventing guard constants in the fixture.

### N3 — An unguarded post-native mkdir can leave the consumed one-shot unrecorded (low cost, mandatory)

- **Candidate code:** `retainedDir` is only `_abs_norm`-checked (L612–613), and its parent's existence is never checked before dispatch. After native and teardown have run, L2426 `retained_dir.mkdir(parents=False, exist_ok=True)` is outside every `try`. `main` (L2582–2588) catches only `BadInput`/`Blocked`.
- **Failure path:** if the parent is missing, `_preserve_elf` fails first. That failure is caught at L2366 and recorded as a preservation failure. Then L2426 raises `FileNotFoundError`, so `cleanup.json`, `outer-observations.json`, `outer-stages.json` and `result.json` are never written. The result is a traceback exit after a consumed native attempt.
- **Minimal remedy:** in `resolve_selection` or `_require_existing_provisioned_host`, require `Path(retainedDir).parent` to be an existing real non-symlink directory. Also make post-native record writing record an unknown on `OSError`.

## Independent disposition of source07 findings

| Finding | Disposition | Grounding |
|---|---|---|
| A-M1 op0..op6 replay | **RESOLVED** | `PROVISION_STEPS=(("prelaunch-verify",False),)` L107–109. The stage table (L761–764) and order (L2252–2254, L2333–2336) contain no op*. `FORBIDDEN_PROVISION_SUBCOMMANDS` (L105) is documentation only: 1 occurrence, never consumed. `_require_existing_provisioned_host` (L1963–1992) runs before and after PREEXEC (L2234, L2285). prelaunch-verify argv equals pinned OPERATOR-COMMANDS (unprivileged; operator L636→L480 `require_unprivileged_operator`). Q-OBS request/provisioner shas equal the request file and operator pins (432538f4 / 7a8e55b9), as operator `main` L814–817 requires. |
| A-M2 abort of a foreign host | **RESOLVED** | When `native_entry is None`, the branch (L2412–2424) runs no teardown or abort. Abort runs only after the native stage was invoked and the marker is absent (L2371–2373), which matches operator `cmd_abort_prelaunch_cleanup` (L792–806). Residual limitation: a runner exception before the child starts still leads to abort of the selected host, which is documented operator semantics. |
| A-M3 helper sibling | **RESOLVED** | L641–648 check the path against the guard sibling and the sha against the guard's `CACHED_CURRENT_BUILD_SHA256` (guard L1075). This is consistent with guard L1759–1761: the path is resolved, and `executing_identity` (L644–647) forces the guard path to equal the resolved runner path. |
| A-M4 cgroup + 3-mount restoration | **RESOLVED** | `_restoration_probe` (L1913–1960) checks E/S/P absence and the absence of execution_root/source_home/packet and their submounts in mountinfo. `complete` requires cleanup ok and restored (L2391–2392). Note: teardown ok with a residual present maps to `unknown`, not `owed`. That fails closed. |
| A-M5 debit/sub-object typing | **PARTIAL → N1, N2** | The exact key sets match the producers: 11-key state = helper L2445–2455 / L2316–2325 / L2140–2149; layers = helper L186–239, L348–373, L2182–2205; policy 13 keys and established 10 keys = guard L466–472 / L485–489; reconstitution 15 keys = helper L968–984; seed counts = L2554; env = guard L841–844. `effective==prior+live` (L1244–1254) = helper L2435–2437. The process argv typing is wrong (N1). The remaining-wall arithmetic is not bound (N2). |
| B-M1 unlaunchable/destructive | **RESOLVED** | Same as A-M1 and A-M2. |
| B-M2 GNU time 0-byte file | **RESOLVED** | `_check_time_output_preexec` (L1995–2012), used at L2241 and L2286. The author-run real `/usr/bin/time` wrap test is at L679–718 (not rerun here). Limitation: source also accepts `absent`, so Root must enforce the wrap (condition C4). |
| B-M3 fingerprint binds the outer recipe | **RESOLVED** | All 7 pins must be members of `task_obj.input_files` (L620–637). `delivery.Task.input_files` is built from identity source/tools/config (delivery L319–338), and `fingerprint` hashes `identity_json` (L347–360). `retainedDir` and `timeOutput` are bound only through `selectionSha256` in PREEXEC, which is acceptable. |
| B-M4 KeyError / allowance holes | **RESOLVED** | Exact disposition key sets come before indexing (L1155–1157, L1215–1217). Start and end must be equal as whole dicts (L1520–1529). Arithmetic residual: see N2. |

## Whole-operation checks (pass, subject to N1–N3)

- **Singleton S and privilege drop:**
  - `build_native_argv` (L692–711) equals the hash-verified NATIVE-COMMAND template byte for byte in structure. Only the root `sh` writes `$$` into S, then execs setpriv (reuid/regid 1000, clear-groups, inh/bounding -all, no-new-privs), then `env -i`, then `python3 -I -B -S`, keeping the same PID.
  - This satisfies guard `supervisor_envelope` (L1010–1044): uid≠0, zero caps, cgroup == leaf, leaf procs == [pid], E unpopulated.
  - Interpolated values are `SAFE_PATH` words or validated by the CG regexes; the config sha is HEX.
  - No selection argv, env or hooks exist (exact key sets L572–599). There is one `subprocess.run` (shell=False) plus the pinned capture.
- **Operator and counters argv:** both equal the pinned OPERATOR-COMMANDS and RAW-COUNTER files exactly (timeout TERM / kill-after 2s / 30s, wall 35, sudo only for teardown and abort).
- **Lifecycle order:** native (pinned `capture`) → counters-after before any removal (L2356) → marker → `_preserve_elf` (fsynced byte copy of the unique fresh ELF at `target/<relative>`, live packet hashes recorded, never executed; L2060–2119) → collect-evidence → teardown, or abort if no marker → evidence verified against hashes captured before teardown plus manifest request/provisioner pins (L2122–2153) → restoration probe → typed cleanup-observation/1 (L2428) → outer observations → ledger → raw `result.json` → the collector runs only if native ok, preservation present and evidence verified (L2498–2500), and it reads only the retained copies.
- **Unknown handling:** a null-exit capture or a `controller_failure` becomes `StageUnknown`, then `EXIT_UNKNOWN`, with no consumer Result (L1850–1858, L2459–2490). A runner exception or a non-integer return code becomes unknown and is never retried. The attempt is consumed through `mkdir(exist_ok=False)`.
- **Readiness and history D1/D2/D3:**
  - `delivery.py`, `reconciliation.py` and `settle_observation.py` are unchanged; their hashes equal the task pins.
  - One `reconciliation.load_readiness` path, with recheck at actual use (L339–365, L2230, L2282).
  - History is optional (reconciliation L578–581), so Root must supply it (condition C3).
- **Accounting scopes:** `outer-observations` (L2435–2453) keeps E/S/P raw counters, controller rusage self and children, and the time output as separate scopes. sudo, orphan and all-descendant CPU are recorded as null. Excluded tails are named.
- **No promotion:** observation is `{built:true, test_bodies_executed:false}`, the decoded receipt is `UNACCEPTED`, and review/adoption are null. C13 is untouched.

## Non-mandatory observations and limitations

- `doctorwatson.sh --execute` execs `python3` from PATH without `-I` (unchanged behavior).
- The capture deadline equals the guard's own 4162 s wall alarm. At the edge, capture can preempt the guard's own terminal receipt, which yields unknown (fail-closed).
- `killpg` on a root-owned sudo group can raise `EPERM` inside capture's handler. That also yields unknown, and the operator teardown's `cgroup.kill` remains the backstop.
- A prelaunch refusal records cleanup `owed`, which needs a later reconciliation (conservative).
- The author REPORT table and its "guard constants" claim are stale or false. Root already qualified the table.
- The mock runner models operator predicates but cannot establish native compatibility.

## Finite at-use conditions (after N1–N3 are fixed and freshly reviewed)

1. **C1:** Two fresh non-author reviews of the revised exact bytes, then Root integration of exactly those bytes. Any later byte change invalidates them.
2. **C2:** The plan task identity binds all 7 pins with exact (path, bytes, sha256):
   - guard 6694bbdd… at the config `bindings.runner` path;
   - helper e439a2cd… at the guard sibling;
   - operator 7a8e55b9… (= Q-OBS `provisioner_sha256`);
   - request 432538f4… (= Q-OBS `request_sha256`);
   - capture 713119d9…;
   - config b8ee9dc9… at `paths.config`;
   - the actual `/usr/bin/time` bytes, hashed by Root at selection (this review did not verify a time-binary hash).
3. **C3:** Selection/2 meets all of these:
   - outputs exactly code-derived;
   - `retainedDir` absent, with an existing real parent;
   - `evidence_dir` `…/sherlock-sdk-evidence-20261020-07` absent;
   - a fresh attempt dir;
   - the complete raw history (and any reconciliations) for the task supplied, never omitted;
   - Root's selection decision recorded (ISSUED currently `executionSelected:false`).
4. **C4:** The single invocation is exactly `/usr/bin/time -v -o <timeOutput> -- ./scripts/doctorwatson.sh --execute …`, run as uid/gid 1000 with zero caps and HOME=/home/charl, with `python3` resolving to the pinned interpreter. Root must treat PREEXEC `timeOutputStateAtPreexec != "precreated-zero-byte"` as a missing whole-wrapper bracket.
5. **C5:** The same-request prelaunch-verify passes at use: Q-OBS mounts 190/191/240 on devices 0:80/0:81/0:83, cgroups unpopulated, children empty, MemAvailable ≥ 23622320128, marker absent. Non-interactive sudo is available only for the fixed native and teardown/abort shapes. If prelaunch fails, no automatic cleanup runs, and Root dispositions the host.
6. **C6:** One attempt with no retry on any outcome. An unknown outcome goes to unknown-reconcile. The raw packet, the preserved unexecuted ELF and the counters are retained. The Result stays UNACCEPTED until a separate result review. Proof, ledger and stagenet acceptance remain unclaimed.
7. **C7:** Historical c and exact S remain null. The 113 s holdback is not measured CPU. Root carries forward reduced allowances from measured deltas. There are no refunds and no cap raises. The two prior resource votes and Root's policy adoption are unchanged. C13 stays stopped and unchanged.
8. **C8:** After the run, Root reads the cleanup, outer-stages and observations records. Any owed or unknown cleanup is completed and reconciled before any further selection.

## Coverage

- **Complete:**
  - all 8 candidate files, whole;
  - all 32 FULL-CONTEXT-MAP units, whole, except as noted under Partial;
  - packet top-level REVIEW-READ-ME, SNAPSHOT-MAP, FULL-CONTEXT-MAP, FINAL-DELIVERY-MATCH, ROOT-REVIEW-FACT-QUALIFICATIONS and OUTPUT-PINS;
  - author FILE-MAP;
  - receipts: both result JSONs, the pins verification, the red probe, and the whole 41-test stderr.
- **Partial:**
  - `config.final.json`: lines 1–200 and 17450–17672 read directly, plus stdlib extraction of top-level keys, `limits` and `paths`. The bulk lists in between were not read line by line.
  - The 170-test stderr: only the tail (L160–175) was read, which shows `Ran 170 tests in 2.783s OK`.
  - The 0-byte stdout receipts were not opened.
- **Not read:** patches, mock-stage, source07 afterimages and PROMPTs, author-records PROMPT/LAUNCH, policy vote reports, config06 review reports. Per instruction, the source08 reviewer-B report was not read.
- **Out-of-packet reads (hash-verified, read-only):** OPERATOR-COMMANDS, NATIVE-COMMAND template, and the P9/P10 F results.
