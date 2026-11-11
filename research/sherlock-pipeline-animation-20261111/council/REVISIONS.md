# Revisions applied from COUNCIL.md

Run: pipeline-anim council revisions 20261111. Work order: `council/COUNCIL.md`, section "Revisions in apply order". Nothing was built, tested, proved or committed.

Abbreviations: `ARCH` = `research/sherlock-final-design-20261111/ARCHITECTURE.md`; `DEC` = `research/sherlock-final-design-20261111/DECISIONS.md`; `ML/` = `openspec/changes/sherlock-modular-leaf-20261111/`.

Decision ids: the placeholders K-1 to K-6 are replaced by a new `P-` series (one letter per source section, as the file does; `P-` is unused in DEC and ARCH) in a new DEC section "From the pipeline-animation design check".

Placeholder mapping (COUNCIL.md still cites the K ids): K-1 = P-5 (C1), K-2 = P-6 (C2), K-3 = P-3 (C4), K-4 = P-8 (C6), K-5 = P-10 (C7, OWNER), K-6 = P-4 (C8). C5, C9, C3 and C5 part 2 had no placeholder and got P-1, P-2, P-7 and P-9.

ARCH line numbers cited elsewhere in the design folder are shifted by these edits and are not updated here.

Baseline: the work started on `680d2b5af`. HEAD moved to `3adece369` during the run, and that commit holds only the early C5 section of this file and part of the edits; everything after it is uncommitted in the worktree.

## C5 part 1: node `reg_root` equality

- ARCH, NODE INTEGRATION: Provides `reg_root` (equality rule and constraint); Invariants (new "Registry root" item); Row budget (new bullet, under 10 rows ESTIMATE); Verification (Quint invariant `RegRootEqualAcrossSlots`, mutant `node_reg_root_mixed`).
- ARCH, GP-3 hook: Interface `reg_root` line and Assumed line.
- ML/specs/sherlock-node-integration: new NODE-008 with scenario.
- Decision: P-1. New mutant: `node_reg_root_mixed`. Specified here (not in COUNCIL): Quint invariant name `RegRootEqualAcrossSlots`.

## C9: FV shape case, FV payload, statement-tag table

- ARCH, section 2 recursion conventions: `FV = (fv_state_d)`.
- ARCH, NODE INTEGRATION: `key_sel_j` (permitted `stmt_tag` per key kind); payload FV `(fv_state_d)`.
- ARCH, GP-4 hook: `(out_a, out_b) = (0, 0)` for an FV output, FOREIGN SET uses `(set_root, 0)`; mutant list.
- ARCH, GP-3 general verifier: statement `FV = (fv_state_d)`; `FvCells` without `reg_root`.
- ML/specs/sherlock-proof-shape: new SHP-005. ML/specs/sherlock-node-integration: new NODE-009.
- Decision: P-2. New mutant: `shape_fv_state_in_shape`.

## C4: one seal path (full revision, not the minimum fix)

- ARCH, DECIDER: Requests (SET root sealed by NODE; M4 path runs one SEAL node first), Provides run entry (claim hash plus one-hash `LEAF` or `DEP` head opening), Invariants (new Guaranteed item; Assumed), Row budget (glue, M4 extra node proof about 65 s ESTIMATE), Verification (mutants; positive), Lean DEC-L1 (premise NODE-L2), Code skeleton (`open_terminal` removed, `open_run_head` added), Reads, Dependencies.
- ARCH, NODE INTEGRATION Invariants: "One seal path" item. BOUNDARY COMMITMENT: terminal sealing row, Lean header-opening row, row-budget opening bullet, pinned-header reason. Lead notes item 1 (section 2). OD-13, OD-19, N-2, N-14 cells; X10 row; section 5.2.2 decider scenario and BND requirement title.
- ML/specs/sherlock-decider: DCD-001 forged-statement scenario reduced; new DCD-006. ML/specs/sherlock-node-integration: NODE-003 one-seal sentence. ML/specs/sherlock-chip-boundary: BND-003 (title and seal-only user). ML/tasks.md: X.10.
- Decision: P-3. New mutants: `decider_seg_root_as_run`, `decider_set_head_forged`. Removed from the decider list (now NODE mutants): `decider_running_as_terminal`, `decider_unbound_terminal_payload`, `decider_nonzero_h_lo`. Specified here: positive `pos_decider_set_head_leaf_and_dep` replaces `pos_decider_genesis_terminal_opening`; function `open_run_head`.

