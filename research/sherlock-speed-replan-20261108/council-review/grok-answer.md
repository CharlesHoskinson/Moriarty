I will review the aggregation design and the F20/F21 artefacts read-only, starting with the design sections the brief names and then the Lean and Quint statements.The design index is in place. Next I will read the composition rules, the F8 constraints, and the Lean and Quint statements that are supposed to match them.BUNDLE: base=49bb0200b848b72e45eb377c1ed63936874cc427 head=49bb0200b848b72e45eb377c1ed63936874cc427 diff=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 manifest=f7519e46987713d8a1025d5644b8996fb92427ae41e4649b934ca495991b7852b855

VERDICTS
| Item | Verdict | One-line reason |
| 1 Soundness of composition | OBJECT | The statement-level chain is proved in Lean only under a node statement and a genesis fold that F8, the Quint model, and `ivc.rs` do not share. |
| 2 Generality | OBJECT | F21 quantifies over shapes and S; F20 is a bounded sample; the key that 2.3 will cut is the linear statement, and the shape-general node is already over the row kill. |
| 3 S-slot leaf semantics | OBJECT | Design §2.3 and Quint `leafOk` state the slot rules; the Lean `Leaf` that task 4.2 is told to match has no slots, and the terminal-last rule has no mutant. |
| 4 Gate criteria | OBJECT | G-A is ordered two ways, the tree keep-clause can override the F22 kill, and an F20/F21 pass does not measure the SRC-004/SRC-005 fixes 2.3 exists to make. |

FINDINGS

- id: 1.1
- item: 1
- severity: OBJECT
- location: `lean/Aggregation.lean:1169-1179`; `research/tree.md:157-159`; `code/ivc.rs:327-336` and `code/ivc.rs:377`; `design/design.md:84`; `research/builds.md:63-68`
- problem: `genesis_scale_by_bit` scales only `proofAcc` and leaves `carried` in the fold. That is the current circuit (`prev_acc` assigned and folded unscaled). It is not SRC-005 realization (a) or (b), and it is not the F8 fold, which scales both the proof accumulator and the carried accumulator by the slot bit. Task 2.3 tells the implementer to match this lemma. A key cut from the lemma keeps an unauthenticated carry at genesis.
- proposed change: Lean statement. Add a lemma that, at a disabled or genesis slot, both the previous-proof accumulator and the carried accumulator are scaled to the neutral element, or the carried accumulator is constrained to the neutral encoding, and the pairing value equals the fold of the remaining enabled inputs. Name that same realization in F8 and in task 2.3, and record it before node keygen. `genesis_scale_by_bit` can stay only as a lemma about today's `proof_acc` collapse.
- evidence: The theorem concludes `accumulate [scale false proofAcc, carried, leaf] = accumulate [carried, leaf]`. F8 writes `scale(proof_L, has_L)` and `scale(carried_L, has_L)` (and the same for R). `ivc.rs:377` folds `[proof_acc, prev_acc, leaf_acc]` after scaling only `proof_acc`. `builds.md:66-67` already says realization (b) needs a new lemma.

- id: 1.2
- item: 1
- severity: OBJECT
- location: `code/ivc.rs:7-8` and `code/ivc.rs:184-188`; `research/tree.md:145-159`; `lean/Aggregation.lean:27-33` and `lean/Aggregation.lean:1040-1086`; `model/aggregation.qnt:136-149`; `design/design.md:74` and `design/design.md:79` and `design/design.md:115`; `design/tasks-excerpt.md:8`
- problem: Three node statements are in use. The linear circuit publishes `[self_vk_repr, prog_d, init_d, cur_d, h, acc]` and treats `is_zero(prev_h)` as genesis. F8 and Lean publish a segment `(prog, pre, post, h_lo, h_hi)` and check genesis at the decider (`root.lo = 0`, `root.pre = init`). Quint checks genesis inside the node only when `TREE = false`, and only at the decider when `TREE = true`. Design §3.1 and §3.6 say the chain and the tree share one statement and one decider. Task 2.11 records the placement as open. Task 2.3's edit list (range checks, `steps <= S_MAX`, genesis carry) does not change the public statement, so a node keyed from that list will not be the `link` the theorems talk about.
- proposed change: Design, before any key. Either keep the linear statement and write the refinement in §3.2 (prefix chain, `is_zero(prev_h)` means no carried child only after the SRC-004 checks, decider pins `init_d` and final `h`), and drop the sentence that the chain and the tree share a statement. Or change the linear statement to the F8 segment and an explicit slot bit before keygen, and make `decider_pinning` the decider for both. Quint `genesisAtNode` and Lean `Decided` must match the choice.
- evidence: Task 2.11 note: "genesis placement for the linear node (in-node vs decider only) is open for 2.3/2.14." Task 2.12 note: `ivc.rs` gates the carried slot by `is_zero(prev_h)`, while F8 uses an explicit bit.

