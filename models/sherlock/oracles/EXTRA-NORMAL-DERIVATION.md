# Two additional normal K-A oracle programs

Status: independently hand-authored source expectations, specified-only and
unexecuted. Nonsecurity source preparation only; independent audit is pending.
Baseline: `30b65d925d1fc8210cb3521e7091aba787a69141`.
Profile: `sherlock-k-a/1`.

The source task predicates are: N07 has a complete charged-state sequence and
every budget prefix for three source-ordered arguments producing `uiU(321)`;
N08 has the same coverage for named self-recursive factorial two producing
`uiU(2)`, with caller environments and pending multiplication frames preserved.
Any missing state, field, budget or wrong source-rule locator falsifies these
source preparation predicates. Implementation agreement is a separate predicate
that has not been run or claimed here.

The derivation uses the human-readable rules in
[`../k/main.k`](../k/main.k), SHA256
`b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41`.
[`../reference/rules.json`](../reference/rules.json), SHA256
`71b4e4e67173cf7da6fbff8cd7dfbb36533250278565469b67bfd0b77468ad2a`,
supplies the external source-rule labels and locators. Existing normal JSON files
were read for serialization format only. No production Rust, Lean, Quint or K
implementation/evaluator was imported, run or used to obtain expected states,
values or work. The retained
[`build-extra-normal-literals.py`](build-extra-normal-literals.py) lists all
states and budget rows manually. Its repeated K-text constants are serialization
abbreviations. It contains no evaluator or transition implementation.

There was no qualified Fresh graph in this isolated checkout; source files were
read directly without rebuilding a graph. The development skill, checkout
instructions, product contract and guarded `status --json` were read first.
Historical campaign blockers were preserved. No private source was read.

## Rule and field conventions

Each numbered state is after that many charged machine rules. An incoming
`via_rule_start_line` always locates the first line of the K rule. Relevant
catalog labels and starts are:

| Start | External catalog label |
| --- | --- |
| 74 | `k.admin.startup.valid` |
| 80 | `k.admin.publish.value` |
| 84 | `k.admin.publish.exhausted` |
| 102 | `k.machine.eval.literal.valid` |
| 108 | `k.machine.eval.variable.present` |
| 117 | `k.machine.eval.binary` |
| 123 | `k.machine.eval.if` |
| 131 | `k.machine.eval.call.arguments` |
| 135 | `k.machine.push.accept` |
| 149 | `k.machine.return.left` |
| 153 | `k.machine.return.right` |
| 161 | `k.machine.return.if.true` |
| 165 | `k.machine.return.if.false` |
| 177 | `k.machine.return.arguments.next` |
| 181 | `k.machine.return.arguments.final` |
| 185 | `k.machine.return.call.valid` |
| 200 | `k.machine.invoke.find` |
| 204 | `k.machine.enter.bind` |
| 208 | `k.machine.bound.valid` |
| 223 | `k.machine.primitive.value` |

All machine edges here charge one A fuel unit. Startup74 and publication80/84
charge zero. Helpers are unmetered in A; helper visits and B work are unknown,
not reported as measured zero. All literal integers are valid UInt128 values.
Rule230 validates those literals; rule238 recognizes UInt128 return/argument
types. Lookup282 selects the matching function from the immutable ordered table.
Append286/287 preserves argument order; bind290/291/294 builds a fresh parameter
map with distinct binders. This pair uses one function each, but adds no new
closed-body or unique-function-table admission rule. Existing first-matching
ordered lookup behavior remains unchanged.

The JSON representation retains every configured K cell:

| K cell or observation | Packet location |
| --- | --- |
| `k` | Every state's literal `control_k`; explicit `uiRun` initial configuration; publication `.K` |
| `env` | Every state's `environment_k`, plus saved maps in control/frames |
| `stack` | Every state's complete `frames_top_first_k`, folded right to `uiNoFrames()` |
| `depth` | Every state's literal `depth` |
| `stackLimit` | `fixed_state_fields.stack_limit=8`; pre-start configuration is zero |
| `code` | Immutable ordered source `code_k` via exact case/input digest `code_ref`; pre-start `uiNoFuns()` |
| `fuel` | Literal `remaining_fuel` in each state and every budget row |
| `initialFuel` | Fixed exact-completion value, replaced by the literal budget row; pre-start zero |
| `out` | Fixed `uiPending()` until explicit `uiDone(...)` or `uiExhausted()` publication |
| Consumed A work | Literal `charged_steps`, equal to boundary state index |
| Rule identity | Incoming source-rule start, plus explicit zero-charge startup/publication |

