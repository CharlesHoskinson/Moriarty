# Fresh non-author delta reviewer B: Core `c6_phys.rs` 97378 B (`98941c4a…`)

## Dispositions

| | |
|---|---|
| **design** | **AGREE** |
| **source** | **AGREE** |
| **overall** | **AGREE** (bounded delta; nothing below blocks) |

**Basis.** This vote rests on source text only. I ran no compiler, tests, SDK, native, K, Lean, provider or Git command, and made no edits. A passive rustfmt parse is not a typecheck, and none of the 19 proposed tests counts as run. I treated the two prior full reviews as historical context only, and consulted no other delta review.

## 1. R1A: env-changing call fixture

**Fixture source (H2, H12).**
- `ExtFunction { name, parameters, return_type, body }` and `ExtParamSrc { name, ty }` match the actual `owner_ext_t3_uses` fields exactly.
- Both types are `pub` in `pub mod owner_ext_t3_uses`.
- The `var` and `pair` closures and the moves into `vec![mk]` typecheck despite the `Drop` impls on `ExtExpr` and `TypeExprSrc`.
- `lit`, `box_uint` and `field` are all still used, so no new dead-code warning.

**Admission bounds, by source.** The FIFO in `owner_ext_type_builder` gives:

| Table | Content | Used | Limit |
|---|---|---|---|
| types | builtins + Pair = 4 (T1, decl 1) + `Box<UInt>` = 5 | 6 | 16 |
| constructors | | 6 | 16 |
| ctor fields | | 8 | 32 |
| expressions | 0 Let, 1 Call, 2 Lit, 3 Var, 4 MkPair, 5 Var, 6 MkBox, 7 Var | 8 | 128 |

The children cursor is `[2 | 5,6 | 7]`, which is consistent with V2. The function `return_type` `TypeIdC(4)` has `decl == Some(1)`, i.e. `"Pair"`. Bounds, layout, FUEL 4096 and frames 64 are unchanged.

**Push trace, from `legacy.proposed.rs` and `adt_constructor.rs`.**
1. `LetF` (None → None).
2. `ArgsF`, from `KMachineEvalCallArguments`: `completed` empty, `rest = tail(args) = (0,0)`, `saved_env == env == None`, origin 1, top frame `LetF`.
3. `ReturnF`, from `KMachineBoundValid`: `saved_env = pre.env_root() = None` (restored by `ReturnArgumentsFinal`), `env = BindOk.env = Some(cell)`, origin `control_origin = 1`. This equals the `ArgsF` origin, because Invoke, Enter and Bound never write the origin. Top frame is `LetF`.
4. `CtorF(MkPair)` with `saved_env Some(cell)`.
5. `CtorF(MkBox)` pushed over `CtorF(MkPair)`. This sets `nested_ctor_push`.

That is exactly 5 pushes, so `>= 5` holds. Every `H15` assertion matches these values.

**Mutation witness.** The env check is genuine, not tautological:
- `env_changing_push` fires only when `before.env_root != ScheduleNext.env`.
- The exact after value comes from the existing `after.env_root == env` assertion, plus the strict recorder's `first_host_difference`.
- Removing `root.env_root = env` from `apply_push_structural` would yield `AfterMismatch{"after.env_root"}` at push 3. `must` would then panic.
- The lane-level `assert_ne!` on `[L_ENV_ROOT..+2]` (`[0,0]` vs `[1,cell]`) is well typed (`[u128] == [u128]`).
- The `ReturnF` arm independently forces `env != saved_env`.

**Retained coverage.**
- The nested `ConstructorFieldsF`-over-older-`ConstructorFieldsF` block is retained.
- The charged-counter lane mask loop is retained.
- The `wrong_rule` and `wrong_charge` mutations and the `CounterOverflow` check are retained.
- `caller_snapshot_refusals` is retained; it now runs on a `Some(cell)`-env snapshot.
- The program-codec captured-host fixture still exercises a parameterized `Box<A>` and an `Apply` field, plus non-empty `functions` and `parameters` tables.

**A140 test (H17, H18).** With `stack_limit 1`:
- The `LetF` push is accepted at depth 0.
- The next `ScheduleNext` is `ArgsF` at depth 1, giving `Refuse`.
- The A135 recorder refuses with `UnexpectedPushSelection` and leaves the image untouched.
- The A140 recorder records `Error 4`, with the event's `source_expr_id == control_origin == 1`.
- `p1_publish` is refused untouched.
- `KAdminPublishError` follows with `Published`, work 0 and `Rejected 4`.

This remains meaningful.

**Scope caveat.** Everything beyond the Push and legacy rows rests on unsupplied helper family bodies (Validate, Append, Find, Bind, EnvGet, Scalar TestType, Build). One consequence is that `BindOk.env` is assumed to be `Some`. If any of these assumptions is wrong, the test fails loudly; it cannot pass falsely. This is execution-evidence territory.

## 2. R1B: builtin TypeId cap

**The cap sites.**
- `type_index_cap(c) = c.max_types.max(4)` is used exactly five times:
  - `frame.return_type`
  - `frame.type_id`
  - `frame.field_type`
  - `control.return_type`
  - `control.type_id`
- `ctor_id` is still checked against `max_constructors`.
- `from_parts` keeps `max_types: bounds.type_rows`.
- `c6_phys_program` still caps the `types` and `type_args` counts with `max_types`.
- The 27 `C6PhysCaps` fields and the encoder bytes are unchanged.
- The decoder's accepted domain widens only for caps with `max_types < 4`.

