## Verdict: APPROVE_WITH_SCOPE — Muse independent concurrence only

Top finding: no blocker was found in the exact 53,993-byte driver text supplied.

This is not the separate Opus security approval. Security eligibility remains solely Opus's decision.

- Requested driver: `af9f49b07af4b8d16607fd7cbe01df79ff30302a782084ab502a17dda10e56d2`
- Reviewing identity: Muse Code powered by `muse-wonder-1.0`, high
- Returned identity `I`: not yet in existence. It must come from the actual `--preflight-only` report and then be reviewed separately by Opus and Muse.
- No preflight or engine attempt was run in this review. No attempt is claimed.

### Full-source findings

The driver matches the stated bounded profile:

- Exactly 12 fixed stages: kast, driver-only initializer, depth-0, four depth-1 steps, then five kprint observations.
- Exactly one frozen input: `N01-fuel-001.ui`, 43 bytes, pinned hash.
- No recompile, kparse, krun, llvm-krun, SDK, native Moriarty work, decoder, adapter, comparison, label inference, install, fetch, or network operation is present.
- Depth, stage order, commands, and products never branch on state content or on the expected locations 74/102/80. Those locations appear only in the non-branching `EXPECTATIONS` receipt text.
- Each child receives a minimal stage-specific environment, `/dev/null` stdin, a new session, a 30-second wall limit, SIGKILL to the group, a 3-second token sweep, `RLIMIT_CORE=0`, `RLIMIT_FSIZE=512 MiB`, and umask `077`.
- Unexpected stage files, scratch outside `home`/`tmp`, protected-tree changes, captured-artifact changes, nonzero exits, signals, timeouts, token escapes, and containment breaches all stop the attempt.
- `--execute` refuses before claiming anything if the approved driver hash, a blocker-free fresh preflight, or the approved identity does not match. Once `attempt-engine-N01-01` is created, the driver provides no retry, reset, cleanup, or alternate method.

### New primary-source facts

**Main dispatch — closed.** The supplied `Main.java` contains an exact `case "-kast"` dispatch to `KastFrontEnd.getModules()`. The driver's direct invocation of `org.kframework.main.Main -kast` therefore matches the actual dispatch. The former shim-only uncertainty is no longer a factual gap for this command, although failure at stage 01 would still be visible and terminal.

**Log entries — closed.** The supplied `Kore/Log/Registry.hs` registers both `DebugAppliedRewriteRules` and `InfoExecDepth`. Those are exactly the driver's two `--log-entries` values. No applied-rule label is inferred by the driver. Raw stderr is captured independently of log-entry selection.

**Threaded RTS defaults — factual agreement; Opus decides eligibility.** The supplied `kore.cabal` confirms the threaded executable defaults are `-threaded -rtsopts "-with-rtsopts=-N -T -A32M -qn4"`. Thus `-N` can use all available cores. The driver adds no `+RTS` option and sets neither `GHCRTS` nor `KORE_EXEC_OPTS`. Its stage records state `memory_cpu='tool defaults ... only the JVM is bounded'`.

I therefore agree there is no silent cap increase in the driver's text: the all-core possibility is the installed tool default and is recorded. Whether that remains within ordinary-scope eligibility is an Opus security decision. The new cabal fact matches Opus's previously admitted “possibly `-N`” residual rather than contradicting it.

### Supplemental definition-report note errors

Neither note error blocks this bounded raw-capture driver:

1. **Receipt/main hash confusion:** `946c567c…` is the compile receipt hash, not the `main.k` hash. The driver correctly keeps them separate:
   - `MAIN_SHA256 = b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41`
   - `COMPILE_RECEIPT_SHA256 = 946c567c291edf13524a3641d14c7811e855b44073475f6770c9bb0e81399f69`

   The erroneous wording in the supplemental report therefore does not propagate into the driver.

2. **Residual `Lbl'Stop'Map`:** expecting no residual `Lbl'Stop'Map` is not factually compatible with the declared empty environment. The definition audit identifies `initEnvCell` as producing `<env>(Lbl'Stop'Map{}())`, the MAP unit. The driver does not assert absence, presence, or any semantic interpretation of that term; it retains the full raw Kore state. No additional generated context is needed for this bounded diagnostic. Pretty kprint output remains observation only.

### Other exact facts

- The binary closure record is acceptable for this scope as a bound record. The driver itself pins the kore-exec hash, checks ELF magic, records advisory store references and inventories in the identity, and invokes the binaries directly. Missing dynamic dependencies would surface as a visible child failure and stop the attempt. The driver contains no repair, installation, or fetch path. Any `false` value in the preflight `native_refs` must be treated as a preflight review finding before execution.
- I found no HEAD/commit guard in the audited driver. It binds content through exact source-file hashes instead. I have not silently repinned anything. The stated docs-only Root HEAD delta does not alter those content pins.
- The initializer binds only `$PGM` under the compiled `LblinitGeneratedTopCell`, using literal markers checked against the pinned definition. It writes no cell, default, rule, or label by hand.
- Source 04 and the decoder are not imported or invoked by this driver.

### Commands approved only in sequence

Approved as the next read-only command, and only that command at this stage:

```bash
/usr/bin/python3 -I -B /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/k_engine_attempt_n01_01.py --preflight-only
```

The execute form is source-correct only after the actual preflight output has zero blockers, Opus and Muse both review and approve its returned `driver_sha256` and `identity_sha256`, and Root has coordinated the ordinary slot and runtime freeze:

```bash
/usr/bin/python3 -I -B /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/k_engine_attempt_n01_01.py --execute --approved-driver-sha256 af9f49b07af4b8d16607fd7cbe01df79ff30302a782084ab502a17dda10e56d2 --approved-identity-sha256 I
```

`I` must be replaced only with the identity returned by that reviewed preflight. Neither hash approval nor a successful attempt would certify K semantics, labels, comparison readiness, or any later reliance.
