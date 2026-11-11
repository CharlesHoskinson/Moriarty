# Fable: independent answer (written before reading the other members)

Paths are relative to `/home/hoskinson/Projects/Moriarty`. `design.md`, `requirements.md`, `tasks.md` and `general-purpose.md` are in `openspec/changes/sherlock-vm-consensus-20261107/`. `owner_runtime.rs` is `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs`. Ledger paths are in `repos/midnightntwrk/midnight-ledger` at tag `ledger-9.1.0.1` (`a2b30128`), read with `git show`. Registry paths are in `~/.cargo/registry/src/*/`. Nothing was built, tested or proved.

## Q1 Outcome-only publication

Verdict: **adopt with changes**.

### 1. Soundness and completeness

Outcome-only publication is sound and it is necessary. It is not sufficient alone: four changes go with it (C1 to C4 below).

Facts:

- Today the decider publishes `[prog_d, init_d, fin_d, n_steps, result_be]` and `stmt = Poseidon(SHK_RUN, prog_d, init_d, fin_d, n_steps, result)` (`crates/mori-recursion/src/run_decider.rs:8-12,48-56,289-301`). `fin_d = Poseidon(SD, HALTED, result, lanes_d)` with `lanes_d` a free witness (`run_decider.rs:286-288`). `lanes_d` hashes the raw carrier (`crates/mori-backend/src/sherlock_c6_witness_stream.rs:216-241`).
- The contract refuses a replay by `stmt` only: `assert(!runs.member(disclose(stmt)))` (`tools/sherlock-sdk/sherlock-run.compact:39-46`). `fin` is an argument of `acceptRun`, but the contract does not store it.
- Under D3 every leaf takes one segment of C = 8 addresses, and the prover chooses where a leaf closes (`design.md` §2.7.7, §2.7.8 D3; `qa/gemini-quint/chipleaf-pairrow-m3/verdict.md:41`).

Consequence that the brief does not state: with `fin_d` public under D3, one run of `(prog_d, input_d)` has many valid `fin_d` values, so it has many valid `stmt` values. The same run is then accepted more than once, because the replay check keys on `stmt`. This defeats the "run already accepted" refusal of SRC-007. It is the strongest reason to remove `fin_d`: the defect is not only that the value is not canonical; an acceptance rule depends on it.

With `stmt = Poseidon(SHK_RUN, prog_d, input_d, outcome_d, n_steps)`:

- Two runs of the same `(prog_d, input_d)` with different partitions publish the same statement, if these hold:
  - the machine is deterministic (one owner-engine event per step, `design.md` §2.1);
  - no step reads an address as a number. `RefC` derives only `Clone, Copy, PartialEq, Eq, Debug` (`crates/mori-core/src/sherlock/helper_machine/shared.rs:701-705`). Equality of values is structural: the Equal helper walks fields (`helper_machine/families/structural.rs:133-200`), it does not compare references. I re-read these two. The other citations of `research/sherlock-processor-analogy-20261110/experiments-4.md` §4 (`owner_si21_closure.rs:131-195`, `shared.rs:1635-1639`) I did not re-read;
  - `n_steps` counts enabled step slots only. A disabled slot contributes zero (`design.md` §2.2 item 5, line 41), and the node requires `1 <= steps <= S_MAX` per leaf (`crates/mori-recursion/src/ivc.rs:24,360-372`);
  - `outcome_d` is a function of the value, not of its placement (section 2).
