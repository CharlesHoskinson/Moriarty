# s01-intent-theorem-freeze

Status: S01 specification package complete; ten local package predicates passed

This package specifies the architecture-neutral S01 intent-safety gate. The
validator recomputed all ten local package predicates and returned
`recomputed-package-gate-passed`. Task 7 verification and independent review
are complete after the settlement-process terminology correction. All
24 prompt release gates remain open at this S01 boundary. Mechanization, authenticated complete-effect
verification, runtime `SignBeforeResolve` verification, backend and ledger
correspondence, ACTUS work, and human pilots remain specified-only.

## Dependencies

- Prompt version `1.3`.
- The approved S01 design.
- Semantic scope `0.0.0-e00.2`.

## Immutable inputs

- Semantic-scope digest
  `9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`.
- The current atomic-swap trace model.
- Prior immutable evidence and semantic-scope files.

## Exact outputs

This change produces package identifier `s01-intent-theorem-freeze`, ten named
acceptance predicates, the evidence manifest, and a recomputed validation
report. The ten local predicates passed; the result is not a prompt release
gate or a mechanized theorem.

## Scope

Do not change Core. Do not select an S02 architecture. Do not change the prompt,
the approved design, semantic scope, or old evidence.

The local atomic-swap experiment applies only to `SignAfterResolve`. It performs
exact transfer comparison only. It cannot establish authenticated effect
completeness. It cannot authorize signing.

Complete effect projection remains an open gap. The `SignBeforeResolve`
boundaries remain an open gap.

## Initial implementation

The [effect checker](../../../moriarty/intent.py) compares transfer multisets.
The [vector](../../../evidence/s01-intent-theorem-freeze/atomic-swap-extra-effect.json)
contains the canonical settlement and one extra third-party payment.
The [tests](../../../tests/test_intent_verifier.py) reconstruct settlement from Core
and check rejection, restoration, malformed inputs, and duplicate transfers.

The [evidence manifest](../../../evidence/s01-intent-theorem-freeze/evidence-manifest.json)
binds the local S01 inputs, outputs, and limitations. The [execution audit](../../../docs/superpowers/reviews/2026-09-04-moriarty-v1.3-execution-audit.md)
records the original obligations and their dispositions. The
[final verification](../../../docs/superpowers/reviews/2026-09-04-moriarty-s01-final-verification.md)
and [review ledger](../../../docs/superpowers/reviews/2026-09-04-moriarty-s01-review-ledger.md)
record completion. The candidate theorem is unmechanized and the local checker
grants no signing authority. S02 owns the next architecture experiment.