## C8: inline kernel from the leaf kit code

- ARCH, GP-4 hook: Inline work bullet (separate emitter removed), Invariants, Verification (build check), Code skeleton (`InlineExecutionKernel` Requires). N-6 recommendation.
- ML/specs/sherlock-proof-shape: new SHP-006.
- Decision: P-4. Specified here (not in COUNCIL): build check `graph_inline_kernel_separate_encoding`.

## C1: program admission (ADMIT-WORK) discharges PROG canonicality at the seal

- Verifier's revision used; the member's read-time `vd` check (part 1) is not applied.
- ARCH, NODE INTEGRATION: statement tag AW = 6 and payload; SEAL mode text and relation-table rows SEAL, SEAL-WITH-DEPS, new row ADMIT-WORK; new paragraph "Program admission (ADMIT-WORK)"; Invariants; Row budget (seal check about 140 rows ESTIMATE; unit budget unmeasured); mutants; Quint invariant; Lean NODE-L4; skeleton (`NodeMode::AdmitWork`, `StmtTag::Aw`, `seal` premise argument, `admit_work`); Dependencies.
- ARCH, GP-2 hook: SET grammar form `ADM`, head claims, members, `DL` gains `prem_root`, DL-STEP premise element, items 5 and 6, `DlCells`, mutant `claim_premise_in_chain`.
- ARCH, GP-4 hook: `AdmitWork` work kind 3; `(0, 0)` for AW. DECIDER: run entry refuses `ADM` roots.
- ARCH, PAGE/ROM items 17 and 18; VALUE DIGEST ROM rows and Assumed; INIT Assumed and Not provided; OD-14 recommendation cell (Owner cell unchanged).
- ML/specs: NODE-005 mode list, NODE-009 tag table, new NODE-010 and NODE-011; CLM-001 (forms), new CLM-007; INIT-003; PAGE-004.
- Decision: P-5. New mutants: `node_seal_unadmitted_prog`, `node_seal_admission_other_prog`, `admit_const_vd_forged`, `admit_ty_d_forged`, `admit_ctor_arity_forged`, `admit_pad_noncanonical`, `admit_claim_before_done`. Specified here (not in COUNCIL): the AW statement and `adm_state_d` layout, the `prem_root` DL word and premise element, mutant `claim_premise_in_chain`, Quint invariant `SealRequiresAdmittedProgram`, Lean NODE-L4.

## C2: certified read-only input view (VIEW-WORK, `AuthIn`)

