//! Circuit oracle for the ZKIR-in-K semantics (plan-iter3 M1/M1a).
//!
//! Loads a .zkir program and a preimage (same JSON as zkir-oracle), runs
//! `IrSource::preprocess`, builds the stdlib's `MidnightCircuit` with the
//! known instance and witness, runs the halo2 `MockProver` and verifies.
//! Prints one JSON line whose `outcome` is one of
//!   preprocess-error | witness-consistency-error | synthesis-error |
//!   constraint-failure | panic | accepted
//! classified by where the error arises:
//!   preprocess returns Err                       -> preprocess-error
//!   MockProver::run returns Err and the run logged the crate's
//!     "Misalignment between `prepare` and `synthesize`" error event
//!     (raised only by `mem_insert` / `pi_push` in ir_vm.rs)
//!                                                 -> witness-consistency-error
//!   MockProver::run returns any other Err         -> synthesis-error
//!   verify returns Err(failures)                  -> constraint-failure
//!   a panic anywhere after loading                -> panic
//!
//! `--inject INJECT.json` replaces (or adds) entries of the preprocessed
//! memory before the circuit is built, as `prove_unchecked` intends, for
//! Native values only: `{"reg": {"type": "Native", "value": "<decimal>"}}`.
//! `--pis PIS.json` likewise replaces the public-input vector (list of
//! decimal strings) and `--binding-input DEC` the binding input; both keep
//! the MockProver instance column equal to the (perturbed) `pis`, so a
//! disagreement surfaces in `pi_push` as a witness-consistency error.
//! `--instance PIS.json` sets only the instance column handed to MockProver,
//! leaving the witness alone, which is the way to reach a genuine
//! constraint failure (public-input cells disagree with the instance).
//! `--model-only` skips `preprocess` and MockProver and reports `k` and
//! `rows` from the stdlib cost model (unknown witness); outcome `model-only`.
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use midnight_proofs::circuit::Value;
use midnight_proofs::dev::MockProver;
use midnight_zk_stdlib::MidnightCircuit;
use midnight_zkir::ir_types::IrValue;
use midnight_zkir::{Identifier, IrSource, Preprocessed};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use tracing::Subscriber;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::{Context, SubscriberExt};
use transient_crypto::curve::Fr;
use transient_crypto::proofs::{KeyLocation, ProofPreimage};

const MISALIGNMENT: &str = "Misalignment between `prepare` and `synthesize`";

#[derive(Deserialize)]
struct PreimageJson {
    inputs: Vec<String>,
    binding_input: String,
    #[serde(default)]
    communications_commitment: Option<(String, String)>,
    #[serde(default)]
    private_transcript: Vec<String>,
    #[serde(default)]
    public_transcript_inputs: Vec<String>,
    #[serde(default)]
    public_transcript_outputs: Vec<String>,
}

#[derive(Deserialize)]
struct InjectValue {
    #[serde(rename = "type")]
    ty: String,
    value: String,
}

#[derive(Serialize, Default)]
struct Output {
    outcome: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    k: Option<u32>,
    /// 2^k rows actually allocated by MockProver (length of an advice column).
    #[serde(skip_serializing_if = "Option::is_none")]
    rows: Option<usize>,
    /// Usable rows (rows minus blinding), as reported by MockProver.
    #[serde(skip_serializing_if = "Option::is_none")]
    usable_rows: Option<usize>,
    elapsed_ms: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    preprocess_ms: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optimal_k_ms: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mockprover_run_ms: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verify_ms: Option<u128>,
    peak_rss_kb: Option<u64>,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    failures: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    injected: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pis: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trace_errors: Option<Vec<String>>,
}

