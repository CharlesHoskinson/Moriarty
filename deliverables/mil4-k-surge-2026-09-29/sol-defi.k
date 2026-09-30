// MIL/4 DeFi first-profile proposal. Specified only; no source/Core/native correspondence.
// All M4DSeal and M4DEvidence constructors are trusted inputs to this projection.
module M4D-SYNTAX
  imports INT-SYNTAX
  imports STRING-SYNTAX

  // A seal records the selected program, exact signed bytes and authenticated heads.
  // Equality here checks binding; authentication is an external, still-open premise.
  syntax M4DSeal ::= m4dSeal(String, String, String, String, String, String, String, String, String, String, Int) [symbol(m4dSeal)]
  // Evidence is typed by family, domain, object, round/epoch, and verified payload.
  syntax M4DEvidence ::= m4dLocal(String, String, String, Int, String) [symbol(m4dLocal)]
                       | m4dForeign(String, String, String, Int, String, String) [symbol(m4dForeign)]
                       | m4dNoReceipt(String, String, String, Int, Int, String) [symbol(m4dNoReceipt)]
  syntax M4DReq ::=
       m4dAMM(M4DSeal, M4DEvidence, String, String, String, Int, Int, Int, Int, Int, Int, Int, Int, Int) [symbol(m4dAMM)]
     | m4dRepay(M4DSeal, String, String, String, Int, Int, Int, Int, Int, Int, Int) [symbol(m4dRepay)]
     | m4dCDP(M4DSeal, M4DEvidence, String, String, String, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int) [symbol(m4dCDP)]
     | m4dOption(M4DSeal, M4DEvidence, String, String, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int) [symbol(m4dOption)]
     | m4dObserve(M4DSeal, M4DEvidence, String, String, String, Int, Int, Int, Int, Int) [symbol(m4dObserve)]
     | m4dGovern(M4DSeal, M4DEvidence, String, String, String, Int, Int, Int, Int, Int, Int) [symbol(m4dGovern)]
     | m4dBridgeLock(M4DSeal, String, String, String, Int, Int, Int) [symbol(m4dBridgeLock)]
     | m4dBridgeReceive(M4DSeal, M4DEvidence, String, String, Int, Int, Int, Int, Int, Int) [symbol(m4dBridgeReceive)]
     | m4dBridgeRefund(M4DSeal, M4DEvidence, String, String, Int, Int, Int, Int) [symbol(m4dBridgeRefund)]
     | m4dVault(M4DSeal, String, String, String, Int, Int, Int, Int, Int, Int, Int, Int, Int) [symbol(m4dVault)]
     | m4dLater(String) [symbol(m4dLater)]
endmodule

