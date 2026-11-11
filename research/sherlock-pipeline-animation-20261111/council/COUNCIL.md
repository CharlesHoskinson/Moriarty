# Council synthesis: does the Sherlock design need improvement?

Run: pipeline-anim council verification 20261111. Verifier: non-author (did not write the design or the findings).
Bundle checked: head `680d2b5afec18a4b8686d20d28cdb7a551feee70`; `sha256(ARCHITECTURE.md)` = `9e10bc3c...b054`, equal to the bundle value, so every line number below refers to the reviewed text.

Abbreviations: `ARCH` = `research/sherlock-final-design-20261111/ARCHITECTURE.md`; `DEC` = `research/sherlock-final-design-20261111/DECISIONS.md`; `ML/` = `openspec/changes/sherlock-modular-leaf-20261111/`; `O/` = `openspec/changes/sherlock-vm-consensus-20261107/`; `SB` = `council/bundle/STORYBOARD.md`.

## Member verdicts

- m-3a9f: changes_requested (3 findings: 3 major)
- m-e42d: changes_requested (5 findings: 3 major, 2 minor)
- m-b71c: changes_requested (6 findings: 3 blocker, 2 major, 1 minor)

Council verdict after verification: **changes_requested**. One blocker (C5) and five major items (C1 to C4, C6) hold after a search for text that already closes them.

## Items

| Id | Title | Raised by | Severity after verification | Verdict |
|---|---|---|---|---|
| C1 | PROG canonicality premise has no discharge point on the acceptance path | m-3a9f F1, m-b71c F1 | major | CONFIRMED (m-b71c revision part 1 is insufficient) |
| C2 | By-digest input admission has no schedule for wide constructors and no read-only view | m-3a9f F2, m-b71c F3 | major | PARTIAL |
| C3 | Foreign-verifier phases have no authenticated operand store across units | m-3a9f F3 | major | CONFIRMED |
| C4 | Two seal paths: the M4 run decider skips the `deps_d` check, and the GP-path run entry has no specified head opening | m-e42d F1, m-b71c F2 (part 2) | major | CONFIRMED |
| C5 | `reg_root` is not tied across child slots | m-e42d F2, m-b71c F2 (parts 1 and 3) | blocker | CONFIRMED (node equality); PARTIAL (run-entry part is latent) |
| C6 | No kit owns the claim ledger contract; recommended N-11(a) splits replay state | m-e42d F3, m-b71c F5 | major | CONFIRMED |
| C7 | `rules_d` has no consumer after the leaf | m-e42d F4 | minor | CONFIRMED |
| C8 | MERGE-WORK fallback allows a second encoding of the machine relation | m-e42d F5 | minor | CONFIRMED |
| C9 | `shape_d` has no `(out_a, out_b)` for an FV output; FV payload duplicates `reg_root` | m-b71c F4 | minor | CONFIRMED (main part); PARTIAL (stmt_tag table) |
| C10 | DATA cell packing for `e = 1` needs limb range checks | m-b71c F6 | none | REFUTED |

No finding was INADMISSIBLE in full. No finding narrows GP-1 to GP-4. Where a sub-claim only restates an open decision, the section says so.

---

## C1 PROG canonicality premise has no discharge point on the acceptance path

Raised by m-3a9f F1 and m-b71c F1 (m-b71c rated it blocker).

Evidence (all quotes checked):
- ARCH:1097 (PAGE/ROM row 18): "`ty_d` and constant `vd` in ROM are trusted only through the PROG canonicality premise (`O/design.md:84`, open). | No circuit mutant (premise). Host differential `rom_ty_d_matches_admission`".
- ARCH:1400 (VALUE DIGEST): ROM rows `tbl_types`, `tbl_ctors`, `tbl_consts` "are under the PROG canonicality premise (provider: the program admission claim of `O/design.md:84`, OD-14)".
- ARCH:2276 (INIT): "PROG canonicality is not proved here: it is a premise on `prog_root`, and its provider is the program admission claim of `O/design.md:84`, whose form is open (OD-14, OWNER)". ARCH:2277: "Not provided: a proof that the program is well typed or admitted".
- O/design.md:86 (2.7.4): "PROG canonicality is a premise on `prog_root`, discharged by an admission claim [OPEN: its form]". O/design.md:195: "Constants carry `vd` under the PROG canonicality premise. Halt copies the `vd` of the result."
- ML/specs/sherlock-chip-page-cache/spec.md:37 and ML/specs/sherlock-init-admission/spec.md:26 repeat the premise.
- ARCH:2754-2755 (N-11): the recommended contract takes `prog_d` as call data. SB:2255 shows this.
- ARCH:1096 (row 17) says a nonzero pad row is "refused by the normal-form check of admission", so the design assumes that an admission check exists.

