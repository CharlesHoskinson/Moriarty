This is a static adversarial reading of the specified-only draft in `deliverables/mil4-k-surge-2026-09-29/sol-transition.k`, checked against the packet’s MIL/4 successor working design (2026-09-29, §§1–3 and the numeric/effect conditions). It is not a proof, not an adoption, not a U0 or M3/M4 gate result, and not a kompile run.

Per-step token movement on the four balance slots sums to zero in every `m4tValid` plan. `A + G = GC` and `D + N = NG` are preserved by those same four equations. An RID already present in the supplied list makes the plan invalid. A release that actually reaches `complete` does so with duty 0 and escrow 0. Those local facts do not carry the episode.

The six-obligation map in the working design (`stage_p`, `intent_p`, `effect_p`, `authority_p`, `history_p`, `failure_p`) is not met. One lumped plan failure, `STAGE_OR_EVIDENCE`, stands in for history, evidence, authority, replay, work, and bounds. `m4tPrepared` is also the result of a refund that retains duty.

## Compile defects

Static reading only. No `kompile` was run.

**C1. `m4tPhase` and `m4tDone` are declared `[function, total]` and have no equation for a negative argument.**

```k
syntax String ::= m4tPhase(Int) [function, total]
rule m4tPhase(0) => "complete"
rule m4tPhase(N) => "pending" requires N >Int 0
syntax Bool ::= m4tDone(Int) [function, total]
rule m4tDone(0) => true
rule m4tDone(N) => false requires N >Int 0
```

`m4tPhase(-1)` and `m4tDone(-1)` match nothing. The release equation calls both on `D -Int Q` only after `Q <=Int D` and `D >=Int 0`, so that call stays non-negative. The attribute still claims totality for every `Int`.

Corrected helpers:

```k
rule m4tPhase(0) => "complete"
rule m4tPhase(N) => "pending" requires N >Int 0
rule m4tPhase(N) => "invalid" requires N <Int 0 [owise]
rule m4tDone(0) => true
rule m4tDone(N) => false requires N >Int 0
rule m4tDone(N) => false requires N <Int 0 [owise]
```

Release must then require `m4tPhase(D -Int Q) =/=String "invalid"`. A negative result belongs on the named `BOUNDS` rejection in S1, and it must not be stored as a phase.

The `K-EQUAL` import and the use of `m4tPhase`, `m4tDone`, and `m4tReceipt` before their syntax lines are consistent with current K module parsing. They are not charged here as compile failures.

## Semantic defects

**S1. Every failed plan is `STAGE_OR_EVIDENCE`, and the later conservation judgment is unreachable.**

`m4tPlan(_,_,_,_) => m4tInvalid("STAGE_OR_EVIDENCE") [owise]` collapses phase, evidence, replay, head, work, caps, and balances. `m4tInspect` then tests `EF ==K EXPECTED` before `m4tConserves(EF)`. Every `m4tValid` effect vector already sums to zero, so a prepared plan never yields `"CONSERVATION"`. `"POST_INVARIANT"` is the same kind of dead code: it runs only after `POST ==K EXPECTEDPOST`.

The working design says a profile must give rejection precedence, and that changing only the predecessor commitment selects `history_p`, while changing only an unpaid duty selects `failure_p`.

Counterexample. Predecessor head `"h0"`, fresh fund event, effects and post-state exactly as the fund equation computes, successor head set back to `"h0"`. `NH =/=String H` fails. The result is `m4tRejected("STAGE_OR_EVIDENCE")`. The same code is produced by a duplicate RID, by `W = 0`, and by `m4tPending()` as the observation. A hostile trace cannot name the judgment it hit.

Corrected plan and inspect shape, first failing guard wins:

```k
syntax M4TPlan ::= m4tInvalid(String) | m4tValid(M4TEffects,M4TState)
// precedence inside m4tPlan:
// SHAPE, REPLAY, HISTORY, PHASE, EVIDENCE, AUTHORITY, WORK, BOUNDS, CONSERVATION
rule m4tInspect(..., m4tInvalid(CODE), ...) => m4tRejected(CODE)
rule m4tInspect(..., m4tValid(EXPECTED, EXPECTEDPOST)) =>
  m4tRequire(m4tConservesAll(PRE, EF, POST), "CONSERVATION",
    m4tRequire(EF ==K EXPECTED, "INCOMPLETE_EFFECTS",
      m4tRequire(POST ==K EXPECTEDPOST, "WRONG_POST_STATE",
        m4tPrepared(EF, POST))))
```