- id: 1.3
- item: 1
- severity: OBJECT
- location: `research/builds.md:56-58`; `research/tree.md:152-154` and `research/tree.md:313-318`
- problem: The 2.3 skeleton says the carried slot is absent if and only if `prev_h = 0`, and gives the reason as `h_lo < h_hi`. That reason is false on the tree. The parent of the genesis segment has `h_lo = 0` and `has_L = 1`. An `is_zero(h_lo)` gate on that parent scales away the left child's proof while the statement still claims the child's segment.
- proposed change: Design and the 2.3 skeleton. State that `is_zero(prev_h)` is a carried-slot bit only for a prefix chain whose heights cannot return to 0. F8 keeps the explicit `has_L` / `has_R` / `has_leaf` bits. Do not copy the linear genesis test into `tree.rs`.
- evidence: F8's L-constraint is `has_L ? (L.pre_d = pre_d, L.h_lo = h_lo, ...) : (m1 = pre_d, h1 = h_lo)`. A leftmost parent has `h_lo = 0` and `has_L = 1`.

- id: 1.4
- item: 1
- severity: concern
- location: `lean/Aggregation.lean:979-1036` and `lean/Aggregation.lean:1070-1118`; `design/design.md:117`; `design/tasks-excerpt.md:9`; `design/assurance.md:44`
- problem: `segment_sound` and `decider_pinning` are proved for natural-number heights. They do not use `field_eq_exact` or `height_no_wrap`. `result_unique` and `statement_independent_of_S` add determinism and terminal halting as extra hypotheses. Design §3.6 and assurance §3 state L1 as "the root statement implies a valid execution" without those hypotheses. SAS-009 requires the premises in the claim.
- proposed change: Design §3.6 and assurance §3. Put next to L1/L6/L-C the premises already in `Premises` and the `det`/`stuck` hypotheses of `result_unique`. Say the field reading is L5, applied to the circuit checks, and is not a hypothesis of `decider_pinning`.
- evidence: `decider_pinning` rewrites `Run` with `root.lo = 0` and `root.hi = nSteps`. `field_eq_exact` is a separate theorem at `Aggregation.lean:1144-1147` and is not referenced by `segment_sound`.

- id: 1.5
- item: 1
- severity: concern
- location: `lean/Aggregation.lean:1123-1147`; `design/requirements.md:26`; `design/tasks-excerpt.md:9`
- problem: The comment on `height_no_wrap` calls `prev_h < 2^64 - S` and `steps <= S` "the SRC-004 check pair". SRC-004 requires `prev_h`, `steps`, and `h = prev_h + steps` all below `2^64`, and refuses `steps = 0` and `steps > S`. Both pairs can be sufficient. They are not the same pair. `field_eq_exact` does not take `h₂ < 2^64`. `weaker_pair_insufficient` shows that `prev_h < 2^64` and `steps <= S` without an `h` check admits the sum `2^64`.
- proposed change: Lean statement. Rename `height_no_wrap` so it is not labelled SRC-004. Add a lemma whose hypotheses are the SRC-004 triple, `1 <= steps <= S`, the field equation, and `2^65 <= p`, and whose conclusion is `h = prev_h + steps` as natural numbers and `h < 2^64`. Task 2.3 should cite that lemma.
- evidence: `requirements.md` SRC-004 row. `height_no_wrap` uses `prevH < 2^64 - S`. Task 2.12's wording note says L-A sufficiency "needs only `prev_h < 2^64 - S`".

