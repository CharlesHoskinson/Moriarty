//! Complete C6 encodings of one actual fixed-owner observation.
//!
//! This adapter calls `record_step` once and encodes its saved boundaries with
//! the existing compact HOST codec. It does not run a loop, serialize a new
//! format, reconstruct an event, or admit a caller-supplied record as actual.
//! The private fields establish only which host call produced this value.
//! They do not establish circuit transition/source authentication, StepInv,
//! financial support, persistence, range correspondence or proof authority.
//!
//! Codec/layout errors happen AFTER the actual observation. They retain that
//! complete observation and any completed pre-encoding. They never roll back
//! the owner, retry it, or replace its result with a successful empty capture.
//! Caller limits and the original retained context are used without widening.
//! As in `capture_host`, ext capacities have no HostSnapshot counterpart;
//! their original caller binding is a retained premise, not proved here.
//!
//! The failure archive at the end of this file is a separate diagnostic text
//! export of borrowed values. It is not a C6 codec format and has no decoder.

use std::fmt;
use std::io::{self, Write};

use super::c6_phys::C6CodecBudgets;
use super::c6_phys_host::{C6HostConfig, C6HostError, C6HostLimits, encode_host};
use super::c6_phys_state::{C6StateError, retained_layout_agrees_with_host};
use super::complete_observation::{
    AdmissionSnapshot, CaptureStepError, CompleteStepRecord, HostSnapshot, ObservedStep,
    ProgramSnapshot, record_step,
};
use super::shared::owner_ext_type_builder::{CtorDeclSrc, CtorFieldSrc, TypeDeclSrc, TypeExprSrc};
use super::shared::{Admitted, HostViolation, LayoutSchemaC, MachineImage};

/// Encoded boundaries and the exact observation from one actual owner call.
/// There is deliberately no public constructor from a supplied snapshot pair.
pub struct C6CompleteStepCapture {
    observed: ObservedStep,
    config: C6HostConfig,
    before_lanes: Vec<u128>,
    after_lanes: Vec<u128>,
}

impl C6CompleteStepCapture {
    pub fn observed(&self) -> &ObservedStep {
        &self.observed
    }
    pub fn config(&self) -> &C6HostConfig {
        &self.config
    }
    pub fn before_lanes(&self) -> &[u128] {
        &self.before_lanes
    }
    pub fn after_lanes(&self) -> &[u128] {
        &self.after_lanes
    }

    /// Transfer the original raw observation and both complete codec outputs.
    /// A consumer must preserve its own storage/continuity/proof obligations.
    pub fn into_parts(self) -> (ObservedStep, C6HostConfig, Vec<u128>, Vec<u128>) {
        (
            self.observed,
            self.config,
            self.before_lanes,
            self.after_lanes,
        )
    }
}

impl C6CompleteStepCapture {
    /// Copy five counters of THIS capture's own saved BEFORE image (Event or
    /// Terminal) and the native checked results over those same copies. O(1);
    /// no owner, StepInv, codec or archive call. Data only, not a certificate.
    pub fn before_counter_observation(&self) -> C6BeforeCounterObservation {
        observe_before_counters(&self.observed)
    }
}

/// Read-only copy of `initial_fuel`, `remaining_work`, `consumed_work`,
/// `helper_work` and `reference_cursor` from one actual saved BEFORE image,
/// plus Rust's `checked_add`/`checked_sub` results over exactly those copies.
/// Only produced from a genuine capture/in-flight value; no public constructor.
/// It does not evaluate full SI03 (bounds, helper_visits, value_slots unread),
/// establish StepInv, reachability, dispatch, Rust/Fin or C6 correspondence,
/// or carry proof authority. A match describes this one saved image only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6BeforeCounterObservation {
    initial_fuel: u128,
    remaining_work: u128,
    consumed_work: u128,
    helper_work: u128,
    reference_cursor: u128,
    remaining_checked_add_consumed: Option<u128>,
    consumed_checked_sub_helper: Option<u128>,
}

