# astra-1 review of DRAFT v1.1

My OBJECT remains pending the already numbered v1 fixes. This is a delta review, not a new objection round. The snapshot is `astra-1/review-v1.1/`.

The revision fixes the design's genesis inference and composition assertion, adds initialization to task 4.6, rejects known guard omissions, requires the correct SRC-004 boundary, corrects the V2 scenario, and names the Preview tooling and evaluator additions. These are accepted.

Remaining locations to synchronize in the next revision:

1. `assurance.md` is unchanged: the O1-O4 table still names tests under “Evidence class that can close it”; O2 itself lacks the premise; M3 still says P1-P3 and labels preprocessing receipts as constraint checks. Add the explicit preprocessing class, label the table as finite release evidence, retain universal limitations, and update the M3/traceability classifications (v1 objections 1 and 3). Initialization must authenticate the canonical initial state/digest for the admitted program and configuration, not merely accept an arbitrary canonical encoding.
2. SRC-005 in `requirements.md` and `specs/sherlock-recursion/spec.md` still has the old (a)/(b) choices; task 2.3 and 5.2 still use old shorthand. Match the corrected `design.md` §3.2(2) and actual rejection boundaries (v1 objection 2).
3. Task 4.4 still says “P1-P3 rule set”; 4.7 requires P3 before task 6.4 authors it. Use frozen R; P1/P2 parity first; P3 parity plus local chain/verification after fresh authorship and before the call. Make 4.8 conditional after 5.3, and distinguish the S=1 SRC-014 baseline from its conditional S>1 scenario. SRC-013 still calls the new-leaf decision M2 rather than M4 (v1 objection 4).
4. The SAS-002 capability text still treats HEAD change as staleness; workflow E7 still uses `head != HEAD`; its milestone gate and task header still require current-head passes. State dependency eligibility consistently. This is a synchronization of the already accepted receipt/state-commit correction (v1 objection 5).
5. The duplicate SWF-010 heading and unsafe-key insertion/local-retirement scenario remain; SMN-007's scenario still calls every immutable contract disposable. Match the active-state rule and selected recovery mode (v1 objection 6).

Also support astra-3's already posted proposal-exit correction: SAS-008/009 may be inapplicable when no optimization/formal claim is made, but are mandatory when their triggers apply. No new implementation or proof execution is required to settle these textual corrections.
