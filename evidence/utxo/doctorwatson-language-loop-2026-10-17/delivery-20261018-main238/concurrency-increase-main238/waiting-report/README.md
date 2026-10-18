# Complete Waiting assertions — direct main71c refresh

`candidate.patch` now applies directly to exact main `71c22f8a477231be869786eb42f04e3b50d1b3a7`. The original PR229 test is present with identical bytes, so the former after-PR229 condition no longer applies. `fullsource/` holds the complete changed test and new prefix fixture. Source-only equality showed no API or expectation edit was needed.

The exhaustive Waiting pattern checks every existing field: advanced, Some segment, continuation, attempted15 and empty positive residual. Whole segment equality checks both frames, Execution semantics/action/pre/post, six complete ordered Steps and five ordered effects. Original complete public Program/Plan roundtrip and continuation export/reopen assertions remain intact. Zero paid-invoice stays present while later-credit is absent. No resume, security/admission or new codec claim is added.

Frozen EXPECTED-PREFIXES SHA-256 remains `6c4897e35bf0b7b29529fb2c622f9919c88dfe0b8fcfad94215505831310ebe8`; the packaged fixture remains `d771ae9aa9affbee7889e8154cae4d524cc26b5342c48112e469180ec30a104e`. The author previously wrote/inspected the case and runtime. This integrates the existing independent finite financial freeze; it is not fresh independent derivation. Explicit Statement ExprIds are author-side schema/PR229 mappings, and inherited hash checks remain producer-codec consistency. Disclosures and original oracle provenance are retained under `PROVENANCE.json` and `oracle/`.

`MANIFEST.json` pins exact main source blobs, full test/fixture bytes and patch. `FIELD-COVERAGE.json` preserves the detailed field map. `ORDINARY-COMMAND.json` gives the exact inspected, bounded command; it remains **unrun**:

```sh
cargo test --frozen -j1 -p mori-core --no-default-features --test program_plan_roundtrip invoice_program_plan_and_zero_duty_continuation_roundtrip_as_complete_values -- --exact --test-threads=1
```

Develop, exact AGENTS/product and read-only guarded status were restored. Source/JSON/hash/application checks are separate from Rust/compiler/runtime execution, which did not occur. No shared/Git mutation, provider, SDK/proof/native or subject execution occurred. Root remains the sole integrator. The prior source packet is unchanged.