`m4tConservesAll` sums the four balance deltas and checks the book identities in S8, including the refunded accumulator. Native and ledger echoes stay outside `m4tPrepared` (U3).

**S2. The successor head is any different string. History can cycle, and one ledger reservation accepts every fork.**

Fund, mark, release, and refund all require only `NH =/=String H`. The head is not a function of `(H, S, event, effects)`. `m4tLedgerReserved(m4tReceipt(EV), H)` pins the predecessor head. The comment on `M4TLedger` calls that pair the current authenticated head checked at acceptance. The post-state head `NH` is absent from that pair.

Counterexample, reachable.

- Policy caps as in S6’s genesis, `W >= 2`.
- Fund `10` from head `"h0"`, stage `0`, with `NH = "h1"`, RID `"r1"`. Plan is valid. Ledger term `m4tLedgerReserved("r1","h0")` also accepts `NH = "hZ"` for the same event and the same reservation.
- From the `"h1"` post-state, `m4tMarkUnknown("r2")` with `NH = "h0"` is valid. The head chain is `"h0" → "h1" → "h0"`.

Stage is `S +Int 1` and is not tied to the length of `IDS`. A shape-valid state may carry stage `0` and a non-empty consumed list; the next plan still advances stage by one.

Corrected extension, as a constructor rather than an unchecked string:

```k
syntax M4THead ::= m4tGenesis(String)
                 | m4tExtend(M4THead, Int, M4TEvent, M4TEffects)
rule m4tPlan(POL, m4tState(H,S,PH,B,BOOKS,IDS,T), NH, EV) =>
  m4tValid(EF, m4tState(m4tExtend(H,S,EV,EF), S +Int 1, ...))
  requires NH ==K m4tExtend(H, S, EV, EF)
   andBool notBool m4tHeadSeen(NH, H) // H is the authenticated chain, not a bare string
```

Ledger reservation binds the extension, the episode digest, and the receipt:

```k
m4tLedgerReserved(m4tReceipt(EV), m4tExtend(H, S, EV, EF), m4tPolicyDigest(POL))
```

A second successor from the same `(H, EV)` fails `HISTORY` because the extension constructor has one value.

**S3. Episode, owner, payee, asset, late-race priority, and intent digest are policy inputs that no rule reads.**

`m4tBooksOK` binds only `GC`, `FC`, `NG`, and `WC`. `m4tShape` does not read `EP`, `OW`, `PY`, `AS`, `PR`, or `ID`. The fund equation binds those names and never uses them. The native statement copies `EP` and `ID` into an echo. The balance tuple has an unnamed payee slot.

The working design’s M4-C5 discriminator is: with a valid envelope, substitute one signed recipient byte, and name the rejecting judgment. `authority_p` requires a current grant and a replay-consumed right. `intent_p` requires exact signed bytes and recipients.

Counterexample, same pre-state, both plans valid. Head `"h"`, stage `1`, phase `"pending"`, `E = 10`, `D = 10`, `F = 0`, `N = 0`, `G = 10`, `A = 90`, `W = 3`, tombstone `false`, caps `GC = 100`, `FC = 5`, `NG = 10`, `WC = 4`.

- Policy R: payee `"bob"`, owner `"alice"`, asset `"USD"`, episode `"e"`, `PR = "release"`, `ID = "digest-bob"`.
- Policy C: payee `"carol"`, owner `"mallory"`, asset `"EUR"`, episode `"e2"`, `PR = "refund"`, `ID = "digest-carol"`.

Event `m4tRelease(10, 0, "r9", m4tSuccess())`. Both policies produce the same effects and the same post balances. Both inspect to `m4tPrepared` if the native echo and `m4tLedgerReserved("r9","h")` are supplied. Carol’s digest is accepted for the same escrow slot. From that same pre-state, `m4tRefund("r8", m4tBoth())` is valid under a policy whose `PR` is `"refund"`, and `m4tRelease(10, 0, "r9", m4tBoth())` is valid under a policy whose `PR` is `"release"`. The race winner is chosen by the submitted policy.

Corrected state carries the bound policy, and every plan requires it:

