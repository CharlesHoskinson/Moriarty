**A. Errors**

References: `D/` means `openspec/changes/sherlock-vm-consensus-20261107/`. `F:` means [fable/review-1.md](/home/hoskinson/Projects/Moriarty/research/sherlock-recursion-design-review-20261110/fable/review-1.md). Ledger references below use **`a2b30128ca4b660b73a068e29b56b48ede8c9a74`**, the commit resolved by `ledger-9.1.0.1`, rather than the clone’s working tree.

1. **`MAX_K = 19` is a real implementation limit, but Fable overstates its scope.** The existing C6 digest prover rejects an options cap above 19, then rejects a circuit exceeding that cap. Fable cites the raw-child path; the directly relevant digest checks are at `crates/mori-backend/src/sherlock_c6_child.rs:868` and `:880`. Thus a k=20 leaf cannot use this API unchanged. However, the universal relation and its prover integration remain unfinished tasks. This constant does not establish an upstream k=19 limit or invalidate the proposed k=20 construction. Fable’s downward threshold shift is conditional on retaining the current wrapper unchanged. (`crates/mori-backend/src/proving.rs:22`; `D/tasks.md:66`, `:68`, `:69`; F:33,112–114,269–272.)

2. **The ledger’s “2^14 setup” comment is technically misleading; k=20 verification does not depend on violating a KZG parameter limit.** The comment exists exactly where Fable says. But the ledger converts the embedded prover parameters into verifier parameters. Those contain the G2 verification elements, with no k or G1-power-array length. Their construction discards the proving-domain size. Larger circuits using the same setup can therefore use these verifier parameters. The comment merits correction; it does not establish a cryptographic incompatibility or evidence of a forthcoming k cap. A future policy cap remains speculation. (`a2b30128:transient-crypto/src/proofs.rs:114`, `:121`, `:563`; `a2b30128:onchain-runtime/src/transcript.rs:80`; `tools/profile/vendor/midnight-proofs-0.8.3/src/poly/kzg/params.rs:235`, `:273`; F:208–214.)

   The other cited ledger rules check out: 65,536 public inputs, enforcement in `verify.rs`, 256 contract operations and a 50,000-byte **raw VK buffer** limit. The recorded k=20 verification success is also real evidence, although it concerns the stand-in chain, not the final universal-leaf chain. (`a2b30128:onchain-runtime/src/transcript.rs:87`; `a2b30128:ledger/src/verify.rs:2138`; `a2b30128:ledger/src/structure.rs:1928`; `a2b30128:transient-crypto/src/proofs.rs:422`; `qa/sherlock/decisions/speed-levers.json:95`.)

3. **The `IrSource::model()` citation is wrong.** At the specified tag, `zkir-v3/src/ir.rs:810–822` contains instruction definitions. `Model::k`, `Model::rows` and `IrSource::model` are at lines **936, 941 and 948**. Also, task 3.1 does not currently promise k measurements: it specifies build time, preprocessing time and instruction count. Adding rows and k would amend it. (`a2b30128:zkir-v3/src/ir.rs:809`, `:934`; `D/tasks.md:56`; F:62–74.)

4. **The evidence does not establish “one rule circuit is probably near k=17–18.”** The pilot records roughly 2.2–2.5 million reference instructions for the particular publication and call/bind cases shown. It does not characterize every rule. Its 4,255-lane figure describes one layout involved in a layout-mismatch failure. The scalar-failure test explicitly requests k=18; that is not a minimum-k measurement, and its fixture uses a different, small configuration. Advice-cell counts are not row counts. (`research/skeleton-coding-20261108/pilot-smoke.md:28`, `:43`; `crates/mori-backend/src/sherlock_machine/c6_scalar_failure_step.rs:1849`, `:2232`; F:52–68.)

   The dispatch claim itself is correct: every pre-image and helper lane receives a 128-bit constraint, and the pinned ZKIR implementation expands `ConstrainBits` through `assigned_to_le_bits`. This identifies a cost worth measuring, without determining k. The approximately 47 GB figure concerns cached dispatch circuits in the test harness, not a universal prover’s RSS. Its citation should be `testing.md:21`, not `:20`. (`crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:508`, `:612`; `a2b30128:zkir-v3/src/ir_vm.rs:844`; `research/skeleton-coding-20261108/testing.md:21`.)

