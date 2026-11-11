# Audit of CONSENSUS.md (run `sherlock-runlength-consensus-20261111`)

Date: 2026-11-11. Target: `research/sherlock-runlength-consensus-20261111/CONSENSUS.md`. Checkout: `/home/hoskinson/Projects/Moriarty` at `9ffc2fe76`. Read-only audit: no build, test, proof or model run. Paths are relative to the repository root. `O/` means `openspec/changes/sherlock-vm-consensus-20261107/`. `C:n` means line n of CONSENSUS.md.

## 1. Verdict

**Adopt with changes.**

The answer to the owner's question is correct: 2^64 is the range of the step counter that SRC-004 checks (`crates/mori-recursion/src/ivc.rs:79,363-376`), it is not a circuit size, and no circuit grows with the run length. The memory direction is also sound (dense memory with a carried partial segment, a fixed-depth tree, and hash-chain stacks): I found no attack on its core, on condition that the invariants of finding 8 are written down, and the report rejects "keep D3 and chain runs" for good reasons (the 2^17 leaf limit and the open SI-12 conflict). But the report cannot be adopted as written, for four reasons: (1) the proposed GP-001 text narrows GP-1 and GP-4 (findings 1 to 3); (2) it does not choose one memory design, because two variants stay open at 3 against 2 (finding 4); (3) the second continuation mode (address-free checkpoint with lazy unfold) is not specified well enough to judge, and the report presents it as agreed and as closing attacks (finding 5); (4) several "5/5 unanimous" statements do not match the lane files (finding 7). Corrected GP-1 and GP-001 text is in section 5.

## 2. Findings

### Blockers

**1. GP-001 narrows "every program, every input".**
Evidence: `C:54` and `C:61` say "every admitted program of the Sherlock language, on every valid input". The current text says "every program of the Sherlock language, on every input" (`O/requirements.md:56`, `O/general-purpose.md:9`). The words "admitted" and "valid" come from one lane only (`lanes/E/round2.md:144`). Lanes A, B, C and D keep the original words (`lanes/A/round2.md:163`, `lanes/B/round2.md:143`, `lanes/C/round2.md:141`, `lanes/D/round2.md:117`). `C:37` repeats the narrowed wording.
Required change: restore "every program of the Sherlock language, on every input" in GP-001 and in the spec mirror.

**2. The claim-counter bound narrows GP-4 at a reachable size.**
Evidence: `C:54` puts "claim counter `< 2^64` in aggregation" into GP-001, and `C:238` tests "refusal at diamond-DAG `SET.n_claims = 2^64`". `n_claims` counts occurrences, not distinct claims (`research/sherlock-gp-architecture-20261110/architecture.md:100`). If a join adds the counts of its children, a DAG in which each level feeds two consumers doubles the count per level. The count reaches 2^64 at depth 64 with about 128 proofs. GP-4 requires "DAGs of any depth" (`O/general-purpose.md:44`). The range check stops the wrap attack (E-1) and also refuses an honest DAG. Without the check, the count wraps the field at depth 255, which is also reachable.
Required change: remove the claim counter from GP-1 and GP-001. Record an open item against `architecture.md:100`: an occurrence count must not be range-checked, and it must not carry soundness. `set_root` binds the DAG. A count of distinct claims, or no count, are the candidates.

**3. The memory-counter bound removes "bounded only by the height field".**
Evidence: the current bullet is "The run length is bounded only by the height field (below 2^64)" (`O/general-purpose.md:14`). The proposed text (`C:46`, `C:54`) replaces it with three bounds: steps, heap cells and claims. With TOP = 61 the address space is 2^64 - 1 cells (`C:98`), and the report states up to 3 appends in one step (`C:106`). A run of fewer than 2^64 steps can then exhaust the address space, and such a run "gives no proof" (`O/requirements.md:18`, SVM-010). This is physically out of reach, but it is a formal narrowing, and the owner rule is absolute (`O/general-purpose.md:58`).
Required change: keep the sentence of line 14 verbatim (section 5). Size each address counter so that the height field always binds first. With at most 3 appends per step in one arena that is TOP >= 63 (2^66 cells; ESTIMATE +280 rows per dual walk at 70 rows per permutation), plus the input cells. Alternative: an explicit owner decision that accepts a cell bound.

