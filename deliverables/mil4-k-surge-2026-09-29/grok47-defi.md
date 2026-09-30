The integer checks for the packet’s AMM 23 B floor, the S0 repay split, the capped-call floor of 185,185,183, and the vault 999-share identity match those traces. `deliverables/mil4-k-surge-2026-09-29/sol-defi.k` still returns `m4dAccept` for forged heads, empty effect digests, and undisposed M4-C1–C5 choices, and `bridge-pair/1` accepts on an unqualified foreign verifier string. This is a specified-only defect list from the packet. It is not a proof, not an adoption, not a U0 rescore, and not a passed M3-D, M3-R, or M3-N gate. No transaction was opened.

Line numbers below are the packet body of `sol-defi.k`, comment line 1 through `endmodule`.

## Boundary

The working design (§1–§2) keeps six language judgments — `stage_p`, `intent_p`, `effect_p`, `authority_p`, `history_p`, `failure_p` — and leaves source, Core, native, and Preview/Midnight ledger effects to later evidence. An optional kernel may only evaluate a closed predicate over authenticated inputs and derived effect lines.

This file is a pure function. `M4DSeal` and `M4DEvidence` are trusted inputs (line 2). `m4dBound` (lines 49–51) checks pairwise string equality and `W >Int 0`. `m4dDecide` (lines 92–94) copies the seal’s post head and effect digest into `m4dAccept`. There are no K cells, no canonical lines, and no separate history or duty result. `m4dPending()` (line 34) is never produced. Venue commit is absent, which matches §1. The breach is the opposite direction: unqualified foreign evidence becomes the same `m4dAccept` a venue would have to treat as success (bridge receive, lines 194–202; refund, lines 207–214), while §3 says that without a qualified foreign verifier `bridge-pair/1` stays specified-only.

Shared correction for the kernel edge:

```k
syntax M4DResult ::= m4dPending(String, String)

rule m4dEval(R) => m4dReject(m4dFamily(R), "M4_GATE_UNDECIDED")
  requires m4dFamily(R) =/=String "unsupported"
   andBool notBool m4dGatesDecided(m4dFamily(R))

rule m4dDecide(S, F, DUTY, true, _, LINES) =>
  m4dAccept(F, m4dDerivePost(m4dPre(S), LINES, DUTY), m4dDigest(LINES), DUTY)
  requires m4dProgram(S) ==String F
   andBool m4dSigned(S) =/=String ""
   andBool m4dEffects(S) ==String m4dDigest(LINES)
   andBool m4dPost(S) ==String m4dDerivePost(m4dPre(S), LINES, DUTY)
   andBool m4dConserves(LINES)
   andBool notBool m4dMovesResidual(LINES)
```

`m4dGatesDecided` is false in this file for every open M4-C1–C5 choice. Structured `LINES` are language objects. Byte tags `/3` and `/4` stay out of the kernel until M4-C1 is disposed. No `m4dAccept` result is a ledger write.

## Compile defects

**C1. Builtin modules are imported and never required.** Lines 4–5 and 30–32 import `INT-SYNTAX`, `STRING-SYNTAX`, `INT`, `BOOL`, and `STRING`. The file has no `requires "domains.md"`. A standalone `kompile` fails module resolution before any rule runs. Not executed here.

```k
requires "domains.md"
module M4D-SYNTAX
  imports INT-SYNTAX
  imports STRING-SYNTAX
```

**C2. No second compile failure is shown by static reading.** Argument counts match the productions. Repeated pattern variables are nonlinear equalities, which K allows. Under standard domain priorities (`*Int` `/Int` above `+Int` `-Int` above `==Int` `<Int`), the AMM equation (lines 109–111), the CDP comparison (line 139), the option quotient (lines 153–154), and the vault equation (lines 230–232) associate as the comments describe. The five `[owise]` rules are constructor-disjoint. `minInt` and `maxInt` are INT builtins. `m4dEval` on `m4dLater` (line 60) is disjoint at runtime from lines 61–64 because `m4dFamily` returns `"unsupported"` (line 90). That overlap was not sent to a compiler.

## Semantic defects

### Cross-cutting

