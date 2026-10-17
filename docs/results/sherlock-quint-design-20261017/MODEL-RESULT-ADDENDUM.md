# Narrow finite-model result addendum

**FQA-FINAL-1 is closed for ordinary implementation-loop readiness.** No correction or further rearm condition remains from this formal/QA lane. Root may apply the user-authorized ordinary loop rearm after its other checks. This reviewer did not arm a goal or execute any model/product command.

This addendum resolves only the pending receipt condition in FINAL-REVIEW.md. The approved final03 design remains bound to `0e2f90a51d112855cb75a40f6e0c17e448f1282f8cdb5becf4bb075210232fc4`. Source/hash/JSON/log inspection confirms:

| Sealed input | SHA256 |
|---|---|
| R16 RESULTS.json | `3d77a75b94e70386d8752770d45a5b4663c0a7c250c8faee92c93fad6fc58f29` |
| SOURCE-FREEZE-06.json | `03c87eddd65ff08a530e57c652139851e227f370256f787746b3d5cc2a675936` |
| COMMANDS-06.json | `06e305b66e555df2d7dcf7415c39c5bb0ec8785cbc7c4942d09d8802566422b0` |

All frozen source/config files and external design pins matched current bytes. The retained A frontend from source02 and four B/directed frontends from source04 exited0; their complete selected import-member hashes still match source06. Actual test/run command vectors match COMMANDS06, explicitly select the TypeScript backend and bind source06. Node v24.21.0 and Quint0.32.0 are supported by their actual version receipts. This is local receipt/byte verification, not process-authenticity or toolchain certification.

All44 declared tests were selected and passed:33 A,8 B,3 directed. I compared every current declared run name with the exact command selection and raw passing-test names, rather than trusting only an aggregate count. In particular, raw `test-interpreterChunks_test.stdout` names `wrongSuccessorValueSensitivityTest` as passing. Its source reaches the real modeled UInt7 literal/publication path, consistently changes returned value to UInt0 across the first after-state, next before-state, final after-state and returned state, and checks that this corrupted trace still satisfies the bookkeeping invariant while failing equality to the independently frozen complete N01 states. This resolves the distinction between internal consistency and independent semantic expectation sensitivity.

Actual bounded observations:

| Run | Bounds | Observed result |
|---|---|---|
| A interpreter |100 samples, max160 steps, seed1 | No violation reported; all15 requested witnesses reached; retained trace-length statistics5–59 states |
| B helper |100 samples, max24 steps, seed1 | No violation reported; all9 requested witnesses reached; statistics3–14 states |
| Directed A |1 sample, max16 steps, seed1 |13 states; all6 witnesses1/1, including repeated no-next and scalar projection |
| Directed B |1 sample, max16 steps, seed1 |12 states; all6 witnesses1/1, including helper/pending-result cuts and publication/no-next |
| Directed refusal |1 sample, max4 steps, seed1 |3 states; both witnesses1/1 |

Raw stdout witness names/counts exactly match RESULTS.json; all selected run receipts exited0. The main read/hash pass checked183 pinned file references, including42 freeze members, frontend member reuse, receipts/logs and all49 ITF artifacts. ITF state counts and declared-variable presence match their recorded inventory. Pre-code expectation pins also remain unchanged. No model or comparator was rerun during this review.

Earlier failures remain preserved: QNT101 for the reserved B parameter `size`, QNT000 for qualified variant matching in the new consumer, and QNT502 from broad imported-boolean selection. Their receipts still exit1. The documented fixes are a runtime-excerpt alpha rename, a pure consumer access adapter and exact declared-test selection. Successful source06 results do not overwrite those failures or imply production source/native changes.

The result supports the first finite interpreter/observer/helper/boundary/financial-fixture validation, with explicit limits. Random traces ending before max-steps are not all classified as completed programs: terminal cuts, projection refusal and the16-call cutoff remain documented, and only one random example per run was retained. The49 ITFs consist of44 test traces and5 sample examples, not all200 random traces; no full implementation replay or exhaustive checking was performed.

Shared-tree physical unfolding, complete Core/2 F01/F02 correspondence, constructors, five composition operators/multi-parent ancestry, residual/MatchPlan/internal widths1/2/4/8, exhaustive field sensitivity and full39/full96/general B/ADT/admission/UInt128-field-byte/canonical/K/Lean/Clean/handwritten/native/target correspondence remain explicitly unvalidated. The new B literal rows remain labeled manual contract transcriptions; this narrow receipt review does not claim a full independent derivation audit or close B semantic acceptance. Those limitations are retained implementation/correctness work, not additional conditions on the ordinary loop entry reviewed here.

The earlier final design approval now has its finite-result condition discharged. Every full-product, security-owner, native/resource and target exit remains unchanged and open where unobserved. No finite pass is promoted to theorem validity, general correspondence, native authority or full Sherlock completion.
