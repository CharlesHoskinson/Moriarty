//! Public host grouping baseline for the existing frozen K-A reference events.
//!
//! This ordinary host baseline groups genuine Machine::step records; it does
//! not implement a new transition relation, helper clock or optimized machine.
//! Full snapshots and record order remain observable. A technical cut records
//! a grouping boundary only: it creates no financial commit, effect, suspension
//! or new machine Outcome. Preparation and full observation cloning are unmetered
//! host work. This is no bounded B schema, native payload or capacity claim.

use super::Machine;
use super::reference_types::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliceBoundaryKind {
    Terminal,
    TechnicalCut,
}

/// Position counts retained records, so zero names the slice's start state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SliceBoundary {
    pub position: usize,
    pub kind: SliceBoundaryKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SliceStop {
    Outcome(Outcome),
    TechnicalCut,
}

/// Host-only K-A ReferenceSlice observation. It carries only fields available
/// in the existing K-A machine. Source suspension and proposed B identities,
/// layouts and helpers remain separate from this host API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReferenceSlice {
    pub profile_id: &'static str,
    pub start: Snapshot,
    pub records: Vec<StepRecord>,
    /// Length records.len()+1, starts zero, accumulates each actual record.charge.
    pub prefix_work: Vec<u128>,
    pub boundaries: Vec<SliceBoundary>,
    pub end: Snapshot,
    pub stop: SliceStop,
}

/// Retain at most m reference events, including free publication as an event.
/// Normal comparison budgets are 1/2/4/8. Other host usize budgets merely choose
/// grouping length; no native width or resource policy is selected here.
///
/// Zero is an explicit observation-only cut: pending input yields identical
/// start/end, no records, prefix_work=[0], and a TechnicalCut at position zero.
/// Already published outcomes, including with m=0, yield an empty terminal
/// slice with the unchanged Outcome and a Terminal boundary at position zero.
///
/// Each next step is attempted in order on the live ordinary Machine, with its
/// own fuel/cap/publication behavior. There is no stepping ahead or replay.
/// Reaching a pending terminal control at the event budget leaves publication
/// for the next group; when the budget includes publication, its zero charge
/// is retained and the group stops immediately at that first published outcome.
pub fn advance_m(machine: &mut Machine, m: usize) -> ReferenceSlice {
    let start = machine.snapshot();
    let mut records = Vec::new();
    let mut prefix_work = vec![0];

    if start.outcome == Outcome::Pending {
        for _ in 0..m {
            let Some(record) = machine.step() else {
                break;
            };
            let cumulative =
                prefix_work.last().copied().expect("initial zero work") + u128::from(record.charge);
            let terminal = record.after.outcome != Outcome::Pending;
            prefix_work.push(cumulative);
            records.push(record);
            if terminal {
                break;
            }
        }
    }

    let end = machine.snapshot();
    let (stop, kind) = if end.outcome == Outcome::Pending {
        (SliceStop::TechnicalCut, SliceBoundaryKind::TechnicalCut)
    } else {
        (
            SliceStop::Outcome(end.outcome.clone()),
            SliceBoundaryKind::Terminal,
        )
    };
    let boundary = SliceBoundary {
        position: records.len(),
        kind,
    };
    ReferenceSlice {
        profile_id: PROFILE,
        start,
        records,
        prefix_work,
        boundaries: vec![boundary],
        end,
        stop,
    }
}