### Major

**4. No single memory design is chosen.**
Evidence: `C:15`, `C:105-107` and `C:262-268` keep variant d1 (one tagged arena, 3 lanes) and d2 (four arenas, 2 lanes) "side by side", but `C:15` says "all five lanes adopt". The two variants differ in the state digest, the Init table and the model. d2 matches the four `(root, length)` pairs of SRC-021 (`O/requirements.md:46`; `O/design.md:266` allows one log only if its root opens to the four pairs). d2 breaks the gate `k_leaf <= 16` at `N_page = 8` (`C:107` against `O/design.md:105`). d1 depends on a "108-rule non-interleaving proof" (`C:264`) that is a code reading, not a checked invariant. Round 2 lanes read only the round 1 files of the others (`BRIEF.md:57`; confirmed in `lanes/B/round2.md:284` and `lanes/E/round2.md:232`), so the rebuttal of Lane B's objection (`lanes/B/round2.md:35`) is the orchestrator's own. My sample supports the rebuttal in part: the Append helper allocates only value slots, one per step (`crates/mori-core/src/sherlock/helper_machine/families/environment.rs:233-265`). I did not check the other rules.
Required change: state the decision as "tail-buffered dense memory at fixed depth is the direction; the arena count is open". Decide it by (a) a mechanised contiguity invariant over the reference machine (Quint, or an assertion in `mori-core` run on the corpus), and (b) measured leaf rows for both variants. Until (a) passes, d2 is the default, because it needs no global invariant and no address refinement map. Neither variant lists the Init fields it adds (`tail_d`, `fp_hash`, `tp_hash`) or removes (`fp`, `tp` links). Add that list.

**5. Continuation mode 2 is not ready, and tier 2 is described as GP-2 composition, which GP-002 forbids.**
Evidence:
- `C:144` presents the address-free checkpoint with lazy `UNFOLD` as "agreed by all 5/5 lanes" and as "closing SRC-021 [OPEN]". Lane C leaves the slice case open: slices "materialize their m-element pointer block ... or keep Mode 1 for large-arity constructors" (`lanes/C/round2.md:205`). A block of m > 8 cells in one step does not fit the 15-row window and breaks the "at most 3 appends" argument (`C:106`). If it takes extra steps, `n_steps` differs from the uninterrupted run and SRC-023 fails.
- After a mode 2 resume the addresses differ. So `state_d(h)` is not "a deterministic function of `(prog_d, input_d, h)`" (`C:131`) across modes.
- The checkpoint work is O(live roots + stack depth) (`C:144`). That is not one step, and SRC-004 and SRC-014 require every leaf to carry at least one step. No lane says which proof carries that work.
- The `cont_claim` formula (`C:140`) has no mode field. Lane C's statement has one (`lanes/C/round2.md:211`).
- `quantum_steps` is not in the claim and is bound nowhere (`C:137`). A cut does not change the outcome, so it must be a prover choice, like a leaf boundary. A keyed constant would put a run-length parameter into the key.
- `C:46` and `C:54` say continuations compose "via GP-002". GP-002 states "a segment statement or a state digest SHALL NOT be accepted as a claim of another run" (`O/requirements.md:57`), and `C:141` itself says GP-002 consumers reject the continuation claim.
Checkpoint replay, my result for mode 1: I found no attack. An epoch proof binds the key, `prog_d`, the unchanged `input_d`, both heights and both state digests. The join requires equal digests and adjacent, strictly increasing heights. A chain seals only from `h_lo = 0`, where the first leaf proves `Init`. An epoch proof used again in another chain of the same program and input is a proof of the same deterministic run, so it gives the same outcome. It cannot be moved to another input, program or height. No epoch claim reaches the chain, so the only replay key is `claim_d`, as for an uninterrupted run. This result does not cover mode 2, where the resume check is not defined.
Required change: adopt tier 1 and mode 1 only. Mode 1 is the segment recursion of tier 1 with proofs and the memory store kept on disk between sessions: same heights, same `pre_d`/`post_d` link, same Init at `h_lo = 0`. State whether it needs a new tag at all; a stored SEG proof may be enough. Move mode 2 to "open research" with the gaps above, and remove "closes SRC-021 [OPEN]". Replace "under GP-2" by "by the same segment recursion".

