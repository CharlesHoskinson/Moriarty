# Disposition: ELIGIBLE_WITH_SCOPED_NEW_ORDINARY_DISPOSITION (op01 only)

**Reviewer:** Opus 5.5 HIGH, independent, non-author.

**Method:** I read only the supplied text. I ran no commands, Git, compiler, provider, native tool or SDK, and opened no expected bodies. Everything below is static reading. This disposition is a proposal only. It does not select or authorize any operation; Root selects separately.

## Basis

**What the receipts establish:**
- The historical old-source normal build exited 0 in 7.24 s wall under 30 s / 25 CPU / 2 GiB / 64 MiB.
- Its rlib is 51,751,002 B, leaving 15,357,862 B of FSIZE headroom. The current pin still matches.
- The consumed subset attempt ended with controller `wall deadline`, raw exit −9, at 30.0038 s. Its streams are incomplete and it produced no rlib.

**What the receipts do not establish:**
- Current CPU, peak virtual memory, rlib size, or the cause of the timeout.
- The only recorded terminator is the wall deadline. The 25 s CPU cap was not the recorded terminator, but no CPU quantity follows from that, and none is used here.

**Why the delta is responsive:**
- The prior review required a changed disposition grounded in actual receipts. The only actual controller observation is the wall deadline.
- Raising wall and CPU, and nothing else, changes exactly the bound that fired.
- AS and FSIZE stay unchanged. Their refusal mechanisms are immediate failures (allocation abort, SIGXFSZ), not stalls. Keeping them is therefore not in tension with a wall-deadline receipt. This is a mechanism note, not a cause finding.

**Accepted unknowns:** The owner explicitly accepts these as one bounded, fail-closed attempt:
- Current CPU, wall, and peak AS against 2 GiB.
- Current rlib size against 64 MiB. Historical headroom is about 23% of the cap, and the current size is unknown.
- Whether 120 s / 90 CPU suffices at all.

The worst case is one more consumed, frozen failure receipt. No current-fit claim is made, and no subject-prefix replay is required.

**Effective budget:** `RLIMIT_CPU` counts all of rustc's threads.
- A CPU-bound single-threaded compile meets the 90 s CPU cap before the 120 s wall.
- A multi-threaded codegen meets it earlier still.
- The 120 s wall is reachable only if rustc is off-CPU for at least 30 s.

Record and read the attempt as "≤90 CPU-s within ≤120 wall-s", not "120 s of compile".

## Conditions

### A. Selection record (scope)
1. The selected op01 must equal the frozen op01 in `PROPOSED-ELEVEN-OPERATIONS.json` (sha `d1a435c5…`), except for exactly:
   - argv[2] `30s` → `120s`
   - argv[4] `--cpu=25:25` → `--cpu=90:90`
   - caller `wallSeconds` 30 → 120
   - the status field

   Each of these tokens occurs once, so the replacement is unambiguous. Root must store a token diff showing nothing else changed.
2. Unchanged items:
   - `--as=2147483648:2147483648`, `--fsize=67108864:67108864`, `--core=0:0`
   - wrapper order: `timeout` outermost
   - env {PATH, LANG, TMPDIR} only, stdin DEVNULL, `keptBytesPerStream` 131072
   - `-C metadata=moriarty_nominal_command_20261019_v1`
   - the output paths
3. The frozen eleven-operation file, the consumed actual-eight-01 receipts, and its partial `core.d` (non-artifact) are not modified, relabelled, read for content, or debited against.
4. Native, SDK, Am10, T3 and k17 budgets and global caps are untouched.
5. The new attempt's RESULT names its own cap set (120/90) so that it cannot be confused with the frozen original.

### B. Before exec
1. Root writes the retained caller, including the appendix if selected, to a pinned path before exec.
2. `prospective-ordinary/` and its `build/`, `tmp/` and `output/` subdirectories are freshly created and empty. Both the rlib and `core.d` targets are absent.
3. Re-pin and match all 176 Core inventory files and the 22 approved files, including `wire.rs` at `4421605b…`, plus the two manifests.
4. Re-pin and match all 133 tool/extern pins, including the rustc wrapper, `timeout`, `prlimit`, the six `--extern` rlibs, both `-L` directories' candidates, and the `serde_derive`, `thiserror_impl` and `soa_rs_derive` dylibs. Any mismatch means stop without exec.
5. Reuse the existing closure and provenance acceptance as stated. The 133 pins do not themselves cover sysroot `std` or `librustc_driver`, so record that reliance as inherited rather than re-audited.
6. The environment is a genuinely clean `env -i`-equivalent, with no leaked `CARGO_*` or `RUSTFLAGS` variables.

