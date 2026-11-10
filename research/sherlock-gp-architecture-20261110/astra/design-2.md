**Round 2: architecture assessment**

Keep the binary node with inline work as the first candidate. Named lifts remain a useful optimization candidate, but Fable’s proposal does not yet establish GP-1 through GP-4. Its largest gaps concern adapter obligations, registry membership, capacity limits and shape semantics. The available row measurements do not establish the proposed node or decider costs. [A1:84, F1:76, F1:267, F1:295, GP:9, GP:35, GP:46]

No files were written. No builds, tests, proving, Cargo, Lake or local models were run.

Citation prefixes used throughout:

| Prefix | Source |
|---|---|
| `A1` | `research/sherlock-gp-architecture-20261110/astra/design-1.md` |
| `F1` | `research/sherlock-gp-architecture-20261110/fable/design-1.md` |
| `GP` | `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md` |
| `MC` | `/home/hoskinson/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.3.0/src/` |
| `ZS` | `/home/hoskinson/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.4.0/src/` |
| `MP` | `tools/profile/vendor/midnight-proofs-0.8.3/src/` |
| `L` | `repos/midnightntwrk/midnight-ledger`, read with `git show ledger-9.1.0.1:<path>` |
| `Lean` | `clean/fork/Clean/Sherlock/Aggregation.lean` |
| `Quint` | `models/sherlock/quint/aggregation.qnt` |
| `C6` | `crates/mori-core/src/sherlock/helper_machine/` |

Repository HEAD was `3d0b622242d8ec00b9cc1aedbd612ab061f3ed58`. The ledger tag resolves to `a2b30128ca4b660b73a068e29b56b48ede8c9a74`. Numerical projections below are marked **ESTIMATE**. Proposed constructions and logical counterexamples are distinguished from implementation results.

**Task A. Findings on Fable’s proposal**

**A.1. Named lifts can share verification logic, subject to an exact class match**

**Claim:** N, R and F can use one polymorphic verifier slot because they share a constraint system and k. [F1:160]

**Verdict: CONFIRMED, conditionally.**

`AssignedVk` keeps the domain and constraint system outside the circuit. Its only assigned key field is `transcript_repr`. `MidnightVK::read` reconstructs its constraint system from the architecture and k. This supports a common verifier for different keys generated through the same stdlib configuration. [MC:verifier/mod.rs:55, MC:verifier/verifier_gadget.rs:207, ZS:lib.rs:1573]

The condition is stronger than “all three circuits fit k=20.” Their actual architecture flags, range-column configuration, processed constraint system and supported transcript behavior must match. N, R and F also need a compatible child-statement layout, or a constrained normalization mechanism. [MC:verifier/verifier_gadget.rs:218, MC:verifier/verifier_gadget.rs:284, ZS:lib.rs:1584]

**Consequence:** Named lifts are technically plausible. Declare and compare the complete core class before estimating their costs.

**A.2. Private accumulator fields do not make a fork necessary**

**Claim:** Selecting the fixed-base name set requires a small gadget change because `fixed_base_scalars` is private. [F1:153]

**Verdict: WRONG as a necessity claim.**

The public API provides:

- An assigned accumulator’s complete field encoding through `VerifierGadget::as_public_input`. [MC:verifier/verifier_gadget.rs:112]
- Assignment of an accumulator with explicit variable-term counts and fixed-base names. [MC:verifier/accumulator.rs:280]
- Assignment of a collapsed accumulator with caller-selected fixed-base names. [MC:verifier/verifier_gadget.rs:160]
- Off-circuit MSM constructors and accessors for constructing the corresponding witness. [MC:verifier/msm.rs:76]

A proposed implementation using these public APIs is:

1. Prepare the slot under a generic key name and the selected `transcript_repr`.
2. Obtain the assigned accumulator encoding.
3. Assign a replacement accumulator with the union of the N, R and F names.
4. Constrain its variable bases and scalars to the original encoding.
5. Constrain each key-specific coefficient to the selected coefficient:
   \[
   s_{N,j}=b_Ns_j,\quad s_{R,j}=b_Rs_j,\quad s_{F,j}=b_Fs_j.
   \]
6. Keep the shared `-G` term correctly represented.
7. Accumulate the replacement only after those constraints.

The encoding contains bases, variable scalars and fixed-base scalar values. Fixed-base values follow `BTreeMap` order. Names themselves are not encoded. Therefore the implementation must use an explicit, fixed coordinate map. [MC:verifier/msm.rs:229, MC:verifier/msm.rs:269, MC:verifier/msm.rs:367, MC:verifier/mod.rs:145]

There are two implementation choices. Reassigning an already collapsed accumulator uses the convenient public helper, but requires a prior collapse. Reassigning the known uncollapsed shape can avoid that collapse, but may lose scalar-bound optimizations and adds assignment constraints. Neither cost is measured here. [MC:verifier/accumulator.rs:280, MC:verifier/verifier_gadget.rs:160, MC:verifier/msm.rs:451]

**Consequence:** A fork is optional engineering, not an established requirement. Compare the public-API implementation with a direct renaming helper before choosing either.

**A.3. Key switching is prevented by transcript binding plus constrained base routing**

**Claim:** One-hot selection of `transcript_repr` and deferred scalar names soundly distinguishes N, R and F. [F1:166]

**Verdict: INCOMPLETE, but the proposed mechanism has a sound algebraic interpretation.**

The proof transcript absorbs `transcript_repr` before its challenges. The representative hashes the key commitments, domain and pinned constraint-system description. Consequently, passing R’s proof bytes through a verifier using N’s representative changes the challenges. [MC:verifier/verifier_gadget.rs:295, MP:plonk/mod.rs:246]

The accumulator challenge is different. It hashes accumulator encodings, without explicitly hashing the key name or representative. Since names are absent from those encodings, arbitrary relabeling is not automatically authenticated by accumulation. [MC:verifier/accumulator.rs:387, MC:verifier/msm.rs:229]

The construction therefore needs all of these constraints:

- The selector values are Boolean, and their sum equals the slot-enabled bit.
- The same selectors choose the transcript representative, coefficient destinations and permitted statement type.
- All representations of the expanded accumulator use one fixed ordering.
- Children carry the same core-key representatives, and D pins them to actual keys.
- A child’s inherited accumulator retains its existing names. Only its newly prepared proof accumulator is routed according to the selected child key.
- Disabled proof and carry encodings are canonical before challenge derivation.
- Every obligation uses the pairing parameters expected by the eventual decider.

These conditions follow from the proposed selection mechanism, the name-free encoding and the existing genesis-normalization constraint. [F1:102, MC:verifier/msm.rs:229, MC:verifier/accumulator.rs:387, crates/mori-recursion/src/ivc.rs:538, MP:poly/kzg/params.rs:235]

