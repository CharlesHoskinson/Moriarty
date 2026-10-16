# Fixed Core-source ordinary oracle provenance

Status: independent hand-derived expectations, frozen before producer execution;
specified-only/unexecuted. Author: independent source-oracle lane. Baseline checkout
HEAD: `07e85a639bb2f38451bba83704d49b33162f36b2` (2026-10-16).

The falsifiable predicate is that each actual fixed Core-source projection,
post-startup Snapshot and StepRecord for every listed finite fuel budget equals
its complete independent literal expectation, including the free terminal edge.
This document records expected behavior, not an observed producer pass.

## Sources and isolation

Source facts were read from the existing adapter, parser, IR, diagnostic display,
Snapshot/StepRecord types and pinned K rules. The new source observation producer
was neither read nor executed. No evaluator/compiler/model execution, cargo,
SDK/native/target operation, external model call or expected-from-actual
transcription occurred. Python was used only to serialize explicitly chosen
literal rows, calculate hashes and validate their structure. Constructor string
formatting in the temporary transcription script performs no transition,
expression evaluation, source parsing, name binding or rule selection.
The consumer author supplied field names only; no actual observations or expected
values were exchanged before this freeze. Its proposed counts of 34 and 32 were
hypotheses; the independently enumerated source-rule schedules below agree.

| Existing source | SHA256 |
| --- | --- |
| `models/sherlock/k/main.k` | `b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41` |
| `crates/mori-core/src/sherlock/source_adapter.rs` | `d7fba670a7903ff751d895806375c652ecaa21a9f71689446b3cabf9f960e065` |
| `crates/mori-core/src/parser.rs` | `83c8d4fabbd5837517c43353528545a783784e202aa913f7924a5d72c50abed2` |
| `crates/mori-core/src/ir.rs` | `0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d` |
| `crates/mori-core/src/sherlock/reference_types.rs` | `4b4bb364a0c8182e117a9c36efb5d91cb7e405f653a6d3870f54dc03eefa317f` |
| `crates/mori-core/src/sherlock/reference_display.rs` | `3f3456bfeaa6ce043986de29673ba038d94cfa405de2d715fd774ad8aa1872e4` |
| `crates/mori-core/src/sherlock/source_adapter_fixtures/weighted-total.mori` | `073474d19e6294ab77a053dcc6e0a66defdfdb32f047a795cb16652ce4b5bb65` |
| `crates/mori-core/src/sherlock/source_adapter_fixtures/threshold-check.mori` | `caba5067afdddf62835af5fa8864168fdb5ca4b1493281b3085796262cdbf796` |

Existing adapter and source-origin tests were inspected as existing descriptions
of the adapter/path and caller-restoration contracts; they were not run and were
not used to obtain observations of these new complete programs.

## Source projection

Repository observation: the current adapter accepts a stateless Core/1 action
with ordered lets and one final emit. It preserves input order, wraps the action
as a typed host function and folds lets in reverse to preserve source nesting.
It returns the emit VALUE only; the ordered financial emit effect remains in
Core. These expectations execute no financial effects and invent no balances,
fees, debit, liability, authority, settlement or residual-duty observation.

S01 selects `quote` from WeightedTotal with arguments `(4,9)`; source arithmetic
is `gross=4*9=36`, `result=36+3=39`. S02 selects `inspect` from ThresholdCheck with
arguments `(7,10)`; `below=(7<10)=true`, `result=not true=false`.
The current adapter generates a function wrapper; this establishes no source
function, recursion, branch or general-data feature.

The parser appends each expression after its children. Weighted expression IDs
are `0 units`, `1 price`, `2 Mul(0,1)`, `3 gross`, `4 UInt(3)`, `5 Add(3,4)`,
`6 result`; statements are let gross=2, let result=5, emit total=6. Threshold IDs
are `0 observed`, `1 threshold`, `2 Lt(0,1)`, `3 below`, `4 Not(3)`, `5 result`;
statements are let below=2, let result=4, emit atOrAbove=5. Let occurrences have
no Core expression ID. The weighted body preorder paths are `[]`, `[0]`,
`[0,0]`, `[0,1]`, `[1]`, `[1,0]`, `[1,0,0]`, `[1,0,1]`, `[1,1]`; threshold omits
`[1,0,1]`. Every source occurrence includes its statement, node kind, exact
Core-expression Debug text or null, and path in the literal preparation report.

