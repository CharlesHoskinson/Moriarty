**Verdict: adopt with changes.**

The overall direction is sound: dense tail-buffered memory, authenticated cache lines, hash-chain stacks, canonical value digests, explicit segment/claim separation, and no claim-occurrence counter. The document is not ready to freeze as an implementation specification. Several interfaces cannot express the landed machine, and several essential assumptions have no implementing owner.

This audit distinguishes defects in the proposed contracts from vulnerabilities in the landed circuit. The findings below do not claim an executed exploit against the current implementation.

Evidence references use these repository-relative prefixes:

- **A**: `research/sherlock-final-design-20261111/ARCHITECTURE.md`
- **D**: `research/sherlock-final-design-20261111/DECISIONS.md`
- **O**: `openspec/changes/sherlock-vm-consensus-20261107/`
- **C**: `crates/mori-chip/src/`
- **L**: `clean/fork/Clean/Sherlock/`

The source baseline inspected was commit `b85b00213f63fdc13cb2b84fa23c613d17706653`, together with the working-tree architecture.

**Numbered findings**

1. **Blocker — CORE: the specified scratch enforcement does not establish write-before-read.**

   **Evidence:** A:537, A:1406; D:16; C/rfcore.rs:1731.

   The proposed enforcement zeros a scratch register only when the selected rule does not write it. A rule that reads unconstrained `Pre(v0)` into a persistent result and subsequently writes `v0` escapes that condition. Removing cross-slot links does not close the gap. The resulting persistent state can depend on a witness value outside the boundary commitment.

   The landed spike has a stronger syntactic check: its test rejects reads of `Pre(vk)` outside the scratch register’s own next-state constraint.

   **Required change:** Specify ordered scratch production and consumption, with source cells copy-linked to their producers. Use SSA-style temporaries or an equivalent definite-assignment check. Add the specific mutant “read scratch, then write that scratch later in the same slot.” State the theorem as independence from incoming scratch, rather than merely dependence on the full pre-state.

2. **Blocker — STACKS and CORE: one operation per stack cannot express landed owner transitions.**

   **Evidence:** A:520, A:616, A:623; `crates/mori-core/src/sherlock/helper_machine/families/structural.rs:86`, :169; `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:3566`; L/PartitionIndependence.lean:228, :261.

   `BHelperValidateField` and `BHelperEqualField` can rewrite the parent task’s cursor and then push a child task in one owner step. The runtime applies `task_rewrite_top` before `TaskOp::Push`. The architecture permits only one of `none`, `push`, `pop`, or `rewrite`; its push spills the old top. That either loses the cursor update or fails to push the child. Splitting the transition into two owner slots would change accounting.

   There is also a false stack invariant at A:631: `below == EMPTY` is possible at count one, without any collision. The cited Lean theorem concerns the hash of a complete list, not the hash of the suffix below a cached top.

   **Required change:** Add an ordered compound operation or bounded stack micro-operation sequence within one owner event. Spill the rewritten parent. Update budgets, models and tests. Define the cached-top abstraction explicitly; the canonical empty suffix corresponds to counts zero or one.

3. **Blocker — Init, PAGE/ROM, VALUE DIGEST and BOUNDARY: genesis authentication has no complete kit or owner.**

   **Evidence:** A:540, A:808, A:930, A:980, A:1412, A:1970, A:2708; O/requirements.md:46.

   CORE assumes canonical Init. Memory assumes an empty or proved initial root. PAGE/ROM delegates the `prog_d` opening to an unspecified “Init/PROG owner.” VALUE DIGEST assumes an authenticated `val_d`. NODE then treats genesis as already proved.

   No kit supplies the complete one-arena Init relation, program opening, initial input-memory relation and dependency initialization. Repeating the record-kind gate, as proposed in V-3, checks a type range; it does not authenticate the initial contents or connect them to `input_d`.

   **Required change:** Add an Init/input-admission kit, or an equally complete named subkit with one owner. Specify every initialized field, the genesis selector’s equality to public `h_pre`, program/profile binding, resource parameters, empty dependencies and input-value authentication. Preserve the explicit PROG canonicality premise where the governing specification permits it, but identify its provider. Do not introduce a whole-input scan or a new input-size limit.

