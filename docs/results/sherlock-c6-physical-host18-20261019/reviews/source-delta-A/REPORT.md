# Fresh non-author Opus HIGH delta reviewer A: Core `c6_phys.rs` 97378 B (`98941c4a…`)

## Dispositions

| Field | Value |
|---|---|
| design_disposition | **AGREE** |
| source_disposition | **AGREE** |
| overall_disposition | **AGREE** (bounded delta scope only; see §6) |

There are no blocking findings. The three requested fixes (R1A, R1B, R2B) are present, correct by inspection, and confined to docs, caps, fixtures and tests. I found no regression in masks, counters, geometry, tags, raw stack order, origins or selectors. Four non-blocking notes follow in §3.

## 1. Identity, independence, execution

- **Reviewer:** Opus 5.5 (`claude-opus-5-5`), fresh non-author delta reviewer A, HIGH effort.
- **Independence:** I consulted no other fresh delta review. I used both prior full reviews (02A, 02B) only as historical context, not as approvals.
- **Execution:** I ran no compiler, tests, SDK, native, K, Lean, provider or Git command, made no edits, and called no tools. This is a source-text audit only. The passive rustfmt parse result (exit 0) is not a typecheck, and I do not count source assertions as executed traces.

## 2. Delta audit

### Hunk correspondence (H1–H18)

I located each "new" span verbatim, exactly once, in the supplied 97378 B body:

| Hunk | Location |
|---|---|
| H1 | module doc |
| H2 | fixtures import |
| H3 | `from_parts` doc |
| H4 | `type_index_cap` after `cap_kids` |
| H5–H9 | the five `type_index_cap(c)` sites |
| H10 | `PushOwnerCharge` doc |
| H11 | `from_event` doc |
| H12 | `nominal_program` |
| H13 | new test before `synthetic_root` |
| H14 | admitted-test flags |
| H15 | env/ArgsF/ReturnF assertions |
| H16 | final assertions |
| H17 | A140 comment |
| H18 | A140 expect plus `ArgsF` matches |

Additional checks:
- No `c.max_types` remains in `frame_caps` or `control_caps`.
- `max_types` is still used only in `from_parts`, `test_caps` and the program `types`/`type_args` counts.
- Δ = 7043 B, which is consistent with my rough size estimate of the 18 hunks.
- **Not verified by me:** the 90335 B baseline body was not supplied, so the old-span uniqueness and the byte-exact reverse-application claim rest on the owner's `EXACT-HUNKS-AND-INVERSE.json`.

### R1A: `mk(7)` fixture (traced against the supplied owner, legacy and ADT sources)

**Admission fits the bounds:**
- Types: 4 builtin, then T1 Pair = 4, then FIFO Box<UInt> = 5. That is 6 ≤ 16, with `type_args` 1.
- Constructors: 6 ≤ 16. Ctor fields: 8 ≤ 32.
- Expressions: 8. Functions: 1. Parameters: 1.
- `admit_run`: fuel 4096 is within `max_reference_work` and `value_slots` 4096. Stack limits 64 and 1 are ≤ 64.
- V14 decode handles the function: type 4 decodes to `Named Pair`, param sort 0 to `UInt`.

**Push sequence (exactly 5):**
1. **LetF** (`KMachineEvalLet`): env None → None.
2. **ArgsF** (`KMachineEvalCallArguments`): `completed` empty, saved_env and env both equal the caller's None, origin is the Call expression, top frame is LetF.
3. **ReturnF** (`KMachineBoundValid`): `return_type = functions[0].return_type` = Pair, `saved_env = env_root` (None after `ReturnArgumentsFinal`), env = `BindOk.env` = `Some(cell)`.
   - Origin = `control_origin`. This is the ArgsF origin because `ReturnArgumentsFinal` sets it, and SI‑20 allows only a Call origin through Invoke/Enter/Bound. There is a single Call.