**6. The report gives no maximum of reads, appends and flushes for one step.**
Evidence: a step that needs more chip rows than the leaf has is unprovable even with one step per leaf. The report uses this argument against linked slices (`C:79`), but it does not apply it to its own design. At TOP = 61 a cold read costs 4,480 to 4,620 rows (ESTIMATE, `C:102`), and the leaf has `N_page` = 4 to 8 (`C:117`). The report gives no per-step maximum. Its citation for "a single step projects any field" (`C:79`, `owner_adt_runtime.rs:1430-1538`) is partly wrong: `walk_path_from` (`crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime/owner_adt_runtime.rs:1430-1454`) walks a whole program-defined path, but its callers are `evaluate_plan_dag` (`:1464`) and the SI-21 invariant check `si21_adt_result_ok` (`:1776,1843,1861`), which are host checks and not machine steps. The step rule `XAdtHelperPlanWalk` reads one field per step (`:894-907`), which supports the report's point. I checked no other rule. "At most 3 appends per step" comes from three programs (`O/design.md:119`) and a lane code reading.
Required change: before adoption, add a per-rule table of the maximum cold reads, appends and arenas touched in one step, taken from the 108 rules. Add the requirement that this maximum fits the chip heights at S = 1.

**7. The "5/5 unanimous" statements overstate the lane files.**
Evidence:
- The GP-1 and GP-001 text (`C:46-55`) is a composite that no lane wrote. In the acceptance line of `general-purpose.md`, the continuation chain comes from Lane B only (`lanes/B/round2.md:136`) and the three-partition replay from Lane E only (`lanes/E/round2.md:137`).
- D-Q1.1 is counted "3 vs 2" with Lane A "structurally" in the verbatim group (`C:259`). Lane A rewrites the whole block, headline included (`lanes/A/round2.md:141-151`). The true split is 2 verbatim (C, E) against 3 that edit the headline (A, B, D).
- "All five lanes agree" on slot-0 scratch zeroing (`C:92,103`). Lane C's round 2 does not mention it. Lane B asks for zero at Init and on disabled slots, with write-before-read (`lanes/B/round2.md:62`). The report's own table lists it for A, D and E only (`C:247`).
Required change: restate each agreement with the lanes that hold it. Mark the GP text as the orchestrator's synthesis.

**8. The adopted memory design omits invariants that its soundness and partition independence need.**
I checked the flush rule by hand. After any partition, the root holds exactly the full segments below `floor(len / 8)`, and the tail holds the rest. So `(root, tail, len)` is a function of the append sequence only. One leaf adds at most 8 cells to at most 7 tail cells, so it crosses at most one boundary. No index is written twice. `len = 2^64 - 1` is reachable and `2^64` is not. The last segment (`q = 2^61 - 1`) can hold 7 cells and is never flushed. These results hold only with the following rules, which `C:97-102` does not state:
- `tail_d` is canonical: positions `>= r` are a fixed pad record, and `r = 0` gives one constant. Only Lane E writes this (`lanes/E/round2.md:156`). Without it `post_d(h)` is not unique.
- A leaf that reads or extends the tail opens it against `pre_d.tail_d`. A leaf that does neither passes `tail_d` through unchanged.
- `len_pre` and `len_post` are both decomposed as `8q + r` with 61 boolean bits and `r < 8`. `O/design.md:86` range-checks written values only.
- In d1, every read constrains the record kind tag to the kind that the rule expects. `C:106` defines the tags but no read rule.
- The W_flush = 1 argument is per leaf (`r_pre + used <= 15`, with `used <= 8` for the leaf), not per step as `C:106` argues. The conclusion holds; correct the reason.
- Every hash carries a domain tag in its first input. The sponge separates input lengths (`crates/mori-chip/src/poseidon.rs:99`), so `SHK_CONT_V1` (11 inputs) and `SHK_RUN` (5 inputs) cannot collide by position. But the spike's tree nodes and state digest have no tag (`crates/mori-chip/src/m3.rs:12-16,29`), and the empty slot is 0 there. I found no tag confusion on these conditions.
- "`frame_count == 0 <-> fp_hash == EMPTY`" (`C:96`) can be enforced in one direction only. The count is the rule; the other direction is a collision assumption. The same applies to "`EMPTY_SEG` differs from any valid hash" (`C:101`).
- The design replaces D2, D3 and D6, which have Quint counterexamples in their present form (`O/design.md:92-99`). The hash-chain stacks are not D7: D7 links frames by an address in memory (`O/design.md:100`), and only that form is modelled. `C:94` calls the new protocol "D7".
Required change: add these rules to section 1.2.3. State that new chip models (tail, flush, hash stack) are a condition of adoption, not later work.