fn fr_from_dec(s: &str) -> anyhow::Result<Fr> {
    let n: BigUint = s.parse().map_err(|e| anyhow::anyhow!("bad decimal {s}: {e}"))?;
    let mut bytes = n.to_bytes_le();
    if bytes.len() > 32 {
        anyhow::bail!("{s} does not fit in 32 bytes");
    }
    bytes.resize(32, 0);
    Fr::from_le_bytes(&bytes).ok_or_else(|| anyhow::anyhow!("{s} is not a canonical field element"))
}

fn fr_to_dec(x: &Fr) -> String {
    BigUint::from_bytes_le(&x.as_le_bytes()).to_string()
}

fn peak_rss_kb() -> Option<u64> {
    let s = std::fs::read_to_string("/proc/self/status").ok()?;
    s.lines()
        .find(|l| l.starts_with("VmHWM:"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())
}

/// A tracing layer that records the message of every ERROR-level event, so
/// the `mem_insert` / `pi_push` misalignment errors of `ir_vm.rs` can be told
/// apart from other synthesis errors.
struct ErrorCapture(Arc<Mutex<Vec<String>>>);

struct MsgVisitor(String);

impl tracing::field::Visit for MsgVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if !self.0.is_empty() {
            self.0.push(' ');
        }
        if field.name() == "message" {
            self.0.push_str(&format!("{value:?}"));
        } else {
            self.0.push_str(&format!("{}={value:?}", field.name()));
        }
    }
}

impl<S: Subscriber> Layer<S> for ErrorCapture {
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        if *event.metadata().level() == tracing::Level::ERROR {
            let mut v = MsgVisitor(String::new());
            event.record(&mut v);
            self.0.lock().unwrap().push(v.0);
        }
    }
}

struct Args {
    program: String,
    preimage: String,
    inject: Option<String>,
    pis: Option<String>,
    binding_input: Option<String>,
    instance: Option<String>,
    model_only: bool,
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut pos = Vec::new();
    let mut inject = None;
    let mut pis = None;
    let mut binding_input = None;
    let mut instance = None;
    let mut model_only = false;
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--inject" => {
                i += 1;
                inject = argv.get(i).cloned();
            }
            "--pis" => {
                i += 1;
                pis = argv.get(i).cloned();
            }
            "--binding-input" => {
                i += 1;
                binding_input = argv.get(i).cloned();
            }
            "--instance" => {
                i += 1;
                instance = argv.get(i).cloned();
            }
            "--model-only" => model_only = true,
            other => pos.push(other.to_string()),
        }
        i += 1;
    }
    if pos.len() != 2 {
        eprintln!(
            "usage: zkir-circuit-oracle PROGRAM.zkir PREIMAGE.json [--inject INJECT.json] [--pis PIS.json] [--binding-input DEC] [--instance PIS.json] [--model-only]"
        );
        std::process::exit(2);
    }
    Args { program: pos.remove(0), preimage: pos.remove(0), inject, pis, binding_input, instance, model_only }
}

fn load_preimage(path: &str) -> anyhow::Result<ProofPreimage> {
    let pj: PreimageJson = serde_json::from_reader(BufReader::new(File::open(path)?))?;
    let vec_fr = |v: &Vec<String>| -> anyhow::Result<Vec<Fr>> { v.iter().map(|s| fr_from_dec(s)).collect() };
    Ok(ProofPreimage {
        binding_input: fr_from_dec(&pj.binding_input)?,
        communications_commitment: match &pj.communications_commitment {
            Some((c, r)) => Some((fr_from_dec(c)?, fr_from_dec(r)?)),
            None => None,
        },
        inputs: vec_fr(&pj.inputs)?,
        private_transcript: vec_fr(&pj.private_transcript)?,
        public_transcript_inputs: vec_fr(&pj.public_transcript_inputs)?,
        public_transcript_outputs: vec_fr(&pj.public_transcript_outputs)?,
        key_location: KeyLocation(Cow::Borrowed("builtin")),
    })
}

