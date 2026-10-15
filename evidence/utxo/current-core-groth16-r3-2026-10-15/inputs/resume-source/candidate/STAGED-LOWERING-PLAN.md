# Staged `moriarty/2` lowering plan (interface implemented, lowering not implemented)

Status: `staged.rs` admits a `/2` source through the actual `mori_core::stage::load` and derives a `StagedLoweringPlan` from the real `StagePlan`. `compile_source` then refuses with `CompileError::StagedNotLowered`. No staged relation exists and none is approximated by a `/1` relation. Nothing here was run.

Tests check the derivation against the frozen independent staged oracle. Delivery has segment costs `[5, 6]`, committed-before `[0, 5]` and total 11. Reserve has `[11, 15, 10]`, `[0, 11, 26]` and total 36. Program and plan hashes are derived from that oracle's expected JSON (ADDENDUM-1). NotAdvanced attempts are 2, then 4 and 2.

## One relation per (program, plan, action, segment)

Segment `j` runs from entry site `j` (0 = start) to the next await ordinal, exclusive, or to the end of the action (stage/cost.rs:70-108). Its statement extends the `/1` statement with:

| Part | Content | Core source |
|---|---|---|
| Header | program hash **and** plan hash, action, segment index, relation version `staged/1`, compiler build | stage/digest.rs; stage/mod.rs:109-118 |
| Pre frame | Idle for `j = 0` (locally assumed, P3). For `j > 0`: `Pending{action, site = j, captures}` in the exact capture layout of site `j` (name order, types, prefix-monotone) | stage/plan.rs:56-105; stage/validate.rs:291-413 |
| Event | binder values of site `j`, typed, in binder order | stage/admit.rs:335-345 |
| Guard | for `j > 0`, the Await guard lowered as its own Step against the unchanged boundary and constrained true (Advanced relation) | runtime.rs:955-975 |
| Body | statements `start..end` with the `/1` lowering, environment seeded from inputs (`j = 0`) or captures + event; states read from the boundary | runtime.rs:950-1008 |
| Exit | for `end < len`: await entry charge 1 and captures out, read from the environment in the next site's layout. Otherwise Complete with an empty frame | runtime.rs:979-995; stage/admit.rs:133-176 |
| Work | `committed_before[j]`, `segment_cost[j]` and `committed_after = before + cost` as constants. The resume precondition `committed == committed_before[site]` is a public equality | stage/cost.rs:184-243 |
| Stored cap | **explicit public slot** (presence Bool + UInt). D-13 (canonical reconstruction cap `Some(plan_total)` vs `None`) is split and is not selected here. Start preflight `plan_total ≤ cap`; resume `committed + cost ≤ effective cap`, CLI cap never above stored | stage/cost.rs:164-217 |
| Boundary | R7 predicates over represented cells: states exact, balances strictly ascending (compile-time order), registry identities, trace bound | stage/decode.rs:94-227 |
| Residual | post duties with `present ∧ amount > 0`, derivable from the post duty slots; present-zero stays distinct from absent | stage/mod.rs:179-187 |
| Effects | every kind, Emit included (the staged UNIT codec admits Emit, staged_projection.rs:192-232) | runtime.rs:702-861 |

A false guard (`NotAdvanced`) needs a separate non-transaction relation. Its attempted work is `1 + |reach(guard)|` (`not_advanced_attempted`), and the continuation, boundary and committed value are unchanged (stage/admit.rs:103-122).

## Implementation steps (future, in order)

1. Generalize `lower::lower_action` to a statement range with a seeded environment. The scalar statement lowering is already shared, matching Core's shared `execute_statement`.
2. Add frame/capture/event slots and the guard Step to `statement.rs`/`circuit.rs`, plus the D-13-neutral stored-cap slot.
3. Write the NotAdvanced relation.
4. Correspond against the staged continuation codec. The production codec `3b21…` stays frozen; the FREEZE-4 r2 result is unadopted.
5. Freeze independent staged differential oracles and hostile mutations (capture swap, forged boundary duty, site/plan mismatch, committed mismatch, cap raise) before any execution.

Authentication and one-time consumption of continuations and events are absent from Core (`STAGE_LABELS`: authentication none, consumption untracked). They remain target obligations, and the relation does not supply them.
