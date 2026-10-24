//! Actual-owner compact C6 storage stream. This is witness storage, not a proof.
//! One immutable prefix and one prior compact HOST are retained, never a trace Vec.
use super::c6_complete_step_capture::{
    C6CompleteStepCapture, C6CompleteStepCaptureError, record_c6_complete_step_into,
};
pub use super::c6_complete_step_capture::{C6InFlight, C6StepCustody};
use super::c6_phys::C6CodecBudgets;
use super::c6_phys_host::{C6HostConfig, C6HostError, C6HostLimits, encode_host};
use super::c6_stream_event::{EventCodecError, encode_event};
use super::complete_observation::{
    CaptureStepError, ConstantSnapshot, DataSnapshot, ExpressionSnapshot, HostSnapshot,
    ObservedStep,
};
use super::shared::*;
use sha2::{Digest, Sha256};
use std::io::{self, Write};

#[derive(Clone, Copy)]
pub struct StreamBounds {
    pub max_record_bytes: u64,
    pub max_total_bytes: u64,
    pub max_records: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StreamBinding {
    pub source_sha256: [u8; 32],
    pub input_sha256: [u8; 32],
    pub expected_result_sha256: [u8; 32],
}
#[derive(Debug)]
pub enum StreamFailure {
    Codec(C6HostError),
    Event(EventCodecError),
    Io(io::Error),
    Bound,
    Overflow,
    Continuity,
    Immutable,
    Progress,
    Terminal,
    Poisoned,
    /// Step custody occupied before stepping (no step), or an advanced step not
    /// complete in custody (it stays there; advance unknown).
    Custody,
}
/// Every variant carries the stream's poisoned progress at refusal (never complete).
pub enum RangeCaptureError {
    Capture {
        error: C6CompleteStepCaptureError,
        progress: StreamProgress,
    },
    Stream {
        failure: StreamFailure,
        capture: Option<Box<C6CompleteStepCapture>>,
        progress: StreamProgress,
    },
}
impl std::fmt::Debug for RangeCaptureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Capture { error, progress } => f
                .debug_struct("Capture")
                .field("error", error)
                .field("progress", progress)
                .finish(),
            Self::Stream {
                failure,
                capture,
                progress,
            } => f
                .debug_struct("Stream")
                .field("failure", failure)
                .field("genuine_capture_retained", &capture.is_some())
                .field("progress", progress)
                .finish(),
        }
    }
}
/// Whether the fixed owner may have stepped in THIS call. Never a retry licence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnerAdvance {
    NotStepped,
    Advanced,
    Unknown,
}
impl RangeCaptureError {
    pub fn owner_advance(&self) -> OwnerAdvance {
        match self {
            Self::Capture {
                error: C6CompleteStepCaptureError::Owner(CaptureStepError::BeforeObservation(_)),
                ..
            } => OwnerAdvance::NotStepped,
            Self::Capture {
                error: C6CompleteStepCaptureError::Owner(CaptureStepError::Step { .. }),
                ..
            } => OwnerAdvance::Unknown,
            Self::Capture { .. }
            | Self::Stream {
                capture: Some(_), ..
            } => OwnerAdvance::Advanced,
            Self::Stream {
                failure: StreamFailure::Custody,
                ..
            } => OwnerAdvance::Unknown,
            Self::Stream { .. } => OwnerAdvance::NotStepped,
        }
    }
}
pub struct BeginError<W> {
    pub failure: StreamFailure,
    pub writer: W,
    pub partial: Option<StreamProgress>,
}
/// The exact first finish cause and the original owned stream/partial output.
/// This is not a successful trace; every contained stream is poisoned.
pub struct FinishError<W: Write> {
    pub failure: StreamFailure,
    pub stream: C6RangeStream<W>,
}