The input document retains the full source text and source hash independently
of the six-field input identity. Full `code_k` and `entry_k` bind the expected
host program. `preparation.inputs[].type_debug` and `return_type_debug` follow
the producer's declared diagnostic schema; only these field names were adapted
from the actual report's `ty` and `return_type`. No expected semantic value was
changed to follow a candidate output.

Opaque names such as `quote`, `units` and `gross` are preserved. KDisplay prints
them verbatim. These are typed-host diagnostic terms outside main.k's numeric
name grammar. No numeric renaming, executed K result or K source-name
correspondence is asserted.

## Complete state and edge contract

The existing normal JSON reconstruction convention is retained. Each state
resolves full immutable program identity from `code_ref` and retains control,
environment, ordered top-first frames, frame count, stack limit8, initial and
remaining fuel, charged work and pending outcome. No Snapshot field is absent
from the resolved observation. Each consecutive state pair yields a StepRecord
with its incoming rule and charge1. Startup74 has charge0 and is excluded from
the post-startup observation boundary.

Every explicitly listed budget row fixes its complete boundary and free
publication. The prefix reuses the shared pending states through that boundary,
replacing only initial fuel and remaining fuel. The final record preserves all
program/env/frame/depth/limit/work/fuel fields; only control becomes `.K` and
outcome becomes the row's `uiExhausted()` or `uiDone(...)`. Exhaustion84 applies
to all incomplete zero-fuel boundaries; done80 applies after the final charged
return, even at zero. Terminal has no next transition. Pure helpers are
unmetered in profile A, and their work is not asserted as zero.

## Independently enumerated schedules

In this table `E` is the parameter environment, `G` adds gross/below, and `R`
adds result. Frame identities and all saved maps/expressions are literal in the
JSON. Both start with empty map and empty stack. Arguments are evaluated in
source order. At bind204 and schedule208 the active environment stays empty;
only push135 activates the parameter map.

| State | S01 incoming rule and expected boundary | S02 incoming rule and expected boundary |
| --- | --- | --- |
| 0 | free74: evaluate quote(4,9) | free74: evaluate inspect(7,10) |
| 1 | 131 schedule argument frame | 131 schedule argument frame |
| 2 | 135 push args, evaluate4 | 135 push args, evaluate7 |
| 3 | 102 return4 | 102 return7 |
| 4 | 177 append4, evaluate9 | 177 append7, evaluate10 |
| 5 | 102 return9 | 102 return10 |
| 6 | 181 append9, pop args, invoke quote | 181 append10, pop args, invoke inspect |
| 7 | 200 find quote | 200 find inspect |
| 8 | 204 bind units4,price9 | 204 bind observed7,threshold10 |
| 9 | 208 schedule UInt return frame/body | 208 schedule Bool return frame/body |
| 10 | 135 push return, activate E | 135 push return, activate E |
| 11 | 114 schedule gross let | 114 schedule below let |
| 12 | 135 push let, evaluate multiplication | 135 push let, evaluate comparison |
| 13 | 117 schedule left-mul | 117 schedule left-lt |
| 14 | 135 push binary, evaluate units | 135 push binary, evaluate observed |
| 15 | 108 return4 | 108 return7 |
| 16 | 149 retain right-mul4, evaluate price | 149 retain right-lt7, evaluate threshold |
| 17 | 108 return9 | 108 return10 |
| 18 | 153 pop binary, stage36 | 153 pop binary, stage true |
| 19 | 223 return36 | 223 return true |
| 20 | 145 replace let with scope(E), bind gross36=G | 145 replace let with scope(E), bind below true=G |
| 21 | 114 schedule result let | 114 schedule result let |
| 22 | 135 push let, evaluate addition | 135 push let, evaluate not |
| 23 | 117 schedule left-add | 120 schedule unary |
| 24 | 135 push binary, evaluate gross | 135 push unary, evaluate below |
| 25 | 108 return36 | 108 return true |
| 26 | 149 retain right-add36, evaluate literal3 | 157 pop unary, stage false |
| 27 | 102 return3 | 223 return false |
| 28 | 153 pop binary, stage39 | 145 replace let with scope(G), bind result false=R |
| 29 | 223 return39 | 108 return false with both scopes and return pending |
| 30 | 145 replace let with scope(G), bind result39=R | 195 pop inner scope, restore G |
| 31 | 108 return39 with both scopes and return pending | 195 pop outer scope, restore E |
| 32 | 195 pop inner scope, restore G | 185 check Bool, pop return, restore empty |
| 33 | 195 pop outer scope, restore E | complete already |
| 34 | 185 check UInt, pop return, restore empty | complete already |
| publish | free80 done39 | free80 done false |