**9. The slot-0 zeroing argument is wrong.**
Evidence: `C:104` says that zero scratch registers at slot 0 of every leaf remove cross-leaf smuggling "without relying on manual write-before-read proofs". If any rule reads a scratch value written by an earlier step, zeroing at leaf boundaries makes the behaviour depend on the partition: 0 after a boundary, the old value inside a leaf. That breaks SRC-023 instead of protecting it. If no rule reads scratch before it writes, the zeroing is redundant. The design already states the real rule: "Registers outside the state digest are write-only" (`O/design.md:96`), and the spike writes all three in every step (`crates/mori-chip/src/rf.rs:10-11`). Lane E lists the write-before-read audit as not done (`lanes/E/round2.md:229`).
Required change: make the obligation "no rule reads `v0..v2` before it writes them in the same step", enforced per slot (zero or fresh at every slot's pre row) or by a checked audit. Remove the claim that zeroing replaces that proof.

**10. "D3 falsifies SRC-023 and the Lean theorem" is overstated.**
Evidence: SRC-023 constrains `outcome_d` and `n_steps` only (`O/requirements.md:48`). `vd` is address-free, and the record states that under D3 "the prover can only fail to prove a long run. It cannot change the outcome" (`O/dissent.md:230`). So D3 padding does not violate SRC-023 as written. `statement_independent_of_S` (`clean/fork/Clean/Sherlock/Aggregation.lean:1154-1164`) is a proved theorem about the final decided root, under the premise `det`. Padding makes that premise impossible to discharge for the physical state. It does not falsify the theorem, and the theorem says nothing about `post_d(h)` at an intermediate height. `C:78`, `C:83`, `C:85` and the table row at `C:121` say "FALSIFIED".
Required change: say "under D3 the physical state at height h depends on the partition, so the `det` premise cannot be discharged at digest level, and parallel provers must fix the partition first". The rejection of options (a), (b) and (c1) still stands, on the 2^17 limit and on SI-12 (`O/dissent.md:198-202`).

**11. The node and decider row claims are not estimates of this design.**
Evidence: "+4..+12 node rows" (`C:15`, `C:34`) is the E2 fit for leaf k alone (`research/sherlock-recursion-design-review-20261110/experiments/e2-e4.md:25-35`). The node grows with the leaf's columns and lookups: about 7,298 rows per advice column and 22,600 per lookup (`O/design.md:134`). A 15-row window with a multiplexer, tagged records and two hash stacks may add columns, selectors or lookups. "+0 decider rows" is correct for the leaf. But the decider has 1,032,467 table rows of 1,048,576, and the review did not check whether they grow (`research/sherlock-c6-outcome-ledger9-review-20261110/review.md:213`). The table header at `C:119` gives the decider gate as `<= 1,048,570`; the gate is 1,000,000 (`O/design.md:105`).
Required change: state the node delta as "+4 rows per leaf k bit, plus an unmeasured column and lookup term". Keep the 1,000,000 gate.

**12. Throughput figures assume full leaves, and some are unmarked.**
Evidence: every s/step figure in `C:212-214` and `C:218-227` assumes 16 steps per leaf. Cold reads cost about 3.2 times the D3 cost (`C:116`), and the early-close rate was not simulated (`C:346`). So these are lower bounds on s/step, not central estimates. Figures without ESTIMATE: `~1.8-4.0 s/leaf` (`C:213`), which also contradicts `4.5-8.0 s` solo (`C:205`); the lane positions in `C:250`; "11x ... 0.39 s/step ... 29 min" (`C:287`); and the external rates "5-20 kHz" and "200-813 Hz" (`C:214`, `C:250`). The proposed GP-001 acceptance (`C:54`) carries time caps (`< 2 h`, `<= 6 h`) that rest on these figures and on a leaf that is not built.
Required change: mark all of them ESTIMATE and add "assumes 16 steps per leaf". Keep time caps out of GP-001; they belong to SRC-011 and SRC-015 rows after measurement.

**13. The report closes dissent D-1 without the measurement that the record requires.**
Evidence: D-1 is open, and "the candidate to measure first is Carbon's linked-slice form, and Astra's mapping is the alternative (task 4.18, row A42)" (`O/dissent.md:202`; `O/tasks.md:86`). The report rejects linked slices "5/5" from a code reading (`C:79`) and does not mention Astra's mapping from logical identifiers to addresses at all. Its own design is a third form: addresses as references, with SI-12 restored because padding is removed. That form may be right, but it changes the status of D-1 and the content of task 4.18.
Required change: add a section that proposes the D-1 status change (new candidate: contiguous slices in dense memory), compares it with Astra's mapping, and rewrites task 4.18 to measure the new candidate with the scenario of `review.md:174`. The lead or the owner makes the status change in `O/dissent.md`, not this report.

### Minor

**14.** `crates/mori-recursion/src/verifier_gadget.rs:16-17` (`C:17`, `C:89`, `C:155`, `C:315`) does not exist. The file is in the dependency `midnight-circuits` 7.3.0 (`src/verifier/verifier_gadget.rs:16-17`, cargo registry). It says `num_challenges = 1`, not `<= 1`. The claim itself is correct. `O/design.md:122` cites the file correctly.
Required change: correct the path and the wording in all four places.

**15.** "73 rows for all SRC-004 + SRC-005 checks" cites `qa/sherlock/decisions/speed-levers.json:31` (`C:32`). That line is a `q_note` about a trace. No 73-row figure is there. "~24-50 rows" has no source.
Required change: give the measured source, or mark both figures ESTIMATE and remove the citation.

**16.** Line references: D3 is at `O/design.md:96`, not `:94` (`C:74`). D-1 is `O/dissent.md:198-202`, not `198-203`. The Lean theorems start at `:1201` and `:1211`; the cited ranges include the doc comments, which is acceptable.
Required change: correct the two line references.

**17.** "2^61 x 8 = 2^64 dense cells" (`C:15`, `C:34`, `C:97`) against "2^64 - 1 dense cells" (`C:114`). The second is correct.
Required change: use "at most 2^64 - 1 cells; the last segment is never flushed" everywhere (or the wider space of finding 3).

**18.** Rows A33b and A34c (`C:234`) do not exist in `O/`. SAS-012 (`O/requirements.md:76`) separates evidence classes and does not record bounds; SAS-001 does. `6,432 B` is the proof size of today's node and decider and must not be a constant of GP-001. "`>= 512`-`1,024` steps" (`C:48`) is a range and cannot be tested; dissent D-Q5.2 is unresolved.
Required change: mark A33b and A34c as proposed new rows; cite SAS-001 only for tested bounds; replace `6,432 B` by "the byte length of a one-leaf run"; name one threshold after D-Q5.2 is settled.

**19.** OSV dates CVE-2026-62333 at 2026-10-30; `C:189` says "Jun 2026". The two advisories that I fetched exist and match the described root causes.
Required change: correct the date, and state for each advisory row the date on which it was fetched.

**20.** The executive summary gives `k_leaf = 16..17` (`C:14`), while the gate is `k_leaf <= 16` at S = 16 (`O/design.md:105`).
Required change: either say that d2 with `N_page = 8` fails the gate, or propose a gate change in plain words.

## 3. Claims verified

Short names in this section: `ivc.rs` is `crates/mori-recursion/src/ivc.rs`; `Aggregation.lean` is `clean/fork/Clean/Sherlock/Aggregation.lean`; `review.md` is `research/sherlock-c6-outcome-ledger9-review-20261110/review.md`; `experiments-4.md` is `research/sherlock-processor-analogy-20261110/experiments-4.md`; `e2-e4.md` is `research/sherlock-recursion-design-review-20261110/experiments/e2-e4.md`; `architecture.md` is `research/sherlock-gp-architecture-20261110/architecture.md`; `owner_runtime.rs` is `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs`; `rf.rs` is `crates/mori-chip/src/rf.rs`.

- SRC-004 guard: `H_BITS = 64` and `S_MAX = 16` (`ivc.rs:77-79`); `constrain_heights` checks `prev_h`, `steps - 1 < S_MAX` and `h` (`ivc.rs:363-376`).
- `src004_height_bridge` (`Aggregation.lean:1201-1206`), `unchecked_prev_wraps_to_genesis` (`:1211-1213`), `statement_independent_of_S` (`:1154-1164`), `result_unique` (`:1136`).
- Row counts: leaf 6,511 at k = 13, node 823,466, decider 929,166 (`O/design.md:129-131`); decider table rows 1,032,467 (`review.md:213`).
- D3 in code: `TOP = 17` (`crates/mori-chip/src/m3.rs:51`), `len_post = base + 8` (`:23,132`), empty slot 0 (`:12`). D3 ceiling sources: `review.md:40,94`; `experiments-4.md:148-149`; `O/dissent.md:228-232`.
- D-1 text and "open" status (`O/dissent.md:198-202`); task 4.19 (`O/tasks.md:87`).
- E2 (+4 node rows per leaf k, decider constant) and E4 (66.2 s, 11.1 GB; node 90.2 s when concurrent) (`e2-e4.md:7-35`).
- Write-once memory (`O/design.md:82`); rotations -1, 0, +1 (`O/design.md:122`; `midnight-circuits` 7.3.0 `verifier_gadget.rs:605-609`).
- Stacks are push, pop and rewrite-top only (`owner_runtime.rs:3566-3591`); `alloc_data` excludes `alloc_data_field` and `alloc_constructor` (`:2960`).
- LT does not range-check its operands (`crates/mori-chip/src/alu.rs:6-9`). `NDIG = 29` leaves out `v0..v2` (`rf.rs:7-8,34`).
- `R` verifies one or two `U` proofs (`architecture.md:75`); the SEG statement carries `input_d` (`:98`).
- Arithmetic: 2^64 steps at 4.28 s is 2.50e12 years; 146 years at 4.004 GHz; 590 EB per column; 131,071 x 16 = 2,097,136.
- Lane positions on Q2 (A, B four arenas; C, D, E one arena) and on Q3 (Lane E alone standardises on the address-free checkpoint, `lanes/E/round2.md:38-47`). Citations of D-Q1.1 for B, C, D and E.
- Current GP-1 lines `O/general-purpose.md:9-16`, GP-001 at `O/requirements.md:56`, spec mirror at `O/specs/sherlock-general-purpose/spec.md:7-15`. GP-2 to GP-4 are not edited by the proposal.
- External: GHSA-g6qg-9j8p-7j87 (RISC Zero) and CVE-2026-62333 (OpenVM) exist with the stated root causes.

## 4. Claims not verified

- The 108-rule non-interleaving claim, "at most 3 appends per step", and "no step touches three arenas".
- The maximum cold reads of one step over all rules (I checked `XAdtHelperPlanWalk` only).
- All leaf row estimates of the new design (36k to 79k), the cost of the tail opening, and `CONT-FOLD` at 250 to 350 rows.
- `59.62 s` node time (not found in `qa/sherlock/runs.jsonl`); `43.83 s/node` is 131.50 / 3 from `wiki-llm/recursion-perf/profiles/c0-first-profile.md:57`. Host hardware figures, `budgets.json`, `gpu.md`.
- `models/sherlock/quint/aggregation.qnt` and `init.qnt` line claims.
- The prior-art matrix (17 systems) and every URL except the two advisories above.
- Round 1 lane files and the lane line citations for D-Q2.2, D-Q4.1, D-Q5.1 and D-Q5.2.

## 5. Corrected GP-1 and GP-001 text

The rule used: keep every current sentence, and add. Nothing that the current text grants is removed, so nothing is narrowed. Lines 9 to 13 of `general-purpose.md` and GP-2 to GP-4 stay verbatim.

### 5.1 `O/general-purpose.md` line 14 (run-length bullet)

```markdown
- The run length is bounded only by the height field (below 2^64). The height field is the 64-bit step counter that every node proof carries. SRC-004 checks its range so that the count cannot wrap in the proof field. It is not a circuit size and it is not a length that a run must reach: no circuit has 2^64 rows, and no circuit domain, key, proof size or verification cost grows with the run length. A run of N steps is proved by folding fixed-size leaf proofs through the fixed-size node and sealing once in the fixed-size decider, so the proving work grows linearly with N. No leaf count, memory address counter or other circuit constant sets a lower limit, and a step that allocates no memory consumes no memory address. The proving of one run can stop and resume under the same key set, with the same public statement. Tested run lengths are evidence (SAS-001), not limits of GP-1.
```

### 5.2 `O/general-purpose.md` line 16 (acceptance)

```markdown
Acceptance: one key set and one deployed verifier contract accept proofs of at least three programs that the key set was not built from, including one program larger than one leaf's carriers, one run with input, and one run of each outcome kind. In addition: one run of more than one leaf that allocates more memory than one leaf holds and builds a constructor of more than 8 fields across a leaf boundary, with a final proof of the same byte length as a one-leaf proof; the same allocating run proved under three leaf partitions, and proved with one stop and resume, gives the same `prog_d`, `input_d`, `outcome_d` and `n_steps`; tests at the constraint boundary accept a height of 2^64 - 1 and refuse 2^64.
```

### 5.3 `O/requirements.md` line 56 (GP-001)

Requirement cell: keep the current text to the end ("... subject only to the run-length bound of GP-1.") and append:

```markdown
(Added 2026-11-11 by the run-length consensus and its audit, `research/sherlock-runlength-consensus-20261111/AUDIT.md`. The text above is unchanged.) The bound 2^64 is the range of the step counter of SRC-004. It SHALL NOT be a circuit size, and no circuit domain, key, final proof size or on-chain verification cost SHALL depend on the run length. No leaf count, memory address counter, segment index or other circuit constant SHALL make a run of fewer than 2^64 steps unprovable. A step that allocates no memory SHALL consume no memory address. The proving of one run MAY stop and resume under the same key set; the resumed run SHALL give the same `prog_d`, `input_d`, `outcome_d` and `n_steps` as the uninterrupted run. An intermediate proof SHALL NOT be accepted as a run (SRC-006) or as a claim of another run (GP-002). Tested run lengths SHALL be recorded as tested bounds (SAS-001) and SHALL NOT be stated as limits of this requirement.
```

Acceptance cell: keep the current text and append before "(row A33)":

```markdown
; one run of more than one leaf whose allocations exceed one leaf's memory window and that builds a constructor of more than 8 fields across a leaf boundary, with a final proof of the same byte length as a one-leaf run; the same allocating run under three partitions (SRC-023, row A38) and with one stop and resume gives equal `outcome_d` and `n_steps` and one `claim_d`; raw-witness tests at the node constraint boundary accept `h = 2^64 - 1` and refuse `h = 2^64` (SRC-004); a read-only review of the constraints finds no constant that limits leaves, segments or cells below the range of the step counter
```

Source cell: add `SRC-004; SRC-023; SAS-001; runlength consensus and audit 2026-11-11`.

### 5.4 Spec mirror and notes

- Apply 5.3 to `O/specs/sherlock-general-purpose/spec.md:8-10`, and add the new acceptance items to the scenario at `:12-14`.
- Step thresholds (512, 1,024, 4,096), time caps and gate names stay out of GP-1 and GP-001. Put them in new qualification rows under SRC-011 and SRC-015 after the owner settles dissent D-Q5.2 and the leaf is measured.
- The sentence "No ... memory address counter ... SHALL make a run of fewer than 2^64 steps unprovable" is a design constraint on the address width (finding 3). If the owner prefers a 2^64 cell space, replace that sentence by an owner-approved bound and record the decision in `O/dissent.md`.
