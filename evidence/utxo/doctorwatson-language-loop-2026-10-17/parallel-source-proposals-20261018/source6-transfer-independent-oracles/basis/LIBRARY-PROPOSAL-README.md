# Unapplied transfer-only Rust Source/6 implementation slice

Candidate is source preparation by GPT-6.1 Sol high, not adopted or executed. `source6-transfer-slice.patch` adds a feature-gated `mori-source6` library and ordinary normal-case tests. `proposed/` contains the reviewable full files. Root owns integration, independent design/source review and actual checks. The existing Rust CLI/Core expand and all compiler/proof files are untouched.

Callable proposal:

```rust
mori_source6::expand_transfer(beta_source, selected_action, &typed_local_scenario)
```

The function runs the existing real beta parser/checker; selects one checked LocalS0 transfer; reads its exact computed atomic quantities through current side tables; and emits the existing closed Source/6 presentation. It copies original source claims and explicit opening scenario cells. It does not calculate a post-state, signature status, stage success, proof, circuit, native result or target cost. Return qualification is `local-stipulation-only`; the result has no accepted/success flag. Generated text still requires consumption by the real Source/6 lowerer.

This slice deliberately exposes a **typed Rust local-input interface**, not the Node scenario JSON protocol or full BetaExpansion wire shape. Candidate-effects overrides, provenance field/origin maps, source/scenario report identities, repayment emission and CLI integration are absent. Checked repayment/family selection explicitly returns `mori::source6::unsupported_operation`; missing action returns `mori::source6::unknown_action`. No previously admitted public program is reclassified: this is a separate feature-gated internal slice and is not wired into public commands.

Contract and review boundaries:

- Exact normal mapping: existing Node bridge lines85–114,162–219; Source/6 specification lines7–36. Decimal scaling/evaluation stays exclusively in `mori-checker`; no second decimal evaluator or constructed Core/target IR is added.
- Typed local cells follow Node's existing ordered transfer scenario. All balances, opening allowance/work counters, replay tag, heads and round remain explicit claims. Representation checks retain existing Source/6/string/counter bounds and scenario identities; no new resource policy is selected.
- The feature satisfies the guideline location for an unadopted prototype. It is implementation preparation, not public developer permission.
- The four literal fixture files and normal arithmetic were written and hashed before implementation. The implementation author also wrote expectations; a separate non-author expectation/source audit is still required. Manual snapshots copy those frozen literals, not evaluator output. `EXPECTATION-FREEZE.json` records the four original digests unchanged.
- The only Cargo.lock proposal is the new workspace package and its already present dependencies. No dependency version or new remote source is introduced. The package is found by the root's existing `members = ["crates/*"]`; no shared root dependency/CLI API change is needed for this library-only slice.

Unrun checks and smallest integration decisions:

1. Inspect and adopt the typed local scenario API and transfer-only feature scope. It is a proposed library interface; it is not an adopted public expansion result. Decide separately whether public beta expand will port the complete Node JSON/provenance contract or call this smaller internal interface. Core expand behavior must remain unchanged.
2. Root must authorize/inspect ordinary Cargo selection and validate the lock entry, formatting, compilation, lints and two normal tests with the feature enabled. No Cargo/rustc/build/package/Node/Mori/SDK/native command ran here. The patch is not claimed to compile.
3. Root must capture **actual emitted Rust text**, then call the unchanged real `parseAndLowerSource6` on those bytes. Compare every lowered AST/state/intent/effects field with the frozen source semantics; do not pass only an expected fixture or a mock DTO. Run the unchanged `prepareSource6S0Unqualified` separately only within authorized scope; any local result stays PreparedUnqualified. Do not generate/update goldens from implementation output.
4. Add the named zero-fee normal fixture and ordinary repayment/family/unknown-action refusal checks, with source-described diagnostic snapshots; preserve parser panic/property coverage requirements before considering a complete adopted crate. No such additional test result is claimed by this bounded proposal. Security testing/judgments stay with designated Opus owners.

Native Source/6/Core/5→Core/1/Core/2/Sherlock embedding remains unspecified. Program-versus-stage-instance data and exact security-owned bindings/phases need their own adopted contract; this patch does not select them. No lowerer is bypassed and no handwritten target IR, proof stub or success Boolean is introduced.

Verification performed here: source/API inspection, expectation-freeze digest comparison, TOML parsing with Python, generated diff consistency and source-hash inventory only. These establish artifact consistency, not Rust correctness. All original U0–U7, M1–M17, MC/SP/G and finance/composition/native/Preview/Fenrir exits remain unchanged.
