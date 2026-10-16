## Verdict: AGREE — scope-limited

I agree, within the stated bounds only, that candidate01’s source corresponds to the 33 Enabled structural row effects and is coherent against the supplied provisional shared v3a Rust interface. This is not a certification of security, guard or selection semantics, charge/refusal, plan validation/apply, StepInv, execution, product behaviour, native behaviour, or acceptance.

No structural blocker was found in the bounded source/interface review. Agreement is conditional on the dependencies listed below. Compilation and tests remain unverified.

## Actual source coverage

Reviewed inline, in full:

- Candidate membership for `structural-row-effects01`: three candidate files only — report, row-effect mapping, and the proposed Rust source. Prior partial drafts were not treated as candidate.
- The candidate membership states: `installed: false`, `compiled: false`, `tests_run: false`, `adopted: false`, `native_authority: 0`, `acceptance: none`.
- The row mapping covers Bind 10, Validate 7, Equal 6, and Match 10 Enabled rows, plus the four out-of-domain Disabled/ExtensionOnly dispositions with `candidate_function: null`.
- Normative authority was checked against the supplied qualified HOST R4 material as adopted only under the exact Root qualified HOST Stage1 adoption record. The v3a shared ABI remains a proposal. Catalog event adoption remains pending; I did not treat it as completed.
- The known coverage report was treated as an author claim, not independent evidence. No independent oracle outputs were provided or used.

## Cross-cutting Rust/interface checks

- The four public free functions exist: `plan_bind`, `plan_validate`, `plan_equal`, `plan_match`. They are intended for Root forwarding after `self` from the corresponding `FamilyPlans` methods. No implementation of the other seven `FamilyPlans` methods is claimed or present.
- All `PreImageView` methods used by the candidate exist in the supplied provisional shared v3a with matching shapes: `control`, `reference_cursor`, `helper`, `top_task`, `top_frame`, `active`, `task_live`, `expr`, `function_row`, `parameter`, `value_slot`, `field_at`, `kind_of`, `env_cell`, `arm`, and `pattern`.
- `GuardEvaluator::eval_guard(&GuardId, &dyn PreImageView)` exists. The candidate calls exactly the six conditional-delta guards: `GBindTypeReject`, `GBindDupHit`, `GValidateOccLeaf`, `GValidateFieldNontail`, `GEqualPairLeaf`, and `GEqualFieldNontail`. Guard meaning remains owner work.
- `RuleIdC` is used consistently. The module aliases `RuleIdC as RuleId` for the private Validate/Equal/Match factors and otherwise matches on `RuleIdC` variants. No raw frozen `RuleId` is constructed or copied into violations.
- Foreign-rule handling is complete at every public entry and at the private family dispatch factors `plan_validate_nonentry`, `plan_equal_nonentry`, `plan_match_postarm`, `bind_unwind_payload`, and `plan_bind_terminal`. The rendering is exactly `HostViolation { kind: WrongVariant, clause: None, rule: None, locator: Some("foreign rule for family") }`, as bound by the supplied owner resolution. Row-specific payload factors such as `bind_entry_payload`, `bind_pair_payload`, and `match_arm_payload` are only reached after the public dispatcher has matched the exact row; they are not independent entry points.
- Entry common metadata is preserved as specified: `HelperCommon { site: sel.site, origin_reference_cursor: pre.reference_cursor(), parent_resume: None }`. Entries consume `sel.site` and the pre-image reference cursor; they do not derive site, helper choice, or event metadata.
- Full helper copies are taken with `.copied()` before mutation, so `common` and all untouched/dormant fields are preserved. Rows that do not change the helper leave `PlannedWrites.helper` at `Write::Keep`; terminal rows set `Write::Set(None)`.
- No candidate plan mutates work, visits, reference cursor, depth/cap counters, or `Outcome`. `PlanDelta` is used only for the projected allocation/task effects; all other deltas remain owner-derived.
- Checked arithmetic is used for cursor increments and slice-position additions, returning `ArithmeticUndefined` on overflow. Task field rows set `task_rewrite_top` first and `task_op = Push` second in the planned write set; tail rows set only the rewrite. Owner apply ordering remains pending owner work.
- Private payload helpers have real bodies; there are no stubs in the candidate source.