/// Prefix bytes alone are never a complete trace or a proof artifact.
#[derive(Clone, Debug)]
pub struct StreamProgress {
    pub records: u64,
    pub bytes: u64,
    pub completed_intents: u64,
    pub range: u8,
    pub terminal: bool,
    pub poisoned: bool,
    pub complete: bool,
    pub sha256: [u8; 32],
}
pub struct C6RangeStream<W: Write> {
    writer: W,
    bounds: StreamBounds,
    config: C6HostConfig,
    limits: C6HostLimits,
    previous: Vec<u128>,
    bytes: u64,
    records: u64,
    completed: u64,
    range: u8,
    terminal: bool,
    poisoned: bool,
    hash: Sha256,
}
fn add(a: u64, b: u64) -> Result<u64, StreamFailure> {
    a.checked_add(b).ok_or(StreamFailure::Overflow)
}
fn lane_bytes(n: usize) -> Result<u64, StreamFailure> {
    u64::try_from(n)
        .map_err(|_| StreamFailure::Overflow)?
        .checked_mul(16)
        .ok_or(StreamFailure::Overflow)
}
pub(crate) fn state_start(v: &[u128]) -> Result<usize, StreamFailure> {
    if v.len() < 14 {
        return Err(StreamFailure::Immutable);
    }
    let mut n = 14usize;
    for i in [6, 8, 10] {
        n = n
            .checked_add(v[i].try_into().map_err(|_| StreamFailure::Overflow)?)
            .ok_or(StreamFailure::Overflow)?
    }
    let state: usize = v[12].try_into().map_err(|_| StreamFailure::Overflow)?;
    if n.checked_add(state) != Some(v.len()) {
        return Err(StreamFailure::Immutable);
    }
    Ok(n)
}
fn same_prefix(a: &[u128], b: &[u128]) -> Result<(), StreamFailure> {
    let x = state_start(a)?;
    let y = state_start(b)?;
    if x != y || a[..12] != b[..12] || a[13..x] != b[13..y] {
        return Err(StreamFailure::Immutable);
    }
    Ok(())
}
// Only the genuine fixed-owner ReturnCallValid restoring this source callsite
// can advance the logical transaction cursor. Names alone do not authenticate
// source: immutable complete program + external source/input bindings remain.
fn constructor(
    h: &HostSnapshot,
    r: RefC,
    ty: &str,
    name: &str,
    arity: usize,
) -> Result<Option<Vec<RefC>>, StreamFailure> {
    let (t, c, fields) = match r {
        RefC::Const(i) => match h.admission.view.constants.get(i) {
            Some(ConstantSnapshot::Constructor {
                marker_free: true,
                type_id,
                ctor_id,
                fields,
            }) => {
                if fields.len != arity {
                    return Err(StreamFailure::Progress);
                }
                let end = fields
                    .start
                    .checked_add(fields.len)
                    .ok_or(StreamFailure::Overflow)?;
                let v = h
                    .admission
                    .view
                    .constant_fields
                    .get(fields.start..end)
                    .ok_or(StreamFailure::Progress)?;
                (
                    *type_id,
                    *ctor_id,
                    v.iter().copied().map(RefC::Const).collect::<Vec<_>>(),
                )
            }
            _ => return Ok(None),
        },
        RefC::Data(i) => match h.image.data.get(i) {
            Some(DataSnapshot::Constructor {
                type_id,
                ctor_id,
                fields,
            }) => {
                if fields.len != arity {
                    return Err(StreamFailure::Progress);
                }
                let end = fields
                    .start
                    .checked_add(fields.len)
                    .ok_or(StreamFailure::Overflow)?;
                (
                    *type_id,
                    *ctor_id,
                    h.image
                        .data_fields
                        .get(fields.start..end)
                        .ok_or(StreamFailure::Progress)?
                        .to_vec(),
                )
            }
            _ => return Ok(None),
        },
    };
    let tr = h
        .admission
        .view
        .types
        .get(t.0)
        .ok_or(StreamFailure::Progress)?;
    let cr = h
        .admission
        .view
        .constructors
        .get(c.0)
        .ok_or(StreamFailure::Progress)?;
    if cr.owning_type != t {
        return Err(StreamFailure::Progress);
    }
    let tn = h
        .admission
        .view
        .names
        .get(tr.name_id)
        .ok_or(StreamFailure::Progress)?;
    let cn = h
        .admission
        .view
        .names
        .get(cr.name_id)
        .ok_or(StreamFailure::Progress)?;
    if tn != ty || cn != name {
        return Ok(None);
    }
    if cr.field_len != arity || fields.len() != arity {
        return Err(StreamFailure::Progress);
    }
    Ok(Some(fields))
}
fn uint(h: &HostSnapshot, r: RefC) -> Result<u128, StreamFailure> {
    match r {
        RefC::Const(i) => match h.admission.view.constants.get(i) {
            Some(ConstantSnapshot::U128 {
                marker_free: true,
                n,
            }) => Ok(*n),
            _ => Err(StreamFailure::Progress),
        },
        RefC::Data(i) => match h.image.data.get(i) {
            Some(DataSnapshot::U128 { n }) => Ok(*n),
            _ => Err(StreamFailure::Progress),
        },
    }
}
pub(crate) fn completed_call(o: &ObservedStep) -> Result<Option<u64>, StreamFailure> {
    let ObservedStep::Event(r) = o else {
        return Ok(None);
    };
    if r.event.rule_id != RuleIdC::KMachineReturnCallValid {
        return Ok(None);
    }
    let Some(FrameRowC::ReturnF { origin, .. }) = r.before.image.frames.last() else {
        return Err(StreamFailure::Progress);
    };
    let p = &r.before.admission.view;
    let Some(ExpressionSnapshot::Call { function_name, .. }) = p.expressions.get(*origin) else {
        return Err(StreamFailure::Progress);
    };
    if p.names.get(*function_name).map(String::as_str) != Some("apply_tx") {
        return Ok(None);
    }
    let occ = p.occurrences.get(*origin).ok_or(StreamFailure::Progress)?;
    let f = p
        .functions
        .get(occ.function)
        .ok_or(StreamFailure::Progress)?;
    if !occ.owner_is_function
        || p.names.get(f.function_name).map(String::as_str) != Some("run_header_transactions")
    {
        return Err(StreamFailure::Progress);
    }
    if r.after.image.control_origin != *origin {
        return Err(StreamFailure::Progress);
    }
    let ControlC::Return { value } = r.after.image.control else {
        return Err(StreamFailure::Progress);
    };
    let Some(good) = constructor(&r.after, value, "Progress", "Good", 2)? else {
        return Err(StreamFailure::Progress);
    };
    let Some(run) = constructor(&r.after, good[0], "Run", "Run", 6)? else {
        return Err(StreamFailure::Progress);
    };
    uint(&r.after, run[3])?
        .try_into()
        .map(Some)
        .map_err(|_| StreamFailure::Progress)
}
// Borrowed view of a complete capture held in caller custody during publish.
struct Parts<'a> {
    observed: &'a ObservedStep,
    config: &'a C6HostConfig,
    before: &'a [u128],
    after: &'a [u128],
}
impl<'a> Parts<'a> {
    fn observed(&self) -> &'a ObservedStep {
        self.observed
    }
    fn config(&self) -> &'a C6HostConfig {
        self.config
    }
    fn before_lanes(&self) -> &'a [u128] {
        self.before
    }
    fn after_lanes(&self) -> &'a [u128] {
        self.after
    }
}
impl<W: Write> C6RangeStream<W> {
    /// `initial` must be the actual consumer's unchanged capture; the first actual
    /// step's full before equality is checked. This constructor admits no proof.
    pub fn begin(
        writer: W,
        initial: &HostSnapshot,
        config: C6HostConfig,
        limits: &C6HostLimits,
        bounds: StreamBounds,
        binding: StreamBinding,
    ) -> Result<Self, BeginError<W>> {
        let previous = match encode_host(initial, &config, limits) {
            Ok(v) => v,
            Err(e) => {
                return Err(BeginError {
                    failure: StreamFailure::Codec(e),
                    writer,
                    partial: None,
                });
            }
        };
        if let Err(failure) = state_start(&previous) {
            return Err(BeginError {
                failure,
                writer,
                partial: None,
            });
        }
        let mut s = Self {
            writer,
            bounds,
            config,
            limits: *limits,
            previous,
            bytes: 0,
            records: 0,
            completed: 0,
            range: 0,
            terminal: false,
            poisoned: false,
            hash: Sha256::new(),
        };
        let result = (|| -> Result<(), StreamFailure> {
            let size = add(112, lane_bytes(s.previous.len())?)?;
            s.reserve(size)?;
            s.put(b"C6RNG001")?;
            s.put(&binding.source_sha256)?;
            s.put(&binding.input_sha256)?;
            s.put(&binding.expected_result_sha256)?;
            let n = s.previous.len() as u64;
            s.put(&n.to_le_bytes())?;
            for i in 0..s.previous.len() {
                s.put(&s.previous[i].to_le_bytes())?
            }
            s.writer.flush().map_err(StreamFailure::Io)?;
            Ok(())
        })();
        match result {
            Ok(()) => Ok(s),
            Err(failure) => {
                s.poisoned = true;
                let p = s.progress();
                Err(BeginError {
                    failure,
                    writer: s.writer,
                    partial: Some(p),
                })
            }
        }
    }
    fn reserve(&mut self, size: u64) -> Result<(), StreamFailure> {
        if size > self.bounds.max_record_bytes
            || add(self.bytes, size)? > self.bounds.max_total_bytes
        {
            self.poisoned = true;
            return Err(StreamFailure::Bound);
        }
        Ok(())
    }
    fn put(&mut self, b: &[u8]) -> Result<(), StreamFailure> {
        let mut at = 0;
        while at < b.len() {
            match self.writer.write(&b[at..]) {
                Ok(0) => {
                    self.poisoned = true;
                    return Err(StreamFailure::Io(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "C6 stream partial",
                    )));
                }
                Ok(n) if n <= b.len() - at => {
                    self.hash.update(&b[at..at + n]);
                    self.bytes = add(self.bytes, n as u64)?;
                    at += n
                }
                Ok(_) => {
                    self.poisoned = true;
                    return Err(StreamFailure::Io(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "invalid writer count",
                    )));
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => {
                    self.poisoned = true;
                    return Err(StreamFailure::Io(e));
                }
            }
        }
        Ok(())
    }
    pub fn progress(&self) -> StreamProgress {
        StreamProgress {
            records: self.records,
            bytes: self.bytes,
            completed_intents: self.completed,
            range: self.range,
            terminal: self.terminal,
            poisoned: self.poisoned,
            complete: false,
            sha256: self.hash.clone().finalize().into(),
        }
    }
    /// Transfer partial owned output on refusal. It is never labelled complete.
    pub fn into_partial(self) -> (W, StreamProgress) {
        let p = self.progress();
        (self.writer, p)
    }
    fn publish(&mut self, c: &Parts<'_>) -> Result<(u64, bool), StreamFailure> {
        if c.config() != &self.config || c.before_lanes() != self.previous {
            return Err(StreamFailure::Continuity);
        }
        same_prefix(c.before_lanes(), c.after_lanes())?;
        let (event, terminal, run_id) = match c.observed() {
            ObservedStep::Event(r) => {
                if r.before.admission != r.after.admission {
                    return Err(StreamFailure::Immutable);
                }
                (Some(&r.event), false, &r.before.image.run_id)
            }
            ObservedStep::Terminal { before, after } => {
                if before != after {
                    return Err(StreamFailure::Terminal);
                }
                (None, true, &before.image.run_id)
            }
        };
        let event = match event {
            Some(e) => encode_event(
                e,
                run_id,
                (self.bounds.max_record_bytes / 16)
                    .try_into()
                    .map_err(|_| StreamFailure::Overflow)?,
            )
            .map_err(StreamFailure::Event)?,
            None => Vec::new(),
        };
        let completed = match completed_call(c.observed())? {
            Some(n) if self.completed.checked_add(1) == Some(n) && n <= 16 => n,
            Some(_) => return Err(StreamFailure::Progress),
            None => self.completed,
        };
        let cut = completed != self.completed && matches!(completed, 4 | 8 | 12);
        let at = state_start(c.after_lanes())?;
        let state = &c.after_lanes()[at..];
        let frame = add(26, add(lane_bytes(event.len())?, lane_bytes(state.len())?)?)?;
        let total = add(frame, if cut { 18 } else { 0 })?;
        self.reserve(total)?;
        self.put(&[if terminal { 2 } else { 1 }, self.range])?;
        self.put(&self.records.to_le_bytes())?;
        self.put(&(event.len() as u64).to_le_bytes())?;
        for v in event {
            self.put(&v.to_le_bytes())?
        }
        self.put(&(state.len() as u64).to_le_bytes())?;
        for v in state {
            self.put(&v.to_le_bytes())?
        }
        let next = self.records.checked_add(1).ok_or(StreamFailure::Overflow)?;
        if cut {
            self.put(&[3, self.range + 1])?;
            self.put(&next.to_le_bytes())?;
            self.put(&completed.to_le_bytes())?
        }
        self.writer.flush().map_err(|e| {
            self.poisoned = true;
            StreamFailure::Io(e)
        })?;
        Ok((completed, terminal))
    }
    /// The sole publishing path consumes one actual fixed-owner capture. The step
    /// is MOVED into caller-owned `custody` before encoding/publication, so an
    /// unwinding codec/stream leaves it there; a returned failure moves it into
    /// the error and empties the cell. No retry occurs. The stream is poisoned
    /// across the owner step and publish and cleared only on success, so a caught
    /// unwind (custody still held) cannot step the owner a second time.
    pub fn record_next(
        &mut self,
        admitted: &Admitted,
        image: &mut MachineImage,
        retained: &LayoutSchemaC,
        budgets: C6CodecBudgets,
        limits: &C6HostLimits,
        custody: &mut C6StepCustody,
    ) -> Result<ObservedStep, RangeCaptureError> {
        if self.poisoned
            || self.terminal
            || self.records >= self.bounds.max_records
            || limits != &self.limits
            || C6HostConfig::from_retained_layout(retained, budgets) != self.config
            || custody.held().is_some()
        {
            let failure = if self.poisoned {
                StreamFailure::Poisoned
            } else if self.terminal {
                StreamFailure::Terminal
            } else if self.records >= self.bounds.max_records {
                StreamFailure::Bound
            } else if custody.held().is_some() {
                StreamFailure::Custody
            } else {
                StreamFailure::Immutable
            };
            self.poisoned = true;
            return Err(RangeCaptureError::Stream {
                failure,
                capture: None,
                progress: self.progress(),
            });
        }
        self.poisoned = true;
        if let Err(error) =
            record_c6_complete_step_into(admitted, image, retained, budgets, limits, custody)
        {
            self.poisoned = true;
            return Err(RangeCaptureError::Capture {
                error,
                progress: self.progress(),
            });
        }
        let published = match custody.held().and_then(C6InFlight::complete) {
            Some((observed, config, before, after)) => self.publish(&Parts {
                observed,
                config,
                before,
                after,
            }),
            None => Err(StreamFailure::Custody),
        };
        let (completed, terminal) = match published {
            Ok(v) => v,
            Err(failure) => {
                self.poisoned = true;
                return Err(RangeCaptureError::Stream {
                    failure,
                    capture: custody.take_capture().map(Box::new),
                    progress: self.progress(),
                });
            }
        };
        let Some(c) = custody.take_capture() else {
            self.poisoned = true;
            return Err(RangeCaptureError::Stream {
                failure: StreamFailure::Custody,
                capture: None,
                progress: self.progress(),
            });
        };
        let (observed, _, _, after) = c.into_parts();
        self.previous = after;
        self.records += 1;
        if completed != self.completed && matches!(completed, 4 | 8 | 12) {
            self.range += 1
        }
        self.completed = completed;
        self.terminal = terminal;
        self.poisoned = false;
        Ok(observed)
    }
    /// A footer means complete storage, not oracle/financial/native validity.
    /// The consumer calls this only after its unchanged whole-value oracle check.
    pub fn finish(mut self) -> Result<(W, StreamProgress), FinishError<W>> {
        let result = if self.poisoned {
            Err(StreamFailure::Poisoned)
        } else if !self.terminal || self.completed != 16 || self.range != 3 {
            Err(StreamFailure::Terminal)
        } else {
            (|| -> Result<(), StreamFailure> {
                self.reserve(17)?;
                self.put(&[4])?;
                self.put(&self.records.to_le_bytes())?;
                self.put(&16u64.to_le_bytes())?;
                self.writer.flush().map_err(StreamFailure::Io)?;
                Ok(())
            })()
        };
        match result {
            Err(failure) => {
                self.poisoned = true;
                Err(FinishError {
                    failure,
                    stream: self,
                })
            }
            Ok(()) => {
                let mut p = self.progress();
                p.complete = true;
                Ok((self.writer, p))
            }
        }
    }
}