4. **Major — VALUE DIGEST and PAGE/ROM: independent record requests lack a complete read contract.**

   **Evidence:** A:91, A:511, A:945, A:975, A:979, A:1254, A:1356.

   VALUE DIGEST may issue MEM requests itself, but the current-length check and address provenance are assigned to CORE. Its skeleton accepts “authenticated” fields without defining a token that carries address, expected kind, step-local frontier and provenance.

   Membership in the window is insufficient: the window includes records appended later in the leaf. PAGE/ROM correctly delegates `addr < len_cur` to the requester, but the VALUE DIGEST requester has no corresponding interface obligation. The frozen range-owner table also assigns ROM row bounds to PAGE/ROM, while the detailed kit requires the requester to check them.

   **Required change:** Define a shared authenticated-read interface. Every consumer must receive fields tied to the requested address, expected kind and frontier at that precise read point. Define who ranges each ROM request index before packing it with the table id. Add VALUE DIGEST-specific future-record, wrong-reference and ROM-row-overflow mutants.

5. **Major — BOUNDARY, NODE and DECIDER: the required opening interface is absent, and its cost argument uses a different commitment structure.**

   **Evidence:** A:883, A:1398, A:1404, A:1484, A:1538, A:1607, A:1614, A:2159; `crates/mori-recursion/src/ivc.rs:81`; `crates/mori-recursion/src/run_decider.rs:278`.

   BOUNDARY specifies one flat sponge over all state words. Its public API only serializes and commits. NODE requires `StateOpeningCells`, and its read list refers to an opening signature that does not exist.

   Placing terminal fields at fixed positions in a flat sponge does not provide a selective opening. The landed decider’s small opening works because its state digest has a short outer header containing an opaque `lanes_d`. It is not the proposed flat commitment.

   **Required change:** Choose and specify the commitment structure. Either rehash the complete fixed-width state at sealing and budget that cost, or define a header/body commitment with explicit domains and equality constraints. Supply typed opening APIs for Init, terminal sealing and continuation, including the bridge between leaf cells and recursion cells. Measure the chosen opening inside NODE and DECIDER.

6. **Blocker — GP-2: the recommended dependency format is not a complete discharge protocol.**

   **Evidence:** A:1750, A:1753, A:1754, A:1777, A:2104, A:2705.

   Replacing the running dependency chain with “the SET hash” does not by itself make equality exact. A consumed claim can reside inside a JOIN or a `SET_DEP` containing its own ancestors. The verified producer root can therefore differ from the consumer’s accumulation of consumed claims. Empty dependencies, ordering, duplicates, claim extraction and authenticated `n_steps` metadata are also unspecified.

   The consumer records `claim_d`, while the proposed singleton SET includes `n_steps`. The document supplies no complete update rule connecting those representations.

   **Required change:** Define the SET grammar and dependency accumulation/discharge relation together. Specify how a claim is authenticated inside a verified SET and how it is added to the consumer’s dependencies without requiring an accidental producer batching shape. Cover empty dependencies, repeated consumption, nested dependencies and different join bracketings. Define the consume transition against the owner machine, including its accounting.

   The value-opening API also needs a bounded protocol for structured values; VALUE DIGEST currently supplies construction/folding methods, not the opening signature GP-2 is told to read.

7. **Major — BUS and all copied ports: the proposed composition proof does not yet cover the actual module graph.**

   **Evidence:** A:66, A:67, A:192, A:193; L/ModularComposition.lean:37, :49, :63, :91; C/bus.rs:50.

   C3’s composition theorem propagates semantic validity through requested and provided tuples. Many essential architecture guarantees travel only through copied cells: STACKS, boundary commitments, whole-sponge HASH calls, RANGE and DEC outputs.

   These guarantees cannot simply be omitted from the proof interface. Nor can the listed levels be reused unchanged: MEMORY BACKEND consumes HASH at the same stated level. The semantic relationship between newly stored value digests and MEM providers also needs a causal invariant.

   Separately, the landed lookup gives membership, not permutation equality between raw request and provider lists. Unused providers and repeated requests are legal.

   **Required change:** Extend the formal interface to typed port certificates and their copy equalities, or provide a composition theorem that explicitly uses producer relations for those ports. Define a well-founded dependency order, including memory’s history invariant. Use `LookupSound` for the landed bus, or define multiplicity-expanded lists before invoking `BusBalanced`. Add a concrete composition example involving copied ports.