// Plain A observation data. This leaf also compiles in the existing std-only
// host source graph. Core's already-registered resource_report module supplies
// serialization adapters; no B/serde dependency is introduced here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceUnmeasuredStatus {
    Unmeasured,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReferenceUnmeasuredGroup {
    pub status: ReferenceUnmeasuredStatus,
    pub reason: &'static str,
    pub metrics: &'static [&'static str],
}
fn gap(reason: &'static str, metrics: &'static [&'static str]) -> ReferenceUnmeasuredGroup {
    ReferenceUnmeasuredGroup {
        status: ReferenceUnmeasuredStatus::Unmeasured,
        reason,
        metrics,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReferenceUnmeasured {
    pub peaks: ReferenceUnmeasuredGroup,
    pub physical: ReferenceUnmeasuredGroup,
    pub circuit: ReferenceUnmeasuredGroup,
    pub proof: ReferenceUnmeasuredGroup,
    pub host: ReferenceUnmeasuredGroup,
    pub overhead: ReferenceUnmeasuredGroup,
    pub financial_target: ReferenceUnmeasuredGroup,
    pub event_reconciliation: ReferenceUnmeasuredGroup,
    pub remaining_nonhelper_reference_work: ReferenceUnmeasuredGroup,
}
impl ReferenceUnmeasured {
    pub fn from_snapshot_only() -> Self {
        Self {
            peaks: gap(
                "final-snapshot-is-not-a-history",
                &[
                    "maximum-logical-depth",
                    "peak-live-helper-tasks",
                    "peak-storage-bytes",
                ],
            ),
            physical: gap(
                "no-physical-accounting-observation",
                &[
                    "physical-dispatches",
                    "frame-bytes",
                    "environment-bytes",
                    "node-bytes",
                    "allocator-reserved-bytes",
                ],
            ),
            circuit: gap(
                "no-circuit-observation",
                &[
                    "active-rows",
                    "padded-rows",
                    "active-cells",
                    "padded-cells",
                    "degree",
                    "table-bytes",
                    "code-bytes",
                ],
            ),
            proof: gap(
                "no-native-proof-observation",
                &[
                    "key-bytes",
                    "proof-bytes",
                    "witness-time",
                    "witness-memory",
                    "proving-time",
                    "proving-memory",
                    "verification-time",
                    "verification-memory",
                ],
            ),
            host: gap("no-host-measurement", &["wall-time", "peak-memory"]),
            overhead: gap(
                "no-overhead-measurement",
                &[
                    "preparation",
                    "wrappers",
                    "checkpoint-io",
                    "conversion",
                    "report-serialization",
                    "end-to-end-total",
                ],
            ),
            financial_target: gap(
                "no-financial-or-target-observation",
                &[
                    "target-identity",
                    "dust",
                    "financial-fees",
                    "service-fees",
                    "reserve-recovery-accounting",
                ],
            ),
            event_reconciliation: gap(
                "final-snapshot-and-summary-are-not-an-event-stream",
                &["per-rule-counts", "per-event-allocation-ledger"],
            ),
            remaining_nonhelper_reference_work: gap(
                "no-total-nonhelper-reference-budget",
                &["remaining-nonhelper-reference-work"],
            ),
        }
    }
}

/// Versioned observation manifest. Missing metadata remains named and unavailable;
/// a source-definition pin is not a rule-inventory or prepared-program digest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReferenceResourceManifest {
    pub format: &'static str,
    pub reference_schedule_id: Option<&'static str>,
    pub profile_id: Option<u8>,
    pub source_definition_sha256: Option<&'static str>,
    pub layout_version: Option<(String, String)>,
    pub bounds: Option<()>,
    pub unavailable_metadata: ReferenceUnmeasuredGroup,
    pub measurement_scope: &'static str,
}
impl ReferenceResourceManifest {
    fn reference_a() -> Self {
        Self {
            format: "moriarty-sherlock-resource-manifest/1",
            reference_schedule_id: Some(PROFILE),
            profile_id: None,
            source_definition_sha256: Some(K_SOURCE_SHA256),
            layout_version: None,
            bounds: None,
            unavailable_metadata: gap(
                "not-carried-by-this-reference-observation",
                &[
                    "rule-inventory-digest",
                    "program-image-id",
                    "layout-version",
                    "bounds",
                    "lowering-digest",
                    "frame-bytes-by-function",
                    "helper-scratch-bounds",
                ],
            ),
            measurement_scope: "supplied-k-a-reference-slice-observations-only",
        }
    }
}

/// Cumulative K-A observation totals; not physical/economic measurements.
/// Original complete slices remain caller-owned. No B clock is substituted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReferenceResourceReport {
    pub format: &'static str,
    pub manifest: ReferenceResourceManifest,
    pub reference_work: u128,
    pub recorded_reference_events: u128,
    pub maximum_observed_logical_depth: usize,
    pub helper_work: ReferenceUnmeasuredGroup,
    pub helper_visits: ReferenceUnmeasuredGroup,
    pub unmeasured: ReferenceUnmeasured,
}
impl ReferenceResourceReport {
    fn observed(work: u128, events: u128, depth: usize) -> Self {
        let mut unmeasured = ReferenceUnmeasured::from_snapshot_only();
        unmeasured.physical.metrics = &[
            "physical-dispatches",
            "physical-transitions",
            "frame-bytes",
            "environment-bytes",
            "node-bytes",
            "allocator-reserved-bytes",
        ];
        unmeasured.host.metrics = &["wall-time", "peak-memory", "peak-rss"];
        unmeasured.overhead.metrics = &[
            "preparation",
            "wrappers",
            "checkpoint-io",
            "conversion",
            "report-serialization",
            "end-to-end-total",
            "preparation-bytes",
            "serialization-bytes",
        ];
        unmeasured.financial_target.metrics = &[
            "target-identity",
            "dust",
            "financial-fees",
            "service-fees",
            "reserve-recovery-accounting",
            "ledger-fees",
        ];
        unmeasured.peaks = gap(
            "only-logical-frame-depth-is-observed",
            &["peak-live-helper-tasks", "peak-storage-bytes"],
        );
        unmeasured.event_reconciliation = gap(
            "no-per-rule-or-allocation-ledger-report",
            &["per-rule-counts", "per-event-allocation-ledger"],
        );
        unmeasured.remaining_nonhelper_reference_work = gap(
            "not-in-this-cumulative-report",
            &["remaining-nonhelper-reference-work"],
        );
        Self {
            format: "moriarty-sherlock-a-resource-report/1",
            manifest: ReferenceResourceManifest::reference_a(),
            reference_work: work,
            recorded_reference_events: events,
            maximum_observed_logical_depth: depth,
            helper_work: gap("k-a-pure-helpers-are-unmetered", &["helper-work"]),
            helper_visits: gap("k-a-pure-helpers-are-unmetered", &["helper-visits"]),
            unmeasured,
        }
    }
}

/// Data-accounting diagnostics on supplied slices, with no rule evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceReportError {
    NoChunks,
    Profile {
        chunk: usize,
    },
    Continuity {
        chunk: usize,
        difference: super::compare::ContinuityDifference,
    },
    BetweenChunks {
        previous: usize,
        next: usize,
        difference: super::compare::ContinuityDifference,
    },
    PrefixWork {
        chunk: usize,
        position: usize,
    },
    ChargeOverflow {
        chunk: usize,
        record: usize,
    },
    EventCountOverflow {
        chunk: usize,
    },
}