5. **The proposed subtraction formula is not an established lower bound.** Fable proposes:

   `rows(shared dispatch) + Σ(rows(rule circuit) − rows(shared dispatch))`.

   The cost model reports row extents, not an additive instruction-cost measure. It derives required domain size from the maximum of ordinary rows, table rows, instance rows and minimum circuit requirements. Standalone circuits also repeat allocations and checks that a shared relation can reorganize. Conversely, selectors, gating, candidate selection and digesting add work. Without a construction-specific argument, subtraction and addition of these totals produce an estimate, not a bound that can decide G-A′. The expression also computes rows, not the exponent `k_U`. (`tools/profile/vendor/midnight-proofs-0.8.3/src/dev/cost_model.rs:341`, `:373`; `D/design.md:35`, `:39`, `:40`; F:70–76,295–300.)

6. **The 741,708-versus-652,437 comparison is valid; its extrapolation is not established.** Both are ordinary `cost_model` row counts. A11 records `m.rows`; the profile records that same field and checks its instrumented copy against the original relation. The difference is **89,271 rows, or 13.68%**. (`crates/mori-recursion/tests/sherlock_keygen_regression.rs:81`, `:132`; `tools/profile/src/node_rows.rs:230`, `:248`, `:263`; `qa/sherlock/manifests/node.json:22`; `qa/sherlock/decisions/speed-levers.json:22`.)

   Three qualifications are missing:

   - The C6 key is a small digest-bridge fixture. Its producer has six public lanes, while the keyed node supplies seven. This is structural keygen evidence, not a working C6 chain benchmark. (`qa/sherlock/manifests/c6-leaf.json:26`, `:27`; `crates/mori-recursion/src/ivc.rs:549`.)
   - More universal-leaf instructions do not necessarily mean more chips, columns or commitments. ZKIR chooses architecture from operation/type presence and fixes four range columns. The claim that the architecture **will** widen is unsupported. (`a2b30128:zkir-v3/src/ir_vm.rs:1212`, `:1235`, `:1247`; F:137–139.)
   - **306,868** is ordinary-row headroom below `2^20`, not a complete fit calculation. Tables and unusable rows also matter. Neither doubled runtime nor exceeding 48 GB follows automatically from k=21. Even doubling the recorded 16.58 GB yields only 33.16 GB—an illustrative estimate, not a k=21 measurement. A16’s performance target is 16 GB, not 48 GB. (`tools/profile/vendor/midnight-proofs-0.8.3/src/dev/cost_model.rs:373`; `D/tasks.md:44`; F:139–140.)

7. **“Exactly one run” confuses a conditional uniqueness result with contract capacity and existence.** The contract rejects an already-recorded `stmt`; it has no one-element capacity check. Under sound leaves, an authenticated initial state, determinism and terminal halted states, Lean proves uniqueness of the final step count, digest and result. That supports **at most one valid accepted statement** for a fixed pair. It does not establish that a halted execution exists. The decider hashes those values into `stmt`, independently of proof randomness. (`tools/sherlock-sdk/sherlock-run.compact:27`, `:40`; `clean/fork/Clean/Sherlock/Aggregation.lean:1136`; `crates/mori-recursion/src/run_decider.rs:290`; F:163–165.)

8. **`h_pre` already has a stated purpose.** The current leaf does not derive it internally, but the node supplies its bounded predecessor height as that public input. Calling it unused or simply free loses the division of responsibility between relations. It can additionally select a first-leaf initialization check. (`D/design.md:42`; `crates/mori-recursion/src/ivc.rs:497`, `:549`; F:35,185–192.)