4. **MkPair CFF** (`XAdtEvalConstructFields`, per `adt_constructor.rs`): env stays `Some(cell)`.
5. **MkBox CFF**: pushed over the MkPair CFF. `XAdtReturnCtorFieldsNext` is a RewriteTop, not a push.

**The new flags are genuine witnesses, not tautologies:**
- `env_changing_push` is set only when `before.env_root != ScheduleNext.env`. It is backed by the per-push `after.env_root == env` check against the actual owner after-image, and by an explicit inequality of the lanes `[L_ENV_ROOT..+2]` (before `[0,0]`, after `[1,id]`).
- The ReturnF arm independently asserts `saved_env == before.env_root` and `env != saved_env`.
- **Mutant check:** removing `root.env_root = env` makes `project_push_structural` keep None. `first_host_difference` then returns `after.env_root`, producing `Mismatch`, which makes `must` panic. The mutant is detected.

**Retained coverage:** the nested CFF-over-older-CFF block, the charged full-owner after-state, lane re-encode, `OwnerCapture`, `caller_snapshot_refusals` (now at depth 3 with ReturnF and CFF frames, all within `test_caps`), event mutation, and the counter overflow check are all unchanged.

**A140 (stack_limit 1):**
- The Let push is accepted at depth 0. The ArgsF `ScheduleNext` at depth 1 selects Refuse.
- The A135 recorder refuses it untouched. The A140 recorder records Error 4, charged, with frames, env and origin unchanged.
- Both recorders refuse the publication step as `p1_publish`, untouched. Publication 82 is then a separate step: `Published`, work 0, `Rejected{4}`. The refusal remains meaningful.

**Program test:** the new view still has a parameterized template (Box) and an `Apply` field (`snd`). It fits `test_caps`. The `template_bytes = 0` refusal still fires first, at the `decl_templates` name.

**Readiness by inspection (my opinion, not a compile result):**
- `ExtFunction`/`ExtParamSrc` field names match `owner_ext_t3_uses`.
- `String == &str` comparisons in `assert_eq!` are valid.
- Unsized `[u128]` slice comparisons in `assert_ne!` follow the existing pattern in this file.
- `pending` is `Copy` across the `match` and the later `matches!`.
- All new bindings are used. The closure Ok-type inference (`exceeded`) mirrors the existing `refusal` pattern in `c6_phys_program`.
- There are no edition-2024 hazards in the new lines, and no deny-warnings or lint tables (per the G1/G2 facts).

### R1B: TypeId index cap

- Exactly five sites use `max(max_types, 4)`:
  - `ReturnF.return_type`
  - `CFF.type_id`
  - `CFF.field_type`
  - `Bound.return_type`
  - `Materialize.type_id`
- The `CtorId`, `index` and all other caps are unchanged. The 27 `C6PhysCaps` fields, the `from_parts` mapping (`max_types = type_rows`, `decl = type_rows − 4`), and the program codec are byte-unchanged.
- This matches owner SI‑15 (`types.len().max(4)` for ReturnF and Bound) and dedicated `check_program_view` (`n_types = 4`). It closes 02B‑R1 for dedicated call programs under `from_layout` caps.
- **New test, checked arithmetically.** With `from_parts(type_rows 0)`, `code_rows`/`constructor_rows`/`max_arity` 1, and everything else zero:
  - Every passing vector clears the other caps: origin 0 < 1, ctor 0 < 1, index 0 < 1, empty slices give end 0 ≤ 0, env None.
  - All five fields accept 3.
  - All five fields refuse 4 with `needed 5, limit 4`, at the right section. In `ctor(4,0)` the `type_id` error fires first.
  - `max_types` and `max_decl_templates` stay 0/0.
  - Cap 6 accepts 5 and refuses 6 (`7/6`) on ReturnF and Materialize.
- The docs state the index exception and explicitly disclaim membership (Chunk 4).

### R2B: doc only

