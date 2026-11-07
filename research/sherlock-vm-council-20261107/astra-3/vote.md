# VOTE astra-3 ACCEPT — DRAFT v2.1

I ACCEPT the Sherlock VM consensus design v2.1. This supersedes my OBJECT votes on v1 and v1.1. I have no remaining design, soundness or security objection.

The reviewed 13-file snapshot is `draft-v2.1/`, identified by `draft-v2.1.sha256`. Before this vote, `sha256sum -c` against the live change reported all 13 files matching. Read-only ID comparisons found all 56 requirement IDs in both the capability specifications and the traceability table, with no missing or extra IDs. These checks establish the reviewed document identity and ID coverage; they do not establish implementation conformance.

My objections are resolved in the design text:

1. Genesis alternatives and tests agree across design, SRC-005 and tasks. The accumulator hash observation is not presented as a security proof. Canonical authenticated initialization, preservation of `StepInv(pre)` and shared-carrier premises are explicit obligations (SVM-002, SAS-010, tasks 4.6/4.9).
2. IR preprocessing, keyed constraint checks, native cryptography and target observations have separate evidence classes. A synthesis error cannot satisfy constraint rejection, and a partition known to omit a relevant guard dependency cannot satisfy its acceptance receipt (SAS-003, SAS-013/014). Finite release evidence does not discharge universal claims (`assurance.md` §1–2).
3. Receipt commits followed by `state-commit` can retain eligibility under unchanged declared dependencies. Changed source, specifications, keys, pins, chain identity or relied-on artifacts invalidate the affected evidence (SAS-002, SWF-005, workflow E7).
4. Preview SDK and evaluator-owned holdout changes are explicit tasks. Retirement applies to selectors, sweeps and dependencies. All retained B work is optional for this qualification, including existing B6/B7; Sherlock fee and security evidence remains mandatory in A/U/F rows (SWF-011/012, SMN-005, tasks 1.0/1.1, E12–E14).
5. Applicable optimization, formal-claim and S>1 checks become mandatory when their features are adopted. An unexecuted applicable gate blocks completion (`proposal.md` exit, SWF-012).
6. The S=1 baseline and frozen admitted inventory precede fresh P3 authorship. P3 receives local parity and proof verification before its public call. S>1 experiments follow the complete S=1 phase profile (SRC-013/014, tasks 4.4/4.7/4.8/6.4).

The three central recommendations remain: keep native Midnight recursion and its explicit statement/accumulator obligations; qualify the exact candidate with stage-specific evidence; and implement the smallest measured S=1 path through ledger-9 compatibility and Preview before optional throughput work (design D4–D14, SAS-001–014, workflow M1–M6).

Open implementation limitations remain explicit: the universal semantic obligations are not proved by finite tests, complete-path resource cost is not yet established, and local source provenance does not reproduce the unavailable deployed node source mapping (`assurance.md` §1, SRC-011, SMN-004). These are recorded obligations or claim limits, not dissent from the design. This vote performs no implementation, proving, deployment or Mainnet authorization. I remain active until the chair's DONE broadcast or the agreed cutoff.
