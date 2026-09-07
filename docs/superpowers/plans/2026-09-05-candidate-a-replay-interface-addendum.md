# Candidate A replay interface correction addendum

This bounded correction supplements, and does not rewrite, checker plan
`2edb50ec93c88c62d8fddf1ba9d29b893e494b1564d28fdef6dd82674fca739e`
adopted at d4f6196. Root's independent inventory enumeration exposed the error
after adoption. Both plan authors independently confirmed the corrected sums.
No case, required event, semantic assertion or profile is removed.

## Exact inventory arithmetic

| Family | Cases | Events |
| --- | ---: | ---: |
| Installment ordinary | 22 | 516 |
| Installment negatives | 10 | 122 |
| Swap ordinary | 24 | 484 |
| Swap verified-stale | 2 | 44 |
| Swap negatives | 20 | 391 |
| Total | 78 | 1557 |

Swap negative events sum32+192+17+28+40+40+42=391, not591. Installment retains
638events/637steps. Swap has919events/918steps, not1119events/1118steps.
Every individual per-case count and ordered instruction schedule stays unchanged.
Do not insert dummy events to satisfy the old arithmetic.

At future checker implementation, replace each specified aggregate1757 with1557
and each swap-negative591 with391 in Task4 inventory assertions, Task5 report,
Task6 full-package assertion and associated expected-result text. The old plan
and original failing pure-enumeration receipt remain preserved. This arithmetic
check is not authority execution, behavioral RED/GREEN or model verification.

Root's literal inventory is evidence/s02-candidate-a-completion/a4/inventory.json.
Its canonical compact sorted-key UTF-8 JSON SHA256, without a final newline, is
8a1a6136afc9fda3c29e050df8b80144750cc21365e9194924401c155dc6402b.
Both independent enumerators must match all78ordered descriptor/count records.
Actual history execution must still produce every required event.

## Complete Python package input

In Task5's PYTHON_SOURCES set add exactly `moriarty/__init__.py`. Its current
SHA256 is6d6b58f9875c357d5d16858e55ce81edd1e56ecb1c5b28476c7a911cad2e1127.
The file is an imported package initializer, presently only a docstring; it is
part of source closure nonetheless. Include it in FROZEN_EXTRA with this digest
and in producer source pins. Do not modify it. Task1 already archives its bytes.

## Artifact layout

The adopted checker input root contains `installment.itf.json`, `swap.itf.json`
and every separately admitted receipt-relative path. Source pins resolve under
source_root; input and receipt pins resolve under input_root. The two pin groups
are disjoint. The root-admitted required receipt set must include original
command, source snapshot, tool identity, raw stream and terminal records.

Producer validation must use this same layout. Verify each admitted file digest,
the exact required source/import pin set and exactly two raw input pins. Unrelated
notes in those roots are not admitted evidence and need not cause a whole-tree
equality failure. No submitted pin map defines its own required source/receipt
inventory. Root owns admission and independently audits complete original receipts.
This changes no checker CLI signature or actual raw adjacency requirement.

Task1 carrier/bridge and Task2 authority semantics are unaffected. Native source
review, full actual exports/replay, mutation tests, A5 checking and Council remain
mandatory future gates. This addendum does not accept an exported package.
