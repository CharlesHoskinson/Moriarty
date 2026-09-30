# Governance recommendation: timelock, pause, upgrade and emergency lifecycle

**Scope.** This is a read-only design review of a specified-only proposal. Nothing here claims implementation, proof or ledger acceptance. All edits below are **recommendations**. `D` = `concepts/intent-language/DESIGN-MIL2.md`, `P` = `MIL2-PROPOSED-SEMANTICS.tex`, `G` = `category-review/06-governance.md`, `R6` = `deliverables/u0-study-2026-09-28/defi-coverage/R6-governance.md`. The guarded CLI `status --json` ran and reported that SP01 dispatch is blocked. That block does not affect this review.

## 1. Verdict

**Not yet fit for governance lifecycle claims, but repairable without new machinery.** MIL/2 has the parts:
- right kinds including `amend` (`D:191`);
- a `policy(id)` cell (`D:242`);
- typed `after` predicates (`D:146`);
- ledger-linked episodes (`D:46`);
- a pending→terminal transition with a one-shot tombstone and a signed priority rule (`D:199-216`, `P:192-199`).

It does not connect them. Specific gaps:
- No state records *what is queued*, *what is paused*, or *which policy version a position lives under*.
- `AuthorityFresh` (`P:133`) has no revocation read.
- `now` elaborates at **signing** (`D:78`). An intent-side `after(now+48h)` therefore measures delay from signature, not from queueing.

I agree with `G:28` that `M:160`'s "Expressible" overstates the design. The central insight is simple: **a timelock is an escrow whose funded object is an administrative effect.** The ESC rule (`P:194-198`) already provides pending, a race priority and a tombstone.

## 2. Five ranked design edits

### E1: Policy head as a pinned, authenticated stage input
**Rule sketch:**
```
policy(id) ::= { version: u32, paramDigest, suspendMask: ActionClassSet,
                 queueRoot, maxSuspend: Duration(c), timelockDelay: Duration(c) }
Stage binds  policyHead = (id, version, paramDigest)   -- authenticated ledger read
Origination: position.pinnedPolicy := policyHead
Later stage on position x:  eval under params(x.pinnedPolicy)
                            unless x.consent ∋ migrate(v → v')
```
**Counterexample:** Alice signs a loan when the rate cap is 10%. Governance raises it to 30%. Her next accrual stage reads the current `policy(id)` and is accepted at 30%. Nothing in `D` or `P:108-127` forbids this (`G:25`).

**Placement:** **U0 boundary change.** Add `policyHead` to the stage public-input list (`P:240` binds program/Core identity but not policy identity). R6 argued the same point (`R6:594-609`).
- *Why U0:* unknown tags reject (`D:285`), and the header governs migration. Adding the field later forces a version bump, which would strand positions pinned under the old header.
- Evaluation semantics follow at U2. Parameter-schema content stays in libraries (U6/DA22).

### E2: A ninth right kind, `suspend`, with protected exits
**Rule sketch:**
```
suspend(scope ⊆ ActionClass, until: Instant(c))
  effects: writes policy(id).suspendMask only; no balance/supply/obligation writes
  authoring: scope ∩ ProtectedExits = ∅
    ProtectedExits ⊇ {repay, discharge, own-escrow refund/recover per ESC,
                      withdraw-unencumbered}
  until ≤ stageTime + policy.maxSuspend            -- self-lapsing
  renewal beyond maxSuspend: only via the timelock (E3)
Stage guard (every stage): actionClass ∉ suspendMask(head) ∨ stageTime ≥ until
```
**Counterexamples:**
- A guardian whose authority is only `enforce` or `amend` "pauses" repayment. Accrual continues, and borrowers are liquidated.
- A compromised guardian renews the pause forever.

Today `enforce` is debtor-consented collateral seizure (`D:193`), and `recover` is an owner-scoped remedy (`D:322`). Neither is a guardian power (`G:27`).

**Placement:**
- **U0:** add `suspend` to the right-kind enum (`D:345`). *Why:* the same freeze logic as E1. Folding pause into `amend` would merge the emergency and upgrade authorities that G7 requires to be separate.
- **U3:** the protected-exit guard and fixture, because U3 owns "separate recovery authority" (`ROADMAP.md:24`).

