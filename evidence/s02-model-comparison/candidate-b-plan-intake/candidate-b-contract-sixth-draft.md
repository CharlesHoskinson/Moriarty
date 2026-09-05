# Candidate B sixth-draft literal contract

Planning input only; no implementation or acceptance claim.

## Exact finite declarations

`Principal={Alice,Bob,Mallory}`; `Asset={A,B}`; `Account={AliceA,AliceB,BobA,BobB,MalloryA,MalloryB}`.
`BTime={T0,T1,T2,T100,T101}` with `time(T0)=0,time(T1)=1,time(T2)=2,time(T100)=100,time(T101)=101`.
`BPhase={SA,SB,SD,SC,IF,IS,IC}`. `BNode={AD,BD,SET,RFD,DE,DA,DF,F1,F2,R10,R5,D1,D2}`.
`BTag={ad,bd,set,refund,deadlineEmpty,deadlineAlice,deadlineFunded,fill1,fill2,recover10,recover5,deadline10,deadline5,negativeDeposit}`.
`BInput=NoInput|Deposit(account,owner,depositor,q)|Choice(id,chooser,v)`, with deposit q in {-1,0,1,5,10,20,21} and choice v in {-1,0,1,2}.
`BState={phase,balances:Account->0..20,choices:{settle,fill1,fill2,recover,other}->NoInt|-1|0|1|2,time}`.
`BAttempt={node,before:BState,input,now,coreError:Optional[CoreError],nativeError:Optional[BNativeError]}`; exactly one error field is present on rejection. `BResult={after:BState,effects:List[Transfer],raw:{accepted,error,payments,warnings,state,reductions},attempt:Optional[BAttempt],tag}`.

`phaseNodes={SA:{AD,DE},SB:{BD,DA},SD:{SET,RFD,DF},SC:{},IF:{F1,R10,D1},IS:{F2,R5,D2},IC:{}}`.
All 13 keys exist in each workload table. A node is enabled iff it is in `phaseNodes[state.phase]`, its listed prerequisites are discharged, its input predicate holds, and it is not excluded.

## Literal node table

|node|phase|prereq/input|effects|next/excludes/tag|
|---|---|---|---|---|
|AD|SA|deposit AliceA,Alice,Alice,10|wallet Alice→escrow AliceA 10|SB;{};ad|
|BD|SB|AD; deposit BobB,Bob,Bob,20|wallet Bob→escrow BobB20|SD;{};bd|
|SET|SD|AD,BD; settle Bob1|escrow AliceA→Bob10; escrow BobB→Alice20|SC;{RFD};set|
|RFD|SD|AD; settle Bob0|refund nonzero balances account order|SC;{SET};refund|
|DE|SA|NoInput,T100|[]|SC;{AD};deadlineEmpty|
|DA|SB|AD,NoInput,T100|refund AliceA|SC;{BD};deadlineAlice|
|DF|SD|AD,BD,NoInput,T100|refund AliceA;refund BobB|SC;{SET,RFD};deadlineFunded|
|F1|IF|Choice(fill1,Bob,1)|escrow AliceA→Bob5|IS;{R10};fill1|
|R10|IF|Choice(recover,Alice,1)|refund AliceA10|IC;{F1};recover10|
|F2|IS|F1,Choice(fill2,Bob,1)|escrow AliceA→Bob5|IC;{R5};fill2|
|R5|IS|F1,Choice(recover,Alice,1)|refund AliceA5|IC;{F2};recover5|
|D1|IF|NoInput,T100|refund AliceA10|IC;{F1,R10};deadline10|
|D2|IS|F1,NoInput,T100|refund AliceA5|IC;{F2,R5};deadline5|

Swap uses AD..DF and makes F1..D2 inert; installment uses F1..D2 and makes AD..DF inert. Inert nodes have their displayed row but are absent from phaseNodes; no undefined successor exists.

## Result equations

For every accepted node: `after.time=now`; choices update only for its Choice input; deposit changes only its account; payment/refund subtracts each emitted amount; other fields retain. `raw.state` is this exact after state mapped to the fixed frozen continuation: SA=N6, SB=N5, SD=N4, SC=N0, IF=N4, IS=N2, IC=N0; its program is the literal swap or two-When installment program. AD/BD have reductions0,payments[],warnings[]; SET reductions3/payments[AliceA→Bob10,BobB→Alice20]; F1 reductions1/payments[AliceA→Bob5]; F2 reductions1/payments[AliceA→Bob5]; refund/deadline reductions equal number of nonzero refunded accounts; warnings=[] except the diagnostic below. Exact payment order is table order.

At `now < before.time`, or supplied input at a timeout, raw is `{accepted:false,error:time_before_state|contract_closed,state:before,payments:[],warnings:[],reductions:0}`, effects=[] and one complete BAttempt. Native out-of-domain inputs instead have `nativeError` and no Core result.

`negativeDepositDiagnostic` is a separate literal diagnostic fixture: program phase SA, request `Deposit(AliceA,Alice,Alice,-1)` at T2, before empty SA/T1; raw is `{accepted:false,error:non_positive_deposit,state:before,payments:[],warnings:[],reductions:0}`, effects=[]; it is never in phaseNodes and never a successful template.

## Intake closure

Items 1–6 map respectively to declarations/phaseNodes, BAttempt, diagnostic fixture, split domains, explicit phase map (not successor rank), and result equations above. Remaining limitation: these equations cover the finite listed workloads only; arbitrary Core programs and external authority/bridge controls remain open.
