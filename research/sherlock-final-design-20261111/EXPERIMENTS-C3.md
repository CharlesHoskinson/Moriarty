# Experiment C3

## C3 Lean

Two new modules in `clean/fork/Clean/Sherlock/`, each with an axiom-report companion. `Aggregation.lean` and `AggregationAxioms.lean` are unchanged. The new modules import `Aggregation` and reuse `Run`, `Run.trans`, `Run.unique`, `Premises`, `Decided`, `AcceptRun` and `result_unique`.

| File | Content |
|---|---|
| `PartitionIndependence.lean` | Dense tail-buffered memory, the reference machine, the leaf, partitions, the `det` bridge and the D3 counterexample |
| `PartitionIndependenceAxioms.lean` | `#print axioms` for all 41 theorems |
| `ModularComposition.lean` | Modules, bus contracts and the composition theorem, with a toy CORE + ALU instance |
| `ModularCompositionAxioms.lean` | `#print axioms` for all 11 theorems |

Status: all 52 theorems are proved. There is no `sorry` and no `axiom`. Every axiom report contains only `propext`, `Quot.sound` and `Classical.choice`, or no axiom at all. No statement was left open, so no obligations file was created. `lake build SherlockSpec` is green (27 jobs, through `scripts/doctorwatson heavy --id c3-lake`). `scripts/qa/check-lean-aggregation.sh` (F21) is still green with 90 theorems. The library imports only `Init`, with no Mathlib.

### 1. Partition independence (SRC-023)

The model has two levels:
- Logical state `LState`: memory `List C` (append-only), `frames`, `tasks` (hash-chain stacks, abstracted as lists) and the registers.
- Reference step: `LState.next eff` applies the step's `Effect`, which holds the real appends, one `StackOp` per stack and the new registers. `len` is the memory length, so only real appends advance it.

Physical memory `Mem` holds two parts: the flushed full 8-cell segments, which are the content of the tree under `mem_root`, and the tail, `tail_d` with `r < 8`. A leaf opens a window, buffers the appends of its steps and then closes with one of three rules:
- `closeTail`: the per-cell flush rule.
- `closeFlush`: the circuit rule, at most one flush per leaf, which happens iff `r_pre + used >= 8`.
- `closePad`: the D3 rule from `m3.rs`, gate `m3 walk first`, `len_post = base + 8`.

Main theorems:

| Theorem | Statement |
|---|---|
| `partition_independent` | For every cut `ks` of the steps into leaves, the physical state after the leaves is `abs (next^[ks.sum] s)`. This uses the per-cell rule and has no side condition. |
| `partition_independent_flush` | The same theorem for the circuit rule, for every cut in which each leaf holds at most 8 appends (`FitsLeaf`). |
| `cuts_agree` | Two valid cuts (leaves of `1..S` steps) with the same sum give the same physical state, and so the same digest under any digest function. |
| `leaf_tail_commutes`, `leaf_flush_commutes` | One leaf maps `abs s` to `abs (next^[k] s)`. |
| `leafFlush_eq` | The one-flush circuit rule equals the per-cell rule while `r_pre + used < 16`. |
| `pushCell_flush_iff`, `leafFlush_iff` | A segment is flushed iff `floor(len / 8)` grows. |
| `appendCells_len`, `canon_len` | `len` advances exactly by the real appends. |
| `Mem.wf_unique` | A well-formed `(segs, tail)` is determined by its cells. This is the canonical `tail_d` of audit finding 8. |
| `chainHash_inj` | The list abstraction of a hash-chain stack is faithful if `H` is injective and `EMPTY` is not in its range. Both conditions are collision premises (finding 8). |
| `ones_fit` | If every step appends at most 8 cells, the cut into single steps satisfies `FitsLeaf`. |
| `runLogical_eq` | Logical partition independence. This is trivial because the step is a function. |

Connection to `Aggregation.lean`. `det` is not a field of `Premises`. It is a hypothesis of `result_unique` and `statement_independent_of_S`.

