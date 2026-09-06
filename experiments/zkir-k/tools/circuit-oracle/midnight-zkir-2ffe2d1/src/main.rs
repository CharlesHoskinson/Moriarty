//! Circuit oracle for the ZKIR-in-K semantics (plan-iter3 M1/M1a).
//!
//! Loads a .zkir program and a preimage (same JSON as zkir-oracle), runs
//! `IrSource::preprocess`, builds the stdlib's `MidnightCircuit` with the
//! known instance and witness, runs the halo2 `MockProver` and verifies.
//! Prints one JSON line whose `outcome` is one of
//!   preprocess-error | witness-consistency-error | synthesis-error |
//!   instance-length-mismatch | constraint-failure | panic | accepted
//! classified by where the error arises:
//!   preprocess returns Err                       -> preprocess-error
//!   MockProver::run returns Err, the relation stashed the
//!     `error_if_known_and` error of `mem_insert` / `pi_push` (ir_vm.rs)
//!     AND the run logged the crate's "Misalignment between `prepare` and
//!     `synthesize`" error event; both are required, the event alone or the
//!     error alone is not enough           -> witness-consistency-error
//!   MockProver::run returns any other Err         -> synthesis-error
//!   the instance column handed to MockProver has a different length from
//!     the number of `pi_push` calls the circuit made (the length of the
//!     preprocessed `pis`): the surplus instance cells are constrained by
//!     nothing and a shortened column is a constraint failure on the first
//!     missing cell, so neither is a verdict on the witness
//!                                                 -> instance-length-mismatch
//!   verify returns Err(failures)                  -> constraint-failure
//!   a panic anywhere after loading                -> panic
//! A malformed `--inject`, `--pis` or `--instance` file (unparseable JSON, a
//! non-Native type, a non-canonical decimal) is `inject-error`, printed as a
//! JSON line with exit status 2: it is a harness error, not an outcome of
//! the circuit, and is never counted as a preprocess error.
//!
//! `--inject INJECT.json` replaces (or adds) entries of the preprocessed
//! memory before the circuit is built, as `prove_unchecked` intends, for
//! Native values only: `{"reg": {"type": "Native", "value": "<decimal>"}}`.
//! `--pis PIS.json` likewise replaces the public-input vector (list of
//! decimal strings) and `--binding-input DEC` the binding input. Both are
//! witness-side perturbations: the MockProver instance column is set equal
//! to the (perturbed) `pis`, so a changed entry surfaces in `pi_push` as a
//! witness-consistency error (`W`), never as a constraint failure; a `--pis`
//! vector of a different length is `instance-length-mismatch`. They do not
//! model a verifier that is handed a wrong statement: for that use
//! `--instance`. `--instance PIS.json` sets only the instance column handed
//! to MockProver, leaving the witness alone, which is the way to reach a
//! genuine constraint failure (public-input cells disagree with the
//! instance); a vector of the wrong length is `instance-length-mismatch`,
//! because a longer vector would be `accepted` (the extra cells are
//! unconstrained) and a shorter one `constraint-failure`, neither of which
//! says anything about the witness.
//! `--model-only` skips `preprocess` and MockProver and reports `k` and
//! `rows` from the stdlib cost model (unknown witness); outcome `model-only`.
//!
//! Provability modes (plan-iter3 M5b). Both read the KZG parameters
//! `bls_midnight_2p{k}` from `--params DIR`, else `$MIDNIGHT_PP`, else
//! `/home/charl/Moriarty/repos/_build/params`, exactly as the crate's test
//! provider `TestParams` does; a missing file for the circuit's `k` is the
//! outcome `params-unavailable`, never a silent skip.
//! `--keygen` (the preimage argument is optional and unused): `Zkir::k`
//! (`optimal_k`), then `Zkir::keygen_vk` and `Zkir::keygen`, both of which
//! synthesise the circuit with an unknown witness (`setup_vk` panics on a
//! synthesis error, so a circuit that cannot be keyed reports `panic`);
//! reports `keygen_vk_ms`, `keygen_ms`, `pk_k` (the k recorded in the
//! proving key) and `vk_match` (the two verifier keys serialise equal).
//! `--prove`: `Zkir::keygen`, `IrSource::preprocess` (for the classification
//! of preimage errors), `Zkir::prove` with a fixed ChaCha20 seed, then
//! `VerifierKey::verify` against the crate's embedded `PARAMS_VERIFIER` and
//! the public inputs `prove` returned; reports `proof_bytes`, `prove_ms`,
//! `verify_ms`; outcome `verify-failure` when the verifier rejects.
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs::File;
use std::future::Future;
use std::io::BufReader;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;
use std::pin::pin;
use std::task::{Context as TaskContext, Poll, Waker};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use midnight_proofs::circuit::Value;
use midnight_proofs::dev::MockProver;
use midnight_zk_stdlib::MidnightCircuit;
use midnight_zkir::ir_types::IrValue;
use midnight_zkir::{Identifier, IrSource, Preprocessed};
use num_bigint::BigUint;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use tracing::Subscriber;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::{Context, SubscriberExt};
use transient_crypto::curve::Fr;
use transient_crypto::proofs::{KeyLocation, PARAMS_VERIFIER, ParamsProver, ParamsProverProvider, ProofPreimage, Zkir};

