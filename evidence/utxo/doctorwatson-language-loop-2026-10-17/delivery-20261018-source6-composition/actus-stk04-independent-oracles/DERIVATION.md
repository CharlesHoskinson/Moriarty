# STK04 derivation from operands and rules

No original normative result values or historical authored expected/test bodies
were read. The exact source and initial scenario are author-exposed proposed
inputs, not independent ACTUS requirements. The selected public consumer is
parse/validate/simulate in the ordinary Core/1 financial_pair target; no target
body or product operation was inspected/executed in this derivation.

## Normative payoff and date arithmetic

The input terms specify STK/RPA, currencyEUR, purchase2013-01-01T00:00:00 at1200,
termination2016-12-31T23:59:59 at3000, status2012-12-30T00:00:00, notional1000,
and no settlementCurrency. Normative R(RPA)=+1. The settlement FX multiplier
is1 when settlementCurrency is absent. STK PRD is X*R*(-1)*priceAtPurchaseDate:
1*(+1)*(-1)*1200=-1200. STK TD is X*R*priceAtTerminationDate:
1*(+1)*3000=+3000. No notional multiplier occurs in these STK equations.
The same-as-PAM date rules place PRD/TD at those literal term dates, and both
STK transitions assign statusDate=t. The selected Core input uses finite UInt
YYYYMMDDhhmmss literals20130101000000 and20161231235959. This supplies no general
calendar, timezone, EOM or authenticated-clock implementation.

Only cashflow semantic fields eventDate/eventType/payoff/currency are asserted
against the reference concepts. The STK state table exposes Prf and Sd; it does
not establish the proposed extra Core fields' reference-result mapping. Term
notional1000 stays retained input; Core notionalPrincipal0/nir0/accruedInterest0
are source-declared slots, not derived ACTUS result values. All three present
reference keys remain explicitly unsupported. No dividend/schedule/performance
or equity/quantity delivery completeness is claimed.

## Complete Core purchase trace

Empty scenario.state selects all six source initializers: purchasedfalse,
terminatedfalse,statusTime20121230000000,notionalPrincipal0,
nominalInterestRate0,accruedInterest0. Inputs are not state. Exact EUR asset
identity is (actus-reference,fixture-only,EUR,scale0), a local source identifier.
Canonical balances are counterparty3000,investor2000; duties[].

Seven statements, indices0..6, produce the following literal results:
0 requires not purchased: Booltrue; unchanged full snapshot; delta[].
1 requires not terminated: Booltrue; unchanged full snapshot; delta[].
2 requires eventTime==purchaseTime: Booltrue; unchanged snapshot; delta[].
3 Transfer1200 investor→counterparty: UInt1200; balances4200/800; one Transfer.
4 NextstatusTime: UInt20130101000000; only that state slot changes; one State.
5 Nextpurchased: Booltrue; only that state slot changes; one State(false→true).
6 EmitPRD: UInt20130101000000; unchanged final snapshot; one Emit.
Each complete Step snapshot and each complete ordered delta/prefix is literal
in oracle.json. Overall effects are Transfer,State(statusTime),State(purchased),
Emit(PRD) in that exact order. No fee or liability effect is invented.

## Complete actual-boundary successor trace

Terminate begins from actual purchase.post, retaining all six state slots,
all balance rows and empty duties, while replacing only the eventTime input
with20161231235959. Its seven source statements produce:
0 requires purchased: Booltrue; unchanged full snapshot; delta[].
1 requires not terminated: Booltrue; unchanged snapshot; delta[].
2 requires eventTime==terminationTime: Booltrue; unchanged snapshot; delta[].
3 Transfer3000 counterparty→investor: UInt3000; balances1200/3800; one Transfer.
4 NextstatusTime: UInt20161231235959; one State(oldpurchaseTime→terminationTime).
5 Nextterminated: Booltrue; one State(false→true); purchased remains true.
6 EmitTD: UInt20161231235959; unchanged snapshot; one Emit.
The same other state zeros persist in every snapshot, and duties stay[].

The complete8-effect pair history is the four purchase deltas followed by the
four terminate deltas. Gross investor debit1200 and credit3000 give net+1800;
counterparty grossdebit3000 and credit1200 give net-1800. Opening and closing
balance sums are each5000 as fixed fixture arithmetic. Source-local fee effects
and fee amount are0, while external/native fees are unasserted. No absent duty
is turned into a present-zero tombstone; zero-valued state slots are not duties.

## Work and representation limits

The public conceptual cost schedule is dispatch1 plus uncached expression-node
visits, resetting the cache for each statement. If each source operator and
atom maps to one public node with no inserted/folded/desugared nodes, purchase
node counts2,2,3,1,1,1,1 give units3,3,4,2,2,2,2 totaling18. Terminate counts
1,2,3,1,1,1,1 give units2,3,4,2,2,2,2 totaling17; pair35. These are independently
frozen conceptual arithmetic with an explicit missing mapping premise. No
metered invocation or CostReport is part of the selected input, so runtime work
is UNASSERTED. No cache/meter/product call was made to establish these values.

Public DTOs define ExprId fields but do not specify this Core/1 parser's exact
source-wide node allocation. Therefore statement metadata/kinds are retained,
but numeric predicate/amount/value references are unasserted. No invented
numeric placeholder, full wire Program/Execution or exact stdout expectation is
provided. A new independent representation premise/freeze would be required
before those fields could be added. Prior producer behavior cannot fill them.

Static expected-data checks are not behavior evidence. All277 fixtures/18types/
32dispositions/DS01–07, all fields and every full-family/parent obligation remain.