/// Applies the `--inject` / `--pis` / `--binding-input` perturbations to the
/// preprocessed witness. Only `Native` registers can be expressed.
fn inject(args: &Args, pre: &mut Preprocessed, out: &mut Output) -> anyhow::Result<()> {
    let mut injected = Vec::new();
    if let Some(path) = &args.inject {
        let entries: BTreeMap<String, InjectValue> = serde_json::from_reader(BufReader::new(File::open(path)?))?;
        for (reg, iv) in entries {
            if iv.ty != "Native" {
                anyhow::bail!("inject {reg}: only Native values can be injected (got {})", iv.ty);
            }
            let v = fr_from_dec(&iv.value)?;
            let old = pre.memory.insert(Identifier(reg.clone()), IrValue::Native(v));
            injected.push(format!(
                "{reg}: {} -> Native({})",
                old.map(|o| format!("{o:?}")).unwrap_or_else(|| "<absent>".into()),
                iv.value
            ));
        }
    }
    if let Some(path) = &args.pis {
        let v: Vec<String> = serde_json::from_reader(BufReader::new(File::open(path)?))?;
        let new: Vec<Fr> = v.iter().map(|s| fr_from_dec(s)).collect::<anyhow::Result<_>>()?;
        injected.push(format!("pis: {} -> {}", pre.pis.len(), new.len()));
        pre.pis = new.into_iter().map(|f| f.0).collect();
    }
    if let Some(b) = &args.binding_input {
        let old = Fr(pre.binding_input);
        pre.binding_input = fr_from_dec(b)?.0;
        injected.push(format!("binding_input: {} -> {b}", fr_to_dec(&old)));
    }
    if !injected.is_empty() {
        out.injected = Some(injected);
    }
    Ok(())
}

fn run(args: &Args, ir: &IrSource, preimage: &ProofPreimage, errors: &Arc<Mutex<Vec<String>>>, out: &mut Output) {
    if args.model_only {
        // The cost model synthesises with an unknown witness; the preimage is not used.
        let t = Instant::now();
        let model = ir.model();
        out.optimal_k_ms = Some(t.elapsed().as_millis());
        out.k = Some(model.k() as u32);
        out.rows = Some(model.rows());
        out.outcome = "model-only";
        out.message = format!("cost model: k={} rows={} (no preprocess, no MockProver run)", model.k(), model.rows());
        return;
    }

    let t = Instant::now();
    let mut pre = match ir.preprocess(preimage) {
        Ok(p) => p,
        Err(e) => {
            out.outcome = "preprocess-error";
            out.message = format!("{e:#}");
            out.preprocess_ms = Some(t.elapsed().as_millis());
            return;
        }
    };
    out.preprocess_ms = Some(t.elapsed().as_millis());

    if let Err(e) = inject(args, &mut pre, out) {
        out.outcome = "preprocess-error";
        out.message = format!("inject: {e:#}");
        return;
    }
    out.pis = Some(pre.pis.iter().map(|x| fr_to_dec(&Fr(*x))).collect());

    let t = Instant::now();
    let k = midnight_zk_stdlib::optimal_k(ir);
    out.optimal_k_ms = Some(t.elapsed().as_millis());
    out.k = Some(k);

    let mut instance = pre.pis.clone();
    if let Some(path) = &args.instance {
        let v: Vec<String> = match File::open(path)
            .map_err(anyhow::Error::from)
            .and_then(|f| Ok(serde_json::from_reader::<_, Vec<String>>(BufReader::new(f))?))
        {
            Ok(v) => v,
            Err(e) => {
                out.outcome = "preprocess-error";
                out.message = format!("instance: {e:#}");
                return;
            }
        };
        instance = match v.iter().map(|s| fr_from_dec(s).map(|f| f.0)).collect::<anyhow::Result<Vec<_>>>() {
            Ok(i) => i,
            Err(e) => {
                out.outcome = "preprocess-error";
                out.message = format!("instance: {e:#}");
                return;
            }
        };
        out.injected.get_or_insert_with(Vec::new).push(format!("instance: {} -> {}", pre.pis.len(), instance.len()));
    }
    let circuit = MidnightCircuit::new(ir, Value::known(instance.clone()), Value::known(pre), Some(k));

    // Instance columns as `midnight_zk_stdlib::prove` lays them out: the
    // committed-instance column (empty for IrSource) and the public inputs.
    let t = Instant::now();
    let prover = match MockProver::run(&circuit, vec![vec![], instance]) {
        Ok(p) => p,
        Err(e) => {
            out.mockprover_run_ms = Some(t.elapsed().as_millis());
            let inner = circuit.take_error().map(|e| format!("{e:?}"));
            let logged = errors.lock().unwrap().clone();
            let misaligned = logged.iter().any(|m| m.contains(MISALIGNMENT));
            out.outcome = if misaligned { "witness-consistency-error" } else { "synthesis-error" };
            out.message = match inner {
                Some(i) => format!("{e:?}; relation error: {i}"),
                None => format!("{e:?}"),
            };
            if !logged.is_empty() {
                out.trace_errors = Some(logged);
            }
            return;
        }
    };
    out.mockprover_run_ms = Some(t.elapsed().as_millis());
    out.rows = prover.advice().first().map(|c| c.len());
    out.usable_rows = Some(prover.usable_rows().end);

    let t = Instant::now();
    match prover.verify() {
        Ok(()) => {
            out.verify_ms = Some(t.elapsed().as_millis());
            out.outcome = "accepted";
            out.message = "MockProver::verify ok".into();
        }
        Err(failures) => {
            out.verify_ms = Some(t.elapsed().as_millis());
            out.outcome = "constraint-failure";
            out.message = format!("{} constraint failure(s)", failures.len());
            out.failures = Some(failures.iter().map(|f| f.to_string()).collect());
        }
    }
    let logged = errors.lock().unwrap().clone();
    if !logged.is_empty() {
        out.trace_errors = Some(logged);
    }
}

