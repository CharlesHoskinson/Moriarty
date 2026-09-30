// Provisional Source/6 -> Core/5 S0 stage. External verification is abstract.
module MIL4-S0-SYNTAX
  imports INT-SYNTAX
  imports STRING-SYNTAX
  // Opaque head identity. A trusted premise must authenticate the submitted successor.
  syntax Head ::= head(String) [symbol(head)]
  syntax ReplayKey ::= replayKey(String, String, String) [symbol(replayKey)]
  syntax ReplaySet ::= noReplays() [symbol(noReplays)] | used(ReplayKey, ReplaySet) [symbol(used)]
  syntax Balance ::= noBalance() [symbol(noBalance)] | balance(String, String, Int) [symbol(balance)]
  syntax Allowance ::= allowance(String, String, Int, Int) [symbol(allowance)]
  syntax Obligation ::= noObligation() [symbol(noObligation)]
                      | obligation(String, String, String, String, Int, Int, Int, String) [symbol(obligation)]
  syntax State ::= state(Balance, Balance, Balance, Allowance, Obligation, Head, ReplaySet, Int, Int) [symbol(state)]
  syntax Action ::= transfer(Int, Int) [symbol(transfer)] | repay(String, String, Int, Int, Int, String) [symbol(repay)]
  // The intent digest is an opaque identifier; its exact signed bytes remain W-D1/W-D2.
  syntax Intent ::= intent(String, String, String, String, String, String, Head, Int, Int,
                           String, String, Int, Int, Int, Action, String) [symbol(intent)]
  syntax Fill ::= fill(String, String, String, String, Head, String, String,
                       Int, Int, Int, Action, Head) [symbol(fill)]
  syntax Effect ::= debit(String, String, Int) [symbol(debit)]
                  | credit(String, String, Int) [symbol(credit)]
                  | setObligation(String, Int, Int, Int, String) [symbol(setObligation)]
                  | useAllowance(String, String, Int) [symbol(useAllowance)]
                  | useReplay(ReplayKey) [symbol(useReplay)]
                  | advanceHead(Head, Head) [symbol(advanceHead)]
  syntax Effects ::= noEffects() [symbol(noEffects)] | effect(Effect, Effects) [symbol(effect)]
  syntax Phase ::= terminalSuccess() [symbol(terminalSuccess)] | requestedFailure(String) [symbol(requestedFailure)]
  syntax Duty ::= noDuty() [symbol(noDuty)] | retainedDuty(String) [symbol(retainedDuty)]
  syntax Duties ::= noDuties() [symbol(noDuties)]
                    | duty(String, Duties) [symbol(duty)]
  // Requested outcome is typed and bound by the stipulated premise. It is not
  // evidence of a signature or supported failure semantics.
  syntax Outcome ::= outcome(Phase, Effects, Duties) [symbol(outcome)]
  syntax Premise ::= unavailable() [symbol(unavailable)]
                   | authenticated(Intent, State, Int, Head, Outcome) [symbol(authenticated)]
  syntax Family ::= ammCP1() [symbol(ammCP1)] | loanFixed1() [symbol(loanFixed1)]
                  | cdp1() [symbol(cdp1)] | optCapped1() [symbol(optCapped1)]
                  | obs1() [symbol(obs1)] | gov1() [symbol(gov1)]
                  | bridgePair1() [symbol(bridgePair1)] | vault1() [symbol(vault1)]
  syntax Request ::= submit(Intent, Fill, State, Effects, Outcome, Int) [symbol(submit)]
                   | submitFamily(Family) [symbol(submitFamily)]
                   | unsupported(String) [symbol(unsupported)]
  syntax Result ::= pending() [symbol(pending)]
                  | rejected(String, String, Int) [symbol(rejected)]
                  | accepted(State, Effects, State, Phase, Duty, Int) [symbol(accepted)]
endmodule