Source arithmetic helpers256/250 normalize 4*9=36 and254/250 normalize36+3=39
inside charged rule153. Threshold helper257 normalizes7<10=true inside153 and
helper274 normalizes not true=false inside157. None introduces another A charge.

Manual count: call entry takes10. A let adds scheduling114, pushing135,
binding145 and eventual restoration195 around its RHS/body. Each normal binary
costs7 including its two one-step leaves; unary costs5 including its leaf.
Thus weighted costs `10 + 4 + 7 + 4 + 7 + 1 + 1 = 34`, and threshold costs
`10 + 4 + 7 + 4 + 5 + 1 + 1 = 32`. The final two ones are the result variable
and caller-return check. Both maxima are depth4, below the selected limit8.

| Cardinality | S01 | S02 | Total |
| --- | ---: | ---: | ---: |
| Shared complete pending states, including state0 | 35 | 33 | 68 |
| Distinct charged edges in full run | 34 | 32 | 66 |
| Explicit fuel budgets/prefix runs | 36 (0..35) | 34 (0..33) | 70 |
| Exhausted runs | 34 | 32 | 66 |
| Done runs (exact and plus one) | 2 | 2 | 4 |
| Charged records across all runs | 629 | 560 | 1189 |
| Free publication records across all runs | 36 | 34 | 70 |
| All post-startup StepRecords | 665 | 594 | 1259 |
| Initial snapshots across all runs | 36 | 34 | 70 |
| Terminal snapshots across all runs | 36 | 34 | 70 |
| Source occurrences | 9 | 8 | 17 |

## Pre-execution freeze

The two independently authored oracle files are frozen at these byte hashes:

| Artifact | SHA256 |
| --- | --- |
| `source-normal-programs.json` | `a3542b1101251f71e6b41ef6e7a8d7d7694a3ec3c9d594ed79c8c9ea0d3eaa00` |
| `source-normal-prefixes.json` | `184f9136da321975d6d447bf46bb0d1ac6459c12a67d0ae112ebf92fefa5f652` |

Frozen N01–N08 artifacts were not edited. Their before/after hashes remain:

| Artifact | SHA256 |
| --- | --- |
| `normal-programs.json` | `ef87f94f7e5d22a75b1c2e5e61af016b08175beee9a3599e3e69ddf92945d32f` |
| `normal-prefixes.json` | `dca67b553d7483fdf7d9148e7f4bf1a6022b42b8843635f33f23c58a2415d56d` |
| `extra-normal-programs.json` | `dd85e5d400f24e9be5e2ff017f4b8c141a0bf6e4270053917b735cbe5153f153` |
| `extra-normal-prefixes.json` | `acf460094b258910bad0499fb8feda0cd85b680023939800ad714d2cb71bee16` |

Only JSON parsing, literal row/field/count/hash validation, source hash checking
and diff inspection were performed. No actual consumer result is claimed.
Independent audit and any observed result belong to separate scoped work.
K execution, full grammar, K/Quint/Lean/Clean/circuit correspondence, native and
target qualification remain open, as do U0–U7, M1–M17 and MC/SP/G exits.
