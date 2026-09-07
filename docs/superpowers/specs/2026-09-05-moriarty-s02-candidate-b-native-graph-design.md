# Candidate B native obligation-graph design

Date: 2026-09-05 UTC. Classification: proposed experimental design, not an
implemented model or adopted architecture. Source boundary: S02 branch at
`7a7bf5d`; frozen Core/swap and the common design remain unchanged.

This replaces the sixth draft as a review input, not as evidence of completion.
It addresses all six findings in the fifth/sixth-draft intake. No B implementation
may start from this document until its concrete types, native rules and mapping
boundary have received review and an explicit local adoption disposition.

## Requirement, alternatives and model boundary

XML v1.3 S02 requires four genuinely different alternatives. B makes typed
obligations, dependency discharge, alternatives and state/effect constraints
primary. Its evaluator must not invoke the A evaluator or frozen Python to obtain
an answer. The common signature/nonce/parent/commit implementation stays shared.

Considered: (1) a phase table with Core-shaped states, (2) an obligation graph with
native discharge state, and (3) executing A then relabeling its answer. Propose
(2). A phase table alone obscures the intended dependency semantics; (3) is not
an independent alternative. A library of literal graphs is an input to B, not
the entirety of B's native graph language. A separate observation map exposes
the frozen-Core meaning of the two comparison workloads and diagnostic library.

Alice, Bob and Mallory coordinate through shared state, not messages. Use plain
Quint, not Choreo. Agreement evaluation is a pure atomic proposal computation;
signing, proposal verification and commitment remain distinct shared-envelope
transitions. The graph itself grants no cryptographic authority. Physical clocks
are the existing finite values; no scheduler, network or cryptographic proof is
invented. The complete graph, request and native result must later be equality-bound.

## Exact carrier sketch

Native modules may import neutral `effects` and `observations` data types, but
not candidate_a_core, candidate_a_projection or a reference interpreter.

```quint
type BTime = BTime0 | BTime1 | BTime2 | BTime100 | BTime101
type BChoiceId = BSettle | BFill1 | BFill2 | BRecover | BOther
type BNodeId = BDepositAlice | BDepositBob | BSettleSwap | BRefundSwap
  | BTimeoutEmpty | BTimeoutAlice | BTimeoutFunded
  | BFirstFill | BSecondFill | BRecoverTen | BRecoverFive
  | BTimeoutTen | BTimeoutFive
type BInput = BNoInput
  | BDepositInput({account: CoreAccount, depositor: Principal, quantity: int})
  | BChoiceInput({id: BChoiceId, chooser: Principal, chosen: int})
type BTrigger = BDepositTrigger({account: CoreAccount, depositor: Principal, quantity: int})
  | BChoiceTrigger({id: BChoiceId, chooser: Principal, lower: int, upper: int})
  | BTimeoutTrigger
type BPayment = {source: CoreAccount, recipient: Principal, quantity: int}
type BTransferRule = BDepositTransfer | BPayTransfers(List[BPayment]) | BRefundAll
type BNode = BInactive | BObligation({requires: Set[BNodeId], excludes: Set[BNodeId],
  trigger: BTrigger, transferRule: BTransferRule})
type BGraph = {deadline: BTime, nodes: BNodeId -> BNode}
type BState = {accounts: CoreAccount -> int, choices: BChoiceId -> OptionalInt,
  minimumTime: BTime, discharged: Set[BNodeId], excluded: Set[BNodeId]}
type BPredecessor = {graph: BGraph, state: BState}
type BRequest = {before: BPredecessor, input: BInput, now: BTime}
type BRejection = BTimeBeforeState | BClosed | BInputRequired | BNoMatchingInput
  | BChoiceOutOfBounds | BNonPositiveDeposit | BInsufficientBalance | BExclusionConflict
type BDiagnostic = BInvalidGraph | BInvalidState | BInputOutsideDomain | BResultOutsideDomain
type BSuccess = {request: BRequest, node: BNodeId, after: BState,
  effects: List[Transfer], payments: List[BPayment]}
type BFailure = {request: BRequest, reason: BRejection}
type BExecution = BAccepted(BSuccess) | BRejected(BFailure)
  | BDiagnosticResult({request: BRequest, reason: BDiagnostic})
type BGraphStatus = BOpen | BFinanciallyComplete | BExhausted | BStranded
```

