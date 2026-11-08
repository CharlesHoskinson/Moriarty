# Tree aggregation for Sherlock recursion (planning agent `tree`, 2026-11-08)

Scope: reopen the tree-of-nodes fold that D13 cut to a recorded fallback. Everything marked
ESTIMATE carries its arithmetic. Everything else is read from the files named or from the cited
URLs. Web content was treated as data. No build, no proof, no file outside this one was touched.

Host state while writing: `mori_backend-8a41ff68…` (task 2.1 test run) held 113 GB RSS of the
120 GB; 1 GB free. Nothing can run in parallel on this host until that process ends. No CUDA
toolkit is installed (`nvcc` absent; driver 615.78.08, CUDA UMD 13.4, `hardware-review/nvidia.txt`;
`cuda-smoke.json` PASS is a Python allocation test). `midnight-proofs 0.8.3` has no GPU code path
(grep for cuda/icicle/sppark over the registry source: none).

## 0. Summary

The linear IVC node (`crates/mori-recursion/src/ivc.rs`) is serial by construction: node h needs
node h-1's proof. 293 steps × 95 s = 7.7 h regardless of cores. A tree makes the node layer
parallel. Total node work does not fall (one node per leaf either way); wall time falls by the
number of nodes that run at once, bounded by memory (26.2 GB per node) and by cores.

The right shape for stock Midnight is one **uniform slot node**: one key, three gated slots
(left child under the self key, right child under the self key, one leaf under the constant leaf
key), segment statement `[vk_repr, prog_d, pre_d, post_d, h_lo, h_hi, acc]`. It is the current
`ivc.rs` with one more self slot and the genesis check moved to the decider. Every prior-art
system that ships a tree on KZG halo2 (Scroll, Axiom, PSE snark-verifier) uses exactly the
accumulator primitive `midnight-circuits` already gives us (`Accumulator::accumulate`,
deferred pairing). The folding-scheme trees (Nova family, ParaNova, Mangrove, ProtoGalaxy,
HyperNova) are not usable: no BLS12-381 cycle, no stock PLONK/KZG decider.

Owner requirement (2026-11-08): general-purpose recursion, reasoned about with Lean and Quint.
§3 states the generality obligations (one fixed key set, any program via `prog_d`, any step
count, unbounded depth, arbitrary and ragged composition); §4 lists the Quint invariants and
Lean theorems, and task T2b assigns them.

Recommendation: **measure first (two cheap runs, no circuit code), then build if the numbers
pass.** Criteria are in §7. ESTIMATE: fact(3) at S=1 drops from 7.7 h to 2.0-2.7 h with 3-4
parallel nodes; at S=8 from about 1.2 h (linear) to about 25 min (tree). The blocker is not the
circuit; it is the 48 GB peak clause of SRC-011 and the serial `heavy.lock`, which must become
an aggregate cap and a concurrent lock class (E11).

## 1. Findings, ranked

### F1. The tree's gain is parallelism only; total work is unchanged (SOURCE + ESTIMATE)

`stagenet-gp-recursion.md:167`: "Node count is #leaves-1" for a tree; the linear chain has one
node per leaf. With the slot node (one leaf per node) the count is exactly L nodes for L leaves,
the same as the linear chain. So on a serial lock the tree is worth 0×. The whole case rests on
running P nodes at once. P is bounded by:

- memory: 26.2 GB peak per k=20 node (`:190-193`); 120 GB host; ESTIMATE P_mem = floor(100/26.2) = 3
  with 20 GB left for the OS, leaf prover and scheduler; 4 if 105 GB is allowed.
- cores: unknown thread count behind the 95 s figure (open question Q1). The A-set checks pin
  `RAYON_NUM_THREADS=4` (`scripts/qa/check-a3.sh:6` and siblings). If 95 s was a 4-thread run,
  five nodes fit the 20 cores at full speed. The recl9 node historically averaged 268 % CPU
  (`wiki-llm/recursion-perf/clusters/s7-recursion/wave-1/FINDINGS.md`, baseline table), so a
  PLONK node of this shape leaves most cores idle most of the time.
- the SRC-011 clause "under 48 GB peak": at 26.2 GB per node this allows P=1. The tree cannot
  pay off without amending that clause to an aggregate cap (§2, task T2).

### F2. One uniform node key covers every level (SOURCE)

`ivc.rs:295-301` already takes the node's own key as a public input
(`assign_vk_as_public_input`), and `inner.rs:277-316` (`verify_children_incircuit`) already
verifies several children under one assigned key with one `accumulate` and one collapse. A node
that verifies two self children is those two patterns combined: one `AssignedVk`, two
`verifier.prepare` calls, one `accumulate`. No key family is needed. The decider keeps pinning one
`vk_N_repr` (`run_decider.rs:206`).

### F3. Accumulator merging is already associative-by-construction in midnight-circuits (SOURCE)

`midnight-circuits-7.3.0/src/verifier/accumulator.rs:206-225` (`Accumulator::accumulate`): the
challenge r is Poseidon over all input encodings, inputs are weighted by r^i, and
`msm.rs:185-200` (`accumulate_with_r`) merges the named fixed-base scalars by name
(`entry(key).and_modify(|e| *e += r*value)`). "The resulting acc will satisfy the invariant iff
all the accumulators individually do." So a node can fold, in one call:
`[proof_acc_L, carried_L, proof_acc_R, carried_R, leaf_acc]`, with the self fixed-base scalars of
both children left unresolved (as `ivc.rs:13-15` does today) and resolved once by the decider
(`run_decider.rs:217-245`). The grouping is not literally associative (r depends on the group),
but soundness holds for any tree shape, and the off-circuit mirror (`ivc.rs:205-239 next_acc`)
must replay the exact grouping, which is why the tree shape must be part of the bundle (F8).