- The prover cannot change the outcome with the partition, if `Exhausted(kind)` is decided only on logical counters. Today the capacity kinds are decided on the logical allocation counters (`owner_runtime.rs:3140,3820,4557-4560`; kinds at `shared.rs:367-378`). The chip leaf must keep that: no outcome may be derived from `len`, `fp`, `tp`, a segment index or the free address space (change C2).
- The prover can still decide whether a long run is provable at all. `base / 8 < 2^17` (`experiments-4.md:148-149`), and every leaf takes a segment, so a run has fewer than 2^17 leaves and fewer than 2^20 cells. A leaf holds at most C = 8 appended cells. With S = 16 the ceiling is about 2.1 million steps; a prover that closes after every step reaches the ceiling at 131,071 steps. This is a liveness limit, not an acceptance defect: the result is "no proof", never a different outcome. But it conflicts with GP-1 ("run length bounded only by the height field, below 2^64", `general-purpose.md` GP-1). Outcome-only publication does not repair it. It needs its own decision (a deeper tree costs 2 Poseidon permutations per level per leaf, ESTIMATE 140 rows; or a memory that reclaims). I mark it as a GP-1 gap that this review found, not as a blocker of the C6 decision.

What else is public or can become public:

| Item | Depends on raw addresses? | Action |
|---|---|---|
| `prog_d` | No. It covers `prog_root`, layout version and profile (`design.md` §2.7.7). | none |
| `init_d` (sealed in the contract, `sherlock-run.compact:26,41`) | Yes in form: it hashes the initial carrier. It is fixed per `(prog_d, input_d)` because genesis `len` is C and Init pads to a multiple of C (D3), so it is not partition-dependent. But it is a raw-carrier hash, and GP-1 wants `input_d`. | replace by `input_d` (SRC-021 already defines it) |
| `fin_d` | Yes. | remove from the public values, the `stmt` and the `acceptRun` arguments |
| `n_steps` | No (see above). | keep |
| `result_be` (u128) | No, the decider dereferences to the UInt (`run_decider.rs:279-283`). It covers one value type only. | replace by `outcome_d` |
| Node instance `[vk_repr, prog_d, init_d, cur_d, h, acc]` (`ivc.rs:68`; `run_decider.rs:201-206`) | `cur_d` yes. | internal only. A node proof is never a public claim; only a sealed claim is (C4) |
| Leaf statement `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]` (SRC-001) | `pre_d`, `post_d` yes. | internal only |
| GP-2 claim `(prog_d', input_d', outcome_d')` (`general-purpose.md` GP-2) | No, if `outcome_d` is section 2's encoding and `input_d` uses the same value digest. | see C3, C4 |
| GP-4 shape binding | No, if the shape is a structure over sealed claims. The leaf partition of one run must not be part of the bound shape. | state it (EARS below) |
| Diagnostics: `"Data(17)"`, wire and resource-report JSON (`experiments-4.md:304-309`, not re-read) | Yes. | not part of any statement; label as partition-dependent, or print the value digest |

Changes that the adoption needs:

- **C1** Remove `fin_d` from every public place, not only from the "outcome". Replace `init_d` by `input_d`.
- **C2** Every `Rejected(code)` and `Exhausted(kind)` is decided on logical counters and values. Physical address exhaustion has no outcome.
- **C3** `input_d` and `outcome_d` use one value digest, so that a GP-2 consumer can equate them.
- **C4** A consumable claim is a sealed claim: the consumer opens the terminal `cur_d` of the producer chain in circuit (as the decider does) and takes `(prog_d, input_d, outcome_d)`. A segment `(pre_d, post_d)` is never a claim across runs.

### 2. Canonical value encoding

`design.md` §3.2(5) (lines 166-167) already proposes a digest, `val_d = Poseidon(SHK_VAL, 1 + ty(x), root(DATA, enc(x)), n_d, root(FIELDS, enc(x)), n_f, ref_x)`: the roots of a dense, children-first re-serialization of the value. I recommend to **replace** it. Reasons:

- At halt the memory is not in that layout (garbage, D3 padding, sharing, another order). The run would have to copy the result into fresh dense tables and build two roots. The cost is linear in the unfolded tree, and a shared structure unfolds exponentially (n constructor steps `x = Cons(x, x)`-style give 2^n nodes).
- It has no `Constructor` case, covers lists of UInt only, and does not say how a `RefC::Const` is encoded.
- It exposes `n_d`, `n_f` and `ref_x`, which are layout facts.
- It keeps an open premise (well-formed `enc` behind `val_d`, [OPEN: input admission claim]).

