## 1. Verdict

**Makes sense with changes.** Keep Sherlock as a bounded VM with program-independent keys, a self-recursive linear node and a final ledger decider. The current node implements the important height, genesis-neutrality and collapsed-carry constraints; these are consistent with the revised Lean composition model. However, the design has not yet established either a sound universal leaf or a two-hour execution path: the current C6 producer exports six lanes, the node expects seven, and the performance records use stand-in leaves. F at S=1 does not meet G-C at the recorded node cost, and the proposed tree failed its row gate. The design also needs to distinguish general recursion over admitted Sherlock programs from aggregation of arbitrary circuits. These are reasons to revise the implementation gates and scope claims, rather than replace the working recursion core. (`crates/mori-recursion/src/ivc.rs:363`, `:383`, `:409`; `clean/fork/Clean/Sherlock/Aggregation.lean:1454`; `qa/sherlock/manifests/c6-leaf.json:26`; `qa/sherlock/manifests/node.json:14`; `qa/sherlock/decisions/speed-levers.json:68`, `:99`.)

## 2. Keep / change / stop

Document references below are under [`openspec/changes/sherlock-vm-consensus-20261107/`](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/design.md). Ledger references are under `repos/midnightntwrk/midnight-ledger/`, inspected at **`a2b30128`**, not the clone’s current HEAD. Measurements and test results below are repository records; I did not rerun them or change files.

