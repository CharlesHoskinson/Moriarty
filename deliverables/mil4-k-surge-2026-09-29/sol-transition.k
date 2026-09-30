// MIL/4 proposal: bounded, one-domain, one-asset conditional escrow episode.
// Standalone reference relation. Native and ledger premises are opaque inputs
// supplied by a qualified boundary; their constructors are not proof rules.
module M4T-SYNTAX
  imports INT-SYNTAX
  imports STRING-SYNTAX
  imports BOOL-SYNTAX
  syntax M4TIds ::= m4tNil() | m4tCons(String,M4TIds)
  syntax M4TBals ::= m4tBals(Int,Int,Int,Int) // owner, escrow, payee, reserve
  syntax M4TBooks ::= m4tBooks(Int,Int,Int,Int,Int,Int)
    // cumulative gross, cumulative fees, cumulative net, unpaid duty,
    // remaining spend authority, remaining episode work
  syntax M4TState ::= m4tState(String,Int,String,M4TBals,M4TBooks,M4TIds,Bool)
    // authenticated head, stage, phase, balances, books, consumed IDs, escrow tombstone
  syntax M4TPolicy ::= m4tPolicy(String,String,String,String,Int,Int,Int,Int,String,String)
    // episode, owner, payee, asset, gross cap, fee cap, net goal,
    // stage work cap, late-race priority, signed intent digest
  syntax M4TObservation ::= m4tPending() | m4tUnknown() | m4tSuccess()
                          | m4tTerminalNonreceipt() | m4tRemainderNonreceipt(Int)
                          | m4tBoth()
  syntax M4TEvent ::= m4tFund(Int,String)
                    | m4tMarkUnknown(String)
                    | m4tRelease(Int,Int,String,M4TObservation)
                    | m4tRefund(String,M4TObservation)
  syntax M4TEffects ::= m4tEffects(Int,Int,Int,Int,Int,Int,Int,Int,Int,String)
    // exact owner/escrow/payee/reserve deltas; gross, fee, net, duty decrease,
    // authority use, receipt. Zero is mandatory for every unchanged line.
  syntax M4TPlan ::= m4tInvalid(String) | m4tValid(M4TEffects,M4TState)
  syntax M4TStatement ::= m4tStatement(String,M4TPolicy,String,Int,
                                      M4TEvent,M4TEffects,M4TState)
    // profile, entire signed policy, predecessor head, stage, event,
    // complete effects, exact successor state
  syntax M4TNative ::= m4tNativeAbsent() | m4tNativeQualified(M4TStatement)
  syntax M4TLedger ::= m4tLedgerAbsent() | m4tLedgerReserved(String,String)
    // one-use receipt and current authenticated head, checked at ledger acceptance
  syntax M4TRequest ::= m4tRun(M4TPolicy,M4TState,String,M4TEvent,M4TEffects,
                                M4TState,M4TNative,M4TLedger)
  syntax M4TResult ::= m4tWaiting()
                     | m4tRejected(String)
                     | m4tPrepared(M4TEffects,M4TState)
endmodule

