# Opus 5.5 security-owner review: k_compile_attempt_03.py (Sol-authored, four bounded corrections)

**Requested identity:** claude-opus-5-5, high effort, designated security owner. **Returned identity:** claude-opus-5-5.

## Verdict: APPROVE_WITH_SCOPE (conditional)

This approval binds only to the driver bytes with sha256 `05086da9bc2dc5f3f9d89f0f6734a57b57e408f9fef934351b990edc013011a3` (28943 bytes), as reviewed here.

- I cannot compute hashes without tools. I reviewed the supplied text and diff. The driver hashes itself in `execute()`, which mechanically enforces the binding.
- If `--preflight-only` prints any other `driver_sha256`, this approval is void.

The approval takes effect only when every gate listed below holds. The prior Opus correction refusal, the original approval bound to `a499067f…43e8`, and the original failed preflight remain historical and unchanged. This is a fresh review of a new artifact, not a regeneration of withheld material.

## Scope check: the diff stays within the four categories

I compared every diff hunk against the full _03 and _01 texts. No change falls outside the four authorized categories, and nothing is silently broadened.

**1. Exact source pins: accepted.**
- `k_capture_observations.py` is now pinned to `2f9d…f8bd`/11336 and `test_k_capture_observations.py` to `e4b7…8767`/5866. Both match the values observed by the historical child-free preflight and the handoff.
- `main.k`, `k_observations.py`, `k_terms.py`, `host_observations.py` and the four oracles are unchanged.
- `check.py` is now pinned to `7a6d…12e0`/36429. My additive review of it is below.
- The driver only hashes these files. It never imports or executes them, so updating the pins changes identity binding only. It does not change execution surface.

**2. Observed installed launcher chain: accepted.**
- `K_LIBRARY = K_ROOT + '/lib/kframework/k'` is read through `read_store`, so it must resolve inside the store. It is hashed into `packaging_evidence`, and an unreadable file is a blocker.
- The shim markers are now `../lib/kframework/k` and `-kompile`. Both appear in the supplied 117-byte shim.
- `org.kframework.main.Main` is now required in the library script. It is present in `eval "$JAVA" org.kframework.main.Main '"$@"'`.
- `K_LIBRARY` is added to `inventories()`, so it is covered by the pre/postflight comparison.
- The wrapper check, which requires the JRE plus the four tool roots, is unchanged. The supplied wrapper text contains all five.
- `texts['kompile-library']` is always assigned, so there is no `KeyError` path.

**3. Isolated compile-input directory: accepted.**
- `CANDIDATE_ROOT` now points to `…/k-actual-01/compile-input`. The tree shape gate, the exactly-`main.k` check, the symlink check, the hash check, the creation mode (0o700 directories, 0o444 file, `O_EXCL|O_NOFOLLOW`) and prelaunch/postflight `candidate_problem()` are all unchanged.
- The `/candidate` audit snapshot is no longer referenced anywhere in the driver: not read, written, created or deleted.
- Neither `candidate` nor `compile-input` matches the `attempt-` prefix, so the no-retry detection is unaffected.

**4. Python HOME symbol cleanup: accepted.**
- All four uses are renamed to `K_HOME`: the constant, `-Duser.home`, the env `'HOME'` value, and `os.mkdir` in `run_child`. No bare `HOME` symbol remains.
- The child environment key is still `'HOME'` and its value is the same path, so behavior is identical.

**Confirmed unchanged:**
- **Limits:** `CHILD_SECONDS=30`, `SWEEP_SECONDS=3`, 512 MiB `RLIMIT_FSIZE`, `RLIMIT_CORE=0`, umask 077, and JVM flags `-Xms64m -Xmx512m -Xss32m`, `ActiveProcessorCount=2`, `ExitOnOutOfMemoryError`, `-UsePerfData`, `DisableAttachMechanism`, `ErrorFile`.
- **Environment:** the minimal four-root PATH with no host `/usr/bin` or `/bin`.
- **Attempt semantics:** `ATTEMPT_NAME=attempt-compile-01`, the exclusive `mkdir` claim, the append-only ledger claim/finish records, the prior-attempt and ledger refusal, and the no-retry, never-reset terminal semantics.
- **Process control:** new session, group SIGKILL at deadline and after exit, the token-tagged `/proc` sweep and the containment-breach flag.
- **Postflight and gates:** re-hash of sources, primary sources and inventory; the attempt-tree inventory; receipt and ledger persistence; the root, `-I` and Linux gates.
- **Absent capabilities:** no network, no install, no engine invocation, and no decoder import.