Search for a closing text: `rg -i "admission|ProgCanonical|SHK_ADM"` over ARCH and O/ finds no SEAL, DECIDER or contract check that consumes an admission claim. OD-14 (ARCH:2690) is about the `decl_schema_d` normal form and "the form of the PROG canonicality premise". It does not name the point where the premise is checked. The old per-program path relied on a sealed `program_d` chosen by the deployer (O/requirements.md:32, SRC-007) and on "the parser walk" (O/design.md:50). Neither applies once `prog_d` is call data.

Reasoning: This is not a "Lean open" item. The chain has no step that checks the premise, so any caller can supply a non-canonical `prog_d`. The m-b71c attack holds: a ROM whose `tbl_consts[0]` holds payload `UInt 0` and a `vd` column `X` yields `outcome_d = H(O, 1, 0, X)` through INIT, PAGE/ROM and halt-copies-`vd`. The impact is narrower than m-b71c says. A relying party who recomputes `prog_d` from source gets the canonical digest and never matches the forged one. The harm reaches parties that take `prog_d` as given: a universal `acceptRun` reader, a GP-2 consumer whose input link names the forged producer, and a type-digest spoof across programs through `ty_d`. For that reason the severity is major, not blocker.

m-b71c revision part 1 (verify a constant's `vd` from its children's `vd` columns at read time) is not sound by itself. The child rows' `vd` columns are also unverified, so the forgery moves one level down. Only a check over the whole table, done once per program, closes the gap. m-3a9f's admission certificate and m-b71c part 2 (`SHK_ADM`) have that form.

Revision:
1. ARCH, NODE INTEGRATION, SEAL and SEAL-WITH-DEPS rows (ARCH:1744, 1754-1755) and NODE-003: add the check "the sealed `prog_d` is admitted: a verified SET or SUB child in the seal's free slot (SEAL) has head claim `adm_d = H(SHK_ADM, 1, prog_d)`, or `adm_d` is the first element of the DL (SEAL-WITH-DEPS)". New mutants `node_seal_unadmitted_prog` and `node_seal_admission_other_prog`.
2. New administrative node mode ADMIT-WORK (same pattern as FV-WORK: bounded units, no height change). It walks the ROM tables under `prog_root` and checks, for every row, the constant `vd` recomputed from payload and earlier rows, `ty_d` from the `decl_schema_d` normal form (OD-14), constructor ordinal and arity, reference ranges and canonical padding (rows 17 and 18). It emits `SET` in a new form `ADM(prog_d) = H(SHK_SET_ADM, adm_d)`. Add ADMIT-WORK to the node relation table and to NODE-005. Mutants `admit_const_vd_forged`, `admit_ty_d_forged`, `admit_ctor_arity_forged`, `admit_pad_noncanonical`, `admit_claim_before_done`.
3. PAGE/ROM row 18 (ARCH:1097): replace "No circuit mutant (premise)" with a reference to the ADMIT-WORK mutants. INIT (ARCH:2276) keeps the premise for the leaf and names SEAL as its discharge point.
4. New DECISIONS entry (proposed id K-1): "PROG canonicality is discharged at the seal by an admission claim per `prog_d`, produced by ADMIT-WORK; OD-14 decides only the `ty_d` normal form inside it."