| Decision | Recommendation | Evidence and reason |
|---|---|---|
| D1 — reference machine and `/1` | **Keep.** Preserve a single reference semantics. | The bounded pure machine is the declared foundation; the aggregation theorem takes its step relation as a parameter. (`design.md:9`; `clean/fork/Clean/Sherlock/Aggregation.lean:183`.) |
| D2 — bounded carriers and `StepInv` | **Keep; complete initialization first.** | Leaf soundness is conditional on a canonical pre-state. Canonical genesis remains an explicit unfinished task. (`Aggregation.lean:204`; `tasks.md:72`.) |
| D3 — derived one-hot dispatch | **Keep.** Measure the complete selector and gated fragments together. | The design correctly separates private candidate post-states from the selected post-state; current dispatch emission still specializes to one rule. (`design.md:38–41`; `crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:1317`.) |
| D4 — universal leaf versus family F | **Change.** Decide from complete leaf/node/decider cost and G-C, not the contradictory `k_U <= 17` rule. | §3.4 forces F above 17, while task 4.10 considers U at 18 with S=4. F is explicitly S=1. (`design.md:95`; `tasks.md:65`.) |
| D5 — seven-lane statement | **Keep; finish its semantics before keying.** | The producer currently has six lanes; the node requires seven. The one-step `rules_d` convention also differs between requirements and design. (`qa/sherlock/manifests/c6-leaf.json:26`; `crates/mori-recursion/src/ivc.rs:70`; `requirements.md:23`; `design.md:42`.) |
| D6 — IVC, decider and `acceptRun` | **Keep.** Treat canonical initialization as a separate obligation. | The decider verifies, folds, pairs and opens the halted result; the contract pins program/init digests and rejects replay. Those checks do not themselves establish canonical genesis. (`crates/mori-recursion/src/run_decider.rs:208`, `:243`, `:278`; `tools/sherlock-sdk/sherlock-run.compact:29`, `:40`.) |
| D7 — host witness generation | **Keep.** Host checks cannot substitute for leaf constraints. | The design assigns witness generation to the host; the current digest bridge takes outcome labels as circuit constants. (`design.md:15`; `crates/mori-backend/src/sherlock_c6_child.rs:560`, `:571`.) |
| D8 — `/2` deferred | **Keep.** State this boundary in “general-purpose” claims. | The extension remains outside the current admitted language. (`design.md:16`; `tasks.md:119`.) |
| D9 — zero fragment gaps | **Keep; update the inventory used to demonstrate it.** | The native inventory describes exercised examples, and its `RECEIPTED` list is empty; this is not equivalent to complete coverage of the dispatcher’s supported rules. (`crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:1`, `:638`.) |
| D10 — dispatch partition evidence | **Keep; move selector-model evidence before the universal leaf.** | Predicate partition work is in M3, but the one-hot selector model is currently bundled into task 5.4 after chain construction. (`tasks.md:56–60`, `:84`.) |
| D11 — ledger 9 pins | **Keep; correct stale status.** | Root Cargo dependencies already use ledger `a2b30128`, circuits 7.3.0 and stdlib 2.4.0; design §5.1 still describes older workspace pins. (`Cargo.toml:28`; `design.md:145`.) |
| D12 — Preview qualification | **Keep; update network facts.** | The saved observation reports Preview, Preprod and Mainnet on ledger 9.1.0.1. The design’s “Preprod on 8.1.3” statement is stale. (`research/midnight-ledger9-20261107/api/rpc-status-20261110.json:2`; `design.md:142`.) |
| D13 — historical A102/A80 route | **Stop treating it as current GP verification.** Retain it as historical evidence. | `verify_chain.rs` still hardcodes A102/A80 and their old statement shapes. (`crates/mori-recursion/examples/verify_chain.rs:89`, `:134`.) |
| D13′ — three-slot tree | **Stop treating this candidate as an available fallback.** Require a changed design and new gate evidence to reopen it. | The `{self,self,leaf}` probe is 1,078,147 rows, above both k=20 capacity and its 996,147-row gate. (`tasks.md:36`.) |
| D14 — independent synthetic rollup | **Keep.** Preserve the separate qualification workload. | This is the declared separation in D14. (`design.md:23`.) |
| `S_MAX` and keyed S | **Change.** Make the final bound explicit in relation serialization and align node and leaf acceptance. | The node uses compile-time `S_MAX=16`, but `write_relation` serializes only magic, columns, k and leaf VK. (`crates/mori-recursion/src/ivc.rs:75`, `:571`.) |
| Node k | **Keep k=20 as the measured baseline. Stop the existing small-trim k=19 plan.** | The hardened stand-in node is 128,145 rows over k=19; two collapse gadgets account for about 78% of its rows. (`qa/sherlock/decisions/speed-levers.json:66`.) |
| Leaf workers P=3 | **Keep provisionally; requalify mixed node/leaf execution.** | The concurrency evidence is three genesis node provers at six threads, not a node plus three real universal-leaf workers. (`tasks.md:35`; `qa/sherlock/decisions/speed-levers.json:70`.) |
| GPU track | **Change its funding metric; retain it as optional.** | The decision adds overlapping m and q shares, contrary to the profiler’s definition; q includes quotient work, not just FFT work. (`qa/sherlock/decisions/speed-levers.json:67`; `scripts/qa/profile-recursion.py:868–877`.) |
| SRC-017 generality | **Change the wording.** Promise fixed keys within an admitted Sherlock profile and a selected recursion backend. | The node fixes one leaf VK, linkage requires one program digest, and the decider requires a Sherlock halted-state opening. (`crates/mori-recursion/src/ivc.rs:548`; `clean/fork/Clean/Sherlock/Aggregation.lean:49`; `crates/mori-recursion/src/run_decider.rs:278`.) |
| G-C | **Keep unchanged.** Recording a shortfall must not count as passing. | G-C includes leaf proving, recursion, decider, independent verification and local call validation under wall and memory limits. (`requirements.md:33`; `tasks.md:86`.) |

## 3. Ranked concerns

**1. Critical completion blocker — the leaf-to-run soundness boundary is unfinished. Focus points 1 and 4.**

The six-versus-seven-lane mismatch is real, but adding `steps` alone is insufficient. The C6 bridge pins `pre_outcome` and `post_outcome` tag/value pairs as constants supplied while constructing the relation. Its public `rule` lane is published without an equality tying it to dispatch. `prog_d` currently hashes the program section, rather than the expanded image/layout/bounds/profile identity required by the design. (`crates/mori-backend/src/sherlock_c6_child.rs:538–572`, `:598`; `design.md:54`.)

The universal leaf must derive outcome/result from constrained carriers, derive `rules_d` from selected transitions, count enabled steps, and authenticate every internal link. Otherwise the seven-lane shape would describe an interface without establishing its intended meaning. This is an unfinished obligation, not a demonstrated exploit against the completed node. (`requirements.md:25`, `:28`; `tasks.md:66–72`.)

Canonical genesis is equally important. Lean’s `leaf_sound` is conditional on `Carrier(pre)`, and `decider_pinning` assumes a canonical initial state. The contract constructor checks only that program/init digests are nonzero; the node checks equality with `init_d`. Neither establishes the carrier premise. (`clean/fork/Clean/Sherlock/Aggregation.lean:204`, `:1116`; `tools/sherlock-sdk/sherlock-run.compact:29`; `crates/mori-recursion/src/ivc.rs:478`.)