impl C6BeforeCounterObservation {
    pub fn initial_fuel(&self) -> u128 {
        self.initial_fuel
    }
    pub fn remaining_work(&self) -> u128 {
        self.remaining_work
    }
    pub fn consumed_work(&self) -> u128 {
        self.consumed_work
    }
    pub fn helper_work(&self) -> u128 {
        self.helper_work
    }
    pub fn reference_cursor(&self) -> u128 {
        self.reference_cursor
    }
    /// Raw `remaining_work.checked_add(consumed_work)`; `None` is overflow.
    pub fn remaining_checked_add_consumed(&self) -> Option<u128> {
        self.remaining_checked_add_consumed
    }
    /// Raw `consumed_work.checked_sub(helper_work)`; `None` is underflow.
    pub fn consumed_checked_sub_helper(&self) -> Option<u128> {
        self.consumed_checked_sub_helper
    }
    /// The SI03 sum conjunct only: the checked sum is `Some(initial_fuel)`.
    pub fn fuel_sum_matches_initial(&self) -> bool {
        self.remaining_checked_add_consumed == Some(self.initial_fuel)
    }
    /// The SI04 relation only: the checked difference is `Some(reference_cursor)`.
    pub fn cursor_difference_matches_reference(&self) -> bool {
        self.consumed_checked_sub_helper == Some(self.reference_cursor)
    }
    /// Observed `remaining_work > 0`; not evidence of actual nullary dispatch.
    pub fn remaining_work_is_positive(&self) -> bool {
        self.remaining_work > 0
    }
}

// Private: reads only the saved BEFORE image of an actual observation.
fn observe_before_counters(observed: &ObservedStep) -> C6BeforeCounterObservation {
    let before = match observed {
        ObservedStep::Event(record) => &record.before,
        ObservedStep::Terminal { before, .. } => before,
    };
    let image = &before.image;
    let (initial_fuel, remaining_work, consumed_work, helper_work, reference_cursor) = (
        image.initial_fuel,
        image.remaining_work,
        image.consumed_work,
        image.helper_work,
        image.reference_cursor,
    );
    C6BeforeCounterObservation {
        initial_fuel,
        remaining_work,
        consumed_work,
        helper_work,
        reference_cursor,
        remaining_checked_add_consumed: remaining_work.checked_add(consumed_work),
        consumed_checked_sub_helper: consumed_work.checked_sub(helper_work),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C6CaptureSide {
    Before,
    After,
}

#[derive(Debug)]
pub enum C6BoundaryCaptureFailure {
    RetainedLayout(C6StateError),
    Codec(C6HostError),
}

pub enum C6CompleteStepCaptureError {
    /// Unmodified owner/capture error, including any actual saved boundaries.
    Owner(CaptureStepError),
    /// The actual step has already occurred. No retry or rollback is implied.
    Materialization {
        side: C6CaptureSide,
        failure: C6BoundaryCaptureFailure,
        observed: Box<ObservedStep>,
        config: C6HostConfig,
        completed_before: Option<Vec<u128>>,
    },
}

impl fmt::Debug for C6CompleteStepCaptureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Owner(_) => f.write_str("Owner(CaptureStepError; original payload retained)"),
            Self::Materialization {
                side,
                failure,
                completed_before,
                ..
            } => f
                .debug_struct("Materialization")
                .field("side", side)
                .field("failure", failure)
                .field("has_completed_before", &completed_before.is_some())
                .finish(),
        }
    }
}

