# Fable design 1: general-purpose recursion architecture (independent, round 1)

Run `sherlock-gp-architecture-20261110`. Written before reading any Astra text. Nothing was built,
run or proved. ESTIMATE marks a computed or reasoned number.

Conventions. Paths are relative to the repository root. `D` =
`openspec/changes/sherlock-vm-consensus-20261107`. `L:<path>:<line>` is the ledger at tag
`ledger-9.1.0.1` read with `git show` (not the working tree). `MC:` =
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.3.0/src`, `ZS:` =
`.../midnight-zk-stdlib-2.4.0/src`, `MP:` = `tools/profile/vendor/midnight-proofs-0.8.3/src`.

## 0. Six facts that decide the design

F-1. **The in-circuit verifier is specialised to one constraint system and one k.** `AssignedVk`
holds `domain` and `cs` off-circuit; "the only in-circuit field is the `transcript_repr`"
(`MC:verifier/mod.rs:55-66`). A slot whose key is chosen by witness therefore works only across
keys that share the constraint system and the domain.

F-2. **A Midnight constraint system is a function of `(ZkStdLibArch, k)`.** `MidnightVK::read`
rebuilds it with `ZkStdLib::configure(&mut cs, (architecture, k - 1))` (`ZS:lib.rs:1573-1586`);
the arch is 13 chip flags plus `nr_pow2range_cols` (`ZS:lib.rs:159-209`). ZKIR v3 picks the arch
per circuit from the instructions used (jubjub, poseidon, sha2_256, keccak_256, secp256k1, p256,
curve25519; `bls12_381: false`, `nr_pow2range_cols: 4`; `L:zkir-v3/src/ir_vm.rs:1212-1263`) and k
by `optimal_k` (`L:zkir-v3/src/ir.rs:105-106`; `ZS:lib.rs:2122`). So Compact circuits fall into
many `(arch, k)` classes, and none of them is the node's class (`bls12_381 + poseidon`,
`crates/mori-recursion/src/lib.rs:23-30`).

F-3. **Ledger proofs use a Blake2b transcript; the gadget verifies Poseidon transcripts only.**
`pub type TranscriptHash = blake2b_simd::State` (`L:transient-crypto/src/proofs.rs:64-65`), used
by `IrSource::prove` (`L:zkir-v3/src/ir.rs:82-102`) and `VerifierKey::verify`
(`L:transient-crypto/src/proofs.rs:563-575`). The gadget's `SelfEmulation::Hash` is
`PoseidonState` for both emulations (`MC:verifier/types.rs:144,149,191,196`). A proof taken from
a Midnight transaction cannot enter the node as it is. The key is transcript-independent (keygen
uses no transcript; `transcript_repr` is always Blake2b of the key, `MP:plonk/mod.rs:255-278`),
so the same Compact circuit can be proved again under its deployed key with a Poseidon
transcript by whoever holds the witness.

F-4. **Fixed-base work is deferred by name, and only for keys the decider knows.** A partial MSM
keeps fixed-base terms as scalars; "the bases are assumed to be fixed and globally known"
(`MC:verifier/msm.rs:14-17,43-53`). The node leaves its own fixed-base scalars unresolved and
"whoever pins the node key (the decider) resolves them"; the leaf key is a constant and its bases
are resolved in circuit (`crates/mori-recursion/src/ivc.rs:10-15`). A key taken from a registry by
witness has neither route: its fixed and permutation commitments must be witnessed as points,
hashed to the registry leaf, and multiplied in circuit as variable bases. `prover.md:155` puts
one variable base at 3-4k rows (ESTIMATE in that file).

F-5. **Measured rows at 7.3.0** (`qa/sherlock/decisions/speed-levers.json:22-30`;
`qa/sherlock/manifests/node.json`): node self + stand-in leaf 652,437; self + C6 digest leaf
741,708; self + self 730,243; self + self + leaf 1,078,147; self + self + leaf + leaf 1,278,670;
decider 929,166 (`qa/sherlock/manifests/run-decider.json:20`). 2^20 = 1,048,576. Two collapses are 78% of the node
(`speed-levers.json:66`, "collapse_self 239,577 + collapse_final 267,514"). Marginal slot costs
from these numbers: second leaf slot +200,523; first leaf slot on a self-self node +347,904.

F-6. **Hashing state is what binds capacity to the key.** The leaf hashes every carrier lane
(`crates/mori-backend/src/sherlock_c6_child.rs:551-569`, per `review.md` C5). Poseidon here is
width 3, rate 2, 8 full + 60 partial rounds with 5 skips (`MC:hash/poseidon/constants/mod.rs:17-26`;
`MC:hash/poseidon/poseidon_chip.rs:50`): 18 rows per permutation, 9 rows per absorbed lane
(ESTIMATE from the round counts; not a `cost_model` reading). At the holdout draft config one
carrier side is about 35k lanes (`docs/sherlock/stagenet-gp-recursion.md:157-158`): about 315k
rows per side per step (ESTIMATE), before any VM logic.

## 1. Recommended architecture (one page)

Name: **typed PCD over KZG accumulation, with named lifts and a claim tree.** It keeps the
proof system (midnight-proofs PLONK/KZG, in-circuit verifier with deferred pairing) and changes
the node from "self + one constant leaf" to "two polymorphic slots over a fixed set of named core
keys", moves every non-core proof behind a lift circuit, and replaces whole-state hashing in the
leaf with committed memory.

```
L0  base proofs (any number of keys, all in the registry or core)
      U    universal Sherlock leaf: S steps over committed state        [core key]
      X_c  foreign circuits (Compact contract circuits, other PLONK)   [registry, kind=direct]
      A_c  adapter per (arch, k) class: verifies one X_c proof under a
           witnessed key, emits a common-form claim                    [registry, kind=adapter]
