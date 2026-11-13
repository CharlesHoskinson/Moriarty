# X7: binary node shapes

Date: 2026-11-12. Loop item X-EXP of `research/sherlock-e-path-20261112/LOOP.md`. Branch `x-exp` (from `final-design-2` at `efb0eb78a`). Experiment definition: `research/sherlock-final-design-20261111/ARCHITECTURE.md` section 5.4, row X7: "leaf key direct against a one-leaf lift (N-1); the MERGE-WORK node, two enabled prior proofs plus one real owner step with its memory, stack and value-digest work in one node (N-6; two successive nodes are not this measurement); decider with and without a closing proof (N-5)". It gates N-1, N-5, N-6 (OWNER), N-13, the GP-4 hook and I-13.

Labels: MEASURED is read from `out/x7-nn.jsonl` and from `../sherlock-x10-20261112/out/x10-run.jsonl` (shared machine, no `--exclusive`, so the times are not timings). DERIVED is computed from MEASURED numbers by a stated rule. READ is a code or text reading.

## Result

All shapes are STARK nodes at the OD-19 internal set (`internal_ud()`, the s10x parameters), built with `VerifierSubCircuit<N>` of OpenVM. Leaves are W1 app leaves of 2^16 rows. Bounds of the internal set (ARCHITECTURE line 2265): height at most 2^19, 1,000 constraints and 100 interactions per AIR, 50 AIRs, 2,000 columns.

| Node | Children | AIRs | Sum of AIR widths | Max constraints / interactions per AIR | Tallest AIR | Stacked width | Main cells | Proved and verified | Label |
|---|---|---:|---:|---:|---|---:|---:|---|---|
| leaf direct, arity 2 | 2 app leaves | 39 | 1,543 | 282 / 77 | 2^18 | 69 | 35,334,472 | yes | MEASURED (X10 run) |
| one-leaf lift, arity 1 | 1 app leaf | 39 | 1,531 | 282 / 77 | 2^17 | 35 | 17,692,136 | yes | MEASURED |
| node over lift, arity 1 | 1 lift node | 39 | 1,537 | 282 / 77 | 2^16 | 34 | 17,296,624 | yes | MEASURED |
| node over lifts, arity 2 | 2 lift nodes | 39 | 1,549 | 282 / 77 | 2^17 | 66 | 34,199,384 | yes | MEASURED |

The tallest AIR is `FinalPolyQueryEvalAir` in every row. It needs 2^17 rows per app-leaf child (117,800 valid rows, as s10x measured) and 2^16 rows per internal-set node child (DERIVED from the heights). So a node child costs about half of a leaf child in the tallest AIR.

### N-1: leaf key direct (a) or one-leaf lift (b)

- Both shapes fit the internal set. Leaf direct at arity 2: 2^18 of 2^19 rows, 39 of 50 AIRs, 1,543 of 2,000 columns. Lift path: the arity-2 node over two lifts is smaller (2^17, 1,549 columns), but each leaf needs one extra arity-1 lift proof (2^17, 1,531 columns). MEASURED.
- The written rule of N-1 is "(b) if the node measurement shows the leaf names push (a) outside the shape bounds of the internal set; else (a), which has one relation fewer". These rows show (a) inside the bounds for a node whose slots all hold leaves. This supports (a). It does not decide N-1, for this reason:
- Not measured: a node slot that accepts **either** a leaf key or a node key. `VerifierSubCircuit<N>` verifies children of one fixed child key, so a leaf-or-node slot (the "fixed-base name union" of (a)) needs a multi-key verifier that this bench does not have (READ, `VerifierSubCircuit::new(Arc<child_vk>, ..)`). Under (a), the union cost is the open part. Under (b), every slot holds a node-kind key, but lift and node are still two keys unless the lift is the same circuit as the node. The key-root inclusion check is PENDING (frontend M2a, ARCHITECTURE line 2265) and applies to both options.

### N-6: MERGE-WORK, two enabled prior proofs plus one real owner step (OWNER)

