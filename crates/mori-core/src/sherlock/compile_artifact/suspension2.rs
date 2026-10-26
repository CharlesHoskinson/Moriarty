//! `sherlock-helper-microstep/2` suspension routes (design note §3.5, §4, §5).
//!
//! Pure route (`moriarty-sherlock-draft/2 --entry`): the ordinary owner engine
//! (profile /2) runs the program; at an admitted await row `BAwaitEnter`
//! publishes Waiting, and the checkpoint is the complete `HostSnapshot`
//! (`C6RAWHS1` archive) inside a /2 envelope that also carries the image
//! header lanes (`RESUME_EPOCH`, `LAST_OBS_DIGEST`, `WAIT_SITE`), the Await
//! capture section and the await-table digest. Resume applies R0-R5 in order,
//! evaluates the straight-line guard and either applies the atomic
//! `BResumeAccept` relation (then continues the run) or returns `not_ready`
//! with the input bytes unchanged.
//!
//! Financial route (`moriarty/2 --action`): see [`super::suspension2::fin`].
//!
//! Host digests (stated difference from the note, which specifies the
//! circuit's `transientHash`): `CK` is SHA-256 over the domain tag
//! `SHK_CK2`, profile id 3, the carrier version and the whole envelope body;
//! `OD` is SHA-256 over `SHK_OBS2`, `CK` and the fixed 21-lane observation
//! vector. The in-circuit Field digests remain later work with the /2
//! circuits. Authentication is none and consumption untracked, as Core/2.
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use super::CompileFailure;
use super::parse_draft_config;
use crate::sherlock::Type;
use crate::sherlock::helper_machine::c6_raw_snapshot_archive::{
    RawSnapshotReadBudgets, RawSnapshotWriteBudgets, decode_raw_host_snapshot,
    encode_raw_host_snapshot,
};
use crate::sherlock::helper_machine::complete_observation::{HostSnapshot, capture};
use crate::sherlock::helper_machine::shared::owner_host::host_owner_engine_profile2;
use crate::sherlock::helper_machine::shared::owner_resume::resume_profile2_checkpoint;
use crate::sherlock::helper_machine::shared::owner_suspend::{
    self as suspend, AwaitEnter, AwaitTableC, GuardResult, ScalarC,
};
use crate::sherlock::helper_machine::shared::{
    Admitted, ControlC, ExhaustReason, FinanceInput, MachineImage, OutcomeC, OwnerEngine,
    PROFILE_ID_MICROSTEP_2, PROFILE_MICROSTEP_2, PreImage, StepResult,
};

#[path = "suspension2_fin.rs"]
pub mod fin;

pub const SOURCE_PROFILE_DRAFT_2: &str = crate::sherlock_draft::DRAFT_LANGUAGE_2;
pub const SOURCE_PROFILE_FIN_2: &str = crate::stage::LANGUAGE_2;
pub const OBSERVATION_FORMAT: &str = "sherlock-observation/2";
pub const QUALIFICATION: &str = "local-reference-suspension";
pub const MAX_OBSERVATION_BYTES: usize = crate::stage::MAX_EVENT_BYTES;
/// The /2 checkpoint carrier (envelope) version.
pub const CARRIER_VERSION: u16 = 2;
const MAGIC: [u8; 8] = *b"SHK2CKP1";
const DOM_CK2: &[u8] = b"SHK_CK2";
const DOM_OBS2: &[u8] = b"SHK_OBS2";
pub(crate) const ROUTE_PURE: u8 = 0;
pub(crate) const ROUTE_FIN: u8 = 1;
const MAX_CHECKPOINT_BYTES: usize = 64 * 1_048_576;

/// One invocation's outcome: the stdout JSON, its exit code, and the bytes a
/// `--checkpoint-out` file receives (none for a refusal).
pub struct Report {
    pub json: Value,
    pub exit: u8,
    pub checkpoint: Option<Vec<u8>>,
}

