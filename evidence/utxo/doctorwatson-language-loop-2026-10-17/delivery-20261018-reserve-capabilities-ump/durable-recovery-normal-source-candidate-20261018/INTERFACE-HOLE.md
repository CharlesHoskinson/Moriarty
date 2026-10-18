# Exact production interface hole — no substitute implementation

## Existing actual consumer and why its interface cannot carry the requested result

At exact E4D, `crates/mori/src/commands/simulate.rs::run_stage_start` calls `stage::start`, `run_resume` reads the bounded old Continuation and StageEvent and calls `stage::resume`, and `print_stage_result` prints `stage::outcome_report`. There is no paired module, recovery event, reopen or checkpoint-output argument. `inspect.rs` has no recovery module input. The actual `test` command's `/2` branch consumes `stage_test`; it has no durable recovery process profile.

`stage::Continuation` carries base program/plan identities, action/site/captures, committed logical count/cap and a boundary. It does not carry total attempted history, committed ordered-prefix history, companion identity, recovery closed control or a complete closed checkpoint. `StageOutcome::Complete` means the existing normal forward plan completed; it cannot be reused to mean a differently-costed recovery without a version/semantics lie. `StageRejection` supplies error/T/C but no replacement durable state or history. Reopen cannot simply call `serde_json::from_str` and trust these missing fields.

The current E4D complete escrow/partial-delivery public tests already write actual old continuation values and resume them in fresh processes. That existing capability is not the missing cancellation/compensation transition. This candidate therefore exercises the actual proposed paired public commands, rather than adding another passing legacy file test.

## Required returned information and consumer connection

These names are explanatory proposal boundaries, not adopted Rust signatures, validation algorithms or public permission gates. The designated owner may require DTO/API changes; the exact source candidate then needs fresh review.

| Actual consumer insertion point | Required input/output boundary before a genuine production patch is coherent | Unselected prerequisite |
| --- | --- | --- |
| `inspect::run` paired source branch | Base source + recovery source → complete paired Program/Plan/identity view, preserving old no-module route | O-ID/O-REG/O-ACCT/O-TRACE/O-CAP; adopted grammar/DTO and truthful supported subset |
| `simulate::run_stage_start` paired branch | Pair + scenario + cap → complete versioned pending durable record and whole report with C/T/effects/history | Combined registry/account/trace admission and successful_path_ceiling preflight; O-CAP refusal order; origin/history policy |
| New paired checkpoint reader before reopen/resume/recover | Actual file bytes + retained pair → admitted complete durable state or a defined full refusal result, with no scenario replacement | O-ID/O-DURABLE/O-PEND/O-LIVE; byte/node/history/accounting bounds, canonical binding, provenance and migration rules |
| New read-only reopen route | Admitted pending OR closed record → unchanged record and attempted0 report | Exact admission failures/order and public error DTO; whether decode/admission work counts; no execution/consumption claim |
| New paired resume/recover route | Admitted pending record + typed normal/recovery event → complete normal result or precisely selected failure/pending/unknown outcome | O-EVIDENCE/O-CONTROL/O-FAIL; actual owner-defined permission/finality/control rules; lifetime budget policy |
| `simulate` publication/report boundary | Complete resulting record + output destination → known exact published/report outcome, or exact known/unknown failure disposition | O-IO: serialization/export bounds, publication ordering, overwrite/path/crash/output failure, attempted/committed accounting/error rules |
| `test::run` new profile branch | Fixed manifest + bounded actual executable/process/file operations → complete ordered case reports/failures | Adopted aggregate schema, process/file resource envelope, output retention, error semantics; no arbitrary command/plugin dispatch |

A production writer is not separable from the final publication/report row: choosing create-new, replacement, sync/hard-link/rename, output order or returning a generic old command error already adopts rules that the current durable design explicitly reserves. A decoder or `ValidatedCheckpoint` token with a fake successful constructor would be an even stronger false substitute. This packet creates neither.

## Consequential ordinary choices still unadopted

The companion `moriarty-recovery/1`, new semantics/DTO/manifest profiles, separate handler arena, guard2+body4 schedule, successful_path_ceiling metadata, complete attempted history, mandatory new checkpoint output, fresh-process reopen, source/plan/report layout and public operation matrix remain the refined proposal. Full major3 and prior L03 alternatives remain intact in their original directories. No choice is silently adopted by writing expected test source. Owner decisions can alter these shapes; revise the whole candidate and obtain new votes rather than changing hidden validator internals.

## Required scope retained, not supplied by this partial source candidate

MPLR-001/002 require actual accepted-prefix state and persistent bound continuation; MPLR-004/005 retain residual duties and distinguish custody refund from forward compensation; MPLR-009 retains cumulative work; MPLR-010 forbids treating missing evidence as nonexecution; MPLR-017 requires complete financial accounting. MPLR-018 and UNI-024 require truthful inspectable public operations and developer-journey reporting. The complete clauses and product contract are pinned under `before/`. Public local unsigned tests cannot close authenticated persistent resume-once, target lowering/proofs, actual ledger phases, late external results, private handoff, live replay/race/consumption or release qualification.

The next genuine production patch begins after the owner supplies/reviews these binding/admission/failure/publication contracts and the revised ordinary design obtains its two substantive votes. Independent financial/work/byte expectations must then be frozen before execution. This is a dependency on missing semantics and interfaces, not a request for another user permission round.