8. **Major — MEM and MEMORY BACKEND: the partition theorem’s capacity premise is misstated.**

   **Evidence:** A:689, A:728, A:846, A:848; L/PartitionIndependence.lean:397, :413, :423.

   `FitsLeaf` requires at most eight appends over the entire leaf. A per-rule maximum does not discharge that premise for a multi-step leaf.

   Moreover, the landed W=1 window permits up to `15 - r_pre` new records, including more than eight when the initial tail is short. The lower-level one-flush equality can cover the corresponding state-dependent bound, but the cited `cuts_agree` theorem still requires the stronger eight-appends premise. A:728 incorrectly lists no premises.

   **Required change:** Make the scheduler enforce the exact proved cumulative bound, or strengthen the partition theorem to the window’s state-dependent capacity and then generalize it to W flush units. Keep single-step fit and cumulative leaf fit as separate obligations. Test a W=1 leaf with 9–15 appends where legal, and multiple individually fitting steps whose sum exceeds the proved bound.

9. **Major — MEM and PAGE/ROM: capacity proofs are incomplete, and narrowing alternatives must not remain acceptance paths.**

   **Evidence:** A:542, A:695, A:731, A:947, A:2168, A:2505; D:10, D:12; O/general-purpose.md:9, :14.

   The address-bound obligation accounts for appended cells but not initial memory length. Wide logical records also expand into several physical cells. The ROM depth argument must similarly include sub-row expansion, not merely the logical table index width.

   An observed corpus maximum is not a bound over every valid rule state. Early close cannot repair a step that exceeds an empty leaf’s capacity.

   **Required change:** Prove bounds over physical cells and physical ROM rows, including initialization and record expansion. State field-injectivity bounds for packed keys. Derive per-rule maxima from all rule branches and validate them with measurements.

   Smaller ROM depths, per-program depth keys, or a memory bound below the required run scope must remain rejected alternatives under this task. Failure to fit leaves GP-1 incomplete; it does not authorize narrowing it.

10. **Major — BUS and DEC: fixed tags and witness-controlled disabled slots have conflicting encodings.**

    **Evidence:** A:97, A:156, A:370, A:532.

    The tagged-table query tag is described as a fixed selector, while a disabled DEC slot must query `(0,0,0)`. Slot enable is witness-controlled under the same key. A fixed DEC tag cannot become the idle tag when that witness disables the slot.

    **Required change:** Specify the exact query expressions. Either retain the DEC tag and provide a canonical DEC idle row, or gate all three query components by the constrained enable bit. Preserve the prohibition on an independently chosen advice tag. Test full and partial leaves under the same key, plus DEC-to-RANGE alias attempts.

11. **Major — CORE and MEM: append transport is inconsistent across the frozen interfaces.**

    **Evidence:** A:108, A:508, A:518, A:692, A:2584.

    The overview describes appends as named copied cells. MEM and the conflict resolution instead use a copy-linked append chain with values authenticated through bus requests. CORE’s request inventory lists record reads but no append requests.

    The hybrid can be sound, but only if its one-to-one relation, activation conditions and request budget are explicit. An implementer cannot determine whether append values consume bus lanes or are copied directly.

    **Required change:** Freeze one transport. If retaining the landed hybrid, list append requests, bind their enable/address/kind/data to the ordered append events, and relate those events bijectively to active non-imported window rows. Include their requests in every capacity calculation.

12. **Major — CORE, HASH and NODE: `rules_d` is treated as a naming issue instead of a required computation.**

    **Evidence:** A:522, A:583, A:1518, A:2102; O/requirements.md:26, :28.

    SRC-001 specifies an exact `SHK_RULES` hash over all S rule lanes in slot order, including zero lanes for disabled slots and no S=1 exception. CORE exports rule lanes, but no kit owns this hash or its equality to the fifth public lane.

    Retaining seven lanes and renaming `rule` does not discharge SRC-001 or SRC-003.

    **Required change:** Assign ownership of the exact hash, its HASH slot and public equality. Specify the reserved-zero rule encoding. Add forged-digest, reordered-lane, disabled-rule-lane and S=1 vectors. Include the hash in row budgets.

13. **Major — CONTINUATION: the kit refers to a host persistence interface that the cited code does not provide.**

    **Evidence:** A:1690, A:1696, A:1725, A:1735; C/membackend.rs:95.

    `MemBackend::persist` assigns circuit constraints and returns boundary cells. It does not write a durable memory store or expose a `StoreHandle`. The continuation agent’s prescribed read therefore cannot supply the advertised checkpoint operation.

    The checkpoint skeleton also lacks explicit register state, raw tail records and stack-opening preimages. A root and cached top alone are insufficient to resume future stack pops.

    **Required change:** Specify a separate host store/checkpoint interface and versioned codec. Include every witness resource needed to resume: memory records and paths or their reconstructible backing data, tail, registers, stack preimages, proof-prefix representation and manifest binding. Distinguish host integrity failures from constraint failures. A corrupted unused store cell is not necessarily detected by the next leaf unless the restore procedure validates it.