**S0. Accept disposes open gates.** Working design §2 says M4-C1 through M4-C5 are undisposed, and that a category cannot inherit another row’s review. Every concrete family rule reaches `m4dAccept` on host `*Int` (line 93) with no decision token, no certified Ω premise, no `Price<Base,Quote,Scale>` production, and no signature scheme, digest, locus, or public inputs (M4-C5).

Counterexample: `m4dEval(m4dAMM(...))` with reserves 1000/2200, `DX=11`, `DY=Q=23`, `F=3`, `N=1000`, `R=875159`, and any matching seal reduces through lines 100–112 and 93 to `m4dAccept`. M4-C2 still lists retained rational fee and upfront rounded fee as open. M4-C3 says host computation is not enough.

```k
syntax Bool ::= m4dGatesDecided(String) [function]
rule m4dGatesDecided(_) => false
// After a real disposition, replace the one decided family with true
// and keep every other family on M4_GATE_UNDECIDED.
```

**S1. Selected program is not the family.** `m4dBound` (lines 49–51) requires `SP ==String AP` and never `SP ==String m4dFamily(R)`. Family literals are hardcoded at lines 102, 120, 133, 149, 164, 177, 187, 196, and 222.

Counterexample: seal program `"loan-s0/1"` on both `SP` and `AP`, AMM body of the 23 B trace. Result family is `"amm-cp/1"`.

```k
rule m4dEval(R) => m4dFamilyCheck(R)
  requires m4dBound(m4dSealOf(R))
   andBool m4dProgram(m4dSealOf(R)) ==String m4dFamily(R)
   andBool m4dSigned(m4dSealOf(R)) =/=String ""
```

**S2. `history_p` and `failure_p` cannot fire.** §2 says changing only a predecessor commitment selects `history_p`, and changing only an unpaid duty selects `failure_p`. The only success constructor is `m4dAccept` (line 93). `PH ==String AH` still holds when both are replaced together. Duties are synthesized labels (`"pool-head:" +String PID`, `"reserve:" +String IID`, `"duties:" +String ACTION`), not inputs.

Counterexample: any accepting term; replace `PH` and `AH` with `"forged-predecessor"`. `m4dBound` stays true and the result stays `m4dAccept`.

```k
syntax M4DJudgment ::= m4dStage(...) | m4dIntent(...) | m4dEffect(...)
                     | m4dAuthority(...) | m4dHistory(...) | m4dFailure(String)
rule m4dEval(R) => m4dReject(m4dFamily(R), "HISTORY")
  requires m4dPre(m4dSealOf(R)) =/=String m4dAuthenticatedPre(m4dSealOf(R))
```

The predecessor check has to compare the seal’s pre image with an authenticated predecessor cell, not with a second copy of itself.

**S3. Effect digest and recipients are caller text.** `EF ==String CE` (line 51) admits the empty string. AMM, CDP, option, observe, govern, and vault have no recipient. Loan names debtor and creditor and never reads a balance. Bridge receive has no payee.

Counterexample: the 23 B integers, seal `EF=CE=""`, `W=1`. Lines 107–111 hold (`24127400 = 23 * 1010967 + 875159`). `m4dAccept("amm-cp/1", NH, "", "pool-head:…")` has no trader, no four balance lines, and no conservation.

```k
syntax M4DLine ::= m4dMove(String, String, Int)
                 | m4dSet(String, Int, Int)
                 | m4dMeta(String, Int)
rule m4dConserves(LINES) =>
  m4dSum(LINES, "A") ==Int 0 andBool m4dSum(LINES, "B") ==Int 0
```

**S4. Replay and authority are inputs.** `USED ==Int 0` (line 190) and `CLAIM ==Int 1` (line 152) are supplied by the caller. Two `m4dEval` calls of the same term both accept. There is no consumed nullifier cell.

```k
rule m4dReplay(PRE, CLAIM) => m4dReject(F, "REPLAY")
  requires m4dConsumed(PRE, CLAIM) ==Bool true
```