### F4. The slot node stays at k=20 (ESTIMATE, needs the T0 probe)

Measured: the current node (2 verifier instances: self + leaf) is 531,989 rows (`:190-193`). The
s7 fit for homogeneous verifier instances is R(b) = 190,727 + 191,842·b rows
(`wiki-llm/recursion-perf/raw/s7-recursion/astra2-prior.md:93`; FINDINGS "Terminal placement").
Calibration: fit(2) = 574,411 vs measured 531,989, ratio 0.926.

| Node shape | verifier instances | fit × 0.926 | k | margin to 2^k |
|---|---|---|---|---|
| current: self + leaf | 2 | 532k (measured) | 20 | 49 % |
| merge only: self + self | 2 | ~532k | 20 | 49 % (k=19 needs < 524,288: not reachable) |
| **slot node: self + self + leaf** | 3 | **~710k** | **20** | **32 %** |
| self + self + leaf + leaf | 4 | ~887k | 20 | 15 % (tight) |
| five instances | 5 | ~1,065k | 21 | over |

Consequence: the per-node proving time and memory of the slot node should be close to today's
95 s / 26.2 GB (same domain 2^20, same column count; PLONK prover cost follows the domain and
columns, not the filled rows). The k=19 lever (node 1.5 % over 2^19) is forfeited by any tree
node: a two-instance merge is already ~532k rows. The 4-slot variant is a second-stage option
only after T0 shows the margin.

### F5. Prior art: KZG-halo2 aggregation trees are usable, folding trees are not (SOURCE)

| System | What it does | Usable with stock Midnight PLONK/KZG + BLS12-381 decider? |
|---|---|---|
| PSE `snark-verifier` `KzgAs` | Accumulator = (lhs, rhs) G1 pair; several accumulators combined by `r = transcript.squeeze_challenge()` and `r.powers(n)`; pairing deferred to a decider. https://docs.rs/crate/snark-verifier/0.2.2/source/src/pcs/kzg/accumulation.rs | **Yes, as the pattern**: identical to `midnight-circuits` `Accumulator` (lhs/rhs MSM plus named fixed-base scalars). Nothing to port; our `verifier.accumulate` is this. Note CVE-2026-62337 (point-at-infinity handling in the aggregation circuit before 0.2.5): our negative tests must include identity-point child accumulators. https://app.opencve.io/cve/CVE-2026-62337 |
| Axiom aggregation | Universal aggregation circuit; each inner snark verified in circuit, pairing deferred by folding G1 points into a running `KzgAccumulator` exposed as public instances; keygen writes a `.tree` JSON of circuit ids and the aggregation tree. https://docs.rs/crate/axiom-query/2.0.14/source/src/verify_compute/README.md , https://rust-digger.code-maven.com/src/axiom-query-2.0.17/KEYGEN.md | **Yes, as the pattern**: accumulator-as-public-instance is what `ivc.rs:370` does; the committed tree manifest is what F8 asks for. |
| Scroll zkEVM (halo2 era) | Chunk proofs -> 2 compression layers -> `BatchCircuit` aggregates k chunks, checks chunk hash and state-root linkage -> `RecursionCircuit` over batches; recursion rounds set by L1 latency target. https://github.com/scroll-tech/zkevm-circuits/pull/568/files , https://scrollzkp.notion.site/Upgrade-4-Darwin-Documentation-05a3ecb59e9d4f288254701f8c888173 | **Yes, as the pattern** (fixed layers with per-layer keys). We prefer one self-recursive key (F2) because the layer count depends on the program. Scroll has since moved to OpenVM (https://l2beat.com/zk-catalog/scroll). |
| Taiko raiko | Multi-prover (Risc0, SP1, SGX); aggregation inside zkVM guests. https://github.com/taikoxyz/raiko | No halo2 tree to reuse; not applicable. |
| Mina scan state | "a forest of full binary trees, where each node in the tree is a job"; base jobs = transactions, merge jobs created as children complete; root proof "attests to the correctness of all transactions at the base of the tree"; parameters `transaction_capacity_log_2`, `work_delay`; partially filled trees are normal. https://docs.minaprotocol.com/mina-protocol/scan-state . Statement = {source, target, supply_increase, fee_excess, pending coinbase, sok_digest}; merge requires left.target = right.source and sums the additive fields. https://chainsafe.github.io/mina-rs/rustdoc/src/mina_rs_base/snark_work.rs.html | **Yes, as the scheduler and statement design**: our segment statement `[pre_d, post_d, h_lo, h_hi]` is Mina's `{source, target}` with an additive height. Mina's proof system (Kimchi/Pickles on Pasta, IPA) is not reusable. |
| Nova / SuperNova / parallel Nova (PSE `parallel_prover_bench`), ParaNova notes | Binary tree of folded instances; each node = one base computation (the "median" of its children) plus a folded instance; "4 seems to be the upper bound" of fan-in; PSE PoC: "The final fold verification never succeeds", "uses a lot of RAM", 3 folds per step for consistency checks, about 35 % gain. https://hackmd.io/@9MPvD9czTQS18zEjAERVAg/Hy-vKK04h , https://hackmd.io/80cPXRc0Sa2MPuYX4Wft8w , https://scrapbox.io/bitpickers/Paranova (no paper exists) | **No** as a proof system: R1CS folding on curve cycles (Pasta, BN254/Grumpkin); no BLS12-381 cycle (a 255-bit scalar-field curve cannot host the 381-bit base-field order, s7 FINDINGS); final proof is not stock Midnight PLONK. **Yes** as a shape: "node = boundary computation + two children" is our slot node. |
| Mangrove (CRYPTO 2024) | Uniformizing compiler plus folding IVC; two-pass, parallel, low memory. https://eprint.iacr.org/2024/416 | No: folding-based, transparent setup, not a KZG decider. |
| HyperNova, ProtoGalaxy | Multi-instance folding over CCS / high-degree relations; constant marginal verifier cost per extra instance; HyperNova notes Nova "does not immediately support parallel proof generation" and a generic compiler exists. https://eprint.iacr.org/2023/573 , https://eprint.iacr.org/2023/1106 ; Sonobe multi-instance HyperNova: https://git.arnaucube.com/arnaucube/sonobe/commit/edadcdd520b8cf657cfa5c679dd09c6579759b0c | No (same reasons; Sonobe KZG "needing revamp", s7 FINDINGS). |
| BCMS20 / BCLMS21 accumulation PCD | PCD from accumulation schemes; KZG accumulation is one of the schemes; arkworks `ark-accumulation` prototype. https://eprint.iacr.org/2020/499 , https://eprint.iacr.org/2020/1618 , https://github.com/arkworks-rs/accumulation | **Yes, as the theory our construction instantiates.** Depth result (F6). |
| `midnight-aggregation 0.1.0` | IVC aggregator: one inner proof per step, claims as a Poseidon hash chain, running accumulator, decider. https://docs.rs/crate/midnight-aggregation/latest/source/src/multi_circuit_aggregator/mod.rs | Linear only; confirms the vendor's own recursion is the same primitive we use. No tree there. |

