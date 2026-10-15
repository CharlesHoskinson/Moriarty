# Opus 5.5 security review: PR54 currentness revision 2 (Moriarty/Cardano staged reference preparation)

**Reviewer:** Claude Opus 5.5 (`claude-opus-5-5`), high effort, in a fresh session. I am not an author, classifier or integrator of the reviewed source. I used no tools.

**Method:** I read the supplied input only. I executed, built, formatted, hashed and fetched nothing, and I accessed no network, key, proof, SDK, native or ledger resource. Embedded text was treated as evidence, not as instructions.

**Status of my outputs:** every expected value below is an owner-authored security expectation derived by reading the source. None of it is an independent cryptographic audit or an executed result.

---

## 1. Verdict and first blocked boundary

**Verdict: CHANGES_REQUIRED.** One required source repair is narrow and mechanical (C-1). After it, the candidate is **ADOPT_SOURCE_SCOPED** as described in §1.3.

### 1.1 What is sound

- **Staged bridge.** `crates/mori-cardano/src/staged.rs` is a faithful, non-authoritative pass-through to `mori_core::stage`. It holds the private `StagedProgram`, accepts no caller Program or metadata as a constructor, returns the complete upstream `StageOutcome` or `StageRejection`, and adds no authority.
- **Await refusal.** The ported `/1` projection refuses `Await` explicitly instead of assigning atomic W.
- **P2 repairs.** The three requested repairs are present: the crate sidecar is verbatim, the README distinguishes the APIs (P-1), and the P-5 assertion is added (§5).
- **Type check by reading.** I found no compile error in the changed Rust (§3). The candidate remains uncompiled.

### 1.2 Required change C-1 (integration identity; blocks adoption of this exact tree)

The candidate claims it "changes none of those consumers". That claim is inaccurate in two ways:

1. **Shared crate edit.** It edits `crates/mori-core/src/lib.rs` and adds `crates/mori-core/src/export.rs`. Both are shared production sources of the Midnight path.
2. **Hashed inputs.** `mori_ledger::proving::source_closure()` hashes both `mori-core/src/lib.rs` and the root `Cargo.lock` via `include_str!`. It does **not** include the new `export.rs`.
   - This closure feeds `relation::Manifest.compiler_hash` for mapped-role exports, the `static_compile` marker `build` field (checked on `reopen`), and `PublicDerivedCallKey` admission.
   - PR54 added `crates/mori-ledger/tests/source_closure_coverage.rs`. From its name and the explicit enumeration of `mori-core/src/stage/*` in `source_closure`, I infer that a new uncovered mori-core source file probably violates it. I have not seen that test, so this is inference.

### 1.3 Adopted repair (pre-authorized; no fresh Opus review is needed if the delta is exactly this)

- Move `export_source` and `CoreSourceExportV1` unchanged into `crates/mori-cardano/src/export.rs`. It uses only public `mori_core::parse`, `intent::canonical` and `intent::hash_bytes`.
- Restore `crates/mori-core/src/lib.rs` to base bytes `9d920393…` and delete `crates/mori-core/src/export.rs`.
- Change only the `use` paths in `mori-cardano` (`lib.rs`, `types.rs`, `projection.rs`). The serialized field names and order of `CoreSourceExportV1` are unchanged, so the frozen metadata/1 goldens are unaffected.
- Keep the root `Cargo.lock` `mori-cardano` entry. It is unavoidable, because `crates/*` membership makes the crate a member.
- **Disclose** that the lock change alters the `source_closure()` value and therefore the declared native build-identity strings. This is expected for any lock change, but it must be stated.
- **PRJ-01 wording:** the P2 EARS text "through `mori_core::export_source`" should read "through the source-owned export (current `mori_core::parse` + canonical Program hash)". This is a non-semantic wording amendment I make as its author.
- Any other delta needs a fresh review.

### 1.4 First blocked boundaries, in order

| # | Boundary | Blocked on |
|---|---|---|
| 1 | Adoption of this tree | **C-1** |
| 2 | Current-tree evidence | The scoped pure check (§6) on the C-1 tree, which is authorized |
| 3 | Stage→UTXO body codec implementation | A second vote on the body-shape decisions D-1, D-4, D-5 and D-10 (§12) |
| 4 | Projection extension or any refusal-dependent consumer | The frozen, green NT-1–24 and NS-1–17 suites (§8) |
| 5 | Target or ledger | Unchanged R5/P2 gates, plus §9 Q6 |

Merge readiness additionally needs a separately scoped re-execution of the ledger source-closure selection (§4, C-1 item 3). It is **not** authorized here, because it compiles `mori-ledger` with its SDK closure.

---

## 2. Scope, method and limits

### 2.1 Read in full

