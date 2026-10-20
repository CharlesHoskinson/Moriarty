# C07 financial admission and genesis API migration

This dated source-and-evidence record covers the corrected cut’s 13 passing ordinary operations and 28 selected passing tests. Fresh independent review approved the bounded actual results, source-cut distinctions and API qualifications. Original C07 results and its five-test admission failure remain separate historical evidence. Publication review and installation are separate; this page does not change acceptance.

## Changes to existing traits

`PreImageView::admitted_function_count(&self)` now returns `Result<usize, HostViolation>`. Update trait implementations and propagate or handle the error at callers. The pure view returns its admitted function-table length; a financial view returns a typed violation, not zero. Returning zero would erase the distinction between a pure empty table and unavailable financial pure-view access. The active Core source inventory contains two `PreImageView` implementations and one `OwnerEngine` implementation, migrated in this candidate. Frozen model/joined09 copies are historical, not active Cargo implementations. The inventory does not certify external or macro-generated implementations and callers.

`OwnerEngine` adds `admit_fin(prepared, &bounds, &layout) -> Result<Admitted, Tooling>` and `admit_fin_run(&admitted, &fuel, &stack_limit, &request) -> Result<MachineImage, FinRunError>`. Existing `admit`/`admit_run` remain the pure route. Financial preparation is a free-function API, not another trait method: `owner_host::prepare_fin_staged_source` runs `stage::load` and returns sealed `FinPreparedStagedSource`; `prepare_fin_staged_program` accepts an existing `StagedProgram`. Supply explicit layout and `FinCapacitiesC` and retain preparation errors.

`owner_host::admit_fin_genesis_image` uses `host_owner_engine()` to compose financial admission and genesis, returning `(Admitted, MachineImage)`. `FinGenesisRequest` carries `action`, `Scenario` and optional `cap`; it is not a runtime step or external event. Genesis resolves the action, builds the scenario snapshot and input bindings, checks the plan cap and initializes committed work to zero. `FinRunError` distinguishes `Genesis` and `Tooling` failures.

## New public financial capture APIs

At the recorded public baseline `87a6a59154c8aebc2f6f35d74722175099aebc86`, `FinHostSnapshot` and `fin_capture_json` do not exist. They are new public APIs in the candidate, not breaking changes to published capture APIs. Earlier R-only candidate shape changes are a separate development history.

`capture_fin(&Admitted, &MachineImage) -> Result<FinHostSnapshot, HostViolation>` observes the actual admission-bound image before copying it. `FinHostSnapshot` fields are private: read `admission()`, `image()` and `state()` through immutable references instead of constructing or replacing its fields.

The new `fin_capture_json(&FinHostSnapshot)` API returns `Result<String, FinCaptureJsonError>`. The `Value` return belonged only to an earlier unmerged R-only prototype. It writes the complete diagnostic capture through the retained `admission.bounds.serialization_bytes` limit, including a real zero limit and escaped UTF-8/Debug output. Retain errors. A caller needing a JSON DOM can parse the returned bounded String afterward; parsing is a separate operation. Earlier uninstalled prototypes with public fields or `Value` results were never a published compatibility promise.

`OwnerEngine`’s two new financial methods are required methods: out-of-tree implementations must migrate explicitly. `HostViolationKind` also gains `FinancialUnsupportedBeyondGenesis { op: FinUnsupportedOp }`; external exhaustive matches may need to handle it. No default-success, zero-count or unsafe compatibility shim is provided. Three backend cfg-test blocks use unchanged `admit`/`admit_run`; SDK/backend compilation against this Core remains unobserved.

## Genesis is not financial runtime

The candidate prepares, admits, initializes and observes financial genesis. Financial `select`, `step` and `run` still refuse with `FinancialUnsupportedBeyondGenesis`; no financial statement is evaluated by this owner route. Pure complete capture and C6 retain their admission-binding-first financial refusal. This diagnostic JSON is not a financial C6 codec, checkpoint or adopted native wire format.

Keep the existing Core/2 `start_observed`/`resume_observed` invocation producer separate from this owner-genesis API. Preserve complete program/plan/configuration, ordered inputs, snapshots, steps/effects and residual obligations in their applicable carriers; `same_source` or standalone `FinAdmittedExtras` is not a replacement for admission/context binding. No financial transition, continuation, fees, proof, target settlement or complete-effects correctness result follows from an initialized snapshot.

## Observed checks and remaining qualification

C07 ran 13 financial-genesis tests and one bounded-writer test successfully, then 21 owner regression tests successfully. A later 12-test nominal-admission suite returned 101: seven passed and five failed. Its exact diagnostic differences remain recorded. Listing the harness was a membership observation, not a passing execution result. Earlier R435's five individual V8 tests passed on their own source cut; they do not certify this later generic/public composition.

The focused correction changes diagnostic routing in `owner_prepare.rs` and adds three tests. Its actual 13 operations all passed: fresh normal/cfg(test) Core builds, required listings, two external-consumer builds, and 28 selected tests (15 admission, five C6 host, four complete-observation and four generic-owner/refusal). Normal/cfg(test) dependency memberships were exactly 117/190; their compiler summaries reported 31/34 warnings and zero errors. The 439-name whole-harness listing is membership, not 439 executed tests.

The earlier 14 financial tests and 21 owner passes were not rerun or refreshed onto this corrected cut. Their source identity remains historical. Fresh independent actual-result review approved these bounded results and their source-cut distinctions; unobserved consumers remain unqualified. Do not fit diagnostics to output or transfer a historical pass across changed source. Full financial behavior, retained independent outcomes, native/proof/target and EARS obligations remain outside this migration record.
