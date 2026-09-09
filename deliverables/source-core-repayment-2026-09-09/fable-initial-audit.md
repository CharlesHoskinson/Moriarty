# Fable 5.1 independent audit: bounded .mori → Core → funded repayment candidate

**Verdict: PASS** for the stated bounded scope. No Critical, High or Medium findings. One Low finding and several evidence limitations are listed below. Scope confirmed: the candidate produces local `Prepared` candidates from an unauthenticated supplied projection only. Nothing here constitutes successor freeze, K, native, signing, proof or ledger acceptance, and the spec in `funded-source.md` states that explicitly.

I did not run any tests or tools. Everything below is from inspection of the supplied source and the supplied evidence text.

## What checks out

- **Runtime typed bindings.** Argument admission in `evaluate.ts:15-29` requires an exact `{type, value}` shape, exact `{kind}` or `{kind, name}` type shape, canonical UInt128 text for Debt/Amount/UInt, JSON booleans for Bool, and declared names for Party/Asset. Settlement-unit binding happens at lowering (`evaluate.ts:55-56`), and nominal-unit binding happens against the kernel-validated obligation denomination after kernel success (`evaluate.ts:68-71`). The documented precedence in `funded-source.md` matches the code order.
- **State preservation.** The adapter passes `input.state` untouched into the retained kernel (`evaluate.ts:66`), and the kernel's `copyState` preserves order, unrelated obligations and tombstones. The preservation test and probe both assert an unrelated obligation and prior tombstones survive.
- **Gross cash versus nominal debt.** The cash 40 / debt 30 case in `independent-financial-cases.json` debits balance and allowance by 40 and discharges 30. That follows directly from the kernel's `applyTransfer` and `applyRepay`, and no refund path exists. The changed-conversion test with mantissa 2 correctly expects settlement 60 against nominal 30.
- **Ordered effects.** Source emission order is preserved in Core (`elaborate.ts`, `emissions` map), and the kernel emits one effect per action in action order.
- **Malformed and forged Core.** `prepareSuccessor` accepts only a source string and recompiles on every call. Core has no execution or deserialization entry point. A `core` key in the invocation is rejected by the closed top-level shape (`evaluate.ts:39`). A serialized Core passed as source is not valid `.mori` syntax.
- **Source selection.** The recipient-mutation probe changes `to: Lender` to `to: Payer` and rejects via kernel `SELF_TRANSFER`, which shows execution follows the source rather than a canned result. The multi-action test selects `smaller` and gets principal 80, confirming the selected action is executed and all actions are statically checked.
- **Closed shapes and bounds.** Every invocation record uses `closed()` exact-key matching with `Object.hasOwn`, so `__proto__` and extra keys reject. Identifier and UInt128 helpers in `core.ts` are full-string anchored and bounded. Emission count is capped at 128 before lowering. The invocation is length-checked in UTF-16 first, then UTF-8, with short-circuit to avoid encoding oversized input.
- **Rejection leakage.** Every rejection path in `evaluate.ts` and `simulate-cli.ts` returns a literal `{status, code, actionIndex}`. The tentative kernel candidate on a `NOMINAL_UNIT` mismatch is discarded and never exposed. Both test files assert absence of `post` and `effects` on rejection.
- **Kernel semantics.** `repayment.ts` is not in the candidate manifest and is supplied as a retained reference with its own hash. The adapter calls only its public `prepareRepayment` with the retained schema version. I found no path that alters kernel behavior.
- **Expected values.** I hand-checked all six financial cases and every coded rejection expectation in the test file against the kernel rules. All are consistent, including `EXCEEDS_OUTSTANDING` preceding funding checks, `INSUFFICIENT_UNALLOCATED` for cash 20 / debt 30, and `OVERFLOW` on the maximal mantissa before division.

## Findings

**Low. Invocation JSON admission is weaker than kernel admission.** `evaluate.ts:38` uses plain `JSON.parse`, then re-serializes the parsed state at `evaluate.ts:66`. Duplicate keys, whitespace and non-minimal number forms in the invocation are silently normalized last-wins before the kernel's compact-roundtrip check runs, so that check can no longer reject them. The effective state is still fully validated by the kernel closed schema, so there is no financial impact. The spec's statement that state is "passed intact" is slightly overstated. Minimal reproduction:

```json
{"schemaVersion":"moriarty-funded-source/0","action":"pay","arguments":{...},"state":{...A...},"state":{...B...}}
```

This prepares against state B with no rejection. Recommend either documenting the normalization or applying the kernel's compact-roundtrip rule to the invocation.

**Info. Transfer-only actions prepare.** A source action with only a Transfer emission produces a `Prepared` candidate that moves cash without debt. The kernel spec calls this legal, but `funded-source.md` does not mention it. Documentation only.

## Limitations

- **`frontend.ts` is not in the candidate or reference.** The claims that a hostile non-string source rejects with `SOURCE_TYPE` without coercion, that `SOURCE_BOUND` fires before parsing, and that `SuccessorSyntaxError` carries a stable `code` all depend on that file. The hostile-source assertions in both test files exercise this path, but I cannot verify the implementation, nor confirm the parser is unchanged, since no hash for it is supplied.
- **Test evidence is partial.** The summary shows 235 passing tests across 7 suites, but the only test names listed are the final twelve from a syntax suite. No output from `successor-source-repayment.test.mjs` is shown. The "24 independent source probes" claim matches my count of assertions in `independent-source-probes.mjs`, but no captured stdout is provided. The real CLI run is asserted in prose only.
- **`AGENTS.md` and the assignment file** are hashed in the manifest but not supplied, so I cannot confirm the candidate satisfies any constraints stated there.
- **Kernel hash provenance.** I can confirm the adapter does not modify `repayment.ts`, but I cannot independently confirm the supplied reference hash matches the retained baseline.

Per the audit rules, these missing items are limitations, not approval. The PASS covers the inspected code and the internal consistency of the supplied evidence, within the bounded local `Prepared` scope only.