- **Packet documents:** CURRENTNESS, PLAN-AND-OPUS-QUESTIONS, REVISION-1/2 JSON, P2-PRESERVATION, IMPORT-ORIGINS, SOURCE-MANIFEST, DEPENDENCY-SOURCE-CLOSURE, READ-ONLY-COMMANDS, the attributed P2 Opus report and sidecar, the upstream closure JSON and both closure reviews, and the root pin check.
- **Changed and imported source:**
  - `mori-cardano`: every `src` file, README, both sidecars, the consumer (`lib.rs`, `p2.rs`, manifest, lock) and all fixtures;
  - `mori-core`: `export`, `ir`, `json`, `lib`, `logical_cost`, `parser`, `runtime`, `intent`, `policy`, `replay`, `source_sponsor`, and `stage/{mod, admit, cost, decode, digest, plan, profile, validate}`;
  - `mori-lexer`, `mori-diagnostics`, `mori-span`;
  - consumer context: backend `lib`, `proving`, `signed_relation`, `stage/*`, and ledger `layout`, `ops`, `proving`, `relation/*`, `signed/mod`, `static_compile`;
  - the root manifest, root lock and toolchain; `AGENTS.md`, `ROADMAP.md`, `CORE-CONTINUATIONS.md` and the product contract.

### 2.2 Not seen

- **Example files:** `examples/core/continuations/{reserve.mori, delivery-scenario.json, reserve-scenario.json, *-resume.json}`. I use the delivery text, which the guide reproduces and which matches `frame.rs::ESCROW`. For reserve I use `frame.rs::RESERVE` as a proxy, and the lane must confirm it.
- **Ledger test:** `source_closure_coverage.rs`.
- **Other:** the R5 normative text (only its effects through P2 are visible), the CLI JSON report schema, and the live tree.

### 2.3 Not recomputed

No hash was recomputed. I rely on the supplied root rehash (95 rows unchanged at 07:48:26Z) as a root observation.

---

## 3. Source assessment

| Topic | Finding |
|---|---|
| **Staged authority** | `prepare_staged_reference` calls `stage::load` (`parse_v2` → `require_core2` → `validate_v2` → plan → invariants → cost → digests). `StagedReferenceContext` has a private field and no `Clone`, `Default`, `Deserialize` or other constructor. `StagedSourceMetadataV1` is Serialize-only and no API accepts it. The wrapper adds no authority beyond public `stage::load`, which anyone can call. **Sound.** |
| **`/1` versus `/2` separation** | `/1` source into the staged API: `parse_v2` refuses with `mori::core::profile` (NS-1). `/2` source into `prepare_projection` or `inspect_cardano`: `parse` refuses with `mori::core::profile` (NS-2). In `/1`, `await` in statement position is `unsupported` and `await`/`when` remain ordinary identifiers. **Sound.** |
| **Await in atomic projection** | `statement_root` now returns a `Result` and `Await` returns `CoreInvariant("Atomic /1 logical work cannot represent Await…")`. The arm is unreachable from the public API, because `export_source` reparses `/1` and `validate` refuses Await. No atomic W is assigned to a staged action. **Sound** (advisory A-1 on the variant). |
| **`/1` behaviour preservation** | `logical_work` arithmetic is unchanged. The shared `Statement` serializer adds an `Await` variant, but tag-based serialization is order-independent, so `/1` Program bytes and `program_hash` are unchanged in principle. The consumer test pins both hashes, which makes this a concrete currentness check. The runtime refactor keeps charge order (dispatch, then attempted nodes, with a fresh cache per statement). By reading, NT-17 to NT-21 values are unchanged. |
| **Rust well-formedness (by reading)** | All imports in `staged.rs`, `projection.rs` and `p2.rs` resolve to public items. Signatures match `stage::start` and `stage::resume`. `&[u8;32]` coerces to `&[u8]`. Diverging `return` inside `Ok(match …)` type-checks. No issue found; compilation remains unverified. |
| **Outcome retention** | `Waiting{advanced, segment, continuation, attempted, residual}`, `Complete{segment, committed, attempted, residual}` and `StageRejection{error, attempted, committed}` are returned whole. Three properties matter for the codec: <ul><li>present-zero duties survive in `execution.post` and `continuation.boundary`, while `residual` filters `amount > 0`;</li><li>`continuation.boundary == segment.execution.post`, by construction in `admit.rs`;</li><li>a false guard returns the input continuation unchanged.</li></ul> **Sound.** |
| **Labels** | The labels appear in `metadata()` only. `StageOutcome` and `StageRejection` carry no labels, and their derived `Serialize` would emit unlabelled JSON (A-2). Labels are copied by **position** from `STAGE_LABELS` (A-3). |
| **Upstream properties the codec must not upgrade** | <ul><li>`StageRejection.committed` **echoes the unverified input** when R0–R6 refuse (comment in `admit.rs`).</li><li>The stored cap is outside both digests and is advisory.</li><li>R7 `canonical_boundary` ignores captures (U-1), so captures and boundary are jointly caller-forgeable locally.</li><li>Event values are caller-chosen.</li><li>Continuations are re-resumable.</li></ul> All are labelled upstream, and none is a bridge defect. They drive §9 and §10. |
| **Dependencies** | Selected local libraries have no `build.rs` and no fs, process, network or `unsafe`. The `mori-core` and lexer-family manifests are unchanged from the historic P2 closure. The consumer lock lists exactly 63 packages, consistent with the inherited closure. Registry crates in that closure have build scripts and proc-macros (`serde_derive`, `thiserror-impl`, `soa-rs-derive`, and build scripts in `libc`, `serde*`, `proc-macro2`, `typenum`, `generic-array`, `serde_json`, `thiserror`). They execute during a build, and that is an effect (§6). |