**The rationale matches owner source.**
- SI-15 checks `ReturnF` and `Bound` with `types.len().max(4)`.
- Dedicated `check_program_view` uses `n_types = 4`.

**The new test, checked by hand at `from_parts(type_rows 0)`.**
- The other limits are `code_rows 1`, `constructor_rows 1`, `max_arity 1`, value slots 0 and children 0.
- All non-TypeId fields of `ret`, `ctor`, `bound` and `mat` pass these limits: origin 0 < 1, ctor 0 < 1, index 0 < 1, and empty slices against tables of 0.
- `max_types` and `max_decl_templates` are both 0.
- In all five fields, TypeId 3 passes and TypeId 4 fails with `needed 5, limit 4`.
- At `type_rows 6`, return type 5 passes; 6 fails with `7,6` for `ReturnF` and for `Materialize`.
- `BoundsC` and `LayoutCapacitiesC` derive `Default`. `ExtCapacitiesC::all_zero` is `pub const`.
- The `exceeded` closure's `Ok` type infers to `()` through Result's single `PartialEq` impl.

The module and `from_parts` docs state an index exception only. The module doc keeps "membership … Chunk 4 … not claimed".

## 3. R2B: `PushOwnerCharge` provenance docs

H10 and H11 are documentation only. `from_event` stays `pub`.

The new text is accurate:
- `StepEventC` and `HostSnapshot` both have all-public fields.
- `from_event` checks the event against `before` and `selection` only.
- `C6PushObserved` has private fields and is built only inside `record_push_step`.
- The backend accepts only `C6PushObserved`, which is unchanged.
- The text claims no authenticity.

N1 (`source_occurrence`) was correctly not added.

Comparing the 18 hunks against everything else, I found no change to selectors, masks, counters, the 13/17/48 geometry, tags, raw stack order, origin handling or publication.

## 4. Non-blocking notes (optional, no owner action required)

- **NB1.** No refusal is witnessed with `ScheduleNext.env != before.env_root`. The admitted A140 test refuses at `ArgsF` (None → None). In `caller_snapshot_refusals` the env is `Some(cell)`, but it is the same before and after. So a "Refuse also writes env" mutant would still pass. This existed before the delta and was not requested. A future QA fixture could use a zero-argument call (`EvalCallEmpty`, no `ArgsF` push) with `stack_limit 1`.
- **NB2.** In the H10 sentence "Owner provenance exists only in `C6PushObserved`", the scope is Push charges. `C6BoundImageSection::OwnerCapture` separately carries capture provenance. Rewording is optional.
- **NB3.** The `max(4)` relaxation is owner-relevant only for `ReturnF` and `Bound`. Owner `ConstructorFieldsF` and `Materialize` require `type_id >= 4` anyway. The relaxation is harmless at codec level.
- **NB4.** The separate 3A source (`a0e8515c…`) is pinned to the original Core. The explicit compatibility follow-up recorded by Root is still required before the two are combined. I did not audit 3A.

## 5. Attestation

**Read completely:**
- Current bodies:
  - `c6_phys.rs` (97378)
  - `c6_phys_program.rs` (54963)
  - `c6_a135_push.rs` (2573)
  - both `mod.rs` files
- Delta material:
  - `FULL-AUTHOR-RESPONSE`
  - `EXACT-HUNKS-AND-INVERSE`
  - `CLASSIFICATION`
  - `PASSIVE-SYNTAX-READBACK`
- Prior reviews 02A and 02B, and results 03A and 03B.
- Frozen owner and shared source:
  - `04-shared`
  - `05-complete_observation`
  - `06-owner_host`
  - `07-adt`
  - `08-adt_storage`
  - `09-owner_ext_source_program`
  - `10-owner_ext_t3_uses`
  - `11-owner_ext_type_builder`
  - `12-owner_ext_admit`
  - `13-owner_prepare`
  - `14-owner_runtime`
  - `15-owner_adt_runtime`
  - `16-legacy.proposed`
- Crate configuration and families:
  - `lib.rs`, `sherlock/mod.rs`
  - the core, workspace and backend `Cargo.toml` files
  - `adt_constructor.rs`, `adt_build.rs`, `adt_plan.rs`
  - `G1-G2-PASSIVE-FACTS`

**Hunk verification.** All 18 new spans are present in the current file at the stated locations, and every new single-line span is unique. The 90335 B baseline was not supplied, so the old-span uniqueness and byte-exact inverse are accepted as passive facts, not independently verified.

**Missing definitions (not supplied; none is needed for the vote):**
- `families/mod.rs` and the Validate, Append, Find, Bind, EnvGet, EnvPut and Scalar family bodies
- `adt_runtime` (read-carrier types, `frame()`/`helper()`, `load_plan_node`)
- `owner_ext_c_expression`, `ext_program_c`, `ext_program_c6`, `ext_program_p`
- `owner_ext_prepare_types`, `ext_pass_b_walk`, `ext_source_traversal`, `nominal_declaration_order`
- `host_implied_prefix`, `raw.rs`, `owner_si21_*`, `witness_plan`

## 6. Next step

There is no blocking change and no next owner hunk. Root's ordinary Core and backend compilation, plus the 19 exact named tests on these exact five afterimages, remain the execution evidence. Chunks 3, 4 and 5 stay OPEN.

This vote establishes none of the following: compilation, test results, global encoder conformance, native, SDK or proof results, K/Quint/Clean correspondence, full-host or program binding, or a full-state physical POST.