impl ReferenceSlice {
    /// The same cumulative report used for multiple technical chunks.
    /// A pending or empty present slice remains a valid reportable prefix.
    pub fn resource_report(&self) -> Result<ReferenceResourceReport, ReferenceReportError> {
        cumulative_resource_report(&[self])
    }
}

/// Report one ordered K-A observation from its present technical chunks.
/// Reuses the complete typed continuity checker and exact record charges;
/// grouping boundaries contribute no event, work, helper or financial cost.
/// Prefix sums are checked independently of the caller-supplied prefix_work.
/// No transition, rule charge, source authenticity or producer is validated.
/// Reporting inherits A's unbounded structural traversal; it creates no public
/// untrusted-input resource policy and does not claim equal-fuel cross profiles.
pub fn cumulative_resource_report(
    chunks: &[&ReferenceSlice],
) -> Result<ReferenceResourceReport, ReferenceReportError> {
    use super::compare::{TraceIdentity, TraceView, first_continuity_difference};
    let Some(first) = chunks.first() else {
        return Err(ReferenceReportError::NoChunks);
    };
    let identity = TraceIdentity {
        profile: PROFILE,
        source_sha256: K_SOURCE_SHA256,
    };
    let mut total_work = 0u128;
    let mut total_events = 0u128;
    let mut depth = first.start.frames.len();
    for (chunk, slice) in chunks.iter().enumerate() {
        if slice.profile_id != PROFILE {
            return Err(ReferenceReportError::Profile { chunk });
        }
        let trace = TraceView {
            identity,
            initial: &slice.start,
            records: &slice.records,
        };
        if let Some(difference) = first_continuity_difference(trace, &slice.end) {
            return Err(ReferenceReportError::Continuity { chunk, difference });
        }
        if chunk > 0 {
            let edge = TraceView {
                identity,
                initial: &chunks[chunk - 1].end,
                records: &[],
            };
            if let Some(difference) = first_continuity_difference(edge, &slice.start) {
                return Err(ReferenceReportError::BetweenChunks {
                    previous: chunk - 1,
                    next: chunk,
                    difference,
                });
            }
        }
        if slice.prefix_work.len() != slice.records.len() + 1 {
            return Err(ReferenceReportError::PrefixWork {
                chunk,
                position: slice.prefix_work.len(),
            });
        }
        if slice.prefix_work[0] != 0 {
            return Err(ReferenceReportError::PrefixWork { chunk, position: 0 });
        }
        depth = depth
            .max(slice.start.frames.len())
            .max(slice.end.frames.len());
        let mut prefix = 0u128;
        for (record, event) in slice.records.iter().enumerate() {
            prefix = prefix
                .checked_add(u128::from(event.charge))
                .ok_or(ReferenceReportError::ChargeOverflow { chunk, record })?;
            if slice.prefix_work[record + 1] != prefix {
                return Err(ReferenceReportError::PrefixWork {
                    chunk,
                    position: record + 1,
                });
            }
            total_work = total_work
                .checked_add(u128::from(event.charge))
                .ok_or(ReferenceReportError::ChargeOverflow { chunk, record })?;
            depth = depth
                .max(event.before.frames.len())
                .max(event.after.frames.len());
        }
        total_events = total_events
            .checked_add(slice.records.len() as u128)
            .ok_or(ReferenceReportError::EventCountOverflow { chunk })?;
    }
    Ok(ReferenceResourceReport::observed(
        total_work,
        total_events,
        depth,
    ))
}

