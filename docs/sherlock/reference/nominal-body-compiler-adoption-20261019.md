# Nominal body compiler adoption and source status

Root adopted exactly the three judgments in the preserved
[static judgment proposal](../proposals/nominal-static-judgments-20261019.md)
and its [full integration contract](../proposals/nominal-public-integration-contract-20261019.md):
Eq-Any, distinct parameters in every declaration, distinct source-preorder binders
in every arm. The proposals retain their historical status text unchanged.
Root's adoption is recorded at
`.moriarty-dev/runtime/nominal-body-review-20261019/static-judgments-design-votes/ROOT-ADOPTION-20261019.json`,
2521 bytes, SHA-256
`e3f53911e6d3de11fb718cf9e9d2afe372dc8b83e33fc6e3dbfbe65e727b6a6b`.
It counts two substantive Opus opinions and the fresh Muse amendment
applicability review. It does not adopt more judgments or qualify execution.

The public Core source entry and report now have a reviewable implementation in
`compile_artifact/nominal_body_compile.rs`; see the
[public usage guide](../how-to/compile-body-typed-nominal.md). The raw compiler,
its report, owner guards and existing exhaustive error enums are unchanged.
The selected CLI integration remains Root's follow-up after its existing source
review; this library source is not a claim of CLI delivery.

Source-derived boundary tests were frozen before wrapper implementation at
`91f8a4af` (21 functions). Six supplemental assertions were added after wrapper
source, with their distinct provenance labelled in the test file. They cover
exact repeated-parameter offset, literal field suffix mapping, absent invalid
sites, nominal field mismatch, recursive-call argument mismatch and supplied
layout use. All 27 functions remain UNRUN. No compiler, SDK/dev dependency,
interpreter, native or provider operation ran for this candidate. Passive
source inspection and diff checks do not establish that it builds or passes.

Body checking precedes owner configuration guards and repeats T3. Resource and
owner review, a qualified full normal Core rlib build and actual external public
consumer checks remain required. Security review stays with the designated
Muse outage fallback; this author supplies no security verdict. H1, compiler
correspondence, formal/circuit/financial/lifecycle/native/target evidence and the
full revision03 goal remain open with their independent acceptance requirements.


## Later BW observation and CLI successor

Root's separate BW30 cut passed the full normal Core build, external consumer
build, exact27 listing and27 individual exact tests on unchanged BW47b284b1.
Its ACTUAL-FREEZE.json is88517bytes with SHA-256
`d8217c6d16154ebb958c51c1f46e4ec2a52b765c6d7ce930f905343c326a51b0`.
The earlier unrun statements above describe the source-review stopping point.
This observation qualifies only that exact public-library cut; it does not
establish H1, semantic correspondence, native/target or full requirement closure.
The CLI successor imports those exact checker/wrapper bytes and CW's pinned
structural source. Its21 new selected-handler tests remain UNRUN; their freeze
and source-derived expectation correction history is recorded separately.


## Later NW selected-handler observation

On October19, Root observed NW `6d4125dc2f58a554c185b08bf55f324d8d41820a`
pass its separate24-operation normal Core/real-Args/selected-handler qualification,
including all21 exact tests. Historical UNRUN statements above describe their
dated source-review stopping points. See the [observation record](nominal-body-typed-cli-observation-20261019.md).
This supersedes no native, target, proof, H1 or full-requirement evidence gap.
