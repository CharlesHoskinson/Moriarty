# Fresh Core operator and resource result

At the 2026-10-17 05:47 UTC run, a fresh direct `rustc --test` compile and link
of the actual Core `lib.rs` from a then-uninstalled candidate snapshot exited 0.
The selected test run passed **47 tests**, with 0 failed, 0 ignored and 191
filtered out. Compilation emitted 13 warnings and no errors. The same 23
reviewed source and fixture afterimages are now included in the combined
delivery; ordinary source integration has been adopted.

The compiler child took 25.923 seconds and the test child 0.269 seconds. Libtest
reported 0.21 seconds. The recorded parent interval from compilation through the
test run was 26.443 seconds. These are operation timings, not per-case physical,
circuit or proving measurements.

[`test-stdout.txt`](test-stdout.txt) preserves the complete original test stdout.
[`f2-reports.jsonl`](f2-reports.jsonl) contains its four unchanged report lines,
each a JSON array `[case_id, resource_report, event_accounting]`.
[`result.json`](result.json) records the selected result facts, source and output
digests, and scope. Decimal counter strings and every `unmeasured` field in the
original report objects are preserved.

| F2 case | Outcome | Events | Work | Helper work | Reference cursor | Maximum observed logical depth |
|---|---|---:|---:|---:|---:|---:|
| ascending-comparisons | halted | 375 | 374 | 229 | 145 | 11 |
| subtract-positive | halted | 88 | 87 | 58 | 29 | 3 |
| equal-comparisons | halted | 417 | 416 | 257 | 159 | 13 |
| subtract-zero | halted | 89 | 88 | 59 | 29 | 3 |

The depth column comes from actual event accounting. The final-snapshot resource
report retains its separate unmeasured peak fields. Exact aggregate accounting
does not supply a retained complete state/event prefix or physical allocation
history.

Across the combined weighted, threshold and operator tests, the owner pipeline
exercised all eleven binary operators plus `Not`. The four F2 runs explicitly
compared B scalar results with `Core::simulate` for `Ne`, `Le`, `Gt`, `Ge`, `And`,
`Or`, `Sub` and `Eq`. The existing weighted `Add`/`Mul` and threshold `Lt`/`Not`
tests check owner wiring, resources and reports; they do not perform that same
explicit simulation differential. This is ordinary cross-path testing, without
an independently frozen O comparison or complete state/prefix comparison.

The supplied code-128/path-512/fuel-1000/stack-32 inputs succeeded for these four
F2 scenarios only. No general runtime adequacy follows. All four reports use
DedicatedSorts, finance Absent and the host-only profile.

The exact compile closure contains 121 source inputs. The operator checked the
165-file snapshot, 253 candidate-member pins and unchanged recorded binary/tool
metadata. Cached crate and proc-macro source authenticity, actual SVH-selected
macro loading, transitive dependencies and linker/sysroot/runtime trust remain
explicit limitations. Ordinary integration of these 23 afterimages does not
establish generic ADTs, finance-present/full B, or close consequential security,
physical-resource, formal/circuit, SDK/proof/target, financial or language-wide
acceptance. Those gates remain open. C4 remains open and full native R2 remains
unsatisfied. No rerun was made for this publication preparation.