Cost: one admission proof chain per distinct `prog_d`, with work linear in the image size and spread over bounded units, reused by every run of that program. Per run, one extra head opening at the seal (about one hash plus one child slot). Kits touched: NODE INTEGRATION, GP-2 hook (SET grammar gains a fifth form), PAGE/ROM, VALUE DIGEST, INIT (text only). This shares a mechanism with C2, which makes it cheaper to do both.

## C2 By-digest input admission has no schedule for wide constructors and no read-only view

Raised by m-3a9f F2 and m-b71c F3 (m-b71c rated it blocker).

Evidence:
- ARCH:1411 (VALUE DIGEST): "A constructor of arity `m` is opened by the fold, one field per call ... the fields are bound to `vd` only when the completion hash equals `vd`, and a rule may use a field only after that completion". ML/specs/sherlock-value-digest/spec.md:44-45 (VDG-005) has the same rule.
- ARCH:2263-2268 (INIT I1 to I6): I1 no scan; I2 no size bound; I3 bounded work per step; I4 `n_steps` equal to the reference owner-event count of `apply(prog, x)`.
- ARCH:2270: recommended form (b) "a step that inspects an input node opens it with `open_value_node` and appends its record ... its open points are the appends it adds to a step (against the per-rule maximum of 3) and the opening of wide constructors".
- ARCH:600-602 and 619: one rule per enabled slot, disabled slots are the identity, one enabled slot adds one step, "Early close cannot repair a step that does not fit an empty leaf". ML/specs/sherlock-node-integration/spec.md (NODE-005) allows only the eight listed modes.
- ARCH:2259: "`Main(in_fn, in_ty)` requires a present input whose type is `in_ty`. A statement outside these two cases has no Init and the gate is unsatisfiable."
- O/design.md:183-187: builtin forms carry no `ty_d`.

Search for a closing text: the design itself names wide constructors as an open point of OD-17 (ARCH:2270, 2693) and gives X11 the job of measuring "a constructor input of more than 8 fields" (ARCH:3235). No kit supplies a mechanism.

Reasoning, per sub-claim:
- (a) Wide constructor schedule. CONFIRMED. The first step that matches on an `m`-field input constructor needs the completion hash, which takes `m` fold calls. Per-step work is bounded (ARCH:617-619), extra steps break I4, disabled slots must be the identity, and no administrative mode does input work. So a measurement cannot fix this; the design needs a mechanism. GP-2 has the same problem, because a consumer always receives the producer's value by digest (ARCH:1987). m-b71c's top-down peel does not close it: from `acc_m`, field `f_k` costs `m - k` hashes, so field 0 still costs `m` hashes in one step.
- (b) Appends of opened input records into MEM (m-b71c F3 part 2). PARTIAL. The design lists the appends as an open point (ARCH:2270). What it does not state is whether the appends are deterministic (opened once per node or once per traversal) and whether they count against the DataNodes and DataFields caps. Both affect `len`, `Exhausted` outcomes and I4.
- (c) Type admissibility of uninspected subtrees (m-b71c F3 part 4). PARTIAL, minor. ARCH:2259 claims the gate is unsatisfiable for an input outside `in_ty`. Under I1 and with untyped builtin digests, no mechanism checks this. m-b71c's "false Halted claim" attack is weaker than stated: a non-value `val_d` has no preimage, so the claim is about no real input. The text still claims a check that does not exist.
- m-b71c's proposal to put `ty_d` into every `sherlock-value/1` node changes SRC-022 for every value. It is not needed for (a) or (b).