`OptionalInt` is the imported NoInt/IntValue sum, not an undeclared Option type.
Every rejection/diagnostic retains the entire original graph, state, input and
clock through BRequest. There is no unconditional raw-Core field that must
somehow be absent. The selected node is an actual result, not a trusted node
name supplied by the caller. This intentionally removes the old draft's
caller-selected node parameter: node selection is determined by graph/input/time.

Time values map to 0,1,2,100,101. Input deposit quantities are
{-1,0,1,5,10,20,21}; choices and choice bounds are {-1,0,1,2}. Account keys are all
six CORE_ACCOUNTS and choice keys all five BChoiceIds. Native balances range
0 through 280. Each transfer-rule payment has quantity in {5,10,20}, at most
two ordered payments per obligation; refund order is the full explicit list
Alice/A, Alice/B, Bob/A, Bob/B, Mallory/A, Mallory/B. Derive asset from source.
Deposit-trigger quantities use the deposit-input domain, including nonpositive
diagnostic declarations. Do not saturate an arithmetic result at the bound.

All graph maps have exactly thirteen keys. Native graphs are not restricted to
the library's three literal maps. An obligation's prerequisites must name active
nodes strictly earlier in the fixed BNodeId order above; exclusions name active
nodes other than itself. The deadline is BTime100 for this finite model. A deposit
trigger requires BDepositTransfer; a choice trigger permits BPayTransfers or
BRefundAll; a timeout requires BRefundAll. Validate all references and complete
maps before lookup. Unsupported declarations are BInvalidGraph, not Core errors.

State validity requires admitted graph, complete maps/domains, nonnegative bounded
balances, discharged/excluded subsets of active nodes, disjoint discharged and
excluded sets, prerequisite closure of discharged nodes, and excluded equal to
the union of exclusions of all discharged nodes. It does not assert that every
such state is reachable or corresponds to a frozen agreement. The action harness
starts from literal initial states and tests preservation; it does not initialize
arbitrary successful histories or hide invalid successors in its guard.

For a valid predecessor, define remaining nodes as active minus discharged and
excluded, and the frontier as remaining nodes whose prerequisites are discharged.
`graphStatusB` is total on valid predecessors, with this ordered classification:
nonempty frontier is BOpen; otherwise any positive account balance is BStranded;
otherwise nonempty remaining nodes is BExhausted; otherwise BFinanciallyComplete.
BOpen is structural availability, not a claim that a permitted input or solvent
payment exists. BFinanciallyComplete means zero native escrow and no unresolved
nodes, not that all obligations were discharged (some can be explicitly excluded).
BExhausted records unresolved nodes blocked by exclusions with zero escrow.
BStranded records unrecoverable native escrow in an empty-frontier state, whether
or not unresolved nodes remain. The classification never moves money. BClosed
below means either an empty frontier or supplied input when a frontier timeout
must run instead. It does not imply financial completion or even empty frontier;
graphStatusB, not the rejection code, provides the state classification.

Generic graph admission deliberately does not guarantee non-locking: the admitted
single-AD graph (deposit10, no prerequisites/exclusions, all other nodes inactive)
can reach BStranded after one deposit. Preserve that negative experiment in the
comparison; do not add implicit Core Close/refund behavior or reject the graph to
hide it. A separate library property requires every reachable empty-frontier
state to be BFinanciallyComplete. Universal generic non-locking must not be
claimed; selection must price this restriction or specify a separately reviewed
admission analysis. An excluded-prerequisite graph must also witness BExhausted.

## Literal library graph maps

The abbreviations below expand to the thirteen BNodeIds in carrier order:
AD,BD,SET,RFD,DE,DA,DF,F1,F2,R10,R5,D1,D2. Empty set is `{}`. Every unused key is
explicitly BInactive, never absent. The table gives every active row; all graph
deadlines are BTime100.

