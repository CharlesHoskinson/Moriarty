//! Sherlock A102 Literal native statement bridge. One complete immutable
//! ProgramBlock, complete pre/post StateBlocks and the wire envelope form one
//! statement; the real emit_literal_step fragment runs with a forced in-circuit
//! enable. Scope: exactly one charged Eval(Literal)->Return transition, not other
//! rules, continuity or program finish. Only a listed host subset of public-data
//! checks runs, off-chain, and no ledger operation VK enforces it. The post is a
//! Step-mode image, never a canonical-boundary image. projectA, StepInv, F4, G4
//! and literal uiValid remain open. Compile/keygen/prove/verify: UNOBSERVED.
use super::*;
use crate::parameters::OfflineParameters;
use crate::proving::{MAX_K, MAX_PROOF_BYTES, ProofOptions, hex};
use crate::{Error, LEDGER_REVISION, ZKIR_VERSION, authorization::decode_digest, error};
use midnight_transient_crypto::proofs::{
    KeyLocation, PARAMS_VERIFIER, Proof, ProofPreimage, VERIFIER_MAX_DEGREE, Zkir,
};
use midnight_zkir_v3::ir::TypedIdentifier;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const STATEMENT_FORMAT: &str = "moriarty-sherlock-a102-literal-statement/1";
pub const BINDING_VERSION: &str = "moriarty-sherlock-a102-literal-binding/1";
pub const PROFILE: &str = "sherlock-k-a/1";
pub const SCOPE: &str = "one-charged-transition:A102-Eval-Literal-to-Return";
pub const RULE_START_LINE: u128 = 102;
pub const RULE_TAG: u128 = 3;
pub const RULE_CHARGE: u128 = 1;
const STEP_SCOPE: &str = "sherlock.a102.step";
const KEY_LOCATION: &str = "moriarty-sherlock-a102-literal/1";
pub const OPEN_OBLIGATIONS: [&str; 8] = [
    "canonical ProgramBlock equals prepareA(source) (GAP-1 unimplemented); the F4 least-equal mechanism is unselected and applies only at canonical boundaries",
    "the post is a Step-mode image (CR0=Return holding the Literal's Constant-space ValueRef, all five arenas equal to pre), never a canonical-boundary StateBlock; checkpoint, chunk or external-claim use needs the separately reviewed, unimplemented projectA boundary operation",
    "full StepInv/O1-O4 bounds, acyclicity, suffix slices, variant padding and Constants typing/literal uiValid beyond the listed host subset, which no ledger operation VK enforces",
    "OBL.G4 canonical-byte projection of nonfuel carriers",
    "the other 38 rows of the 39-row public rule_line_tags dictionary; K's invalid-Literal sibling head, which has no wire tag and whose exclusion depends on unimplemented prepareA typing/uiValid (40 post-startup K heads per a supplied public QA fact; K bodies not read here); the exactly-one dispatch partition; post-to-pre continuity",
    "trace, chunk and profile binding across proofs and K source correspondence; the three source digests are caller-asserted and unverified",
    "SRS provenance, keygen correspondence and adversarial soundness review",
    "ledger-9 installation, transaction acceptance, call address/entry-point readback and stagenet settlement; the ledger statement opening is off-chain",
];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiteralSourceIdentity {
    /// Pinned K semantics in which ruleStartLine 102 resolves.
    pub k_source_sha256: String,
    /// Pinned reference.rs supplying rule_start_line and the A step.
    pub reference_source_sha256: String,
    /// Program source from which a reviewed prepareA built the ProgramBlock.
    pub program_source_sha256: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapacityRecord {
    pub program: Vec<u64>,
    pub state: Vec<u64>,
    pub stack_limit: String,
}
/// Concrete logical lanes in LiteralImage order; every value is public.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageValues {
    pub root: Vec<u128>,
    pub tables: Vec<Vec<Vec<u128>>>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiteralValues {
    pub program: ImageValues,
    pub pre: ImageValues,
    pub post: ImageValues,
    pub rule_tag: u128,
    pub charge: u128,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CircuitMode {
    /// PublicInput/Impact positional statement, communications disabled.
    LocalPublic,
    /// Statement lanes are call inputs under the communications commitment.
    LedgerCall,
}

struct Hasher(Sha256);
impl Hasher {
    fn new(domain: &str) -> Self {
        let mut h = Self(Sha256::new());
        h.bytes(domain.as_bytes());
        h
    }
    fn bytes(&mut self, b: &[u8]) {
        self.0.update((b.len() as u64).to_le_bytes());
        self.0.update(b);
    }
    fn num(&mut self, n: u128) {
        self.0.update(n.to_le_bytes());
    }
    fn finish(self) -> [u8; 32] {
        self.0.finalize().into()
    }
}
fn construction(e: LiteralConstructionError) -> Error {
    error("sherlock-construction", format!("{e:?}"))
}
fn wide(name: &str) -> LiteralLane {
    LiteralLane { name: name.into(), bits: 128, boolean: false, maximum: None }
}
fn image_lane_count(root: &[LiteralLane], tables: &[LiteralTable]) -> Option<usize> {
    tables.iter().try_fold(root.len(), |n, t| n.checked_add(t.capacity.checked_mul(t.lanes.len())?))
}
/// Checked statement length from the schema's table shapes alone: two context
/// limbs, ProgramBlock, pre and post StateBlocks, rule_tag and charge.
fn statement_lane_count(s: &LiteralSchema) -> Option<usize> {
    let program = image_lane_count(&s.program_header, &s.program_tables)?;
    let state = image_lane_count(&s.state_root, &s.state_tables)?;
    program.checked_add(state.checked_mul(2)?)?.checked_add(4)
}
/// Existing local verifier capacity: the binding plus the statement fit 2^VERIFIER_MAX_DEGREE.
pub fn local_max_statement_lanes() -> usize {
    (1usize << VERIFIER_MAX_DEGREE) - 1
}
fn image_lanes(out: &mut Vec<(String, LiteralLane)>, block: &str, root: &[LiteralLane], tables: &[LiteralTable]) {
    out.extend(root.iter().map(|l| (format!("{block}.root"), l.clone())));
    for t in tables {
        for r in 0..t.capacity {
            out.extend(t.lanes.iter().map(|l| (format!("{block}.{}.{r}", t.name), l.clone())));
        }
    }
}
fn shape(image: &ImageValues, root: &[LiteralLane], tables: &[LiteralTable], block: &str) -> Result<(), Error> {
    let ok = image.root.len() == root.len()
        && image.tables.len() == tables.len()
        && image.tables.iter().zip(tables).all(|(rows, t)| {
            rows.len() == t.capacity && rows.iter().all(|r| r.len() == t.lanes.len())
        });
    if ok { Ok(()) } else { Err(error("sherlock-shape", format!("{block} image differs from the L4 schema."))) }
}
fn take_image(it: &mut impl Iterator<Item = Operand>, root: &[LiteralLane], tables: &[LiteralTable]) -> Result<LiteralImage, Error> {
    let mut take = |n: usize| -> Result<Vec<Operand>, Error> {
        let row: Vec<Operand> = (&mut *it).take(n).collect();
        if row.len() == n { Ok(row) } else { Err(error("sherlock-shape", "Statement operands ended early.")) }
    };
    let root = take(root.len())?;
    let mut rows = Vec::with_capacity(tables.len());
    for table in tables {
        let mut table_rows = Vec::with_capacity(table.capacity);
        for _ in 0..table.capacity {
            table_rows.push(take(table.lanes.len())?);
        }
        rows.push(table_rows);
    }
    Ok(LiteralImage { root, tables: rows })
}
// Listed host subset of representation-domain checks. This is not StepInv: it
// checks no Constants typing/uiValid, acyclicity, suffix slices or variant padding.
// Prover and verifier run it off-chain only, and no ledger operation VK enforces
// it. A refusal is tooling, never an Outcome.
fn representation(s: &LiteralSchema, v: &LiteralValues) -> Result<(), Error> {
    let refuse = |what: &str| error("sherlock-representation", what.to_string());
    let padded = |rows: &Vec<Vec<u128>>, active: u128| {
        rows.iter().enumerate().all(|(r, row)| (r as u128) < active || row.iter().all(|c| *c == 0))
    };
    let p = &v.program;
    if !p.tables.iter().enumerate().all(|(i, rows)| padded(rows, p.root[i + 1])) {
        return Err(refuse("Program rows at or above an active length are nonzero."));
    }
    for image in [&v.pre, &v.post] {
        if !image.tables.iter().enumerate().all(|(i, rows)| padded(rows, image.root[i])) {
            return Err(refuse("State rows at or above an active length are nonzero."));
        }
    }
    if p.root[0] != 0 || p.root[5] == 0 {
        return Err(refuse("Header needs entryExprId=0 and activeExpressions>=1."));
    }
    let builtin: [(usize, Vec<Vec<u128>>); 3] = [
        (10, vec![vec![0, 0, 0], vec![1, 0, 0], vec![2, 0, 2], vec![3, 2, 2]]),
        (11, vec![vec![2, 0, 0, 0], vec![2, 1, 0, 2], vec![3, 0, 2, 1], vec![3, 1, 3, 2]]),
        (12, vec![vec![0], vec![2], vec![0], vec![3], vec![3]]),
    ];
    for (table, rows) in &builtin {
        if p.root[*table + 1] != rows.len() as u128 || p.tables[*table][..rows.len()] != rows[..] {
            return Err(refuse("Builtin Types/Constructors/ConstructorFieldTypes rows differ."));
        }
    }
    for (table, lane) in [(4usize, "occurrenceId"), (6, "sourceTableIndex")] {
        let column = s.program_tables[table].lanes.iter().position(|l| l.name == lane).expect("fixed L4 column");
        if p.tables[table].iter().enumerate()
            .any(|(i, row)| (i as u128) < p.root[table + 1] && row[column] != i as u128) {
            return Err(refuse("An active row index column differs from its row."));
        }
    }
    Ok(())
}

/// Value-independent relation identity: one circuit per capacities and identity.
#[derive(Clone, Debug)]
pub struct LiteralLayout {
    schema: LiteralSchema,
    capacities: LiteralCapacities,
    identity: LiteralSourceIdentity,
    lanes: Vec<(String, LiteralLane)>,
    layout_hash: [u8; 32],
    relation_hash: [u8; 32],
    context: [u8; 32],
}
impl LiteralLayout {
    /// `max_statement_lanes` is the caller's explicit bound. The checked lane count
    /// is admitted before any per-lane allocation. This is native statement
    /// admission, not F5 canonical-projection refusal.
    pub fn new(capacities: &LiteralCapacities, identity: &LiteralSourceIdentity, max_statement_lanes: usize) -> Result<Self, Error> {
        for d in [&identity.k_source_sha256, &identity.reference_source_sha256, &identity.program_source_sha256] {
            decode_digest(d)?;
        }
        let schema = literal_schema(capacities).map_err(construction)?;
        let count = statement_lane_count(&schema).ok_or_else(|| error("limit", "Statement lane count overflows usize."))?;
        if count > max_statement_lanes {
            return Err(error("limit", "Statement lane count exceeds the caller's explicit bound."));
        }
        let mut lanes = Vec::with_capacity(count);
        lanes.push(("context".to_string(), wide("limb.0")));
        lanes.push(("context".to_string(), wide("limb.1")));
        image_lanes(&mut lanes, "program", &schema.program_header, &schema.program_tables);
        image_lanes(&mut lanes, "pre", &schema.state_root, &schema.state_tables);
        image_lanes(&mut lanes, "post", &schema.state_root, &schema.state_tables);
        // Exactly the fragment's tag(rule_tag, 39) and unsigned(charge, 8) lanes.
        lanes.push(("envelope".into(), LiteralLane { name: "rule_tag".into(), bits: 6, boolean: false, maximum: Some(38) }));
        lanes.push(("envelope".into(), LiteralLane { name: "charge".into(), bits: 8, boolean: false, maximum: None }));
        if lanes.len() != count {
            return Err(error("sherlock-shape", "Statement layout differs from its admitted lane count."));
        }
        let mut h = Hasher::new(STATEMENT_FORMAT);
        for c in capacities.program.iter().chain(&capacities.state) {
            h.num(*c as u128);
        }
        h.num(capacities.stack_limit);
        for (path, lane) in &lanes {
            h.bytes(path.as_bytes());
            h.bytes(lane.name.as_bytes());
            h.num(lane.bits.into());
            h.num(lane.boolean.into());
            h.num(lane.maximum.is_some().into());
            h.num(lane.maximum.unwrap_or(0));
        }
        let layout_hash = h.finish();
        let mut h = Hasher::new("moriarty-sherlock-a102-relation/1");
        for source in [
            include_str!("mod.rs"),
            include_str!("fetch.rs"),
            include_str!("enabled_fetch.rs"),
            include_str!("literal_step.rs"),
            include_str!("literal_bridge.rs"),
            include_str!("../../../../models/sherlock/literal-step/MAPPING.md"),
        ] {
            h.bytes(source.as_bytes());
        }
        let relation_hash = h.finish();
        let mut h = Hasher::new("moriarty-sherlock-a102-context/1");
        for part in [
            STATEMENT_FORMAT, BINDING_VERSION, PROFILE, SCOPE, LEDGER_REVISION, ZKIR_VERSION,
            identity.k_source_sha256.as_str(), identity.reference_source_sha256.as_str(),
            identity.program_source_sha256.as_str(),
        ] {
            h.bytes(part.as_bytes());
        }
        for n in [RULE_START_LINE, RULE_TAG, RULE_CHARGE] {
            h.num(n);
        }
        h.bytes(&layout_hash);
        h.bytes(&relation_hash);
        let context = h.finish();
        Ok(Self { schema, capacities: capacities.clone(), identity: identity.clone(), lanes, layout_hash, relation_hash, context })
    }
    pub fn statement_len(&self) -> usize {
        self.lanes.len()
    }
    pub fn context_hex(&self) -> String {
        hex(&self.context)
    }
    fn limbs(&self) -> [u128; 2] {
        let (lo, hi) = self.context.split_at(16);
        [u128::from_le_bytes(lo.try_into().expect("16 bytes")), u128::from_le_bytes(hi.try_into().expect("16 bytes"))]
    }
    fn record(&self) -> CapacityRecord {
        CapacityRecord {
            program: self.capacities.program.iter().map(|c| *c as u64).collect(),
            state: self.capacities.state.iter().map(|c| *c as u64).collect(),
            stack_limit: self.capacities.stack_limit.to_string(),
        }
    }
    /// Ordered public statement after host admissibility of public data.
    pub fn statement(&self, v: &LiteralValues) -> Result<Vec<u128>, Error> {
        let s = &self.schema;
        shape(&v.program, &s.program_header, &s.program_tables, "program")?;
        shape(&v.pre, &s.state_root, &s.state_tables, "pre")?;
        shape(&v.post, &s.state_root, &s.state_tables, "post")?;
        let mut out = self.limbs().to_vec();
        for image in [&v.program, &v.pre, &v.post] {
            out.extend(image.root.iter().chain(image.tables.iter().flatten().flatten()).copied());
        }
        out.extend([v.rule_tag, v.charge]);
        if out.len() != self.lanes.len() {
            return Err(error("sherlock-shape", "Statement length differs from the layout."));
        }
        for (value, (path, lane)) in out.iter().zip(&self.lanes) {
            let fits = if lane.boolean { *value <= 1 } else { lane.bits >= 128 || *value >> lane.bits == 0 };
            if !fits || lane.maximum.is_some_and(|m| *value > m) {
                return Err(error("sherlock-range", format!("{path}.{} is outside its L4 lane.", lane.name)));
            }
        }
        if v.rule_tag != RULE_TAG || v.charge != RULE_CHARGE {
            return Err(error("sherlock-envelope", "Only wire tag 3 with charge 1 (line 102) is admitted."));
        }
        representation(s, v)?;
        Ok(out)
    }
    /// Emit the full relation: bound statement operands, forced A102 enable,
    /// the unmodified literal fragment and the mode's Output terminator.
    pub fn emit(&self, mode: CircuitMode) -> Result<(IrSource, String), Error> {
        let local = mode == CircuitMode::LocalPublic;
        let mut code = Vec::new();
        let mut inputs: Vec<TypedIdentifier> = Vec::new();
        let mut operands = Vec::with_capacity(self.lanes.len());
        for position in 0..self.lanes.len() {
            let id = Identifier(format!("%sherlock.a102.statement.{position}"));
            let operand = Operand::Variable(id.clone());
            if local {
                code.push(Instruction::PublicInput { guard: None, val_t: IrType::Native, output: id });
                code.push(Instruction::Impact { guard: immediate(1), inputs: vec![operand.clone()] });
            } else {
                inputs.push(serde_json::from_value(serde_json::json!({"name": id, "type": IrType::Native}))
                    .map_err(|e| error("zkir", e.to_string()))?);
            }
            operands.push(operand);
        }
        let [lo, hi] = self.limbs();
        code.push(Instruction::ConstrainEq { a: operands[0].clone(), b: immediate(lo) });
        code.push(Instruction::ConstrainEq { a: operands[1].clone(), b: immediate(hi) });
        let s = &self.schema;
        let mut rest = operands[2..].iter().cloned();
        let program = take_image(&mut rest, &s.program_header, &s.program_tables)?;
        let pre = take_image(&mut rest, &s.state_root, &s.state_tables)?;
        let post = take_image(&mut rest, &s.state_root, &s.state_tables)?;
        let (rule_tag, charge) = match (rest.next(), rest.next(), rest.next()) {
            (Some(t), Some(c), None) => (t, c),
            _ => return Err(error("sherlock-shape", "Envelope operands differ from the layout.")),
        };
        let enable_id = Identifier("%sherlock.a102.dispatch.enable".into());
        code.push(Instruction::TestEq { a: rule_tag.clone(), b: immediate(RULE_TAG), output: enable_id.clone() });
        let enable = Operand::Variable(enable_id);
        code.push(Instruction::ConstrainToBoolean { val: enable.clone() });
        code.push(Instruction::ConstrainEq { a: enable.clone(), b: immediate(1) });
        let block = emit_literal_step(s, &LiteralStepInputs {
            enable, program, pre, post: post.clone(),
            envelope: LiteralEnvelope { rule_tag: rule_tag.clone(), charge: charge.clone() },
        }, STEP_SCOPE).map_err(construction)?;
        code.extend(block.instructions);
        let vals: Vec<Operand> = if local {
            post.root.iter().chain(post.tables.iter().flatten().flatten()).cloned().chain([rule_tag, charge]).collect()
        } else {
            Vec::new()
        };
        let outputs = vec![IrType::Native; vals.len()];
        code.push(Instruction::Output { vals });
        let ir = IrSource {
            version: Default::default(),
            inputs,
            outputs,
            do_communications_commitment: !local,
            instructions: Arc::new(code),
        };
        let mut disk = serde_json::to_value(&ir).map_err(|e| error("serialization", e.to_string()))?;
        disk["version"] = serde_json::json!({"major": 3, "minor": 0});
        let zkir = serde_json::to_string(&disk).map_err(|e| error("serialization", e.to_string()))?;
        let parsed = IrSource::load(zkir.as_bytes()).map_err(|e| error("zkir", e.to_string()))?;
        if parsed != ir {
            return Err(error("zkir", "Pinned upstream round-trip changed the IR."));
        }
        Ok((ir, zkir))
    }
}

#[derive(Clone, Debug)]
pub struct LiteralTransitionRequest {
    pub capacities: LiteralCapacities,
    pub identity: LiteralSourceIdentity,
    pub values: LiteralValues,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiteralClaim {
    pub format: String,
    pub binding: String,
    pub profile: String,
    pub scope: String,
    pub ledger_revision: String,
    pub zkir_version: String,
    pub rule_start_line: u32,
    pub rule_tag: u32,
    pub charge: u32,
    pub capacities: CapacityRecord,
    pub identity: LiteralSourceIdentity,
    pub layout_hash: String,
    pub relation_hash: String,
    pub context_hash: String,
    pub zkir_hash: String,
    pub circuit_k: u8,
    /// Decimal: two context limbs, ProgramBlock, pre, post, rule_tag, charge.
    pub public_values: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiteralProofBundle {
    pub claim: LiteralClaim,
    pub proof_hex: String,
}
impl LiteralProofBundle {
    pub fn from_json(text: &str) -> Result<Self, Error> {
        let value = mori_core::json::parse(text).map_err(|e| error("json", e.to_string()))?;
        serde_json::from_value(value).map_err(|e| error("json", e.to_string()))
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct LiteralVerification {
    pub qualification: &'static str,
    pub context_hash: String,
    pub zkir_hash: String,
    pub circuit_k: u8,
    pub proof_bytes: u32,
    pub open_obligations: Vec<&'static str>,
}
struct Prepared {
    ir: IrSource,
    values: Vec<Fr>,
    binding: Fr,
    claim: LiteralClaim,
}
impl Prepared {
    fn statement(&self) -> Vec<Fr> {
        std::iter::once(self.binding).chain(self.values.iter().copied()).collect()
    }
}
fn prepare(request: &LiteralTransitionRequest, options: &ProofOptions) -> Result<Prepared, Error> {
    if options.max_k == 0 || options.max_k > MAX_K {
        return Err(error("limit", "Native k cap must be between 1 and 19."));
    }
    let layout = LiteralLayout::new(&request.capacities, &request.identity, local_max_statement_lanes())?;
    let statement = layout.statement(&request.values)?;
    if statement.len() + 1 > (1usize << VERIFIER_MAX_DEGREE) {
        return Err(error("limit", "Public statement exceeds the pinned verifier capacity."));
    }
    let (ir, zkir) = layout.emit(CircuitMode::LocalPublic)?;
    let mut claim = LiteralClaim {
        format: STATEMENT_FORMAT.into(),
        binding: BINDING_VERSION.into(),
        profile: PROFILE.into(),
        scope: SCOPE.into(),
        ledger_revision: LEDGER_REVISION.into(),
        zkir_version: ZKIR_VERSION.into(),
        rule_start_line: RULE_START_LINE as u32,
        rule_tag: RULE_TAG as u32,
        charge: RULE_CHARGE as u32,
        capacities: layout.record(),
        identity: layout.identity.clone(),
        layout_hash: hex(&layout.layout_hash),
        relation_hash: hex(&layout.relation_hash),
        context_hash: layout.context_hex(),
        zkir_hash: hex(&Sha256::digest(zkir.as_bytes())),
        // Widest admitted k until the native cost selection below.
        circuit_k: MAX_K,
        public_values: statement.iter().map(|v| v.to_string()).collect(),
    };
    // Mirror proving.rs: the bundle with the largest admitted proof must fit the
    // bounded JSON profile before any native k, key or proof work.
    crate::proving::validate_json_profile(&LiteralProofBundle { claim: claim.clone(), proof_hex: "00".repeat(MAX_PROOF_BYTES) })?;
    // Native cost selection, never ordinary metadata.
    let circuit_k = ir.k();
    if circuit_k == 0 || circuit_k > options.max_k {
        return Err(error("limit", format!("Circuit k={circuit_k} exceeds the explicit native k cap.")));
    }
    claim.circuit_k = circuit_k;
    let hash = crate::digest(&(BINDING_VERSION, &claim))?;
    let low = u128::from_le_bytes(hash[..16].try_into().expect("16 bytes"));
    let high = u128::from_le_bytes(hash[16..].try_into().expect("16 bytes"));
    let binding = Fr::from(low) + (Fr::from(u128::MAX) + Fr::from(1u64)) * Fr::from(high);
    Ok(Prepared { ir, values: statement.into_iter().map(Fr::from).collect(), binding, claim })
}
fn decode_proof(text: &str) -> Result<Proof, Error> {
    if text.is_empty() || text.len() > MAX_PROOF_BYTES * 2 || text.len() % 2 != 0 {
        return Err(error("proof", "Expected nonempty bounded even-length proof hex."));
    }
    let digit = |b: u8| match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        _ => None,
    };
    text.as_bytes().chunks_exact(2)
        .map(|p| Some((digit(p[0])? << 4) | digit(p[1])?))
        .collect::<Option<Vec<u8>>>()
        .map(Proof)
        .ok_or_else(|| error("proof", "Proof hex must use canonical lowercase digits."))
}
/// Native work: pinned SRS load, keygen, prove and producer-side verify.
pub fn prove_literal_transition(request: &LiteralTransitionRequest, options: &ProofOptions) -> Result<LiteralProofBundle, Error> {
    let prepared = prepare(request, options)?;
    let params = OfflineParameters::load(&options.params_dir, prepared.claim.circuit_k)?;
    let expected = prepared.statement();
    let preimage = ProofPreimage {
        inputs: vec![],
        private_transcript: vec![],
        public_transcript_inputs: prepared.values.clone(),
        public_transcript_outputs: prepared.values.clone(),
        binding_input: prepared.binding,
        communications_commitment: None,
        key_location: KeyLocation(KEY_LOCATION.into()),
    };
    let proof = futures::executor::block_on(async {
        let (pk, vk) = prepared.ir.keygen(&params).await.map_err(|e| error("keygen", e.to_string()))?;
        let (proof, actual, skips) = prepared.ir.prove(rand::rngs::OsRng, &params, pk, &preimage).await
            .map_err(|e| error("proving", e.to_string()))?;
        if actual != expected || skips.len() != prepared.values.len() || skips.iter().any(Option::is_some) {
            return Err(error("statement", "Native producer returned a different or skipped public statement."));
        }
        if proof.0.is_empty() || proof.0.len() > MAX_PROOF_BYTES {
            return Err(error("limit", "Native proof exceeds the nonempty 262144-byte cap."));
        }
        vk.verify(&PARAMS_VERIFIER, &proof, expected.into_iter()).map_err(|e| error("verification", e.to_string()))?;
        Ok(proof)
    })?;
    let bundle = LiteralProofBundle { claim: prepared.claim, proof_hex: hex(&proof.0) };
    crate::proving::validate_json_profile(&bundle)?;
    Ok(bundle)
}
/// Independent verification: rebuild layout, statement, circuit, claim and VK;
/// no bundle field, exported key or status is verification authority.
pub fn verify_literal_transition(request: &LiteralTransitionRequest, bundle: &LiteralProofBundle, options: &ProofOptions) -> Result<LiteralVerification, Error> {
    let prepared = prepare(request, options)?;
    if bundle.claim != prepared.claim {
        return Err(error("claim", "Claim differs from the independently reconstructed A102 statement."));
    }
    let proof = decode_proof(&bundle.proof_hex)?;
    let params = OfflineParameters::load(&options.params_dir, prepared.claim.circuit_k)?;
    let expected = prepared.statement();
    futures::executor::block_on(async {
        let vk = prepared.ir.keygen_vk(&params).await.map_err(|e| error("keygen", e.to_string()))?;
        vk.verify(&PARAMS_VERIFIER, &proof, expected.into_iter()).map_err(|e| error("verification", e.to_string()))
    })?;
    Ok(LiteralVerification {
        qualification: "local-native-a102-literal-transition-proof-unqualified",
        context_hash: prepared.claim.context_hash.clone(),
        zkir_hash: prepared.claim.zkir_hash.clone(),
        circuit_k: prepared.claim.circuit_k,
        proof_bytes: proof.0.len() as u32,
        open_obligations: OPEN_OBLIGATIONS.to_vec(),
    })
}

#[cfg(test)]
mod unrun_fixture_tests {
    //! UNRUN ordinary construction checks: no k, keygen, proof or verification.
    //! Capacities, stack limit and placeholder digests are construction data only.
    use super::*;
    fn zero(root: &[LiteralLane], tables: &[LiteralTable]) -> ImageValues {
        ImageValues {
            root: vec![0; root.len()],
            tables: tables.iter().map(|t| vec![vec![0; t.lanes.len()]; t.capacity]).collect(),
        }
    }
    fn set(image: &mut ImageValues, lanes: &[LiteralLane], at: Option<(usize, usize)>, name: &str, value: u128) {
        let column = lanes.iter().position(|l| l.name == name).unwrap();
        match at {
            None => image.root[column] = value,
            Some((t, r)) => image.tables[t][r][column] = value,
        }
    }
    fn fixture() -> (LiteralLayout, LiteralValues) {
        let digest = "00".repeat(32);
        let identity = LiteralSourceIdentity {
            k_source_sha256: digest.clone(),
            reference_source_sha256: digest.clone(),
            program_source_sha256: digest,
        };
        let capacities = LiteralCapacities {
            program: [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0],
            state: [0, 0, 0, 0, 0, 1],
            stack_limit: 5,
        };
        let layout = LiteralLayout::new(&capacities, &identity, local_max_statement_lanes()).unwrap();
        let s = layout.schema.clone();
        let mut program = zero(&s.program_header, &s.program_tables);
        for (name, v) in [("activeOccurrences", 1), ("activeExpressions", 1), ("activeTypes", 4),
            ("activeConstructors", 4), ("activeConstructorFieldTypes", 5), ("activeConstants", 1)] {
            set(&mut program, &s.program_header, None, name, v);
        }
        for (t, rows) in [
            (10usize, vec![vec![0u128, 0, 0], vec![1, 0, 0], vec![2, 0, 2], vec![3, 2, 2]]),
            (11, vec![vec![2, 0, 0, 0], vec![2, 1, 0, 2], vec![3, 0, 2, 1], vec![3, 1, 3, 2]]),
            (12, vec![vec![0], vec![2], vec![0], vec![3], vec![3]]),
        ] {
            for (r, row) in rows.into_iter().enumerate() {
                program.tables[t][r] = row;
            }
        }
        set(&mut program, &s.program_tables[13].lanes, Some((13, 0)), "uintBytes.0", 7);
        let mut pre = zero(&s.state_root, &s.state_tables);
        for (name, v) in [("controlLength", 1), ("stackLimit", 5), ("remainingFuel.0", 1), ("initialFuel.0", 1)] {
            set(&mut pre, &s.state_root, None, name, v);
        }
        let mut post = pre.clone();
        set(&mut post, &s.state_root, None, "remainingFuel.0", 0);
        set(&mut post, &s.state_tables[5].lanes, Some((5, 0)), "tag", 1);
        (layout, LiteralValues { program, pre, post, rule_tag: 3, charge: 1 })
    }
    #[test]
    fn uint7_literal_fixture_builds_complete_statement_and_both_relations() {
        let (layout, values) = fixture();
        let statement = layout.statement(&values).unwrap();
        assert_eq!(statement.len(), layout.statement_len());
        assert_eq!(&statement[..2], &layout.limbs()[..]);
        let (local, _) = layout.emit(CircuitMode::LocalPublic).unwrap();
        assert!(!local.do_communications_commitment && local.inputs.is_empty());
        let (ledger, _) = layout.emit(CircuitMode::LedgerCall).unwrap();
        assert!(ledger.do_communications_commitment && ledger.outputs.is_empty());
        assert_eq!(ledger.inputs.len(), statement.len());
    }
    #[test]
    fn explicit_statement_lane_bound_refuses_before_layout_allocation() {
        let (layout, _) = fixture();
        let n = layout.statement_len();
        assert!(LiteralLayout::new(&layout.capacities, &layout.identity, n).is_ok());
        assert!(LiteralLayout::new(&layout.capacities, &layout.identity, n - 1).is_err());
    }
    #[test]
    fn wrong_rule_or_builtin_rows_refuse_before_native_work() {
        let (layout, values) = fixture();
        let mut bad = values.clone();
        bad.rule_tag = 4;
        assert!(layout.statement(&bad).is_err());
        let mut bad = values;
        bad.program.tables[11][0][0] = 3;
        assert!(layout.statement(&bad).is_err());
    }
}