#[cfg(test)]
mod footer_failure_tests {
    use super::*;
    const PREFIX: &[u8] = b"supplied-private-footer-fixture";
    struct Output {
        bytes: Vec<u8>,
        stop_at: Option<usize>,
        flush_error: bool,
    }
    impl Write for Output {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            let n = match self.stop_at {
                Some(end) if self.bytes.len() >= end => {
                    return Err(io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        "injected footer write",
                    ));
                }
                Some(end) => b.len().min(end - self.bytes.len()),
                None => b.len(),
            };
            self.bytes.extend_from_slice(&b[..n]);
            Ok(n)
        }
        fn flush(&mut self) -> io::Result<()> {
            if self.flush_error {
                Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "injected footer flush",
                ))
            } else {
                Ok(())
            }
        }
    }
    // Supplied PRIVATE state only exercises finish IO/arithmetic mechanics. It is
    // never an owner capture, real16 transaction run, range proof or fit evidence.
    fn supplied_footer_state(
        bounds: StreamBounds,
        stop_at: Option<usize>,
        flush_error: bool,
        byte_counter: u64,
    ) -> C6RangeStream<Output> {
        let oracle: serde_json::Value = serde_json::from_slice(include_bytes!(
            "testdata/c6_complete_step_capture_oracle01.json"
        ))
        .unwrap();
        let lanes: Vec<u128> = oracle["enabledPositive"]["preCompactHOST"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| u128::from(v.as_u64().unwrap()))
            .collect();
        let limits = C6HostLimits {
            max_total_lanes: 2048,
            max_state_lanes: 1024,
            max_metadata_bytes: 256,
            max_string_bytes: 32,
            max_width_entries: 0,
        };
        let metadata =
            super::super::c6_phys_host::decode_metadata(&lanes[14..102], &limits).unwrap();
        let config = C6HostConfig {
            layout: metadata.layout,
            ext: None,
            budgets: C6CodecBudgets {
                type_expr_nodes: 16,
                template_bytes: 64,
            },
        };
        let mut hash = Sha256::new();
        hash.update(PREFIX);
        C6RangeStream {
            writer: Output {
                bytes: PREFIX.to_vec(),
                stop_at,
                flush_error,
            },
            bounds,
            config,
            limits,
            previous: Vec::new(),
            bytes: byte_counter,
            records: 7,
            completed: 16,
            range: 3,
            terminal: true,
            poisoned: false,
            hash,
        }
    }
    fn bounds() -> StreamBounds {
        StreamBounds {
            max_record_bytes: 4096,
            max_total_bytes: 4096,
            max_records: 100,
        }
    }
    fn failure(s: C6RangeStream<Output>) -> FinishError<Output> {
        match s.finish() {
            Err(e) => e,
            Ok(_) => panic!("injected finish failure became success"),
        }
    }
    #[test]
    fn footer_byte_refusal_returns_original_bound_and_owned_prefix() {
        let mut b = bounds();
        b.max_total_bytes = PREFIX.len() as u64 + 16;
        let e = failure(supplied_footer_state(b, None, false, PREFIX.len() as u64));
        assert!(matches!(e.failure, StreamFailure::Bound));
        let p = e.stream.progress();
        assert!(p.poisoned && !p.complete);
        assert_eq!(p.bytes, PREFIX.len() as u64);
        let (writer, _) = e.stream.into_partial();
        assert_eq!(writer.bytes.as_slice(), PREFIX);
    }
    #[test]
    fn footer_partial_write_returns_exact_io_and_partial_owned_bytes() {
        let e = failure(supplied_footer_state(
            bounds(),
            Some(PREFIX.len() + 5),
            false,
            PREFIX.len() as u64,
        ));
        match &e.failure {
            StreamFailure::Io(error) => {
                assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
                assert_eq!(error.to_string(), "injected footer write")
            }
            _ => panic!("first write error was replaced"),
        }
        assert!(e.stream.progress().poisoned && !e.stream.progress().complete);
        assert_eq!(e.stream.progress().bytes, PREFIX.len() as u64 + 5);
        let e = failure(e.stream);
        assert!(matches!(e.failure, StreamFailure::Poisoned));
        let (writer, _) = e.stream.into_partial();
        let mut expected = PREFIX.to_vec();
        expected.push(4);
        expected.extend_from_slice(&7u64.to_le_bytes()[..4]);
        assert_eq!(writer.bytes, expected);
    }
    #[test]
    fn footer_flush_returns_exact_io_even_after_complete_footer_bytes() {
        let e = failure(supplied_footer_state(
            bounds(),
            None,
            true,
            PREFIX.len() as u64,
        ));
        match &e.failure {
            StreamFailure::Io(error) => {
                assert_eq!(error.kind(), io::ErrorKind::WriteZero);
                assert_eq!(error.to_string(), "injected footer flush")
            }
            _ => panic!("flush error was replaced"),
        }
        assert!(e.stream.progress().poisoned && !e.stream.progress().complete);
        let (writer, p) = e.stream.into_partial();
        let mut expected = PREFIX.to_vec();
        expected.push(4);
        expected.extend_from_slice(&7u64.to_le_bytes());
        expected.extend_from_slice(&16u64.to_le_bytes());
        assert_eq!(writer.bytes, expected);
        assert_eq!(p.bytes, expected.len() as u64);
    }
    #[test]
    fn footer_checked_overflow_poisoned_without_losing_owned_prefix() {
        let b = StreamBounds {
            max_record_bytes: u64::MAX,
            max_total_bytes: u64::MAX,
            max_records: 100,
        };
        let e = failure(supplied_footer_state(b, None, false, u64::MAX - 8));
        assert!(matches!(e.failure, StreamFailure::Overflow));
        assert!(e.stream.progress().poisoned && !e.stream.progress().complete);
        let (writer, _) = e.stream.into_partial();
        assert_eq!(writer.bytes.as_slice(), PREFIX);
    }
}

