# Fresh nonsecurity source / result / publication review

## Top finding and scoped verdict

**VERDICT: APPROVE_WITH_SCOPE — publication of the exact frozen initial selection is approved from the nonsecurity source/result/documentation/membership perspective, including the executor-evidence files only as historical evidence under the conditions in §6.**

This verdict covers:

- the exact 58-file / 813,918-byte initial selection frozen at `2026-10-16T21:52:24.188500+00:00`, selection SHA-256 `c3808afd4bec4a01fc29b7c2c42ce8e3d022d4f1a304ca7d403158972b5aa8fd`;
- source-composition SHA-256 `5a8950587b7e50029bcb4a049f21ee36763498c07ef9dc2a3aace13fa509553d`;
- all ten Rust sources, 139,554 bytes in total;
- the recorded one-time metadata compilation result;
- README scope, membership joins, copy provenance, and historical reports as preserved provenance.

This verdict does **not** cover:

- security design, publication-security, confidentiality, guard/refusal, or StepInv semantics — those remain with the separately designated security reviewer, and this report is not a substitute security certification;
- adoption of the proposed v3a ABI/interface or the event-metadata amendment;
- selector, driver, admission, charging/capacity enforcement, plan validation/application, `StepInv`, or owner-engine correctness;
- tests, runtime row application, linked/loaded/executed binaries, SDK/model/native/proof/circuit/ledger results;
- native, target, full-B, or parent acceptance.

No tools were used in this review, per the task constraint. Hash and byte statements below are textual comparisons against the supplied inline bindings, not independent recomputation. That limit applies to my own verdict exactly as it did to the earlier text-only reports.

## 1. Source assessment

The ten Rust files in `models/sherlock/b-core-host-source-20261016/` are internally coherent as a metadata-compilable payload-source composition:

- `lib.rs` forbids `unsafe_code`, binds `inputs/raw.rs` and `inputs/shared.rs` by fixed relative `#[path]`, declares the private `families` module, and re-exports only `HostFamilyPlans`.
- `families/mod.rs` contains the sole `FamilyPlans` implementation: eleven one-line forwarders (`plan_legacy`, `plan_scalar`, `plan_env_get`, `plan_env_put`, `plan_find`, `plan_bind`, `plan_append`, `plan_validate`, `plan_equal`, `plan_match`, `plan_bookkeeping_release`) that pass `sel, pre, guards` unchanged and recompute no site, rule, charge, cap, or guard fact.
- `families/legacy.rs` and `families/scalar.rs` are thin adapters whose only inclusions are `../fragments/legacy.proposed.rs` and `../fragments/scalar.proposed.rs`. No other `include!`, `include_str!`, `include_bytes!`, `env!`, build script, Cargo step, or SDK reference reaches outside the ten-file closure in the supplied text.
- The row payload bodies are consistent with the prior scoped family reports preserved in `reviews/`: 37 legacy + 6 scalar + 20 environment/find/append/release + 33 structural = 96 Enabled row payloads. Disabled/ExtensionOnly rows have no branches and fall to the sole foreign-rule literal `HostViolation{WrongVariant, None, None, Some("foreign rule for family")}`.
- `inputs/shared.rs` contains declarations and pure helpers only where the candidate claims them: `PreImageView`, `GuardEvaluator`, `FamilyPlans`, and `OwnerEngine` are trait declarations; `OwnerGuards::new_owner`, `ValidatedPlan::from_validated`, `PreImage`, `Admitted`, and `MachineImage` internals have no family-reachable construction path in this composition. No selector, driver, admission, `validate_plan`, `apply`, or `step`/`run` body exists. The README claim "interfaces are declarations and effects are pure planned payload construction" is accurate.

Two source-level observations, neither blocking for this publication scope:

1. **Stale in-file status headers.** Several preserved snapshots still say compilation is unrun: `families/environment.rs` ("Rust compilation/execution is UNRUN"), `families/structural.rs` ("Uncompiled"), and both fragments ("Compilation, Rust tests and native execution are UNRUN"). These are byte-preserved historical comments from before the actual run. The crate README and the audit README correctly supersede them for the metadata result. Because the selection is byte-frozen, these comments must **not** be edited under this publication; readers must read them at their recorded stage, as both READMEs instruct. This is a documentation-reading condition, not a source defect requiring removal.
2. **Placement fragility is disclosed and real.** The composition binds the crate-root layout in `lib.rs`; the `super::super::shared` imports in `environment.rs`/`structural.rs` and the `../fragments/…` includes resolve only under this layout, not under the `helper_machine/` layout sketched in the `shared.rs` header comment. The READMEs do not claim the other layout. Any future relocation is new integration work requiring fresh compilation evidence.

