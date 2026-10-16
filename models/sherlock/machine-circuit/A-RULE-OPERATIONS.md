Unadopted source integration candidate: A-rule manifest r5 and the separate 12-rule P1 r2 proposal now use the seven frozen L4 files adopted by Root as the current-A host codec source interface only. MD5–MD7 remain unchanged. The other 27 P1 rows are UNWRITTEN; P2–P6, F4, physical masks/appends and the A185/A195 control-write choice remain unresolved. Fresh full Opus and Muse reviews are required before any use.

This manifest records the current A source operations, not an implemented machine-step circuit. It covers all 42 K machine-rule entries, 39 Rust StepRecord labels, startup, both typed-domain exclusions, pure helper branches and all eight Snapshot fields. L4 has only the recorded host source adoption; every physical append sequence and write mask is explicitly unresolved. The MD1–MD7 corrections are retained; fresh full reviews of this combined candidate are required.

The current source pins in A-RULE-OPERATIONS.json bind reference.rs, reference_types.rs, main.k and all seven frozen L4 files from astra-machine-layout-proposal-r4/source. L4 manifest SHA256 is `91d8bd43ac4ce41f0d9a89127e9f39865a53b153216f6424fb10e7a3ad91af88`; the adoption record is `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-reference-20261015/opus-sprint-loop-20261015/L4-ROOT-HOST-SOURCE-ADOPTION.json` (SHA256 `8790d17b807c8491e92e6a5cc7cf6f7ed8813067564144e627902b42bbdf28aa`). Prior L2 inputs and r4 provenance are retained as history in the JSON and frozen packet. Current sources do not use mutable layout files. The checkout HEAD observed before work was `1fb794d1598e19fe34b9735f90996fc316b743fa`; L and the existing circuit candidates were untracked local source. This file changes none of them.

A step first returns no event for a published outcome or absent control. Pending with absent control is unreachable from Machine::new and outside the observation domain under frozen adopted L4 design.md:73–80 (O3); it is not an admitted circuit API state. Return with an empty stack and Error publish for zero charge before exhaustion. Any other zero-fuel control publishes Exhausted. A charged transition performs its local frame/map/helper operations, assigns next control, then subtracts one fuel unit. Error/final Return controls produced by a charged step leave outcome Pending until a subsequent free publication. Helpers add no A events or charges.

Each rule below lists source lines, exact patterns/premises, ordered operations, all writes and all guaranteed equal fields. `S` is the original top-frame suffix. `saved` is the complete map in the matched frame/control. Reads of `pre.frames[0]` always refer to the captured original frame. A written field may equal its old value by coincidence; `write_equal` marks guaranteed equality despite assignment. Rule185 and rule195 assign a structurally equal Return control while changing stack/environment/fuel. Both write_equal and guaranteed_equal require post.control == pre.control equality constraints. The physical ControlRow rewrite versus leave-untouched choice is UNSELECTED; neither choice removes the equality obligation.

StepRecord.rule_start_line and charge are event metadata. before/after are full observations. Depth and charged_steps are derived, not additional Snapshot cells. Program, stack_limit and initial_fuel remain unchanged in every actual step. Pending is retained by every charged rule. Startup initializes all fields and emits no Rust StepRecord. K78 (negative startup) and K105 (invalid literal) remain visible but have no typed Rust transition.

| K line / rule | Rust source lines | Kind | Logical writes | Guaranteed equal Snapshot fields |
|---|---|---|---|---|
| 74 Post-startup constructor | 14–27 | startup-constructor | program, control, environment, frames, stack_limit, remaining_fuel, initial_fuel, outcome |  |
| 78 K negative startup rejection | 14–27 | K-only-outside-typed-domain | control | program, environment, frames, stack_limit, remaining_fuel, initial_fuel, outcome |
| 80 Publish completed return | 46–50 | free-publication | control, outcome | program, environment, frames, stack_limit, remaining_fuel, initial_fuel |
| 82 Publish error | 51–55 | free-publication | control, outcome | program, environment, frames, stack_limit, remaining_fuel, initial_fuel |
| 84 Publish fuel exhaustion | 56–59 | free-publication | control, outcome | program, environment, frames, stack_limit, remaining_fuel, initial_fuel |
| 102 Evaluate typed literal | 140–141 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 105 K invalid literal rejection | 140–141 | K-only-outside-typed-domain | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 108 Variable lookup found | 142–143 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 111 Variable lookup missing | 142–144 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 114 Schedule let | 146–152 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 117 Schedule left operand | 153–159 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 120 Schedule unary operand | 160–167 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 123 Schedule condition | 168–174 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 126 Schedule match scrutinee | 175–181 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 129 Empty argument call | 182–184 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 131 Schedule first call argument | 185–195 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 135 Install pending frame | 81–85 | charged-step | control, environment, frames, remaining_fuel | program, stack_limit, initial_fuel, outcome |
| 140 Frame limit reached | 81–88 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 145 Bind let and retain lexical scope | 204–208 | charged-step | control, environment, frames, remaining_fuel | program, stack_limit, initial_fuel, outcome |
| 149 Schedule right operand | 210–213 | charged-step | control, environment, frames, remaining_fuel | program, stack_limit, initial_fuel, outcome |
| 153 Compute binary primitive result | 215–220 | charged-step | control, frames, remaining_fuel | program, environment, stack_limit, initial_fuel, outcome |
| 157 Compute unary primitive result | 222–224 | charged-step | control, frames, remaining_fuel | program, environment, stack_limit, initial_fuel, outcome |
| 161 Select true branch | 226–235 | charged-step | control, environment, frames, remaining_fuel | program, stack_limit, initial_fuel, outcome |
| 165 Select false branch | 226–235 | charged-step | control, environment, frames, remaining_fuel | program, stack_limit, initial_fuel, outcome |
| 169 Reject non-Boolean condition | 226–237 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 173 Start ordered case search | 238–241 | charged-step | control, environment, frames, remaining_fuel | program, stack_limit, initial_fuel, outcome |
| 177 Accumulate argument and schedule next | 243–253 | charged-step | control, environment, frames, remaining_fuel | program, stack_limit, initial_fuel, outcome |
| 181 Finish argument list | 243–253 | charged-step | control, environment, frames, remaining_fuel | program, stack_limit, initial_fuel, outcome |
| 185 Typed function return | 255–259 | charged-step | control, environment, frames, remaining_fuel | program, control, stack_limit, initial_fuel, outcome |
| 190 Reject wrong return type | 255–262 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 195 Restore lexical scope | 264–267 | charged-step | control, environment, frames, remaining_fuel | program, control, stack_limit, initial_fuel, outcome |
| 200 Lookup first named function | 91–100 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 202 Missing function | 101–101 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 204 Bind function parameters | 102–107 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 206 Reject binding failure | 109–109 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 208 Schedule function body | 110–112 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 212 No remaining match arms | 114–114 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 214 Try first remaining pattern | 115–121 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 217 Continue after pattern miss | 123–125 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 220 Schedule matching arm body | 126–129 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 223 Expose successful primitive value | 130–130 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |
| 225 Expose primitive error | 131–131 | charged-step | control, remaining_fuel | program, environment, frames, stack_limit, initial_fuel, outcome |

**A074 — Post-startup constructor**

Control: `K uiRun(functions,entry,fuel,limit); no pre-Snapshot in Rust API`. Frames: `K initial configuration has empty stack`. Premises: K fuel >= 0 and limit >= 0; Rust inputs are unsigned; Initial K env/stack/depth/outcome come from configuration, not arbitrary retained machine data.

1. Construct Eval(program.entry.clone()), retain complete program, create empty environment and frames, store stack_limit, set both fuel fields to supplied fuel, and Pending outcome.