- The verifier's revision is used, not the member's top-down peel. OD-17 is not resolved: the mechanism is written as the mechanism of form (b), and the Owner cell is unchanged.
- ARCH, section 1.3: new port row `AuthIn`. CORE: read port `read_input` paragraph and skeleton. BOUNDARY COMMITMENT item 5: `view_root` body word under form (b).
- ARCH, INIT: new Init table row `view_root`; admissibility sentence; new I7; candidate form (b) text; Requests; mutant `in_read_outside_view`; Dependencies.
- ARCH, VALUE DIGEST: bounded value opening paragraph (fold for construction; `AuthIn` for inputs under form (b)).
- ARCH, NODE INTEGRATION: tag VW = 7 and payload; SEAL mode text and SEAL row (premise `JOIN(ADM, VIEW)` with an input); VIEW-WORK mode and relation-table row; new paragraph "Input view (VIEW-WORK)"; Invariants; Row budget (seal view check about 280 to 350 rows ESTIMATE; per read 20 to 64 hashes ESTIMATE); mutants; Quint invariant; Lean NODE-L5; skeleton.
- ARCH, GP-2 hook: SET form `VIEW`, head claims, members, premise element, consumer input text. GP-4 hook: work kind 4 `ViewWork`, `(0, 0)` for VW. DECIDER: refuses `VIEW` roots.
- ARCH, section 4.1 OD-17 row (options, recommendation with the open points and the I7 owner question, decided-by); section 5.4 X11 row. ML/tasks.md X.11.
- ML/specs: NODE-005, NODE-009, new NODE-012 and NODE-013; CLM-001, CLM-007; new CORE-013; new INIT-006; VDG-005.
- Decision: P-6 (Adopted as the mechanism of form (b); OWNER for OD-17 and I7). New mutants: `seal_view_other_input`, `in_read_outside_view`, `view_record_forged`, `view_claim_before_done`, and `seal_view_omitted` (specified here). Specified here (not in COUNCIL): the VW statement and `view_state_d` layout, the record and child-vd leaf layout, `read_input`, `VIEW_NONE`, Quint invariant `SealRequiresCertifiedInputView`, Lean NODE-L5.
- FLAG for the lead: COUNCIL C2 puts `view_root` in the body and has the seal compare it, but `open_state_header` returns only `body_d` and the node never re-hashes the body. The text keeps `view_root` as a body word and records the seal read path as an open point of OD-17 (a header word would change the six-input header, OD-19 and X10). NODE-013 says this point must be decided before it freezes.
- Seal case split, specified here: the seal tells "no input" from "input" by the re-hashed `val_d`. Only the absent-input constant (open in the spec, `O/design.md:214`) allows an `ADM`-only premise; INIT then sets `view_root = VIEW_NONE`, which `read_input` refuses. New mutant `seal_view_omitted` (specified here) covers a run with an input that seals without the `VIEW` claim.
- Deviation from COUNCIL's record tuple: COUNCIL gives `(path_id, tag, ty_d, c, m, child vd list position)`; the text uses records `(idx, tag, ty_d, c, m, vd, child_pos)` plus child-vd leaves `(child_pos + k, vd_k, idx_k or NOT_ENTERED)`, so a parent's `vd` can be recomputed when a child is not entered (lazy view). A field read is one opening for the child's digest, plus one more for the child's record when the step inspects it.
- C1 consequence on M4: the M4 SEAL node also needs the `ADM` claim, so M4 needs one ADMIT-WORK chain per distinct program (stated in DECIDER Requests and Row budget).
- FLAG: a lazy view is sound but its `view_root` is not canonical, so boundary values at common cuts are equal across partitions only for one view; recorded as an open point, not resolved.

## C3: authenticated operand store in the foreign-verifier state

- ARCH, GP-3 general verifier: `fv_state_d` schema gains `work_root`; new "Operand store" bullet; subkit table rows FV-PARSE, FV-TRANSCRIPT, FV-ARITH, FV-FINAL, FV-SCHED; cost line (one permutation per unit, one opening per cross-unit read or write, ESTIMATE); Invariants; Quint invariant; mutants; `FvRecord` in the skeleton; GP3-L2 premises.
- ML/specs/sherlock-foreign-verifier: FVR-001 schema; new FVR-005. ML/tasks.md 8.1 to 8.5 (FVR-005 and the new mutants in the gates).
- Decision: P-7. New mutants: `fv_operand_substituted`, `fv_challenge_index_swapped`. New Quint invariant: `LaterPhaseReadsAbsorbedValue`.

## C6: LEDGER CONTRACT kit (one contract, one ledger)

