# Claim-to-evidence boundaries

| Predicate | Consumed production source | Actual record | Boundaries |
| --- | --- | --- | --- |
| Explicit nominal selection | cli_args/compile.rs, draft_contract::select_source, selected compile_sherlock::run | Five tests in actual/actual-op02-08-01/04..08 | Args and direct handler only; top-level CLI dispatch/backend binary unrun. |
| Bounded text to owner preparation | nominal_command::compile -> compile_nominal_host -> existing owner preparation/admission | Both actual artifact captures | Structural declarations/body forms; no expression-body typechecking or evaluation. Existing guards unchanged. |
| Real tables published | wire::admission, production ProgramView serializer | Complete artifacts with23 program_view fields | Output-only wire, not reopen/native codec. Names/count checks differ from complete equality. |
| Wrapped complete tables | Offline source-derived normalizer + typed expected-format adapter | comparison/expected-format-adapter/ROOT-COMPLETE-WRAPPED-PROGRAM-COMPARISON-AFTER-REPRESENTATION-ALIGNMENT.json | Root comparison receipt; protected expected body remains opaque to publication preparation/reviewer. No independent mutual oracle. |
| Representation-kind repair | expected_snapshot_adapter.py | Five synthetic format tests and original MISMATCH4ONLY retained | U128.n only, typed nonnegative bounded integer -> canonical decimal string; no global numeric coercion or product/oracle mutation. |
| Core result reuse | First compiler exit0 plus separate metadata-only recovery | Original STOPPED receipt and recovery receipt | Recovery is not a second compile or proof of unmeasured CPU/VM. |
| Source applicability | Exact22 afterimages and before/absence guards | source/SOURCE22-FREEZE-AND-APPLICABILITY.json | Existing tracked preimages and new-file absence agree at base79f and remote87c. No currentness refresh or EARS adoption inferred. |

Retain: body typing, old-profile byte compatibility, mutual full-table/preparation semantic oracle, wire cfg(test) test, full Cargo/DoctorWatson integration, full mori CLI, source/circuit correspondence, runtime/native/target/financial/H1 and all requirement/refinement closure obligations. Existing owner-authored refusal tests and synthetic format tests do not replace designated security testing. The separate body-typed Core and new CLI integration are outside this cut; this package does not qualify them. The new CLI integration remains unrun.