**Consequence:** An R proof cannot simply be relabeled N in a correctly constrained implementation. However, the library does not enforce the complete named-lift protocol. A security argument and adversarial proof tests remain necessary.

**A.4. The N, R and F row intervals are hypotheses, not supported bounds**

**Claim:** N costs 740–800k rows, R costs 650–800k and F costs 550–700k. [F1:153, F1:181]

**Verdict: UNSUPPORTED as planning bounds.**

| Circuit | Evidence | Assessment |
|---|---|---|
| N | The two-self probe reports 730,243 rows. It uses one shared self-key name set and one final collapse. [qa/sherlock/decisions/speed-levers.json:26, crates/mori-recursion/tests/tree_node_rows.rs:8] | **ESTIMATE 740–800k is plausible only as a hypothesis.** It excludes demonstrated costs for three name sets, final statement width, correct disabled-slot normalization, type rules and any inline work. |
| R | The self-plus-leaf constructions report 652,437 and 741,708 rows. [qa/sherlock/decisions/speed-levers.json:22, qa/sherlock/manifests/node.json:22] | These are poor direct proxies. R has no recursive carry and can combine two leaf obligations before resolving their shared key bases once. That implementation pattern already exists. [crates/mori-recursion/src/inner.rs:273] |
| F | F must prepare a proof, authenticate its key, resolve witnessed bases and preserve any carried inner obligation. [F1:262, MC:verifier/verifier_gadget.rs:683] | **ESTIMATE 550–700k remains unverified.** The proposed statement does not yet specify all obligations that must be included. |

The two-self probe assigns a carried accumulator and scales slots by a bit. It does not implement the completed canonical carry and disabled-slot construction. The current IVC explicitly performs an extra collapse to normalize genesis before hashing the fold. [crates/mori-recursion/tests/tree_node_rows.rs:178, crates/mori-recursion/src/ivc.rs:538]

Fable’s argument that R is necessarily larger because both leaf keys are resolved is also inconclusive. Shared-base resolution and removal of recursive-carry work act in the opposite direction. [F1:181, crates/mori-recursion/src/inner.rs:273]

**Consequence:** Measure exact relations. Do not choose named lifts using these intervals.

**A.5. Three named base sets probably require a separate resolution stage**

**Claim:** D might resolve three named sets within its remaining 119,410 rows. Otherwise, add a second stage. [F1:341]

**Verdict: INCOMPLETE. The second stage should be a first-class candidate.**

The recorded decider uses 929,166 rows. Subtracting that from \(2^{20}\) gives 119,410 rows of raw arithmetic headroom. This is not a measured budget for the new statement or architecture. [qa/sherlock/manifests/run-decider.json:20]

The current decider resolves one node-key base set, collapses the combined obligation and checks the pairing. Resolving additional names moves their coefficients into the MSM. [crates/mori-recursion/src/run_decider.rs:217, MC:verifier/msm.rs:491]

Using Fable’s own **ESTIMATE** of 30–45 bases per key and 3–4k rows per additional base gives **ESTIMATE 180–360k additional rows** for two extra sets. This exceeds 119,410 before other changes. It is only a sensitivity calculation: shared points, constant-base optimizations and MSM batching could change the result. [F1:154, F1:343, MC:verifier/msm.rs:451]

A proposed two-stage construction is:

- **Resolution proof:** Verify N, authenticate its carry, pin N/R/F, resolve all three sets and publish a fully collapsed accumulator with the root statement.
- **Pairing proof:** Verify the resolution proof, authenticate that published accumulator, combine both obligations and check the final pairing.

The second stage must verify both the resolution proof’s own obligation and its carried obligation. The existing child-verification helper illustrates that distinction. [crates/mori-recursion/src/inner.rs:182, crates/mori-recursion/src/run_decider.rs:243]

**Consequence:** Measure one-stage and two-stage deciders together. “One additional 70-second proof” is an **ESTIMATE**, not a consequence of the old timing.

**A.6. Witnessed-key binding can be sound without recomputing Blake2b inside F**

**Claim:** `vk_id = H(class_id, transcript_repr, comm_d)` binds a witnessed key even though F does not recompute its representative. [F1:249]

**Verdict: CONFIRMED for an authenticated, correctly normalized registry entry. INCOMPLETE for the proposed admission interface.**

Suppose admission has authenticated the exact tuple containing the class, representative and commitment digest. F then constrains its representative and witnessed points to that tuple. Replacing either component changes `vk_id`, absent a hash collision. Recomputing the representative inside every F proof is unnecessary under that admission policy. [F1:249, F1:262, MP:plonk/mod.rs:246]

However, hashing a tuple does not prove that it is the normalization of a valid serialized Midnight key. `admitKey(vk_id, kind)` alone does not establish this correspondence. [F1:256, F1:339]

The proposed identity also omits the actual public-input count. `MidnightVK` serializes that count separately, while the underlying PLONK representative does not include that wrapper field. A cap cannot substitute for the exact count. [ZS:lib.rs:1539, ZS:lib.rs:1555, MP:plonk/mod.rs:249]

**Consequence:** Authenticate normalization at admission, through checked governance or an admission proof. Bind the exact PI count, format/transcript interpretation and SRS identity. Permissionless submission of arbitrary identity tuples does not establish compatibility with deployed Midnight keys.

**A.7. Admitting an adapter does not automatically admit its inner circuit**

**Claim:** Foreign circuits enter through an authenticated key registry and class adapters. [F1:71, F1:262]

**Verdict: INCOMPLETE, with a concrete registry-bypass path in the stated interface.**

F proves membership of the adapter’s key. The described adapter computes the inner key identity and emits it, but its stated relation does not prove that the inner key belongs to the registry. Its public outputs are only `[vk_id_inner, out_d]`. [F1:262, F1:267]

As written, an admitted generic adapter could verify any compatible inner key, including one absent from the registry. F would authenticate the adapter while accepting its asserted inner identity. This contradicts the required circuit-key membership unless admission of the adapter explicitly authorizes that entire family. [GP:35, F1:267]

**Consequence:** The adapter must prove inner-key membership against the authorized `reg_root`, or F must verify that membership separately. The adapter statement must carry the root and inner identity. A general adapter’s admission cannot silently replace inner-key admission.

**A.8. The adapter interface omits its deferred verification obligation**

**Claim:** An adapter verifies a foreign proof, resolves its witnessed bases and emits `[vk_id_inner, out_d]`. F then verifies the adapter. [F1:267, F1:286]

**Verdict: INCOMPLETE. Unsound if “verifies” means prepare-and-collapse without a pairing.**