**S5. One failure code covers distinct judgments.** Quote, custody, floor, and fee bounds share `"QUOTE_OR_CUSTODY"` (line 112). Debt math and funding share `"UNFUNDED_OR_WRONG_DEBT"` (line 127) even though no balance is read. Epoch, threshold, pause, and duties share `"REVOKED_OR_APPROVAL"` (line 180). Precedence that does exist is only seal-before-family (lines 61–64) and evidence-shape `[owise]`.

**S6. Width checks are not closed.** `m4dMax` is `2^128-1` (line 39). `m4dProduct` (line 43) does not cover sums of products. M4-C3’s overflow case still accepts.

AMM counterexample, lines 103–111: `XP = DX = 2^127-1`, `YP = 1`, `N = 2`, `F = 0`, `DY = Q = MIN = 0`, `R = 2^128-2`. Each `m4dProduct` holds, `XP+DX = 2^128-2 <= m4dMax`, and the denominator `XP*N + DX*(N-F) = 2^129-4` is never ranged. The Euclidean test holds with quotient 0. Result is `m4dAccept`.

```k
andBool m4dU(XP *Int N +Int DX *Int (N -Int F))
andBool DY >Int 0
```

**S7. `m4dLater` can wear a real family name.** Line 60 rejects with the argument string. `m4dEval(m4dLater("amm-cp/1"))` is `m4dReject("amm-cp/1", "LATER_PROFILE_UNSUPPORTED")`. Lines 234–235 are unreachable from `m4dEval`.

```k
rule m4dEval(m4dLater(F)) => m4dReject("unsupported", "LATER_PROFILE_UNSUPPORTED")
  requires notBool m4dFirstProfile(F)
```

### AMM `amm-cp/1` (lines 100–115)

The 23 B identity is the retained-fee floor: numerator `2200*11*997 = 24127400`, denominator `1000*1000+11*997 = 1010967`, quotient 23, remainder 875159. `DY=24` fails that equation. Those two integers match §3 and the numeric section. The rule then treats that one formula as the profile.

**S8. Upfront fee is inexpressible, so M4-C2’s discriminator cannot be run.** Numeric section: upfront ceil of 1 A on the same 11 A quote yields 21 B (`floor(2200*10/1010) = 21`). No constructor carries that policy. `F=3,N=1000` accepts 23 only.

**S9. Fee encoding changes the width gate.** `F=30,N=10000` is the same rate as `3/1000` and, for `DX=11`, the same quotient 23. For `DX > floor(m4dMax/9970)` the reduced pair can pass line 106 while the scaled pair fails `m4dU` inside `m4dProduct`. §3 says `30/10000` is an encoding/width comparison, not a changed rate. There is no canonical denominator.

**S10. Reserves and the pool head are different inputs.** Line 100 unifies evidence `HEAD` with the request `HEAD` and never with seal `PH`. `XP` and `YP` are not read from that head. Epoch is only `>= 0` (line 103).

Counterexample: evidence head `"H-1000-2200"`, seal `PH=AH="H-other"`, `XP=1`, `YP=10^9`, a quote computed from those reserves. Lines 100–112 accept it.

```k
andBool HEAD ==String m4dHead(m4dPre(S))
andBool XP ==Int m4dReserve(m4dPre(S), PID, "A")
andBool YP ==Int m4dReserve(m4dPre(S), PID, "B")
andBool m4dFeeCanon(F, N)  // gcd(F,N)==1 and N==K profile denominator
andBool LINES ==K m4dAmmLines(TRADER, PID, DX, DY)  // four balances, fee not a Qty
```

Until W-D4/M4-C2 selects retained versus upfront, `m4dGatesDecided("amm-cp/1")` stays false (S0). Four `m4dMove` lines are required for the retained form only, as the numeric section says.

### Loan S0 repay (lines 119–127)

`P=1000`, `I=10`, `N=DEBIT=CREDIT=30` yields `NI=0`, `NP=980` by lines 125–126. `CREDIT=0` is rejected by line 124. That is the packet’s stated hostile debt/payment split. Originate and accrue are absent, which matches “keep S0 restricted to repay.”

**S11. The failure code claims a funding check the rule does not perform.** No balance cell exists. Debtor `""` with creditor `"c"`, zero balance, and the 30/980 integers is accepted. An impairment narrative in `EF` is accepted whenever `EF==CE` (S3).

