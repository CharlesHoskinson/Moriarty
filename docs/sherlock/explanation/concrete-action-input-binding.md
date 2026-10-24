# Concrete action input binding

Status: implementation candidate, no new test or runtime result yet.

The scalar consumer already accepts the same named typed map as Scenario.inputs.
This public follow-on supplies that map without importing Scenario state,
balances or liabilities into a pure scalar projection. It reuses the existing
strict JSON parser, UInt/Value codecs, input guards, source adapter and owner.

The consumer walks the action's declaration order and looks up each map value.
It constructs Literal arguments at the entry Call while preserving function
parameters, Vars, lets, operators and the original body. BTreeMap order therefore
controls report name ordering, not parameter binding. The compiler does not
evaluate the source and replace its body with a result literal.

This is visible concrete input binding. The values are in the report and compiled
entry arguments; they are not symbolic private witnesses. Changing them can
change constants, admitted tables and program identity. A stable circuit across
private inputs needs a separate implemented input/witness boundary and
correspondence argument.

The new option extends the existing artifact rather than introducing an
unconsumed report format. Current prove/verify read source, Scenario and target
through their native backend path; no HostOnly artifact importer was found in
the inspected repository consumers. This follow-on adds no such bridge.

The two frozen public specimens distinguish declaration order from lexical map
order and produce different scalar kinds. Separate independent expectations cover
complete source-denotational values conditional on actual admission and Halted.
They do not independently predict IDs, tables, images, counters, intermediate
states, work costs, event traces or unconditional termination. Author tests and
shared-producer equivalence checks are identified separately.

General helpers/ADTs, persistent input, witness/constraint/export, grouping,
residual duties, complete finance, whole-backend CLI, process-wide H1,
formal correspondence and actual native/target acceptance remain separate
obligations.
