# Root integration recipe

Status: SOURCE_ONLY / UNRUN. Fresh designated Opus/Gemini audit of the complete
candidate is required before execution. This author changed only R200 artifacts.

After review and Root selection, the two Rust afterimages replace the matching W
files. No new module registration is needed: the checker already includes its
existing test child module. Original tests are a byte-identical prefix; the new
AST tests are appended and do not invoke a parser.

The R120 proposed `compile_artifact/draft_diagnostics.rs` exhaustively matches
DraftTypeErrorKind and therefore needs a Budget arm before compiling that consumer.
A precise ordinary projection, consistent with its decimal-string sizes, is:

```rust
DraftTypeErrorKind::Budget(budget) => json!({"Budget": {
    "resource": match budget.resource {
        crate::sherlock_draft_types::DraftTypeBudgetResource::TypeDepth => "type-depth",
    },
    "limit": budget.limit.to_string(),
    "observed": budget.observed.map(|n| n.to_string()),
    "child_index": budget.child_index.map(|n| n.to_string()),
}}),
```

Keep its existing outer `source` projection: it denotes the admitted parent for a
refused child; `child_index` identifies the precise attempted child. Root refusal
has root source and null child_index. A null observed means checked depth-counter
overflow, not success/unknown count/saturation. Do not materialize the refused
child path merely for reporting. This is a separate integration source recipe,
not a change or tested serialization result in this lane.

At the later reviewed public compiler boundary, consume the actual caller extent
with `check_program_types_bounded(&draft.program, max_type_depth)` in place of the
legacy type-check call. R120 shape already names `max_type_depth`; Root owns actual
config conversion and propagation. No value/default is selected here. Keep
legacy `check_program_types` available with no implicit cap.

Root/R201 may independently select AST fixture checks and compare the existing
sealed R115 normal type/provenance cells under sufficient explicit extents.
Candidate execution, including ordinary tests, remains deferred to reviewed Root
selection. No parser/bridge/copy/report/Drop/config/full public-consumer adoption
is established by these two source files.
