# Independent Candidate A boundary review

Reviewer: native agent /root/a0_final_review, requested model gpt-6-astra.
Role: nonauthor task review, not a Council seat or provider-identity attestation.
Base: e84f737dc97cf923579f8a59c91ee04c03ff0233.
Head: d14cfea98a1c9213e5ef5f12c1a088f4e966083d.

## Verdict

Spec compliance and code quality approved for the four boundary files.
No blocking source defect found. Runtime and exact-copy archive acceptance
are separately coordinator-owned.

## Source findings

- candidate_a_authority_boundary.qnt:18 validates complete maps before all six
  account-to-escrow comparisons.
- Lines 23–43 recompute complete planned records, check all one-to-four
  operations, and require after-resolution outer/identity equality.
- Lines 46–72 recheck A fidelity at signing, verification and commitment while
  retaining unchanged common authorization.
- Lines 74–109 keep rejection reachable after semantic failure and preserve
  classification precedence and the exact original attempt/evidence/context.
- candidate_a_authority_boundary_fixtures.qnt:15 declares concrete independent
  funding expectations. candidate_a_authority_boundary_test.qnt:106 compares
  the whole committed state.
- candidate_a_authority_boundary_harness.qnt:48 and :60 have false-guarded
  complete assignments. No enabled stutter is introduced.

## Evidence observations

All four diff-reconstructed files and all sixteen final-corrected-source files
match live bytes. Every relative import resolves within the closure.
Failed final-source differs only in the two disclosed fallback corrections.
Recorded source hashes across six historical RED/GREEN directories match.
Author runtime receipts record successful typechecks, forty boundary tests,
fifteen adapter tests, and one hundred samples. These are inspected author
results, not this reviewer's execution.

## Minor finding and disposition

final-verification/python-suite.json:27 records command/output/exit for the
historical 441-test run but no full Python source/test/dependency closure.
environment.json binds only Core/swap reference hashes for Python.
Disposition: retain as a historical provenance limitation. Do not describe
that Python receipt as fully source-closed or reconstruct historical bytes.

## Independence and scope

Focused unchanged-import checks covered policies.qnt:117 full equality,
authorization.qnt:48 map/signing rules, execution.qnt:94 and :227 proof
inventory and atomic updates, and candidate_a_authority_adapter.qnt:38 actual
recomputation. This is not a broader frozen-interpreter audit.
Root runs independent runtime checks; the reviewer did not duplicate them.
Rejection action tests construct tampered starting states, rather than
establishing their reachability from an honest workload.
No Council, full lifecycle, correspondence, bounded-checking or integration
acceptance follows from this review.