**S12. The obligation is not at the head.** `OID`, `P`, and `I` are not loaded from `PH`. A second eval with the same `OID` and the original `P=1000` accepts again (S4).

```k
andBool m4dU(m4dBal(m4dPre(S), DEBTOR)) andBool m4dBal(m4dPre(S), DEBTOR) >=Int N
andBool P ==Int m4dPrincipal(m4dPre(S), OID)
andBool I ==Int m4dAccrued(m4dPre(S), OID)
andBool m4dCreditor(m4dPre(S), OID) ==String CREDITOR
andBool LINES ==K ( m4dMove(DEBTOR, ASSET, 0 -Int N)
                  , m4dMove(CREDITOR, ASSET, N)
                  , m4dSet(OID +String ":p", P, NP)
                  , m4dSet(OID +String ":i", I, NI) )
```

Line 81’s family string `"loan-s0/1"` does not appear in the packet. The packet’s S0 object is the existing repay slice; `loan-fixed/1` and `loan-coll/1` are later. Emit the S0 name the decision record actually assigns, and reject those two slugs only through `m4dLater`.

### CDP `cdp/1` (lines 131–142)

With `PX=1`, `NUM=3`, `DEN=2`, `LOCK=150`, `MINT=DEBT=ISSUE=CREDIT=100`, line 139 is `300 >= 300`. `DEBT=0` with `MINT=100` fails line 135. That is the integer content of the 150-for-100 trace and of “supply without matching debt,” if the caller fills the fields honestly.

**S13. Price is an unbound integer.** §2 M4-C1 and the CDP numeric paragraph require `Price<Synthetic,Collateral,S>`, signed decimals, the 3/2 rule, and an authenticated ceiling. `OBS` is copied as a string (line 131). `PX` is not parsed from it. `ROUND >=Int 0` (line 134) does not require the selected round. There is no scale.

Counterexample: `LOCK=150`, `PX=1000000` meaning 1.0 at scale `10^6`, `NUM=3`, `DEN=2`, `MINT=100000000`. Line 139 is `150*1000000*2 >= 100000000*3`. The rule accepts a mint of `10^8` against 150 collateral. Cap is whatever integer the caller put in `CAP` (line 136); it is not an aggregate cell.

**S14. The triple product skips the width guard.** `LOCK=2^64`, `PX=2^63`, `DEN=4`, `MINT=NUM=1`. Lines 138’s products fit in `m4dMax`. Line 139’s value is `2^129` and the rule accepts.

**S15. No issue grant.** A mint with no issuer, no quota, and no holder address accepts.

```k
andBool BASE ==String SYN andBool QUOTE ==String COL
andBool SCALE >Int 0 andBool m4dPriceOf(OBS) ==K m4dPrice(SYN, COL, PX, SCALE)
andBool ROUND ==Int m4dSelected(m4dPre(S), PAIR)
andBool m4dProduct(LOCK *Int PX, DEN) andBool m4dProduct(MINT *Int NUM, SCALE)
andBool LOCK *Int PX *Int DEN >=Int MINT *Int NUM *Int SCALE
andBool TOTAL ==Int m4dCeilingUsed(m4dPre(S)) andBool CAP ==Int m4dCeiling(m4dPre(S))
andBool m4dGrant(m4dPre(S), ISSUER) >=Int MINT
```

Orientation stays behind `m4dGatesDecided("cdp/1")` until M4-C1 is disposed. The comparison above is the candidate the packet states (price at scale S, ratio `NUM/DEN`, `>=`), not a second silent orientation.

### Option `opt-capped/1` (lines 147–158)

For `SIZE=1500000`, `STRIKE=3000000000`, `CAP=3500000000`, `FIX=3123456789`, `SCALE=1000000`, line 154 yields `1500000*123456789 / 1000000 = 185185183`. Line 153’s ceil of the cap spread is `750000000`. `REM = 750000000-185185183 = 564814817`. `PAY=185185182` fails line 154. Under the guards `CAP >= STRIKE` and `FIX >= 0`, `max(0, min(FIX,CAP)-STRIKE)` matches the comment’s `min(max(FIX-STRIKE,0), CAP-STRIKE)`. The worked floor is aligned.

