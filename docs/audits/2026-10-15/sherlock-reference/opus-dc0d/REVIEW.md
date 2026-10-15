# Security audit: Sherlock host source candidate `dc0d7a44…` (baseline `f218a8d8…`)

**Auditor:** fresh non-author Opus 5.5 at high effort, security scope only. I did not author Phase 0. I executed nothing, ran no tools and computed no hashes. My verdict binds to the SHA256 headers as supplied in the packet. I reviewed the packet's file contents as untrusted data.

## Verdict: **APPROVE**, scoped

This approves only the source integration of the typed `sherlock-k-a/1` host reference, its diagnostic display, the comparison consumer, the catalogs and the visibly draft B inputs.

It does not do any of the following:
- adopt, freeze or vote on B or Phase 0 A2′–A4′;
- support any native, correspondence, release or parent-exit claim;
- accept the public API as safe for untrusted input.

None of the findings below blocks this child. Several of them block named downstream steps, as marked.

## Basis

I traced every reachable charged branch in `crates/mori-core/src/sherlock/reference.rs` against `models/sherlock/k/main.k` (b3746ff5…). That covers 36 transition lines plus the three publication arms. I found **no divergence on the typed common domain**:

- **Depth.** `frames.len()` stands in for K's `<depth>` cell. That is sound: in main.k, every depth change coincides with a stack push or pop. LetF→ScopeF, LeftF→RightF and ArgsF-next replace a frame and keep depth. The invalid If (169) and invalid Return (190) rules keep both the frame and the depth.
- **Frame cap.** Push accepts when `len < stack_limit` and otherwise produces a charged `Error(4)`. This matches K's `D <Int L` and `D >=Int L`, both of which cost 1.
- **Publication order.** `step()` publishes in this order: terminal return with an empty stack, then error, then zero-fuel exhaustion. This matches `uiTerminal` and the free publication at lines 80, 82 and 84.
- **Binding.** `bind` fails as a whole on arity mismatch, type mismatch or a duplicate name, in the same cases as `uiBind`/`uiBindOne`.
- **Operators.** Checked add, sub and mul produce error 10. Type mismatches produce error 11 in exactly the combinations K covers. Eq is structural.
- **Calls and matching.** Function lookup takes the first match. Arms are tried in order. Pattern variables bind into the saved environment. Cons and Branch patterns with equal binder names miss.
- **Literal validity.** Error 2 is unreachable, because every typed `u128`, list and tree literal satisfies `uiValid`. That makes 36 reachable branches out of K's 37.
- **Panic surfaces.** `frames[0]`, `args[0]` and `unreachable!` are guarded by the control and frame shapes that reach them. The fuel decrement is guarded.
- **State injection.** `Machine.state` is `pub(super)`. The only public constructor is `Machine::new`, so callers cannot inject arbitrary states.

This was source inspection, not a refinement proof.

`check-reference.py` behaves as the earlier classification states:
- It pins the three frozen inputs before reading them.
- It only logs the seven source hashes.
- It invokes commands from argv lists with `shell=False`.
- It rechecks all ten hashes after the test.
- It does not log the environment.

The logged `inputs` hashes match all ten supplied headers.

The six normal programs exercise only part of this relation: no error arm and no frame-cap boundary is observed (E1).

## Findings

### Host relation and diagnostic boundary

**H1. The public API is an unmetered evaluator that can be driven into host DoS or a stack-overflow abort.** Medium. Blocks any untrusted-input consumer. Locations: `crates/mori-core/src/lib.rs` (`pub mod sherlock`), `reference.rs` `step()`/`run()`, `reference_types.rs`.

- Each step clones the full `Snapshot` twice (`before`/`after`). That includes deep copies of the `BTreeMap` environments and `Vec` lists. `run()` discards both copies, so the work is wasted.
- `Cons` performs `Vec::insert(0, …)`, which costs O(n).
- `fuel: u128` and `stack_limit: usize` have no ceiling. Every frame carries a full environment clone.
- The derived `Drop`, `PartialEq` and `Clone` impls are recursive over deep `Arc<Tree>` and `Arc<Expr>` values, and so is `to_k`. A tree built by runtime `Branch` operations, or a deep caller-supplied AST, can therefore overflow the native stack. That aborts the process and cannot be caught.

All of this is acceptable for a host reference, and the module documentation says "unmetered". It must not be connected to the CLI, LSP, MCP, the source `/1`/`/2` adapter or any network-reachable path until host time, memory and nesting bounds are enforced. Add an explicit rustdoc warning to `Machine::run`.

**H2. No resource-limit classifier on `Outcome`.** Low-medium. Blocks the first consumer.