const MISALIGNMENT: &str = "Misalignment between `prepare` and `synthesize`";
/// The Synthesis message `Value::error_if_known_and` produces, which is what
/// `mem_insert` / `pi_push` stash in the relation on a mismatch.
const ERROR_IF_KNOWN: &str = "error_if_known_and";
const DEFAULT_PARAMS_DIR: &str = "/home/charl/Moriarty/repos/_build/params";
/// The seed the crate's integration tests hand to `prove` (tests/common/mod.rs).
const PROVE_SEED: [u8; 32] = [42; 32];

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
    #[serde(skip_serializing_if = "Option::is_none")]
    keygen_vk_ms: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keygen_ms: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prove_ms: Option<u128>,
    /// Length of the serialised proof.
    #[serde(skip_serializing_if = "Option::is_none")]
    proof_bytes: Option<usize>,
    /// The k recorded in the proving key produced by `keygen` (should equal `k`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pk_k: Option<u32>,
    /// `--keygen` only: the verifier key of `keygen_vk` equals the one of `keygen`.
    #[serde(skip_serializing_if = "Option::is_none")]
    vk_match: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n_pis: Option<usize>,
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

/// Drives a future to completion on the current thread. The crate's async
/// `Zkir` methods never suspend (the parameter provider below is synchronous),
/// so a no-op waker suffices and no runtime dependency is needed.
fn block_on<F: Future>(fut: F) -> F::Output {
    let waker = Waker::noop();
    let mut cx = TaskContext::from_waker(waker);
    let mut fut = pin!(fut);
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
        std::thread::yield_now();
    }
}

/// The provider the crate's integration tests use (`TestParams` in
/// tests/common/mod.rs): `DIR/bls_midnight_2p{k}` read with `ParamsProver::read`.
struct FileParams(String);

impl FileParams {
    fn file(&self, k: u8) -> String {
        format!("{}/bls_midnight_2p{k}", self.0)
    }
}

impl ParamsProverProvider for FileParams {
    async fn get_params(&self, k: u8) -> std::io::Result<ParamsProver> {
        ParamsProver::read(BufReader::new(File::open(self.file(k))?))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Mock,
    ModelOnly,
    Keygen,
    Prove,
}

struct Args {
    program: String,
    preimage: Option<String>,
    inject: Option<String>,
    pis: Option<String>,
    binding_input: Option<String>,
    instance: Option<String>,
    mode: Mode,
    params_dir: String,
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut pos = Vec::new();
    let mut inject = None;
    let mut pis = None;
    let mut binding_input = None;
    let mut instance = None;
    let mut mode = Mode::Mock;
    let mut params_dir = None;
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
            "--model-only" => mode = Mode::ModelOnly,
            "--keygen" => mode = Mode::Keygen,
            "--prove" => mode = Mode::Prove,
            "--params" => {
                i += 1;
                params_dir = argv.get(i).cloned();
            }
            other => pos.push(other.to_string()),
        }
        i += 1;
    }
    let usage = || {
        eprintln!(
            "usage: zkir-circuit-oracle PROGRAM.zkir PREIMAGE.json [--inject INJECT.json] [--pis PIS.json] [--binding-input DEC] [--instance PIS.json] [--model-only]\n       zkir-circuit-oracle PROGRAM.zkir [PREIMAGE.json] --keygen [--params DIR]\n       zkir-circuit-oracle PROGRAM.zkir PREIMAGE.json --prove [--params DIR]"
        );
        std::process::exit(2);
    };
    let program = if pos.is_empty() { usage() } else { pos.remove(0) };
    let preimage = if pos.is_empty() { None } else { Some(pos.remove(0)) };
    if !pos.is_empty() || (preimage.is_none() && mode != Mode::Keygen) {
        usage();
    }
    let params_dir = params_dir
        .or_else(|| std::env::var("MIDNIGHT_PP").ok())
        .unwrap_or_else(|| DEFAULT_PARAMS_DIR.to_string());
    Args { program, preimage, inject, pis, binding_input, instance, mode, params_dir }
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

