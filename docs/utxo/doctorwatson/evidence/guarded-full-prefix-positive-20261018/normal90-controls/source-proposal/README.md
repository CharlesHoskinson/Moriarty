# Guarded full financial prefix consumer source candidate V2

Status: source prepared and frozen; build, consumer execution, tests and fresh
nonauthor review UNRUN. No acceptance claim. Root remains sole shared integrator.

The requested gap is that the existing positive production-consumer test checks
full endpoints, effects and 18 values but does not compare every changing
financial field at every prefix. This candidate adds one separate integration
test and a separate fixture directory. It changes no production semantics and
leaves every original test and fixture unchanged.

The V2 patch base supplied by the integrator is
68decd2ccb2596c4a0b643b65bc9efea52fa4a51,
tree 758de86ba33bdf488f9cdbf411b7dcfada8a491a. The integrator reports that all
inspected production sources and the original guarded test remain byte-identical
to the original inspected bdbe1f9edc70e7670c240fe865003465e21926c5 base, while
69 unrelated additive test/fixture files are new. This subtask did not run Git
to independently attest the composition identity. SOURCE-EXPOSURE.json retains
the original source hashes and records this V2 assignment separately.

V1 remains intact at the sibling guarded-full-prefix-consumer-source-20261018
path. V2 formats only its copied new test with the explicitly authorized pinned
Rust 1.99.0 rustfmt, then rebuilds the exact additions-only patch. No assertion,
expected value, adapter logic or test semantics was edited. Rustfmt reflows
whitespace, reorders the independent serde imports, and inserts expression block
braces/optional commas. The exact diff and source-only lexical audit are under
formatting/. Both the formatter and its --check exited 0 with empty stdout and
stderr. This formatting result supplies no compile or product-test evidence.

`candidate/` holds every complete proposed source/fixture body at its intended
repository-relative location. `source.patch` is the exact additions-only patch
for those bodies. `PREDICATE.md` states the falsifiable consumer comparison.
The candidate fixture README explains every representation adapter and semantic
limit. FREEZE-MANIFEST.json hashes all packet artifacts except itself.

The copied independent oracle's full original FREEZE-MANIFEST SHA256 is
ac6c8a507d14a8ed94dff00993394d096921c52f77896f8cb6bdb463515003a4.
EXPECTED-SEMANTICS.json remains 207332 bytes, SHA256
459c78c56da7e5897110e5d3239c7b8b9f55b15de7ab7d59f0bb6da4d5eecc7a.
INDEPENDENCE.json records the frozen author's premises separately from this
consumer author's actual implementation exposure. No derivation was rerun and
no expected field was filled from production output.

There is no new metered execution. The original sufficient-cap test is unchanged;
cost50 remains conditional here. The copied oracle's cap49 refusal subsection
is outside scope, unselected, unadopted and unused. No security, negative,
cryptographic or target test is authored or run.

The exact callable ordinary check after integration is:

```sh
cargo test --locked -p mori-core --test guarded_quantity_fee_full_prefix_positive guarded_quantity_fee_matches_every_complete_independent_financial_prefix -- --exact --nocapture
```

The parent must arrange fresh Gemini4ArgonHigh source review through AGY, sole
integration, and the bounded actual check. Changed bytes require new exact-byte
review. This packet neither dispatches that review nor runs any build/consumer/
test/provider or changes any shared Git/worktree/runtime state. Local shell and
Python operations were limited to read-only inspection, external source writing,
byte copying, the explicitly authorized external-only formatter, metadata checks
and patch/manifest preparation. The task's explicit
source-only restriction deferred plugin status execution and all ordinary runtime
verification; no historical queue was selected.
