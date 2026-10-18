# `doctorwatson<UTXO>` reporting requirements

These requirements concern the local reporting tool. Passing them supplies no compiler, cryptographic or ledger acceptance. The original product requirements remain in their existing files.

| ID | EARS requirement |
| --- | --- |
| DWU-01 | WHEN a report loads, it SHALL retain exact requirement membership and ordered scenario names from each original pinned catalogue. |
| DWU-02 | WHEN closure is declared, the reader SHALL require an explicit applicable original profile and target and SHALL refuse implicit profile or network migration. |
| DWU-03 | WHEN a declared supporting input changes or disappears, the reader SHALL mark dependent declarations non-current while retaining the historical acceptance declaration. |
| DWU-04 | WHEN closure is declared, the reader SHALL require complete observed scenarios, actual evidence, independent source review and actual-result review declarations for that scope. |
| DWU-05 | WHEN a valid report contains open requirements, the reader SHALL distinguish successful integrity validation from a failing requirement-closure gate. |
| DWU-06 | WHEN next work is requested, the reader SHALL expose the missing predicate, actual consumer, expected observation, owner and blockers. |
| DWU-07 | WHEN family status is reported, the reader SHALL preserve all twenty original families with independent source, admission, compiled, native and private states. |
| DWU-08 | WHEN result evidence is referenced, the reader SHALL preserve its identity and SHALL NOT infer successful product behavior from an expected missing-implementation result. |
| DWU-09 | WHILE the default report runs, the entrypoint SHALL perform local report validation and ordinary reporting regressions only; embedded commands SHALL remain data. |
| DWU-10 | WHEN the UTXO reader is integrated, it SHALL leave Sherlock's entrypoint, corpus and evidence identities unchanged. |
| DWU-11 | AFTER the bounded reporting bootstrap, delivery SHALL advance an actual product predicate; further reporting changes SHALL address a demonstrated defect in a real closure attempt. |

The reader checks recorded declarations and declared dependencies. It does not authenticate a reviewer, verify a proof, establish completeness of a dependency graph or decide whether an expectation expresses the intended semantics. DWU-08 includes a reporting prohibition, not a new implementation of substantive failure classification. DWU-11 is a delivery rule, not a scheduler.