Proposed encoding `sherlock-value/1`: a Merkle digest over the value as a tree. `H` is the Poseidon of zk-stdlib 2.4.0 over the BLS12-381 scalar field; `V = dom("SHK_VAL")`. The value types are those of `DataRow` (`helper_machine/raw.rs:740-760`; `complete_observation.rs:334-355`).

| Value | Digest `vd` |
|---|---|
| `UInt n`, `n < 2^128` | `H(V, 0, n)` |
| `Bool b`, `b` in {0, 1} | `H(V, 1, b)` |
| `Nil` | `H(V, 2)` |
| `Cons(h, t)` | `H(V, 3, vd(h), vd(t))` |
| `Leaf(x)` | `H(V, 4, vd(x))` |
| `Branch(l, r)` | `H(V, 5, vd(l), vd(r))` |
| `Constructor{type, ctor, f_1..f_k}` | `H(V, 6, ty_d, ctor_ord, k, vd(f_1), ..., vd(f_k))` |
| a reference (`RefC::Const(i)` or `RefC::Data(i)`) | the `vd` of the value it refers to; the index is never hashed |

- Every tag has a fixed arity, and tag 6 carries `k`, so the encoding is injective on trees.
- `ctor_ord = ctor_id - ctor_start` of the type row. `ty_d` is a program-independent digest of the type declaration. Today `TypeIdC` and `CtorIdC` are table indices; only the builtin rows 0 to 3 are fixed (`helper_machine/shared/adt.rs:4-13,32-41`). So a raw `type_id` must not be hashed, or two programs disagree on the same value. [OPEN: the definition of `ty_d`; it belongs to the PROG type table and to admission.]
- Sharing is not visible: the digest of a shared structure equals the digest of its unfolded tree. This matches the machine's structural equality. A first-visit numbering of a DAG would be cheaper to state, but two structurally equal values would get different digests, and GP-2 could not compare them.
- Outcome: `outcome_d = H(dom("SHK_OUT"), 1, vd(v))` for `Halted(v)`, `H(dom("SHK_OUT"), 2, code)` for `Rejected(code)`, `H(dom("SHK_OUT"), 3, kind)` for `Exhausted(kind)`. `Running` has no `outcome_d`.
- Input: `input_d = Poseidon(SHK_IN, val_d, fuel_lo, fuel_hi, stack_limit)` as in SRC-021, with `val_d = vd(x)`, and `VAL_NONE = H(V, 7)` for an absent input.

How the circuit gets `vd` from address-dependent memory. Three mechanisms:

| Mechanism | Halt cost | Run cost | Problem |
|---|---|---|---|
| (a) Digest at allocation, carried with the reference (a reference is the pair `(addr, vd)`); recommended | zero: the Halt rule copies `vd` into the outcome register | one hash chain per data allocation | wider references; Init needs a rule for the input |
| (b) Canonical traversal at halt (the brief's "digest of the reachable subgraph in a canonical order") | linear in the unfolded tree: per node one read (a read of a cell of an earlier leaf is a Merkle opening, ESTIMATE 20 or more permutations) plus the hash | zero | exponential on shared structure; adds helper steps after the last program step; a result too large to traverse cannot be published |
| (c) Dense logical ids beside M3 addresses | zero for the digest question; needs (a) or (b) anyway to publish a digest | a map from logical id to address | does not give a digest; it only restores SI-12 to SI-14 |

Invariant for (a): every reference in the state is a pair created by one allocation step, in which the core computes `vd` from the row's tag and the `vd` words of its field references, and both words are copied together afterwards. Constants carry their `vd` in the PROG constant row, under the PROG canonicality premise on `prog_root` (§2.7.4). The invariant becomes a clause of `StepInv` and needs a Quint model and negative tests.

Cost of (a), all ESTIMATE. Basis: 87 permutations take 6,090 rows, so one permutation is 70 rows, and 4 inputs take 2 permutations (`design.md` §2.7.11).

- Scalar, `Nil`, `Leaf`: 1 to 2 permutations (70 to 140 rows). `Cons`, `Branch`: 2 permutations (140 rows). `Constructor` with k fields: `ceil((5 + k) / 2)` permutations.
- The census has 0.20 to 0.22 appends per step, at most 3 (`design.md` §2.7.10); data allocations are a part of these. Upper bound 0.22 x 140 = 31 rows per step, against 407 rows per step today (6,511 / 16): at most +8% leaf rows on average. The leaf stays at k = 13 on the census average; a leaf with many allocations closes early (§2.7.7) or the key moves to k = 14, inside the gate `k_leaf <= 16`.
- The node grows with leaf columns and lookups, not rows (§2.7.11). If the `vd` word of each reference uses existing pair-row columns, the node (823,466 rows) does not change. If it needs new columns: 7,298 rows per advice column, 8,189 per equality column (§2.7.11 fit); the headroom is 126,534 rows. The memory record is `H(tag, a, b, c)` (`crates/mori-chip/src/mem.rs:113-114`); whether a field cell `(ref, vd)` and a data row with its `vd` fit in three payload words was not checked [OPEN].
- Each reference-holding register adds one digested word: half a permutation per state digest, two digests per leaf, so 70 rows per leaf per such register.
- Decider: it opens `cur_d` with the outcome tag and one payload word, as today, and adds one Poseidon for `outcome_d`; it loses the 16-byte decomposition of `result`. ESTIMATE under 1,000 rows of change on 929,166; k = 20 stays.
- Scalar-only first step: if only UInt and Bool results are needed now, the Halt rule can dereference the result once and hash it (2 permutations, 140 rows, once per run), with no fat references. The format above must still be fixed now, so that `vd(UInt n)` does not change later.

Init under (a). The D1 kill criterion forbids an Init check that scans the input (`design.md:164`). So the first leaf must not hash the input. Two ways, both [OPEN], to be chosen by experiment:

- lazy unfold: Init places the input as one reference `(UNMATERIALIZED, val_d)`; a dereference of such a reference is a step in which the prover supplies the row and the child digests, the core checks the hash and appends the row. The cost falls on the cells the program reads. The step count stays deterministic. This also removes the input admission premise for structure, because the hash preimage defines the value;
- load steps: counted machine steps that append the input in canonical post-order before `ENTRY`.

A cheaper sound alternative: I found none for structured values. (b) is cheaper only when results are small trees and almost no step allocates. For scalar results the scalar-only step above is the cheapest and is sound.

### 3. Effects on GP-1 to GP-4

- GP-1: completed, not narrowed. "Every outcome is a provable statement" now has a defined `outcome_d` for all seven value forms and the two error outcomes. One contract can serve every program if `prog_d` and `input_d` are data in the statement and not sealed fields (already a GP-1 gap: `general-purpose.md` GP-1 "Gaps today").
- GP-2: the claim `(prog_d', input_d', outcome_d')` is exactly the new public statement without `n_steps`. Consumption: B's `Init` takes `val_d`; the consuming proof opens A's `outcome_d'` as `Halted` with `vd` and requires `vd == val_d` of B. That needs C3 and C4. A `Rejected` or `Exhausted` claim can be consumed as a claim but not as an input value.
- GP-3: the adapter maps a foreign circuit's public inputs to the claim form. For a Compact circuit that returns a value to Sherlock, the adapter must output `vd` of that value, so the Compact side must compute `sherlock-value/1` (a Poseidon chain; whether Compact's `transientHash` equals zk-stdlib `poseidon` with these inputs was not checked). Nothing is narrowed.
- GP-4: the shape commitment must range over sealed claims. If it ranged over node digests or heights of one run's chain, the partition would enter the statement again. With that rule nothing is narrowed.
- Not solved by this decision, and it touches GP-1: the 2^17 leaf and 2^20 cell ceiling of §2.7.8 D3 (section 1).

### 4. Requirement, design, task and SI changes

