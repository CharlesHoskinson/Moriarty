# Review: outcome-only publication (C6) and ledger 9 fit of the chip leaf design

Run `sherlock-c6-outcome-ledger9-review-20261110`, 2026-11-10. Brief: `BRIEF.md`. Member answers: `fable/answer.md`, `astra/answer.md`, `carbon/answer.md`. Each member answered both questions before it saw another answer.

Paths are relative to the repository root. `D` is `openspec/changes/sherlock-vm-consensus-20261107`. `L9` is `repos/midnightntwrk/midnight-ledger` at tag `ledger-9.1.0.1` (`a2b30128`). `REG` is the cargo registry source directory. No build, test, keygen or proof was run in this review; every row count is quoted from the repository.

## Verdicts

| Question | Fable | Astra | Carbon | Review verdict |
|---|---|---|---|---|
| Q1 Outcome-only publication for C6 | adopt with changes | adopt with changes | adopt with changes | **adopt with changes** |
| Q2 Runs on Midnight ledger 9 | yes with changes | yes with changes | yes with changes | **yes with changes** |

## How the members were run

- Fable (Claude Fable 5.2): lead. Wrote `fable/answer.md` before it opened the other answers. After reading them it appended one "Correction" section (a wrong proof size, and the 0.8.2 lockfile fact); the text above that section was not changed.
- Astra (GPT-6 Astra): `codex exec -m gpt-6-astra -c model_reasoning_effort=high -s read-only --ephemeral`, the CLI's own sign-in, answer written by `-o` to `astra/answer.md`. Exit 0, one attempt.
- Carbon (Gemini 4 Carbon): `agy --model gemini-4-carbon-high`, headless, JSON output, the lead saved `.response` to `carbon/answer.md`. Four attempts, all on `gemini-4-carbon-high`; no other model was used and no quota (429) error occurred.
  1. One-line smoke test: passed.
  2. Full brief, `write_file(*)` denied: empty answer after 18 s, because headless mode refused every shell command (`denied_actions: command`).
  3. Full brief with read commands allowed (`rg`, `git`, `sed`, `cat`, `ls`, `grep`, `head`, `tail`, `wc`, `find`, `awk`, `cut`): failed after 29 minutes with `stream idle timeout (5m0s)`, no answer, 1.6 M input tokens.
  4. The brief split into two scoped runs (Q1 only, Q2 only), with explicit command rules in the prompt. A first launch of these was stopped by the lead after 5 minutes (the Q2 run had already ended empty on a refused command) so that `fable/answer.md`, `astra/answer.md` and the draft `review.md` could be moved out of the checkout; they were restored after Carbon finished. The relaunch succeeded: Q1 `SUCCESS` in 1,006 s, Q2 `SUCCESS` in 488 s, no denied action. `carbon/answer.md` is the two responses, Q1 then Q2, unedited.
  Deviations from the brief: Carbon answered the two questions in separate runs; it had shell read commands, not a no-tool plan mode; file writes were denied. Carbon's text contains no term that exists only in the other answers (checked by grep). `git status` of the checkout and of the ledger clone was unchanged after each run. Attempt 3 overlapped by 24 minutes with the time `fable/answer.md` and `astra/answer.md` were already on disk; it returned no answer, so nothing from it is used.

## Q1 Outcome-only publication

### Positions

**Fable: adopt with changes.** Outcome-only publication is sound and necessary, but not sufficient alone. Changes: remove `fin_d` from every public place (public values, `stmt`, `acceptRun` arguments) and replace `init_d` by `input_d`; decide every `Rejected` and `Exhausted` on logical counters only; use one value digest for `input_d` and `outcome_d`; allow only sealed claims `(prog_d, input_d, outcome_d)` across runs. Replace the `val_d` of `D/design.md` §3.2(5) by a Merkle digest over the value tree, computed at allocation and carried with the reference. SI-13 and SI-14 hold unchanged on monotone addresses; SI-12 (contiguous field slices) cannot be met under C = 8 and stays open.

**Astra: adopt with changes.** "Outcome-only publication is the right semantic boundary. It does not, by itself, make M3 sound and complete. Adoption must include canonical value commitments, relocation-preserving execution, partition-independent resource accounting, and replacement of the current contract statement." Astra asks for a stated logical state and a relocation refinement (each chip step refines one logical step; a leaf close leaves the logical state unchanged), a recursive Merkle value digest `SherlockValue/1` maintained when each value is created, and dense logical identifiers kept beside the sparse physical addresses.

**Carbon: adopt with changes.** "Outcome-only publication is the right architectural boundary for M3 segmented memory", but "changing the published digest in isolation is neither sound nor complete until five coupled defects in the decider statement, internal slice addressing, resource-exhaustion guards, canonical value digest definition, and GP-2 input admission are fixed." Carbon calls the slice addressing a completeness blocker and the exhaustion guards a soundness blocker, proposes an inductive digest `val_d` computed when each `DATA` cell is appended, linked slice cells in place of `start + k`, a program-independent type digest `ty_d`, and an input-loading prefix that verifies the input DAG bottom-up.

### Agreements