### F6. The tree improves the generic PCD soundness argument (SOURCE)

Chiesa, Guan, Samocha, Yogev (ePrint 2023/1646): prior PCD analyses give "security loss ...
exponential in the recursion depth"; the loss disappears only for SNARKs with straightline
extraction. Bünz (Simons 2025 slides) says tree-based IVC "still has a high loss" and that
practice assumes depth-independence with no known attack. https://eprint.iacr.org/2023/1646 ,
https://simons.berkeley.edu/sites/default/files/2025-07/CRY25-2%20Benedikt%20Bunz_slides%20.pdf .
Our linear chain has depth 293 for fact(3); the slot tree has depth 9 (S=1) or 6 (S=8). This is
a real, if theoretical, improvement in the only generic theorem we can cite; SAS-009 would label
it "experimental results", not a formal claim.

### F7. Why it was cut, and which reasons still hold (SOURCE)

`stagenet-gp-recursion.md:146`: "The original proposal (kept for the record) was a binary tree of
nodes over per-rule leaves." The reasons visible in `:138,:164-169`, `proposal.md` supersession
table, D13 and fable-2's position (`research/sherlock-vm-council-20261107/fable-2/position.md:49`):

| Reason then | Holds now? |
|---|---|
| "per-rule leaf keys cannot feed one node" (`:138`); the tree was bundled with per-rule leaves and cut with them | No. D4 makes the leaf universal; the leaf slot takes one constant key (or the F-table membership check) exactly as `ivc.rs` does. |
| Node count is #leaves-1 either way, and proofs run "serial under heavy.lock" (`:167-168`), so a tree bought nothing | Only while the lock is serial. The owner made throughput mandatory; §3.5 already names the second lock class (E11) as the enabling work. |
| Linear IVC was the smallest change to `parent.rs`, and M3 proved it (`:190-193`) | Still true. The slot node is a bounded delta on `ivc.rs` (one more self slot, bits, decider pin), not a rewrite. |
| recursion-perf s7: "a balanced binary tree adds three internal proofs; the table floor prevents rewarding its smaller advice count" (FINDINGS, four fixed rollup leaves) | Not applicable: that is the 16-tx rollup with four leaves, where arity-4 needs one node. Sherlock has 37-293 leaves in a dependency chain. |
| SRC-011 Optional; D13 recorded the tree as a fallback because nothing was measured | Reversed by the owner. Measurement is this plan's first step. |

### F8. Preserved properties: what SRC-001..007 become in the tree

Node statement (segment form, replaces `[vk_N_repr, prog_d, init_d, cur_d, h, acc]`):

```
[vk_N_repr, prog_d, pre_d, post_d, h_lo, h_hi, acc...]      (acc = collapsed lhs/rhs + self fixed-base scalars, unresolved)
witness: has_L, has_R, has_leaf (bits); child_L instance+proof+carried acc; child_R likewise;
         leaf [binding, prog_d, m1, m2, rules_d, h1, steps]
constraints:
  at least one of has_L, has_R, has_leaf                      (no empty node)
  L: has_L ? (L.prog_d = prog_d, L.pre_d = pre_d, L.h_lo = h_lo, m1 = L.post_d, h1 = L.h_hi) : (m1 = pre_d, h1 = h_lo)
  leaf: has_leaf ? (leaf.prog_d = prog_d, leaf.pre_d = m1, leaf.h_pre = h1, m2 = leaf.post_d, h2 = h1 + steps, 1 <= steps <= S) : (m2 = m1, h2 = h1)
  R: has_R ? (R.prog_d = prog_d, R.pre_d = m2, R.h_lo = h2, post_d = R.post_d, h_hi = R.h_hi) : (post_d = m2, h_hi = h2)
  h_lo, h_hi, steps, h1, h2 < 2^64 (assert_lower_than_fixed, pattern parent.rs:665-670)
  L.vk_repr = R.vk_repr = self transcript_repr (lane 0 of each child instance)
  acc = collapse(accumulate([scale(proof_L, has_L), scale(carried_L, has_L),
                             scale(proof_R, has_R), scale(carried_R, has_R),
                             scale(leaf_acc, has_leaf)]))
```

