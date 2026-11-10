# Proposal

Choose an authenticated initialization computation and a register-based processor with chiplets inside a binary recursive node. Target two core keys, `{N, D}`. Preserve GP-1 through GP-4 without narrowing.

The performance figures below are **ESTIMATE**, except where explicitly identified as repository measurements. No experiment described below was run for this proposal.

## D1 Genesis mechanism

- **decision:** Choose **(c): authenticated initialization through a proved `INIT` computation**, using the same processor and node key as execution. This retains the in-circuit canonicality guarantee of option (a), while allowing initialization to span bounded proof chunks.

  A base node starts from constrained reset registers and canonical empty mutable stores. `INIT` authenticates and validates the program, input, configuration, fuel and stack limit, then constructs the canonical execution state. The first execution transition, at `h_pre == 0`, must consume that completed initialization state. An unfinished initialization cannot produce a run claim.

  Bind execution parameters explicitly into the versioned run identity. Preserve parameters already committed by `prog_d`; where the new format places parameters in `input_d`, constrain the correspondence. Do not accept independently supplied fuel or stack limits.

- **rationale:** The existing node checks equality with supplied `init_d`; that does not establish canonical initialization. The contract constructor similarly checks only that its digests are nonzero. Initialization itself takes fuel and stack limit. These are distinct obligations. [ivc.rs:498](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/ivc.rs:498), [sherlock-run.compact:29](/home/hoskinson/Projects/Moriarty/tools/sherlock-sdk/sherlock-run.compact:29), [reference.rs:14](/home/hoskinson/Projects/Moriarty/crates/mori-core/src/sherlock/reference.rs:14)

  The reported genesis prefixes have different lengths, so comparing one fixed padded carrier image is not an established solution. Streaming canonical initialization also avoids making program length a key parameter. [BRIEF.md:7](/home/hoskinson/Projects/Moriarty/research/sherlock-gp-decisions-council-20261110/BRIEF.md:7), [general-purpose.md:13](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:13)

- **risks:** The boot computation becomes part of the soundness boundary. Its parser, length encoding and transition into execution must establish the complete state invariant. Since initialization can span chunks while owner-step height remains zero, `h_pre == 0` alone cannot distinguish reset from initialization continuation.

- **kill_criterion:** Reject this implementation if a fresh witness can skip initialization, change a committed execution parameter, introduce arbitrary initial memory, or enter execution from an unfinished initialization. Prefer a direct first-transition check if E3 demonstrates equivalent canonicality with lower total cost and no capacity bound.

- **experiment:** First specify the reset, initialization-continuation and execution-entry relations. Use the differing `literal` and `add` layouts to compare their generated initial states with the reference initialization. The first constraint experiment should mutate fuel, stack limit, logical lengths, inactive data and the initialization-complete flag independently, using fresh witnesses.

## D2 Leaf architecture

- **decision:** Replace the carrier-based leaf with **a small processor trace, processor registers, and specialised chiplets inside `N`**. Use two predecessor-proof slots and bounded inline work. There is no separately proved execution leaf in the selected architecture.

  The processor carries control state, semantic height, fuel, active operands and commitments to larger structures. Route operations through an authenticated lookup/permutation bus to chiplets for:

  - Program decoding and authenticated PROG reads.
  - Memory reads, writes and page authentication.
  - Hashing and canonical value encoding.
  - Limb-based 128-bit arithmetic and range checks.

  Stack compatible chiplet regions in shared advice columns where possible. Minimise both trace rows and the number of polynomial commitments. Authenticate program normalization once during `INIT`; subsequent instruction fetches must open the normalized program commitment.

  Close the bus argument within each chunk. Link chunks through authenticated memory roots and complete continuation state. A lookup proving that an operation appears in a table is insufficient for stateful memory: the design must also establish operation conservation, ordering, initialization and read-after-write consistency.

  **Expected design envelope:**

  | Quantity | Proposed envelope |
  |---|---|
  | Complete binary node, including chiplets and changed self-verifier | **ESTIMATE:** 0.90–0.99 million rows, `k_N = 20` |
  | Terminal decider over the actual new node key | **ESTIMATE:** `k_D = 20`; fit remains uncertain |
  | Node proving time | **ESTIMATE:** 75–90 seconds |
  | Prover peak memory | **ESTIMATE:** 20–35 GB per process |
  | Qualification workload packing | **ESTIMATE:** average eight owner steps per execution chunk |
  | Amortized execution proving cost | **ESTIMATE:** approximately 9–12 seconds per owner step, excluding initialization and finalization |

  These are hypotheses to test, not measured speedups. Chunk capacity limits work per proof, not program size, memory size or run length. A long semantic operation continues across chunks without changing keys or consuming extra semantic fuel.

  **G-C plan:** require each qualification run to use at most 60 node proofs, including initialization, execution, continuation and outcome serialization. With **ESTIMATE** 90 seconds per node and **ESTIMATE** 1,200 seconds for setup, the decider, independent verification and call validation, total wall is **ESTIMATE** 6,600 seconds. Use a serial qualification schedule initially; this projection takes no concurrency credit. Actual acceptance still requires both prescribed runs below two hours, 48 GB per process and 100 GB aggregate. The requirements expressly prohibit credit for unmeasured improvements. [requirements.md:33](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/requirements.md:33), [requirements.md:37](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/requirements.md:37)