**S16. Premium and fixing are not exact terms.** `PREMIUM >=Int 0` (line 151) accepts `PREMIUM=0`. `FIX` is not read from `OBS`. `ROUND >=Int 0` is not the selected round. The packet says premium and fixing policy still need exact terms before this case counts.

Counterexample: the integers above, `PREMIUM=0`, `OBS="unspecified"`, `CLAIM=1`. Lines 150–155 accept.

**S17. The one-shot claim and the reclaim duty are comments.** Line 146 says exercise consumes the claim and leaves `REM` pending reclaim. Line 152 requires the caller to pass `CLAIM=1` and does not require a post cell of 0. Success duty is the label `"reserve:" +String IID`. Effects that pay `REM` to the writer on this stage still accept (S3). A second identical eval accepts (S4).

**S18. Reserve width.** `PAY=1`, `REM=m4dMax`, `RESERVE=m4dMax+1` passes line 155 because only `REM` is passed to `m4dU`.

```k
andBool FIX ==Int m4dFixing(OBS) andBool ROUND ==Int m4dSelected(m4dPre(S), IID)
andBool PREMIUM ==Int m4dSignedPremium(m4dSigned(S))
andBool m4dClaim(m4dPre(S), IID) ==Int 1
andBool m4dU(RESERVE) andBool m4dU(PAY) andBool m4dU(REM)
andBool m4dCell(LINES, "claim") ==K m4dSet(IID, 1, 0)
andBool m4dCell(LINES, "reserve") ==K m4dSet(IID, RESERVE, REM)
andBool notBool m4dPays(LINES, WRITER, REM)
```

Explicit versus automatic exercise is a close-column choice with no field. Until it is a constructor (`m4dExerciseExplicit` versus `m4dExerciseAuto`), reject `"EXERCISE_RULE_UNSPECIFIED"`.

### Oracle `obs/1` (lines 162–170)

`ROUND=4`, `SELECTED=9`, fresh timestamps, fails line 165. `ROUND=SELECTED=9` with `OBSERVED <= NOW` and age inside `MAXAGE` passes. That is only a comparison of two request integers.

**S19. Selected round, time, status, and provenance are not in the authenticated tuple.** Evidence carries `PAYLOAD` and `ROUND` (line 162). `SELECTED`, `NOW`, `OBSERVED`, and `MAXAGE` are naked. No scheme, key epoch, or status is read. M4-C5’s verifier locus is absent. Negative clocks pass: `NOW=OBSERVED=-1`, `MAXAGE=0`, `ROUND=SELECTED=0` satisfies lines 165–167.

Counterexample: `m4dLocal("observation","d","ETH-USD",5,"status=disputed")` with request `ROUND=5`, `SELECTED=5`, `NOW=100`, `OBSERVED=100`, `MAXAGE=10`, while the seal head’s selected round is 9 and its clock is 1000. Lines 162–168 accept.

**S20. Admission is branch-dependent.** The same stale round is rejected by `obs/1` and accepted by `cdp/1`. CDP line 134 requires `ROUND >=Int 0` only.

Counterexample: observe term with `ROUND=4`, `SELECTED=9` rejects `"ROUND_OR_FRESHNESS"`. CDP term with that `ROUND`, the same `OBS` string, `LOCK=150`, `MINT=100`, `PX=1`, `NUM=3`, `DEN=2` accepts.

```k
syntax M4DObs ::= m4dObs(String, String, String, Int, Int, Int, String, String)
// domain, pair, canonicalBytes, round, observedTime, keyEpoch, status, scheme
rule m4dAdmit(PRE, m4dObs(DOM, PAIR, BYTES, ROUND, T, EPOCH, STATUS, SCHEME), NOW, MAXAGE)
  => true
  requires ROUND ==Int m4dSelected(PRE, DOM, PAIR)
   andBool STATUS ==String "final"
   andBool SCHEME =/=String "" andBool EPOCH ==Int m4dKeyEpoch(PRE, DOM)
   andBool m4dU(T) andBool m4dU(NOW) andBool T <=Int NOW
   andBool NOW -Int T <=Int MAXAGE andBool NOW ==Int m4dTime(PRE)
```