```k
syntax M4TState ::= m4tState(M4THead, Int, String, M4TBals, M4TBooks,
                             M4TIds, Bool, M4TPolicy)
rule m4tPlan(POL, m4tState(_,_,_,_,_,_,_,POL), NH, EV) => ...
  requires m4tPayee(POL) ==String m4tPayee(EV)
   andBool m4tAsset(POL) ==String m4tAsset(EV)
   andBool m4tDigest(POL) ==String m4tDigest(EV)
```

A substituted recipient fails `AUTHORITY` before any effect is built. Priority used for `m4tBoth()` is the priority inside that stored policy.

**S4. Pending, unknown, and partial are not evidence states. Terminal nonreceipt ignores receipts already made.**

`m4tReleaseWins(_, m4tSuccess()) => true` ignores priority. `m4tRefundWins(_, m4tTerminalNonreceipt()) => true` ignores priority and ignores `N`. Both settlement rules allow phase `"pending"` or `"unknown"` with the same guards. `m4tPhase(N)` for `N >Int 0` is the string `"pending"`, so a partial release from `"unknown"` stores `"pending"`. `m4tMarkUnknown` takes no observation. `m4tPending()` and `m4tUnknown()` never win. `m4tBoth()` wins for exactly one of the strings `"release"` and `"refund"`.

The refund comment says the rule closes the escrow only on certified terminal nonreceipt of the remaining claim. The working design’s bridge row says a partial receipt is a state distinct from no receipt and from completed delivery, and that destination terminal nonreceipt refers to the remaining entitlement. The inherited MIL/3 clause in that same row says `destTimeout` exists only when no receipt exists.

Counterexamples, reachable.

1. Priority bypass. `PR = "refund"`, phase `"pending"`, `E = D = 10`, fee cap allows 0. `m4tRelease(10, 0, "r", m4tSuccess())` is valid. `m4tRelease(10, 0, "r", m4tBoth())` is invalid under that priority. The submitter selects the one-sided constructor.
2. Unknown is inert. From `"pending"`, `m4tMarkUnknown("u")` stores `"unknown"` and zero balance deltas. `m4tRelease(Q, 0, "r", m4tSuccess())` from that state is valid whenever the numeric guards hold.
3. Unknown is erased. `D = 10`, `E = 10`, phase `"unknown"`, release `Q = 4`, `FEE = 0`. Post phase is `"pending"`, `D = 6`. The successor state has no observation cell.
4. Nonreceipt after a receipt. After that partial release, `N = 4` and `E = 6`. `m4tRefund("z", m4tTerminalNonreceipt())` is valid. Payee already holds 4. The observation carries no remainder and is not tied to `D` or `N`.

Corrected evidence, stored in the state and scoped to the remainder:

```k
syntax M4TObservation ::= m4tSuccess(Int)          // certified delivered net
                        | m4tRemainderNonreceipt(Int)
                        | m4tConflict()
rule m4tReleaseWins(ST, m4tSuccess(Q)) => true
  requires Q ==Int m4tReleaseQty(EV) andBool m4tVerified(ST, EV)
rule m4tRefundWins(ST, m4tRemainderNonreceipt(R)) => true
  requires R ==Int m4tDuty(ST) andBool m4tDuty(ST) >Int 0
   andBool m4tVerified(ST, EV)
rule m4tRefundWins(ST, m4tTerminalNonreceipt()) => false
  requires m4tNet(ST) >Int 0   // a receipt exists; this constructor does not apply
```

`m4tConflict()` consults only `m4tPriority(ST)`. A release from phase `"unknown"` stays in `"unknown"` until a verified `m4tSuccess(Q)` is stored. Partial success stores phase `"partial"` with that `Q` retained in the head extension. `m4tMarkUnknown` without a verified unknown premise fails `EVIDENCE` and writes nothing (S12).

**S5. Release admits every pair `(Q, FEE)` inside the caps. The fee is not an effect of the policy.**

```k
m4tEffects(0, 0 -Int (Q +Int FEE), Q, FEE, 0, FEE, Q, Q, 0, RID)
requires Q >Int 0 andBool Q <=Int D
 andBool FEE >=Int 0 andBool Q +Int FEE <=Int E
 andBool F +Int FEE <=Int FC
 andBool (Q <Int D orBool Q +Int FEE ==Int E)
```

`Q <Int D` admits a partial release that drains the whole escrow. The working design says the fee-cap unit, the beneficiary, and whether the fee is an extra transfer are an open record, and that a fractional remainder is not a ledger quantity. This equation treats a caller-chosen integer as that fee and always credits the unnamed reserve slot.