---

## 4. Findings

### C-1. Shared-crate edit and undisclosed identity effect (required)

- **Severity:** integration and identity. No exploit.
- **Problem:** see §1.2 for the full statement.
- **Repair:** as specified in §1.3.
- **Merge note:** after repair, merge still requires the ledger closure selection to be re-run under its own scope.

### A-1. Await classification (advisory)

`CoreInvariant` now carries a third unreachable reason. That is acceptable, since Await after `export_source` is an invariant breach. Callers must treat every `CoreInvariant` as an implementation defect and never as a domain refusal. Pin the text only through an internal construction test, without production hooks.

### A-2. Unlabelled serialization (required before the codec; advisory now)

Any record that serializes a wrapper outcome, rejection, continuation or segment must embed the three label pairs verbatim (CUR-05, STG-01).

### A-3. Positional label copy (advisory)

`metadata()` takes `STAGE_LABELS[i].1` by index. Destructure the pairs by key, or assert the keys, so that a reorder upstream cannot mislabel the output.

### A-4. Unseen reserve source (advisory)

The guide's "completion retains an allocation duty of 5" is not reproduced by `frame.rs::RESERVE` with a duty-free start. That proxy discharges `held` fully, which leaves a present-zero duty. The likely explanation is a pre-existing duty in `reserve-scenario.json`. Expectations must be derived from the pinned files `260f5044…` and `f20a6342…`, not from the guide's prose.

### H-1 (F-11). Delivery await is not a condition against adversaries (disclosure)

This is `/2` Program behaviour, not an adapter defect.

- `deliver` debits KeyHeld `alice` only in segment 0. Segment 1 debits only script-custodied `escrow` and needs no signer.
- `ready` is a caller-supplied binder, and authentication is `none`.
- Any holder of the continuation can resume with `ready=true` immediately after start, releasing the escrowed amount to `bob`.

### H-2 (F-12). Reserve stuck thread from a late-checked event value (disclosure)

- `quote` is constrained only by `quote >= held` at site 1. `requires quote <= 50` runs in segment 2.
- Any party supplying `quote = 51` makes segment 1 commit: `basis = 35+51 = 86`, the allocation is transferred and discharged, and the frame waits at site 2.
- Every later resume then refuses with `mori::core::requires` at step 9. The thread can never complete, and the `extra` fee is never charged.
- Under the single-frame rule (D-1), `allocate` can never start again on that thread.

### Proposed staged sidecar text (first vote)

These two paragraphs would accompany any citation of `/2` reference results for these specimens:

> **Delivery (F-11).** In `delivery.mori`, segment 1 of `deliver` debits only script-custodied `escrow` and requires no signer, and its `ready` binder is a caller-supplied value with authentication `none`. Any party holding or copying the continuation may resume with `ready = true` immediately after start and move the escrowed amount to `bob`. The await is not a delivery condition against an adversary. Nothing in these results is evidence of conditional settlement.
>
> **Reserve (F-12).** In the reserve specimen, `quote` is checked only against `held` at the first await, and `quote <= 50` is checked after the second await. A caller supplying a larger `quote` commits the second segment (state, transfer and discharge) and leaves the action permanently unable to complete. Local reference results neither detect nor prevent this.

---

## 5. Verification of the three requested P2 repairs

| Repair | Status |
|---|---|
| **§6 sidecar** | The crate file `PROJECTION-SCOPE-AND-F10-DISCLOSURE.md` matches the four §6 paragraphs verbatim by reading. The `/1` sidecar is untouched (`1012d0b7…`). **Satisfied.** |
| **P-1 README** | Both sidecars are linked, metadata/1 and projection `/2` are distinguished, and "These API version numbers do not denote `moriarty/2` source support" is stated. The text says the historical results have not been re-executed on PR54. P-6 wording is mostly applied; "refusals use less" is absent, which is acceptable as P-6 was advisory. **Satisfied.** |
| **P-5** | `assert_eq!(serde_json::to_value(&inputs).unwrap(), expected["typed_inputs"])` is present. Re-serialization emits canonical `{"integer":"n"}` and `{"constructor":u64,"fields":…}`, so `+5`/`007` now fail. Unexecuted. **Satisfied.** |