Complete post fields: `program = input.program` (initialize); `control = Some(Eval(input.program.entry))` (initialize); `environment = empty map` (initialize); `frames = []` (initialize); `stack_limit = input.stack_limit` (initialize); `remaining_fuel = input.fuel` (initialize); `initial_fuel = input.fuel` (initialize); `outcome = Pending` (initialize).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): `ProgramHeader`: entryExprId — Constructor starts Eval(program.entry); no step event. Program preparation is separate.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [STARTUP_SEPARATE_FROM_STEP]: Constructor initialization has no StepRecord; constructor/codec work is separate from a step rule, and no physical initialization plan is supplied.
- P2 [STARTUP_SEPARATE_FROM_STEP]: Constructor initialization has no StepRecord; constructor/codec work is separate from a step rule, and no physical initialization plan is supplied.
- P3 [STARTUP_SEPARATE_FROM_STEP]: Constructor initialization has no StepRecord; constructor/codec work is separate from a step rule, and no physical initialization plan is supplied.
- P4 [STARTUP_SEPARATE_FROM_STEP]: Constructor initialization has no StepRecord; constructor/codec work is separate from a step rule, and no physical initialization plan is supplied.
- P5 [STARTUP_SEPARATE_FROM_STEP]: Constructor initialization has no StepRecord; constructor/codec work is separate from a step rule, and no physical initialization plan is supplied.
- P6 [STARTUP_SEPARATE_FROM_STEP]: Constructor initialization has no StepRecord; constructor/codec work is separate from a step rule, and no physical initialization plan is supplied.

**A078 — K negative startup rejection**

Control: `uiRun(_,_,F,L)`. Frames: `unchanged`. Premises: F < 0 or L < 0.

1. Apply K control rewrite and no fuel update; no corresponding Rust Machine step exists.