fn encode_boundary(
    host: &HostSnapshot,
    retained: &LayoutSchemaC,
    config: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<Vec<u128>, C6BoundaryCaptureFailure> {
    retained_layout_agrees_with_host(retained, host)
        .map_err(C6BoundaryCaptureFailure::RetainedLayout)?;
    encode_host(host, config, limits).map_err(C6BoundaryCaptureFailure::Codec)
}

// Private so only the actual fixed-owner entry point can publish a success.
fn encode_observed(
    observed: ObservedStep,
    retained: &LayoutSchemaC,
    budgets: C6CodecBudgets,
    limits: &C6HostLimits,
) -> Result<C6CompleteStepCapture, C6CompleteStepCaptureError> {
    let config = C6HostConfig::from_retained_layout(retained, budgets);
    let (before_raw, after_raw) = match &observed {
        ObservedStep::Event(record) => (&record.before, &record.after),
        ObservedStep::Terminal { before, after } => (before, after),
    };
    let before_lanes = match encode_boundary(before_raw, retained, &config, limits) {
        Ok(lanes) => lanes,
        Err(failure) => {
            return Err(C6CompleteStepCaptureError::Materialization {
                side: C6CaptureSide::Before,
                failure,
                observed: Box::new(observed),
                config,
                completed_before: None,
            });
        }
    };
    let after_lanes = match encode_boundary(after_raw, retained, &config, limits) {
        Ok(lanes) => lanes,
        Err(failure) => {
            return Err(C6CompleteStepCaptureError::Materialization {
                side: C6CaptureSide::After,
                failure,
                observed: Box::new(observed),
                config,
                completed_before: Some(before_lanes),
            });
        }
    };
    Ok(C6CompleteStepCapture {
        observed,
        config,
        before_lanes,
        after_lanes,
    })
}

/// Observe the bound pair with the existing owner exactly once, then encode its
/// OWN saved before/after. Financial and other owner refusals remain unchanged.
/// The pair may have advanced even when the returned error is Materialization.
pub fn record_c6_complete_step(
    admitted: &Admitted,
    image: &mut MachineImage,
    retained: &LayoutSchemaC,
    budgets: C6CodecBudgets,
    limits: &C6HostLimits,
) -> Result<C6CompleteStepCapture, C6CompleteStepCaptureError> {
    let observed = record_step(admitted, image).map_err(C6CompleteStepCaptureError::Owner)?;
    encode_observed(observed, retained, budgets, limits)
}

// ---------- lossless failure archive (M2, component A) ----------
//
// The archive BORROWS the caller's value: the original first cause and every
// owned payload stay with the caller whatever happens here. There is no owner
// retry, rollback, `encode_host` re-validation, or fabricated after/event; an
// unavailable value is written as an explicit `unavailable:`/`absent:` line.
// Each line is `key=value`: keys never contain `=`, and each value is one
// compact derived `Debug` rendering (every field, no custom summary), a lane
// list, or a `TypeExprSrc` prefix form. `HostSnapshot`/`ProgramSnapshot` are
// destructured so `TypeDeclSrc` never reaches an unreviewed formatter, and
// `TypeExprSrc` is written by an iterative, node-capped walk. A missing
// `end=complete` trailer marks a prefix. Caps are diagnostic archive caps only.
// The former premise leaves (FinUnsupportedOp, ExtCapacitiesC, C6PhysError with
// PushSelection, C6StateError with C6StateChild, C6HostChild with C6HostError)
// derive Debug over integers, `&'static str` and each other. Derived Debug is
// not a stable Rust format: the text is toolchain-bound and no parser exists.
// `CompleteLogical` is logical only: no fsync, rehash or durability.

pub const C6_FAILURE_ARCHIVE_FORMAT: &str = "mori-c6-failure-archive/2";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6ArchiveBudgets {
    /// Hard cap on bytes handed to the caller's writer.
    pub max_bytes: u64,
    /// Cumulative `TypeExprSrc` nodes over every archived template.
    pub max_type_expr_nodes: u64,
}

#[derive(Debug)]
pub enum C6ArchiveStatus {
    /// Every available value and the trailer were accepted and `flush` returned Ok.
    CompleteLogical,
    /// The byte cap refused a write; a trailerless prefix may exist.
    TruncatedBytes,
    /// The type-expression node cap was reached; a trailerless prefix may exist.
    TruncatedTypeExprNodes,
    /// A sink write or a formatter failed; a trailerless prefix may exist.
    WriteFailed(io::Error),
    /// The trailer was accepted but the final `flush` failed. Not complete.
    FlushFailed(io::Error),
}

#[derive(Debug)]
pub struct C6ArchiveReport {
    pub status: C6ArchiveStatus,
    /// Bytes the sink's `write` accepted. No flush follows a stop; never durable.
    pub bytes_accepted: u64,
}

impl C6ArchiveReport {
    pub fn is_complete_logical(&self) -> bool {
        matches!(self.status, C6ArchiveStatus::CompleteLogical)
    }
}

/// Write every available value of the borrowed error. Call BEFORE formatting,
/// dropping or exiting; the error is never taken, so it survives any status.
pub fn archive_c6_complete_step_error<W: Write + ?Sized>(
    error: &C6CompleteStepCaptureError,
    sink: &mut W,
    budgets: C6ArchiveBudgets,
) -> C6ArchiveReport {
    archive(sink, budgets, Subject::Error(error))
}

/// Same contract for a genuine capture retained by a later stream failure.
pub fn archive_c6_complete_step_capture<W: Write + ?Sized>(
    capture: &C6CompleteStepCapture,
    sink: &mut W,
    budgets: C6ArchiveBudgets,
) -> C6ArchiveReport {
    archive(sink, budgets, Subject::Capture(capture))
}

enum Subject<'a> {
    Error(&'a C6CompleteStepCaptureError),
    Capture(&'a C6CompleteStepCapture),
}

enum Stop {
    Io(io::Error),
    TypeNodes,
}
impl From<io::Error> for Stop {
    fn from(e: io::Error) -> Self {
        Stop::Io(e)
    }
}

struct Budgeted<'a, W: Write + ?Sized> {
    inner: &'a mut W,
    written: u64,
    cap: u64,
    refused: bool,
}

impl<W: Write + ?Sized> Write for Budgeted<'_, W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() as u64 > self.cap.saturating_sub(self.written) {
            self.refused = true;
            return Err(io::Error::new(io::ErrorKind::Other, "c6 archive byte cap"));
        }
        let n = self.inner.write(buf)?.min(buf.len());
        self.written += n as u64;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

