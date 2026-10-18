# Complete ordinary positive financial prefix comparison

This fixture preserves an independent arithmetic oracle frozen before this
consumer candidate was authored. All files under `inputs/` and `oracle/` are
unchanged copies. The original oracle provenance and manifest remain intact.
Do not rerun `oracle/derive.py` or adapt expectations to consumer output.

The adjacent integration test `guarded_quantity_fee_full_prefix_positive.rs`
calls the production `parse`, `validate`, and `simulate` APIs. It compares every
one of the 18 actual statement prefixes with the independently frozen complete
opening/closing frame and expression value. Complete state, exact asset/account
balance rows, exact duty/party rows, row presence, retained zero duties, all
captured bindings, ordered effect deltas, and cumulative histories are checked.
The final pre/post and complete raw effect sequence are checked as well. Actual
endpoint effects and frames supply gross debit including fees, nonfee debit, fee
credit/debit, gross credit, signed net flow, per-asset token totals, and separately
typed duty addition/discharge projections for comparison with oracle literals.
Reserve balances, reserveDuty, witness Cash, marker/active and settledDuty remain
checked at every prefix. The borrowed scenario must remain exactly unchanged.

The expectation schema deliberately does not specify raw Core encoding. The
test uses these explicit, specimen-scoped representation adapters:

- Public `UInt`/`Value`/`AssetId`/`Snapshot` serde types decode the independent
  primitive decimal strings, Bool values, identities and frame fields. This is
  representation decoding; it never computes expected finance.
- Complete balance maps use `(exact AssetId, account)` keys. Complete duty maps
  use `(name, exact AssetId, debtor, creditor)` keys. Duplicate keys fail the
  comparison and no row is dropped. Input row order is not a compiler or wire
  ordering rule.
- Primitive evaluated-expression oracle results become typed actual Step.value
  comparisons: Bool to Bool, decimal string to UInt. Captured bindings are
  reconstructed only from actual Let results; the execution API has no exposed
  environment. Later uses and emissions are compared separately.
- Actual public Statement/Expr types are rendered into the oracle's existing
  source-statement spellings for this specimen's Name and one-level Binary
  shapes. No ExprId integer, allocation order, general lowering, hidden-node,
  interning or cache rule is imposed. A different lowering requires a disclosed
  adapter review; it cannot change the frozen expectation silently.
- Every actual raw Effect is projected into the independent semantic operation
  schema using its actual adjacent snapshots and actual typed step result.
  Transfer and fee remain distinct. Liability and discharge share a raw variant;
  their actual statement kind distinguishes the operation. State writes retain
  before/after values, and emissions retain typed values. The full raw effects
  must equal the concatenation of these individually checked step effects.
- Account-flow and conservation reporting sums actual effect amounts and frame
  balances only. Signed net uses i128 because these frozen amounts are tiny;
  this helper does not claim generic u128 financial reporting. No expected
  literal is computed with the production evaluator or compiler.

The independent oracle's U1–U6 remain recorded in the unchanged semantic file.
All numerical positive fields needed by this predicate are present. Textual
absence/preservation statements are exercised through complete keyed frames,
exact row counts and the explicit retained-row checks. The oracle's statement
logical-work fields are not observed by this test. EXPECTED-LOGICAL-WORK.json
and derive.py are archival copies only: the new test includes neither. The
unsolicited cap49 refusal subsection is unselected, unadopted and unused. The
conditional cost50 mapping is neither newly established nor executed here.
The original sufficient-cap test and all original fixtures remain untouched.

This is specified-only ordinary Rust Core positive coverage. It supplies no
security/negative test, sponsorship, authorization, native cost, circuit, proof,
K/Quint/Lean-Clean correspondence or Midnight target evidence and closes no
parent roadmap exit.

After sole-integrator adoption and fresh nonauthor source review, the exact
callable check from the Moriarty checkout root is:

```sh
cargo test --locked -p mori-core --test guarded_quantity_fee_full_prefix_positive guarded_quantity_fee_matches_every_complete_independent_financial_prefix -- --exact --nocapture
```

This source packet has not run that command, any build, any product consumer,
any oracle deriver, provider request or Git mutation. No test or acceptance
result is claimed.