EARS text (replace or add in `requirements.md` and the matching `specs/*/spec.md`):

**SRC-006 (replace).** The run decider SHALL verify the last node proof under the node key as constants, pin `self_vk_repr`, fold and pair the carried accumulator in circuit, open the final state digest as a terminal state (`Halted`, `Rejected` or `Exhausted`) whose outcome tag and payload are derived from the proved state, compute `outcome_d` from that tag and payload, and expose exactly `[prog_d, input_d, outcome_d, n_steps]`. The decider SHALL NOT expose `fin_d`, `init_d`, `mem_root`, `lanes_d`, `len`, a reference, or any value that depends on a memory address. If the final state is `Running`, the decider SHALL refuse.

**SRC-007 (replace).** `acceptRun` SHALL take `stmt = Poseidon(SHK_RUN, prog_d, input_d, outcome_d, n_steps)`, SHALL refuse a `stmt` already in `runs`, and SHALL write `outcome_d` and `n_steps`. The contract SHALL NOT take or store a state digest.

**SRC-022 (new, ubiquitous).** The published outcome of a run SHALL be `outcome_d = Poseidon(SHK_OUT, 1, vd(v))` for `Halted(v)`, `Poseidon(SHK_OUT, 2, code)` for `Rejected(code)` and `Poseidon(SHK_OUT, 3, kind)` for `Exhausted(kind)`, where `vd` is the value digest `sherlock-value/1`: `H(V,0,n)` for `UInt n`; `H(V,1,b)` for `Bool b`; `H(V,2)` for `Nil`; `H(V,3,vd(h),vd(t))` for `Cons`; `H(V,4,vd(x))` for `Leaf`; `H(V,5,vd(l),vd(r))` for `Branch`; `H(V,6,ty_d,ctor_ord,k,vd(f_1),...,vd(f_k))` for a constructor. The digest of a reference SHALL be the digest of the value it refers to. A table index, an address, a type-table index or a constructor-table index SHALL NOT be an input of `vd`.

**SRC-023 (new, unwanted behaviour).** If two accepted proofs have the same `prog_d` and `input_d`, then their `outcome_d` and `n_steps` SHALL be equal, for every choice of leaf boundaries. Acceptance: one program with at least three partitions (all leaves full; one step per leaf; a mixed partition) gives byte-equal public inputs apart from the binding and commitment slots; the second submission is refused as a replay.

**SVM-0xx (new, ubiquitous).** No machine rule SHALL compute a UInt, a Bool, a branch condition, an error code, an exhaust kind or a published value from a memory address, from `len`, `fp`, `tp` or a segment index. Addresses SHALL be used only to read and to append. `Exhausted(kind)` SHALL be decided on the logical counters of the run (fuel, allocation counts, helper work) and on limits carried in `input_d`.

**SVM-0xx (new, state-driven).** While a reference is held in a register, a value slot, a field cell or a frame, the reference SHALL carry the value digest `vd` of the value at its address; a step that allocates a data row SHALL compute that row's `vd` from its tag and the `vd` words of its field references. (Mechanism (a); adopt only after its experiment.)

**SRC-021 (amend).** Replace the `val_d` definition by: "`val_d` SHALL be `vd(x)` of `sherlock-value/1` for an input `x`, and `Poseidon(SHK_VAL, 7)` for an absent input." Remove "the DATA and FIELDS roots and lengths equal to the canonical encoding of the run input behind `val_d`"; the Init mechanism for the input is [OPEN: lazy unfold or load steps].

**GP-002 (add a sentence).** "A consumed claim SHALL be a sealed claim `(prog_d', input_d', outcome_d')` opened from a terminal state in circuit; a segment statement or a state digest SHALL NOT be accepted as a claim of another run."

**GP-004 (add a sentence).** "The bound shape SHALL be a structure over sealed claims; the leaf boundaries inside one run SHALL NOT be part of the statement."

SI invariants (`owner_runtime.rs`):