Complete post fields: `program = pre.program` (preserve); `control = uiError(1)` (K-only-write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel` (preserve); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [OUTSIDE_TYPED_STEP_DOMAIN]: This K-only rule has no current-A typed transition; retain the catalog for scope review, without inventing a physical rule.
- P2 [OUTSIDE_TYPED_STEP_DOMAIN]: This K-only rule has no current-A typed transition; retain the catalog for scope review, without inventing a physical rule.
- P3 [OUTSIDE_TYPED_STEP_DOMAIN]: This K-only rule has no current-A typed transition; retain the catalog for scope review, without inventing a physical rule.
- P4 [OUTSIDE_TYPED_STEP_DOMAIN]: This K-only rule has no current-A typed transition; retain the catalog for scope review, without inventing a physical rule.
- P5 [OUTSIDE_TYPED_STEP_DOMAIN]: This K-only rule has no current-A typed transition; retain the catalog for scope review, without inventing a physical rule.
- P6 [OUTSIDE_TYPED_STEP_DOMAIN]: This K-only rule has no current-A typed transition; retain the catalog for scope review, without inventing a physical rule.

**A080 — Publish completed return**

Control: `Return(value)`. Frames: `[]`. Premises: outcome == Pending; control is Some; frames.is_empty(); terminal takes precedence even if fuel == 0.

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Assign Snapshot.control = None
3. Assign Snapshot.outcome = Done(value)
4. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = None` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel` (preserve); `initial_fuel = pre.initial_fuel` (preserve); `outcome = Done(value)` (write).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. No charged decrement; outcome publication for free publication rules.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A082 — Publish error**

Control: `Error(code)`. Frames: `any`. Premises: outcome == Pending; control is Some; Every u8 code retained; terminal takes precedence even if fuel == 0.

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Assign Snapshot.control = None
3. Assign Snapshot.outcome = Rejected(code)
4. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = None` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel` (preserve); `initial_fuel = pre.initial_fuel` (preserve); `outcome = Rejected(code)` (write).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. No charged decrement; outcome publication for free publication rules.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A084 — Publish fuel exhaustion**

Control: `any nonterminal control`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel == 0; control is not Error and not Return with empty frames.

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Return with empty frames and Error publication did not match; remaining_fuel is zero.
3. Assign Snapshot.control = None
4. Assign Snapshot.outcome = Exhausted
5. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = None` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel` (preserve); `initial_fuel = pre.initial_fuel` (preserve); `outcome = Exhausted` (write).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. No charged decrement; outcome publication for free publication rules.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A102 — Evaluate typed literal**

Control: `Eval(Literal(value))`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); Typed Value has only u128 scalars; K uiValid holds by this representation.

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Read and clone the complete literal Value; construct next Return.
4. Assign Snapshot.control = Some(Return(clone(value)))
5. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
6. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Return(clone(value)))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): `Expressions`: row selected by pre.control.expr: tag,valueRef; childSlice is empty; armSlice is unused — Expr tag dispatch and variant payload. Only the charged Eval path evaluates this row.; `ExprChildren`: childSlice length zero; no element fetch — Ordered source child references; unused tables/empty slices do not imply an element fetch.; `Arms`: armSlice unused; no arm fetch — This Expr variant has no arms.; `Constants`: Expressions.valueRef in Constant space — Literal value reference; recursive literal payload follows ConstantFields/Constants if unfolded.; `ConstantFields`: fields of referenced Constants, recursively as required by the literal value — Logical structure of the cloned literal; reference reuse versus physical materialization remains unresolved.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A105 — K invalid literal rejection**

Control: `Eval(Literal(V))`. Frames: `unchanged`. Premises: remaining_fuel > 0 and not uiValid(V).

1. Apply K control rewrite and one semantic fuel decrement; no corresponding Rust Machine step exists.

Complete post fields: `program = pre.program` (preserve); `control = Some(Error(2))` (K-only-write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (K-only-write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [OUTSIDE_TYPED_STEP_DOMAIN]: This K-only rule has no current-A typed transition; retain the catalog for scope review, without inventing a physical rule.
- P2 [OUTSIDE_TYPED_STEP_DOMAIN]: This K-only rule has no current-A typed transition; retain the catalog for scope review, without inventing a physical rule.
- P3 [OUTSIDE_TYPED_STEP_DOMAIN]: This K-only rule has no current-A typed transition; retain the catalog for scope review, without inventing a physical rule.
- P4 [OUTSIDE_TYPED_STEP_DOMAIN]: This K-only rule has no current-A typed transition; retain the catalog for scope review, without inventing a physical rule.
- P5 [OUTSIDE_TYPED_STEP_DOMAIN]: This K-only rule has no current-A typed transition; retain the catalog for scope review, without inventing a physical rule.
- P6 [OUTSIDE_TYPED_STEP_DOMAIN]: This K-only rule has no current-A typed transition; retain the catalog for scope review, without inventing a physical rule.

**A108 — Variable lookup found**

Control: `Eval(Var(name))`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); environment.get(name) = Some(value).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Look up exact Name in current map; return complete found Value.
4. Assign Snapshot.control = Some(Return(clone(value)))
5. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
6. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Return(clone(value)))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): `Expressions`: row selected by pre.control.expr: tag,nameId; childSlice is empty; armSlice is unused — Expr tag dispatch and variant payload. Only the charged Eval path evaluates this row.; `ExprChildren`: childSlice length zero; no element fetch — Ordered source child references; unused tables/empty slices do not imply an element fetch.; `Arms`: armSlice unused; no arm fetch — This Expr variant has no arms.; `Names/NameBytes`: Expressions.nameId — Exact source name used for variable lookup, let binder or call name; logical reference may be carried without eagerly reading bytes.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: H-env-get newest-first chain walk on found branch.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A111 — Variable lookup missing**

Control: `Eval(Var(name))`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); environment.get(name) = None.

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Look up exact Name; construct Error(3) on absence.
4. Assign Snapshot.control = Some(Error(3))
5. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
6. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Error(3))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): `Expressions`: row selected by pre.control.expr: tag,nameId; childSlice is empty; armSlice is unused — Expr tag dispatch and variant payload. Only the charged Eval path evaluates this row.; `ExprChildren`: childSlice length zero; no element fetch — Ordered source child references; unused tables/empty slices do not imply an element fetch.; `Arms`: armSlice unused; no arm fetch — This Expr variant has no arms.; `Names/NameBytes`: Expressions.nameId — Exact source name used for variable lookup, let binder or call name; logical reference may be carried without eagerly reading bytes.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: H-env-get newest-first chain walk on missing branch.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A114 — Schedule let**

Control: `Eval(Let(name,value,body))`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Capture source expression payload and any saved environment; construct pending frame.
4. Construct Eval child and Push(next, cloned environment); helper push only constructs control, it does not install a frame.
5. Assign Snapshot.control = Some(Push(Let(name,body,clone(pre.environment)),Eval(value),clone(pre.environment)))
6. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
7. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Push(Let(name,body,clone(pre.environment)),Eval(value),clone(pre.environment)))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): `Expressions`: row selected by pre.control.expr: tag,nameId,childSlice; armSlice is unused — Expr tag dispatch and variant payload. Only the charged Eval path evaluates this row.; `ExprChildren`: childSlice[0] value and childSlice[1] body — Ordered source child references; unused tables/empty slices do not imply an element fetch.; `Arms`: armSlice unused; no arm fetch — This Expr variant has no arms.; `Names/NameBytes`: Expressions.nameId — Exact source name used for variable lookup, let binder or call name; logical reference may be carried without eagerly reading bytes.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Spine creation writes ControlRows 0–1. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A117 — Schedule left operand**

Control: `Eval(Binary(op,left,right))`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Capture source expression payload and any saved environment; construct pending frame.
4. Construct Eval child and Push(next, cloned environment); helper push only constructs control, it does not install a frame.
5. Assign Snapshot.control = Some(Push(Left(op,right,clone(pre.environment)),Eval(left),clone(pre.environment)))
6. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
7. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Push(Left(op,right,clone(pre.environment)),Eval(left),clone(pre.environment)))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): `Expressions`: row selected by pre.control.expr: tag,binaryOp,childSlice; armSlice is unused — Expr tag dispatch and variant payload. Only the charged Eval path evaluates this row.; `ExprChildren`: childSlice[0] left and childSlice[1] right — Ordered source child references; unused tables/empty slices do not imply an element fetch.; `Arms`: armSlice unused; no arm fetch — This Expr variant has no arms.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Spine creation writes ControlRows 0–1. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A120 — Schedule unary operand**

Control: `Eval(Unary(op,value))`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Capture source expression payload and any saved environment; construct pending frame.
4. Construct Eval child and Push(next, cloned environment); helper push only constructs control, it does not install a frame.
5. Assign Snapshot.control = Some(Push(Unary(op),Eval(value),clone(pre.environment)))
6. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
7. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Push(Unary(op),Eval(value),clone(pre.environment)))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): `Expressions`: row selected by pre.control.expr: tag,unaryOp,childSlice; armSlice is unused — Expr tag dispatch and variant payload. Only the charged Eval path evaluates this row.; `ExprChildren`: childSlice[0] operand — Ordered source child references; unused tables/empty slices do not imply an element fetch.; `Arms`: armSlice unused; no arm fetch — This Expr variant has no arms.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Spine creation writes ControlRows 0–1. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A123 — Schedule condition**

Control: `Eval(If(condition,yes,no))`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Capture source expression payload and any saved environment; construct pending frame.
4. Construct Eval child and Push(next, cloned environment); helper push only constructs control, it does not install a frame.
5. Assign Snapshot.control = Some(Push(If(yes,no,clone(pre.environment)),Eval(condition),clone(pre.environment)))
6. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
7. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Push(If(yes,no,clone(pre.environment)),Eval(condition),clone(pre.environment)))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): `Expressions`: row selected by pre.control.expr: tag,childSlice; armSlice is unused — Expr tag dispatch and variant payload. Only the charged Eval path evaluates this row.; `ExprChildren`: childSlice[0] condition, [1] yes, [2] no — Ordered source child references; unused tables/empty slices do not imply an element fetch.; `Arms`: armSlice unused; no arm fetch — This Expr variant has no arms.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Spine creation writes ControlRows 0–1. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A126 — Schedule match scrutinee**

Control: `Eval(Match(value,arms))`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Capture source expression payload and any saved environment; construct pending frame.
4. Construct Eval child and Push(next, cloned environment); helper push only constructs control, it does not install a frame.
5. Assign Snapshot.control = Some(Push(Match(arms,clone(pre.environment)),Eval(value),clone(pre.environment)))
6. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
7. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Push(Match(arms,clone(pre.environment)),Eval(value),clone(pre.environment)))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): `Expressions`: row selected by pre.control.expr: tag,childSlice,armSlice — Expr tag dispatch and variant payload. Only the charged Eval path evaluates this row.; `ExprChildren`: childSlice[0] scrutinee — Ordered source child references; unused tables/empty slices do not imply an element fetch.; `Arms`: ordered armSlice named by Expressions.armSlice, carried into pending Match frame — The complete source arm list is retained as a slice; no pattern match or arm selection occurs in this rule.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Spine creation writes ControlRows 0–1. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A129 — Empty argument call**

Control: `Eval(Call(name,args))`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); args.is_empty().

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Retain name and construct empty ordered values; no frame or lookup yet.
4. Assign Snapshot.control = Some(Invoke(name,[]))
5. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
6. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Invoke(name,[]))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): `Expressions`: row selected by pre.control.expr: tag,nameId,childSlice (length zero); armSlice is unused — Expr tag dispatch and variant payload. Only the charged Eval path evaluates this row.; `ExprChildren`: childSlice length zero; no element fetch — Ordered source child references; unused tables/empty slices do not imply an element fetch.; `Arms`: armSlice unused; no arm fetch — This Expr variant has no arms.; `Names/NameBytes`: Expressions.nameId — Exact source name used for variable lookup, let binder or call name; logical reference may be carried without eagerly reading bytes.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A131 — Schedule first call argument**

Control: `Eval(Call(name,args))`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); args is nonempty.

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Save name, empty completed values, remaining expression suffix and current environment in pending Args.
4. Construct Eval first argument and pending Push using current environment.
5. Assign Snapshot.control = Some(Push(Args(name,[],args[1..],saved),Eval(args[0]),saved))
6. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
7. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Push(Args(name,[],args[1..],saved),Eval(args[0]),saved))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical table dependencies (MD4/MD6/MD7; no physical fetch implementation): `Expressions`: row selected by pre.control.expr: tag,nameId,childSlice (length positive); armSlice is unused — Expr tag dispatch and variant payload. Only the charged Eval path evaluates this row.; `ExprChildren`: childSlice[0] first argument; carry normalized suffix (start+1,len−1) when len>1, or (0,0) when len=1 as remaining expressions — Ordered source child references; unused tables/empty slices do not imply an element fetch.; `Arms`: armSlice unused; no arm fetch — This Expr variant has no arms.; `Names/NameBytes`: Expressions.nameId — Exact source name used for variable lookup, let binder or call name; logical reference may be carried without eagerly reading bytes. MD7: Each suffix output is (start+1, len−1) when len > 1, and (0,0) when len = 1. This is an owner-prescribed prerequisite; no physical P1 table is selected.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied. MD7 normalization prerequisite: Each suffix output is (start+1, len−1) when len > 1, and (0,0) when len = 1.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Spine creation writes ControlRows 0–1. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A135 — Install pending frame**

Control: `Push(frame,next,environment)`. Frames: `S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); len(S) < stack_limit.

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Compare current frame count with source stack_limit.
4. Insert pending frame at index zero; retain ordered old suffix S.
5. Install complete environment carried by Push.
6. Unbox next control without evaluating it.
7. Assign Snapshot.control = Some(*next)
8. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
9. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(*next)` (write); `environment = Push.environment` (write); `frames = [Push.frame] ++ pre.frames` (write); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan. Copy the code/slice reference already held in the matched frame/control (or unbox next for A135); no fresh Expressions/Arms evaluation is asserted. P1 requires the exact output reference rule before lowering.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: FrameSlots shift down. Push unboxing moves ControlRows row 1 to row 0 and zeroes it. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A140 — Frame limit reached**

Control: `Push(frame,next,environment)`. Frames: `S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); len(S) >= stack_limit.

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Compare frame count with source stack_limit; construct Error(4), discarding the pending Push frame and pending Push environment. Preserve Snapshot.environment and Snapshot.frames.
4. Assign Snapshot.control = Some(Error(4))
5. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
6. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Error(4))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A145 — Bind let and retain lexical scope**

