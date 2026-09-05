# S02 observation carrier: independent code review

Reviewer: native GPT-6 Astra agent `/root/quint_observation_review`, explicit
model selection, high effort. Scope: base `941a6fb7ab1df22508f90a679ba71f780f4bb1e3`
through candidate `ba26098433915a03d4faffe4239303ab77916b71`. This is one
independent source review, not three-vendor Council admission or a passed gate.

The reviewer read the model, harness, tests, plan, frozen Core/swap, and evidence.
It confirmed all fourteen manifest pins and the five command/exit pairs. It
did not repeat the existing test suite. Its focused Rust diagnostic found the
negative-time condition below. The controller then independently reproduced
three failing regression tests; their output is preserved with the correction.

## Findings and dispositions

1. **Important: negative state minimum time admitted.** `validCoreState` omitted
   the `minimumTime >= 0` constructor rule in frozen `moriarty/core.py:129–131`.
   The omission also admitted a preserved rejection at -1 and an accepted
   progression from -2 to -1. The plan had the same omission. Disposition:
   correct the plan and guard after the three new regression tests fail.
2. **Minor: public rollback coverage incomplete.** Most mutation cases directly
   tested `rejectionPreserved`, so deleting checks from the public validator
   could survive them. Disposition: retain helper tests and assert rejection
   through `validCoreObservation` for every changed state, warning, payment,
   effect, and reduction component.

No Critical finding was reported. The reviewer found the complete carriers,
error/warning guards, deposit filtering, explicit state updates, finite example
harness, and claim limitations appropriate. The initial readiness judgment
required correcting the minimum-time guard. This report does not elevate
structural tests to Core execution, correspondence, authorization, or S02 evidence.

Correction verification and any subsequent reviewer disposition are recorded
separately; do not treat this initial review as an approval of unseen edits.