Counterexample, reachable from genesis. `FC = 100`, `NG = D = 100`, `GC = A = O = 100`, `W >= 2`, other books 0, phase `"unfunded"`. Fund `100`. Then `m4tRelease(1, 99, "r", m4tSuccess())`: `1 < 100`, `1 + 99 = 100 = E`, `F + 99 <= 100`. Post: payee `+1`, reserve `+99`, duty `99`, escrow `0`, phase `"pending"`. The four deltas sum to zero. Ninety-nine units of escrow become reserve on a one-unit partial payment.

Corrected release event drops the caller fee. Until the W-D4 record exists, this profile’s fee function is zero, which refuses a silent reserve beneficiary:

```k
syntax M4TEvent ::= m4tRelease(Int, String, M4TObservation) // net Q only
syntax Int ::= m4tFee(M4TPolicy, Int) [function, total]
rule m4tFee(_,_) => 0
```

A nonzero `m4tFee` belongs to a later versioned rule that names the beneficiary cell. It is not an input of `m4tRelease`.

**S6. The settling equation forces `Q + FEE = E` whenever `Q = D`. Surplus has no owner line, so a no-fee overfund cannot finish on success.**

Shape requires escrow 0 in `"complete"`. The plan enforces that by inflating the last payment until it equals escrow. `m4tFund` does not require the funded amount to equal `D`.

Counterexample, reachable, fee cap zero. Genesis: `FC = 0`, `NG = D = 30`, `GC = A = O = 100`, `G = F = N = E = P = R = 0`, `W = 5`, `WC = 5`, phase `"unfunded"`, tombstone `false`.

- Fund `40`. Post: `E = 40`, `G = 40`, `A = 60`, `D = 30`, phase `"pending"`.
- `m4tRelease(20, 0, "r2", m4tSuccess())`. `20 < 30`, so the drain conjunct is not required. Post: `E = 20`, `N = 20`, `D = 10`, phase `"pending"`.
- `m4tRelease(10, 0, "r3", m4tSuccess())` requires `10 + 0 = 20`. It fails.
- `m4tRelease(10, 10, "r3", m4tSuccess())` requires `F + 10 <= 0`. It fails.

No further success-shaped release is valid. The only valid exit is `m4tRefund(..., m4tTerminalNonreceipt())`, which returns the leftover 20 after the payee has already received 20 (S4). With success as the only observation, the 20 stays in escrow.

Corrected settling release, with `FEE = m4tFee(POL, Q)` and surplus back to the owner:

```k
// SURPLUS > 0 only on the settling payment. Partial leaves escrow in place.
rule // Q == D
  m4tEffects(SURPLUS, 0 -Int E, Q, FEE, 0, FEE, Q, Q, 0, RID)
  requires FEE ==Int m4tFee(POL, Q)
   andBool Q ==Int D andBool Q >Int 0
   andBool SURPLUS ==Int (E -Int Q -Int FEE)
   andBool SURPLUS >=Int 0
// post: escrow 0, owner + SURPLUS, payee + Q, reserve + FEE,
//       phase "complete", tombstone true, duty 0
rule // Q < D
  m4tEffects(0, 0 -Int (Q +Int FEE), Q, FEE, 0, FEE, Q, Q, 0, RID)
  requires FEE ==Int m4tFee(POL, Q)
   andBool Q >Int 0 andBool Q <Int D
   andBool Q +Int FEE <Int E
// post phase "partial" (or "unknown" if that was the pre-phase and S4's rule keeps it)
```

`Q + FEE = E` with `Q < D` is no longer a valid partial. That case is the shortfall rule in S11.

**S7. The last unit of episode work can be spent before settlement. Escrow then has no exit.**

Every valid equation requires `W >Int 0` and stores `W -Int 1`. `m4tBooksOK` allows `W = 0`. No rule fires from `"pending"` or `"unknown"` when `W = 0`. `"recovering"` and `"complete"` have no successor.

Counterexample, reachable. `WC = 1`, `W = 1`, `NG = D = 10`, `GC = A = O = 10`, other books 0, phase `"unfunded"`. Fund `10` with a fresh RID. Post: phase `"pending"`, `E = 10`, `D = 10`, `W = 0`. Release, refund, and mark each require `W >Int 0`. The result is `m4tRejected("STAGE_OR_EVIDENCE")`. Escrow 10 is stuck. The same lock is reachable with `W = 2` by funding once and then applying `m4tMarkUnknown` once.