/// `--keygen` and `--prove`: unknown-witness key generation with the file
/// parameter provider, then (for `--prove`) a real proof and its verification.
fn run_provability(args: &Args, ir: &IrSource, preimage: Option<&ProofPreimage>, errors: &Arc<Mutex<Vec<String>>>, out: &mut Output) {
    let provider = FileParams(args.params_dir.clone());
    let t = Instant::now();
    let k = ir.k();
    out.optimal_k_ms = Some(t.elapsed().as_millis());
    out.k = Some(k as u32);

    let file = provider.file(k);
    if !Path::new(&file).is_file() {
        out.outcome = "params-unavailable";
        out.message = format!("no KZG parameter file {file} for k={k}");
        return;
    }
    out.params_file = Some(file);

    if args.mode == Mode::Keygen {
        let t = Instant::now();
        let vk_only = match block_on(ir.keygen_vk(&provider)) {
            Ok(vk) => vk,
            Err(e) => {
                out.keygen_vk_ms = Some(t.elapsed().as_millis());
                out.outcome = "synthesis-error";
                out.message = format!("keygen_vk: {e:#}");
                return;
            }
        };
        out.keygen_vk_ms = Some(t.elapsed().as_millis());
        let t = Instant::now();
        let (pk, vk) = match block_on(ir.keygen(&provider)) {
            Ok(kp) => kp,
            Err(e) => {
                out.keygen_ms = Some(t.elapsed().as_millis());
                out.outcome = "synthesis-error";
                out.message = format!("keygen: {e:#}");
                return;
            }
        };
        out.keygen_ms = Some(t.elapsed().as_millis());
        out.pk_k = pk.init().ok().map(|p| p.k() as u32);
        out.vk_match = Some(vk_only == vk);
        out.outcome = "accepted";
        out.message = format!("keygen_vk and keygen ok at k={k}");
        return;
    }

    // --prove
    let preimage = preimage.expect("--prove requires a preimage");
    let t = Instant::now();
    let (pk, vk) = match block_on(ir.keygen(&provider)) {
        Ok(kp) => kp,
        Err(e) => {
            out.keygen_ms = Some(t.elapsed().as_millis());
            out.outcome = "synthesis-error";
            out.message = format!("keygen: {e:#}");
            return;
        }
    };
    out.keygen_ms = Some(t.elapsed().as_millis());
    out.pk_k = pk.init().ok().map(|p| p.k() as u32);

    // `prove` runs preprocess itself; running it first classifies preimage
    // errors the same way the MockProver mode does.
    let t = Instant::now();
    match ir.preprocess(preimage) {
        Ok(pre) => {
            out.preprocess_ms = Some(t.elapsed().as_millis());
            out.n_pis = Some(pre.pis.len());
        }
        Err(e) => {
            out.preprocess_ms = Some(t.elapsed().as_millis());
            out.outcome = "preprocess-error";
            out.message = format!("{e:#}");
            return;
        }
    }

    let t = Instant::now();
    let (proof, pis, _pi_skips) = match block_on(ir.prove(ChaCha20Rng::from_seed(PROVE_SEED), &provider, pk, preimage)) {
        Ok(r) => r,
        Err(e) => {
            out.prove_ms = Some(t.elapsed().as_millis());
            let logged = errors.lock().unwrap().clone();
            let misaligned = logged.iter().any(|m| m.contains(MISALIGNMENT));
            out.outcome = if misaligned { "witness-consistency-error" } else { "synthesis-error" };
            out.message = format!("prove: {e:#}");
            if !logged.is_empty() {
                out.trace_errors = Some(logged);
            }
            return;
        }
    };
    out.prove_ms = Some(t.elapsed().as_millis());
    out.proof_bytes = Some(proof.0.len());

    let t = Instant::now();
    match vk.verify(&PARAMS_VERIFIER, &proof, pis.iter().copied()) {
        Ok(()) => {
            out.verify_ms = Some(t.elapsed().as_millis());
            out.outcome = "accepted";
            out.message = format!("proof verified against PARAMS_VERIFIER with {} public inputs", pis.len());
        }
        Err(e) => {
            out.verify_ms = Some(t.elapsed().as_millis());
            out.outcome = "verify-failure";
            out.message = format!("verify: {e:#}");
        }
    }
    let logged = errors.lock().unwrap().clone();
    if !logged.is_empty() {
        out.trace_errors = Some(logged);
    }
}