- The `PushOwnerCharge` and `from_event` docs now say correctly that the function validates a caller-supplied event against a caller-supplied snapshot, and carries no owner provenance or authenticity.
- `from_event` stays `pub`.
- `C6PushObserved` keeps private fields and is built only in `record_push_step`. It remains the only checked backend input.
- N1 (`source_occurrence`) was not added, as Root directed.

## 3. Non-blocking notes

- **D1.** `adt_constructor.rs` is now supplied and shows exactly 5 pushes. H16 could be tightened from `pushes >= 5` to `== 5`. Optional.
- **D2.** For `CFF.type_id` and `Materialize.type_id` the exception never changes the result in practice:
  - The owner requires type id ≥ 4 there (SI‑15 CFF, `adt_checked_ctor_arity`).
  - Generic admission forces `type_rows ≥ 4`.
  
  The doc's "builtin 0..=3 exist" rationale is precise only for ReturnF, Bound and `field_type`. It is harmless because membership is disclaimed.
- **D3.** H3 keeps "never raised" next to the derived `max(…,4)` index cap. The fields really are never raised, so this is a wording tension only.
- **D4.** Runtime completion of the fixture depends on families that were not supplied: Scalar `RetCall` type-test with a nominal expected type, Bind, Find, EnvGet, Append and Validate. Any defect there fails closed (the test panics); it cannot pass falsely. Only ordinary execution can settle it.

## 4. Read and missing-definition attestation

**Read completely:**
- Current `c6_phys.rs` (97378 B), `c6_phys_program.rs` (54963 B), `c6_a135_push.rs` (2573 B), and both `mod.rs` files.
- FULL-AUTHOR-RESPONSE, EXACT-HUNKS-AND-INVERSE, CLASSIFICATION, PASSIVE-SYNTAX-READBACK.
- 02A/03A and 02B/03B.
- `04-shared`, `05-complete_observation`, `06-owner_host`, `07-adt`, `08-adt_storage`, `09`/`10`/`11`/`12`/`13`/`14`/`15`/`16`.
- `20`–`24` (lib, sherlock mod, manifests).
- `adt_constructor`, `adt_build`, `adt_plan`, and the G1/G2 facts.
- The SEPARATE‑3A note and the QA proposal, read as context only.

**Missing definitions (named, not inferred):**
- **Baseline:** the 90335 B baseline `c6_phys.rs` body (inverse not verified).
- **Preparation passes:** `owner_ext_c_expression` (C1–C6/P), `ext_pass_b_walk`, `ext_source_traversal`, `nominal_declaration_order`, `owner_ext_prepare_types`.
- **ADT runtime:** `adt_runtime` (`ConstructorFieldsReadC::frame`, `BuildReadC::helper`, `load_plan_node`).
- **Families:** `families/mod.rs` and the Scalar/EnvGet/EnvPut/Find/Bind/Append/Validate/Equal/Match bodies.
- **Other owner modules:** `host_implied_prefix`, `raw.rs`, `owner_si21_*`, `witness_plan`.
- **SDK:** `Operand`/`Fr`.

None of these changes the vote. The backend is byte-unchanged and keeps the prior full-review scope.

## 5. QA proposal (unrun)

The proposal is adequate. The 19 names are consistent (8 Core, 10 program, 1 backend). The `--lib c6_phys` filter matches both Core modules. No result is claimed or counted.

## 6. Scope

**Adoptable on AGREE:**
- The isolated C6P1 root plus frames (13/17/48 lanes, all 10 frames and 11 controls).
- The strict A135 and A140 recorders.
- The passive `ProgramSnapshot` codec.
- Host immediates.

All 39 selectors and the frozen A114/L4 identities are unchanged.

**Still OPEN:** Chunk 3, Chunk 4 and Chunk 5, plus the 3A compatibility follow-up (not part of this vote).

**Not established:** compilation, tests, encoder conformance, native/SDK/proof results, K/Quint/Clean correspondence, full-host or program binding, and a full-state physical POST.

**Next owner hunk:** none required. D1–D3 are optional.