### C. Fail-stop
1. Allow one attempt with no retry, no automatic escalation, and no prefix replay.
2. Any of the following is a failure:
   - nonzero raw exit
   - controller firing
   - incomplete stream
   - missing output
   - post-pin mismatch
3. On failure, ops 02–11 do not run. Freeze the receipts. Anything left in `build/` or `tmp/` is an unhashed, unconsumed non-artifact, enumerated by name and size only.
4. Record the raw exit, elapsed time and controller label exactly as captured. Do not relabel or diagnose a cause beyond which controller fired.
5. A second failure is not by itself grounds for a further cap raise. Any new attempt needs a fresh owner disposition.

### D. Success and outputs
1. Success requires exit 0 and `full_stream_retained: true` on both streams.
2. Post-pin all sources and tools again. Any drift invalidates the result.
3. Before any consumer runs, pin the bytes and sha256 of the rlib and `core.d`, then fsync them and set mode 0400. Record the rlib's FSIZE headroom.
4. `build/` must contain no crate-candidate file (`.rlib`, `.rmeta`, `.so`) other than the named rlib, because `build/` is on `-L` for op02 and op09.
5. `core.d` membership is determined only from this new file. Every listed path must lie within the pinned 176-file inventory or the pinned extern set, and its hash must match.
6. These files must be present with matching pins:
   - `compile_artifact.rs`
   - `ext_compile.rs`
   - `draft_contract.rs`
   - `nominal_command.rs`
   - `wire.rs`
   - `intent.rs`
   - the bounded parser consumers
7. Record `env-dep` lines verbatim.

### E. RUSAGE appendix: qualified as passive, eligible only if Root selects it
1. It must bracket only the op01 exec.
   - Pin checks and hashing performed via subprocesses must fall outside the bracket.
   - If `existing_capture` spawns anything besides the argv, record that the delta includes it.
2. The BEFORE and AFTER files go to the receipt phase directory, never into `build/`, `tmp/` or `output/`. A failure to write BEFORE before exec means stop with no exec.
3. `existing_capture` must return the mapping that RESULT.json is serialized from. Otherwise `receipt.get(...)` raises inside `finally`, masks the outcome and drops the AFTER record. An appendix fault never changes the operation's pass/fail.
4. **Wall-kill case:** this case is uninformative. The −9 receipt is consistent with `timeout --signal=KILL` group-killing itself before reaping rustc, so rustc's usage is not charged to the caller. Record the delta as "incomplete; not process-tree CPU".
5. **Exit 0 or a rustc CPU-cap death:** `timeout` reaps rustc, so the delta includes rustc's all-thread CPU. It is still a waited-child observation, not a tree proof.
6. **`ru_maxrss`:**
   - It is a high-water mark in KiB across all children the caller has ever waited for.
   - If AFTER equals BEFORE, op01's peak is unknown and at most BEFORE.
   - It is RSS, not address space, so it says nothing about `--as` headroom.
7. The measurement applies to this attempt only. It does not retroactively qualify the consumed attempt, which stays UNMEASURED, and supports no cause inference.

### F. Downstream ops 02–11
1. Caps stay unchanged as proposed.
2. op02 and op09 are rustc builds against an rlib of about 50 MB or more under 30 s / 25 CPU / 64 MiB FSIZE, and op02 and op09 are linked executables. The owner should knowingly accept that a fail-stop there is possible.
3. op01's success data must not be used to relabel any later cap without a fresh disposition.
4. All prior P2–P8 conditions stand unchanged:
   - C1 and C2 citations
   - the exact five-test listing, with each selected test at 1 passed / 0 failed / 0 ignored / 0 measured / 4 filtered
   - full raw capture of all 23 production `program_view` fields and freezing them before any protected expected read
   - a Root source-derived wire→snapshot normalization mapping, frozen before comparison
   - the mutual-generic `preparation` and plan output recorded as an open gap with no independent oracle or test assertion

## Preserved limitations
- No semantic, body-typing, native, backend-CLI, Cargo or DoctorWatson acceptance.
- No old-profile artifact byte compatibility.
- The 176-file inventory is not a compile-cost measurement.
- Historical CPU and VM remain unmeasured.
- Source approval 61762 and the docs/citation addendum are separate from this resource disposition and are not affected by it.

**No operation is authorized by this response.**