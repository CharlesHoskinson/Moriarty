# Guarded full financial prefix consumer source candidate

Status: source prepared and frozen; build, consumer execution, tests and fresh
nonauthor review UNRUN. No acceptance claim. Root remains sole shared integrator.

The requested gap is that the existing positive production-consumer test checks
full endpoints, effects and 18 values but does not compare every changing
financial field at every prefix. This candidate adds one separate integration
test and a separate fixture directory. It changes no production semantics and
leaves every original test and fixture unchanged.

The assigned source base is bdbe1f9edc70e7670c240fe865003465e21926c5,
tree 7bdc7a87aa81fa22271781277f405f8d1452f973, checkout
/home/charl/Moriarty/.worktrees/utxo-actus-prefix-main-20261018. This identity was
supplied by the integrator; this subtask did not run Git to independently attest
it. SOURCE-EXPOSURE.json records byte hashes of the actual inspected files.

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
byte copying, metadata checks and patch/manifest preparation. The task's explicit
source-only restriction deferred plugin status execution and all ordinary runtime
verification; no historical queue was selected.
