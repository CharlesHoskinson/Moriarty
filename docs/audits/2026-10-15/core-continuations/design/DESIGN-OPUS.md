# Opus 5.5 security vote: U3 explicit Core continuations (Sol proposal, baseline `fe01bc8b…`)

**Status.** This is a security design vote and requirements contract. Nothing was executed. The file bytes and SHAs are as supplied by root; I did not check out or hash anything. No test, model or proof results are claimed. This vote neither adopts the design nor closes any U0–U7/M1–M17/MC/SP/G exit.

## 1. Vote

**Conditional agree** with Sol option 1: explicit typed `await` in Core, plus a finite, source-derived StagePlan consumed by one evaluator and by the ZKIR lowerer.

I agree with these parts as proposed:
- the version split (`moriarty/2`, `moriarty-core/2`, logical-cost `/2`, artifact/ABI/relation `/4`);
- `/3` and `/1` stay unchanged;
- the native-call lowerer, signed paths and beta are excluded;
- a Snapshot is the single duty source;
- a false guard makes no financial change;
- suffix rejection is atomic with the prefix retained.

I **reject option 2** as a public surface. It remains acceptable only as internal backend structure. I **reject option 3** as out of scope.

The agreement is conditional on mandatory changes C1–C10 (§3). Three of them change Sol's design:
- **No wait-branch circuits.** A false guard creates no transaction.
- **Pending state lives in the contract frame.** It is not a free-floating trust object.
- **Committed logical work is a plan constant.** It is not a file-carried counter.

Sol's predicted cumulative value of 13 for start→false→true becomes committed 11 plus a separately reported attempted 2.

## 2. Security premises (normative, must appear in docs and outputs)

- **P1 — no authentication.** Unsigned `/2` stages do not authenticate event origin, correlation, finality or the resumer. Event binders are values chosen by the prover or caller, exactly like `/1` action inputs. A guard over event binders gives sequencing, not authentication. `await delivery(ready: Bool) when ready` lets anyone who can submit the resume set `ready=true`. The escrow example is therefore **not** MPLR-003/006 conditional settlement and must not be labeled as such.
- **P2 — what the relation proves.** The `/4` relation constrains:
  - the stage-out pending frame and captures to equal the values the prefix actually computed;
  - the stage-in frame to be the exact expected site, with a canonical layout.

  Linking a stage-in frame to the latest accepted stage-out frame is the existing open gate: authenticated ledger frame/predecessor binding. Under that premise:
  - replay of a consumed continuation is unsatisfiable, because the post site has changed;
  - cross-action or cross-stage attachment is unsatisfiable, because the context hash and per-action frame differ;
  - capture substitution is unsatisfiable.

  Without that premise, a verifier accepts captures and a pending site chosen by the prover. **MPLR-002, -006, -008 and -011 acceptance is not claimed.** Only the relation-side half of MPLR-002 consumption is supplied.
- **P3 — no local consumption.** The local evaluator is a pure function of untrusted files. It **cannot reject duplicate resumption of a copied continuation**, or a start from a fabricated idle scenario while another instance is pending. No authoritative consumption state exists locally.
  - Do not add a local "consumed" registry, lock file or nonce cache that is presented as consumption.
  - Outputs must state `consumption: "untracked"`.
- **P4 — no confidentiality.** Captures and event values are public positions. No confidentiality claim is made; this fails MPLR-013 qualification by declaration.
- **P5 — no authority.** There is no stage authority, signed stage policy, revocation, expiry or fee envelope. Signed, preparation, native-call and native consumers must refuse `/2`.

## 3. Mandatory changes before adoption

**C1. Pending frame is contract state.** Each action with ≥1 await owns one pending frame:

```
pending_site: UInt   // 0 = idle, k ∈ 1..=N = waiting at k-th await
pending_capture[0..L)  // L = capture layout length at last site
```

- Start requires `pre_site == 0` and all pre slots are 0.
- An advance from site k requires `pre_site == k`.
- If another await follows, the post sets `site = k+1`. Otherwise the post sets `site = 0` and all slots are 0.
- Slots beyond `len_k` must be 0 in both pre and post.

This permits **one in-flight instance per action**. That is the deliberate bounded restriction that makes MPLR-011 reserve contention unsatisfiable rather than silent.

**C2. No wait-branch circuit.** A false guard is a non-committing local observation. It produces no ZKIR, no frame write and no cumulative charge.

Rationale: an unauthenticated wait circuit lets anyone repeatedly burn a monotone budget. That could exhaust a cap and lock escrowed funds behind a duty that can never be discharged. Sol's "N wait branches" are deleted from compiler output. The local `simulate` still reports the false outcome.

