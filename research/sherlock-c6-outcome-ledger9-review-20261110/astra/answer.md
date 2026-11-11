# Q1

**Verdict: adopt with changes.**

Outcome-only publication is the right semantic boundary. It does not, by itself, make M3 sound and complete. Adoption must include canonical value commitments, relocation-preserving execution, partition-independent resource accounting, and replacement of the current contract statement. The existing implementation publishes `fin_d` and accepts only a halted `u128`. [Sources: `crates/mori-recursion/src/run_decider.rs:278`, `tools/sherlock-sdk/sherlock-run.compact:40`.]

Source abbreviations used below:

- `D` = `openspec/changes/sherlock-vm-consensus-20261107`
- `HM` = `crates/mori-core/src/sherlock/helper_machine`
- `L9` = `repos/midnightntwrk/midnight-ledger`, **tag `ledger-9.1.0.1`**
- `REG` = `/home/hoskinson/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f`

Statements labelled **Proposal** are recommended changes, not descriptions of implemented behavior.

## 1. Soundness, completeness, and partition independence

### What the change fixes

The decider currently checks:

```text
fin_d = Poseidon(SHK_SD, HALTED, result, lanes_d)
```

It then includes `fin_d` in the public values and run statement. The witness producer computes `lanes_d` from the raw image, data, and helper carriers. Consequently, different physical layouts can produce different public statements for the same semantic result. [Sources: `crates/mori-recursion/src/run_decider.rs:279`, `crates/mori-recursion/src/run_decider.rs:291`, `crates/mori-backend/src/sherlock_c6_witness_stream.rs:216`.]

This also affects replay protection: `acceptRun` rejects a previously stored `stmt`, and `stmt` currently includes `fin_d`. **Inference:** if two valid partitions produce distinct `fin_d` values, they produce distinct replay identities except for a hash collision. Canonicalizing only the separately published result does not fix this. [Sources: `tools/sherlock-sdk/sherlock-run.compact:12`, `tools/sherlock-sdk/sherlock-run.compact:43`; `qa/gemini-quint/chipleaf-pairrow-m3/verdict.md:41`.]

### What must additionally hold

**Proposal:** define a logical state independently of its physical representation. A relocation maps each live logical object to a written physical record and preserves:

- Record kinds and scalar contents.
- Ordered fields and reference targets.
- Environment links, frames, tasks, and control.
- Logical allocation counts and all semantic resource counters.

Discarding segment padding and applying the inverse relocation must recover the same logical state. Each chip transition must refine one logical transition, and leaf closure must preserve that logical state.

Under that refinement, deterministic logical execution and a binding canonical value commitment imply that two accepted runs with the same authenticated program, input, and resource policy have the same outcome. This is a proposed proof obligation; the current M3 model uses partition-dependent allocation in its reference and therefore does not establish it. [Source: `qa/gemini-quint/chipleaf-pairrow-m3/verdict.md:130`.]

Three gaps prevent an unconditional soundness-and-completeness conclusion:

1. **Dense-allocation assumptions remain.** Fresh data and environment identifiers equal active lengths. SI-12 requires consecutive field blocks, SI-13 requires earlier data references, and SI-14 requires earlier environment links. These checks need a logical-to-physical refinement, not deletion. [Sources: `HM/shared/owner_runtime.rs:3021`, `HM/shared/owner_runtime.rs:4698`, `HM/shared/owner_runtime.rs:4742`, `HM/shared/owner_runtime.rs:4807`.]

2. **Physical exhaustion must not become a semantic outcome.** D3 reserves eight addresses per leaf, including leaves that append nothing. Its 17-bit segment index gives a fixed physical address space. **Inference:** sufficiently many early closures can exhaust that space without corresponding logical allocation. Publishing a canonical digest cannot repair this completeness limit. It must neither cause nor authorize a different `Exhausted(kind)`. [Sources: `D/design.md:97`; `qa/gemini-quint/chipleaf-pairrow-m3/verdict.md:41`; `HM/shared.rs:367`.]

3. **Genesis and input admission remain necessary.** A terminal commitment proves the intended run only if the initial state is constrained to `Init(prog_d,input_d)`. Input well-formedness and program admission are still explicit open premises. [Source: `D/requirements.md:43`.]