There is also an encoding disagreement: `requirements.md:23` says `rules_d == rule` when **`steps == 1`**, whereas `design.md:42` gives the shortcut for **`S == 1`**. A padded S=16 leaf with one enabled step distinguishes those cases.

**Proposed change:** settle the complete statement encoding and initialization mechanism before final key generation. Require fresh malicious-witness checks for forged outcomes, selectors, counts, internal links and initial carriers. Preserve the existing node hardening.

**Tasks:** 4.2, 4.3, 4.5, 4.6, 4.9 and 5.2.

---

**2. High — universal-leaf feasibility is unmeasured, and the current sizing gate ignores downstream growth. Focus point 1.**

By counting the groups assembled by `supported_rules()`, the current dispatcher supports **108 distinct rules**: 3 publication rules + 50 distinct helper rules + 46 legacy rules + 9 helper-entry rules. This is a source count, not a measured universal-circuit size. Gating those fragments does not justify budgeting only the largest fragment; shared work must be identified and the complete relation synthesized. (`crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:140`, `:214`, `:360`.)

The relevant envelope is **leaf + node + decider**, not merely `k_U + log2(S)`. The stand-in node has 652,437 rows; the C6-key manifest records 741,708. That is **89,271 additional rows, or approximately 13.7%**, calculated from those records. The associated decider has 929,166 rows, leaving **119,410 rows, approximately 11.4% of the k=20 domain**. A larger universal-leaf verifier can therefore invalidate the downstream budget even when the leaf itself fits k=20. (`qa/sherlock/decisions/speed-levers.json:22`; `qa/sherlock/manifests/node.json:22`; `qa/sherlock/manifests/run-decider.json:20`.)

The branch policy is internally inconsistent: §3.4 requires F above k_U=17; task 4.10 permits consideration of U at k_U=18; task 4.4 uses yet another expression, `k <= 17 + log2(S)`. (`design.md:95`; `tasks.md:65`, `:68`.)

**Proposed change:** measure the full admitted relation at candidate S values, then measure both recursive relations using its actual VK architecture. Prefer U if the complete path fits. F is a useful correctness fallback, but its current S=1 definition is not a better default for G-C. A batched mixed-rule F construction would be a new design requiring its own cost and soundness evidence.

**Tasks:** 3.3–3.5, 4.1, 4.2, 4.4, 4.10; move the structural part of 5.3 into this gate.

---

**3. High — S determines whether G-C is plausible; neither F nor the failed tree currently rescues small batches. Focus point 2.**

The qualification traces are recorded as fact(3) **293 steps** and list123 **303 steps**. Using the requested node range of **61–67 seconds** and a rounded **70-second decider**, the following are **node-plus-decider projections only**:

\[
L=\lceil H/S\rceil,\qquad T_{\text{node-only}}\approx L(61\ldots67)+70.
\]

They exclude leaf proving, setup, independent verification, call validation and contention. The underlying timing evidence uses stand-in leaves. (`tasks.md:81`; `qa/sherlock/decisions/speed-levers.json:32`, `:43`, `:99–109`.)

| S | Maximum k_U under the proposed sizing rule | fact(3): folds; projected seconds | list123: folds; projected seconds |
|---:|---:|---:|---:|
| 1 | 20 | 293; **17,943–19,701** | 303; **18,553–20,371** |
| 2 | 19 | 147; **9,037–9,919** | 152; **9,342–10,254** |
| 4 | 18 | 74; **4,584–5,028** | 76; **4,706–5,162** |
| 8 | 17 | 37; **2,327–2,549** | 38; **2,388–2,616** |
| 16 | 16 | 19; **1,229–1,343** | 19; **1,229–1,343** |

The sizing rule is from `design.md:107`; every time in the table is an estimate calculated from the cited timing records.

Thus:

- **k_U=19–20:** S=2 or S=1 already misses 7,200 seconds under this cost model before leaf proving.
- **k_U=18:** S=4 is plausible but has limited margin.
- **k_U≤17:** S≥8 provides substantially more room, subject to actual leaf cost and downstream circuit size.