Revision:
1. INIT, input admission (ARCH:2261-2270), and OD-17 option (b): replace "opens it with `open_value_node` and appends its record" with a certified read-only input view. A new administrative node mode VIEW-WORK (bounded units, no height change, same pattern as ADMIT-WORK in C1) walks `val_d` and emits `VIEW(val_d, view_root)`. `view_root` is a Merkle root over records `(path_id, tag, ty_d, c, m, child vd list position)`, each recomputed against `sherlock-value/1`. Field `k` of any node is then one Merkle opening of fixed depth. A record is entered only when VIEW-WORK reaches it, so the work grows with the touched part of the input. Whether the view is built for the whole input or lazily is a prover choice, because the view is canonical per `val_d`.
2. BOUNDARY COMMITMENT: add `view_root` as a body word, set at genesis from witness. CORE reads input nodes through a new read-port class `AuthIn` against `view_root`, with 0 MEM appends and no cap charge, unless X11 shows that the reference charges `Literal(x)` cells, in which case the charge is a constant per read and is stated in the rule contract. This resolves (b).
3. SEAL and SEAL-WITH-DEPS: require a `VIEW(val_d, view_root)` claim whose `val_d` is the opened `input_d` preimage word and whose `view_root` is the body word. Carry it as a DL element, or in the same seal slot as the admission claim of C1. Mutants `seal_view_other_input`, `in_read_outside_view`, `view_record_forged`, `view_claim_before_done`.
4. ARCH:2259 and I-list: add "I7. `in_ty` admissibility is checked on inspected nodes only; an uninspected subtree is carried by digest, and the claim is relative to `val_d`", or have VIEW-WORK check `ty(x) == in_ty` over the whole input when the owner wants a full check. Owner choice: proposed DECISIONS id K-2.
5. VDG-005: keep the bottom-up fold for construction. For inputs, replace "usable only after its opening fold reaches the digest" with "read through `AuthIn` under a certified `view_root`".

Cost: VIEW-WORK proofs linear in the touched input, outside owner steps. Per input read in a leaf, one Merkle path, about 20 to 64 hashes at fixed depth (ESTIMATE), in place of the fold. One extra body word and one seal check. Kits touched: INIT, VALUE DIGEST, CORE (read port), BOUNDARY COMMITMENT, NODE INTEGRATION, GP-2 hook. X11 then measures a mechanism instead of searching for one.

## C3 Foreign-verifier phases have no authenticated operand store across units

Raised by m-3a9f F3.

Evidence:
- ARCH:2164: `fv_state_d = H(SHK_FV, entry_d, phase, cursor, transcript_d, acc_d, pi_d, consumed_d)`. `consumed_d` is "the running digest of every proof element consumed so far".
- ARCH:2170-2174 (subkit table): FV-PARSE outputs "typed element cells", FV-TRANSCRIPT outputs "challenges", FV-ARITH takes "elements, challenges".
- ARCH:2179: "every unit continues exactly the state its predecessor committed; all units consume one proof stream in order, with no element skipped, repeated or left over".
- ML/specs/sherlock-foreign-verifier/spec.md:4 (FVR-001) has the same state schema.

Search: no indexed store, re-read rule or replay protocol is stated in ARCH:2155-2210 or in FVR-001. The mutant list (ARCH:2187) has no case where a later phase uses a value other than the one absorbed earlier.

Reasoning: CONFIRMED. A halo2 verifier absorbs commitments before it squeezes the challenges, and it uses those same commitments later in the opening MSM. Across unit boundaries, cells do not survive. A running digest can be checked only by replaying the prefix, and the "consumed exactly once" invariant forbids a second pass. A later unit therefore has no sound way to obtain an earlier commitment or challenge. This is an interface gap, separate from the unmeasured unit budget (X14). Without a fix, GP-3's general path is incomplete or unsound.

Revision:
1. ARCH:2164 and FVR-001: extend the state to `H(SHK_FV, entry_d, phase, cursor, transcript_d, acc_d, pi_d, consumed_d, work_root)`. `work_root` is a Merkle root (or an append-only indexed log root) of typed verifier records `(index, kind, value)`.
2. FV-PARSE writes each parsed element at index `cursor`. FV-TRANSCRIPT writes each challenge at a fixed schema index. FV-ARITH and FV-FINAL read only through openings against `work_root`. FV-SCHED owns the index schema per `transcript_id`, initialization (empty root at the first unit) and the rule that `DONE` needs every scheduled record.
3. New mutants `fv_operand_substituted` (an MSM term uses a commitment other than the one absorbed) and `fv_challenge_index_swapped`. New Quint invariant `LaterPhaseReadsAbsorbedValue` in `foreignRegistry.qnt`.