- **Not measurable today.** The owner step needs the BabyBear kits CORE, DEC, ALU, RANGE, HASH, STACKS, MEM, the memory backend, VALUE DIGEST and INIT, built with their `configure` and `assign` functions inside the node (ARCHITECTURE line 2622). No BabyBear kit exists: CORE is a halo2 32-word spike. A stand-in AIR set would not be N-6 evidence (ARCHITECTURE N-6: "two successive nodes are not this measurement"; a stand-in is not "one real owner step").
- **The residual budget for the inline step (DERIVED from the arity-2 node over two node children, the MERGE-WORK case of two prior proofs):** 50 - 39 = **11 AIRs**, 2,000 - 1,549 = **451 columns** in total, each added AIR at most 2^19 rows (the children use at most 2^17), 1,000 constraints and 100 interactions per AIR. If one prior proof is a leaf, the tallest AIR goes to 2^18 (DERIVED: 2^17 + 2^16 valid rows, padded), and the other figures stay about the same.
- An owner step must fit in 11 AIRs and 451 columns. The leaf menu reaches stacked width 336 (LD-S10x-5). That is a stacked width at the app set, not a sum of AIR widths, so the two numbers do not compare directly. Whether one step fits is open until the kits exist.

Options for the owner (N-6 carries an OWNER mark):

1. **(a) as written, frozen as the target, measured when the kits exist.** MERGE-WORK stays the inline-work row of the node relation table. N-6 stays FREEZE-PENDING on the first BabyBear build of the leaf kits, with the residual above (11 AIRs, 451 columns, 2^19 rows) as the budget the kits must meet. GP-4 is not narrowed. Risk: if the kits need more than 11 AIRs or 451 columns for one step, the node shape bounds (50 AIRs, 2,000 columns, own choice of the internal set) must move, which changes the universal key set.
2. **(a) with a raised internal-set column and AIR budget decided now.** The owner fixes a larger node budget for MERGE-WORK (for example 64 AIRs and 3,000 columns) before the kits exist, so the kits get a known target. Cost: the bounds are an internal choice (ARCHITECTURE section 1.9). Raising them changes every node key, so this must happen before FP-8 freezes the key set. No measurement shows it is needed yet.
3. **New work as a leaf child of an arity-3 node.** The node has two prior-proof slots and one leaf slot for the new step: GP-4 "arity >= 2 plus new work" with the new work proved by the leaf circuit and verified in the same node. The node arity limit is 4 (LD-S10x-2), so arity 3 fits that rule. Cost: this is the closed option (c) of N-6 (a three-slot node, closed under halo2 k = 21), now under STARK bounds. It needs a mixed leaf-and-node slot (the N-1 open part) and an owner ruling that a leaf-proved step counts as "new work in the node". It is not option (b): one node, not two successive nodes.

Recommendation: **option 1**. It keeps the written N-6 (a) and the one-encoding rule (the inline kernel from the leaf kits' `configure` and `assign`) and narrows nothing. The measured residual gives the kits a concrete budget. Option 3 stays a fallback if the first kit build does not fit, and it reopens a closed option, so it needs the owner.

### N-5: decider with and without a closing proof

Not measured. Under OD-24 option E, `L` and the wrap are the BLS12-381 side (`L` opens the sealed SET head, ARCHITECTURE section 1.11, FP-3). This bench has no `L` over the multi-key node. N-5 (a) (the decider resolves the node bases directly) stays the recommendation by default; it decides nothing until `L` over the multi-key node is measured (the m7-wrap worktree is the place for it).

### N-13 and I-13

- N-13 (staged or unified key set): these rows show that all node shapes of a unified set (leaf direct or lift, node) fit the internal set at arity 1 and 2. The foreign lift and the closing proof are not measured (X8, N-5). N-13 (a) in code structure is supported, and not decided.
- I-13 (74-word node envelope) and the GP-4 hook: not changed by this experiment. The envelope's public values are a few dozen words in one AIR, inside the bounds (DERIVED). They stay FREEZE-PENDING on N-6 and X8.

## Method

- Code: `../sherlock-x10-20261112/src/x_exp_node.rs`, mode `nn`. One W1 leaf proof, an arity-1 node over it (the lift), then nodes of arity 1 and 2 over copies of the lift proof. Keygen, trace generation, prove and verify for each; `shape()` reads AIR widths, constraints, interactions and the stacked layout from the proving key.
- Run: `x_exp_node nn 16 1 1,2` under `heavy-slot --mem 40G`, rc 0, 87 s. The leaf-direct row is the `node_base` record of the X10 run (`x_exp_node x10 16 2`).

## Files

- `out/x7-nn.jsonl`: the leaf, the lift node, the nodes over lifts at arity 1 and 2, with per-AIR shapes and height histograms.
- `out/x7-nn.err`: run log.
