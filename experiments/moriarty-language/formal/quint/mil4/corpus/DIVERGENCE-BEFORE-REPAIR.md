# S0 Quint divergence witnesses

**Target:** frozen `../s0.qnt` candidate, Quint 0.32.0.  
**Corpus:** `s0_divergence_witnesses.qnt`.  
**Scope:** two fixed witness traces. The four external premises are stipulated
in the import, not established by these runs.

From the checkout root:

```text
$ quint typecheck experiments/moriarty-language/formal/quint/mil4/corpus/s0_divergence_witnesses.qnt
(no output; exit 0)

$ quint test experiments/moriarty-language/formal/quint/mil4/corpus/s0_divergence_witnesses.qnt --main s0_divergence_witnesses --match '.*Test' --seed 0x5 --backend typescript

  s0_divergence_witnesses
    ok overflowAndMissingFeeTest passed 1 test(s)
    ok repaymentAliasTest passed 1 test(s)

  2 passing (80ms)
(exit 0)
```

| Witness | Two competing conditions | Observed first Quint decision | Relevant K/TypeScript decision |
| --- | --- | --- | --- |
| `overflowAndMissingFeeTest` | Recipient has `UINT128_MAX`, so credit 10 overflows; submitted vector omits fee credit 1 | `Effect`, `S0_EFFECT_MISMATCH` | K and TypeScript check effect range before vector mismatch, so the intended first result is `Effect`, `S0_EFFECT_RANGE` |
| `repaymentAliasTest` | Signed payer and obligation creditor are both `O`; valid debt and account cells otherwise | `Intent`, `S0_INTENT_ALIAS` | K rejects this shape at `Stage`; the Source formation expectation also excludes it. TypeScript currently reports `Effect`, `S0_EFFECT_RANGE` |

Each test checks the decision and a blocked submit action. The decision is an
observation in the pre-state. Quint's `fail()` does not produce a committed
successor. These sampled traces expose judgment order; they do not prove
general equivalence or ledger behavior.

## Proposed precise model patch, not applied

1. In `beforeEffect`, remove `lines != expected` from the early effect checks.
   Keep the generic numeric range check there. In both `transferObservation`
   and `repayObservation`, evaluate the current operation-specific range branch
   first, then compare `lines != expected` and return
   `reject(Effect, "S0_EFFECT_MISMATCH")`. This makes range win when both are
   invalid, while retaining the same result for a vector-only mismatch.
2. In `repayObservation`, add `bound.payer == o.creditor` to the existing
   obligation stage-shape rejection after loading `o`, before `beforeEffect`.
   This aligns the self-creditor shape with K's `stageShape` check. Keep the
   existing transfer endpoint alias check at intent. The shared
   `beforeEffect` repayment alias branch can then be removed or restricted to
   transfers, because a bound repayment self-creditor shape will already have
   stopped at stage. Use the **bound** payer in the stage predicate so a
   malicious supplied record is still handled by the later signed-record
   equality check.

The TypeScript repayment result differs from K's stage result. A single Quint
first judgment cannot match both. The proposed Quint patch follows K and the
Source formation expectation; a separate cross-layer decision is needed for
the TypeScript result. No frozen model or existing corpus file was edited.