9. **The initialization discussion needs two corrections.** The constructor does check that `init_d` is nonzero; it does not check canonical initialization. More substantially, the contract does not “already imply” off-chain recomputation. Its executable checks only pin the supplied digests. The named `verify_chain` still follows the historical A102/A80 path. Off-chain initialization verification would be new functionality and a revised assurance statement. Also, canonical encoding alone is insufficient: initialization must establish the correct starting control, environment, counters and configuration for the program. (`tools/sherlock-sdk/sherlock-run.compact:29`; `crates/mori-recursion/examples/verify_chain.rs:89`, `:134`; `crates/mori-core/src/sherlock/reference.rs:14`; `D/assurance.md:11`; F:186–197.)

10. **Several performance conclusions turn estimates into exclusions.** Three concurrent genesis-node provers do not establish an upper bound for a node running beside two universal-leaf workers. Their slowdown uses a six-thread solo baseline. Nor does failure of the schedules examined prove that S=4 can pass only with a roughly 35-second leaf. The table labelled “node chain alone” includes the 70-second decider. A leaves-first schedule needs `ceil(76/3)` waves, not `76/3`. Finally, a 7,138-second proof-only estimate does not establish a complete G-C pass. Details follow in B1. (`wiki-llm/recursion-perf/profiles/c0-first-profile.md:57`; `D/requirements.md:33`; F:81–109,271–272.)

11. **“Leaf cost is linear in state size” needs a narrower noun.** The current full-carrier hash has linear work in the number of lanes. That does not establish linear total circuit cost, proving time or RSS. Nor does every increase in Bounds immediately increase k, which changes at domain thresholds. The fixed-capacity concern is valid. (`crates/mori-backend/src/sherlock_c6_child.rs:505`, `:551`; `tools/profile/vendor/midnight-proofs-0.8.3/src/dev/cost_model.rs:373`; F:157–159.)

12. **Runtime inputs are not merely a contract/decider-template change.** A new input convention must define admission, canonical initialization, commitment binding and witness production. The current stream takes source and configuration, then constructs genesis; it has no separate run-input argument. A registry-only contract change may be narrower, but Fable’s combined claim that both extensions are template-only and cheap is unsupported. The cited “qualification config … in the tens” passage is at `stagenet-gp-recursion.md:162–163`, not `:152–154`. (`crates/mori-backend/src/sherlock_c6_witness_stream.rs:170`; `D/assurance.md:11`; F:153–155,174–179.)

13. **Task 3.4 does not mean twenty entirely new fragments.** It requires fragments **and receipts**. Publication, validation, equality and several match fragments already have source entries; five listed match rules remain gaps. The receipt list is empty. The work should distinguish missing constraints from missing evidence. (`D/tasks.md:59`; `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:121`, `:474`, `:503`, `:559`, `:593`, `:638`; F:240–241.)

14. **The proposed task closures overstate what is known.** A recorded overhead bound of 1.017 does not satisfy the stated 1.01 qualification criterion. It can support a proposal to change or retire that criterion, not a claim that it passed or no longer affects evidence. Likewise, a three-rule prototype does not establish the full leaf’s timing, memory or G-B behavior. (`D/tasks.md:34`, `:65`, `:78`, `:80`; F:244–250,308.)

15. **Two broad statements require qualification.** “Sound as designed, built and cheap enough” exceeds the evidence for the complete run claim: the formal development assumes leaf and cryptographic soundness, and the measured throughput chain uses stand-ins. The proposed “every step count below `2^64`” scope also needs a positive lower bound: the node requires at least one step. (`clean/fork/Clean/Sherlock/Aggregation.lean:198`, `:211`; `qa/sherlock/decisions/speed-levers.json:99`; `crates/mori-recursion/src/ivc.rs:359`; F:12–18,279–283.)

**B. Disagreements**

**1. S=4 at k_U=18**

Fable is right about the planning decision: **the available estimates do not support selecting S=4 for G-C**. My “plausible but limited margin” wording was too favorable. Fable has not proved that every possible mixed schedule fails.

For list123, `ceil(303/4)=76`. Using a rounded 70-second decider gives these **estimates**, excluding setup, verification and call validation:

| Solo node time | Nodes + decider | Maximum serial leaf time | Total with 94-second leaves |
|---:|---:|---:|---:|
| 61 s | 4,706 s | less than 32.82 s | 11,850 s |
| 67 s | 5,162 s | less than 26.82 s | 12,306 s |