| Node | Requires | Excludes on discharge | Trigger | Transfer rule |
| --- | --- | --- | --- | --- |
| AD | {} | {DE} | Deposit Alice/A by Alice, quantity10 | DepositTransfer |
| BD | {AD} | {DA} | Deposit Bob/B by Bob, quantity20 | DepositTransfer |
| SET | {AD,BD} | {RFD,DF} | Choice settle by Bob, bounds1..1 | Pay Alice/A to Bob10, then Bob/B to Alice20 |
| RFD | {AD,BD} | {SET,DF} | Choice settle by Bob, bounds0..0 | RefundAll |
| DE | {} | {AD,BD,SET,RFD,DA,DF} | Timeout | RefundAll |
| DA | {AD} | {BD,SET,RFD,DF} | Timeout | RefundAll |
| DF | {AD,BD} | {SET,RFD} | Timeout | RefundAll |
| F1 | {} | {R10,D1} | Choice fill1 by Bob, bounds1..1 | Pay Alice/A to Bob5 |
| F2 | {F1} | {R5,D2} | Choice fill2 by Bob, bounds1..1 | Pay Alice/A to Bob5 |
| R10 | {} | {F1,F2,R5,D1,D2} | Choice recover by Alice, bounds1..1 | RefundAll |
| R5 | {F1} | {F2,D2} | Choice recover by Alice, bounds1..1 | RefundAll |
| D1 | {} | {F1,F2,R10,R5,D2} | Timeout | RefundAll |
| D2 | {F1} | {F2,R5} | Timeout | RefundAll |

`swapGraphB` has AD through DF active and the six installment rows inactive.
`installmentGraphB` has F1 through D2 active and the seven swap rows inactive.
The negative-deposit diagnostic graph has only AD and DE active: AD declares
Deposit Alice/A by Alice **quantity -1**, excludes DE, and uses DepositTransfer;
DE excludes AD and uses Timeout/RefundAll. Its other eleven keys are BInactive.
Its references do not reuse the swap DE exclusions pointing to inactive nodes.

Initial swap accounts are all zero, minimumTime0. The separately declared negative
diagnostic starts with zero accounts and minimumTime1; initial installment
accounts have Alice/A10 and others zero, minimumTime2. All choices are absent,
discharged/excluded are empty. Wallet balances live outside BState in the common
ledger (swap initial wallets Alice/A10, Bob/B20; installment prefunded escrow10).
The swap semantic clock is not the nonzero clock of the authority fixtures.

## Total native evaluation rule

`evaluateB(request): BExecution` uses this priority, with no A/Python call:

1. Invalid graph, then invalid state, then input-domain failure produce the
   corresponding BDiagnosticResult with the original request. No Core result.
2. now < before.minimumTime yields BRejected(BTimeBeforeState), before any
   terminal/input/timeout decision. It changes no native state or output.
3. Compute the frontier: active, not discharged/excluded, all prerequisites
   discharged. Empty frontier yields BClosed for any input.
4. If now >= deadline, first find frontier timeout obligations. If there are
   none, return BNoMatchingInput for both BNoInput and supplied input. Otherwise
   select the first timeout in fixed node order: supplied input yields BClosed
   and complete rollback; BNoInput discharges that timeout. The no-timeout branch
   takes priority over the supplied-input branch. The literal libraries have exactly
   one timeout in every nonterminal frontier; assert this separately.
5. Before deadline, BNoInput yields BInputRequired. With supplied input, scan
   non-timeout frontier obligations in the fixed order. A deposit matches full
   account/depositor/quantity. A matched nonpositive quantity yields
   BNonPositiveDeposit; a mismatched quantity does not. Choice matching uses
   id/chooser and then inclusive bounds. Preserve an identity-matched bounds
   failure while scanning; a later matching row can still win. Thus settle0
   selects RFD despite failing SET's1..1 bound. If no row wins, return
   BChoiceOutOfBounds when any identity matched, otherwise BNoMatchingInput.
6. Before discharge, reject BExclusionConflict if the row would exclude an
   already discharged node. Compute DepositTransfer from the matched actual
   input; compute ordered PayTransfers from the declared payments; RefundAll
   walks the six actual account balances in canonical order and emits only
   positive owner refunds. Check each escrow debit prefix; insufficient native
   balance gives BInsufficientBalance, not a fake partial payment or success.
   Wallet solvency remains the shared envelope's separate canApply check.