The five-column prefix convention is the existing normal-corpus format. Each
literal row identifies its full boundary state and replaces only the initial and
remaining fuel counters. The complete preceding sequence uses the row's initial
fuel minus each state's charged count. Prefix startup also replaces the `uiRun`
fuel argument; the pre-start configured fuel counters remain zero. Publication
changes only control and output, preserving the boundary map, frames, depth,
code, limits and counters. No committed financial or attempted-work observation
is invented for these scalar programs.

Input identities use the existing six-source-field convention (compact UTF-8
sorted-key JSON of case_id, code_k, entry_k, initial_arguments, stack_limit and
exact_completion_fuel):

| Case | Input SHA256 |
| --- | --- |
| N07 | `4ea276354b7344fdb7bb0456fccad4a2d07e8c8b95968f45a8a29a5820b91172` |
| N08 | `9a407ff6b874bb837093b771e323187927a61ff706dba236a3cdd69c0d3dc8b7` |

## N07: weighted ordered arguments

Function17 has parameters `(0,1,2)` and body
`(a + 10*b) + 100*c`; entry calls it on `(1,2,3)`.
The nested addition is explicit in the source. Ordered append gives values
`[1,2,3]`; binding gives `0 |-> uiU(1) 1 |-> uiU(2) 2 |-> uiU(3)`.
The result arithmetic is `10*2=20`, `1+20=21`, `100*3=300`, `21+300=321`.
Helpers254/256/250 normalize primitive results without another A charge.

In this table, `R` is the caller-return frame saving the empty map. `O` is the
outer addition frame, `I` the inner addition frame, and `M` the current
multiplication frame. Exact saved expressions and maps are in every JSON state;
the table explains the manually selected next state.

| State | Incoming rule | State reasoning |
| --- | --- | --- |
| 0 | startup74, free | Evaluate the call with empty map, empty stack, fuel38. |
| 1 | 131 | Schedule the argument frame; first argument1 is next. |
| 2 | 135 | Push the argument frame at depth1; evaluate literal1. |
| 3 | 102 | Return1 to the argument frame. |
| 4 | 177 | Append1; retain the frame and evaluate argument2. |
| 5 | 102 | Return2. |
| 6 | 177 | Append2 after1; evaluate argument3. |
| 7 | 102 | Return3. |
| 8 | 181 | Append3, pop arguments, invoke17 on `[1,2,3]`; depth0. |
| 9 | 200 | Lookup selects the source function17. |
| 10 | 204 | Bind all three values; active caller map remains empty. |
| 11 | 208 | Schedule `R` saving empty map and body with the new bindings. |
| 12 | 135 | Push `R`; activate parameter map; depth1. |
| 13 | 117 | Schedule `O`; evaluate the left inner addition first. |
| 14 | 135 | Push `O` above `R`; depth2. |
| 15 | 117 | Schedule `I`; evaluate variable0 first. |
| 16 | 135 | Push `I` above `O,R`; depth3. |
| 17 | 108 | Variable0 returns1. |
| 18 | 149 | Replace `I` with right-add frame holding1; evaluate `10*b`. |
| 19 | 117 | Schedule multiplication frame for10 and variable1. |
| 20 | 135 | Push `M` above `I,O,R`; depth4. |
| 21 | 102 | Return10. |
| 22 | 149 | Replace `M` with right-mul frame holding10; evaluate variable1. |
| 23 | 108 | Variable1 returns2. |
| 24 | 153 | Pop `M`; stage checked primitive20; depth3. |
| 25 | 223 | Return20. |
| 26 | 153 | Pop `I`; stage checked primitive21; depth2. |
| 27 | 223 | Return21. |
| 28 | 149 | Replace `O` with right-add frame holding21; evaluate `100*c`. |
| 29 | 117 | Schedule multiplication frame for100 and variable2. |
| 30 | 135 | Push `M` above `O,R`; depth3. |
| 31 | 102 | Return100. |
| 32 | 149 | Replace `M` with right-mul frame holding100; evaluate variable2. |
| 33 | 108 | Variable2 returns3. |
| 34 | 153 | Pop `M`; stage checked primitive300; depth2. |
| 35 | 223 | Return300. |
| 36 | 153 | Pop `O`; stage checked primitive321; depth1. |
| 37 | 223 | Return321; `R` still needs its charged return check. |
| 38 | 185 | Pop `R`, restore empty map, depth0; terminal return321. |