An invalid or overfilled partition may have no proof. That is different from proving a semantic rejection. GP-1 additionally requires a suitable proof decomposition for every supported logical execution without a capacity-specific key. The current fixed-depth M3 prototype does not establish that property. [Sources: `D/general-purpose.md:13`, `D/design.md:97`.]

### Remaining public surfaces

| Surface | Finding and required treatment |
|---|---|
| `acceptRun` | Replace the current sealed `init_d`, public `fin_d`, and `Bytes<16>` result interface. Use authenticated program/input identities and a canonical outcome commitment. The current interface is unsuitable for general outcomes. [Source: `tools/sherlock-sdk/sherlock-run.compact:23`.] |
| Node statement | It still contains `init_d` and `cur_d`; leaf statements contain `pre_d` and `post_d`. These may remain address-dependent **internal proof inputs**, provided the outer decider verifies their linkage and does not export them as application claim identities. [Source: `crates/mori-recursion/src/ivc.rs:67`.] |
| GP-2 claims | Consume `(prog_d,input_d,outcome_d)`, with the proof checked in-circuit. A consumer that uses the value must also check its canonical opening. [Source: `D/general-purpose.md:24`.] |
| GP-4 shape | Preserve a separate authenticated shape commitment. Do not build it from raw state roots, proof bytes, accumulator encodings, or physical addresses. GP-4 explicitly requires shape binding. [Source: `D/general-purpose.md:47`.] |
| Step count | It can remain optional semantic metadata if it counts logical events, excluding leaf padding and publication-hash work. The current node height counts machine steps rather than leaves. [Source: `crates/mori-recursion/src/ivc.rs:70`.] |

**Proposal:** distinguish the semantic run claim from the aggregation envelope:

```text
run_claim = (encoding_version, prog_d, input_d, outcome_d)
aggregation_envelope = (claims_d, shape_d)
```

Different aggregation shapes may legitimately produce different `shape_d` values. The outcome and semantic replay identity must remain unchanged. Requiring the entire public statement to be identical across all shapes would conflict with GP-4.

## 2. Canonical value encoding and circuit computation

### Value domain

The complete observation types include `U128`, `Bool`, `Nil`, `Cons`, `Leaf`, `Branch`, and general `Constructor` values. References distinguish constant and dynamic storage. Structural equality dereferences values and recursively compares fields, with visited-pair tracking for shared DAGs. Thus storage sharing is not part of value equality. [Sources: `HM/complete_observation.rs:299`, `HM/complete_observation.rs:334`, `HM/shared.rs:702`, `HM/shared/owner_runtime.rs:3887`.]

### Proposed encoding: `SherlockValue/1`

**Proposal:** specify a canonical typed tree encoding and a recursive Merkle digest of that encoding. This must be stated explicitly: the digest below is a hierarchical commitment, not a flat Poseidon hash over an expanded serialization.

Let:

- `F` be the existing BLS12-381 scalar field.
- `word(x)` be exactly 32 little-endian bytes encoding the canonical integer representative of `x ∈ F`.
- `D_VAL = dom("SHK_VALUE_V1")`.
- `D_OUT = dom("SHK_OUTCOME_V1")`.
- `H(xs)` be the pinned fixed-length Poseidon hash of the field sequence `xs`, with the actual sequence length supplied to its length-domain initialization.

The existing domain helper packs up to 16 ASCII bytes little-endian. The pinned Poseidon implementation initializes its capacity with the input length. [Sources: `crates/mori-recursion/src/util.rs:13`, `crates/mori-recursion/src/parent.rs:115`, `REG/midnight-circuits-7.3.0/src/hash/poseidon/poseidon_cpu.rs:131`.]

Every value starts with four words:

```text
[D_VAL, 1, tag, arity]
```

The following table defines both the canonical tree encoding and its digest.

