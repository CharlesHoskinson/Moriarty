# Sherlock suspension and finance (profile sherlock-helper-microstep/2)

Status: DESIGN NOTE, Phase 1, 2026-10-25. Decided by sherlock-u under the owner
decision relayed by the coordinator ("change whatever you like in the design of
sherlock ... we can't change ledger 9"). Nothing here is implemented, compiled or
run. Pending Opus security review and GPT Sol review before Phase 2. It lifts the
option-C hold on U014, U015, U017, U018, U019 and U020.

Claims scope. This note decides host semantics (the reference machine, the draft
language, the Core/2 financial route, the CLI/JSON contracts) that U014–U020 test
at tier T1. Native C6 constraints for the new control/outcome/frame tags, the
decider statement change and a stagenet run are specified here for direction and
remain later work with their own evidence. No section below claims a proof,
circuit or target result.

Hard constraint honoured throughout: no change to Midnight ledger 9, node, SDK or
proof server. Everything settles through the accept/settle anchor pattern that
A9 (`tools/sherlock-sdk/sherlock-anchor.compact`) and B4 (factory-b
`tools/sherlock-sdk/rollup-anchor.compact`) used on stagenet.

## 0. Sources read

- Spec: `openspec/changes/sherlock-universal-interpreter-design/specs/universal-interpreter/spec.md:98-145` (UIREQ-014..020); holdout case descriptions `qa/holdout/cases/uireq/{014,015,017,018,019,020}/case.json` (read-only).
- `openspec/.../interfaces.md:240` (technical cut is not Waiting) and `:308` (financial extension must add committed/tentative state, ordered effects, residual duties, pending observations and phase/cost fields together; Commit/Yield/Await and phase mapping OPEN).
- `docs/sherlock/design.md:73-77` (profile adapter matrix; "Pending/None is outside the A observation domain"; profiles kept separate).
- Core/2: `crates/mori-core/src/ir.rs:100-127` (`Statement::Await{label,binders,guard}`), `stage/mod.rs:113-193` (`Continuation`, `StageOutcome`, `residual`), `stage/admit.rs:24-67,84-217,287` (`layout_matches`, `finish`, `emit_checked`, `resume` R0–R7), `runtime.rs:942-1029` (`run_segment`), `docs/CORE-CONTINUATIONS.md`.
- Machine: `crates/mori-core/src/sherlock/helper_machine/shared.rs` (`ControlC` :932, `OutcomeC` :1022, `FrameRowC` :855, `IdentityC` :542, `FinUnsupportedOp` :1426, `FinImageState` :1436, `BoundaryKind` :70), `shared/owner_runtime.rs:3685-3851` (`owner_step`, `owner_run`), `shared/fin_exec_state.rs`, `shared/fin_selection.rs`, `shared/fin_carrier.rs`, `c6_phys.rs` lane map.
- Ledger 9 pinned `04c9c5d`: `spec/transaction-format.md:155,178-219`, `spec/contracts.md:94-113`, `ledger/src/verify.rs:1946-1959`.
- Oracle: `models/sherlock/oracles/financial-prefixes.json` case F01 (reserve 100, liability 40).

## 1. Settlement shape first (the hard constraint)

A Sherlock run on chain is a linear chain of settled roots. Today's anchors hold
`root`, `height`, `batch_no`, a `validity_ok` set filled by `acceptValidity` (whose
verifier key is the handwritten ledger-9 decider D, installed by key override) and
a `settled` set consumed by `settleValidity`, which requires the predecessor root to
equal the current `root` (sherlock-anchor.compact `f[6] == r`; rollup-anchor.compact
`f[0] == root`) and then advances `root` (A: `root = f[9]`; B4: `root = f[1]`).

Decisions:

1. **The digest of a Waiting checkpoint is an ordinary `root`.** A batch that ends
   in `Waiting(site)` settles exactly like any other batch; its post root is the
   checkpoint digest `CK(image)` (§3.4). Nothing on chain distinguishes "waiting"
   from "running"; the distinction lives in the image committed under the root.
2. **The observation digest rides inside the Sherlock-owned statement, not in a new
   ledger lane.** The ledger exposes only `[binding_input, communication_commitment,
   transcript ops]` to a call proof (verify.rs:1946-1959). The D statement already
   hashes a trace commitment over the batch (A: `f[15] == trace`, B4: leaf lanes).
   A resume batch's trace commitment additionally absorbs `OD(observation)` (§3.3)
   at the position of the resume event. D and its statement layout are ours to change.
3. **Anchor contract.** Neither existing anchor is generic: the A anchor pins a
   two-step A102/A80 program (`f[13] == N_STEPS()`, `f[14] == WORK()`), and the B4
   anchor pins the rollup header span (`f[16] == h + 50`). A general Sherlock run
   therefore needs a new Compact contract `sherlock-run-anchor` of the same shape:
   `validity_ok`/`settled` sets, `root`, `height`, `batch_no`, sealed pins for
   `instance_id`, `keys_id`, `profile_id`, `program_c` and `genesis_root`, and a
   `settleValidity(batch, fields)` that checks predecessor root/height against state,
   recomputes `stmt` and advances `root`. This is a new contract deployment (allowed:
   stagenet redeploys are authorized), not a ledger, node, SDK or proof-server change.
   It uses only `Set<Field>`, `Field`, `Uint<64>`, `transientHash` and `disclose`,
   all already used by the deployed anchors.
4. **Phase policy on chain is `guaranteed-only`.** All Sherlock calls keep the
   guaranteed-only shape A9/B4 used (`crates/mori-recursion/tests/fixtures/accept-template.json`:
   "all guaranteed, none fallible"; deployments pin `phase_policy == "guaranteed-only"`,
   `crates/mori-ledger/src/layout.rs:142`). §8 maps source outcomes onto that.

Consequence: suspension adds no on-chain state machine. Waiting, resume and local
failure are all deterministic machine transitions inside a batch; the chain only
sees roots, and root linearity is what makes a checkpoint single-use on chain (§5).

## 2. Two surfaces, one machine

| Surface | Language | Requirement | Where await lives |
| --- | --- | --- | --- |
| Pure | `moriarty-sherlock-draft/2` | U014, U015 | expression level, inside calls with pending frames (`10 + inner(3)`) |
| Financial | Core/2 `moriarty/2` actions on the Sherlock fin route | U017, U018, U019, U020 | statement level, top of an action (Core/2 places awaits only there) |

Both lower to the same machine primitives: one `Await` control, one `Waiting`
outcome, one `AwaitGuardF` frame, one observation format, one `resume` transition
and one `XFinCommit` rule. §6 and §7 give the two lowerings.

## 3. (a) Universal machine primitives

### 3.1 Primitive set (decided)

The universal machine gets exactly these additions. **Await** is the primitive.
**Yield** is not a separate opcode: `yield L;` is surface sugar for
`await L() when true` (zero binders, constant-true guard). **Commit** is a machine
rule `XFinCommit`, never a surface statement in /2. Core/2 has no commit statement,
and commits stay implicit at every await and at completion, exactly as Core/2
publishes its boundary there. A technical chunk cut is none of these
(interfaces.md:240 and UIREQ-016 unchanged).

### 3.2 New tags

| Kind | Name | C6 tag | Payload | Notes |
| --- | --- | --- | --- | --- |
| `ControlC` | `Await{site, guard, binders}` | 11 | site: u16 (1-based, per program), guard: ExprId, binders: ordered (BinderId, Sort) slice id | existing tags 0–10 unchanged |
| `OutcomeC` | `Waiting{site}` | 4 | site | not terminal, not Running; existing 0–3 unchanged |
| `FrameRowC` | `AwaitGuardF{site, saved_env, origin}` | 10 | existing 13-lane frame record | existing tags 0–9 unchanged |
| `RuleIdC` (fin group) | `XFinCommit`, `XFinAwaitEnter`, `XFinResumeAccept` | n/a | | joins the existing `FinRuleId::AwaitEnter` header in fin_selection.rs |
| `RuleIdC` (pure) | `BAwaitEnter`, `BAwaitGuardReturn` | n/a | | replaces the reserved `BBookkeepingResumeStore/Restore` extension rules |
| `BoundaryKind` | `SourceSuspensionExtensionReserved` (name kept) | 3 | | /2 starts using the existing variant and tag 3; no /1 enum or codec is renamed |

`ExhaustReason` is unchanged.

### 3.3 Surface syntax and typing

Draft language `moriarty-sherlock-draft/2` (the `/1` grammar is unchanged and
keeps refusing `await` as an unknown keyword):

```
expr ::= ... | "await" LABEL "(" [binder ("," binder)*] ")" "when" expr ";" expr
           | "yield" LABEL ";" expr
binder ::= IDENT ":" ("UInt" | "Bool")
```

`await L(x1: T1, ..., xn: Tn) when G; E` has the type of `E`. The binders scope
over `G` and `E` only. Typing rules:

- `T_i` is `UInt` or `Bool`, the exact Core/2 observation domain (`ir.rs:34`).
- `G : Bool`.
- `G` is **call-free**: literals, variables, `+ - * == < not if` only. No named
  calls, constructors or `match` (decided in §6.3).
- Binders may not shadow any name in scope.
- Labels are unique per program.
- The Core/2 bounds apply: at most 16 binders per site and 16 sites per function.

Example (U014/U015):

```
language "moriarty-sherlock-draft/2";
program PendingInner {
  fn inner(n: UInt): UInt { await reply(v: UInt) when v == n + 1; v }
  fn outer(): UInt { 10 + inner(3) }
  entry outer();
}
```

Core/2 surface is unchanged (`await L(b: T, ...) when G;`, parser.rs:333-348,
validate.rs:166-200). The fin route admits it as is.

### 3.4 Checkpoint and digest

A checkpoint is the complete `MachineImage` encoded by the existing C6 HOST codec
(META, PROG, CTX, STATE), extended for /2:

- **Image header** gains three lanes after `FRAME_COUNT`: `RESUME_EPOCH` (u64,
  accepted resumes so far), `LAST_OBS_DIGEST` (2 lanes, lo/hi of the last accepted
  observation digest, or zero at genesis) and `WAIT_SITE` (mirrors the outcome
  payload for fixed-offset reads).
- **Fin section:** a new child of STATE holds `FinImageState` (committed snapshot,
  committed effect log, tentative snapshot and effects, retained env/captures,
  statement cursor, segment index, committed logical work, phase policy).
- **Carrier version:** the C6 padded carrier version is bumped. /1 carriers
  decode only under /1.

**One checkpoint identity.** `CK(image)` is the Field-domain state commitment
over the C6 HOST lane vector, the same function the decider leaf uses for `root`,
computed identically by host and circuit. It is written as 64 hex characters of
the field element's canonical bytes. There is no separate host SHA-256 name.
The CK an observation binds to (R3) is therefore exactly the root the anchor
consumes (§5.3).

`OD(obs)` is the same Field-domain hash over the admitted fixed-width lane vector
`[DOM_OBS, CK, site, label_id, v_1, ..., v_n]`. In that vector `label_id` is the
site's admitted label index, UInt values are their u128 lanes and Bool values are
0 or 1. The JSON file is only a carrier: R3–R5 decode it into that vector, and the
digest never hashes JSON text, so JSON canonicalization is not part of the
binding.

### 3.5 Resume API

Pure (draft) route:

```
mori simulate SOURCE --execution-profile sherlock-helper-microstep/2 \
  --execution-config CONFIG --entry \
  [--checkpoint-out FILE]                      # write the Waiting checkpoint
mori simulate SOURCE --execution-profile sherlock-helper-microstep/2 \
  --execution-config CONFIG \
  --resume CHECKPOINT --observation OBS [--checkpoint-out FILE]
```

Financial route: the same flags with `--source-profile moriarty/2 --action NAME
--scenario FILE` on start. `--resume`/`--observation` replace action/scenario on
resume, mirroring Core/2's `--resume/--event`. `--event` is accepted as an alias
of `--observation` only on the Core/2 legacy route, which stays unchanged.

Observation file (`--observation`, at most 64 KiB, the Core/2 `MAX_EVENT_BYTES`):

```json
{"format":"sherlock-observation/2",
 "checkpoint":"<hex CK of the checkpoint being resumed>",
 "site":1, "label":"reply",
 "values":[{"name":"v","value":{"type":"UInt","value":"4"}}]}
```

Report (stdout JSON). Every report carries `profile` and the Core/2 labels
`qualification`, `authentication: "none"` and `consumption: "untracked"`.

| `status` | Exit code | Members |
| --- | --- | --- |
| `waiting` | 0 | `site`, `label`, `checkpoint` (CK hex), `checkpoint_file` (if `--checkpoint-out`), `binders` (name/type layout the next observation must match), `committed_work`, `attempted_work`, `frames` (bottom-to-top summary), `fin` (§7.4, or null) |
| `not_ready` | 3 | `checkpoint` (= input CK, unchanged), `site`, `attempted_work`, `reason` (`guard_false`, `guard_error{code}` or `guard_exhausted`); with `--checkpoint-out` the written file is byte-identical to the `--resume` input (`cmp` holds) |
| `refused` | 4 | `stage` (`R0`..`R7`, §4), `error`; the checkpoint is not read past the failing check and no successor exists (2 is left to clap usage errors) |
| `halted` | 0 | `value`, `committed_work`, `fin` |
| `rejected` | 1 | `code`, `committed_work`, `fin` (§7.5) |
| `exhausted` | 1 | `reason`, `committed_work`, `fin` |

`committed_work` is the image's `consumed_work`. `attempted_work` is host-reported
work that is not in any checkpoint.

## 4. (b) Observation validity and binding

An observation is admissible against checkpoint `C` iff all of the following hold,
in this order. The first failure is the reported `stage`. R0–R7 mirror Core/2
`stage::resume` (admit.rs:287) plus `layout_matches` (admit.rs:24).

| Stage | Check |
| --- | --- |
| R0 | `C` decodes under `sherlock-helper-microstep/2` (profile id 3 in META, carrier version /2, full SI01–SI24 host invariants plus the new SI25–SI28 below). Low fuel is not a refusal: a guard that runs out of fuel is `not_ready{guard_exhausted}` (§6.3), the only outcome for that condition |
| R1 | `C.outcome == Waiting{site}` and `C.control == Await{site, ..}` with the same site; a Running/terminal checkpoint is refused, not "not ready" |
| R2 | admission identity in `C` (program membership digest, layout and bounds digests, and for fin the Core/2 `program_hash`/`plan_hash`) equals the supplied source's admission; the CLI action, if given, equals the checkpoint's action |
| R3 | `obs.format == "sherlock-observation/2"` and `obs.checkpoint == CK(C)` (binds the observation to exactly one checkpoint) |
| R4 | `obs.site == site` and `obs.label` equals the site's label |
| R5 | `obs.values` matches the site's binder layout exactly: same length, same names in the same order, sorts equal; UInt is a canonical decimal u128 string; Bool is a JSON bool |
| R6 | (fin route) the Core/2 cap preflight: effective cap ≥ committed + next segment static cost; the cap now lives in bound META (§7.2) |
| R7 | the canonical-boundary decode of `C`'s fin snapshot (the Core/2 `canonical_boundary`) |

Sorts are restricted to UInt/Bool so that an observation is a fixed-width,
admission-bounded vector. It cannot inject unbounded data or references into the
heap. Binder values enter as fresh scalar value slots.

**Binding.** On accept, the successor image records `LAST_OBS_DIGEST = OD(obs)` and
`RESUME_EPOCH + 1`. The resume batch's trace commitment absorbs `(CK(C), OD(obs))`.
A settled successor is therefore bound to its exact predecessor and observation.

**Authentication: none (deliberate, as Core/2).** Sherlock does not decide whether
an observation is true. The guard is the program's own admission check on the
observation, and on chain the observation is whatever the settling submitter
proved. An application that needs an attested source must check it in the guard
(for example, equality with a value fixed at an earlier, settled step) or at the
contract layer. /2 adds no oracle or signature primitive.

## 5. (c) Replay rules

1. **Not-ready is byte-identical.**
   - `resume` evaluates the guard on a scratch copy of the image.
   - If the guard does not return `true` (false, a runtime `Error`, or fuel
     exhaustion inside the guard), the copy is discarded. The output checkpoint is
     the input bytes, and `CK` is unchanged.
   - Attempted guard work is reported host-side (`attempted_work`) and is in no
     checkpoint.
   - This deliberately departs from the historical `models/sherlock/quint/universalMachine.qnt`,
     where a false observation increments an in-state `observations` counter. In
     /2 nothing about a rejected attempt enters state. That is what lets an
     attacker's junk observations change nothing.
   - The Core/2 match: a false resume returns `Waiting{advanced:false, continuation:
     <input unchanged>}` (admit.rs:101-119), with attempted work only.
2. **Local resume is a pure function.** `resume(C, obs)` is deterministic: the same
   `(C, obs)` gives the same successor bytes. Locally, consumption is untracked, the
   same as Core/2 today. A user can resume one checkpoint file many times, and with
   different observations gets different successors. The tool does not and cannot
   stop that.
3. **On chain a checkpoint is single-use.**
   - The anchor's `settleValidity` requires predecessor root == current `root` and
     then replaces `root`. Two successors of the same `CK` can both reach
     `validity_ok`, but only the first to settle advances the root. The second then
     fails the predecessor check (refused by the node before inclusion, as A9
     observed for stale settles P1).
   - `settled` additionally refuses an identical statement replay.
   - So a resumed checkpoint can be resumed into at most one settled state.
4. **No state recurs.** `RESUME_EPOCH` increases on every accepted resume, and
   `consumed_work` increases on every charged event. So a successor's `CK` never
   equals any earlier `CK` of the same run, and root linearity cannot be defeated
   by returning to an old root.

## 6. (d) Work charging across suspension, and the pure lowering

### 6.1 Charging

| Event | Reference fuel | helper_work | Committed? |
| --- | --- | --- | --- |
| `BAwaitEnter` / `XFinAwaitEnter` (Eval of await, Running→Waiting) | 1 | 0 | yes, part of the batch that ends Waiting |
| Waiting itself; delivering an observation; R0–R7 checks | 0 | 0 | no work; checks are host work |
| guard evaluation events | normal per-rule charges | normal | only if the guard returns `true` |
| `BAwaitGuardReturn` / `XFinResumeAccept` (guard returned true) | 1 | 0 | yes |
| not-ready attempt | 0 in state | 0 | reported only as `attempted_work` |
| `XFinCommit` | 0 (it is part of the event that triggers it, §7.3) | 0 | yes |

- **Fuel is fixed at genesis.** An observation never adds fuel, and `initial_fuel`,
  `max_reference_work` and `step_cap` are unchanged by resume. This bounds total
  computation per run regardless of how many observations arrive.
- If `remaining_work == 0` when `Await` would be entered, the existing P2 rule
  publishes `Exhausted(Fuel)` uncharged, as for any event.
- Waiting is checked before P2: a Waiting image is never exhausted while it waits.
- The SI-03/SI-04 invariants hold across suspension unchanged
  (`remaining + consumed == initial_fuel`; `reference_cursor == consumed - helper_work`).
- Core/2 logical work (`moriarty-reference-logical-cost/2`) stays a separate
  counter in `FinImageState.committed` on the fin route. It equals
  `plan.committed_before[site]` at every Waiting and is never mixed with Sherlock
  fuel (UIREQ-013).

### 6.2 Pure lowering (U014, U015)

- **`Eval(await)` charges `BAwaitEnter`.**
  - control := `Await{site, guard, binders}`
  - outcome := `Waiting{site}`
  - The frames, env root, heap tables and counters are untouched. This is the
    checkpoint.
  - For `10 + inner(3)` the frame stack bottom-to-top is
    `outer`'s return frame, then the pending `+` frame holding the evaluated
    left operand 10, then `inner`'s return frame. The exact `FrameRowC` rows
    (variant per row: `ReturnF`, `RightF`, `ReturnF`, and any `ArgsF`/`ScopeF`
    the existing call schedule leaves) are pinned by the Phase 2 visible test from
    an actual run, not asserted here. The current env is inner's
    (`n = 3`), and the caller env is saved in the frames.
  - This is the UIREQ-014 snapshot: caller frame, control, lexical contexts,
    reachable values, budgets. On this route the financial prefix is `null`.
- **`owner_step` on a Waiting image** returns `Terminal` (no next) and changes
  nothing. `owner_run` stops there, and the CLI prints `waiting`.
- **`resume(C, obs)` after R0–R7**, on the scratch copy:
  - extend the env with fresh cells binding `x_i := v_i`;
  - push `AwaitGuardF{site, saved_env = env before the binders, origin}`;
  - control := `Eval{G}`;
  - outcome := `Running`;
  - step until control is `Return{b}` with `AwaitGuardF` on top.
- **If `b == true`, charge `BAwaitGuardReturn`.**
  - Pop `AwaitGuardF` and keep the binder env.
  - control := `Eval{E}`.
  - Set `LAST_OBS_DIGEST` and increment `RESUME_EPOCH`.
  - Continue `owner_run`.
  - With `v = 4`, `inner` returns 4 and `outer` returns `10 + 4`, giving
    `Halted(14)` (U015).
- **Otherwise (false, Error, Exhausted) the scratch copy is discarded** and the
  result is `not_ready` (U015 "false/not-yet-ready leaves the waiting checkpoint
  unchanged").

### 6.3 Why the guard is call-free

- A call-free guard has a static cost bound: one event per expression node, at
  most the guard's node count, recorded at admission.
- A not-ready attempt costs at most that bound in host time.
- Exhaustion inside a guard is possible only when remaining fuel is below the
  bound. The report then says `guard_exhausted` and the checkpoint is unchanged.
- An adversary cannot make the guard loop or allocate.
- Recursive or allocating checks belong in `E`, after acceptance, where their work
  is committed and settled like any other work.

## 7. (e)/(f) Financial route: Core/2 typed await on the universal machine (U017, U018, U019)

### 7.1 Model

- Balances, liabilities and state are **contract-internal accounting committed
  under `root`**, as in Core/2, where `Snapshot{state, balances, liabilities}` is
  program state. They are not Midnight coins.
- Moving real NIGHT/DUST/shielded value is out of /2 scope. It would be a custody
  adapter over the anchor contract and is not assumed here.
- The reference figures (reserve 68, liability 10, gross debit 32, recipient
  receipt 30, residual duty 10) are the independently authored oracle F01 in
  `models/sherlock/oracles/financial-prefixes.json`, not implementation output.

### 7.2 Owner run beyond genesis (`FinUnsupportedOp::Run`/`Step`/`Select` lifted)

- **Genesis is unchanged** (`admit_fin` / `admit_fin_run`, owner_prepare.rs:2892,
  3068).
- **`owner_step` and `owner_run` execute the XFin rules over the admitted Core/2
  action**, one statement per owner event, driven by `FinExecState`:
  - `statement_cursor`, `segment_index`, `tentative_snapshot`,
    `tentative_effects` and `retained_env` (MU1, adopted by this note);
  - the MU2 `FinStatementKind`/`FinRuleId` selection (adopted);
  - each rule applies through the existing checked `fin_carrier` mutators
    (`apply_transfer`, `apply_liability_add`, `apply_set_state`, plus a new
    `apply_discharge` with Core/2's "Unknown residual liability" and "Discharge
    exceeds residual liability" checks).
- Effects are appended to `tentative_effects` in source order (`Effect::{State,
  Transfer, Fee, Liability, Emit}`, ir.rs:221-251).
- **Statement kinds:** `Let`, `Transfer`, `Fee`, `Liability`, `Discharge`, `Next`,
  `Emit`, `Requires`, `Ensures` and `Await`.
- **The Core/2 cap moves into bound META.** It is `logical.cap`, which in Core/2
  sits outside both hashes and is caller-editable (CORE-CONTINUATIONS.md:132).
  In /2 it is fixed at admission and cannot be edited between resumes.
- The remaining `FinUnsupportedOp` variants (Capture, Codec, ObservePure, StepInv)
  are lifted alongside: capture and codec gain the fin section (§3.4).
  `ObservePure` stays refused, because a financial image has no pure view by
  design.

### 7.3 Commit and await are one event (checkpoint atomicity)

At an `Await` statement at site `s`, a single owner event `XFinAwaitEnter` does
all of the following atomically:

1. `XFinCommit`:
   - committed snapshot := tentative snapshot;
   - committed effect log ++= tentative effects;
   - tentative := empty;
   - `committed` logical work := `plan.committed_before[s]`, checked equal, as
     Core/2 `finish` checks.
2. Capture `retained_env` per the site's capture layout (inputs, lets, earlier
   binders).
3. control := `Await{s, guard, binders}`, outcome := `Waiting{s}`.

At the end of the action, `XFinComplete` = `XFinCommit` then `Halted(unit)` in one
event.

A technical cut lands only between owner events (draft_chunked.rs). So no
checkpoint can exist with a committed prefix but no Waiting control, or with a
Waiting control over an uncommitted tentative delta. That is the Core/2
`emit_checked` atomicity carried into the machine. Resume follows §6.2 with the
fin route's rule names. On accept, execution continues at statement `ordinal + 1`
as a new tentative segment `segment_index + 1`.

### 7.4 Correspondence with Core/2 (U018)

| Core/2 `Continuation` / `StageOutcome::Waiting` | /2 Waiting image |
| --- | --- |
| `program_hash`, `plan_hash` | admission `FinPreparedStagedSource` hashes (META) |
| `action` | `FinImageState.action_index` → action name |
| `site` | `Await.site` = `Waiting.site` |
| `captures` | `FinExecState.retained_env` in site capture order |
| `logical.profile`, `logical.committed` | `FinImageState.committed` (same cost profile) |
| `logical.cap` | bound META cap (deliberate change: no longer editable) |
| `boundary: Snapshot` | committed snapshot |
| `residual` | positive-amount liabilities of the committed snapshot (`stage::residual`) |
| `segment` (published) | the committed effect-log slice of the last segment, plus its pre/post snapshots |

The U018 adapter `project_core2(image) -> (Continuation, StageOutcome-shape)`
compares field by field with `stage::start`/`stage::resume` on the same program
and inputs. For the F01 action with `await` after the first paid prefix, the
committed snapshot shows reserve 68 and liability 10, and the residual is
`[pay-beneficiary: 10]` in both, as are committed logical work and captures.

**Stated difference from Core/2 (U019).** Core/2 `StageRejection` publishes nothing
and leaves the caller's input continuation valid for retry. /2 instead publishes a
terminal `Rejected{code}` image (§7.5) that retains the committed prefix and
charges the failed segment's consumed work. This is the behaviour UIREQ-019
requires ("retained prefix ... plus later consumed work"). The U018 equality claim
covers Waiting and Complete. On rejection, the adapter maps /2's (committed prefix,
consumed − committed-at-last-commit) to Core/2's (input continuation boundary,
`attempted`). It does not claim the same object.

