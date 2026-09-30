# MIL/4 S0 finite witness results

**Checkout:** `mil2-primary-research-20260929`  
**Quint:** 0.32.0  
**Model:** `../s0.qnt`  
**Corpus:** `s0_witnesses.qnt`

This is the diagnostic-unit repair rerun of the unchanged fifteen-witness corpus. The
[previous result](RESULTS-BEFORE-REPAIR.md) is retained. Exact command outputs
and candidate SHA256 values are in
[diagnostic-command-results.json](diagnostic-command-results.json). The fourteen
additional ordering witnesses are reported in
[DIVERGENCE-RESULTS.md](DIVERGENCE-RESULTS.md).

The corpus stipulates `signatureVerified`, `authenticatedSnapshots`,
`nativeQualified`, and `ledgerAtomicReady`. These are assumptions supplied to
the model, not observations of a signature, proof, snapshot, or ledger.

From the checkout root, the final commands and output were:

```text
$ quint typecheck experiments/moriarty-language/formal/quint/mil4/corpus/s0_witnesses.qnt
(no output; exit 0)

$ quint test experiments/moriarty-language/formal/quint/mil4/corpus/s0_witnesses.qnt --main s0_witnesses --match '.*Test' --seed 0x5 --backend typescript

  s0_witnesses
    ok transferTenOneTest passed 1 test(s)
    ok repayThirtyTest passed 1 test(s)
    ok repayNearBoundTest passed 1 test(s)
    ok recipientSubstitutionTest passed 1 test(s)
    ok feeCapTest passed 1 test(s)
    ok grossCapTest passed 1 test(s)
    ok missingFeeTest passed 1 test(s)
    ok missingCreditorTest passed 1 test(s)
    ok wrongCreditorTest passed 1 test(s)
    ok allowanceTest passed 1 test(s)
    ok staleHeadTest passed 1 test(s)
    ok replayTest passed 1 test(s)
    ok overflowTest passed 1 test(s)
    ok overpayTest passed 1 test(s)
    ok missingFeeAndStaleTest passed 1 test(s)

  15 passing (114ms)
(exit 0)
```

Each `run` is one fixed trace. The positive traces check the complete effect
vector and resulting cells. The hostile traces check the proposed first
judgment and that the submit action is blocked. Quint's `fail()` ends its test
trace, so no later state is sampled after the failed action. An action that is
blocked has no committed successor in this model. The tests are finite witness
executions. They are neither exhaustive model checking nor K/Quint
correspondence, native proof, or ledger acceptance evidence.

## Expected outcome comparison

| Discriminator | Corpus witness | Observation |
| --- | --- | --- |
| T-10-1 | `transferTenOneTest` | 11 debit, 10 and 1 credits, allowance/replay/head effects and state observed |
| R-30 | `repayThirtyTest` | P980/I0/outstanding980 and creditor credit 30 observed |
| R-near-bound | `repayNearBoundTest` | Creditor and spent reach UInt128 maximum; debt reaches S−1 |
| H-recipient | `recipientSubstitutionTest` | Intent rejection; blocked submit |
| H-fee-cap | `feeCapTest` | Signed fee exceeds signed fee cap; intent rejection |
| Additional gross cap | `grossCapTest` | Signed gross cap below 11; intent rejection |
| H-missing-fee | `missingFeeTest` | Effect rejection; blocked submit |
| H-missing-credit | `missingCreditorTest` | Effect rejection; blocked submit |
| H-wrong-creditor | `wrongCreditorTest` | Effect rejection; blocked submit |
| H-allowance | `allowanceTest` | Authority rejection; blocked submit |
| H-stale-head | `staleHeadTest` | History rejection; blocked submit |
| H-replay | `replayTest` | History rejection of a new signed digest with the same replay key at the current head |
| H-overflow | `overflowTest` | Effect rejection; blocked submit |
| H-overpay | `overpayTest` | Effect rejection with signed amount 1011 and sufficient funds |
| H-both | `missingFeeAndStaleTest` | Effect rejection precedes stale history check |

The H-replay witness uses a second signed digest at head 1 with the same
domain, signer, and nonce as the committed first digest. Submitting the exact
old intent at head 1 reaches stale head first under the model's judgment order.

## Limits and mismatch

H-nominal expects a **Source/6 admission** rejection for a source nominal
above `S=2^127−1`. This Quint model starts after source admission and has no
source admission action. It checks signed nominal bounds at its intent
judgment, so a model-level oversized signed amount would report intent. That
is a scope mismatch between this model and the full discriminator, not evidence
that Source/6 admission is correct. A separate Source/6 witness is required.

The default Rust evaluator cannot execute this model because `UINT128_MAX`
exceeds i64. Its first attempted run reported:

```text
$ quint test experiments/moriarty-language/formal/quint/mil4/corpus/s0_witnesses.qnt --main s0_witnesses --match '.*Test' --seed 0x5
Error [QNT600]: Integer literal 340282366920938463463374607431768211455 is outside i64 range and is not supported by the Rust evaluator.
(exit 1)
```

The TypeScript backend executed the fixed witness values, including the
UInt128 edge. No `quint verify` model check was run: the current model has
unbounded integer and string state and no bounded `step` relation or invariant
configured for exhaustive checking.

## Diagnostic observation update

Rejected decisions now report `diagnosticWork=1`, one abstract diagnostic unit.
Accepted observations retain zero. The diagnostic observation does not debit
committed `workRemaining` or `workSpent`. Representative full Stage, Effect
and History decisions are exercised in
[DIAGNOSTIC-RESULTS.md](DIAGNOSTIC-RESULTS.md). The command record linked above
captures the current diagnostic-unit candidate. The earlier ordering-only
candidate and outputs remain in
[repair-command-results.json](repair-command-results.json).