The 38 charges split into 12 to enter the body, 25 to evaluate four binary
operations and five leaf expressions, and 1 to restore the caller. Max depth 4 is
within stackLimit 8. There are 39 literal states and 40 budget rows. Budgets 0–37
publish exhaustion84; budget 37 preserves the pending return frame. Budget 38
publishes done321 by80 at remaining zero; budget 39 publishes the same value with
one remaining fuel. Terminal helper89 makes exhaustion84 inapplicable after the
last charged return.

## N08: named self-recursive factorial two

Function18 has parameter0 and body
`if n == 0 then 1 else n * call18(n - 1)`; entry calls it on2. Each recursive
lookup uses the same source function and table; source recursion is not unrolled.
The three body entries have maps `E2={0:2}`, `E1={0:1}`, `E0={0:0}`. Helper258
computes equality, helper255 with250 computes decrements2−1=1 and1−1=0, and
helpers256/250 compute1*1=1 and2*1=2. No helper charge is added to A.

`RE` saves the empty caller map, `R2` saves `E2`, `R1` saves `E1`.
`M2` and `M1` are pending right-multiplication frames holding2 and1. The exact
top-first stack at body entry64 is `[R1,M1,R2,M2,RE]`.

| State | Incoming rule | State reasoning |
| --- | --- | --- |
| 0 | startup74, free | Evaluate call18 on literal2 with empty map/stack and fuel82. |
| 1 | 131 | Schedule the initial argument frame. |
| 2 | 135 | Push arguments at depth1; evaluate literal2. |
| 3 | 102 | Return2. |
| 4 | 181 | Pop arguments; invoke18 on `[2]`; depth0. |
| 5 | 200 | Lookup function18. |
| 6 | 204 | Bind `E2`; active caller map remains empty. |
| 7 | 208 | Schedule `RE` and body under `E2`. |
| 8 | 135 | Push `RE`, activate `E2`; depth1. |
| 9 | 123 | Schedule conditional frame with both branches and `E2`. |
| 10 | 135 | Push conditional above `RE`; depth2; evaluate equality. |
| 11 | 117 | Schedule left-equality frame; variable0 first. |
| 12 | 135 | Push equality above conditional; depth3. |
| 13 | 108 | Read2. |
| 14 | 149 | Save left2; evaluate literal0. |
| 15 | 102 | Return0. |
| 16 | 153 | Pop equality; stage false; depth2. |
| 17 | 223 | Return false. |
| 18 | 165 | Pop conditional; select multiplication under `E2`; depth1. |
| 19 | 117 | Schedule multiplication with recursive call on the right. |
| 20 | 135 | Push left-multiplication frame; depth2; evaluate variable0. |
| 21 | 108 | Read2. |
| 22 | 149 | Replace frame with `M2`; evaluate recursive call. |
| 23 | 131 | Schedule argument frame saving `E2`; decrement expression next. |
| 24 | 135 | Push arguments above `M2,RE`; depth3. |
| 25 | 117 | Schedule subtraction; variable0 first. |
| 26 | 135 | Push subtraction; depth4. |
| 27 | 108 | Read2. |
| 28 | 149 | Hold left2; evaluate literal1. |
| 29 | 102 | Return1. |
| 30 | 153 | Pop subtraction; stage1; depth3. |
| 31 | 223 | Return1 to arguments. |
| 32 | 181 | Pop arguments; invoke18 on `[1]`; retain `M2,RE`; depth2. |
| 33 | 200 | Lookup the same source function18. |
| 34 | 204 | Bind `E1`; active map still `E2`. |
| 35 | 208 | Schedule `R2` and body under `E1`; active map still `E2`. |
| 36 | 135 | Push `R2`, activate `E1`; stack `R2,M2,RE`, depth3. |
| 37 | 123 | Schedule conditional saving `E1`. |
| 38 | 135 | Push conditional; depth4; evaluate equality. |
| 39 | 117 | Schedule equality; variable0 first. |
| 40 | 135 | Push equality; depth5. |
| 41 | 108 | Read1. |
| 42 | 149 | Hold left1; evaluate literal0. |
| 43 | 102 | Return0. |
| 44 | 153 | Pop equality; stage false; depth4. |
| 45 | 223 | Return false. |
| 46 | 165 | Pop conditional; select multiplication under `E1`; depth3. |
| 47 | 117 | Schedule multiplication with recursive call on the right. |
| 48 | 135 | Push left-multiplication frame; depth4. |
| 49 | 108 | Read1. |
| 50 | 149 | Replace frame with `M1`; evaluate recursive call. |
| 51 | 131 | Schedule argument frame saving `E1`; decrement next. |
| 52 | 135 | Push arguments; depth5. |
| 53 | 117 | Schedule subtraction; variable0 first. |
| 54 | 135 | Push subtraction; depth6. |
| 55 | 108 | Read1. |
| 56 | 149 | Hold left1; evaluate literal1. |
| 57 | 102 | Return1. |
| 58 | 153 | Pop subtraction; stage0; depth5. |
| 59 | 223 | Return0 to arguments. |
| 60 | 181 | Pop arguments; invoke18 on `[0]`; depth4. |
| 61 | 200 | Lookup the same source function18. |
| 62 | 204 | Bind `E0`; active map still `E1`. |
| 63 | 208 | Schedule `R1` and body under `E0`; active map still `E1`. |
| 64 | 135 | Push `R1`, activate `E0`; stack `R1,M1,R2,M2,RE`, depth5. |
| 65 | 123 | Schedule conditional saving `E0`. |
| 66 | 135 | Push conditional; depth6; evaluate equality. |
| 67 | 117 | Schedule equality; variable0 first. |
| 68 | 135 | Push equality; depth7. |
| 69 | 108 | Read0. |
| 70 | 149 | Hold left0; evaluate literal0. |
| 71 | 102 | Return0. |
| 72 | 153 | Pop equality; stage true; depth6. |
| 73 | 223 | Return true. |
| 74 | 161 | Pop conditional; select base literal1; depth5. |
| 75 | 102 | Return1 under `E0`, stack `R1,M1,R2,M2,RE`. |
| 76 | 185 | Type-check/pop `R1`, restore `E1`; stack `M1,R2,M2,RE`, depth4. |
| 77 | 153 | Pop `M1`; stage1*1=1 under `E1`; depth3. |
| 78 | 223 | Return1, stack `R2,M2,RE`. |
| 79 | 185 | Type-check/pop `R2`, restore `E2`; stack `M2,RE`, depth2. |
| 80 | 153 | Pop `M2`; stage2*1=2 under `E2`; depth1. |
| 81 | 223 | Return2, stack `RE`; its charged return check remains. |
| 82 | 185 | Type-check/pop `RE`, restore empty map, depth0; terminal return2. |