Locally the earlier Waiting checkpoint file still exists. On chain, settling the
Rejected successor consumes the root (§5).

### 7.5 Local segment failure (U019)

- **What triggers it:** a runtime error inside a tentative segment, such as
  insufficient balance, an unknown or over-discharged liability, `requires` or
  `ensures` false, or overflow.
- **What it publishes:** one event `XFinLocalFailure` with
  outcome := `Rejected{code}`;
  - committed snapshot and effect log are unchanged;
  - tentative effects move to `discarded_effects`, a terminal-only diagnostic
    field, so the report can show the discarded delta;
  - `consumed_work` keeps every charged event of the failed segment.
- **Report `fin` block:**

```json
{"committed":{"snapshot":{...},"effects":[...],"logical_work":N},
 "discarded_tentative":{"effects":[...],"segment_index":2},
 "residual_duties":[{"name":"pay-beneficiary","amount":"10"}],
 "accounting":{"gross_debit":"32","fees":"2","recipient_receipt":"30"},
 "ledger_rollback":"none-inferred"}
```

- **F01 continued:** reserve 68 and liability 10 are retained, later consumed work
  is reported, and no global ledger rollback is inferred.
- **The accounting block** is `account_effects` (financial_projection.rs:451) over
  the committed log only, with gross = payments + fees and fees counted once.
  U017's first paid prefix reports reserve 68, liability 10, gross debit 32,
  recipient receipt 30 and residual duty 10.

