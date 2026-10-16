# Sherlock host client source proposal

These three source modules prepare an ordinary consumer of Sherlock's proposed host engine. They are unregistered source proposals, with no combined compilation or actual engine capture performed for this package. Publication does not establish a new product capability or adopt the host API or implementation.

- `execution_capture.rs` supplies `capture_case`, which calls the existing fixed `shared::owner_host::host_owner_engine()` factory, `admit`, `admit_run`, the bound full observer and `step`. The caller supplies the typed fuel and stack limit unchanged.
- `typed_inputs.rs` supplies literal program, bounds and layout constructors for thirteen input fixtures, without an evaluator or expected results.
- `observation_json.rs` supplies pure complete typed JSON presentation for the public observation and event carriers. It is not a canonical codec or bounded encoding.

The capture source emits one initial full observation, then each actual returned step event with its full postimage, using the `sherlock-host-capture-v1` JSONL envelope. Initial keys are `schema`, `kind`, `case_id`, `fuel`, `stack_limit`, `observation`; event records add `event` before `observation`. The existing typed JSON writer preserves the complete RunIntC carrier. Startup and P0 emit no StepEvent under the adopted event contract, so no event is synthesized for them. The client stops at an observed non-Running outcome and retains typed admission, host, rendering and I/O errors. A caller must keep the Result with its output; an I/O error can leave a partial final line.

The modules require the separately prepared `shared`, owner observation and `raw` type closure. They are intended as sibling modules; this directory is not a standalone build target. No engine, module registration, CLI, wrapper or execution framework is included. Fresh Muse Wonder 1.0 high source audits approved or agreed with the exact three source candidates within their stated source-only scopes; those audits do not establish combined compilation or behavior.

Compiler/tests, actual engine capture, model/circuit correspondence, proofs, native and target execution, financial conformance and product acceptance remain separate requirements. No proof, native, financial or roadmap exit is closed here. See the [facts, reasons and open questions](../../../wiki-llm/sherlock-implementation-journal/HOST-CLIENT-SOURCE-20261016.md).