## 2. Actual-result assessment

The actual evidence is internally closed and I verified it as a chain, not from the summary paragraph:

- `actual/receipt.json` (SHA-256 `939c0c42…fb14`) records two children: `rustc -vV` (exit 0, 0.114 s) and one compile child (exit 0, 0.366 s, started `2026-10-16T21:47:18.503+00:00`) with argv `rustc --crate-type=lib --edition=2024 --crate-name=sherlock_b_shared_source_diagnostic --emit=dep-info=…/source.d,metadata=…/source.rmeta …/source/lib.rs`, a three-key child environment, 30 s per-child timeout, and `terminal_state: success`.
- `actual/result.json`, `actual/ROOT-TOOL-TERMINAL-01.json` (overall CLI exit 0, wall 0.401 s), and `actual/ROOT-INDEPENDENT-RESULT-01.json` (recorded 21:47:49 UTC) agree on receipt/result hashes, both child exits, the 0.366 s compile duration, six warnings, and the exact ten dep-info inputs. No record claims a linked, loaded, or executed binary, tests, or row application.
- `actual/logs/compile-diagnostic.stderr` contains exactly the six retained warnings (dead fields on `AdmittedBody`, `MachineImage`, `PreImage`, `ValidatedPlan`; unused `OwnerGuards::new_owner` and `ValidatedPlan::from_validated`) and the line "6 warnings emitted". These warnings corroborate, rather than contradict, the stated scope: the warned items are precisely the owner-side carriers whose bodies are absent. `compile-diagnostic.stdout` and `rustc-version.stderr` are empty; `rustc-version.stdout` records rustc 1.99.0 (b940084d7, 2026-09-28, x86_64-unknown-linux-gnu, LLVM 23.1.1).
- `actual/source.d` lists exactly the same ten source files for both the `source.d` and `source.rmeta` targets, matching the receipt's `compile_inputs` and `ROOT-INDEPENDENT-RESULT-01.json`'s `exact_dep_info_inputs`. The `.rmeta` itself (880,752 bytes, SHA-256 `b68e8979…0f2e8`) is recorded only and is **not** in the selection. That exclusion is correct and required.
- Receipt `sources` rows show before/snapshot-after/checkout-after hashes identical for all ten files, and `bound_files_after` re-binds the executor, closure, operation, composition, all ten sources, and both preflight reports. `ROOT-OPERATOR-PREFLIGHT-01.json` (21:47:08 UTC, before the run) pins the same executor and membership hashes and both preflight report hashes with actual exit 0. `provenance/ACTUAL-RESULT-VERIFICATION.json` (21:51:10 UTC) re-states the same chain as a read-only copy verification, explicitly not a rerun.

Conclusion on the result: the evidence supports exactly one claim — **the exact ten-file composition typechecked as Rust metadata under that one recorded invocation, exit 0, six warnings**. It does not support row-semantics, runtime, enforcement, cost, or adoption claims, and no selected document makes those claims for this result. Compiler success settles metadata compilation only. The earlier reports' UNRUN statements remain correct at their recorded stages and are preserved unrewritten, as required.

Two honesty limits are recorded in the receipt itself and must survive publication: the rustc sysroot/stdlib, system libc, Python stdlib, and kernel are unhashed inputs, and local SHA-256 bindings do not attest compiler sysroot, host, kernel, or backend identity. Both READMEs state this. No document in the selection overstates the binding.

## 3. Membership, joins, and provenance