pub(crate) fn labelled(mut value: Value) -> Value {
    let o = value.as_object_mut().expect("reports are objects");
    o.insert("profile".into(), json!(PROFILE_MICROSTEP_2));
    o.insert("qualification".into(), json!(QUALIFICATION));
    o.insert("authentication".into(), json!("none"));
    o.insert("consumption".into(), json!("untracked"));
    value
}

pub(crate) fn report(value: Value, exit: u8, checkpoint: Option<Vec<u8>>) -> Report {
    Report {
        json: labelled(value),
        exit,
        checkpoint,
    }
}

pub(crate) fn refused(stage: &str, error: impl Into<String>) -> Report {
    report(
        json!({"status": "refused", "stage": stage, "error": error.into()}),
        4,
        None,
    )
}

pub(crate) fn failure(f: CompileFailure) -> Report {
    Report {
        json: serde_json::to_value(&f).unwrap_or(Value::Null),
        exit: 1,
        checkpoint: None,
    }
}

// ---------------------------------------------------------------- envelope

/// Decoded /2 envelope.
pub(crate) struct Envelope {
    pub route: u8,
    pub body: Vec<u8>,
    pub ck: [u8; 32],
}

fn ck_of(route: u8, body: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(DOM_CK2);
    h.update([PROFILE_ID_MICROSTEP_2]);
    h.update(CARRIER_VERSION.to_le_bytes());
    h.update(MAGIC);
    h.update([route]);
    h.update((body.len() as u64).to_le_bytes());
    h.update(body);
    h.finalize().into()
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub(crate) fn seal(route: u8, body: Vec<u8>) -> (Vec<u8>, [u8; 32]) {
    let ck = ck_of(route, &body);
    let mut out = Vec::with_capacity(body.len() + 8 + 1 + 8 + 32);
    out.extend_from_slice(&MAGIC);
    out.push(route);
    out.extend_from_slice(&(body.len() as u64).to_le_bytes());
    out.extend_from_slice(&body);
    out.extend_from_slice(&ck);
    (out, ck)
}

/// R0 carrier check: magic, length, and the trailing `CK` over the body. A
/// flipped byte anywhere fails here, before any field is trusted.
pub(crate) fn open(bytes: &[u8]) -> Result<Envelope, String> {
    if bytes.len() > MAX_CHECKPOINT_BYTES {
        return Err("checkpoint exceeds the carrier size bound".into());
    }
    if bytes.len() < 8 + 1 + 8 + 32 || bytes[..8] != MAGIC {
        return Err("not a sherlock-helper-microstep/2 checkpoint carrier".into());
    }
    let route = bytes[8];
    let len = u64::from_le_bytes(bytes[9..17].try_into().unwrap());
    let len = usize::try_from(len).map_err(|_| "carrier length overflow")?;
    if bytes.len() != 17 + len + 32 {
        return Err("checkpoint carrier length mismatch".into());
    }
    let body = bytes[17..17 + len].to_vec();
    let ck: [u8; 32] = bytes[17 + len..].try_into().unwrap();
    if ck_of(route, &body) != ck {
        return Err("checkpoint digest mismatch (corrupted or foreign checkpoint)".into());
    }
    Ok(Envelope { route, body, ck })
}

/// Little-endian field reader over an envelope body.
pub(crate) struct Rd<'a> {
    b: &'a [u8],
    at: usize,
}
impl<'a> Rd<'a> {
    pub(crate) fn new(b: &'a [u8]) -> Self {
        Self { b, at: 0 }
    }
    pub(crate) fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        let end = self.at.checked_add(n).ok_or("overflow")?;
        let s = self
            .b
            .get(self.at..end)
            .ok_or("truncated checkpoint body")?;
        self.at = end;
        Ok(s)
    }
    pub(crate) fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    pub(crate) fn u16(&mut self) -> Result<u16, String> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }
    pub(crate) fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    pub(crate) fn h32(&mut self) -> Result<[u8; 32], String> {
        Ok(self.take(32)?.try_into().unwrap())
    }
    pub(crate) fn rest(&mut self) -> &'a [u8] {
        let s = &self.b[self.at..];
        self.at = self.b.len();
        s
    }
}

