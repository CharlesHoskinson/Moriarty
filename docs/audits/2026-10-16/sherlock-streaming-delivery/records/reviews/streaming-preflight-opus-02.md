# APPROVE_WITH_SCOPE

This is a static exact-candidate confirmation for manifest `73fb5864bbb05e7e0d8d1949fb7bc7250c3e1c02d7bb7b3abffbe118cb94a4b1`. Nothing was compiled or executed.

## Diff confirmation

**Sole change.** I compared all 28 manifest rows against the previous manifest (`c08444d1…3a823`). Only `run_streaming.py` changed:

| | sha256 | bytes |
|---|---|---|
| before | `4f785b05…0a1d84` | 3477 |
| after | `7b4a615b…f410c` | 4310 |

The other 27 rows have the same paths, digests and sizes, in the same order. That includes `run_host.py` (`ae5a04da…`), `CONFIG.json`, `streaming_harness.rs`, both stream modules and all four oracle documents.

**Verbatim.** The supplied `run_streaming.py` text matches my replacement line for line. That covers the constants, the single verified read, the CONFIG parse from `contents`, `exec(compile(contents[GUARD], …))`, the claims dictionary including the corrected `product_wiring` text, the 11-field `Realization`, the receipt bindings, the plan, the candidate-postflight step and the `-I -B` gate. I see no independent edits.

**Limitation.** I compared the pasted text. I did not hash the on-disk bytes. That binding comes from the run itself, through the attribution conditions below.

## Disclosure: a defect in my own patch

I found one defect in the patch I supplied.

**Trigger.** If reading a manifest member raises an exception during the post-run re-read, `candidate-postflight` is never recorded. Example: `candidate/AGENTS.md` or `CANDIDATE.json` is deleted mid-run. Four of the 28 manifest members are outside the guard's 24-entry snapshot: `CONFIG.json`, `AGENTS.md`, the product contract, and `run_streaming.py`.

**Effect.** `finish()` still runs, and the guard's own postflight does not cover those four files. So `receipt.json` and `result.json` can say `terminal_state: "success"` while the process exits with a traceback. In that case `reviewed_candidate_after` and `sha256_after` are missing from the receipt.

**Why this is not a blocker.**
- It needs the trusted checkout to be modified during the run. That is already outside the stated trust model.
- It fails closed under the attribution rules below, because those rules require the after-fields and the step record to be present.

**Follow-up.** Fixing it in this round would need another candidate cycle. Fix it in the next revision of the driver: record the post-run reads inside the step, so an exception becomes a failed step.

## Scope approved

**Covered:** one attempt of `/usr/bin/python3 -I -B R/run_streaming.py` against this exact manifest.
- Fixed trusted normal typed ASTs only: N01, N02 and N05, with fuel ≤ 27 and stack 8.
- Exactly the 8 named `reference_stream_tests` tests.
- Steps: preflight, rustc-version, compile-tests, run-tests, candidate-postflight, postflight.
- Fresh exclusive `attempt-01`.
- 30 s wall clock per child, with process-group kill on timeout. Sequential, no retry.

**Result attribution.** The result counts only if all of the following hold:
1. The driver process exits with status 0, with no traceback.
2. `receipt.reviewed_manifest.sha256` and `sha256_after` both equal `73fb5864…a4b1`.
3. `reviewed_candidate == reviewed_candidate_after`, and both equal the manifest digests. This includes `run_streaming.py = 7b4a615b…f410c`.
4. For every snapshot member P, `receipt.sources[P].sha256_before`, `snapshot_sha256_after` and `checkout_sha256_after` equal the manifest digest for `candidate/P`. This binds the bytes actually compiled to the reviewed manifest, not only the driver's own read.
5. Steps `candidate-postflight` and `postflight` are both present with state `ok`.
6. `terminal_state == "success"`.
7. `run-tests.tests` lists exactly the 8 names from CONFIG.
8. `result.json.receipt.sha256` matches `receipt.json`.

Any failure is reported as a failure. There is no rerun under this approval; a second attempt needs a new approval.

**Not covered:**
- Product or native acceptance, or approval of actual results.
- Production `mod.rs`/`lib.rs` wiring, which remains a future root edit and is neither applied nor executed.
- Whole-crate verification.
- Untrusted or network inputs.
- Any memory, disk or process-count bound.
- The sysroot, linker, libc, Python stdlib and kernel, which remain unhashed.
- Read-only snapshot modes, which are not isolation.

H1 remains in force. Native authority stays at zero, and C4, resource limits and counters are unchanged. No K, Quint, Lean, Clean, circuit, native, U, M, MC, SP or G exit is closed.

**Muse reconfirmation still needed.** Muse's nonsecurity approval was pinned to `c08444d1…`. The candidate changed, so a fresh Muse confirmation on `73fb5864…` is still required before execution.