| Value | Tag / arity | Canonical encoding after the header | Digest preimage after the header |
|---|---:|---|---|
| `UInt(n)` | `0 / 0` | `word(n)`, with `0 ≤ n < 2^128` | `n` |
| `Bool(b)` | `1 / 0` | `word(b)`, with `b ∈ {0,1}` | `b` |
| `Nil` | `2 / 0` | Empty | Empty |
| `Cons(head,tail)` | `3 / 2` | `E(head) ‖ E(tail)` | `d(head), d(tail)` |
| `Leaf(n)` | `4 / 1` | `E(UInt(n))` | `d(UInt(n))` |
| `Branch(left,right)` | `5 / 2` | `E(left) ‖ E(right)` | `d(left), d(right)` |
| General constructor with fields `v₀…vₘ₋₁` | `6 / m` | `word(p) ‖ word(t) ‖ word(c) ‖ E(v₀)…E(vₘ₋₁)` | `p, t, c, d(v₀)…d(vₘ₋₁)` |

For every row, `d(v) = H(header ‖ digest_payload)`.

For a general constructor:

- `p` is the canonical program identity that owns its nominal type.
- `t` is its canonical admitted type-table identifier.
- `c` is the constructor’s source-order ordinal within that type.
- `m`, `t`, and `c` use canonical unsigned 64-bit integers.
- Admission must authenticate the type arguments, owning declaration, constructor ordinal, field count, and ordered field types.
- Builtin constructors always use their dedicated rows above.

This program-scoped type identity prevents unrelated declarations with the same local constructor number from colliding. These are authenticated program identifiers, not runtime allocation addresses. The current type and constructor tables contain the required declaration, argument, ownership, and source-order information. [Sources: `HM/shared/adt.rs:35`, `HM/shared/adt_storage.rs:4`.]

**References:** `E(Const(i))` and `E(Data(j))` mean the encoding of the dereferenced value. Neither reference tag nor index is encoded.

**Nested and shared structure:** recurse in field order. Repeated references and separately allocated equal values produce the same digest. The conceptual encoding expands sharing, but its digest can be computed once per distinct reachable object.

**Invalid values:** dangling references, cycles, padding records, invalid scalar encodings, wrong arity, and ill-typed fields do not have a valid halted-value encoding. `U128OutOfRange` is an observation variant, not an additional valid scalar value. Backward data-reference checks currently provide an acyclicity condition that the new representation must preserve logically. [Sources: `HM/complete_observation.rs:329`, `HM/shared/owner_runtime.rs:4742`.]

### Proposed outcome encoding

```text
Halted(v):      H([D_OUT, 1, 0, d(v)])
Rejected(code): H([D_OUT, 1, 1, code])
Exhausted(k):   H([D_OUT, 1, 2, k])
```

`code` is a range-checked `u8` and must be justified by the executed terminal transition.

Define exhaustion codes explicitly, independently of Rust enum layout:

```text
0 Fuel
1 DataNodes
2 DataFields
3 EnvironmentCells
4 HelperTasks
5 HelperWork
6 SerializationBytes
7 LogicalCap
```

These names cover the current exhaustion enum. `LogicalCap` applies only to its supporting profile. `Running` and profile `/2`’s `Waiting` are not terminal outcomes. [Sources: `HM/shared.rs:367`, `HM/shared.rs:1041`.]

### Computing the digest in circuit

**Proposal, preferred implementation:** maintain an authenticated value digest when each immutable value is created.

1. Range-check scalars and constrain their digest.
2. For a constructor, authenticate its type and ordered field references.
3. Authenticate each child’s previously established digest.
4. Compute the constructor digest and bind it to the immutable value record.
5. At halt, authenticate the result record and copy its digest into the outcome computation.

Constant and input values require the same digest invariant at admission. Merely storing a host-provided digest beside a value is unsound.

This uses write-once storage and avoids an unbounded traversal inside the fixed-size decider. The proposed chip already requires append-only data and field tables. [Source: `D/design.md:85`.]

**Alternative:** a separate recursive finalization phase can traverse only the result’s reachable graph after execution freezes. It must authenticate every opened record against the final memory or program root, verify every edge and type, and prove every memoized digest. A private worklist is acceptable; an unconstrained host traversal is not.

Do not serialize the entire machine state. Do not encode DFS back-reference numbers: those preserve sharing differences that structural value equality ignores. Physical-address memoization is an internal optimization only.

### Cost

The measured M3 HASH implementation uses 70 rows per Poseidon permutation and two absorbed field words per permutation. [Sources: `research/sherlock-processor-analogy-20261110/experiments-4.md:157`; `REG/midnight-circuits-7.3.0/src/hash/poseidon/constants/mod.rs:20`.]

