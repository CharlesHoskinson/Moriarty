# General-purpose Sherlock recursion: architecture proposal

## Recommended architecture

Build **one binary self-recursive processor that executes bounded work directly inside the node**, followed by one Midnight-compatible terminal decider. Programs, memory, foreign verifier descriptions, registry entries and graph structure are authenticated data. Do not add a third proof slot for new work.

This recommendation has a concrete starting point: the recorded `{self,self}` probe uses **730,243 rows**, while `{self,self,leaf}` uses **1,078,147 rows**. The former leaves room to investigate an inline processor at k=20. These are modeling results, not a working binary recursive implementation. [`qa/sherlock/decisions/speed-levers.json:26`; `crates/mori-recursion/tests/tree_node_rows.rs:1`]

| Layer | Proposed relation and statement | Fixed keys |
|---|---|---|
| Semantic processor | A bounded sequence of interpreter microsteps transforms an authenticated configuration. Configurations commit to program, input, memory, continuation, execution counters and typed claims. Long operations continue in subsequent nodes. | Part of the node relation. No program-specific key. |
| Binary PCD node | Verify up to two predecessor certificates, execute new processor work, bind the ordered dependencies and new work into `shape_d`, and publish a fixed-width statement digest plus a collapsed KZG accumulator. | One node key, `N`, for leaves, chains, trees and DAGs. |
| Terminal decider | Verify the root under `N`, discharge its accumulator, require a completed claim or batch, and bind its claims, shape, registry policy and settlement transition to the contract transcript. | One decider key, `D`. |
| Settlement contract | Accept dynamic program/input/outcome claims or a claim-set root under `D`. Check the authorized registry root and settlement replay state. | One installed verifier key. |

Proposed satisfaction of the binding requirements:

- **GP-1:** Run a complete Sherlock interpreter over committed memory. `prog_d` and `input_d` are inputs. Prove every terminal outcome through canonical value encodings. Processor chunk size limits work per proof, not program size, memory capacity or owner-step count. [`openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:9`]
- **GP-2:** Give verified runs stable claim identities. A consumer authenticates its dependencies in-circuit. Batch construction proves a canonical set of claims from which membership openings are available. [`openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:22`]
- **GP-3:** Execute foreign verification and statement adapters inside the processor, against authenticated registry entries. Large keys or verifier computations span nodes. This must include the actual Blake2b-based Compact proof format. [`openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:33`; ledger `a2b30128`, `transient-crypto/src/proofs.rs:64`]
- **GP-4:** Two recursive predecessor slots plus inline work provide binary PCD. Commit graph structure in the proved statement. Reuse claim identities at DAG fan-out, while accounting for execution and settlement separately. Keep all public statements and the terminal proof format fixed-width. [`openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:44`]

**Feasibility boundary.** No inspected implementation demonstrates all four requirements within the present resource envelope. The proposed binary node is **ESTIMATE 0.82–1.00 million rows, k=20**, including a small processor. Its decider is **ESTIMATE 0.93–1.08 million rows, k=20–21**. Both must be measured before adoption. The existing decider already uses 929,166 modeled rows. [`qa/sherlock/manifests/run-decider.json:20`]

If this node cannot fit with useful work, or cannot meet the existing throughput requirement, replace the inner processor with a recursive STARK system and retain a Midnight KZG settlement wrapper. Do not reduce GP-1 through GP-4 to preserve the current implementation.

### Evidence conventions

Repository citations refer to the inspected source at `9001a80af6a1c3542ee83a888cdeb589e38e833b`. The subsequent inspected commit, `3d0b622242d8ec00b9cc1aedbd612ab061f3ed58`, adds only the study brief.

The following prefixes identify exact source roots:

- `D`: `openspec/changes/sherlock-vm-consensus-20261107/`
- `P`: `tools/profile/vendor/midnight-proofs-0.8.3/src/`
- `C`: `/home/hoskinson/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.3.0/src/`
- `Z`: `/home/hoskinson/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.4.0/src/`
- `L`: `repos/midnightntwrk/midnight-ledger`, read at **tag `ledger-9.1.0.1`, commit `a2b30128ca4b660b73a068e29b56b48ede8c9a74`**.

Statements below described as proposals specify new behavior. They are not claims that the repository implements it. All unmeasured numerical projections are marked **ESTIMATE**.

## 1. Accumulation scheme

### The present node already defers pairings

The current architecture does not perform a complete KZG pairing check for every child. `SHKIVC06` prepares the predecessor proof and leaf proof, combines their accumulators with the carried accumulator, collapses the result and publishes it. The terminal decider performs the pairing. [`crates/mori-recursion/src/ivc.rs:531`; `crates/mori-recursion/src/ivc.rs:548`; `crates/mori-recursion/src/run_decider.rs:243`]

The measured bottleneck is the non-native MSM work needed to collapse those accumulators. The recorded `collapse_self` and `collapse_final` costs are 239,577 and 267,514 rows, approximately 78% of the 652,437-row node. Merely moving pairings to the end repeats an optimization already present. [`qa/sherlock/decisions/speed-levers.json:66`]

The relevant baselines are:

| Existing construction | Recorded rows / k | Evidence and qualification |
|---|---:|---|
| `SHKIVC06`, stand-in leaf | 652,437 / k=20 | Working recursion baseline. [`qa/sherlock/decisions/speed-levers.json:22`] |
| `SHKIVC06`, C6 digest-leaf key | 741,708 / k=20 | Key-manifest modeling result. [`qa/sherlock/manifests/node.json:22`] |
| Two self slots | 730,243 / k=20 | Probe only. [`qa/sherlock/decisions/speed-levers.json:28`] |
| Two self slots and one leaf slot | 1,078,147 / k=21 | Probe only. [`qa/sherlock/decisions/speed-levers.json:27`] |
| Two self slots and two leaf slots | 1,278,670 / k=21 | Probe only. [`qa/sherlock/decisions/speed-levers.json:29`] |
| Current run decider | 929,166 / k=20 | Over the recorded C6 digest-leaf node key. [`qa/sherlock/manifests/run-decider.json:20`] |

