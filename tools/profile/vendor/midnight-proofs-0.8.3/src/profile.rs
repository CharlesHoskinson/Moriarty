//! Bounded, public-metadata-only observation for the isolated profiling workspace.
//!
//! Accounting spans own their residual after nested accounting spans are removed
//! by the reducer. Diagnostics (including NTTs) never earn additional wall credit.

/// Stable phase IDs in `recursion.phase-events/1`.
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum Phase {
    /// Command wrapper.
    Command = 1,
    /// Key deserialization.
    KeyLoad = 2,
    /// Proving key preparation.
    SetupPk = 3,
    /// Caller proof interval, diagnostic only.
    Prove = 4,
    /// Caller verification.
    Verify = 5,
    /// Public instance preparation.
    Instances = 6,
    /// One circuit in one advice phase.
    Synthesis = 7,
    /// Rational resolution and blinding.
    Resolution = 8,
    /// Advice commitment batch.
    AdviceCommitments = 9,
    /// Lookup permutation.
    LookupPermutation = 10,
    /// Lookup grand products.
    LookupProducts = 11,
    /// Copy permutation products.
    PermutationProducts = 12,
    /// Vanishing random polynomial.
    Vanishing = 13,
    /// Quotient evaluation and transforms.
    Quotient = 14,
    /// Quotient commitment batch.
    QuotientCommitments = 15,
    /// Polynomial evaluations and opening queries.
    Evaluations = 16,
    /// Multiopen residual work.
    Multiopen = 17,
    /// Commitment MSM, subtracted from the enclosing owner.
    Msm = 18,
    /// Transcript operations.
    Transcript = 19,
    /// Fixed-column commitment batch.
    FixedCommitments = 20,
    /// Keygen permutation commitment batch.
    PermutationCommitments = 21,
    /// NTT diagnostic, owned by the enclosing phase.
    Ntt = 22,
    /// Trash argument.
    Trash = 23,
    /// Advice coefficient conversion.
    AdviceNtt = 24,
}

/// Whether a span owns residual time or is only an inclusive diagnostic.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Accounting {
    /// No additional accounting credit.
    Diagnostic = 0,
    /// Residual time after nested exclusive children are removed.
    Exclusive = 1,
}

/// Observe an expression without evaluating any profiler code when disabled.
/// Early returns and `?` drop an unfinished guard and retain the error interval.
#[cfg(feature = "recursion-phase-profile")]
#[macro_export]
macro_rules! profile_span {
    ($phase:ident, $accounting:ident, $body:expr) => {{
        let mut profile_guard = $crate::profile::global().span(
            $crate::profile::Phase::$phase,
            $crate::profile::Accounting::$accounting,
        );
        let profile_value = $body;
        profile_guard.finish();
        profile_value
    }};
}

/// Observe a Result expression, retaining returned errors as failed intervals.
#[cfg(feature = "recursion-phase-profile")]
#[macro_export]
macro_rules! profile_result {
    ($phase:ident, $accounting:ident, $body:expr) => {{
        let mut profile_guard = $crate::profile::global().span(
            $crate::profile::Phase::$phase,
            $crate::profile::Accounting::$accounting,
        );
        let profile_value = $body;
        if profile_value.is_ok() {
            profile_guard.finish();
        }
        profile_value
    }};
}

/// Disabled observation expands directly to the original expression.
#[cfg(not(feature = "recursion-phase-profile"))]
#[macro_export]
macro_rules! profile_span {
    ($phase:ident, $accounting:ident, $body:expr) => {
        $body
    };
}

/// Disabled observation expands directly to the original result.
#[cfg(not(feature = "recursion-phase-profile"))]
#[macro_export]
macro_rules! profile_result {
    ($phase:ident, $accounting:ident, $body:expr) => {
        $body
    };
}

