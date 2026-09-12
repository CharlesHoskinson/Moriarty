# Source audit: BLOCK

Fresh exact-candidate audit of the repaired one-case SP03.4 local-runtime diagnostic adapter. All three frozen hashes match before and after. Source approval, dispatch approval and actual-result approval are withheld. No K or strace ran.

## A03: Trace parsing still invents interpreter signal attribution (blocking)

Only exactly one space after pid is accepted; other lines collapse to synthetic PID0 and are searched without trace grammar anchoring. Padded PID10 interpreter exit0 plus padded PID20 parser SIGSEGV reports interpreter SIGSEGV PID0. Ordinary debug execve text plus child killed-by prose does the same. A successful interpreter exec followed by successful parser exec at same PID still attributes the later death to interpreter. Basename-only matching does not bind the frozen interpreter path.

Required repair: Parse valid strace records with padded IDs; keep unprefixed/root ambiguity explicit, enforce exact selected executable identity and current successful exec image per PID, and never promote ordinary child stderr or ambiguous trace into interpreter evidence. Handle unfinished/resumed records conservatively.

## A05: Selected transitive tool closure remains incomplete before K (blocking)

Actual verify-only checks535 entries but omits lib/kframework/k, setenv, checkJava, Java executable and selected Python3.11 interpreter. kast-unwrapped invokes k, which sources setenv/checkJava and executes Java; pinned jar does not bind those selectors. Parser shebang is /usr/bin/env python3; wrapper PATH selects Nix Python3.11, not pinned adapter Python3.14. krun invokes llvm-krun to construct the config even with --no-expand-macros, and conditionally kore-print after interpreter. These selected script paths are absent from preflight; inherited PATH utility selections also remain uncommitted. See closure JSON for exact current bytes.

Required repair: Follow actual selected scripts and PATH/shebang resolution through the complete bounded command, hash/recheck selected tools/config before spawn or establish a reviewed immutable closure. Include concrete llvm-krun, kast Java loader/setenv/checkJava/Java, parser Python, applicable kore-print and utility selection; do not equate a wrapper hash with its transitive closure.

## A04: Cleanup completion overclaims descendant termination (blocking)

Bounded detached Python grandchild remains alive and writes its marker after adapter returns in0.102seconds, but cleanupIncomplete=false. Parent reaping alone sets this field; original-group kill cannot establish descendant cleanup. Timeout bounds are now finite, yet cleanup/evidence unknowns are not accurately recorded. On unexpected select/read errors the finally block only closes streams; no unconditional child termination/reap is attempted.

Required repair: Keep finite cleanup, but explicitly report group-only scope and descendant cleanup unknown/incomplete unless strong containment has supplied completion evidence. Ensure exceptional capture exits attempt bounded termination/reaping and retain partial evidence. No extra launcher is needed.

## A01: Disk manifest disagrees with emitted retention status (medium)

Exact injected binary stderr including native_stack_site recovered byte-for-byte from temporary output. However manifest is written before rawRetentionComplete is finalized, so disk manifest permanently saysfalse on a successful observation while returned/emitted JSON saystrue. Raw byte hashes on overflow describe all drained bytes rather than retained prefix and have no separately named retained-file digest in manifest.

Required repair: Finalize and persist a consistent observation manifest after verifying raw writes, with separately named retained-byte hashes/lengths when truncated. Keep retention failure fail-closed.

## A06: Observed K count confuses trace root identity and exec chain (medium)

Synthetic unprefixed successful /bin/krun followed by same-chain prefixed /bin-unwrapped/krun yields observedKrunExecs2, counting synthetic PID0 and real root independently. Conversely resumed successful exec cannot be recognized. This is not a reliable exact invocation count. Attempted spawn is now separately labeled correctly for returned launcher failures, but a Popen exception escapes with no attempted observation record.

Required repair: Separate attempted invocation, successful selected wrapper exec events, process identity and unknown counts; reconcile root transition only with evidence, never assume unknown equals zero or a second K invocation. Retain spawn-error result with truthful attempt status.

The 21 author offline tests pass. Fresh independent probes verify exact raw stderr/stack recovery and bounded capture, but reproduce the findings above. See audit-adapter-probes-02.json and audit-adapter-closure-02.json. Existing runner containment and prepaid binding were inspected as source only. Actual admission and result review remain separate. C7/D1/G60, zero compiles/retries, original resource liabilities and all product gates remain unchanged. All-six goal remains open.