- Fresh id (`owner_runtime.rs:3021-3028`: "new data id" equals the table length): restate as "the reference returned by an allocation is the address the row was appended at"; in the chip that is the step's current `len`, which is already load-bearing in §2.7.8.
- SI-13 (`owner_runtime.rs:4744-4753`, child `id < owner`) and SI-14: keep. Addresses stay strictly increasing under D3, so the order checks hold unchanged.
- SI-12 (contiguous field slices): [OPEN]. A slice written by field microsteps can cross a leaf close, and the next leaf starts at `base + 8`, so `start + i` no longer finds field `i`. A leaf also appends at most 8 cells, so a slice longer than 8 cannot be contiguous under any partition. I do not write EARS for SI-12: either slices become linked cells, or the constructor row commits to its fields by digest (mechanism (a) gives that digest), or the leaf may not close inside a slice (not possible above 8 fields). This needs a design decision before D3 is adopted for the C6 machine.
- Codec bound `index < max_data_nodes` (`c6_phys.rs:453-457`, cited by `experiments-4.md:289-290`, not re-read): must bound addresses by the address space, and the logical count by its own counter.

Design text for `design.md` §2.7.11, replacing the `[OPEN, C6: address partition]` paragraph:

> C6 decision (2026-11-10, owner, after review): the public statement of a run is `[prog_d, input_d, outcome_d, n_steps]`. `outcome_d` is the digest of the outcome under `sherlock-value/1` (SRC-022). `pre_d`, `post_d`, `cur_d`, `mem_root`, `len`, `fp`, `tp` and stored links stay inside the recursion: they are witnesses of the node and the decider and never public inputs of the on-chain proof, never part of `stmt`, and never part of a claim that another run consumes. Reason: under D3 they depend on where the prover closes leaves, and `stmt` is the replay key of `acceptRun`. Addresses are opaque handles (SVM-0xx). `Exhausted` kinds are decided on logical counters only. Not solved here: SI-12 contiguity under D3 [OPEN]; the ceiling of 2^17 leaves and 2^20 cells per run against GP-1 [OPEN].

Design text for §3.3: replace "opens `cur_d = Poseidon(SHK_SD, HALTED, result, lanes_d)` and binds `values`" by "opens `cur_d = Poseidon(SHK_SD, tag, payload, rest_d)` for a terminal tag, computes `outcome_d`, and binds `[prog_d, input_d, outcome_d, n_steps]`"; replace "writes `result` as big-endian `Bytes<16>`" by "writes `outcome_d`".

Tasks: amend 2.5 and 5.1c (the template is recaptured for the new `acceptRun` signature); amend 4.2a (statement contract: the four public values and `sherlock-value/1`); amend 5.2 (negatives: altered `outcome_d`; same run under two partitions gives one `stmt`; replay of the second refused); new task "value digest experiment" (mechanism (a) in `mori-chip`: rows, columns, node rows, MockProver negatives for a forged `vd`, Quint model of the pair invariant); new task "SI-12 under D3"; new task "address ceiling against GP-1"; amend 4.15 (host `Init` differential computes `vd`).

## Q2 Does this run on Midnight ledger 9?

Verdict: **yes with changes**. Nothing in ledger 9.1.0.1 blocks the design. The on-chain step has not yet been accepted by a 9.1.0.1 node, the chip leaf has not been proved inside the node, and the contract and template must change for Q1.