pub(crate) fn put_scalar(out: &mut Vec<u8>, v: &ScalarC) {
    match v {
        ScalarC::UInt(n) => {
            out.push(0);
            out.extend_from_slice(&n.to_le_bytes());
        }
        ScalarC::Bool(b) => {
            out.push(1);
            out.extend_from_slice(&u128::from(*b).to_le_bytes());
        }
    }
}

pub(crate) fn get_scalar(r: &mut Rd<'_>) -> Result<ScalarC, String> {
    let tag = r.u8()?;
    let n = u128::from_le_bytes(r.take(16)?.try_into().unwrap());
    match (tag, n) {
        (0, n) => Ok(ScalarC::UInt(n)),
        (1, 0) => Ok(ScalarC::Bool(false)),
        (1, 1) => Ok(ScalarC::Bool(true)),
        _ => Err("bad scalar lane".into()),
    }
}

/// The /2 image header lanes and Await capture section (§3.4).
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub(crate) struct Header {
    pub resume_epoch: u64,
    pub last_obs_digest: [u8; 32],
    pub wait_site: u16,
    pub capture: Vec<ScalarC>,
}

impl Header {
    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.resume_epoch.to_le_bytes());
        out.extend_from_slice(&self.last_obs_digest);
        out.extend_from_slice(&self.wait_site.to_le_bytes());
        out.push(self.capture.len() as u8);
        for v in &self.capture {
            put_scalar(out, v);
        }
    }
    pub(crate) fn get(r: &mut Rd<'_>) -> Result<Self, String> {
        let resume_epoch = r.u64()?;
        let last_obs_digest = r.h32()?;
        let wait_site = r.u16()?;
        let n = r.u8()? as usize;
        if n > crate::sherlock_draft::MAX_AWAIT_OPERANDS.max(crate::stage::MAX_EVENT_BINDERS) {
            return Err("capture section exceeds its bound".into());
        }
        let capture = (0..n).map(|_| get_scalar(r)).collect::<Result<_, _>>()?;
        Ok(Self {
            resume_epoch,
            last_obs_digest,
            wait_site,
            capture,
        })
    }
}

// ------------------------------------------------------------- observation

/// `OD(obs)` over the fixed 21-lane vector `[DOM, CK, site, label_id, n, v0..v15]`.
pub(crate) fn observation_digest(ck: &[u8; 32], site: u16, values: &[ScalarC]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(DOM_OBS2);
    h.update(ck);
    h.update(site.to_le_bytes());
    // label_id: the site's admitted label index, which is its site number.
    h.update(site.to_le_bytes());
    h.update([values.len() as u8]);
    for i in 0..16 {
        let lane: u128 = match values.get(i) {
            Some(ScalarC::UInt(n)) => *n,
            Some(ScalarC::Bool(b)) => u128::from(*b),
            None => 0,
        };
        h.update(lane.to_le_bytes());
    }
    h.finalize().into()
}

fn canonical_u128(text: &str) -> Option<u128> {
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if text.len() > 1 && text.starts_with('0') {
        return None;
    }
    text.parse().ok()
}