fn archive<W: Write + ?Sized>(
    sink: &mut W,
    budgets: C6ArchiveBudgets,
    subject: Subject<'_>,
) -> C6ArchiveReport {
    let C6ArchiveBudgets {
        max_bytes,
        max_type_expr_nodes,
    } = budgets;
    let mut w = Budgeted {
        inner: sink,
        written: 0,
        cap: max_bytes,
        refused: false,
    };
    let mut left = max_type_expr_nodes;
    let status = match write_subject(&mut w, &mut left, subject) {
        Ok(()) => match w.flush() {
            Ok(()) => C6ArchiveStatus::CompleteLogical,
            Err(e) => C6ArchiveStatus::FlushFailed(e),
        },
        Err(Stop::TypeNodes) => C6ArchiveStatus::TruncatedTypeExprNodes,
        Err(Stop::Io(_)) if w.refused => C6ArchiveStatus::TruncatedBytes,
        Err(Stop::Io(e)) => C6ArchiveStatus::WriteFailed(e),
    };
    C6ArchiveReport {
        status,
        bytes_accepted: w.written,
    }
}

fn write_subject<W: Write + ?Sized>(
    w: &mut Budgeted<'_, W>,
    left: &mut u64,
    subject: Subject<'_>,
) -> Result<(), Stop> {
    writeln!(w, "format={C6_FAILURE_ARCHIVE_FORMAT}")?;
    match subject {
        Subject::Capture(capture) => {
            let C6CompleteStepCapture {
                observed,
                config,
                before_lanes,
                after_lanes,
            } = capture;
            writeln!(w, "subject=complete_capture")?;
            observed_step(w, left, "observed", observed)?;
            host_config(w, config)?;
            lanes(w, "before_lanes", before_lanes)?;
            lanes(w, "after_lanes", after_lanes)?;
        }
        Subject::Error(C6CompleteStepCaptureError::Owner(cause)) => owner(w, left, cause)?,
        Subject::Error(C6CompleteStepCaptureError::Materialization {
            side,
            failure,
            observed,
            config,
            completed_before,
        }) => {
            writeln!(w, "subject=materialization")?;
            writeln!(w, "materialization.side={side:?}")?;
            writeln!(w, "materialization.failure={failure:?}")?;
            observed_step(w, left, "observed", observed)?;
            host_config(w, config)?;
            match completed_before {
                Some(values) => lanes(w, "completed_before", values)?,
                None => writeln!(w, "completed_before=absent")?,
            }
        }
    }
    writeln!(w, "end=complete")?;
    Ok(())
}