module M4T-TRANSITION
  imports M4T-SYNTAX
  imports INT
  imports BOOL
  imports STRING
  imports K-EQUAL

  syntax Bool ::= m4tMember(String,M4TIds) [function, total]
  rule m4tMember(_,m4tNil()) => false
  rule m4tMember(X,m4tCons(X,_)) => true
  rule m4tMember(X,m4tCons(Y,YS)) => m4tMember(X,YS)
    requires X =/=String Y

  syntax Bool ::= m4tNonnegative(M4TBals) [function, total]
  rule m4tNonnegative(m4tBals(O,E,P,R)) =>
    O >=Int 0 andBool E >=Int 0 andBool P >=Int 0 andBool R >=Int 0

  syntax Bool ::= m4tPolicyOK(M4TPolicy) [function, total]
  rule m4tPolicyOK(m4tPolicy(EP,OW,PY,AS,_,_,_,_,PR,ID)) =>
    EP =/=String "" andBool OW =/=String "" andBool PY =/=String ""
    andBool AS =/=String "" andBool ID =/=String ""
    andBool OW =/=String PY
    andBool (PR ==String "release" orBool PR ==String "refund")

  syntax Bool ::= m4tBooksOK(M4TPolicy,M4TBooks) [function, total]
  rule m4tBooksOK(m4tPolicy(_,_,_,_,GC,FC,NG,WC,_,_),
                  m4tBooks(G,F,N,D,A,W)) =>
    GC >=Int 0 andBool FC >=Int 0 andBool NG >=Int 0 andBool WC >Int 0
    andBool G >=Int 0 andBool G <=Int GC
    andBool F >=Int 0 andBool F <=Int FC
    andBool N >=Int 0 andBool N <=Int NG
    andBool D >=Int 0 andBool D +Int N ==Int NG
    andBool A >=Int 0 andBool A +Int G ==Int GC
    andBool W >=Int 0 andBool W <=Int WC
    andBool NG <=Int GC

  syntax Bool ::= m4tConserves(M4TEffects) [function, total]
  rule m4tConserves(m4tEffects(O,E,P,R,_,_,_,_,_,_)) =>
    O +Int E +Int P +Int R ==Int 0

  syntax Bool ::= m4tShape(M4TPolicy,M4TState) [function, total]
  rule m4tShape(P,m4tState(_,S,PH,B,m4tBooks(G,F,N,D,A,W),_,T)) =>
    S >=Int 0 andBool
    (PH ==String "unfunded" orBool PH ==String "pending"
      orBool PH ==String "unknown" orBool PH ==String "recovering"
      orBool PH ==String "complete")
    andBool (T ==Bool (PH ==String "recovering" orBool PH ==String "complete"))
    andBool ((PH =/=String "recovering" andBool PH =/=String "complete")
      orBool m4tEscrowZero(B))
    andBool ((PH =/=String "unfunded") orBool
      (G ==Int 0 andBool F ==Int 0 andBool N ==Int 0 andBool m4tEscrowZero(B)))
    andBool ((PH =/=String "complete") orBool D ==Int 0)
    andBool ((PH =/=String "pending" andBool PH =/=String "unknown")
      orBool notBool m4tEscrowZero(B))
    andBool G >=Int N +Int F +Int m4tEscrowAmount(B)
    andBool m4tPolicyOK(P) andBool m4tNonnegative(B)
    andBool m4tBooksOK(P,m4tBooks(G,F,N,D,A,W))

  // This derived monotone quantity records funds returned to the owner.
  // Gross and authority are never replenished by a refund.
  syntax Int ::= m4tRefunded(M4TState) [function, total]
  rule m4tRefunded(m4tState(_,_,_,m4tBals(_,E,_,_),
                            m4tBooks(G,F,N,_,_,_),_,_)) => G -Int N -Int F -Int E
  syntax Int ::= m4tEscrowAmount(M4TBals) [function, total]
  rule m4tEscrowAmount(m4tBals(_,E,_,_)) => E

  syntax Bool ::= m4tEscrowZero(M4TBals) [function, total]
  rule m4tEscrowZero(m4tBals(_,E,_,_)) => E ==Int 0

  syntax Bool ::= m4tReleaseWins(String,M4TObservation) [function, total]
  rule m4tReleaseWins(_,m4tSuccess()) => true
  rule m4tReleaseWins("release",m4tBoth()) => true
  rule m4tReleaseWins(_,_) => false [owise]
  syntax Bool ::= m4tRefundWins(String,M4TObservation,Int,Int) [function, total]
  rule m4tRefundWins(_,m4tTerminalNonreceipt(),0,D) => D >Int 0
  rule m4tRefundWins(_,m4tRemainderNonreceipt(R),N,D) =>
    N >Int 0 andBool R ==Int D andBool D >Int 0
  rule m4tRefundWins("refund",m4tBoth(),0,D) => D >Int 0
  rule m4tRefundWins(_,_,_,_) => false [owise]

  // Symbolic extension over the complete predecessor, policy, event and
  // effects. Length prefixes make concatenation unambiguous at this level.
  // This is not a cryptographic commitment or a native hash implementation.
  syntax String ::= m4tPack(String) [function, total]
                  | m4tPolicyText(M4TPolicy) [function, total]
                  | m4tStateText(M4TState) [function, total]
                  | m4tIdsText(M4TIds) [function, total]
                  | m4tObsText(M4TObservation) [function, total]
                  | m4tEventText(M4TEvent) [function, total]
                  | m4tEffectsText(M4TEffects) [function, total]
                  | m4tHeadOf(M4TPolicy,M4TState,M4TEvent,M4TEffects) [function, total]
  rule m4tPack(X) => Int2String(lengthString(X)) +String ":" +String X
  rule m4tIdsText(m4tNil()) => "N"
  rule m4tIdsText(m4tCons(X,XS)) => "C" +String m4tPack(X)
    +String m4tPack(m4tIdsText(XS))
  rule m4tPolicyText(m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID)) =>
    m4tPack(EP) +String m4tPack(OW) +String m4tPack(PY) +String m4tPack(AS)
    +String m4tPack(Int2String(GC)) +String m4tPack(Int2String(FC))
    +String m4tPack(Int2String(NG)) +String m4tPack(Int2String(WC))
    +String m4tPack(PR) +String m4tPack(ID)
  rule m4tStateText(m4tState(H,S,PH,m4tBals(O,E,P,R),
                             m4tBooks(G,F,N,D,A,W),IDS,T)) =>
    m4tPack(H) +String m4tPack(Int2String(S)) +String m4tPack(PH)
    +String m4tPack(Int2String(O)) +String m4tPack(Int2String(E))
    +String m4tPack(Int2String(P)) +String m4tPack(Int2String(R))
    +String m4tPack(Int2String(G)) +String m4tPack(Int2String(F))
    +String m4tPack(Int2String(N)) +String m4tPack(Int2String(D))
    +String m4tPack(Int2String(A)) +String m4tPack(Int2String(W))
    +String m4tPack(m4tIdsText(IDS))
    +String m4tPack(m4tBoolText(T))
  syntax String ::= m4tBoolText(Bool) [function, total]
  rule m4tBoolText(true) => "true"
  rule m4tBoolText(false) => "false"
  rule m4tObsText(m4tPending()) => "pending"
  rule m4tObsText(m4tUnknown()) => "unknown"
  rule m4tObsText(m4tSuccess()) => "success"
  rule m4tObsText(m4tTerminalNonreceipt()) => "nonreceipt"
  rule m4tObsText(m4tRemainderNonreceipt(R)) =>
    "remainder:" +String m4tPack(Int2String(R))
  rule m4tObsText(m4tBoth()) => "both"
  rule m4tEventText(m4tFund(Q,RID)) =>
    "fund:" +String m4tPack(Int2String(Q)) +String m4tPack(RID)
  rule m4tEventText(m4tMarkUnknown(RID)) => "unknown:" +String m4tPack(RID)
  rule m4tEventText(m4tRelease(Q,F,RID,OBS)) =>
    "release:" +String m4tPack(Int2String(Q))
    +String m4tPack(Int2String(F)) +String m4tPack(RID)
    +String m4tPack(m4tObsText(OBS))
  rule m4tEventText(m4tRefund(RID,OBS)) =>
    "refund:" +String m4tPack(RID) +String m4tPack(m4tObsText(OBS))
  rule m4tEffectsText(m4tEffects(O,E,P,R,G,F,N,D,A,RID)) =>
    m4tPack(Int2String(O)) +String m4tPack(Int2String(E))
    +String m4tPack(Int2String(P)) +String m4tPack(Int2String(R))
    +String m4tPack(Int2String(G)) +String m4tPack(Int2String(F))
    +String m4tPack(Int2String(N)) +String m4tPack(Int2String(D))
    +String m4tPack(Int2String(A)) +String m4tPack(RID)
  rule m4tHeadOf(POL,PRE,EV,EF) => "M4T/1:"
    +String m4tPack(m4tPolicyText(POL))
    +String m4tPack(m4tStateText(PRE))
    +String m4tPack(m4tEventText(EV))
    +String m4tPack(m4tEffectsText(EF))

  // A plan is calculated from the entire authenticated predecessor. The
  // candidate must match both the complete effect vector and successor state.
  syntax M4TPlan ::= m4tPlan(M4TPolicy,M4TState,String,M4TEvent) [function, total]

  rule m4tPlan(m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID),
               m4tState(H,S,"unfunded",m4tBals(O,E,P,R),
                        m4tBooks(G,F,N,D,A,W),IDS,false),
               NH,
               m4tFund(Q,RID)) =>
    m4tValid(m4tEffects(0 -Int Q,Q,0,0,Q,0,0,0,Q,RID),
      m4tState(NH,S +Int 1,"pending",m4tBals(O -Int Q,E +Int Q,P,R),
               m4tBooks(G +Int Q,F,N,D,A -Int Q,W -Int 1),
               m4tCons(RID,IDS),false))
    requires NH ==String m4tHeadOf(
      m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID),
      m4tState(H,S,"unfunded",m4tBals(O,E,P,R),
               m4tBooks(G,F,N,D,A,W),IDS,false),
      m4tFund(Q,RID),m4tEffects(0 -Int Q,Q,0,0,Q,0,0,0,Q,RID))
      andBool Q >Int 0 andBool Q <=Int O andBool Q <=Int A
      andBool G +Int Q <=Int GC andBool W >Int 1
      andBool Q >=Int D
      andBool notBool m4tMember(RID,IDS)
      andBool m4tShape(m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID),
                            m4tState(H,S,"unfunded",m4tBals(O,E,P,R),
                              m4tBooks(G,F,N,D,A,W),IDS,false))

  rule m4tPlan(POL,m4tState(H,S,PH,B,m4tBooks(G,F,N,D,A,W),IDS,false),NH,
               m4tMarkUnknown(RID)) =>
    m4tValid(m4tEffects(0,0,0,0,0,0,0,0,0,RID),
      m4tState(NH,S +Int 1,"unknown",B,
               m4tBooks(G,F,N,D,A,W -Int 1),m4tCons(RID,IDS),false))
    requires NH ==String m4tHeadOf(POL,
      m4tState(H,S,PH,B,m4tBooks(G,F,N,D,A,W),IDS,false),
      m4tMarkUnknown(RID),m4tEffects(0,0,0,0,0,0,0,0,0,RID))
      andBool PH ==String "pending"
      andBool W >Int 1 andBool notBool m4tMember(RID,IDS)
      andBool m4tShape(POL,m4tState(H,S,PH,B,
                                   m4tBooks(G,F,N,D,A,W),IDS,false))

  // W-D4 has not selected a fee formula or beneficiary. The current slice
  // admits only zero fee, including when the policy's fee cap is positive.
  // Partial release leaves positive custody and a closing work slot.
  rule m4tPlan(m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID),
               m4tState(H,S,PH,m4tBals(O,E,P,R),
                        m4tBooks(G,F,N,D,A,W),IDS,false),NH,
               m4tRelease(Q,0,RID,OBS)) =>
    m4tValid(m4tEffects(0,0 -Int Q,Q,0,0,0,Q,Q,0,RID),
      m4tState(NH,S +Int 1,"pending",m4tBals(O,E -Int Q,P +Int Q,R),
               m4tBooks(G,F,N +Int Q,D -Int Q,A,W -Int 1),
               m4tCons(RID,IDS),false))
    requires NH ==String m4tHeadOf(
      m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID),
      m4tState(H,S,PH,m4tBals(O,E,P,R),
               m4tBooks(G,F,N,D,A,W),IDS,false),
      m4tRelease(Q,0,RID,OBS),
      m4tEffects(0,0 -Int Q,Q,0,0,0,Q,Q,0,RID))
      andBool (PH ==String "pending" orBool PH ==String "unknown")
      andBool m4tReleaseWins(PR,OBS)
      andBool Q >Int 0 andBool Q <Int D andBool Q <Int E
      andBool W >Int 1 andBool notBool m4tMember(RID,IDS)
      andBool m4tShape(m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID),
                            m4tState(H,S,PH,m4tBals(O,E,P,R),
                              m4tBooks(G,F,N,D,A,W),IDS,false))

  // On final release, return every surplus unit to the owner. It remains
  // charged to cumulative gross and residual authority is not restored.
  rule m4tPlan(m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID),
               m4tState(H,S,PH,m4tBals(O,E,P,R),
                        m4tBooks(G,F,N,D,A,W),IDS,false),NH,
               m4tRelease(Q,0,RID,OBS)) =>
    m4tValid(m4tEffects(E -Int Q,0 -Int E,Q,0,0,0,Q,Q,0,RID),
      m4tState(NH,S +Int 1,"complete",m4tBals(O +Int E -Int Q,0,P +Int Q,R),
               m4tBooks(G,F,N +Int Q,0,A,W -Int 1),
               m4tCons(RID,IDS),true))
    requires NH ==String m4tHeadOf(
      m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID),
      m4tState(H,S,PH,m4tBals(O,E,P,R),
               m4tBooks(G,F,N,D,A,W),IDS,false),
      m4tRelease(Q,0,RID,OBS),
      m4tEffects(E -Int Q,0 -Int E,Q,0,0,0,Q,Q,0,RID))
      andBool (PH ==String "pending" orBool PH ==String "unknown")
      andBool m4tReleaseWins(PR,OBS)
      andBool Q >Int 0 andBool Q ==Int D andBool Q <=Int E
      andBool W >Int 0 andBool notBool m4tMember(RID,IDS)
      andBool m4tShape(m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID),
                            m4tState(H,S,PH,m4tBals(O,E,P,R),
                              m4tBooks(G,F,N,D,A,W),IDS,false))

  // Refund closes this escrow only on certified terminal nonreceipt of the
  // remaining claim. Unpaid duty is carried into recovering, not erased.
  rule m4tPlan(m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID),
               m4tState(H,S,PH,m4tBals(O,E,P,R),
                        m4tBooks(G,F,N,D,A,W),IDS,false),
               NH,
               m4tRefund(RID,OBS)) =>
    m4tValid(m4tEffects(E,0 -Int E,0,0,0,0,0,0,0,RID),
      m4tState(NH,S +Int 1,"recovering",m4tBals(O +Int E,0,P,R),
               m4tBooks(G,F,N,D,A,W -Int 1),m4tCons(RID,IDS),true))
    requires NH ==String m4tHeadOf(
      m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID),
      m4tState(H,S,PH,m4tBals(O,E,P,R),
               m4tBooks(G,F,N,D,A,W),IDS,false),
      m4tRefund(RID,OBS),m4tEffects(E,0 -Int E,0,0,0,0,0,0,0,RID))
      andBool
      (PH ==String "pending" orBool PH ==String "unknown")
      andBool m4tRefundWins(PR,OBS,N,D) andBool E >Int 0
      andBool W >Int 0 andBool notBool m4tMember(RID,IDS)
      andBool m4tShape(m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID),
                            m4tState(H,S,PH,m4tBals(O,E,P,R),
                              m4tBooks(G,F,N,D,A,W),IDS,false))

  rule m4tPlan(_,_,_,_) => m4tInvalid("STAGE_OR_EVIDENCE") [owise]

  syntax String ::= m4tPhase(Int) [function, total]
  rule m4tPhase(0) => "complete"
  rule m4tPhase(N) => "pending" requires N >Int 0
  rule m4tPhase(N) => "invalid" requires N <Int 0
  syntax Bool ::= m4tDone(Int) [function, total]
  rule m4tDone(0) => true
  rule m4tDone(N) => false requires N >Int 0
  rule m4tDone(N) => false requires N <Int 0

  syntax M4TResult ::= m4tEvaluate(M4TRequest) [function, total]
                     | m4tInspect(M4TPolicy,M4TState,M4TEvent,M4TEffects,
                                  M4TState,M4TNative,M4TLedger,M4TPlan)
                       [function, total]
                     | m4tRequire(Bool,String,M4TResult) [function, total]
  rule m4tEvaluate(m4tRun(P,S,NH,EV,EF,POST,NATIVE,LEDGER)) =>
    m4tInspect(P,S,EV,EF,POST,NATIVE,LEDGER,m4tPlan(P,S,NH,EV))
  rule m4tInspect(_,_,_,_,_,_,_,m4tInvalid(CODE)) => m4tRejected(CODE)
  rule m4tRequire(false,CODE,_) => m4tRejected(CODE)
  rule m4tRequire(true,_,RESULT) => RESULT

  rule m4tInspect(POL,
                  m4tState(H,S,PH,B,Q,IDS,T),EV,EF,POST,NATIVE,LEDGER,
                  m4tValid(EXPECTED,EXPECTEDPOST)) =>
    m4tRequire(m4tConserves(EF),"CONSERVATION",
      m4tRequire(EF ==K EXPECTED,"INCOMPLETE_EFFECTS",
        m4tRequire(POST ==K EXPECTEDPOST,"WRONG_POST_STATE",
          m4tRequire(m4tShape(POL,POST),
                     "POST_INVARIANT",
            m4tRequire(NATIVE ==K m4tNativeQualified(
              m4tStatement("MIL/4-M4T/1",POL,H,S,EV,EF,POST)),
                       "NATIVE_PREMISE_ABSENT",
              m4tRequire(LEDGER ==K m4tLedgerReserved(m4tReceipt(EV),H),
                         "LEDGER_CONSUMPTION_ABSENT",
                m4tPrepared(EF,POST)))))))

  syntax String ::= m4tReceipt(M4TEvent) [function, total]
  rule m4tReceipt(m4tFund(_,R)) => R
  rule m4tReceipt(m4tMarkUnknown(R)) => R
  rule m4tReceipt(m4tRelease(_,_,R,_)) => R
  rule m4tReceipt(m4tRefund(R,_)) => R
endmodule
