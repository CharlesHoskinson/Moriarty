//! Linux process counters. All proc parsing and sampling stays in the harness.
//! Proc reads are not atomic: an RSS read is bracketed and is attributed only
//! when its entire read interval lies strictly inside the requested span.
use std::{
    cell::Cell,
    collections::BTreeMap,
    fs,
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use midnight_proofs::profile::{self, BoundaryObserver, Event};
use serde_json::{Value, json};

type Counter = Result<u64, String>;

thread_local! { static EXPECTED_REJECTION: Cell<bool> = const { Cell::new(false) }; }
pub(crate) fn expected_rejection(operation: impl FnOnce() -> bool) -> bool {
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            EXPECTED_REJECTION.set(self.0);
        }
    }
    let _restore = Restore(EXPECTED_REJECTION.replace(true));
    operation()
}

pub fn parse_status(text: &str, field: &str) -> Counter {
    let line = text
        .lines()
        .find_map(|line| line.strip_prefix(&format!("{field}:")))
        .ok_or_else(|| format!("/proc/self/status: {field} missing"))?;
    let mut words = line.split_whitespace();
    let value = words.next().and_then(|s| s.parse::<u64>().ok());
    if words.next() != Some("kB") || words.next().is_some() {
        return Err(format!("/proc/self/status: {field} invalid unit"));
    }
    value
        .and_then(|n| n.checked_mul(1024))
        .ok_or_else(|| format!("/proc/self/status: {field} invalid or overflowed"))
}

pub fn parse_cpu_ticks(text: &str) -> Counter {
    // comm (field 2) may contain spaces and ')' characters. Fields 14/15 are
    // utime/stime, not cutime/cstime (16/17), and /proc/self/stat sums threads.
    let rest = text
        .rsplit_once(") ")
        .map(|(_, rest)| rest)
        .ok_or_else(|| "/proc/self/stat: malformed comm".to_string())?;
    let words: Vec<_> = rest.split_whitespace().collect();
    let number = |index| words.get(index).and_then(|s: &&str| s.parse::<u64>().ok());
    number(11)
        .zip(number(12))
        .and_then(|(u, s)| u.checked_add(s))
        .ok_or_else(|| "/proc/self/stat: utime/stime missing, invalid or overflowed".into())
}

pub fn parse_io(text: &str, field: &str) -> Counter {
    text.lines()
        .find_map(|line| line.strip_prefix(&format!("{field}:")))
        .and_then(|s| s.trim().parse().ok())
        .ok_or_else(|| format!("/proc/self/io: {field} missing or invalid"))
}

fn read(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("{path}: {:?}", e.kind()))
}

#[derive(Clone, Debug)]
pub struct Counters {
    rss: Counter,
    hwm: Counter,
    cpu: Counter,
    read: Counter,
    write: Counter,
}
impl Counters {
    pub fn parse(
        status: Result<String, String>,
        stat: Result<String, String>,
        io: Result<String, String>,
    ) -> Self {
        Self {
            rss: status.clone().and_then(|s| parse_status(&s, "VmRSS")),
            hwm: status.and_then(|s| parse_status(&s, "VmHWM")),
            cpu: stat.and_then(|s| parse_cpu_ticks(&s)),
            read: io.clone().and_then(|s| parse_io(&s, "read_bytes")),
            write: io.and_then(|s| parse_io(&s, "write_bytes")),
        }
    }
    fn read() -> Self {
        Self::parse(
            read("/proc/self/status"),
            read("/proc/self/stat"),
            read("/proc/self/io"),
        )
    }
}

#[derive(Debug)]
pub struct RssSample {
    start: u64,
    end: u64,
    rss: Counter,
    hwm: Counter,
}
impl RssSample {
    pub fn new(start: u64, end: u64, rss: Counter, hwm: Counter) -> Self {
        Self {
            start,
            end,
            rss,
            hwm,
        }
    }
}