| Check | Finding | Source |
|---|---|---|
| Verifier for a contract call | `ProofVersioned::V3` goes to `op.v3_vk()` and `vk.verify(&PARAMS_VERIFIER, proof, pis)` | ledger `ledger/src/structure.rs:446-506` |
| Proof system | `midnight_zk_stdlib::verify::<DummyRelation, TranscriptHash>`: PLONK with KZG on BLS12-381 | ledger `transient-crypto/src/proofs.rs:563-577`; `midnight-zk-stdlib-2.4.0/src/lib.rs:2009-2032` |
| Transcript on chain | Blake2b: `TranscriptHash = blake2b_simd::State` | ledger `transient-crypto/src/proofs.rs:65` |
| Transcript of the decider proof | Blake2b (`type B2 = blake2b_simd::State`; `prove::<RunDeciderRelation, B2>`) | `crates/mori-recursion/tests/sherlock_gp.rs:37,323` |
| Transcript of leaf and node proofs | Poseidon, verified in circuit only | `crates/mori-recursion/src/inner.rs:102-104,233-235`; `ivc.rs:322-326` |
| SRS on chain | static `bls_midnight_2p14` verifier parameters | ledger `transient-crypto/src/proofs.rs:108-128` |
| Maximum k on chain | No check exists. `ParamsVerifierKZG` holds only G2 elements (`s_g2`, two prepared G2), so it does not bound k. `VERIFIER_MAX_DEGREE` appears only at its definition and in a doc comment. | `midnight-proofs-0.8.3/src/poly/kzg/params.rs:235-239`; `git grep VERIFIER_MAX_DEGREE ledger-9.1.0.1` (2 hits, both in `proofs.rs`) |
| k = 20 decider against the ledger verifier | The decider proof is verified with the ledger crate's `VerifierKey::verify(&PARAMS_VERIFIER, ..)`, and each mutated public input is refused. README: "prove and verify end to end on ledger 9.1.0.1". This is the ledger's verifier library, not a node. | `sherlock_gp.rs:329-345`; `README.md:409`; `qa/sherlock/manifests/run-decider.json` (k 20, 929,166 rows, SRS `bls_midnight_2p20`, sha `1cc62978...`) |
| Upstream belief to watch | A comment says no verifiable circuit exceeds 2^14 rows. It is not enforced. A later release could enforce it. | ledger `onchain-runtime/src/transcript.rs:80-87` |
| What the on-chain key must be | Not ZKIR. `MidnightVK::read` rebuilds the constraint system with `ZkStdLib::configure(arch, k - 1)`, so the installed key must be a key of a zk-stdlib `Relation`. The decider is one (`impl Relation for RunDeciderRelation`, `run_decider.rs:181`). `ContractOperation` stores keys; the ledger needs no IR to verify. | `midnight-zk-stdlib-2.4.0/src/lib.rs:1573-1594`; ledger `ledger/src/structure.rs:3106-3130`, `semantics.rs:1594-1606` |
| Chip leaf outside ZKIR and outside zk-stdlib | It is verified only inside the node, which takes the leaf key as constants (`assign_fixed_vk(domain, cs, transcript_repr)`), so it needs no `MidnightVK` form. The decider consumes only the node key. | `run_decider.rs:209-218`; `design.md` §2.7.10 last paragraph |
| Leaf, node and decider on one curve and SRS | KZG on BLS12-381 with the official ceremony SRS, downsized (`crates/mori-chip/tests/e3_spike.rs:84-96`, sha `1cc62978...`, same as the decider manifest). The leaf's rotations are -1, 0, +1, the limit of the in-circuit verifier. | `design.md` §2.7.11; `qa/sherlock/manifests/run-decider.json` |
| Raw verifier key limit 50,000 bytes | decider key 2,189 bytes raw (k 20, stagenet record), 2,217 tagged | ledger `transient-crypto/src/proofs.rs:422-431`; `docs/sherlock/stagenet-deployment.md:102-103`; `qa/sherlock/manifests/sherlock-run-contract.json` (`acceptRun_verifier_bytes` 2217) |
| Public inputs <= 65,536 | 70 slots today. After Q1 the signature loses `fin` and replaces `r` by a Field: ESTIMATE 60 to 70 slots. | ledger `onchain-runtime/src/transcript.rs:87`; `design.md` §3.3 |
| Operations per contract <= 256 | 1 (`acceptRun`) | ledger `ledger/src/structure.rs:1928`; `sherlock-run.compact:39` |
| Proof size | 3,440 bytes (GP decider trace) | `qa/sherlock/traces` (`proof_bytes=3440`, two traces) |
| Transaction size | source default 1 MiB; the proof plus transcript is a few kB | ledger `ledger/src/structure.rs:1283-1296` (`INITIAL_LIMITS`) |
| Cost model | verification is charged `proof_verify_constant + proof_verify_coeff_size * size` with size the public-input count; it does not depend on k | ledger `ledger/src/structure.rs:1138-1142`; `construct.rs:868` |
| Key installation | deploy installs the hand-written decider key for `acceptRun` and checks its hash; a maintenance `VerifierKeyInsert` refuses a second key of the same version at one entry point | `tools/sherlock-sdk/deploy-run.mjs:4,42,56`; ledger `ledger/src/semantics.rs:1594-1606` |
| Q1 statement in the contract | `outcome_d: Field` and `n: Uint<64>` are ordinary ledger fields; `runs: Set<Field>` stays. One Field replaces `Bytes<16>`. | `sherlock-run.compact:23-27` |