#[cfg(test)]
mod custody_tests {
    use super::*;
    fn progress() -> StreamProgress {
        StreamProgress {
            records: 3,
            bytes: 9,
            completed_intents: 2,
            range: 0,
            terminal: false,
            poisoned: true,
            complete: false,
            sha256: [7; 32],
        }
    }
    #[test]
    fn refusal_before_step_keeps_exact_progress_and_reports_no_step() {
        let e = RangeCaptureError::Stream {
            failure: StreamFailure::Bound,
            capture: None,
            progress: progress(),
        };
        assert_eq!(e.owner_advance(), OwnerAdvance::NotStepped);
        let RangeCaptureError::Stream {
            failure: StreamFailure::Bound,
            capture: None,
            progress: p,
        } = e
        else {
            panic!("refusal payload replaced")
        };
        assert_eq!(
            (p.records, p.bytes, p.completed_intents, p.sha256),
            (3, 9, 2, [7; 32])
        );
        assert!(p.poisoned && !p.complete);
    }
    #[test]
    fn custody_refusal_without_capture_is_unknown_advance() {
        let e = RangeCaptureError::Stream {
            failure: StreamFailure::Custody,
            capture: None,
            progress: progress(),
        };
        assert_eq!(e.owner_advance(), OwnerAdvance::Unknown);
    }
    #[test]
    fn empty_step_custody_yields_no_capture_or_fabricated_step() {
        let mut c = C6StepCustody::default();
        assert!(c.held().is_none());
        assert!(c.take_capture().is_none());
        assert!(c.take().is_none());
    }
}