Control: `Return(value)`. Frames: `Let(name,body,saved)::S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Replace top Let with Scope(clone(saved)); keep S and depth.
4. Restore saved environment.
5. Insert name -> value into restored map, overwriting any same-name binding.
6. Construct Eval(body).
7. Assign Snapshot.control = Some(Eval(body))
8. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
9. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Eval(body))` (write); `environment = saved[name <- value]` (write); `frames = Scope(saved)::S` (write); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan. Copy the code/slice reference already held in the matched frame/control (or unbox next for A135); no fresh Expressions/Arms evaluation is asserted. P1 requires the exact output reference rule before lowering.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Owner output obligation: copy expr0/expr1 from the matched frame as applicable; exact lane selection remains part of the unresolved physical plan.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner expectation: one EnvCell for let insertion.
- P4 [UNRESOLVED_PER_RULE]: Replace FrameSlots row 0 only. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A149 — Schedule right operand**

Control: `Return(value)`. Frames: `Left(op,right,saved)::S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Replace top Left with Right(op,value); no environment is stored in Right.
4. Restore saved environment.
5. Construct Eval(right).
6. Assign Snapshot.control = Some(Eval(right))
7. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
8. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Eval(right))` (write); `environment = saved` (write); `frames = Right(op,value)::S` (write); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan. Copy the code/slice reference already held in the matched frame/control (or unbox next for A135); no fresh Expressions/Arms evaluation is asserted. P1 requires the exact output reference rule before lowering.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Owner output obligation: copy expr0/expr1 from the matched frame as applicable; exact lane selection remains part of the unresolved physical plan.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: Replace FrameSlots row 0 only. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A153 — Compute binary primitive result**

Control: `Return(value)`. Frames: `Right(op,left)::S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Remove top frame before calling the pure primitive helper.
4. Evaluate ordered left/right helper operands and construct PrimitiveResult(Ok or Err).
5. Assign Snapshot.control = Some(PrimitiveResult(binary_result(op,left,value)))
6. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
7. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(PrimitiveResult(binary_result(op,left,value)))` (write); `environment = pre.environment` (preserve); `frames = S` (write); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical table dependencies (MD4/MD6/MD7; no physical fetch implementation): `Data/DataFields or Constants/ConstantFields`: Data/DataFields or Constants/ConstantFields per ValueRef.space — Inspect both binary operands as required by every binary branch: UInt arithmetic/Lt payloads, structural-Eq node/field traversal, and Cons/Branch operand typing; branch-specific field reads remain logical dependencies, not implemented components. Each value-node or field fetch needs its own G.4 guard, including the applicable source branch. Guard components and physical fetch implementation remain unresolved. This entry includes StateBlock Data/DataFields as well as ProgramBlock Constants/ConstantFields. The historical program_block_logical_reads key is retained; it must not restrict inspected values to one space.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: Structural Eq branch requires bounded node-pair worklist; other binary branches retain their source checks.
- P3 [UNRESOLVED_PER_RULE]: Owner expectations by successful primitive: arithmetic/Lt/Eq one Data node; Cons/Branch one Data node plus two DataFields. Error branches remain distinct; exact sequences unresolved.
- P4 [UNRESOLVED_PER_RULE]: FrameSlots shift up and zero row depth−1. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A157 — Compute unary primitive result**

Control: `Return(value)`. Frames: `Unary(op)::S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Remove top frame before calling pure unary helper.
4. Compute helper result and retain Ok/Err discriminator.
5. Assign Snapshot.control = Some(PrimitiveResult(unary_result(op,value)))
6. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
7. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(PrimitiveResult(unary_result(op,value)))` (write); `environment = pre.environment` (preserve); `frames = S` (write); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical table dependencies (MD4/MD6/MD7; no physical fetch implementation): `Data/DataFields or Constants/ConstantFields`: Data/DataFields or Constants/ConstantFields per ValueRef.space — Inspect the unary operand variant and its UInt or Bool payload for Leaf/Not and the type-error branch; fetch fields only where the source operation requires them. Each value-node or field fetch needs its own G.4 guard, including the applicable source branch. Guard components and physical fetch implementation remain unresolved. This entry includes StateBlock Data/DataFields as well as ProgramBlock Constants/ConstantFields. The historical program_block_logical_reads key is retained; it must not restrict inspected values to one space.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner expectations by successful primitive: Not one Data node; Leaf one Data node plus one DataField. Error branches remain distinct.
- P4 [UNRESOLVED_PER_RULE]: FrameSlots shift up and zero row depth−1. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A161 — Select true branch**

Control: `Return(Bool(true))`. Frames: `If(yes,no,saved)::S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Identify Bool condition; remove top If frame.
4. Restore saved environment before selecting the branch.
5. Select yes expression using the Boolean and construct Eval.
6. Assign Snapshot.control = Some(Eval(yes))
7. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
8. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Eval(yes))` (write); `environment = saved` (write); `frames = S` (write); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical table dependencies (MD4/MD6/MD7; no physical fetch implementation): `Data/DataFields or Constants/ConstantFields`: Data/DataFields or Constants/ConstantFields per ValueRef.space — Inspect the returned value variant and Boolean true payload for the true-branch premise; no recursive field traversal is implied by this Bool test. Each value-node or field fetch needs its own G.4 guard, including the applicable source branch. Guard components and physical fetch implementation remain unresolved. Copy the code/slice reference already held in the matched frame/control (or unbox next for A135); no fresh Expressions/Arms evaluation is asserted. P1 requires the exact output reference rule before lowering. This entry includes StateBlock Data/DataFields as well as ProgramBlock Constants/ConstantFields. The historical program_block_logical_reads key is retained; it must not restrict inspected values to one space.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Owner output obligation: copy expr0/expr1 from the matched frame as applicable; exact lane selection remains part of the unresolved physical plan.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: FrameSlots shift up and zero row depth−1. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A165 — Select false branch**

Control: `Return(Bool(false))`. Frames: `If(yes,no,saved)::S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Identify Bool condition; remove top If frame.
4. Restore saved environment before selecting the branch.
5. Select no expression using the Boolean and construct Eval.
6. Assign Snapshot.control = Some(Eval(no))
7. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
8. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Eval(no))` (write); `environment = saved` (write); `frames = S` (write); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical table dependencies (MD4/MD6/MD7; no physical fetch implementation): `Data/DataFields or Constants/ConstantFields`: Data/DataFields or Constants/ConstantFields per ValueRef.space — Inspect the returned value variant and Boolean false payload for the false-branch premise; no recursive field traversal is implied by this Bool test. Each value-node or field fetch needs its own G.4 guard, including the applicable source branch. Guard components and physical fetch implementation remain unresolved. Copy the code/slice reference already held in the matched frame/control (or unbox next for A135); no fresh Expressions/Arms evaluation is asserted. P1 requires the exact output reference rule before lowering. This entry includes StateBlock Data/DataFields as well as ProgramBlock Constants/ConstantFields. The historical program_block_logical_reads key is retained; it must not restrict inspected values to one space.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Owner output obligation: copy expr0/expr1 from the matched frame as applicable; exact lane selection remains part of the unresolved physical plan.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: FrameSlots shift up and zero row depth−1. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A169 — Reject non-Boolean condition**