`prepare` produces a PCS obligation. The proof becomes valid only when the resulting accumulator satisfies the pairing invariant. Resolving bases and collapsing do not perform that check. [MC:verifier/verifier_gadget.rs:683, MC:verifier/accumulator.rs:333]

Therefore an adapter has two valid choices:

1. Check the foreign proof’s pairing inside the adapter.
2. Publish its collapsed foreign accumulator, and require F to authenticate and accumulate it along with the adapter proof’s own obligation.

The existing helper explicitly distinguishes these own and carried accumulators. [crates/mori-recursion/src/inner.rs:182]

The stated two-field adapter output describes neither the carried accumulator nor an in-circuit pairing. [F1:267]

**Consequence:** Repair this interface before measuring F. If the adapter performs a full pairing, its row budget changes substantially. If it carries an obligation, F must consume that obligation, and the registry must authenticate the corresponding adapter semantics.

**A.9. Class adapters cover a useful subset, not arbitrary PLONK/halo2**

**Claim:** Adapters indexed by `(arch, k)` cover the foreign-proof requirement. [F1:133, F1:267]

**Verdict: INCOMPLETE.**

For stdlib-produced Midnight keys, architecture and k reconstruct the constraint system. Other PLONK/halo2 circuits need not use that configuration or the same transcript and opening protocol. [ZS:lib.rs:1584, GP:33]

The gadget also asserts a single challenge phase. Its comment about “exactly one instance column” is misleading as a description of the pinned stdlib: that stdlib configures two columns, one committed and one ordinary. The verifier checks their combined count and evaluates queries by column. [MC:verifier/verifier_gadget.rs:308, ZS:lib.rs:547, MC:verifier/verifier_gadget.rs:284, MC:verifier/verifier_gadget.rs:432]

There is another class issue. F expects the BLS-plus-Poseidon node class. An adapter that enables the stdlib Blake2b chip changes its architecture. It cannot then enter that unchanged F merely because both fit k=20. [F1:76, F1:267, ZS:external/blake2b.rs:66, ZS:lib.rs:1584]

**Consequence:** Keep class adapters as fast paths. Provide a general verifier-program path, or prove that a fixed universal adapter covers every admitted verifier description. For Blake2b, choose explicitly between a larger common architecture and an implementation using the existing core gates.

**A.10. The transcript mismatch is real, and Poseidon reproving does not close it**

**Claim:** Ledger proofs use Blake2b, while the recursive gadget uses Poseidon. Reproving the same circuit under Poseidon is a first stage. [F1:28, F1:276]

**Verdict: CONFIRMED for the mismatch. INCOMPLETE as GP-3 completion.**

The ledger’s transcript hash is `blake2b_simd::State`. Its verifier invokes stdlib verification with that hash. The BLS recursive emulation uses `PoseidonState`. [L:transient-crypto/src/proofs.rs:64, L:transient-crypto/src/proofs.rs:563, MC:verifier/types.rs:140]

The underlying VK construction is independent of the chosen proof transcript. Reproving under Poseidon can establish satisfaction of the same relation and instance. It does not verify supplied Blake2b proof bytes, and requires access to a witness. [MP:plonk/mod.rs:246, ZS:lib.rs:2009, F1:35]

The available Blake2b wrapper is not a ready transcript adapter. Its public digest methods are unkeyed. The native transcript initializes a keyed 64-byte hash and uses common-message and challenge prefixes. [ZS:external/blake2b.rs:101, MP:transcript/implementors.rs:13]

**Consequence:** Poseidon reproving is an intermediate integration test. GP-3’s foreign-proof path must include actual Compact proof bytes under their native transcript.

**A.11. Native Compact compatibility requires more than a transcript substitution**

**Claim:** A Compact proof is essentially a stdlib PLONK proof with a Blake2b transcript and an opaque public-input vector. [F1:276]

**Verdict: INCOMPLETE.**

| Boundary | Pinned behavior | Required consequence |
|---|---|---|
| Proof version | Envelope tag `proof[v5]`. [L:transient-crypto/src/proofs.rs:131] | Select and authenticate the supported format. |
| Key version | Envelope tag `verifier-key[v7]`. [L:transient-crypto/src/proofs.rs:370] | Bind the format and canonical key interpretation. |
| Key contents | Architecture, k, exact PI count and PLONK VK. [ZS:lib.rs:1539] | Include every field needed to select verification semantics. |
| Binding input | PI vector begins with the binding input. Ledger construction binds call address, entry point, gas/effects and parent binding commitment. [L:ledger/src/verify.rs:2102, L:ledger/src/verify.rs:2151] | Do not describe arbitrary PI satisfaction as authentication of a particular call without checking this binding. |
| Communications commitment | ZKIR optionally places it after the binding input and checks a Poseidon commitment over randomness plus encoded inputs and outputs. [L:zkir-v3/src/ir_vm.rs:801, L:zkir-v3/src/ir_vm.rs:1173] | Typed input/output claims require an opening or separately exposed values. |
| Operation layout | Guarded-off `Impact` entries become zero fields. [L:zkir-v3/src/ir_vm.rs:869] | Preserve positions and guards. |
| `pi_skips` | Proving returns skip metadata separately. Guarded-off impacts record their width while retaining zero PI positions. [L:zkir-v3/src/ir.rs:88, L:zkir-v3/src/ir_vm.rs:507] | Do not delete skipped positions or treat the metadata as an independently authenticated semantic statement. |
| Committed instance | Ledger passes `None`, which stdlib replaces with the identity commitment. [L:transient-crypto/src/proofs.rs:572, ZS:lib.rs:2020] | Match the committed-instance convention. |
| Complete parsing | Native verification rejects trailing proof bytes. Ledger initialization rejects trailing key bytes. [ZS:utils/plonk_api.rs:154, L:transient-crypto/src/proofs.rs:532] | Enforce these when claiming byte-level native-verifier equivalence. |
| Point handling | The recursive transcript does not constrain subgroup membership and cannot prove complete byte consumption. [MC:verifier/transcript_gadget.rs:76, MC:verifier/transcript_gadget.rs:123] | Supply an explicit security argument or the missing checks. Host parsing is not an in-circuit constraint. |

**Consequence:** Define separate claims for `CircuitSatisfied(vk_id, public_inputs_d)` and a typed contract-call claim. Neither alone proves transaction inclusion or accepted ledger effects. [L:ledger/src/verify.rs:2102, A1:317]

**A.12. Zero-padding shorter public statements is incompatible with unchanged proofs**

**Claim:** Fix a PI cap and zero-pad shorter statements. [F1:270]

**Verdict: WRONG if the padded vector is passed to the existing verifier unchanged.**

