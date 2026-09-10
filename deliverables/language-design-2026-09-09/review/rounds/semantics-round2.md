# Semantics convergence, round two

GPT-6 Codex subagent `/root/pl_design_semantics`; inherited model suffix not independently exposed. Reviewed all three round-one opinions and final research at `b7c47f55b346a217a5f8d6e958dcaac12136cf2c`, including the report's added citations. Recommendation only, S2; checks remain specified-only. No product edits or dispatch. Agreement below is this seat's position, not an assertion that peers accepted these resolutions.

## Canonical dispositions

| ID | Disposition | Position |
|---|---|---|
| F01 | adapt | Agree: authoritative TypeScript-style `.mori`; explicit semantic divergences. |
| F02 | defer | Agree; require non-executing extraction, not just TS typechecking. |
| F03 | reject | Agree: unrestricted host runtime excluded from trusted semantics. |
| F04 | adopt | Agree: explicit nominal financial interfaces. |
| F05 | adapt | Agree: unique, visible local inference; no mandatory repeated annotations. |
| F06 | adopt | Agree; prohibit every `next` read and duplicate field write. |
| F07 | adapt | Agree; separate effect summaries, agreement policies and signed instance authority. |
| F08 | defer | Agree: general resumable handlers need separately justified resource semantics. |
| F09 | adapt | Agree; bounded events/schedules express useful reactivity without promising execution. |
| F10 | reject | Agree; aggregate lifetime work across successors, including splits/joins. |
| F11 | adopt | Agree; domain conservation and authorized discharge, without requiring generic linear syntax. |
| F12 | adopt | Agree; reject policy violations, not every payer-favorable rounding rule. |
| F13 | adapt | Agree; evaluate safe repair and truthful status separately. |
| F14 | adapt | Agree; full references, bounded dependency closure, specified canonicalization. |
| F15 | adopt | Agree; distinguish actual output bytes from build recipe and proof statement. |
| F16 | defer | Agree; ordinary files suffice. Reject permanent exclusion absent evidence. |
| F17 | adopt | Agree; migration cannot reset duty, authority or consumed history. |
| F18 | adopt | Agree; Midnight Preview target fixed, financial acceptance unfinished. |
| F19 | adopt | Agree; formative work first, confirmatory comparison after stabilization. |
| F20 | adopt | Agree; versioned grammar and separate correspondence obligations. |

## Six resolutions

1. **Duties:** agree with Opus's conservation objective and usability's automatic carry-forward. Use a bounded duty map with stable identity, parties, denomination, amount, due condition and priority. A domain rule carries unchanged duties and explicitly discharges/transforms affected ones. For principal-only repayment, old principal equals funded allocation plus successor principal; interest, fees, forgiveness and restructuring require their own authorized relations. Generic linearity alone proves neither that equation nor creditor correctness; affinity permits unwanted discard. Static checks reject structurally missing handling where decidable; value-dependent conservation remains preparation/proof work. Remove “unwriteable” claims. Cancellation/expiry cannot delete debt.

2. **Authority:** agreement declarations specify supported policies and authorization requirements; signed authority is a separately authenticated runtime input, bound to the execution statement. Require its quantitative effect on the proposal display. Do not yet require every `emit` to name an authority variable or place mutable nonce/counters in source. A header alone cannot establish maximum economic loss. Preserve outcome-intent mode without an extra exact-plan signature.

3. **Updates and work:** retain single assignment and no `next` reads; named locals express dependencies. Charge accepted work under a versioned model: consumed work plus all live successors' remaining allowance cannot exceed the originally admitted allowance. Split partitions; join combines only distinct eligible residuals; recovery reserve is included, not newly minted. Bound compiler, data and proof resources separately. Static representability and closure reserves do not establish scheduling liveness, witness availability or settlement.

4. **Borrowing and identity:** Elm-style pure messages/proposals and finite scheduled financial events remain positive features. Unison-style names resolving to immutable dependencies remain positive features. Keep ordinary files and expandable identity details. A compiler change changes build identity; whether Core changes depends on actual elaboration, not an unconditional stability promise. Neither identical hashes nor text review proves correct lowering.

5. **Correct overclaims:** rounding direction is agreement-policy-dependent, never universally payer-hostile. Typechecking an interpolated template cannot prevent prior JS evaluation; future extraction must parse without executing the module, prohibit interpolation and specify raw/cooked bytes. U02 includes financial smart-contract authoring; it does not test Moriarty or justify transferred effect sizes. Midnight Preview is already the target. Useful diagnostics need no unsupported claim that Elm guarantees fully considered programs or that a language server is necessarily cheaper.

6. **Complete example before grammar freeze:** publish an explicitly illustrative data-level fixture alongside the source fragment, not invented admitted syntax. Require the following concrete values and bindings:

```ts
// Proposed fixture data; no signature, proof or settlement is asserted.
{
  target: "Midnight Preview", program: "principal-payment-v0",
  instance: "loan-1", predecessor: "state-0",
  asset: { id: "usd-fixture", unit: "USD", quantum: 1 },
  parties: ["borrower", "lender"],
  pre: { borrowerCash: 100, lenderCash: 0, debt: 100 },
  duty: { id: "debt-1", obligor: "borrower", obligee: "lender",
          asset: "usd-fixture", principal: 100, due: 10 },
  policy: { allocation: "principal-only", interest: 0, fee: 0 },
  observation: { id: "time-1", provider: "fixture-clock", time: 1, expires: 10 },
  authorization: { mode: "outcome", principal: "borrower", nonce: "n1",
    grossCap: 40, priorGross: 0, feeCap: 0, recipient: "lender",
    minNet: 30, newDebtCap: 0, expires: 10, partialFill: true },
  action: { pay: 30 },
  expected: { status: "Prepared", borrowerCash: 70, lenderCash: 30,
    transfer: ["borrower", "lender", "usd-fixture", 30],
    allocation: ["debt-1", 30], residualDebt: 70, residualGross: 10,
    predecessorConsumption: "proposed-exclusive", work: 96 },
  bounds: { lifetimeWork: 100, stepCharge: 4, recoveryReserve: 10,
            horizon: 10, observations: 1, queue: 1 }
}
```

Bind those fixture names to exact nominal, custody, observation-authentication, profile/encoding and claim/verifier references in its manifest. Specify complete ordered effects and unchanged duty fields, authority-consumption successor and rejection outputs. All four mandatory claims remain required for acceptance. Add independent wrong-recipient, unfunded-payment, deleted-duty, stale-observation and excess-gross cases. These fixture requirements do not claim an executable encoding.

## Input SHA-256

```text
4405f003e889d2e19889f329ea0dccd2cd1cdd95ba17cbecdb6d91d9e18cbac7 REPORT.md
7c83ef1d778714ebe4964c9d3f95d68be50d20f8edfd0fb6ebe07721abb00190 FEATURE-CHECKLIST.md
10b44dfc33064c395665a5438cad92791aecfc65280e54c12fc7426b7bf739d0 features.json
b74e867c78035514458bbcfb30ae9b9876220183e71c9bb4770a55538ec99c69 opus-round1.md
bbeb7b3366277a98b96e31aeffed29d1a29e10542afa08cec892eb2346b051e7 semantics-round1.md
c9d4f9d2f3b20a0696ae2632530d149708c8ae94e5262debf02a81c57bce74c4 usability-round1.md
```
