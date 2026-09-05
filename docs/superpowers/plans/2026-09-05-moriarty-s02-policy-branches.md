# S02 Branch Policies and Plan Bindings

**Goal:** Implement the adopted common design's pure authorization constraints,
including inspectable complete plan bindings. This is an unsigned policy check,
not signing, verification evidence, a commit transition, or candidate execution.

**Design authority:** `docs/superpowers/specs/2026-09-05-moriarty-s02-common-design-decision.md`.
The three independent design proposals are complete. This unit implements their
adopted disposition; it does not reopen the design vote.

**Files:** `specs/quint/s02/policies.qnt`, `policies_harness.qnt`,
`policies_test.qnt`; evidence under `evidence/s02-model-comparison/policies/`.
Pure policy helpers remain separately importable inside the combined S02 package.

## Binding requirements

- Conjunction inside each clause, disjunction between clauses; one canonical
  policy body shared by both signing profiles.
- Clauses use neutral input, time, ledger balance, and exact parent facts.
  Semantic outcome and public display never grant permission.
- Required and allowed effects retain multiplicity; exact-order clauses retain
  ordering. Every debit owner needs its own policy. Recipients need no signature
  merely for receiving. Effects-free cancellation still needs the exact parent
  operation key and capability.
- Keys range over the two domains, three principals, and nonces 0/1. Successful
  execution time equals physical time in the finite set {1,2,100,101}. Policy
  validity, implementation version, and enforcement mechanism are checked.
- Parent facts are constraints, not evidence of parent authority. Later lifecycle
  code must derive them from validated full parent entries and bind evidence to
  the complete state/ledger/environment/registry context.
- An after-resolution binding includes opaque complete plan identity and a
  neutral list of full operations: predecessor, successor, artifact/call, input,
  operation kind, time, ordered effects, optional Core projection, and context
  facts. All operations must satisfy the policy; actual operation matching uses
  exact equality. Candidate adapters must later establish view fidelity.
- Before-resolution bindings pin an artifact or permit any artifact under the
  policy mechanism. Actual resolution still must satisfy the same canonical
  body. No duplicated bounded-intent policy terms.
- No cryptographic claim, candidate interpreter, signed authority, atomic commit,
  recovery trace, correspondence, or S02 release gate is established by this unit.
- Preserve frozen Core, all previous immutable receipts, and all other worktrees.
  No Foreman repairs.

## Execution

- [ ] Introduce typed carriers and fail-closed public predicates; write behavior
  tests for settlement/refund branches and record meaningful RED assertions.
- [ ] Implement pure conditions, effect bounds, capabilities, operation keys,
  complete profile-specific bindings, and all-debit-owner policy checking.
- [ ] Cover positive settlement/refund/fill/cancel/recovery constraints and
  adversarial missing legs, multiplicity, wrong keys/capabilities, stale time,
  altered parent facts, plan/artifact/state/effect substitutions, and label/display
  independence. Recovery examples are policy checks only, never recovery traces.
- [ ] Typecheck, run focused Quint tests and existing unit regressions. Preserve
  command output and hashes; do not describe tests as exhaustive model checking.
- [ ] Obtain independent implementation review, address findings, and commit.
- [ ] Continue with persistent signatures, per-operation evidence, and atomic
  authorization/consumption/recovery transitions before candidate A–D execution.
