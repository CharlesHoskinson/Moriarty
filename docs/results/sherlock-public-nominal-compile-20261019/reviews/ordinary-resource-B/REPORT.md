# Op01 resource delta disposition: proposed 120 s wall / 90 s CPU

**Reviewer:** Opus 5.5 HIGH, fresh and independent.

**Method.** I read only the supplied text. I ran no commands, Git, compiler, provider, native tools or SDK, and opened no expected bodies. This is a static reading, not an execution observation, and it authorizes no operation.

## Disposition: **ELIGIBLE_WITH_SCOPED_NEW_ORDINARY_DISPOSITION**

The scope is one fresh attempt of revision02 op01. The only changes are `30s`→`120s`, `--cpu=25:25`→`--cpu=90:90` and `wallSeconds` 30→120. AS stays at 2 GiB, FSIZE at 64 MiB and core at 0. op02–op11 are unchanged.

**Rationale**

- **This is not a blind retry.** My prior opinion made op01 ineligible only at identical caps. The proposal changes the one limit that the consumed receipt names as the controller (`controller_failure: "wall deadline"`, elapsed 30.0038 s). It also raises CPU so CPU is not left tighter than the old wall.
- **The unknowns are accepted, not resolved.** CPU, VM and rlib size for the current source are unmeasured. I do not infer CPU from wall time, and I do not infer from the 7.24 s historical success that the current source fits. Every remaining unknown fails closed under a hard limit:
  - CPU → kill
  - AS → allocation abort
  - FSIZE → SIGXFSZ
  - wall → kill
- **The cost of being wrong is bounded.** It is one consumed attempt, with no artifact and no downstream consumption. That is an acceptable owner trade. No subject-prefix replay or current-fit observation is required.
- **The jump from 7.24 s to >30 s is unexplained.** Possible causes include source growth, nonlinear type-check or monomorphization cost, or host contention on WSL2. No supplied evidence decides between them. 120 s may still be insufficient, and that risk is accepted.

## Exact conditions

**Pre-exec**

1. **Record the selection as a new file.** It must reference the frozen packet (`d1a435c5…`, 26,884 B) and this delta (`1c555626…`, 7,403 B).
   - Do not edit the original eleven-operation packet in place.
   - Do not touch or relabel the consumed `actual-eight-01` receipt.
2. **Check the argv diff.** Comparing the selected op01 argv positionally with the frozen op01 argv must show exactly two differing tokens, at index 2 (`120s`) and index 4 (`--cpu=90:90`). The only other change is `wallSeconds` = 120.
   - If the caller enforces its own deadline from `wallSeconds`, it must be updated together with the argv. It must not be shorter than the `timeout` token.
   - Environment, cwd, stdin=DEVNULL and `keptBytesPerStream` stay identical.
3. **Retain the caller first.** Root retains and pins the exact caller before exec, including the appendix if it is selected.
4. **Use fresh directories.** `revision02/prospective-ordinary/{build,tmp,output}` must be fresh and empty, `TMPDIR` must exist, and both output paths must be absent.
   - The failed attempt's `core.d` is at a different path (`public-nominal-text-compile-candidate-…/build/core.d`). It stays unread, unhashed and unconsumed.
5. **Pin the source inputs.** Pin all 176 Core files and both manifests before exec and again after exit. Any mismatch voids the attempt, and it is not retried.
6. **Pin the tools.** Re-hash the 133 accepted pins before and after the attempt, at least every file on the argv and every file in both `-L` directories.
   - Directory membership of both `-L dependency=` directories must be identical before and after, with no new candidate files.
   - The accepted closure and provenance are reused, and no sysroot or loader audit is added.

**Controller and failure classification**

7. **Stop at the first failure.** Any outcome other than exit 0 consumes the attempt and stops the serial packet. Partials are frozen as unhashed non-artifacts.
   - No retry, prefix replay or further cap raise without a fresh, separately owned disposition.
8. **Do not assume the controller from the exit code.** A raw −9 alone does not say which limit fired. With soft = hard, an RLIMIT_CPU kill also arrives as SIGKILL, and the `timeout` group-kill shows up as −9 too.
   - Record "wall deadline" only if elapsed ≥ 120 s and the caller's controller fired.
   - Any −9 earlier than that is recorded as **an unattributed kill before wall**. CPU is likely, but it is not established.
   - Record −25/SIGXFSZ, abort or allocation failure as such. Do not infer a root cause.
9. **CPU may bind before wall.** rustc codegen threads count toward process RLIMIT_CPU, so the 90 s CPU cap can fire before the 120 s wall. That is accepted.

**Success and output**

10. **Success criteria.** Success requires exit 0 and `full_stream_retained: true` on both streams.
11. **Pin before any consumer.** Pin the rlib and `core.d` (path, bytes, sha256, mode 0400, fsync) before op02 or op09.
    - Record the rlib size against 67,108,864. For reference only: historical 51,751,002 B, headroom 15,357,862 B.
    - No linker process is expected for `--crate-type=rlib`, so the `-C linker`/`link-arg` flags should be inert.
12. **`core.d` determines membership after success.**
    - Every source entry must be in the pinned 176-file inventory. Any entry outside it means stop.
    - It must include the six revision02-relevant sources: `compile_artifact.rs`, `ext_compile.rs`, `draft_contract.rs`, `nominal_command.rs`, `wire.rs` at `4421605b…`, and `intent.rs`.
    - The three changed docs must not appear as inputs.

## RUSAGE appendix (`bfe451dd…`): **qualified as a passive diagnostic only, if Root selects it**

**Limitations accepted as written**

- The child-accounting delta may be incomplete after a group kill.
- `ru_maxrss` is a raw cumulative high-water value. It is also RSS, not address space, so it says **nothing about AS-cap headroom**.

**Additional qualifications**

- **Pass/fail:** it must not affect pass/fail, controller classification or any cap.
- **Bracket contents:** the bracket must contain no other reaped children. If `existing_capture` spawns any other child, such as a hashing subprocess, the delta is contaminated and must be labeled that way.
  - On the exit-0 path, `prlimit` execs into rustc and `timeout` reaps it, so the delta should cover rustc. This is static reasoning, not a proof.
- **`finally` block hazards:**
  - `receipt.get(...)` requires the receipt to be a mapping, like the shown RESULT.json dicts.
  - `phase / ...` requires `phase` to be a `pathlib.Path`.
  - Otherwise an exception in `finally` would replace the returned receipt or the original error. Confirm both against the retained caller before selection.
  - A failure in `durable_put` would likewise mask the capture outcome. Record that as a caller failure, not a compile result.
- **Current attempt:** it has no measurement. Any later numbers are observations of this one attempt only.

## Preserved and unchanged

- **op02–op11 sequencing:** run them only after a pinned op01 success, with exact five-test listing.
- **Full capture and freeze:** full raw capture, then the freeze of all 23 production captures, then the Root-frozen wire→snapshot mapping derived from source, all **before** any protected expected read.
- **Test coverage gap:** mutual-generic preparation and plan output still has no independent oracle and no test assertion.
- **Not accepted by this disposition:** semantics, body typing, native, SDK or the CLI backend.
- **Separate scope:** C1/C2 citations, the documentation addendum and the 61762 source approval are not decided here.
- **Not caps decided here:** native, SDK, Am10, T3 and k17 global caps, and any debit routing.

**Foreseeable, not gating:** op02 and op09 link against the new, larger rlib at unchanged 30 s / 25 CPU / 64 MiB caps. If either one fails on resources, the serial fail-stop handles it. A cap change for them would need its own separately owned disposition.