P2 §7 conditions 1, 3, 5 and 6 remain met by source. Condition 4 (the NT gate) remains open.

---

## 6. Pure-check disposition

**Authorized on the C-1-repaired tree only.** This is ordinary current-tree evidence. It is not acceptance, not cryptographic, and not target evidence.

### 6.1 Preconditions (read-only)

1. `git rev-parse HEAD HEAD^{tree}` equals `9c63048a…` and `40be71ec…`.
2. `git status --porcelain=v1`: the only changes are the C-1 delta plus the candidate.
3. Rehash the root 95-row manifest, adjusted for exactly the C-1 rows. In particular record:
   - root `Cargo.toml` `dea75cec…` and root `Cargo.lock` `c31f8bac…`;
   - consumer `Cargo.toml` `3484aa84…` and consumer `Cargo.lock` `77021b30…`.
4. `cargo +1.99.0 tree --offline --locked --manifest-path crates/mori-cardano/tests/metadata-consumer/Cargo.toml -e normal,build --prefix none`. The package set must be a subset of the 63 locked packages, with no git source and no `midnight-*`, `proptest` or SDK crate.

### 6.2 Run A (currentness re-execution)

```
env CARGO_NET_OFFLINE=true cargo +1.99.0 test --offline --locked \
  --manifest-path crates/mori-cardano/tests/metadata-consumer/Cargo.toml \
  --target-dir /tmp/mori-cardano-pr54-currentness-A --lib --no-fail-fast
```

- **Expectation:** 9 tests (3 metadata, 6 P2), compiled together with `staged.rs`. Expectations were frozen historically. The pinned `/1` `program_hash` values are part of the currentness check.
- **Optional read-only format check:** `rustfmt +1.99.0 --edition 2024 --check` on the changed `.rs` files. No mutating formatter.

### 6.3 Run B (after an independent non-author freeze)

- **Content:** test-only additions in `metadata-consumer/src/*` for SP-1–SP-7, NS-1–NS-17 and NT-1–NT-24.
- **Library:** no library byte may change between Runs A and B.
- **Command:** identical to Run A, with a fresh `--target-dir …-B`.
- **Chronology:** a first green on pre-existing code is acceptable, because no implementation is under test. The freeze must precede assertion authoring, and this must be disclosed.

### 6.4 Effect constraints (both runs)

- **Network and locks:** no network, no `cargo update`, `generate-lockfile` or `fix`, and no root-workspace build.
- **Files:** no write to any lock or manifest, and no SDK, Aiken, native, key or ledger action.
- **Build scripts:** registry build scripts and proc-macros execute from the local cache, at exactly the locked versions run in the historic P2 lane.
- **Post-run checks:** re-hash every precondition file and require byte equality. `git status` must be unchanged apart from test-only files in Run B.

### 6.5 Stop conditions

- Any lock or manifest change, any network attempt, or any package outside the lock: stop and report.
- Compile failure or assertion failure: report the full output verbatim. Do not fit expectations.
- A library change needed to pass: new review.

---

## 7. Required independent positive staged cases

These must be frozen before any projection extension. Each case retains the complete Program, plan, frame, inputs or event, boundary, effects and work, and the labels.

Delivery numbers assume `delivery-scenario.json` gives `amount=10`, `alice 100`, `escrow 0`, `bob 0`, `marker 77` and no duties, as the guide states. The lane must derive and confirm this from the pinned bytes.

### SP-1. Delivery start, `cap = Some(11)`

| Field | Expected |
|---|---|
| Outcome | `Waiting{advanced:true, attempted:5}` |
| Frames | `frame_pre {0,[]}`, `frame_post {1,[amount:UInt 10]}` |
| `execution.semantics` | `"moriarty-core/2"` |
| Steps | Index 0: Transfer `alice→escrow` 10. Index 1: Liability `release` 0→10. No step for the exit await. |
| Post state | alice 90, escrow 10, bob 0; `release` 10 |
| Continuation | `moriarty-continuation/1`; the context's `program_hash` and `plan_hash`; site 1; `logical {profile:"moriarty-reference-logical-cost/2", cap:"11", committed:"5"}`; boundary equals post |
| Residual | `[release 10]` |

### SP-2. False resume of the SP-1 continuation

- `Waiting{advanced:false, segment:null}`.
- The output continuation is byte-equal to its input.
- `attempted:2`, `residual [release 10]`.

### SP-3. True resume