/// R3-R5 over the observation text against the waiting site.
pub(crate) fn admit_observation(
    text: &str,
    ck: &[u8; 32],
    site: u16,
    label: &str,
    layout: &[(String, Type)],
) -> Result<Vec<ScalarC>, Report> {
    let r3 = |m: &str| refused("R3", m);
    if text.len() > MAX_OBSERVATION_BYTES {
        return Err(r3("observation exceeds 64 KiB"));
    }
    let obs: Value = crate::json::parse(text).map_err(|_| r3("observation is not JSON"))?;
    let o = obs
        .as_object()
        .ok_or_else(|| r3("observation is not an object"))?;
    if o.get("format").and_then(Value::as_str) != Some(OBSERVATION_FORMAT) {
        return Err(r3("observation format is not sherlock-observation/2"));
    }
    if o.get("checkpoint").and_then(Value::as_str) != Some(hex(ck).as_str()) {
        return Err(r3("observation is bound to another checkpoint"));
    }
    if o.get("site").and_then(Value::as_u64) != Some(u64::from(site)) {
        return Err(refused(
            "R4",
            "observation site differs from the waiting site",
        ));
    }
    if o.get("label").and_then(Value::as_str) != Some(label) {
        return Err(refused(
            "R4",
            "observation label differs from the waiting site's label",
        ));
    }
    let r5 = |m: &str| refused("R5", m);
    let values = o
        .get("values")
        .and_then(Value::as_array)
        .ok_or_else(|| r5("observation values are not a list"))?;
    if values.len() != layout.len() {
        return Err(r5("observation binder count differs from the site layout"));
    }
    let mut out = Vec::with_capacity(values.len());
    for (v, (name, ty)) in values.iter().zip(layout) {
        if v.get("name").and_then(Value::as_str) != Some(name.as_str()) {
            return Err(r5("observation binder name differs from the site layout"));
        }
        let typed = v
            .get("value")
            .ok_or_else(|| r5("observation binder has no value"))?;
        let sort = typed.get("type").and_then(Value::as_str);
        let raw = typed.get("value");
        out.push(match (ty, sort, raw) {
            (Type::UInt, Some("UInt"), Some(Value::String(s))) => ScalarC::UInt(
                canonical_u128(s)
                    .ok_or_else(|| r5("UInt binder is not a canonical decimal u128"))?,
            ),
            (Type::Bool, Some("Bool"), Some(Value::Bool(b))) => ScalarC::Bool(*b),
            _ => return Err(r5("observation binder sort differs from the site layout")),
        });
    }
    Ok(out)
}

pub(crate) fn sort_name(t: &Type) -> &'static str {
    match t {
        Type::UInt => "UInt",
        Type::Bool => "Bool",
        Type::List => "List",
        Type::Tree => "Tree",
    }
}

pub(crate) fn exhaust_name(r: ExhaustReason) -> String {
    format!("{r:?}")
}

// --------------------------------------------------------------- pure route

/// The /2 config is the draft host config with `profile` set to
/// `sherlock-helper-microstep/2`; every other field keeps its /1 meaning.
pub(crate) fn parse_draft2_config(
    source: &str,
) -> Result<super::draft_contract::DraftExecutionConfig, CompileFailure> {
    let mut value: Value =
        crate::json::parse(source).map_err(|e| CompileFailure::core("config", e))?;
    let o = value
        .as_object_mut()
        .ok_or_else(|| CompileFailure::input("config", "config is not an object"))?;
    if o.get("profile").and_then(Value::as_str) != Some(PROFILE_MICROSTEP_2) {
        return Err(CompileFailure::input(
            "config",
            "the /2 route requires config profile sherlock-helper-microstep/2",
        ));
    }
    o.insert("profile".into(), json!("sherlock-helper-microstep/1"));
    parse_draft_config(&value.to_string())
}

struct Pure {
    admitted: Admitted,
    table: AwaitTableC,
    serialization_bytes: usize,
}