Corrected guards:

```k
// fund and markUnknown:
requires W >Int 1
// release and refund:
requires W >Int 0
```

A work cap of 1 fails fund with `"WORK"`. After a legal fund, at least one settlement step remains, and mark cannot consume it.

**S8. Refund returns escrow to the owner and leaves cumulative gross and spend authority unchanged. The custody identity breaks. A zero refund still tombstones the episode.**

```k
m4tEffects(E, 0 -Int E, 0, 0, 0, 0, 0, 0, 0, RID)
m4tBooks(G, F, N, D, A, W -Int 1)   // duty, gross, authority retained
requires ... andBool E >=Int 0 andBool m4tRefundWins(PR, OBS)
```

`E >=Int 0` restates non-negativity. Authority-use on this vector is 0. There is no refunded-gross component. `m4tConserves` underscores every book field:

```k
rule m4tConserves(m4tEffects(O,E,P,R,_,_,_,_,_,_)) =>
  O +Int E +Int P +Int R ==Int 0
```

Counterexample, reachable. `FC = 0`, `NG = D = 30`, fund `40`, release net `10` with fee 0. Post: `E = 30`, `G = 40`, `N = 10`, `F = 0`, `A = 60`, and `E = G - N - F`. Then refund under `m4tTerminalNonreceipt()`. Post: owner `+30`, `E = 0`, `G = 40`, `N = 10`, `F = 0`, `A = 60`, phase `"recovering"`, duty still `30 - 10 = 20` wait: release of 10 from duty 30 leaves `D = 20`. Recount the duty: start `D = 30`, release `Q = 10`, `D = 20`, `N = 10`, `E = 30`. Yes. After refund, `G - N - F - E = 30`. Those 30 units sit in the owner balance. Remaining spend authority is still 60, so the returned 30 stays counted as consumed gross. The effect vector records authority use 0 and duty decrease 0.

A second reachable case: phase `"pending"`, `E = 0`, `D = 10`, `W > 0`, observation `m4tTerminalNonreceipt()`. Refund’s effect vector is all zeros plus a receipt. Post phase is `"recovering"`, tombstone `true`, duty 10. The episode is closed by a zero transfer.

Corrected books add refunded principal `Rfd` and keep a one-way gross budget explicit:

```k
// invariant inside m4tBooksOK / m4tConservesAll:
// G ==Int N +Int F +Int E +Int Rfd
// A +Int G ==Int GC
// refund of escrow E > 0:
m4tEffects(E, 0 -Int E, 0, 0, 0, 0, 0, 0, 0, RID) // authority use stays 0
// post books: Rfd +Int E, A unchanged, D unchanged, phase "recovering"
requires E >Int 0 andBool m4tRefundWins(ST, OBS)
```

The authority-use line staying 0 is then the stated meaning of a one-way gross cap. `Rfd` is what makes the returned units visible to conservation. A zero-escrow close is S11’s shortfall rule, not this refund.

**S9. Unpaid duty is independent of funded gross. `m4tBooksOK` admits a net goal above the gross cap.**

Fund’s effect is `m4tEffects(0 -Int Q, Q, 0, 0, Q, 0, 0, 0, Q, RID)`: duty decrease 0, net 0. `m4tBooksOK` checks `D +Int N ==Int NG` and `A +Int G ==Int GC` and never `NG <=Int GC` or `D <=Int G`.

Counterexample, shape-valid, then one fund. `GC = 10`, `A = 10`, `G = 0`, `NG = 100`, `D = 100`, `N = 0`, `FC = 0`, `O = 1000`, `E = 0`, `W >= 2`, phase `"unfunded"`. `m4tShape` is true. Fund `10` is valid. Post: `E = 10`, `G = 10`, `A = 0`, `D = 100`. Every later release has `Q <=Int D` and `Q + FEE <=Int E`, so at most 10 of duty 100 can move, and S6’s settling equation cannot succeed.

Corrected `m4tBooksOK` and fund:

```k
andBool NG <=Int GC
andBool D <=Int G +Int (GC -Int G)   // duty cannot exceed the gross cap room plus gross already taken
// fund additionally, while FEE is 0:
requires Q >=Int D andBool Q ==Int D   // one-shot escrow
  orBool (Q <Int D andBool W >Int 1)   // installment only if a later step remains
```