**ESTIMATE — hashing only, using that implementation:**

| Value record | Field words hashed | Estimated rows |
|---|---:|---:|
| UInt or Bool | 5 | 210 |
| Nil | 4 | 140 |
| Cons or Branch | 6 | 210 |
| Leaf | 5 | 210 |
| Constructor with `m` fields | `7 + m` | `70 × ceil((7+m)/2)` |
| Outcome wrapper | 4 | 140 |

These estimates exclude authentication, range checks, type checks, bus constraints, memoization, and storage overhead.

For `V` distinct reachable value nodes and `E` field edges, memoized hashing takes **ESTIMATE `O(V+E)` work**. Independent depth-20 record openings add approximately **ESTIMATE 1,400 hashing rows per opened record**, before record hashing and access constraints. Shared Merkle paths can reduce that cost. The current M3 tree consists of eight-record segments under a depth-17 upper tree. [Source: `research/sherlock-processor-analogy-20261110/experiments-4.md:137`.]

The cheapest halt path is therefore an authenticated cached digest: one result opening plus the outcome wrapper. Its total lifetime cost may exceed reachable-only finalization because it also hashes values that never reach the result. Both approaches require measurement.

## 3. Effects on GP-1 through GP-4

| Property | Required treatment |
|---|---|
| **GP-1** | Support every listed value and every terminal outcome under the universal key set. Canonicalize input values too. Include every semantic resource-policy parameter in authenticated program/input data. A fixed-capacity output traversal or memory tree cannot silently become the new universality limit. [Source: `D/general-purpose.md:7`.] |
| **GP-2** | Verify the producer claim in-circuit. If the consumer needs the value, verify its canonical opening and type. Cross-program nominal types require an explicit authenticated mapping or preservation of the producer’s namespace. Independent-run batching must commit to openable claims, including its defined ordering or set semantics. [Source: `D/general-purpose.md:20`.] |
| **GP-3** | Keep authenticated key-registry membership and statement adapters. An adapter must bind the foreign public inputs to the common typed claim; it cannot relabel an arbitrary foreign field as a Sherlock value digest. Foreign proofs retain their actual proof-system and transcript obligations. [Source: `D/general-purpose.md:31`.] |
| **GP-4** | Keep chains, trees, and DAGs, arity at least two, depth-independent keys, and constant-size final verification. Bind the ordered edges and reuse of claim occurrences with address-free descriptors. Shape may differ while the semantic outcome remains equal. Replay protection must not use shape as a fresh execution identity. [Source: `D/general-purpose.md:42`.] |

Outcome-only publication must mean **only the outcome is exported from execution state**. It cannot mean removal of program identity, input identity, registry authentication, claim-set binding, or shape binding.

## 4. Required changes and exact EARS text

### Locations to change

- Replace SRC-006 and SRC-007 in `requirements.md` and `specs/sherlock-recursion/spec.md`; both currently specify the raw final digest and `u128` result. [Sources: `D/requirements.md:28`, `D/specs/sherlock-recursion/spec.md:57`.]
- Amend SRC-001, SRC-003, SRC-017, and SRC-021 for internal state commitments, semantic step counts, canonical claims, and input admission. [Sources: `D/specs/sherlock-recursion/spec.md:7`, `D/specs/sherlock-recursion/spec.md:27`, `D/specs/sherlock-recursion/spec.md:167`, `D/requirements.md:43`.]
- Update design §§2.7.3, 2.7.8 D3, 2.7.11, 3.2, 3.3, and 3.6. Preserve GP-001 through GP-004 and extend their acceptance cases. [Sources: `D/design.md:85`, `D/design.md:97`, `D/design.md:137`, `D/design.md:228`, `D/general-purpose.md:58`.]
- Extend tasks 2.4–2.5, 4.2–4.7, 4.9, 4.14, and 5.1a–5.1c to cover the new relation, statement, template, key cascade, and end-to-end validation. [Sources: `D/tasks.md:39`, `D/tasks.md:64`, `D/tasks.md:82`, `D/tasks.md:88`.]

### Proposed replacement design text