14. **Major — CORE integration: the nominated reference harness cannot represent the required corpus.**

    **Evidence:** A:2008–2020; `crates/mori-core/src/sherlock/helper_machine/reference_projection.rs:1`, :48.

    The architecture requires comparison with `sherlock-k-a/1`, including generic constructors with more than eight fields. The cited projection explicitly says generic nominal constructs, planned matches and constructor state are outside that grammar and return `UnsupportedGeneric`.

    The architecture correctly says that projection gaps must fail coverage, but supplies no implementing kit that can close these gaps. A permanently failing coverage obligation is not an executable completion plan.

    **Required change:** Add a semantic-reference/projection task covering the full target language and structured inputs. Specify the mapping between helper events, owner events, source steps and free terminal publication. Complete it before using the whole-leaf harness as the acceptance gate. Do not skip unsupported cases or reduce the language to the current reference subset.

15. **Major — GP-3: the required general verification path is acknowledged but absent from the implementation plan.**

    **Evidence:** A:1807, A:1838, A:1839, A:1993–2002, A:2138, A:2480, A:2512; O/general-purpose.md:33.

    The kit itself says general foreign verification requires separate parsing, transcript, field/group arithmetic and final-equation kits. None appears in the build groups or capability-spec outline. The proposed experiment measures one Compact adapter class.

    The native path is a useful milestone. Passing it cannot establish the full stated GP-3 scope.

    **Required change:** Add the missing kits, their authenticated bounded-work state, operation inventory, scheduling and completion gates. Freeze the registry-entry encoding and adapter statement. Specify handling of supported proofs that cannot share the core accumulation SRS. Keep the native probe and full GP-3 completion as distinct tasks.

16. **Major — GP-4 and NODE: inline work and shape construction are not integrated into the node relation.**

    **Evidence:** A:1535, A:1550, A:1596, A:1853, A:1855, A:1856, A:1873; O/general-purpose.md:46.

    NODE’s mode enum and skeleton contain merge, seal, dependency seal and join, but no complete relation for two child proofs plus inline owner work. GP-4 adds a port without defining the mode/type matrix, work-state linkage or height update.

    The shape formula also needs base cases and canonical encodings for LEAF children, FOREIGN children and disabled slots. A LEAF has no `shape_d` lane to forward.

    **Required change:** Specify each legal child-type/local-work combination and its output statement. Bind inline execution to the adjacent segment state and height. Define base and disabled shape encodings and the ordering of every hashed field. Measure two enabled prior proofs plus real owner work in one node. Two successive nodes do not satisfy this requirement.

17. **Major — ZmLogUp: imported-tail authentication is not connected explicitly to the balance equation.**

    **Evidence:** A:1084–1088, A:1102, A:1106, A:1125, A:1197.

    Imported-tail entries are committed, but the kit does not explicitly classify them as read debits or otherwise authenticate them against prior writes. The backend returns no root or tail token. Committing a fabricated imported record before deriving challenges does not establish that it was previously written.

    A window-served read could therefore escape the stated old-record read-entry path unless this coverage condition is added.

    **Required change:** Define a complete mapping from imported/provider/read cells to the LogUp entries. Every used imported record must be matched to its prior write or an authenticated initial source. State deduplication and multiplicity semantics. Add a two-leaf mutant that changes an imported tail record while preserving all stated commitment bindings.

    Also resolve `open_line` capability at configuration time; a backend that unexpectedly returns `Error` is not interchangeable with a configured line-using leaf. This finding concerns the benchmark profile, not the default dense backend.

18. **Major — Build order and gates: completion can be reported before required interfaces and proofs are complete.**

    **Evidence:** A:1993, A:2029, A:2035, A:2499, A:2505–2515; O/general-purpose.md:16.

    Group F names X1 and B2 as entry conditions, while A:2515 says X1 and X2 block the freeze. The complete dependency discharge, Init relation, boundary opening and reference coverage are not scheduled as freeze prerequisites or separately owned work.

    The Lean gate permits `sorry` in an obligations file without explicitly requiring the implementation’s required obligations to be discharged. `quint typecheck` and `quint test` do not, by themselves, state which named invariants were explored or the exploration bounds.

    The listed GP-001 integration gate also omits the explicit one-key-set/one-contract acceptance over three previously unseen programs and equal final proof size.

    **Required change:** Make freeze prerequisites consistent and assign every missing interface to a task. Gate completion on the required concrete theorems, with no `sorryAx` in their dependency closure. Record Quint invariants, bounds, seeds and mutant results. Include the complete GP acceptance matrix, actual table/domain headroom, and the repository’s mandatory Rust checks. Prototype milestones may pass while GP completion remains open.

