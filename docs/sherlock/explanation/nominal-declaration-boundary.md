# Why nominal tables are a compiler step, not constructor execution

The installed generic declaration unit connects raw typed declaration inputs to the
existing owner preparation module. Name collection, checked declaration views,
source-order traversal and the owner's T1/T2 FIFO builder produce actual type,
constructor and field vectors. This is useful compiler output: a later consumer
can refer to a type instance and its declaration-driven constructor layout
rather than relying on dedicated List/Tree tags. The
[API reference](../reference/nominal-declaration-api.md) gives the exact inputs,
outputs and public paths.

T1 assigns all non-parameterized declaration TypeIds before field resolution.
A field in the first declaration can therefore refer to a later declaration
without copying its definition or unfolding recursive types. T2 processes queued
instances in FIFO order, resolving fields under their instance's type arguments
and interning newly discovered instances. That declaration graph is not a runtime
value graph: producing its rows neither evaluates constructor operands nor
allocates the immutable values required by UIREQ-007.

The two public producers make name-pool scope visible. The declaration-only
entry supplies NameIds for declaration type names, constructor names and field
names; declaration type parameters and names appearing only in type uses do not
add entries. Candidate02 included those extra names; independent frozen
expectations and source review exposed the actual policy defect. Candidate03
replaces only that collector, leaving the oracle unchanged. The program entry
includes names from
the real Core program and binds declaration names against the unified sorted
pool. It does not add nominal constructors to CoreExpr, compile a new generic
source syntax or complete T3's whole-program type-use traversal. A larger name
pool must not be reported as whole-program generic elaboration.

The next execution boundary remains concrete. Generic runtime dispatch still
needs a ConstructorFields continuation retaining constructor/type identity,
source occurrence, saved environment, evaluated prefix, remaining source fields
and return context. Zero-field, start, field-return, failure and finish behavior
must preserve once-only left-to-right field evaluation. Allocation and value
materialization follow field completion. Ordered matching must select the first
matching source arm, retain binder occurrence paths and fresh bindings, and
avoid executing unselected arm bodies. Constructor table source order alone is
not evidence of either field evaluation or matching behavior.

A separate [whole-program preparation unit](../reference/generic-program-preparation-api.md)
now retains the owned extended source and observes T3 signatures followed by
entry/function-body uses on the same type builder. Generic expression/constant/
program lowering and admission, iterative decoding, witness and boundary
representations remain outside that preparation result. Existing DedicatedSorts behavior and its
nine frames do not implement those generic features. Strict recursive-first and
recursive-last field tests and a genuine declaration-driven ordered-match case
remain independent observable requirements.

Root's [scoped declaration adoption](../../results/sherlock-nominal-declaration-preparation-20261018/ROOT-DECLARATION-ADOPTION.json)
records candidate03's normal Core compile (exit0, 3.987870814 seconds) and the
independent consumer's compile, exact one-test listing and passing test. The
comparison checks the full returned preparation object: seven type rows, one
type-argument slot, eight constructor rows, ten constructor-field slots, all
metadata and six local declaration type occurrences. These are declaration
preparation observations, not six runtime tests or generic value execution.
The two author cfg(test) cases remain unrun, including the author positive case;
no new negative-case or formal result is claimed. The earlier author and
placement ERROR replies remain preserved. The successful normal library was
reused for the external consumer instead of rebuilding that passing prefix. No whole original UIREQ-004,
UIREQ-007 or UIREQ-034 acceptance follows. Their existing
strict evaluation, finite nominal value and first-arm matching scenarios stay
intact. This batch changes no circuit, proof, deployment or target status.
