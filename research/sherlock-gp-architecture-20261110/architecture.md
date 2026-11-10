# General-purpose recursion architecture for Sherlock

Run `sherlock-gp-architecture-20261110`. Date: 2026-11-10. Editor: `fable`. Second designer and
critic: `astra` (GPT-6 Astra through the Codex CLI, read-only, two rounds).

Inputs: `BRIEF.md`; `fable/design-1.md` and `astra/design-1.md` (independent proposals);
`astra/design-2.md` (Astra's attack on the Fable proposal and its answer to the disagreements).
Nothing was built, run or proved. Every Astra claim used here was re-read in the repository or
at the cited source by the editor, except where marked "per Astra". Round 2 found errors in
`fable/design-1.md`; they are corrected here and listed in section 3.3, and that file is left as
written.

Conventions. Paths are relative to the repository root. `D` =
`openspec/changes/sherlock-vm-consensus-20261107`. `L:<path>:<line>` is the ledger at tag
`ledger-9.1.0.1` (`a2b30128`), read with `git show`. `MC:` =
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.3.0/src`; `ZS:` =
`.../midnight-zk-stdlib-2.4.0/src`; `MP:` = `tools/profile/vendor/midnight-proofs-0.8.3/src`.
ESTIMATE marks a number that is computed or reasoned, not measured.

## 1. Recommended architecture

### 1.1 The facts that decide it

1. **The node's cost is the non-native MSM, and three proof slots do not fit k=20.** Measured
   model rows at circuits 7.3.0: self + stand-in leaf 652,437; self + self 730,243;
   self + self + leaf 1,078,147; self + self + leaf + leaf 1,278,670
   (`qa/sherlock/decisions/speed-levers.json:22-29`); self + C6 digest leaf 741,708
   (`qa/sherlock/manifests/node.json:22`); decider 929,166
   (`qa/sherlock/manifests/run-decider.json:20`). 2^20 = 1,048,576. Two collapses are 78% of the
   node (`speed-levers.json:66`). Pairings are already deferred to the decider
   (`crates/mori-recursion/src/ivc.rs:10-31`), so "defer the pairings" is not an available gain.
2. **The in-circuit verifier is specialised to one constraint system and one k.** `AssignedVk`
   keeps `domain` and `cs` off-circuit; only `transcript_repr` is in circuit
   (`MC:verifier/mod.rs:55-66`). A Midnight constraint system is a function of
   `(ZkStdLibArch, k)` (`ZS:lib.rs:159-209,1573-1586`). ZKIR v3 picks the arch per circuit from
   the instructions used, always with `bls12_381: false`, and k by `optimal_k`
   (`L:zkir-v3/src/ir_vm.rs:1212-1263`; `L:zkir-v3/src/ir.rs:105-106`). Compact circuits
   therefore fall into many `(arch, k)` classes, none of which is the node's
   (`crates/mori-recursion/src/lib.rs:23-30`).
3. **Ledger proofs use a Blake2b transcript; the gadget verifies Poseidon transcripts only.**
   `L:transient-crypto/src/proofs.rs:64-65,563-575`; `MC:verifier/types.rs:144,149,191,196`.
   Keys do not depend on the transcript (`transcript_repr` is Blake2b of the key in both cases,
   `MP:plonk/mod.rs:255-278`).
4. **Fixed-base work is deferred by name, only for keys the decider knows.**
   `MC:verifier/msm.rs:14-17,43-53`; `ivc.rs:10-15`. A key chosen by witness from a registry
   must have its fixed and permutation commitments witnessed and multiplied in circuit as
   variable bases; `resolve_fixed_bases` accepts assigned points (`MC:verifier/msm.rs:491`). One
   variable base is 3-4k rows (ESTIMATE, `research/sherlock-speed-replan-20261108/prover.md:155`).
5. **Whole-state hashing binds capacity to the key.** The leaf hashes every carrier lane
   (`crates/mori-backend/src/sherlock_c6_child.rs:551-569`). Poseidon is width 3, rate 2, 8 full
   and 60 partial rounds with 5 skips (`MC:hash/poseidon/constants/mod.rs:17-26`;
   `MC:hash/poseidon/poseidon_chip.rs:50`): 18 round rows per permutation, before absorption,
   output and encoding rows (a lower bound, not a `cost_model` reading; section 3.3). One carrier side at the holdout draft config is about
   35k lanes (`docs/sherlock/stagenet-gp-recursion.md:157-158`).
6. **No BLS12-381 curve cycle, and no folding or split-accumulation prover in the pinned
   crates.** `research/sherlock-speed-replan-20261108/tree.md:27,111`; the only trace of split
   accumulation in midnight-proofs is a comment on an unused field (`MP:plonk/traces.rs:14-18`).

### 1.2 The architecture

**A binary proof-carrying-data node over the existing KZG accumulation, with typed statements,
a claim tree, a key registry, and committed memory in the VM.** Both designers arrive at the
same core: one self-recursive node with two predecessor slots at k=20, never three proof slots;
claims as the unit of composition; a registry root held by one contract; committed memory;
in-circuit genesis. They differ on where new work enters the node (section 3).

```
L0  base proofs
      U    universal Sherlock leaf: S steps over committed state           core key
      X    foreign circuits (Compact contract circuits, other PLONK/KZG)  registry, kind = direct
      A_c  adapter for one class (arch, k, exact public-input count,
           transcript): prepares one X proof under a witnessed key,
           publishes [vk_id_inner, out_d, carried accumulator]            registry, kind = adapter
L1  lifts (node class: bls12_381 + poseidon, k=20)
      R    run lift: verifies one or two U proofs, links them -> SEG      core key, named
      F    foreign lift: verifies one adapter-class proof under a
           witnessed registry key, folds the adapter's carried
           accumulator, proves membership of the adapter key and of
           the inner key, emits a claim -> SET                            core key, named
L2  node N: two slots; each slot is off or one proof under N, R or F     core key, self-recursive
      SEG + SEG -> SEG   adjacent segments of one run
      SEG       -> SET   seal: opens post_d; h_lo = 0, terminal outcome, deps_d empty; proves the
                         Sherlock leaf identity is a registry member -> one RUN claim
      SEG + SET -> SET   seal with dependencies (the opened deps_d equals the SET root)
      SET + SET -> SET   join
L3  closing proof C (node class): verifies the root N proof, resolves the named bases of
    N, R and F, collapses                                                 core key
    decider D (k=20, Blake2b transcript, ledger template): verifies C, checks the pairing,
    exposes [reg_root, set_root, n_claims, shape_d]                       core key, on chain
L4  contract SherlockClaims: acceptRoot, openClaim, admitKey
```

Statements.

| Layer | Statement |
|---|---|
| U | `[binding, prog_d, input_d, pre_d, post_d, h_pre, steps, deps_d]`. `pre_d` and `post_d` digest a fixed register file plus memory commitments. The leaf with `h_pre = 0` proves `pre = Init(prog_d, input_d)` in circuit. |
| SEG | `(leaf_id, prog_d, input_d, pre_d, post_d, h_lo, h_hi)`: the `Seg` of `clean/fork/Clean/Sherlock/Aggregation.lean:27` plus `input_d` and the leaf identity. |
| claim | `c = H(CLAIM, kind, id, in_d, out_d)`. RUN: `id = prog_d`, `in_d = input_d`, `out_d = H(outcome_kind, value_d, n_steps)`. FOREIGN: `id = vk_id`, `out_d = H(public inputs)`. |
| SET | `(set_root, n_claims)`. `set_root` is the hash of the claim DAG with domain-separated tags (claim leaf, JOIN, DEP); `n_claims` counts occurrences, not distinct claims. |
| every R, F, N statement | also `shape_d = H(SHAPE, relation tag, slot bits, child shape_d values, local statement)`: the proof graph itself, including segment bracketing and unary nodes (Astra's separation of shape from claim content, adopted; section 3.3). |
| N | `[n_repr, r_repr, f_repr, reg_root, tag, payload, shape_d, acc]`. A node requires its children to carry the same three key representations and the same `reg_root`. R and F statements end with their own carried accumulator, which the parent folds. |
| D | public values `[reg_root, set_root, n_claims, shape_d]` in the `acceptRoot` transcript template. |

Key set: `{U, R, F, N, C, D}`, fixed and independent of program, input, run length, memory
size, registry content, shape and depth. Programs, inputs, registry members (foreign keys and
adapter keys) and the shape are data.

How the node takes R and F without a third slot or an in-circuit MSM over their keys. The
decider already resolves the node's own fixed bases by name. R and F do not verify N, so they
are keyed first and N takes their `transcript_repr` as public inputs, as it takes its own
(`MC:verifier/verifier_gadget.rs:207`). R, F and N are all node-class relations at k=20, so one
verifier instance per slot serves all three (fact 2). In a slot, the witnessed representation
is constrained to one of the three by a one-hot selector, and each deferred fixed-base scalar is
recorded under the selected name set. The transcript absorbs the key representation first
(`MC:verifier/verifier_gadget.rs:296`), so a proof made under one key does not verify under
another. C resolves the three name sets with constant bases. The accumulation challenge hashes
accumulator encodings, which do not contain names (`MC:verifier/msm.rs:229-290`), so the
routing is sound only with the full set of constraints listed in section 3.1 item 3; the
library does not enforce them.

### 1.3 How each requirement is met

- **GP-1 universal VM.** U is keyed over the rule inventory and a fixed register file, not
  over `Bounds`. `prog_d` is a commitment to the program table and `input_d` a commitment to the
  input and execution parameters; both are opened in circuit. Frames are a hash chain, the
  append-only data arena and the program table are Merkle trees. Every outcome kind is a
  terminal state that the seal rule opens and turns into `out_d`. Height is range-checked below 2^64
  (exists, `ivc.rs` `constrain_heights`). Acceptance: three unseen programs, one above the old
  carrier size, one with input, one of each outcome kind, one key set, one contract.
- **GP-2 cross-run composition.** A sealed run is a claim. Batching is `SET + SET`; each claim
  opens from `set_root` by a Merkle path (`openClaim`). Consumption: a run's state carries
  `deps_d`, the commitment to the claims it consumed; the seal-with-dependencies rule requires a
  verified SET whose root equals it.
- **GP-3 foreign proofs.** The registry is a Poseidon Merkle root in contract state. The
  Sherlock leaf is a registry entry too (kind `run`, identity from U's key representation and
  the machine version, carried as a lane of every SEG): the seal rule proves its membership, so
  that the run and the foreign proof are "both checked in-circuit against the registry" as the
  GP-3 acceptance test says. U stays a constant of R; the path is a policy check, not key
  selection. F takes
  the adapter key by witness, computes its identity, proves its membership and the inner key's
  membership against `reg_root`, and folds both the adapter proof's obligation and the inner
  proof's carried obligation. Adapters map public inputs to the claim form. A Compact circuit
  enters through the adapter of its class.
- **GP-4 any shape.** One node key, two slots, any nesting, any depth. A proof is data and can
  be an input to two parents, which gives DAG fan-out. `set_root` binds which claims were combined and how; `shape_d` binds the proof graph. The
  final object is always one decider proof.

### 1.4 What is not met as written, and the closest design

No evidence shows that a requirement is impossible on this stack, and none needs a different
proof system on present evidence. Nothing is measured for the new relations either: every row
figure for N, R, F, C and the adapters is an ESTIMATE until experiment 1 and 2 run. Four points
where the recommended design does not meet the text as written:

1. **"A node can take at least two prior proofs (arity >= 2) plus new work"** (GP-4). In the
   recommended design a k=20 node takes two proofs, each either prior (N) or new (R, F); two
   prior proofs plus new work is two nodes. Three verifier slots are 1,078,147 rows (fact 1).
   The designs that meet the sentence in one node: (a) Astra's inline work, constraints of the
   node itself beside two self slots, within what is left of 2^20 after about 730k rows; (b) a
   three-slot node at k=21 at about twice the node cost, which needs an SRS above 2^20 (only
   `bls_midnight_2p20` is in the local parameter cache; GP-4 does not fix k). Experiment 1
   measures (a) beside the named lifts. Astra holds that the two-node reading is a narrowing
   and must not be put to the owner as an interpretation (section 3.2 dissent 1).
2. **"Other PLONK/halo2 circuits on the same curve and commitment scheme"** (GP-3). The
   verifier circuit is a function of the foreign circuit's constraint system (fact 2) and of
   its exact public-input count, because the count is absorbed into the transcript as a
   circuit constant (`MP:plonk/verifier.rs:68-74`; `MC:verifier/verifier_gadget.rs:303-306`).
   The recommended design therefore needs one adapter key per class `(arch, k, public-input
   count, transcript)`, and covers circuits built with the Midnight stdlib configuration: two
   instance columns, one committed and one ordinary (`ZS:lib.rs:547-548`), and one challenge
   phase (`MC:verifier/verifier_gadget.rs:308-310`). Adapter keys are registry data; both
   designers now read them as compatible with the fixed core key set. A halo2 circuit outside
   that configuration, or over another SRS, has no path in the recommended design. The design
   that covers it is Astra's verifier program run by an in-node processor (section 3.2
   dissent 2); it is built nowhere and unmeasured.
3. **A proof already accepted on Midnight** (ledger bytes, no witness). Stage 1 verifies a
   Poseidon-transcript proof of the same Compact circuit under the same deployed key, which
   needs the witness holder. Ledger bytes need the keyed Blake2b transcript with its prefixes
   (`MP:transcript/implementors.rs:13-24`), complete parsing (`ZS:utils/plonk_api.rs:154`) and
   the point checks the recursive gadget leaves out
   (`MC:verifier/transcript_gadget.rs:70-80,118-128`). The stdlib Blake2b wrapper is unkeyed
   (`ZS:external/blake2b.rs:101`) and enabling its chip changes the adapter's arch. The row
   cost is unmeasured (experiment 3). Astra holds that GP-3 is not complete before this works
   (section 3.2 dissent 3).
4. **"Capacity does not depend on the key"** (GP-1). Fixed Merkle depths are a capacity fixed
   by the relation. With depth equal to the machine's address width the bound is the address
   space, as the height bound is 2^64; Astra holds that this is still a narrowing and that
   storage must be expandable with paths processed across proofs (section 3.2 dissent 4).

Also open, and not a stack limit: no concrete security bound exists for arbitrary-depth PCD
with this accumulation. The Lean model takes accumulation validity as a premise
(`Aggregation.lean:1230-1244`), and known PCD analyses "incur expensive blowups" with depth
unless the SNARK has a straightline extractor (https://eprint.iacr.org/2023/1646). A balanced
tree has depth log M where the chain has depth M.

## 2. Options considered

### 2.1 Q1 Accumulation scheme

| Option | Node rows / k | Proof size, prover cost | In the pinned crates | Call |
|---|---|---|---|---|
| Full in-circuit verification with the pairing in every node | ESTIMATE 1.8-3.5M, k=21-22 (per Astra, from the decider's pairing workload) | fixed final proof; ESTIMATE minutes per node | yes (`crates/mori-recursion/src/pairing.rs`) | Rejected: repeats the decider in every node. |
| Today: KZG accumulation, self + one constant leaf (`SHKIVC06`) | 652,437-741,708, k=20, measured | 6,432 B proof class; 61-67 s, 16.6-17.6 GB measured (`speed-levers.json`) | yes | Baseline; cannot reach arity 2 plus a leaf. |
| KZG accumulation, self + self + leaf | 1,078,147, k=21, measured model | ESTIMATE 2x time, about 33 GB | yes; needs SRS above 2^20 | Fallback only (1.4 item 1b). |
| **KZG accumulation, two polymorphic slots over named core keys (recommended)** | ESTIMATE 760-850k, k=20: 730,243 measured for self + self with one combined collapse (`crates/mori-recursion/tests/tree_node_rows.rs:8-20`), plus per-slot collapse and name selection, canonical disabled slots, tag rules | as today per node (same domain) | gadget, accumulator, `scale_by_bit`, `AssignedAccumulator::assign` with a name list: yes. Name selection: through the accumulator's public encoding after a per-slot collapse, or a small fork (`fixed_base_scalars` is private, `MC:verifier/msm.rs:50-64`) | Kept. Decided by experiment 1. |
| KZG accumulation, two self slots plus inline processor work (Astra) | ESTIMATE 0.82-1.00M, k=20 (Astra) | as today per node; work per node limited to the rows left | same gadgets; the processor is new | Kept as the alternative (section 3.2 dissent 1). |
| Split accumulation (BCLMS) | ESTIMATE 0.2-0.8M rows non-native (Astra) | - | no (`MP:plonk/traces.rs:14-18` is a comment) | Rejected for now: no cycle, so the deferred group work stays non-native; nothing to reuse. https://eprint.iacr.org/2020/1618 |
| Folding (Nova, HyperNova, ProtoStar, ProtoGalaxy) | folding verifier is 1-3 scalar multiplications on a cycle (https://eprint.iacr.org/2023/1192); non-native here, ESTIMATE 10-40k rows | fast steps; needs a wrapper that proves the inner verification relation to the stock Midnight verifier | no; no BLS12-381 cycle | Not first: nothing to reuse, and a foreign PLONK proof is not a folding instance, so the folded step must contain the PLONK verifier. Kept as a fallback for the whole inner layer, not only for U (Astra's correction: multi-instance folding exists, https://eprint.iacr.org/2023/573, https://eprint.iacr.org/2023/1106, and the final relation need not be a KZG opening). |
| Inner STARK or folding layer, outer KZG wrap | wrap ESTIMATE k=20-23 | 50-500 KiB inner proofs (Astra, ESTIMATE) | no | Not now. Astra holds it as the fallback if the KZG node fails fit or throughput (section 3.2 dissent 6). |
| Per-level or per-shape keys | fits | - | yes | Rejected: GP-4 forbids per-depth keys. |
| Unreleased upstream gadget cuts (5-15% fewer verifier rows) | helps every KZG option | changes keys and the proof format | no (`prover.md:123`) | Watch; not a plan input. |

Lift sizes are hypotheses, not planning bounds (Astra's verdict "unsupported"; accepted). The
only measured binary baseline is the 730,243-row probe, which has one shared name set, one
collapse and no canonical disabled-slot construction. R = up to two leaf verifications with a
constant key and one shared base resolution (the pattern of
`crates/mori-recursion/src/inner.rs:273-314`): ESTIMATE 600-800k. F = one adapter-class
verification, witnessed-key resolution of about 40 bases at 3-4k rows, a carried accumulator,
two registry paths: ESTIMATE 550-750k. C = one verification plus three name sets: ESTIMATE
700-950k. The decider cannot do C's work itself: two more name sets at 30-45 bases and 3-4k
rows are 180-360k rows (ESTIMATE) against 119,410 rows of headroom (2^20 minus 929,166). The
two-stage end (C then D) is therefore the expected case, and D must fold both C's own
obligation and C's published accumulator (`inner.rs:182-207` is the pattern).

Throughput (ESTIMATE). A run of M leaves needs about M/2 R proofs, M/2 - 1 merge nodes and one
seal, about M node-class proofs, close to today's chain count; merges wait for their children,
and the only contention figure on record is a 1.69-1.74 slowdown for three concurrent provers
(`speed-levers.json:11-16`), so P workers give less than P. Final cost: one C proof and one D
proof; their time is unmeasured.

### 2.2 Q2 Universal VM and committed memory

The leaf sees a fixed register file and commitments. Everything of unbounded size is behind a
commitment that a step opens where it reads or writes.

`sd = H(SD, control registers, outcome, fuel and counters, stack_h, arena_root, arena_len, env
handle, deps_d)`. `prog_d = H(PROG, prog_root, machine version, rule inventory id)`.
`input_d = H(IN, input value digest, fuel, stack limit)`. Initial fuel and the stack limit are
semantic parameters of initialization (`crates/mori-core/src/sherlock/reference.rs:14`), so they
are in `input_d`; a prover's memory capacity is not.

| Structure | Commitment | Cost per access, as a formula (`c` = rows of one tagged tree compression with ordering and selection, at least 18 and unmeasured; both designers withdrew their round-1 figures, 18 and 50-150) |
|---|---|---|
| Program table (immutable) | Merkle root, depth 20 | read: d x c |
| Frames (13 lanes each, `D/design.md` §2.1) | hash chain, top frames in registers | push, pop or rewrite of the top: about 7-8 absorption chunks (13 lanes plus the previous digest and a tag) |
| Data arena (append-only, child before parent) | append-only Merkle tree, depth 32, with length | read: d x c; append or write: 2d x c |
| Environment | persistent structure in the arena | as arena |
| Input | Merkle or hash chain under `input_d` | as arena read |
| Data-field references, environment cells, value slots (three more append tables, `crates/mori-core/src/sherlock/helper_machine/c6_phys_data.rs:11-18`) | one append-only tree each, or one tree with a table tag | as arena |
| Helper task stack (7-lane entries, push, pop, rewrite of the top; `c6_phys_helper.rs:10-13`; `shared/owner_runtime.rs:3566-3577`) | hash chain | as frames |
| Helper record (23 lanes) and result record (4) | registers | 0 |

| Memory option | Composes in any PCD shape | Call |
|---|---|---|
| Merkle trees and hash chains | yes: a segment is provable from `pre_d` alone | Kept as the first implementation. Both designers. |
| Paged Merkle (page-in, page-out per leaf; RISC Zero's continuations, https://dev.risczero.com/api/zkvm/optimization) | yes | Kept as the first optimisation, after locality is measured. |
| Lookup or permutation memory with a challenge | yes, if every record is committed before the challenge that tests it: inside one proof, or over a sealed run or an epoch with authenticated boundaries (https://eprint.iacr.org/2025/105) | Kept as a throughput candidate, after Merkle. The round-1 Fable statement that it cannot compose was too strong (section 3.3). |
| Multiset hash into a curve (https://arxiv.org/abs/1601.06502) | yes, additive and challenge-free; needs a final sweep and a linear-use discipline | Alternative if Merkle paths dominate (experiment 5). |
| Whole-state hashing (today) | yes | Rejected: fact 5. |

Depth is disputed (section 3.2 dissent 4).

Outcomes: the terminal step writes `(outcome_kind, value_d)`; `value_d` digests the canonical
value encoding. For a structured value this is a hash of the value itself, computed by a
traversal, not an arena root and handle: two runs with different allocation histories must
give the same `value_d` (Astra's correction; adopted). Owner-step height counts machine steps only; memory openings never consume a
height (Astra's point; adopted).

Genesis: with committed memory the initial state is a few constants plus `prog_root` and
`input_d`, so the first leaf proves it in circuit for one state hash. Both designers now hold
that in-circuit genesis is required: a claim consumed in circuit cannot rest on off-chain
recomputation. This closes dissent 2 of
`research/sherlock-recursion-design-review-20261110/review.md` 4.2.

Risk: the fragments read lanes of the bounded image `sherlock-c6-phys/1` today. How much of
each fragment survives depends on how many distinct cells a rule touches (experiment 5).

### 2.3 Q3 Registry and adapters

Key identity: `vk_id = H(VK, class_id, transcript_repr, comm_d)`, with `class_id = H(arch, k,
exact public-input count, proof and key format version, transcript id, srs id)` and `comm_d` the Poseidon digest of the fixed and
permutation commitments in key order. It is a deterministic function of the key bytes, so
anyone can recompute it from a deployed Midnight key (`ZS:lib.rs:1537-1562` for the layout).
The public-input count is in `MidnightVK` and not in `transcript_repr`, so it must be bound
here (Astra's finding; verified). F does not recompute Blake2b; the binding of
`transcript_repr` to `comm_d` is as strong as admission, so admission must check that the
tuple is the normalization of real key bytes, by governance or by an admission proof.
The SRS identity is part of the class because the pairing equation is specific to one SRS
(`MP:poly/kzg/params.rs:235-237`; Astra's point, adopted): a key over another SRS cannot be
accumulated with ours.

Registry: Poseidon Merkle tree of `H(REG, vk_id, kind)`, kind in {direct, adapter, run};
root in contract state; `admitKey` appends. A direct key is a foreign circuit: the claim about
it has `id = vk_id`, which the adapter computes from the witnessed key and F checks against the
registry. An adapter key may emit FOREIGN claims about direct keys. A run entry is a Sherlock
leaf identity allowed to emit RUN claims: U at first; a different leaf prover later would be a
second run entry and would enter through F. Adapter and run entries make claims on behalf of
other identities and need governed admission; an adapter key can be checked by regenerating
it from its class. Admitting an adapter does not admit the circuits it can verify: F also
proves that the adapter's `vk_id_inner` is a registry member (round 1 omitted this; section
3.3). Whether a proof made under an older `reg_root` stays usable after an admission is a
policy choice: current root only, or an authenticated root history with a revocation rule.

| Mechanism for a foreign circuit | Per-class key | Status | Call |
|---|---|---|---|
| Adapter circuit `A_c` using the existing gadget, generic over the verified class; all adapters share one adapter class, which F's verifier is built for | yes, registry data | gadget exists; the repository has verified ZKIR children in circuit (`qa/sherlock/manifests/c6-leaf.json` records a ZKIR leaf with a Poseidon transcript) | Recommended. |
| Verifier as an authenticated program run by an in-node processor (Astra) | no | not built anywhere | Alternative; meets GP-3 without adapter keys. Cost unmeasured. |
| Foreign key as a node constant | per key | exists (today's leaf) | Rejected: GP-3 forbids it. |
| One slot per class in the node | no | - | Rejected: rows. |

Compact contract proofs. Format (verified): proof tag `proof[v5]`, key tag `verifier-key[v7]`
(`L:transient-crypto/src/proofs.rs:129-133,368-372`); the key is `MidnightVK` (arch, k,
public-input count, PLONK key); Blake2b transcript; one public-input vector whose first
elements are the binding input and the optional communications commitment, followed by the
transcript of ledger operations (`L:zkir-v3/src/ir_vm.rs:213-217`; `L:ledger/src/verify.rs`
`public_inputs_len`, "the binding input and the communication commitment"); trailing key bytes
are refused (`L:transient-crypto/src/proofs.rs:528-535`). Consequences:

- The adapter commits to the whole public-input vector (`out_d`) and interprets nothing. A
  typed adapter that exposes a return value needs the opening of the communications commitment
  as a witness (Astra's point; adopted).
- A verified contract-circuit proof shows circuit satisfaction for those public inputs. It does
  not show that a transaction was included on chain (Astra's point; adopted). The binding input
  ties the proof to a call context; inclusion is the ledger's business.
- The adapter only prepares the foreign proof. The proof is valid when the resulting
  accumulator satisfies the pairing, so the adapter publishes that accumulator and F folds it
  beside the adapter proof's own obligation (Astra's finding; round 1 omitted it).
- The stdlib configuration has two instance columns (committed and ordinary) and the gadget
  asserts one challenge phase; the ledger passes no committed instance, which the stdlib
  replaces with the identity commitment (`L:transient-crypto/src/proofs.rs:563-575`;
  `ZS:lib.rs:2020-2023`). Compact circuits use this configuration.
- The public-input count is exact per adapter class (1.4 item 2). Zero-padding to a cap, as
  round 1 proposed, changes the transcript and is wrong. A variable-length absorption would
  need a changed verifier gadget. The ledger maximum is 65,536 inputs per call
  (`L:onchain-runtime/src/transcript.rs:87`).
- Guarded-off ledger operations stay in the vector as zero fields
  (`L:zkir-v3/src/ir_vm.rs:507-523`); the adapter keeps positions as they are.

### 2.4 Q4 PCD shape

Node: two slots, each off or one proof under N, R or F. The all-off node is refused. R has a
one-leaf mode for odd counts and single-leaf runs. A disabled
slot contributes canonical neutral accumulator encodings to the fold (Astra's point; the
current genesis handling at `ivc.rs:538-546` is the pattern, and it is why `collapse_self`
cannot be removed as a local optimisation).

Shape binding has two parts. `set_root` binds the claim DAG by content: `leaf(c) =
H(LEAFCLAIM, c)`, `join = H(JOIN, l, r)`, `dep = H(DEP, c_run, deps_root)`. Content addressing
identifies equal subgraphs, ignores unary nodes and ignores how a run's segments were
bracketed; that is intended for claims and it is not the whole proof graph. `shape_d` binds
the proof graph at every R, F and N. Both are in the proved statement, not in an off-chain
manifest. Today it is
the manifest: the Quint decider "does not read the manifest" and shape binding is "decider plus
verify_chain" (`models/sherlock/quint/aggregation.qnt:16-22,464-467`). A SEG carries no shape by
design: `link` is associative (`Aggregation.lean:291`) and the bracketing of one run does not
change its statement (`same_leaves_same_stmt`, `:708`).

DAG fan-out: the SET proof of claim A is the right child of the seal node of B and of the seal
node of C. A is proved once and verified twice.

| Object | Linear or reusable | Mechanism |
|---|---|---|
| Machine steps of one run | linear: each height exactly once | segment adjacency (`left.post = right.pre`, `left.hi = right.lo`); existing `Covers`, `linkAll_covers` (`Aggregation.lean:787,858`); Quint I3 (`aggregation.qnt:410`) |
| Genesis | exactly one per `(prog_d, input_d)` | the seal rule requires `h_lo = 0` and the first leaf's `Init` check; replaces the decider check (`non_genesis_refused`, `Aggregation.lean:1106`; Quint I6) |
| Claims | reusable facts | a claim may appear under several parents |
| Settlement effects | linear | the contract's set of opened claims (a claim identifier is the nullifier). The set of accepted roots alone does not prevent replay: `JOIN(A, B)` and `JOIN(B, A)` are different roots containing A (Astra's finding) |

Model changes needed: the Quint invariants I8 "program fixity" and I9 "no certificate is folded
twice" (`aggregation.qnt:457-467`) forbid a mixed-program batch and a shared ancestor and must
be replaced for the claim layer (Astra's finding; verified). Lean needs a claim layer above
`Seg` (an inductive `SetCert` with seal, dep and join), the theorem "every claim under a
decided `set_root` has a sound certificate", injectivity of `shape_d`, "no unfinished object is
decided", an extended `result_unique` for the new outcomes and dependencies, and a
well-founded extraction argument for reused certificates (no dependency-cycle forgery was
found by either designer; none is proved absent).

### 2.5 Q5 On-chain fit

One contract for every program, `SherlockClaims`:

- state: `reg_root`, `roots: Set<Field>`, `claims: Set<Field>`.
- `acceptRoot(set_root, n, shape_d)`: verifier key = decider D, by the same mechanism as today (a
  hand-written key behind a Compact transcript template,
  `tools/sherlock-sdk/sherlock-run.compact:1-15`). The template reads `reg_root` from the
  ledger, so a proof made against another registry is refused, as `acceptRun` does for
  `program_d` (`:40-47`). A root already accepted is refused.
- `openClaim(root, item, path)`: an ordinary Compact circuit with a fixed maximum path length
  (for example 64); proves the path from a claim or from a sub-root to an accepted root. A
  sub-root is added to `roots`, so a tree deeper than the maximum is opened in several calls
  (Astra's finding: a Compact circuit cannot take a path of arbitrary depth). A claim is
  inserted into `claims`, once per claim, so a shared ancestor has one effect. The claim tree
  must use the hash Compact exposes (experiment 7).
- `admitKey(vk_id, kind)`.

| Limit | Value | Fit |
|---|---|---|
| Verifier key bytes | 50,000 (`L:transient-crypto/src/proofs.rs:422-431`) | decider key is 2,217 B (`docs/sherlock/stagenet-gp-recursion.md:198`) |
| Public inputs per call | 65,536 (`L:onchain-runtime/src/transcript.rs:87`) | about 70 today; the new template is of the same order |
| Operations per contract | 256 (`L:ledger/src/structure.rs:1928`) | three |
| Decider k | no k check in `VerifierKey::verify` (`L:transient-crypto/src/proofs.rs:563-575`); a k=20 decider verifies (`speed-levers.json:95-109`) | D is expected to stay at k=20 because C does the base resolution (experiment 2) |

Options: a single `acceptRun`-style entry with optional single-run fields (Astra) or
`acceptRoot` plus `openClaim` (Fable). They are compatible: the single-run entry is a second
template over the same decider that exposes one claim's fields.

## 3. Agreement and dissent

Process. Round 1: two proposals written without sight of each other (`fable/design-1.md`,
`astra/design-1.md`). Round 2: Astra read both and wrote 26 findings against the Fable
proposal, a table of the differences with its position on each, and its withdrawals (`astra/design-2.md`); Codex exit 0 in
both rounds. The editor then re-read the sources for each finding used below. The first Astra
run was started with a network flag that Codex ignored ("`sandbox_read_only` is ignored",
`astra/design-1.log`); the lane still had Codex's own web search, and round 2 ran with
`--search`. The URLs cited in this document were fetched for the editor and checked against
their abstracts or page text, except https://eprint.iacr.org/2025/105, which is per Astra.

### 3.1 Agreed

1. **Primitive.** Keep midnight-proofs PLONK/KZG with the existing accumulation and a terminal
   pairing decider. It is the only option with local implementation evidence. "Defer the
   pairings" is already done; the cost is the collapse MSM.
2. **Node.** One self-recursive node with two predecessor slots; no third proof slot at k=20.
   The first thing to measure is the exact binary node with canonical disabled slots, because
   the 730,243-row probe is incomplete (`crates/mori-recursion/tests/tree_node_rows.rs:1-20`).
3. **Named core keys are sound in principle and need no fork.** Astra confirmed that N, R and
   F can share a verifier slot when their full stdlib configuration matches, and showed a
   public-API route: prepare the slot, take the accumulator's encoding, assign a replacement
   accumulator over the union of the three name sets (`MC:verifier/accumulator.rs:280`;
   `MC:verifier/verifier_gadget.rs:160`), and constrain each coefficient to `b_x * s_j`. The
   conditions for soundness, which the library does not enforce: boolean selectors summing to
   the slot bit; the same selectors choosing the representation, the coefficient destination
   and the allowed statement type; one fixed coordinate order (names are not in the encoding);
   children carrying the same three representations, pinned at the end; a child's inherited
   accumulator keeping its names; canonical disabled encodings before the challenge; one SRS.
   Astra revised its round-1 rule "carry only the self name set": a fixed finite union does not
   grow with the registry. A security argument and adversarial proof tests are still owed.
4. **For the named-lift design the end is two proofs**, a resolution proof then the pairing
   decider (Astra: "a first-class candidate"; one-stage and two-stage are measured together).
   Astra's own design ends in one decider, whose fit it puts at k=20-21, unmeasured.
5. **Committed memory, Merkle first.** Whole-state hashing is replaced by commitments opened
   per access. Frames and helper tasks as hash stacks (Astra adopted this from the Fable
   proposal after checking that the machine rewrites the top,
   `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:3566-3585`).
6. **In-circuit genesis.** Off-chain recomputation of `init_d` cannot stand once claims are
   consumed in circuit.
7. **Claims are reusable; steps are linear.** The same proof may enter two parents. The Quint
   invariants I8 and I9 must change. Settlement uniqueness is a separate, contract-level
   property keyed by claim, not by root.
8. **Registry.** A root in contract state, membership proved in circuit, identity binding the
   full verification interpretation including the exact public-input count, the format
   versions, the transcript and the SRS. Per-class adapter keys are registry data and do not
   change the core keys (Astra withdrew its round-1 kill criterion on this point).
9. **Compact proof facts.** Blake2b on the ledger, Poseidon in the gadget; the binding input,
   the communications commitment, guarded-off zeros, the identity committed instance, complete
   parsing. A verified circuit proof is circuit satisfaction, not inclusion.
10. **Shape and claim content are separate commitments**, both in the statement.
11. **One contract**, a decider key behind a Compact template, limits as in section 2.5; k=20
    is a resource target, not a ledger maximum.
12. **No concrete security bound** exists for unbounded-depth PCD with this construction; the
    Lean accumulator law is a premise.
13. **Hash costs are unmeasured.** Both round-1 figures are withdrawn.

### 3.2 Dissent kept

1. **Where new work enters the node.**
   Fable: through proofs. The leaf U stays a separate proof and enters by the run lift R; the
   node does only linking, sealing and hashing. Reasons: a leaf at k=20 has the whole domain
   for VM work and a k=20 node has about 266k rows left after two self slots (ESTIMATE: 0.95 x
   2^20 minus 730,243, before disabled-slot hardening), so inline work gives about half the VM
   rows per node-class proof (ESTIMATE); the rule fragments and the leaf measurements now
   running carry over; leaves prove in parallel.
   Astra: inline. A bounded processor runs inside N beside the two self slots; core keys are N
   and D only. Reasons: it meets "two prior proofs plus new work" in one node; fewer core keys;
   long operations (memory paths, foreign verification, set union) become continuations
   instead of new circuits. Astra's verdict on the Fable position: the two-node reading is a
   narrowing and should not be put to the owner as an interpretation; named lifts are "a
   useful optimization candidate".
   Settled by: experiment 1 (rows of both nodes) and experiment 5 (VM rows per step). Astra's
   kill criterion for the k=20 inline candidate is "no useful inline work"; the editor proposes
   150k rows left as that threshold.
2. **How a foreign verifier runs.**
   Fable: adapter circuits per class, built from the existing gadget; a class outside the
   stdlib configuration has no path until someone writes an adapter for it.
   Astra: class adapters as fast paths, plus a general path: the verifier description is
   authenticated data executed by the processor across nodes, so no new key is ever needed for
   a new architecture.
   Editor's note: the general path is a PLONK verifier interpreter with non-native MSM as its
   inner loop; at 3-4k rows per base it is several nodes per foreign proof even with a native
   scalar-multiplication instruction (ESTIMATE), and nothing like it exists in the pinned
   crates. Astra agrees it is "a proposed construction, not an implemented feasibility result".
   Settled by: experiment 4, and by the owner's reading of "other PLONK/halo2 circuits".
3. **Whether Poseidon re-proving counts for GP-3.**
   Fable: a Poseidon-transcript proof under the deployed key proves the same statement and
   satisfies the acceptance sentence "a proof from a Compact contract circuit"; ledger bytes
   are the second stage.
   Astra: it is an integration test. GP-3 is complete only when ledger proof bytes are
   verified under their native transcript; re-proving needs the witness and verifies nothing
   that was supplied.
   Not resolved; owner question 2.
4. **Fixed tree depth.**
   Fable: a depth equal to the address width (32 or 64 bits) is not a capacity ladder; GP-1
   itself bounds the height at 2^64.
   Astra: any depth fixed by the relation is capacity bound to the key; storage must be
   expandable, with path processing across proofs. The round-1 Fable depths (20 and 32) are a
   narrowing; the editor accepts that for 20 and 32 and holds the position for the address
   width.
   Not resolved; owner question 3. It changes the leaf: a 64-level path costs twice a 32-level
   one, and expandable storage needs continuations.
5. **Claim collection.**
   Fable: `set_root` is the content hash of the claim DAG; duplicates stay as occurrences;
   uniqueness is enforced where effects happen (`openClaim`). No in-circuit set union.
   Astra: a canonical map keyed by semantic claim identity, with insertion proved in circuit
   as a streamed computation, so that the statement is a duplicate-free set and `n_claims`
   counts distinct claims; plus a run identity that separates request context from the
   computation.
   Settled by: experiment 6 (cost of the union) and the owner's reading of "commits to the set
   of claims".
6. **Scope of the alternative proof systems.**
   Fable: folding or a STARK is a possible later prover for the leaf, admitted as a registry
   key that emits RUN claims; the node stays KZG because foreign PLONK proofs must be verified
   in a PLONK circuit anyway.
   Astra: a recursive STARK or multi-folding system is the fallback for the whole inner layer
   if the KZG node fails fit or throughput.
   Both: not first. Settled by experiment 8 only if experiments 1, 2 and 5 fail.
7. **Order.**
   Fable: shape first (M1, M2), memory last, because the node treats state digests as opaque
   and the current chain work continues meanwhile.
   Astra: memory and the native Compact probe early, because they decide whether the
   architecture stands; do not freeze a GP interface on the current chain.
   Section 4 keeps the Fable order for building and takes Astra's for measuring: experiments 3
   and 5 run before M2 starts.

### 3.3 Corrections to `fable/design-1.md` from round 2 (all verified by the editor)

- **Adapter registry bypass.** The adapter published `[vk_id_inner, out_d]` and nothing checked
  that the inner key is a registry member, so one admitted adapter admitted every circuit of
  its class. Fixed: F proves the inner key's membership.
- **Adapter obligation dropped.** `prepare` yields an accumulator, not a verdict
  (`MC:verifier/verifier_gadget.rs:691`). The adapter must publish the inner accumulator and F
  must fold it. Fixed in the statements.
- **Zero-padding public inputs is wrong.** The count is absorbed
  (`MP:plonk/verifier.rs:68-74`; `MC:verifier/verifier_gadget.rs:303-306`). The class includes
  the exact count.
- **Public-input count missing from `vk_id`.** It is a `MidnightVK` field
  (`ZS:lib.rs:1537-1541`), not part of `transcript_repr`.
- **A Blake2b adapter is not node class.** Enabling the chip changes the arch; adapters form
  their own class, for which F's verifier is built.
- **"One instance column"** is the gadget's comment; the stdlib has a committed and an
  ordinary instance column (`ZS:lib.rs:547-548`).
- **A fork is not needed** for name selection (3.1 item 3).
- **18 rows per permutation** is the round count only (initial constants, output row and
  absorption additions come on top; per Astra `MC:hash/poseidon/poseidon_chip.rs:430-536`,
  not re-derived by the editor). The per-access totals are withdrawn.
- **"Challenge-based memory cannot compose"** was too strong.
- **`set_root` does not bind every DAG** (segment bracketing, unary nodes, shared against
  copied subgraphs). `shape_d` added.
- **Accepted roots do not prevent replay.** Claim-level set added.
- **`openClaim` with an unbounded path** is not a Compact circuit. Bounded path with sub-root
  opening.
- **Seal** needs an opening of `post_d` and an empty-dependency condition; R needs a one-leaf
  mode.
- **State inventory** missed the helper task stack and three of the four append tables;
  structured results need a canonical digest, not a root and a handle.
- **Row intervals for N, R, F** are hypotheses; "decider time unchanged" was unsupported; the
  proof count omitted seals; ideal worker scaling contradicts the recorded f3.
- **Folding**: "needs a KZG opening of the folded witness" was too strong.
- **"Cannot be met"** for two prior proofs plus new work was wrong as an impossibility; it is a
  property of the recommended design, not of the stack.

Astra's withdrawals: the kill criterion "changing keys for a foreign circuit"; 50-150 rows per
compression as a planning cost; "carry only one name set"; the one-instance-column reading; its
node and decider intervals stand only as unvalidated estimates.

### 3.4 What each found alone

Astra: the adapter's missing obligation and the registry bypass; the absorbed public-input
count; the keyed Blake2b transcript and the unkeyed chip wrapper; trailing-byte and subgroup
gaps in the recursive transcript gadget; the SRS identity; replay across re-bracketed roots;
the unbounded `openClaim` path; the Quint I8/I9 conflict with fan-out; the helper task stack;
the public-API route for names. Fable: named lifts (deferral by name for more than one core
key); the per-`(arch, k)` class structure of Compact circuits from `used_chips` and
`optimal_k`; `transcript_repr` as Blake2b of the key and the witnessed-key resolution cost;
the two-stage end from the decider headroom; hash stacks for frames; DAG fan-out by proof
reuse without a mechanism; lifts keeping the leaf's whole domain for VM work.

## 4. Migration and milestones

Carries over: the verifier-gadget use, accumulator encoding, collapse and genesis-carry
constraints of `ivc.rs`; the height range checks; the pairing decider (`run_decider.rs`,
`pairing.rs`) and the transcript-template mechanism; the Lean `Seg`, `link`, `NodeCert`
algebra and the height lemmas; the Quint slot model as the segment layer; the reference
semantics and the C6 rule reasoning as specification and differential oracle; the witness
stream as producer; the row probes (`tests/tree_node_rows.rs`, `ivc_cost.rs`) and the
profiling harness.

Replaced: node `SHKIVC06` (by N, R, F, C); the decider's public values and the
one-contract-per-program `acceptRun`; whole-carrier hashing and the `prog_d` definition;
`Bounds` as a key input; the halted-u128-only boundary; Quint I8 and I9 at the claim layer;
shape binding by off-chain manifest.

| Milestone | Working software | Acceptance |
|---|---|---|
| M0 (in progress) | The linear chain on P1-P3 as planned in `D/tasks.md`. | Unchanged. It stays the first milestone (`general-purpose.md` Consequences) and produces the leaf measurements. |
| M1 binary kernel | N with two self slots and canonical disabled slots, a minimal base case, C and D, over stand-in leaves. | Real proofs of base, unary and binary nodes under one node key; a tampered child, carry, key or shape is refused through the decider. Common to both designs. |
| M2 GP-4 and GP-2 (b) | R and the named-key slots (or the inline kernel, by experiment 1); SET, join, `set_root`; `SherlockClaims` with `acceptRoot` and `openClaim`. | Chain, balanced tree and fan-out DAG at depth >= 8, equal final proof size; three runs of different programs in one proof, each claim opened. |
| M3 GP-2 (a), outcomes, input | `deps_d` in the leaf statement, seal with dependencies, all outcome kinds, run input, in-circuit genesis. | A run of B consuming the verified result of A, accepted once. |
| M4 GP-3 | F, the registry, `admitKey`, one adapter class; a Compact circuit proved with a Poseidon transcript; then the Blake2b adapter. | One proof with a Sherlock run and a Compact circuit proof, both checked against the registry. |
| M5 GP-1 capacity | Committed memory in U: program table, then frames, then arena. | A program above the old carriers runs under the same keys; three unseen programs; every outcome kind. |
| M6 | Throughput, independent bundle verifier, full local call validation, Preview. | The retained throughput gate and all four GP acceptance tests under one frozen key set. |

Order. M1 first because both designs need it and it is cheap. M2 before M5 because the node
treats `pre_d` and `post_d` as opaque digests, so the largest rewrite (the leaf's memory) does
not block the shape work. Experiments 3 and 5 run before M2 starts (Astra's order for measuring; section 3.2 dissent 7). Astra orders the memory processor and the complete interpreter (its
M2, M3) before composition (its M4), because in its design the processor is the node. Keys are
frozen once, after M5: R, N, C and D are re-keyed when U changes, so no key before that point
is a release key (Astra's rule; adopted).

## 5. Ranked experiments

Row probes use `cost_model` only (minutes, a few GB) and can run beside the heavy jobs. The
running experiments (4.1a per-rule rows, E2 headroom against leaf k, E3 in-circuit genesis
cost, E4 k=20 leaf time and RSS) are inputs to 1 and 5 and are not repeated. None had landed
in `qa/sherlock/decisions/speed-levers.json` (`G-A-prime` is still pending) when this was
written.

Both designers' lists are merged; where the order differs it is noted. Costs are ESTIMATE for
20 cores, 120 GB shared, one heavy job at a time; engineering time is separate.

| # | Hypothesis | Measurement | Kill criterion | Cost |
|---|---|---|---|---|
| 1 | A hardened binary node fits k=20 with margin, (a) with three named key sets and no inline work, (b) with one self key and inline work. Name routing works through the public API. | `cost_model` rows of both nodes with the final statement width, canonical disabled slots, carry constraints, type rules; base, unary and binary cases. Rows of R (one and two leaves) and F. Then real proofs with wrong-key, wrong-name and tampered-carry cases through the end. | Any accepted key or name mismatch kills that construction. Above 996,147 rows kills k=20 for that node. No useful inline work (editor's threshold: under 150k rows left) kills the k=20 inline candidate, not GP-4. | 3-7 days of code; modeling 10-60 min, 3-8 GB; proofs 1-3 h, 20-45 GB. |
| 2 | The end closes: one-stage D does not fit three name sets; C plus D does, with no obligation dropped. | Rows, key bytes, proof bytes, time and RSS of D alone and of C then D, over the node of 1. Mutate each carried obligation. | An unauthenticated obligation kills the design. C or D above 2^20 means k=21 for that stage or another split. | 2-5 days after 1; modeling under 1 h; proofs 1-4 h, 20-50 GB. |
| 3 | A ledger-format Compact proof can be verified in circuit. (Astra rank 3; Fable ranked it sixth, as stage 2.) | First, off circuit: reproduce the pinned verifier's challenges and acceptance on a real ZKIR v3 contract proof. Then rows of the keyed Blake2b transcript, complete parsing and point checks in an adapter; the adapter class arch. Beside it: the same circuit re-proved with Poseidon through a plain adapter. | Adapter above 2^20 rows at one verification means splitting it or the general path. A divergence from the native verifier fails. | 1-3 weeks of code; modeling 30-120 min; first proof 2-12 h, 20-60 GB. |
| 4 | Registry and adapter hold against an adversary. | Registered and unregistered inner keys through an admitted adapter; mutated representation and commitment pairs, public-input counts, class, transcript, SRS, kind, root epoch. Count the distinct classes among real Compact circuits (arch, k, input count). | Acceptance of an unregistered inner key or a mismatched normalization kills the interface. Hundreds of classes in practice argues for variable-length absorption or the general path. | 3-7 days beyond 3; proofs 2-8 h. |
| 5 | Committed memory makes the leaf independent of capacity at a useful cost. | Rows of one tagged compression, read, append, stack push, pop and rewrite. Distinct cell accesses per rule from the reference machine, for all four tables, both stacks and the helper record. Same key across tree growth. Canonical result digest across allocation histories. Uses the running 4.1a and E3 results. | A capacity-dependent key, an unauthenticated table, or a median step whose memory rows exceed its rule rows by 10x. | 1-3 weeks for a vertical slice; modeling 30-120 min; proofs 2-8 h. |
| 6 | Typed PCD: coverage, reuse, dependencies, shape and settlement coexist. | Lean and Quint claim layer. Chain, tree, diamond; unary nodes; one-leaf R; alternate bracketings; missing genesis; nonempty dependencies through plain seal; mixed roots; replay across re-batching. Cost of an in-circuit canonical set union against the content-hash root. | Any false sealed claim, undischarged dependency or duplicated effect. Refusing a legitimate shared ancestor also fails. | 4-10 days; model runs 10-60 min; minimal proofs 2-6 h. |
| 7 | The claim tree can be opened by a Compact circuit. | Hash one vector with Compact `transientHash` and with the node's Poseidon; compile `openClaim` with a 64-level path and read its k. | Mismatch means hashing the claim tree with the Compact hash in circuit, or a hand-written key for `openClaim`. | 1-2 h. |
| 8 | If k=20 fails: k=21, or another inner system. | k=21 binary or ternary node: time, RSS, SRS source. Only if that also fails: one folding and one STARK candidate on the same memory and foreign-verification workload with the complete Midnight wrapper. | k=21 above 48 GB or 2.5x the k=20 step. For an alternative: no fixed-key binary composition, no Compact path, or no wrapper inside the machine. | k=21: 2-5 days, 1-2 h heavy. Alternative inner layer: 3-8 weeks. |

Later, after an architecture is chosen: mixed-load concurrency (P node-class provers beside
leaf provers; the record is f3 = 1.69-1.74 for three node provers, `speed-levers.json:11-16`),
GPU offload, and the frozen-key acceptance run of all four GP tests with full local call
validation (Astra rank 8; 16-48 h serial).

The adoption decision follows experiments 1 to 4.

## 6. Proposed requirements and design changes (text only; no file was changed)

### 6.1 EARS requirements

| Id | Text |
|---|---|
| GPR-001 keys | The key set `{U, R, F, N, C, D}` SHALL NOT depend on the program, the input, the run length, the memory size, the registry content, the aggregation shape or its depth. When any of these changes, no key, contract or ceremony SHALL change. |
| GPR-002 node | The node N SHALL verify at most two proofs, each under a key whose transcript representation equals one of the node's public key lanes `n_repr`, `r_repr`, `f_repr`, and SHALL refuse a node with no enabled slot. |
| GPR-003 slot neutrality | When a slot is disabled, the node SHALL fold the canonical neutral accumulator encoding for that slot and SHALL constrain every statement lane of that slot to its canonical empty value. |
| GPR-004 key pinning | The closing proof C SHALL resolve the deferred fixed-base scalars of N, R and F against the constant keys, and the decider D SHALL pin C's key. A proof whose `n_repr`, `r_repr` or `f_repr` differs from the pinned keys SHALL be refused. |
| GPR-005 memory | The leaf U SHALL authenticate every program, input and mutable-memory access against a commitment in its pre-state and SHALL publish the post-state commitment. The leaf relation SHALL NOT contain a constant that bounds the number of program nodes, frames, tasks or table cells below the address width (or, if the owner decides dissent 4 for Astra, at all). |
| GPR-006 genesis | When `h_pre = 0`, the leaf SHALL prove that the pre-state equals `Init(prog_d, input_d)`. The seal rule SHALL require `h_lo = 0`, SHALL open `post_d`, and without a dependency set SHALL require the opened `deps_d` to be the empty commitment. |
| GPR-007 outcomes | The seal rule SHALL accept `Halted(v)` for every value type, `Rejected(code)` and `Exhausted(kind)`, and SHALL publish `out_d = H(outcome_kind, value_d, n_steps)` with `value_d` the digest of the canonical value encoding. A segment whose final state is `Running` SHALL NOT be sealed. |
| GPR-008 height | Owner-step height SHALL be range-checked below 2^64 and SHALL count machine steps only. |
| GPR-009 segments | The node SHALL link two segments only if they have the same `leaf_id`, `prog_d` and `input_d`, `left.post_d = right.pre_d` and `left.h_hi = right.h_lo`. |
| GPR-010 claims | A claim SHALL be `H(CLAIM, kind, id, in_d, out_d)`. A set statement SHALL be `(set_root, n_claims)` where `set_root` is computed in circuit from the children's roots with domain-separated tags for claim leaf, join and dependency. |
| GPR-011 consumption | When a run consumes claims, its state SHALL carry the commitment `deps_d` to them, and the run SHALL be sealed only together with a verified set whose root equals `deps_d`. |
| GPR-012 registry | The foreign lift F SHALL compute `vk_id` from the witnessed adapter key, SHALL prove membership of `(vk_id, kind)` and of the adapter's published inner `vk_id` under `reg_root`, SHALL resolve the witnessed key's commitments in circuit, and SHALL fold the adapter's published accumulator with the adapter proof's own. `vk_id` SHALL bind the arch, k, exact public-input count, format versions, transcript and SRS identity. |
| GPR-012b run entry | The seal rule SHALL prove that the Sherlock leaf identity carried by the segment is a registry member of kind `run` under `reg_root`. |
| GPR-013 registry root | Every node SHALL require its children to carry its own `reg_root`. The contract SHALL accept a root only if the proof's `reg_root` equals the contract's registry root. |
| GPR-014 adapters | An adapter SHALL prepare one proof of its class under a witnessed key with the exact public-input count of that key, SHALL publish that key's `vk_id`, the digest of the complete public-input vector and the resulting accumulator, and SHALL NOT publish a value that is not a public input or an opened commitment. |
| GPR-015 format | Where a claim asserts that ledger proof bytes were verified, the adapter SHALL enforce the ledger transcript, encodings, instance layout and complete parsing. Where the proof was made with another transcript under the same key, the claim kind SHALL say so. |
| GPR-016 shape | The decided statement SHALL carry `set_root`, binding the claim DAG by content, and `shape_d`, binding the proof graph including slot bits, unary nodes and segment bracketing. Acceptance SHALL NOT depend on an off-chain manifest. |
| GPR-017 completeness | The decider SHALL refuse a root that is not a set statement. |
| GPR-018 settlement | The contract SHALL accept each decided root at most once and SHALL record each opened claim at most once, keyed by the claim identifier, whatever root it is opened from. |
| GPR-019 size | The final proof and the decider's public-input count SHALL be the same for every shape, depth and number of claims. |

### 6.2 `D/design.md`

1. **D4, §2.3, §2.4**: the leaf is U over committed state. Replace "S a keyed constant ...
   option F" with the leaf statement of section 1.2; replace `prog_d =
   Poseidon(dom, programImageDigest, layoutSchemaDigest, boundsDigest, profileId)` with
   `prog_d = H(PROG, prog_root, machine version, rule inventory id)`; remove `Bounds` and the
   layout caps from the leaf key identity; add the memory table of section 2.2.
2. **D5**: statement `sherlock-leaf-stmt/2 = [binding, prog_d, input_d, pre_d, post_d, h_pre,
   steps, deps_d]`.
3. **D6, §3.1**: replace the chain diagram with the layers of section 1.2. State that
   `SHKIVC06` is milestone M0 and is replaced by N, R, F, C.
4. **§3.2**: add GPR-003 (canonical disabled slots) beside the genesis-carry rule; add the
   named-key slot and GPR-004.
5. **§3.3**: closing proof C; decider public values `[reg_root, set_root, n_claims, shape_d]`;
   contract `SherlockClaims`.
6. **D13'**: "closed: the three-slot node is replaced by the two-slot node; a k=21 node is a
   recorded fallback (experiment 8)".
7. **§3.6**: replace the single-program segment model with the typed composition of section
   1.2 (SEG, claim, SET; seal, dep, join); keep segment linkage as the within-run relation;
   state the linear and reusable objects of section 2.4.
8. **New §3.8 Registry and adapters**: section 2.3, including the SRS-identity rule and the
   statement that a contract-circuit claim is circuit satisfaction, not inclusion.
9. **§5**: the limits table of section 2.5.

### 6.3 `D/requirements.md`, `D/tasks.md`, `D/dissent.md`

- SRC-013 and SRC-014 (U or F, S rule): restate around leaf work per proof with committed
  memory; option F is withdrawn (the registry replaces "finite family with key membership").
- SRC-017: replaced by GPR-001 to GPR-019.
- Tasks: add M1 to M6 of section 4 as task groups after the current M5; add experiments 1 to 8
  as gated tasks before them; mark 5.7 and 5.8 (tree) "superseded by M1/M2".
- Owner questions for `dissent.md`: (1) may new work enter the node as a verified proof in
  one of its two slots, or must one node take two prior proofs and new work (inline work, or
  k=21); (2) does a Poseidon-transcript proof under the deployed key satisfy "a proof from a
  Compact contract circuit", or must ledger bytes be verified before GP-3 is met; (3) is a
  tree depth equal to the address width "capacity not bound to the key", or must storage be
  expandable; (4) must "other PLONK/halo2 circuits" include circuits outside the Midnight
  stdlib configuration; (5) is the committed "set of claims" a content hash with occurrences
  or a duplicate-free set; (6) registry policy: who admits adapter and run entries, and
  whether old roots stay valid.