`m4dCDP` and `m4dOption` must call `m4dAdmit` on the price or fixing observation. A local observation shape that skips `m4dAdmit` rejects `"OBSERVATION_BRANCH"`.

### Governance `gov/1` (lines 175–182)

**S21. Revocation with unchanged bytes accepts.** §3’s hostile case is execution after grant revocation with the same signed action bytes. `LIVE` and `EPOCH` are both caller integers (line 178). The seal head is not read. `ADIGEST` is not a function of `SB`.

Counterexample: `SB=AB="exec:action42:epoch4"`, seal `PH=AH="head-live-epoch-5"`, evidence `m4dLocal("approvals","dom","action42",4,"digest-action42")`, request `EPOCH=4`, `LIVE=4`, `DISTINCT=3`, `THRESH=3`, `PAUSED=0`, `DUTIES=1`. Line 178 holds. Result is `m4dAccept`. A second term with `SB="revoke-grant"` and the same digest also accepts, so unchanged bytes are not what the rule binds.

**S22. Distinct approvals are a count.** `DISTINCT=2`, `THRESH=2` accepts one repeated signer. Nothing is consumed.

**S23. Pause and duties do not match the close column.** `PAUSED ==Int 0` (line 179) rejects every nonzero mask, including a mask that pauses some other class. The packet names a mask and does not assign bits, so `0` is an invented encoding. `DUTIES >=Int 0` accepts `DUTIES=0`, which drops protected duties. The comment at lines 172–174 says execution-time head check and duty retention; the Boolean does not mention the policy head.

```k
andBool EPOCH ==Int m4dLiveEpoch(m4dPre(S))
andBool ADIGEST ==String m4dDigestBytes(m4dSigned(S))
andBool ADIGEST ==String m4dQueued(m4dPre(S), ACTION)
andBool m4dDistinct(m4dSigners(EV)) >=Int THRESH
andBool m4dPauseBit(m4dPre(S), ACTION) ==Int 0
andBool m4dDuties(m4dDerivePost(...)) >=Set m4dDuties(m4dPre(S))
```

If the policy head does not publish a pause bit, reject `"PAUSE_MASK_UNSPECIFIED"` rather than treating 0 as unpaused. Queue and cancel need their own constructors; a `m4dGovern` term is only an execute-shaped predicate, so a cancel cannot be represented and cannot be given its own failure code.

### Bridge `bridge-pair/1` (lines 186–216)

The hostile shape “time elapsed, destination nonreceipt unknown” misses `m4dNoReceipt` and hits line 216, code `"UNKNOWN_NONRECEIPT"`. Timeout alone has no success rule. That part matches the comment at lines 205–206.

**S24. The 100 / 99 / 1 trace accepts without naming the unit, and so does the trace that drops it.** Lines 199–200 require `CREDIT + FEE <= REMAIN`, not equality, and no ratio, decimals, or beneficiary. Lock’s `FEEBOUND` (line 189) is not in the receive term. The success duty `"remaining:" +String CLAIM` does not carry a residual integer, so a full delivery and a partial delivery share a label.

Counterexample that the packet forbids: `LOCK=100` accepted by lines 186–191 with `FEEBOUND=1`. Separate receive `REMAIN=100`, `CREDIT=99`, `FEE=0`, `COUNT=0`, `DELIVERED=0`, `VER="x"` accepted by lines 197–201. The missing unit is neither fee, nor credit, nor refund. The term the packet proposes, `CREDIT=99`, `FEE=1`, also accepts, including when lock’s fee bound was 0, because receive never sees `FEEBOUND`.

**S25. One-shot and partial delivery are not states.** `COUNT ==Int 0` (line 198) is caller input. Two receives of `CREDIT=50`, `REMAIN=100`, `COUNT=0`, `DELIVERED=0` both accept. `DELIVERED=99` together with `m4dNoReceipt` and `REFUND=REMAIN=1` accepts lines 211–213. A refund with `DELIVERED=0`, `REFUND=REMAIN=100`, and no prior lock accepts. `destTimeout` is therefore available after a receipt and without a source claim. §3 says timeout is defined only when no receipt exists, and a 99-unit receipt needs a state distinct from no receipt and from completed delivery.