Control: `Return(value)`. Frames: `If(yes,no,saved)::S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); value is UInt, List, or Tree.

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Fail Bool variant match; construct Error(5) without popping/restoring.
4. Assign Snapshot.control = Some(Error(5))
5. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
6. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Error(5))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical table dependencies (MD4/MD6/MD7; no physical fetch implementation): `Data/DataFields or Constants/ConstantFields`: Data/DataFields or Constants/ConstantFields per ValueRef.space — Inspect the returned value variant for the non-Boolean premise; no recursive field traversal is implied by this variant test. Each value-node or field fetch needs its own G.4 guard, including the applicable source branch. Guard components and physical fetch implementation remain unresolved. This entry includes StateBlock Data/DataFields as well as ProgramBlock Constants/ConstantFields. The historical program_block_logical_reads key is retained; it must not restrict inspected values to one space.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A173 — Start ordered case search**

Control: `Return(value)`. Frames: `Match(arms,saved)::S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Remove top Match frame.
4. Restore clone of saved environment.
5. Construct Cases with value, all ordered arms and original saved environment.
6. Assign Snapshot.control = Some(Cases(value,arms,saved))
7. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
8. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Cases(value,arms,saved))` (write); `environment = saved` (write); `frames = S` (write); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan. Copy the code/slice reference already held in the matched frame/control (or unbox next for A135); no fresh Expressions/Arms evaluation is asserted. P1 requires the exact output reference rule before lowering.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: FrameSlots shift up and zero row depth−1. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A177 — Accumulate argument and schedule next**

Control: `Return(value)`. Frames: `Args(name,values,rest,saved)::S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); rest is nonempty.

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Append returned value after all previously completed values in a local sequence.
4. Restore clone of saved caller environment before checking remaining arguments.
5. Inspect whether remaining expression sequence is empty.
6. Remove first remaining expression into next; retain ordered suffix.
7. Replace top Args with accumulated values and remaining suffix, preserving name and saved map.
8. Construct Eval(next expression).
9. Assign Snapshot.control = Some(Eval(rest[0]))
10. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
11. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Eval(rest[0]))` (write); `environment = saved` (write); `frames = Args(name,values ++ [value],rest[1..],saved)::S` (write); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical table dependencies (MD4/MD6/MD7; no physical fetch implementation): `ExprChildren`: remainingExprSlice[start]; carry normalized suffix (start+1,len−1) when len>1, or (0,0) when len=1 — Source rest.remove(0) selects next expression after value append and environment restore; direct occurrence output obligation P1. MD7: Each suffix output is (start+1, len−1) when len > 1, and (0,0) when len = 1. This is an owner-prescribed prerequisite; no physical P1 table is selected.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Owner output obligation: ExprChildren[slice.start] and the normalized suffix. Each suffix output is (start+1, len−1) when len > 1, and (0,0) when len = 1. Direct occurrence link, without in-step least-equal search; exact P1 table remains unresolved.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: ValueSlots growth; contiguous completed-values slice may require copying (owner O(n²) warning).
- P4 [UNRESOLVED_PER_RULE]: Replace FrameSlots row 0 only. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A181 — Finish argument list**

Control: `Return(value)`. Frames: `Args(name,values,rest,saved)::S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); rest is empty.

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Append returned value after all previously completed values in a local sequence.
4. Restore clone of saved caller environment before checking remaining arguments.
5. Inspect whether remaining expression sequence is empty.
6. Remove Args frame.
7. Construct Invoke(name, accumulated complete values).
8. Assign Snapshot.control = Some(Invoke(name,values ++ [value]))
9. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
10. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Invoke(name,values ++ [value]))` (write); `environment = saved` (write); `frames = S` (write); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): `ExprChildren`: remainingExprSlice length is zero; no element fetch — Source rest.is_empty() selects completion after append and environment restore.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: ValueSlots growth; contiguous completed-values slice may require copying (owner O(n²) warning).
- P4 [UNRESOLVED_PER_RULE]: FrameSlots shift up and zero row depth−1. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A185 — Typed function return**

Control: `Return(value)`. Frames: `Return(ty,saved)::S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); has_type(value,ty).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Check outer Value/Type sort only.
4. Remove Return frame on success.
5. Restore saved caller environment.
6. Retain identical Return(value) as next control.
7. Assign Snapshot.control = Some(Return(value))
8. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
9. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Return(value))` (write_equal); `environment = saved` (write); `frames = S` (write); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical table dependencies (MD4/MD6/MD7; no physical fetch implementation): `Data/DataFields or Constants/ConstantFields`: Data/DataFields or Constants/ConstantFields per ValueRef.space — Inspect the returned value sort for has_type success. The source has_type tests the sort, not recursive contents. Each value-node or field fetch needs its own G.4 guard, including the applicable source branch. Guard components and physical fetch implementation remain unresolved. MD2: post.control == pre.control must remain constrained; rewrite versus leave untouched is UNSELECTED. This entry includes StateBlock Data/DataFields as well as ProgramBlock Constants/ConstantFields. The historical program_block_logical_reads key is retained; it must not restrict inspected values to one space.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: FrameSlots shift up and zero row depth−1. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A190 — Reject wrong return type**

Control: `Return(value)`. Frames: `Return(ty,saved)::S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); not has_type(value,ty).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Check outer sort and fail; do not pop frame or restore saved environment.
4. Assign Snapshot.control = Some(Error(6))
5. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
6. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Error(6))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical table dependencies (MD4/MD6/MD7; no physical fetch implementation): `Data/DataFields or Constants/ConstantFields`: Data/DataFields or Constants/ConstantFields per ValueRef.space — Inspect the returned value sort for has_type failure. The source has_type tests the sort, not recursive contents. Each value-node or field fetch needs its own G.4 guard, including the applicable source branch. Guard components and physical fetch implementation remain unresolved. This entry includes StateBlock Data/DataFields as well as ProgramBlock Constants/ConstantFields. The historical program_block_logical_reads key is retained; it must not restrict inspected values to one space.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A195 — Restore lexical scope**

