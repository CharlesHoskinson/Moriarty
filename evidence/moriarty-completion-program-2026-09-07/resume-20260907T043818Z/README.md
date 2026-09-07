# MC01 preparation on explicit resume

Scope: source inspection and bounded readiness checks on 2026-09-07 UTC.
The user requested `resume` after the completion-plan checkpoint.
No implementation package is accepted by this record.

## Execution and audit status

The [live goal observation](goal-status.json) still reports the superseded A4/A5 goal as `usageLimited`.
No second goal-creation request was needed; the unfinished-goal condition remains present.
The new MC01–MC08 loop is not armed.
The available goal tools cannot cancel or replace the old unfinished goal.

One user-triggered [Fable readiness recheck](fable-readiness.json) again returned an explicit usage-credit limit.
The exact model identity remains unverified, and no substantive audit ran.
This is not evidence of missing authentication.
No alternate model or automatic retry was used.

The existing GPT-6 planning verdict retains its candidate-03 scope.
It does not approve new implementation or replace Fable's required verdict.
Native proving and public submissions remain stopped.

## Useful independent preparation

MC01 source inspection identifies the concrete decisions needed before its language-profile freeze.
It separates reusable local behavior from missing units, authoring, numeric, lifecycle, and encoding semantics.
The [source report](source-intake.md), [source manifest](source-manifest.json), and [root intake](intake-verification.json) preserve the findings and exact inputs.
All MC01 implementation and freeze tasks remain unchecked.

The intake found inconsistent accepted names, identifier lengths, and nesting limits across Core and signing encodings.
It also identified the exact-plan `IntentEffects` and outcome `IntentRefinement` claim-version decision.
These are source-level design gaps; no new failing runtime test or implemented fix is claimed.
The first-profile decision packet must resolve them before grammar and semantics freeze.

[Reference calculations](reference-calculations.json) independently recompute the retained loan and constant-product sample formulas.
The local loan profile floors its interest to a micro-USD quantum.
That rounding rule does not establish a comparator for ACTUS reference decimals.
The sample closes its first-period episode with remaining contractual notional; it does not repay the whole loan.
The sample swap covers constant-product exact input only.

Reproduce the arithmetic and source-digest checks:

```sh
python3 evidence/moriarty-completion-program-2026-09-07/resume-20260907T043818Z/verify-reference-calculations.py
```

These checks do not run the evaluator, prove correctness, establish conformance, or submit ledger transactions.
Implementation can proceed through its required gates once the external execution and audit blockers are resolved.
