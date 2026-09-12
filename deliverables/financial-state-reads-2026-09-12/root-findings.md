# Root observations during authoring

## Test oracle: remaining256/spent1 is a valid work record

The initial author test `invalid state is admitted before false guards and missing
lookups` sets work to remaining256, spent1, closureReserve16, with snapshot
workInitial256, and expects WORK_MISMATCH or INVARIANT. This is not invalid under
the frozen plan or existing kernel: remaining matches snapshot, and arithmetic
fits UInt128. There is no universal initial budget256 invariant in the API.
Please correct this test oracle, not production semantics. A real mismatch is
snapshot255 versus remaining256; a real invalid work record overflows remaining
plus spent/closureReserve per existing parseWork. Preserve the initial failed
oracle receipt if executed. Valid nonzero spent must be carried forward.

The implementation has not yet been completed or tested by root. This is a
bounded source/test observation, not a final candidate defect verdict.
Independent root probes and work expectations are in this directory.

## Core /2 result type must describe state-admission rejection

The in-progress runFinancialContract returns `admitted.result as ExpressionResult`
for invalid state, but ExpressionResult's Rejected branch promises span,
nodePath and workUsed. The kernel-shaped result has actionIndex/null or only
status/code instead. Do not conceal that new public API variant with a cast.
Either normalize Core /2 state-admission failures to a documented expression
rejection shape (same kernel code, synthetic span, zero work), or publish an
accurate explicit Core /2 result union and propagate it through callers. Source
/3 may still retain its specified kernel-shaped admission rejection envelope.
Please pin the chosen exact Core /2 envelope in spec/tests. This observation is
from in-progress code, to recheck after implementation before final audit.

Reproduced the Core /2 envelope mismatch against the current implementation:
`root-core-envelope-01.json` records evaluate(valid literal request, context '{}')
returning `{status:Rejected,code:SCHEMA,actionIndex:null}` with all three fields
promised by ExpressionResult absent. This needs an accurate public result type
or normalized rejection before publication; the source API probes otherwise pass.
Root current probes:52 API/CLI cases and36 Core cases pass,174 old-source
comparisons match. These do not waive the result-type defect.

Resolved observation: author corrected the prior-work test to remaining255,
spent0 with snapshot256 and exact WORK_MISMATCH. Root's valid prior-spent17
case passes. No new fixed-total invariant was introduced. The Core /2 rejection
type/envelope mismatch remains open pending author correction.

## Canonical /3 EBNF currently disagrees with parser on generic arity

`financialRead` currently spells `< typeArgument >`, while the shared parser
accepts `outstanding<Cash,Cash>(...)` and static checking rejects SOURCE_ARITY.
Actual observation: root-grammar-arity-01.json. Because this is advertised as
complete syntax grammar in README, make it match parser syntax using the
existing `typeArgs` production (with exact-one/simple-symbol restriction stated
as a static rule), or consistently reject extra generic arguments in parsing.
The first is the minimal choice and preserves current SOURCE_ARITY semantics.
Update both canonical EBNF and copied README block, plus a focused parser/static
arity test. This is separate from the old-profile compatibility contract.

README completeness check: include an explicit result-type rule/table for the
six reads (obligation -> Quantity<Units<U,1>,0>, balances/allowances -> Amount<A>)
and an explicit pre-state lookup/debit rule, not only a list of constructor names.
The plan requires typed-read and reduction/work rules in README; current prose
names the argument type but does not state which result type each family returns.

## Final disposition

All three production/documentation findings were repaired by the same Grok author. Final root checks and fresh Astra audit approve manifest `a4c365c08cf4af09555c6cb60642e366c447e40ed1923723f26c000b173423d0`. Historical failed probes remain unchanged. The root legacy-baseline-02 failure was an incorrect fixture parameter assumption in the probe; corrected comparisons passed 174 cases.
