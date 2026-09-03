# Moriarty work-package Council advisory

<!-- markdownlint-disable MD013 MD060 -->

- Sprint: S00 instruction and feasibility sprint
- Review status: completed advisory; not a security audit
- Frozen base: `006c4d91ed09c0a89261861b6e7203b3efa3e2df`
- Frozen head: `f702692895e8be811fb9eac2a1c0bfafca5dea70`
- Frozen diff SHA-256: `94f7ede745b5f3f7cee71c82e380fcfb57f28d40294a28b8c9f8b368fc9bea52`
- Review bundle SHA-256: `43c3c36cb42fee4e63aac50f1f5bb9d0dc1a717526da064a298d41921cb32044`

## Council identity evidence

| Seat | Requested identity | Terminal evidence | Result SHA-256 | Outcome |
|---|---|---|---|---|
| Grok | `grok-4.6` | Raw `modelUsage` records `grok-4.6-build`; one model call; `end_turn` | `d1a3d7fd8cc8143a7d77f1220fa726a02f564c91ae63edee9c9ccad6c20a89ce` | changes requested |
| Sol | `gpt-5.6-sol` | Codex terminal banner records `model: gpt-5.6-sol`; the review object's self-label is the non-authoritative string `GPT-5` | `e217f5adfbaa460914fb314765eb139a6ba02fab6bc7f8f399496607467efc0c` | changes requested |
| Fable | `claude-fable-5-1` | Raw `modelUsage` records canonical `claude-fable-5-1` | `8b5df10717dc97198f9134382624f554f552d27d71da7a84e0b750b9f4fe9371` | changes requested |

Fable's raw terminal result also records one `claude-haiku-4-5` helper call with
19 output tokens. This disclosure does not replace the exact Fable reviewer
identity. It remains part of the provenance record.

The first transport attempt failed before review for all three seats. It did not
produce a review and is not counted. The preserved second attempt returned one
structured review from each seat. Each result contains WP01 through WP12 exactly
once.

## Council result

All three seats requested changes. The result does not reject Moriarty. It
rejects the initial work-package instruction set as insufficiently deterministic.
The material findings were consistent across providers:

1. Remove active calendar milestones.
2. Use one normative, machine-checked dependency graph.
3. Give every package exact outputs, commands, gates, and fallbacks.
4. Make semantic-scope changes immutable and journaled by digest.
5. Keep E00 claims at S3 and reproduce them in a clean environment.
6. Separate Compact's disclosure error from Moriarty's disclosure validator.
7. Contract every SDK component and shared wire artifact individually.
8. Treat the prover and proof-parameter provider as untrusted.
9. Implement a minimum SDK safety spine before proof-cost work and audit.
10. Precommit scorecards, measurement rules, and human-data protections.
11. Assign the audited Compact-library baseline to WP10.
12. Require named authority before testnet submission. Exclude mainnet.

## Per-package disposition

| Package | Council verdict | Deterministic remediation |
|---|---|---|
| WP01 | changes requested | Correct status to S3; pin inputs; reproduce cleanly; split two disclosure controls; define closed fallback. |
| WP02 | changes requested | Pin algorithms, ties, precision, and outputs; add genuine-rater privacy and provisional-taxonomy fallback. |
| WP03 | changes requested | Pin source locks, inventory, matrix, vectors, toolchain, correspondence outputs, and exclusion rule. |
| WP04 | changes requested | Add WP01-WP03 dependencies; require immutable scope snapshots, motion records, model results, and journal digests. |
| WP05 | changes requested | Freeze weighted scorecard, three obligations, two-strategy prototype rule, maintainer evidence, and toolchain outputs. |
| WP06 | changes requested | Route all dispositions through WP04; extend the E00 certificate and disclosure validators. |
| WP07 | changes requested | Depend on WP01-WP06; require seven compiled slices; name the six-family fallback if Prediction fails. |
| WP08 | changes requested | Pin the 72-row roster digest; name MR-01 through MR-13; add independent reviewer pseudonyms. |
| WP09 | changes requested | Define 65 component contracts, 28 wire contracts, an untrusted prover boundary, and the minimum safety spine. |
| WP10 | changes requested | Reconcile dependencies; precommit budgets; own the library baseline; bind prover parameters; require testnet authority. |
| WP11 | changes requested | Audit only implemented scope; preregister; add consent, pseudonyms, retention, deletion, and withdrawal. |
| WP12 | changes requested | Remove calendar language; freeze identities and scorecard; define veto, tie, abstention, signing, and fallback rules. |

## Applied instruction revision

The remediation is represented by the following normative artifacts:

- `openspec/work-packages.json` defines dependencies, output paths, validator
  commands, gates, failure outcomes, and selection rules.
- `scripts/validate_sprint_evidence.py` validates instruction consistency and
  declared evidence closure.
- `evidence/semantic-scope/index.json` identifies the immutable active scope.
- WP05 and WP12 contain frozen machine-readable scorecards.
- WP08 names thirteen legacy regression vectors.
- WP09 contracts 65 components and 28 shared data artifacts.
- WP10 owns library-baseline qualification and testnet authorization.
- WP11 contains the research-data and implemented-audit boundary.

Council protocol permits one correction pass against this frozen review. The
project performs deterministic verification after the correction. It does not
reinterpret the initial `changes_requested` results as approvals and does not
run a second Council under the same review contract.

## Remaining gates

The instruction revision does not complete WP02 through WP12. WP01 now has a
clean-environment S3 reproduction receipt. The next language-wide feasibility
claims still require the WP04 scope freeze, a repeated WP01 gate against that
scope, WP06 high-risk composition, and WP07 shared compilation across seven
canonical slices.