#[derive(Debug)]
pub struct Observation {
    ticks_per_second: u64,
    capacity: usize,
    boundaries: BTreeMap<u64, Counters>,
    samples: Vec<RssSample>,
    dropped: u64,
    failures: u64,
    expected_rejections: Vec<u64>,
}
impl Observation {
    pub fn new(ticks_per_second: u64, capacity: usize) -> Self {
        Self {
            ticks_per_second,
            capacity,
            boundaries: BTreeMap::new(),
            samples: Vec::new(),
            dropped: 0,
            failures: 0,
            expected_rejections: Vec::new(),
        }
    }
    pub fn boundary(&mut self, _span_id: u64, at: u64, _entry: bool, counters: Counters) {
        if self.boundaries.len() >= self.capacity.saturating_mul(2) {
            self.failures += 1;
            return;
        }
        if counters.rss.is_err() || counters.hwm.is_err() {
            self.failures += 1;
        }
        if self.boundaries.insert(at, counters).is_some() {
            self.failures += 1; // Ambiguous clock boundary; never silently qualify.
        }
    }
    pub fn sample(&mut self, sample: RssSample) {
        if sample.rss.is_err() || sample.hwm.is_err() || sample.start > sample.end {
            self.failures += 1;
        }
        if self.samples.len() >= self.capacity {
            self.dropped += 1;
        } else {
            self.samples.push(sample);
        }
    }
    pub fn valid(&self) -> bool {
        self.failures == 0 && self.dropped == 0
    }
    pub fn dropped_samples(&self) -> u64 {
        self.dropped
    }