## Additive review: check.py `7a6d…12e0`

The changes add a K layer to `LAYER_ARTIFACTS`, a `--k-observations` mutually exclusive comparison mode, a K branch in `layer_raw_context`, and a K entry in `supplied_comparison_gaps`.

- The file still uses no subprocess, network or engine.
- The K branch selects runs only by exact `str` case and exact `int` fuel. Supplied text is retained raw and is never decoded.
- `check.py` does not import `k_capture_observations`.

**I accept the pin.**

One finding is non-blocking for this compile and goes to the separate R-only guard track: `compare_supplied_observations` requires the K artifact to *have* `comparison_ready` but never gates on its *value*. A K `match` could therefore be reported for a bundle whose own `comparison_ready` is false. No K comparison result may be relied on until this is closed together with the outstanding adapter findings A1, A2, A3, A6 and A8.

## Gates before the first and only execute

1. **Muse concurs independently** on these exact bytes. Muse concurrence does not substitute for this review, and this review does not substitute for Muse.
2. **The exact read-only preflight passes** with zero blockers:
   `/usr/bin/python3 -I -B …/k-actual-01/k_compile_attempt_03.py --preflight-only`
   Its output must show:
   - `driver_sha256` equal to `05086da9…11a3`
   - `runtime_state.candidate_exists=false`, with no `attempt-*` entries and no ledger
   - `packaging_evidence` with hashes `386e0502…3632` (wrapper), `f5fa362e…1c0d` (unwrapped shim) and `d56ba8f4…a66c` (library), matching the handoff observations
   - an interpreter sha256 of `52e0a13e…cb41`, matching the historical preflight; a different value is a stop-and-report finding
3. **Snapshot preservation is checked by the operator, read-only.** Immediately before execute, record a read-only hash inventory of the `/candidate` audit snapshot. Compare it again after the attempt. The driver does not inventory that tree, and the child has no filesystem namespace. Any difference is a breach finding, never a retry trigger.
4. **The independent Core lane has released the ordinary compile slot.**
5. **Then the exact command runs once:**
   `/usr/bin/python3 -I -B …/k-actual-01/k_compile_attempt_03.py --execute --approved-driver-sha256 05086da9bc2dc5f3f9d89f0f6734a57b57e408f9fef934351b990edc013011a3`
   This is one terminal `attempt-compile-01`, whatever its outcome. A refused preflight consumes nothing. Once the claim `mkdir` succeeds, the attempt is consumed: a crash, timeout, OOM or nonzero exit is a terminal observation, and any follow-up needs a new, separately approved attempt.

**Not covered:** any engine, krun, kparse, kore-exec, kore-print or llvm-krun run; any second compile; installs or fetches; SDK, native, ledger, circuit, proof or key work; counter resets; the 216-input target; and any certification of outputs. Compiler outputs are observations only, not semantics evidence.

## Non-blocking notes (no change requested; changing them would alter the approved hash)

- **Stale docstring.** It still names `k_compile_attempt_01.py` and says "Opus-authored". Provenance for the _03 bytes is Sol-authored and Opus-audited, as recorded in COMPILE-CANDIDATE-03.json. Do not rely on the docstring for provenance.
- **Weak marker check.** The `../lib/kframework/k` check is a substring test. The recorded hashes are the stronger evidence, so compare them in gate 2.
- **Source path for future engine work (Muse finding 2).** Kore `SourceLocation` will name `…/k-actual-01/compile-input/models/sherlock/k/main.k`. Any future `source_path` must be that exact observed path, read from `definition.kore`, not `/candidate/…` and not the test's `/fixed/candidate/main.k`.
- **Bypassed wrapper environment (residual, already accepted).** Direct Java bypasses the library's `ulimit -s` raise and setenv's `native/linux64` additions to PATH and LD_LIBRARY_PATH. Any resulting failure, for example a native-library load, is a visible terminal observation, not a containment issue.

## What Muse must independently agree on

1. Each of the four corrections stays within its category, with no broadening.
2. The `check.py` additive pin, including the unconsumed `comparison_ready` finding.
3. Moving the compile input to `compile-input` preserves both the no-retry semantics and the `/candidate` snapshot, together with operator gate 3.
4. The launcher-chain gate (wrapper → shim → library → `Main`) adequately evidences the direct-JVM equivalence decision.
5. Approval binds the printed driver sha256.