Linkage is the associative "child post = sibling pre" with the height as an additive field,
Mina-style. Position is pinned: a leaf with `h_pre = 5` can only sit at height 5, so a leaf proof
cannot be consumed twice or out of order (the second use would need `pre = post` of a non-empty
step, which the digest and `steps >= 1` refuse). Ordered child coverage exactly once is therefore
preserved by `(digest, height)` chaining, as in the linear node.

| Obligation | Linear node today | Slot tree |
|---|---|---|
| SRC-001 seven-lane leaf statement | consumed by the one leaf slot | unchanged; one leaf slot per node |
| SRC-002 `prog_d` | passthrough, pinned by the decider | every child and the leaf assert `prog_d` equality; decider pins sealed `program_d` |
| SRC-003 leaf-internal `rules_d`, `steps`, chain | inside the leaf | unchanged |
| SRC-004 range checks, `steps` in [1,S], `h = prev_h + steps` | one addition | two additions (`h1`, `h2`), all heights range-checked; decider `n_steps = h_hi` with `h_lo = 0` |
| SRC-005 genesis: `prev_d == init_d`, no unauthenticated carry | one genesis case, `scale_by_bit(not_genesis)` on `proof_acc`, carried acc folded unscaled (the ISSUE) | no genesis case in the node. Each disabled slot is the SRC-005 situation: realization (a) neutral-encoding constraint or (b) scale both the proof acc and the carried acc by the slot bit. The genesis check moves to the decider: `root.pre_d == init_d` and `root.h_lo == 0`. Authenticated genesis (task 4.9) is unchanged: the leaf at `h_pre = 0` binds canonical initial carriers to `init_d`. |
| SRC-006 decider | verifies last node under the node key as constants, pins `vk_N_repr`, folds carried acc, pairs, opens `fin_d` | same, plus two equalities (`pre_d == init_d`, `h_lo == 0`); `values = [prog_d, init_d, fin_d, n_steps = h_hi, result_be]` unchanged; ESTIMATE +~10 rows |
| SRC-007 `acceptRun`, replay set | unchanged | unchanged (the root `stmt` is one value) |
| SRC-010 independent verifier | `verify_chain` walks a list | walks the committed tree manifest, replays `next_acc` per node in the exact slot order, checks every link and every height; refuses a manifest whose shape differs from the proof bytes |
| SRC-008/009 keys | leaf -> node -> decider closure | same closure; one new node relation magic (`SHKTRE01`), decider re-keyed (already due in task 2.4) |
| SAS-006 undischarged obligations | altered accumulator, skipped decider, substituted key, wrong predecessor | add: swapped children, reused leaf, disabled slot with non-neutral carry, identity-point child accumulator (CVE-2026-62337 class), manifest/proof shape mismatch |

### F9. Composition with D4, S>1 and the run decider

- Universal leaf (option U): the leaf slot is `verify_child_incircuit_with(leaf_vk const)`, as
  `ivc.rs:356-365`. Option F: the leaf slot carries the in-circuit key-table membership check
  exactly as the linear node would (SRC-013); the tree adds nothing.
- S>1 (SRC-014): `steps in [1, S]` at the leaf slot; the tree is orthogonal to S. S shrinks L and
  therefore the tree: S=8 gives 37 leaves, 37 nodes, depth 6.
- Run decider: unchanged values; two more equalities; the root is one node proof under the same
  key, so `vk_N_repr` pinning is as today.
- Leaves are already independent and parallel (2.55 s at k=13 for the M2 digest shape); the tree
  changes only the node layer.

### F10. Ragged shapes need no padding and no identity element

The slot bits are the identity: a node with `has_R = 0` is a unary node; a node with only a leaf
is a bottom node. Any L is a valid tree; the scheduler chooses the shape. For the
leaf-at-every-node full shape, nodes = L and depth = ceil(log2(L+1)): L=293 -> 9, L=37 -> 6,
L=6 (`entry 7;`) -> 3. An identity node (all bits 0) is refused ("at least one slot"), so the
decider's `n_steps >= 1` cannot be met by an empty tree. The ParaNova "padding with dummy
computations" and Mina's partially filled trees are both avoided.

### F11. Parallel resources and wall time (ESTIMATE)

Inputs: node 95 s, 26.2 GB; leaf 2.55 s (M2 shape, k=13); decider 100 s; 20 cores; 120 GB.
Scheduler bound: W_nodes ≈ max(depth, ceil(L/P)) × t_node(P); here L/P ≥ depth, so
W_nodes ≈ ceil(L/P) × t_node(P). t_node(P) = 95 s × f, f = slowdown from sharing cores
(f = 1.0 if the 95 s run used ≤ 20/P threads; f ≈ 1.1-1.3 if the node averages ~3 cores as the
recl9 node did; f up to 4 only if the node saturates 20 cores, in which case the tree gives
nothing on one host).