The working stand-in chain records approximately 60–67 seconds per node and 16.6–17.6 GB process peak, with a 70.35-second decider in the A16 run. These timings do not measure a universal interpreter or foreign verification. [`qa/sherlock/decisions/speed-levers.json:83`; `qa/sherlock/decisions/speed-levers.json:95`]

### Options

For foreign systems, an “equivalent k” below means `ceil(log2(constraint count))`. It does **not** establish a Midnight Halo2 row count. Application work, memory checks and foreign verification must be added to the recursion overhead.

| Option | Node size | Proof size and prover cost | Decision |
|---|---|---|---|
| Full in-circuit KZG verification, including pairings for both children | **ESTIMATE:** 1.8–3.5 million Midnight rows, k=21–22, before substantial application work. Based on the existing terminal verifier’s combined preparation, MSM and pairing workload. | **ESTIMATE:** fixed 6–15 KiB final PLONK proof; 2–8 minutes per node and 30–100 GB, highly dependent on columns and degree. | Reject for the main path. It repeats terminal pairing work at each node. Existing pairing implementation: `crates/mori-recursion/src/run_decider.rs:247`. |
| Existing KZG accumulation, two self proofs plus an independently proved leaf | 1,078,147 modeled rows, k=21. | **ESTIMATE:** 2–3 times the current node time and roughly twice its domain-buffer memory if architecture stays similar. Final proof remains fixed-size. | Reject this node shape at the existing k=20 target. [`qa/sherlock/decisions/speed-levers.json:27`] |
| **Existing KZG accumulation, two self proofs plus inline work** | **ESTIMATE:** 0.82–1.00 million rows, k=20, for a deliberately small work kernel. | **ESTIMATE:** 60–100 seconds, 17–30 GB per node; fixed 6–15 KiB terminal proof. These are same-domain extrapolations, not measured results. | **First implementation candidate.** Its measured starting point is 730,243 rows. [`qa/sherlock/decisions/speed-levers.json:28`] |
| Genuine split accumulation | **ESTIMATE:** 50–150 thousand constraints over a suitable native cycle, equivalent k=16–18; 0.2–0.8 million Midnight rows for a non-native BLS implementation, k=18–20. | **ESTIMATE:** 1–30 seconds per native-cycle step, excluding application work. Final Midnight compression remains necessary. | Keep as a protocol-development option. The published 52,000-constraint result is for a particular Pasta-based construction, not this BLS stack. [PCD without Succinct Arguments](https://eprint.iacr.org/2020/1618.pdf) |
| Nova / HyperNova over a curve cycle | **ESTIMATE:** 30–250 thousand recursion constraints, equivalent k=15–18, before the processor. | **ESTIMATE:** 1–30 seconds per fold at this scale. A terminal compression proof and a Midnight verifier for it are additional work. | Prefer HyperNova’s multi-instance formulation over assuming a sequential Nova API supplies PCD. Neither is a drop-in verifier for existing Compact proof bytes. [HyperNova](https://eprint.iacr.org/2023/573.pdf), [Nova implementation](https://github.com/microsoft/Nova) |
| ProtoStar / ProtoGalaxy-style folding | **ESTIMATE:** 50–300 thousand native-cycle recursion constraints, equivalent k=16–19; non-native realization could consume 0.2–1.0 million Midnight rows, k=18–20. | **ESTIMATE:** 1–40 seconds per fold, excluding application work and final compression. | Keep as an alternative, not the initial commitment. Multi-folding does not supply the registry, adapter or graph semantics. [ProtoStar](https://eprint.iacr.org/2023/620.pdf), [ProtoGalaxy](https://eprint.iacr.org/2023/1106.pdf) |
| Recursive STARK processor, then Midnight KZG wrapper | **ESTIMATE:** fixed inner recursion traces at k=17–21; direct Midnight wrapper k=20–23. These are broad sizing hypotheses. | **ESTIMATE:** 50–500 KiB internal proof, seconds to minutes per bounded inner proof; fixed 6–15 KiB outer proof. Direct wrapper memory might exceed the present cap. | **Fallback architecture if the binary KZG processor fails fit or throughput.** Existing STARK-to-Halo2 systems establish the layering pattern, not a ready Midnight wrapper. [OpenVM proof generation](https://docs.openvm.dev/book/writing-apps/generating-proofs/) |

The current libraries provide PLONK verification, KZG opening reduction, accumulator combination, foreign-curve MSMs and a Poseidon recursive transcript. The accumulator API explicitly identifies collapse as the likely recursion bottleneck. [`C:verifier/verifier_gadget.rs:187`; `C:verifier/accumulator.rs:333`; `C:verifier/types.rs:140`]

The inspected source does not supply a finished split-accumulation replacement. Its prover trace contains a field described as useful for future split accumulation. That comment is not an implemented PCD protocol. [`P:plonk/traces.rs:14`]

The maintained Nova implementation lists Pallas/Vesta, BN254/Grumpkin and secp/secq cycles. Selecting one changes the inner arithmetic and introduces a bridge to Midnight. It does not require changing Midnight’s on-chain verifier if the bridge emits the existing outer format. [Nova implementation](https://github.com/microsoft/Nova)

### Proposed binary node

Use two predecessor slots and no independent leaf-proof slot:

```text
For each enabled child i:
    verify its self-key identity
    prepare its proof -> proof_acc_i
    bind its carried accumulator to its proved public statement

For each disabled child:
    replace every accumulator encoding absorbed by the fold
    with a canonical neutral encoding

Execute B bounded processor operations.

A_out = collapse(accumulate(
    proof_acc_left, carry_left,
    proof_acc_right, carry_right
))
```

New work is enforced by this node’s constraints. Its validity enters the recursive chain when the parent, or terminal decider, verifies the node proof.

The two-self probe performs one combined collapse. It does not implement the hardened canonical disabled-slot construction proposed here, and its self architecture and statement must be rechecked against the completed processor. Its row count is therefore a starting point, not a bound. [`crates/mori-recursion/tests/tree_node_rows.rs:8`; `crates/mori-recursion/tests/tree_node_rows.rs:178`]

**Do not remove `collapse_self` from `SHKIVC06` as a local optimization.** That collapse currently makes the genesis encoding agree with the host computation. The replacement needs explicit normalization before the accumulator hash, including bases whose scalars have been zeroed. [`crates/mori-recursion/src/ivc.rs:538`; `C:verifier/accumulator.rs:387`]

For arbitrary foreign keys, resolve their key-dependent bases before carrying an obligation onward. A carried dictionary that grows with the number of admitted keys would violate fixed-size recursion. Keep only the fixed self-key scalar set and a bounded number of collapsed points.

A different SRS requires separate treatment. The KZG verifier parameters contain the SRS-specific G2 element. Do not combine obligations under different SRS values into one pairing equation. Fully verify such foreign proofs in the processor, or implement a separately justified multi-SRS construction. [`P:poly/kzg/params.rs:235`]

### What cannot presently be claimed

No evidence establishes a useful **concrete security bound for arbitrary-depth PCD instantiated by this exact construction**. The Lean accumulator model assumes validity preservation by accumulation; it does not prove that cryptographic property. [`clean/fork/Clean/Sherlock/Aggregation.lean:1230`]

This is an assurance gap, not proof that BLS12-381 PCD is impossible. Closing it requires a composition argument for the chosen transcript, accumulation scheme, self-key binding and adversarial graph construction. Generic recursive-composition analyses can incur substantial depth-dependent loss; stronger results require specified extractor properties and models. [Security Bounds for Proof-Carrying Data](https://eprint.iacr.org/2023/1646.pdf)

## 2. Universal VM: program, input, outcomes and memory

### Proposed execution statement

Separate a run’s semantic identity from its proof packaging:

```text
run_id = H(
    "SherlockRun/v2",
    machine_version,
    prog_d,
    input_d,
    execution_parameters_d,
    request_context_d
)

RunSegment = (
    run_id,
    prog_d,
    input_d,
    pre_state_d,
    post_state_d,
    owner_h_before,
    owner_h_after,
    continuation_d
)

RunClaim = (
    run_id,
    prog_d,
    input_d,
    initial_state_d,
    final_state_d,
    n_owner_steps,
    outcome_tag,
    outcome_d
)
```

`execution_parameters_d` includes semantic resource parameters such as initial fuel. It must not include a prover’s resident-memory capacity as a language admission condition. Initialization already depends on fuel and stack configuration in the reference evaluator. [`crates/mori-core/src/sherlock/reference.rs:14`]

The processor must prove execution of the complete Sherlock language. A rule inside that language cannot remain “unsupported” merely because its monolithic fragment is too large. The existing design’s unsupported-rule refusal and profile restriction cannot serve as the completed GP-1 implementation. [`D:design.md:16`; `D:design.md:41`; `D:general-purpose.md:9`]

Use a small processor instruction set and authenticated interpreter code to express expensive Sherlock operations as continuations. The required refinement is:

```text
completed processor execution of the Sherlock interpreter
    implies
the corresponding Sherlock execution and outcome
```

Authenticating the interpreter code is essential. Arbitrary application bytecode must not be able to emit a privileged `RunClaim` or `ForeignVerified` claim by jumping directly to a publication instruction.

### Canonical initialization

Prove initialization in-circuit, possibly over several processor chunks:

1. Authenticate the program and canonical input representation.
2. Establish the empty mutable store, initial control state and frame state.
3. Bind fuel and other semantic parameters.
4. Establish the state invariant.
5. Publish the unique initial state commitment for `run_id`.

A run cannot begin from an arbitrary nonzero `init_d`. The present node checks equality with a supplied initial digest, and the contract constructor checks only that the digest is nonzero. The formal soundness theorem separately assumes a carrier state opening that digest. [`crates/mori-recursion/src/ivc.rs:498`; `tools/sherlock-sdk/sherlock-run.compact:29`; `clean/fork/Clean/Sherlock/Aggregation.lean:204`]

### Committed memory

Replace whole-image carriers with a fixed register window and authenticated stores for:

- program data and constants;
- canonical input;
- environments and frames;
- dynamic values;
- processor continuations;
- foreign verifier state;
- claim collections.

The current bridge hashes entire program/state carrier sections. Its size therefore depends on those carriers. [`crates/mori-backend/src/sherlock_c6_child.rs:551`]

Start with an authenticated, expandable tree. Tree depth and path processing are data. If a path exceeds one chunk’s work budget, authenticate it across chunks. Bind the pending operation, address, old root, candidate new root and path cursor into the continuation. Publish the changed memory root only after the operation finishes.

A fixed page size is acceptable. A fixed number of pages admitted by the key is not. Similarly, silently imposing a maximum Merkle depth through the circuit would recreate the forbidden capacity ladder. [`D:general-purpose.md:13`]

Let `d` be the actual authentication depth and `C_H` the measured cost of a domain-separated tree compression:

| Memory option | Proposed cost per access | Decision |
|---|---|---|
| Word-level Merkle memory | Read: approximately `d × C_H`. Write: approximately `2d × C_H`, plus encoding and address checks. | First correctness implementation. Naturally supports authenticated continuation boundaries. |
| Paged Merkle memory | Page authentication costs `d × C_H`; amortize across operations on that page. Internal page access must also be constrained. | Add after measuring locality. Do not assume a free RAM array inside a page. |
| Sorted access table with permutation/lookup checking | **ESTIMATE:** 20–100 native arithmetic rows per access, plus range tables, sorting/permutation work and authenticated shard boundaries. | Throughput candidate. Proving only local table consistency is insufficient. |
| Offline memory checking with a separate global argument | Small access records plus a final consistency proof. **ESTIMATE:** 10–100 processor rows per record, excluding the global argument and compression. | Candidate for a folding or STARK inner layer. More complex completion and boundary obligations. |

For initial sizing, use **ESTIMATE `C_H = 50–150` Midnight rows**, including the chosen domain encoding and selection work. A depth-32 read then costs **ESTIMATE 1,600–4,800 rows**, and a write **ESTIMATE 3,200–9,600 rows**. These are experiment inputs, not measurements of the installed Poseidon implementation. The implementation packs partial rounds, so generic Poseidon round counts cannot substitute for a row probe. [`C:hash/poseidon/poseidon_chip.rs:45`]

Offline memory arguments must establish read-after-write consistency, initialization and finalization across the entire execution. Jolt’s memory treatment provides a concrete timestamp-based reference. Twist and Shout provide a different sumcheck-based route, with separate read/write and read-only arguments. [Jolt](https://eprint.iacr.org/2023/1217.pdf), [Twist and Shout](https://eprint.iacr.org/2025/105.pdf)

For offline checking, derive challenges after committing the relevant records. Do not expose a challenge and then allow unconstrained future records to cancel an earlier inconsistency.

### Inputs and outcomes

Use a canonical typed input encoding. Authenticate input reads against `input_d`; proving only a host-computed input hash is insufficient.

Define:

```text
Halted:
    outcome_d = H("Halted", type_d, canonical_value_encoding_d)

Rejected:
    outcome_d = H("Rejected", canonical_error_code)

Exhausted:
    outcome_d = H("Exhausted", canonical_exhaustion_kind)
```

Large structured values are validated and hashed incrementally. Their size changes proving work, not public-statement width.

The current witness stream accepts only halted UInt values, and the decider opens only a halted u128 result. Both need replacement. [`crates/mori-backend/src/sherlock_c6_witness_stream.rs:138`; `crates/mori-recursion/src/run_decider.rs:278`]

Maintain separate concepts:

- **Owner-step height:** range-checked below `2^64`, as GP-1 requires.
- **Processor progress:** includes memory paths, parsing, hashing and verifier operations.

An unfinished memory check must not consume an extra Sherlock owner step or publish an outcome. Processor continuation machinery must not introduce a smaller run-length limit.

## 3. Foreign-key registry and statement adapters

### Proposed registry entry

Authenticate the complete verification interpretation:

```text
RegistryEntry = (
    vk_id,
    raw_vk_digest,
    normalized_vk_digest,
    proof_system_id,
    proof_format_version,
    curve_id,
    commitment_scheme_id,
    srs_id,
    transcript_id,
    constraint_system_descriptor_d,
    public_input_schema_d,
    verifier_program_d,
    adapter_program_d,
    claim_kind,
    policy_epoch
)
```

The contract authorizes a registry root or an authenticated registry-history root. The node proves entry membership against that root. The prover cannot select an arbitrary root and thereby authorize its own key.

Admission should validate:

1. Canonical key parsing and complete byte consumption.
2. Agreement between raw bytes and the normalized verifier description.
3. The curve, commitment scheme, transcript and SRS.
4. The adapter’s input schema and allowed output claim kind.
5. The admission authority’s policy.

Use the same processor to prove normalization when it is too expensive to repeat. The admission certificate must bind both representations. A host-produced normalized key is not sufficient.

Register the Sherlock processor/decider relations once. Individual Sherlock programs remain data and require no registry key entry.

### Why the existing “public key” API is insufficient

`assign_vk_as_public_input` assigns the transcript representative as a public input, but receives the evaluation domain and constraint system as host-side parameters. Those remain fixed by circuit construction. It is not an arbitrary-key verifier. [`C:verifier/verifier_gadget.rs:204`]

The gadget also documents assumptions about challenge phases and instance columns. The native verifier has broader phase-processing logic. A registry cannot erase that difference. [`C:verifier/verifier_gadget.rs:14`; `P:plonk/verifier.rs:78`]

For arbitrary admitted architectures, execute the verifier description as authenticated data in the processor. Its loops, expression evaluations, transcript parsing and MSM construction can span chunks. This keeps key size and architecture complexity out of the node key.

A uniform-architecture fast path is useful, but it cannot be the only GP-3 path. The existing finite-family option explicitly fixes architecture and k. [`D:design.md:95`]

### Compact proof compatibility

A Compact contract proof **can be verified by a newly implemented in-circuit verifier**, but it cannot simply be passed to the current Poseidon recursive gadget.

The exact compatibility requirements are:

| Boundary | Source fact | Required implementation |
|---|---|---|
| Proof envelope | Ledger proof tag is `proof[v5]`; the payload is bytes. [`L:transient-crypto/src/proofs.rs:131`] | Parse the correct payload and version. |
| Key envelope | Ledger key tag is `verifier-key[v7]`. [`L:transient-crypto/src/proofs.rs:370`] | Bind the complete registered key identity. |
| Key structure | `MidnightVK` contains architecture, k, public-input count and the underlying PLONK key. Deserialization reconstructs the constraint system from the architecture. [`Z:lib.rs:1537`; `Z:lib.rs:1573`] | Authenticate architecture and reconstructed verifier semantics. |
| Transcript | Ledger verification uses `blake2b_simd::State`. Recursive BLS verification uses `PoseidonState`. [`L:transient-crypto/src/proofs.rs:64`; `C:verifier/types.rs:140`] | Implement the actual Blake2b transcript. Reproving the circuit with Poseidon does not verify the supplied Compact proof. |
| Blake2b details | The transcript uses a 64-byte keyed hash initialization, common-message prefixes and challenge prefixes. [`P:transcript/implementors.rs:13`] | Match byte encodings, prefixes, challenge extraction and field reduction exactly. |
| VK transcript representative | The representative hashes commitments, domain and pinned constraint-system information. [`P:plonk/mod.rs:246`] | Bind this representative to the registered descriptor, rather than accepting an unrelated scalar. |
| Public inputs | Ledger verification passes one ordinary public-input vector and no explicit committed instance; stdlib substitutes the identity commitment. [`L:transient-crypto/src/proofs.rs:563`; `Z:lib.rs:2020`] | Reproduce instance counts, lengths and committed-instance treatment. |
| Complete parsing | Native verification rejects unused proof bytes; ledger key parsing rejects trailing key bytes. [`Z:utils/plonk_api.rs:144`; `L:transient-crypto/src/proofs.rs:532`] | Match these checks when claiming byte-level ledger compatibility. |

There is a Blake2b chip wrapper in zk-stdlib. It is useful implementation material, but its existence does not make the recursive transcript compatible. [`Z:external/blake2b.rs:37`]

The recursive transcript gadget also leaves subgroup checks out of point assignment and explicitly does not prove that every proof byte was consumed. Those choices must not be copied into a byte-exact foreign-proof adapter without a separate argument. [`C:verifier/transcript_gadget.rs:76`; `C:verifier/transcript_gadget.rs:123`]

### Statement mapping

ZKIR v3’s public statement is not simply its function arguments and return values. The circuit first publishes a binding input, optionally a communications commitment, and then guarded `Impact` values. Typed inputs and outputs enter the communications commitment when enabled. [`L:zkir-v3/src/ir_vm.rs:801`; `L:zkir-v3/src/ir_vm.rs:869`; `L:zkir-v3/src/ir_vm.rs:1173`]

Provide two adapter levels:

- **Generic circuit claim:** `CircuitSatisfied(vk_id, public_inputs_d)`.
- **Typed application claim:** selected public fields, or values whose communications-commitment opening is checked in-circuit, mapped to a declared claim schema.

Do not manufacture a public return value from an opaque communications commitment. If the adapter requires that value, it needs a valid opening or an explicitly exposed public value.

A verified contract circuit proof establishes circuit satisfaction. It does not alone establish that a transaction was included on-chain or that its ledger effects were accepted. The ledger separately constructs transaction-bound inputs and validates calls. [`L:ledger/src/verify.rs:2119`; `L:ledger/src/verify.rs:2151`]

“Same curve and commitment scheme” must not silently become “same transcript, same SRS and same gate layout.” Registry metadata selects the correct verifier program. Unrecognized variants require an admitted verifier implementation, not a new recursion key.

## 4. PCD shape, DAG fan-out and formal properties

### Proposed typed composition

Use distinct claim kinds:

```text
Continuation
RunSegment
RunClaim
ForeignCircuitClaim
BatchClaim
SettlementClaim
```

A segment join requires the same run identity, matching state boundaries and adjacent owner heights.

A batch join permits different programs and inputs.

A consumer edge binds a producer claim to a specified input position of another run. It checks the producer certificate and the adapter, rather than merely comparing two host-supplied digests.

Incomplete continuations cannot be consumed as completed results or settled on-chain.

### Bind shape inside the proved statement

Propose a fixed-width node statement:

```text
NodeStatement = (
    self_vk_repr,
    policy_d,
    message_d,
    claim_set_d,
    shape_d,
    accumulator_encoding
)
```

The opened message contains the mode-specific fields and processor continuation.

Compute a shape commitment from typed, ordered edges:

```text
shape_d = H(
    "SherlockPCDShape/v1",
    node_kind,
    enabled_slot_bitmap,
    left_certificate_id,
    right_certificate_id,
    dependency_mapping_d,
    local_work_d,
    output_message_d
)
```

A disabled slot has one canonical identifier. Swapping children changes the shape. A shared producer has one certificate identity referenced by both consumers.

Keep semantic claim identity separate from proof-shape identity. Different aggregation schedules may prove the same semantic claim while having different shape commitments. Proof randomness must not create a new semantic run identity.

The current Quint model explicitly makes shape binding a property of the decider **plus off-chain manifest replay**. The decider does not read that manifest. That does not establish GP-4’s in-statement shape guarantee. [`models/sherlock/quint/aggregation.qnt:18`; `models/sherlock/quint/aggregation.qnt:223`]

### DAG reuse and exact coverage

Permit a verified claim to feed several consumers. For example:

```text
        A
       / \
      B   C
       \ /
        D
```

`B` and `C` each authenticate `A`. `D` authenticates both consumers. The claim collection contains `A` once, while the dependency graph contains both outgoing edges.

Do not sum child claim counts or financial effects without checking overlap. Propose a canonical authenticated map keyed by semantic claim identity. Insertion is idempotent for the same claim and rejects conflicting contents under the same identity.

Exact set union can require substantial work. Prove it as a streamed processor computation over the authenticated child collections. A hash of two set roots does not by itself prove a canonical, duplicate-free union. A partially completed union stays a `Continuation`.

Duplicate verification obligations are different from duplicate semantic effects. The recursive proof may authenticate a shared ancestor along both branches. That does not authorize counting its transfer or settlement twice.

### Extend the formal models

The existing Lean `link` requires equal programs and contiguous state/height boundaries. Its certificate datatype is a tree. Those definitions remain useful for **within-run segment composition**, but are not a model of cross-program DAG composition. [`clean/fork/Clean/Sherlock/Aggregation.lean:49`; `clean/fork/Clean/Sherlock/Aggregation.lean:72`]

Preserve and extend the properties as follows:

| Property | Proposed GP form |
|---|---|
| Unique genesis | Exactly one canonical genesis **per run identity**. A batch can contain many distinct runs and therefore many genesis states. |
| Exactly-once execution | For each run, enabled owner transitions tile its claimed interval without gaps or repeated positions. |
| DAG fan-out | Repeated references to a verified claim are allowed. Each reference has an explicit consumer and input position. |
| Exact batch coverage | The published claim set is the proven union of the selected claims, with duplicates handled canonically. |
| Replay prevention | Settlement effects are accepted once under a semantic nullifier policy. Rebracketing the proof cannot evade replay protection. |
| Shape binding | Every accepted root commits to its typed edges and local work. It is no longer conditional on an external manifest checker. |
| Fixed keys | No key depends on program, memory size, registry population, graph shape or depth. |
| Terminal completeness | A settled root contains no unfinished initialization, memory operation, verifier job, adapter or set-union job. |

The current Quint invariants reject duplicate accumulator certificate references and require all children to share the node’s program. Those clauses must change; retaining them would reject the required shared-ancestor DAG. [`models/sherlock/quint/aggregation.qnt:457`; `models/sherlock/quint/aggregation.qnt:464`]

Keep the existing height arithmetic lemmas for owner steps. Do not introduce a 64-bit **global PCD depth** counter as an extra acceptance bound. Model finite DAG well-foundedness with natural-number ranks in the formal model. The cryptographic construction must separately justify extraction of a finite valid computation graph. [`clean/fork/Clean/Sherlock/Aggregation.lean:1185`; `D:general-purpose.md:44`]

The key new refinement theorem should establish:

> A decided root yields a well-founded graph of typed claims. Every Sherlock run has authenticated initialization and the claimed execution. Every foreign claim satisfies its registered verifier and adapter. Every dependency is authenticated. Every published claim-set entry has the claimed provenance.

This theorem must retain explicit cryptographic premises. The current `leaf_sound` and `valid_accumulate` assumptions do not constitute proofs of the processor circuit or the accumulation scheme. [`clean/fork/Clean/Sherlock/Aggregation.lean:204`; `clean/fork/Clean/Sherlock/Aggregation.lean:1242`]

### Constant-size interpretation

The final verifier checks a fixed-width commitment to the graph and claim set. Opening or enumerating the entire set is a separate operation.

If “knows exactly which claims” were interpreted as reading every claim during the constant-cost final verification, the requirements would conflict: the verifier cannot read an arbitrarily long list in constant work. The commitment-and-opening interpretation matches GP-2’s explicit requirement that individual claims can be opened. [`D:general-purpose.md:25`; `D:general-purpose.md:47`]

## 5. On-chain fit

Retain one Midnight-format outer proof. Replace the program-sealed contract with a dynamic settlement interface.

A proposed interface binds:

```text
acceptRun(
    request_d,
    registry_policy_d,
    claim_set_d,
    shape_d,
    settlement_pre_d,
    settlement_post_d,
    optional_single_run_fields
)
```

The decider proves that these arguments describe its authenticated root. For a singleton, the fixed argument vector can include `prog_d`, `input_d`, outcome tag and `outcome_d`. For a batch, it exposes the claim-set commitment. Unused fields have canonical values.

The contract checks the authorized registry policy and the current settlement root. It updates the settlement root only after proof verification. The proved transition inserts semantic nullifiers, so another shape or another batch grouping cannot replay an already consumed effect.

The present contract seals `program_d` and `init_d` and stores a u128 result. Those choices must change. Its deployed operation must continue to use the decider key, rather than the compiled key of a template that only performs ledger operations. [`tools/sherlock-sdk/sherlock-run.compact:3`; `tools/sherlock-sdk/sherlock-run.compact:23`]

The relevant ledger limits are:

| Limit | Pinned source |
|---|---|
| Raw serialized verifier key: at most **50,000 bytes** | `L:transient-crypto/src/proofs.rs:422` |
| Public inputs per call: at most **65,536 field elements** | `L:onchain-runtime/src/transcript.rs:87`; enforcement at `L:ledger/src/verify.rs:2138` |
| Contract operations: at most **256** | `L:ledger/src/structure.rs:1928` |
| Ordinary public-input vector length must match the key | `Z:lib.rs:2020` |

Proposed interface target: **ESTIMATE fewer than 256 public-input elements**, including transcript overhead. The actual count must come from the recaptured Compact template.

There is no demonstrated k=14 verifier ceiling. The KZG verifier parameter object holds G2 material rather than a k-sized commitment basis, the ledger verification function has no explicit k limit, and the recorded k=20 decider passes the installed ledger verifier. [`P:poly/kzg/params.rs:235`; `L:transient-crypto/src/proofs.rs:563`; `qa/sherlock/decisions/speed-levers.json:95`]

Thus k=20 is a resource and architecture target here, not a source-proven ledger maximum. Increasing k still changes proving cost and SRS requirements and must not be presented as a free remedy.

Full transaction validation remains necessary. The earlier review found that the GP harness exercised `VerifierKey::verify`, while the named independent `verify_chain` example still belonged to the historical aggregation route. A successful cryptographic verification alone does not close contract integration. [`research/sherlock-recursion-design-review-20261110/review.md:164`]

## 6. Migration and milestones

### Preserve

- **Reference semantics and C6 rule reasoning.** Reuse them as the specification and differential oracle for the committed-memory interpreter. The existing full-carrier encoding is not the new memory architecture. [`D:design.md:27`; `crates/mori-backend/src/sherlock_c6_child.rs:551`]
- **Height and genesis-carry hardening.** Preserve their obligations and negative cases, while adapting the accumulator representation and disabled-slot normalization. [`crates/mori-recursion/src/ivc.rs:497`; `crates/mori-recursion/src/ivc.rs:517`]
- **Terminal pairing implementation.** Reuse its arithmetic and binding checks in the new decider. [`crates/mori-recursion/src/run_decider.rs:247`]
- **Transcript-template machinery and local ledger verification harness.** Reuse the mechanism with a newly captured dynamic contract template. [`crates/mori-recursion/src/run_decider.rs:290`]
- **Lean segment composition.** Keep it as a component of the typed PCD model. [`clean/fork/Clean/Sherlock/Aggregation.lean:958`]

### Replace

- `SHKIVC06` as the final processor: it has one predecessor and one fixed leaf key. [`crates/mori-recursion/src/ivc.rs:548`]
- The monolithic, capacity-bound universal leaf as the sole route to GP-1. [`D:design.md:52`; `D:general-purpose.md:13`]
- The fixed leaf-key field and foreign-proof assumptions.
- The halted-u128-only decider boundary.
- The program-sealed contract.
- Global program fixity and no-duplicate-reference clauses in the aggregation model.
- Off-chain-only graph-shape binding.

### Proposed milestone order

| Milestone | Working software delivered | Acceptance boundary |
|---|---|---|
| **M1: binary recursive kernel** | Two self slots, canonical disabled slots, a minimal inline transition, root accumulator and terminal decider. | Real proofs for base, unary and binary nodes under one node key. Tampered child, carry, key and shape refused through final verification. |
| **M2: committed-memory processor** | Authenticated loads, stores, continuation boundaries and canonical initialization. | One program exceeds the resident carrier window without changing keys. Malformed paths and interrupted writes cannot publish a completed claim. |
| **M3: complete Sherlock interpreter** | Program and input as data, every language rule, all outcome kinds and canonical structured values. | GP-1’s unseen-program acceptance suite passes under one frozen key set. |
| **M4: typed composition** | Consume verified results, authenticate input mappings, build claim sets and support DAG reuse. | GP-2 and GP-4 tests, including depth-eight shapes and a shared producer, pass under the same keys. |
| **M5: foreign-proof verification** | Registry admission, canonical VK interpretation, actual Compact Blake2b verification and statement adapters. | One proof combines a Sherlock run and a Compact contract proof, with both identities authenticated against the registry. |
| **M6: settlement and performance** | Dynamic contract, semantic replay protection, independent bundle verification and full local call validation. | All four GP acceptance suites and the separately retained throughput requirements pass. |

Develop the typed claim definitions and formal refinements alongside M1–M4. They must describe the implementation’s continuation and publication boundaries before that implementation is presented as a completed GP milestone.

Do not freeze final keys after M1 merely because recursion works. Freeze after the processor architecture, complete instruction semantics, statement encoding and decider relation are stable. Subsequent application programs, registry entries and graph shapes must use those keys unchanged.

### Proposed EARS requirements and design changes

These are proposed text, not file edits.

| Proposed requirement | Text |
|---|---|
| **GP-MEM** | The processor SHALL authenticate every program, input and mutable-memory access against committed storage. Storage capacity SHALL NOT depend on a proving key. |
| **GP-INIT** | Before the processor publishes a run segment as originating at height zero, it SHALL prove canonical initialization from the committed program, input and execution parameters. |
| **GP-FOREIGN** | When a foreign proof is consumed, the processor SHALL authenticate its complete verifier interpretation and statement adapter against an authorized registry commitment. |
| **GP-FORMAT** | When a claim asserts compatibility with a ledger proof format, the processor SHALL enforce that format’s transcript, encodings, instance layout and complete parsing rules. |
| **GP-SHAPE** | Every decided root SHALL bind the typed dependency graph and local work into its public statement. |
| **GP-DAG** | The processor SHALL permit a verified claim to feed multiple consumers. It SHALL prevent that reuse from duplicating execution coverage or settlement effects. |
| **GP-COMPLETE** | The decider SHALL refuse a root containing an unfinished initialization, memory, verifier, adapter or claim-set operation. |
| **GP-KEYS** | Changing a program, input, memory capacity, registry entry, graph shape or graph depth SHALL NOT require a different processor or decider key. |

Replace D4 and §§2.3–2.4 with the bounded processor and committed-memory design. Replace §3.1 with binary PCD. Extend §3.3 to typed terminal claims. Replace §3.6’s single-program segment model with typed composition while retaining segment linkage as a subrelation. Rewrite SRC-013/014 around processor work per proof, rather than a finite family or fixed-capacity carrier. [`D:design.md:12`; `D:design.md:46`; `D:design.md:70`; `D:design.md:114`; `D:requirements.md:35`]

Two previous positions should not survive this change:

1. Off-chain genesis recomputation cannot be the only authentication mechanism for claims consumed in-circuit.
2. The failed three-proof node does not establish that every binary node with new work requires k=21.

The first follows from the required consumption boundary. The second follows from the distinct two-self modeling result. [`D:general-purpose.md:24`; `qa/sherlock/decisions/speed-levers.json:26`]

## 7. Ranked experiments

All experiments below are future work. Costs are **ESTIMATE** for 20 cores, approximately 120 GB shared RAM and one heavy job at a time. They exclude implementation effort unless stated. A measurement that exceeds a cap is a failure, not permission to change the workload.

First ingest the already-running per-rule, leaf-headroom, genesis and k=20 leaf results. Do not duplicate them. They inform the comparison with the present route, but do not measure the proposed binary processor. [`research/sherlock-gp-architecture-20261110/BRIEF.md:24`]

| Rank | Hypothesis | Measurement | Kill criterion | Rough machine cost |
|---|---|---|---|---|
| **1. Binary node and decider closure** | Two self slots, canonical disabled encodings and useful inline work fit at k=20. | Model exact self architecture, final statement width, both active slots, base case and decider. Then prove base/unary/binary cases and run fresh invalid-child tests through the decider. | Node exceeds 996,147 rows with the minimum useful kernel, or decider exceeds its approved envelope. Any normalization soundness failure stops adoption. | **ESTIMATE:** 5–20 minutes and 2–5 GB for modeling; 15–45 minutes and 20–40 GB for a small proof closure. |
| **2. Memory and initialization budget** | Committed-memory operations leave enough useful work per node to avoid excessive proof counts. | Rows per tree compression, read, write, page operation and continuation boundary. Test growing storage with identical keys. Include malformed genesis and partial-write publication. | Capacity changes keys; path length becomes a hidden admission cap; or measured work per node cannot support the throughput budget. | **ESTIMATE:** 10–40 minutes modeling; 30–90 minutes for selected proofs, 20–40 GB peak. |
| **3. Actual Compact proof compatibility** | A processor implementation can reproduce ledger verification exactly. | Use a real ZKIR v3 contract proof. Compare transcript challenges, VK representative, PI vector, KZG reduction and acceptance with the pinned native verifier. Mutate every boundary, including trailing bytes. | Requires reissuing the proof under Poseidon, trusting a host verifier result, or changing keys for each foreign circuit. | **ESTIMATE:** 15–60 minutes for native differential work; 1–6 hours for the first segmented in-circuit proof, 20–40 GB peak. |
| **4. Dynamic architecture and registry admission** | Different foreign architectures can be authenticated without changing `N` or `D`. | Admit keys with different chip sets, k, PI counts and verifier descriptors. Test descriptor/key mismatch, wrong adapter, unauthorized root and alternate SRS. | Any accepted mismatch, or a new architecture requires rebuilding the processor key. | **ESTIMATE:** 30–120 minutes for small modeled cases; several hours for proof-backed cases, depending on verifier trace length. |
| **5. DAG and claim-set semantics** | Fan-out and exact coverage coexist without forbidding shared ancestors. | Extend Lean and Quint with typed claims and shared-node maps. Exercise diamond graphs, repeated references, conflicting identities, omitted union members and replay under another shape. | A shared ancestor is refused merely because it appears twice, or a reused ancestor duplicates an effect. | **ESTIMATE:** 5–30 minutes for bounded model runs; proof-backed minimal cases take 30–120 minutes. |
| **6. Complete GP acceptance and throughput** | The same frozen keys satisfy all four GP tests at acceptable cost. | Unseen programs, oversized state, input, every outcome, cross-run consumption, mixed-program batch, Compact proof, chain/tree/DAG depth ≥8, equal terminal-proof lengths and local call validation. Record total kernel steps, proof count and wall time. | Any requirement needs a narrower profile or new application key. Existing two-hour workload target still fails if total measured wall exceeds it. | **ESTIMATE:** 8–16 hours for a straightforward full binary depth-eight suite on the KZG candidate. A perfect binary tree of that depth can require hundreds of node proofs. Peak 20–40 GB. |
| **7. Different inner proof system** | Recursive STARKs or multi-folding reduce total processor cost enough to justify a new inner layer. | Port the same memory/claim workload to one STARK candidate and one multi-folding candidate. Measure binary composition, foreign verifier cost and the complete Midnight bridge. | No fixed-key PCD implementation, no actual Compact proof path, or no wrapper within the resource envelope. Inner-only speedups do not count. | **ESTIMATE:** 1–4 hours per small benchmark after implementation; direct wrapper probes may exceed 48 GB. Engineering cost is **ESTIMATE weeks**, not a library substitution. |
| **8. GPU acceleration after architecture selection** | Offloading measured MSM/NTT work materially improves the selected end-to-end path. | Measure exclusive offloadable phases, mixed CPU/GPU memory pressure, group equality and final stock-verifier acceptance. | Gains do not meet the remaining wall-time gap, or shared-memory usage breaches the cap. | **ESTIMATE:** 15–60 minutes per qualified comparison, excluding backend development. |

Rank 7 is an architectural fallback, not a reason to postpone the inexpensive row test in rank 1. Its first wrapper measurement should happen before investing in a full alternative VM.

GPU work cannot change the number of rows required by the recursive relation. The existing backend seam accelerates MSM execution, while the machine’s CPU and GPU share memory. The current speed record also contains an overlapping `m + q` sum; the review correctly distinguishes it from the union of those phases. [`P:poly/kzg/msm.rs:163`; `research/sherlock-speed-replan-20261108/gpu.md:14`; `research/sherlock-recursion-design-review-20261110/review.md:50`]

The decisive measurements are therefore the **hardened binary node**, the **complete decider**, and **useful processor work per proof**. If they fail, retain the complete GP specification and change the inner proof system.