fn main() {
    let args = parse_args();
    let start = Instant::now();

    let errors: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let subscriber = tracing_subscriber::registry().with(ErrorCapture(errors.clone()));
    tracing::subscriber::set_global_default(subscriber).expect("set tracing subscriber");

    let ir = match File::open(&args.program).map_err(anyhow::Error::from).and_then(|f| Ok(IrSource::load(BufReader::new(f))?)) {
        Ok(ir) => ir,
        Err(e) => {
            eprintln!("load error: {e:#}");
            std::process::exit(2);
        }
    };
    let preimage = match load_preimage(&args.preimage) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("preimage error: {e:#}");
            std::process::exit(2);
        }
    };

    // Capture the panic message instead of letting the default hook print it.
    let panic_msg: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    {
        let pm = panic_msg.clone();
        std::panic::set_hook(Box::new(move |info| {
            let msg = info
                .payload()
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| info.payload().downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "<non-string panic payload>".into());
            let loc = info.location().map(|l| format!(" at {}:{}", l.file(), l.line())).unwrap_or_default();
            *pm.lock().unwrap() = Some(format!("{msg}{loc}"));
        }));
    }

    let mut out = Output { outcome: "panic", ..Default::default() };
    let res = catch_unwind(AssertUnwindSafe(|| {
        let mut o = Output::default();
        run(&args, &ir, &preimage, &errors, &mut o);
        o
    }));
    match res {
        Ok(o) => out = o,
        Err(_) => {
            out.outcome = "panic";
            out.message = panic_msg.lock().unwrap().clone().unwrap_or_else(|| "panic".into());
            let logged = errors.lock().unwrap().clone();
            if !logged.is_empty() {
                out.trace_errors = Some(logged);
            }
        }
    }
    out.elapsed_ms = start.elapsed().as_millis();
    out.peak_rss_kb = peak_rss_kb();
    println!("{}", serde_json::to_string(&out).expect("serialize output"));
}
