# Independent positive Core3 financial/work freeze

Two parent-selected source programs; three bounded cases including P1's full
discharge variant. The sources here reconstruct the parent message and do not
claim original source-byte identity. No product implementation or selected-case
test assertions/results were used. Only standalone stdlib arithmetic ran.

P1 discharges 3 from an existing loan5, leaving present loan2; the separate loan3
variant leaves present loan0. Both have one Liability effect, no balance rows,
no cash movement and no explicit fee. The zero row remains in the snapshot but
is absent from the positive residual. Expected local work is 2.

P2 starts waiting with loan5; a false go event retains that boundary and emits no
segment; a true go event has Await Bool(true), then Discharge UInt3, leaving loan2.
Expected attempted work is 1,2,4, with committed successful-prefix work 1,1,5.
The plan costs are [1,4], committed_before [0,1], plan_total5. Total attempted is7.
False-guard attempted work2 does not enter committed work. Cap stays absent/null.

These staged and work predictions explicitly assume that the proposed Core3
extension preserves the normative Core2 financial/stage schedule. The provided
/3 clause is labeled PROPOSED-UNADOPTED. No Core3 adoption, source admission,
security identity, actual test, proof, native result or general acceptance is
claimed. Concrete ExprIds, raw wire bytes, program/plan identities and consumer
results remain unpredicted. External DUST and service costs remain unmeasured.

EXPECTED-FINANCIAL-AND-WORK.json contains complete case/operation/statement
frames, typed values, effect deltas, prefixes, residuals and arithmetic schedules.
NORMATIVE-PINS-AND-CLAUSES.json retains exact selected public normative sections.
PROVENANCE.json discloses prior context, administrative reads and independence.
FREEZE-MANIFEST.json hashes all packet artifacts except itself. Preserve this
frozen packet before any reproduction; rerunning derive.py changes its timestamp.
The parent must compare actual results separately without changing this oracle.
