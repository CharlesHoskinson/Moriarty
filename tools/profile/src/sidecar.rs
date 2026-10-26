//! Public-metadata event sidecars. Raw observations remain unqualified until
//! workload identity, coverage and paired overhead checks are complete.
use serde_json::{Value, json};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Seek, Write},
    path::Path,
    time::Instant,
};

#[cfg(feature = "recursion-phase-profile")]
use crate::counters::{Collector, Sampler};

pub struct Sidecar {
    file: File,
    started: Instant,
    #[cfg(feature = "recursion-phase-profile")]
    collector: std::sync::Arc<Collector>,
    #[cfg(feature = "recursion-phase-profile")]
    sampler: Option<Sampler>,
    #[cfg(feature = "recursion-phase-profile")]
    sampler_error: Option<String>,
}
impl Sidecar {
    /// Reserve a fresh path before proving. A capped/interrupted invocation keeps
    /// an explicit incomplete marker rather than a success-shaped empty record.
    pub fn start(path: &Path) -> io::Result<Self> {
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
        let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
        writeln!(
            file,
            "{}",
            json!({"schema":"recursion.phase-incomplete/1","profile_valid":false,"reason":"Invocation has not completed"})
        )?;
        file.sync_all()?;
        #[cfg(feature = "recursion-phase-profile")]
        let (collector, sampler, sampler_error) = {
            let collector = Collector::new(65_536);
            let installed = midnight_proofs::profile::global().set_observer(collector.clone());
            let (sampler, error) = if installed {
                match Sampler::start(collector.clone(), std::time::Duration::from_millis(10)) {
                    Ok(sampler) => (Some(sampler), None),
                    Err(e) => (
                        None,
                        Some(format!("RSS sampler could not start: {:?}", e.kind())),
                    ),
                }
            } else {
                (None, Some("Boundary observer already installed".into()))
            };
            (collector, sampler, error)
        };
        Ok(Self {
            file,
            started: Instant::now(),
            #[cfg(feature = "recursion-phase-profile")]
            collector,
            #[cfg(feature = "recursion-phase-profile")]
            sampler,
            #[cfg(feature = "recursion-phase-profile")]
            sampler_error,
        })
    }

    /// Call only after the command guard and joined worker spans close. Caller
    /// errors are reduced to a fixed status; error strings and environments are
    /// never serialized. This returns writer errors separately from proof errors.
    pub fn finish(mut self, workload: Value, success: bool) -> io::Result<()> {
        public_metadata(&workload)?;
        let wall_ns = self.started.elapsed().as_nanos();
        let flush_start = Instant::now();
        #[cfg(feature = "recursion-phase-profile")]
        let mut raw = {
            let overhead = self.sampler.take().map(Sampler::stop).unwrap_or_else(|| json!({
                "sampler_core_seconds":null,"sampler_cpu_null_reason":"sampler did not start",
                "sampler_read_wall_seconds":null,"sampler_read_wall_null_reason":"sampler did not start",
                "sampler_rate_hz":null,"sampler_rate_null_reason":"sampler did not start"
            }));
            let recorder = midnight_proofs::profile::global();
            let mut bytes = Vec::new();
            recorder.write_json(&mut bytes)?;
            let mut raw: Value = serde_json::from_slice(&bytes)?;
            let snapshot = recorder.snapshot();
            let mut observation = self.collector.report(&snapshot.events);
            observation["sampler_overhead"] = json!(overhead);
            if let Some(error) = &self.sampler_error {
                observation["sampler_valid"] = json!(false);
                observation["reason"] = json!(error);
            }
            if observation["sampler_overhead"]["sampler_valid"] == false {
                observation["sampler_valid"] = json!(false);
            }
            raw["profile_valid"] = json!(
                self.collector.accepts(&snapshot)
                    && observation["sampler_valid"] == true
                    && success
                    && (observation["expected_rejections"]
                        .as_array()
                        .is_none_or(|v| v.is_empty())
                        || workload["verification"]["negative_controls"] == "pass")
            );
            raw["observation"] = observation;
            raw
        };
        #[cfg(not(feature = "recursion-phase-profile"))]
        let mut raw = json!({"schema":"recursion.phase-events/1","event_capacity":0,
            "dropped_events":0,"timer_failures":0,"reentrant_events":0,"profile_valid":false,
            "events":[],"flush_wall_ns":0});
        raw["workload"] = workload;
        raw["workload"]["profile_enabled"] = json!(cfg!(feature = "recursion-phase-profile"));
        raw["workload"]["command_wall_ns"] = json!(wall_ns);
        raw["workload"]["exit"] = json!(if success { 0 } else { 1 });
        raw["workload"]["qualification"] = json!("not_run");
        raw["workload"]["note"] = json!(
            "Raw event evidence; not a qualified recursion.phase-profile/1 sidecar. Off runs intentionally have no phase counters."
        );
        self.file.rewind()?;
        // Keep the measured body flush separate from its own metadata write.
        let mut encoded = serde_json::to_vec(&raw)?;
        encoded.pop();
        self.file.write_all(&encoded)?;
        self.file.flush()?;
        write!(
            self.file,
            ",\"sidecar_flush_wall_ns\":{}}}\n",
            flush_start.elapsed().as_nanos()
        )?;
        let end = self.file.stream_position()?;
        self.file.set_len(end)?;
        self.file.sync_all()
    }
}

// Reject secret-shaped metadata before storing any of it. This mirrors the
// reducer's conservative field/assignment exclusions, without logging values.
fn public_metadata(value: &Value) -> io::Result<()> {
    let forbidden_key = |key: &str| {
        let key = key.to_ascii_lowercase().replace(['_', '-'], "");
        [
            "witness",
            "seed",
            "wallet",
            "privatekey",
            "secret",
            "environment",
        ]
        .iter()
        .any(|word| key.contains(word))
            || key == "env"
            || key == "scalar"
            || key == "scalars"
            || (key.contains("challenge") && !key.contains("challengecount"))
    };
    let denied = || {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "secret-shaped profile metadata rejected",
        )
    };
    match value {
        Value::Object(fields) => {
            for (key, value) in fields {
                if forbidden_key(key) {
                    return Err(denied());
                }
                public_metadata(value)?;
            }
        }
        Value::Array(values) => {
            for value in values {
                public_metadata(value)?;
            }
        }
        Value::String(text) => {
            let compact: String = text
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .to_ascii_lowercase();
            for word in [
                "witness",
                "witness_values",
                "rng_seed",
                "seed",
                "wallet",
                "private_key",
                "secret",
                "environment",
                "openai_api_key",
            ] {
                if compact.contains(&format!("{word}=")) || compact.contains(&format!("{word}:")) {
                    return Err(denied());
                }
            }
        }
        _ => {}
    }
    Ok(())
}
