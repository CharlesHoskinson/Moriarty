// MIL/4 proposal composition: six ordered judgments for the M4T escrow slice.
// No configuration or native verification rule is defined here.
requires "sol-static.k"
requires "sol-transition.k"
requires "sol-formula.k"
module M4C-SYNTAX
  imports M4S-SYNTAX
  imports M4T-SYNTAX
  imports M4F-SYNTAX
  imports INT-SYNTAX
  imports STRING-SYNTAX
  imports BOOL-SYNTAX

  // Closed finite snapshots. The order and keys are canonical for this slice.
  syntax M4CCells ::= m4cNoCells() | m4cCell(String,String,M4CCells)
  syntax M4CEffects ::= m4cNoEffects() | m4cEffect(String,String,M4CEffects)

  // A qualified claim is an EXTERNAL premise. K provides no rule that creates it.
  // It binds the exact source identity, typed contract, selected policy, event,
  // complete pre/post snapshots and complete effect list.
  syntax M4CStatement ::= m4cStatement(String,String,M4SEnv,M4SFormula,M4FFrame,Int,
     M4TPolicy,M4TState,String,M4TEvent,M4TEffects,M4TState,
     M4CCells,M4CCells,M4CEffects)
  syntax M4CQualified ::= m4cQualificationAbsent()
                        | m4cNativeQualified(M4CStatement)

  syntax M4CCandidate ::= m4cCandidate(String,String,M4SEnv,M4SFormula,M4FFrame,Int,
     M4TPolicy,M4TState,String,M4TEvent,M4TEffects,M4TState,
     M4CCells,M4CCells,M4CEffects,M4CQualified,M4TNative,M4TLedger)
    // source ID, selected program ID, static environment/formula/depth,
    // policy, predecessor, successor head, event, exact effects/post,
    // complete snapshots/lines, contract qualification, transition native,
    // and atomic ledger-consumption premises
  syntax M4CJudgment ::= m4cPass() | m4cFail(String)
  syntax M4CResult ::= m4cRejected(String,String) | m4cPrepared(M4TEffects,M4TState)
    // first field of rejection is the failed judgment name
endmodule

