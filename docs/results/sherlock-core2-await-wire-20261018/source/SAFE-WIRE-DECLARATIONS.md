# Core2 await boundary structural wire declarations

Schema sherlock-core2-await-boundary-wire/1; profile sherlock-quint-core2-boundary/1.
Observation root is {schema,profile,result,awaitPrefix}. result has exactly all
B06 StageResult fields; awaitPrefix has exactly all B06 AwaitPrefix fields. Named
sum variants use {Variant:payload}; nullary variants are plain strings. Optionals
retain NoUInt/SomeUInt, NoScalar/SomeScalar, NoSegment/SomeSegment. Scalar types
retain UIntValue/BoolValue, never bool-as-int. Source UInt decimal strings become
arbitrary-precision JSON integers; no new fee/work/domain/range rule is assigned.
Snapshot.state is a sorted [{name,value}] entry array; captures, balances, duties,
steps/effects keep their full source list order, including zero liability rows.

Core input supplies {outcome,awaitPrefix} from separately serialized public
FinancialObservation and AwaitPrefixObservation of the SAME returned invocation.
The adapter NEVER computes awaitPrefix from outcome, runs stage/runtime, constructs
a continuation, filters residual or synthesizes a missing segment. Same-call
provenance and admission/runtime/work/plan/program pairing remain Root producer
premises, not field-derived conclusions. Full Execution/Step/Statement/Effect,
Continuation, logical, hashes, captures, boundary/residual/error data are retained.
Core ExprIds already are numeric source IDs and are not expanded/evaluated.

Quint input is a supplied ITF document plus explicit raw state ordinal and record
index into the full B06 machine.records. The ordinary existing decoder is reused.
It normalizes that supplied Record.result and Record.awaitPrefix ONLY; complete raw
ITF, ordinal, record index and the decoded source record remain provenance. It
never searches for matching results, stutter-erases, computes expected outcomes,
repairs pairing, evaluates publication or derives runtimeResult. The full Invocation
is retained raw but is outside this result/await boundary comparison projection.
No whole-program/whole-invocation equality or additional independence is claimed.

This is an ordinary offline data consumer, not a public runtime/guard/admission or
new launcher/framework. Offline declaration-shape failures are not application
rejections. Synthetic positive supplied carrier tests are authored before adapter
code; they are representation checks, not independent semantic goldens. Actual
Core2 captures, independent prefreeze, B06 frontend/finite tests and full actual
comparisons remain UNRUN and Root-selected after fresh nonauthor review. No source
model/guard/runtime/charge policy is changed. This adapter has no 1024 finite-model
bound; that bound remains a B06 premise and never truncates source UInt128 values.