## 8. (g) U020 phase policy

**Two layers, kept separate.**

**L1: source phase policy.** This is a bound META field selected at admission:

- **`all-or-nothing/1` (default).**
  - A local segment failure discards the whole tentative segment, fees included.
  - This is Core/2's and U019's semantics.
- **`retain-guaranteed-fees/1`.**
  - The segment's `Fee` statements are its guaranteed part, and everything after
    them is its fallible part.
  - Admission requires that, within each segment, every `Fee` statement precedes
    every non-`Let`/`Requires` effect statement. Fees therefore apply to the
    committed snapshot directly, and applying them alone can never underflow.
  - On local failure in the fallible part, `XFinLocalFailure` commits exactly the
    segment's fee effects. The rest is discarded, and the outcome is
    `Rejected{code}`.
  - If a fee itself fails, nothing is retained, as in the default.
  - The report adds `"retained_by_policy":{"policy":"retain-guaranteed-fees/1",
    "effects":[fee...]}`. The committed prefix, the policy-retained fee, the
    discarded delta and the remaining duties are four separate members, which is
    the U020 scenario.

**L2: target phase.** Every Sherlock settle is one Midnight call whose transcript
is entirely guaranteed (segment 0). That is the only target-qualified policy and
the one A9/B4 used. The compiler's phase report states the correspondence exactly:

| Source outcome | Machine image | Ledger 9 phase result | DUST fee |
| --- | --- | --- | --- |
| segment committed (await or complete) | Waiting / Halted, root advances | guaranteed section applies: `SucceedEntirely` | paid |
| local segment failure (either policy) | Rejected with retained prefix (+ fees under retain policy), root advances | guaranteed section applies: `SucceedEntirely` (the failure is a deterministic machine result, not a ledger failure) | paid |
| not-ready observation | no successor, no batch | no transaction | none |
| settle refused (stale root, replay, bad stmt) | none | guaranteed section fails, the transaction is not included (A9 P1–P4: node refusal code 1010/104 before inclusion) | none taken |
| fallible-segment placement | not used | not used | n/a |

A mapping of `retain-guaranteed-fees/1` onto a ledger-9 fallible segment (DUST and
fee in segment 0, Sherlock batch in segment ≥ 1, `PartialSuccess` retaining the
fee) is **report-only, with concrete target evidence pending**. The current ledger
consumer refuses fallible transcripts (`sherlock_literal.rs:310`), so it is not
selectable in /2. The ledger's guaranteed/fallible semantics are used as they
are, not changed.

## 9. (h) Profile id bump and compatibility

- **Identity.** New profile `sherlock-helper-microstep/2` with `IdentityC.profile_id = 3`.
  The values 0, 1 and 2 keep their meaning: k-a/1, quint-a/1 and microstep/1.