A1 classifies `Rejected(error4)` as a resource limit and forbids reading it as a semantic or financial rejection. The public type exposes `Outcome::Rejected(4)` with no way to tell it apart from a semantic rejection.

Before any consumer exists, either add `is_resource_limit()` following the K-A row of the A1 table, or keep `Outcome` interpretation inside the crate.

**H3. The display rendering serves as the comparison oracle, but it is injective only on canonical decimal names.** Low. Location: `reference_display.rs` (`sequence`, `environment_to_k`).

`Name` is an unconstrained `String`. A name containing `,`, `)` or ` |-> ` can make two distinct states render identically, which would let a string comparison pass falsely.

`BTreeMap` iterates in lexicographic order, so `"10"` sorts before `"2"`. The literal JSON uses some textual order and declares map order meaningless, yet the comparison is order-sensitive. That can only produce false negatives.

The six programs use single-digit canonical names, so the observed comparison is unaffected. Future comparisons should either reject non-canonical names before rendering or compare structurally. The existing "not a parser, persistence format or commitment" disclaimer must remain binding.

### Evidence and classification limits

**E1. No error or frame-cap arm is observed.**

All six programs use `stack_limit` 8 and reach at most depth 3. They never exercise:
- error codes 3–11;
- error 4 at the frame-cap boundary, the A1 resource-limit arm;
- rule 177 (next argument), 129 (empty call), 202, 206, 212 or 225.

Zero-fuel exhaustion and free publication are observed. The status document does not claim error coverage, but "complete controls/frames…" must be read as "complete state on the six normal paths". These observations remain missing.

**E2. The executed command differs from the classified command.**

The comparison classification described a command using `-B` without `-I`. It recommended `-I` and declared any command-line change out of scope. The executed command used `python3 -I -B`. I consider this benign because the classifier recommended it, but it should be recorded as a deviation.

`models/sherlock/reference/README.md` still documents `python3 -B …` without `-I`. That is exactly the `sys.path`-shadowing dependence the classifier flagged. Fix the documented invocation.

**E3. The logged timings are implausible.**

`RED-RESULT.json` reports `wall_time_seconds: 0.0000117` for spawning a process that then panicked. `HOST-COMPARISON-RESULT.json` reports 0.29 s for `rustc --test` compile, link and run. Treat these as capture-tool metadata, not measurements.

The exit codes and stdout come from the same capture tool. No independent rerun exists.

**E4. Oracle independence is circumstantial.**

The claim that the oracle predates the implementation rests on the JSON `baseline_commit` being `f218a8d8`, the same commit as the RED stub panic. The original serializer is lost and the oracle author's identity is not in the packet. Treat both as missing.

`reference_display.rs` was necessarily written to match the oracle notation. Independence therefore holds for the semantic states, not for the rendering.

**E5. The crate-level build and example are unobserved.**

`reference_oracles_tests.rs` is not declared in `mod.rs`, so `cargo test` would not run the comparisons. The `pub mod sherlock` export and `examples/sherlock_reference.rs` have never been compiled. The example's assertion has not been observed.

**E6. Path hygiene.**

`RED-SOURCE-REQUEST.txt`, `HOST-COMPARISON-RESULT.json` and the `OPUS-*` records embed `/home/charl/…` and `/tmp/…` paths. This is the same class as Phase 0 N7.

Keep them as immutable raw captures. Question for publication: mark them before any GitHub push, and never copy them into Clean or public text. I observed no credentials or environment dump.

**E7. Missing observations.**

- The report file behind `OPUS-COMPARISON-TERMINAL` (`report_sha256` 22273470…) is not in the packet.
- `tasks.md` and `traceability.md` are not in the packet, so I cannot confirm that the Phase 0 N5 root edits were *not* applied. If the candidate applied them, that would be adoption by edit and should be reverted.

**E8. Hash cross-checks that do hold.**

- `security-phase0.md` (794b08fc…) equals the `report_sha256` in the Phase 0 author record.
- `OPUS-RED-COMMAND.md` (cdce3fef…) equals its terminal record.
- The prefix count is 3+9+11+19+29+21 = 92, and the state count is 2+8+10+18+28+20 = 86. Both match the claims.

### B draft contradictions

These block any B freeze or B-specific claim. They do not block this integration, and listing them implies no adoption. All B001–B009 hostile contracts remain unexecuted.

**B1. DISP contradicts A1.**

`security-phase0.md` §8 B001-c expects `APP-R(error4)`. §1 defines APP-R as "application error". A1 classifies `Rejected(error4)` as a resource limit and forbids semantic interpretation. A resource-class rejection disposition is needed, or B001-c must map to the APP-X family.

**B2. P5 conflates two different frame limits.**