fn owner<W: Write + ?Sized>(
    w: &mut Budgeted<'_, W>,
    left: &mut u64,
    cause: &CaptureStepError,
) -> Result<(), Stop> {
    match cause {
        CaptureStepError::BeforeObservation(v) => {
            writeln!(w, "subject=owner.before_observation")?;
            host_violation(w, "owner.violation", v)?;
            writeln!(w, "owner.before=unavailable:before_observation_failed")?;
        }
        CaptureStepError::Step {
            violation: v,
            before,
            after,
        } => {
            writeln!(w, "subject=owner.step")?;
            host_violation(w, "owner.violation", v)?;
            host(w, left, "owner.before", before)?;
            match after {
                Ok(a) => host(w, left, "owner.after", a)?,
                Err(av) => {
                    writeln!(w, "owner.after=unavailable:capture_failed")?;
                    host_violation(w, "owner.after.violation", av)?;
                }
            }
        }
        CaptureStepError::AfterObservation {
            violation: v,
            before,
            event,
        } => {
            writeln!(w, "subject=owner.after_observation")?;
            host_violation(w, "owner.violation", v)?;
            host(w, left, "owner.before", before)?;
            match event {
                Some(e) => writeln!(w, "owner.event={e:?}")?,
                None => writeln!(w, "owner.event=absent:terminal_disposition")?,
            }
            writeln!(w, "owner.after=unavailable:after_observation_failed")?;
        }
    }
    Ok(())
}

fn host_violation<W: Write + ?Sized>(
    w: &mut Budgeted<'_, W>,
    label: &str,
    v: &HostViolation,
) -> Result<(), Stop> {
    let HostViolation {
        kind,
        clause,
        rule,
        locator,
    } = v;
    writeln!(w, "{label}.kind={kind:?}")?;
    writeln!(w, "{label}.clause={clause:?}")?;
    writeln!(w, "{label}.rule={rule:?}")?;
    writeln!(w, "{label}.locator={locator:?}")?;
    Ok(())
}

fn observed_step<W: Write + ?Sized>(
    w: &mut Budgeted<'_, W>,
    left: &mut u64,
    label: &str,
    observed: &ObservedStep,
) -> Result<(), Stop> {
    match observed {
        ObservedStep::Event(CompleteStepRecord {
            before,
            event,
            after,
        }) => {
            writeln!(w, "{label}=Event")?;
            host(w, left, &format!("{label}.before"), before)?;
            writeln!(w, "{label}.event={event:?}")?;
            host(w, left, &format!("{label}.after"), after)
        }
        ObservedStep::Terminal { before, after } => {
            writeln!(w, "{label}=Terminal")?;
            host(w, left, &format!("{label}.before"), before)?;
            writeln!(w, "{label}.event=absent:terminal")?;
            host(w, left, &format!("{label}.after"), after)
        }
    }
}