**C3. Committed logical work is static.** The runtime has no short-circuit evaluation (`And`/`Or` evaluate both operands), and it caches per statement. So the cost of a successful segment is independent of input values.

- `StagePlan` stores `segment_cost[j]` and `committed_before[k]`.
- A continuation's `committed` must equal the plan constant, or the resume is rejected.
- Start with `--logical-cap C` refuses before evaluation if `plan_total(action) > C`, with attempted = 0. This prevents locking funds behind an unaffordable suffix.
- Resume cannot raise the stored cap.

Treat static cost as a premise checked by a property test (§7 H17).

**C4. Resume input is the continuation object only.**
- The `--resume FILE` argument holds exactly a `Continuation`.
- Waiting stdout embeds it under `"continuation"`.
- Do not decode or trust the segment history at resume.

**C5. Remove `predecessor_ref` and `stage_policy_ref` from the local continuation.** Unkeyed digests and empty policy references imply binding that does not exist. The predecessor is the pending frame plus the boundary state.

**C6. Reserve `await` and `when` only in `moriarty/2`.** `/1` programs using these words as identifiers must parse and run byte-identically.

**C7. Profile isolation.** All existing entry points refuse `moriarty-core/2` with the existing `mori::core::profile` error before any evaluation or layout:
- `simulate`, `simulate_metered`, `replay`, `replay_detailed_metered`;
- backend `compile`, `compile_signed`, `replay::generate`;
- static `compile_source_static_roles`.

New stage entry points refuse `/1` the same way.

**C8. Type-level artifact separation.**
- `/4` is a distinct Rust type `StagedArtifact` with no `From`/`Into` to `Artifact`.
- Proving, signed, ledger static/layout and native consumers accept only `Artifact` (`/3`).
- The `/4` decoder accepts only `authorization_mode == "unsigned"` and `semantics == "moriarty-core/2"`.

**C9. Context binds the stage.** The `/4` context preimage is domain-separated from `/3` by the relation version string. It commits to:
- `plan_hash`, action, segment index, entry site and exit site;
- capture layouts in and out, and the event layout;
- all existing financial layout terms.

**C10. Atomic segment output.** A rejected segment emits no segment, no effects and no new continuation. The input continuation remains valid for a corrected retry.

## 4. Exact interface (core)

```rust
pub const LANGUAGE_2: &str = "moriarty/2";
pub const SEMANTICS_2: &str = "moriarty-core/2";
pub const LOGICAL_COST_PROFILE_2: &str = "moriarty-reference-logical-cost/2";
pub const CONTINUATION_FORMAT: &str = "moriarty-continuation/1";
pub const STAGE_EVENT_FORMAT: &str = "moriarty-stage-event/1";
pub const MAX_AWAITS_PER_ACTION: usize = 16;
pub const MAX_EVENT_BINDERS: usize = 16;
pub const MAX_CAPTURE_SLOTS: usize = 256;

// New variant; /1 serialization of programs not using it is unchanged.
Statement::Await { label: String, binders: Vec<InputDecl>, guard: ExprId }

pub struct CaptureSlot { name: String, ty: Type, origin: Origin /*Input|Let|Event*/, ordinal: u16 }
pub struct AwaitSite { site: u16 /*1-based*/, ordinal: u16, label: String,
                       binders: Vec<InputDecl>, captures: Vec<CaptureSlot> /*prefix-monotone*/ }
pub struct ActionPlan { action: String, sites: Vec<AwaitSite>,
                        segment_cost: Vec<UInt> /*N+1*/, committed_before: Vec<UInt> /*index k = Σ cost[0..k)*/ }
pub struct StagePlan { program_hash: [u8;32], actions: Vec<ActionPlan> }  // plan_hash = digest(StagePlan)

#[serde(deny_unknown_fields)]
pub struct Continuation { format: String, program_hash: String /*64 lc hex*/, plan_hash: String,
    action: String, site: u16, captures: Vec<NamedValue>,
    logical: StageLogical, boundary: Snapshot }
#[serde(deny_unknown_fields)] pub struct NamedValue { name: String, value: Value }
#[serde(deny_unknown_fields)] pub struct StageLogical { profile: String,
    #[serde(deserialize_with="Option::deserialize")] cap: Option<UInt>, committed: UInt }
#[serde(deny_unknown_fields)]
pub struct StageEvent { format: String, action: String, label: String, values: Vec<NamedValue> }

pub enum StageOutcome {
  Waiting { advanced: bool, segment: Option<Execution>, continuation: Continuation,
            attempted: UInt, residual: Vec<Residual> },
  Complete { segment: Execution, committed: UInt, attempted: UInt, residual: Vec<Residual> } }
pub struct StageRejection { error: Error, attempted: UInt, committed: UInt }
```