/// Begin a named guard across statements without changing their scopes.
#[cfg(feature = "recursion-phase-profile")]
#[macro_export]
macro_rules! profile_start {
    ($guard:ident, $phase:ident, $accounting:ident) => {
        let mut $guard = $crate::profile::global().span(
            $crate::profile::Phase::$phase,
            $crate::profile::Accounting::$accounting,
        );
    };
}
/// Complete and close a named guard.
#[cfg(feature = "recursion-phase-profile")]
#[macro_export]
macro_rules! profile_finish {
    ($guard:ident) => {
        $guard.finish();
        drop($guard);
    };
}
/// Disabled guard declaration emits no code.
#[cfg(not(feature = "recursion-phase-profile"))]
#[macro_export]
macro_rules! profile_start {
    ($guard:ident, $phase:ident, $accounting:ident) => {};
}
/// Disabled guard completion emits no code.
#[cfg(not(feature = "recursion-phase-profile"))]
#[macro_export]
macro_rules! profile_finish {
    ($guard:ident) => {};
}

/// Tag the following circuit spans with public phase/circuit ordinals.
#[cfg(feature = "recursion-phase-profile")]
#[macro_export]
macro_rules! profile_advice {
    ($phase:expr, $circuit:expr) => {
        let _profile_advice_context =
            $crate::profile::AdviceContext::new($phase as u64, $circuit as u64);
    };
}
/// Disabled advice tagging emits no code.
#[cfg(not(feature = "recursion-phase-profile"))]
#[macro_export]
macro_rules! profile_advice {
    ($phase:expr, $circuit:expr) => {};
}

#[cfg(feature = "recursion-phase-profile")]
mod enabled {
    use std::{
        cell::Cell,
        io::{self, Write},
        marker::PhantomData,
        rc::Rc,
        sync::{
            atomic::{AtomicU64, Ordering},
            Mutex, OnceLock,
        },
        time::Instant,
    };

    use super::{Accounting, Phase};

    static EPOCH: OnceLock<Instant> = OnceLock::new();
    static NEXT_THREAD: AtomicU64 = AtomicU64::new(1);
    thread_local! {
        static BUSY: Cell<bool> = const { Cell::new(false) };
        static PARENT: Cell<(usize, u64)> = const { Cell::new((0, 0)) };
        static ADVICE: Cell<(u64, u64)> = const { Cell::new((u64::MAX, u64::MAX)) };
        static THREAD: u64 = NEXT_THREAD.fetch_add(1, Ordering::Relaxed);
    }

    fn now() -> Option<u64> {
        Instant::now()
            .checked_duration_since(*EPOCH.get_or_init(Instant::now))?
            .as_nanos()
            .try_into()
            .ok()
    }

    struct Enter;
    impl Enter {
        fn new() -> Option<Self> {
            BUSY.with(|busy| if busy.replace(true) { None } else { Some(Self) })
        }
    }
    impl Drop for Enter {
        fn drop(&mut self) {
            BUSY.with(|busy| busy.set(false));
        }
    }

    /// Thread-bound public circuit ordinals; no witness data is accepted.
    #[derive(Debug)]
    pub struct AdviceContext((u64, u64), PhantomData<Rc<()>>);
    impl AdviceContext {
        /// Set ordinals until this guard drops. MAX means not applicable.
        pub fn new(phase: u64, circuit: u64) -> Self {
            Self(
                ADVICE.with(|context| context.replace((phase, circuit))),
                PhantomData,
            )
        }
    }
    impl Drop for AdviceContext {
        fn drop(&mut self) {
            ADVICE.with(|context| context.set(self.0));
        }
    }

    /// One reserved interval; status 0=open, 1=ok, 2=returned error, 3=unwind.
    #[derive(Clone, Copy, Debug)]
    pub struct Event {
        /// Process-local interval identity.
        pub id: u64,
        /// Enclosing interval on this thread, or zero.
        pub parent_id: u64,
        /// Stable public phase ID.
        pub phase_id: u16,
        /// Zero for diagnostic, one for exclusive residual.
        pub accounting: u8,
        /// Nanoseconds since process epoch.
        pub start_ns: u64,
        /// Nanoseconds since process epoch, zero while open.
        pub end_ns: u64,
        /// Process-local thread identity; no OS or environment data.
        pub thread_id: u64,
        /// Advice phase ordinal, or MAX when outside advice processing.
        pub advice_phase: u64,
        /// Circuit ordinal, or MAX when outside advice processing.
        pub circuit_index: u64,
        /// Completion status, not an error string.
        pub status: u8,
    }