fn admit_pure(source: &str, config_source: &str) -> Result<(Pure, MachineImage), CompileFailure> {
    use super::recursive_source::DraftSourceFailure as F;
    let config = parse_draft2_config(config_source)?;
    let bounds = config.bounds.owner();
    let layout = config.layout.owner(bounds);
    let (draft, awaits) = crate::sherlock_draft::parse_draft2_source_bounded(
        source,
        config.preprocessing.parser_budget(),
    )
    .map_err(|e| CompileFailure::draft_source(F::Source(e)))?;
    crate::sherlock_draft_types::check_program_types_bounded(
        draft.program.as_ref(),
        config.preprocessing.max_type_depth.0,
    )
    .map_err(|e| CompileFailure::draft_source(F::Types(e)))?;
    let owner_source = crate::sherlock::helper_machine::source_input_bridge::from_program_bounded(
        draft.program.as_ref(),
        &config.preprocessing.copy_budget(),
    )
    .map_err(|e| CompileFailure::draft_source(F::Copy(e)))?;
    let engine = host_owner_engine_profile2();
    let admitted = engine
        .admit(&owner_source, &bounds, &layout)
        .map_err(|e| CompileFailure::draft_source(F::Admit(e)))?;
    let genesis = engine
        .admit_run(
            &admitted,
            &config.fuel.owner(),
            &config.stack.owner(),
            &FinanceInput::Absent,
        )
        .map_err(|e| CompileFailure::draft_source(F::AdmitRun(e)))?;
    let table = suspend::bind_await_table(&admitted, &awaits)
        .map_err(|e| CompileFailure::input("admit", e))?;
    Ok((
        Pure {
            admitted,
            table,
            serialization_bytes: bounds.serialization_bytes.max(1),
        },
        genesis,
    ))
}

fn run_failure(stage: &'static str, message: String) -> Report {
    failure(CompileFailure {
        stage,
        actual_error: json!(message),
        evaluation_performed: true,
        events_forwarded: "0".into(),
        completed: false,
    })
}

/// Drive the /2 owner from `img` until it leaves Running.
fn drive(p: &Pure, img: &mut MachineImage) -> Result<Vec<ScalarC>, String> {
    let engine = host_owner_engine_profile2();
    loop {
        if suspend::outcome(img) != OutcomeC::Running {
            return Ok(Vec::new());
        }
        match suspend::await_enter(&p.admitted, img, &p.table)? {
            Some(AwaitEnter::Waiting { operands, .. }) => {
                let pre = PreImage::bind(&p.admitted, img).map_err(|v| format!("{v:?}"))?;
                engine.check_step_inv(&pre).map_err(|v| {
                    let clauses: Vec<String> = v
                        .iter()
                        .map(|x| format!("{:?} {}", x.clause, x.detail))
                        .collect();
                    format!("Waiting image violates step invariants: {clauses:?}")
                })?;
                return Ok(operands);
            }
            Some(AwaitEnter::Exhausted { .. }) => return Ok(Vec::new()),
            None => {}
        }
        match engine
            .step(&p.admitted, img)
            .map_err(|v| format!("{v:?}"))?
        {
            StepResult::Terminal => return Ok(Vec::new()),
            StepResult::Event { .. } => {}
        }
    }
}

fn archive(p: &Pure, img: &MachineImage) -> Result<Vec<u8>, String> {
    let snapshot = capture(&p.admitted, img).map_err(|v| format!("capture: {v:?}"))?;
    let limit = p.serialization_bytes;
    let mut out = Vec::new();
    encode_raw_host_snapshot(
        &snapshot,
        RawSnapshotWriteBudgets {
            max_output_bytes: limit,
            max_type_nodes: limit,
            max_type_depth: 64,
            max_walk_bytes: limit,
        },
        &mut out,
    )
    .map_err(|e| format!("archive refused: {e:?}"))?;
    Ok(out)
}

fn unarchive(p: &Pure, bytes: &[u8]) -> Result<HostSnapshot, String> {
    let limit = p.serialization_bytes;
    decode_raw_host_snapshot(
        bytes,
        RawSnapshotReadBudgets {
            max_input_bytes: limit,
            max_elements: limit,
            max_alloc_bytes: limit.saturating_mul(64),
            max_type_nodes: limit,
            max_type_depth: 64,
        },
    )
    .map(|d| d.into_unattested_snapshot())
    .map_err(|e| format!("archive decode refused: {e:?}"))
}