| Theorem | Statement |
|---|---|
| `pstep_det` | The physical step relation `PStep` (one reference step between canonical physical states) is deterministic. It follows from two facts: the logical step is a function, and `abs` is injective (`abs_inj`). |
| `leaf_run` | A k-step leaf under the circuit rule is a `Run (PStep eff) k`. This is the shape of the `leaf_sound` conclusion. |
| `premises_det` | The `det` premise `∀ p s t u, P.Step p s t → P.Step p s u → t = u` holds for any `Premises` whose states represent `PState (Mem C)` injectively and whose `Step p` is `PStep (effOf p)`. |
| `result_unique_tail` | `result_unique` with `det` discharged by `premises_det`. |

D3 (audit finding 10):

| Theorem | Statement |
|---|---|
| `d3_partition_dependent` | On a trace that appends one cell per step, the cuts `[2]` and `[1, 1]` give different physical memories: `len` is 8 against 16. Decided by the kernel. |
| `tail_det_run` | The positive twin: for every effect function, `PStep` is deterministic, and every circuit-rule leaf from a reachable state is a `Run` of exactly its number of steps. |
| `runPart_append` | Running the cut `ks ++ ks'` is running `ks` and then `ks'`. |
| `tail_same_example` | The same trace gives one state for both cuts under the tail rule. |
| `d3_no_det_run` | No deterministic step relation on the padded physical state makes every padded leaf a `Run` of exactly its number of steps, even when only reachable states are considered: a pre-state reached by honest padded leaves from genesis, with its consistent witness. So under D3, `det` and the `Run` form of `leaf_sound` cannot both hold at the physical level. D3 does not falsify `statement_independent_of_S`; it makes that theorem's premise impossible to discharge. |

Still assumed, as hypotheses and not as axioms:
- `digest_inj`: collision resistance, which lifts equal physical states to equal `post_d` and back.
- `stuck`: halted states are terminal. It stays a hypothesis of `result_unique_tail`.
- The leaf circuit's soundness (`leaf_sound`). Its decomposition is part 2 below, plus each module's contract.
- At most 8 appends per leaf (`FitsLeaf`). This is the per-rule maximum of audit finding 6.
- The address bound `len < 2^64` (audit finding 3). The lists in this model are unbounded.
- The rest of the machine must be write-before-read. The model has no scratch registers outside the state (audit finding 9).

### 2. Modular composition

The model:
- A `Module W Tup` has a local `rel`, the tuples it `requests` and the tuples it `provides`, all over one leaf witness `W`.
- `LeafRel` means that every module's `rel` holds and the bus is lookup-sound (`LookupSound`: every requested tuple is provided).
- `BusBalanced` is the multiset balance (`List.Perm`) that a LogUp check establishes.
- `Contract Valid level` has two parts. `guarantee`: valid requests and `rel` imply that every provided tuple is valid. `ordered`: every requested tuple has a lower level than every provided tuple, so the module DAG has no cycle.

| Theorem | Statement |
|---|---|
| `balanced_lookup` | `BusBalanced → LookupSound`. |
| `provided_valid`, `requested_valid` | If every module meets its contract, every tuple on the bus of an accepted leaf is valid. The proof is by induction on the level. |
| `leaf_refines` | If, in addition, CORE's contract gives `Goal` from valid requests, `LeafRel` implies `Goal`. |
| `leaf_refines_balanced` | The same theorem with `BusBalanced` as the bus hypothesis. |
| `leaf_refines_step` | If CORE's decoded rows follow the reference step function, an accepted leaf is a `Run` of exactly `rows.length` reference steps, from the decoded pre-state to the decoded post-state. |
| `follows_run`, `fun_det` | Rows that follow a step function form a `Run`, and the functional step relation is deterministic. |
| `toy_contracts`, `toy_refines` | A non-vacuous instance: CORE does `x ↦ x + 3` through an ALU add request. |
| `toy_needs_bus` | Without the bus hypothesis, the module relations alone admit `post = pre + 4`. |

Each hypothesis is a fact about one module, so a module kit discharges its own `Contract` and does not need the other modules' relations.

Still assumed:
- LogUp soundness, that is, `BusBalanced` from the fingerprint equation with the Schwartz-Zippel error. This is a proof-system premise and is not stated, because the library has no field or polynomial theory.
- The concrete contracts of the Sherlock modules (CORE over the 108 rules, MEM, ALU, RANGE, HASH, VALUE DIGEST). The module kits must instantiate them.

ALU and RANGE are small enough to write directly in Clean as `Module` instances, with `Valid` set to their arithmetic meaning, in the way `toyAlu` is written.
