# Design: WP11 independent evaluation

## Context

Moriarty claims value only if qualified reviewers find a measurable assurance
or comprehension advantage over direct Compact libraries.

## Inputs

- WP07 canonical applications.
- WP09 complete SDK specification and available implementation.
- WP10 proof and cost evidence.
- `evidence/wp11/study-preregistration.json` created before recruitment.
- `evidence/wp11/research-data-protocol.json` created before collection.
- `evidence/wp11/audit-scope.json` bound to implemented artifacts.

## Outputs

- `evidence/wp11/audit-findings.json` and independent retest evidence.
- `evidence/wp11/study-results.json`.
- `evidence/wp11/pilot-specifications.json`.
- `evidence/wp11/evidence-manifest.json`.

## Decisions

Keep auditors and study participants outside the implementation authorship
chain. Preserve raw results before adjudication. Separate qualitative feedback
from measured outcomes.
Audit only implemented components. Review specified-only components for design
gaps, but do not count that review as a release security audit.
The research-data protocol defines consent, pseudonyms, data minimization,
retention, deletion, withdrawal, access control, and incident response.

## Failure Handling

Any unresolved critical or high finding blocks the language path. Missing pilot
demand or no measured advantage selects library-only or stop.

## Verification

Check auditor independence and scope. Reproduce remediated findings. Validate
study assignments, exclusions, raw data, and analysis scripts.
Verify that preregistration predates recruitment and collection. Run `uv run
python scripts/validate_sprint_evidence.py --package WP11 --manifest
openspec/work-packages.json`.