- `program_hash` is `digest(Program)` computed with `mori_core::intent::canonical`, the same digest the backend uses.

**Static validation (`/2`).** Use existing error codes where they fit.
- `Await` appears only as a top-level statement.
- `label` is an identifier that is unique in its action → `duplicate`.
- Binders are identifiers; they must not shadow states, assets, inputs, lets or earlier binders → `duplicate`.
- Binders count ≤ 16 and awaits ≤ 16 per action → `bound`.
- Captures per site ≤ 256 → `bound`.
- `guard` has type Bool → `type`. The guard is typed in the environment of prefix bindings plus its own binders.
- `await` and `when` are reserved.

**Captures at site k.** These are the action inputs in declaration order, then lets and earlier event binders in statement order, all before ordinal(k). State names are **never** captured. A suffix read of state or liabilities reads the **current** boundary. Authors who need the suspended value must write `let x0 = x;`.

**Resume predicates.** Evaluate in this order; the first failure rejects with attempted = 0:
- R1: `format == CONTINUATION_FORMAT`; `logical.profile == LOGICAL_COST_PROFILE_2`. Otherwise `stage_format`.
- R2: The program file is `/2`, else `profile`. `program_hash` and `plan_hash` equal the recomputed values, else `stage_program`.
- R3: `action` exists, has ≥1 site, and `site ∈ 1..=N`. Otherwise `stage_site`.
- R4: `captures` equals `sites[site-1].captures` exactly in length, order, name and type. Otherwise `stage_capture`.
- R5: Event format, action and label equal the pending site, and the values equal the binder list in order, name and type. Otherwise `stage_event`.
- R6: `committed == committed_before[site]`. If CLI `--logical-cap` is given, it must be ≤ the stored cap. `committed + segment_cost[site] ≤ cap`. Otherwise `stage_logical`.
- R7: The boundary passes every `scenario_snapshot` check, with the inputs check replaced by R4. It must also be canonical: balances sorted by (asset, account) and liabilities sorted by name. Unsorted input is rejected, not re-sorted. The trace-cell formula applies using the whole action's statement count.

Then the evaluator charges dispatch, evaluates the guard and continues **in the same ordered statement evaluator**, seeded at ordinal+1. There is no second interpreter.

**CLI.** `simulate FILE --resume C --event E`.
- `--resume` requires `--event`.
- It conflicts with `--trace`, `--target`, authorization arguments and a scenario. Conflicts are usage errors raised before any file read.
- If `ActionInput` forces an action positional, it must equal `C.action`; otherwise `stage_site`.
- The continuation file is limited to ≤ 4 MiB and the event file to ≤ 64 KiB. Size is checked before parsing.
- Both files must be decoded through the strict parser that rejects duplicate keys (`mori_core::json::parse`).

**Output labels.** Every `/2` stdout object contains:
- `"qualification":"local-reference-stage"`;
- `"authentication":"none"`;
- `"consumption":"untracked"`.

Complete never says settled. `residual` lists every liability in the post snapshot whose amount is greater than 0. Exit codes:
- waiting or complete → 0;
- rejected → 1;
- a false guard → 0 with `advanced:false`, no segment, and a `continuation` equal to the canonical re-serialization of the input.

**Segment shape.**
- The start Execution has steps for the prefix only.
- The advance Execution's first step has `index = await ordinal`, `statement = Await`, `value = Bool(true)`, `snapshot = pre` and `effects = []`.
- All indices are original ordinals.
- `Error.step` is the original ordinal.

## 5. `/4` circuit and ABI security interface

- The lowering produces N+1 circuits per action with N awaits. Each circuit records `segment j`, `entry_site` (0 for start, otherwise j) and `exit_site` (j+1, or 0 for the last segment).
- Filenames are `{index:03}-{action}.s{j}.zkir`. The manifest is published last, using the existing `publish_artifact` refusal discipline.
- **Roles** are added after the existing pre-liability positions:
  - `pre_pending_site`, constrained equal to the constant `entry_site`, with `constant` set to Some;
  - `pre_pending_capture.{i}` for i < L;
  - `event.{name}` per binder.

  After the post-liability positions:
  - `post_pending_site`, constrained equal to the constant `exit_site`;
  - `post_pending_capture.{i}`.
