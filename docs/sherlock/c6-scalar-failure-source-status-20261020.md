# C6 ScalarFailure circuit source status — 2026-10-20

This publication adds the full C6 `BHelperScalarFailure` instruction emitter, its public host-pair and fixed-owner-record consumers, and the HELPER canonicality dependency. The existing C6 Push, DATA, META and PROG/CTX modules remain unchanged.

The selected physical leaf covers a Scalar helper at `Done/PrimError`: an applied step charges remaining, consumed and helper work once, clears the helper and stores the same error code; the helper-work refusal preserves state and publishes `Exhausted{HelperWork}`. The emitter carries the current ten frame variants, eleven controls and source origins. Admission and predecessor invariants remain explicit premises.

The implementation and security test source were authored by designated Muse Wonder HIGH during the recorded Opus outage. Two independent reviews approved the exact bounded production source and normal API by source inspection. Two further independent reviews approved the added public-owner test APIs, the real code-11 fixture and the zero-cap reachability facts. Reviewer B required an explicit Root disposition of the impossible active zero-cap target; Root recorded that disposition without changing admission, invariants or circuit policy. The source-author response is not counted as an independent review.

The 20 tests are **UNRUN**. This publication contains no Rust compilation, SDK preprocessing, constraint-synthesis, cryptographic proof, resource-fit, deployment or settlement receipt. The source includes preprocessing tests and one separate MockProver test; source review does not establish that either stage passed.

The zero-value-slot test asserts the actual public boundary: R5 refuses positive fuel under a zero-slot capacity; a valid zero-fuel genesis reaches P2 before Scalar selection; the selected-step wrapper refuses that publication. A positive active zero-cap admitted specimen is unreachable under the unchanged R5/SI03/P2 domain. This boundary test is not an active zero-cap witness and does not waive malformed-witness rejection. The code-11 test starts from an admitted `Add(UInt 1, Bool true)` program, steps the real public owner and calls the selected wrapper once. It is a selected fixture, not a proof of full SI21.

Public module consumers live in `sherlock_machine::c6_scalar_failure_step`. Parent exports include `C6ScalarFailureStepEmitter`, the circuit builder/checker, the selected recorder and explicit aliases `witness_scalar_failure_from_host_pair` and `witness_scalar_failure_from_owner_record`; these aliases preserve the existing Push witness exports.

Source pins:

- `c6_scalar_failure_step.rs`: 136232 bytes, SHA256 `d37c6db795a4be27e108a3df99171859933b42b368973e85622553a6a5cd89d3`.
- Unchanged reviewed production prefix: 56708 bytes, SHA256 `7bc68340a55750d06d7a9d1ac34c290a22080f376a1145114651718e19ac4256`.
- `c6_helper_canonical.rs`: 24559 bytes, SHA256 `ce90d38d4ec8d04fb3b889816c800c78fb3c2e2a45abbcbbf7853d3a82445253`.
- Production reviews: A `d19a9bf1622334d5e528f3b7fd3193c01774731b776576692d1a66a2590f7273`; B `6eec90c605992e089b0badd337c563c80cc04f80ca765f4f96fdd70daabcec97`.
- Added-test reviews: A `90cfa5adf27b39756a9a6133ca8e216cbd80324081fa7129f23883a8b6312f4b`; B `d5a523966b74177c1c0548ea39be0c07da20a50fc2b1b687348354bf44864e83`.
- Root unreachable-target disposition: `dfce44a1b601f93a37d614aa03c6d1e35b0f47f663ab653998f30b9fd5d156d5`.

Full SI08/SI21/SI22, admission/program-membership and verifier-issuer binding, all 39 selectors, refinement, required witness and constraint exports, grouping 1/2/4/8, residual execution, financial and persistent semantics, and native/stagenet acceptance remain separate required work. This leaf is not whole Sherlock or EARS completion.
