# Offline nullary-binding tool control

Classification: tool experiment and factoring research only, 2026-09-05.
This does not verify Candidate A or discharge A5.

The prior93,413,860-byte input still matches its archived digest. Its single
flattened module contains176 declarations (169defs,6typedefs,1var). The old
eight-GiB log already says InlinePass keeps only relevant root operators, so
dead-definition pruning alone is not a supported changed hypothesis.

Apalache's [inlining design](https://apalache-mc.org/docs/adr/018adr-inlining.html)
describes retained nullary local bindings as a means to share evaluated terms.
Its [preprocessing reference](https://apalache-mc.org/docs/apalache/preprocessing.html)
distinguishes parameterized local operators, which are inlined. This motivates
testing explicit local argument bindings; it does not establish that the actual
Candidate A evaluator will become tractable. Existing model locals already
survive Quint compilation as nullary let definitions.

The archived control has no Candidate A imports. It compares direct repeated
integer computation with an explicit local binding while x advances0,1,2.
Quint0.32.0 compiled both invariant selections from the identical source.
Apalache0.56.1, build70cdaf4, ran offline with one-GiB heap,120-second command
limit, depth2, no fairness and explicitly disabled terminal deadlock checking.
The agreement check reached states0–2 and exited0/NoError. The deliberately
false invariant x<1 produced a state1 counterexample and exit12. Original raw
streams, generated inputs, SMT logs and the counterexample remain archived.
The unchanged control source and recorder were pinned before each command;
Java identity was additionally recorded before the two checks.

This establishes only that this local control translates and produces the
expected bounded result/negative control. It is not a performance comparison,
a general equivalence proof, or evidence that Candidate A properties passed.
No repeated heap-only Candidate A attempt was launched. A concrete reviewed
factoring plan, complete original/factored corpus comparison, generated-size
measurement and actual meaningful authority-slice verification remain required.

Next hypothesis to review: explicit nullary argument sharing and joint raw/
effect computation, preserving public supplied-result equality, all diagnostics,
effect order and complete authority histories. Do not modify frozen originals.
If this does not reduce expansion, require a different justified hypothesis
before another expensive checker attempt; microstep or table abstractions would
require separate trace-preservation/coverage arguments and are not adopted here.

The JVM emitted deprecated/native-access warnings. They are retained separately
from the successful or counterexample checker outcome. The clock in Apalache
logs is local time; command bounds and durations are recorded independently.
