# Independent positive derivation

The target connected producer/publication does not exist in inspected source.
No evaluator/parser/product module was imported or executed. Existing type,
parser allocation, stage plan/work and serializer definitions were read to bind
the literal data contract; monetary results were derived by integer arithmetic
from retained-payment.mori and scenario.json. No prior EXPECTED output file was
read to create this first freeze. Existing ledger/scenario facts are disclosed.
The Python preparation file only writes supplied literals/copies and hashes
manual data; it contains no transition interpreter.

State initial literals77/true are fields, not expression rows. Amount literals
30,2,30; guard name ready; amounts10,1,10; unused duty declarations0,0 give
ExprIds0..8 in source allocation order. The pay action has seven statements;
its await is source index3/site1, with binder ready:Bool and no earlier input/let
captures. declareDuties is retained in the Program but is not invoked.

Each monetary/discharge statement has one literal expression: dispatch1+node1=2.
Start executes three such statements and reaches await with entry charge1:
2+2+2+1=7. False resume evaluates dispatch1+ready-name1=2, changes no state,
returns no segment and keeps committed7. True resume evaluates the same guard2,
then three statements2 each: attempted8, committed7+8=15. The unused two-duty
action has cost4. Total attempted over this three-invocation sequence is17;
committed15 excludes the false guard's2. No B clock is inferred.

Start transfers30: reserve100→70, beneficiary0→30. Fee2: reserve70→68,
fee recipient0→2. Discharge30: pay liability40→10. The ordered effects are
Transfer, Fee, Liability40→10; the three step snapshots reflect each intermediate
state in exactly that order. First encounter accounting rows are reserve,
beneficiary, fee_recipient, with gross debit32/0/0 and gross receipts0/30/2.
Fees are already included in gross; there is no second fee addition.

At false release, all returned continuation fields, boundary values, captures[],
positive residual rows and stored cap null stay unchanged. Its segment is None,
not an empty successful execution. True release produces an Await(true) step
at original index3 with unchanged boundary, then transfer10, fee1 and discharge10
at indices4/5/6. Reserve68→58→57; beneficiary30→40; fee recipient2→3; pay liability
10→0. The zero liability remains in the full snapshot. Only auditDuty5 remains
in the positive residual list; its identity and all AUX/unaffected entries stay.
Second-segment accounting is11gross debit,10recipient receipts,1fee receipt.
Across both successful segments the financial totals are43/40/3; the adapter
itself still reports per-segment accounting, not an invented prefix aggregate.

Canonical identities use the manually transcribed PROGRAM.json and PLAN.json,
not parsed target output: recursively sorted object keys, compact UTF8 JSON,
SHA256 as documented by stage/digest.rs and intent::canonical. Arrays retain
source order. This is independent data hashing; no Rust compiler/parser/evaluator
is run. Result, enum, UInt and Value representations follow the actual public
serde declarations. Later readback against historical independent goldens, if
performed, is recorded AFTER this freeze and never changes these values silently.