    /// Inclusive counters for a measured interval. Exclusive residuals use the
    /// same recorded boundary snapshots, never wall-proportional CPU estimates.
    pub fn metrics(&self, start: u64, end: u64, isolated: bool) -> Value {
        let missing = Counters::parse(
            Err("boundary not observed".into()),
            Err("boundary not observed".into()),
            Err("boundary not observed".into()),
        );
        let entry = self.boundaries.get(&start).unwrap_or(&missing);
        let exit = self.boundaries.get(&end).unwrap_or(&missing);
        let mut points: Vec<_> = self
            .samples
            .iter()
            .filter(|s| start < s.start && s.start <= s.end && s.end < end)
            .collect();
        points.sort_by_key(|s| s.end);
        points.dedup_by_key(|s| s.end);
        let rss_error = entry
            .rss
            .as_ref()
            .err()
            .or(exit.rss.as_ref().err())
            .or_else(|| points.iter().find_map(|s| s.rss.as_ref().err()));
        let values: Vec<_> = points
            .iter()
            .filter_map(|s| {
                s.rss
                    .as_ref()
                    .ok()
                    .map(|v| json!({"at_monotonic_ns":s.end,"rss_bytes":v}))
            })
            .collect();
        let peak = if rss_error.is_none() {
            entry
                .rss
                .as_ref()
                .ok()
                .into_iter()
                .chain(exit.rss.as_ref().ok())
                .chain(points.iter().filter_map(|s| s.rss.as_ref().ok()))
                .copied()
                .max()
        } else {
            None
        };
        let delta = peak
            .zip(entry.rss.as_ref().ok())
            .map(|(p, e)| p.saturating_sub(*e));
        let net = entry
            .rss
            .as_ref()
            .ok()
            .zip(exit.rss.as_ref().ok())
            .map(|(e, x)| i128::from(*x) - i128::from(*e));
        let mut times = vec![start];
        times.extend(points.iter().map(|s| s.end));
        times.push(end);
        let gap = if rss_error.is_none() {
            times
                .windows(2)
                .map(|w| w[1].saturating_sub(w[0]))
                .max()
                .map(seconds)
        } else {
            None
        };
        let cpu = if !isolated {
            Err("overlapping jobs/worker diagnostics: process CPU is unattributable".into())
        } else if self.ticks_per_second == 0 {
            Err("_SC_CLK_TCK unavailable".into())
        } else {
            difference(&entry.cpu, &exit.cpu, "process CPU")
                .map(|v| v as f64 / self.ticks_per_second as f64)
        };
        let storage = if isolated {
            difference(&entry.read, &exit.read, "storage read").and_then(|r| {
                difference(&entry.write, &exit.write, "storage write").map(|w| (r, w))
            })
        } else {
            Err("overlapping jobs: process I/O is unattributable".into())
        };
        let mut reasons = vec![
            "logical_copy_payload_bytes: explicit copy counting unavailable".to_string(),
            "logical_input_footprint_bytes: explicit footprint counting unavailable".to_string(),
            "logical_output_footprint_bytes: explicit footprint counting unavailable".to_string(),
            "dram_read_bytes, dram_write_bytes: hardware memory-controller counters unavailable"
                .to_string(),
            "h2d_bytes, d2h_bytes, d2d_bytes: CPU-only workload; device counters unavailable"
                .to_string(),
        ];
        if let Err(reason) = &storage {
            reasons.push(format!("storage_read_bytes, storage_write_bytes: {reason}"));
        }
        json!({
            "cpu_core_seconds":cpu.as_ref().ok(), "cpu_null_reason":cpu.as_ref().err(),
            "cpu_scope":if cpu.is_ok() { "isolated_process_all_threads" } else { "unattributable" },
            "rss":{
                "entry_bytes":entry.rss.as_ref().ok(), "exit_bytes":exit.rss.as_ref().ok(),
                "sampled_peak_bytes":peak, "peak_delta_bytes":delta, "net_delta_bytes":net,
                "samples":values.len(), "sample_points":values, "max_sample_gap_seconds":gap,
                "method":if rss_error.is_none() { "proc_status" } else { "unavailable" },
                "quality":if rss_error.is_some() { "unavailable" } else if points.len() < 2 { "boundary_only" } else { "sampled_lower_bound" },
                "reason":rss_error.cloned().or_else(|| if points.len() < 2 { Some("Fewer than two strictly interior samples; boundary-only lower bound".into()) } else { None }),
                "certifies_instantaneous":false
            },
            "bytes":{
                "logical_copy_payload_bytes":null,"logical_input_footprint_bytes":null,"logical_output_footprint_bytes":null,
                "storage_read_bytes":storage.as_ref().ok().map(|v|v.0),"storage_write_bytes":storage.as_ref().ok().map(|v|v.1),
                "dram_read_bytes":null,"dram_write_bytes":null,"h2d_bytes":null,"d2h_bytes":null,"d2d_bytes":null,
                "logical_coverage":"unavailable", "storage_method":if storage.is_ok() { "proc_pid_io" } else { "unavailable" },
                "dram_method":"unavailable","device_method":"cpu_only","null_reasons":reasons,"scope":"diagnostic",
                "notes":"read_bytes/write_bytes are storage I/O, not rchar/wchar, logical copies or DRAM traffic."
            }
        })
    }

    pub fn summary(&self) -> Value {
        let hwm = self
            .boundaries
            .values()
            .filter_map(|c| c.hwm.as_ref().ok())
            .chain(self.samples.iter().filter_map(|s| s.hwm.as_ref().ok()))
            .copied()
            .max();
        json!({"pid":std::process::id(),"sampler_valid":self.valid(),"sampler_failures":self.failures,
            "expected_rejections":self.expected_rejections,
            "dropped_samples":self.dropped,"sample_count":self.samples.len(),
            "max_rss_bytes":hwm,"max_rss_scope":"process_lifetime",
            "max_rss_null_reason":if hwm.is_none() { Some("VmHWM unavailable") } else { None },
            "cpu_method":"proc_pid_stat","ticks_per_second":if self.ticks_per_second == 0 { None } else { Some(self.ticks_per_second) },
            "cpu_method_reason":if self.ticks_per_second == 0 { Some("_SC_CLK_TCK unavailable") } else { None },
            "cgroup_memory_peak_bytes":null,"cgroup_memory_null_reason":"cgroup counter not collected",
            "device_live_peak_bytes":null,"device_reserved_peak_bytes":null,"device_memory_null_reason":"CPU-only workload"})
    }
}
fn difference(start: &Counter, end: &Counter, name: &str) -> Counter {
    match (start, end) {
        (Ok(a), Ok(b)) => b
            .checked_sub(*a)
            .ok_or_else(|| format!("{name} counter regressed")),
        (Err(e), _) | (_, Err(e)) => Err(e.clone()),
    }
}
fn seconds(ns: u64) -> f64 {
    ns as f64 / 1_000_000_000.0
}