For example, list123 at S=4 and a 67-second node takes `76 × 67 + 70 = 5,162` seconds. The design permits up to 1.5× node slowdown under contention; at that slowdown, the projection becomes `76 × 67 × 1.5 + 70 = 7,708` seconds. The allowed contention threshold can therefore defeat G-C even before leaf work is included. (`design.md:108`.)

F at S=1 would need a node below approximately **23.5 seconds** for list123 merely to fit node work and the decider: `(7,200−70)/303`. That is a calculated requirement, not a measured capability.

The tree is not an available remedy. Its proposed three-slot relation already failed the row gate. Moreover, f3≈1.7 was measured using three six-thread genesis node provers, with individual times around 128–132 seconds; it is not evidence that the serial chain can be divided by three. (`tasks.md:35–36`.)

As an **illustrative scheduling estimate**, assigning the recorded 130.68-second concurrent-node time to an S=1 tree would require at least `ceil(293/3)=98` or `ceil(303/3)=101` three-worker waves: approximately **3.58 and 3.69 hours**, including the rounded decider but excluding dependencies and leaf work. This is not a tree benchmark; the actual proposed tree also has a larger domain. (`wiki-llm/recursion-perf/profiles/c0-first-profile.md:57`; `tasks.md:36`.)

Memory remains conditional too. An illustrative `17.6 GB node + 3 × 16 GB leaves = 65.6 GB` fits the nominal aggregate target, but the 16 GB universal-leaf assumption is unmeasured, and the decision record reports a lower operational `heavy.slice` cap of 72G. (`qa/sherlock/decisions/speed-levers.json:36`, `:70`; `tasks.md:68`.)

**Proposed change:** use an end-to-end pipeline estimate with ceilings, measured mixed-workload contention, setup/drain costs and verification. Keep S≥8 as the preferred candidate, S=4 conditional, and S≤2 as a redesign trigger at current node cost.

**Tasks:** 4.10, 5.1, 5.3, 5.5, 5.6, 5.9 and 5.10. Leave 5.7 closed.

---

**4. High — the formal results support composition, not complete circuit soundness. Focus point 4.**

The 90-theorem source at `bd7703f4d`, the recorded F20 result and the separate IVC06 model provide useful evidence. Their scope must remain explicit. The Lean development assumes leaf soundness, digest injectivity over admitted domains and accumulator soundness. In particular, `valid_accumulate` assumes that a folded accumulator is valid iff every input is valid. That is an ideal cryptographic premise, not a theorem about the implemented transcript. (`bd7703f4d`; `tasks.md:40`; `clean/fork/Clean/Sherlock/Aggregation.lean:175–213`, `:1230–1244`.)

The current node does match the revised genesis construction at the relevant interface: neutral carry at genesis, scalar-one collapsed carry at every height, bounded heights, and collapse after scaling the predecessor proof. This is a reason to keep the implementation rather than reopen the earlier genesis alternatives. (`crates/mori-recursion/src/ivc.rs:363`, `:383`, `:409`, `:531`; `clean/fork/Clean/Sherlock/Aggregation.lean:1248`.)

There are nevertheless model-to-leaf gaps:

- Lean’s `Leaf` describes boundary fields; it does not define selectors, rule-digest encoding or terminal-publication slots. (`clean/fork/Clean/Sherlock/Aggregation.lean:35`.)
- Quint models synthetic programs whose digests are `(program, step index)`. Its slot model checks prefix padding, links and enabled counts, but has no explicit publication event or `rules_d`. (`models/sherlock/quint/aggregation.qnt:87–118`.)
- The F20 check requires S=1,2,4,8 coverage, not the provisional S=16. (`scripts/qa/check-f20.sh:28–30`.)
- The IVC06 toy model forces a zero challenge to one; its report correctly labels its results as model evidence, not circuit evidence. (`qa/gemini-quint/ivc06/ivc06.qnt:124–130`; `qa/gemini-quint/ivc06/REPORT.md:194–198`.)

**Proposed change:** add a concrete leaf-slot refinement covering selection, publication, padding, counting and digest encoding. Add S=16 to finite model coverage. State the cryptographic soundness assumptions and exceptional-challenge treatment separately from algebraic identities and finite mutation results.

**Tasks:** extend 2.11/2.12; make the leaf refinement a prerequisite of 4.2 and 4.6; retain real-trace replay in 5.4.

---

**5. High — “general-purpose recursion” currently means Sherlock execution, not arbitrary proof aggregation. Focus point 3.**