- `membership/SOURCE-COMPOSITION.json` and `executor-evidence/ROOT-SOURCE-COMPOSITION.json` are byte-identical (both SHA-256 `5a895058…553d`) and list the same ten files in the same order as the receipt's source rows and the crate directory. Per-file byte counts sum to 139,554. The six `exact_snapshots` origins and four `new_root_glue` files are consistently declared.
- `membership/ROOT-ROW-MEMBERSHIP-JOIN.json` covers 96 Enabled IDs exactly once across three groups (legacy-scalar 43, environment 20, structural 33), bound to `rows-supplement.json` SHA-256 `a0dd03b1…5e75e` and to the three family mapping files whose hashes match the copies in `source-basis/`. Its own scope line correctly limits it to mechanical membership, not semantic or circuit correctness.
- `provenance/COPY-PROVENANCE.json` accounts for the ten Rust files, the actual-result records and logs, both membership files, the executor-evidence files, all six review reports, and the R4/v3a/family source-basis files, each with an original private-runtime path and the public path. The two READMEs are correctly listed as new prose, not copies. Original absolute paths and the `.rmeta` digest remain in the records as provenance; no compiled artifact, cache, provider stream, or private reasoning is selected. That treatment is correct.
- README link targets resolve as written: the audit README's `../../../../models/sherlock/b-core-host-source-20261016/README.md` and the crate README's `../../../docs/audits/2026-10-16/sherlock-b-core-host-source/README.md` are mutually correct relative paths.
- Scope statements in both READMEs are accurate and non-inflating: HOST `sherlock-helper-microstep/1`, `DedicatedSorts`, finance Absent; qualified R4 HOST Stage 1 normative adoption distinguished from the unadopted v3a interface/event amendment; logical work/row counts explicitly not circuit costs; all native/parent gates open.

Identity-receipt defects to note (documentation quality, not blockers):

1. `reviews/legacy-scalar/IDENTITY.json` has an empty `startup_configured_identity` list and an empty `actual_terminal` list. The report's provenance for that lane therefore rests on the `RESULT.json` hash linkage alone. It must not be cited as a configured-identity-attested review.
2. `reviews/structural/IDENTITY.json` records `elapsed_seconds: null`. Minor, but it means that receipt is not a complete terminal record.
3. `provenance/OPUS-OUTAGE-ROUTING.json` correctly records that the requested Opus review never occurred (CLI exit 1, API 429, no canonical returned model) and that the security lane was a user-authorized Muse fallback. Any publication summary must keep that wording; describing the historical security preflight as an "Opus review" would be a false claim.

## 4. Prior reports

All six preserved reports stay within their stated scopes and none is rewritten by the later compile:

- The three family source reports (legacy-scalar PASS source-only 37+6; environment PASS row-effect fidelity 20; structural AGREE scope-limited 33) predate compilation and claim no compiler result.
- The composition report's PASS is explicitly mechanical-composition-only, and its compiler-needed concerns (F-1/F-2 borrow/reborrow forms) are exactly what the later actual metadata result settles for this composition — no more, no less. Its UNRUN status at its stage was correct.
- The two compiler-preflight reports approve only the wrapper's execution eligibility for the one diagnostic, with matching `VERDICT: APPROVE_WITH_SCOPE` lines bound to membership SHA-256 `3ad719d8…c85c`, which is the membership hash the receipt and preflight records actually carry. Their security/nonsecurity role split is respected in this report.

## 5. Independence limit

Both current publication review roles, and all preserved Muse reviews, use fresh separate sessions of the same provider/model family (configured `muse-wonder-1.0`, high effort). This is task/session independence only. It is not cross-provider or cross-model-family independence, and no backend identity or effort attestation exists or is claimed. The READMEs and every `IDENTITY.json` state this correctly. Root's final summary must preserve this limitation verbatim in substance.

## 6. Separate permissions

### 6(a). Executor evidence — decided explicitly

**PERMITTED, as historical evidence only, from the nonsecurity perspective.** Root may publish `executor-evidence/diagnose_once.py` together with its exact `SOURCE-CLOSURE.json`, `EXACT-OPERATION-PROPOSAL.json`, `EXECUTOR-MEMBERSHIP.json`, and `ROOT-SOURCE-COMPOSITION.json`, subject to all of these conditions:

1. They remain framed exactly as the audit README frames them: already-used historical executor records, not an installed runner, workflow, CI entry point, replay instruction, or future execution grant.
2. No new runner, framework, CI wiring, resource cap, or replay/execution authority is created or implied by the publication.
3. The pre-execution status fields inside them (`actual_compile: "UNRUN"`, `compiler: "UNRUN"`, `tests: "UNRUN"` in the closure/operation/composition records) are preserved unrewritten and read as proposal-stage records superseded by `actual/receipt.json`. Editing them to say "ran" would falsify provenance and is not permitted.
4. It is understood and stated that "no future execution authority" is a governance statement, not a technical disablement: the published script is executable Python text that, if deliberately placed in its original fixed private root alongside its bound files and preflight reports, would run. Publication itself installs nothing and runs nothing, and the script's own bindings (fixed absolute root, exact hashes, fresh attempt directory, both preflight approvals) mean the public copies alone cannot execute the diagnostic. The security reviewer's separate publication decision on this code remains theirs; this nonsecurity permission does not substitute for it.

No removal is required under this permission. The executor text is load-bearing provenance: without it, the receipt's executor/membership hash bindings (`12c53589…848`, `3ad719d8…c85c`) and the preflight approvals could not be checked against the actual result. Removing it would break result closure — the opposite of the publication's purpose. If the designated security reviewer withholds publication permission for the code, the fallback removal would have to be the whole `executor-evidence/` directory (script, closure, operation, membership, and its duplicate composition copy) together with the audit README's executor paragraph; publishing the four JSON records without the script they bind would leave dangling hash references and is not an acceptable partial removal.

### 6(b). Post-review additions outside the initial selection

Each item is answered independently:

1. **The two final publication reports, verbatim — PERMITTED.** Conditions: byte-verbatim inclusion (no edits, no excerpting that changes a verdict or condition); placed outside the frozen 58-file selection and labelled as post-review additions; each report's own hash recorded in the joining summary.
2. **Factual requested/configured identity, actual terminal/exit, report/input hashes, and same-family independence-limit receipts — PERMITTED.** Conditions: factual records only, derived from the structured receipts; they must state configured identity as a label, actual terminal/exit as observed, and the same-provider/model-family independence limit; no backend identity or effort attestation may be asserted.
3. **A final joining membership/summary — PERMITTED, with a no-new-claims condition.** It may join the initial selection and the permitted additions by path and hash, restate the verdicts and conditions as recorded, and state the remaining open gates. It must not upgrade any scoped verdict into adoption, security certification, runtime/test evidence, or acceptance; must not describe the historical security preflight as an Opus review; and must not silently absorb the stale UNRUN/in-file headers without the temporal reading in §1. If it contains any new substantive claim, that claim is outside this permission.
4. **The exact wiki append — PERMITTED.** Root may append the exact bytes of `WIKI-LLM-ITERATION-NOTE.proposed.md` (SHA-256 `d782250e…ec7d1`) to `wiki-llm/sherlock-implementation-journal/2026-10-16.md`, byte-identical, append-only, exactly once. The note's text is accurate as written: metadata compilation only, six warnings retained, all owner/driver/admission/StepInv/native/parent obligations open, v3a and event metadata unadopted, work counts not circuit costs. Root must verify it is appending to the tail of the existing journal file (no overwrite or insertion) as part of its integrator duties; the target file was not supplied inline, so that check is Root's, not verified here.

No permission in §6 covers rewriting any source, normative, result, or prior review wording.

## 7. Remaining limitations

- Metadata typechecking only; no tests, no row application, no runtime behaviour observed by anyone in this evidence chain.
- All owner bodies (selection, admission, guards, `validate_plan`/apply, StepInv checking, driver) remain absent; the six dead-code warnings are the visible signature of that absence.
- v3a interface and event-metadata amendment unadopted; I-B event adoption still pending; full-B ABI unadopted.
- Unhashed sysroot/stdlib/libc/kernel and no sandbox claim bound the actual result's evidentiary weight, as the receipt itself states.
- All native, proof, circuit, target, and parent roadmap gates remain open. Publication of this selection closes none of them.

## Tail acknowledgement

Full-source coverage closed at the exact supplied tail, quoted as required: `MUSE_B_CORE_HOST_DELIVERY_NONSECURITY_FULL_TAIL_20261016`

**Final disposition: APPROVE_WITH_SCOPE for the exact initial selection as historical source-plus-actual-metadata-result evidence; executor evidence permitted as historical evidence only under §6(a); post-review additions and the exact wiki append permitted under §6(b). No new source, ABI, security, or acceptance adoption is granted or implied by this nonsecurity review.**