#[derive(Debug)]
pub struct Collector(Mutex<Observation>);
impl BoundaryObserver for Collector {
    fn boundary(&self, event: &Event, entry: bool) {
        let counters = Counters::read();
        if let Ok(mut o) = self.0.lock() {
            o.boundary(
                event.id,
                if entry { event.start_ns } else { event.end_ns },
                entry,
                counters,
            );
            if !entry && event.status == 2 && event.phase_id == 19 && EXPECTED_REJECTION.get() {
                if o.expected_rejections.len() < o.capacity {
                    o.expected_rejections.push(event.id);
                } else {
                    o.failures += 1;
                }
            }
        }
    }
}
impl Collector {
    /// Expected transcript errors do not erase proof measurements. Other failed,
    /// unfinished, overflowing or clock-corrupted spans remain invalid.
    pub fn accepts(&self, snapshot: &profile::Snapshot) -> bool {
        let Ok(o) = self.0.lock() else {
            return false;
        };
        snapshot.profile_valid
            || (!o.expected_rejections.is_empty()
                && !snapshot.events.is_empty()
                && snapshot.events.len() < snapshot.event_capacity
                && snapshot.dropped_events == 0
                && snapshot.timer_failures == 0
                && snapshot.reentrant_events == 0
                && snapshot.events.iter().all(|e| {
                    e.end_ns > e.start_ns
                        && (e.status == 1
                            || (e.status == 2 && o.expected_rejections.contains(&e.id)))
                }))
    }
    pub fn new(capacity: usize) -> Arc<Self> {
        // sysconf has no pointer arguments and does not mutate Rust memory.
        let ticks = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };
        Arc::new(Self(Mutex::new(Observation::new(
            u64::try_from(ticks).unwrap_or(0),
            capacity,
        ))))
    }
    pub fn report(&self, events: &[Event]) -> Value {
        let Ok(o) = self.0.lock() else {
            return json!({"sampler_valid":false,"reason":"counter lock poisoned","measurements":[]});
        };
        let mut report = o.summary();
        let mut measurements = Vec::new();
        let mut cuts_by_owner: BTreeMap<u64, Vec<(u64, u64)>> = BTreeMap::new();
        for child in events.iter().filter(|e| e.accounting == 1) {
            let mut parent = child.parent_id;
            while let Some(ancestor) = parent.checked_sub(1).and_then(|id| events.get(id as usize))
            {
                if ancestor.accounting == 1 {
                    cuts_by_owner
                        .entry(parent)
                        .or_default()
                        .push((child.start_ns, child.end_ns));
                }
                parent = ancestor.parent_id;
            }
        }
        let proofs: Vec<_> = events.iter().filter(|e| e.phase_id == 4).collect();
        let mut overlapping = Vec::new();
        for (i, a) in proofs.iter().enumerate() {
            for b in proofs.iter().skip(i + 1) {
                if !descendant(a, b.id, events)
                    && !descendant(b, a.id, events)
                    && a.start_ns < b.end_ns
                    && b.start_ns < a.end_ns
                {
                    overlapping.push((a.start_ns.max(b.start_ns), a.end_ns.min(b.end_ns)));
                }
            }
        }
        for e in events
            .iter()
            .filter(|e| e.end_ns >= e.start_ns && e.status != 0)
        {
            // Worker diagnostics may overlap other workers. No per-worker CPU
            // or I/O delta is attributed from a process-wide counter.
            let isolated = e.phase_id != 22
                && !overlapping
                    .iter()
                    .any(|(start, end)| e.start_ns < *end && *start < e.end_ns);
            let mut intervals = vec![(e.start_ns, e.end_ns)];
            if e.accounting == 1 {
                let mut cuts = cuts_by_owner.remove(&e.id).unwrap_or_default();
                cuts.sort_unstable();
                let mut cursor = e.start_ns;
                for (start, end) in cuts
                    .into_iter()
                    .chain(std::iter::once((e.end_ns, e.end_ns)))
                {
                    if cursor < start {
                        intervals.push((cursor, start));
                    }
                    cursor = cursor.max(end);
                }
            }
            intervals.sort_unstable();
            intervals.dedup();
            for (start, end) in intervals {
                measurements.push(json!({"event_id":e.id,"start_ns":start,"end_ns":end,"metrics":o.metrics(start,end,isolated)}));
            }
        }
        report["measurements"] = json!(measurements);
        report
    }
}
fn descendant(event: &Event, ancestor: u64, events: &[Event]) -> bool {
    let mut parent = event.parent_id;
    for _ in 0..events.len() {
        if parent == 0 {
            return false;
        }
        if parent == ancestor {
            return true;
        }
        parent = events.get(parent as usize - 1).map_or(0, |e| e.parent_id);
    }
    false
}