- `Complete{committed:11, attempted:6, residual:[]}`.
- Frames: `frame_pre {1,[amount 10]}`, `frame_post {0,[]}`.
- Steps:
  - index 2: `Await`, value `Bool true`, snapshot equal to pre, no effects;
  - index 3: Transfer `escrow→bob` 10;
  - index 4: Liability `release` 10→0.
- Post: `release` is **present with amount 0** and is absent from `residual`.

### SP-4. True resume after a false resume

Resuming SP-2's output continuation with a true event gives exactly SP-3.

### SP-5. Reserve, all five steps

Confirm that `reserve.mori` equals `frame.rs::RESERVE`.

**Captures:**
- site 1: `[extra:UInt (Input,0), held:UInt (Let,0)]`;
- site 2: `[extra, held, quote:UInt (Event,3)]`.

**Work per step:**

| Step | Attempted | Committed |
|---|---|---|
| Start | 11 | 11 |
| First false (`quote < held`) | 4 | 11 |
| First true | 15 | 26 |
| Second false | 2 | 26 |
| Second true | 10 | 36 |

**Final segment:**
- Steps 8 (Await), 9, 10, 11.
- Effects: `Fee reserve→treasury extra`, then `Emit{name:"settled_quote", value:UInt quote}`.
- Boundaries from the pinned scenario, with present-zero duties retained.

### SP-6. Zero-await `/2` action

Use the `frame.rs::MIXED` text, newly authored as a fixture. `b` starts and completes directly:
- frames idle to idle;
- committed equals `plan_total` = 4, attempted 4.

### SP-7. `/1` currentness

- The six P2 cases and three metadata tests (Run A).
- The H15 source `let await = 1; let when = await; next count = when;` passes `prepare_projection` and `inspect_cardano`. Its Program equals the upstream `compat_v1_identifiers/program.json`.

---

## 8. Owner-authored negatives and suite status

### 8.1 NT-1–NT-24

These are specified in P2 §8 and still unexecuted. They are unchanged by PR54 by my reading. NT-17 to NT-21 metering is unchanged. They gate any consumer of projection refusals (TST-01).

### 8.2 New bridge negatives, reference-only (NS)

Requirements:
- All errors compare exactly.
- `C1` denotes the SP-1 continuation.
- Each case also asserts that no segment and no replacement continuation is published.

| Test | Input | Expected |
|---|---|---|
| NS-1 | `prepare_staged_reference(installment-invoice.mori)` | `Err{code:"mori::core::profile", message:"Expected language \"moriarty/2\"; found \"moriarty/1\".", offset:Some(21), step:None}` |
| NS-2 | `prepare_projection(delivery.mori, bindings)` | `Inspection(Core{code:"mori::core::profile", message:"Expected language \"moriarty/1\"; found \"moriarty/2\".", offset:Some(21)})`. `inspect_cardano` gives the same `Core` error. |
| NS-3 | `/1` invoice with an inserted `await go(ready: Bool) when ready;` | code `mori::core::unsupported`, message `Unsupported statement "await"; use let, requires, ensures, next, emit, transfer, fee, liability or discharge.` |
| NS-4 | `C1` with `program_hash` replaced by reserve's | `stage_program` "Continuation does not belong to this program and stage plan.", attempted 0, committed 5 |
| NS-5 | `C1`, CLI cap 12 | `stage_logical` "Logical cap cannot exceed the stored cap.", attempted 0, committed 5 |
| NS-6 | `C1` with `logical.cap` set to `null`, true event | `Complete`, committed 11. **Hazard pin:** the cap is advisory. |
| NS-7 | `C1` with `committed` set to 4 | `stage_logical` "Committed work differs from the stage plan.", attempted 0, **committed 4**. **Pin:** the rejection echoes the unverified input. |
| NS-8 | Event label `"other"` | `stage_event` "Event does not match the pending await.", attempted 0, committed 5 |
| NS-9 | `C1` capture `amount` set to 1000, boundary unchanged | `mori::core::balance` `Insufficient balance for "escrow".`, step `Some(3)`, attempted 4, committed 5 |
| NS-10 | `C1` capture set to 1000 **and** boundary escrow set to 1000 | `Complete`, bob +1000. **Hazard pin:** a local continuation is forgeable and is never authority. |
| NS-11 | `C1` with two boundary balance rows swapped | `mori::core::balance` "Balances are not canonical", attempted 0 |
| NS-12 | `C1` boundary plus a `ghost` liability (declared asset) | `mori::core::liability` `Existing liability identity differs for "ghost".`, attempted 0 |
| NS-13 | Resume with `action = Some("other")` | `stage_site` "Selected action differs from the continuation action.", attempted 0, committed 5 |
| NS-14 | Delivery start with cap 10 | `stage_logical` "Action plan total exceeds the logical cap.", attempted 0, committed 0 |
| NS-15 | `C1` resumed twice with a true event | Two identical `Complete` results. **Pin:** consumption is untracked. |
| NS-16 | Reserve with `quote = 51` at site 1, then `ready = true` | Waiting at site 2, then `mori::core::requires` "Action predicate evaluated to false.", step `Some(9)`, attempted 6, committed 26, on every retry (F-12 pin) |
| NS-17 | Delivery `C1`, true event, submitted by a party who is not `alice` | `Complete`. The local reference has no caller concept; this pins F-11. |