Cost: one more word in the FV state sponge (about one permutation per unit), plus one Merkle opening per cross-unit read, inside the unit budget that X14 sizes. Kits touched: FV-SCHED, FV-PARSE, FV-TRANSCRIPT, FV-ARITH, FV-FINAL.

## C4 Two seal paths: the M4 run decider skips the `deps_d` check, and the GP-path run entry has no specified head opening

Raised by m-e42d F1 and m-b71c F2 (part 2).

Evidence:
- ARCH:1744 (SEAL) and ML/specs/sherlock-node-integration/spec.md:25-26 (NODE-003): SEAL requires "`deps_d == EMPTY_DEPS`"; mutant `node_seal_nonempty_deps`.
- ARCH:1582: terminal sealing is owned by "NODE INTEGRATION (SEAL, SEAL-WITH-DEPS) and the M4 run decider".
- ARCH:1842-1843: the M4 path opens `open_state_header` on the root SEG's `post_d`. The listed checks are the `n_steps` range, not Running, and `claim_d`. There is no `deps_d` check and no `h_lo` check in the text, though the mutant `decider_nonzero_h_lo` exists (ARCH:1863). The mutant list has no dependency mutant. DCD-001 (ML/specs/sherlock-decider/spec.md:4-11) has no `deps_d` clause.
- ARCH:1851: "Assumed: ... SEAL refuses Running (GP path); BOUNDARY COMMITMENT authenticates the terminal opening (M4 path)".
- ARCH:1989: "Until then `deps_d` is constant in every run". ARCH:2696 (OD-20): the chain side is specified "so that (b) needs no new node relation", so nothing forces the decider to change when (b) lands.
- ARCH:1871-1879 (skeleton): `open_terminal` is "M4 path"; `decide_run` takes `TerminalCells`. Nothing opens a SET head for the GP-path run entry, although ARCH:1842 lists "GP path: a SET root" as a request.

Reasoning: CONFIRMED. The seal predicates live in two places and the lists already differ. Today the hole is latent, because no rule writes `deps_d`. Once OD-20(b) adds a consume rule, a prover can take an unsealed root SEG to the M4 decider and settle a run whose consumed claim was never discharged. Separately, a GP-path run entry from a `LEAF` or `DEP` SET root has no specified opening of `(prog_d, input_d, outcome_d, n_steps)`. The storyboard placing SEAL in the decider scene (SB:1892, 2068) is presentation, not a design conflict.

Revision:
1. DECIDER (ARCH:1842-1843, DCD-001): the run entry accepts only a SET root sealed by NODE. It witnesses `(prog_d, input_d, outcome_d, n_steps)`, recomputes `claim_d`, and opens the root as `LEAF(claim_d, n_steps)` or `DEP(claim_d, n_steps, cover_root, in_link)` with one hash, then range-checks `n_steps`. On the M4 linear path, one SEAL node runs before the decider. `open_terminal` is removed, and every seal predicate lives only in NODE `seal()` and `seal_with_deps()`.
2. New mutants `decider_seg_root_as_run` (a SEG root offered to the run entry) and `decider_set_head_forged`.
3. If the M4 path must keep the direct SEG opening, the minimum fix is: add "opened `deps_d == EMPTY_DEPS`" and "`h_lo == 0`" to ARCH:1843 and DCD-001, add mutant `decider_nonempty_deps`, and add a note to OD-20 that (b) also changes the decider.
4. DECISIONS (proposed id K-3): "one seal; the run decider opens a SET head".

Cost: one more node proof on the M4 path (about 65 s on the reference host, SB:1653). In the decider, a six-input header sponge is replaced by a claim hash plus a SET head hash, about 2 permutations more (ESTIMATE). The minimum fix costs two equalities and one mutant. Kits touched: DECIDER, NODE INTEGRATION.

## C5 `reg_root` is not tied across child slots

Raised by m-e42d F2 and m-b71c F2 (parts 1 and 3). Severity blocker: m-b71c rated it blocker, m-e42d major; the attack forges any foreign claim and the fix is small.