The trace length and timing inputs come from `D/tasks.md:81`, `qa/sherlock/decisions/speed-levers.json:32`, `:43`, `:107`. The 94-second leaf assumption comes from `research/sherlock-speed-replan-20261108/prover.md:33–41`.

Under Fable’s **130-second concurrent-proof assumption**, leaves first require `26 × 130 = 3,380` seconds. Adding the faster solo chain gives **8,086 seconds**, rather than 7,999. But this schedule would require concurrent leaf time below approximately **95.9 seconds**, not necessarily solo leaf time below 35 seconds. An overlapping schedule instead needs measured node service below roughly `(7,200−70)/76 = 93.8` seconds, with adequate leaf supply and allowance for startup and completion. These are necessary scheduling budgets, not evidence of achievable performance. (`wiki-llm/recursion-perf/profiles/c0-first-profile.md:57`; F:100–104.)

**I would defend no measured k=20 universal-leaf time.** For planning, I would retain approximately **94 seconds nominal and 315 seconds as a stress estimate**, from the source’s 0.09–0.3 ms-per-row assumptions. Neither is a guaranteed bound. I would not defend a 27–33-second leaf without measurements. (`research/sherlock-speed-replan-20261108/prover.md:33`, `:39`.)

The omitted costs matter. The A16 process took 464.416 seconds, while its three node proofs and decider totalled 253.31 seconds. The remaining **211.106 seconds** cannot simply be transferred to another workload, but demonstrates why 62 seconds of proof-only margin proves little. Moreover, Fable’s `311-second leaf` estimate gives `20×(61+311)+70 = 7,510` seconds at the requirement’s 320-step limit. (`qa/sherlock/decisions/speed-levers.json:102–109`; `D/requirements.md:33`.)

**2. Option F**

I withdraw **“a useful correctness fallback” as a recommendation for the current deliverable**. Defined F fixes S=1. At the recorded node cost, its node chain alone exceeds the mandatory target. It cannot rescue qualification merely by being semantically sound. (`D/requirements.md:33`, `:35`; `qa/sherlock/decisions/speed-levers.json:43`.)

F can survive as a diagnostic construction, a short-run test vehicle or a starting point for a separately reviewed batching design. Those uses do not make it a fallback for this milestone. I agree with marking the current F branch **defined, unqualified and not planned for satisfying SRC-011**. (`D/design.md:95`.)

**3. Early lower-bound probe**

**An early diagnostic probe is worthwhile. Fable’s claimed lower bound is not yet valid.** I would move measurements of existing circuits and shared dispatch ahead of further broad fragment expansion. This changes my proposed work order. Task 4.1 already asks for representative fragment rows and k, so this is a useful split of existing work. (`D/tasks.md:64`; `research/sherlock-recursion-design-review-20261110/astra/review-1.md:255–259`.)

Record the exact configuration, circuit composition, architecture, ordinary rows, tables and minimum k. Call the summed/subtracted result a projection until its additivity is justified. It becomes misleading when:

- layouts differ between circuits;
- standalone allocation and canonicality work is counted repeatedly;
- separately optimized architectures or k values are mixed;
- selector, gating, hashing or initialization costs are omitted;
- a three-rule subset is treated as the complete frozen inventory.

The pilot already demonstrates differing layouts. The design explicitly changes dispatch sharing and candidate-post selection. The cost model separately accounts for tables and instance size. (`research/skeleton-coding-20261108/pilot-smoke.md:113–121`; `D/design.md:35–44`; `tools/profile/vendor/midnight-proofs-0.8.3/src/dev/cost_model.rs:373`.)

A small, actual shared relation is the next useful measurement. It can reveal whether the proposed emission strategy is viable before all fragments are adapted. It cannot close the full-inventory gate. Soundness remains my highest-consequence concern; that does not require postponing a cheap feasibility measurement.

**4. Off-chain canonical genesis**

**Off-chain verification can be sound for the verifier that performs it. It gives a weaker on-chain claim unless initialization is authenticated for on-chain consumers.**