7. On acceptance, update accounts by those actual escrow credits/debits, store
   the actual choice when present, set minimumTime=now, add the selected node to
   discharged and union its exclusions. Preserve every other field. Payment
   output excludes wallet deposits and retains the actual ordered escrow payments.
   Check the complete computed successor with validBState, including every
   structural invariant and finite bound. Failure returns BResultOutsideDomain
   with the original request, not saturation or a hidden action guard. A failed
   successor check must fail the harness safety predicate, even though the pure
   evaluator retains a diagnostic instead of installing the invalid state.

Every BRejected has unchanged native predecessor, empty committed outputs and
no discharged obligation; the failure record is still observable. Action wrappers
retain it as a distinct attempt. They do not silently omit failed requests from
coverage or label a no-op as an executed payment.

## Frozen-Core observation map: explicit, narrower than the native language

The native graph language is distinct from its comparison library. Do not claim
arbitrary obligation graphs have frozen Core meaning. The observation module may
use A's finite **data types and literal programs only**, never A computation or
extraction, to encode the same full neutral continuation expected by the existing
Python decoder. Native evaluation remains independent of that module.

Define `BMappingFailure = BUnexpectedNativeOutcome | BUnexpectedSuccessor` and
`BMappingResult = BMappedCore(CoreResultObservation[AContinuation])
| BNoLibraryMapping | BMappingFailed({execution: BExecution, reason: BMappingFailure})`.
The failure variant retains the entire native execution. An unexpected rejection
or diagnostic for a mapped request is BUnexpectedNativeOutcome; a success whose
complete successor is outside the canonical mapping domain (accounts, choices,
minimumTime, discharged/excluded sets and mapped continuation) is
BUnexpectedSuccessor. A wrong in-domain successor must still fail the independent
correspondence comparison; mapping admission alone never certifies equivalence.
No missing map becomes an invented Core error or a passing
correspondence case. The comparison harness requires BMappedCore for every
mandatory library record. Unmapped generic-native cases are separately counted;
they cannot satisfy frozen-library coverage or justify architecture selection.

The mapping domain is exactly each literal graph plus these canonical discharged
sets and corresponding states, not every structurally well-formed B state:

- Swap: {}, {AD}, {AD,BD}, {DE}, {AD,DA}, {AD,BD,SET}, {AD,BD,RFD}, {AD,BD,DF}.
  Nonterminal account maps are respectively zero, Alice/A10, Alice/A10+Bob/B20;
  terminal account maps are zero. Only SET records settle=1 and RFD settle=0;
  all other choice entries remain absent. Initial minimumTime0; open funded
  states and ordinary terminals have minimumTime0,1 or2; timeout terminals100 or101.
- Installment: {}, {F1}, {F1,F2}, {R10}, {F1,R5}, {D1}, {F1,D2}.
  Initial Alice/A10, residual Alice/A5, terminals zero. F1/F2 record their actual
  chosen1, R10/R5 record recover1, other choices absent. Non-timeout minimumTime2;
  timeout terminals100 or101. Excluded sets always equal the native row union.
- Negative diagnostic: {} at minimumTime1 and zero accounts/absent choices, or
  {DE} at minimumTime100/101 with the same zero balances/choices.

The correspondence checker must reject a producer that falsely labels an
out-of-domain predecessor as mapped. These domain restrictions are published
before implementation and must be visible in the A–D scope comparison. They are
not evidence that native generic graph semantics preserve all agreement programs.

Map swap {}, {AD}, {AD,BD}, terminal to full canonicalSwap continuations
N6,N5,N4,N0. Map installment {}, {F1}, terminal to full installmentProgram
N4,N2,N0. Diagnostic program is a separate literal full sixteen-node AProgram:
rootN6, all other nodes CloseA; N6 is When with Deposit Alice/A by Alice amount -1
to N0, timeout100 to N0. It is not canonicalSwap with a merely negative request.
Map every account, all five optional choices, and minimumTime exactly.