> Execution state commitments bind the physical witness and remain internal to recursive verification. External run claims bind the encoding version, canonical program identity, canonical input identity, and canonical outcome digest. Leaf partitioning may change physical memory commitments and aggregation shape. It shall not change logical execution, semantic resource accounting, the outcome digest, or semantic replay identity. Value commitments follow SherlockValue/1 and erase storage addresses and sharing. The final relation authenticates the terminal outcome through the verified execution. A host-computed digest alone provides no acceptance authority.

### Proposed EARS requirements

**C6-001 — External claim**

> When the decider publishes a run claim, it SHALL expose the encoding version, `prog_d`, `input_d`, and `outcome_d`. It SHALL NOT expose `init_d`, `pre_d`, `post_d`, `mem_root`, `lanes_d`, or physical allocation identifiers as execution-result or replay-identity components.

**C6-002 — Canonical value**

> When a run halts with value `v`, the proof SHALL establish `outcome_d` using SherlockValue/1 and the Halted outcome encoding. The proof SHALL authenticate every value record, type identifier, constructor ordinal, field position, and child digest used in that computation.

**C6-003 — Representation independence**

> When two valid value representations differ only in constant-versus-data storage, physical addresses, padding, or sharing of equal subvalues, their SherlockValue/1 digests SHALL be equal.

**C6-004 — Logical refinement**

> When an enabled chip step executes, its decoded logical post-state SHALL equal the reference transition from its decoded logical pre-state. When a leaf closes, its decoded logical state SHALL remain unchanged.

**C6-005 — Resource independence**

> When a leaf closes or publication hashing proceeds, semantic fuel and logical resource counters SHALL remain unchanged. If physical proving capacity is unavailable, the prover SHALL NOT encode that condition as a semantic `Rejected` or `Exhausted` outcome.

**C6-006 — Universal capacity**

> When a supported run exceeds one leaf’s storage or hashing capacity, the prover SHALL continue through authenticated memory and recursive proof decomposition under the same key set, subject to the GP-1 run-length bound.

**SRC-006 — Decider pins and discharges**

> When the decider accepts a run, it SHALL verify the recursive root under the authorized verification relation, discharge every deferred accumulator obligation, establish canonical initialization and execution linkage, and authenticate one terminal outcome. It SHALL bind that outcome to the external run claim using C6-001 and C6-002.

**SRC-007 — Ledger binding and replay**

> When `acceptRun` accepts a run claim, it SHALL bind the claim fields to the verified call transcript, store the canonical outcome commitment, and reject an already consumed semantic run identity. Changes to leaf partition, proof randomness, physical layout, or aggregation shape SHALL NOT create a new semantic run identity.

**SRC-021 addition — Input and policy**

> When the first execution segment begins, the proof SHALL authenticate the canonical input value, dependency claims, and every resource-policy parameter that can affect the outcome. It SHALL establish `Init(prog_d,input_d)` from those authenticated values.

**GP-002 addition — Consumption**

> When a consumer uses a producer’s outcome, it SHALL verify the producer claim in-circuit. When the consumer uses a value behind that outcome, it SHALL verify its SherlockValue/1 opening and the required type relation.

**GP-003 addition — Adapter binding**

> When an admitted foreign proof contributes a claim, the node SHALL verify registry membership and apply the authenticated statement adapter to the verified foreign public inputs.

**GP-004 addition — Shape**

> When proofs are aggregated, the proof SHALL bind the ordered edges and reused claim occurrences of the aggregation graph through address-free descriptors. The final statement SHALL commit to that graph without requiring physical execution-state commitments as graph identifiers.

### SI changes

**Proposal:** retain dense logical identifiers and authenticate their mapping to sparse physical storage. This preserves the existing reference invariants while changing the chip representation.

- **SI-10:** logical active counts count live logical records; physical high-water marks are separate.
- **SI-12:** logical field blocks retain exact ownership and field order. Their physical records may span segments through an authenticated mapping.
- **SI-13:** references resolve to authenticated written records of the correct kind. Data edges retain strict logical creation order. Padding never resolves as a value.
- **SI-14:** environment predecessors are absent or refer to earlier logical environment cells.
- Fresh logical identifiers remain equal to logical active counts; they are not required to equal physical append addresses.