fn pure_body(p: &Pure, header: &Header, img: &MachineImage) -> Result<Vec<u8>, String> {
    let mut body = Vec::new();
    body.extend_from_slice(&p.table.digest());
    header.put(&mut body);
    body.extend_from_slice(&archive(p, img)?);
    Ok(body)
}

/// Report the image `drive` stopped at, with its sealed checkpoint.
fn publish(p: &Pure, img: &MachineImage, mut header: Header, operands: Vec<ScalarC>) -> Report {
    let outcome = suspend::outcome(img);
    header.wait_site = match outcome {
        OutcomeC::Waiting { site } => site,
        _ => 0,
    };
    header.capture = operands;
    let body = match pure_body(p, &header, img) {
        Ok(body) => body,
        Err(e) => return run_failure("checkpoint", e),
    };
    let (bytes, ck) = seal(ROUTE_PURE, body);
    let committed = suspend::consumed_work(img).to_string();
    let (value, exit) = match outcome {
        OutcomeC::Waiting { site } => {
            let s = p.table.site(site).expect("waiting site is admitted");
            (
                json!({
                    "status": "waiting",
                    "site": site,
                    "label": s.label,
                    "checkpoint": hex(&ck),
                    "binders": s.binders.iter().map(|(_, n, t)| json!({"name": n, "type": sort_name(t)})).collect::<Vec<_>>(),
                    "committed_work": committed,
                    "attempted_work": "0",
                    "resume_epoch": header.resume_epoch.to_string(),
                    "frames": suspend::frames_summary(&p.admitted, img),
                    "fin": null,
                }),
                0,
            )
        }
        OutcomeC::Halted { value } => (
            json!({
                "status": "halted",
                "value": suspend::value_json(&p.admitted, img, value),
                "committed_work": committed,
                "checkpoint": hex(&ck),
                "resume_epoch": header.resume_epoch.to_string(),
                "fin": null,
            }),
            0,
        ),
        OutcomeC::Rejected { code } => (
            json!({"status": "rejected", "code": code, "committed_work": committed,
                   "checkpoint": hex(&ck), "fin": null}),
            1,
        ),
        OutcomeC::Exhausted { reason } => (
            json!({"status": "exhausted", "reason": exhaust_name(reason), "committed_work": committed,
                   "checkpoint": hex(&ck), "fin": null}),
            1,
        ),
        OutcomeC::Running => return run_failure("run", "the run stopped while Running".into()),
    };
    report(value, exit, Some(bytes))
}

/// Pure start: `--entry` under profile /2.
pub fn start_pure(source: &str, config_source: &str, _target_source: &str) -> Report {
    let (p, mut img) = match admit_pure(source, config_source) {
        Ok(x) => x,
        Err(f) => return failure(f),
    };
    match drive(&p, &mut img) {
        Ok(operands) => publish(&p, &img, Header::default(), operands),
        Err(e) => run_failure("run", e),
    }
}

