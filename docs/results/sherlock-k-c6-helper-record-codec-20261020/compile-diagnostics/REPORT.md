# Complete first-compile stderr classification

The actual K compile exited 0 in 5.289 seconds, with no timeout and no remaining process-group members according to the retained launch/Root records. It was **not warning-free**. The complete 152,635-byte stderr contains exactly **330 Compiler warnings**: 316 deprecated `klabel(...)` attributes and 14 unused variables. Every byte is accounted for; no error/critical/info diagnostic or unclassified message occurs in this stream. Every rendered code excerpt matches its pinned source lines after renderer whitespace is removed, and every remaining renderer tail contains only whitespace/caret marks; no extra unclassified trailing output is hidden in those excerpts.

| Source file | Deprecated klabel | Unused variable | Total |
|---|---:|---:|---:|
| c6-helper-codec.k | 57 | 14 | 71 |
| c6-helper-syntax.k | 142 | 0 | 142 |
| c6-typed-root-codec.k | 51 | 0 | 51 |
| c6-typed-root-syntax.k | 66 | 0 | 66 |
| Total | 316 | 14 | 330 |

The [complete readable catalog](ALL-DISTINCT-DIAGNOSTIC-CONTENT.md) gives the two message templates once, all 316 dynamic label/source-location entries, every indicated declaration line, and all fourteen unused-variable sites/rules. The [full membership](DIAGNOSTIC-MEMBERSHIP.json) retains exact message text, original rendered excerpt/caret text, source lines, byte spans and hashes for every diagnostic. [stderr.original](stderr.original) is byte-exact to the original SHA256 `b027eb3abf3c6eb1d0524d76da75e20c520d2b5426d3c8b94341e45093e4f95b`. The ordered330 byte segments reconstruct all2258 lines exactly, with zero uncovered bytes; this is a text-data check, not a K run.

## Deprecated annotation category

All316 messages say the `klabel(_)` attribute is deprecated and suggest removing `klabel(LABEL)` or replacing it with `symbol(LABEL)`. They point at explicit annotations in the four pinned source files. They do not report a missing symbol, unresolved rule, parser error or failed type resolution. Representative sites include helper-codec line11 (`chSiteTag`), helper-syntax line106 (`chScalar`), typed-root-codec line197 (`ctEncodeRoot`) and typed-root-syntax line65 (`ctRoot`). The full membership lists every site rather than treating one example as coverage.

There are315 distinct deprecated bare-label strings. `chMatchPhaseTag` occurs at two different declarations: the Int-returning helper-codec function at line149 and the helper-syntax nullary CHMatchPhase constructor at line66. This is not an unresolved-label diagnostic. The separate retained209-label qualification records full actual role/signature/argument/result/source/location association, and records an earlier surface-name-only metadata attempt that stopped on this overload. That static qualification is not rerun here and is not semantic reduction. Do not conflate316 attribute occurrences,315 bare strings,330 warnings or209 selected actual labels.

The deprecation is a compiler compatibility/style finding at this cut. It is not a warning-free result, and this report does not certify that a future compiler will retain the annotation. No source migration, compile retry or new execution was performed.

## Unused-variable category

All14 unused-variable warnings refer to the authored helper-codec projection rules. Variable counts are S4, O3, P2, B2, I1, T1 and C1. All seven message spellings have the same instruction: prefix an intentionally unused variable with underscore. Together with the deprecation template there are eight parameterized message forms (or322 distinct messages when concrete label/variable spellings are retained and whitespace is normalized).

Eight warnings occur in `chCommonLane0/1/2/3` at lines249,251,253,255. Source inspection shows the rules intentionally project one component: site tag, origin, resume tag, or resume index. Other pattern fields are not referenced in that rule's right-hand side. The warning counts match these ignored fields: (O,P), (S,P), (S,O), (S,O).

Six occur in `chStagedTag` at lines263,266,269,272,275,278. These rules select a constructor's tag and do not use its payload. Adjacent `chStagedPayload` rules reference the payload separately. This explains why the compiler flags the named payload variable in a tag-only rule; it does **not** independently approve the ScalarStaged numeric tag map or the encoder/decoder semantics. The [exact pinned source context](SOURCE-CONTEXT-244-284.k.txt) and its provenance record retain the inspected text. This task has now inspected those source projections, so it must not be represented as a new source-blind Scalar semantic derivation.

## Evidence boundary

The retained Root compile record reports exit0, 15 compiled files and317 static pins. Its historical `stderrQualification: UNADJUDICATED_UNTIL_FULL_READ` and earlier label-qualification status remain unchanged. This report adds complete diagnostic classification; it does not rewrite those receipts. Current hashes of all four diagnostic source files match their original source-freeze pins at this readback. A matching snapshot is not continuous immutability.

The later qualification reports209 unique selected actual raw labels with matching arities/sorts/roles/source locations. It explicitly leaves the31-case runtime cohort UNRUN and independent Scalar approval OPEN. This diagnostic inspection provides no K runtime/equality result, function totality/rule coverage theorem, general codec correctness, security judgment, admission, native/proof/target, T1/fullABI or EARS acceptance. The absence of a reported error is not evidence that unexecuted semantic predicates passed. No model/expectation/private-body/source changes, provider, Git, build or runtime execution occurred.