If the episode is an escrow created by funding, fund also sets the duty (`D' = D + Q` or `D' = Q` at genesis) and the effect vector gains a duty-increase line. A pre-existing loan duty belongs to the unimplemented `loan-fixed/1` profile (U1), not to an unbound integer in this escrow.

**S10. Shape admits episode states whose balances are unrelated to the books, including a finished phase that still carries duty.**

`m4tShape` checks phase membership, tombstone agreement, escrow 0 on `"recovering"` and `"complete"`, non-negative balances, and `m4tBooksOK`. It does not check `E`, `P`, or reserve against `G`, `N`, `F`, or `D`. Phase `"unfunded"` may carry a positive escrow. Phase `"complete"` may carry `D > 0`.

Counterexample. `m4tState("h", 0, "unfunded", m4tBals(10, 5, 0, 0), m4tBooks(0, 0, 0, 5, 10, 3), m4tNil(), false)` with policy `GC = 10`, `NG = 5`, `FC = 0`, `WC = 3`. Shape is true. Fund `5` yields escrow `10` and gross `5`. The five units that were already in escrow never passed the authority-use line.

Second counterexample. Phase `"complete"`, tombstone `true`, `E = 0`, `D = 10`, `N = 0`, `NG = 10`, other bounds satisfied. Shape is true. No equation leaves `"complete"`, so the unpaid duty is a finished state.

Corrected shape, for this one-asset escrow:

```k
andBool E ==Int (G -Int N -Int F -Int Rfd)
andBool (PH ==String "unfunded" impliesBool G ==Int 0 andBool E ==Int 0
          andBool N ==Int 0 andBool F ==Int 0 andBool Rfd ==Int 0)
andBool (PH ==String "complete" impliesBool D ==Int 0 andBool E ==Int 0)
andBool (PH ==String "recovering" impliesBool E ==Int 0 andBool Rfd >Int 0)
```

Genesis is an explicit rule, not an arbitrary shape-valid pre-state: unfunded, zero books except `D = NG`, `A = GC`, `W = WC`, empty `IDS`, `m4tGenesis(...)` head, and the policy stored as in S3.

**S11. The effect vector omits the episode deltas that distinguish recovery, and recovery is returned as `m4tPrepared`.**

`M4TEffects` is four balance deltas, gross, fee, net, duty decrease, authority use, and a receipt. Work, phase, tombstone, head, stage, and the consumed-ID cons are only in the post-state. `failure_p` in the working design is a named rejection with no accepted writes, or signed retained phase effects and duties. Refund’s result is `m4tPrepared(EF, POST)`, the same constructor as a completed payment. Duty decrease on refund is 0. The retained duty sits in the post books unsigned.

`m4tMarkUnknown` and a zero-escrow refund (S8) both build an all-zero financial vector plus a receipt. Only the post-state tells them apart.

Counterexample. After S8’s refund, a client that classifies `m4tPrepared` as acceptance treats the retained duty `D = 20` as a successful episode. There is no successor that pays or discharges that duty: `"recovering"` matches none of the four equations.

Corrected result sort and a distinct shortfall equation:

```k
syntax M4TResult ::= m4tWaiting(M4TState)
                   | m4tRejected(String)
                   | m4tPrepared(M4TEffects, M4TState)          // phase complete, D == 0
                   | m4tRetained(M4TEffects, M4TState, Int)     // signed retained duty
// E == 0, D > 0, verified remainder nonreceipt of D:
rule m4tPlan(..., m4tShortfall(RID, m4tRemainderNonreceipt(D))) =>
  m4tValid(m4tEffects(0,0,0,0,0,0,0,0,0,RID),
           m4tState(..., "recovering", ..., D, ..., true, POL))
  requires E ==Int 0 andBool D >Int 0 andBool W >=Int 0
```

`m4tEvaluate` maps that plan to `m4tRetained(EF, POST, D)`, and it maps a zero-duty complete release to `m4tPrepared`. The retained-duty value is an effect field, not only a book inside `POST`. Recovering remains a sink until a later profile adds a signed discharge.

**S12. `m4tMarkUnknown` is a repeatable accepted write with an empty financial vector.**

