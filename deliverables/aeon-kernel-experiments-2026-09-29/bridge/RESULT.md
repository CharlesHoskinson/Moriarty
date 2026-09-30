# Aeon bridge refund experiment

Date: 2026-09-29. Evidence kind: **bounded authoring experiment**. This result does not establish bridge operation, external finality, a Moriarty compiler judgment, or a ledger proof.

## Source and execution

- Primary tool source: [alcides/aeon](https://github.com/alcides/aeon), local checkout `/home/charl/.graphify/repos/alcides/aeon`, `origin=https://github.com/alcides/aeon.git`, commit `ef66bd95e6b7d2d5309453ee63640bc7fa1d988f` (full hash, previously pinned in the Moriarty Aeon study). Local checkout version: AeonLang 4.9.0; Z3 5.1.0.0; cvc5 1.4.0. The [project README](https://github.com/alcides/aeon) describes refinement types and SMT checking. The checkout is the executed source.
- Exact command, from `/home/charl/Moriarty/.worktrees/mil2-primary-research-20260929`:

  ```sh
  /home/charl/.graphify/repos/alcides/aeon/.venv/bin/python deliverables/aeon-kernel-experiments-2026-09-29/bridge/probe.py > deliverables/aeon-kernel-experiments-2026-09-29/bridge/results.json
  ```

- Driver configuration: SMT synthesizer, synthesis budget 0, `no_main=True`, `strict_decidable=True`. The program calls `AeonDriver.parse` separately for each source term and retains full errors and trust reports. It uses no `native` escapes. Reproduction script: [probe.py](probe.py). Raw judgments: [results.json](results.json).
- Script SHA-256: `9fcacbad85a2a40c487ed99aa81512dd94f07e58aa3d2dfbd143979385ba0feb`. Raw judgment SHA-256: `df45213c73d6b8936c3b233d0fa53da2e02de142268e10aaff78e339be40bab4`.

## Tested predicate

For a fixed source amount `sent`, actual destination delivery `delivered`, and authorized source refund `refund`, the experiment checks `sent > 0`, `0 <= delivered <= sent`, and `0 <= refund` with `delivered + refund <= sent`. This is an intentionally small conservation condition. Integer amounts are mathematical integers in this Aeon fragment; no token scale, rounding, fee, supply, representation ratio, or foreign chain semantics are included.

The probes use status code `1` for **unknown** as an uninterpreted operational value. No status code proves delivery, nonreceipt, finality, or refund eligibility. The separate `observed_delivery` variable represents an observation that may lag actual delivery. `timed_out` is a Boolean with no justified relationship to actual delivery.

| Case | Aeon result | Material observation |
| --- | --- | --- |
| Withhold refund while status is unknown | Accepted | Refund 0 preserves the bounded accounting relation. |
| Full refund on timeout alone | Rejected | Counterexample `sent=1, delivered=1`. Timeout supplies no nonreceipt fact. |
| Full refund on unknown status | Rejected | Counterexample `sent=1, delivered=1`. |
| Full refund when observed delivery is zero | Rejected | Counterexample `sent=1, actual_delivery=1`. An empty observation cannot establish nonreceipt. |
| Full refund after positive partial delivery | Rejected | Counterexample `sent=1, delivered=1`. |
| Refund only `sent - delivered` | Accepted | Arithmetic holds if the supplied `delivered` is complete and authoritative for the claim. |
| Full refund with `delivered = 0` input refinement | Accepted | The checker trusts the input refinement; it does not verify a foreign chain. |

## Interpretation and limit

**Experiment observation:** Aeon rejects refund policies that violate the stated conservation refinement. It supplies concrete arithmetic counterexamples for the unsafe cases. The positive judgments depend on their preconditions. In particular, a bridge adapter cannot turn `observed_delivery=0` or a timeout into the stronger `delivered=0` premise. The `refund_remaining` case also requires a final, complete delivery count; if additional delivery can arrive after refund authorization, this simple relation is insufficient.

**Recommendation:** The MIL/4 kernel interaction profile should keep `unknown` as a live state, use an evidence-qualified final delivery count or exclusivity certificate before source refund, and ensure refund plus cumulative destination delivery stays within the authorized entitlement. The profile should carry claim identity, finality policy, cost, and late-delivery handling. Aeon can check the arithmetic policy at authoring time after its premises are supplied. The bridge adapter and verifier must justify those premises separately; the experiment cannot certify live bridge finality, prevent replay, or prove exact correspondence to Moriarty/Midnight.