module MIL4-S0
  imports MIL4-S0-SYNTAX
  imports INT
  imports BOOL
  imports STRING
  imports K-EQUAL
  configuration <s0> <k> $PGM:Request </k> <out> pending() </out>
                     <external> unavailable() </external> </s0>

  syntax Int ::= maxU() [function] | maxNominal() [function]
  rule maxU() => 340282366920938463463374607431768211455
  rule maxNominal() => 170141183460469231731687303715884105727
  syntax Bool ::= uint(Int) [function] | nominal(Int) [function]
  rule uint(N) => N >=Int 0 andBool N <=Int maxU()
  rule nominal(N) => N >=Int 0 andBool N <=Int maxNominal()
  syntax Bool ::= hasReplay(ReplayKey, ReplaySet) [function, total]
  rule hasReplay(_, noReplays()) => false
  rule hasReplay(K, used(K, _)) => true
  rule hasReplay(K, used(J, R)) => hasReplay(K, R) requires notBool K ==K J
  syntax String ::= debtStatus(Int) [function]
  rule debtStatus(0) => "Settled"
  rule debtStatus(N) => "Outstanding" requires N >Int 0
  syntax Int ::= accruedPaid(Int, Int) [function]
  rule accruedPaid(N, A) => minInt(N, A)

  syntax Bool ::= goodState(State) [function, total]
  rule goodState(state(balance(O,A,OB),balance(R,A,RB),balance(F,A,FB),
                       allowance(O,A,AR,AS),_,_,_,WR,WS))
    => O =/=String R andBool O =/=String F andBool R =/=String F
       andBool uint(OB) andBool uint(RB) andBool uint(FB)
       andBool uint(AR) andBool uint(AS) andBool uint(WR) andBool uint(WS)
       andBool AR +Int AS <=Int maxU() andBool WR +Int WS <=Int maxU()
  rule goodState(state(balance(O,A,OB),balance(R,A,RB),noBalance(),
                       allowance(_,A,AR,AS),_,_,_,WR,WS))
    => O =/=String R andBool uint(OB) andBool uint(RB)
       andBool uint(AR) andBool uint(AS) andBool uint(WR) andBool uint(WS)
       andBool AR +Int AS <=Int maxU() andBool WR +Int WS <=Int maxU()
  rule goodState(_) => false [owise]
  syntax Bool ::= goodDebt(Obligation) [function, total]
  rule goodDebt(noObligation()) => true
  rule goodDebt(obligation(_,D,C,_,P,A,O,S))
    => D =/=String C andBool nominal(P) andBool nominal(A) andBool nominal(O)
       andBool P +Int A ==Int O andBool P +Int A <=Int maxU()
       andBool ((O >Int 0 andBool S ==String "Outstanding")
                orBool (O ==Int 0 andBool S ==String "Settled"))
  rule goodDebt(_) => false [owise]
  syntax Bool ::= matchingScope(Intent, Fill) [function, total]
  rule matchingScope(intent(_,_,D,S,A,N,H,_,_,R,F,G,FC,NF,transfer(V,Fee),_),
                     fill(D,S,A,N,H,R,F,G,FC,NF,transfer(V,Fee),_)) => true
  // Repay has no recipient or fee endpoint in Source/6; empty strings are absent fields.
  rule matchingScope(intent(_,_,D,S,A,N,H,_,_,"","",G,FC,NF,repay(ID,Payer,V,M,Scale,Rnd),_),
                     fill(D,S,A,N,H,"","",G,FC,NF,repay(ID,Payer,V,M,Scale,Rnd),_)) => true
  rule matchingScope(_,_) => false [owise]
  syntax Bool ::= actionIntentOK(Action, Int, Int, Int) [function, total]
  rule actionIntentOK(transfer(V,F),G,FC,NF)
    => V >Int 0 andBool nominal(V) andBool nominal(F) andBool nominal(G)
       andBool nominal(FC) andBool nominal(NF) andBool F <=Int FC
       andBool V +Int F <=Int G andBool V >=Int NF
  rule actionIntentOK(repay(_,_,N,1,0,"none"),G,FC,NF)
    => N >Int 0 andBool nominal(N) andBool nominal(G) andBool FC ==Int 0
       andBool NF ==Int 0 andBool N <=Int G
  rule actionIntentOK(_,_,_,_) => false [owise]

  syntax Effects ::= transferEffects(Intent, Int, Int, Head) [function]
                   | repayEffects(Intent, Obligation, Int, Head) [function]
  rule transferEffects(intent(_,_,D,S,A,N,H,_,_,R,_,_,_,_,_,_),V,0,Post)
    => effect(debit(S,A,V),effect(credit(R,A,V),
       effect(useAllowance(S,A,V),effect(useReplay(replayKey(D,S,N)),
       effect(advanceHead(H,Post),noEffects())))))
  rule transferEffects(intent(_,_,D,S,A,N,H,_,_,R,F,_,_,_,_,_),V,FEE,Post)
    => effect(debit(S,A,V +Int FEE),effect(credit(R,A,V),effect(credit(F,A,FEE),
       effect(useAllowance(S,A,V +Int FEE),effect(useReplay(replayKey(D,S,N)),
       effect(advanceHead(H,Post),noEffects()))))))
    requires FEE >Int 0
  rule repayEffects(intent(_,_,D,S,A,N,H,_,_,_,_,_,_,_,repay(_,Payer,_,_,_,_),_),
                    obligation(ID,_,C,_,P,AC,_,_),PAY,Post)
    => effect(debit(Payer,A,PAY),effect(credit(C,A,PAY),
       effect(setObligation(ID,P -Int (PAY -Int accruedPaid(PAY,AC)),
                            AC -Int accruedPaid(PAY,AC),P +Int AC -Int PAY,
                            debtStatus(P +Int AC -Int PAY)),
       effect(useAllowance(S,A,PAY),effect(useReplay(replayKey(D,S,N)),
       effect(advanceHead(H,Post),noEffects()))))))

  syntax KItem ::= s0Stage(Request) | s0Intent(Request) | s0Effect(Request)
                 | s0Authority(Request) | s0History(Request) | s0Failure(Request)
                 | s0Commit(Request) | s0Stop(String, String, Int)
  rule <k> unsupported(_) => s0Stop("stage","S0_STAGE_UNSUPPORTED",1) </k>
  rule <k> submitFamily(_) => s0Stop("stage","FAMILY_STAGE_ADAPTER_ABSENT",1) </k>
  rule <k> submit(I,F,S,E,Requested,Round) => s0Stage(submit(I,F,S,E,Requested,Round)) </k>
  rule <k> s0Stop(J,C,W) => .K </k> <out> pending() => rejected(J,C,W) </out>

  rule <k> s0Stage(submit(I,_,S,_,Requested,Round) #as R) => s0Intent(R) </k>
    <external> X </external>
    requires goodState(S) andBool goodDebt(debtOf(S)) andBool selected(I)
             andBool stageShape(I,S) andBool premiseOK(X,I,S,Round,Requested)
  rule <k> s0Stage(submit(I,_,S,_,Requested,Round)) => s0Stop("stage","S0_STAGE_PREMISE",1) </k>
    <external> X </external>
    requires goodState(S) andBool goodDebt(debtOf(S)) andBool selected(I)
             andBool stageShape(I,S) andBool notBool premiseOK(X,I,S,Round,Requested)
  rule <k> s0Stage(_) => s0Stop("stage","S0_STAGE_UNSUPPORTED",1) </k> [owise]
  syntax Obligation ::= debtOf(State) [function]
  rule debtOf(state(_,_,_,_,O,_,_,_,_)) => O
  syntax Bool ::= selected(Intent) [function, total]
  rule selected(intent("Source/6","Core/5",_,_,_,_,_,_,_,_,_,_,_,_,transfer(_,_),_)) => true
  rule selected(intent("Source/6","Core/5",_,_,_,_,_,_,_,_,_,_,_,_,repay(_,_,_,_,_,_),_)) => true
  rule selected(_) => false [owise]
  syntax Bool ::= balancePresent(String,String,Balance) [function, total]
  rule balancePresent(O,A,balance(O,A,_)) => true
  rule balancePresent(_,_,_) => false [owise]
  syntax Bool ::= stageShape(Intent,State) [function, total]
  // Distinct authenticated rows may support signed endpoint aliases. Presence
  // belongs to Stage; a valid direct alias is diagnosed later at Intent.
  rule stageShape(intent(_,_,_,S,A,_,_,_,_,R,F,_,_,_,transfer(_,_),_) #as I,
                  state(balance(S,A,_) #as B0,B1,B2,allowance(S,A,_,_),
                        noObligation(),_,_,_,_))
    => (balancePresent(R,A,B0) orBool balancePresent(R,A,B1) orBool balancePresent(R,A,B2))
       andBool (balancePresent(F,A,B0) orBool balancePresent(F,A,B1) orBool balancePresent(F,A,B2))
       andBool (notBool distinct(I) orBool (balancePresent(R,A,B1) andBool balancePresent(F,A,B2)))
  rule stageShape(intent(_,_,_,S,A,_,_,_,_,_,_,_,_,_,repay(ID,S,_,_,_,_),_),
                  state(balance(S,A,_),balance(C,A,_),noBalance(),allowance(S,A,_,_),
                        obligation(ID,S,C,A,_,_,_,"Outstanding"),_,_,_,_))
    => S =/=String C
  rule stageShape(_,_) => false [owise]

  rule <k> s0Intent(submit(I,F,_,_,_,Round) #as R) => s0Effect(R) </k>
    requires matchingScope(I,F) andBool inRound(I,Round) andBool distinct(I)
             andBool intentAmountOK(I)
  rule <k> s0Intent(submit(I,F,_,_,_,_)) => s0Stop("intent","S0_INTENT_SCOPE",1) </k>
    requires notBool matchingScope(I,F)
  rule <k> s0Intent(submit(I,F,_,_,_,Round)) => s0Stop("intent","S0_INTENT_SCOPE",1) </k>
    requires matchingScope(I,F) andBool (notBool inRound(I,Round) orBool notBool intentAmountOK(I))
  rule <k> s0Intent(submit(I,F,_,_,_,Round)) => s0Stop("intent","S0_INTENT_ALIAS",1) </k>
    requires matchingScope(I,F) andBool inRound(I,Round) andBool intentAmountOK(I)
             andBool notBool distinct(I)
  rule <k> s0Intent(_) => s0Stop("intent","S0_INTENT_SCOPE",1) </k> [owise]
  syntax Bool ::= inRound(Intent,Int) [function, total] | distinct(Intent) [function, total]
                | intentAmountOK(Intent) [function, total]
  rule inRound(intent(_,_,_,_,_,_,_,Lo,Hi,_,_,_,_,_,_,_),R)
    => uint(Lo) andBool uint(Hi) andBool Lo <=Int R andBool R <=Int Hi
  rule distinct(intent(_,_,_,S,_,_,_,_,_,R,F,_,_,_,transfer(_,_),_))
    => S =/=String R andBool S =/=String F andBool R =/=String F
  rule distinct(intent(_,_,_,_,_,_,_,_,_,_,_,_,_,_,repay(_,_,_,_,_,_),_)) => true
  rule intentAmountOK(intent(_,_,_,_,_,_,_,_,_,_,_,G,FC,NF,A,_))
    => actionIntentOK(A,G,FC,NF)

  rule <k> s0Effect(submit(I,F,S,E,_,_) #as R) => s0Authority(R) </k>
    requires effectRange(I,S) andBool E ==K expected(I,F,S)
  rule <k> s0Effect(submit(I,_,S,_,_,_)) => s0Stop("effect","S0_EFFECT_RANGE",1) </k>
    requires notBool effectRange(I,S)
  rule <k> s0Effect(_) => s0Stop("effect","S0_EFFECT_MISMATCH",1) </k> [owise]
  syntax Effects ::= expected(Intent,Fill,State) [function]
  rule expected(intent(_,_,_,_,_,_,_,_,_,_,_,_,_,_,transfer(V,F),_) #as I,Fill,_)
    => transferEffects(I,V,F,postOf(Fill))
  rule expected(intent(_,_,_,_,_,_,_,_,_,_,_,_,_,_,repay(_,_,N,_,_,_),_) #as I,Fill,S)
    => repayEffects(I,debtOf(S),N,postOf(Fill))
  syntax Head ::= postOf(Fill) [function]
  rule postOf(fill(_,_,_,_,_,_,_,_,_,_,_,Post)) => Post
  syntax Bool ::= effectRange(Intent,State) [function, total]
  rule effectRange(intent(_,_,_,S,A,_,_,_,_,R,F,_,_,_,transfer(V,FEE),_),
                   state(balance(S,A,OB),balance(R,A,RB),balance(F,A,FB),_,_,_,_,_,_))
    => V +Int FEE <=Int maxU() andBool OB >=Int V +Int FEE
       andBool RB +Int V <=Int maxU() andBool FB +Int FEE <=Int maxU()
  rule effectRange(intent(_,_,_,_,A,_,_,_,_,_,_,_,_,_,repay(ID,Payer,N,1,0,"none"),_),
                   state(balance(Payer,A,OB),balance(C,A,CB),noBalance(),_,
                         obligation(ID,_,C,A,P,AC,O,"Outstanding"),_,_,_,_))
    => N <=Int O andBool OB >=Int N andBool CB +Int N <=Int maxU()
       andBool P +Int AC ==Int O
  rule effectRange(_,_) => false [owise]

  rule <k> s0Authority(submit(I,_,S,_,_,_) #as R) => s0History(R) </k>
    requires authorityOK(I,S)
  rule <k> s0Authority(_) => s0Stop("authority","S0_AUTH_SCOPE",1) </k> [owise]
  syntax Bool ::= authorityOK(Intent,State) [function, total] | premiseOK(Premise,Intent,State,Int,Outcome) [function, total]
  rule authorityOK(intent(_,_,_,S,A,_,_,_,_,_,_,_,_,_,transfer(V,F),_),
                   state(balance(S,A,_),_,_,allowance(S,A,AR,AS),_,_,_,WR,WS))
    => AR >=Int V +Int F andBool AS +Int V +Int F <=Int maxU()
       andBool WR >=Int 1 andBool WS +Int 1 <=Int maxU()
  rule authorityOK(intent(_,_,_,S,A,_,_,_,_,_,_,_,_,_,repay(_,S,N,_,_,_),_),
                   state(balance(S,A,_),_,_,allowance(S,A,AR,AS),
                         obligation(_,S,_,A,_,_,_,_),_,_,WR,WS))
    => AR >=Int N andBool AS +Int N <=Int maxU()
       andBool WR >=Int 1 andBool WS +Int 1 <=Int maxU()
  rule authorityOK(_,_) => false [owise]
  rule premiseOK(authenticated(I,Pre,Round,_,Requested),I,Pre,Round,Requested) => true
  rule premiseOK(_,_,_,_,_) => false [owise]

  rule <k> s0History(submit(I,F,S,_,_,_) #as R) => s0Failure(R) </k>
    <external> X </external>
    requires historyOK(I,S) andBool successorOK(X,I,F)
  rule <k> s0History(submit(I,_,S,_,_,_)) => s0Stop("history","S0_HISTORY_STALE",1) </k>
    requires notBool headOK(I,S)
  rule <k> s0History(submit(I,_,S,_,_,_)) => s0Stop("history","S0_HISTORY_REPLAY",1) </k>
    requires headOK(I,S) andBool notBool historyOK(I,S)
  rule <k> s0History(_) => s0Stop("history","S0_HISTORY_SUCCESSOR",1) </k> [owise]
  syntax Bool ::= headOK(Intent,State) [function, total] | historyOK(Intent,State) [function, total]
                 | successorOK(Premise,Intent,Fill) [function, total]
  rule headOK(intent(_,_,_,_,_,_,H,_,_,_,_,_,_,_,_,_),state(_,_,_,_,_,H,_,_,_)) => true
  rule headOK(_,_) => false [owise]
  rule historyOK(intent(_,_,D,S,_,N,_,_,_,_,_,_,_,_,_,_) #as I,
                 state(_,_,_,_,_,_,Rs,_,_) #as St)
    => headOK(I,St) andBool notBool hasReplay(replayKey(D,S,N),Rs)
  rule successorOK(authenticated(I,_,_,Post,_),I,Fill)
    => Post ==K postOf(Fill) andBool notBool Post ==K signedHead(I)
  rule successorOK(_,_,_) => false [owise]
  syntax Head ::= signedHead(Intent) [function]
  rule signedHead(intent(_,_,_,_,_,_,H,_,_,_,_,_,_,_,_,_)) => H

  rule <k> s0Failure(submit(_,_,_,_,outcome(terminalSuccess(),noEffects(),noDuties()),_) #as R) => s0Commit(R) </k>
  rule <k> s0Failure(_) => s0Stop("failure","S0_FAILURE_UNSUPPORTED",1) </k> [owise]

  rule <k> s0Commit(submit(intent(_,_,D,S,A,N,H,_,_,R,F,_,_,_,transfer(V,FEE),_),Fill,state(balance(S,A,OB),balance(R,A,RB),balance(F,A,FB),
                                     allowance(S,A,AR,AS),O,H,Rs,WR,WS) #as Pre,E,outcome(terminalSuccess(),noEffects(),noDuties()),_))
       => .K </k>
       <out> pending() => accepted(Pre,E,
         state(balance(S,A,OB -Int V -Int FEE),balance(R,A,RB +Int V),
               balance(F,A,FB +Int FEE),allowance(S,A,AR -Int V -Int FEE,AS +Int V +Int FEE),
               O,postOf(Fill),used(replayKey(D,S,N),Rs),WR -Int 1,WS +Int 1),
         terminalSuccess(),noDuty(),WR -Int 1) </out>
  rule <k> s0Commit(submit(intent(_,_,D,S,A,N,H,_,_,_,_,_,_,_,repay(ID,S,PAY,1,0,"none"),_),Fill,state(balance(S,A,OB),balance(C,A,CB),noBalance(),
                                     allowance(S,A,AR,AS),obligation(ID,S,C,A,P,AC,O,"Outstanding"),
                                     H,Rs,WR,WS) #as Pre,E,outcome(terminalSuccess(),noEffects(),noDuties()),_))
       => .K </k>
       <out> pending() => accepted(Pre,E,
         state(balance(S,A,OB -Int PAY),balance(C,A,CB +Int PAY),noBalance(),
               allowance(S,A,AR -Int PAY,AS +Int PAY),
               obligation(ID,S,C,A,P -Int (PAY -Int accruedPaid(PAY,AC)),
                          AC -Int accruedPaid(PAY,AC),O -Int PAY,debtStatus(O -Int PAY)),
               postOf(Fill),used(replayKey(D,S,N),Rs),WR -Int 1,WS +Int 1),
         terminalSuccess(),noDuty(),WR -Int 1) </out>
endmodule
