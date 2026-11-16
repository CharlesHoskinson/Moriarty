# Verdict (lane run): X.3, hash stacks with the compound step (kit STACKS)

Status: LANE RUN, 2026-11-15, kit-stacks lane (Claude). The Carbon (Gemini) Quint lane is OWED: its prompt is `~/.cache/gemini-lanes/x3-hash-stacks/PROMPT.md`. The lead re-run is also OWED. Do not tick X.3 and do not call STK-005 met before both are done.

Model: `models/sherlock/quint/hashStacks.qnt` (quint 0.32.0). Commands: `commands.tsv` (49 lines, expected exit codes). Re-run: `rerun.sh OUTDIR`. Every quint call runs in heavy.slice through `heavy-run`. Results of this run: `rerun-results.txt`. Logs: `~/.cache/eui-kits/stacks/quint-run2/NN.log`.

## What the model checks

- The reference list function `refStep`: rewrite the top if there is one, then push, pop or none. Its source is Moriarty `owner_runtime.rs`, the image update at about lines 3609-3630. The validated owner step refuses a rewrite or a pop on an empty stack ("frame existence" and "task existence", about lines 2957-2965). That refusal is a CORE and DEC guard. STACKS follows STK-002: on an empty stack a rewrite or a pop leaves the list unchanged.
- The circuit acceptance predicate `cands` of one STACKS row, after `StacksAir::eval` (openvm `kit-stacks` d007862, `crates/sherlock-leaf/src/stacks.rs`). It is an abstraction of the AIR. The limb adder with signed carries is an integer add with a no-wrap bound. The RANGE checks, including the second check of the restored lanes, and the port messages are not modelled. The prover chooses the witness: the post cells, the hash enable `hs` and the opened preimage `lo`. Each named mutant drops the one conjunct that its Rust weak bit drops.
- The abstraction `absCached` (`decode`): the list is `chainD^-1(below) ++ [top]`, with the depth and the domain checked at every level, and the walk must end at `EMPTY`.
- The hash is a random oracle: injective, with ids starting at 1. `ZERO` = 0, `EMPTY_F` = -1 and `EMPTY_T` = -2 are not ids.
- Two modes. In the genesis mode (`ADV = false`), both stacks start empty, CORE asks any request and the STACKS prover is adversarial. In the forged-pre mode (`ADV = true`), the pre cells are any cells, and the oracle is seeded with a wrong-depth entry (id 4, depth 3 below `EMPTY`) and a task-domain entry (id 3). The forged-pre mode checks the port certificate of one accepted row.
- Bounds: records {1, 2} (0 is the empty top); count modulo 4 (0..3), and the no-wrap check refuses count + 1 = 4; 2 stacks; `--max-samples 20000 --max-steps 10 --seed 0x20261115`. Apalache bounded check (`quint verify`): genesis mode 3 steps, forged-pre mode 2 steps.

Invariants:
- `cachedTopRefinesList` (gate): every stack whose list is defined decodes to the reference list.
- `compoundStepIsOneEvent` (gate): the last accepted row equals ONE application of `refStep` to the pre list, and the other stack is unchanged. This is the per-step form of the refinement. The other-stack conjunct holds by construction, because a row touches one stack.
- `belowEmptyAtCountLe1` (gate): count 0 gives a zero top and `below = EMPTY`, and count 1 gives `below = EMPTY`. It is checked on the pre and post cells of the last accepted row, and in genesis mode on every stack. It is one direction only: no gate gives `below = EMPTY -> count <= 1`.
- `restoreReturnsSpilled` (STK-003): a pop with a hash returns exactly the preimage of the old `below`, with the domain of the stack and with depth = post count.

## Results (run 2, after the decode fold was given a constant bound for Apalache)

All 49 lines of `commands.tsv` match their expected exit codes. The sound model holds all four invariants in genesis mode and in forged-pre mode (cmds 2-9; 20,000 samples x 10 steps each). Apalache bounded check: `allInvariants` holds on `hashStacks_sound` at 3 steps (cmd 46) and on `hashStacks_sound_adv` at 2 steps (cmd 47). Apalache finds the `m01` violation at 3 steps (cmd 48), so the bounded check is not vacuous.