These changes address the actual current checks rather than weakening them. [Sources: `HM/shared/owner_runtime.rs:3021`, `HM/shared/owner_runtime.rs:4676`, `HM/shared/owner_runtime.rs:4698`, `HM/shared/owner_runtime.rs:4742`, `HM/shared/owner_runtime.rs:4807`.]

Required future acceptance cases include multiple partitions of the same execution, shared versus duplicated structures, constant versus dynamic values, every terminal outcome, resource-boundary cases, malformed graph openings, replay across partitions, and GP-2/GP-4 composition. These are proposed checks, not checks performed in this review.

# Q2

**Verdict: yes with changes.**

The inspected verifier architecture supports this route on ledger 9. The evidence does **not** establish that the completed chip design and revised outcome statement already run on the deployed network. The final chip measurement used an unsafe SRS and produced no proof; full local validation remains an open task. [Sources: `research/sherlock-processor-analogy-20261110/experiments-4.md:26`, `D/tasks.md:90`.]

## Evidence table

| Check | Finding | Source |
|---|---|---|
| Ledger and node versions | The public `node-2.1.101` tag pins `midnight-ledger-v9 =1.0.1` and runtime spec `002_001_100`. | `repos/midnightntwrk/midnight-node`, tag `node-2.1.101`, `Cargo.toml:104`, `runtime/src/lib.rs:283`. |
| Deployed provenance | Pins record node build `913ec011` and ledger revision `3e69c5e3`; the public node tag has a different commit. This establishes recorded version alignment, not exact deployed-source identity. | `qa/sherlock/target-pins.json:21`, `qa/sherlock/target-pins.json:51`. |
| Cryptographic dependency versions | Moriarty pins proofs 0.8.3, circuits 7.3.0, and stdlib 2.4.0. The ledger tag’s lockfile resolves proofs 0.8.2, while its manifest permits `^0.8.2`. Cross-version compatibility still needs qualification. | `Cargo.toml:43`; `L9:transient-crypto/Cargo.toml:32`; `L9:Cargo.lock:3509`. |
| Installed verifier | `SherlockRun` must install the hand-written run-decider VK, replacing the compiled `acceptRun` VK. Deployment code supplies that key and checks its installed identity. | `tools/sherlock-sdk/sherlock-run.compact:3`; `tools/sherlock-sdk/deploy-run.mjs:41`, `:57`. |
| Ledger dispatch | `ProofVersioned::V3` selects `ContractOperation.v3` and invokes the current verifier with the call’s public inputs. The maintenance wrapper for this key is named `ContractOperationVersionedVerifierKey::V4`; these version numbers name different layers. | `L9:ledger/src/structure.rs:490`, `:3106`; `L9:ledger/src/verify.rs:2069`. |
| Outer proof system and transcript | The ledger uses stdlib verification over BLS12-381 KZG with `blake2b_simd::State`. The Sherlock harness proves the outer decider with that transcript. | `L9:transient-crypto/src/proofs.rs:63`, `:569`; `REG/midnight-zk-stdlib-2.4.0/src/lib.rs:1539`; `crates/mori-recursion/tests/sherlock_gp.rs:37`, `:323`. |
| Inner transcript | Leaf/node recursive verification uses Poseidon. Poseidon state and value hashing do not change the outer ledger transcript requirement. | `crates/mori-recursion/src/inner.rs:102`; `crates/mori-recursion/tests/sherlock_gp.rs:36`. |
| Recursive discharge | The decider verifies the node, binds its carried accumulator, resolves fixed bases, and constrains the final pairing. Ledger verification of the outer proof is therefore the intended discharge path. | `crates/mori-recursion/src/run_decider.rs:208`, `:230`, `:247`, `:273`. |
| SRS | Ledger embeds `bls_midnight_2p14` for verifier parameters. Its authenticated data catalogue also lists ceremony files through `bls_midnight_2p25`, including the pinned `2p20` hash. | `L9:transient-crypto/src/proofs.rs:108`, `:124`; `L9:base-crypto/src/data_provider.rs:182`, `:207`; `qa/sherlock/manifests/run-decider.json:28`. |
| Meaning of maximum `k` | In proofs 0.8.3, KZG verifier parameters contain pairing parameters, not a circuit-size `k`. VK decoding bounds `k` by field two-adicity, and domain construction also bounds the extended domain. Therefore the embedded `2p14` file is not evidence of a `k=14` circuit ceiling. The standard catalogue reaches `k=25`; this is not a claim that every circuit at that size passes other limits. | `REG/midnight-proofs-0.8.3/src/poly/kzg/params.rs:235`, `:273`; `REG/midnight-proofs-0.8.3/src/plonk/mod.rs:169`; `REG/midnight-proofs-0.8.3/src/poly/domain.rs:38`; `REG/midnight-curves-0.3.1/src/bls12_381/fq.rs:126`. |
| Conflicting ledger comment | The public-input-cap comment says the `2p14` setup limits circuits to `2^14` rows. That explanation conflicts with the inspected 0.8.3 verification mechanism. It should not be used to reject `k=20`; exact 0.8.2/deployed compatibility remains unverified here. | `L9:onchain-runtime/src/transcript.rs:80`; preceding KZG sources. |
| Decider rows | The final chip-key model reports 929,166 non-table rows and `k=20`. It also reports 1,032,467 table rows. Table occupancy and unusable rows matter; 929,166 alone does not describe domain headroom. | `/home/hoskinson/.cache/chip4/logs/node1.log:912`; `REG/midnight-proofs-0.8.3/src/dev/cost_model.rs:135`, `:370`. |
| VK byte limit | The raw serialized VK buffer is limited to 50,000 bytes. The existing decider manifest records a 2,217-byte tagged verifier artifact, whose raw payload is therefore smaller. This is evidence for that existing artifact, not a measurement of the revised outcome key. | `L9:transient-crypto/src/proofs.rs:417`; `qa/sherlock/manifests/sherlock-run-contract.json:2`; `crates/mori-recursion/tests/sherlock_keygen_regression.rs:115`. |
| Public inputs | The hard call cap is 65,536 field elements. Inputs include binding, communication commitment, and transcript operations—not just application arguments. The existing template is documented as 70 slots. A new template must be counted again. | `L9:onchain-runtime/src/transcript.rs:65`, `:87`; `D/design.md:230`. |
| Proof size | **ESTIMATE:** the final chip/node decider cost model reports 6,320 proof bytes. This is a model prediction, not a produced proof or complete transaction size. | `/home/hoskinson/.cache/chip4/logs/node1.log:912`; `research/sherlock-processor-analogy-20261110/experiments-4.md:26`. |
| Transaction size | Ledger initial limits specify 1 MiB per transaction and enforce the target parameter value. The node tag additionally sets a 1 MiB block length with a 75% normal-dispatch ratio. Wallet components, signatures, transcripts, and extrinsic overhead must be included. | `L9:ledger/src/structure.rs:1283`; `L9:ledger/src/verify.rs:628`; node tag `node-2.1.101`, `runtime/src/lib.rs:307`, `:319`. |
| Fees and cost limits | Verification cost includes VK loading and a term proportional to public-input count. Declared transcript gas is checked against block limits. Initial limits include compute/read time, block usage, bytes written, and bytes churned. No final fee can be established without the actual transaction and target parameters. | `L9:ledger/src/structure.rs:1138`, `:1287`, `:2370`; `L9:ledger/src/verify.rs:2015`. |
| No ZKIR requirement for inner circuits | Ledger verifies the outer VK and proof; it does not receive or compile the chip leaf. The current outer key reader reconstructs a supported stdlib architecture. Thus a hand-written inner leaf is compatible, but an arbitrary unsupported outer VK serialization is not automatically compatible. | `L9:transient-crypto/src/proofs.rs:534`, `:569`; `REG/midnight-zk-stdlib-2.4.0/src/lib.rs:1573`; `crates/mori-recursion/src/run_decider.rs:208`. |
| Curve and commitment compatibility | The recursive verifier uses `KZGCommitmentScheme<Bls12>` and `BlstrsEmulation`; stdlib’s outer VK uses the same curve/commitment family. The final chip measurement’s SRS was unsafe, so production SRS identity is still an outstanding artifact check. | `crates/mori-recursion/src/inner.rs:32`, `:104`; `REG/midnight-zk-stdlib-2.4.0/src/lib.rs:1543`; `/home/hoskinson/.cache/chip4/logs/node1.log:912`. |
| Outcome-only state | A field-sized outcome commitment fits the existing use of `Field` ledger values. The present contract nevertheless exposes a 16-byte result and sealed program/initial-state identities, so it needs a new interface and template. | `tools/sherlock-sdk/sherlock-run.compact:23`, `:40`. |
| Current qualification | The contract manifest says template recapture is pending. Tasks still require complete proof bundles, independent recursive verification, and proof-verifying local ledger validation. | `qa/sherlock/manifests/sherlock-run-contract.json:12`, `:48`; `D/tasks.md:88`. |