Blockers: none in the ledger. Work that "yes" depends on:

1. The Q1 statement changes `acceptRun`, so the template, the decider key and the contract are new: keygen in the order leaf, node, decider, template, deploy (`design.md` §5.2 item 2). The acceptRun template fixture still predates the re-pin (`sherlock-run-contract.json` compiler note; task 2.5 open).
2. No `acceptRun` transaction has been validated by a 9.1.0.1 node. Tasks 2.5, 5.1c and 6.x are open (`tasks.md:43,90,114-120`). The stagenet deployment of a k = 20 key was on a pre-release ledger 9 and had zero calls (`docs/sherlock/stagenet-deployment.md:3,17`).
3. The chip leaf is a spike: "Modeling only ... No proof" (`crates/mori-chip/tests/e3_spike.rs:11-12`). The node over the chip leaf key is measured in rows (823,466), not proved. A node proof over a chip leaf proof is the missing link between the design and the on-chain step.
4. GP-1 with one contract for all programs needs `prog_d` and `input_d` as call data, not sealed fields; that is a contract change, not a ledger limit.

## Not checked

- No build, test, keygen or proof was run. All row counts are quoted from the repository.
- The deployed `LedgerParameters` on Preview and Mainnet (transaction byte limit, block limits, cost-model constants, fees). `INITIAL_LIMITS` are source defaults.
- midnight-node 2.1.101 limits outside the ledger crate (block length, extrinsic size, weights). The deployed node build `913ec011` is not public (`qa/sherlock/target-pins.json`).
- Whether Compact 0.36.0 compiles the changed `acceptRun`, and its slot count.
- Whether Compact `transientHash` equals zk-stdlib `poseidon` on the inputs of `sherlock-value/1`.
- The time and memory to load a k = 20 key in the node, and `verifier_key_load` cost.
- Whether a record `(tag, a, b, c)` has room for the `vd` word; the column cost of mechanism (a).
- The citations of `experiments-4.md` §4 other than `shared.rs:701-705`, `structural.rs`, `owner_runtime.rs:3021-3028,4744-4753`, `run_decider.rs` and `sherlock_c6_witness_stream.rs`.
- GP-2 to GP-4 architecture documents (`research/sherlock-gp-architecture-20261110/`); I checked the requirement text only.
- Web sources: none used.

## Correction (added after reading the other answers; the text above is unchanged)

- Proof size. The row "Proof size: 3,440 bytes" in the Q2 table is wrong. Those two trace lines are the A2 child proofs (`qa/sherlock/traces/20261108T173416Z-A2.log:912-913`, k = 15 and k = 12), not a decider proof. The k = 20 terminal decider proof recorded on stagenet is 6,432 bytes (`docs/sherlock/stagenet-deployment.md:70,142`). No decider proof over the chip-leaf node key exists.
- The ledger tag's `Cargo.lock` resolves `midnight-proofs` 0.8.2 (`Cargo.lock:3509-3510` at `ledger-9.1.0.1`); the workspace uses 0.8.3. I read the KZG verifier parameters in 0.8.3 only.
