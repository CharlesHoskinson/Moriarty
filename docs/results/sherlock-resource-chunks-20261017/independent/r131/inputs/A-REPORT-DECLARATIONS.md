# R130 proposed A reporting declaration contract (source-only, unadopted)

Normal input: existing ReferenceSlice on sherlock-k-a/1; no rule execution in reporting.
ReferenceSlice::resource_report(&self) -> Result<ReferenceResourceReport, ReferenceReportError>
cumulative_resource_report(chunks: &[&ReferenceSlice]) -> Result<ReferenceResourceReport, ReferenceReportError>

ResourceManifest (Serialize, Clone, Debug, Eq):
  format: &'static str = "moriarty-sherlock-resource-manifest/1"
  reference_schedule_id: Option<&'static str> (Some existing A PROFILE for A)
  profile_id: Option<u8> (None for A; no numeric A/B identity invented)
  source_definition_sha256: Option<&'static str> (Some existing K_SOURCE_SHA256 for A)
  layout_version: Option<(String,String)> (None for A)
  bounds: Option<SuppliedBounds> (None for A; A fuel/stack are not B BoundsC)
  unavailable_metadata: UnmeasuredGroup {
    status:"unmeasured", reason:"not-carried-by-this-reference-observation",
    metrics:["rule-inventory-digest","program-image-id","layout-version","bounds",
      "lowering-digest","frame-bytes-by-function","helper-scratch-bounds"] }
  measurement_scope: &'static str = "supplied-k-a-reference-slice-observations-only"

ReferenceResourceReport (Serialize, Clone, Debug, Eq):
  format: &'static str = "moriarty-sherlock-a-resource-report/1"
  manifest: ResourceManifest
  reference_work: Counter (existing exact u128 decimal-string serialization)
  recorded_reference_events: Counter (reference record count, NOT physical transitions)
  maximum_observed_logical_depth: usize (frames.len over every supplied boundary/record state)
  helper_work: UnmeasuredGroup {status:"unmeasured", reason:"k-a-pure-helpers-are-unmetered", metrics:["helper-work"]}
  helper_visits: UnmeasuredGroup {status:"unmeasured", reason:"k-a-pure-helpers-are-unmetered", metrics:["helper-visits"]}
  unmeasured: existing complete Unmeasured (all nine groups retained)

Unmeasured A construction reuses from_snapshot_only with these exact amendments:
- physical.metrics adds "physical-transitions" after "physical-dispatches" (all original metrics retained).
- host.metrics adds "peak-rss" after "peak-memory".
- overhead.metrics adds "preparation-bytes", "serialization-bytes" after original list.
- financial_target.metrics adds "ledger-fees" after original list.
- peaks = {status:"unmeasured",reason:"only-logical-frame-depth-is-observed",
    metrics:["peak-live-helper-tasks","peak-storage-bytes"]}.
- event_reconciliation = {status:"unmeasured",reason:"no-per-rule-or-allocation-ledger-report",
    metrics:["per-rule-counts","per-event-allocation-ledger"]}.
- remaining_nonhelper_reference_work = {status:"unmeasured",reason:"not-in-this-cumulative-report",
    metrics:["remaining-nonhelper-reference-work"]}.
All other groups/reasons/metrics preserve original resource_report.rs exactly.

ReferenceReportError (Clone, Copy, Debug, Eq; diagnostic data handling only):
  NoChunks
  Profile { chunk:usize }
  Continuity { chunk:usize, difference: existing compare::ContinuityDifference }
  BetweenChunks { previous:usize, next:usize, difference: existing compare::ContinuityDifference }
  PrefixWork { chunk:usize, position:usize }
  ChargeOverflow { chunk:usize, record:usize }
  EventCountOverflow { chunk:usize }

Report construction checks present nonempty chunk list, exact A profile per chunk,
complete existing continuity per chunk and prior.end == next.start through the
existing complete-state checker on an empty edge. It checks prefix_work length,
initial zero and every checked cumulative charge; aggregate sum/count uses checked
u128 arithmetic. It never evaluates a rule, treats charge as physical cost, estimates
bytes/time/RSS/fees, assumes source authenticity, or compares A/B equal fuel.
Technical boundaries are observer cuts only; their annotations are not semantic
transitions. The report covers supplied observations; it does not authenticate
producer, prove rules, admit untrusted input or close UIREQ013 globally.

R131 expected cells must be derived independently from the two frozen R73 input
packets and applicable original A source, not from candidate output. No actual
outputs have been produced. Primary one-vs-two predicate excludes chunk count
and technical boundary annotation from resource totals; full original slices
remain caller-owned and unchanged. Empty slices are present observations.