/// Pure resume: R0-R5, then the guard, then `BResumeAccept` or `not_ready`.
pub fn resume_pure(
    source: &str,
    config_source: &str,
    _target_source: &str,
    checkpoint: &[u8],
    observation: &str,
) -> Report {
    // R0: carrier, digest, body layout.
    let env = match open(checkpoint) {
        Ok(env) => env,
        Err(e) => return refused("R0", e),
    };
    if env.route != ROUTE_PURE {
        return refused("R2", "checkpoint belongs to the financial route");
    }
    let mut r = Rd::new(&env.body);
    let parsed = (|| -> Result<([u8; 32], Header, &[u8]), String> {
        let table_digest = r.h32()?;
        let header = Header::get(&mut r)?;
        Ok((table_digest, header, r.rest()))
    })();
    let (table_digest, header, archived) = match parsed {
        Ok(x) => x,
        Err(e) => return refused("R0", e),
    };
    // Admission of the supplied source (needed to decode within its bounds).
    let (p, genesis_img) = match admit_pure(source, config_source) {
        Ok(x) => x,
        Err(f) => {
            return refused(
                "R2",
                format!("supplied source is not admitted: {}", f.actual_error),
            );
        }
    };
    let snapshot = match unarchive(&p, archived) {
        Ok(s) => s,
        Err(e) => return refused("R0", e),
    };
    // R1: Waiting{s} with Await{s, ..}.
    let site = match (snapshot.image.outcome, snapshot.image.control) {
        (OutcomeC::Waiting { site }, ControlC::Await { site: c, .. }) if site == c => site,
        _ => return refused("R1", "checkpoint is not Waiting at an await site"),
    };
    if header.wait_site != site {
        return refused("R0", "WAIT_SITE disagrees with the Waiting outcome");
    }
    // R2: admission identity (program view, bounds, layout, identity) and table.
    let genesis = match capture(&p.admitted, &genesis_img) {
        Ok(s) => s,
        Err(e) => return run_failure("admit", format!("{e:?}")),
    };
    if genesis.admission != snapshot.admission || table_digest != p.table.digest() {
        return refused("R2", "checkpoint does not belong to the supplied program");
    }
    let Some(s) = p.table.site(site) else {
        return refused(
            "R2",
            "waiting site is not an await site of the supplied program",
        );
    };
    // R0 (continued): full host invariants and SI-31 on the rebuilt image.
    let mut img = match resume_profile2_checkpoint(&p.admitted, &snapshot) {
        Ok(img) => img,
        Err(e) => return refused("R0", e.to_string()),
    };
    match suspend::guard_operands(&p.admitted, &img, s) {
        Ok(ops) if ops == header.capture => {}
        _ => {
            return refused(
                "R0",
                "Await capture section disagrees with the image (SI-31)",
            );
        }
    }
    // R0 (continued): the epoch has a successor and SI-29 holds, both checked
    // before the guard so an unresumable import is refused, never not_ready.
    if header.resume_epoch == u64::MAX {
        return refused("R0", "resume_epoch is at its maximum");
    }
    if let Err(e) = suspend::check_resumable(&p.admitted, &img, s) {
        return refused("R0", e);
    }
    // R3-R5.
    let layout: Vec<(String, Type)> = s.binders.iter().map(|(_, n, t)| (n.clone(), *t)).collect();
    let values = match admit_observation(observation, &env.ck, site, &s.label, &layout) {
        Ok(v) => v,
        Err(refusal) => return refusal,
    };
    let attempted = s.accept_charge().to_string();
    match s.guard.run(&values, &header.capture) {
        GuardResult::True => {}
        other => {
            let mut v = json!({
                "status": "not_ready",
                "checkpoint": hex(&env.ck),
                "site": site,
                "attempted_work": attempted,
                "committed_work": suspend::consumed_work(&img).to_string(),
                "fin": null,
            });
            match other {
                GuardResult::Error(code) => {
                    v["reason"] = json!("guard_error");
                    v["code"] = json!(code);
                }
                _ => v["reason"] = json!("guard_false"),
            }
            return report(v, 3, Some(checkpoint.to_vec()));
        }
    }
    let Some(resume_epoch) = header.resume_epoch.checked_add(1) else {
        return refused("R0", "resume_epoch is at its maximum");
    };
    let operands = header.capture.clone();
    if let Err(e) = suspend::resume_accept(&p.admitted, &mut img, &p.table, &operands, &values) {
        return run_failure("resume", e);
    }
    let next = Header {
        resume_epoch,
        last_obs_digest: observation_digest(&env.ck, site, &values),
        wait_site: 0,
        capture: Vec::new(),
    };
    match drive(&p, &mut img) {
        Ok(operands) => publish(&p, &img, next, operands),
        Err(e) => run_failure("run", e),
    }
}