| Case | L | P | f | node layer | leaves (8 parallel) | decider | total | vs today |
|---|---|---|---|---|---|---|---|---|
| linear S=1 (measured basis) | 293 | 1 | 1 | 293 × 95 = 27,835 s | 747 s serial | 100 s | 7.97 h (doc: 7.7 h) | 1× |
| tree S=1 | 293 | 3 | 1.0 | 98 × 95 = 9,310 s | 93 s | 100 s | 2.64 h | 2.9× |
| tree S=1 | 293 | 4 | 1.2 | 74 × 114 = 8,436 s | 93 s | 100 s | 2.40 h | 3.2× |
| tree S=1 | 293 | 4 | 1.0 | 74 × 95 = 7,030 s | 93 s | 100 s | 2.03 h | 3.8× |
| tree S=1, unlimited P (critical path) | 293 | 293 | 1 | 9 × 95 = 855 s | 93 s | 100 s | 17 min | lower bound only |
| linear S=8 (leaf cost unknown; assume 20 s/leaf) | 37 | 1 | 1 | 37 × 95 = 3,515 s | 740 s serial | 100 s | 1.21 h | 6.4× |
| tree S=8 | 37 | 4 | 1.2 | 10 × 114 = 1,140 s | 185 s (4 par.) | 100 s | 24 min | 20× |
| tree S=8, critical path | 37 | 37 | 1 | 6 × 95 = 570 s | 185 s | 100 s | 14 min | lower bound only |

Memory: P=3 uses 79 GB; P=4 uses 105 GB plus leaves and OS; P=4 is only safe if nothing else
runs (today the task-2.1 test alone holds 113 GB). Under the SRC-011 48 GB peak clause P=1 and
the tree gain is 1.0×. GPU: zero credited; no toolkit, no GPU path in midnight-proofs; a GPU MSM
(perf tracker C3) would cut t_node for every node and compete for the same unified memory.

Implementation cost ESTIMATE: `tree.rs` ~500 lines (from `ivc.rs` 396), off-circuit mirror and
bundle manifest ~200, `verify_chain` tree walk ~150, scheduler (Python, under `doctorwatson
heavy` with an aggregate cap) ~300, tests ~400; one agent, 1-2 weeks, plus the keygen regression
(SRC-009) that task 2.4 already requires.

## 2. Recommended plan: ordered tasks

Each task names the requirement change, the predicate and the measurement that confirms or kills
it. T0 and T1 need no new circuit code and run before any decision.

**T0. Row probe of the slot node (light lane, ~0.5 day).** Copy
`crates/mori-recursion/tests/ivc_cost.rs` (`ivc_node_rows`, modeling only, no SRS) to a probe of
three relations: {self, self, leaf}, {self, self}, {self, self, leaf, leaf}. Record rows and k.
Requirement: none yet. Predicate: a `budgets.json` row `tree-node-rows` with rows and k.
Kill: {self, self, leaf} > 2^20 - 5 % (k=21 doubles time and memory; stop the tree).
Confirm: k=20 with margin ≥ 20 %.

**T1. Thread scaling and concurrency of the existing node (heavy.lock, ~1 day, no new code).**
(a) `sherlock_ivc::ivc_node_folds_three_leaves` at `RAYON_NUM_THREADS` ∈ {4, 10, 20}; record
wall per node step, peak RSS and average CPU (`/usr/bin/time -v`: user+sys over wall).
(b) Three independent genesis node proofs started together at `RAYON_NUM_THREADS=6` under one
aggregate cap of 100 GB; record each wall and the aggregate peak. This is the tree's throughput
with zero tree code. Requirement: amend SRC-011 (see T2) before the run, or run (b) as a labelled
diagnostic (SAS-014). Predicate: `budgets.json` rows `node-threads-{4,10,20}`, `node-concurrent-3`.
Decision numbers (§7): U = average cores of one node at 20 threads; f3 = wall(concurrent)/wall(solo).
Kill: f3 ≥ 2.5 (the node already saturates the host; the tree gains < 1.3× here).
Confirm: f3 ≤ 1.4 and aggregate peak ≤ 100 GB.

**T2. Requirement changes (spec only, with the build decision).**
- SRC-011: O -> U (mandatory). Replace "under 48 GB peak" by "under 48 GB peak per process and
  under 100 GB aggregate for a declared concurrent run"; acceptance check adds the aggregate peak.