fn host<W: Write + ?Sized>(
    w: &mut Budgeted<'_, W>,
    left: &mut u64,
    label: &str,
    snapshot: &HostSnapshot,
) -> Result<(), Stop> {
    let HostSnapshot { admission, image } = snapshot;
    let AdmissionSnapshot {
        identity,
        bounds,
        view,
        layout,
    } = admission;
    writeln!(w, "{label}.admission.identity={identity:?}")?;
    writeln!(w, "{label}.admission.bounds={bounds:?}")?;
    writeln!(w, "{label}.admission.layout={layout:?}")?;
    program(w, left, label, view)?;
    writeln!(w, "{label}.image={image:?}")?;
    Ok(())
}

fn program<W: Write + ?Sized>(
    w: &mut Budgeted<'_, W>,
    left: &mut u64,
    label: &str,
    p: &ProgramSnapshot,
) -> Result<(), Stop> {
    let ProgramSnapshot {
        entry,
        expressions,
        occurrences,
        occurrence_paths,
        children,
        functions,
        parameters,
        arms,
        patterns,
        pattern_children,
        types,
        type_args,
        constructors,
        ctor_fields,
        decl_param_counts,
        decl_ctor_counts,
        decl_templates,
        plan_nodes,
        plan_cases,
        field_paths,
        constants,
        constant_fields,
        names,
    } = p;
    let flat: [(&str, &dyn fmt::Debug); 22] = [
        ("entry", entry),
        ("expressions", expressions),
        ("occurrences", occurrences),
        ("occurrence_paths", occurrence_paths),
        ("children", children),
        ("functions", functions),
        ("parameters", parameters),
        ("arms", arms),
        ("patterns", patterns),
        ("pattern_children", pattern_children),
        ("types", types),
        ("type_args", type_args),
        ("constructors", constructors),
        ("ctor_fields", ctor_fields),
        ("decl_param_counts", decl_param_counts),
        ("decl_ctor_counts", decl_ctor_counts),
        ("plan_nodes", plan_nodes),
        ("plan_cases", plan_cases),
        ("field_paths", field_paths),
        ("constants", constants),
        ("constant_fields", constant_fields),
        ("names", names),
    ];
    for (name, value) in flat.iter() {
        writeln!(w, "{label}.admission.view.{name}={value:?}")?;
    }
    writeln!(
        w,
        "{label}.admission.view.decl_templates.len={}",
        decl_templates.len()
    )?;
    for (i, decl) in decl_templates.iter().enumerate() {
        let TypeDeclSrc {
            name,
            parameters,
            ctors,
        } = decl;
        let at = format!("{label}.admission.view.decl_templates.{i}");
        writeln!(w, "{at}.name={name:?}")?;
        writeln!(w, "{at}.parameters={parameters:?}")?;
        writeln!(w, "{at}.ctors.len={}", ctors.len())?;
        for (j, ctor) in ctors.iter().enumerate() {
            let CtorDeclSrc { name, fields } = ctor;
            writeln!(w, "{at}.ctor.{j}.name={name:?}")?;
            writeln!(w, "{at}.ctor.{j}.fields.len={}", fields.len())?;
            for (k, field) in fields.iter().enumerate() {
                let CtorFieldSrc { name, ty } = field;
                writeln!(w, "{at}.ctor.{j}.field.{k}.name={name:?}")?;
                write!(w, "{at}.ctor.{j}.field.{k}.ty=")?;
                type_expr(w, left, ty)?;
                writeln!(w)?;
            }
        }
    }
    Ok(())
}

// Prefix form: U B L T | N"name" | P"name" | A"name"#arity followed by args.
// Iterative; the pending stack never exceeds the remaining node cap.
fn type_expr<W: Write + ?Sized>(
    w: &mut Budgeted<'_, W>,
    left: &mut u64,
    root: &TypeExprSrc,
) -> Result<(), Stop> {
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        if *left == 0 {
            return Err(Stop::TypeNodes);
        }
        *left -= 1;
        match node {
            TypeExprSrc::UInt => write!(w, "U")?,
            TypeExprSrc::Bool => write!(w, "B")?,
            TypeExprSrc::List => write!(w, "L")?,
            TypeExprSrc::Tree => write!(w, "T")?,
            TypeExprSrc::Named { name } => write!(w, "N{name:?}")?,
            TypeExprSrc::Param { name } => write!(w, "P{name:?}")?,
            TypeExprSrc::Apply { name, args } => {
                if (stack.len() as u64).saturating_add(args.len() as u64) > *left {
                    return Err(Stop::TypeNodes);
                }
                write!(w, "A{name:?}#{} ", args.len())?;
                stack.extend(args.iter().rev());
            }
        }
    }
    Ok(())
}