19. **Major — OpenSpec outline: top-level coverage is good, but the executable work decomposition and several evidence mappings are incomplete.**

    **Evidence:** A:2250, A:2256, A:2269, A:2295, A:2456, A:2499; `research/sherlock-final-design-20261111/MERGE-BRIEF.md:32`.

    There are 19 capability outlines, covering every listed top-level kit, including the benchmark backend. The written requirement lines are all below 500 characters; the longest is 101 characters. Those parts meet the brief.

    Missing work identified above has no corresponding capability: Init/input admission, the full reference bridge, the general foreign-verifier subkits and the durable checkpoint store. CORE family subkits are promised but not outlined.

    Some evidence mappings do not establish their scenario. `noZeroChipProvider` does not prove rejection of every unassigned nonzero chip id. `DiamondAncestorOpenedAtMostOnce` does not prove claim-opening existence. Scenarios saying removal of an equality constraint must be refused confuse a weakened circuit with a malicious witness.

    **Required change:** Add a spec/task for each implementing kit and explicit amendments to the governing VM specification. Map scenarios to the property actually tested. State the mutant protocol consistently: the correct circuit rejects the malicious witness, and the deliberately weakened control accepts it. Make the contract/template changes required by universal program acceptance and collection opening explicit work.

20. **Minor — MEM, HASH and shared schemas: several local descriptions contradict their sources or each other.**

    **Evidence:** A:106, A:135, A:439, A:569, A:744, A:951, A:966, A:1420; C/membackend.rs:329.

    - `WindowCfg::bits` returns successive quotient/accumulator cells, not the individual bit cells claimed by the skeleton.
    - HASH says every digest leaving the leaf uses the pinned sponge, then classifies the public boundary digest as capacity-tagged.
    - One seam description permits Horner with dense memory, while the BOUNDARY kit forbids it.
    - `LeafShape` inconsistently includes `n_cold`.
    - The blanket disjoint-provider rule conflicts with the explicitly permitted duplicate cache lines.

    **Required change:** Correct the `bits` return contract; distinguish semantic public digests from internal state commitments; choose one profile compatibility matrix and one `LeafShape`; scope provider disjointness to different provider classes while allowing identical duplicate lines.

**Judgment of the proposals introduced in §7.3**

| Proposal | Judgment |
|---|---|
| ROM chip id 4 | **Adopt.** It does not collide with the landed ALU/MEM/HASH ids. Freeze it in the versioned registry and reject unassigned ids. |
| Tagged-table values RANGE=1, DEC=2, idle=0 | **Adopt with correction.** The values are suitable; the disabled-query expressions must resolve finding 10. |
| Method name `open_line` | **Adopt.** It is a reasonable extension of the landed circuit interface. It must not be confused with host persistence, and backend capabilities must be explicit. |
| `H(SHK_SET_LEAF, claim_d, n_steps)` | **Adopt with conditions.** Authenticate `n_steps` from the sealed interval, keep it outside `claim_d`, and define domain separation and the typed SET grammar. |
| Dependency-join discharge | **Do not adopt as specified.** It is a candidate, not a complete protocol. Resolve finding 6 before freezing it. |
| `E = key + γa + γ²b + γ³c` | **Adopt as a tuple compression.** It is suitable for the fixed MEM namespace, subject to injective key encoding and challenge assumptions. It does not establish coverage of all reads, commitment binding or the full soundness bound. |
| New Lean, Quint and spec names | **Adopt as names.** Names carry no verification status. Correct obligation references and add the missing implementing kits. |
| Boundary schema item 5 | **Adopt the binding requirement; revise the opening design.** Program/input identities and terminal/dependency fields must be authenticated. Fixed positions in a flat sponge do not supply the claimed selective opening. |

The additional OWNER labels record unresolved choices; they are not permission to narrow GP-1 through GP-4. The recommendations generally preserve those targets. Smaller program bounds, adapter-only GP-3, two-node substitutes for inline work and per-program contracts cannot become final acceptance paths.

**Can one agent implement each kit from its prescribed reads?**