Evidence:
- ARCH:1735: "`keys_repr` ... equal in every child and the parent". ARCH:1736: "`reg_root`: full field; forwarded unchanged; consumed by the GP-3 hook." No equality across two children is stated.
- ARCH:2048: `reg_root` is "forwarded by NODE INTEGRATION unchanged and read into the decider transcript from contract state".
- ARCH:1970: `FOREIGN(fclaim_d) = H(SHK_SET_FOREIGN, fclaim_d)`, with no registry inside.
- ML/specs/sherlock-node-integration/spec.md (NODE-001 to NODE-007): no `reg_root` requirement and no `reg_root` mutant. The registry mutant `foreign_wrong_registry_root` (ML/specs/sherlock-foreign-registry/spec.md:12) checks the lift against its own lane, not the parent against both children.
- ARCH:1843: the run entry publishes no `reg_root`.

Reasoning: CONFIRMED for the node. In JOIN, DL-STEP or SEAL-WITH-DEPS, a FOREIGN child that proved membership under an attacker's root can sit in one slot, while the parent publishes the contract's root taken from the other slot. The contract's root check (SB:2940) then passes. The run-entry part is PARTIAL. Under built path OD-20(a), an input link must be a `SHK_RUN` claim (ARCH:1985), so no foreign claim affects a run claim. It matters only once mid-run consumption (OD-20(b)) lets a foreign claim enter the `deps_d` chain.

Revision:
1. NODE INTEGRATION (ARCH:1736) and NODE-004: "`reg_root` SHALL be equal in every enabled NODE or FOREIGN child and in the parent; a LEAF child carries none; with no such child the parent takes it from its witness." The constraint is `en_j * (1 - is_leaf_j) * (child_j.reg_root - parent.reg_root) == 0` for j in {0, 1}. Mutant `node_reg_root_mixed`.
2. DECIDER run entry, effective with OD-20(b): refuse a `DEP` head whose cover holds a FOREIGN member, or add `reg_root` to the run-entry transcript and check it against contract state. Record as a dependency in OD-20.
3. Optional, only if N-8 picks a root history in which subtrees lifted under different roots may be joined: `FOREIGN(reg_root, fclaim_d)`, with the contract checking root history at opening (C6). Strict equality from step 1 is sound without it.

Cost: two field equalities per node (a handful of rows) and one mutant. Kits touched: NODE INTEGRATION, GP-3 hook, DECIDER (text).

## C6 No kit owns the claim ledger contract; recommended N-11(a) splits replay state

Raised by m-e42d F3 and m-b71c F5.

Evidence:
- ARCH:1867 (DEC-L2): "`claim_d` settles at most once in the contract set even when it has many paths in `set_root`", which is a property of contract state, assigned to the DECIDER kit.
- ARCH:1850 and DCD-004 (ML/specs/sherlock-decider/spec.md:29-37): no per-claim side effect at collection settlement; at-most-once settlement; `DiamondAncestorOpenedAtMostOnce`.
- ARCH:2754-2755 (N-11): recommended option (a), "two templates: `acceptRun` ... and a claims contract with root acceptance and claim opening".
- ML/proposal.md: the 23 kits include no contract kit; ML/tasks.md:40,43 (F.7, F.10) are gated tasks only.
- O/general-purpose.md:27: GP-002 claims are "accepted once on-chain".
- O/tasks.md:44: ledger cap `MAX_CONTRACT_OPERATIONS = 256` per call, so an opening of unbounded path depth cannot happen in one call.

Reasoning: CONFIRMED. N-11 is an open owner item, but two points are design errors and not just open choices. First, DEC-L2 and the at-most-once invariant are assigned to a circuit kit that cannot enforce them. Second, the recommended option (a) as worded uses two contracts with separate ledgers, so one `claim_d` can be accepted once in each, which breaks "accepted once on-chain". m-b71c's depth point holds: GP-4 allows a DAG of any depth (ML/specs/sherlock-proof-shape/spec.md:4), and a per-call operation cap requires an opening in steps.