Control: `Return(value)`. Frames: `Scope(saved)::S`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Remove Scope frame.
4. Restore saved environment.
5. Retain identical Return(value).
6. Assign Snapshot.control = Some(Return(value))
7. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
8. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Return(value))` (write_equal); `environment = saved` (write); `frames = S` (write); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan. MD2: post.control == pre.control must remain constrained; rewrite versus leave untouched is UNSELECTED.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: FrameSlots shift up and zero row depth−1. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A200 — Lookup first named function**

Control: `Invoke(name,values)`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Scan complete program.functions in original order; stop at first exact-name match; clone found Function or return None.
4. Construct Enter with lookup result and unchanged ordered values.
5. Assign Snapshot.control = Some(Enter(first matching function or None,values))
6. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
7. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Enter(first matching function or None,values))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): `Functions`: scan rows in sourceTableIndex order through the first matching name, or all rows on miss — H-find preserves first-name-match behavior. Selected Function carries parameterSlice, returnType and bodyExprId; it does not bind parameters here.; `Names/NameBytes`: requested name and each scanned Functions.nameId — Exact source String equality; encoding/constraint mechanism unresolved.; `Parameters`: selected Functions.parameterSlice is carried, not bound or duplicate-scanned here — Logical payload of the cloned selected Function; parameter iteration belongs to A204.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: H-find ordered first-name-match scan.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A202 — Missing function**

Control: `Enter(None,values)`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Construct missing-function Error(7); no parameter binding.
4. Assign Snapshot.control = Some(Error(7))
5. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
6. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Error(7))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A204 — Bind function parameters**

Control: `Enter(Some(function),values)`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Run complete tail-first parameter binding, including count/type/duplicate failures; current state environment is not installed here.
4. Construct Bound retaining success/failure option, declared return type and body.
5. Assign Snapshot.control = Some(Bound(bind(function.parameters,values),function.return_type,function.body))
6. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
7. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Bound(bind(function.parameters,values),function.return_type,function.body))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical table dependencies (MD4/MD6/MD7; no physical fetch implementation): `Functions`: selected Enter.functionId: parameterSlice,returnType,bodyExprId — Source obtains the selected Function fields; binding result carries type and body.; `Parameters`: parameterSlice length first, then pairs in reverse source order — H-bind checks length, then has_type, then duplicate-name membership before insertion for each reversed pair.; `Names/NameBytes`: each visited Parameters.nameId — Parameter map keys and exact duplicate-name comparison; physical scan unresolved P2.; `Data/DataFields or Constants/ConstantFields`: Data/DataFields or Constants/ConstantFields per ValueRef.space — Inspect each argument sort when H-bind reaches has_type after the length check, in reverse parameter order; each source has_type tests sort, not recursive contents. Each value-node or field fetch needs its own G.4 guard, including the applicable source branch. Guard components and physical fetch implementation remain unresolved. This entry includes StateBlock Data/DataFields as well as ProgramBlock Constants/ConstantFields. The historical program_block_logical_reads key is retained; it must not restrict inspected values to one space.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: H-bind reverse pairs, type check before duplicate scan; duplicate scan O(k²).
- P3 [UNRESOLVED_PER_RULE]: One EnvCell per parameter in reverse order on the binding path; failure/partial-work constraints and full append sequence unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A206 — Reject binding failure**

Control: `Bound(None,ty,body)`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Construct binding Error(8).
4. Assign Snapshot.control = Some(Error(8))
5. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
6. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Error(8))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan. Copy the code/slice reference already held in the matched frame/control (or unbox next for A135); no fresh Expressions/Arms evaluation is asserted. P1 requires the exact output reference rule before lowering.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A208 — Schedule function body**

Control: `Bound(Some(bindings),ty,body)`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); Some(empty map) is successful and distinct from None.

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Capture current caller environment in pending Return frame.
4. Construct pending Push with body Eval and bindings to install later.
5. Assign Snapshot.control = Some(Push(Return(ty,clone(pre.environment)),Eval(body),bindings))
6. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
7. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Push(Return(ty,clone(pre.environment)),Eval(body),bindings))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan. Copy the code/slice reference already held in the matched frame/control (or unbox next for A135); no fresh Expressions/Arms evaluation is asserted. P1 requires the exact output reference rule before lowering.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Spine creation writes ControlRows 0–1. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A212 — No remaining match arms**

Control: `Cases(value,[],saved)`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Observe empty arms; construct Error(9).
4. Assign Snapshot.control = Some(Error(9))
5. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
6. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Error(9))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): `Arms`: held armSlice length is zero; no element fetch — Empty case list selects Error(9).

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A214 — Try first remaining pattern**

Control: `Cases(value,arms,saved)`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); arms nonempty.

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Remove first arm from local ordered list; retain its body and remaining suffix.
4. Match exactly that pattern against value starting from complete saved map; retain Some(bindings) or None.
5. Construct CaseResult preserving value, body, ordered suffix and saved environment separately from optional bindings.
6. Assign Snapshot.control = Some(CaseResult(matching(first.pattern,value,saved),value,first.body,rest,saved))
7. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
8. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(CaseResult(matching(first.pattern,value,saved),value,first.body,rest,saved))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical table dependencies (MD4/MD6/MD7; no physical fetch implementation): `Arms`: held armSlice[start].patternId and .bodyExprId; carry normalized suffix (start+1,len−1) when len>1, or (0,0) when len=1 — Source removes first arm before matching; P1 direct occurrence output, without boundary least-equal rewriting.; `Patterns`: selected Arms[start].patternId: tag,name0,name1 — H-matching selects pattern branch and zero to two bindings.; `Names/NameBytes`: selected binder names when present — Exact map keys; Cons/Branch equal-binder miss and head/left-before-tail/right insertion order.; `Data/DataFields or Constants/ConstantFields`: Data/DataFields or Constants/ConstantFields per ValueRef.space — Inspect the matched value variant and the list/tree fields required by the selected pattern, including Nil/Cons shape and Leaf/Branch fields; Var binding alone does not require recursive value inspection. Each value-node or field fetch needs its own G.4 guard, including the applicable source branch. Guard components and physical fetch implementation remain unresolved. This entry includes StateBlock Data/DataFields as well as ProgramBlock Constants/ConstantFields. The historical program_block_logical_reads key is retained; it must not restrict inspected values to one space. MD7: Each suffix output is (start+1, len−1) when len > 1, and (0,0) when len = 1. This is an owner-prescribed prerequisite; no physical P1 table is selected.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Owner output obligation: Arms[start] and its pattern/body references, with the normalized remaining suffix. Each suffix output is (start+1, len−1) when len > 1, and (0,0) when len = 1. Direct occurrence links, without in-step least-equal search; exact P1 table remains unresolved.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Zero to two EnvCells by pattern result; Cons-tail/Leaf/Branch children reuse existing field references with zero copies, per owner expectation.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A217 — Continue after pattern miss**

Control: `CaseResult(None,value,body,rest,saved)`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Discard failed body candidate; construct Cases with same value, rest and saved map.
4. Assign Snapshot.control = Some(Cases(value,rest,saved))
5. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
6. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Cases(value,rest,saved))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan. Copy the code/slice reference already held in the matched frame/control (or unbox next for A135); no fresh Expressions/Arms evaluation is asserted. P1 requires the exact output reference rule before lowering.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A220 — Schedule matching arm body**

Control: `CaseResult(Some(bindings),value,body,rest,saved)`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error); Some(empty map) remains successful.

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Construct pending Scope holding saved match environment and Eval selected body; supply bindings to pending Push, without installing either yet.
4. Assign Snapshot.control = Some(Push(Scope(saved),Eval(body),bindings))
5. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
6. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Push(Scope(saved),Eval(body),bindings))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan. Copy the code/slice reference already held in the matched frame/control (or unbox next for A135); no fresh Expressions/Arms evaluation is asserted. P1 requires the exact output reference rule before lowering.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Spine creation writes ControlRows 0–1. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A223 — Expose successful primitive value**

Control: `PrimitiveResult(Ok(value))`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Unwrap Ok and construct Return with complete value.
4. Assign Snapshot.control = Some(Return(value))
5. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
6. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Return(value))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

**A225 — Expose primitive error**

Control: `PrimitiveResult(Err(code))`. Frames: `any`. Premises: outcome == Pending; control is Some; remaining_fuel > 0; not (Return with empty frames or Error).

1. Require Pending outcome; obtain Some(control); capture complete before Snapshot before transition.
2. Terminal publication cases did not match; require remaining_fuel > 0 before charged transition.
3. Unwrap Err and construct Error with unchanged u8 code.
4. Assign Snapshot.control = Some(Error(code))
5. Subtract exactly one from remaining_fuel after transition returned and control was assigned.
6. Construct StepRecord(rule_start_line, charge, before, cloned complete after); this is event metadata, not another state update.

Complete post fields: `program = pre.program` (preserve); `control = Some(Error(code))` (write); `environment = pre.environment` (preserve); `frames = pre.frames` (preserve); `stack_limit = pre.stack_limit` (preserve); `remaining_fuel = pre.remaining_fuel - 1` (write); `initial_fuel = pre.initial_fuel` (preserve); `outcome = pre.outcome` (preserve).

Physical schedule: unresolved for this rule. The logical writes above do not choose arena append order, table active lengths, full row/column masks, canonical boundary reconstruction or preservation constraints.

Logical ProgramBlock dependencies (MD4; no physical fetch implementation): No additional ProgramBlock table dereference is asserted for this rule; held references and immutable ProgramBlock preservation still require the physical plan.

Owner extension prerequisites (exact P1–P6 text below; no obligation is discharged):

- P1 [UNRESOLVED_PER_RULE]: Specify exact direct output references for this rule; least-equal rewriting belongs only to boundary projection. No in-step equality search or deterministic post-image claim is supplied.
- P2 [UNRESOLVED_PER_RULE]: No scan/worklist is specifically named for this rule in owner P2; per-rule lowering must account for its logical operations. This entry does not invent a component or discharge boundedness.
- P3 [UNRESOLVED_PER_RULE]: Owner P3 lists no rule-specific arena append count here. Exact sequence, including a constrained empty sequence if appropriate, remains unresolved.
- P4 [UNRESOLVED_PER_RULE]: No rule-specific FrameSlots mask supplied by owner; derive exact writes and preservation before lowering. Exact ControlRows writes/zeroing remain unresolved. Guarded 128-bit fuel decrement for this charged rule.
- P5 [UNRESOLVED_PER_RULE]: Enable/guard every fetch/assert; exactly-one selector; preserve ProgramBlock, stack_limit, initial_fuel and untouched rows; Constant references may remain mid-segment, Data references required only at boundaries. All wiring unresolved.
- P6 [UNRESOLVED_PER_RULE]: For each append in the eventual per-rule sequence, capacity overflow must refuse and never become a semantic outcome. No append sequence or capacity number is selected.

The two no-event API branches (`outcome != Pending`, then `control == None`) preserve all eight fields and emit no StepRecord. They are not padding events or a newly admitted arbitrary-state relation. In particular, N-control `(Pending, None)` violates frozen adopted L4 O3 (design.md:73–80): it is unreachable from Machine::new and outside the observation domain. Encoding refuses it at Root.controlLength; it is not a circuit API state.

The inline operation catalog below supplies all outcomes inside charged rules. Its local temporary maps/vectors/results are not extra Snapshot cells or B helper states.

**H-push — Construct pending Push**

Source: Rust-only helper/map operation.

1. Return Push(frame,Box(next),environment). No installation, depth check, map installation or fuel update occurs inside helper.

- construct: all supplied typed inputs → Push(frame,next,environment).

**H-terminal — Publication discriminator**

Source: K89, K90, K91, K92, K93, K94, K95, K96, K97, K98, K99.

1. Return(empty-stack) and Error count as terminal. Every other control, including Return(nonempty-stack), is nonterminal.
2. Rust checks Return(empty) then Error then fuel zero.

- return-empty: Return(_) and frames empty → terminal.
- error: Error(_) with any frames → terminal.
- other: Eval/Push/Invoke/Enter/Bound/Cases/CaseResult/PrimitiveResult or Return(nonempty) → nonterminal.

**H-valid — K literal validity / typed representation**

Source: K230, K231, K232, K233, K234, K235.

1. K validates UInt range and all recursive list/tree children; Bool and Nil are valid.
2. Rust typed API has u128 leaves and finite constructed values; evaluate does not call a recursive validity function.

- uint: 0 <= N < 2^128 → true.
- uint-invalid: N outside range; K-only → false.
- bool-nil: Bool or empty List → true.
- cons: Cons(head,tail) → valid(head) and valid(tail).
- leaf: Leaf(value) → valid(value).
- branch: Branch(left,right) → valid(left) and valid(right).

This representation observation is not a circuit range/shape proof or admission statement.

**H-has-type — Outer sort check**

Source: K238, K239, K240, K241, K242, K243, K244, K245.

1. Match only the outer Value variant and requested Type. Contents are not traversed.

- uint: UInt(_), UInt → true.
- bool: Bool(_), Bool → true.
- list: List(_), List → true.
- tree: Tree(_), Tree → true.
- mismatch: all twelve unequal pairs of four Value variants and four Type variants → false.

**H-checked — Checked UInt128 arithmetic**

Source: K250, K252.

1. Return UInt result iff within 0..2^128-1; checked_add/sub/mul translate None to Err(10).

- in-range: 0 <= result < 2^128 → Ok(UInt(result)).
- out-of-range: result < 0 or result >= 2^128 → Err(10).

**H-binary — Binary primitive dispatch**

Source: K254, K255, K256, K257, K258, K259, K260, K261, K263, K265, K267, K269, K271.

1. Match (op,left,right). For arithmetic use checked operation; preserve left/right order.
2. Cons inserts head at index zero of the local tail vector; Branch constructs ordered left/right tree children.
3. Eq uses derived complete structural Value equality, including variant distinctions; it accepts all Value pairs.
4. Every unmatched operation/type combination returns Err(11).

- add: Add,UInt(a),UInt(b) → H-checked(a+b).
- sub: Sub,UInt(a),UInt(b) → H-checked(a-b).
- mul: Mul,UInt(a),UInt(b) → H-checked(a*b).
- lt: Lt,UInt(a),UInt(b) → Ok(Bool(a < b)).
- eq: Eq,a,b for any typed Values → Ok(Bool(structural_equal(a,b))).
- cons: Cons,UInt(head),List(tail) → Ok(List([head] ++ tail)).
- branch: Branch,Tree(left),Tree(right) → Ok(Tree(Branch(left,right))).
- type-error: Add/Sub/Mul/Lt without two UInt; Cons without UInt,List; Branch without Tree,Tree → Err(11).

Numeric range branches belong to H-checked. Eq never returns type error solely because variants differ. Source calls derived equality; no extra A event or physical visit order is specified.

**H-unary — Unary primitive dispatch**

Source: K273, K274, K275, K277.

1. Match operation and Value sort; construct Leaf or Boolean negation, otherwise Err(11).

- leaf: Leaf,UInt(n) → Ok(Tree(Leaf(n))).
- not: Not,Bool(b) → Ok(Bool(!b)).
- leaf-type-error: Leaf with Bool/List/Tree → Err(11).
- not-type-error: Not with UInt/List/Tree → Err(11).

**H-find — Ordered first-match function lookup**

Source: K281, K282, K283.

1. Scan from first function to last, comparing exact names.
2. Stop immediately at first match and clone its complete Function; do not inspect later duplicate-name rows for selection.
3. Return None after the sequence is exhausted.

- empty-or-absent: no matching row → None.
- first-equal: row i matches and no row j<i matches → Some(clone(functions[i])).

**H-env-get — Current environment lookup**

Source: Rust-only helper/map operation.

1. Use map.get(name); distinguish absent from a found UInt(0), Bool(false), empty List or any other Value.

- found: name in map → Some(complete value).
- absent: name not in map → None.

Map lookup is an atomic pure dependency of A108/A111; no EnvGet helper-state event is added.

**H-env-put — Map insertion / overwrite**

Source: Rust-only helper/map operation.

1. Insert key with new Value, replacing any old mapping for exactly the same Name; retain every other map entry.

- new-key: name absent → map union name->value.
- shadow: name present → same complete map except name maps to new value.

Used in let, local function binding, and matching; the rule/helper context determines which map is updated. It does not imply one physical EnvCell append under unadopted L. This entry describes a map operation, not a pure Rust function: when its argument is Snapshot.environment (A145), that caller explicitly writes the state cell.

**H-append — Argument order append**

Source: K286, K287.

1. Clone/capture completed sequence from top frame; append new value at the end before inspecting remaining expressions.

- empty: completed=[] → [value].
- nonempty: completed=[v0,...,vn] → [v0,...,vn,value].

Rust Vec::push and K uiAppend have this result; no Append microsteps or helper-work charges are emitted.

**H-bind — Tail-first complete parameter binding**

Source: K290, K291, K292, K293, K294, K296, K297.

1. Check lengths first; unequal lengths return None before map creation or per-parameter tests.
2. Create empty local Environment.
3. Visit zipped pairs from last to first (reverse source order). At each pair, call has_type first; only if it succeeds inspect contains_key because the || expression short-circuits.
4. On wrong type or already-bound same name, return None immediately; discard local partial map.
5. Otherwise insert cloned name/value and continue toward first pair.
6. Return Some(complete map), including Some(empty map) for zero pairs. State.environment is not written by this helper.

- too-few: values length < parameters length → None.
- too-many: values length > parameters length → None.
- empty: both lengths zero → Some(empty map).
- type-mismatch: first visited failing reverse-order pair has wrong outer sort → None.
- duplicate: current pair name already exists among successfully bound later parameters → None.
- success: equal lengths; every sort matches; names distinct → Some(all parameter bindings).

K recursively builds the tail before uiBindOne. Rust checks total length up front. Same returned binding behavior is observed from source; internal helper trace equivalence is not claimed or charged.

**H-matching — One pattern against one value**

Source: K300, K301, K302, K304, K305, K307, K308, K310, K311.

1. Clone complete saved environment before dispatch.
2. Var inserts whole value. Nil accepts only empty List and performs no insertion.
3. Cons requires List, distinct head/tail names and nonempty list; insert head UInt first, then tail List.
4. Leaf requires Tree::Leaf; insert contained UInt. Branch requires Tree::Branch and distinct names; insert left Tree first, then right Tree.
5. Successful inserts shadow existing saved bindings; all untouched saved keys remain.
6. Any other combination returns None; no partial local map is installed into Snapshot.

- var: Var(name), any Value → Some(saved[name <- value]).
- nil: Nil,List([]) → Some(clone(saved)).
- cons: Cons(h,t),List([x]++xs),h!=t → Some(saved[h <- UInt(x)][t <- List(xs)]).
- leaf: Leaf(n),Tree(Leaf(x)) → Some(saved[n <- UInt(x)]).
- branch: Branch(l,r),Tree(Branch(a,b)),l!=r → Some(saved[l <- Tree(a)][r <- Tree(b)]).
- nil-miss: Nil with nonempty List or UInt/Bool/Tree → None.
- cons-miss: Cons with non-List, empty List, or equal binders → None.
- leaf-miss: Leaf with non-Tree or Tree::Branch → None.
- branch-miss: Branch with non-Tree, Tree::Leaf, or equal binders → None.

**H-kind — K helper kind partition**

Source: K314, K315, K316, K317, K318, K319.

1. Partition UInt, Bool, Nil, Cons, Leaf, Branch for pattern-miss rules. Rust matching uses enum and Vec emptiness tests directly.

- uint: UInt → 0.
- bool: Bool → 1.
- nil: empty List → 2.
- cons: nonempty List → 3.
- leaf: Tree::Leaf → 4.
- branch: Tree::Branch → 5.

The JSON enumerates every source enum variant and names the covering rules/helpers, all source struct fields, exact K/Rust source spans, per-operation read/write dependencies, helper branches, and a full eight-field effect map on every rule. Static validation checks membership against source text; it does not execute the evaluator or establish semantic equivalence.

A rules retain their current clock. Candidate B helpers, source origins, helper/event counters and allocation accounting remain separate. Financial continuations/effects/liabilities and general nominal ADTs remain extensions requiring their own semantics. None is represented by empty fields here. The generic G2 row-fetch candidate is still unregistered source and does not implement these rules. This manifest neither registers it nor changes its six frozen files.

The next concrete work is fresh full nonsecurity and designated Opus review, then a reviewed L-specific physical operation/write map and actual emitter components. Source observations, metadata coverage, circuit correctness, native execution and parent acceptance remain separate. No Rust/Cargo/SDK/Lean/K/Quint execution was performed.


**Owner P1–P6: unresolved physical extension prerequisites**

The following text is copied exactly from /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-reference-20261015/opus-sprint-loop-20261015/machine-layout-and-rule-manifest-security-opus-r2/RESULT.md (SHA256 `986b3ec742e008924261bd9b297feca8a968f288525f2e6f2e2d0f437a6e984d`). It records the owner’s requirements, not a new security decision or a completed physical plan. All per-rule masks and append sequences remain null.

**P1 — Reference-output rule per rule.** Mid-segment outputs should be the direct occurrence links:
- A177 outputs `ExprChildren[slice.start]` and the slice (start+1, len−1);
- A214 outputs `Arms[start]`;
- A145/A149/A161/A165 copy `expr0`/`expr1`.

Least-equal rewriting then belongs only to boundary projection (F4). This removes any need for in-step equality search and is required for a deterministic post-image (A4).

Owner follow-up correction MD7 (historical P1 quotation above preserved): Each suffix output is (start+1, len−1) when len > 1, and (0,0) when len = 1. This applies to A131, A177 and A214. The old quotation omits the empty-suffix case and is not the complete current requirement. Source: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-reference-20261015/opus-sprint-loop-20261015/machine-layout-l3-and-rules-r2-opus-r1/RESULT.json (`text`, MD7), SHA256 `4bf361ba4b4d5f1e30d407b64bf8030ed79c4d1a6a565f2e603feb8f4863c5ee`. This correction selects no P1 table or physical implementation.

**P2 — Bounded intra-rule work within one charge.** Structural Eq in A153 is the dominant item: arbitrary DAG values, with exponential unfolded size (F3), within one A event. It needs a bounded worklist component over node pairs, with Capacity refusal that is never Err(11) or `false`. Two more scans need bounded components too:
- H-find's ordered scan;
- H-env-get's newest-first chain walk.
H-bind's duplicate scan is O(k²).

**P3 — Exact append sequences.** Expected appends per rule:
- arithmetic, Lt, Eq and Not: one Data node;
- Cons and Branch: one node and two DataFields;
- Leaf: one node and one field;
- A145: one EnvCell;
- A204: one EnvCell per parameter, in reverse order;
- A214: zero to two EnvCells;
- A177/A181: ValueSlots growth.

The completed-values slice must be contiguous, so it may need copying, giving O(n²) usage. H-matching Cons-tail, Leaf and Branch children reuse existing field references with zero copies; this alignment with L2 is good and should be stated.

**P4 — FrameSlots and ControlRows write masks.**
- A135: shift down.
- Pops (153/157/161/165/173/181/185/195): shift up and zero row depth−1.
- Replaces (145/149/177): row 0 only.
- Spine creation writes rows 0–1; unboxing moves row 1 to row 0 and zeroes it.
- Root writes: a guarded 128-bit decrement on charged rules only, and outcome publication.

**P5 — Guards and preservation.**
- G.4 enable/guard on every family's fetches and asserts.
- An exactly-one-rule selector realizing the partition above.
- Preservation wiring for the ProgramBlock, `stack_limit`, `initial_fuel` and untouched rows.
- Constant-space references allowed mid-segment and Data-space references required only at boundaries.

**P6 — Capacity separation per append.** Each append must refuse on overflow and never fall through to a semantic outcome.

**R2 source and review limits**

The r1 runtime packet and source history are unchanged. This revision changes the two A-manifest files only. It preserves all 42 K machine rules, 39 Rust event labels, 71 helper rules, 55 variants and eight Snapshot field maps. Source operations, fuel charges, effect maps and helper definitions are unchanged except A140 wording. No Rust, K, Lean, Quint, SDK, build or native execution occurred. No G3, L, library registration or policy change is included. Old counters and terminal boundaries are unchanged; native authority remains zero. The old mixed-model SDK refusal supplies no accepted owner decision or grant.


**R5 current source integration limits**

The source-anchor map binds O3 to frozen L4 design.md:73–80 and the full ProgramBlock table list, including ConstantFields, to lines 110–126. The separate A-REFERENCE-OUTPUTS-EVAL.json r2 candidate expands all basis hashes and states controlLength'=1 explicitly for A102/A108/A111/A129, as directed by Muse and Root. Its 12 Opus-authored reference-output rows otherwise retain their original aliases and obligations; 27 rows remain UNWRITTEN. Original Opus bytes and prior L2/r4 records are frozen unchanged in the packet. Astra authored only this metadata integration and the four explicit source-fact clarifications. No rule source/effect/fuel/helper fact, guard policy, MD2 choice, physical mask/append, or F4 mechanism is selected here. Fresh full Opus and Muse reviews are required before use.