“Conditional” below means after a complete interface freeze, without inventing another kit’s semantics.

| Kit | Assessment |
|---|---|
| BUS | Conditional on corrected tagged queries, provider rules and composition interface. |
| RANGE | Locally implementable after the tagged-query and width interfaces are frozen. |
| ALU | Locally implementable from its contract and sources, including in-chip ranges and explicit overflow outputs. |
| DEC | Blocked by the complete rule inventory, dispatch classes and packed-control schema. |
| HASH | Locally implementable; constant-start mode needs its stated differential validation. |
| CORE | Framework is conditional; rule-family subkits and the full reference mapping remain unwritten. |
| STACKS | Not implementable correctly with the current single-operation contract. |
| MEM window | Landed implementation is readable; append transport and capacity-proof interfaces need correction. |
| Dense MEMORY BACKEND | Landed circuit seam is usable; genesis and generalized partition obligations remain external prerequisites. |
| PAGE/ROM | Blocked by PROG/Init, ROM layout/depth, requester range ownership and authenticated-read interfaces. |
| ZmLogUp | Not implementation-ready; appropriately deferred beyond M4. |
| VALUE DIGEST | Blocked by record layout, read provenance, input authentication and the consumer opening protocol. |
| BOUNDARY | Blocked by the complete state schema and opening API. |
| NODE | Blocked by boundary openings, child routing, dependency discharge and inline-work integration. |
| DECIDER | Blocked by final statements/openings and explicit contract/template work. |
| CONTINUATION | Blocked by the missing durable store/checkpoint interface. |
| GP-2 | Blocked by dependency and value-opening protocols. |
| GP-3 | Native path is conditional; the general path requires several additional kits. |
| GP-4 | Blocked by the complete node relation and shape base cases. |

**Claims verified**

- The architecture carries all 28 lookup-cache checklist items textually. This is traceability, not discharge. The principal deficiencies concern read ownership, append transport, scratch, ROM bounds, the backend seam, genesis and public-lane semantics.
- The default preserves the decided dense-memory interface: MEM id 2, key `8 addr + kind`, an opaque backend token, scalar cold reads and parameterized flush capacity.
- The inspected dense implementation authenticates the imported tail, uses domain-separated record/tree hashes, performs conditional flushes and ties scalar cold reads to the pre-state root. The proposed cache-line rules preserve that root and namespace separation. I found no additional local cache-line forgery under those stated premises.
- The architecture correctly identifies that landed ALU operand ranges are caller obligations and deliberately moves them inside ALU. It retains overflow in the tuple so overflowing arithmetic can produce the specified rejected outcome.
- The landed range and Poseidon implementations support the architecture’s main observations: bounded limb decomposition, length-dependent sponge initialization, whole-sponge hashing and a two-input HASH bus provider. The rejection of one universal precomputed value prefix is justified.
- The value-digest preimage table preserves the specified forms and uses length-specific constants. Its semantic authentication prerequisites remain open.
- The inspected Lean sources prove generic composition and abstract partition results. They do not establish concrete full-machine leaf soundness. The `FitsLeaf`, stack-abstraction and copied-port limitations above follow directly from their statements.
- C1 measurements and C3 proof status are identified as prior experimental evidence. Full-leaf, cache, stack and recursion estimates are generally labeled as estimates.
- Dropping the claim-occurrence counter avoids the stated exponential-count limit on shared DAGs.
- Retained-memory continuation, SEG/SET separation and replay by `claim_d` are consistent with the decided direction.
- The OpenSpec outline contains 19 top-level capability specs, short requirement lines, scenario/test mappings and named Quint/Lean work. Its incompleteness is in missing work and inaccurate mappings, not the character limit.

**Claims not verified**

- No build, MockProver test, real proof, Lean build, Quint run or OpenSpec validation was rerun.
- No executable counterexample against the landed circuit was constructed. Counterexamples above concern the proposed interfaces or the sufficiency of their enforcement.
- The complete rule inventory, all per-rule maxima, one-arena non-interleaving, final state/record layouts and full language-index bounds were not proved.
- No integrated leaf/node/decider row count, table headroom, throughput or proof-size claim was measured.
- Cryptographic premises—including concrete collision resistance, accumulation soundness, Fiat–Shamir security and Profile B’s grinding-adjusted error bound—were not independently established.
- The six drafts and external prior-art claims were not independently audited line by line.
- Full GP-1 through GP-4 support, deployed-contract behavior and universal foreign verification remain unverified.

This was a read-only audit. No files were written.