- **rationale:** The measurements identify an execution-representation problem before a proof-routing problem. Dispatch alone costs 2,747,489 rows; measured individual rules reach `k=23`. The code range-checks image/helper lanes and performs a PROG walk in the shared facts block. The digest bridge also hashes complete carriers. [BRIEF.md:9](/home/hoskinson/Projects/Moriarty/research/sherlock-gp-decisions-council-20261110/BRIEF.md:9), [c6_dispatch_step.rs:608](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:608), [sherlock_c6_child.rs:551](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_c6_child.rs:551)

  The candidates compare as follows:

  | Candidate | Assessment |
  |---|---|
  | Shrink shared facts but retain a separate universal leaf | Committed memory and changed value encoding are necessary ingredients. They need measured row reductions; merely sharing existing dispatch once does not remove its measured `k=22` cost. A standalone `k≤19` leaf is a target, not an established result. |
  | Per-rule or per-class named keys | A finite program-independent rule-key set need not violate GP-1. However, it does not fix carrier capacity or the measured `k=21–23` rule costs. Forty-five rules also lack fragments. Do not select it as the throughput solution. |
  | Inline processor using existing expanded fragments | Reject unchanged: moving the same constraints into `N` does not reduce them. |
  | **Processor registers plus chiplets/coprocessors** | **Select.** Authenticate only accessed state, reuse constrained arithmetic/hash services, and execute bounded micro-operations rather than instantiate full carrier checks for every rule. |
  | Different inner proof system, followed by a Midnight wrapper | Keep as the fallback if the complete inline node fails. A different inner system does not eliminate the outer verification cost. The prior study’s wrapper range is **ESTIMATE `k=20–23`**, not a demonstrated fit. |

  Evidence for the measured candidate limitations is in the brief; the current leaf wrapper rejects `k>19`. [BRIEF.md:10](/home/hoskinson/Projects/Moriarty/research/sherlock-gp-decisions-council-20261110/BRIEF.md:10), [proving.rs:22](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/proving.rs:22), [architecture.md:209](/home/hoskinson/Projects/Moriarty/research/sherlock-gp-architecture-20261110/architecture.md:209)

  Miden supplies a concrete example of specialised chiplet traces; SP1 exposes a LogUp-based interaction argument. These support the architecture pattern, not the numerical estimates above. The pinned local proof library has expression-to-expression lookup support, but that API alone is not a complete authenticated bus or memory argument. [Miden chiplets](https://docs.rs/miden-vm/latest/miden_vm/trace/chiplets/struct.Chiplets.html), [SP1 permutation trace](https://docs.rs/sp1-core/latest/sp1_core/stark/fn.generate_permutation_trace.html), [circuit.rs:1885](/home/hoskinson/Projects/Moriarty/tools/profile/vendor/midnight-proofs-0.8.3/src/plonk/circuit.rs:1885)

- **risks:** Additional columns and lookups enlarge the self-verifier and decider. Consequently, “730,243 baseline rows plus execution rows” is not a valid final estimate. The binary probe is a modelling stand-in, and the existing decider already uses 929,166 rows. [tree_node_rows.rs:1](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/tests/tree_node_rows.rs:1), [run-decider.json:20](/home/hoskinson/Projects/Moriarty/qa/sherlock/manifests/run-decider.json:20)

  Other risks are excessive authenticated-memory traffic, incorrect bus multiplicities, an unsound program-normalization shortcut, and failure to preserve dispatch priority or canonical value semantics. Do not substitute an arena address for a canonical result digest. [architecture.md:264](/home/hoskinson/Projects/Moriarty/research/sherlock-gp-architecture-20261110/architecture.md:264)

- **kill_criterion:** End the `k=20` hypothesis if the complete node exceeds the existing 996,147-row engineering threshold or requires a larger domain after table and blinding overhead. Reverse the architecture recommendation if measured complete-system costs then miss G-C or memory limits. Also reject it if witness scheduling, table selection or program normalization changes the keyed constraint system.

  Crossing `k=20` is an engineering failure of this estimate, not proof that GP-4 is impossible. [tree_node_rows.rs:53](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/tests/tree_node_rows.rs:53)

- **experiment:** First construct a structural cost probe with the intended columns, lookups, public statement, two self-verifiers, canonical disabled slots and terminal decider. Measure the complete key cascade before implementing all semantics. Then implement the smallest slice containing authenticated PROG fetch, a memory write and checked 128-bit arithmetic. Compare its operation counts against the qualification traces, followed by measured proving and finally both complete G-C runs.

## D3 GP-3/GP-4 wording points

### (i) Two prior proofs plus new work

- **decision:** Meet the existing wording literally: one `N` invocation verifies two enabled predecessor proofs and performs nonempty inline processor work. Do not count a second aggregation node as satisfying this clause. Long work may continue through subsequent nodes, but at least one useful operation must be possible alongside both predecessors.

- **rationale:** GP-4 explicitly requires two prior proofs plus new work. The measured three-slot construction has 1,078,147 rows; the two-self probe has 730,243. These measurements support testing inline work, while leaving its full cost unresolved. [general-purpose.md:46](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:46), [speed-levers.json:26](/home/hoskinson/Projects/Moriarty/qa/sherlock/decisions/speed-levers.json:26)

- **risks:** A design may quietly replace useful new work with bookkeeping, or require disabling one predecessor whenever execution occurs. Neither demonstrates the intended capability.

- **kill_criterion:** Reject the implementation if two enabled predecessors and a checked operation cannot coexist under the same node key, or if doing so breaks the measured resource envelope.

- **experiment:** Model and then prove one node consuming two distinct verified claims and computing a checked operation from them. Mutate either predecessor, an operand and the local result independently. Subsequently run the specified chain, tree and shared-ancestor DAG acceptance cases. [general-purpose.md:50](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:50)

### (ii) Circuits outside the Midnight stdlib configuration

- **decision:** Keep GP-3’s scope. Execute an authenticated foreign-verifier program in the processor, across continuations when necessary. Registry membership must bind the complete verification interpretation: key bytes, constraint-system descriptor, proof format, transcript, public-input schema, commitment parameters and statement adapter.

  Use the existing specialised verifier only as an optional fast path. For foreign proofs using different authenticated KZG parameters, complete their pairing verification inside the processor; do not fold their obligations into the outer accumulator under unrelated parameters.

- **rationale:** The current assigned-key interface receives the domain and constraint system outside the circuit; only its transcript representative is assigned. The gadget also asserts a single phase, while the native verifier iterates through phases. Registry membership alone cannot remove those restrictions. [verifier_gadget.rs:207](/home/hoskinson/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.3.0/src/verifier/verifier_gadget.rs:207), [verifier_gadget.rs:308](/home/hoskinson/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.3.0/src/verifier/verifier_gadget.rs:308), [verifier.rs:78](/home/hoskinson/Projects/Moriarty/tools/profile/vendor/midnight-proofs-0.8.3/src/plonk/verifier.rs:78)

- **risks:** Key normalization, verifier-program admission and adapter semantics become security boundaries. A registered scalar disconnected from the actual commitments or descriptor must never authorize verification. Generic foreign verification may require many processor chunks; no throughput claim for it follows from the Sherlock-only G-C estimate.

- **kill_criterion:** Reject this implementation if admitting a supported foreign architecture requires a new core key, or if any descriptor field affecting verification remains an unauthenticated host input.

- **experiment:** Choose one small same-curve KZG circuit outside the stdlib configuration, such as a different phase structure. Trace its verifier operations and authenticated descriptor fields first. The eventual proof experiment must accept it under unchanged `{N,D}` and reject descriptor, key, SRS and adapter substitutions.

### (iii) Ledger-format Compact proofs

- **decision:** GP-3 completion requires verification of existing ledger-format proof bytes without the circuit witness. Implement the actual Blake2b transcript in the processor. Poseidon re-proving is an intermediate test only.

  Match keyed initialization, domain prefixes, byte encodings, challenge derivation, exact public-input lengths, canonical point/scalar parsing and complete input consumption. A completed foreign claim asserts circuit satisfaction for those public inputs; it must not assert transaction inclusion without separate inclusion evidence.

- **rationale:** Ledger verification selects Blake2b, and the transcript initializes a keyed 64-byte hash with explicit absorption and challenge prefixes. Public-input lengths are transcript inputs. Replacing the transcript or padding public inputs therefore changes the verification relation. [ledger tag `ledger-9.1.0.1`, `transient-crypto/src/proofs.rs:65,563`](https://github.com/midnightntwrk/midnight-ledger/blob/ledger-9.1.0.1/transient-crypto/src/proofs.rs#L563), [implementors.rs:13](/home/hoskinson/Projects/Moriarty/tools/profile/vendor/midnight-proofs-0.8.3/src/transcript/implementors.rs:13), [verifier.rs:69](/home/hoskinson/Projects/Moriarty/tools/profile/vendor/midnight-proofs-0.8.3/src/plonk/verifier.rs:69)

  The existing recursive transcript gadget explicitly omits complete-byte-consumption enforcement and subgroup enforcement for received points. Those omissions cannot silently define byte-exact ledger compatibility. [transcript_gadget.rs:76](/home/hoskinson/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.3.0/src/verifier/transcript_gadget.rs:76), [transcript_gadget.rs:123](/home/hoskinson/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.3.0/src/verifier/transcript_gadget.rs:123)

- **risks:** Transcript differences can survive positive tests. Typed result extraction also requires the adapter to authenticate the relevant public-input interpretation; a proof of circuit satisfaction does not establish ledger inclusion. [architecture.md:320](/home/hoskinson/Projects/Moriarty/research/sherlock-gp-architecture-20261110/architecture.md:320)

- **kill_criterion:** Reject compatibility if any transcript challenge differs from native verification, accepted bytes require the original witness, or malformed encodings accepted by the processor are rejected by the intended native verifier.

- **experiment:** Start with a stored Compact proof and compare the complete transcript event/challenge sequence against native verification. Exercise altered lengths, trailing bytes, invalid points and transcript selection before attempting an end-to-end recursive proof.

### (iv) Fixed tree depth versus expandable storage

- **decision:** Use expandable authenticated storage. Tree shape, logical extent and path progress are authenticated data. Process long paths across chunks; publish a changed root only after the entire operation completes. Apply the same principle to program storage, registry membership and claim opening.

  Keep the existing semantic run-height bound below `2^64`. Do not reuse that field as a storage-capacity bound or count proof chunks as owner steps.

- **rationale:** GP-1 explicitly rejects key-dependent capacity, and GP-4 forbids depth-dependent keys. A fixed circuit path length would reintroduce such a limit. The prior architecture study identifies fixed Merkle depths as an unresolved conflict. [general-purpose.md:13](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:13), [general-purpose.md:44](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:44), [architecture.md:186](/home/hoskinson/Projects/Moriarty/research/sherlock-gp-architecture-20261110/architecture.md:186)

- **risks:** A continuation can omit an address component, permit an early write, alias two tree shapes or wrap a length counter. Fixed-width implementation counters must not become hidden capacity limits.

- **kill_criterion:** Reject any implementation where a larger valid program, store or authentication path requires re-keying, or where a partial operation can authorize a completed claim.

- **experiment:** Authenticate a path crossing the chosen chunk boundary, extend the tree, and resume under the same keys. Mutate the address, old root, direction sequence and continuation cursor. Also attempt to seal while an operation remains pending.

## D4 Inline processor versus separate leaf plus lifts

- **decision:** Select **inline execution and inline foreign-verifier programs, with core keys `{N,D}`**. Keep chiplets as constrained subcircuits of the processor, not independent proofs requiring lift keys.

  `N` must distinguish unfinished initialization, execution and foreign-verification continuations from completed claims. Cross-run consumption checks a proved predecessor claim and binds it into the consuming run’s input/dependency commitment. Adjacent execution segments require matching state commitments and semantic heights; independent runs compose as claims. Bind child order, enabled slots, operation kind and local work into the shape commitment.

  `D` verifies the root node, pins its key, resolves the outer accumulator and checks the final pairing. It accepts only completed claims. Register the Sherlock machine identity as well as foreign key identities to satisfy GP-3’s acceptance condition.

- **rationale:** Separate lifts are technically possible, but they do not reduce the measured dispatch work. Their proposed architecture also introduces additional named base sets and an expected closing proof before the decider. The current decider already demonstrates the useful terminal pattern: verify the root, fold its carried obligation, resolve bases and pair. [architecture.md:105](/home/hoskinson/Projects/Moriarty/research/sherlock-gp-architecture-20261110/architecture.md:105), [architecture.md:214](/home/hoskinson/Projects/Moriarty/research/sherlock-gp-architecture-20261110/architecture.md:214), [run_decider.rs:208](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/run_decider.rs:208)

  Inline work also directly serves GP-4’s arity requirement, while authenticated verifier programs provide the general GP-3 path. [general-purpose.md:33](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:33), [general-purpose.md:46](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:46)

- **risks:** One processor key concentrates implementation risk, and adding chiplets can make recursion more expensive. Arbitrary-depth PCD security also needs an argument for the actual accumulator construction; fixed proof size alone does not establish it. The straightline-extractor literature supplies sufficient conditions, not a ready-made proof for this implementation. [Security Bounds for Proof-Carrying Data from Straightline Extractors](https://eprint.iacr.org/2023/1646)

- **kill_criterion:** Reverse the inline choice if a separate chiplet-based universal leaf demonstrably gives a better complete G-C result, including lifts and closing, while preserving literal two-predecessor-plus-work support and the generic foreign-verifier path. Reject `{N,D}` as sufficient if the actual terminal relation cannot close every outstanding obligation within the accepted resource envelope.

- **experiment:** Compare the same small chiplet processor in two placements: inside the complete binary node, and as a standalone leaf with all required recursive verification and closing work included. Model both complete key cascades before measuring wall time. Include malformed disabled-slot carries and attempted finalization of unfinished continuations.

## requirement_text

No narrowing of GP-1, GP-2, GP-3 or GP-4 is proposed. The following are **OWNER AMENDMENT** additions or replacements that make the selected implementation obligations explicit.

1. **OWNER AMENDMENT — replace SRC-021 with authenticated initialization**

   When a run is accepted as canonical, the proof SHALL establish that its initial execution state is the result of canonical initialization of the authenticated program, input, configuration, fuel and stack limit. Initialization MAY span proof chunks. The first execution transition SHALL require completed initialization, and equality with a supplied `init_d` SHALL NOT satisfy this requirement.

2. **OWNER AMENDMENT — processor continuation semantics**

   While an operation spans proof chunks, each chunk SHALL authenticate its complete continuation state. A chunk boundary SHALL NOT change the language transition relation, consume additional semantic fuel, increment owner-step height without a completed owner step, or authorize an unfinished result as a completed claim.

3. **OWNER AMENDMENT — chiplet and memory integrity**

   When the processor delegates an operation to a chiplet, the proof SHALL bind the operation identity, operands and result to the processor transition. Stateful interactions SHALL establish operation conservation, ordering and memory consistency. Cross-chunk state SHALL be authenticated independently of prover-supplied table contents.

4. **OWNER AMENDMENT — literal GP-4 new-work acceptance**

   When both predecessor slots are enabled, one node proof under the same node key SHALL verify both predecessors and constrain nonempty local computation. A composition requiring a separate node solely to add that computation SHALL NOT satisfy this acceptance case.

5. **OWNER AMENDMENT — complete GP-3 verification interpretation**

   When a foreign proof is admitted, the node SHALL authenticate its verifying-key identity and all verification-relevant descriptors, transcript parameters, commitment parameters and statement-adapter identity against the authorized registry. Supported circuits outside the Midnight stdlib configuration SHALL remain verifiable without changing the core key set.

6. **OWNER AMENDMENT — existing Compact proof bytes**

   When a Compact proof is claimed as verified, the recursion SHALL verify the supplied ledger-format proof bytes and exact public inputs under the registered key without requiring the original circuit witness. Verification SHALL match the specified native transcript and parsing rules. Re-proving with another transcript SHALL NOT satisfy this requirement.

7. **OWNER AMENDMENT — expandable authenticated structures**

   When a valid authenticated structure or authentication path exceeds one chunk’s work capacity, the processor SHALL continue its validation across chunks under unchanged keys. A keyed path-depth, carrier-size or page-count limit SHALL NOT impose an additional program, input or storage-capacity limit.

8. **OWNER AMENDMENT — completion and dependency closure**

   When the decider accepts a claim root, every reachable claim SHALL have completed its required initialization, computation, dependency checks and cryptographic verification obligations. Unfinished continuations SHALL NOT be exposed as completed claims. Reuse of a proved claim by multiple consumers SHALL remain permitted, and the proof statement SHALL bind the composition shape.

G-C’s existing time, memory and complete-pipeline acceptance limits remain unchanged.