- **Grammar and finance domains.** /2 accepts grammar `moriarty-sherlock-draft/2`
  and `moriarty/2` fin. `FinanceInput` keeps its /1 variants and encoded tags. /2
  admits the existing `TypedExtensionReserved` variant as the Core/2 staged finance
  domain. No /1 variant is renamed or retagged.
- **profile_id is bound in /2.** In the /1 bound META relation, META lane 3
  (`profile_id`) is deliberately runtime and unbound (c6_meta_canonical.rs docs).
  The /2 circuits constrain it to the constant 3, and the /2 decider statement
  and run-anchor pin `profile_id`. Without this, a /1 statement could be presented
  as /2.
- **What /1 keeps.**
  - Programs: the `/1` profile, its tags, codecs, circuits, receipts and evidence
    are untouched.
  - CLI: `--execution-profile sherlock-helper-microstep/1` refuses `await`/`yield`
    and finance as today (AdmissionRejected / FinancialUnsupportedBeyondGenesis).
- **Behaviour of a /1-shaped program under /2.** A program using no /2 feature
  admitted under /2 produces the same event sequence and the same work counters as
  under /1 (no rule changed). Its checkpoint digests differ, because META
  profile_id differs.
- **No cross-profile resume or claim transfer.**
  - No cross-profile resume or import: SI-01 refuses a /1 checkpoint under /2 and
    vice versa.
  - No /1 receipt or holdout pass transfers to /2 without a /2 run.