Revision:
1. New kit LEDGER CONTRACT (Compact, outside the recursion crates), with a spec `ML/specs/sherlock-ledger-contract/spec.md`. Move DEC-L2, `DiamondAncestorOpenedAtMostOnce` and `EveryJoinedClaimOpens` to it.
2. Interface, on one contract with shared state: `runs: claim_d -> (outcome_d, n_steps)`, `foreign: fclaim_d -> ()`, `verified_roots: Set<Field>`, the registry root (history per N-8).
   - `acceptRun(prog_d, input_d, outcome_d, n_steps)`: SRC-007 with `prog_d` as call data; refuse a `claim_d` already in `runs`.
   - `acceptCollection(reg_root, set_root, shape_d)`: verify, check `reg_root`, insert `set_root` into `verified_roots`. No per-claim write.
   - `descend(root, k openings)`: requires `root` in `verified_roots`, checks up to `K_open` JOIN or DEP openings (a constant sized to the 256-operation cap), and inserts the reached sub-root.
   - `openClaim(sub_root, head preimage)`: one LEAF, DEP or FOREIGN head hash; insert into `runs` or `foreign`; idempotent if the key is present.
   - Invariant: a `claim_d` is in `runs` at most once, whichever entry inserted it.
3. Reword the N-11 recommendation (ARCH:2754-2755): "(a) as two entry points of one contract over one `runs` map".
4. DECISIONS (proposed id K-4).

Cost: no circuit rows. On chain, one Poseidon per path level opened, paid only by whoever needs a claim settled. One new kit and spec. SRC-007's sealed `program_d` question stays with the owner.

## C7 `rules_d` has no consumer after the leaf

Raised by m-e42d F4.

Evidence: ARCH:589 (`S + 1`-input sponge); ARCH:1748 (a LEAF child counts as a SEG without lane 5); ARCH:2106 (leaf shape uses `h_pre`, `steps`); ARCH:1843 (decider publishes four values). `rg rules_d` over ARCH finds no node, shape or decider use. SRC-001 and SRC-003 (O/requirements.md:26,28) fix the lane.

Reasoning: CONFIRMED as a cost observation. Rule selection is already constrained inside the leaf by DEC and CORE, so the lane adds no guarantee after the leaf. Removing it is an edit of SRC-001, which is an owner spec.

Revision: owner choice (proposed id K-5). Either give it a consumer (put `rules_d` into the LEAF base case of `shape_d`, ARCH:2106), or amend SRC-001 so that lane 5 is a reserved zero lane and drop the CORE sponge and its three mutants (ARCH:2935).

Cost: removing it saves about 9 permutations per leaf at S = 16 (about 630 rows, ESTIMATE). Keeping it with a consumer costs one word in `leaf_shape`. Low priority.

## C8 MERGE-WORK fallback allows a second encoding of the machine relation

Raised by m-e42d F5.

Evidence: ARCH:2110: "If the leaf's layout cannot be embedded in the node's configuration, a node emitter of the same rule specification is run against the same differential vectors". ARCH:2102: the inline kernel "constrains the same reference steps as the leaf". NODE-L1 and GP4-L2 assume one CORE contract.

Reasoning: CONFIRMED. Differential vectors check agreement only on the vectors, and a divergence would be a soundness hole in the weaker circuit. No other text requires the emitter to share code with the leaf kits.

Revision: in ARCH:2110, require the inline kernel to be built from the same kit `configure` and `assign` functions as the leaf (CORE, DEC, ALU, RANGE, HASH, STACKS, MEM, backend, VALUE DIGEST, INIT), instantiated in the node's constraint system. Remove the separate-emitter option. If it does not fit, the result goes to the owner under N-6, and GP-4 is not narrowed. Proposed id K-6.

Cost: the kits must be generic over the host constraint system, which their port and bus design already supports (ARCH:70). No rows.

## C9 `shape_d` has no `(out_a, out_b)` for an FV output; FV payload duplicates `reg_root`

Raised by m-b71c F4 (rated major).