For scale only, the bundled mainnet cost coefficients give **ESTIMATE 3.59 ms** for the proof-verification term at 70 public inputs, before VK loading, scaling factors, VM work, storage, and other transaction components. This is cost-model arithmetic, not measured runtime or a fee quote. The bundled JSON and current source defaults are different parameter representations; neither substitutes for observed target parameters. [Sources: `L9:ledger/params/mainnet.json:127`, `L9:ledger/src/structure.rs:1138`, `:1257`.]

## Blockers and fixes

1. **The revised semantic relation is not implemented.**  
   Implement Q1’s value invariant, relocation refinement, terminal outcomes, and partition-independent resource accounting. The chip still has stand-in rules and lacks PAGE, PROG, TRAP, and complete initialization. [Source: `research/sherlock-processor-analogy-20261110/experiments-4.md:319`.]

2. **The existing contract statement cannot express the proposal.**  
   Replace the `fin_d`/`Bytes<16>` interface, define semantic replay identity, and preserve separate claim-set and shape commitments. Recapture the actual Compact 0.36.0 transcript, including every operation and communication-commitment input. [Sources: `tools/sherlock-sdk/sherlock-run.compact:40`; `qa/sherlock/manifests/sherlock-run-contract.json:12`.]

3. **The final chip proof lacks production-SRS evidence.**  
   Generate the leaf, node, and decider closure using authenticated ceremony parameters. Check the inner accumulator pairing parameters and outer ledger parameters explicitly. Measure the resulting raw VK, proof, public-input vector, and complete transaction. [Sources: `research/sherlock-processor-analogy-20261110/experiments-4.md:26`; `crates/mori-recursion/tests/sherlock_keygen_regression.rs:103`.]