- **Code sites.** About 9 string and 5 numeric sites select the profile (listed in
  the Phase 2 diff). Each /2 site is added beside the /1 one, never by editing /1.

## 10. (i) Security argument

Threats and the invariant that answers each:

| # | Threat | Invariant / mechanism |
| --- | --- | --- |
| T1 | Observation replayed against a different checkpoint | R3 binds `obs.checkpoint == CK(C)`; the trace commitment absorbs `(CK, OD)` |
| T2 | Double resume into divergent states | Locally untracked by design (as Core/2). On chain, root linearity (`f_pre == root`, then root replaced) and `settled` give at most one settled successor per root; `RESUME_EPOCH` and monotone `consumed_work` prevent root recurrence (§5.3–5.4) |
| T3 | Forged observation | Authentication is `none` (labelled). The program's guard is the admission check; the binder sorts UInt/Bool and fixed layout bound what can enter. Applications needing attestation must encode it in the guard |
| T4 | Griefing via junk observations | Not-ready is byte-identical and work-free in state; the guard is call-free with a static cost bound (§6.3); no fuel is added by observations |
| T5 | Profile confusion (/1 vs /2) | profile_id = 3 constrained in /2 circuits and pinned in the anchor; no cross-profile resume (SI-01) |
| T6 | Technical cut presented as commit or Waiting | Commit happens only inside `XFinAwaitEnter`/`XFinComplete`/`XFinLocalFailure`; a cut produces no event and leaves `BoundaryKind::TechnicalCut`; SI-25 requires `fin.tentative` to be empty whenever outcome ≠ Running |
| T7 | Cap editing between resumes | The cap is in bound META (unlike Core/2 `logical.cap`) |
| T8 | Fee-retention underflow or reordering | `retain-guaranteed-fees/1` admission requires fees first in each segment; retained fees apply to the committed snapshot with the existing checked mutators |
| T9 | Waiting image treated as terminal or exhausted | `owner_step` on Waiting returns no-next before P2; only `resume` leaves Waiting; Waiting is not Halted (UIREQ-011 distinction preserved) |
| T10 | Guard escapes its scope | `AwaitGuardF` saves the pre-binder env; binders are fresh cells; no shadowing; guard call-free |
| T11 | Discarded delta leaking into later state | `discarded_effects` exists only on a Rejected (terminal) image; SI-26 requires it to be empty otherwise |

