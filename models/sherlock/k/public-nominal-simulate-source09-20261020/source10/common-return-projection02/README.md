# Common-return semantic projection — observed scope, 2026-10-21

This additive module consumes already typed NP/NG return objects and the existing positive source10 port metadata. `project_return_pair(np_value, ng_value, ported)` returns exactly one independent logical-result node: UInt uses canonical decimal text, Bool stays native, and every nominal node retains its runtime type/global constructor identity and complete ordered recursive fields. It checks NP names/generic argument order against NG IDs and source metadata, including constructor ownership, ordered fields and each declared child type. Unsupported/inconsistent values raise `ProjectionDomainError`.

The source has no imports or file/process I/O. Caller-supplied complete, finite, acyclic positive metadata and already typed values are premises; it is not an admission/parser/security boundary. The original source document/options/raw empty slices stay unchanged. The source10 positive port in the parent `source/` directory supplies the existing metadata API. No K evaluator or receiving parser is added or replaced.

Observed: twelve new pure tests passed (ten prefrozen complete positive subcases, 0.003s test time, 0.069s F03 child). A separately selected existing-F03 comparison child (0.582s) projected both frozen actual K return trees and matched the unchanged independently source-derived complete expected logical returns with native-type-exact equality. Exact full source identity, original23-field ProgramView and source digest matched too. All original95 capture members and original expectation pins remained unchanged; three private expectation body reads occurred only after both actual observations were durably frozen. No earlier test, compile or model prefix reran.

NG maps/frames/full states and ordered74/108 events remain retained, but the oracle supplies no state/event schedule; their independent equality is NOT claimed. Logical cuts/events are not Core fuel/work clocks. No compiler table-generation proof, admission, genuine Core prefix, StepInv, refinement, native/proof or EARS closure follows. The author had source/expectation/actual exposure. Synthetic unit expectations were same-author source-first, not a fresh independent semantic vote. Fresh Gemini source/API review had zero mandatory changes; actual results are separate observations.

The exact source/test/two-fixture tree is portable and standalone for pure tests. From repository root the prospective command is:

```sh
python3 -I -B -S -m unittest discover -s models/sherlock/k/public-nominal-simulate-source09-20261020/source10/common-return-projection02/tests -p test_common_return_projection.py -v
```

This public-path command is UNRUN; historical results used R paths. Runtime comparison commands in the archive need historical absolute paths, existing received data and intentionally excluded private expectations; they are archival evidence, not a publicly runnable comparison pipeline. [Complete selected source/result archive](../../../../../../docs/results/sherlock-k-source10-common-return-projection-20261021/README.md).
