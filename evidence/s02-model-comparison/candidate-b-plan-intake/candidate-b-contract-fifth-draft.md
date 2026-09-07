# Candidate B fifth-draft finite contract

Planning only. This resolves the fourth-draft intake; it authorizes neither
implementation, model checking, correspondence, selection, nor Council action.

## Closed finite carriers

`BPhase = SwapAlice | SwapBob | SwapDecision | SwapClosed | InstallmentFirst |
InstallmentSecond | InstallmentClosed`. `BNode` has exactly the thirteen names in
the fourth draft. A node has `{phase, rank, prerequisites, excludes, templates,
successPhase, tag}`; `successPhase` is a phase, never a node. The graph additionally
has `phaseNodes: BPhase -> Set[BNode]`. Thus an unused node has a valid successor
(`SwapClosed` or `InstallmentClosed`) and is disabled because it is outside every
admitted phase-node set.

`BInput = NoInput | Deposit({account, owner, depositor, quantity}) |
Choice({id, chooser, value})`, where `owner == account.owner` is checked explicitly.
Deposit values are `{-1,0,1,5,10,20,21}`; choice values are `{-1,0,1,2}`. Times are
`T0,T1,T2,T100,T101`; only T0 initializes, and transition now is T1/T2/T100/T101.
Balances are six accounts in 0..20; choices are five optional values; event and attempt
history are bounded to 16 entries.

`BRejectedAttempt = {request:{node,before,input,now}, coreError: Optional[CoreError],
nativeError: Optional[BNativeError]}` has exactly one nonempty error field. Rejections
retain all projected state fields. A backward clock has `CoreErrorCode(time_before_state)`;
a matched negative deposit has `CoreErrorCode(non_positive_deposit)` and no transfer;
outside finite domain is native only.

## Literal total tables

All omitted node-table fields are `prerequisites:{}`, `excludes:{}`, `templates:[]`,
`successPhase:Closed`, and an explicit non-enabled phase membership.

| Node | Phase/rank | prerequisites; input | templates | successor; excludes; tag |
|---|---|---|---|---|
| AliceDeposit | SwapAlice/1 | {}; Alice/A owner Alice, depositor Alice,10 | wallet Alice→escrow Alice/A 10 | SwapBob; {}; swap-A-deposit |
| BobDeposit | SwapBob/2 | AliceDeposit; Bob/B owner Bob,depositor Bob,20 | wallet Bob→escrow Bob/B 20 | SwapDecision; {}; swap-B-deposit |
| Settle | SwapDecision/3 | AliceDeposit,BobDeposit; settle Bob=1 | escrow A→Bob10, escrow B→Alice20 | SwapClosed; Refund; swap-settle |
| Refund | SwapDecision/3 | AliceDeposit; settle Bob=0 | refunds current balances canonical account order | SwapClosed; Settle; swap-refund |
| DeadlineEmpty | SwapAlice/1 | {}; no input at T100 | [] | SwapClosed; AliceDeposit; timeout-empty |
| DeadlineAlice | SwapBob/2 | AliceDeposit; no input T100 | refund Alice/A current | SwapClosed; BobDeposit; timeout-A |
| DeadlineFunded | SwapDecision/3 | AliceDeposit,BobDeposit; no input T100 | refund Alice/A then Bob/B current | SwapClosed; Settle,Refund; timeout-funded |
| FillOne | InstallmentFirst/1 | {}; fill1 Bob=1 | escrow Alice/A→Bob5 | InstallmentSecond; RecoverTen; fill1 |
| RecoverTen | InstallmentFirst/1 | {}; recover Alice=1 | refund Alice/A current 10 | InstallmentClosed; FillOne; recover10 |
| FillTwo | InstallmentSecond/2 | FillOne; fill2 Bob=1 | escrow Alice/A→Bob5 | InstallmentClosed; RecoverFive; fill2 |
| RecoverFive | InstallmentSecond/2 | FillOne; recover Alice=1 | refund Alice/A current 5 | InstallmentClosed; FillTwo; recover5 |
| DeadlineFirst | InstallmentFirst/1 | {}; no input T100 | refund Alice/A current 10 | InstallmentClosed; FillOne,RecoverTen; timeout10 |
| DeadlineSecond | InstallmentSecond/2 | FillOne; no input T100 | refund Alice/A current 5 | InstallmentClosed; FillTwo,RecoverFive; timeout5 |

Ranks increase only across discharged prerequisites; phase transitions follow the listed
successors, so no successor-node rank assertion is required. At deadline, supplied input
creates a retained Core rejection, never a deadline discharge.

## Exact observation rule

Each accepted transition records `{accepted:true,error:NoCoreError,state,warnings,payments,
reductions}`. State uses post balances, all five choices (absent stays NoInt), exact next
phase continuation, and `minimumTime=now`. Deposit reductions=0/payments=[]; a payment
or refund uses the frozen Core reduction count and warnings (zero/negative pay warnings,
partial warning requested/paid); rejection records original state, empty lists, reductions
0 and exact Core error. Transfers are ordered pre-payment, accepted deposit, post-payment,
matching the separately constructed frozen vector. This fixed table is elaborated by an
independent checker; B contains no Core interpreter.

Recovery is agreement-legal before lifecycle cancellation. Authority, nonce, signatures,
evidence, and bridge controls remain external. The only unresolved design limitation is
fixed-corpus rather than arbitrary-Core elaboration; bridge controls remain unresolved,
not N/A.