## Family audit

### Bind — 10/10 rows via `plan_bind`

- `entry` (`helper_events/22`): re-checks `ControlC::Enter` with a present function, reads the function row’s `params` in the wrapper, then installs `Bind` with `function`, `params`, `args`, cursors 0, `partial: None`, `dup_cursor: None`, `Pairing`, `Ok`. Delta zero.
- `pair` (`/23`): the wrapper performs the checked parameter read (`params.start + p_cursor`) before `bind_pair_payload` performs the value-slot read (`args.start + a_cursor`), preserving parameter-before-argument order. Pushes `PairTask { binder, sort, arg }`, increments both cursors with checked arithmetic, `task_live: +1`.
- `empty` (`/24`), `arity` (`/25`): phase branches use the wrapper-supplied pre-image `task_live` (`> 0`); `arity` also sets `status: Bad`.
- `type` (`/26`), `duplicate` (`/27`): the owner guard decides the reject/hit branch. Reject/hit pops with `task_live: -1`, sets `Bad`, and uses the `task_live > 1` phase test. Non-reject `type` copies `partial` into `dup_cursor` and chooses `UnwindInsert`/`UnwindDup` from `partial.is_none()`. Non-hit `duplicate` advances `dup_cursor` to `cell.previous` and chooses the phase from `cell.previous.is_none()`. Inserted cells already allocated stay counted; no delta retracts them.
- `insert` (`/28`): reads the top `PairTask`, stages `EnvCellC { binder, value: arg, previous: partial }` with `alloc_environment_cells: +1`, pops (`task_live: -1`), and sets `partial: Some(pre.active().env)` — the pre-image next env index, matching the owner-allocated cell identity.
- `propagate` (`/29`): verifies the top task is a `PairTask`, pops, phase from `task_live > 1`.
- `exit`/`failure` (`/30`–`/31`): `BindOk { env: partial }` or `BindBad`, helper cleared, delta zero.

### Validate — 7/7 rows via `plan_validate`

- `entry` (`helper_events/39`): requires `ControlC::Eval` and an `ExpressionRowC::Literal`, builds `root = RefC::Const(constant)`, installs `Validate` as `Pending`, and pushes `ValidateTask { r: root, visit: TaskVisitC::Unvisited, cursor: 0 }` with `task_live: +1`.
- `occurrence` (`/40`): under `GValidateOccLeaf`, pops (`task_live: -1`); otherwise rewrites the top to `Fields` with cursor 0. The pre-image task ref is preserved.
- `field` (`/41`): reads `field_at(r, cursor)` first. Under `GValidateFieldNontail`, rewrites the top with the checked `increment(cursor)` and pushes the child task (`task_live: +1`). On the tail branch, the top is replaced by the child task with no push.
- `empty` (`/43`): sets `status: Positive` on the full helper copy. `invalid` (`/44`): pops (`task_live: -1`) and sets `Negative`. `exit`/`failure` (`/45`–`/46`): stage `Validated { ok: true/false }` and clear the helper.

### Equal — 6/6 rows via `plan_equal`

