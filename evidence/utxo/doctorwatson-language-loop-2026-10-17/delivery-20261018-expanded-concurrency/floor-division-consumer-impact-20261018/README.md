# Floor-division consumer impact

**No additional accidental exhaustive-match compile gap was established by source inspection.** The preserved candidate already accounts for all eight identified production matches on the shared flat Core `Expr`. No correction patch is necessary for those sites. Compilation and all product execution remain unrun; this is not a build-pass claim.

Current checkout capture: `823ce081c00786a0c55b92af81009f9e7c5e6a16`. All ten modified-file bases in the earlier `defi-floor-division-current-source-20261018` packet are byte-identical to the current checkout. That earlier packet was preserved unchanged.

| Exact consumer | Existing candidate handling |
| --- | --- |
| `runtime::type_cached` | Explicit UInt/UInt result typing |
| `runtime::eval_cached` | Explicit division node evaluation |
| `stage::cost::expression_nodes` | Explicit unsupported-profile result |
| `mori_core::sherlock_source::map_expression`, kind match | Explicit node label |
| Same function, translation match | Explicit unsupported projection |
| `mori-backend::Builder::expression`, value match | Generic unsupported expression; no division lowering |
| `mori-ledger::relation::Compiler::expr`, value match | Generic expression-not-lowered; no division lowering |
| CLI `inspect`, expression-rule match | Unsupported display fallback after existing cases |

[CONSUMER-IMPACT.json](CONSUMER-IMPACT.json) pins exact current and candidate file hashes, line spans and complete match bodies. It separately enumerates seven partial reference walks and two selective readers. Their wildcard/if-let patterns create no exhaustive-match compile gap. No new traversal or consumer capability is inferred: default CLI routing, staged admission, Sherlock projection and both native lowerers remain outside the proposed reference API.

[PATCH-CORRECTIONS.json](PATCH-CORRECTIONS.json) therefore records zero patches. Adding a native arithmetic rule, a new wire format, a default route or a new traversal capability would make a substantive choice outside this compile-gap task. `BinaryOp` is unchanged. Sherlock's separate `Expr` enum, beta `NodeTag` AST, captured DTO copies and the standalone Cardano compiler's separate Core dependency do not need broad compatibility rewrites.

Five exact matters remain open:

1. The proposal-only source/semantics selectors and explicit library API versus production CLI routing remain unadopted.
2. With the feature enabled, shared `Expr` derive recognizes `FloorDiv`, and frozen `v1::Program` reuses `Vec<Expr>`. Decode can therefore recognize it before the separate profile validator. The candidate's ordinary validation refusal does not establish identical frozen decode behavior. Wire isolation/diagnostic selection remains unadopted.
3. The proposed cost identity and node charge remain unadopted; authored arithmetic/work values are not an independent oracle.
4. `mori-backend` has no division lowering. Its unsupported fallback supplies no quotient, remainder or numeric proof policy.
5. `mori-ledger` has no division lowering. Its unsupported fallback supplies no quotient, remainder or numeric proof policy.

Wire scope is explicit: Step/Execution carry statement ExprIds and snapshots, without a Program expression arena. `StagedProgram` has no public constructor or Deserialize and is produced by source `load`; Continuation/Event carry scalar values and snapshots, not Expr arenas. No nonexistent staged Program decoder is treated as a missing patch.

The prior packet retains all 72 rows, DA01–24, all three heldouts, eight intents, eight regression classes and five composition operators. No acceptance status changes here. Only this new directory was written. No build, tests, product runtime, provider call, shared/Git write, security judgment, test design or compiler/proof-policy decision occurred. The included source-inspection script performs read-only source/byte comparisons and writes this packet's metadata only.