**S26. Foreign evidence is a non-empty string.** `VER =/=String ""` (lines 197 and 211) with `MSG =/=String ""` is the whole verifier premise. `m4dForeign` and `m4dNoReceipt` are trusted constructors (line 2). §3’s alternative, defer until a qualified verifier exists, and M4-C5’s scheme/digest/locus/public-input contract, both reject this success path. The file returns `m4dAccept` anyway. That is the kernel/venue breach.

```k
rule m4dFamilyCheck(m4dBridgeReceive(...)) =>
  m4dPending("bridge-pair/1", "FOREIGN_VERIFIER_UNQUALIFIED")
  requires notBool m4dQualified(VER)

rule m4dFamilyCheck(m4dBridgeRefund(S,
      m4dNoReceipt(DOM, CLAIM, VER, EPOCH, 1, MSG, REMAIN0), ...)) =>
  m4dDecide(...)
  requires m4dQualified(VER)
   andBool DELIVERED ==Int 0
   andBool REMAIN ==Int LOCK0
   andBool REFUND ==Int REMAIN
   andBool notBool m4dHasReceipt(m4dPre(S), CLAIM)
```

Partial delivery needs a different premise, `m4dRemainderOpen(CLAIM, REMAIN)`, mutually exclusive with a receipt for that remainder, with `CREDIT + FEE + REMAIN' ==Int REMAIN`, `FEE <=Int FEEBOUND`, explicit `ratioNum/ratioDen` (first profile `1/1`), and a named fee-versus-undelivered flag. `CREDIT=99`, `FEE=0`, `REMAIN=100` then fails. Until `m4dQualified` exists, both receive and refund stay `m4dPending`, including the 99-unit proposal.

### Vault `vault/1` (lines 220–233)

`MANAGED=999`, `SUPPLY=1998`, `VA=VS=1`, `DEPOSIT=500`, `SHARES=999`, `REM=500`: line 230–232 is `500*1999 = 999*1000 + 500` and `500 < 1000`. `SHARES=998`, `REM=1500` satisfies the equation and fails `REM < MANAGED+VA`. That hostile remainder is rejected. Post managed 1499 and post supply 2997 are not inputs, so the predicate does not check them.

**S27. Virtual offsets and surplus are attacker integers.** They are not loaded from the pool head, and no recognition grant exists.

Counterexample: `MANAGED=999`, `SUPPLY=1998`, `VA=998501`, `VS=1`, `DEPOSIT=500`, `SHARES=1`, `REM=0`, `CUSTODY=999`, `SURPLUS=0`. Left side `999500`, right side `1*(999+998501)`. Lines 223–232 accept one share. The packet’s offset pair `(1,1)` accepts 999 shares on the same deposit. A second term with honest share math and `SURPLUS=500` also accepts; line 225 is only an identity, with no recognition authority and no beneficiary.

**S28. Product remainder can still be a ledger Qty, and custody can leave uint128.** Line 219 says remainder is metadata. `EF` is not derived, so a move of 500 assets is compatible with `REM=500` (S3). `CUSTODY == MANAGED + SURPLUS` (line 225) is not passed to `m4dU`. `MANAGED=999`, `SURPLUS=m4dMax`, and the 999-share integers accept.

**S29. Rounding beneficiary is unnamed while M4-C2 is open.** The 500 is in product units (`divisor 1000`), and the 500 deposit is a whole-unit increase of managed assets. Shareholder dilution and a protocol-reserve line are different beneficiaries. §2 says a different beneficiary is a versioned amendment. This rule accepts with neither an amendment id nor a stated beneficiary. Zero-share dust (`DEPOSIT=1`, `SUPPLY=0`, `VS=1`, `MANAGED=1000`, `VA=1`, floor 0) is rejected by `SHARES >Int 0` (line 228), so that residue cannot be classified either.