### 8.3 Codec negatives (CN), authored against the future codec API (§10)

Each case must refuse unless marked otherwise.

| Test | Case |
|---|---|
| CN-1 | Encoding a post-body for `advanced:false` or for a rejection |
| CN-2 | Start from a Pending frame (frame overwrite) |
| CN-3 | Resume where `(action, site)` differs from the frame |
| CN-4 | Idle frame with captures, or Pending with `site = 0`, or `site > N_a` |
| CN-5 | Capture count, order or type differs from the plan site layout |
| CN-6 | Continuation `committed` not equal to `plan.committed_before[site]` |
| CN-7 | `continuation.boundary` not equal to `segment.execution.post` |
| CN-8 | Boundary rows outside the bound matrix, or a KeyHeld cell placed in the body |
| CN-9 | Present-zero duty decoded as absent, or residual used as the duty source |
| CN-10 | A round-trip that does not reproduce the continuation byte-for-byte |
| CN-11 | Labels missing from a record |
| CN-12 | Emit dropped from a record's effects |
| CN-13 | Required signers computed per action instead of per segment. Delivery segment 0 = `[alice]`, segment 1 = `[]`. |

---

## 9. Answers to Q1–Q6

### Q1. Fields: target-bound or reference-only

| Item | Target-bound | Reference-only | Condition |
|---|---|---|---|
| `program_hash` | Script parameter, plus an opaque datum header field | — | Equality only. No on-chain recomputation of SHA-256 over JSON. |
| `plan_hash` | Same as `program_hash` (new header field) | — | Same |
| `source_sha256` | — | Yes | Never authority |
| Action and site | `frame.action_index`, `frame.site`, redeemer | Action name; site label | `(action, site)` determines the label through the layout |
| Typed captures | Values (exact-length positional list) | Names, origins, ordinals | Layout fixed by `plan_hash`. Values persist **only** from a validated predecessor transition. |
| Boundary | State values, framed amounts, duty cells including presence | KeyHeld balances (unauthenticated premises) | Complete registry. Present-zero is `(1,0)`. |
| Cap | — | Advisory `stored_cap` | Never in the datum; never authority |
| Committed | Derived as `committed_before[site]` | Recorded | Not stored. The encoder checks equality. |
| Attempted | — | Yes | Per-invocation local work only |
| Event tuple | Redeemer values, positional | Label, binder names | Unauthenticated (Q6) |
| Effects | Recomputed by the target, compared by the comparator | Ordered list including Emit | Emit target policy per Q5 |

### Q2. Outcome handling

| Reference outcome | Target handling |
|---|---|
| `Waiting{advanced:true}` | One transition: pre frame (Idle, or Pending(a,k)) to Pending(a, k+1). The post-body is the continuation boundary. |
| `Waiting{advanced:false}` | **No transaction.** The validator must refuse a resume whose guard is false, mirroring `/4`'s missing circuit. The record asserts that the continuation is unchanged. It is excluded from matched coverage. |
| `Complete` | Transition to the Idle frame, with the body retained, including all present-zero duties. The residual is derived from the body, never from the positive-only list. |
| `StageRejection` | No transaction, and the target must refuse. The rejection's `committed` is unverified for R0–R6. |

### Q3. R5 composition

- **Bootstrap:** must mint an **Idle** frame only. A bootstrapped Pending frame would be capture fabrication, as NS-10 shows.
- **W(a):** replaced per transition by `segment_cost[a][j]`, with a reference meter limit equal to that cost. `stage_logical "static cost premise violated"` maps to `Incomplete{ReferenceInvariant}`; this is my V-C3 extension.
- **Sequence:** `seq` increments once per committed segment. There is none for `NotAdvanced`.
- **Comparator:**
  - a target refusal of Start while a frame is Pending is `OutOfCommonSubset{FrameBusy}`;
  - an accepted target transaction where the reference outcome is `NotAdvanced` or a rejection is `Different{critical}`;
  - V-C0 and V-C0a apply per thread.
