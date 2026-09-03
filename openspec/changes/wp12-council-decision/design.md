# Design: WP12 terminal council decision

## Context

WP01 through WP11 create independent stop gates. The terminal decision must use
their frozen results without allowing a reviewer to redefine acceptance.

## Inputs

- Immutable outputs and gate results from WP01 through WP11.
- The current semantic scope ledger and contradiction register.
- `decision-scorecard.json`, frozen before the terminal review.

## Outputs

- `evidence/wp12/review-bundle-manifest.json`.
- `evidence/wp12/provider-reviews.json` for Grok, Sol, and exact Fable 5.1.
- `evidence/wp12/dissent-register.json`.
- `deliverables/moriarty-terminal-decision.json` and its minisign signature.

## Decisions

The decision authority considers Council advice, but owns the result. Provider
failure is an abstention. It is never approval. Prior stop-gate failures remain
binding.
Apply the scorecard eligibility, veto, selection order, tie, and abstention rules
without reviewer discretion. Verify Grok through `modelUsage`, Sol through the
Codex terminal banner, and Fable through its canonical `modelUsage`. Preserve
all auxiliary-model disclosures.

## Failure Handling

If evidence is insufficient, the language option fails. The decision selects
audited libraries or stop. It does not select indefinite exploration.

## Verification

Verify provider and model terminal evidence. Verify review-bundle identity.
Check every score against preserved evidence and every unresolved dissent.
Verify the decision with the Ed25519 minisign public key at
`governance/decision-authority.pub`. Run `uv run python
scripts/validate_sprint_evidence.py --package WP12 --manifest
openspec/work-packages.json`.
