# 4.1a row probe, 2026-11-10 (partial)

Run `20261110T074428Z-t3x-rowprobe` (`scripts/doctorwatson heavy`, test `c6_universal_probe_tests::row_probe_every_c6_circuit`, branch `worktree-agent-acf90d07b6b60767c` at `79a3fad8c`, qualification config `testdata/dispatch/config.json`). Measured with `IrSource::model()`. A projection, not a bound (SAS-014).

The run reached its wall cap of 30,000 s (exit 4, peak RSS 11.5 GB). Coverage: the dispatch block and 15 of the 63 per-rule circuits. The other 48 were not measured. Each k=23 circuit took 25 to 30 min to model.

| Circuit | File | Rows | k | IR instructions | Seconds |
|---|---|---:|---:|---:|---:|
| `dispatch_block` | `c6_dispatch_step` | 2,685,804 | 22 | 1,283,999 | 924 |
| `KAdminPublishValue` | `c6_admin_publish` | 4,534,681 | 23 | 2,206,676 | 2044 |
| `KMachineEvalLiteralValid` | `c6_eval_schedule` | 5,116,402 | 23 | 2,516,671 | 1872 |
| `KMachineEvalVariablePresent` | `c6_eval_schedule` | 5,116,400 | 23 | 2,516,671 | 1873 |
| `KMachineEvalBinary` | `c6_eval_schedule` | 5,116,513 | 23 | 2,516,757 | 1909 |
| `KMachineEvalUnary` | `c6_eval_schedule` | 5,116,513 | 23 | 2,516,757 | 1918 |
| `KMachineEvalIf` | `c6_eval_schedule` | 5,116,513 | 23 | 2,516,757 | 2050 |
| `KMachineEvalMatch` | `c6_eval_schedule` | 5,116,513 | 23 | 2,516,757 | 1985 |
| `KMachineEvalCallArguments` | `c6_eval_schedule` | 5,698,210 | 23 | 2,826,739 | 2436 |
| `KMachineReturnUnary` | `c6_legacy_control` | 4,534,996 | 23 | 2,206,950 | 1728 |
| `KMachineReturnIfTrue` | `c6_legacy_control` | 4,534,980 | 23 | 2,206,937 | 1642 |
| `KMachineReturnIfFalse` | `c6_legacy_control` | 4,534,990 | 23 | 2,206,946 | 1631 |
| `KMachineReturnMatch` | `c6_legacy_control` | 4,535,092 | 23 | 2,207,025 | 1630 |
| `KMachineReturnArgumentsFinal` | `c6_legacy_control` | 4,534,998 | 23 | 2,206,955 | 1630 |
| `KMachinePrimitiveValue` | `c6_legacy_control` | 4,534,849 | 23 | 2,206,800 | 1631 |
| `KMachineCasesTry` | `c6_legacy_control` | 5,099,169 | 23 | 2,505,841 | 1833 |

Result: every measured per-rule circuit is k=23 (4,534,681 to 5,698,210 rows), and the dispatch block alone is k=22 (2,685,804 rows). This confirms the 4.1 probe (k_U >= 23) on 15 more circuits. Raw lines: `rows.jsonl`.