- **V-T7:** add `plan_hash`, `action_index`, `site` and the event values.
- **LAY-01:** extends to labels, binder names and capture names.
- **Still open:** G1–G4, the layout hash and its staged descriptor, TC-2/TC-3, BLAKE, Data/CBOR, the C-table, and the slot/shared-VK ABI.
- **Owner text needing amendment:** R5S-04/05 (W) and the R5 bootstrap. Neither text is visible to me, so this is a precise blocking uncertainty.

### Q4. Permission

Yes, as specified in §6, only after C-1. Required owner-authored cases before projection extension are SP-1–SP-7, NS-1–NS-17 and NT-1–NT-24. CN-1–CN-13 apply when the codec exists.

### Q5. Emit

The reference record and codec must preserve Emit in the ordered effects. My first vote for the first target profile is to **refuse** any segment containing Emit (`UnsupportedDomain`), until a redeemer-committed effect commitment (a TC) is adopted. Reserve is therefore codec-supported and target-refused. The `/1` metadata Emit refusal stays as it is.

### Q6. Conditions before any on-ledger claim

- Local labels remain verbatim in all records. There is no upgrade from reference to authority.
- Await guards over unauthenticated binders shall be labelled "unauthenticated public event". No conditional-settlement or delivery claim is allowed until a stage-event-authentication design is adopted and evidenced.
- Consumption claims need target evidence: thread uniqueness, V-C0a single thread input, exactly one continuing output carrying the post-body, and authenticated bootstrap. That evidence is separate from the local `untracked` label.
- The F-11 and F-12 disclosures accompany every claim.
- Cardano Groth16 experiments are not evidence for any of these points.

---

## 10. Smallest fully preserved stage→UTXO schema (first vote)

### 10.1 Context

`StagedProjectionContext` is constructed only from raw `/2` source plus symbolic bindings (the R5S-06 `/2` rule), through `stage::load`.

Its private `StagedLayout` contains:

- states, sorted bytewise as `(name, type)`;
- assets in source order;
- accounts as `(name, class)`;
- framed cells: ScriptCustodied accounts × assets, ordered by `(AssetId, account)`;
- the duty registry over all actions, sorted by name, as `(name, AssetId, debtor, creditor)`;
- actions in declaration order, each with:
  - inputs;
  - `N`;
  - for each site: ordinal, label, binders, and captures with origin and ordinal;
  - `segment_cost`, `committed_before` and `plan_total`;
  - for each segment: statement range, `required_signers` (the KeyHeld `from` names of Transfer and Fee statements in **that segment**), and `has_emit`;
- `program_hash` and `plan_hash`.

Emit is allowed in this layout. Do not reuse the `/1` `describe_duties` Emit refusal for it.

### 10.2 Body (positional; header = R5 DatumV1 header as adopted, plus `plan_hash`)

```
StagedBodyV1 = Constr0[
  state_values   : List[Value]                     -- layout state order
  framed_amounts : List[Integer]                   -- layout framed-cell order
  duties         : List[Constr0[Integer p∈{0,1}, Integer amount]]  -- full registry; absent=(0,0); present-zero=(1,0)
  frame          : Constr0[]                                          -- Idle
                 | Constr1[Integer action_index, Integer site∈1..N_a,
                           List[Value] captures]   -- exact len(captures(a,site)); no padding
]
Value = Constr0[Integer u]  (UInt, 0≤u<2^128) | Constr1[Integer b∈{0,1}] (Bool)
Redeemer(Start)  = Constr0[Integer action_index, List[Value] inputs]
Redeemer(Resume) = Constr1[Integer action_index, Integer site, List[Value] event_values]
```

### 10.3 Transition record (reference-only; `StagedTransitionRecordV1`)

The record contains:

- the labels triple;
- `program_hash`, `plan_hash` and `source_sha256`;
- kind, action name and index;
- typed inputs, or the event (label and values);
- pre-body and post-body (null for `NotAdvanced` and for rejection);
- `premises_pre` and `premises_post`;
- `frame_pre` and `frame_post` (named);
- the complete Execution (original ordinals, ordered effects including Emit), or null;
- logical work: profile, `stored_cap`, `committed_before`, `committed_after`, `segment_cost` and `attempted`;
- the outcome: `Advanced{entry, exit}`, `Completed`, `NotAdvanced`, or `Rejected{error, attempted, committed, committed_verified}`;
- residual, and duty cells from the full snapshot;
- `required_signers(a, j)`;
- the verbatim upstream continuation.

### 10.4 Predicates

| ID | Predicate |
|---|---|
| P1 | Advanced: `continuation.boundary == segment.execution.post`. The post-body is computed from that boundary. The post frame is Pending(a, `continuation.site`, capture values in plan order). |
| P2 | Encoding requires `continuation.logical.committed == plan.committed_before[site]`. |
| P3 | Decoding body + premises + `stored_cap` reproduces the continuation byte-for-byte, with canonical sorting. |
| P4 | Start requires an Idle pre-frame. Resume requires `pre.frame == Pending(a, c.site, c.captures)` and `pre-body == body(c.boundary)`. |
| P5 | Complete yields an Idle post-frame. |
| P6 | `NotAdvanced` and rejection yield no body. |
| P7 | Duties come only from full snapshots. |
| P8 | Boundary rows outside the bound matrix are refused. |