Both native and recursive proof transcripts absorb each ordinary instance vector’s length. Stdlib also requires that length to equal the key’s exact PI count. Appending zeros changes the transcript even when it leaves the instance polynomial unchanged. [MP:plonk/verifier.rs:69, MC:verifier/verifier_gadget.rs:302, ZS:lib.rs:2020]

**Consequence:** Authenticate the actual length. Padding may exist in storage, but must not alter transcript absorption. Supporting variable lengths requires additional constrained transcript logic or specialization by exact length.

**A.13. Fixed-depth memory trees narrow GP-1**

**Claim:** Depth-20 program memory and depth-32 arena memory make capacity independent of the key. [F1:111, F1:208]

**Verdict: WRONG under the literal requirement.**

A fixed binary-tree depth fixes its addressable capacity. Calling that limit an “address width” does not remove it from the circuit relation. GP-1 explicitly rejects capacity bound to the key. [F1:114, GP:13]

**Consequence:** Use expandable authenticated storage with path processing across bounded chunks, or establish a language-level address bound that already covers the full required language. The proposal establishes neither for depths 20 and 32. [A1:199, GP:9]

**A.14. Eighteen rows is a round count, not the complete hash cost**

**Claim:** Poseidon costs 18 rows per permutation, giving 360-row program reads, 126-row frame operations and 576-row arena reads. [F1:54, F1:206]

**Verdict: WRONG as a complete cost calculation.**

The round-transition count is \(8+60/6=18\). However, the implementation also adds initial round constants, assigns the final output row and performs sponge absorption additions. Its initialization sets the input-length domain value. [MC:hash/poseidon/constants/mod.rs:17, MC:hash/poseidon/poseidon_chip.rs:430, MC:field/native/native_chip.rs:523, MC:hash/poseidon/poseidon_chip.rs:475, MC:hash/poseidon/poseidon_chip.rs:536]

A Merkle access also needs leaf encoding, address bits, left/right ordering, root equality and any write constraints. These are absent from the table. A hash of three absorbed fields, such as an explicit tag plus two children, takes two rate-two absorption chunks. [MC:hash/poseidon/mod.rs:142, MC:hash/poseidon/poseidon_chip.rs:536]

Thirteen frame lanes plus the previous stack digest occupy fourteen fields, or seven chunks. Adding an absorbed domain tag makes fifteen fields and eight chunks. The proposal does not specify which domain-separation convention its cost assumes. [F1:209, C6:c6_phys.rs:50, MC:hash/poseidon/constants/mod.rs:20]

**Consequence:** Retain the formulas `read ≈ d × compression_cost` and `write ≈ 2d × compression_cost`, with explicit encoding overhead. Withdraw the listed totals until the actual gadgets are measured.

**A.15. Sherlock fits authenticated storage, but the proposed state inventory is incomplete**

**Claim:** Hash-chained frames, an append-only arena and a program table cover the machine. Helper cursors and active value slots are free registers. [F1:202]

**Verdict: INCOMPLETE.**

The C6 layout supports the broad direction:

- Frames have thirteen lanes and support push, top replacement and pop. A hash stack can implement these operations. [C6:c6_phys.rs:50, C6:shared/owner_runtime.rs:3579]
- Data, data-field references, environment cells and value slots occupy separate tables. [C6:c6_phys_data.rs:11]
- The runtime appends to all four structures. [C6:shared/owner_runtime.rs:3506]
- Helper state includes a 23-lane record, a result record and a separate task stack with seven-lane entries. Tasks also support top replacement. [C6:c6_phys_helper.rs:10, C6:shared/owner_runtime.rs:3566]

A fixed register window can cache active entries. It does not eliminate authentication of the tables or task stack when their size exceeds that window. [GP:13, C6:c6_phys_helper.rs:25]

The proposed structured result identity also needs revision. An ambient arena root and handle identify a stored object, but do not establish a canonical value encoding independent of unrelated allocations or handle placement. GP-1 requires a digest of a canonical value encoding. [F1:229, GP:12, C6:c6_phys_data.rs:13]

**Consequence:** Specify commitments and invariants for every table, both stacks, allocation counters and cached entries. Prove a refinement from this representation to Sherlock semantics. Canonicalize structured results separately from storage identity.

**A.16. Challenge-based memory arguments are not incompatible with arbitrary PCD shapes**

**Claim:** Arbitrary shape and unbounded length leave no point at which the complete access log is committed, so permutation/LogUp memory arguments cannot compose. [F1:214]

**Verdict: WRONG as a general impossibility claim.**

The actual restriction is that records must be bound before the challenges that test them. A finite run can commit its access log and then prove its memory argument. Completed run claims can subsequently compose in a DAG. Alternatively, bounded segments can combine local memory arguments with authenticated boundary state. This is a construction option, not a measured Sherlock implementation. [GP:22, GP:44, A1:216]