New host invariants for /2:

- SI-25: outcome ≠ Running ⇒ tentative empty.
- SI-26: `discarded_effects` ≠ ∅ ⇒ outcome is Rejected.
- SI-27: Waiting{s} ⇔ control is `Await{s,..}`, and `WAIT_SITE == s`.
- SI-28: on the fin route, Waiting{s} ⇒ `committed == plan.committed_before[s]`.

## 11. States and transitions (for the chip animation)

### 11.1 States

| State | `OutcomeC` tag | Control tag | Fin tentative | Steppable | Resumable | Settles as root |
| --- | --- | --- | --- | --- | --- | --- |
| Running | 0 | 0–10 | any | yes | no | yes (mid-run cut) |
| Waiting(site) | 4 | 11 (Await) | empty | no | yes | yes |
| Halted(value) | 1 | 1 (Return, depth 0) | empty | no | no | yes |
| Rejected(code) | 2 | 9 (Error) | empty (discarded kept aside) | no | no | yes |
| Exhausted(reason) | 3 | unchanged | empty after commit rule; pure route n/a | no | no | yes |

On the fin route, `Exhausted` mid-segment applies the same rule as local failure:
the tentative delta is discarded and the committed prefix kept. The report names
the reason, not a code.

### 11.2 Transitions