### 10.5 Blocking uncertainty

The header fields are defined by R5 text that is not visible here. The codec is blocked on a second vote for D-1, D-4, D-5 and D-10.

---

## 11. EARS

### [ADOPT-COND] This candidate

- **CUR-01.** The mori-cardano crate shall derive `/1` exports from raw source through the current Core parser, and shall require no mori-core source modification.
- **CUR-02.** Where the candidate is described, it shall state that its root lockfile change alters the value of `mori_ledger::proving::source_closure()`.
- **CUR-03.** Before merge, the ledger source-closure selection shall be re-executed under its own scope.
- **CUR-04.** When `/1` source is supplied to the staged API, the system shall refuse it with `mori::core::profile`. When `/2` source is supplied to the projection or metadata API, the system shall refuse it with `mori::core::profile`.
- **CUR-05.** Where a staged outcome, rejection, continuation or segment is serialized, the serialization shall include the three `STAGE_LABELS` pairs verbatim.
- **TST-02.** Before projection extension, SP-1–SP-7 and NS-1–NS-17 shall be frozen by a non-author lane and shall pass under §6.

### [ADOPT-COND] Codec implementation

- **STG-01.** The codec shall construct context only from raw `/2` source plus symbolic bindings through `stage::load`.
- **STG-02.** If an outcome is `NotAdvanced` or a rejection, then the codec shall emit no post-body.
- **STG-03.** The codec shall encode duty presence and amount from complete snapshots, preserving present-zero, and shall never use `residual` as a duty source.
- **STG-04.** The codec shall round-trip every continuation byte-for-byte from body + premises + `stored_cap`.
- **STG-05.** The codec shall compute required signers per segment.
- **STG-06.** The codec shall preserve the complete ordered effects, including Emit, in every transition record.
- **STG-07.** The codec shall record rejection `committed` with an explicit verification flag that is false for R0–R6 refusals.

### [DEF-VOTE] Target

- **SGT-01.** The target thread shall hold at most one active frame. When a Start is submitted while a frame is Pending, the validator shall refuse.
- **SGT-02.** If a resume guard evaluates false, then the validator shall refuse.
- **SGT-03.** Bootstrap shall create only an Idle frame.
- **SGT-04.** The target shall store neither the logical cap nor committed work, and shall bound each transition by `segment_cost`.
- **SGT-05.** While stage-event authentication is unperformed, the system shall describe await guards as unauthenticated public events.
- **SGT-06.** If a target segment contains Emit and no adopted effect commitment exists, then the system shall refuse target emission for that segment.

### [ADOPT-COND] Disclosure

- **DSC-03.** Where `/2` reference results for delivery or reserve are cited, the citation shall be accompanied by the F-11 and F-12 text in §4.

---

## 12. Decisions needing a second vote

These carry my first vote only. None is adopted by this report.

| ID | My first vote |
|---|---|
| D-1 | Single active frame per thread, with concurrent multi-action pending refused |
| D-2 | Idle-only bootstrap |
| D-3 | A false guard produces no transaction |
| D-4 | Cap excluded from the target; committed derived, not stored |
| D-5 | Exact-length captures, no padding; reuse of the P2 Value codec |
| D-6 | Emit refused at the target until a TC is adopted; preserved by the codec |
| D-7 | Per-segment signers |
| D-8 | `program_hash` and `plan_hash` as opaque script parameters, plus header copies; the layout-hash preimage includes the staged layout and `plan_hash` |
| D-9 | Positional redeemers with the label implied |
| D-10 | KeyHeld cells as premises only |
| D-11 | Comparator amendments (§9 Q3) |
| D-12 | R5 W → `segment_cost` and bootstrap text amendments |

C-1 is a repair, not a vote.

---

## 13. Limitations and non-claims

**Limitations:**
- Nothing was executed. Every value in this report is derived by reading.
- The reserve, scenario and event files, the R5 text, the ledger coverage test and the CLI report schema were not seen.
- No hash was recomputed.

**Non-claims.** This report establishes none of the following:
- compilation or passing tests on PR54;
- profile admission, a layout hash, Data/CBOR, a validator, Aiken, or comparator results;
- ledger, Preprod or Groth16 evidence;
- event authentication, consumption or stage authority;
- safety of any specimen's funds;
- coverage of the 20 patterns, general obligations, private PCD or recursion, or any U, M, MC, SP or G exit.

Native authority for Midnight remains zero. Maintainer review is not a developer requirement.