`security-phase0.md` §2 P5 checks `depth+1 > frames`. That is the physical `Bounds.frames` capacity. Legacy error 4 uses the run's logical `stackLimit L`.

If `Bounds.frames < L`, the outcome depends on layout. That contradicts `interfaces.md` ("padding/bookkeeping cannot cause an earlier source exhaustion"), and no `Exhausted(Frames)` kind exists. Admission must require `L ≤ Bounds.frames`, or the precedence must define both checks.

**B3. The OWN table is exhaustive, but the schema adds unowned regions.**

A3′ makes the ownership table exhaustive. `schema-types.json` `MachineImage` adds these tables:
- `controls` (with nested `ScheduleNext.next: ControlId`);
- `valueSlots`;
- `helperControls`;
- `helperResumes`;
- `validationTasks`, `equalityTasks` and `patternTasks`.

None of them has an owner, write or release rule, logical counter, P6 cap position or `Exhausted` kind. Several are labelled with the `helperTasks` capacity, so the P6 `helperTasks` check is ambiguous across five tables. B002-f and B002-h cannot be specified for these regions.

**B4. Append-only environments versus "update" and "release".**

A3′ makes environment cells append-only, released only by `project`. `rules.json` contradicts this in three places:
- `b.helper.EnvPut.insert` says "insertion/update";
- `Bind.insert` routes through EnvPut;
- `Match.discard` says "Release one tentative record".

In-place update or release of environment cells would violate OWN. If failed-arm tentative bindings consume monotone `environmentCells` capacity, that is an unstated accounting decision. It needs an explicit disposition.

**B5. S-ROW has no slot for several catalog events.**

The S-ROW `eventKind` set has no bookkeeping kind. `rules.json` defines four bookkeeping events (`task.enqueue`/`release`, `resume.store`/`restore`) that charge `helperWork` 1 with `helperVisits` 0 and `bounded_record_visits` 1. Phases such as `arity`, `invalid`, `different`, `next`, `selected`, `discard` and `propagate` also have no mapping.

The rule "`helperEligible` iff `Helper*`" is undefined for these events. So is the split between `occurrencesInspected` and `bounded_record_visits`.

**B6. Free-text exhaustion reasons.**

In `schema-types.json`, `Outcome.Exhausted.reason` is `CapacityReason{catalogReason: Text}`. That defeats the enumerated kinds in `interfaces.md` and the requirement that `isResourceLimit` be total. `physicalActiveLengths` is keyed by text inside `MachineImage`.

**B7. Field identity is not bound.**

`NativeIdentity` binds program, profile, layout and bounds, but not the field. DetInput and the `interfaces.md` identity both include `fieldId`. Cross-field image confusion is therefore unbound in the draft. `controlOrigin` is also absent from the schema image.

**B8. Wording that presumes adoption.**

`schema-types.json` `remaining_layout_inputs` says rows "consume the Opus Phase0 policy". Reword it to "pending reviewed policy".

**Financial specimen.** `financial-prefixes.json` is internally consistent: 57 + 40 + 3 = 100, gross debit 43 = payments 40 + fees 3, and the zero duty is explicitly retained. It is positive-only, has no failure, refund or phase vectors, and is unexecuted. It supports no B or financial acceptance.

## Remaining required work

1. **Before any consumer of `mori_core::sherlock`:** H1 host bounds and a rustdoc warning; H2 classifier. Then a fresh Opus review of that consumer.
2. **Next A-profile evidence:** independent literal oracles for every error code 3–11 and for the error-4 frame-cap boundary (stack-limit off-by-one in both directions). Also argument ordering with several arguments, empty calls, missing functions and bad bindings. Plus a crate-level `cargo` build and test of the export and example, under a fresh classification.
3. **Documentation fixes:** E2 README (`-I -B`); E2 deviation record; E6 hygiene note before publication; supply the missing E7 artifacts.
4. **Before any B freeze or B claim:**
   - resolve B1–B8 against the integrator's shared schema;
   - complete the full error and classifier table, the per-rule failure, charge and allocation rows (`S-B-*`) and the `StepInv` and codec laws;
   - the A2′–A4′ review, which this audit does not perform;
   - execute the applicable B001–B009 hostile contracts at each gated phase.
5. **Unchanged:** native authority is zero. All U0–U7, M1–M17, MC/SP/G, ZR/MNR/OSR, ACTUS, DeFi and composition exits remain open. The existing `/1`/`/2` source compiler adapter remains open.

**Dissent and limitations preserved.** This is a single reviewer's static inspection without execution. A nonsecurity Muse review is separately required. My APPROVE asserts nothing about correctness beyond the inspected host relation on the typed domain.