- id: 1.6
- item: 1
- severity: note
- location: `code/ivc.rs:316-318` and `code/ivc.rs:327-336`; `model/aggregation.qnt:21-24`; `qa/ivc/REPORT.md:18-21` and `qa/ivc/REPORT.md:245`
- problem: The open circuit gaps are real and sit in the current code: no range check on `prev_h`, `steps`, or `h` (only `steps != 0` and a field add), and `prev_acc` folded unscaled at genesis. F20 states that SRC-004 gadgets and SRC-005 scaling are outside the model. A passing F20 run does not close them. The host note on the ivc model says the counterexamples reproduce both gaps, and that the CEX-3 cancellation is algebraic because the fold challenge hashes `prev_acc`. The owed artifact is still the constraint or a written argument.
- proposed change: Code, in task 2.3, after findings 1.1 and 1.2. Keep the gaps labelled as current-code defects. Do not treat an F20 pass as closing them.
- evidence: `aggregation.qnt:21-24`: "Not covered (on purpose): ... SRC-004 range gadgets, SRC-005 accumulator scaling at genesis". `qa/ivc/REPORT.md` section 5: "Task 2.3 still owes the constraint or the written argument."

- id: 1.7
- item: 1
- severity: note
- location: `qa/ivc/ivc_circuit.qnt:377-387`; `qa/ivc/ivc_circuit_test.qnt:140-155`; `design/tasks-excerpt.md:11`
- problem: `hardenedTask23SoundAndCompleteTest` expects the wrap attack `prevH = P` to fail the circuit under the proposed range checks. In that model `P` reduces to 0 before the bound is applied, so a correct `< HMAX` check on the field element cannot refuse it. The failed test does not show that the SRC-004 checks are the wrong fix. `steps = P - 1` (attack A2) is the attack that a range check can refuse.
- proposed change: Quint model `qa/ivc/ivc_circuit.qnt`. Retarget attack A1 at a non-canonical residue that is at least `HMAX` and is not `0` in the field, produced as the published `h` of a previous step. Re-run the hardened test after that retarget and after finding 1.1's lemma exists.
- evidence: `attackWrapAroundDropsPrevAcc` sets `prevH: P` and comments "`P == 0 mod P`". `circuitSatisfied` binds `prevH = fmod(w.prevH)` and then checks `prevH < HMAX`.

- id: 2.1
- item: 2
- severity: OBJECT
- location: `model/aggregation.qnt:32-35` and `model/aggregation.qnt:318-324`; `model/aggregation_test.qnt:181-187` and `model/aggregation_test.qnt:365-372`; `design/design.md:117`; `design/tasks-excerpt.md:8`; `model/check-f20.sh:28-35`
- problem: The design and task 2.11 say F20 checks height below `2^64`, every `L` in `1..9`, and every bracketing. The model bound is `HMAX = 2^62`. The harness fixes `L = 9` and `S` in `{1, 2, 4, 8}`. `randomBracketing` is one sampled schedule. `threeLeafShapes` covers one width of three leaves. `check-f20.sh` sweeps `aggregation_chain`, `aggregation_tree`, and `aggregation_tree_s1` at `max-steps 30`. Those runs are bounded model executions. They do not show every step count or every shape.
- proposed change: Design §3.6, task 2.11, and assurance §3. State F20 as a bounded execution (those constants, those samples, mutants caught). Point the unbounded claim at the Lean theorems only.
- evidence: `aggregation.qnt:32-34`: "Quint 0.32's Rust evaluator computes in 64-bit integers and overflows on 2^64 itself, so the model's bound is 2^62." I2's comment still says "below 2^64" while `heightsOk` checks `HMAX`.