An independent verifier can obtain the committed program and configuration, construct the canonical initial carriers, check their validity and recompute both expected digests. If those match the sealed values, it supplies the initial-state premise required by the composition argument. This relies on correct admission, digest binding and leaf soundness. Program image alone is insufficient unless all other initialization choices are fixed or derivable: the reference constructor takes fuel and stack limit, and the current witness path takes configuration. (`clean/fork/Clean/Sherlock/Aggregation.lean:211`, `:1116`; `crates/mori-core/src/sherlock/reference.rs:14`; `crates/mori-backend/src/sherlock_c6_witness_stream.rs:170`.)

Precisely:

- A consumer that recomputes and checks the canonical initialization cannot be fooled by a deployer merely sealing another digest, under those premises.
- A reader or downstream contract that treats any `acceptRun` event as proof of canonical program execution can be fooled about that stronger claim. The constructor permits arbitrary nonzero digests.
- A malicious prover cannot replace an already authenticated, correctly pinned `init_d` merely by supplying another one; the contract checks equality.  
  (`tools/sherlock-sdk/sherlock-run.compact:29`, `:41`; `clean/fork/Clean/Sherlock/Aggregation.lean:1100`.)

For an arbitrary unauthenticated `init_d`, conditional leaf soundness may not establish even a reference execution: a canonical pre-state need not exist. Thus “a valid execution relative to whatever init was supplied” is also too strong. (`clean/fork/Clean/Sherlock/Aggregation.lean:204–213`.)

I accept off-chain recomputation as a possible revised product guarantee. It does **not** satisfy the current requirement that forged initialization be refused at the leaf/node boundary. Choosing it requires explicitly changing O1 and task 4.9. (`D/assurance.md:11`; `D/tasks.md:72`.)

**5. Formal work**

We can reconcile most of this. Pause additional generic composition results that do not answer an implementation question. Continue the small leaf-facing specification before implementing 4.2.

Fable’s own proposed 4.2a already asks for a slot contract and a publication-aware Quint model. That is substantially the work I want. The remaining disagreement is whether Lean must connect that slot contract to its boundary `Leaf` before the full leaf implementation. I still favor that refinement: otherwise “slot semantics equal the Lean Leaf” remains unsupported. The existing Lean structure has boundary fields only; Quint checks prefix padding and links but has no explicit publication field in those slots. (`F:232–235`; `clean/fork/Clean/Sherlock/Aggregation.lean:35`; `models/sherlock/quint/aggregation.qnt:99–118`; `D/tasks.md:66`.)

This refinement need not block exploratory row measurements. An acceptable alternative is to revise the assurance claim explicitly: Lean proves boundary composition, while Quint specifies slots, with the implementation correspondence still open. The previous review record identifies that alternative. (`research/sherlock-speed-replan-20261108/council-review.md:387–394`.)

**6. General-purpose scope**

- **Fixed Bounds and key identity: correct and material.** The promised reuse is within a frozen profile, layout and capacities. Exceeding those capacities requires a different relation/key closure. This bounds the generality claim without invalidating recursion over many programs. (`D/design.md:54–56`; `D/tasks.md:68`.)
- **State-size cost: materially correct, overstated as total linear cost.** Full-state hashing scales with carrier length. Large-state efficiency remains an architectural concern, but k and total proving cost do not increase linearly in every capacity parameter. (`crates/mori-backend/src/sherlock_c6_child.rs:505`, `:551`; cost-model source cited in A11.)
- **No Rejected/Exhausted acceptance: correct.** The decider requires a halted u128 opening. The witness stream also rejects non-UInt halted results and other terminal outcomes. This matters if “general purpose” includes proving failed executions or returning structured values. (`crates/mori-recursion/src/run_decider.rs:278–288`; `crates/mori-backend/src/sherlock_c6_witness_stream.rs:138–158`.)
- **No separate runtime-input interface: correct for the current path.** Programs can contain values in their source, but that is different from repeatedly executing one committed program with independent run inputs. This matters for reusable applications, though it does not invalidate recursion itself. (`crates/mori-backend/src/sherlock_c6_witness_stream.rs:170–177`; `tools/sherlock-sdk/sherlock-run.compact:25–26`.)