The abstract composition theorem is general over a supplied machine relation. The implementation is specific: one leaf VK is fixed in the node; linked segments share one `prog_d`; the decider opens a Sherlock halted state with a u128 result. The contract additionally seals one program/init pair. (`clean/fork/Clean/Sherlock/Aggregation.lean:183`, `:49`; `crates/mori-recursion/src/ivc.rs:548`; `crates/mori-recursion/src/run_decider.rs:278`; `tools/sherlock-sdk/sherlock-run.compact:25`.)

This can serve many Sherlock programs with one key set **once the universal leaf exists**, within fixed bounds, layout, inventory and S. It does not establish support for arbitrary external circuits, unrelated statements, or aggregation of different programs into one execution chain. Such support would need authenticated circuit identity, a permitted-key policy or universal verifier, statement adapters and defined composition semantics.

The claim that linear and tree implementations have the same “statement, decider and `acceptRun`” is also too broad. Task 5.7 explicitly requires a different tree node, root-boundary checks and tree verification. The final application values may remain the same; the recursive statements and dependent keys do not remain identical. (`design.md:79`; `tasks.md:83`.)

**Proposed change:** define three distinct scopes: admitted Sherlock programs; alternative aggregation shapes for Sherlock segments; arbitrary-circuit aggregation as a separate extension. State that a deployed `acceptRun` instance remains program/init-specific even when its verifier key is reusable.

**Tasks:** clarify SRC-017 and 4.4; add cross-program key-reuse evidence to 5.1/5.2. Do not silently expand 5.7 into arbitrary-circuit support.

---

**6. High — the required independent GP verifier and complete proof bundle are missing from the named path. Focus point 4; additional finding.**

SRC-010 requires another process to verify exact leaf, node and decider bytes, reconstruct accumulators and check every link. The named `verify_chain` instead loads the historical A102/A80 pair and asserts statement lengths `(419,419,28)`. (`requirements.md:32`; `crates/mori-recursion/examples/verify_chain.rs:89`, `:134`.)

The GP harness verifies nodes during proving but writes only the final node proof. Its final ledger check invokes `VerifierKey::verify`, not full `ContractCall::validate_call`. These are valuable checks, but they do not satisfy the separately specified independent-bundle and transaction-validation obligations. (`crates/mori-recursion/tests/sherlock_gp.rs:246`, `:287–291`, `:329–336`; `tasks.md:77`.)

**Proposed change:** explicitly implement a versioned GP bundle and independent verifier. Preserve every intermediate node artifact required by SRC-010, bind ordering and statements, reconstruct accumulators, and distinguish proof verification from full local call validation.

**Tasks:** split 5.1 into bundle production, independent verification and ledger-call validation; coordinate with 2.5. Do not tick this work through the legacy verifier.

---

**7. High — task ordering contains dependency cycles and freezes interfaces too late. Focus point 6.**

Task 4.1 requires a universal row probe over frozen R; task 4.2 builds that relation; task 4.4 freezes R. Task 4.10 chooses S using leaf timing/RSS before task 4.4 measures them. The program identity, seven-lane bridge and initialization work appear after the keying step. (`tasks.md:64–72`.)

“One cascade” is a reasonable goal for final keys, but it cannot eliminate exploratory measurements. Choosing S from incomplete fragments or a stand-in VK merely moves the redesign into M5. The difference between stand-in and C6-shaped node rows already demonstrates this risk. (`tasks.md:68`, `:78`; `qa/sherlock/manifests/node.json:22`; `qa/sherlock/decisions/speed-levers.json:22`.)

**Proposed change:** distinguish unkeyed construction, exploratory measurements, final interface freeze and final key generation. Move initialization, statement encoding and selector-model obligations before final keying. Keep expensive qualification after the resulting candidate passes structural checks.

**Tasks:** reorder 3.x, 4.1–4.10 and the structural part of 5.3; split 2.5 and 5.1 where necessary.

---

**8. Medium — ledger 9 appears compatible, but the design names the wrong limits and overstates current acceptance evidence. Focus point 5.**

At ledger commit **`a2b30128`**:

| Item | Actual source rule | Consequence |
|---|---|---|
| Public inputs | `MAX_CALL_PUBLIC_INPUTS = 1 << 16` | A source constant, not a `LedgerParameters` field. (`onchain-runtime/src/transcript.rs:87`.) |
| Contract operations | `MAX_CONTRACT_OPERATIONS = 256` | Limits operations stored by a contract, not Sherlock execution steps. (`ledger/src/structure.rs:1926`; `ledger/src/verify.rs:1836`.) |
| Verifier key bytes | Deserializer rejects a raw VK buffer above 50,000 bytes | This is the relevant explicit VK-size guard. (`transient-crypto/src/proofs.rs:417–435`.) |
| Transaction size and costs | `LedgerParameters.limits` contains transaction bytes, block limits and metadata limits | Read actual target parameters; do not substitute source defaults. (`ledger/src/structure.rs:1265–1307`.) |
| Initial transaction-byte limit | 1 MiB in `INITIAL_LIMITS` | A default, not evidence of the live network value. (`ledger/src/structure.rs:1283`.) |

The manifest’s 2,217-byte tagged decider key and the design’s 70-slot template are comfortably below the cited size/input caps. But the template manifest explicitly says it predates the repin and has an unobserved compiler version. This establishes no full-transaction acceptance result for the final universal-leaf chain. (`qa/sherlock/manifests/sherlock-run-contract.json:2`, `:10–13`; `design.md:91`.)

The `VERIFIER_KEY_SIZE = 2875` constant elsewhere in the ledger is an estimated size, not the deserialization cap. It must not become an accidental acceptance rule. (`a2b30128:ledger/src/structure.rs:1922–1924`.)

The recorded network observation also contradicts the design’s Preprod status. It reports all three networks on node `2.1.101-913ec011`, spec 2001100 and ledger 9.1.0.1; this is a saved observation, not a live check performed for this review. (`research/midnight-ledger9-20261107/api/rpc-status-20261110.json:2–25`; `design.md:142`.)

**Proposed change:** correct SMN-003, recapture the template, measure the complete serialized transaction, and run full call validation against recorded target parameters. Preserve the distinction between local source provenance and observed deployed runtime identity.

**Tasks:** 2.1 status reconciliation, 2.5, 5.1 and Preview qualification.

---

**9. Medium — the GPU adoption argument double-counts work. Additional finding.**

The decision record uses `m + q = 0.256 + 0.57 = 0.83`. The profiler explicitly says q overlaps m and that they must not be summed. It defines q from vanishing, quotient, NTT and advice-NTT spans; therefore q is not the fraction directly removable by an FFT implementation. The q value also comes from a different diagnostic run than the main m measurement. (`qa/sherlock/decisions/speed-levers.json:31`, `:67`; `scripts/qa/profile-recursion.py:868–895`.)

Using m=0.256 alone gives an **idealized MSM-only speedup ceiling** of `1/(1−0.256) ≈ 1.34×`, before transfer and integration overhead. That estimate cannot rescue an S=1 chain requiring roughly 2.6× faster nodes. (`qa/sherlock/decisions/speed-levers.json:9`; arithmetic in concern 3.)

**Proposed change:** base GPU adoption on exclusive, directly offloadable phase shares from one qualified run and measured whole-chain benefit. Keep proof-byte equivalence, CPU fallback and memory accounting. Give the GPU no credit in G-C until measured.

**Tasks:** 2.8, 2.10b–2.10f, 4.10 and 5.8.

## 5. Proposed edits to design.md and tasks.md

These are proposed text changes only.

**A. Replace the scope claim in design §3.6 and clarify SRC-017.**  
Affected text: `design.md:116`; `requirements.md:39`.

> Sherlock provides recursive proofs of executions of the admitted bounded machine. For fixed rule inventory, carrier layout, bounds, profile and batch size S, the leaf, linear-node and decider keys do not depend on the program, initial state or execution length. The deployed acceptRun instance binds one program and initial-state digest.
>
> The segment model also describes tree composition. A tree implementation has its own recursive statement and dependent keys. Arbitrary-circuit aggregation is a separate extension requiring authenticated circuit identity, statement adapters and a composition policy.

**B. Replace the U/F decision and S gate in design §§3.4–3.5.**  
Affected text: `design.md:95`, `:107`, `:110`; `tasks.md:65`, `:68`.

