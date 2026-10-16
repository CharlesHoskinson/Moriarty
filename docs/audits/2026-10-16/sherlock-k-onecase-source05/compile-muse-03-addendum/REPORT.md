## APPROVE_WITH_SCOPE — Muse concurrence addendum only

Top finding: I agree with the fresh Opus report on all five items, including its new operator snapshot condition, for the exact unchanged bytes. No `CHANGES_REQUIRED` concern was found within compile-only scope.

This concurrence binds only to `k_compile_attempt_03.py`, 28943 bytes, SHA-256 `05086da9bc2dc5f3f9d89f0f6734a57b57e408f9fef934351b990edc013011a3`. Requested and returned reviewer identity is `muse-wonder-1.0`, high effort. Security scope remains Opus's decision; this concurrence does not substitute for it. Current compile remains unexecuted.

### Assessment of Opus's five items

1. **Four corrections, no broadening — AGREE.**  
   The complete diff is confined to: updated capture-adapter and `check.py` pins; `K_LIBRARY` and the corrected launcher markers; `CANDIDATE_ROOT` moving to `compile-input`; and `HOME` renamed to `K_HOME` without changing the child `HOME` value or path. Limits, minimal environment, direct JVM command, one-attempt claim, ledger refusal, process-group kill, token sweep, hash postflight, and absence of engine/decoder execution are unchanged.

2. **`check.py` additive pin and unconsumed `comparison_ready` — AGREE on the pin and factual finding; reliance conflict expressly preserved.**  
   I accept pin `7a6d1bbca881f5c2b012278989dac600834d610986694e640da51a40b5a412e0` / 36429 bytes for compile-only identity binding. The driver only hashes `check.py`; it neither imports nor executes it or any decoder.  
   
   Opus is factually correct that `compare_supplied_observations` requires `comparison_ready` to be present but does not gate on its value. Root's separate statement also remains preserved: prior consumer review intentionally approved a projection-only match even when `comparison_ready` is false, with verbatim limitations published. I do not resolve that security-scope/reliance conflict here. This concurrence takes no reliance on `comparison_ready`, any K `match`, actual K semantics, or comparison results, and authorizes none. That unresolved reliance question therefore does not authorize semantics work, but also does not block this non-relying compile pin.

3. **Compile-input move, no-retry semantics, and snapshot preservation — AGREE, with operator gate 3 required.**  
   Moving `CANDIDATE_ROOT` to `…/k-actual-01/compile-input` preserves the exclusive `attempt-compile-01` claim, prior-attempt and ledger refusal, and terminal no-retry semantics. Neither `compile-input` nor `candidate` is an `attempt-*` entry. The driver no longer reads, creates, writes, or deletes `…/candidate`.  
   
   Important clarification: preflight `runtime_state.candidate_exists=false` now concerns **compile-input**, not the `/candidate` audit snapshot. It therefore does not by itself prove the audit snapshot exists or remained unchanged. The operator must, immediately before `--execute`, record a complete read-only file/hash inventory of `/candidate`, preserve that inventory, and compare it again after the terminal attempt. Any difference is a breach finding and never a retry trigger. This is an operator read-only condition, not a driver mutation, resource change, or alteration of the approved hash.

4. **Launcher-chain gate — AGREE.**  
   The gate adequately evidences the direct-JVM entrypoint decision: wrapper → unwrapped shim containing `../lib/kframework/k` and `-kompile` → library script containing `org.kframework.main.Main` → direct invocation of `org.kframework.main.Main -kompile`. `K_LIBRARY` is also included in pre/post inventories. This evidences the intended entrypoint chain, not identical wrapper environment behavior. The accepted residual remains: bypassing `setenv` also bypasses its `ulimit -s` raise and native `PATH`/`LD_LIBRARY_PATH` additions; resulting failure is a terminal observation, not expanded authority.

5. **Approval binds printed driver SHA — AGREE.**  
   If `--preflight-only` prints any `driver_sha256` other than `05086da9bc2dc5f3f9d89f0f6734a57b57e408f9fef934351b990edc013011a3`, both Opus approval and this concurrence are void for those bytes. `execute()` independently self-hashes and compares against `--approved-driver-sha256`.

### Mandatory gates before the sole execute

- Exact read-only preflight must pass with zero blockers using `/usr/bin/python3 -I -B …/k_compile_attempt_03.py --preflight-only`.
- Its output must show `driver_sha256=05086da9…11a3`.
- `runtime_state.candidate_exists=false`; no `attempt-*` entries; no `k-attempt-ledger.jsonl`.
- Interpreter SHA-256 must equal `52e0a13e60a981d8c4b6478be2ba5176f69da07948a056bf49cf6f077e30cb41`.
- Packaging hashes must equal:
  - wrapper: `386e0502d63a10a6b0e9ddb46ef842a97be7bc61c4d4149ce66f51b3f3548632`
  - unwrapped shim: `f5fa362e4d604e18c79409db645154ad32c07e59fa9696d3374086fef4631c0d`
  - library: `d56ba8f42a320cab88708456ec2721ab71b0d49e8b1bad42d47b497b34fda66c`
- Operator completes the before/after `/candidate` snapshot inventory described above.
- The independent Core lane first releases the ordinary compile slot.
- Only then may the exact `--execute --approved-driver-sha256 05086da9…11a3` command run once. Once the claim succeeds, every outcome is terminal; no retry or reset is authorized.

Non-blocking notes are also accepted: stale docstring provenance, substring marker weakness subordinate to the recorded hashes, future `source_path` must be the observed `…/compile-input/models/sherlock/k/main.k` path from `definition.kore`, and compiler outputs remain observations only—not semantics or comparison evidence.