fn host_config<W: Write + ?Sized>(
    w: &mut Budgeted<'_, W>,
    config: &C6HostConfig,
) -> Result<(), Stop> {
    let C6HostConfig {
        layout,
        ext,
        budgets,
    } = config;
    let C6CodecBudgets {
        type_expr_nodes,
        template_bytes,
    } = budgets;
    writeln!(w, "config.layout={layout:?}")?;
    writeln!(w, "config.ext={ext:?}")?;
    writeln!(w, "config.budgets.type_expr_nodes={type_expr_nodes:?}")?;
    writeln!(w, "config.budgets.template_bytes={template_bytes:?}")?;
    Ok(())
}

fn lanes<W: Write + ?Sized>(
    w: &mut Budgeted<'_, W>,
    label: &str,
    values: &[u128],
) -> Result<(), Stop> {
    writeln!(w, "{label}.len={}", values.len())?;
    writeln!(w, "{label}={values:?}")?;
    Ok(())
}

/// Caller-owned custody of one in-flight actual step: an ownership seam only.
/// `record_c6_complete_step_into` MOVES the owner's own observation here before
/// encoding, so an unwinding codec or consumer cannot discard it. Nothing is
/// cloned, replayed or accepted from a caller; only that call fills the cell.
#[derive(Default)]
pub struct C6StepCustody {
    held: Option<C6InFlight>,
}

/// The actual observation plus whichever boundary encodings completed.
/// `None` lanes are unavailable (not completed), never empty encodings.
pub struct C6InFlight {
    observed: ObservedStep,
    config: C6HostConfig,
    before_lanes: Option<Vec<u128>>,
    after_lanes: Option<Vec<u128>>,
}

impl C6InFlight {
    pub fn observed(&self) -> &ObservedStep {
        &self.observed
    }
    pub fn config(&self) -> &C6HostConfig {
        &self.config
    }
    pub fn before_lanes(&self) -> Option<&[u128]> {
        self.before_lanes.as_deref()
    }
    pub fn after_lanes(&self) -> Option<&[u128]> {
        self.after_lanes.as_deref()
    }
    pub fn into_parts(
        self,
    ) -> (
        ObservedStep,
        C6HostConfig,
        Option<Vec<u128>>,
        Option<Vec<u128>>,
    ) {
        (
            self.observed,
            self.config,
            self.before_lanes,
            self.after_lanes,
        )
    }
    pub(crate) fn complete(&self) -> Option<(&ObservedStep, &C6HostConfig, &[u128], &[u128])> {
        Some((
            &self.observed,
            &self.config,
            self.before_lanes.as_deref()?,
            self.after_lanes.as_deref()?,
        ))
    }
    /// Same O(1) read-only copy from this in-flight step's saved BEFORE image;
    /// available whichever encodings completed. Data only, not acceptance.
    pub fn before_counter_observation(&self) -> C6BeforeCounterObservation {
        observe_before_counters(&self.observed)
    }
}

impl C6StepCustody {
    pub fn held(&self) -> Option<&C6InFlight> {
        self.held.as_ref()
    }
    /// Remove after a panic or refusal. It is never re-admitted as a capture.
    pub fn take(&mut self) -> Option<C6InFlight> {
        self.held.take()
    }
    /// Move out a fully encoded capture; an incomplete one stays in the cell.
    pub(crate) fn take_capture(&mut self) -> Option<C6CompleteStepCapture> {
        match self.held.take() {
            Some(C6InFlight {
                observed,
                config,
                before_lanes: Some(before_lanes),
                after_lanes: Some(after_lanes),
            }) => Some(C6CompleteStepCapture {
                observed,
                config,
                before_lanes,
                after_lanes,
            }),
            other => {
                self.held = other;
                None
            }
        }
    }
}