/// Actual A semantic run inputs, not B BoundsC or host-memory admission caps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReferenceRunBounds {
    pub initial_fuel: u128,
    pub semantic_stack_limit: usize,
}

/// Description of the existing typed K-A profile, not a new execution schedule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReferenceProfileDomain {
    pub scalar_bits: u16,
    pub values: &'static str,
    pub frame_order: &'static str,
    pub work_clock: &'static str,
    pub pure_helper_metering: &'static str,
    pub terminal_publication_charge: u8,
}

/// Mandatory identity fields with no current actual A producer. This is a GAP,
/// not a measured zero, optional-field exemption or completed ResourceManifest.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReferenceIdentityGap {
    pub status: &'static str,
    pub reason: &'static str,
    pub fields: &'static [&'static str],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReferenceIdentityManifest {
    pub format: &'static str,
    pub reference_schedule_id: &'static str,
    pub source_definition_sha256: &'static str,
    pub run_bounds: ReferenceRunBounds,
    pub profile_domain: ReferenceProfileDomain,
    pub missing_required_identity: ReferenceIdentityGap,
}

/// Explicit successor: v1 accounting/JSON remains unchanged inside observations.
/// No program-image/layout/lowering identifier is synthesized from Debug or K text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReferenceIdentityResourceReport {
    pub format: &'static str,
    pub manifest: ReferenceIdentityManifest,
    pub observations: ReferenceResourceReport,
}

impl ReferenceSlice {
    /// Versioned identity-enriched report, separate from the unchanged v1 API.
    pub fn resource_report_with_identity(
        &self,
    ) -> Result<ReferenceIdentityResourceReport, ReferenceReportError> {
        cumulative_resource_report_with_identity(&[self])
    }
}

/// Reuse the original complete accounting/continuity implementation once.
/// Original run inputs come from the first actual supplied start state; v1
/// continuity already checks their preservation and the complete bound Program.
/// Reporting neither prepares a program image nor manufactures its identity.
pub fn cumulative_resource_report_with_identity(
    chunks: &[&ReferenceSlice],
) -> Result<ReferenceIdentityResourceReport, ReferenceReportError> {
    let observations = cumulative_resource_report(chunks)?;
    // The existing implementation already returned NoChunks for empty input.
    let start = &chunks[0].start;
    Ok(ReferenceIdentityResourceReport {
        format: "moriarty-sherlock-a-resource-report/2",
        manifest: ReferenceIdentityManifest {
            format: "moriarty-sherlock-resource-manifest/2",
            reference_schedule_id: PROFILE,
            source_definition_sha256: K_SOURCE_SHA256,
            run_bounds: ReferenceRunBounds {
                initial_fuel: start.initial_fuel,
                semantic_stack_limit: start.stack_limit,
            },
            profile_domain: ReferenceProfileDomain {
                scalar_bits: 128,
                values: "uint128-bool-dedicated-list-tree",
                frame_order: "top-first",
                work_clock: "charged-k-machine-transitions",
                pure_helper_metering: "unmetered",
                terminal_publication_charge: 0,
            },
            missing_required_identity: ReferenceIdentityGap {
                status: "gap",
                reason: "no-selected-producer-on-the-public-a-observation",
                fields: &[
                    "rule-inventory-digest",
                    "program-image-id",
                    "layout-version",
                    "lowering-digest",
                ],
            },
        },
        observations,
    })
}