| Id | Name | From → To | Rule | Charge (fuel) | State change |
| --- | --- | --- | --- | --- | --- |
| E1 | Step | Running → Running | any existing rule | per rule | as /1 |
| E2 | AwaitEnter (pure) | Running → Waiting(s) | `BAwaitEnter` | 1 | control := Await, outcome := Waiting |
| E3 | Commit+AwaitEnter (fin) | Running → Waiting(s) | `XFinAwaitEnter` | 1 | tentative → committed; captures saved; control/outcome as E2 |
| E4 | Observe-Refused | Waiting(s) → Waiting(s) | R0–R7 fail | 0 | none (no successor exists) |
| E5 | Observe-NotReady | Waiting(s) → Waiting(s) | guard ≠ true | 0 in state (attempted reported) | none; bytes identical |
| E6 | Observe-Accept | Waiting(s) → Running | guard steps + `BAwaitGuardReturn` / `XFinResumeAccept` | guard events + 1 | binders bound, obs digest and epoch recorded |
| E7 | Complete | Running → Halted | P1 publish (pure) / `XFinComplete` (fin) | 0 / 1 | fin: tentative → committed |
| E8 | LocalFailure | Running → Rejected | `XFinLocalFailure` (fin) / existing error publish (pure) | 0 publish | tentative → discarded (fees → committed under retain policy) |
| E9 | Exhaust | Running → Exhausted | existing P2/P3/P6 | as /1 | fin: tentative → discarded, committed prefix kept; under `retain-guaranteed-fees/1` the segment's fees are committed exactly as in E8 |
| E10 | TechnicalCut | any → same | none (between events) | 0 | none; not a commit, not Waiting |
| S1 | AcceptValidity | (on chain) | D-proved `stmt` inserted in `validity_ok` | DUST fee | none to root |
| S2 | Settle | root R → root R' | `settleValidity` with predecessor == R | DUST fee | root := CK-commitment of the batch's last image (may be Waiting) |

Typical U015 sequence: E1×k, E2 (root R1 = Waiting), E5 (v = 5, R1 unchanged),
E6 (v = 4), E1×m, E7 → `Halted(14)`. Typical U019 sequence: E1×a, E3 (68/10
committed), E6, E1×b, E8 → `Rejected`, with 68/10 retained.

## 12. Phase 2 plan (after review approval)

1. Core types and tags (§3.2), the C6 host codec extension (§3.4), SI-25..28, and
   the profile id 3 sites.
2. Pure route: `moriarty-sherlock-draft/2` parser/checker (await/yield, call-free
   guard), `BAwaitEnter`/`BAwaitGuardReturn`, and `resume`. Visible tests: the U014
   snapshot contents; U015 accept (14), not-ready byte identity and R3 wrong-checkpoint
   refusal.
3. Fin route: the XFin rules with `owner_step`/`owner_run` beyond genesis, plus
   commit+await atomicity, local failure and the phase policy. Visible tests: F01
   U017 figures, the U018 Core/2 field comparison, the U019 retained 68/10 plus
   consumed work, and the U020 retain-policy report.
4. CLI/JSON contracts (§3.5, §7.5, §8 table) posted for the evaluator before the
   holdouts are authored.
5. Update `docs/sherlock/design.md` and the interfaces.md:308 OPEN text to record
   these decisions.
6. Later, separate evidence: /2 C6 circuits for tags 11/4/10 and the bound
   profile_id; the decider statement with `(CK, OD)` absorption; the
   `sherlock-run-anchor` contract; a stagenet run.