/// Same single owner call and encodings as `record_c6_complete_step`, held in
/// `custody` (the caller must pass it empty). On success the complete capture
/// stays there; a returned Materialization error empties the cell into the
/// unchanged error payload. The owner may have advanced in either case.
pub(crate) fn record_c6_complete_step_into(
    admitted: &Admitted,
    image: &mut MachineImage,
    retained: &LayoutSchemaC,
    budgets: C6CodecBudgets,
    limits: &C6HostLimits,
    custody: &mut C6StepCustody,
) -> Result<(), C6CompleteStepCaptureError> {
    let observed = record_step(admitted, image).map_err(C6CompleteStepCaptureError::Owner)?;
    let config = C6HostConfig::from_retained_layout(retained, budgets);
    let held = custody.held.insert(C6InFlight {
        observed,
        config,
        before_lanes: None,
        after_lanes: None,
    });
    let (before_raw, after_raw) = match &held.observed {
        ObservedStep::Event(record) => (&record.before, &record.after),
        ObservedStep::Terminal { before, after } => (before, after),
    };
    let (side, failure) = match encode_boundary(before_raw, retained, &held.config, limits) {
        Err(failure) => (C6CaptureSide::Before, failure),
        Ok(lanes) => {
            held.before_lanes = Some(lanes);
            match encode_boundary(after_raw, retained, &held.config, limits) {
                Err(failure) => (C6CaptureSide::After, failure),
                Ok(lanes) => {
                    held.after_lanes = Some(lanes);
                    return Ok(());
                }
            }
        }
    };
    let C6InFlight {
        observed,
        config,
        before_lanes,
        ..
    } = custody.held.take().expect("custody filled by this call");
    Err(C6CompleteStepCaptureError::Materialization {
        side,
        failure,
        observed: Box::new(observed),
        config,
        completed_before: before_lanes,
    })
}

#[cfg(test)]
#[path = "c6_complete_step_capture_tests.rs"]
mod tests;

// SOURCE ONLY, UNRUN.
#[cfg(test)]
mod archive_unit_tests {
    use super::*;

    // Derived drop glue recurses over a deep chain; dismantle it iteratively.
    fn drop_iteratively(root: TypeExprSrc) {
        let mut pending = vec![root];
        while let Some(mut node) = pending.pop() {
            if let TypeExprSrc::Apply { args, .. } = &mut node {
                pending.append(args);
            }
        }
    }

    #[test]
    fn deep_type_expr_is_iterative_and_node_capped() {
        let mut t = TypeExprSrc::UInt;
        for _ in 0..100_000 {
            t = TypeExprSrc::Apply {
                name: "F".into(),
                args: vec![t],
            };
        }
        let mut out = Vec::new();
        let mut w = Budgeted {
            inner: &mut out,
            written: 0,
            cap: u64::MAX,
            refused: false,
        };
        let mut exact = 100_001;
        assert!(type_expr(&mut w, &mut exact, &t).is_ok());
        assert_eq!(exact, 0);
        let mut short = 100_000;
        assert!(matches!(
            type_expr(&mut w, &mut short, &t),
            Err(Stop::TypeNodes)
        ));
        drop_iteratively(t);
    }

    #[test]
    fn byte_cap_refusal_is_truncation_not_completion() {
        let mut out = Vec::new();
        let mut w = Budgeted {
            inner: &mut out,
            written: 0,
            cap: 3,
            refused: false,
        };
        assert!(writeln!(w, "abcd").is_err());
        assert!(w.refused);
        assert_eq!(w.written, 0);
    }
}
