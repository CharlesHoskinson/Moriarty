# Independent request and message financial cases

Status: proposed design challenges, not executed Moriarty programs. These thirteen
cases confront the archived RequestFulfill/RequestClaim/MessageRefund gaps with
explicit assets, sources, recipients and surviving obligations. See the JSON for
source hashes and exact given/action/expected records. No new standard is claimed.

The old request record uses a single asset and untyped committed/filled/claimed
quantities. That cannot describe a100-share redemption that delivers130 cash
without an additional conversion relation. A minimal replacement must distinguish
committed input, fulfilled input, refunded input, funded output entitlement and
claimed output, with every counter indexed by the correct asset or share identity.
The request also needs source escrow, settlement sink, output funding source,
controller, beneficiary, and accepted transition/history identity. Exact record
names and bounds belong to the complete reviewed financial contract.

For the explicit escrow-based fixture, the conservation obligations are:

- inputCommitted = inputPending + inputFulfilled + inputRefunded;
- outputAccrued = outputClaimed + outputClaimable;
- the designated output escrow backs outputClaimable in the same cash asset;
- gross authorization consumption is monotone across fulfillment, claims and refunds.

These are not interchangeable units. Conversion only connects inputFulfilled to
outputAccrued under a separately selected rate/rounding/fee rule. Escrow backing
here is an explicit fixture design, not a claim that every asynchronous financial
protocol uses pre-funded escrow. Other backed or credit-bearing designs need an
explicit liability and funding model; they cannot inherit an escrow assertion.

After commit100 shares, fulfill40 for52 cash, claim20, and cancel the remaining60
shares, the holder has received60 shares and20 cash, with32 cash still owed.
The fulfilled40 shares remain at the named settlement sink. This case deliberately
challenges the old rule that any claimed amount prevents all cancellation: the
new design must state whether cancellation of *pending input* is authorized,
separately from whether an already funded output claim survives. A combined
status enum cannot silently delete either dimension. Deadline expiry and a
token burn likewise cannot stand in for completed cash redemption (AT03).

The examples presume separately checked authorization, transfer policy, current
predecessor, source ownership and finite resources. RQ09 and MSG03 isolate the
currentness predicate: two locally valid candidates cannot both consume h0.
They are not a proof of remote absence/finality or a cross-chain atomic commit.

`independent-request-cases-01.py` verifies only selected hand-derived arithmetic,
conservation and counterexample inequalities. It contains no implementation of
Moriarty request transitions. Integrating these cases requires the full38-operation
contract, type/encoding binding, exact diagnostics, an evaluator/K counterpart,
and eventual independent financial/Preview evidence. No existing acceptance
record or frozen expression file is changed by these cases.
