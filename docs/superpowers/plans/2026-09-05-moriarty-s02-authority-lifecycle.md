# S02 Persistent Authority and Atomic Lifecycle

**Goal:** Finish the common observation/authorization package after its reviewed
policy unit. Implement actual signing, verification, races, residual authority,
and recovery transitions before starting candidate A–D semantics.

**Authority:** The adopted three-expert decision in
`docs/superpowers/specs/2026-09-05-moriarty-s02-common-design-decision.md` remains
the source of design choices. No new human approval or repeated proposal vote.
No Foreman repairs. Frozen Core and prior evidence remain immutable.

**Files:** `specs/quint/s02/authorization.qnt` owns signing and context validation;
`execution.qnt` owns per-operation evidence and atomic execution. Each has its
own concrete `_harness.qnt` and `_test.qnt`; additional scenario files keep
fixture responsibilities separate. These remain one common package, with
receipts below `evidence/s02-model-comparison/authorization/` and `execution/`.

## Type and authority boundaries

- Reuse the single canonical `Policy` from `policies.qnt`. A symbolic signature
  records the complete policy, principal, and finite signature token. Symbolic
  signatures are an abstraction, never cryptographic evidence.
- Authority registry cells are unused, registered with an exact signed policy,
  or consumed while retaining that signed policy and consumption revision.
- Parent registry cells retain a complete `Parent[SignedPolicy]` and
  `Entry[SignedPolicy]`. Use unchanged consumption predicates to validate them.
  Do not replace exact signed parent equality with parent-key equality.
- Context includes complete candidate state, ledger, environment, and finite
  twelve-key authority/parent maps. Derived `PolicyFacts` is a projection of this
  validated context, not a solver-supplied authority assertion.
- Signing records are per key. Attempts and verification records are per
  operation, so prepared fill and cancel observations coexist at one revision.
- Pre-sign snapshots bind candidate state, ledger, environment, the signing key's
  cell, and every relevant parent dependency plus its authority cell. Derive
  dependencies from canonical clauses/operations. Unrelated signature registration
  must not invalidate another signer's prepared check.
- After-resolution pre-sign checks inspect every neutral planned operation. The
  adapter's later plan-view fidelity obligation remains explicit. Before-resolution
  checks validate bounded policy content without requiring future execution or
  the signature being created.
- Signature verification evidence is distinct from a persistent signature token.
  Evidence binds exact signed policy, actual full observation, and current context.
  A fresh check of an old token is not a new signature. Unavailable/invalid
  evidence cannot authorize a commit.
- Verification includes both effect evidence and all required signature evidence,
  policy constraints, applicable residual authority, and freshness. Commit
  rechecks the verified observation/context. No direct effects application may
  bypass the boundary.
- An attempt binds its finite ID, proposing actor, operation, full observation,
  and actual prepared context. The actor has no authority merely by proposing;
  required signed policies determine permission. Evidence binds that entire
  attempt. Separate initial/fresh cancellation IDs retain both race attempts.
- Evidence disposition models an external verifier's result. It is not itself
  a cryptographic verifier or candidate interpreter. Later A–D adapters must
  derive valid effect evidence from actual execution, not choose a free validity
  flag for an arbitrary proposed successor. Common fixture evidence is explicitly
  supplied for declared effects. This trust boundary must remain visible in claims.

## Implementation and verification sequence

Progress note (2026-09-05): common evidence bindings and atomic commit code are
implemented and reviewed at source `46fe589` / evidence `0d5926b`. The full
signing-to-settlement harness covers prefunded swap escrow under both profiles.
Keep the broader boxes below open until actual concurrent parent attempts,
classified rejection, and both recovery paths execute. A constructed context or
generic parent-update helper does not close a lifecycle obligation.

- [x] Persistent signing: test-first carriers and pure guards; check -> sign
  stateful paths for both profiles. Reject wrong signer, altered checked policy,
  stale key/state/ledger/environment/parent dependency, and duplicate signing.
  Permit Alice and Bob checks to coexist and sign without mutual invalidation.
- [ ] Full current evidence: test-first attempt/verification records and common
  verification guard. Reject missing debit authority, effects-free unauthorized
  cancellation, missing/invalid evidence, artifact/plan/full-observation mutation,
  and stale time/context. Preserve separately prepared fill/cancel attempts.
- [ ] Atomic fill: apply financial transfer and parent/authority consumption in
  one transition. First use consumes nonce 0 and retains exact parent; second use
  requires exact residual, uses slot 2 once, and couples revisions. Reject generic
  consumed-policy reuse, duplicate/out-of-order slots, and enlarged residuals.
- [ ] Atomic cancellation: no transfer, but verified exact-parent cancellation
  authority is required. Exercise cancel-wins and fill-wins; record loser rejection
  without mutating money or authority. Re-resolve/reverify fresh cancellation after
  a first-fill winner. Cancelled accounting retains historical remaining allowance.
- [ ] Atomic recovery: use separate nonce 1 and the complete check/sign/verify/
  commit pipeline. Initial positive fixtures sign after cancellation is known.
  Refund actual escrow only if equal to the expected remaining fixture budget.
  Exercise recovery of 10 before any fill and 5 after first fill under both
  profiles, preserve cancelled parent accounting, reject a second refund, and
  prevent slot 2 after cancellation. Do not introduce policy supersession.
- [ ] Real terminality and enabledness: completed payment or completed recovery
  with no unresolved attempt. First fill is nonterminal. Any backend terminal
  stutter is explicitly terminal-guarded; no blanket stutter or artificial
  environment action may hide finite-domain deadlock.
- [ ] For each new action, preserve meaningful failing tests, passing deterministic
  paths, and sampled reachability before proceeding. Typecheck concrete aliases;
  run effects/consumption/observation/policy regressions and the Python suite.
- [ ] Independently review the complete common package and obtain the requested
  GPT-6 Astra / Fable 5.1 / Grok 4.6 Council gate reviews with dissent preserved.
  Proposal authorship is not implementation review. Never substitute helper-model
  reviews for the requested vendor/model gate.

## Downstream work remains required

Common lifecycle traces are not candidate execution or Core correspondence.
Next implement A–D independently under both workloads/profiles; independently
extract complete Core observations, exercise all properties/witnesses/controls
and both recovery subscenarios, run Quint/Apalache checks (not TLC), then select
an alternative or establish only a genuinely justified stop. Continue S03–S15,
all 24 XML release gates, the full ACTUS fixture matrix, and S01 Council backfill.