fn run(args: &Args, ir: &IrSource, preimage: Option<&ProofPreimage>, errors: &Arc<Mutex<Vec<String>>>, out: &mut Output) {
    if matches!(args.mode, Mode::Keygen | Mode::Prove) {
        run_provability(args, ir, preimage, errors, out);
        return;
    }
    let preimage = preimage.expect("the MockProver modes require a preimage");
    if args.mode == Mode::ModelOnly {
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

    let n_pushes = pre.pis.len();
    if let Err(e) = inject(args, &mut pre, out) {
        out.outcome = "inject-error";
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
                out.outcome = "inject-error";
                out.message = format!("instance: {e:#}");
                return;
            }
        };
        instance = match v.iter().map(|s| fr_from_dec(s).map(|f| f.0)).collect::<anyhow::Result<Vec<_>>>() {
            Ok(i) => i,
            Err(e) => {
                out.outcome = "inject-error";
                out.message = format!("instance: {e:#}");
                return;
            }
        };
        out.injected.get_or_insert_with(Vec::new).push(format!("instance: {} -> {}", pre.pis.len(), instance.len()));
    }
    let instance_len = instance.len();
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
            // Both witnesses of a mem_insert / pi_push mismatch are required:
            // the error the relation stashed comes from `error_if_known_and`
            // (its Synthesis message is that name) and the ERROR event carries
            // the crate's misalignment text. Anything else is a synthesis error.
            let stashed = inner.as_deref().map_or(false, |i| i.contains(ERROR_IF_KNOWN));
            let event = logged.iter().any(|m| m.contains(MISALIGNMENT));
            out.outcome = if stashed && event { "witness-consistency-error" } else { "synthesis-error" };
            out.message = match inner {
                Some(i) => format!("{e:?}; relation error: {i}"),
                None => format!("{e:?}"),
            };
            if stashed != event {
                out.message = format!("{} (misalignment evidence incomplete: stashed error {stashed}, event {event})", out.message);
            }
            if !logged.is_empty() {
                out.trace_errors = Some(logged);
            }
            return;
        }
    };
    out.mockprover_run_ms = Some(t.elapsed().as_millis());
    out.rows = prover.advice().first().map(|c| c.len());
    out.usable_rows = Some(prover.usable_rows().end);

    if instance_len != n_pushes {
        out.outcome = "instance-length-mismatch";
        out.message = format!(
            "instance column has {instance_len} cell(s) but the circuit pushes {n_pushes} public input(s); a longer column leaves the surplus cells unconstrained, a shorter one fails on the first missing cell"
        );
        let logged = errors.lock().unwrap().clone();
        if !logged.is_empty() {
            out.trace_errors = Some(logged);
        }
        return;
    }

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
    let preimage = match &args.preimage {
        Some(path) => match load_preimage(path) {
            Ok(p) => Some(p),
            Err(e) => {
                eprintln!("preimage error: {e:#}");
                std::process::exit(2);
            }
        },
        None => None,
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

    // `out` lives outside the unwinding closure, so the fields filled before a
    // panic (k, preprocess_ms, optimal_k_ms, ...) survive into the panic line.
    let mut out = Output::default();
    let res = catch_unwind(AssertUnwindSafe(|| run(&args, &ir, preimage.as_ref(), &errors, &mut out)));
    if res.is_err() {
        out.outcome = "panic";
        out.message = panic_msg.lock().unwrap().clone().unwrap_or_else(|| "panic".into());
        let logged = errors.lock().unwrap().clone();
        if !logged.is_empty() {
            out.trace_errors = Some(logged);
        }
    }
    out.elapsed_ms = start.elapsed().as_millis();
    out.peak_rss_kb = peak_rss_kb();
    println!("{}", serde_json::to_string(&out).expect("serialize output"));
    if out.outcome == "inject-error" {
        std::process::exit(2);
    }
}