/// Fixed-period sampler with bounded storage and prompt, joined shutdown.
/// Missed deadlines are skipped rather than producing a burst of fake samples.
pub struct Sampler {
    stop: mpsc::Sender<()>,
    thread: Option<JoinHandle<Value>>,
}
impl Sampler {
    pub fn start(collector: Arc<Collector>, period: Duration) -> std::io::Result<Self> {
        if period.is_zero() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "zero sampler period",
            ));
        }
        let (stop, receiver) = mpsc::channel();
        let thread = thread::Builder::new().name("profile-rss".into()).spawn(move || {
            let cpu_start = thread_cpu();
            let mut deadline = Instant::now() + period;
            let mut read_wall_ns = 0_u64;
            loop {
                match receiver.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
                    Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    Err(mpsc::RecvTimeoutError::Timeout) => {}
                }
                let start = profile::monotonic_ns();
                let status = read("/proc/self/status");
                let rss = status.clone().and_then(|s| parse_status(&s, "VmRSS"));
                let hwm = status.and_then(|s| parse_status(&s, "VmHWM"));
                let end = profile::monotonic_ns();
                if let Ok(mut o) = collector.0.lock() {
                    if let (Some(start), Some(end)) = (start, end) {
                        read_wall_ns = read_wall_ns.saturating_add(end.saturating_sub(start));
                        o.sample(RssSample::new(start, end, rss, hwm));
                    } else { o.failures += 1; }
                }
                deadline += period;
                let now = Instant::now();
                if deadline <= now { deadline = now + period; }
            }
            let cpu = difference(&cpu_start, &thread_cpu(), "sampler CPU");
            json!({"sampler_rate_hz":1.0/period.as_secs_f64(),"sampler_read_wall_seconds":seconds(read_wall_ns),
                "sampler_core_seconds":cpu.as_ref().ok().map(|n| seconds(*n)),"sampler_cpu_null_reason":cpu.err()})
        })?;
        Ok(Self {
            stop,
            thread: Some(thread),
        })
    }
    pub fn stop(mut self) -> Value {
        let _ = self.stop.send(());
        self.thread
            .take()
            .unwrap()
            .join()
            .unwrap_or_else(|_| json!({"sampler_valid":false,"reason":"sampler thread panicked"}))
    }
}
impl Drop for Sampler {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}
fn thread_cpu() -> Counter {
    let mut time = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // Only measures observer overhead. Phase CPU always uses /proc/self/stat.
    if unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut time) } != 0 {
        return Err("sampler thread CPU clock unavailable".into());
    }
    u64::try_from(time.tv_sec)
        .ok()
        .and_then(|s| s.checked_mul(1_000_000_000))
        .and_then(|s| {
            u64::try_from(time.tv_nsec)
                .ok()
                .and_then(|n| s.checked_add(n))
        })
        .ok_or_else(|| "sampler thread CPU clock invalid".into())
}