Evidence: ARCH:2105: `(out_a, out_b)` is defined for SEG, SET, DL and SUB only. ARCH:1737 includes `FV = 5`. ARCH:1759: FV-WORK outputs FV or a FOREIGN SET. ARCH:1738: the FV payload is `(reg_root, fv_state_d)`, while `reg_root` is already a top-level lane (ARCH:1736). ML/specs/sherlock-proof-shape/spec.md:15 (SHP-002) forbids "a challenge" in `shape_d`.

Reasoning: CONFIRMED for the missing case and the duplicate word. Every FV-WORK node computes `shape_d`, and the rule has no case for it. `fv_state_d` commits transcript state and must not be used. The stmt_tag-per-key-kind sub-claim is PARTIAL: NODE-004 already says one selector binds "its permitted `stmt_tag`" and ARCH:1761 types FOREIGN children as SET, but the table is not written out. Severity minor: a spec-completeness gap with an obvious fill.

Revision: ARCH:2105 and SHP-002: "`(0, 0)` for an FV output; a FOREIGN SET at completion uses the SET rule `(set_root, 0)`". ARCH:1738: FV payload `(fv_state_d)` only, using the node's `reg_root` lane, which C5 constrains. NODE-004: list `LEAF -> {SEG}`, `FOREIGN -> {SET}`, `NODE -> {SEG, SET, SUB, DL, FV}`. Mutant `shape_fv_state_in_shape`.

Cost: one selector term in shape, one fewer payload word; no rows of note.

## C10 DATA cell packing for `e = 1` needs limb range checks

Raised by m-b71c F6.

Evidence and closing text: ARCH:805 states that wide records take consecutive physical cells and that the record layouts "are not frozen here (section 4, OD-11)". ARCH:850 gives the dependency exactly: "With `a = 3` (M-2) and `e = 1`, TOP = 63 ...; `e = 2` would need TOP = 64". DEC A-5 records it: "TOP = 63 holds only if one logical record is one physical cell (OD-11)". ARCH:2687 (OD-11) requires the schema file to be "frozen before CORE rule families start". ARCH:617 (capacity rule) already requires every rule to publish its "range checks" per step, so any packing limbs are counted.

Verdict: REFUTED. The trade-off is named, owned and gated. The finding restates an open decision. Its packing layout is useful input to the OD-11 schema review, not a design revision.

---

## Revisions in apply order

1. **C5** node `reg_root` equality and mutant `node_reg_root_mixed` (NODE INTEGRATION, NODE-004). No dependencies. Smallest and most urgent.
2. **C9** shape case for FV, FV payload without `reg_root`, stmt_tag table. Depends on 1 (payload uses the constrained lane).
3. **C4** one seal: run decider opens a SET head, M4 path gains a SEAL node, DCD-001 updated (or the minimum fix). No dependencies; do before any OD-20(b) work.
4. **C8** inline kernel from the leaf kit code (ARCH:2110). No dependencies.
5. **C1** ADMIT-WORK mode, `ADM` SET form, seal check, PAGE/ROM row 18. Depends on 3 (one seal to change), and on OD-14 for the `ty_d` normal form inside admission.
6. **C2** VIEW-WORK mode, `view_root` body word, `AuthIn` read port, seal check, I7. Depends on 3 and 5 (same administrative-mode pattern and the same seal slot); X11 then measures it. OD-17 stays OWNER.
7. **C3** `work_root` in the FV state and the cross-unit read rule. Independent of 1 to 6; must land before X14 freezes the unit budget.
8. **C6** LEDGER CONTRACT kit and spec, DEC-L2 moved, N-11 reworded. Depends on 1 (registry check), 3 (run entry shape) and 5 (if `ADM` is a fifth SET form, `openClaim` must refuse to open it as a run claim).
9. **C5 part 2** run-entry `reg_root` rule. Depends on 3 and 8; effective only when OD-20(b) is taken.
10. **C7** `rules_d` consumer or reserved lane. Owner decision; independent.

Proposed DECISIONS ids K-1 to K-6 are placeholders for the lead; they do not collide with the R-, E-, L-, M-, A- or O- series in DEC.