L1  lifts (node class: bls12_381 + poseidon, k=20; core keys, named in N and D)
      R    run lift: verifies 2 leaf proofs of U (constant key), links them -> SEG
      F    foreign lift: verifies 1 node-class proof under a witnessed key,
           resolves its fixed bases in circuit, proves registry membership -> SET
L2  node N (one key, self-recursive, 2 slots; each slot is one of {N, R, F}, or off)
      SEG + SEG -> SEG      (adjacent segments of one run; Lean `link`)
      SEG       -> SET      (seal: canonical genesis and terminal outcome -> one RUN claim)
      SEG + SET -> SET      (seal with dependencies: the run's `deps_root` equals the SET root)
      SET + SET -> SET      (join: root = H(JOIN, left, right))
L3  decider D (k=20, Blake2b transcript, ledger template): verifies the last N proof, resolves
    the named bases of N, R and F, checks the pairing, exposes [reg_root, set_root, n_claims]
L4  one contract `SherlockClaims`: `acceptRoot`, `openClaim`, `admitKey`
```

Statements.

- Leaf U: `[binding, prog_d, input_d, pre_d, post_d, h_pre, steps, deps_d]`. `pre_d`, `post_d`
  are digests of a small register file plus memory commitments (section 3). The first leaf
  (`h_pre == 0`) proves in circuit that the pre-state is `Init(prog_d, input_d)`.
- SEG: `(prog_d, input_d, pre_d, post_d, h_lo, h_hi)`. The `Seg` of `Aggregation.lean:27` plus
  `input_d`.
- Claim: `c = H(CLAIM, kind, id, in_d, out_d)`. RUN claim: `id = prog_d`, `in_d = input_d`,
  `out_d = H(outcome_kind, value_d, n_steps)`. FOREIGN claim: `id = vk_id`, `in_d = 0`,
  `out_d = H(public inputs)`.
- SET: `(set_root, n_claims)`; `set_root` is the hash of the aggregation DAG itself, with
  domain-separated node tags (JOIN, DEP, leaf claim). The shape is the statement.
- Node public inputs: `[n_repr, r_repr, f_repr, reg_root, tag, payload(6), acc]`. `n_repr`,
  `r_repr`, `f_repr` are the transcript representations of the three named keys; children must
  carry the same three values and the same `reg_root`; the decider pins all three.

Key set (fixed, program-independent): `{U, R, F, N, D}`. Everything else is data: programs
(`prog_d`), inputs, registry members (foreign keys, adapter keys), the shape.

How each requirement is met.

- **GP-1.** U is keyed over the rule inventory and a fixed register file, not over `Bounds`.
  Program and input are opened from commitments; memory is committed (hash-chained stack,
  Merkle arena and program table); all three outcome kinds are sealed as `out_d`. Capacity is
  bounded by address width, not by the key.
- **GP-2.** A sealed run is a claim. Batching is `SET + SET`. Consumption is `SEG + SET` with
  `deps_root` carried in the run state; each claim is opened from `set_root` by a Merkle path.
- **GP-3.** The registry is a Poseidon Merkle root held by the contract. F proves membership in
  circuit and takes the key by witness. Adapters map public inputs to the claim form. A Compact
  circuit enters through the adapter of its `(arch, k)` class, as a Poseidon-transcript proof
  under its deployed key (stage 1) or as the on-chain Blake2b proof through a transcoding
  adapter (stage 2).
- **GP-4.** One node key, two slots, any nesting. A proof can be an input to two parents, so a
  DAG needs no extra mechanism. The final proof is always one decider proof.

What cannot be met as written, and the closest design.

1. **"A node can take at least two prior proofs plus new work"** in a single node at k=20: no.
   Three verifier slots measure 1,078,147 rows (F-5). The recommended node takes two proofs, each
   either prior (N) or new (R, F); a three-input step is two nodes. The literal reading is
   available as fallback B (one three-slot node at k=21, section 2). Whether two binary steps
   satisfy the sentence is an owner question; the GP-4 acceptance test (chain, tree, DAG at depth
   >= 8, one key set, equal proof size) is met either way.
2. **"Other PLONK/halo2 circuits on the same curve"** without a per-class adapter key: no. The
   verifier circuit is a function of the verified circuit's constraint system (F-1). The closest
   design is one adapter key per `(arch, k)` class, generated mechanically from one generic
   relation and admitted to the registry like any other key. Core keys and the contract do not
   change. Whether adapter keys count against "one key set" is an owner question. Circuits that
   break the gadget's assumptions (more than one instance column or challenge phase,
   `MC:verifier/verifier_gadget.rs:16-20`) need gadget work before any adapter exists.
3. **A proof already posted on Midnight** (third-party, no witness): not in stage 1 (F-3). It
   needs an in-circuit Blake2b transcript, which the gadget does not have; the stdlib has a
   Blake2b chip (`ZS:lib.rs:186-187`), so this is engineering with an unmeasured row cost
   (experiment X6), not a stack limit.

No requirement needs a different proof system. Folding or a STARK is an optional later
replacement for the leaf prover only (section 2).

## 2. Q1: accumulation scheme

| Option | Node rows / k | Final proof | Prover cost per node | In 0.8.3 / 7.3.0 | Call |
|---|---|---|---|---|---|
| A0. Today: full in-circuit verification, pairing deferred; self + constant leaf | 652,437-741,708, k=20 (measured) | decider, 6,432 B class | 61-67 s, 16.6-17.6 GB (measured) | yes | Baseline. Cannot do arity 2 plus a leaf at k=20 (1,078,147). |
| **A. Named lifts**: node of 2 polymorphic slots over named core keys {N, R, F}; all fixed bases deferred by name | ESTIMATE 740-800k, k=20 (self-self is 730,243 measured; plus selection logic, tag rules, three name sets in the accumulator encoding) | same | ESTIMATE as today (same domain) | gadget and accumulator yes; **bit-selected names need a small change**: `fixed_base_scalars` is private (`MC:verifier/msm.rs:50-64`) | **Recommended.** |
| B. Monolithic three-slot node {self, self, base by witnessed key} at k=21 | ESTIMATE 1.25-1.35M (1,078,147 + in-circuit resolution of a witnessed key, 30-45 bases at 3-4k rows) | same; decider stays k=20 if its rows hold | ESTIMATE 2x time, about 33 GB (the step is 88% domain-proportional, `speed-levers.json` `d`) | yes, no gadget change; needs an SRS above 2^20 (only `bls_midnight_2p20` is in the local cache) | Fallback. Simpler, about 2x the node cost. |
| C. Per-level or per-shape keys | fits | same | as today | yes | Rejected: violates GP-4 "no per-depth key". |
| D. Folding over the same curve (Nova, HyperNova, ProtoStar, ProtoGalaxy) | folding verifier is 1-3 scalar multiplications (https://eprint.iacr.org/2023/1192 abstract: 2 Nova, 1 HyperNova, 3 ProtoStar); here they are non-native, ESTIMATE 10-40k rows | needs a new decider | fast steps | **nothing**: no folding prover in midnight-proofs; BLS12-381 has no curve cycle (`tree.md:112`), so CycleFold does not apply | Rejected for the node (below). Open as a later leaf prover. |
| E. Inner STARK or folding layer, outer KZG wrap | wrap circuit ESTIMATE >= 2^20-2^22 rows of Poseidon for a FRI verifier | decider | fast inner | nothing | Rejected now; same remark as D. |
| F. Upstream unreleased gadget cuts (5-15% fewer verifier rows, `prover.md:123`) | helps A and B | changes proof format | - | no (unreleased; changes VKs and the proof format) | Watch; not a plan input. |

Why A works. The decider already resolves the node's own bases by name. Nothing limits that to
one name. R and F do not verify N, so there is no key cycle: R and F are keyed first, N takes
their `transcript_repr` as public inputs exactly as it takes its own
(`assign_vk_as_public_input`, `MC:verifier/verifier_gadget.rs:207`), and D pins the three
values and resolves three sets of constant bases. R, F and N must share one constraint system
and k (F-1), which holds when all three are node-class relations at k=20. In a slot, the
witnessed `transcript_repr` is constrained to be `b_N*n_repr + b_R*r_repr + b_F*f_repr` with a
one-hot `(b_N, b_R, b_F)`, and each deferred scalar `s_j` is written under name set N, R or F as
`b_x * s_j`. That is native multiplication only.

Why not D for the node. GP-3 inputs are PLONK/KZG proofs. A folding scheme folds instances of
one relation; to bring a foreign proof in, the folded step circuit must contain the PLONK
verifier, which is the same non-native MSM that fills the node today. Folding removes the
recursion overhead only for work that is native to the folded relation (VM steps). Its decider
on this stack must also prove the folded witness commitment opens correctly inside a circuit
that the stock Midnight verifier accepts, which means an in-circuit pairing for a KZG opening
over the whole folded witness; Mira (https://eprint.iacr.org/2024/2025) is the closest
published work on folding pairing-based arguments and has no implementation on this stack.
Because any node-class proof can enter through F, a folded-run prover can be added later as a
registry member that emits RUN claims, without touching the core keys.

What R costs. R = {leaf, leaf} with a constant leaf key: ESTIMATE 650-800k rows from
self + leaf 652,437-741,708 (one of the two slots there is a self slot with deferred bases, so R
is somewhat larger; it depends on the leaf arch, as the +89,271-row step from the stand-in to
the C6 leaf key shows). F = one verification plus witnessed-key resolution plus a registry
path: ESTIMATE 550-700k. Both need a row probe (X1).

Throughput against today's chain (ESTIMATE, node step 61-67 s): a run of M leaves needs M/2 R
proofs and M/2 - 1 merge nodes, about M node-class proofs, the same count as the chain, and
they are independent, so P workers divide the wall time. Proof size and decider time are
unchanged.

Soundness note. Depth: known PCD analyses "incur expensive blowups" with depth unless the SNARK
has a straightline extractor (https://eprint.iacr.org/2023/1646). A balanced tree has depth
log M where the chain has depth M (`tree.md` F6). The accumulation is the BCMS pattern
(https://eprint.iacr.org/2020/499).

## 3. Q2: universal VM (GP-1)

Principle: the leaf sees a fixed-size register file and commitments; everything of unbounded
size is behind a commitment that the step opens where it reads or writes.

State digest `sd = H(SD, control registers, outcome, fuel and counters, stack_h, arena_root,
arena_len, env handle, deps_d)`; `prog_d = H(PROG, prog_root, profile, rule inventory id)`;
`input_d = H(IN, input value digest, fuel, stack limit, deps_root)`.

| Structure (current carrier) | Commitment | Cost per access (ESTIMATE, 18 rows per Poseidon permutation) |
|---|---|---|
| Program table (PROG section, immutable) | Merkle root `prog_root`, depth 20 | read: 20 permutations = 360 rows |
| Frames (13 lanes each, a stack) | hash chain: `stack_h' = H(frame, stack_h)`; top K frames in registers | push or pop: 7 permutations for 13 lanes = 126 rows |
| Data arena (append-only, child before parent; `D/design.md` §2.1 `StepInv`) | append-only Merkle tree, depth 32, with `arena_len` | read: 576 rows; append: 1,152 rows |
| Environment | persistent structure in the arena (a pointer), or its own Merkle map | as arena |
| Helper cursors, value slots in use | registers | 0 |

Why Merkle and hash chains and not a lookup or multiset memory argument. A permutation or
LogUp memory argument needs a challenge drawn after the whole access log is committed. In a PCD
of arbitrary shape and unbounded length there is no such point. A multiset hash into a curve
(https://arxiv.org/abs/1601.06502) needs no challenge and is additive, so it would compose; it
costs about one hash-to-curve per record and needs a final sweep over every touched cell and a
linear-use discipline. It is the cheaper alternative if Merkle paths dominate the leaf (X3
decides). Merkle and hash-chain state is local: any segment is provable from `pre_d` alone, in
any order and in parallel, which is what trees need. RISC Zero's continuations use the same
split, paged memory with Merkle page-in and page-out per segment
(https://dev.risczero.com/api/zkvm/optimization).

No per-program key: the rule inventory and the register file fix the relation. A program is a
Merkle root; a run length is a height below 2^64 (range checks exist, `ivc.rs` `constrain_heights`).
A program "larger than one leaf's carriers" does not exist as a category any more.

Outcomes. The terminal step writes `(outcome_kind, value_d)`; `value_d` is the digest of the
canonical value encoding (for a structured value, its arena root and handle). The seal rule in N
requires `outcome_kind != Running` and emits `out_d`. `Rejected(code)` and `Exhausted(kind)` are
ordinary terminal states.

Genesis. With committed memory the initial state is a few constants plus `prog_root` and
`input_d`, so the first leaf can prove `pre == Init(prog_d, input_d)` in circuit at a cost of one
state hash. This closes dissent 2 of `review.md` 4.2 in favour of the in-circuit mechanism.

Cost against today. A step that makes a program read, 2 arena reads, 1 arena append and 1
frame push costs about 360 + 1,152 + 1,152 + 126 = 2,790 rows of hashing (ESTIMATE), against
about 630k rows for hashing two 35k-lane sides at the holdout config (F-6) and about 1.7k rows
at the 96-lane qualification config. The per-rule logic cost is still unknown (4.1a is running).

Risk: this replaces the `sherlock-c6-phys/1` carrier model for unbounded sections. The
fragments read lanes of the bounded image today. How much of each fragment survives depends on
how many distinct cells a rule touches (X2).

## 4. Q3: registry and adapters (GP-3)

Key identity. `vk_id = H(VK, class_id, transcript_repr, comm_d)` where `class_id = H(arch, k,
n_public_inputs_cap)` and `comm_d` is the Poseidon digest of the fixed and permutation
commitments in key order. `transcript_repr` already binds the commitments, the domain and the
constraint system by Blake2b (`MP:plonk/mod.rs:255-278`); `comm_d` is there because the circuit
needs the points and cannot recompute Blake2b cheaply. `vk_id` is a deterministic function of
the key bytes, so anyone can recompute it from a deployed Midnight verifier key.

Registry. A Poseidon Merkle tree of leaves `H(REG, vk_id, kind)`, `kind` in {direct, adapter,
run-prover}. The root `reg_root` is contract state. `admitKey` appends. Admission policy is the
owner's choice: governance key, or permissionless for `direct` (a direct key can only make
claims about itself) and governed for `adapter` and `run-prover` (they make claims on behalf of
other identities).

In circuit (lift F). F witnesses the node-class key's commitments, computes `comm_d` and
`vk_id`, proves the Merkle path to `reg_root`, verifies the proof with the gadget, resolves the
fixed bases as variable bases (F-4), and emits the claim: for `direct`, `id = vk_id` (computed,
cannot be forged); for `adapter`, `id` and `out_d` are read from the adapter's public inputs.

Adapters. `A_c` is a node-class relation, generic over `(arch, k)`, that verifies one class-c
proof under a witnessed key, computes that key's `vk_id`, hashes its public inputs to `out_d`
and exposes `[vk_id_inner, out_d]`. Its key is reproducible by anyone from `(arch, k)`, so its
admission can be checked by recomputation. Public inputs: the instance polynomial is evaluated
in circuit, linear in the number of inputs; each adapter fixes a cap (for example 1,024) and
shorter statements are zero-padded. The ledger limit is 65,536 per call
(`L:onchain-runtime/src/transcript.rs:87` per `review.md` C7); an adapter at that cap is a
separate, large class.

Compact contract proofs. Yes, with two qualifications. (1) Format: a ledger proof is the stdlib
PLONK proof under `MidnightVK` (arch, k, public-input count, key; `ZS:lib.rs:1537-1562`) with a
Blake2b transcript (F-3). Stage 1 verifies a Poseidon-transcript proof of the same circuit under
the same key; the statement proved, "this circuit accepts these public inputs", is identical.
Stage 2 adds a Blake2b transcript gadget to the adapter. (2) The repository has verified
ZKIR-compiled children in circuit before (the C6 leaf is a ZKIR relation with a Poseidon
transcript, `qa/sherlock/manifests/c6-leaf.json`), so the gadget handles ZKIR arch classes. What
a Compact proof means to a consumer is its public-input vector (the communication commitment
and the transcript of ledger operations); the adapter does not interpret it, it commits to it.

Cost of the path for one foreign proof: adapter proof (k=20 class, one verification with
in-circuit bases) then F (one verification with in-circuit bases) then a node slot. Two
node-class proofs before aggregation. Sherlock leaves skip this through R.

## 5. Q4: PCD shape (GP-4)

Node: two slots, each `off` or one proof under N, R or F. Rules in section 1. The all-off node
is refused. A single-child node is allowed (`unary_left`, `Aggregation.lean:451`).

Shape binding. `set_root` is defined by structural recursion: `leaf(c) = H(LEAFCLAIM, c)`,
`join = H(JOIN, l, r)`, `dep = H(DEP, c_run, deps_root)`. Two different DAGs give different
roots under collision resistance; the verifier learns the exact combination. A SEG carries no
shape, by design: `link` is associative (`link_assoc`, `Aggregation.lean:291`) and
`same_leaves_same_stmt` (`:708`) says the bracketing of a run does not matter. The shape that
GP-4 asks to bind is the shape over claims.

DAG fan-out. A proof is data. The same SET proof for claim A can be the right child of the
seal node of B and of the seal node of C. A is proved once and verified twice. If B and C are
later joined, A's sub-root appears twice in `set_root`; that is a true statement about the DAG.

Constant size. Every N proof has the same public-input length; the decider proof is the final
object in every shape.

Linear and reusable objects. Steps of one run are linear: each height is covered exactly once.
That is the existing `Covers` and `linkAll_covers` (`Aggregation.lean:787,858`) and Quint I3
(`aggregation.qnt:410`); SEG merging keeps them unchanged because adjacency
(`left.post == right.pre`, `left.hi == right.lo`) makes a repeat or a gap unprovable. Claims are
facts and are reusable. On-chain acceptance is linear again, by the contract's set of accepted
roots.

Unique genesis. Today the decider checks the root starts at height 0 from `init_d` (Quint I6,
`aggregation.qnt:441`; `non_genesis_refused`, `Aggregation.lean:1106`). In the new node the seal
rule does it: `h_lo == 0` and the leaf's in-circuit `Init` check. `result_unique`
(`Aggregation.lean:1136`) then reads: for a given `(prog_d, input_d)` there is one RUN claim.

Model extensions needed: a claim layer above `Seg` (inductive `SetCert` with join, dep, seal),
the theorem "every claim under a decided `set_root` is sound" by induction, shape injectivity,
and in Quint a second linear resource (accepted roots) plus the fan-out scenario.

## 6. Q5: on-chain fit

One contract for all programs, `SherlockClaims`:

- state: `reg_root: Field`, `roots: Set<Field>`, optionally `claims: Set<Field>`.
- `acceptRoot(set_root, n)`: verifier key = decider D (the same "hand-written key behind a
  Compact template" mechanism as `acceptRun`, `tools/sherlock-sdk/sherlock-run.compact:1-15`).
  The template reads `reg_root` from the ledger so the decider binds the proof to the current
  registry, as `acceptRun` does for `program_d` (`:40-47`); refuses a root already in `roots`.
- `openClaim(set_root, claim fields, path)`: an ordinary Compact circuit; proves a Merkle path
  from the claim to an accepted root and publishes the claim. Requires the claim tree to use
  the hash Compact exposes as `transientHash` (Poseidon in the stdlib arch,
  `L:zkir-v3/src/ir_vm.rs:1212-1263`); parameter equality with `MC:hash/poseidon` must be
  checked (X7).
- `admitKey(vk_id, kind)`: registry append under the admission policy.

Limits. Decider key 2,217 B against 50,000 (`review.md` C7); public inputs: about 70 today, far
under 65,536; three circuits against 256 per contract. The decider stays k=20 only if resolving
three named base sets fits in its 119,410 rows of headroom (929,166 rows today, F-5). If it does
not: a two-stage decider (stage 1 resolves bases and collapses, node class, Poseidon; stage 2
is today's pairing decider over stage 1), at one more 70 s proof per final proof.

The single-run product (`acceptRun` with a result in ledger state) is `acceptRoot` plus
`openClaim` in one transaction, or a thin second template over the same decider.

## 7. Q6: migration

Carries over: the verifier gadget use, accumulator encoding, collapse and genesis-carry
constraints of `ivc.rs`; height range checks; the pairing decider (`run_decider.rs`,
`pairing.rs`) and the template mechanism; the Lean `Seg`/`link`/`NodeCert` algebra and the
Quint slot model; the rule fragments' logic; the witness stream as producer; the profiling
harness and row probes.

Replaced: node `SHKIVC06` (by N, R, F); the decider's public values and the `acceptRun`
contract (by `acceptRoot`); the leaf's carrier hashing and `prog_d` definition; the
one-contract-per-program deployment; `Bounds` as a key input.

Milestones, each with working software:

- M-A (now, no new design): finish the linear chain on P1-P3 as planned. It stays the first
  milestone (`general-purpose.md` Consequences) and produces the measured leaf numbers.
- M-B GP-4 core: N with named lifts and R over the current leaf statement plus `input_d`;
  decider with `[set_root, n]`; `SherlockClaims` with `acceptRoot`/`openClaim`. Acceptance:
  chain, balanced tree and fan-out DAG at depth >= 8 over stand-in leaves; then GP-2 (b).
- M-C GP-2 (a) and outcomes: `deps_d` in the leaf, seal-with-dependencies, all outcome kinds,
  run input, in-circuit genesis.
- M-D GP-3: F, the registry, one adapter class; a Compact circuit proved with a Poseidon
  transcript; then the Blake2b adapter.
- M-E GP-1 capacity: committed memory in U, in the order program table, frames, arena.
  A program larger than the old carriers runs.

M-B before M-E because the node design does not depend on the leaf's inside, and M-E is the
largest rewrite. M-E's statement (`pre_d`, `post_d` are opaque digests to N) is fixed at M-B, so
re-keying U at M-E changes only the U constant in R (R, N and D are re-keyed once then).

## 8. Q7: experiments, ranked

No heavy job is needed for X1-X3 and X7; they are `cost_model` row probes (minutes, a few GB).

| # | Hypothesis | Measurement | Kill criterion | Cost |
|---|---|---|---|---|
| X1 | A two-slot node over three named keys fits k=20 with margin; R and F fit k=20 | `cost_model` rows of N {poly, poly}, R {leaf, leaf}, F {witnessed key + path}; needs the name-selection change in a patched gadget | N or R above 0.95 x 2^20 -> fallback B | 1-2 days of code, minutes to run |
| X2 | Committed memory makes the leaf independent of `Bounds` at a per-step cost below today's qualification-config hashing plus 3k rows | count distinct cell reads and writes per rule from the reference machine; rows of one step with Merkle and hash-chain openings | median step above 8k rows of memory work | 2-3 days, no proving |
| X3 | The decider resolves three named base sets within k=20 | `cost_model` rows of D over N from X1 | above 2^20 -> two-stage decider, measure its second proof | hours after X1 |
| X4 | Fallback B is affordable: three-slot node at k=21 | prove time and RSS at k=21 (needs a 2^21 SRS) | above 48 GB or above 2.5x the k=20 step | one heavy job, about 1 h |
| X5 | An adapter for the commonest Compact class fits k=20 | rows of `A_c` for (jubjub + poseidon, k of a real contract circuit), public-input cap 1,024 | above 2^20 | 1-2 days, minutes to run |
| X6 | Blake2b transcript in the adapter fits k=20 | rows of the Blake2b chip per block x transcript bytes of a real Compact proof | above 2^20 with one verification | 2 days |
| X7 | Compact `transientHash` equals the node's Poseidon | hash one vector both ways | mismatch -> claim tree hashed with the Compact hash in circuit, or `openClaim` as a hand-written key | 1 hour |
| X8 | Concurrency: P=3 node-class provers hold f3 <= 1.75 with leaf provers beside them | wall and RSS, mixed load | throughput gain below 1.5x | one heavy job |

X1 decides between A and B. X2 decides whether GP-1 capacity is a leaf rewrite or a paging
layer under the current carriers. X3 and X5 decide the key count and the decider shape.

## 9. Open points I am least sure of

- The row estimate for N depends on how the accumulator's public-input encoding grows with
  three name sets (63 instance lanes today, `prover.md:10`).
- Whether Compact circuits always have one instance column and one phase is inferred from the
  stdlib's fixed configuration (`ZS:lib.rs:14-27`), not checked on a compiled contract.
- `midnight-aggregation` 0.1.0 exists (https://docs.rs/crate/midnight-aggregation/latest,
  "Toolkit for proof aggregation of midnight-proofs"); `tree.md` F5 describes it as a linear IVC
  aggregator. I did not read its source. If it already offers witnessed-key slots, F is smaller
  work.
- The seal-with-dependencies rule fixes the order of dependencies in `deps_root`; a run that
  awaits claims dynamically (`/2` profile, `D/design.md` D8) needs `deps_d` built in the state.