    /// A copy taken outside the timed path for reporting.
    #[derive(Debug)]
    pub struct Snapshot {
        /// Reserved intervals, including unfinished ones.
        pub events: Vec<Event>,
        /// Hard event limit.
        pub event_capacity: usize,
        /// Events refused at capacity.
        pub dropped_events: u64,
        /// Clock failures or regressions.
        pub timer_failures: u64,
        /// Observer reentries suppressed before locking or reading the clock.
        pub reentrant_events: u64,
        /// Buffer and timer integrity only; not overhead or proof qualification.
        pub profile_valid: bool,
    }

    /// A preallocated buffer. No allocation or formatting occurs per event.
    #[derive(Debug)]
    pub struct Recorder {
        events: Mutex<Vec<Event>>,
        capacity: usize,
        clock: fn() -> Option<u64>,
        dropped: AtomicU64,
        timers: AtomicU64,
        reentrant: AtomicU64,
    }

    impl Recorder {
        /// Allocate the event buffer before observation begins.
        pub fn new(capacity: usize) -> Self {
            Self::with_clock(capacity, now)
        }

        /// Supply a fallible monotonic clock, also useful for deterministic tests.
        pub fn with_clock(capacity: usize, clock: fn() -> Option<u64>) -> Self {
            EPOCH.get_or_init(Instant::now);
            let mut events = Vec::new();
            let capacity = if events.try_reserve_exact(capacity).is_ok() {
                capacity
            } else {
                0
            };
            Self {
                events: Mutex::new(events),
                capacity,
                clock,
                dropped: AtomicU64::new(0),
                timers: AtomicU64::new(0),
                reentrant: AtomicU64::new(0),
            }
        }

        /// Start an interval. An observer failure never interrupts the caller.
        pub fn span(&self, phase: Phase, accounting: Accounting) -> Span<'_> {
            let mut span = Span {
                recorder: self,
                index: None,
                previous: (0, 0),
                finished: false,
                _thread: PhantomData,
            };
            let Some(_enter) = Enter::new() else {
                self.reentrant.fetch_add(1, Ordering::Relaxed);
                return span;
            };
            let Some(start_ns) = (self.clock)() else {
                self.timers.fetch_add(1, Ordering::Relaxed);
                return span;
            };
            let Ok(mut events) = self.events.lock() else {
                self.dropped.fetch_add(1, Ordering::Relaxed);
                return span;
            };
            if events.len() == self.capacity {
                self.dropped.fetch_add(1, Ordering::Relaxed);
                return span;
            }
            let index = events.len();
            let id = index as u64 + 1;
            let owner = self as *const Self as usize;
            let previous = PARENT.with(|parent| parent.replace((owner, id)));
            events.push(Event {
                id,
                parent_id: if previous.0 == owner { previous.1 } else { 0 },
                phase_id: phase as u16,
                accounting: accounting as u8,
                start_ns,
                end_ns: 0,
                thread_id: THREAD.with(|id| *id),
                advice_phase: ADVICE.with(|context| context.get().0),
                circuit_index: ADVICE.with(|context| context.get().1),
                status: 0,
            });
            span.index = Some(index);
            span.previous = previous;
            span
        }

        /// Copy metadata for serialization after the timed operation.
        pub fn snapshot(&self) -> Snapshot {
            let events = self.events.lock().map(|v| v.clone()).unwrap_or_default();
            let dropped_events = self.dropped.load(Ordering::Relaxed);
            let timer_failures = self.timers.load(Ordering::Relaxed);
            let reentrant_events = self.reentrant.load(Ordering::Relaxed);
            let profile_valid = !self.events.is_poisoned()
                && events.len() < self.capacity
                && dropped_events == 0
                && timer_failures == 0
                && reentrant_events == 0
                && events
                    .iter()
                    .all(|e| e.status == 1 && e.end_ns >= e.start_ns);
            Snapshot {
                events,
                event_capacity: self.capacity,
                dropped_events,
                timer_failures,
                reentrant_events,
                profile_valid,
            }
        }