1. Adopt, with changes. All three members say so. No member rejects, and no member accepts the decision as written.
2. `fin_d` in `stmt` breaks replay refusal under D3. `acceptRun` refuses a replay by `stmt` (`tools/sherlock-sdk/sherlock-run.compact:39-46`), `stmt` hashes `fin_d` (`crates/mori-recursion/src/run_decider.rs:8-12,289-301`), and `fin_d` hashes `lanes_d`, a free witness over the raw carrier (`run_decider.rs:286-288`; `crates/mori-backend/src/sherlock_c6_witness_stream.rs:216-241`). With partition-dependent addresses one run has many valid `stmt` values, so the same run can be accepted again. Publishing a canonical result beside `fin_d` does not repair this; `fin_d` must leave the statement. (All three members found this independently; Carbon: "YES (critical replay bug)".)
3. The outcome cannot change with the partition only if the semantic resource accounting is logical. Physical address exhaustion must never become `Exhausted(kind)`; it may only mean "no proof".
4. The address space of D3 is a completeness limit that this decision does not repair: `base / 8 < 2^17` (`research/sherlock-processor-analogy-20261110/experiments-4.md:148-149`), every leaf takes one segment, a zero-append leaf too (`qa/gemini-quint/chipleaf-pairrow-m3/verdict.md:41`), and the node requires at least one step per leaf (`crates/mori-recursion/src/ivc.rs:24,360-372`). A run has fewer than 2^17 leaves and 2^20 cells; GP-1 states a bound of 2^64 steps (`D/general-purpose.md` GP-1).
5. The value digest is a recursive Merkle digest over the value as a tree. References (`RefC::Const`, `RefC::Data`) are encoded as the value they refer to. Sharing is erased. A first-visit numbering of a DAG is rejected, because structural equality ignores sharing.
6. The preferred mechanism is a digest established when the immutable value is created, read once at halt. A traversal at halt is the alternative; it costs a memory opening per visited record.
7. Node and leaf digests (`pre_d`, `post_d`, `cur_d`, `init_d`) may stay address-dependent as internal proof inputs. A claim that another run consumes is `(prog_d, input_d, outcome_d)`, checked in circuit. The GP-4 shape commitment must not be built from state roots or addresses, and must not be a run identity.

   Lead note on GP-4 (found after the answers, by reading `research/sherlock-gp-architecture-20261110/architecture.md:96-101,349-353,367`): the study's public root statement is `[reg_root, set_root, n_claims, shape_d]`. `set_root` is content-addressed over claims and "ignores how a run's segments were bracketed". `shape_d = H(SHAPE, relation tag, slot bits, child shape_d values, local statement)` binds the proof graph "including segment bracketing", so it is partition-dependent by design, and it is public. That is compatible with this decision because settlement is by claim ("a claim identifier is the nullifier"), not by root or shape. One point is open: if the "local statement" hashed into `shape_d` contains `pre_d` or `post_d` of a segment, then a raw-address digest reaches a public value. The review asks that `shape_d` take heights and claim content, not state digests [OPEN: the study does not define "local statement" at that line].
8. Raw `TypeIdC` and `CtorIdC` must not be hashed: beyond the builtin rows they are table indices (`crates/mori-core/src/sherlock/helper_machine/shared/adt.rs:4-13,32-41`).
9. SRC-006, SRC-007 and SRC-021 must be replaced or amended, and the contract, template and key cascade are new.

### Dissent (kept)

**D-1 SI-12 to SI-14: dense logical identifiers, or addresses as references.**
- Astra: "retain dense logical identifiers and authenticate their mapping to sparse physical storage. This preserves the existing reference invariants while changing the chip representation." SI-12: "logical field blocks retain exact ownership and field order. Their physical records may span segments through an authenticated mapping." Fresh logical identifiers "remain equal to logical active counts; they are not required to equal physical append addresses." Astra adds that these checks "need a logical-to-physical refinement, not deletion".
- Fable: keep the address as the reference. SI-13 and SI-14 are order checks and hold on strictly increasing addresses. The fresh-id rule becomes "the reference is the append address". SI-12 is open: a field slice written by microsteps can cross a leaf close, and a leaf appends at most 8 cells, so a slice longer than 8 is never contiguous. This assumes that field cells share the one address space of D3 (one `len`, one `mem_root`, rows `(chip, key, a, b, c)` with one address column, `crates/mori-chip/src/mem.rs:97`; `D/design.md` §2.7.8 D3). If the final chip gives each table its own memory, the argument holds per table with the same C. An authenticated id-to-address map is a second memory structure with its own openings; its cost is not measured.
- Carbon: sides against the logical identifier ("Not canonical by itself ... `PAGE` reads need an address-translation table `logical_id -> m3_addr`", ESTIMATE +34 walk permutations, 2,380 leaf rows) and gives a concrete fix for SI-12: "Store inline child references directly in the `MEM` row for arity `<= 2` (`Leaf`, `Cons`, `Branch`), and represent variadic `Args` (`value_slots`) and ADT `Constructor` field lists as immutable cons-linked cells `(TAG_SLICE, elem_ref, prev_link)` ... Replace `new_id == active.*` with `new_addr == running_len`. No rule ever computes `addr + k`." Carbon cites the rules that compute `start + k` today (`owner_runtime.rs:2990`; `owner_adt_runtime.rs:1585,1601`; `families/structural.rs:637`; not re-read by the lead) and notes that frame pushes in one address space also separate slice cells inside one leaf.
- Two members (Fable, Carbon) keep addresses as references; one (Astra) keeps dense logical identifiers. All three agree that SI-12 cannot stay as written and that no fix is measured. The verdict makes Carbon's linked-slice form the candidate to measure first, keeps Astra's mapping as the alternative, and makes the task block D3 adoption for the C6 machine, not the publication decision.

**D-2 Type identity inside a constructor digest.**
- Astra: hash `p, t, c`: "`p` is the canonical program identity that owns its nominal type", `t` "its canonical admitted type-table identifier", `c` the source-order ordinal. "This program-scoped type identity prevents unrelated declarations with the same local constructor number from colliding." For GP-2: "Cross-program nominal types require an explicit authenticated mapping or preservation of the producer's namespace."
- Fable: hash a program-independent type digest `ty_d` and the ordinal, so that program B can consume a value of program A without a mapping. The definition of `ty_d` is open.
- Carbon: program-independent, and it defines one: `ty_d(UInt) = H(DOM_TY, 0)`, `Bool` 1, `List` 2, `Tree` 3, and `ty_d(Adt(decl_schema_d, [A_0..A_{p-1}])) = H(DOM_TY, 4, decl_schema_d, p, ty_args_d)`, "where `decl_schema_d` is the canonical Poseidon digest of the ADT declaration's normalized constructor signatures (independent of program-local `TypeIdC` row position)". The normal form behind `decl_schema_d` is not given.
- Two members (Fable, Carbon) want a program-independent type digest; Astra wants the producer's program identity in the digest. The verdict takes the program-independent form, because GP-2 acceptance (a) has program B consume a value of program A; Astra's dissent stays, and the normal form of `decl_schema_d` is open and must be fixed before any key is frozen.