**External practice** (not a Moriarty rule): the Cosmos SDK `x/circuit` module lets a `LEVEL_ALL_MSGS` holder disable *all* messages when given an empty list. "Any address with a permission is able to reset" (https://github.com/cosmos/cosmos-sdk/blob/v0.50.9/x/circuit/README.md). I recommend Moriarty be **stricter**: protected exits cannot be suspended, and suspension lapses by itself.

### E3: Timelock as an ESC instance: queue → (execute | cancel)
**Rule sketch:**
```
queue(pid, payloadDigest)  [right: amend, fresh at head h_q]
  writes queued(pid) = { payloadDigest, grantId, queuedAt := stageTime(h_q),
                         eta := queuedAt + policy.timelockDelay }   -- Reject on overflow (D:124)
  state pending
execute(pid, payload)  guard: stageTime ≥ eta ∧ H(payload) = payloadDigest
                              ∧ ¬revoked(grantId, head) ∧ policyHead = queued.baseHead
cancel(pid)            guard: holder of veto grant ∧ stageTime < eta, or revoked(grantId)
priority cancel                                   -- veto wins a same-head race
terminal tombstone(pid)
```
`queuedAt` is the **ledger stage time** of the queue stage, read from the authenticated clock of the executing domain. It is never signer-supplied and never the elaborated `now`.

**Counterexample:**
1. An intent signed at T0 carries `after(now+48h)`.
2. It is queued at T0+47h.
3. It executes at T0+48h.

The veto window was 1 hour. A second attack substitutes a payload that satisfies the same `after` guard. A validity window is not a timelock (`R6:287`).

**Placement:** **U3.** It reuses ledger-linked continuation plus the late-race machinery; no new core rule beyond ESC over an admin payload. The `queued(pid)` cell joins the cell vocabulary (`D:239-242`) at U0, beside `replay(id)`.

### E4: Revocation as a ledger transition with explicit precedence
**Rule sketch:**
```
allowance(authId) ::= { kind, holder, grantor, scope, window, budget, revokedAt? }
revoke(authId)  [grantor or scoped amend]: writes revokedAt := stageTime
AuthorityFresh(stage, authId) ≜ read(allowance(authId), head).revokedAt = ⊥
Precedence: revocation at head h invalidates every *unexecuted* use
            (queued entries, pending completions) at heads > h;
            it never reverses an accepted stage and never erases residual duties
```
**Counterexamples:**
- An allocator is removed while their queued upgrade matures. Execution succeeds on a stale grant (`G:22`).
- The reverse error: revoking the guardian retroactively "un-pauses" history, or erases a debt created under the revoked grant. That contradicts `MORIARTY-CONSOLIDATED-DESIGN.md:70`, as cited at `R6:291`.

**Placement:** **U0** adds the `revokedAt` read to the authority key (`P:148`); R6 found no revocation field at all (`R6:282`). **U2** enforces it natively.

### E5: Migration as a preservation-or-consent relation, with exit under the old version
**Rule sketch:**
```
migrate(v → v')  requires: executed via E3 (L15: Up → Tg | bounded emergency),
                 and emergency path = suspend only (no emergency amend in profile 1)
For each live x pinned to v:
  Preserve(x, v, v') ≜ beneficiary, outstanding (L1), consumed replay/tombstones,
                       cumulativeFees, recovery path, disclosure  are equal under π
  x moves to v' only if Preserve(x) ∨ consent(x.debtor, migrate(v→v'))
  otherwise x stays pinned to v, and v's exit stages (ProtectedExits) stay acceptable
Replay and tombstone sets are carried forward monotonically; never reset.
```
**Counterexample:** the UNI-015 case (`R6:288`). An interface-compatible verifier changes the payout recipient, or a fresh deployment resurrects consumed authority.

**Placement:**
- **Parameter-only** migration under a fixed program: **U3/U6**, via the DA22 fixture (`R6:290`).
- **Program, verifier or federation-epoch** migration: **U5**, via ZR15 (`ROADMAP.md:26`). This is unchanged.
- Immutable deployment remains a valid initial profile (ZR15, `docs/MORIARTY-BACKEND-REQUIREMENTS.md:29`).

## 3. Core versus library boundary

**Core (Moriarty rules):**
- the policy head and position pinning (E1);
- the right-kind enum including `suspend`;
- the protected-exit rule and self-lapse (E2);
- the `queued` cell with ESC tombstone and cancel priority (E3);
- revocation read and precedence (E4);
- preservation-or-consent, and monotone replay carry-forward (E5).

These bind reachability for every stage, so they cannot be opt-in.

**Library:**
- delay and `maxSuspend` values;
- veto-council composition;
- parameter schemas;
- governor, voting, snapshot and quorum (out of Core per `wiki/defiformal-taxonomy.md:185-192`);
- treasury residual rules.

**Not required:** the federated kernel plays no part. Its epoch policy is a separate U5 service concern (`ROADMAP.md:26`).

Preserved: bounded stages, `|S|=1` (`D:336`), Φ₀ only (all guards above are difference or literal comparisons over one clock), debt separate from supply (`suspend` and `migrate` write no supply), no cross-domain rollback (each lifecycle stays on one domain), and native proof-carrying acceptance with no host-computed Boolean.

## 4. Smallest implementable slice (recommendation)

The slice has one domain, one signer, and one Φ₀ parameter: `maxBorrow` in `policy(p)`. It also has one pre-existing loan L pinned to v1.
- Stage 1: `queue(pid, H(setParam maxBorrow := X))` at stage time t₁, with `timelockDelay = D`.
- Stage 2: `execute` at t₂ ≥ t₁+D, which writes v2.
- Stage 3: repay of L, accepted under v1 params.

**Positive control:** valid, feasible execution at t₂ = t₁+D with a matching digest and an unrevoked grant. L's repay is accepted, and its `pinnedPolicy` still reads v1.

**Hostile pair.** Each hostile differs from the positive in **one field** and must fail at the native relation, not at the envelope (`ROADMAP.md:40`):
- **(h1)** t₂ = t₁+D−1 → `TIMELOCK_NOT_ELAPSED`.
- **(h2)** same time, payload digest ≠ queued → `PAYLOAD_MISMATCH`.

The second-slice extension adds:
- `suspend` over a scope containing `repay` → authoring reject;
- a queue whose grant was revoked at a head between queue and execute → reject.

**Milestones:** the slice belongs to U3, after U2's single-stage path, and depends on the U0 field additions.

## 5. Explicit disagreements

1. **With `G:27`** ("no clear milestone" for emergency control): I place `suspend` semantics in **U3** beside recovery authority, with only the enum tag at U0.
2. **With R6's "a pause is ... a Core change"** (`R6:413`): the pause *mask read and protected exits* are Core. Pause *policy* (who, how long) is library.
3. **With deferring all migration to U5** (`ROADMAP.md:26`): UNI-015 is scoped to federation epochs (`R6:288`). Parameter-only migration is needed earlier for DA22, so I place it at U3/U6.
4. **With the Cosmos `x/circuit` practice** of pause-everything and reset by any permitted holder: I reject it as a Moriarty rule for the reasons in E2.
5. **With using `after(now+Δ)` as a timelock**, including `M:160`: `now` is signing-time (`D:78`), so it is unsound for delay-from-authorization.

## 6. Residual assumptions

- Midnight exposes an authenticated per-domain ledger clock that a stage can bind as `stageTime`. This is an unresolved interface premise, not checked here.
- Veto and queue authority is single-signer until `|S|>1` is admitted post-U4 (`D:351`). A k-of-n council is either a program-custody grant or a named trust premise; `k_of_n` over propositions does not authenticate signers (`G:29`).
- Inclusion liveness: a veto transaction may not be included before `eta`. A timelock gives oversight *opportunity*, not a guarantee (`P:200`, `D:219`). This must be declared as a liveness assumption.
- `ProtectedExits` must be enumerable over a finite `ActionClass` set per program. This assumes the admitted action grammar stays finite, which it does in source/5 (`R6:285`).
- The Φ₁ deferral stands. No edit here needs variable products. U0 changes are limited to three frozen-schema additions: `policyHead`, the `suspend` kind, and the `revokedAt`/`queued` cells.