- ARCH, section 2 intro (24 kits); new kit "LEDGER CONTRACT" after HOST CHECKPOINT STORE, in the section 3.4 form with every template heading (Purpose, Interface, Invariants, Row budget, Verification, Lean obligations, Code skeleton, Agent reads, Dependencies, Owned files, Completion evidence).
- ARCH, DECIDER: Invariants (collection entry publishes no per-claim value; ledger properties moved), Verification (Quint invariants moved), Lean (DEC-L2 moved as LDG-L1).
- ARCH, Lean consolidated table: O35 now LEDGER CONTRACT LDG-L1; new O54 (NODE-L4), O55 (NODE-L5); tally 14 proved, 55 open, 69 total. Section 3.1 kit table row and group 7. Section 4.2 N-11 recommendation reworded (Owner mark kept). Section 5.1 and 5.3 kit count; section 5.2.2 diamond scenario; section 5.2.3 FVR and LDG outline entries.
- ML/specs: new `sherlock-ledger-contract/spec.md` (LDG-001 to LDG-005); DCD-004 (settlement moved; scenarios rewritten so it keeps two). ML/proposal.md (24 kits, LDG row, 55 open), ML/design.md (69 rows), ML/tasks.md (F.10 reworded to one contract with two entry points; new 7.2; 6.1 gate without O35; 5.1 gate with O54 and O55).
- Decision: P-8. New tests (contract tests, not circuit mutants), specified here: `ledger_accept_run_then_open_same_claim_is_noop`, `ledger_open_from_three_run_collection`, `ledger_open_depth_beyond_one_call`, `ledger_foreign_claim_open`, `ledger_double_settle_two_entries`, `ledger_descend_unverified_root`, `ledger_open_join_as_claim`, `ledger_open_premise_as_claim`, `ledger_collection_writes_no_claim`, `ledger_descend_over_op_cap`; Quint invariant `ClaimSettledOnceAcrossEntries`; owned paths `tools/sherlock-sdk/sherlock-ledger.compact` and `.test.mjs` (proposed).

## C5 part 2: run-entry `reg_root` rule (effective only with OD-20 option (b))

- ARCH, section 4.1 OD-20 recommendation cell (dependency of (b), options (i) and (ii); Owner cell unchanged); section 4.2 N-8 (when `FOREIGN(reg_root, fclaim_d)` is needed); DECIDER Interface (conditional sentence) and mutant list.
- ML/specs/sherlock-decider: new DCD-007 (conditional on OD-20 (b)). ML/tasks.md 6.1 range.
- Decision: P-9. New mutant (specified here, conditional on OD-20 (b)): `decider_dep_foreign_other_reg_root`. `reg_root` is not added to the four published run values.

## C7: `rules_d` consumer (not decided)

- ARCH, section 4.1: new row OD-23 with the two options and an OWNER mark. No kit text changed.
- Decision: P-10 (OWNER).

## C10

- Refuted; no change.

## Validation

- `openspec validate sherlock-modular-leaf-20261111 --strict`: valid (run through `mise exec node@26.13.0 --`, because the shell's default node had no `openspec`). First runs after C5 to C8 gave four warnings (requirement text over 500 characters in DCD-001, NODE-004, SHP-002, SHP-003); the additions were moved into new requirements (NODE-008, NODE-009, DCD-006, SHP-005, SHP-006) and the change passes with no warning.
- `sherlock-vm-consensus-20261107` was not touched, so it was not re-validated.
- Checks: no em or en dash in added text; no process words in ARCH or specs; the Owner cells of OD-14, OD-17 and OD-20 are unchanged; GP-1 to GP-4 are not narrowed; one key set for every program.

## Cross-reference notes for the lead

- ARCH line numbers cited in other files of the design folder (and in COUNCIL.md) are shifted.
- ARCH section 5.2 outline lines for NODE and SHAPE are not re-synced with the new requirements; the specs are authoritative. Section 6 history tables still say 23 kits by design (they record past states).
- New spec requirement ids: NODE-008 to NODE-013, DCD-006, DCD-007, SHP-005, SHP-006, CLM-007, CORE-013, INIT-006, FVR-005, LDG-001 to LDG-005.