        /// Write only integer events and observer health. No proof data is accepted.
        /// `flush_wall_ns` measures encoding and the event-body write/flush; its own
        /// final metadata write is necessarily outside that measurement.
        pub fn write_json(&self, mut writer: impl Write) -> io::Result<()> {
            let start = Instant::now();
            let snapshot = self.snapshot();
            write!(writer, "{{\"schema\":\"recursion.phase-events/1\",\"event_capacity\":{},\"dropped_events\":{},\"timer_failures\":{},\"reentrant_events\":{},\"profile_valid\":{},\"events\":[",
                snapshot.event_capacity, snapshot.dropped_events, snapshot.timer_failures,
                snapshot.reentrant_events, snapshot.profile_valid)?;
            for (i, e) in snapshot.events.iter().enumerate() {
                if i != 0 {
                    writer.write_all(b",")?;
                }
                write!(writer, "{{\"id\":{},\"parent_id\":{},\"phase_id\":{},\"accounting\":{},\"start_ns\":{},\"end_ns\":{},\"thread_id\":{},\"advice_phase\":{},\"circuit_index\":{},\"status\":{}}}",
                    e.id, e.parent_id, e.phase_id, e.accounting, e.start_ns, e.end_ns, e.thread_id, e.advice_phase, e.circuit_index, e.status)?;
            }
            writer.flush()?;
            let flush_ns = start.elapsed().as_nanos();
            writeln!(writer, "],\"flush_wall_ns\":{flush_ns}}}")?;
            writer.flush()
        }
    }

    /// Thread-bound RAII guard. A dropped unfinished guard records an error.
    #[derive(Debug)]
    pub struct Span<'a> {
        recorder: &'a Recorder,
        index: Option<usize>,
        previous: (usize, u64),
        finished: bool,
        _thread: PhantomData<Rc<()>>,
    }
    impl Span<'_> {
        /// Mark successful completion before dropping the guard.
        pub fn finish(&mut self) {
            self.finished = true;
        }
    }
    impl Drop for Span<'_> {
        fn drop(&mut self) {
            let Some(index) = self.index else {
                return;
            };
            PARENT.with(|parent| parent.set(self.previous));
            let Some(_enter) = Enter::new() else {
                self.recorder.reentrant.fetch_add(1, Ordering::Relaxed);
                return;
            };
            let end = (self.recorder.clock)();
            if let Ok(mut events) = self.recorder.events.lock() {
                let event = &mut events[index];
                match end {
                    Some(end) if end >= event.start_ns => event.end_ns = end,
                    _ => {
                        self.recorder.timers.fetch_add(1, Ordering::Relaxed);
                    }
                }
                event.status = if std::thread::panicking() {
                    3
                } else if self.finished {
                    1
                } else {
                    2
                };
            }
        }
    }

    /// The process recorder. Initialize before the command timer to exclude allocation.
    pub fn global() -> &'static Recorder {
        static RECORDER: OnceLock<Recorder> = OnceLock::new();
        RECORDER.get_or_init(|| Recorder::new(65_536))
    }

    /// Flush when explicitly requested by the harness; unset means no file is opened.
    pub fn flush() -> io::Result<()> {
        if let Some(path) = std::env::var_os("MORIARTY_PROFILE_EVENTS") {
            let file = std::fs::File::create(path)?;
            global().write_json(io::BufWriter::new(file))?;
        }
        Ok(())
    }
}

#[cfg(feature = "recursion-phase-profile")]
pub use enabled::*;

/// Disabled observation performs no I/O, allocation, or clock access.
#[cfg(not(feature = "recursion-phase-profile"))]
#[inline(always)]
pub fn flush() -> std::io::Result<()> {
    Ok(())
}