```k
rule m4tPlan(POL, m4tState(H,S,PH,B,m4tBooks(G,F,N,D,A,W),IDS,false), NH,
             m4tMarkUnknown(RID)) =>
  m4tValid(m4tEffects(0,0,0,0,0,0,0,0,0,RID),
    m4tState(NH, S +Int 1, "unknown", B,
             m4tBooks(G,F,N,D,A, W -Int 1), m4tCons(RID,IDS), false))
  requires NH =/=String H
   andBool (PH ==String "pending" orBool PH ==String "unknown")
   andBool W >Int 0 andBool notBool m4tMember(RID,IDS)
```

From `"unknown"` it fires again. Each step extends the head to an arbitrary new string (S2), consumes an RID, and burns work (S7). No verifier premise is required. `m4tWaiting` is never produced.

Counterexample. Phase `"pending"`, `E = 100`, `D = 100`, `W = 1`. One `m4tMarkUnknown("u")` with `NH = "hX"` prepares a post-state with `W = 0`, phase `"unknown"`, escrow still 100. Settlement is then impossible (S7).

Corrected mark, no accepted financial or phase write when evidence is absent:

```k
rule m4tEvaluate(m4tRun(POL, ST, NH, m4tMarkUnknown(RID), EF, POST, NATIVE, LEDGER)) =>
  m4tWaiting(ST)
  requires m4tEvidenceUnknown(POL, ST, RID)
   andBool EF ==K m4tEffects(0,0,0,0,0,0,0,0,0,RID)
   andBool POST ==K ST
```

A verified unknown premise may store phase `"unknown"` once, under the S2 head constructor, and only when `W >Int 1` or `E ==Int 0`. A second mark with the same observation fails `REPLAY`.

## Unimplemented scope

These are outside what the equations decide. They are not silent passes.

**U1. Eight family profiles and their traces.** The working design’s first profiles (`amm-cp/1`, S0 repay then `loan-fixed/1` and `loan-coll/1`, `cdp/1`, `opt-capped/1`, `obs/1`, `gov/1`, `bridge-pair/1`, `vault/1`) and the numeric cases (23 B versus the 21 B upfront-fee quote, repay 30, mint 100 against 150, option floor `185,185,183`, bridge 100/99, vault `500·1999 = 999·1000 + 500`) are not these rules. This file is one conditional escrow relation.

**U2. Open arithmetic decisions.** M4-C1 canonical `Price<Base,Quote,Scale>` and `/3` versus `/4` bytes, M4-C2 rounding, dust, and reserve-cell representation, and M4-C3 closed Ω, widths, and overflow are not in the file. `Int` is unbounded. There is no quotient and no remainder distinction between a charged liability, a sub-unit residual, and a whole-unit reserve.

**U3. Authority evidence, native proof, and ledger acceptance.** The file comment states that native and ledger constructors are opaque inputs and are not proof rules. `m4tNativeQualified(statement)` passes when the statement echoes `("MIL/4-M4T/1", EP, ID, H, S, EV, EF, POST)`. `m4tLedgerReserved` passes by the same kind of equality. There is no scheme, signed-byte, digest, verifier-locus, or public-input premise (M4-C5). `m4tPrepared` therefore does not mean native acceptance or Preview-ledger acceptance. Those gates stay open, as the working design says they must.

**U4. Footprints and cell registers.** M4-C4’s aggregate, reserve, custody, supply, claim, receipt, and policy cells, and a derived footprint after holes resolve, are not in `M4TEffects`. The footprint is the fixed four-tuple.

**U5. Episode machinery this relation still lacks even after the S-corrections.** A genesis equation (S10), an observation payload that names a remainder (S4), a waiting result that is actually produced (S12), a shortfall/retained-duty result (S11), and any continuation out of `"recovering"`. Installment count is only “however many releases `W` allows.”

**U6. Gate inheritance.** The working design says W-D0–W-D3 and the S0 part of W-D4 are open, and that MIL/4 does not inherit a passing M3-D, M3-R, or M3-N result by changing the version number. Nothing in this K file closes those gates or the September 23 U0 exit’s absent embeddings and target-pin gap.

## Disposition

The draft is a specified-only escrow sketch with per-step four-slot token conservation and two preserved linear identities (`A + G`, `D + N`). Stage and episode transitions are not a function of an authenticated predecessor. Complete effects omit work, phase, head, and retained duty, and they accept a caller-chosen fee. Cumulative authority is a one-way gross integer that refund does not explain. Pending, partial, and unknown do not constrain settlement. Recovery is an ordinary `m4tPrepared` write, and a one-step work budget can lock escrow. Native and ledger premises are echoes. This reading adopts nothing and proves nothing.