> Select the leaf construction and S from measurements of the complete admitted relation and its dependent node and decider. Treat `k_U + log2(S) <= 20` as an initial sizing estimate; confirm actual rows and architecture at each candidate S.
>
> U remains eligible at k_U=18 if S=4 passes the complete path’s cost and memory gates. F at S=1 is a correctness fallback, not a demonstrated G-C fallback. If neither construction meets G-C, record a design failure requiring a new batching or recursion construction.
>
> The current three-slot tree candidate is closed. A chain shortfall does not override its failed row gate. Reopening requires a changed candidate that passes new structural and concurrency measurements.

**C. Complete the statement and initialization contract before key generation.**  
Affected text: `design.md:42`, `:54`, `:86`, `:91`; `tasks.md:67–72`.

> The leaf derives outcome, result, selector digest and enabled-step count from constrained machine transitions. Its program digest commits to the image, layout schema, bounds and profile. Define one rule-digest encoding, including its domain, active count, padding convention and treatment of a single enabled step.
>
> Canonical initialization is established by the first-leaf relation or an explicitly authenticated initialization mechanism. Equality to a supplied init_d alone is insufficient.
>
> The final node step bound is explicit in the serialized relation identity. A bound change changes the relation version or serialized parameters and triggers the dependent key cascade.

**D. Replace the performance baseline and adoption formula.**  
Affected text: `design.md:79`, `:99`, `:106–112`, `:159`.

> Distinguish stand-in measurements from measurements using the final leaf VK. Record leaf, node and decider rows together.
>
> Use `L = ceil(H/S)`. Project total wall from setup, the slower of node service and aggregate leaf service, pipeline fill/drain, decider proving, independent verification and local call validation. Measure service times under the proposed mixed workload. Record both process and aggregate memory.
>
> At the recorded node cost, S≤2 does not meet G-C for the qualification traces. S=4 is conditional; S≥8 is the preferred candidate. k=19, tree and GPU improvements receive no projected credit without passing evidence.

**E. Correct the formal-assurance wording.**  
Affected text: `design.md:85`, `:118`; tasks 2.11, 2.12, 4.2 and 5.4.

> Composition theorems are conditional on canonical initialization, leaf soundness, digest binding and cryptographic accumulator/verifier soundness. Finite models and mutations check their modeled interfaces; they do not discharge those cryptographic premises.
>
> Add an explicit refinement from constrained S-slot execution to the abstract Leaf claim. Cover selectors, publication, padding, enabled counts, internal links and rule-digest encoding. Include S=16 in the finite model suite. Keep concrete-chain replay as a later check.

**F. Correct ledger §5 and SMN-003.**  
Affected text: `design.md:142`, `:145`, `:152`; `requirements.md:86`; task 2.5.

> At ledger a2b30128, the public-input cap is the source constant 65,536 and the contract-operation cap is the source constant 256. Transaction-byte, block-cost and metadata limits come from the target LedgerParameters. Check the raw verifier-key deserialization limit separately.
>
> Recapture the acceptRun template with the pinned compiler. Record compiler provenance, actual public-input count, verifier-key bytes, full serialized transaction size and full local validation results.
>
> Replace obsolete workspace and network descriptions with dated observations, distinguishing deployed runtime identity from locally available source.

**G. Reorder tasks.md around concrete dependencies.**  
Affected text: `tasks.md:43`, `:64–84`.

1. **M3:** finish dispatch partition and inventory reconciliation; freeze the candidate rule set and per-rule premises before measuring U.
2. **Early M4:** settle program identity, seven-lane encoding, initialization and the leaf-slot refinement. Move the selector model here.
3. **4.2a:** build the parameterized, unkeyed universal relation needed by the row probe.
4. **4.1:** measure complete candidate leaves and downstream node/decider architectures; include candidate S values.
5. **Exploratory measurement:** obtain the timing and memory evidence needed by G-A′. Label exploratory keys separately from final qualification artifacts.
6. **4.10:** choose U/F, S and worker count from the complete cost model.
7. **4.4:** freeze the final profile and generate the dependent key set. Complete boundary negatives and digest parity against those keys.
8. **5.1a:** define and emit the complete GP proof bundle.
9. **5.1b:** implement independent GP verification; do not route this obligation through the A102/A80 verifier.
10. **5.1c:** perform full local `validate_call` using the recaptured template.
11. **5.3 onward:** measure mixed node/leaf execution, pass G-B, then run fact(3)/list123 and G-C.
12. Keep the current tree closed and GPU work optional. Reopen either only with new evidence relevant to the final chain.