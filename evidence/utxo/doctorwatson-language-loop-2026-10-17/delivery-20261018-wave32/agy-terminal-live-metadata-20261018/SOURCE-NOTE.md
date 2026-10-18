UNAPPLIED future-live transport metadata proposal. No provider call or old private capture read occurred. The source is an ordinary format adapter; it supplies no review eligibility, security, resource, or model-identity judgment.

Observed Q1 facts

The physically preserved public TERMINAL-EXPORT-RECEIPT.json says NO_EXPORT / CAPTURE_NOT_COMPLETE, exit -9, zero parse errors, five parsed top-level values, one result event, 9,866 bytes, and unchanged supplied input pins. Its status field is null because selection returns before inspecting status for this exit. That null does not establish a missing provider status. The actual result status, result payload shape/body, result arrival time, EOF timing/cause, and provider cause remain unknown. No context-size explanation or successful review payload is inferred.

CLEANUP.json records ownedGroupRemaining=true and captureReachedEOF=false at termination. POST-TERMINAL-OWNED-PROCESS-CENSUS.json records no rows for group 756529 at 2026-10-18T13:44:32.095554+00:00. These are separate timing observations. They establish neither early cleanup success nor a persistent process leak.

Proposed addition

future_runner.py retains all new parsed values, including nonobjects, exactly as the previous adapter did. LiveCapture now observes only direct top-level event tags. Fixed event count keys are init, step_update, result, otherObject, and nonObject. Arbitrary event names are never copied. The first two direct result events receive local observation offsets and enum-only shape facts. Further direct result events increment counts and resultObservationsOmitted, while all original parsed values remain available to the unchanged selector. This two-record limit bounds metadata only; it changes no existing provider timeout, stream capture, or review selection limit.

Only the exact observed status literal SUCCESS is permitted in metadata. Other strings become OTHER_STRING with a null literal; absent and non-string status receive MISSING and NON_STRING. Nonobject result gets RESULT_NOT_OBJECT. Structured output and result payload are represented by JSON type enum only. No review, response, arbitrary status, tool parameters, init tool list, schema, token, or raw text is copied into LIVE-METADATA.json. Its fixed flags explicitly deny backend identity attestation and payload export.

Times are seconds from local monotonic capture construction immediately before launch setup. Result time is parsing/observation time, not provider generation time. EOF time is recorded when finish is called by the actual os.read EOF path (synthetic tests call it directly). Process exit time is the first observation recorded after existing wait/cleanup completes, not the precise kernel exit time. Thus ordering between these observations is diagnostic and cannot prove actual exit/EOF causal ordering. A null exit or EOF observation means it was not recorded. No historical time is reconstructed.

The runner writes new LIVE-METADATA.json from its in-memory capture after the process returns, before pin checking and review selection. That diagnostic file is not a review vote and is not gated on successful selection; START and the export receipt retain the public task binding and pin outcome. It is not an incremental disk checkpoint. A runner exception or external runner kill before this write may leave no metadata file. Such failure adds no private reread/recovery path.

Preserved gates

The exact ee3536da v2 selector is copied unchanged. load_task, build_argv, and finish_capture are byte-identical to the frozen runner. Exit zero, complete EOF, one direct last result, exact SUCCESS, external schema/token validation, retained nonobjects/parse-error rejection, and unchanged public pins remain necessary for review export. No recursion, response/text fallback, retries, scheduling, old-capture access, or new provider invocation is introduced. The existing process loop, timeout argument, owned-group cleanup operations, and error propagation are retained, with one observation call added after wait/cleanup. No new canonical backend identity claim is made.

Validation and integration

Eight format tests were authored and hash-frozen before candidate source, then run without a subprocess/provider: 8 passed, unittest-reported 0.001s. STATIC-COMPARISON.json records exact source comparisons. Existing earlier test source is preserved under before/ but was not rerun in this packet. No actual timeout producer was needed: the deterministic Clock checks observation offsets without waiting or invoking a process. Production integration/process lifecycle is unrun here. The original runner, original jobs, original private captures, and old review failures remain unchanged.

Root can inspect candidate.patch and full future_runner.py, then choose whether to use this new adapter for a separately authorized future task. Pure check instruction (no provider): python3 -B -m unittest -v test_live_metadata from this new directory. No new provider task is requested by this packet. No old private-file replay is available.