/// Retained schedule declarations, not measurements of another profile's run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedResourceProfile {
    pub reference_schedule_id: &'static str,
    pub terminal_return_charge: u8,
    pub pure_helper_metering: &'static str,
    pub qualification: &'static str,
}
pub const RETAINED_RESOURCE_PROFILES: &[RetainedResourceProfile] = &[
    RetainedResourceProfile {
        reference_schedule_id: "sherlock-k-a/1",
        terminal_return_charge: 0,
        pure_helper_metering: "unmetered",
        qualification: "retained-k-a-clock;no-cross-profile-fuel-equivalence",
    },
    RetainedResourceProfile {
        reference_schedule_id: "sherlock-quint-a/1",
        terminal_return_charge: 1,
        pure_helper_metering: "algorithm-specific",
        qualification: "retained-quint-a-terminal-return-tick;not-an-observation-of-this-run",
    },
    RetainedResourceProfile {
        reference_schedule_id: "sherlock-helper-microstep/1",
        terminal_return_charge: 0,
        pure_helper_metering: "charged-microsteps",
        qualification: "distinct-helper-schedule;no-cross-profile-fuel-equivalence-or-measured-cost-winner",
    },
];

/// Resource-report selection metadata. Required missing identities stay gaps;
/// neither the source definition nor a diagnostic display supplies those IDs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReferenceResourceManifestV3 {
    pub format: &'static str,
    pub reference_schedule_id: &'static str,
    pub source_definition_sha256: &'static str,
    pub run_bounds: ReferenceRunBounds,
    pub profile_domain: ReferenceProfileDomain,
    pub missing_required_identity: ReferenceIdentityGap,
    pub io_bytes: ReferenceUnmeasuredGroup,
    pub retained_profiles: &'static [RetainedResourceProfile],
    pub unknown_metrics: Vec<&'static str>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReferenceManifestResourceReport {
    pub format: &'static str,
    pub manifest: ReferenceResourceManifestV3,
    pub observations: ReferenceResourceReport,
}
impl ReferenceSlice {
    /// Opt-in selected manifest and explicit I/O status, preserving v1/v2.
    pub fn resource_report_with_manifest(
        &self,
    ) -> Result<ReferenceManifestResourceReport, ReferenceReportError> {
        cumulative_resource_report_with_manifest(&[self])
    }
}

/// Use the unchanged v1 cumulative charge/continuity producer once through v2.
/// Technical cuts add no charges. This function performs no I/O or evaluation.
pub fn cumulative_resource_report_with_manifest(
    chunks: &[&ReferenceSlice],
) -> Result<ReferenceManifestResourceReport, ReferenceReportError> {
    let ReferenceIdentityResourceReport {
        manifest,
        observations,
        ..
    } = cumulative_resource_report_with_identity(chunks)?;
    let io_bytes = gap(
        "reference-slices-carry-no-io-byte-observation",
        &["io-bytes"],
    );
    let mut unknown_metrics = Vec::new();
    unknown_metrics.extend_from_slice(observations.helper_work.metrics);
    unknown_metrics.extend_from_slice(observations.helper_visits.metrics);
    let u = observations.unmeasured;
    for group in [
        u.peaks,
        u.physical,
        u.circuit,
        u.proof,
        u.host,
        u.overhead,
        u.financial_target,
        u.event_reconciliation,
        u.remaining_nonhelper_reference_work,
    ] {
        unknown_metrics.extend_from_slice(group.metrics);
    }
    unknown_metrics.extend_from_slice(io_bytes.metrics);
    Ok(ReferenceManifestResourceReport {
        format: "moriarty-sherlock-a-resource-report/3",
        manifest: ReferenceResourceManifestV3 {
            format: "moriarty-sherlock-resource-manifest/3",
            reference_schedule_id: manifest.reference_schedule_id,
            source_definition_sha256: manifest.source_definition_sha256,
            run_bounds: manifest.run_bounds,
            profile_domain: manifest.profile_domain,
            missing_required_identity: manifest.missing_required_identity,
            io_bytes,
            retained_profiles: RETAINED_RESOURCE_PROFILES,
            unknown_metrics,
        },
        observations,
    })
}

#[cfg(test)]
#[path = "reference_slice_tests.rs"]
mod tests;