The hand count is 8 charges to the first body entry, 28 to the second body entry,
28 to the base body entry, 12 through the base return, 3 to complete the n=1
caller and 3 to complete the n=2 caller: 82 total. Max depth 7 is within
stackLimit 8. There are 83 literal states and 84 budget rows. Budgets 0–81 publish
exhaustion84 preserving their exact pending controls, maps and stacks. In
particular budget 75 retains all five return/multiplication frames; 76 restores
`E1` while preserving four frames; 79 restores `E2` while preserving two frames;
81 still has `RE`. Budget 82 publishes done2 by80 at remaining zero; 83 has the
same completed state with one remaining fuel.

## Evidence and remaining boundaries

Only serialization, static JSON parsing, source-text/locator checks, SHA256
calculation and diff inspection were performed. No behavioral test, Rust/Lean/K/
Quint execution, cargo/lake/SDK/model/provider command or native operation was
run. Serialization does not establish that any subject accepts this packet.
Independent Muse/Opus review and consumer integration belong to the integrator;
this note supplies no self-review verdict, security judgment or theorem.

The two complete A cases cover ordered three-argument accumulation, noncommutative
argument placement through asymmetric weights, checked normal sub/mul, structural
equality on UInt128, named self-recursion, and caller-return restoration with
pending arithmetic. N04 remains unchanged and continues to cover nested lexical
shadowing. Existing frozen fixture membership and every remaining normal coverage
row stay intact. These cases do not supply general ADTs, mutual recursion, full
grammar/fixture coverage, B helper/image/capacity freeze, Lean/Clean/circuit
correspondence, financial effects, native target or end-to-end evidence.

No source-rule ambiguity was encountered for these concrete examples. K Map
text order is presentation only; both packets preserve its mapping. General
canonical state serialization and B helper-work/capacity accounting remain
unresolved outside this packet. All U0–U7, M1–M17, MC/SP/G, ZR/MNR/OSR and retained
financial/composition exits remain open.