Offline memory arguments explicitly operate on committed operation vectors and prove their consistency or reordering. The challenge schedule must be justified for the chosen construction. [Twist and Shout, Appendix A](https://eprint.iacr.org/2025/105.pdf)

Fable correctly identifies the danger of allowing future, uncommitted records to cancel earlier errors. That concern does not rule out sealing, epochs or authenticated segment boundaries. [F1:214, A1:216]

The RISC Zero citation does support Merkle page-in/page-out at continuation boundaries. It does not establish an impossibility result for other memory schemes. [RISC Zero optimization guide](https://dev.risczero.com/api/zkvm/optimization)

**Consequence:** Start with Merkle storage for simpler local reasoning. Keep challenge-based memory as a throughput candidate.

**A.17. The claim root does not bind every aggregation DAG**

**Claim:** `leaf`, `join` and `dep` hashes ensure that two different DAGs have different roots. SEG shape is intentionally omitted. [F1:295]

**Verdict: WRONG as stated.**

Three logical counterexamples follow from the proposed definition:

1. Different SEG bracketings seal to the same run claim because SEG has no shape commitment.
2. Unary wrappers have no specified hash constructor and can preserve the same root.
3. A shared subgraph and two separately represented but identical copies have the same recursively computed hashes.

These cases require no hash collision. They follow from which information the proposed hash omits. [F1:293, F1:295, Lean:451, Lean:708]

Content addressing can intentionally identify identical subgraphs. That is a defensible graph model, but it must be declared. Collision resistance then binds the canonical content graph, not every possible occurrence graph or proof-generation history. [F1:295, A1:372]

**Consequence:** Separate `claim_set_d` from `shape_d`. Bind typed edges, slot enablement, local work and child certificate identities at every recursive node. Preserve schedule-independent semantic claims without erasing the required shape statement. [GP:46, GP:47, A1:372]

**A.18. Linear run coverage and reusable claims are compatible**

**Claim:** Run steps are linear, while proven claims can feed multiple consumers. [F1:309]

**Verdict: CONFIRMED, with a narrower scope than the accompanying contract claim.**

The coverage relation requires positive leaf lengths and adjacent heights. This prevents duplication or gaps within a linked run interval. It does not prohibit verifying the same completed claim in two different consumers. [Lean:784, Lean:789, GP:50]

The existing Quint model’s no-duplicate-certificate condition must change for that fan-out. It currently forbids repeated certificate references in the accumulator manifest. [Quint:464]

**Consequence:** Retain linearity for owner-transition intervals. Permit reuse of authenticated facts. Do not infer unique settlement effects from either property.

**A.19. Root uniqueness does not prevent semantic replay**

**Claim:** A contract set of accepted roots makes on-chain acceptance linear. [F1:313, F1:329]

**Verdict: WRONG if this is intended to prevent replay of a run or effect.**

For example, the roots for `JOIN(A,B)` and `JOIN(B,A)` differ under the proposed ordered hash, but both contain A. Rebracketing or joining A with another claim likewise produces a new root. A set of accepted roots rejects only the identical root. [F1:295, F1:329]

`n_claims` also needs a defined meaning. Summing child counts counts repeated occurrences in a diamond. It does not count distinct claims. [F1:100, F1:302]

**Consequence:** Keep root deduplication if useful. Protect settlement with semantic claim identifiers or effect nullifiers. Define whether batching commits to a list, multiset or canonical set, and prove the advertised membership and count semantics. [GP:25, A1:407, A1:466]

**A.20. Seal rules need authenticated terminal openings and dependency discharge**

**Claim:** Sealing proves canonical genesis, a terminal outcome and any dependencies. [F1:81, F1:316]

**Verdict: INCOMPLETE.**

The proposed `h_lo == 0` check is the correct structural requirement. If every zero-height leaf proves canonical initialization, and R/N preserve positive contiguous coverage, a segment beginning later cannot be sealed as a complete run. This is consistent with the existing conditional genesis theorem. [F1:94, F1:318, Lean:1106, Lean:1116]

The relation still needs to specify:

- An opening of `post_d` that binds the outcome, result and dependency commitment.
- Equality of `input_d` and the relevant execution parameters across SEG links.
- Authentication of each consumed dependency and its mapping to a run input.
- An empty-dependency condition for plain seal, or proof that dependencies were discharged earlier.
- No publication from an incomplete memory, initialization or verifier continuation.

A plain seal that merely checks terminal status could otherwise bypass the dependency-checking rule. These obligations follow from the proposed opaque SEG state and the required in-circuit consumption boundary. [F1:95, F1:202, GP:24]

The existing `result_unique` theorem does not establish the extended claim automatically. It assumes a deterministic machine, terminal halted states, a committed program and a common authenticated initial state. It must be extended for the new outcomes and dependency semantics. [Lean:1133]

**Consequence:** Specify seal as a checked relation over state openings, not merely a tag conversion.

**A.21. Registry-root equality is present, but must extend through adapters**

**Claim:** Children carry the same `reg_root`, and the contract binds the current root. [F1:102, F1:330]

**Verdict: CONFIRMED for N’s stated rule. INCOMPLETE across the full pipeline.**

The proposal explicitly requires root equality between N and its children. Mixing roots at that boundary is therefore prohibited as written. [F1:102]

The adapter interface does not carry a registry root, and it omits inner membership. That leaves the earlier registry gap. Also, enforcing only the current root makes previously produced certificates unusable after a root change unless they are re-established under the new policy. [F1:267, F1:332]

**Consequence:** Carry the policy identity through adapters and lifts. Choose current-root or authenticated historical-root acceptance explicitly. Historical roots require an explicit revocation policy.

**A.22. Unary nodes and dependency cycles need a precise certificate model**

**Claim:** Unary nodes and proof reuse need no additional mechanism. [F1:292, F1:302]

**Verdict: INCOMPLETE.**

A unary identity node is semantically valid, but its shape must still be bound if GP-4 distinguishes that node from its child. The existing unary theorem proves statement equality, not shape equality. [Lean:451, GP:47]

The Lean certificate datatype is inductive and therefore describes finite trees. Reusing a previously established certificate can construct a finite DAG. Neither fact proves the cryptographic construction’s security for arbitrary recursive dependency graphs. Its accumulator law remains an explicit premise. [Lean:72, Lean:1230]

I found no concrete dependency-cycle forgery in the proposed rules. The missing result is a well-founded extraction argument covering the new claim types, adapters and dependencies. A global fixed-width depth counter would introduce another bound rather than prove unbounded-depth composition. [GP:44, A1:432, Security Bounds for PCD](https://eprint.iacr.org/2023/1646)

Two additional completeness cases are unspecified:

- R always consumes two U proofs. It needs a one-active-leaf mode for singleton or odd packaging, without inventing an executed identity leaf. [F1:77, Lean:85]
- `openClaim` uses an ordinary Compact circuit with a path input. Arbitrary-depth claim trees need a bounded-depth index or a recursively proved opening path. [F1:334, GP:44]

**Consequence:** Define base, unary and binary constructors, allowed claim types, opening semantics and a well-founded certificate model before treating the depth-eight tests as sufficient.

**A.23. The “two prior proofs plus new work” impossibility claim is false**

**Claim:** GP-4 cannot be met in one k=20 node because three verifier slots exceed the domain. [F1:127]

**Verdict: WRONG. The proposed two-step replacement narrows the literal relation.**

The measurement establishes that the tested **three-proof-slot construction** requires k=21. It does not establish that two predecessor verifications plus inline computation require k=21. The two-self probe has a different measured row count. [qa/sherlock/decisions/speed-levers.json:26]

A node with two prior-proof slots and bounded interpreter work directly satisfies the literal arity-plus-work form. Whether it fits k=20 remains unmeasured. A k=21 version is also compatible with GP-4, which does not specify k=20. [A1:84, GP:46]

**Consequence:** Do not ask the owner to reinterpret “plus new work.” Test inline work. Use k=21 if necessary and affordable, or change the inner proof system.

**A.24. Per-class adapter keys are not automatically a prohibited narrowing**

**Claim:** GP-3 cannot be met without class-specific adapter keys, and whether those violate “one key set” requires clarification. [F1:133]

**Verdict: INCOMPLETE.**

GP-3 explicitly admits foreign circuits by their keys. It cannot mean that all foreign circuits share one VK. The fixed-key obligations concern universal Sherlock execution and recursive shapes. [GP:9, GP:35, GP:46]

Thus, registering class adapters as data is not inherently a violation if the core recursion and contract keys remain unchanged. However, a mandatory family of monolithic adapters does not establish coverage of every required architecture or arbitrarily large verifier computation. [F1:106, F1:267, GP:33]

A literal fixed-core-key design uses a bounded verifier interpreter, authenticated verifier descriptions and continuations. Class adapters can accelerate supported cases while the interpreter provides the general path. This is a proposed construction, not an implemented feasibility result. [A1:250, A1:292]

**Consequence:** Separate “additional registry entries” from “replacement core keys.” The former is expected. The latter cannot depend on new programs or graph shapes.

**A.25. The reasons given for excluding alternative inner systems are too strong**

**Claim:** Folding inevitably requires a KZG opening over the folded witness, and no requirement needs a different proof system. [F1:145, F1:170]

**Verdict: WRONG for the inevitability claim. UNSUPPORTED for the feasibility conclusion.**

Nova’s implementation supports Pedersen commitments with IPA evaluation as well as pairing-based alternatives. A Midnight wrapper must prove the chosen inner verification relation, but that relation need not be a KZG opening of the folded witness. [Nova implementation](https://github.com/microsoft/Nova)

The cited CycleFold paper supports the small scalar-multiplication counts, but does not establish a complete Midnight row count. Mira supports specialized folding for pairing-based arguments, but does not validate this proposed Midnight adapter. [CycleFold](https://eprint.iacr.org/2023/1192), [Mira](https://eprint.iacr.org/2024/2025)

HyperNova and ProtoGalaxy explicitly address folding multiple instances. Therefore folding should not be excluded from the node architecture merely because one sequential folding interface is insufficient. [HyperNova](https://eprint.iacr.org/2023/573), [ProtoGalaxy](https://eprint.iacr.org/2023/1106)

The accumulation and PCD papers support the architectural pattern, not the complete security theorem for this implementation’s named-key routing and transcript. [Proof-Carrying Data from Accumulation Schemes](https://eprint.iacr.org/2020/499), [Security Bounds for PCD](https://eprint.iacr.org/2023/1646)

**Consequence:** Keep the existing KZG path first because it has local implementation evidence. Retain alternative inner systems as architectural fallbacks, not only leaf accelerators.

**A.26. Proof-count and parallel-speed claims omit dependencies and finalization**

**Claim:** Named lifts need approximately the same number of node-class proofs, workers divide wall time, and proof size and decider time remain unchanged. [F1:187]

**Verdict: INCOMPLETE. “Decider time unchanged” is unsupported.**

The stated count assumes paired leaves and omits at least sealing and exceptional packaging. Merge proofs depend on child completion, so all proofs are not independent. The repository already records a concurrency slowdown rather than ideal worker scaling. [F1:77, F1:81, qa/sherlock/decisions/speed-levers.json:68]

A fixed final relation can have proof size independent of graph shape. It does not follow that the new final relation has the old proof length or runtime. The proposal itself contemplates an additional resolution proof. [F1:343, GP:48]

**Consequence:** Compare complete proof graphs, including lifts, adapters, sealing, resolution and settlement. Under this study’s machine constraint, run heavy jobs sequentially.

The cited `midnight-aggregation` documentation was inaccessible through the available web retrieval. I therefore retain no claim that version 0.1.0 supplies witnessed-key slots or resolves these gaps. Fable also identifies that source as uninspected. [F1:404]

**Task B. Disagreements and revised positions**

The table covers the substantive architectural differences. “Now” states the position retained after this review.

| Issue | My round-1 position | Fable’s position | Position now, evidence and deciding measurement |
|---|---|---|---|
| Accumulation primitive | Existing KZG accumulation first, with split accumulation or another inner system available. [A1:66] | Existing KZG accumulation with named lifts. [F1:150] | **Agreement on the initial primitive.** The difference is placement of verification and work. Compare complete relations, not the phrase “deferred pairing.” The current node already defers pairing. [crates/mori-recursion/src/ivc.rs:562] |
| Core key set | N plus terminal D. [A1:9] | U, R, F, N and D. [F1:106] | Prefer the smaller recursive core, with optional registered fast paths. Named lifts remain viable if they reduce total cost. Measure key closure, recursive rows and total proof count. |
| New work | Execute bounded work inside N. [A1:84] | Introduce work through R/F proofs, consuming a slot. [F1:127] | Retain inline work because it satisfies GP-4 literally. Measure useful work remaining after two child verifications. [GP:46] |
| Arity and k | Binary at k=20 if measured feasible. [A1:70] | Named binary at k=20, ternary at k=21 as fallback. [F1:153] | Test binary inline work first, then k=21. Three proof slots are not required for new work. [qa/sherlock/decisions/speed-levers.json:26] |
| Deferred key bases | Carry only the self-key set, resolve foreign keys locally. [A1:110] | Carry three named core sets. [F1:166] | Accept a fixed finite name union as a legitimate alternative. It does not grow with registry population. Measure added PI work and decider MSM cost. [MC:verifier/accumulator.rs:280] |
| Library changes | No named-routing mechanism proposed. [A1:84] | Patch private fixed-base scalar handling. [F1:153] | Prefer a public-API prototype first. Measure reassignment overhead against a direct helper. [MC:verifier/verifier_gadget.rs:112, MC:verifier/accumulator.rs:280] |
| Decider | One terminal stage, with uncertain fit. [A1:23] | One stage, then two if three-name resolution exceeds headroom. [F1:343] | Measure both immediately. Named lifts increase the likelihood that resolution needs its own proof. [crates/mori-recursion/src/run_decider.rs:243] |
| Sherlock execution architecture | Bounded processor and complete interpreter, with continuations. [A1:163] | Fixed S-step universal leaf retaining fragment logic. [F1:72, F1:352] | Retain continuations as the completeness mechanism. Reuse fragments where their semantics refine the processor. Measure the largest rules and all unfinished-operation boundaries. |
| Storage capacity | Expandable trees and streamed paths. [A1:199] | Fixed depths 20 and 32. [F1:208] | Retain expandable storage. Fixed depths do not establish GP-1. Test identical keys across tree growth. [GP:13] |
| Frame storage | General authenticated storage initially. [A1:187] | Hash-chained frames. [F1:209] | Adopt the hash stack for frames and helper tasks where appropriate. Top rewrite is supported by the machine. Measure push, pop, rewrite and cache boundaries. [C6:shared/owner_runtime.rs:3566] |
| Hash-cost prior | **ESTIMATE 50–150 rows** per tree compression. [A1:212] | **ESTIMATE 18 rows** per permutation. [F1:206] | Withdraw both as complete access-cost models. Measure the exact tagged compression and address logic. [MC:hash/poseidon/poseidon_chip.rs:430, MC:hash/poseidon/poseidon_chip.rs:536] |
| Offline memory | Keep as a throughput candidate. [A1:209] | Reject challenge-based memory for arbitrary PCD. [F1:214] | Retain it with explicit commitment/challenge boundaries. Compare sealed-run or segment-local arguments against Merkle storage. [Twist and Shout](https://eprint.iacr.org/2025/105.pdf) |
| Initialization | Authenticate initialization, potentially across chunks. [A1:175] | One state hash in the first leaf. [F1:234] | Retain the full obligation, but permit a compact implementation when roots and empty structures suffice. Measure actual initialization and malformed openings. [Lean:204] |
| Run identity | Separate run identity, execution parameters and request context. [A1:126] | Claim identity from program, input and outcome. [F1:97] | Retain separate semantic and settlement identities. Compare identical computations under different requests and replay policies. [GP:24, F1:329] |
| Structured outcomes | Canonical typed value digest. [A1:224] | Arena root and handle for structured values. [F1:229] | Retain canonical value encoding. Test equal values with different allocation histories. [GP:12] |
| Registry identity | Full verifier interpretation and normalization metadata. [A1:250] | Class, representative, commitment digest and kind. [F1:249] | Retain the complete interpretation, encoded compactly if desired. Test exact PI count, transcript, SRS and descriptor mismatches. [ZS:lib.rs:1539, MP:poly/kzg/params.rs:235] |
| Adapter keys | General verifier program under fixed core keys. [A1:292] | New adapter key per class. [F1:267] | Allow per-class adapter keys as registry data. Keep a general path for unsupported classes. New adapter admission alone is not a GP violation. [GP:35] |
| Adapter obligations | Verifier jobs remain incomplete until all obligations close. [A1:428] | Two-field adapter output. [F1:267] | Require either a pairing inside the adapter or an authenticated carried accumulator. Test a valid adapter proof containing an invalid inner obligation. [MC:verifier/verifier_gadget.rs:683] |
| Compact transcript | Actual Blake2b proof is required for compatibility. [A1:298] | Poseidon reproving first, Blake2b later. [F1:276] | Permit staging, but count GP-3 complete only after native proof compatibility. Compare challenges and acceptance with the pinned ledger verifier. [L:transient-crypto/src/proofs.rs:563] |
| Statement adapters | Generic satisfaction and typed application claims. [A1:321] | Hash the PI vector. [F1:268] | Keep both levels. Opaque PI claims are useful, but typed results need checked communication-commitment openings. [L:zkir-v3/src/ir_vm.rs:1173] |
| Batch semantics | Canonical claim map and explicit union. [A1:407] | Hash the aggregation structure, retaining duplicate appearances. [F1:295] | Separate collection semantics from shape. Measure union cost, membership openings and overlap handling. [GP:25] |
| Shape binding | Every node binds children, local work and dependency mappings. [A1:372] | Only the claim layer binds shape. SEG bracketing is erased. [F1:297] | Retain full recursive shape binding while preserving semantic schedule independence. Compare roots for unary wrappers and alternate SEG bracketings. [GP:47] |
| Fan-out | Shared claims are reusable, effects are separately controlled. [A1:393] | Same proof may enter two parents. [F1:302] | Agree on proof reuse. Require explicit duplicate and settlement semantics. Test a diamond with a shared ancestor. [Quint:464] |
| Cycles and depth | Well-founded extraction without a new fixed-width global-depth bound. [A1:432] | Reuse plus recursive claim induction. [F1:321] | Retain the explicit cryptographic and inductive obligations. Formalize all base cases and dependency constructors. [Lean:1230] |
| Registry updates | Current root or authorized root history. [A1:272] | Current root only. [F1:332] | Either is a product policy. Prefer authenticated epochs if existing certificates must survive updates. Test admission, removal and cross-epoch composition. |
| Contract | Dynamic settlement transition with semantic nullifiers. [A1:450] | `acceptRoot`, `openClaim`, `admitKey`, root deduplication. [F1:327] | Combine the useful root/opening interface with explicit settlement nullifiers. Test reordering and rebatching replay. |
| Alternative inner systems | Architectural fallback for the processor and recursion. [A1:74] | Optional replacement for leaf proving only. [F1:145] | Retain whole-inner-layer alternatives. Measure the complete Midnight bridge before selecting one. [HyperNova](https://eprint.iacr.org/2023/573) |
| Milestone order | Binary kernel, memory, complete interpreter, composition, foreign verification. [A1:509] | Finish current chain, then named PCD, dependencies, adapters, memory last. [F1:364] | Move memory and actual Compact compatibility probes early. Existing chain work can continue, but should not freeze a deficient GP interface. [GP:56] |
| Experiments | Binary closure, memory, actual Compact proof, registry, DAG, full GP, alternative inner, GPU. [A1:552] | Named slots, memory, decider, k=21, adapters, Blake2b, hash parity, concurrency. [F1:384] | Prioritize soundness boundaries and complete costs. Defer GPU and concurrent proving until an architecture satisfies the requirements. The final ranking follows below. |

**Withdrawals and corrections to my round-1 report**

1. **I withdraw the blanket wording that changing keys for a foreign circuit is itself a kill criterion.** Foreign VKs are expected registry data. The invariant is unchanged core recursion/decider keys for new programs and shapes. Class-adapter admission can preserve that invariant. [A1:556, GP:35, GP:46]

2. **I withdraw 50–150 rows as the default planning cost of one tree compression.** It was labeled an estimate, but was insufficiently grounded in this chip. The implementation supports a substantially more specific measurement. Fable’s 18-row estimate also omits required work. [A1:212, MC:hash/poseidon/poseidon_chip.rs:430]

3. **I revise the instruction to carry only one self-key name set.** A fixed finite union of names does not create registry-dependent accumulator growth. Its main unresolved issue is terminal cost, not constant-size representability. [A1:110, MC:verifier/accumulator.rs:280]

4. **I correct any reading of the gadget comment as a one-total-instance-column restriction.** The pinned stdlib has one committed and one ordinary instance column. The single-phase restriction is explicit in executable verifier code. [A1:290, ZS:lib.rs:547, MC:verifier/verifier_gadget.rs:308]

5. **I retain my binary-node and decider intervals only as unvalidated ESTIMATES.** They are not evidence that a complete interpreter, arbitrary foreign verifier and final statement fit k=20. The only relevant local binary baseline remains the incomplete 730,243-row probe. [A1:23, crates/mori-recursion/tests/tree_node_rows.rs:1]

**Task C. Final ranked experiments**

These are proposed future experiments. All costs are **ESTIMATE** for 20 CPU cores, 120 GB shared memory and one heavy job at a time. Engineering effort is separate from machine runtime.

First ingest the existing per-rule, leaf-headroom, genesis and leaf-proving results. Do not duplicate those jobs. [research/sherlock-gp-architecture-20261110/BRIEF.md:26]

| Rank | Hypothesis | Measurement | Kill criterion | Cost — ESTIMATE |
|---|---|---|---|---|
| **1. Exact binary node: inline work versus named lifts** | A binary node can authenticate two predecessors and useful new work within the resource envelope. Named routing can use the public API without losing its expected benefit. | Model the final core architecture and PI layout. Compare one-self-key inline work against N/R/F routing. Include canonical disabled slots, carry scalar constraints, type selection and base/unary/binary cases. Prove wrong-key, wrong-name and tampered-carry cases through final verification. | Any accepted key/name mismatch kills that construction. More than 996,147 modeled rows kills the proposed 5% k=20 margin. No useful inline work kills the k=20 inline candidate, not GP-4. Baseline: [crates/mori-recursion/tests/tree_node_rows.rs:56]. | Engineering 3–7 days. Modeling 10–60 minutes, 3–8 GB. Initial proof closure 1–3 hours, 20–45 GB. |
| **2. Complete decider closure** | One-stage resolution fits, or a two-stage design removes the named-base pressure without dropping obligations. | Measure D with the exact N/R/F bases and final statement. Compare a resolution proof plus pairing proof. Record base counts, MSM rows, PI work, VK bytes, proof bytes, time and RSS. Mutate each carried obligation independently. | Any unresolved or unauthenticated obligation kills the design. Exceeding the measured domain limit kills that stage’s chosen k. A two-stage design that still exceeds the machine envelope needs another architecture. Existing baseline: [qa/sherlock/manifests/run-decider.json:20]. | Engineering 2–5 days after rank 1. Modeling 15–60 minutes, 3–8 GB. Proof comparisons 1–4 hours, 20–50 GB. |
| **3. Native Compact proof compatibility** | The foreign verifier can reproduce the pinned ledger’s acceptance of an actual Blake2b proof. | Differentially compare transcript challenges, PI length, identity committed instance, key representative, opening reduction and pairing. Include binding input, communications commitment, guarded impacts, `pi_skips`, malformed encodings and trailing bytes. Model the actual transcript implementation and its architecture effect. | Reproving under Poseidon, trusting a host verification result, padding the absorbed PI vector, or losing an inner accumulator fails this experiment. Any divergence from the declared native compatibility boundary fails. [L:transient-crypto/src/proofs.rs:563, ZS:utils/plonk_api.rs:154] | Engineering 1–3 weeks. Native differential work 15–60 minutes. Row modeling 30–120 minutes. First complete proof 2–12 hours, 20–60 GB, depending on segmentation. |
| **4. Registry and adapter adversarial closure** | Every emitted foreign claim authenticates the intended inner key and verifier interpretation under an authorized root. | Exercise registered and unregistered inner keys through an admitted adapter. Mutate representative/commitment pairs, exact PI counts, class, transcript, SRS, claim kind and registry epoch. Include a foreign architecture not supported by the initial class adapter, using the general path. | Acceptance of an unregistered inner key, mismatched normalization or wrong SRS kills the interface. Rebuilding core keys for the next supported class fails the fixed-core-key design. [GP:35, ZS:lib.rs:1539, MP:poly/kzg/params.rs:235] | Engineering 3–7 days beyond verifier work. Modeling 30–120 minutes. Proof-backed cases 2–8 hours, 20–60 GB. |
| **5. Committed-memory refinement and cost** | A fixed-key processor covers every C6 storage structure, supports growth and preserves Sherlock outcomes at useful throughput. | Count accesses for every rule. Measure tagged compression, reads, append, frame/task top rewrite and continuation boundaries. Include data fields, environment cells, value slots and helper tasks. Demonstrate tree growth with identical keys and canonical results across different allocation histories. | A capacity-dependent key, unsupported language rule, unauthenticated table, changed owner-step accounting or publication during an unfinished operation kills the design. Use measured total proof count to assess throughput, not median access cost alone. [GP:9, GP:13, C6:shared/owner_runtime.rs:3506] | Engineering 1–3 weeks for a representative vertical slice. Modeling 30–120 minutes. Selected proofs 2–8 hours, 20–50 GB. Full-language refinement requires additional engineering. |
| **6. Typed PCD, shape and replay** | Run coverage, reusable claims, exact dependency mappings and settlement uniqueness coexist. | Extend the formal claim model. Exercise chain/tree/diamond graphs, unary nodes, singleton R, repeated claims, alternate SEG bracketings, missing genesis, nonempty dependencies through plain seal, mixed roots and cross-shape replay. Prove selected hostile witnesses through the decider. | Any false completed claim, unresolved dependency, duplicated settlement effect or unbound required shape distinction kills the relation. Rejecting legitimate shared-ancestor fan-out also fails. [GP:24, GP:47, GP:50, Lean:784] | Engineering 4–10 days. Bounded model work 10–60 minutes, 2–8 GB. Proof-backed minimal cases 2–6 hours, 20–50 GB. |
| **7. Resource fallback: k=21 versus a different inner system** | If k=20 fails, a larger KZG node or a different inner system can meet the same requirements within the machine budget. | Compare a measured k=21 binary/ternary candidate with one selected folding or recursive-STARK candidate on the same memory and foreign-verification workload. Include the complete Midnight wrapper. Measure SRS requirements, proof count, time, RSS and terminal verification. | No fixed-key binary composition, no actual Compact path, an unaffordable wrapper or a narrower language kills the candidate. Inner-only speedups do not count. [GP:9, GP:33, GP:46] | KZG engineering 2–5 days. Alternative-inner engineering 3–8 weeks. Machine work 2–12 hours per small complete comparison. KZG peak 35–75 GB. Alternative-wrapper memory must be measured and stopped before exhausting the host. |
| **8. Frozen-key GP acceptance and full contract call** | One frozen core key set and one deployed verifier interface satisfy all four GP acceptance suites. | Use unseen programs, oversized committed state, input and every outcome. Prove cross-run consumption, a three-program batch with openings, a Sherlock-plus-Compact aggregate and chain/tree/DAG shapes at depth at least eight. Measure final proof length and verification cost across shapes. Validate complete local ledger calls and replay negatives. | Any new program/shape requires replacement core keys, any required outcome is missing, a claim cannot be opened, verification grows with graph size, or only cryptographic verification succeeds while call validation fails. [GP:16, GP:27, GP:38, GP:50] | Engineering 3–7 days after the components work. Serial machine run 16–48 hours, potentially longer for a slow general foreign verifier. Peak target 20–60 GB. Record total proof count and time rather than extrapolating ideal concurrency. |

The first adoption decision should follow ranks 1–4. Those experiments determine whether the named-lift optimization preserves every verification obligation, whether its decider fits, and whether it actually accepts native Compact proofs. [MC:verifier/verifier_gadget.rs:683, GP:38]