| # | Mutant (weak bit) | Mode | Invariant violated (cmd) | Attack (accepted witness) | Honest compound rw+push still accepted (cmd) |
|---|---|---|---|---|---|
| 1 | `stack_compound_spills_unrewritten_top` (PUSH_LO_TOP) | genesis | `compoundStepIsOneEvent` (17); also `cachedTopRefinesList` | rw+push at count >= 1 spills the old top: the cursor update is lost | yes (18) |
| 2 | `stack_compound_child_not_installed` (CHILD) | genesis | `compoundStepIsOneEvent` (19) | rw+push keeps the rewritten parent as the top | yes (20) |
| 3 | `stack_compound_count_plus_two` (COUNT_DELTA) | genesis | `compoundStepIsOneEvent` (21); also `belowEmptyAtCountLe1` | the count does not follow the adder | yes (22) |
| 4 | `stack_count_one_below_nonempty` (COUNT1_BELOW) | forged pre | `belowEmptyAtCountLe1` (23) | a row accepted on pre cells with count 1 and `below != EMPTY` | yes (24) |
| 5 | `stack_empty_nonzero_top` (EMPTY_TOP) | forged pre | `belowEmptyAtCountLe1` (25) | count 0 with a non-zero top accepted | yes (26) |
| 6 | `stack_empty_below_not_empty_const` (EMPTY_BELOW) | forged pre | `belowEmptyAtCountLe1` (27) | count 0 with `below = 0` (or another digest) accepted | yes (28) |
| 7 | `stack_predecessor_forged` (POP_LO_BELOW) | genesis | `cachedTopRefinesList` (29); also `restoreReturnsSpilled` | a pop installs a below that is not the opened predecessor | yes (30) |
| 8 | `stack_restored_record_forged` (POP_LO_TOP) | genesis | `cachedTopRefinesList` (31); also `restoreReturnsSpilled` | a pop installs a top that is not the opened record | yes (32) |
| 9 | `stack_count_unchecked` (POP_LO_CNT) | forged pre | `belowEmptyAtCountLe1` (49); also `restoreReturnsSpilled` (33) | FRAMES pre count 2 with `below` = id 2 (spilled at depth 2): the pop restores at depth 1 and leaves count 1 with `below` = id 1, not `EMPTY` | yes (34) |
| 10 | `stack_frame_task_domain_swapped` (DOMAIN_COL) | genesis | `cachedTopRefinesList` (35) | a FRAMES push spills with the domain DOM_TASKS (38), seen in the log as `lo: {dom: 38, ..}` on stack F | yes (36) |
| 11 | `stack_rewrite_spills` (KEEP_BELOW) | genesis | `cachedTopRefinesList` (37) | a row with no hash changes `below` | yes (38) |
| 12 | `stack_push_without_spill` (HS_DEF) | genesis | `cachedTopRefinesList` (39) | a push at count >= 1 with `hs = 0`: the old top is lost | yes (40) |
| 13 | `stack_count_wrap` (NO_WRAP) | forged pre, and genesis (`m13g`) | `belowEmptyAtCountLe1` (41); genesis `cachedTopRefinesList` (43) | a push at count 3 (the u64 maximum in the model) wraps to 0 with a non-zero top | yes (42, 44) |

13 of 13 named mutants are caught, each by at least one of the three gate invariants, with a tampered witness that the mutant accepts. Each mutant module still reaches a rewrite-then-push at count 1, so the mutant does not disable that step. In the forged-pre modules (4, 5, 6, 9, 13) the run may start from forged cells, so these runs show that the step is enabled, not an honest witness from genesis.

Reachability witnesses on the sound model (each must be violated, and each is): rewrite-then-push at count 1 (10) and at count 2 (11), rewrite-then-pop at count 2 (12), rewrite at count 0 (13), pop at count 0 (14), count 0 -> 1 -> 0 with no hash (15), and the child's pop restoring the rewritten parent (16).

## Findings

1. **`stack_count_unchecked` is an equivalent mutant in the genesis mode** (cmd 45: `m09g` holds all four invariants). On a reachable pre, two things bind the restored depth: the count adder fixes post count = count - 1, and the injective oracle gives the unique preimage of `below`, whose depth was written at the spill. The dropped conjunct `lo.d = post.count` is redundant there. It matters only for pre cells that CORE or a forged boundary supplies (forged-pre mode, cmd 33), which is the form of the Rust mutant (pre count 4 with `below = H(2, ..)`). The depth word is defence in depth at the port. Under honest stitching it is not needed for refinement.
2. The pre-shape mutants (4, 5, 6) cannot be reached from genesis, for the same reason: every reachable pre already has the shape. They are port-certificate gates and are caught in the forged-pre mode.
3. The domain swap (10) is caught because the abstraction is indexed by domain: a frame chain value under DOM_TASKS does not decode as a FRAMES suffix. A cross-stack confusion in reachable states also needs CORE to move a value between the stacks. The Carbon lane is asked to judge whether this check is a free win.
4. Apalache: the first run (run 1) failed with "Expected a constant integer expression" at `decode`, because the fold was over `range(0, count - 1)`. The fold now runs over `range(0, CMOD - 1)` with a guard. Simulation results are the same before and after.

## Lean (tasks.md gate: O11 and O46)

- O11: `clean/fork/Clean/Sherlock/HashStacks.lean` (namespace `Clean.Sherlock.HashStacks`), with the axiom report in `HashStacksAxioms.lean`. It proves `step_refines`: an accepted row (the `StacksAir` constraints over `Nat`) maps a pre state that abstracts `L` to a post state that abstracts `refStep q L`, with the compound step. It also proves `restore_returns_spilled` and `abs_count_one`. The only premise is that `hc` (depth, below, top) is injective; this direction does not need `EMPTY` outside the range.
- O46: `clean/fork/Clean/Sherlock/PartitionCompound.lean` extends `PartitionIndependence.lean` (C3) and has its axiom report in `PartitionCompoundAxioms.lean`. `CStackOp` is the compound op (an optional rewrite of the top, then a `StackOp`), and `CEffect` carries one compound op per stack. The file proves `leaf_tail_commutesC`, `leaf_flush_commutesC`, `partition_independentC`, `partition_independent_flushC` and `cuts_agreeC`. `next_ofEffect` shows that the single-op machine of P7 embeds with no rewrite. `compound_rw_push`, `compound_rw_pop` and `compound_empty` show that the compound op is the reference list function.
- Both modules are built with `lake build Clean.Sherlock.HashStacksAxioms Clean.Sherlock.PartitionCompoundAxioms` through heavy-slot, with Lean v4.33.1. Every theorem reports at most `propext`, `Classical.choice` and `Quot.sound`; there is no `sorryAx`. The log is `~/.cache/eui-kits/stacks/lake1.log`.
