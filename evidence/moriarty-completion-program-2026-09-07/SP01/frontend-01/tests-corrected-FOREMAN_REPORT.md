# FOREMAN report: SP01 frontend tests-only

Role: implement. Worktree: `sp01-frontend-grok`. This worker did not run tests.

## Owned files

- `experiments/moriarty-language/tests/frontend.test.mjs`
- `FOREMAN_REPORT.md`
- `FOREMAN_REPORT.json`

No implementation file was edited.

## Tests added

Seven tests with names that start `SP01 frontend` were appended. Existing tests were kept. New helpers use the `sp01` prefix.

Each distinguishing rejection uses `compile` and `error.diagnostic`. The tests check `code` and `stage`. NAME_RESOLUTION and overflow UINT_RANGE also check `primarySpan`. Spans come from unique ASCII substring offsets. The tests do not call parser or diagnostic helpers for those offsets.

1. Premature episode status plus a later invalid guard must return NAME_RESOLUTION stage `5` on the full `status episode` declaration. A valid guard keeps NAME_RESOLUTION. State before status turns the bad guard into TYPE_MISMATCH stage `6`. A globally missing field with a valid guard returns STATUS_RULE stage `6`.
2. The same controls apply to `status agreement remaining_notional principal`. Unit `U` is declared first. Later `state principal: Amount<U>` is present unless the missing-field case omits it.
3. Policy before and after its action: ordinal `0` compiles. Ordinal `1` and `2^128-1` return POLICY_TARGET stage `6`. Ordinal `2^128` returns UINT_RANGE stage `6` on `effect(run,ORDINAL,amount)`.
4. Policy after action with an earlier `guard uint(1)` still returns TYPE_MISMATCH stage `6`.
5. Policy after action with an earlier uncovered `set principal` still returns POLICY_TARGET stage `6`.
6. An uncovered emit in action `other`, or an emit in unit `V`, still returns earlier POLICY_TARGET. Observation `now` is present.
7. `write(run,missing)` before an overflowing effect target in the same policy returns earlier POLICY_TARGET. The action body is empty so no earlier Amount occurrence exists.

Registered `spec/bounds.json` bytes are unchanged. Compile returning a record is frontend acceptance only. These fixtures do not prove evaluator settlement or Compact support.

## Fixture adjustments

The notional production in `grammar.ebnf` is `status agreement remaining_notional` plus an identifier. The tests keep that spelling.

The original covering-zero adjustment was rejected. A covering valid target `effect(run,0,amount)` bypasses the required uncovered-emit behavior when the invalid ordinal's action, field, and unit match. Dependent uncovered-emit coverage must stay deferred narrowly in that case.

The helper `sp01PolicyOrdinalSource` now takes only `ordinal` and `policyAfterAction`. Policy-before and policy-after overflow fixtures both use the single-target source `effect(run,ORDINAL,amount)`. Other tests keep unrelated earlier errors. Expected UINT_RANGE is unchanged.

## Verification

This worker did not run tests, Git writes, agents, installs, or web work. The parent runs focused RED against unchanged source and freezes the test digest.
