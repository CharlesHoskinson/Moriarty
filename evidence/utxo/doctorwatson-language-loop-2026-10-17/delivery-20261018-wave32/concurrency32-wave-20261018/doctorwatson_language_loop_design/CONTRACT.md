# Proposed normal API contract — unadopted

No new financial semantics, authentication, source admission, authority, identity recipe, replay rule, resource waiver, persistence failure rule or new loop is selected here. New API/command names are proposal text only. No production stubs are supplied.

## Established data that must survive

The complete existing public waiting report has exactly these10 root fields: `status`, `advanced`, `segment`, `continuation`, `attempted`, `committed`, `residual`, `qualification`, `authentication`, `consumption`.

- `status` stays `waiting`; `advanced` remains the original Boolean. `segment` stays either the complete original StageSegment or null.
- A segment retains action, frame_pre, frame_post and Execution. Execution retains semantics/action/pre/post/steps/effects, including every complete snapshot, raw statement, Step value/index and ordered effect. No field/row/step is selected away.
- Continuation retains exactly format/program_hash/plan_hash/action/site/captures/logical/boundary. Logical retains profile/mandatory cap (including null)/committed. Captures remain ordered; every declared/preserved balance, state and liability row stays present, including zero rows.
- Root attempted, committed and residual remain the recorded values. Reopening must not turn old attempted5 into0, pretend it executed5 again, recompute financials or append a fabricated observation.
- Labels remain local-reference-stage / none / untracked as actually stored by the supported original report. They do not authenticate the file.

This reproduces the established complete normal output shape, not a new durable recovery/history DTO. No field is added to old Continuation or old report.

## A — full public waiting report, one file (recommended)

**Carrier:** the exact existing public JSON waiting report, obtained from actual `simulate` stdout. It has no new wrapper, checkpoint format/version, identity or bookkeeping fields. All original supported fields remain in one document. Existing exported values fit the existing1 MiB public JSON/export profile; this proposal supplies no increased byte/depth/node budget.

**Normal proposed public journey:**

```sh
mori simulate SOURCE --action ACTION --scenario SCENARIO > waiting-report.json
mori reopen-waiting waiting-report.json
```

The first command is existing. The second is **proposed and unimplemented**. Ordinary successful reopening reads an actual saved full report without an event or scenario, returns the same full JSON value, and performs no financial transition. Output formatting would use the existing public JSON exporter. Reopening does not select or call source admission; a caller that later resumes extracts the inner Continuation and uses the existing source+event resume path, with its existing behavior unchanged. Document import is not permission to skip that path.

**Precise proposed API surface:**

```rust
// Runtime view only; this is not a new serialized object.
pub struct WaitingView<'a> {
    pub advanced: bool,
    pub segment: Option<&'a StageSegment>,
    pub continuation: &'a Continuation,
    pub attempted: UInt,
    pub residual: &'a [Liability],
}

pub fn export_waiting(view: WaitingView<'_>) -> Result<String, Error>;
pub fn reopen_waiting(text: &str) -> Result<WaitingDocument, Error>;

// New owned document type; no public trust/admission assertion is implied.
impl WaitingDocument {
    pub fn report(&self) -> &serde_json::Value;
    pub fn continuation(&self) -> &Continuation;
}
```

`WaitingView` fields are a borrow of the existing Waiting variant's exact five fields, not a second evaluator. `committed` and the three labels follow the existing `outcome_report` projection. `WaitingDocument` retains the full report tree and the typed inner continuation without interpreting its segment as executable history. It does not require general Deserialize on StageOutcome, StageSegment or Await-bearing Statement. This API declaration is not an implementation or adopted decoder policy.

**Missing actual caller:** new public CLI dispatch before invoking any source simulator; there is currently no `reopen-waiting` command. Existing `simulate::print_stage_result` can continue printing normal reports unchanged. No new production writer is required by A's proposed normal carrier: shell redirection saves existing stdout. This is honest document export, not a promise of transactional file publication, crash durability or a persisted financial commit.

**Decision still required:** root must adopt the normal report-as-carrier / re-emit-original operation meaning, public spelling and exact read/error/shape rules. Their choice is not implied merely by a JSON renderer existing. This document does not design authentication or admission. All non-normal/error choices remain unspecified for their designated owners, not silently mapped to stage rejection with invented counters.

## B — existing bare Continuation plus complete waiting-report sidecar

**Carriers:** file1 is the unchanged existing Continuation wire; file2 is the unchanged whole waiting report from A. No report field is discarded. This uses the familiar existing resume input directly, while retaining all original report-only fields in its sidecar.

**Precise proposed API surface:**

```rust
pub struct WaitingFiles {
    pub continuation_utf8: String,
    pub report_utf8: String,
}

pub fn export_waiting_files(view: WaitingView<'_>) -> Result<WaitingFiles, Error>;
pub fn reopen_waiting_files(
    continuation_text: &str,
    report_text: &str,
) -> Result<WaitingDocument, Error>;
```

The existing `decode::continuation` supplies typed file1 decoding; existing public JSON parse/export supplies file2 values. This does not invent new metadata tying them together. Current specs do not select pair publication, association, discrepancy precedence or failure outcomes. Those unresolved choices make B larger than A for a complete all-fields import, even though direct resume already accepts file1. Neither sidecar omission nor report-field projection is allowed to masquerade as complete preservation. No new pair handling is implemented.

## Why an implementation is not uniquely determined now

Existing clauses define old waiting output and old event-driven continuation resume. They do not define any event-free report import command. A and B both retain the entire established field set, yet select different public carriers and operation semantics. A typed report decoder, opaque JSON document viewer, or full source-bound reopen are different consumer claims. This task proposes A's explicit limited document claim; it does not choose source-bound validation by deleting R5 from resume. Root must select the intended ordinary contract before a real patch. Full recovery still needs a separately versioned complete pending/closed state and all original requirements; neither A nor B is that missing transition.
