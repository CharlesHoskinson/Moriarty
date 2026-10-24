# Resource manifest report API

The four-file implementation and its October18 supplied-data result are
[adopted after corrected independent source/result review](../../results/sherlock-resource-manifest-20261018/ROOT-REPORTING-ADOPTION.json).
The earlier closed UIREQ-002/022/027 evidence remains applicable after explicit
source-pin reconciliation; historical receipts retain their original identities.
Whole UIREQ-013 remains open.

## Public exports and versions

`mori_core::sherlock::reference_slice` is a public host module. Its report data
structures use ordinary Rust types; the Core-only serialization implementations
live in `mori_core::sherlock::helper_machine::resource_report`. The existing v1
accounting, serializers and callers are preserved. The opt-in v2 identity and v3
manifest methods do not replace `resource_report()`.

| Caller | Method or free function | Return type |
| --- | --- | --- |
| `reference_slice::ReferenceSlice` | `resource_report()` | `Result<ReferenceResourceReport, ReferenceReportError>` |
| `reference_slice` | `cumulative_resource_report(&[&ReferenceSlice])` | Same v1 result |
| `ReferenceSlice` | `resource_report_with_identity()` | `Result<ReferenceIdentityResourceReport, ReferenceReportError>` |
| `reference_slice` | `cumulative_resource_report_with_identity(&[&ReferenceSlice])` | Same v2 result |
| `ReferenceSlice` | `resource_report_with_manifest()` | `Result<ReferenceManifestResourceReport, ReferenceReportError>` |
| `reference_slice` | `cumulative_resource_report_with_manifest(&[&ReferenceSlice])` | Same v3 result |
| `resource_report::ResourceReport` | `from_owner_with_identity(&HostObservation<'_>, &RunSummary)` | `IdentityResourceReport` |
| `resource_report::ResourceReport` | `from_owner_with_manifest(&HostObservation<'_>, &RunSummary)` | `ManifestResourceReport` |
| `helper_machine::core_action_consumer::CoreActionRun` | `resource_report_with_identity()` | `Result<resource_report::IdentityResourceReport, ConsumerError>` |
| `CoreActionRun` | `resource_report_with_manifest()` | `Result<resource_report::ManifestResourceReport, ConsumerError>` |
| `mori_core::sherlock_compile_artifact::draft_execution::DraftHostRun` | `resource_report_with_manifest()` | `Result<resource_report::ManifestResourceReport, ConsumerError>` |

The A methods reuse existing supplied-slice accounting and complete-state
continuity checks. Empty chunk lists and invalid accounting retain the existing
`ReferenceReportError` path. They execute no transitions. The B run methods
reuse `self.observe()?` and `self.completed.summary`; they do not run the owner
again. The direct `from_owner_*` functions project supplied observations and a
summary; they add no authentication or admission guarantee.

Source: [A reports](../../../crates/mori-core/src/sherlock/reference_slice.rs),
[B reports and serializers](../../../crates/mori-core/src/sherlock/helper_machine/resource_report.rs),
[Core run methods](../../../crates/mori-core/src/sherlock/helper_machine/core_action_consumer.rs),
[draft run method](../../../crates/mori-core/src/sherlock/compile_artifact/draft_execution.rs).

## Exact report boundaries

A v2 uses `moriarty-sherlock-a-resource-report/2`; its root has `format`,
`manifest`, `observations`. B v2 uses `moriarty-sherlock-b-resource-report/2`;
its root has `format`, `admission_identity`, `observations`. Each retains the
unchanged v1 report under `observations`.

A and B v3 use `moriarty-sherlock-a-resource-report/3` and
`moriarty-sherlock-b-resource-report/3`. Both roots have exactly `format`,
`manifest`, `observations`; the manifest format is
`moriarty-sherlock-resource-manifest/3`.