```k
andBool VA ==Int m4dVirtualAssets(m4dPre(S), POOL)
andBool VS ==Int m4dVirtualShares(m4dPre(S), POOL)
andBool m4dU(MANAGED +Int VA) andBool m4dU(CUSTODY)
andBool MANAGED2 ==Int MANAGED +Int DEPOSIT
andBool SUPPLY2 ==Int SUPPLY +Int SHARES
andBool SURPLUS2 ==Int SURPLUS
andBool BENEFICIARY ==String m4dRoundingPolicy(m4dPre(S))
andBool m4dMetaOnly(LINES, "product-rem", REM)
andBool m4dMoveQty(LINES, DEPOSITOR, ASSET) ==Int (0 -Int DEPOSIT)
andBool m4dMoveQty(LINES, POOL, ASSET) ==Int DEPOSIT
```

`m4dGatesDecided("vault/1")` stays false until the beneficiary amendment is in the decision record. Share-issue authority is a field on that same policy; a deposit with no issuer rejects `"SHARE_AUTHORITY"`.

### Packet integers the current Booleans already meet

These are predicate alignments only. Each still accepts under S1–S3.

| Trace | Rule | Result of the integer Boolean |
| --- | --- | --- |
| AMM 11 A → 23 B, reserves 1000/2200, `F=3`, `N=1000`, `R=875159` | lines 109–111 | Holds. `DY=24` fails. |
| AMM upfront 21 B | — | No term. |
| Repay 30 on principal 1000 / accrued 10 → 980 / 0, debit=credit=30 | lines 124–126 | Holds. `CREDIT=0` fails. |
| CDP 150, mint 100, `PX=1`, `NUM=3`, `DEN=2` | line 139 | Holds. `DEBT=0` fails. |
| Call floor 185,185,183 and remainder 564,814,817; pay 185,185,182 | lines 153–155 | Holds; underpay fails. |
| Vault 999 shares, product remainder 500; 998 shares with remainder 1500 | lines 230–232 | Holds; hostile remainder fails. |
| Observe round ≠ selected | line 165 | Fails if both integers are the request’s own fields. |
| Bridge refund without `m4dNoReceipt` | line 216 | Rejects. |
| Bridge 99 with fee 0 and remain 100 | lines 199–200 | Accepts. |

## Unimplemented scope

Correctly outside the accepting rules, or present only as `m4dLater` (line 60):

| Packet object | Where it stands |
| --- | --- |
| Literal-fee transfer, the other half of the U0 S0 slice (§1) | No constructor. |
| AMM exact-out, LP mint/burn, routes, partial fills, maker orders, multisigner clearing | Later boundary. |
| `loan-fixed/1` originate/accrue and `loan-coll/1` liquidation; variable rates, pooled shares, flash liquidity, portfolio liquidation | Later boundary. S0 repay is the only loan rule. |
| CDP redemption, shutdown, rebasing, shared debt, cross-domain supply, reserve-backed issuance | Later boundary. |
| Canonical CDP burn effects | Close column of §3; no burn rule. |
| Option perpetuals, margin, liquidation, social loss | Later boundary. |
| Explicit versus automatic exercise, reclaim phase | Close column; one exercise Boolean only. |
| Oracle median/TWAP and imported foreign verifiers | Later boundary. Non-local observe evidence hits line 170. |
| Voting snapshots and general multisigner stage admission | Later boundary. |
| Queue and cancel | Specified in the §3 governance row; no constructors. |
| Extra bridge ratios, bonded fast fill, reorg loss | Later boundary. First-profile ratio is implicit 1:1 (S24), not a checked `1/1` field. |
| Qualified foreign verifier | Named in §3 as the condition for leaving specified-only. The kernel accepts instead (S26). |
| Vault withdrawal, rewards, slash, AVS, reward index, strategy mandates, principal/yield splits | Later boundary. `m4dLater` rejects them, with the family-name collision in S7. |
| `/3` bytes, `/4` migration, unknown-tag rejection (M4-C1) | No tag sort. Equal strings, including empty, pass line 51. |
| Certified Ω on a pinned target (M4-C3) | Host `INT` hooks only. |
| U0 exit embeddings, native leaves, Preview ledger, gates M3-D / M3-R / M3-N (§1–§2) | No venue rules. Absence is required. `m4dAccept` does not close them. |

`loan-fixed/1`, `loan-coll/1`, and the vault lifecycle slugs are unimplemented because they do not accept, not because a proof excluded them.