**D-3 Step count in the statement.**
- Astra: the claim is `(encoding_version, prog_d, input_d, outcome_d)`; the step count "can remain optional semantic metadata if it counts logical events, excluding leaf padding and publication-hash work". The whole public statement need not be equal across aggregation shapes: "Requiring the entire public statement to be identical across all shapes would conflict with GP-4."
- Fable: keep `n_steps` in `stmt`; it is deterministic for `(prog_d, input_d)`.
- Lead note: the GP architecture study puts the step count inside the claim (`out_d = H(outcome_kind, value_d, n_steps)`, `research/sherlock-gp-architecture-20261110/architecture.md:98`). `n_steps` is a function of `(prog_d, input_d)` if it counts logical steps only, so all three forms are sound. Resolved for the verdict: the claim is `(version, prog_d, input_d, outcome_d)`, because the owner's definition of the GP-2 claim is `(prog_d', input_d', outcome_d')` (`D/general-purpose.md` GP-2) and Astra's version field is adopted. `n_steps` is published beside it. `architecture.md:98` then needs the matching edit (`out_d` without `n_steps`).

**D-4 Cost of the cached digest.**
- Astra: "Its total lifetime cost may exceed reachable-only finalization because it also hashes values that never reach the result. Both approaches require measurement."
- Fable: the traversal is linear in the unfolded tree unless it memoizes, and it adds steps after the last program step; the cached digest is the only mechanism with zero halt cost. Fable agrees that both need measurement.
- Carbon: rejects a traversal at halt ("Fails completeness on shared DAGs (`O(2^depth)` rows) unless an auxiliary visited map is proved in write-once `MEM`") and adopts the digest at creation.
- Kept as a measurement task. No member has a measured number for either mechanism.

**D-5 Shape of the constructor digest, and smaller encoding differences.**
- Carbon folds the fields: `acc_0 = 0`, `acc_{k+1} = H(DOM_VAL, 7, acc_k, val_d(f_k))`, then `H(DOM_VAL, 6, ty_d(T), ctor_ord, m, acc_m)`. Reason: a constructor is built one field per step, so "Every step has bounded constant HASH work regardless of constructor arity `m`."
- Fable and Astra hash all fields in one preimage.
- Resolved for the verdict in Carbon's direction: the Build helper links one field per step (`owner_runtime.rs:2997-3015`, re-read by the lead), so the fold is the form the circuit can compute with constant work per step.
- Also Carbon: `UInt` as two 64-bit limbs (Fable and Astra: one word below 2^128); `outcome_d` for `Halted` carries `ty_d(v)` beside `val_d(v)`; `stmt` contains `n_steps` (as Fable; Astra not). The verdict uses one word for `UInt` and no top-level type word, and notes that Carbon's top-level `ty_d` is the safer choice if a later value form loses its tag.

**D-6 Is the leaf partition allowed in the GP-4 shape?**
- Carbon: no. "Bind only the inter-claim DAG publicly"; "intra-run leaf partition boundaries and internal segment digests (`pre_d`, `post_d`) SHALL remain existential and hidden".
- Astra: the shape may differ between proofs of the same claims; it must not be a run identity and must not be built from state roots.
- The GP architecture study binds segment bracketing into the public `shape_d` (lead note under agreement 7). Not resolved here: it is a decision on `architecture.md` GPR-016. The EARS text below forbids state digests in the shape and forbids deciding acceptance on it, which both positions accept; whether bracketing is public stays with GPR-016.

### Verdict on Q1: adopt with changes

The owner's decision is adopted with these changes. Each is a condition of adoption.

1. **Statement.** The public values are `[prog_d, input_d, outcome_d, n_steps]`. The claim and replay key is `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`; `n_steps` is published beside the claim and is not in it (D-3). `fin_d` and `init_d` leave the public values, the `stmt` and the `acceptRun` arguments.
2. **Logical accounting.** `Rejected(code)` and `Exhausted(kind)` are decided on logical counters and on limits carried in `input_d`. No outcome is derived from `len`, `fp`, `tp`, a segment index or the free address space.
3. **One value digest** for `input_d` and `outcome_d` (below), so that GP-2 can equate them.
4. **Sealed claims only.** Another run consumes `(prog_d, input_d, outcome_d)` opened from a terminal state in circuit. A segment statement or a state digest is never a claim of another run.
5. **Refinement obligation** (from Astra). A Quint model and tests state the logical state and prove that a leaf close does not change it; the model's reference must not use per-leaf allocation, because the present one does and therefore does not catch the partition (`qa/gemini-quint/chipleaf-pairrow-m3/verdict.md:41`).
6. **Open, named, not solved by this decision:** SI-12 under C = 8 (D-1); the 2^17 leaf ceiling against GP-1 (agreement 4); the type identity (D-2); the Init mechanism for an input under the new digest.

Answers to the brief's questions:

- Can two runs of the same `(prog_d, input_d)` with different partitions publish different outcomes? Today, under D3: yes for the statement (different `fin_d`, so different `stmt`), no for the u128 `result`. After the changes: no, given a deterministic machine, opaque addresses, logical accounting and the digest below. This is a proof obligation (change 5), not yet a checked result.
- Can a prover choose the partition to change acceptance? Today, under D3: yes, a second acceptance of the same run. After the changes: the prover can only fail to prove a long run (address ceiling); it cannot change the outcome.
- What else public depends on raw addresses? `fin_d` and, in form, `init_d` (public values and `stmt` of `acceptRun`); the node instance (`cur_d`, `init_d`) and the leaf statement (`pre_d`, `post_d`), which stay internal; diagnostics that print `Data(i)`. GP-2 claims do not, if conditions 3 and 4 hold. The GP-4 `shape_d` of the architecture study depends on the partition by design (it binds segment bracketing); it must not take state digests and must not decide acceptance (agreement 7, lead note).

### The canonical encoding and its cost

`sherlock-value/1`. `H` is the Poseidon of zk-stdlib 2.4.0 over the BLS12-381 scalar field, with the input length in its initial state (`REG/midnight-circuits-7.3.0/src/hash/poseidon/poseidon_cpu.rs:131-133`). `V = dom("SHK_VALUE_V1")`, `O = dom("SHK_OUTCOME_V1")`. Every value preimage starts with the header `[V, 1, tag, arity]` (Astra's header; it makes the encoding injective and versioned).

| Value | tag / arity | Digest `vd` = `H(header, ...)` with |
|---|---|---|
| `UInt n`, `0 <= n < 2^128` | 0 / 0 | `n` |
| `Bool b`, `b` in {0, 1} | 1 / 0 | `b` |
| `Nil` | 2 / 0 | nothing |
| `Cons(h, t)` | 3 / 2 | `vd(h), vd(t)` |
| `Leaf(x)` | 4 / 1 | `vd(x)` |
| `Branch(l, r)` | 5 / 2 | `vd(l), vd(r)` |
| `Constructor` of type `T`, ordinal `c`, fields `f_0..f_{m-1}` | 6 / m | `ty_d(T), c, acc_m`, with `acc_0 = 0` and `acc_{k+1} = H(V, 1, 7, 2, acc_k, vd(f_k))` (D-5) |
| a reference, `Const(i)` or `Data(i)` | none | the `vd` of the value it refers to |

- The encoding is exact for the six builtin forms and for references. For `Constructor` it is exact except for `ty_d(T)`: the verdict takes Carbon's `ty_d` (D-2), whose `decl_schema_d` has no defined normal form yet. The reason a raw id cannot be used: `TypeIdC` is "canonical" only inside one program (`helper_machine/shared/adt.rs:6-13`: the builtin rows 0 to 3 are fixed, "every other TypeId keeps its ID"). GP-2 acceptance (a), program B consuming a value of program A, is the test that decides D-2.
- Shared structure: the digest equals that of the unfolded tree; it is computed once per stored value.
- Not a value: a dangling reference, a padding record, a scalar out of range, a wrong arity.
- Outcome: `outcome_d = H(O, 1, 0, vd(v))` for `Halted(v)`; `H(O, 1, 1, code)` for `Rejected(code)` with `code` a range-checked u8; `H(O, 1, 2, kind)` for `Exhausted(kind)` with `kind` in 0..7 = Fuel, DataNodes, DataFields, EnvironmentCells, HelperTasks, HelperWork, SerializationBytes, LogicalCap (`helper_machine/shared.rs:367-378`). `Running` has no `outcome_d`.
- Input: `input_d = Poseidon(SHK_IN, val_d, fuel_lo, fuel_hi, stack_limit)` as in SRC-021, with `val_d = vd(x)`.
- It replaces the `val_d` of `D/design.md:166-167` (roots of a dense re-serialization), which has no `Constructor` case, covers lists of UInt only, exposes `n_d`, `n_f` and `ref_x`, and would need a copy of the result into dense tables at halt.

Computation in circuit: the step that creates a value computes its `vd` from its tag and the `vd` of its fields, and binds it to the write-once record; constants carry `vd` under the PROG canonicality premise; Halt copies the `vd` of the result. Fable proposes to carry `vd` with every reference (a reference is `(addr, vd)`) so that no child record is opened at construction; Carbon stores `(ty_d, val_d)` in the data record and reads it once at halt; Astra requires only that each child digest is authenticated. Which one is cheaper depends on the record width and on how a child's digest is read, and was not measured. For the input, Carbon proposes a loading prefix before `Eval(ENTRY)` that appends and verifies the input DAG bottom-up; Fable proposes that or a lazy unfold. The Init check itself must stay free of an input scan (`D/design.md:164`).

Cost, all ESTIMATE, on the measured 70 rows per Poseidon permutation and 2 absorbed words per permutation (`D/design.md` §2.7.11: 87 permutations, 6,090 rows; `experiments-4.md:157`):

| Item | ESTIMATE |
|---|---|
| UInt, Bool, Leaf (5 words with header) | 210 rows |
| Nil (4 words) | 140 rows |
| Cons, Branch (6 words) | 210 rows |
| Constructor with m fields | 210 rows per field step (6 words) and 280 rows at completion (7 words); constant per step (Carbon) |
| Outcome wrapper (4 words) | 140 rows, once per run |
| Worst leaf (Carbon) | 8 data appends in one leaf: about +1,680 rows, 6,511 to about 8,191, so k = 14 if the usable rows of k = 13 are exceeded |
| Average per step | at most 0.22 allocations per step (`D/design.md` §2.7.10 census) x 210 = 46 rows on 407 rows per step (6,511 / 16): at most +11% leaf rows; k = 13 holds on the census average, else the leaf closes early or the key is k = 14 (gate `k_leaf <= 16`) |
| Node (823,466 rows) | unchanged if no column is added; 7,298 rows per added advice column, 8,189 per equality column (`D/design.md` §2.7.11 fit); headroom 126,534 rows |
| Decider (929,166 rows, k = 20) | under 1,000 rows of change; k = 20 holds |
| Halt | zero beyond the wrapper for the cached digest. A traversal costs about 1,400 rows per opened record of an earlier leaf (depth-20 opening), plus the record hash |
| Scalar-only first step | one dereference and one hash at Halt (about 210 rows per run), no change to references; the format above must still be fixed first |

Without the 4-word header (Fable's own table) the numbers are 140 rows for scalars and pairs. The header is kept for versioning.

### EARS text

Replace or add in `D/requirements.md` and the matching `D/specs/*/spec.md`.

**SRC-006 (replace).** The run decider SHALL verify the last node proof under the node key as constants, pin `self_vk_repr`, fold and pair the carried accumulator in circuit, open the final state digest as a terminal state (`Halted`, `Rejected` or `Exhausted`) whose outcome tag and payload are derived from the proved state, compute `outcome_d` from that tag and payload, and expose exactly `[prog_d, input_d, outcome_d, n_steps]`. The decider SHALL NOT expose `fin_d`, `init_d`, `pre_d`, `post_d`, `mem_root`, `lanes_d`, `len`, a reference, or any value that depends on a memory address. If the final state is `Running`, then the decider SHALL refuse.

**SRC-007 (replace).** When `acceptRun` accepts a run, it SHALL bind `prog_d`, `input_d`, `outcome_d` and `n_steps` to the verified call transcript, SHALL read the sealed `program_d` into the transcript so that a proof of another program is refused, SHALL refuse a `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)` already in `runs`, and SHALL store `outcome_d` and `n_steps`. `input_d` SHALL be call data bound by the proof, not a sealed field. A change of leaf partition, proof randomness, physical layout or aggregation shape SHALL NOT create a new `claim_d`. The contract SHALL NOT take or store a state digest.

**SRC-022 (new).** The published outcome of a run SHALL be `outcome_d = H(O, 1, 0, vd(v))` for `Halted(v)`, `H(O, 1, 1, code)` for `Rejected(code)` and `H(O, 1, 2, kind)` for `Exhausted(kind)`, where `vd` is the value digest `sherlock-value/1` of this review. The digest of a reference SHALL be the digest of the value it refers to. A table index, an address, a type-table index or a constructor-table index SHALL NOT be an input of `vd`. The proof SHALL authenticate every value record, constructor ordinal, field position and child digest used to compute `vd`.

Note on SRC-007: the sealed `program_d` stays, so this contract still serves one program, as today. `init_d` was sealed because the run had no input; with `input_d` as call data one contract serves every input of its program. One contract for every program (GP-1) needs `prog_d` as call data too; that is the GP-1 contract change and is not part of this decision.

**SRC-023 (new).** If two accepted proofs have the same `prog_d` and `input_d`, then their `outcome_d` and `n_steps` SHALL be equal for every choice of leaf boundaries. When two value representations differ only in constant or data storage, addresses, padding, or sharing of equal subvalues, their `vd` SHALL be equal. Acceptance: one program proved under at least three partitions (all leaves full, one step per leaf, mixed) gives equal `prog_d`, `input_d`, `outcome_d` and `n_steps`, and the second submission is refused as a replay; a shared and a duplicated structure give one `vd`.

**SVM-0xx (new, opaque addresses).** No machine rule SHALL compute a UInt, a Bool, a branch condition, an error code, an exhaust kind or a published value from a memory address, from `len`, `fp`, `tp` or a segment index. When a leaf closes, fuel and every logical resource counter SHALL remain unchanged. If physical proving capacity is unavailable, then the prover SHALL NOT encode that condition as a `Rejected` or `Exhausted` outcome.

**SVM-0xx (new, refinement).** When an enabled chip step executes, its decoded logical post-state SHALL equal the reference transition from its decoded logical pre-state. When a leaf closes, its decoded logical state SHALL remain unchanged.

**SVM-0xx (new, value digest invariant).** When a step creates a data value, it SHALL compute that value's `vd` from its tag and the authenticated `vd` of its fields and SHALL bind it to the value's write-once record. A digest supplied by the host without that constraint SHALL NOT be accepted.

**SRC-021 (amend).** Replace the definition of `val_d` by: "`val_d` SHALL be `vd(x)` of `sherlock-value/1` for an input `x`." Add: "When the first execution segment begins, the proof SHALL authenticate the input value, the dependency claims and every resource-policy parameter that can affect the outcome." The mechanism that relates `val_d` to the initial memory is open (lazy unfold or load steps); it SHALL NOT scan the input in the Init check (kill criterion of `D/design.md:164`).

**GP-001 (add).** "When a supported run exceeds one leaf's storage or hashing capacity, the prover SHALL continue through authenticated memory and recursive decomposition under the same key set, subject only to the run-length bound of GP-1."

**GP-002 (add).** "A consumed claim SHALL be a sealed claim `(prog_d', input_d', outcome_d')` opened from a terminal state in circuit; a segment statement or a state digest SHALL NOT be accepted as a claim of another run. When the consumer uses the value behind `outcome_d'`, it SHALL verify the `sherlock-value/1` opening."

**GP-003 (add).** "When an admitted foreign proof contributes a claim, the node SHALL verify registry membership and apply the authenticated statement adapter to the verified foreign public inputs; an adapter SHALL NOT relabel a foreign field as a value digest without that binding."

**GP-004 (add).** "The shape commitment SHALL bind the proof graph through address-free descriptors (relation tags, slot bits, heights, claim digests, child shape digests). A physical state commitment SHALL NOT be an input of the shape commitment. The shape commitment MAY differ between two proofs of the same claims; acceptance, replay refusal and consumption SHALL be decided on claims and SHALL NOT be decided on the shape commitment."

No adopted EARS text is given for SI-12, because no form is measured under C = 8. Carbon's candidate text (its SVM-002, kept here as the form to test): "While a run is between canonical boundaries, every accepted step SHALL preserve `StepInv` over write-once segmented memory (tagged non-zero active records, zero-tuple segment padding refused on read, strict topological ordering `child_addr < owner_addr`, linked slice and environment chains with `prev_link < cell_addr`, and per-data-record canonical value digest `val_d`)", with the scenario "WHEN a multi-argument call or multi-field constructor spans a leaf boundary where the first leaf closed with `used < C` padding rows, THEN the linked slice chain proves and evaluates to the identical value and `outcome_d` as an unsplit leaf, and any witness reading a zero-padding row is refused." SI-13 and SI-14 keep their text; whether they range over addresses or over logical identifiers follows D-1. The fresh-id checks (`helper_machine/shared/owner_runtime.rs:3021-3028`) and the codec bound `index < max_data_nodes` change with the same decision.

### Design text

`D/design.md` §2.7.11, replacing the paragraph `[OPEN, C6: address partition]`:

> C6 decision (2026-11-10, owner, after review `research/sherlock-c6-outcome-ledger9-review-20261110/review.md`). Execution state commitments (`pre_d`, `post_d`, `cur_d`, `mem_root`, `len`, `fp`, `tp`, stored links) bind the physical witness and stay inside the recursion: they are never public inputs of the on-chain proof, never part of a replay key, and never part of a claim that another run consumes. The external statement of a run is `[prog_d, input_d, outcome_d, n_steps]`; the claim is `(version, prog_d, input_d, outcome_d)`. `outcome_d` follows `sherlock-value/1` (SRC-022), which erases addresses and sharing. The leaf partition may change physical commitments; it SHALL NOT change the logical execution, the resource accounting, `outcome_d` or the claim. Reason: under D3 the physical commitments depend on where the prover closes leaves, and the replay key of `acceptRun` hashed `fin_d`. Open: SI-12 under C = 8; the ceiling of 2^17 leaves per run against GP-1; the type identity in a constructor digest; the Init mechanism for an input.

`D/design.md` §3.3: replace "opens `cur_d = Poseidon(SHK_SD, HALTED, result, lanes_d)` and binds `values`" by "opens `cur_d` as a terminal state, computes `outcome_d`, and binds `[prog_d, input_d, outcome_d, n_steps]`"; replace "refuses replay of `stmt`, writes `result` as big-endian `Bytes<16>`" by "refuses replay of `claim_d`, writes `outcome_d`". `D/design.md` §3.2(5): replace the two `val_d` bullets by the table above.

Tasks (`D/tasks.md`): amend 2.5, 5.1a to 5.1c (new `acceptRun` signature, template and key cascade); amend 4.2a (statement contract and `sherlock-value/1`); amend 4.9 and 4.15 (Init with `vd`); amend 5.2 (negatives: altered `outcome_d`; one run under three partitions; replay across partitions refused); new: value digest experiment in `crates/mori-chip` (rows, columns, node rows, negatives for a forged digest, Quint model of the invariant, and the halt traversal measured beside it); new: SI-12 under C = 8; new: address ceiling against GP-1; new: refinement model whose reference does not allocate per leaf.

## Q2 Does this run on Midnight ledger 9?

### Positions

**Fable: yes with changes.** Nothing in ledger 9.1.0.1 blocks the design. The on-chain step has not yet been accepted by a 9.1.0.1 node, the chip leaf has not been proved inside the node, and the contract and template change for Q1.

**Astra: yes with changes.** "The inspected verifier architecture supports this route on ledger 9. The evidence does not establish that the completed chip design and revised outcome statement already run on the deployed network." And: "No inspected source requires a ledger fork merely because the leaf is hand-written or the decider uses `k=20`."

**Carbon: yes with changes.** Ledger 9.1.0.1 and the pinned proving stack "require no upstream ledger, runtime, or proof-system changes to verify the chip-leaf recursion's terminal run-decider proof on chain. However, bounded changes in Moriarty's contract, decider circuit, transcript template, and client transaction builder are required". "No blocker exists in Midnight Ledger 9.1.0.1 itself." 

### Evidence table

| Check | Finding | Source | Found by |
|---|---|---|---|
| Versions | Tag `node-2.1.101` pins `midnight-ledger-v9 =1.0.1`, spec 2001100. The deployed build `913ec011` is not a public commit. | `qa/sherlock/target-pins.json`; node tag `Cargo.toml:104`, `runtime/src/lib.rs:283` | Astra, Fable |
| `midnight-proofs` version | The ledger tag and the node tag resolve `midnight-proofs` 0.8.2; the workspace proves with 0.8.3. The brief's "0.8.3" is the workspace pin, not the chain's. A 0.8.3 proof against a 0.8.2 verifier is not yet qualified. | `L9:Cargo.lock:3509-3510`; node tag `Cargo.lock:8713-8714`; `Cargo.lock:2137-2138` | Astra; lead confirmed |
| Verifier for a call | `ProofVersioned::V3` uses `op.v3_vk()` and `vk.verify(&PARAMS_VERIFIER, proof, pis)` | `L9:ledger/src/structure.rs:446-506` | all |
| Proof system | `midnight_zk_stdlib::verify`: PLONK, KZG, BLS12-381 | `L9:transient-crypto/src/proofs.rs:563-577`; `REG/midnight-zk-stdlib-2.4.0/src/lib.rs:2009-2032` | all |
| Transcript on chain | Blake2b (`TranscriptHash = blake2b_simd::State`). The decider is proved with Blake2b. | `L9:transient-crypto/src/proofs.rs:65`; `crates/mori-recursion/tests/sherlock_gp.rs:37,323` | all |
| Inner transcript | Leaf and node proofs use Poseidon and are verified in circuit only | `crates/mori-recursion/src/inner.rs:102-104`; `ivc.rs:322-326` | all |
| SRS on chain | Static verifier parameters `bls_midnight_2p14`. The ledger's data catalogue lists the ceremony files to `2p25`, with the pinned `2p20` hash. | `L9:transient-crypto/src/proofs.rs:108-128`; `L9:base-crypto/src/data_provider.rs:182,207` | Fable; Astra |
| Maximum k | No check. `ParamsVerifierKZG` holds only G2 elements, so the `2p14` file does not bound k. `VERIFIER_MAX_DEGREE` occurs only at its definition and in a doc comment. | `REG/midnight-proofs-0.8.3/src/poly/kzg/params.rs:235-239`; `git grep VERIFIER_MAX_DEGREE ledger-9.1.0.1` | Fable, Astra |
| k = 20 decider against the ledger verifier library | The test verifies the decider proof with the ledger crate's `VerifierKey::verify(&PARAMS_VERIFIER, ..)` and refuses each mutated public input; the recorded run `SHK23-gp-chain2` has exit 0 (`qa/sherlock/runs.jsonl`, 2026-11-10T02:32Z; decider over the stand-in C6 leaf chain, not the chip leaf). This is the ledger's verifier library linked in the workspace against `midnight-proofs` 0.8.3; it is not a node, and the chain resolves 0.8.2. | `crates/mori-recursion/tests/sherlock_gp.rs:329-345`; `README.md:409`; `qa/sherlock/manifests/run-decider.json` | Fable |
| Upstream comment | A comment says no verifiable circuit exceeds 2^14 rows. It is not enforced; a later release could enforce it. | `L9:onchain-runtime/src/transcript.rs:80-87` | Fable, Astra |
| Form of the on-chain key | Not ZKIR, but a zk-stdlib key: `MidnightVK::read` rebuilds the constraint system with `ZkStdLib::configure(arch, k - 1)`. The decider is a zk-stdlib `Relation`. The ledger needs no IR to verify. | `REG/midnight-zk-stdlib-2.4.0/src/lib.rs:1573-1594`; `crates/mori-recursion/src/run_decider.rs:181`; `L9:ledger/src/semantics.rs:1594-1606` | Fable, Astra |
| Chip leaf outside ZKIR | Verified only inside the node, which takes the leaf key as constants; the decider consumes only the node key. Nothing on chain sees the leaf or the node. | `run_decider.rs:209-218`; `D/design.md` §2.7.10 | all |
| Curve, commitment, SRS of leaf, node, decider | KZG on BLS12-381. The decider manifest names the official ceremony SRS `bls_midnight_2p20`. The final chip measurement used an unsafe SRS and made no proof. | `crates/mori-recursion/src/inner.rs:104`; `qa/sherlock/manifests/run-decider.json`; `experiments-4.md:26-28` | Astra; Fable |
| Decider size | 929,166 rows, k = 20 (cost model over the chip-leaf node key, unsafe SRS). The same log line gives 1,032,467 table rows of the 1,048,576 in the domain, The cost model reports table rows apart from the other rows (`REG/midnight-proofs-0.8.3/src/dev/cost_model.rs:131-136`), and both must fit the domain. The table side is the one nearer to 2^20. Whether these tables grow with the circuit was not checked, so the 1,000,000-row gate on `rows` does not alone show the margin to k = 21. | `D/design.md` §2.7.11; `~/.cache/chip4/logs/node1.log:912` | Astra; lead confirmed |
| Raw verifier key <= 50,000 bytes | 2,189 bytes raw at k = 20 (stagenet record), 2,217 tagged. Not measured for the revised key. | `L9:transient-crypto/src/proofs.rs:417-431`; `docs/sherlock/stagenet-deployment.md:102-103`; `qa/sherlock/manifests/sherlock-run-contract.json` | Fable, Astra |
| Public inputs <= 65,536 | 70 slots today; ESTIMATE 60 to 70 after Q1; to be recounted on the new template | `L9:onchain-runtime/src/transcript.rs:87`; `D/design.md` §3.3 | all |
| Operations per contract <= 256 | 1 | `L9:ledger/src/structure.rs:1928` | Fable |
| Proof size | 6,432 bytes for the k = 20 terminal decider on stagenet; ESTIMATE 6,320 bytes from the cost model for the chip-key decider. Fable's first figure (3,440) was a child proof and is withdrawn. | `docs/sherlock/stagenet-deployment.md:70,142`; `~/.cache/chip4/logs/node1.log:912` | Astra; Fable corrected |
| Transaction and block size | Source default 1 MiB per transaction; node tag block length 1 MiB with 75% normal ratio | `L9:ledger/src/structure.rs:1283-1296`; node tag `runtime/src/lib.rs:306-320` | Astra; lead confirmed |
| Cost model | `proof_verify_constant + proof_verify_coeff_size * public_inputs`; no term in k. ESTIMATE 3.59 ms for the verification term at 70 inputs with the bundled mainnet coefficients (Astra). | `L9:ledger/src/structure.rs:1138-1142`; `L9:ledger/params/mainnet.json:127` | Fable; Astra |
| Key installation | Deploy installs the hand-written decider key for `acceptRun` and checks its hash; `VerifierKeyInsert` refuses a second key of the same version | `tools/sherlock-sdk/deploy-run.mjs:4,42,56`; `L9:ledger/src/semantics.rs:1594-1606` | Fable, Astra |
| Q1 statement in the state model | `outcome_d: Field`, `steps: Uint<64>`, `runs: Set<Field>`: ordinary ledger fields | `tools/sherlock-sdk/sherlock-run.compact:23-27` | all |

Rows that only Carbon gave:

| Check | Finding | Source |
|---|---|---|
| Mainnet parameter file | `transaction_byte_limit` 1,048,576; block limits `computeTime` 2e12, `readTime` 2e12, `bytesWritten` 50,000, `bytesChurned` 5e7; `proof_verify` 3,273,586,253 + 4,555,132 per public input; `verifier_key_load` 1.53e9. ESTIMATE: an `acceptRun` call is under 0.5% of the block compute limit. This is the bundled file, not the observed chain parameters. | `L9:ledger/params/mainnet.json:1-175` (not re-read by the lead) |
| Transaction construction on ledger 9 | At least two DUST spends per transaction until midnight-ledger #838 is fixed; no `VerifierKeyRemove` until midnight-node #2375 is fixed; `setNetworkId` still in two SDK scripts; fees from live parameters | `D/design.md:313-315`; `D/tasks.md:29`; `wiki/midnight-ledger-9-hardfork.md:145-162` |
| Where the value digest is computed | The decider must not hash values: it is at 929,166 rows, so the digest is computed in the leaf and the decider opens one fixed-size preimage | `D/design.md:122` |

Dissent in Q2 (kept):

- **Ceiling of the SRS.** Carbon: "practical ceiling is `k = 20` from the ceremony prover SRS `bls_midnight_2p20`", and it lists "Decider SRS ceiling at `k = 20`" as a blocker class if the decider grew. Astra: the ledger's data catalogue lists ceremony files through `bls_midnight_2p25`. The lead confirmed Astra's citation (`L9:base-crypto/src/data_provider.rs:182-209`). So k = 21 is not excluded by the SRS; it is excluded by the project's own gates and prover cost. Carbon's design conclusion (compute the digest in the leaf) stands on the row gate alone.
- **Source read for the proof system.** Carbon read `repos/midnightntwrk/midnight-zk` at `HEAD`, not the pinned crates; Fable and Astra read the registry sources of 0.8.3 and 2.4.0. Carbon's findings on the verifier parameters agree with theirs.
- **Proof version.** Carbon states the stack as `midnight-proofs` 0.8.3 and did not see that the ledger and node tags lock 0.8.2 (Astra's finding).

### Blockers

No blocker in ledger 9. No member found a ledger rule that refuses a k = 20 zk-stdlib key or that requires the leaf or node to be ZKIR.

Work that the "yes" depends on (the changes):

1. New contract, template and keys for the Q1 statement, in the order leaf, node, decider, template, deploy (`D/design.md` §5.2 item 2). The template fixture predates the re-pin (`qa/sherlock/manifests/sherlock-run-contract.json`; task 2.5 open).
2. No `acceptRun` transaction has been validated by a 9.1.0.1 node; tasks 2.5, 5.1c and 6.x are open (`D/tasks.md:43,90,114-120`). The stagenet deployment was on a pre-release ledger 9 and had zero calls (`docs/sherlock/stagenet-deployment.md:3,17`).
3. The chip leaf is a spike with stand-in rules and no proof (`crates/mori-chip/tests/e3_spike.rs:11-12`; `experiments-4.md:26-28,319`). A node proof over a chip leaf proof on the ceremony SRS does not exist yet.
4. Qualify the 0.8.3 prover against the chain's 0.8.2 verifier: verify one decider proof with a build at the ledger tag's lockfile.
5. Read the target `LedgerParameters` and check the serialized transaction against the ledger and node limits before any submission.
6. One contract for all programs (GP-1) needs `prog_d` and `input_d` as call data, not sealed fields: a contract change, not a ledger limit.
7. Transaction construction (Carbon): at least two DUST spends per transaction until midnight-ledger #838 is fixed, no `VerifierKeyRemove` until midnight-node #2375 is fixed, the remaining `setNetworkId` calls removed, fees computed from live parameters.

### Verdict on Q2: yes with changes

The design's on-chain step is one zk-stdlib PLONK/KZG proof on BLS12-381 with a Blake2b transcript, a 2 kB key, a 6 kB proof and about 70 public inputs. The ledger 9.1.0.1 verifier library, built in the workspace with `midnight-proofs` 0.8.3, verifies such a proof at k = 20 (for the present decider over the stand-in leaf chain), and the ledger has no rule about what the proof verifies inside. The chain itself resolves `midnight-proofs` 0.8.2 (item 4). The outcome-only statement fits the contract's public inputs and state. "Yes" is not yet "accepted on chain": items 1 to 5 above are open.

## What a member could not check

- All: no build, test, keygen or proof; no live-node observation; the deployed node build is not public; the deployed `LedgerParameters`, fees and the final transaction size; the revised key, template slot count and proof size.
- Fable: the citations of `experiments-4.md` §4 other than those listed in `fable/answer.md`; whether a memory record has room for a digest word; whether Compact `transientHash` equals zk-stdlib `poseidon` on these inputs; the GP architecture documents; no web source was used.
- Astra: `midnight-proofs` 0.8.2 source ("attempts to open its docs.rs source failed"; its KZG analysis uses 0.8.3); the A11 trace and `~/.cache/a11/out2/run.json` were absent, so the key size rests on the committed manifest; every proposal of Q1 is unverified for soundness, completeness and cost.
- Lead, on Astra's citations: re-read and confirmed `L9:Cargo.lock`, the node tag's `Cargo.lock` and `runtime/src/lib.rs`, `L9:base-crypto/src/data_provider.rs:182`, `L9:ledger/src/verify.rs:628`, `poseidon_cpu.rs:131-133`, `~/.cache/chip4/logs/node1.log:912` and `owner_runtime.rs:3887`. Astra's other citations were not re-read. Note on the last: `structurally_equal` there is the diagnostic `check_step_inv` helper, not the machine's Equal rule.
- Carbon: the cargo registry sources and `midnight-trusted-setup` were not read (it used `midnight-zk` at `HEAD`); SRS availability above 2^20 was not checked; `compactc` 0.36.0 is not installed, so the recaptured slot count was not observed; proof size, transaction size and cost totals are estimates; not every file of the C6 rule families was read; Lean proofs were not read.
- Lead, on Carbon's citations: `owner_adt_runtime.rs:1585,1601`, `structural.rs:637`, `L9:ledger/params/mainnet.json` and `wiki/midnight-ledger-9-hardfork.md:145-162` were not re-read.
