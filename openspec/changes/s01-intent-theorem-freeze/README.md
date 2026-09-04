# s01-intent-theorem-freeze

Status: specified-only and in-progress

This package specifies the architecture-neutral S01 intent-safety gate. The ten
acceptance predicates are requirements, not passed results. Later tasks must
create the evidence and run the validator before the package can pass.

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

This change produces package identifier `s01-intent-theorem-freeze`. It
specifies ten named acceptance predicates. It does not yet produce passed gate
results.

## Scope

Do not change Core. Do not select an S02 architecture. Do not change the prompt,
the approved design, semantic scope, or old evidence.

The eventual atomic-swap checker applies only to `SignAfterResolve`. It performs
exact transfer comparison only. It cannot establish authenticated effect
completeness. It cannot authorize signing.

Complete effect projection remains an open gap. The `SignBeforeResolve`
boundaries remain an open gap.