- id: 2.2
- item: 2
- severity: OBJECT
- location: `design/tasks-excerpt.md:6`; `design/design.md:22` and `design/design.md:109` and `design/design.md:115`; `design/requirements.md:39`; `code/ivc.rs:1-8`
- problem: SRC-017 asks for one key set that is independent of program, initial state, step count, schedule, and tree shape. F21's `NodeCert` is that statement-level object, and `chain_sound` is the chain as a right-slot-off tree. The circuit 2.3 will edit is the linear IVC node, whose statement is the one in finding 1.2. The only node that accepts arbitrary slot shapes is F8, and task 2.13 already measured `{self, self, leaf}` at 1,078,147 rows, `k = 21`, above `0.95 × 2^20`. One key cannot serve every shape until either that probe is replaced or the linear statement becomes the F8 statement.
- proposed change: Design and SRC-017. Say the key set produced by 2.3 and 4.4 serves every committed program and every length for the linear schedule, with `S` fixed in the key. Say every-shape remains the F8 relation and stays closed on the F22 measurement. Remove the claim that today's `ivc.rs` node is the degenerate F8 node.
- evidence: Task 2.13: "{self, self, leaf} 1,078,147 rows (k=21) > 996,147". D13' builds the tree only if the §3.5 criteria pass. The row criterion has already failed.

- id: 2.3
- item: 2
- severity: concern
- location: `lean/Aggregation.lean:690-741` and `lean/Aggregation.lean:1107-1118`; `research/prover.md:362-364`
- problem: `stmt_independent_of_S` says that a tree accepted at two values of `S` certifies the same segment. `stmt_mono_S` says raising `S` preserves acceptance. A leaf with `steps = 8` is refused at `S = 1` (`leaf_out_of_range`). L-C in `prover.md` says a program proved at `S = 1` and at `S = 8` yields the same decider statement. That is `statement_independent_of_S`, and it needs a deterministic machine with terminal halting. It does not say one circuit, with one numeric `S`, accepts every batch size.
- proposed change: Design §3.6. Keep "one keyed `S` serves every program length by repetition." State L-C as agreement of decided roots across `S`, under `det` and `stuck`, which task 2.12 already flagged.
- evidence: `leafSlot` refuses `steps` outside `1..S` (`Aggregation.lean:87-89`). The axioms file checks `steps = 9` refused at `S = 8` and `steps = 8` accepted (`AggregationAxioms.lean:51-53`).

- id: 3.1
- item: 3
- severity: OBJECT
- location: `lean/Aggregation.lean:35-41` and `lean/Aggregation.lean:204-208`; `research/prover.md:359-361`; `design/design.md:46-48`; `design/tasks-excerpt.md:23`; `design/requirements.md:36`
- problem: Task 4.2 and SRC-014 say the S-slot semantics equal the Lean `Leaf`. `Leaf` is `(prog, pre, post, hpre, steps)`. `leaf_sound` assumes a `Run` of exactly `steps` between those digests. There is no slot list, no disabled-slot identity, no zero contribution, and no terminal-last rule. `prover.md` §6b L-B (one-step slots, `post_i = pre_{i+1}`, identity padding, any `S >= 1`) is not a theorem in `Aggregation.lean`. An implementer who matches `Leaf` can ship a statement-only leaf and still meet the written F21 contract.
- proposed change: Lean statement, before task 4.2. Add the slot list to the leaf specification and prove L-B: enabled slots are a prefix of machine steps, a disabled slot is the identity and adds 0 to `steps`, a publishing slot has only disabled slots after it, `S = 1` is the one-slot case, and the resulting segment is `Leaf.seg`. Point task 4.2 and SRC-014 at that theorem.
- evidence: `design.md` §2.3: disabled padding, `steps` counts enabled events, terminal publication is the last enabled step. `Aggregation.lean` `Leaf` has five fields and no `slots`.