| Accepted selected node | Core payments | Core reductions |
| --- | --- | --- |
| AD, BD | empty (wallet deposit is a transfer effect, not a Core payment) | 0 |
| SET | actual two declared ordered payments | 3 |
| RFD | actual two owner refunds, Alice/A then Bob/B | 3 |
| DE | empty | 1 |
| DA | actual Alice/A refund | 2 |
| DF | actual Alice/A then Bob/B refunds | 3 |
| F1, F2 | actual five-unit payment to Bob | 1 |
| R10, R5 | actual ten/five-unit owner refund | 1 |
| D1, D2 | actual ten/five-unit owner refund | 2 |

All mapped accepted results have NoCoreError and warnings[]. Use actual native
payment data and actual successor state; the table supplies only the explicit
library reduction-count correspondence. Failed executions in the mapping domain
map BTimeBeforeState/BClosed/BInputRequired/BNoMatchingInput/BChoiceOutOfBounds/
BNonPositiveDeposit to their exact frozen six error codes with original mapped
state, payments/warnings[], reductions0 and no effects. BInsufficientBalance or
BExclusionConflict inside this canonical library domain is an unexpected mapping
failure to investigate, not BNoLibraryMapping and not a synthesized Core error.
Malformed/domain diagnostics are never projected as successful/rejected Core runs.

At time100/101, supplied input must preserve the **original** minimumTime and all
choices/accounts/dependency state. Do not retain a timeout discharge from speculative
reduction before reporting BClosed. The independent checker executes frozen Python
from the original graph-library mapping and compares all result fields/effects.

## Acceptance obligations and implementation split

First unit: concrete types, generic graph/state validation, the three literal
graphs and runnable initialization tests. Include a well-formed nonlibrary graph
to show the native validator is not a three-template equality test. Typecheck the
carrier sketch before logic; retain exact behavioral RED/GREEN source closures.

Second unit: the pure native evaluator and deterministic tests for every accepted
library node, full rollback and all six mapped errors. Use an actual declared -1
diagnostic to obtain nonpositive_deposit, and canonical AD with supplied -1 to
obtain no_matching_input. Test both100 and101 and choice scan-order/bounds cases.
Keep native insufficient-balance/exclusion/domain controls distinct from mapped
Core errors. No outcome is supplied by a fixture flag or reference interpreter.
Include actual time0 swap funding and settlement, plus the generic deposit-only
graph at deadline100/101 with no frontier timeout and both input forms. Execute
at least one nonlibrary discharge trace, asserting actual effects, successor,
graph status and unmapped observation; validator acceptance alone is insufficient.

Third unit: real swap and installment stateful traces, thin guarded actions,
positive discharge/terminal witnesses and invariants for account/effect arithmetic,
dependency/exclusion preservation and graph-status classification. Witness the
single-deposit BStranded and excluded-prerequisite BExhausted outcomes separately
from library financial completion; never count either as a successful terminal.
Do
not use a library-shaped validity predicate to suppress a faulty successor;
record native diagnostics and let the safety/coverage predicate fail.

Fourth unit: actual native-record exports, independent complete-inventory frozen
Python checking of the declared mapping domain and retained semantic mutations.
Test graph/program substitution, wrong reduction costs, reordered payments,
absent-versus-zero choices, rollback time, lost deposit effects and concealed
unmapped records. Record unmapped native coverage separately, never as agreement.

Fifth unit: adapt native records to the unchanged common authority boundary with
full-plan fidelity, both signing profiles, cancellation identity, fill/cancel
races and newly signed nonce-one recovery. Full B comparison additionally requires
the requested model-checking and Council evidence. Those are not supplied by a
reviewed sketch or library correspondence alone. No S02 selection is made here.

## Review questions to resolve before adoption

1. Does the finite generic graph grammar plus explicit narrower library mapping
   satisfy the intended intent-Core experiment without disguising library-only D?
2. Are the native rejection/domain rules total, and are any admitted library
   requests wrongly diverted to unmapped or native-only errors?
3. Does the exact table, especially timeout exclusions and choice scanning, admit
   all baseline positive/rejection traces while preserving both recovery amounts?

These are bounded design-review obligations under existing delegated authority,
not requests to reopen the completed common-design vote or Foreman work.