- `entry` (`helper_events/47`): requires `ControlC::Return { value: right }` and a top `FrameRowC::RightF { left, .. }`, installs `Equal` as `Pending`, and pushes `EqualTask { left, right, Unvisited, 0 }` with `task_live: +1`.
- `pair` (`/48`): under `GEqualPairLeaf`, pops; otherwise rewrites the top to `Fields` with cursor 0, preserving both refs.
- `field` (`/49`): reads the left field before the right field. Under `GEqualFieldNontail`, rewrites the top with the checked incremented cursor and pushes the child pair (`task_live: +1`); otherwise replaces the top with the child pair.
- `empty` (`/50`): `Positive`. `different` (`/51`): pops (`task_live: -1`) and sets `Negative`.
- `exit` (`/52`): stages `AllocDataWrite::Bool { b: status == Positive }` with `alloc_data_nodes: +1` and stages `EqualDone { value: RefC::Data(pre.active().data) }` — the pre-image active Data identity, i.e. the index the owner allocation will occupy. This matches the report’s stated staging. Helper cleared.

### Match — 10/10 rows via `plan_match`

- `entry` (`helper_events/54`): requires `ControlC::Cases`, installs `arm: arms.start`, `saved_env`, `partial: saved_env`, `pat_tag: PVar` (the Arm-phase placeholder), zeros, `staged: None`, `phase: Arm`, `Pending`.
- `arm` (`/55`): the wrapper reads the arm and pattern rows and projects `(PatternTag, x, y)`, with `x = 0` for `PNil` and `y` set only for `PCons`/`PBranch`. `match_arm_payload` stages the scrutinee and goes to `Bind` for `PVar`; otherwise goes to `Tag`.
- `occurrence` (`/56`): maps `PNil/PCons/PLeaf/PBranch` to `Nil/Cons/Leaf/Branch` (reserved or `PVar` tags return the candidate’s `wrong_variant()`), compares `pre.kind_of(scrutinee)`, and branches to `NoMatchReady`, `Ready`, `Field` with `field_idx = 0`, or `Distinct`. Tests occur before any binding.
- `field` (`/57`): `staged = Some(field_at(scrutinee, usize::from(field_idx)))`, phase `Bind`.
- `binding` (`/58`): binder is `x` for `PVar` or `field_idx == 0`, else `y`; stages `EnvCellC { binder, value: staged, previous: partial }` with `alloc_environment_cells: +1`, sets `partial: Some(pre.active().env)`, clears `staged`, then goes to `Ready` for `PVar`/`PLeaf` or `field_idx == 1`, else advances to the second field.
- `distinct` (`/59`): `x == y` gives `NoMatchReady`; otherwise `field_idx = 0`, phase `Field`. This test precedes binding, and staged refs are copied, not moved.
- `next`/`selected` (`/61`–`/62`): set `Negative`/`Positive` with phase `Done`. `exit`/`failure` (`/64`–`/65`): stage `MatchMatched { env: partial }` or `MatchNoMatch` and clear the helper. Both return immediately, so no trailing `Write::Set(Some(helper))` overwrites the clear.

## Blockers

None within the bounded scope above.

## Dependencies and open items (not certified here)

1. Exact Root qualified HOST Stage1 adoption of the R4 normative material; the candidate, rows/schema/handoff, and v3a ABI are adopted only under that record. This lane does not extend it.
2. Root adoption of provisional shared v3a (sha256 `b91a60783b9b626ddfa17013ed7aefab587557f414e789662a56d2517aa41631`) — still a proposal.
3. Owner implementations of `PreImageView` and `GuardEvaluator`, plus the `OwnerEngine` bodies for selection, plan validation, physical checks, cap refusal, and apply. Those own all decisions this review excluded.
4. Root wiring: module placement at `super::super::shared`, and forwarding from the four `FamilyPlans` methods (`plan_bind`, `plan_validate`, `plan_equal`, `plan_match`) to these free functions after `self`. The other seven `FamilyPlans` methods are not supplied.
5. Separate explicit Root decision on catalog I-B event adoption — still pending; not treated as completed here.
6. Compilation, Rust tests, and all execution/native evidence — unrun. Event-side owner decisions remain pending elsewhere.

MUSE_STRUCTURAL_ROW_EFFECTS_CANDIDATE01_FULL_TAIL_20261016