- E11 (heavy concurrency lock class with aggregate resource enforcement) moves from "optional
  separate work" (§3.5, tasks "Later changes") to a task of this change; `scripts/doctorwatson
  heavy --concurrent N --aggregate-cap 100G`.
- New SRC-015 (U): the node SHALL prove the segment statement of F8 with gated slots, and SHALL
  be unsatisfiable for an empty node, a broken link, a non-monotone height or a child under a
  different `vk_repr`. Acceptance: fresh-witness negatives at the constraint boundary (SAS-003).
- SRC-005 restated per slot: realization (a) or (b) applied to every disabled slot (proof acc and
  carried acc); recorded before node keygen (task 2.3 wording).
- SRC-006 adds `root.pre_d == init_d` and `root.h_lo == 0`.
- SRC-010 adds the committed tree manifest and the per-node accumulate-order replay.
- D13 row: tree-of-nodes moves from "Cut" to "Measured option; built if §7 criteria pass".

**T3. Build the slot node (after T0, T1 pass; 1-2 weeks ESTIMATE).**
`crates/mori-recursion/src/tree.rs` (`SHKTRE01`): relation, witness, `next_acc` mirror for the
five-input fold in slot order, instance encoding, `acc_from_lanes`. `run_decider.rs`: two
equalities, `n_steps = h_hi`. Bundle format: a tree manifest (node id, slot bits, child ids, leaf
id). `examples/verify_chain.rs`: tree walk. Scheduler: list scheduling, children-first, P from
the aggregate cap. Tests (T2 lane): 3-leaf full tree; ragged L=5 and L=7; `entry 7;` (L=6);
negatives: swapped children, leaf reused at two positions, height gap, disabled slot with a
non-neutral carry, identity-point child accumulator, manifest shape mismatch, wrong `vk_repr`
child. Predicates: A13 (negatives) extended; new A14 `tree-fold-entry7`. Keygen regression A11
runs after (SRC-009), decider re-keyed, manifests written (SRC-008).

**T4. fact(3) end to end, tree vs linear (heavy, after T3).** Same leaves, both folds, same
decider key family; record wall, aggregate peak, proof count, depth. Predicate: `budgets.json`
rows `fact3-linear-S1`, `fact3-tree-S1-P{3,4}`. Confirm: tree ≤ 0.4 × linear wall at equal S.
Kill: tree ≥ 0.7 × linear (then keep linear and spend on S and k=19 instead).

**T5. S=8 on the tree (after task 4.8's S=8 leaf exists).** L=37, depth 6. Predicate:
`fact3-tree-S8`. Target: < 30 min total.

Order relative to the rest of the re-plan: T0 now (light lane, can run while the task-2.1 test
holds memory); T1 as soon as the host is free, after the C0 profile of the `entry 7;` chain
(task 5.3) or merged into it as two extra workloads; T2 at the decision; T3-T5 after M4's leaf
is keyed (the node's leaf VK is a circuit constant, `ivc.rs:10-15`, so the tree is keyed once
with the real leaf).

## 3. General-purpose recursion and arbitrary composition (owner requirement 2026-11-08)

Owner: "It has to be general purpose recursion." The slot tree meets this as follows, and each
clause below is a checkable obligation, not a description.

1. **One fixed key set for every program.** Three keys: leaf (`sherlock-leaf/1`, universal, D4),
   node (`SHKTRE01`), run decider. The program enters only as `prog_d` (SVM-005 digest of image,
   layout schema, bounds, profile). No node, decider or leaf constant depends on the program, the
   step count or the tree shape. E2 as written at `stagenet-gp-recursion.md:34-38` holds: P3,
   written after the freeze, needs no new key. Check: the relation bytes of node and decider
   (`write_relation`) contain the leaf VK and k only; a program-dependent constant is a defect.
2. **Any step count.** Heights are u64 lanes range-checked below 2^64 (SRC-004); the decider's
   `n_steps = h_hi` is one field. L leaves for any L ≥ 1; the tree over them has any shape
   (F10). The only bounds are the field (`h < 2^64 < p`) and the host.
3. **Unbounded depth within the field bounds.** The node is self-recursive under one key; depth
   is not a circuit parameter. Nothing in the statement counts levels. The accumulator stays a
   fixed-size encoding at every level (collapsed lhs/rhs plus a fixed list of self fixed-base
   scalars, `ivc.rs:155-181`), so the public instance length is constant in depth and in L.
4. **Arbitrary composition.** A node composes any two valid segments `[a, b, h0, h1]` and
   `[b, c, h1, h2]` into `[a, c, h0, h2]`, with an optional leaf step between them, each slot
   independently present or absent. Segments compose whenever the boundary digest and height
   match, so the scheduler is free to pick any binary bracketing of the leaf sequence (left-deep,
   balanced, ragged); the decider accepts a root iff `pre_d = init_d`, `h_lo = 0`, `post_d` is a
   halted digest. Composition of segments is associative at the statement level (§4, Lean L2);
   the accumulator fold is not literally associative (challenge r depends on grouping) but is
   sound for every grouping (F3), and the grouping is committed in the bundle manifest (F8, risk 4).
5. **Ragged sizes and identity.** The disabled slot is the identity segment `[x, x, h, h]`; an
   all-disabled node is refused. No padding leaves, no dummy steps, no power-of-two requirement
   (F10). S>1 padding inside a leaf stays SRC-014's business (disabled steps, `enable = 0`).
6. **What is not general.** The admitted rule inventory `R` and the profile `/1` bound the
   machine, not the program: a program that fires a rule outside `R` is refused by the leaf
   (SVM-004), which is the same limit the linear chain has.

## 4. Formal targets (for the Quint and Lean agents the owner asked for)

Both models treat proofs as abstract certificates: a node certificate exists iff its constraints
hold on its children's certificates. Cryptographic soundness (the certificate implies the
witness) is the assumption; what is modeled and proved is the *statement-level* argument that
the current design has only informally (`stagenet-gp-recursion.md:45-50`, task 5.4's Quint model
of "linkage, bounded monotone height, unique genesis, replay set").

### 4.1 Quint model: `models/sherlock/tree_fold.qnt` (extends task 5.4)

State: a set of leaf certificates `[prog, pre, post, h_pre, steps]` produced by a reference
machine run (and by an adversary, see A-actions); a set of node certificates
`[prog, pre, post, h_lo, h_hi]` with slot bits and child ids; a decided set; the `runs` replay
set of the contract. Actions: `prove_leaf(i)`, `fold(L?, leaf?, R?)` under the F8 constraints,
`decide(root)`, `accept(stmt)`, and adversarial actions `forge_link`, `reuse_leaf`,
`swap_children`, `empty_node`, `shape_mismatch`, `replay`. Invariants to check:

- **I1 Linkage.** Every node certificate `[p, a, c, h0, h2]` has a chain of slot segments whose
  boundaries match: `post` of each present slot equals `pre` of the next present slot, with
  disabled slots the identity. Violations by `forge_link` are unreachable.
- **I2 Height monotonicity and additivity.** `h_hi - h_lo` equals the sum of `steps` of the
  leaves under the node; every present leaf has `steps >= 1`, so `h_hi > h_lo` for any node with
  a leaf below it; all heights `< 2^64` (modeled as a bound constant).
- **I3 Exactly-once coverage.** The multiset of leaf heights under a decided root is exactly
  `{0, s_1, s_1+s_2, …}` with no gaps and no repeats: `reuse_leaf` and a missing leaf are both
  refused by I1+I2 (a reused leaf at height h would need two segments to start at h).
- **I4 Identity and padding neutrality.** A disabled slot changes neither digest nor height; an
  all-disabled node is never certified; adding or removing a unary (single-child) node does not
  change the segment statement.
- **I5 Schedule independence.** For every two trees over the same leaf sequence (any bracketing,
  any ragged shape, any interleaving of `fold` actions), the root statements are equal. Encode as:
  two independent fold schedules from the same leaf set reach roots with identical
  `[prog, pre, post, h_lo, h_hi]`. This is the model-level form of Mina's "merge proofs are
  identical, regardless of the order merged".
- **I6 Genesis uniqueness.** A decided root has `pre = init_d` and `h_lo = 0`; no decided root
  exists for a segment that does not start at genesis (`h_lo != 0` refused by `decide`).
- **I7 Replay.** `accept(stmt)` succeeds at most once per `stmt`; two roots of different runs
  (different `prog`, `init`, `fin`, `n_steps` or `result`) have different `stmt`.
- **I8 Program fixity.** Every leaf and node under a decided root carries the same `prog`; a
  node whose children disagree on `prog` is unreachable.
- **I9 Shape binding.** `decide` with a manifest whose shape differs from the certified tree is
  refused (models risk 4); the accumulator is abstracted as the ordered list of folded
  certificate ids, and the verifier's replay must produce the same list.
- **I10 Liveness (witness).** For every L in 1..N (N small, say 9) and every bracketing, a
  schedule exists that decides the root: `quint run` finds it; this guards against an
  over-strict constraint (the astra-2 "post == pre under 1-sel" class of mistake).

Also check I1-I9 under the S>1 leaf (steps in [1, S], terminal publication as last enabled step)
to confirm the tree is orthogonal to SRC-014.

### 4.2 Lean theorems: `lean/Sherlock/TreeFold.lean`

Definitions: `Machine` as a step relation `step : State -> State -> Prop` (the general machine,
program fixed by `prog`); `digest : State -> F` injective on the reachable set (assumption, from
Poseidon collision resistance, stated as a hypothesis); `Seg := (prog, pre, post, h_lo, h_hi)`;
`LeafCert seg` := exists states s, t with `digest s = pre`, `digest t = post`, `steps` machine
steps from s to t, `h_hi = h_lo + steps`, `1 <= steps <= S`; `NodeCert` inductively from the F8
rules with optional slots; `RootOk` := `NodeCert seg ∧ seg.pre = init_d ∧ seg.h_lo = 0 ∧ halted
seg.post`.

- **L1 Segment soundness (the main theorem).** For every program `prog` and every
  `seg` with `NodeCert seg`: there exist states `s, t` and an execution trace of the general
  machine of exactly `seg.h_hi - seg.h_lo` steps from `s` to `t` with `digest s = seg.pre` and
  `digest t = seg.post`. Proof by induction on `NodeCert`, composing the slot traces; base case
  `LeafCert`. Corollary: `RootOk` implies a trace from the canonical initial state (via
  `init_d` injectivity) to a halted state with result `r` in `n_steps = h_hi` steps, for every
  program. This is E4's "the tree may have any depth" made a theorem.
- **L2 Associativity of segment linkage.** Define `link : Seg -> Seg -> Option Seg` (Some iff
  `a.post = b.pre ∧ a.h_hi = b.h_lo ∧ a.prog = b.prog`). Theorem: `link` is associative
  (`link (link a b) c = link a (link b c)` in `Option`), the identity segment `[x, x, h, h]` is a
  two-sided unit, and `link a b = Some c` implies `c.h_hi - c.h_lo = (a.h_hi - a.h_lo) +
  (b.h_hi - b.h_lo)`. Hence any bracketing of a leaf sequence yields the same root segment
  (schedule independence, the Lean form of Quint I5).
- **L3 Padding neutrality.** A disabled slot contributes the unit of L2; a node with all slots
  disabled has no `NodeCert` (constructor requires at least one slot); a unary node certifies the
  same `Seg` as its child.
- **L4 Exactly-once coverage.** From `NodeCert seg`, the heights of the leaves below form a
  strictly increasing sequence partitioning `[h_lo, h_hi)` with each leaf's `h_pre` equal to the
  previous `h_post`; no leaf certificate is used at two positions.
- **L5 Height bounds.** If every lane is range-checked `< 2^64` and `2^65 < p`, the field
  additions `h1 = h_lo + steps`, `h2 = h1 + …` do not wrap, so the natural-number reading used in
  L1-L4 is exact (this is SRC-004's justification, stated once).
- **L6 Decider pinning.** `RootOk` plus `acceptRun`'s reads (`program_d = prog`, `init_d = pre`)
  imply the written `result` is the halted value of the trace of L1 for the sealed program and
  initial state; a root for another program or another initial state is refused.
- **L7 Independence from shape and depth (meta).** None of L1-L6 mentions depth or arity; state
  this as a theorem over `NodeCert` of arbitrary height (structural induction suffices), which is
  the formal reading of "unbounded depth within the field bounds".

Out of scope for both: the cryptographic step (certificate implies witness) and the accumulator
algebra; those remain SRC-005/SRC-006 test obligations and the BCMS20-class theorem (F6).

Plan hook: **T2b (parallel with T2; two agents).** Quint model with I1-I10 and seeded mutants
(one per invariant, as `models/recursion/profile.qnt` did); Lean file with L1-L7 and no `sorry`.
Predicates: `quint test models/sherlock/tree_fold_test.qnt` and `lake build` rows under the
assurance §3 model row of task 5.4. Both run before T3 keygen; a failed I10 or an unprovable L1
changes the F8 constraints before any key exists.

## 5. Risks

1. **Memory clause.** Without the SRC-011 aggregate amendment the tree is 1.0×. This is an owner
   decision, not an engineering one. A 100 GB aggregate leaves 20 GB; the task-2.1 test already
   showed a single process at 113 GB, so the aggregate cap must be enforced (cgroup or the
   `capped heavy` wrapper), not advisory.
2. **k=21.** If T0 shows the three-slot node over 2^20, every node costs ~2× and ~52 GB; the
   tree then needs P=2 at 105 GB and gains ~1.5×. Fallback: pure merge node {self, self} plus the
   existing linear node as a second key family (two keys, decider pins the merge key, merge
   verifies either key by a bit, ~4 verifier instances) is worse; prefer dropping the tree.
3. **Per-slot genesis hardening.** Three disabled-slot cases instead of one; each needs the (a)/(b)
   realization and a fresh-witness negative through the final pairing (SRC-005). A wrong scaling
   on the carried acc of a disabled slot is the ISSUE that `ivc.rs:318-364` already has, times
   three.
4. **Shape-dependent challenges.** The fold challenge r depends on the grouping; verifier and
   prover must agree on the tree. If the manifest is not committed into the bundle hash,
   `verify_chain` can be fed a different shape and will refuse valid bundles (liveness) or, worse,
   accept a bundle it did not fully check (SRC-010). The manifest must be hash-bound.
5. **The 95 s basis.** If the node already uses most of 20 cores, f → 3-4 and the tree's gain on
   this host collapses to ≤ 1.3×. T1(b) measures this for 300 s of machine time.
6. **Forfeits k=19.** The linear node at k=19 (if the 1.5 % trim lands) halves time and memory and
   allows the tree's P to double; the tree node cannot reach k=19. The two levers compete for the
   same rows; the tree wins only if P ≥ 3.
7. **Scheduler and lock discipline.** `doctorwatson heavy` is serial by design (SWF-004,
   `heavy.lock`). A concurrent class changes `runs.jsonl` accounting (one run with N children),
   `maxRssBytes` semantics (aggregate) and the stop rule. This is factory work, not circuit work,
   and it is on the critical path.
8. **Soundness theory.** Depth 9 instead of 293 is better under 2023/1646 but still "high loss"
   (Bünz); no formal claim (SAS-009 "not applicable").
9. **GPU.** No toolkit installed; nothing in this plan depends on it; a later GPU MSM shares the
   120 GB and so lowers P unless it also lowers per-node RSS.

## 6. Open questions

- Q1. How many rayon threads did the 95 s / 26.2 GB M3 node measurement use (`588bc5f25`)? The
  test does not set it; the A-set wrappers pin 4. T1(a) answers this.
- Q2. Average core utilization of one node proof at 20 threads (recl9 node: 268 %). T1(a).
- Q3. Cost of the S=8 universal leaf (k, time, RSS); design Q4/Q5. Changes L and the leaf term.
- Q4. Does the `capped heavy` wrapper enforce a per-process or a per-tree cap today? E11 design.
- Q5. Does the real universal leaf (option U, M4) change the leaf-slot verifier rows enough to move
  the slot node toward 2^20? T0 uses the stand-in leaf; re-probe at M4.
- Q6. Under option F, does the key-table membership check in the leaf slot fit beside two self
  slots at k=20? Re-probe after SRC-013's decision.
- Q7. Should the decider accept a root with `h_lo != 0` (a segment) to allow the anchored-segments
  fallback (`:170-174`) to share the tree node? Not needed for E4; record only.
- Q8. Whether `assign_vk_as_public_input` with two `prepare` calls on one `AssignedVk` has been
  exercised anywhere in midnight (the vendor IVC verifies one inner proof per step). T0 synthesizes
  it; T3's 3-leaf test proves it.

## 7. Recommendation and decision criteria

**Measure first, with two runs that need no tree code (T0, T1); build on pass; otherwise keep the
tree as the recorded fallback and spend on S=8 and k=19.**

Build the tree when all of:
1. T0: three-slot node ≤ 0.95 × 2^20 rows (k=20).
2. T1(b): three concurrent node proofs at 6 threads each complete with f3 = wall/solo ≤ 1.4 and
   aggregate peak ≤ 100 GB. (Equivalent: T1(a) shows average utilization U ≤ 6 cores at 20
   threads.)
3. The owner amends SRC-011 to a 100 GB aggregate cap and funds E11 (concurrent lock class).
4. Projected gain from the measured f3: ceil(L/3) × 95 × f3 ≤ 0.5 × L × 95, i.e. f3 ≤ 1.5 at P=3;
   for fact(3) S=1 that is ≤ 3.9 h vs 7.7 h, and ≤ 2.4 h at P=4.

Keep it as a fallback when any of: k=21 at T0; f3 ≥ 2.5; the owner keeps the 48 GB peak clause;
or linear S=8 measures under 1.5 h and the owner accepts that, since the tree's marginal gain
over linear S=8 is ESTIMATE 2.5-3× (1.2 h -> ~25 min) for 1-2 weeks of work plus the factory
concurrency change.

Do not build before measuring: the tree's entire gain is P, and P is a property of this host and
the lock discipline, not of the circuit.