module M4D-DEFI
  imports M4D-SYNTAX
  imports INT
  imports BOOL
  imports STRING

  syntax M4DResult ::= m4dReject(String, String) [symbol(m4dReject)]
                     | m4dProjected(String, String, String, String) [symbol(m4dProjected)]

  syntax Int ::= m4dMax() [function]
  rule m4dMax() => 340282366920938463463374607431768211455
  syntax Bool ::= m4dU(Int) [function]
  rule m4dU(X) => X >=Int 0 andBool X <=Int m4dMax()
  syntax Bool ::= m4dProduct(Int, Int) [function]
  rule m4dProduct(X, Y) => m4dU(X) andBool m4dU(Y) andBool X *Int Y <=Int m4dMax()

  // Seal arguments: selected program, signed bytes, actual program, actual bytes,
  // pre head, authenticated pre head, post head, derived post head, effect digest,
  // complete effect digest, work bound. Every accepted branch shares this check.
  syntax Bool ::= m4dBound(M4DSeal) [function]
  rule m4dBound(m4dSeal(SP, SB, AP, AB, PH, AH, NH, DH, EF, CE, W))
    => SP ==String AP andBool SB ==String AB andBool PH ==String AH
       andBool NH ==String DH andBool EF ==String CE andBool W >Int 0
       andBool SP =/=String "" andBool SB =/=String ""
       andBool PH =/=String "" andBool NH =/=String "" andBool EF =/=String ""
  syntax String ::= m4dProgram(M4DSeal) [function]
  rule m4dProgram(m4dSeal(SP, _, _, _, _, _, _, _, _, _, _)) => SP
  syntax String ::= m4dPre(M4DSeal) [function]
  rule m4dPre(m4dSeal(_, _, _, _, PH, _, _, _, _, _, _)) => PH
  syntax String ::= m4dPost(M4DSeal) [function]
  rule m4dPost(m4dSeal(_, _, _, _, _, _, NH, _, _, _, _)) => NH
  syntax String ::= m4dEffects(M4DSeal) [function]
  rule m4dEffects(m4dSeal(_, _, _, _, _, _, _, _, EF, _, _)) => EF

  // Projection result repeats caller-supplied post/effect strings. No admission
  // or authentication is implied by its constructor.
  syntax M4DResult ::= m4dEval(M4DReq) [function]
  rule m4dEval(m4dLater(_)) => m4dReject("unsupported", "LATER_PROFILE_UNSUPPORTED")
  rule m4dEval(R) => m4dReject(m4dFamily(R), "SEAL")
    requires m4dFamily(R) =/=String "unsupported" andBool notBool m4dBound(m4dSealOf(R))
  rule m4dEval(R) => m4dReject(m4dFamily(R), "PROGRAM_FAMILY")
    requires m4dFamily(R) =/=String "unsupported" andBool m4dBound(m4dSealOf(R))
         andBool m4dProgram(m4dSealOf(R)) =/=String m4dFamily(R)
  rule m4dEval(R) => m4dFamilyCheck(R)
    requires m4dFamily(R) =/=String "unsupported" andBool m4dBound(m4dSealOf(R))
         andBool m4dProgram(m4dSealOf(R)) ==String m4dFamily(R)
  // Every first profile lacks authenticated state/effect adapters. Product
  // admission always fails closed, including both foreign bridge branches.
  syntax M4DResult ::= m4dAdmit(M4DReq) [function]
  rule m4dAdmit(m4dLater(_)) => m4dReject("unsupported", "LATER_PROFILE_UNSUPPORTED")
  rule m4dAdmit(R) => m4dReject(m4dFamily(R), "FAMILY_STAGE_ADAPTER_ABSENT")
    requires m4dFamily(R) =/=String "unsupported"
  syntax M4DSeal ::= m4dSealOf(M4DReq) [function]
  syntax String ::= m4dFamily(M4DReq) [function]
  syntax M4DResult ::= m4dFamilyCheck(M4DReq) [function]

  rule m4dSealOf(m4dAMM(S, _, _, _, _, _, _, _, _, _, _, _, _, _)) => S
  rule m4dSealOf(m4dRepay(S, _, _, _, _, _, _, _, _, _, _)) => S
  rule m4dSealOf(m4dCDP(S, _, _, _, _, _, _, _, _, _, _, _, _, _, _)) => S
  rule m4dSealOf(m4dOption(S, _, _, _, _, _, _, _, _, _, _, _, _, _)) => S
  rule m4dSealOf(m4dObserve(S, _, _, _, _, _, _, _, _, _)) => S
  rule m4dSealOf(m4dGovern(S, _, _, _, _, _, _, _, _, _, _)) => S
  rule m4dSealOf(m4dBridgeLock(S, _, _, _, _, _, _)) => S
  rule m4dSealOf(m4dBridgeReceive(S, _, _, _, _, _, _, _, _, _)) => S
  rule m4dSealOf(m4dBridgeRefund(S, _, _, _, _, _, _, _)) => S
  rule m4dSealOf(m4dVault(S, _, _, _, _, _, _, _, _, _, _, _, _)) => S
  rule m4dSealOf(m4dLater(_)) => m4dSeal("", "", "", "", "", "", "", "", "", "", 0)
  rule m4dFamily(m4dAMM(_, _, _, _, _, _, _, _, _, _, _, _, _, _)) => "amm-cp/1"
  rule m4dFamily(m4dRepay(_, _, _, _, _, _, _, _, _, _, _)) => "loan-s0/1"
  rule m4dFamily(m4dCDP(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _)) => "cdp/1"
  rule m4dFamily(m4dOption(_, _, _, _, _, _, _, _, _, _, _, _, _, _)) => "opt-capped/1"
  rule m4dFamily(m4dObserve(_, _, _, _, _, _, _, _, _, _)) => "obs/1"
  rule m4dFamily(m4dGovern(_, _, _, _, _, _, _, _, _, _, _)) => "gov/1"
  rule m4dFamily(m4dBridgeLock(_, _, _, _, _, _, _)) => "bridge-pair/1"
  rule m4dFamily(m4dBridgeReceive(_, _, _, _, _, _, _, _, _, _)) => "bridge-pair/1"
  rule m4dFamily(m4dBridgeRefund(_, _, _, _, _, _, _, _)) => "bridge-pair/1"
  rule m4dFamily(m4dVault(_, _, _, _, _, _, _, _, _, _, _, _, _)) => "vault/1"
  rule m4dFamily(m4dLater(_)) => "unsupported"

  syntax M4DResult ::= m4dDecide(M4DSeal, String, String, Bool, String) [function]
  rule m4dDecide(S, F, DUTY, true, _) => m4dProjected(F, m4dPost(S), m4dEffects(S), DUTY)
  rule m4dDecide(_, F, _, false, CODE) => m4dReject(F, CODE)

  // AMM: one exact-input hop, rational fee retained inside pool custody.
  // XP/YP are proposed reserves awaiting authenticated state binding; DX/DY
  // are proposed trader transfer quantities,
  // F/N fee ratio, MIN signed output floor, Q/R submitted quotient and remainder.
  // Pool's A reserve rises by DX; B reserve falls by DY. Fee is not a separate Qty.
  rule m4dFamilyCheck(m4dAMM(S, m4dLocal("pool", DOM, PID, EPOCH, HEAD), DOM, PID, HEAD,
                  XP, YP, DX, DY, F, N, MIN, Q, R))
    => m4dDecide(S, "amm-cp/1", "pool-head:" +String PID,
         EPOCH >=Int 0 andBool HEAD ==String m4dPre(S)
         andBool XP >Int 0 andBool YP >Int 0 andBool DX >Int 0
         andBool m4dU(XP) andBool m4dU(YP) andBool m4dU(DX)
         andBool F >=Int 0 andBool F <Int N andBool N >Int 0
         andBool m4dU(F) andBool m4dU(N) andBool MIN >=Int 1
         andBool m4dU(MIN) andBool m4dU(Q) andBool m4dU(R)
         andBool m4dProduct(DX, N -Int F) andBool m4dProduct(XP, N)
         andBool m4dProduct(YP, DX *Int (N -Int F))
         andBool m4dU(XP *Int N +Int DX *Int (N -Int F))
         andBool m4dProduct(Q, XP *Int N +Int DX *Int (N -Int F))
         andBool m4dU(Q *Int (XP *Int N +Int DX *Int (N -Int F)) +Int R)
         andBool m4dU(XP +Int DX) andBool m4dU(YP -Int DY)
         andBool Q ==Int DY andBool DY >=Int MIN andBool DY <Int YP
         andBool YP *Int DX *Int (N -Int F)
                    ==Int Q *Int (XP *Int N +Int DX *Int (N -Int F)) +Int R
         andBool R >=Int 0 andBool R <Int XP *Int N +Int DX *Int (N -Int F),
         "QUOTE_OR_CUSTODY")
  rule m4dFamilyCheck(m4dAMM(S, _, _, _, _, _, _, _, _, _, _, _, _, _))
    => m4dDecide(S, "amm-cp/1", "", false, "POOL_EVIDENCE")
    [owise]

  // Existing funded loan only: AccrualFirst nominal discharge, no originate/accrue.
  // Debtor debit and creditor credit must both equal N; residual debt survives.
  rule m4dFamilyCheck(m4dRepay(S, OID, DEBTOR, CREDITOR, P, I, N, DEBIT, CREDIT, NP, NI))
    => m4dDecide(S, "loan-s0/1", OID,
       DEBTOR =/=String CREDITOR andBool OID =/=String ""
       andBool DEBTOR =/=String "" andBool CREDITOR =/=String ""
       andBool m4dU(P) andBool m4dU(I) andBool m4dU(P +Int I)
       andBool N >Int 0 andBool N <=Int P +Int I
       andBool DEBIT ==Int N andBool CREDIT ==Int N
       andBool NI ==Int I -Int minInt(N, I)
       andBool NP ==Int P -Int (N -Int minInt(N, I)),
       "UNFUNDED_OR_WRONG_DEBT")

  // CDP mint: one collateral and one synthetic, exact issue = debt = holder credit.
  // Collateral health compares scaled products at the selected typed price.
  rule m4dFamilyCheck(m4dCDP(S, m4dLocal("price", DOM, PAIR, ROUND, OBS), DOM, PAIR, OBS,
                  LOCK, MINT, DEBT, ISSUE, CREDIT, TOTAL, CAP, PX, NUM, DEN))
    => m4dDecide(S, "cdp/1", "debt:" +String PAIR,
       ROUND >=Int 0 andBool LOCK >Int 0 andBool MINT >Int 0
       andBool m4dU(LOCK) andBool m4dU(MINT) andBool m4dU(TOTAL)
       andBool m4dU(CAP) andBool m4dU(PX) andBool m4dU(NUM) andBool m4dU(DEN)
       andBool DEBT ==Int MINT andBool ISSUE ==Int MINT andBool CREDIT ==Int MINT
       andBool m4dU(TOTAL +Int MINT) andBool TOTAL +Int MINT <=Int CAP
       andBool PX >Int 0 andBool NUM >=Int DEN andBool DEN >Int 0
       andBool m4dProduct(LOCK, PX) andBool m4dProduct(MINT, NUM)
       andBool m4dProduct(LOCK *Int PX, DEN)
       andBool LOCK *Int PX *Int DEN >=Int MINT *Int NUM,
       "MINT_DEBT_LOCK_OR_HEALTH")
  rule m4dFamilyCheck(m4dCDP(S, _, _, _, _, _, _, _, _, _, _, _, _, _, _))
    => m4dDecide(S, "cdp/1", "", false, "PRICE_EVIDENCE") [owise]

  // Finite funded call: payoff floor(size * min(max(fixing-strike,0),cap-strike)/scale).
  // Premium and writer funding are represented as separate exact values; exercise
  // checks a claimed one-shot flag. Claim consumption and reserve retention
  // require the separate authenticated effect relation.
  rule m4dFamilyCheck(m4dOption(S, m4dLocal("fixing", DOM, IID, ROUND, OBS), DOM, IID,
                  SIZE, STRIKE, CAP, FIX, SCALE, PREMIUM, RESERVE, PAY, REM, CLAIM))
    => m4dDecide(S, "opt-capped/1", "reserve:" +String IID,
       ROUND >=Int 0 andBool SIZE >Int 0 andBool SCALE >Int 0
       andBool STRIKE >=Int 0 andBool CAP >=Int STRIKE
       andBool FIX >=Int 0 andBool PREMIUM >=Int 0
       andBool m4dU(SIZE) andBool m4dU(STRIKE) andBool m4dU(CAP)
       andBool m4dU(FIX) andBool m4dU(SCALE) andBool m4dU(PREMIUM)
       andBool m4dU(RESERVE) andBool m4dU(PAY) andBool m4dU(REM)
       andBool CLAIM ==Int 1 andBool m4dProduct(SIZE, CAP -Int STRIKE)
       andBool m4dU(SIZE *Int (CAP -Int STRIKE) +Int SCALE -Int 1)
       andBool m4dProduct(SIZE, maxInt(0, minInt(FIX, CAP) -Int STRIKE))
       andBool RESERVE >=Int (SIZE *Int (CAP -Int STRIKE) +Int SCALE -Int 1) /Int SCALE
       andBool PAY ==Int (SIZE *Int maxInt(0, minInt(FIX, CAP) -Int STRIKE)) /Int SCALE
       andBool REM ==Int RESERVE -Int PAY andBool m4dU(REM),
       "PAYOFF_FUNDING_OR_CLAIM")
  rule m4dFamilyCheck(m4dOption(S, _, _, _, _, _, _, _, _, _, _, _, _, _))
    => m4dDecide(S, "opt-capped/1", "", false, "FIXING_EVIDENCE") [owise]

  // Observation predicate: selected round, named domain/pair and time values are
  // caller supplied here. Admission requires authenticated selection and clock.
  rule m4dFamilyCheck(m4dObserve(S, m4dLocal("observation", DOM, PAIR, ROUND, PAYLOAD),
                  DOM, PAIR, PAYLOAD, ROUND, SELECTED, NOW, OBSERVED, MAXAGE))
    => m4dDecide(S, "obs/1", "observation:" +String PAIR,
       ROUND ==Int SELECTED andBool m4dU(ROUND)
       andBool m4dU(NOW) andBool m4dU(OBSERVED) andBool m4dU(MAXAGE)
       andBool OBSERVED <=Int NOW andBool NOW -Int OBSERVED <=Int MAXAGE,
       "ROUND_OR_FRESHNESS")
  rule m4dFamilyCheck(m4dObserve(S, _, _, _, _, _, _, _, _, _))
    => m4dDecide(S, "obs/1", "", false, "OBSERVATION_EVIDENCE") [owise]

  // Governance queue execution: exact action digest, current policy head/grant
  // epoch, claimed distinct approval count, and no pause. An external verifier
  // must validate the approval set and preserve actual existing duties.
  rule m4dFamilyCheck(m4dGovern(S, m4dLocal("approvals", DOM, ACTION, EPOCH, ADIGEST),
                  DOM, ACTION, ADIGEST, EPOCH, LIVE, DISTINCT, THRESH, PAUSED, DUTIES))
    => m4dDecide(S, "gov/1", "duties:" +String ACTION,
       EPOCH ==Int LIVE andBool DISTINCT >=Int THRESH
       andBool THRESH >Int 0 andBool PAUSED ==Int 0
       andBool m4dU(EPOCH) andBool m4dU(DISTINCT)
       andBool m4dU(THRESH) andBool m4dU(DUTIES),
       "REVOKED_OR_APPROVAL")
  rule m4dFamilyCheck(m4dGovern(S, _, _, _, _, _, _, _, _, _, _))
    => m4dDecide(S, "gov/1", "", false, "APPROVAL_EVIDENCE") [owise]

  // Bridge source lock: claim must be unique, one-to-one representation, a
  // separately accounted fee ceiling, and a still outstanding source duty.
  rule m4dFamilyCheck(m4dBridgeLock(S, CLAIM, SRC, DST, LOCK, FEEBOUND, USED))
    => m4dDecide(S, "bridge-pair/1", "bridge:" +String CLAIM,
       CLAIM =/=String "" andBool SRC =/=String DST andBool LOCK >Int 0
       andBool m4dU(LOCK) andBool FEEBOUND >=Int 0 andBool FEEBOUND <=Int LOCK
       andBool USED ==Int 0,
       "LOCK_OR_REPLAY")
  // A typed destination receipt proposes one delivery. Qualified verification
  // and source-claim conservation remain external premises.
  rule m4dFamilyCheck(m4dBridgeReceive(S, m4dForeign("receipt", DOM, CLAIM, EPOCH, MSG, VER),
                  DOM, CLAIM, EPOCH, DELIVERED, REMAIN, CREDIT, FEE, COUNT))
    => m4dDecide(S, "bridge-pair/1", "bridge:" +String CLAIM,
       VER =/=String "" andBool MSG =/=String "" andBool EPOCH >=Int 0
       andBool COUNT ==Int 0 andBool DELIVERED ==Int 0 andBool REMAIN >Int 0
       andBool CREDIT >Int 0 andBool FEE >=Int 0
       andBool CREDIT +Int FEE ==Int REMAIN
       andBool m4dU(REMAIN) andBool m4dU(CREDIT) andBool m4dU(FEE)
       andBool m4dU(CREDIT +Int FEE)
       andBool m4dU(DELIVERED +Int CREDIT),
       "RECEIPT_OR_ENTITLEMENT")
  rule m4dFamilyCheck(m4dBridgeReceive(S, _, _, _, _, _, _, _, _, _))
    => m4dDecide(S, "bridge-pair/1", "", false, "FOREIGN_RECEIPT_EVIDENCE") [owise]
  // Timeout alone never authorizes refund. A trusted terminal nonreceipt proof
  // must cover the exact remaining entitlement after all delivered installments.
  rule m4dFamilyCheck(m4dBridgeRefund(S,
                  m4dNoReceipt(DOM, CLAIM, VER, EPOCH, TERMINAL, MSG), DOM, CLAIM,
                  EPOCH, DELIVERED, REMAIN, REFUND))
    => m4dDecide(S, "bridge-pair/1", "bridge:" +String CLAIM,
       VER =/=String "" andBool MSG =/=String "" andBool EPOCH >=Int 0
       andBool TERMINAL ==Int 1 andBool DELIVERED >=Int 0
       andBool REMAIN >Int 0 andBool REFUND ==Int REMAIN
       andBool m4dU(DELIVERED) andBool m4dU(REMAIN),
       "NONRECEIPT_OR_REFUND")
  rule m4dFamilyCheck(m4dBridgeRefund(S, _, _, _, _, _, _, _))
    => m4dDecide(S, "bridge-pair/1", "unknown", false, "UNKNOWN_NONRECEIPT") [owise]

  // Vault deposit: floor assets*(supply+virtual shares)/(managed+virtual assets).
  // Fractional product remainder is metadata, never a separate asset effect.
  rule m4dFamilyCheck(m4dVault(S, POOL, ASSET, CLASS, MANAGED, SUPPLY, CUSTODY,
                    SURPLUS, VA, VS, DEPOSIT, SHARES, REM))
    => m4dDecide(S, "vault/1", "managed:" +String POOL,
       POOL =/=String "" andBool ASSET =/=String CLASS
       andBool m4dU(MANAGED) andBool m4dU(SUPPLY)
       andBool m4dU(CUSTODY) andBool m4dU(SURPLUS)
       andBool CUSTODY ==Int MANAGED +Int SURPLUS andBool SURPLUS >=Int 0
       andBool VA >Int 0 andBool VS >Int 0 andBool DEPOSIT >Int 0
       andBool m4dU(VA) andBool m4dU(VS) andBool m4dU(DEPOSIT)
       andBool m4dU(SUPPLY +Int VS) andBool m4dU(MANAGED +Int VA)
       andBool m4dProduct(DEPOSIT, SUPPLY +Int VS)
       andBool SHARES >Int 0 andBool m4dU(MANAGED +Int DEPOSIT)
       andBool m4dU(SUPPLY +Int SHARES)
       andBool m4dProduct(SHARES, MANAGED +Int VA)
       andBool m4dU(SHARES *Int (MANAGED +Int VA) +Int REM)
       andBool DEPOSIT *Int (SUPPLY +Int VS)
                  ==Int SHARES *Int (MANAGED +Int VA) +Int REM
       andBool REM >=Int 0 andBool REM <Int MANAGED +Int VA,
       "SHARE_QUOTIENT_OR_CUSTODY")
endmodule
