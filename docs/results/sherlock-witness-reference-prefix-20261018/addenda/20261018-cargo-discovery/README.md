# Cargo discovery entry: compile and three-name listing

Date: 2026-10-18 UTC. This additive record publishes one 157-byte product file: `crates/mori-core/tests/sherlock_witness_reference_prefix.rs` (SHA-256 `23be731fb53d7f7f60641399f49369a41c84e3e255a119f06c8468b0c2e2ca9a`). It imports the existing support module as `positive`, making the existing tests available through Cargo's conventional integration-test target layout. The package manifest does not disable automatic tests or declare an explicit alternative target.

The independently authored support consumer (`7391f8f8…`) and four fixtures were already published with PR258 and remain byte-identical. This apply list neither copies nor changes them. The [prior portable evidence](../20261018-portable-currentness/README.md) and original three runtime passes keep their exact historical bindings.

The continuing independent non-author reviewer `21399dd6-faf2-479b-823a-9bfec7feeba7` [approved the new wrapper source and exactly two proposed operations](review/REPORT.md). This uses the same relevant independent review context, not an additional independent vote on unchanged source. The raw report retains its original heading-count typo and explicitly enumerates all nine complete inline bodies.

Root executed [both selected operations once](actual/two-01/RESULT.json):

| Operation | Exit | Elapsed seconds | Observed scope |
| --- | ---: | ---: | --- |
| Wrapper consumer compile | 0 | 0.5171676210011356 | Six compiler source/include inputs; existing Core rlib retained |
| Compiled wrapper `--list` | 0 | 0.0038691859954269603 | Exactly three qualified names; zero test bodies |

Both commands match the frozen descriptors, all streams were fully retained, and all declared pins were unchanged. [Compiler dep-info](actual/two-01/witness-reference-prefix-cargo-discovery.d) records the wrapper, unchanged support and four unchanged fixtures. The separate data-only readback rehashes all 492 static and two generated files; compiler dep-info is membership evidence, not exclusive OS-read attestation. Generated executable bytes are pinned in the actual receipt and are not archived here.

The [218-byte raw listing](actual/two-01/02-listCargoDiscoveryWrapperTests/stdout.bin) names only:

- `positive::factorial_three_complete_reference_prefix_and_pending_caller`
- `positive::list_two_five_complete_reference_prefix`
- `positive::tree_four_one_three_complete_reference_prefix`

No runtime test body, Core rebuild, Cargo invocation, SDK/native/model action or old cohort replay was selected. A future normal Cargo target can use `cargo test -p mori-core --test sherlock_witness_reference_prefix`; this receipt does not establish a full Cargo dependency-resolution/build/run, whose manifest includes separate Midnight dev-dependencies. The ordinary Python DoctorWatson lane is unchanged. Whole035, B-clock/countercase coverage, H1 physical costs, canonical ProjectRel/Link and native/proof/target acceptance remain open. No new EARS closure follows.

Receipt SHA-256: actual result `a3128e3f665d31a3471442db4c2e0609e1406b11d49a925730d26d58cc97c349`; source/operation review `83fcb614c5eadcae1f70764bfde5f8588a38dbf2954cf1405085db22f6b8c9bd`; generated dep-info `8735ab8e04d3ef71db31ee5bad9361561c0d9efb8b0564f9522d025bb6d3e5f6`.