| Manifest | Exact fields |
| --- | --- |
| A `ReferenceResourceManifestV3` | `format`, `reference_schedule_id`, `source_definition_sha256`, `run_bounds`, `profile_domain`, `missing_required_identity`, `io_bytes`, `retained_profiles`, `unknown_metrics` |
| B `ResourceManifestV3` | `format`, `reference_schedule_id`, `admission_identity`, `supplied`, `missing_required_identity`, `io_bytes`, `retained_profiles`, `unknown_metrics` |

A's `run_bounds` records the first supplied start state's `initial_fuel` and
`semantic_stack_limit`. Its typed domain is UInt128/Bool/dedicated List/Tree,
top-first frames, charged K transitions, unmetered pure helpers and a zero-charge
terminal publication. The source-definition SHA is distinct from the four missing
A identities: `rule-inventory-digest`, `program-image-id`, `layout-version`,
`lowering-digest`.

B preserves the owner's carried `AdmissionIdentity` and supplied bounds/layout.
A selected membership digest is not renamed `program-image-id`; unselected
identities remain unselected. Profile IDs 0/1/2 map to K/Quint/B respectively;
other values produce `reference_schedule_id: null`. Required B producer gaps
remain `rule-inventory-digest`, `program-image-id`, `lowering-digest`.

Both manifests retain declarations for K, Quint and B. Their declared terminal
return charges are 0/1/0; pure-helper metering is respectively `unmetered`,
`algorithm-specific`, `charged-microsteps`. These declarations do not measure
another profile's run, equate fuel or identify a measured cost winner.
`unknown_metrics` collects the existing unmeasured metric names in producer
order, retaining multiplicity, and adds `io-bytes`.

## JSON writer and caller-owned output

```rust
pub fn write_report_json<R: serde::Serialize + ?Sized, W: std::io::Write + ?Sized>(
    report: &R,
    writer: &mut W,
) -> Result<ReportWriteObservation, ReportWriteError>;
```

The writer serializes once with `serde_json::to_vec`, then calls `write_all` with
those bytes. It writes no newline and does not flush, create a file, choose a
path or establish an overwrite/publication policy. Ordinary diagnostic
allocation/serialization remains unbounded by this API. `ReportWriteError` has
`Serialization(serde_json::Error)` and `Io(std::io::Error)` variants and derives
`Debug`; no `Display` or `std::error::Error` implementation is supplied here.

A success receipt has exactly `format`, `serialization_bytes`, `io_bytes`,
`measurement_scope`. Its format is
`moriarty-sherlock-report-write-observation/1`; both counters are decimal strings
for the complete compact JSON byte length. Scope is exactly
`complete-report-json-write-only;excludes-input-checkpoint-and-physical-io`.
The receipt exists only after `write_all` succeeds. On failure, no success receipt
is returned; a caller's writer may already have accepted a partial write.
Collection I/O in the report manifest remains explicitly unmeasured.

This example is a caller function over an already present slice. It performs
no SDK or machine run and introduces no new production API. Its `expect` calls
are example caller error policy; applications can match the declared errors.

```rust
use mori_core::sherlock::reference_slice::ReferenceSlice;
use mori_core::sherlock::helper_machine::resource_report::write_report_json;
use std::{fs::OpenOptions, io::Write, path::Path};

fn export_present_slice(slice: &ReferenceSlice, new_path: &Path) {
    let report = slice.resource_report_with_manifest().expect("supplied slice report");
    let mut sink = std::io::sink();
    let _sink_write = write_report_json(&report, &mut sink).expect("sink write");

    let mut file = OpenOptions::new().write(true).create_new(true)
        .open(new_path).expect("new report file");
    let _file_write = write_report_json(&report, &mut file).expect("report write");
    file.flush().expect("caller flush");
}
```

A successful sink write counts accepted logical bytes without filesystem I/O.
A successful File write/flush still does not measure physical device traffic,
input/checkpoint bytes or total host I/O. See the
[reporting explanation](../explanation/resource-reporting-boundaries.md) for
actual test scope and remaining requirements.
