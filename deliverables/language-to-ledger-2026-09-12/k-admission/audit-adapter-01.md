# Diagnostic adapter source audit

Verdict: **BLOCKED_PENDING_SAME_AUTHOR_REPAIR**. No source or dispatch approval.

Exact frozen three-file diagnostic adapter and actual retained binding/tools/artifacts plus existing runner source and approved resource/routing contract; not lifecycle K implementation.

Reviewer: fresh `/root/k_adapter_audit`, requested GPT-6 Astra medium. No separate provider identity/effort attestation was available. Candidate files were not edited.

Exact frozen files:

- `.moriarty-dev/k-macro05-trace106-diagnostic.py`: `de02a76e59089aa2fc129e26449f4876dc7e02c7e474c461cc1d160011242b3b` (12723 bytes).
- `.moriarty-dev/test_k_macro05_trace106_diagnostic.py`: `9b2527938280da15370f77f96845d4c1b696132e28849a0dce08767acaa4aa88` (14387 bytes).
- `.moriarty-dev/k-macro05-trace106-pins.json`: `b2ae44c64b6711a367af235ab6524cb2ee4b10f4bd55bd8a9dbb1603b3f2140e` (6286 bytes).

All three hashes match. The inspected closure manifest is `audit-adapter-closure-01.json`; configured retained checks pass (519 checks, including488 artifact checks). These are hash checks, not proof of runtime behavior.

## A01: Native stderr and stack evidence are discarded (blocking)

Only adapter JSON reaches stderr. No raw native stderr file is written; processEvidence.sample retains at most32 krun-containing lines. Timeout/overflow stdout is also not forwarded or persisted. There is no output-destination preflight or durable raw retention. Existing runner can only retain what adapter emits, so its own byte hashes cannot recover native stderr.

Injected trace containing native_stack_site is absent both as raw bytes and as a stack site in adapter output.

Repair: Retain bounded raw streams and trace bytes through an explicitly checked destination or lossless existing output channel. Preserve diagnostic evidence on non-normal exits and fail closed on retention failure; account for wrapper metadata under outer output cap.

## A02: Capture is unbounded until communicate completes (blocking)

Popen.communicate buffers complete stdout and stderr before applying output_limit. Inner memory consumption scales with all child output; outer runner only sees later adapter output and cannot bound this memory.

Harmless Python4MiB stdout probe: 4194304 counted,1048576 retained,10493916 peak Python allocation bytes. This is simulated process capture, not K.

Repair: Incrementally drain both streams using bounded buffers, running counts/hashes, one deadline and explicit overflow handling.

## A03: Signal attribution is not tied to process identity (blocking)

Any SIGSEGV substring and last killed-by text are used independently of interpreter PID, successful exec, or provenance of stderr. interpreterExit can assert SIGSEGV when another process died; unsuccessful exec and ordinary stderr text are not excluded.

Simulated interpreter PID10 exits0 and parser PID20 diesSIGSEGV; parser reports interpreterExit signal SIGSEGV.

Repair: Retain raw trace, associate successful interpreter exec and terminal signal with the same PID and process tree, and report unknown when evidence is ambiguous. Separate wrapper/strace exit from interpreter and parser evidence.

## A04: Post-timeout drain has no deadline and group kill does not cover detached descendants (blocking)

After timeout, killpg addresses the original child group then communicate() has no timeout. Descendants in a new session can hold pipes past the20-second deadline. Existing strong outer containment remains required and may eventually terminate the namespace, but that does not satisfy the inner deadline or preserve its lost summary.

Bounded harmless Python child plus detached grandchild:0.1-second requested limit returned after0.624563807999948 seconds, timedOut true; grandchild naturally ended at0.6 seconds. No K or strace.

Repair: Use a single monotonic deadline through read, termination and cleanup; guarantee bounded pipe closure/reap and explicit incomplete cleanup/evidence status. Rely on the existing admitted strong containment without adding a launcher.

## A05: Nested tool commitments are not preflight verified (blocking)

Binding hashes the lock files but verify_pins never follows their nested tool/source hashes. krun wrapper execs bin-unwrapped/krun and sources k-util.sh; parser reads kast lock and checks kast only after krun begins. Thus hashing the wrapper and lock file does not establish the approved before-K selected tool closure. All inspected nested locks match now, but a current pass does not enforce future mismatch rejection.



Repair: Add the actual selected transitive tool/source commitments to preflight, including nested kast and unwrapped krun commitments, or explicitly establish the reviewed immutable package closure. Bind final pins/config in candidate and runner files.

## A06: krunCalls is asserted before execution evidence exists (medium)

Result unconditionally sets krunCalls1 after spawning strace, even if strace fails before exec. It records one attempted wrapper spawn, not one observed successful krun exec.



Repair: Label scheduled/attempted spawn separately from process-tree-derived observed exec count; do not promote ptrace/launcher failure to a K invocation observation.

## Actual checks and limits

The11 offline tests pass. Independent simulated checks expose the failures above; their exact script/output are `audit-adapter-probes-01.py` and `.json`. Python child probes test capture only. They are not K reproduction.

Normal113/empty-stdout forwarding and preflight hash/stack checks work for the inspected cases. The existing runner requests strong Foreman containment and bounds its own capture, but cannot recover stderr discarded inside the adapter or bound the adapter’s intermediate memory. Live containment and final admission were not tested.

Draft runner/candidate hashes remain unresolved and the pins JSON must be included in the final bound source closure. Keep the diagnostic stage specified-only and preserve the approved isolated resource scope. Repair with the same author, freeze new bytes, obtain fresh source review, then validate final admission before any one-case native invocation.

No K, strace, compile, Docker, network, provider or accounting mutation occurred. No native crash diagnosis or product acceptance is claimed.