- id: 3.2
- item: 3
- severity: concern
- location: `model/aggregation.qnt:73-94` and `model/aggregation.qnt:193-197`; `model/aggregation_test.qnt:36-53`
- problem: Quint `leafOk` matches the structural half of §2.3: `S` slots, enabled slots are a prefix, a disabled slot is an identity, adjacent slots chain, `steps` is the enabled count, `1 <= k <= S`, and `mkLeaf` is the same function at `S = 1`. The terminal rule is not a predicate on a slot. Halt exists only as `idx = nStepsOf(prog)` on the abstract trace, and `canLeaf` refuses `i + k > nStepsOf(p)`. A leaf that takes an enabled step after `Halted`, `Rejected`, or `Exhausted` inside a longer carrier trace is not a seeded mutant. F20 will not catch a 4.2 circuit that omits that gate.
- proposed change: Quint model. Add a publish bit on the enabled slot and a mutant that enables a later slot, and require `leafOk` to refuse it. Until that exists, say in the model header that terminal-last is a 4.2 circuit obligation outside the checked invariants.
- evidence: `leafOk` (`aggregation.qnt:84-94`) has no outcome tag. `canLeaf` (`aggregation.qnt:194-195`) bounds `i + k` by the program length. The short-chunk check in `everyProgramChain` (`aggregation_test.qnt:49-52`) checks the enabled prefix of an honest chunk, not a post-halt slot.

- id: 3.3
- item: 3
- severity: note
- location: `lean/Aggregation.lean:69-70` and `lean/Aggregation.lean:441-447`; `model/aggregation.qnt:128-134`
- problem: The external leaf statement does agree across the three artifacts: the certified segment is `(prog, pre, post, hpre, hpre + steps)` with `1 <= steps <= S`. `leaf_only` and `leafSlotOk` both refuse `0` and `S + 1`. That part of task 4.2's correspondence holds. The gap is the internal slot contract in finding 3.1.
- proposed change: Keep this external correspondence when L-B is added. Do not replace it with a second leaf relation for `S = 1`.
- evidence: `Leaf.seg` sets `hi = hpre + steps`. The axioms file accepts `steps = 8` at `S = 8` and refuses `steps = 0` and `steps = 9`.

- id: 4.1
- item: 4
- severity: OBJECT
- location: `design/design.md:111`; `design/tasks-excerpt.md:11-13` and `design/tasks-excerpt.md:22`; `research/order.md:142-146` and `research/order.md:177`
- problem: G-A is specified in two orders. Design §3.5 and task 2.15 place G-A after A16, and A16 is the measurement of the 2.3 cut. Task 2.3 sets `S_MAX` to "the G-A provisional S (16 unless G-A says otherwise)". `order.md` §4 lists both `G-A -> 2.3` and `2.3` then G-A. G-A blocks 2.4, and G-A' (task 4.10), which can change `S` after `k_U` is known, does not block 2.4. The node relation constant and the leaf constant can be keyed apart.
- proposed change: Design §3.5, `order.md` §4, and tasks 2.3, 2.4, 2.15, 4.10. One order: 2.3 may cut rows with a scratch `S_MAX` and must not key; G-A records the provisional `S`; G-A' records the final `S`; 2.4 and 4.4 key once, with the node bound equal to the leaf `S`.
- evidence: Task 2.15: "Blocks 2.4 (one keygen), 4.2 and 5.7" and "provisional S from the node numbers (final at G-A')". Task 2.4 is "after the cut, so the node is keyed once" and is not listed as depending on 4.10.

- id: 4.2
- item: 4
- severity: OBJECT
- location: `design/design.md:109`; `research/tree.md:480-492`; `design/tasks-excerpt.md:6` and `design/tasks-excerpt.md:31`; `research/order.md:81`
- problem: The tree gate is not one predicate. `tree.md` §7 builds only when all four criteria hold, and kills at `k = 21`. `order.md` table 2.5 uses the same "all of / any of" shape. Design §3.5 row 5's keep-clause ends with "or `k_U >= 18` / k=19 failed / chain alone misses 2 h", which reopens the tree after a row kill. Task 2.13 already recorded the kill: 1,078,147 rows, `k = 21`, above 996,147. Task 5.7 is still "only if G-A opened the tree" against a criterion that can open it anyway.
- proposed change: Design §3.5 row 5 and task 2.15. Record the tree closed on the F22 numbers. Delete the trailing `or` that overrides the row kill. A later reopen needs a new row probe. Align task 5.7 with that record so it cannot start on the current probe.
- evidence: Task 2.13: "tree KILLED. {self, self, leaf} 1,078,147 rows (k=21) > 996,147". `tree.md` §7 item 1: "three-slot node ≤ 0.95 × 2^20 rows". Design row 5 keep-if lists that bound and then an `or` of throughput failures.

