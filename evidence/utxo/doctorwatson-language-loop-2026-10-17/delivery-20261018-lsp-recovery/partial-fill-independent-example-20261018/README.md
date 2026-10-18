# Two-stage partial fulfillment with cumulative fees

Status: frozen hand-derived expectations and an UNAPPLIED, UNRUN ordinary Rust
test proposal. Source baseline main74a909d69639df8823a527a4d608a7516ccd69a2.
No checkout/Git writes or product/provider/native/model execution occurred.

The example fills one PAY obligation with two accepted payments,4 then6, and
explicit fees1 then2. One false event between them is an observation without an
accepted financial stage. Source counters check cumulative net principal<=10,
fees<=3 and maker gross debit<=13; final ensures filled==10. All snapshots,
state counters, effects, duties, frames and local work are frozen in full.

`two-stage-partial-fill.mori` uses existing Requires, Transfer, Fee, Liability,
Discharge, Next, Await and Ensures. Opening maker30/observer9/taker2/treasury1
becomes maker17/observer9/taker12/treasury4. Full accounting records taker net10,
fees3 to treasury and maker gross13. The selected delivery obligation starts
absent, is created10, becomes6, and ends present-zero. Unrelated duty5 and a
separate present-zero row survive unchanged. Completion retains residual5;
selected-action completion is not a claim that every outstanding duty settled.

The unused declare action registers exact incoming duty identities required by
the documented stage boundary premise. Its statements do not execute and do not
create/discharge the incoming rows in this trace. It remains in the whole
Program/Plan and contributes two literal nodes and its own cost4. One positive
pending site has empty captures because fill has no action inputs or lets;
site1 remains distinct from idle site0.

The root predicate is full source Program/filled Plan equality, then complete
start/false/true report equality, exact actual continuation preservation, and
complete ordered two-segment history. Expected Program61nodes/Plan hashes are
normative local observational values under adopted M01/M03, not observations
or authenticated/native identities. All implementation correspondence is UNRUN.
Work attempted39/2/42 has committed39/39/81; the false attempt2 is separately
visible. Logical units are distinct from the source fees1/2 and all native costs.

`partial_fill_example.rs` is usable unapplied ordinary library test source for
`crates/mori-core/tests/partial_fill_example.rs`; copy the listed fixture members
to `crates/fixtures/two-stage-partial-fill-example` only in a reviewed integration.
`UNAPPLIED-RECIPE.json` states exact destinations. The test compares complete
frozen JSON without generating goldens, passes only actual pending continuations
to resume, and checks the accumulated actual effects. The optional
moriarty-tests/2 manifest exercises a separate existing consumer; neither path
has run. Rust syntax/formatting/build remain unverified; no Rust tool was used.

`UNSUPPORTED.json` maps precise current source/interface limits to the original
EARS. This finite composition adds no signed partial-fill policy, authenticated
stage evidence, once-only consumption, certified lowering, native fee/reserve
or ledger settlement construct. It does not implement those missing predicates
by assertion. No hostile/security judgment or parent-exit closure is supplied.
No CLI/stdout byte comparison is selected. All14historical gap rows and full
language/financial/ACTUS277-18-32-DS01-07/native/target/ledger obligations remain.

`FINANCIAL-AND-WORK.json` includes fourteen complete snapshot literals and all
thirteen ordered effects; `WORK-DETAILS.json` binds every charge to explicit
source statement/ExprIds. `DERIVATION.md` explains their arithmetic. These
histories/counters are not added report wire members. Field names, null/absence,
array order and full preserved rows follow the pinned observation contract.

MANIFEST.json binds every payload except itself and the freeze; FREEZE.sha256
binds every payload including that manifest, excluding itself. Source-only
static consistency and matching hashes never establish an actual passing test.
