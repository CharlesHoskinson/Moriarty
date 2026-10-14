# FOREMAN report: SP01 frontend diagnostic priority

Role: implement. Worktree: `sp01-frontend-grok`. This worker did not run tests.

## Owned files

- `experiments/moriarty-language/src/validate.ts`
- `FOREMAN_REPORT.md`
- `FOREMAN_REPORT.json`

No frozen test, checker, parser, schema, or bounds file was edited.

## Parent typecheck failure

Parent ran all 33 frontend tests. All 33 passed.

Actual `tsc` 7.0.2 failed:

```
experiments/moriarty-language/src/validate.ts(44,89): error TS2339: Property 'name' does not exist on type 'SourceDeclaration'.
```

Cause: `StateDecl` shares a union member with `ConstDecl`. Array `filter` does not produce a narrowed array element type. The `.map(d=>d.name)` access is therefore invalid on `SourceDeclaration`.

This worker did not run typecheck, tests, or any other check. Parent reruns verification.

## Exact edits

Three reviewed changes were applied in `validate.ts`. `validateSource(ast,bounds,through)` and the DiagnosticCode registry are unchanged.

1. Stage 5 now records every `StateDecl` name in `declaredStates` before traversal. `EpisodeStatusDecl` and `NotionalStatusDecl` emit `NAME_RESOLUTION` when the field exists later in the source and is not yet in `states`. Globally absent fields still fall through to stage 6 `STATUS_RULE`. Episode literals still call `literalUse`. Other stage 5 collection and `firstDiagnostic` sorting are unchanged.

2. Stage 6 policy coverage now keeps `deferredCoverage` next to `covered`. Before each target key, an `Effect` target with an invalid ordinal calls the existing `uint` helper. That path records `UINT_RANGE` and defers missing-coverage only for Amount emits with the same action, effect label, and unit. The loop then continues. Independent target, rounding, write, other-action, and other-unit checks still run. The final uncovered Amount condition excludes `deferredCoverage`. There is no global invalid-ordinal shortcut.

3. The `declaredStates` construction was replaced only. The previous `filter` then `map` of `d.name` is now `flatMap(d=>d.tag==='StateDecl'?[d.name]:[])`. The conditional narrows inside the callback. The collected names are the same `StateDecl` names. All other source bytes are unchanged.

Policy declaration may occur before or after the covered action. Both cases still resolve after all actions exist.

## Limitations

This worker did not run tests, Git writes, agents, installs, web work, native proofs, or wallets.

This worker does not claim new passing checks. Parent reruns verification. Independent GPT-6 review follows.

These edits change diagnostic priority and one TypeScript narrowing form only. They do not widen the accepted language. Compile returning a record remains frontend acceptance only.

## Parent red receipt

Parent recorded three substantive failures on frozen `frontend.test.mjs` against the previous `validate.ts`.

- episode status: `TYPE_MISMATCH` instead of `NAME_RESOLUTION`
- notional status: `TYPE_MISMATCH` instead of `NAME_RESOLUTION`
- ordinal overflow: `POLICY_TARGET` instead of `UINT_RANGE`

After those two diagnostic-priority edits, parent recorded that all 33 frontend tests passed. Parent then recorded the `tsc` 7.0.2 failure above. This correction addresses that typecheck failure only.