module M4C-CONTRACT
  imports M4C-SYNTAX
  imports M4S-STATIC
  imports M4T-TRANSITION
  imports M4F-FORMULA
  imports INT
  imports BOOL
  imports STRING

  syntax String ::= m4cBoolText(Bool) [function, total]
  rule m4cBoolText(true) => "true"
  rule m4cBoolText(false) => "false"

  // This is the entire state-cell register of the M4T/1 projection. Every
  // field has one canonical key and position; no caller-selected projection.
  syntax M4CCells ::= m4cCells(M4TState) [function, total]
  rule m4cCells(m4tState(H,S,PH,m4tBals(O,E,P,R),
                        m4tBooks(G,F,N,D,A,W),IDS,T)) =>
    m4cCell("head",H,
    m4cCell("stage",Int2String(S),
    m4cCell("phase",PH,
    m4cCell("owner.balance",Int2String(O),
    m4cCell("escrow.balance",Int2String(E),
    m4cCell("payee.balance",Int2String(P),
    m4cCell("reserve.balance",Int2String(R),
    m4cCell("gross",Int2String(G),
    m4cCell("fees",Int2String(F),
    m4cCell("net",Int2String(N),
    m4cCell("duty",Int2String(D),
    m4cCell("authority.remaining",Int2String(A),
    m4cCell("work.remaining",Int2String(W),
    m4cCell("consumed",m4cIdsText(IDS),
    m4cCell("escrow.tombstone",m4cBoolText(T),m4cNoCells())))))))))))))))

  syntax String ::= m4cIdsText(M4TIds) [function, total]
  rule m4cIdsText(m4tNil()) => "[]"
  rule m4cIdsText(m4tCons(X,XS)) =>
    "[" +String Int2String(lengthString(X)) +String ":" +String X
      +String "]" +String m4cIdsText(XS)

  // Closed effect register: zero lines remain present, and the receipt is
  // bound as a source-event consumption line.
  syntax M4CEffects ::= m4cLines(M4TEffects) [function, total]
  rule m4cLines(m4tEffects(O,E,P,R,G,F,N,D,A,RID)) =>
    m4cEffect("owner.delta",Int2String(O),
    m4cEffect("escrow.delta",Int2String(E),
    m4cEffect("payee.delta",Int2String(P),
    m4cEffect("reserve.delta",Int2String(R),
    m4cEffect("gross.increment",Int2String(G),
    m4cEffect("fees.increment",Int2String(F),
    m4cEffect("net.increment",Int2String(N),
    m4cEffect("duty.decrease",Int2String(D),
    m4cEffect("authority.use",Int2String(A),
    m4cEffect("receipt.consume",RID,m4cNoEffects()))))))))))

  syntax M4CStatement ::= m4cClaim(M4CCandidate) [function, total]
  rule m4cClaim(m4cCandidate(SRC,PROG,ENV,FORM,FRAME,DEPTH,
      POL,PRE,NH,EV,EF,POST,PCS,QCS,LINES,QUAL,NATIVE,LEDGER)) =>
    m4cStatement(SRC,PROG,ENV,FORM,FRAME,DEPTH,POL,PRE,NH,EV,EF,POST,PCS,QCS,LINES)

  // Stage: formation, concrete truth and a defined bounded transition.
  syntax M4CJudgment ::= m4cStage(M4CCandidate) [function, total]
                       | m4cStageTyped(M4FTruth,M4TPlan) [function, total]
  syntax Bool ::= m4cFrameHeads(M4FFrame,M4TState,M4TState) [function, total]
  rule m4cFrameHeads(m4fFrame(PH,QH,_,_,_),m4tState(H,_,_,_,_,_,_),
                     m4tState(NH,_,_,_,_,_,_)) =>
    PH ==String H andBool QH ==String NH
  rule m4cStage(m4cCandidate(SRC,PROG,ENV,FORM,FRAME,DEPTH,
      POL,PRE,NH,EV,EF,POST,PCS,QCS,LINES,QUAL,NATIVE,LEDGER)) =>
    m4cStageTyped(m4fEvaluate(ENV,FORM,m4sOutcome(),FRAME,DEPTH),
                  m4tPlan(POL,PRE,NH,EV))
    requires m4cFrameHeads(FRAME,PRE,POST)
  rule m4cStage(m4cCandidate(_,_,_,_,FRAME,_,_,PRE,_,_,_,POST,
                             _,_,_,_,_,_)) => m4cFail("FRAME_HEAD_MISMATCH")
    requires notBool m4cFrameHeads(FRAME,PRE,POST)
  rule m4cStageTyped(m4fReject(C),_) => m4cFail("FORMULA_UNDEFINED:" +String C)
  rule m4cStageTyped(m4fFalse(),_) => m4cFail("FORMULA_FALSE")
  rule m4cStageTyped(m4fTrue(),m4tInvalid(C)) => m4cFail(C)
  rule m4cStageTyped(m4fTrue(),m4tValid(_,_)) => m4cPass()

  // Intent: exact selected program and source event must be covered by a
  // qualified, signed contract statement. Strings alone cannot qualify it.
  syntax M4CJudgment ::= m4cIntent(M4CCandidate) [function, total]
  rule m4cIntent(C) => m4cPass()
    requires m4cQualifiedOf(C) ==K m4cNativeQualified(m4cClaim(C))
      andBool m4cSourceOf(C) =/=String ""
      andBool m4cProgramOf(C) =/=String ""
  rule m4cIntent(_) => m4cFail("SIGNED_INTENT_OR_SOURCE_PREMISE_ABSENT") [owise]

  syntax M4CQualified ::= m4cQualifiedOf(M4CCandidate) [function, total]
  syntax String ::= m4cSourceOf(M4CCandidate) [function, total]
                  | m4cProgramOf(M4CCandidate) [function, total]
  rule m4cQualifiedOf(m4cCandidate(_,_,_,_,_,_,_,_,_,_,_,_,_,_,_,Q,_,_)) => Q
  rule m4cSourceOf(m4cCandidate(S,_,_,_,_,_,_,_,_,_,_,_,_,_,_,_,_,_)) => S
  rule m4cProgramOf(m4cCandidate(_,P,_,_,_,_,_,_,_,_,_,_,_,_,_,_,_,_)) => P

  // Effect: exact finite cells, exact lines, post equality, and conservation.
  syntax M4CJudgment ::= m4cEffectJudgment(M4CCandidate) [function, total]
  rule m4cEffectJudgment(m4cCandidate(_,_,_,_,_,_,POL,PRE,NH,EV,EF,POST,
                                  PCS,QCS,LINES,_,_,_)) => m4cPass()
    requires PCS ==K m4cCells(PRE) andBool QCS ==K m4cCells(POST)
      andBool LINES ==K m4cLines(EF) andBool m4tConserves(EF)
      andBool m4tPlan(POL,PRE,NH,EV) ==K m4tValid(EF,POST)
      andBool m4tShape(POL,POST)
  rule m4cEffectJudgment(_) => m4cFail("COMPLETE_EFFECT_OR_POST") [owise]

  // Authority: the separate transition-native premise must cover exact
  // predecessor, event, effects, successor, profile, episode and intent digest.
  syntax M4CJudgment ::= m4cAuthority(M4CCandidate) [function, total]
  rule m4cAuthority(m4cCandidate(_,_,_,_,_,_,
      m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID),
      m4tState(H,S,PH,B,BOOKS,IDS,T),NH,EV,EF,POST,
      _,_,_,_,NATIVE,_)) => m4cPass()
    requires NATIVE ==K m4tNativeQualified(
      m4tStatement("MIL/4-M4T/1",
        m4tPolicy(EP,OW,PY,AS,GC,FC,NG,WC,PR,ID),H,S,EV,EF,POST))
  rule m4cAuthority(_) => m4cFail("NATIVE_AUTHORITY_PREMISE_ABSENT") [owise]

  // History: only the ledger can reserve/consume the current head and event
  // receipt atomically. Local list membership does not prove global uniqueness.
  syntax M4CJudgment ::= m4cHistory(M4CCandidate) [function, total]
  rule m4cHistory(m4cCandidate(_,_,_,_,_,_,POL,
      m4tState(H,S,PH,B,BOOKS,IDS,T),NH,EV,EF,POST,
      _,_,_,_,_,LEDGER)) => m4cPass()
    requires LEDGER ==K m4tLedgerReserved(m4tReceipt(EV),H)
      andBool NH ==String m4tHeadOf(POL,
        m4tState(H,S,PH,B,BOOKS,IDS,T),EV,EF)
      andBool notBool m4tMember(m4tReceipt(EV),IDS)
  rule m4cHistory(_) => m4cFail("LEDGER_HEAD_OR_CONSUMPTION_ABSENT") [owise]

  // Failure: an unknown event makes no financial transfer; a refund closes
  // escrow but retains the unpaid duty and previously consumed authority.
  syntax M4CJudgment ::= m4cFailure(M4CCandidate) [function, total]
  rule m4cFailure(m4cCandidate(_,_,_,_,_,_,_,
      m4tState(_,_,_,_,m4tBooks(G,F,N,D,A,W),_,_),_,
      m4tMarkUnknown(_),m4tEffects(0,0,0,0,0,0,0,0,0,_),
      m4tState(_,_,"unknown",_,m4tBooks(G,F,N,D,A,_),_,false),
      _,_,_,_,_,_)) => m4cPass()
  rule m4cFailure(m4cCandidate(_,_,_,_,_,_,_,
      m4tState(_,_,_,_,m4tBooks(G,F,N,D,A,W),_,_),_,
      m4tRefund(_,_),m4tEffects(_,_,0,0,0,0,0,0,0,_),
      m4tState(_,_,"recovering",_,m4tBooks(G,F,N,D,A,_),_,true),
      _,_,_,_,_,_)) => m4cPass()
  rule m4cFailure(m4cCandidate(_,_,_,_,_,_,_,_,_,m4tFund(_,_),_,_,
      _,_,_,_,_,_)) => m4cPass()
  rule m4cFailure(m4cCandidate(_,_,_,_,_,_,_,_,_,m4tRelease(_,_,_,_),_,_,
      _,_,_,_,_,_)) => m4cPass()
  rule m4cFailure(_) => m4cFail("FAILURE_DUTY_OR_PHASE") [owise]

  // Fixed order preserves the rejecting judgment. Nothing is accepted until
  // all six pass; a rejection has no accepted state/effects in this relation.
  // Two diagnostic checks run first: a changed predecessor selects history,
  // and a dropped unpaid duty selects failure before effect equality.
  syntax Bool ::= m4cPredecessorBound(M4CCandidate) [function, total]
                | m4cDutyRetained(M4CCandidate) [function, total]
  rule m4cPredecessorBound(m4cCandidate(_,_,_,_,FRAME,_,_,
      m4tState(H,S,PH,B,BOOKS,IDS,T),NH,EV,EF,POST,
      _,_,_,_,_,LEDGER)) =>
    m4cFrameHeads(FRAME,m4tState(H,S,PH,B,BOOKS,IDS,T),POST)
    andBool LEDGER ==K m4tLedgerReserved(m4tReceipt(EV),H)
  rule m4cDutyRetained(m4cCandidate(_,_,_,_,_,_,_,
      m4tState(_,_,_,_,m4tBooks(_,_,_,D,_,_),_,_),_,_,
      m4tEffects(_,_,_,_,_,_,_,DEC,_,_),
      m4tState(_,_,_,_,m4tBooks(_,_,_,NEXT,_,_),_,_),
      _,_,_,_,_,_)) => DEC >=Int 0 andBool NEXT ==Int D -Int DEC

  syntax M4CResult ::= m4cAccept(M4CCandidate) [function, total]
                     | m4cPriority(M4CCandidate) [function, total]
                     | m4cAfterStage(M4CJudgment,M4CCandidate) [function, total]
                     | m4cAfterIntent(M4CJudgment,M4CCandidate) [function, total]
                     | m4cAfterEffect(M4CJudgment,M4CCandidate) [function, total]
                     | m4cAfterAuthority(M4CJudgment,M4CCandidate) [function, total]
                     | m4cAfterHistory(M4CJudgment,M4CCandidate) [function, total]
                     | m4cAfterFailure(M4CJudgment,M4CCandidate) [function, total]
  rule m4cAccept(C) => m4cPriority(C)
  rule m4cPriority(C) => m4cRejected("history","PREDECESSOR_HEAD_MISMATCH")
    requires notBool m4cPredecessorBound(C)
  rule m4cPriority(C) => m4cRejected("failure","DUTY_DROPPED")
    requires m4cPredecessorBound(C) andBool notBool m4cDutyRetained(C)
  rule m4cPriority(C) => m4cAfterStage(m4cStage(C),C)
    requires m4cPredecessorBound(C) andBool m4cDutyRetained(C)
  rule m4cAfterStage(m4cFail(C),_) => m4cRejected("stage",C)
  rule m4cAfterStage(m4cPass(),C) => m4cAfterIntent(m4cIntent(C),C)
  rule m4cAfterIntent(m4cFail(C),_) => m4cRejected("intent",C)
  rule m4cAfterIntent(m4cPass(),C) => m4cAfterEffect(m4cEffectJudgment(C),C)
  rule m4cAfterEffect(m4cFail(C),_) => m4cRejected("effect",C)
  rule m4cAfterEffect(m4cPass(),C) => m4cAfterAuthority(m4cAuthority(C),C)
  rule m4cAfterAuthority(m4cFail(C),_) => m4cRejected("authority",C)
  rule m4cAfterAuthority(m4cPass(),C) => m4cAfterHistory(m4cHistory(C),C)
  rule m4cAfterHistory(m4cFail(C),_) => m4cRejected("history",C)
  rule m4cAfterHistory(m4cPass(),C) => m4cAfterFailure(m4cFailure(C),C)
  rule m4cAfterFailure(m4cFail(C),_) => m4cRejected("failure",C)
  rule m4cAfterFailure(m4cPass(),m4cCandidate(_,_,_,_,_,_,_,_,_,_,EF,POST,
                                            _,_,_,_,_,_)) => m4cPrepared(EF,POST)
endmodule
