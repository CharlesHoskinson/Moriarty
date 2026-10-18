# PR229 complete Waiting prefix assertions

`candidate.patch` modifies the exact PR229 test from `bcdfd4b27a62e8e618153ef1961d60f889dc4d63` and adds one fixture. Full changed bytes are under `fullsource/`. Assigned wave base `64c6dedff1d99b33c23832330d7911a5fad9ed59` has no `program_plan_roundtrip.rs`, so root first integrates PR229 (or the pinned original test blob), then applies this contextual patch. The sibling resume-suffix test and fixtures are disjoint.

The original pattern discarded `advanced`, `segment`, `attempted` and `residual` through `{continuation,..}`. The new exhaustive pattern asserts true advancement, attempted15, empty positive residual and the complete Some StageSegment: both frames, Execution semantics/action/pre/post, all six whole Steps (index/statement/value/snapshot/effects), and five ordered financial effects. Existing full Program/Plan/Continuation assertions remain unchanged. Complete snapshots retain the zero paid-invoice row while later-credit remains absent.

Financial expectations reuse frozen EXPECTED-PREFIXES (`6c4897e35bf0b7b29529fb2c622f9919c88dfe0b8fcfad94215505831310ebe8`), capture frames/residual and start logical work. This author previously wrote the source candidate and inspected runtime: this is **integration of the existing independent freeze**, not fresh independent derivation. The freeze intentionally omits global ExprIds; Statement IDs2–7 are source-exposed author mappings from PR229/parser schema. No new codec, identity, security or admission claim is made.

`FIELD-COVERAGE.json` lists every nested field. `MANIFEST.json` pins source, expectations and patch. Eight selected API/manifest/docs files agree between wave base and PR229; RUST_GUIDELINES contains a contextual PR229 finite-Program exception and both versions are retained. No whole-tree equivalence is claimed. The fixture holds newly asserted prefix fields; inherited continuation hash checks keep their prior producer-codec limits.

Root command after integration, **specified-only and unrun**:

```sh
cargo test --frozen -j1 -p mori-core --no-default-features --test program_plan_roundtrip invoice_program_plan_and_zero_duty_continuation_roundtrip_as_complete_values -- --exact --test-threads=1
```

Static checks passed: frozen oracle custody, exact case link modulo terminal newline, six ordered steps, aggregate effects equal frozen deltas, final snapshot equals boundary, zero row retained, original continuation assertions unchanged and no resume call added. Rust parsing/compilation, formatting, test execution and result review remain unrun. No source/product/native/roadmap acceptance follows from this packet.

Observed source-only application check: `git apply --check` against the pinned `baseline-pr229/` source exited0. No patch was applied and no shared/Git mutation was performed.