- id: 4.3
- item: 4
- severity: OBJECT
- location: `model/aggregation.qnt:21-26`; `research/prover.md:345-346`; `design/assurance.md:44` and `design/assurance.md:54`; `design/tasks-excerpt.md:11`
- problem: The gate that blocks 2.3 is "F20 passes, F21 passes, 2.14 done". F20's header excludes the SRC-004 range gadgets and the SRC-005 accumulator scaling. `prover.md` Q-B (genesis scales the previous-proof accumulator to the trivial one, and no non-genesis path sets `prev_h = 0`) is not an invariant of `aggregation.qnt`. F21's L-D lemma is the insufficient fold in finding 1.1. Assurance §3 then cites those rows as the formal evidence for the node. The gate measures evidence that does not include the properties the key must have.
- proposed change: Gate, in design §3.6 and assurance §3. Split the 2.3 entry condition. F20/F21 remain the statement-level gate after findings 2.1, 3.1, and 1.4. SRC-004 and SRC-005 need the revised Lean lemma from 1.1 and a circuit-level check (the ivc model, after finding 1.7's retarget) before keygen. State in the assurance row that an F20 pass leaves SRC-004 and SRC-005 open.
- evidence: `aggregation.qnt:21-24` lists those requirements under "Not covered (on purpose)". Assurance §3 says both rows "gate 2.3, 4.2 and 5.7". Q-B is still the formal target in `prover.md:345-346`.

- id: 4.4
- item: 4
- severity: concern
- location: `design/tasks-excerpt.md:11`; `design/requirements.md:23`; `research/builds.md:74-76`; `code/ivc.rs:59-63` and `code/ivc.rs:364`
- problem: Task 2.3 still lists, as a k=19 row candidate, folding the leaf-instance lanes `rule` and `binding` into `rules_d`. SRC-001 freezes the seven-lane statement `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`. The node copies `binding` into the leaf instance and does not constrain it, because it is the leaf's own claim lane. Dropping it changes the keyed statement.
- proposed change: Task 2.3 text. Remove `binding` from the cut candidates. A row cut may still drop other lanes the requirements do not freeze, with the per-gadget row report.
- evidence: `builds.md:74-75`: "the k=19 cut may NOT drop `binding`". `LEAF_LANES = 7` and `leaf_inst` includes `binding` at `ivc.rs:364`.

- id: 4.5
- item: 4
- severity: note
- location: `design/tasks-excerpt.md:4-5`; `design/design.md:111`; `design/assurance.md:54`
- problem: G-A's dependence on A15, A17, and A23 is aimed at measurements that are not in the bundle yet. Task 2.8 records the overhead qualification as inconclusive (bound 1.017 against a 1.01 bar). Waiting on those rows is a real gate. The defect is the contradictory tree clause and the S-ordering in findings 4.1 and 4.2, not the fact that C0 has not been run.
- proposed change: Leave A15, A17, and A23 as predecessors of the G-A JSON. Do not invent their numbers in the decision record.
- evidence: Task 2.8: "until then the first-profile numbers are reported as overhead-unqualified". Task 2.9 is still open.

ABSTENTIONS: none. The practical cost of grinding `prev_acc` against the Poseidon fold challenge is outside this bundle (`qa/ivc/REPORT.md` section 5 treats CEX-3 as algebraic). SRC-005 already requires the constraint or a separate argument, so that missing attack cost does not block the four verdicts.
