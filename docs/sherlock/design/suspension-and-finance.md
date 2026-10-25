# Sherlock suspension and finance (profile sherlock-helper-microstep/2)

Status: DESIGN NOTE, Phase 1, 2026-10-25. Decided by sherlock-u under the owner
decision relayed by the coordinator ("change whatever you like in the design of
sherlock ... we can't change ledger 9"). Nothing here is implemented, compiled or
run. Revision 2 closes the five Opus security review findings (2 HIGH, 1 MED,
2 LOW; §13). Revision 3 closes the three GPT Sol findings on 0bd74deaa (2 High,
1 Medium; §14). Revision 4 closes the Opus re-check of rev 3 (§15). Phase 2
implementation is in progress. The coordinator approved the §1.3 anchor interpretation. The note lifts the
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
   Rev 2: the settle call also discloses the observation itself, and the anchor
   recomputes `OD` (§1.5). This is a contract argument, not a ledger lane.
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
   all already used by the deployed anchors. Coordinator-approved. The exact
   circuit is §1.5.
4. **Phase policy on chain is `guaranteed-only`.** All Sherlock calls keep the
   guaranteed-only shape A9/B4 used (`crates/mori-recursion/tests/fixtures/accept-template.json`:
   "all guaranteed, none fallible"; deployments pin `phase_policy == "guaranteed-only"`,
   `crates/mori-ledger/src/layout.rs:142`). §8 maps source outcomes onto that.

5. **§1.5 `sherlock-run-anchor` specification (rev 2).**

   Ledger fields:
   - `validity_ok: Set<Field>`, `settled: Set<Field>`, `root: Field`,
     `height: Uint<64>`, `batch_no: Uint<64>`;
   - sealed `instance_id`, `keys_id`, `profile_id` (= 3), `program_c`,
     `genesis_root`, `genesis_height`, `genesis_batch`.

   **`keys_id` (MED finding 3).** It commits to the /2-only verifier-key set: the
   /2 leaf/step keys and the /2 decider key. No /1 key is in it. The /2 decider
   refuses any child proof whose key is not in that set. A /1 leaf therefore cannot
   be folded into a /2 statement, even though the /1 META leaves profile_id unbound.
   (rev 4) `keys_id` is not a free statement input. D computes it as
   `transientHash([DOM_KEYS2, vk_hash_1, ..., vk_hash_k])` over the child
   verifier keys it actually verified, in fixed order. The anchor compares that
   derived value to its sealed pin.

   `acceptValidity(stmt)`:
   - asserts `!settled.member(stmt)` (restored from the deployed anchors);
   - inserts `stmt` into `validity_ok`.

   `settleValidity(batch: Uint<64>, fields: Vector<F, Field>, obs: Vector<19, Field>)`.
   `fields` carries the D statement lanes:
   - `keys_id`, `program_c`, `profile_id`;
   - `ck_pre`, `h_pre`, `ck_post`, `n_steps`;
   - `trace`;
   - `od`, which is zero for a batch with no resume.

   `settleValidity` asserts each of the following:
   1. `batch == batch_no + 1`. `batch_no` and `instance_id` are bound into `stmt`
      (LOW finding 4).
   2. The pins equal their sealed values: `keys_id`, `program_c`, `profile_id`.
   3. Root and height:
      - `ck_pre == root` and `h_pre == height`;
      - `n_steps >= 1`;
      - `h_post = h_pre + n_steps` is computed by the anchor, not taken from the
        prover;
      - wrap guards on height and batch.
   4. **Observation disclosure (HIGH finding 1).**
      - `obs = [site, label_id, n, v_0, ..., v_15]`. It is disclosed, so the
        observation is public in the call transcript.
      - When `od != 0` the anchor recomputes
        `transientHash([DOM_OBS2, ck_pre, site, label_id, n, v_0..v_15])` and asserts
        it equals `od`.
      - It casts the count lane to a typed integer, `const n = obs[2] as Uint<8>`
        (the cast range-checks it), then asserts `n <= 16` and `v_i == 0` for every
        `i >= n` (rev 4: Compact compares typed `Uint`, not `Field`).
      - When `od == 0`, `obs` must be all zero.
   5. `stmt = transientHash([DOM_STMT2, instance_id, batch, keys_id, program_c,
      profile_id, ck_pre, h_pre, ck_post, h_post, n_steps, trace, od])`. Then
      `!settled.member(stmt)` and `validity_ok.member(stmt)`.
   6. It moves `stmt` from `validity_ok` to `settled`, then sets `root := ck_post`,
      `height := h_post` and `batch_no := batch`.

   **Decider constraints on batch shape (rev 4).** For every /2 batch, D enforces:
   - **Waiting ends a batch.** An image whose outcome is `Waiting` may appear
     only as the batch's final image, so `ck_post` is its CK. No event follows a
     Waiting image inside a batch.
   - **Resume only first.** At most one `BResumeAccept`/`XFinResumeAccept` occurs
     per batch, and only as event 0, applied to the batch's input image
     (`ck_pre`).
   - **`od` and resumes agree.** `od != 0` if and only if event 0 is a resume.
     Then `od` equals the `LAST_OBS_DIGEST` written by that event; otherwise
     `od == 0`.

   The D statement absorbs `od` at the resume event's position in `trace`, and the
   leaf proves that the image after `XFinResumeAccept` / `BResumeAccept`
   recorded that same `od`. So the disclosed observation, the proved transition and
   the settled root are one object. A settler cannot settle one observation while
   publishing another, and cannot settle without publishing.

   **Who owns the observation on chain (decided).** Authentication stays `none`. The
   observation that counts is the one in the first settled successor of the
   Waiting root. Any party able to produce a valid proof can race to settle, and
   the observation it uses becomes public.

   /2 deliberately adds no observer pin. A sealed observer key would need an
   in-circuit signature or caller-identity check, and this note does not assume
   that Compact facility. It is recorded as a future anchor option. Applications
   that need a single observer must restrict who can produce proofs (private
   inputs held by that observer) or check attestation in the guard.

Consequence: suspension adds no on-chain state machine. Waiting, resume and local
failure are all deterministic machine transitions inside a batch; the chain only
sees roots, and root linearity is what makes a checkpoint single-use on chain (§5).

## 2. Two surfaces, one machine

| Surface | Language | Requirement | Where await lives |
| --- | --- | --- | --- |
| Pure | `moriarty-sherlock-draft/2` | U014, U015 | expression level, inside calls with pending frames (`10 + inner(3)`) |
| Financial | Core/2 `moriarty/2` actions on the Sherlock fin route | U017, U018, U019, U020 | statement level, top of an action (Core/2 places awaits only there) |

Both lower to the same machine primitives: one `Await` control, one `Waiting`
outcome, one observation format, one atomic resume relation
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
| `ControlC` | `Await{site, expr}` | 11 | lanes `[11, site, expr]`: site u16 (1-based, per program); expr = the await expression row, from which binders, guard and body are read (rev 4: matches the implemented `ControlC::Await`) | existing tags 0–10 unchanged |
| `OutcomeC` | `Waiting{site}` | 4 | site | not terminal, not Running; existing 0–3 unchanged |
| `RuleIdC` (fin group) | `XFinCommit`, `XFinAwaitEnter`, `XFinResumeAccept` | n/a | | joins the existing `FinRuleId::AwaitEnter` header in fin_selection.rs |
| `RuleIdC` (pure) | `BAwaitEnter`, `BResumeAccept` (atomic resume relation, §6.2) | n/a | | replaces the reserved `BBookkeepingResumeStore/Restore` extension rules |
| `BoundaryKind` | `SourceSuspensionExtensionReserved` (name kept) | 3 | | /2 starts using the existing variant and tag 3; no /1 enum or codec is renamed |

`ExhaustReason` gains `LogicalCap` (tag 7, /2 only, §6.1); tags 0–6 are unchanged.

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
- `G` is **call-free and branch-free**: literals, variables, `+ - * == < not`
  only. No named calls, constructors, `match` or `if` (decided in §6.3).
  - Rev 4: the circuit evaluates every node of the straight-line guard, but a
    lazy host `if` would skip the arm not taken. A failure in that arm would then
    let the host accept a successor the circuit cannot prove.
  - Forbidding `if` makes host and circuit evaluate exactly the same nodes. Any
    error in any node (`-` underflow, `+`/`*` overflow) makes the guard
    not-ready in both.
  - A Core/2 guard containing a conditional is refused at /2 admission as
    `Unsupported(GuardConditional)`.
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
- **Await capture section (rev 3):** 8 scalar lanes plus a count. It holds the
  guard operands copied at await entry; it is zero outside Waiting (SI-31).
- **Carrier version:** the C6 padded carrier version is bumped. /1 carriers
  decode only under /1.

**One checkpoint identity.** `CK(image) = transientHash-tree(DOM_CK2 ‖ profile_id ‖
carrier_version ‖ C6 HOST lane vector)`. This is the Field-domain state commitment
the decider leaf uses for `root`, computed identically by host and circuit.
`DOM_CK2` is the packed ASCII tag `"SHK_CK2"`, separating /2 checkpoint digests
from every other hash domain, /1 roots included (LOW finding 5). It is written as 64 hex characters of
the field element's canonical bytes. There is no separate host SHA-256 name.
The CK an observation binds to (R3) is therefore exactly the root the anchor
consumes (§5.3).

`OD(obs) = transientHash([DOM_OBS2, CK, site, label_id, n, v_0, ..., v_15])`. The
vector has a fixed width of 21 Field lanes:
- `DOM_OBS2` is the tag `"SHK_OBS2"`;
- `n` is the explicit binder count, at most 16 (the Core/2 bound);
- `v_i` for `i >= n` is required to be 0;
- `label_id` is the site's admitted label index;
- UInt values are their u128 lanes; Bool values are 0 or 1.

This is the same function the anchor recomputes (§1.5). The JSON file is only a carrier: R3–R5 decode it into that vector, and the
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
| `not_ready` | 3 | `checkpoint` (= input CK, unchanged), `site`, `attempted_work`, `reason` (`guard_false` or `guard_error{code}`); with `--checkpoint-out` the written file is byte-identical to the `--resume` input (`cmp` holds) |
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
| R0 | `C` decodes under `sherlock-helper-microstep/2` (profile id 3 in META, carrier version /2, full SI01–SI24 host invariants plus the new SI25–SI29 below). A checkpoint violating SI-29 cannot be produced by the machine and is refused here as malformed |
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
observation. On chain the first settler of the Waiting root's successor owns the
observation. Settlement discloses it, and the anchor recomputes and checks `OD`
(§1.5), so the settler cannot hide it or substitute another. An application that needs an attested source must check it in the guard
(for example, equality with a value fixed at an earlier, settled step) or at the
contract layer. /2 adds no oracle or signature primitive.

## 5. (c) Replay rules

1. **Not-ready is byte-identical.**
   - `resume` evaluates the guard on a scratch copy of the image.
   - If the guard does not return `true` (false or a runtime `Error`;
     exhaustion is unreachable by SI-29), the copy is discarded. The output checkpoint is
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
4. **Replay safety rests on the anchor pins, not on state non-recurrence (rev 2,
   LOW finding 4).**
   - Some events charge 0 (E7, E8 and `XFinCommit`), so `consumed_work` alone is
     not a strict clock.
   - The rev-1 claim "no state recurs" is withdrawn as the safety argument.
   - Single settlement per root comes from the pins in §1.5:
     - the predecessor pins `ck_pre == root` and `h_pre == height`;
     - the anchor-computed `h_post = h_pre + n_steps` with `n_steps >= 1`, so height
       strictly increases;
     - `batch == batch_no + 1`;
     - `instance_id` and `batch` are inside `stmt`;
     - `!settled.member` in both `acceptValidity` and `settleValidity`.
   - A statement can therefore settle at most once, and only at its exact
     (root, height, batch) position, even if an image recurs.
   - `RESUME_EPOCH` remains as a host diagnostic and an extra input to `CK`.

## 6. (d) Work charging across suspension, and the pure lowering

### 6.1 Charging (rev 3)

| Event | Reference fuel | helper_work | Allocation | Committed? |
| --- | --- | --- | --- | --- |
| `BAwaitEnter` / `XFinAwaitEnter` (Running→Waiting) | 1 | 0 | none; guard operands copied into the Await capture section (§3.4) | yes, part of the batch that ends Waiting |
| Waiting itself; delivering an observation; R0–R7 checks | 0 | 0 | none | no work; checks are host work |
| `BResumeAccept` / `XFinResumeAccept` (the atomic resume relation, §6.2) | `1 + |G_s|` (static) | 0 | pure: `n_s` DATA rows + `n_s` env cells; fin: `n_s` retained-env binder slots | yes |
| not-ready attempt | 0 | 0 | none | no relation instance exists; nothing is reported in state |
| `XFinCommit` | 0 (part of the event that triggers it, §7.3) | 0 | none | yes |

`|G_s|` is the guard's admitted node count and `n_s` is the binder count of site
`s`. Both are static per site.

- **The guard is not executed by machine microsteps (rev 3, Sol findings 1 and 3).**
  - It is evaluated inside the single resume relation, as a straight-line scalar
    program over circuit wires.
  - Its intermediate values are never DATA rows, env cells, frames or helper tasks.
  - So the guard's cost is exactly its fixed charge `1 + |G_s|`, and its only
    allocation is the `n_s` binder cells written on accept.
  - The rev-2 bound, derived from expression-node counts of machine microsteps, is
    withdrawn: microsteps include frame, validation, environment-lookup and
    scalar-helper events, and allocate DATA rows (`scalar.proposed.rs:216-235`).
- **Fuel is fixed at genesis.** An observation never adds fuel, and `initial_fuel`,
  `max_reference_work` and `step_cap` are unchanged by resume.
- If `remaining_work == 0` when `Await` would be entered, the existing P2 rule
  publishes `Exhausted(Fuel)` uncharged, as for any event.
- **Resumability check at await entry (HIGH Opus finding 2, revised in rev 3).**
  When control is `Eval(await)` (pure) or the Await statement (fin), the entry rule
  checks, after its own charge of 1:

  | Resource | Requirement | Exhaust reason on failure |
  | --- | --- | --- |
  | fuel | `remaining_work >= 1 + |G_s|` | `Fuel` |
  | DATA rows (pure) | `data_nodes - active.data >= n_s` | `DataNodes` |
  | env cells (pure) | `environment_cells - active.env >= n_s` | `EnvironmentCells` |
  | binder slots (fin) | `await_binder_slots - retained.binders >= n_s` | `DataFields` |
  | Core/2 cap (fin) | `committed_before[s] + segment_cost[s+1] <= cap` | `LogicalCap` (new tag 7, /2 only) |
  | step_cap | Not a runtime counter in /1 or /2. It is an admission bound (`>= 1`) and a host trace-serialization budget; resume consumes none of it. If a future profile makes it a per-run counter, it joins this table | none |

  - Frames, helper tasks, helper work and serialization need no check. The resume
    relation pushes no frame, starts no helper and adds no helper work, and the
    checkpoint's serialized size is fixed by the admitted capacities, which the
    checkpoint already fits.
  - If any check fails, the entry publishes `Exhausted(reason)` instead of Waiting.
    The entry is charged 1, like a P6 capacity refusal. On the fin route the
    tentative segment is committed first, exactly as at a successful await.
  - Invariant SI-29: `Waiting{s}` implies every row of the table holds. So every
    Waiting checkpoint can afford one accept, and no resource can fail inside the
    resume relation.
  - Program liveness is out of scope: a guard that is false or errors for every
    observation leaves the run waiting forever, by the program's own definition.
- Waiting is checked before P2: a Waiting image is never exhausted while it waits.
- SI-03/SI-04 hold across suspension unchanged (`remaining + consumed ==
  initial_fuel`; `reference_cursor == consumed - helper_work`).
  `BResumeAccept`/`XFinResumeAccept` advance `reference_cursor` by `1 + |G_s|`,
  since they are reference events, not helper events.
- Core/2 logical work stays a separate counter in `FinImageState.committed`,
  equal to `plan.committed_before[site]` at every Waiting (UIREQ-013).

### 6.2 Pure lowering (U014, U015)

- **`Eval(await)` charges `BAwaitEnter`.**
  - It resolves the guard's free non-binder variables in the current env. These
    are its guard operands: scalar UInt/Bool values, at most 8 per site (an
    admission bound).
  - Their values are copied into the Await capture section (§3.4) in admitted
    operand order.
  - control := `Await{site, expr}`, outcome := `Waiting{site}`.
  - Frames, env root, heap tables and the remaining counters are untouched. This
    is the checkpoint.
  - For `10 + inner(3)` the frame stack, bottom to top, is `outer`'s return frame,
    the pending `+` frame holding the evaluated left operand 10, then `inner`'s
    return frame. The exact `FrameRowC` rows are pinned by the Phase 2 visible test
    from an actual run, not asserted here.
  - The current env is inner's (`n = 3`); `n` is also guard operand 0. The caller
    env is saved in the frames.
  - This is the UIREQ-014 snapshot: caller frame, control, lexical contexts,
    reachable values, budgets. On this route the financial prefix is `null`.
- **`owner_step` on a Waiting image** returns `Terminal` (no next) and changes
  nothing. `owner_run` stops there.
- **Resume is one atomic, constrained relation, `BResumeAccept` (rev 3, Sol
  finding 1).** Its inputs are the Waiting image `C` and the observation lane
  vector `obs`. It is satisfiable only when all of the following hold:
  1. `C.outcome == Waiting{s}`, `C.control == Await{s, ..}` and `CK(C) == ck_pre`.
  2. `obs` has the exact layout of site `s`: `n == n_s`, sorts by lane range (Bool
     is 0 or 1, UInt is a 128-bit range), and zero padding.
     `OD(obs) == od` is absorbed into the trace.
  3. The site's straight-line guard program evaluated over `(obs values, Await
     capture operands)` yields exactly `1`, with every arithmetic step in range.
     Overflow, a non-Bool result or `0` make the relation unsatisfiable.
  4. The successor image is exactly `C` with these changes:
     - outcome := `Running`;
     - `n_s` fresh DATA rows and env cells binding `x_i := v_i`, extending the
       current env;
     - control := `Eval{E}`;
     - `remaining_work -= 1 + |G_s|`, `consumed_work += 1 + |G_s|`,
       `reference_cursor += 1 + |G_s|`;
     - `LAST_OBS_DIGEST := od`, `RESUME_EPOCH += 1`;
     - the Await capture section is cleared to zero.

     Every other lane is equal to `C`.

  The relation is one event, and it is the only rule enabled on a Waiting image.
  So no image exists in which the observation is inserted but the guard is not yet
  accepted, and no scratch guard prefix can be settled. A batch's first event from
  a Waiting root is either `BResumeAccept` or nothing.

  Natively it is one C6 step fragment (`c6_resume_accept_step`, Phase 3) whose
  guard program is a per-site table entry in PROG. It is not a sequence of
  dispatch steps, so it does not need the Running-only dispatch owner.
- **Host `resume(C, obs)`** runs R0–R7 and then evaluates the same guard program.
  - On `1` it applies the successor exactly as above and continues `owner_run`.
    With `v = 4`, `inner` returns 4 and `outer` returns `10 + 4`: `Halted(14)`
    (U015).
  - On `0` or a guard error it returns `not_ready`. No successor image is built,
    the checkpoint bytes are untouched, and `attempted_work` reports the guard's
    static charge that was not committed.

### 6.3 Why the guard is call-free and straight-line

- Call-free, constructor-free and match-free guards over scalars compile to a fixed
  straight-line program: one operation per node, operands drawn from binders,
  captured operands and literals.
- Its cost is static (`|G_s|`), it allocates nothing, and it has no frames, env
  lookups or helpers. That is what makes the atomic relation fixed-size and the
  resource check in §6.1 exact.
- An adversary cannot make the guard loop, allocate or exhaust anything.
- Recursive or allocating checks belong in `E`, after acceptance, where they run as
  ordinary committed microsteps.

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
3. Copy the guard operands into the Await capture section (§3.4, §6.2). These are
   the guard's free non-binder names, taken from `retained_env` and from the
   committed snapshot (state, `LiabilityPresent`/`LiabilityAmount`). Nothing
   changes the committed snapshot while the run waits, so capture at entry equals
   evaluation at resume.
4. Apply the §6.1 resumability check; on failure publish `Exhausted(reason)` after
   step 1.
5. control := `Await{s, expr}`, outcome := `Waiting{s}`.

At the end of the action, `XFinComplete` = `XFinCommit` then `Halted(unit)` in one
event.

A technical cut lands only between owner events (draft_chunked.rs). So no
checkpoint can exist with a committed prefix but no Waiting control, or with a
Waiting control over an uncommitted tentative delta. That is the Core/2
`emit_checked` atomicity carried into the machine. Resume follows §6.2 with the
fin route's atomic relation `XFinResumeAccept`. Its binders go into `retained_env`
binder slots rather than DATA rows. On accept, execution continues at statement `ordinal + 1`
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
- **`retain-guaranteed-fees/1` (rev 3, Sol finding 2).**
  - **Phases.** Each segment's leading `Fee` statements form its guaranteed fee
    phase. Admission requires that every `Fee` precedes every non-`Let`/`Requires`
    effect statement in the segment. Everything after the fee phase is the fallible
    phase.
  - **Staging.** Fees never touch the committed snapshot directly.
    `FinExecState` holds three staged values:
    - `entry` = the committed snapshot at segment entry (unchanged until a commit
      event);
    - `fee_stage` = a snapshot plus ordered fee effects, starting at `entry`;
    - `tentative` = the fallible-phase snapshot plus effects.
  - **Fee phase.** Each fee applies to `fee_stage` only. When the last fee succeeds
    (the cursor reaches the first fallible statement or the segment end), the fee
    phase seals: `fee_sealed := true`, and `tentative` starts from
    `fee_stage.snapshot`.
  - **If any fee fails.** The whole fee phase is discarded (`fee_stage` and
    `tentative` are both dropped). `XFinLocalFailure` publishes `Rejected{code}`
    with `committed == entry`, so nothing is retained.
  - **Worked example.** Balance 100, fees 10 then 100: the first fee stages 90, the
    second fails, the committed balance stays 100, and `retained_by_policy` is
    empty.
  - **If the fallible phase fails after sealing.** `XFinLocalFailure` sets
    committed := `fee_stage.snapshot` and committed effects ++= `fee_stage.effects`,
    as one event. The fallible delta is discarded, and the outcome is
    `Rejected{code}`.
  - **On success** (await or complete). `XFinCommit` sets committed :=
    `tentative.snapshot`, and effects ++= fee effects then fallible effects, in
    source order.
  - **Technical cuts.** A cut may land inside either phase. It exposes `fee_stage`
    and `tentative` as staged fields, and never a partially fee-reduced committed
    snapshot. New invariant SI-30: `committed` changes only in `XFinCommit`,
    `XFinAwaitEnter`, `XFinComplete`, `XFinLocalFailure` and `XFinExhaust`;
    `fee_sealed` is false implies the fallible tentative is empty.
  - **Exhaustion (rev 4).** `XFinExhaust`, the fin-route form of P2/P3/P6
    exhaustion inside a segment, follows the same rule as `XFinLocalFailure`.
    Under `retain-guaranteed-fees/1` it commits `fee_stage` only if `fee_sealed`
    holds; otherwise it commits nothing (`committed == entry`). Under the
    default it commits nothing. The outcome is `Exhausted(reason)`.
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
| T2 | Double resume into divergent states | Locally untracked by design (as Core/2). On chain: predecessor pins (`ck_pre == root`, `h_pre == height`), anchor-computed `h_post = h_pre + n_steps` (`n_steps >= 1`), `batch == batch_no + 1`, `instance_id` and `batch` in `stmt`, `!settled.member` in accept and settle (§1.5, §5.4) |
| T3 | Forged or hidden observation | Authentication is `none` (labelled); the first on-chain settler owns the observation. `settleValidity` discloses `obs` and recomputes `OD` against the statement, so the settled observation is public and cannot differ from the proved one. The guard is the program's admission check; UInt/Bool sorts and the fixed 16-lane layout with zero padding bound what can enter |
| T4 | Griefing via junk observations | Not-ready is byte-identical and work-free in state; the guard is call-free with a static cost bound (§6.3); no fuel is added by observations |
| T5 | Profile confusion (/1 vs /2) | profile_id = 3 constrained in /2 circuits and pinned in the anchor; `keys_id` commits to a /2-only key set and the /2 decider refuses /1 leaf keys; `DOM_CK2` separates digest domains; no cross-profile resume (SI-01) |
| T12 | Unresumable Waiting root (liveness griefing by resource starvation) | Entry check publishes `Exhausted(Fuel / DataNodes / EnvironmentCells / DataFields / LogicalCap)` instead of Waiting when fuel, DATA rows, env cells, binder slots or the cap cannot cover the atomic accept; step_cap is not a runtime counter; SI-29 |
| T6 | Technical cut presented as commit or Waiting | Commit happens only inside `XFinAwaitEnter`/`XFinComplete`/`XFinLocalFailure`; a cut produces no event and leaves `BoundaryKind::TechnicalCut`; SI-25 requires `fin.tentative` to be empty whenever outcome ≠ Running |
| T7 | Cap editing between resumes | The cap is in bound META (unlike Core/2 `logical.cap`) |
| T8 | Fee-retention underflow, reordering or partial retention | `retain-guaranteed-fees/1` admission requires fees first in each segment; fees apply to `fee_stage`, never directly to the committed snapshot; local failure and exhaustion (`XFinLocalFailure`, `XFinExhaust`) retain `fee_stage` only when `fee_sealed` (SI-30) |
| T9 | Waiting image treated as terminal or exhausted | `owner_step` on Waiting returns no-next before P2; only `resume` leaves Waiting; Waiting is not Halted (UIREQ-011 distinction preserved) |
| T10 | Guard escapes its scope or settles part-way | The guard is a straight-line program inside the one atomic resume relation; there is no intermediate image, frame or env change before acceptance, so no scratch guard prefix can be settled; binders are fresh cells; no shadowing |
| T13 | Fee retention leaks a partial fee phase | Fees are staged in `fee_stage` from the segment-entry snapshot; they are retained only after the whole fee phase seals; any fee failure retains nothing (SI-30) |
| T11 | Discarded delta leaking into later state | `discarded_effects` exists only on a Rejected (terminal) image; SI-26 requires it to be empty otherwise |

New host invariants for /2:

- SI-25: outcome ≠ Running ⇒ tentative empty.
- SI-26: `discarded_effects` ≠ ∅ ⇒ outcome is Rejected.
- SI-27: Waiting{s} ⇔ control is `Await{s,..}`, and `WAIT_SITE == s`.
- SI-28: on the fin route, Waiting{s} ⇒ `committed == plan.committed_before[s]`.
- SI-29: Waiting{s} ⇒ every row of the §6.1 resumability table holds: fuel `>= 1 + |G_s|`, free DATA rows and env cells `>= n_s` (pure), free binder slots `>= n_s` and `committed + segment_cost[s+1] <= cap` (fin).
- SI-30: `committed` changes only in the four commit events; `fee_sealed == false` ⇒ fallible tentative empty.
- SI-31: Waiting{s} ⇒ the Await capture section holds exactly the site's guard operands; outcome ≠ Waiting ⇒ the section is zero.

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
| E2 | AwaitEnter (pure) | Running → Waiting(s) | `BAwaitEnter` (entry check passes) | 1 | control := Await, outcome := Waiting |
| E2x | AwaitEnter-Starved | Running → Exhausted(Fuel / DataNodes / EnvironmentCells / DataFields / LogicalCap) | `BAwaitEnter` / `XFinAwaitEnter` (entry check fails) | 1 | fin: tentative → committed first; no Waiting root is produced |
| E3 | Commit+AwaitEnter (fin) | Running → Waiting(s) | `XFinAwaitEnter` | 1 | tentative → committed; captures saved; control/outcome as E2 |
| E4 | Observe-Refused | Waiting(s) → Waiting(s) | R0–R7 fail | 0 | none (no successor exists) |
| E5 | Observe-NotReady | Waiting(s) → Waiting(s) | guard ≠ true | 0 in state (attempted reported) | none; bytes identical |
| E6 | Observe-Accept | Waiting(s) → Running | atomic `BResumeAccept` / `XFinResumeAccept` (guard evaluated inside) | `1 + |G_s|` | binders bound, obs digest and epoch recorded |
| E7 | Complete | Running → Halted | P1 publish (pure) / `XFinComplete` (fin) | 0 / 1 | fin: tentative → committed |
| E8 | LocalFailure | Running → Rejected | `XFinLocalFailure` (fin) / existing error publish (pure) | 0 publish | tentative → discarded (fees → committed under retain policy) |
| E9 | Exhaust | Running → Exhausted | existing P2/P3/P6; fin: `XFinExhaust` | as /1 | fin: tentative → discarded, committed prefix kept; under `retain-guaranteed-fees/1` `fee_stage` is committed only if `fee_sealed`, otherwise nothing (SI-30) |
| E10 | TechnicalCut | any → same | none (between events) | 0 | none; not a commit, not Waiting |
| S1 | AcceptValidity | (on chain) | D-proved `stmt` inserted in `validity_ok` | DUST fee | none to root |
| S2 | Settle | root R, height H, batch B → R', H + n_steps, B + 1 | `settleValidity`: predecessor pins, disclosed `obs` with `OD` recomputed | DUST fee | root := CK of the batch's last image (may be Waiting); the observation becomes public |

Typical U015 sequence: E1×k, E2 (root R1 = Waiting), E5 (v = 5, R1 unchanged),
E6 (v = 4), E1×m, E7 → `Halted(14)`. Typical U019 sequence: E1×a, E3 (68/10
committed), E6, E1×b, E8 → `Rejected`, with 68/10 retained.

## 12. Phase 2 plan (after review approval)

1. Core types and tags (§3.2), the C6 host codec extension (§3.4), SI-25..28, and
   the profile id 3 sites.
2. Pure route: `moriarty-sherlock-draft/2` parser/checker (await/yield, call-free
   guard), `BAwaitEnter`/`BResumeAccept`, and `resume`. Visible tests: the U014
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
6. Later, separate evidence: /2 C6 circuits for control tag 11, outcome tag 4, the atomic resume fragment `c6_resume_accept_step`, and the bound
   profile_id; the decider statement with `(CK, OD)` absorption; the
   `sherlock-run-anchor` contract; a stagenet run.

## 13. Revision 2: security review dispositions

| Finding | Severity | Disposition |
| --- | --- | --- |
| 1. The first settler picks or hides the observation | HIGH | Fixed. `settleValidity` discloses `obs` (site, label_id, n, 16 values), and the anchor recomputes `OD` and checks it against the statement (§1.5). Ownership is documented as "authentication none = the first on-chain settler owns the observation"; no observer pin in /2 (§1.5, §4, T3) |
| 2. A Waiting root can become unresumable | HIGH | Fixed. The await-entry resumability check covers fuel, DATA rows, env cells and binder slots, plus the cap on the fin route (rev 3/4 reasons: `Fuel`, `DataNodes`, `EnvironmentCells`, `DataFields`, `LogicalCap`); step_cap is not a runtime counter. On failure the entry publishes `Exhausted` instead of Waiting. SI-29 added; `guard_exhausted` removed (§6.1, §6.3, E2x, T12) |
| 3. A /1 leaf passes as /2 | MED | Fixed. `keys_id` commits to a /2-only key set; the /2 decider refuses /1 leaf keys (§1.5, T5) |
| 4. General anchor pins | LOW | Fixed. `h_post = h_pre + n_steps` with `n_steps >= 1` is computed by the anchor; `batch == batch_no + 1`; `instance_id` and `batch` are in `stmt`; `!settled.member` is restored in `acceptValidity`. The "no state recurs" argument is withdrawn as the safety basis (§1.5, §5.4, T2) |
| 5. CK domain tag; OD width | LOW | Fixed. `DOM_CK2` tag plus profile and carrier version in CK; OD is a fixed 21-lane vector with an explicit `n` and zero padding to 16 binders (§3.4) |

## 14. Revision 3: GPT Sol review dispositions

| Finding | Severity | Disposition |
| --- | --- | --- |
| 1. No proved resume-entry relation | High | Fixed. Resume is one atomic, constrained relation (`BResumeAccept` / `XFinResumeAccept`). It is satisfiable only for a Waiting image with matching CK, an exact-layout observation and a guard program evaluating to 1, and it produces an exactly specified successor. It is the only rule enabled on Waiting, so no partly resumed image or scratch guard prefix exists to settle. Natively it is one C6 fragment, not dispatch steps (§6.2, T10). `AwaitGuardF` and `BAwaitGuardReturn` are removed |
| 2. Fee retention breaks rollback | High | Fixed. A staged guaranteed fee phase starts from the segment-entry snapshot and is retained only after all fees seal; any fee failure retains nothing (balance 100, fees 10 then 100 ⇒ committed 100). SI-30 (§8 L1, T13) |
| 3. Guard resource bound wrong | Medium | Fixed by construction. The guard is a straight-line program evaluated inside the resume relation: no microsteps, frames, env lookups, helpers or DATA allocation. Its charge is exactly `1 + |G_s|`, and its only allocation is `n_s` binder cells (pure: DATA rows and env cells; fin: binder slots). The entry check covers fuel, DATA, env, binder slots and cap, each with a named Exhaust reason (§6.1, SI-29, SI-31). The rev-2 node-count bound is withdrawn |

## 15. Revision 4: Opus re-check of rev 3 dispositions

| Item | Severity | Disposition |
| --- | --- | --- |
| SI-01 accepts profile 2 or 3 on every run | CODE | Fixed in code (88d1a0c0b). The owner engine carries a selected profile, and SI-01, admission, step and run are pinned to it. RED tests in `profile2_suspension_tests` |
| N1: `if` guards are evaluated eagerly in the circuit but lazily by the host | MED | `if` is forbidden in guards; guards are call-free and branch-free (§3.3) |
| E9 exhaustion commits a partial fee phase | MED | `XFinExhaust` retains `fee_stage` only when `fee_sealed` and is added to SI-30; T8 and E9 updated (§8 L1) |
| No step_cap row; stale exhaust reasons | Partial | step_cap row added to §6.1. §13 row 2, T12 and E2x now name `Fuel / DataNodes / EnvironmentCells / DataFields / LogicalCap` |
| N2: D batch-shape constraints | LOW | Stated in §1.5: Waiting ends a batch; at most one resume, only as event 0; `od != 0` iff a resume |
| Compact `n <= 16` on Field | LOW | Cast to `Uint<8>` before the comparison (§1.5) |
| `keys_id` as a free input | Condition | D derives `keys_id` from the verified child keys; the anchor compares it to the sealed pin (§1.5) |
| Note/code mismatch on `ControlC::Await` | LOW | The note now says `Await{site, expr}`, matching the code (§3.2) |

