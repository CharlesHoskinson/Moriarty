# Moriarty restart: S02 semantic alternatives

Decision date: 2026-09-05. Status: restart assessment, not a sprint acceptance.

## Scope reset

The user ended Foreman development in this workstream. Do not dispatch another
Foreman repair, request a successor repair contract, or merge the parked repair
branch as a prerequisite for Moriarty implementation. Preserve its history.

The Foreman repository was fetched and checked with a fast-forward-only merge.
Local and remote main both resolved to
`48f0b6eaed0eb25ee04d053317626eb768b60725`, which includes PR 55.
This observation does not claim that every separate Council-runtime change landed.
No Foreman source changed during this reset.

Keep the requested [Council reviews](COUNCIL_REVIEWS.md) at acceptance boundaries.
They are not waived. Missing review evidence blocks the affected acceptance or
integration, not unrelated draft implementation. Use available released tooling.
If a review route is unavailable, report that specific limitation without starting
another infrastructure development project.

## Preserved starting point

- Moriarty main: `e37dde97847b9300b9f82c0a13f91885397088fd` before this restart record.
- Existing worktree: `.worktrees/s01-audit-start`, branch `s02-model-comparison`.
- Foundation commit: `6a60a645c03acad83b7cbc6b85d43996cd40ca65`.
- Controlling XML: version 1.3, SHA-256
  `86b80dd1cbd14d1e5759988be9f619355495fc10c4e2fb6b6d9162367670ddcd`.
- Frozen semantic scope: `0.0.0-e00.2`. Do not change Core in S02.
- Moriarty has no configured Git remote. Local commits are not GitHub publication.

S01 has local specification evidence, not a mechanized theorem or completed
requested Council backfill. S02 has effect arithmetic and exact-parent
consumption bookkeeping. These foundations do not provide signature authority,
financial recovery, candidate execution, or correspondence.

## Actual implementation queue

1. Apply the completed [delegated three-expert design decision](superpowers/specs/2026-09-05-moriarty-s02-common-design-decision.md), which supersedes the earlier [type sketch](superpowers/specs/2026-09-05-moriarty-s02-authorization-recovery-types.md).
   The user selects GPT-6 Astra, Fable 5.1, and Grok 4.6 as formal-methods experts.
   Their proposals and the explicit majority/disagreement dispositions supply the
   delegated design decision, not an implementation gate or a claim of unanimity.
   Preserve unresolved disagreement instead of manufacturing consensus.
   The recommended boundary is one observation/authorization/recovery foundation.
   No additional human type-sketch approval is required by the latest instruction.
2. Implement common observations, per-principal authorization, separate pre-sign
   and execution checks, and recovery before any fill and after the first fill.
   Preserve transaction-time and complete-state bindings. Model both signing profiles.
3. Implement A: finite agreement interpreter plus a separate intent envelope.
4. Implement B: native obligation graph plus independently checked agreement elaboration.
5. Implement C: independent agreement and intent successors plus a paired-state bridge.
6. Implement D: application-specific Compact library behavior plus actual-effect verification.
7. Implement independent E00 correspondence and the abstraction-map corruption control.
   Compare complete Core results, including funded timeout commit and input rollback.
8. Complete bounded Quint/Apalache checks, witness paths, applicable mutations,
   the closed evidence manifest, and an evidence-backed selection or stop.
9. Complete S02 acceptance review before S03 semantic motions. Retain S01 backfill.

Every candidate must cover both workloads and signing profiles. Preserve all
12 property identifiers, 11 witness identifiers, 14 controls, and both additional
recovery paths. Candidate labels over one shared interpreter are not four alternatives.
Missing implementation or inconclusive checks cannot justify rejecting a candidate.

## Restart checks

These commands inspect the existing baseline. They do not close S02:

```text
# main
uv run pytest -q
uv run python scripts/validate_s01_intent_evidence.py

# existing S02 worktree
quint typecheck specs/quint/s02/effects_test.qnt
quint test specs/quint/s02/effects_test.qnt --match 'Test$'
quint typecheck specs/quint/s02/consumption_test.qnt
quint test specs/quint/s02/consumption_test.qnt --match 'Test$'
```

The restarted checks passed before this documentation change. Later verification
must bind its own candidate and commands. Sampled runs are not exhaustive model
checking. A completed job is not a Council verdict, and a recorded task list is
not an unattended continuation loop.

## Progress reporting

Report Moriarty model files, reached behaviors, checked predicates,
counterexamples, correspondence results, and acceptance evidence. Keep tooling
maintenance out of the progress numerator. S03–S15 and all 24 release gates remain
open until their own evidence is produced.

## Full-program continuation authority

The user explicitly requires the entire XML v1.3 program, not a single sprint.
The target includes S01 Council backfill, S02–S15, all 24 release gates, all 22
XML deliverables, and the no-exclusion 277-fixture ACTUS obligation. Follow the
XML's evidence-backed terminal decision rules. Missing work is not a stop result.

The user resumed the existing full-program app goal on 2026-09-05. A subsequent
goal inspection confirmed `active`. Use that product-owned continuation mechanism.
Do not substitute the old uncontracted Foreman supervisor or an unbounded shell
loop. Foreman development stays closed. An active goal is not a guarantee that
the full program finishes overnight or that external blockers cannot occur.