4. **Exact ledger compatibility is not yet demonstrated.**  
   Validate the complete call with proof verification and limit enforcement on ledger 9.1.0.1 using the new template. Exercise altered program, input, outcome, replay, and transcript cases. Check compatibility with the ledger’s 0.8.2 resolution and record the deployed node/build distinction. [Sources: `D/tasks.md:90`; `L9:Cargo.lock:3509`; `qa/sherlock/target-pins.json:51`.]

5. **Limits and fees need the actual target state.**  
   Fetch and record the target `LedgerParameters`, serialize the balanced transaction, and check both ledger and node limits before submission. The small modeled proof is encouraging but insufficient evidence for transaction acceptance. [Sources: `L9:ledger/src/verify.rs:628`, `:2015`; node tag `node-2.1.101`, `runtime/src/lib.rs:319`.]

No inspected source requires a ledger fork merely because the leaf is hand-written or the decider uses `k=20`. The required fixes are presently in the Sherlock relation, contract integration, and qualification evidence.

# Not checked

- No files were changed. No builds, tests, key generation, proving, commits, or network submissions were performed.
- The other review members’ folders were not opened. No local model was used.
- No fresh live-node version, installed-key, contract-state, parameter, fee, or acceptance observation was made.
- The exact deployed node build and its dependency lockfile were not available for inspection.
- The ledger tag resolves proofs 0.8.2. Its complete implementation was not independently checked; attempts to open its docs.rs source failed. Detailed KZG analysis above uses the locally installed 0.8.3 source.
- The final chip has modeled size evidence, not an end-to-end production-SRS proof in the inspected experiment.
- The revised value encoding, relocation theorem, resource-policy binding, and GP-2/GP-3/GP-4 integration are proposals. Their circuit soundness, completeness, costs, and negative cases remain unverified.
- The manifest-named A11 trace and `/home/hoskinson/.cache/a11/out2/run.json` were absent. The existing VK-size conclusion relies on the committed manifest and serialization code, not a fresh binary measurement.
- No exact raw VK size, proof size, transaction size, or fee was established for the proposed outcome-only circuit.