- Every post pending role is created with `expected = Some(wire)`. Slots i ≥ `len_exit` use `expected = Some(immediate 0)`. Pre slots i ≥ `len_entry` are constrained to 0.
- Range checks:
  - UInt slots and events: `ConstrainBits 128`;
  - Bool slots and events: `ConstrainToBoolean`.
- An advance asserts the guard with `Assert`. The suffix environment binds:
  - capture names to pre slot wires;
  - binders to event wires;
  - states to `pre_state`.

  It has **no** `input` role positions.
- The start circuit's capture outputs are constrained to the `input` and let wires.
- No circuit computes a pending, success or capture value on the host without constraining it.
- Gates: the existing six gates, plus these, all `"unperformed"`:
  - `pending-frame-predecessor-binding`;
  - `stage-event-authentication`;
  - `stage-authority-and-consumption`;
  - `arbitrary-witness-stage-soundness`.
- Resources: per-circuit `segment_logical` and `committed_before`, plus action `plan_total`. These are labeled **not circuit-enforced and not DUST**.
- Existing instruction and byte caps apply to the sum over all stage circuits. The liability registry ceiling remains 16.
- **`/4` decoder** rejects:
  - `/3` strings or a signed mode;
  - per action, segments that are not exactly contiguous 0..=N;
  - inconsistent entry/exit sites;
  - out-of-range or duplicate pending positions;
  - capture role counts that differ from L.

  The `/3` decoder must reject `/4` bytes, which `decode_version` already does.

## 6. Refusals and compatibility

- **`/1` compile → `/3`.** Output bytes (`artifact.json` and each `.zkir`) must be byte-identical to baseline output for the existing corpus. Do not update existing `insta` snapshots or goldens. A diff is a failure, not a regeneration.
- **`/1` JSON and errors.** Execution JSON, logical-cost/1 values and existing error code/message/offset/step are unchanged. `/1` IR JSON containing `"kind":"Await"` must still be rejected before evaluation. The code may move from decode failure to `mori::core::profile`; record that as an observed compatibility delta.
- **Signed targets.** A signed Target with a `/2` program → `mori::backend::unsupported` before `create_dir`. The out directory must not exist afterwards.
- **Other `/2` refusals before emission:**
  - `CompileCall` (`run_static`);
  - `--trace` replay generation;
  - signed relation `admit`;
  - preparation and native consumers.

## 7. Expected values (predictions, not observations)

**E1 escrow** (`deliver`, amount = 10). Statement ordinals: 0 transfer, 1 liability, 2 await, 3 transfer, 4 discharge. Initial state: alice 100, escrow 0, bob 0; no `release` liability.

| Invocation | Result | Effects and state | attempted | committed |
|---|---|---|---|---|
| Start | Waiting, site 1, captures `[amount: UInt 10]` | steps [0,1]; Transfer 10 alice→escrow; Liability release 0→10; balances 90/10/0; residual `[release 10]` | 5 | 5 |
| Resume `ready=false` | `advanced:false`, continuation identical | — | 2 | 5 |
| Resume `ready=true` | Complete | steps [2,3,4]; Transfer 10 escrow→bob; Liability 10→0; balances 90/0/10; release present-zero; residual `[]` | 6 | 11 |

- `plan_total` is 11. Start with cap 10 → `stage_logical`, attempted 0, no output state. Cap 11 completes.
- The `/4` lowering yields exactly 2 circuits for `deliver`: `s0` (0→1) and `s1` (1→0). L = 1.

**E2 capture vs current state.**

```mori
state level: UInt = 0;
action a(x: UInt) {
  let y = x + 1;
  next level = y;
  await go(z: UInt) when z == y;
  next level = level + z;
}
```

| Invocation | Result | attempted | committed |
|---|---|---|---|
| Start, x = 4 | level 0→5; captures `[x=4, y=5]` | 7 | 7 |
| Boundary level edited to 100, then resume z = 5 | Complete, level 100→105 | 8 | 15 |
| Resume z = 6 | `advanced:false` | 4 | 7 |

Sol's reserve-allocation example remains required; Sol freezes its literals independently.

## 8. Hostile tests (must exist; expected outcome)

Unless noted, every rejection prints `status:"rejected"`, exits 1, and produces no segment and no new continuation.