I would ask the owner to resolve these scope choices before deployment, without silently adding foreign-circuit aggregation to the current milestone.

**C. What Fable missed, and changes to my round 1**

I still hold these additional concerns:

- The named independent verifier remains the historical A102/A80 verifier, and the GP harness saves only the final node proof. This does not satisfy the specified independent verification of every exact artifact. (`crates/mori-recursion/examples/verify_chain.rs:89`, `:134`; `crates/mori-recursion/tests/sherlock_gp.rs:288`; `D/requirements.md:32`.)
- The GPU argument sums overlapping shares. The profiler explicitly forbids `m+q`, and q includes quotient work. Fable repeats the funding premise without correcting it. (`scripts/qa/profile-recursion.py:868–877`; `qa/sherlock/decisions/speed-levers.json:67`; F:40.)
- `S_MAX` affects constraints but is absent as an explicit serialized parameter. A changed bound needs an unambiguous relation-version or serialization change. F20’s coverage check also stops at S=8. (`crates/mori-recursion/src/ivc.rs:77`, `:372`, `:571`; `scripts/qa/check-f20.sh:29`.)

I change four round-1 positions: withdraw the favorable S=4 planning language; withdraw F as a deliverable fallback; move diagnostic row measurements earlier; and qualify “headroom” as ordinary-row headroom rather than guaranteed capacity.

I also correct my description of `decider_pinning`: it assumes **a Carrier state opening to `init_d`**, not that this state is the canonical starting state of the program. The source explicitly places the latter obligation outside F21. (`research/sherlock-recursion-design-review-20261110/astra/review-1.md:41`; `clean/fork/Clean/Sherlock/Aggregation.lean:1110–1120`.)

**D. Final verdict and top five concerns**

**Makes sense with changes, but the current evidence does not establish a qualifying universal recursion VM.** Keep the hardened linear node and final decider. Their implemented height, genesis-carry and terminal-opening checks remain useful foundations. Move a diagnostic leaf-cost probe forward, then measure an actual shared leaf construction before committing to the final key closure. Prefer S≥8 as a planning candidate, without declaring S=4 mathematically impossible. Remove F and the failed tree candidate from the current qualification fallback list. Establish the leaf semantics and initialization guarantee explicitly, then demonstrate the complete independently verified ledger-call path within SRC-011. (`crates/mori-recursion/src/ivc.rs:363`, `:383`, `:409`; `crates/mori-recursion/src/run_decider.rs:278`; `D/tasks.md:64–72`, `:77`, `:86`; `qa/sherlock/decisions/speed-levers.json:68`.)

1. **Leaf soundness and authenticated initialization.** Tasks **4.2, 4.3, 4.5, 4.6, 4.9**, with the slot refinement before full implementation. (`D/assurance.md:11–16`.)
2. **Complete leaf feasibility and dependent node/decider size.** Tasks **4.1a, 4.1, 4.4, 4.10**, moving structural checks from **5.3** earlier. (`D/tasks.md:64`, `:68`, `:78`; `qa/sherlock/manifests/run-decider.json:20`.)
3. **Unmeasured mixed-workload throughput and memory.** Tasks **4.10, 5.3, 5.5, 5.6, 5.10**. (`D/requirements.md:33`; `wiki-llm/recursion-perf/profiles/c0-first-profile.md:57`.)
4. **Missing independent GP bundle verification and full call qualification.** Tasks **5.1 and 2.5**. (`D/requirements.md:32`; `crates/mori-recursion/tests/sherlock_gp.rs:288`, `:329`.)
5. **Key-reuse and application scope are insufficiently precise.** Tasks **4.3, 4.4 and M6**, including fixed capacities, initialization policy, runtime inputs and accepted outcomes. (`D/design.md:54–56`; `tools/sherlock-sdk/sherlock-run.compact:25–26`; `crates/mori-backend/src/sherlock_c6_witness_stream.rs:138`, `:170`.)