| # | Mutation (applied to the E1 start continuation unless stated) | Expected |
|---|---|---|
| H1 | Program source edited by one literal | `stage_program` |
| H2 | `plan_hash` altered | `stage_program` |
| H3 | `site` 0; `site` 2 | `stage_site` |
| H4 | Capture renamed, reordered, retyped to Bool, extra slot, missing slot | `stage_capture` |
| H5 | Event wrong label, wrong action, missing/extra binder, `UInt` for `Bool` | `stage_event` |
| H6 | Duplicate JSON key in event (`ready` true then false) or continuation | Strict-parse rejection |
| H7 | Non-canonical UInt `"010"` anywhere | Decode rejection |
| H8 | `committed` 4 or 6; resume `--logical-cap` 12 with stored cap 11; cap that makes `committed + segment > cap` | `stage_logical` |
| H9 | Boundary balances unsorted or duplicated | `balance` |
| H10 | Boundary `release` debtor changed | `liability`: "Existing liability identity differs" |
| H11 | Boundary escrow 0, resume true | `balance`, step 3, attempted 4; same input later resumes successfully once escrow is 10 |
| H12 | Boundary release removed, resume true | `liability`, step 4, attempted 6; no Transfer effect emitted |
| H13 | Same valid continuation resumed twice | **Both succeed with byte-identical output.** This is a residual-pinning test asserting `consumption:"untracked"`; it is not a security pass |
| H14 | Capture `y` tampered to 6 (E2), resume z = 6 | Accepted locally, level 100→106. Pins P3/P4: captures are unauthenticated locally |
| H15 | `/1` program with `let await = 1;` | Unchanged parse and Execution bytes |
| H16 | `/2` program to `/1` `simulate` / `compile` / `--trace`; `/1` program to `--resume` | `profile`; `--trace` prints `rejected` |
| H17 | Property test: for random inputs over E1/E2/reserve, successful segment attempted == `segment_cost` | Holds; otherwise C3 is void and the design returns for re-vote |
| H18 | Signed Target with `/2` | `unsupported`, no out directory |
| H19 | `CompileCall` with `/2` | Refused before emission |
| H20 | `/3` decoder on `/4` manifest and vice versa | Rejected |
| H21 | `/4` structural checks | Every `post_pending_*` and `post_pending_site` binding was emitted with an expected wire or constant (test-only builder introspection); every pre slot beyond `len_entry` and post slot beyond `len_exit` is zero-constrained; context hashes of `s0` and `s1` differ, and both differ from any `/3` hash of the same program |
| H22 | `/2` validation | Duplicate label, binder shadowing a state/let/input, 17 awaits, 17 binders, Bool-typed guard misuse, `await` used as an identifier → specified codes |

H21 is structural only. Relation satisfiability and arbitrary-witness soundness require SDK check/model execution, which is excluded under native = 0. Those remain unperformed (MPLR-014).

## 9. Unknown call-path premises (implementer must inspect and report)

1. Whether `core_io::read_json` and `core_io::program` reject duplicate keys and bound input size. If not, `/2` must use `mori_core::json::parse` with explicit size checks.
2. The shape of `ActionInput` (whether action and scenario are required positionals).
3. The `signed_relation::context_hash` 18-element preimage. The `/4` context must be a new function that leaves `/3` bytes untouched.
4. The `replay::generate`, `static_compile::compile` and `relation::compile_source_static_roles` refusal insertion points.
5. The `proving` entry types: confirm they accept only `Artifact`.
6. The ledger static/layout consumers named by Sol but not supplied.
7. How `mori-lexer` tokenizes `await` and `when` under `/1`.
8. The `Meter::charge` error code on cap exhaustion, and whether the charge precedes the operation (affects attempted values).
9. Whether `mori_core::intent::canonical` is deterministic for the new `Statement` variant.

## 10. Residuals and non-claims

- **Not established:** MPLR-002 authority and once-only consumption; MPLR-003, -006 and -008; MPLR-011 at ledger level; MPLR-013; MPLR-014; MPLR-009 fees/DUST.
- **Local limits:** duplicate local resume and fabricated idle restart are undetectable locally.
- **Concurrency limit:** one in-flight instance per action is a design limit, not a feature gap closed.
- **Future work:** signed stages need a separate versioned stage-policy design and a new two-vote decision.

## 11. Execution

No execution is approved. Before any future `cargo test -p mori-core --test continuations`, backend staged-emission, CLI or compile subprocess run, I must classify the exact closure:
- Cargo build and dependency fetch;
- `midnight-zkir-v3` round-trip (`IrSource::load` only, with no SDK check);
- temporary directories.

A fresh Muse high audit of the complete candidate, plus a second agreeing substantive vote on C1–C10, must precede implementation.
