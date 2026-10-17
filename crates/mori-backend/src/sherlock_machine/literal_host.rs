//! Authoritative SDK-free host schema, data carriers, statement layout, and shared
//! backend error/digest primitives for the Sherlock A102 Literal bridge and producer.
//!
//! Native circuit emission (`LiteralLayout::emit`) and proof/verification entry points
//! remain in `literal_bridge`; instruction emission remains in `literal_step`.
//! For direct-rustc SDK-free compilation without `midnight-*` crates, compile this
//! file with `--cfg mori_sdk_free`.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const LEDGER_REVISION: &str = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14";
pub const ZKIR_VERSION: &str = "3.0.0-rc.2";

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[error("{code}: {message}")]
pub struct Error {
    pub code: String,
    pub message: String,
}

pub(crate) fn error(code: &str, message: impl Into<String>) -> Error {
    Error {
        code: format!("mori::backend::{code}"),
        message: message.into(),
    }
}

/// Exactly sized lowercase hex. Supplied field integers are never reduced.
pub(crate) fn unhex(text: &str, bytes: usize) -> std::result::Result<Vec<u8>, Error> {
    if text.len() != 2 * bytes
        || !text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(error(
            "signature",
            "Expected exactly sized lowercase hexadecimal.",
        ));
    }
    (0..text.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&text[i..i + 2], 16)
                .map_err(|_| error("signature", "Invalid hexadecimal."))
        })
        .collect()
}

pub fn decode_digest(text: &str) -> std::result::Result<[u8; 32], Error> {
    unhex(text, 32)?
        .try_into()
        .map_err(|_| error("authorization", "Expected SHA256 digest."))
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstructionError {
    EmptyShape,
    EmptyOrRepeatedColumnName,
    SizeOverflow,
    RowCount,
    ColumnCount { row: usize },
    CellType { row: usize, column: usize },
    EmptyScope,
    UnprefixedVariable,
    TemporaryNameCollision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiteralCapacities {
    /// N, NB, O, OP, X, XC, F, P, R, A, T, Q, QF, K, KF.
    pub program: [usize; 15],
    /// D, DF, E, V, S, C. No numeric defaults or resource admission.
    pub state: [usize; 6],
    pub stack_limit: u128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiteralLane {
    pub name: String,
    pub bits: u32,
    pub boolean: bool,
    /// Inclusive scalar bound, separate from reference active-prefix checks.
    pub maximum: Option<u128>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiteralTable {
    pub name: &'static str,
    pub capacity: usize,
    pub lanes: Vec<LiteralLane>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiteralSchema {
    pub program_header: Vec<LiteralLane>,
    pub program_tables: Vec<LiteralTable>,
    pub state_root: Vec<LiteralLane>,
    pub state_tables: Vec<LiteralTable>,
    pub(crate) capacities: LiteralCapacities,
}
impl LiteralSchema {
    pub const fn capacities(&self) -> &LiteralCapacities {
        &self.capacities
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiteralConstructionError {
    MinimumShape,
    SizeOverflow,
    SchemaMismatch,
    ImageShape { block: &'static str, table: Option<usize>, row: Option<usize> },
    Fetch(ConstructionError),
}
impl From<ConstructionError> for LiteralConstructionError {
    fn from(e: ConstructionError) -> Self { Self::Fetch(e) }
}
type SchemaResult<T> = std::result::Result<T, LiteralConstructionError>;

pub(crate) fn unsigned(name: &str, bits: u32, maximum: Option<u128>) -> LiteralLane {
    LiteralLane { name: name.into(), bits, boolean: false, maximum }
}
fn id(name: &str, capacity: u128) -> LiteralLane {
    unsigned(name, (128 - capacity.saturating_sub(1).leading_zeros()).max(1), None)
}
fn count(name: &str, capacity: u128) -> LiteralLane {
    unsigned(name, (128 - capacity.leading_zeros()).max(1), Some(capacity))
}
pub(crate) fn tag(name: &str, variants: u128) -> LiteralLane {
    let mut lane = id(name, variants); lane.maximum = Some(variants - 1); lane
}
fn boolean(name: &str) -> LiteralLane {
    LiteralLane { name: name.into(), bits: 1, boolean: true, maximum: None }
}
fn prefixed(prefix: &str, fields: Vec<LiteralLane>) -> Vec<LiteralLane> {
    fields.into_iter().map(|mut f| { f.name = format!("{prefix}.{}", f.name); f }).collect()
}
fn slice(prefix: &str, capacity: u128) -> Vec<LiteralLane> {
    prefixed(prefix, vec![count("start", capacity), count("len", capacity)])
}
fn value_ref(prefix: &str, capacity: u128) -> Vec<LiteralLane> {
    prefixed(prefix, vec![boolean("space"), id("index", capacity)])
}
fn env_root(prefix: &str, capacity: u128) -> Vec<LiteralLane> {
    prefixed(prefix, vec![boolean("present"), id("index", capacity)])
}
fn bytes(prefix: &str) -> Vec<LiteralLane> {
    (0..16).map(|i| unsigned(&format!("{prefix}.{i}"), 8, None)).collect()
}
fn node(t: u128, q: u128, fields: u128) -> Vec<LiteralLane> {
    let mut row = vec![tag("kind", 3), id("typeId", t), id("ctorId", q)];
    row.extend(bytes("uintBytes")); row.push(boolean("boolValue"));
    row.extend([count("fieldStart", fields), count("fieldLength", fields)]); row
}

/// Exact L4 logical columns. Zero-capacity unused tables remain represented.
/// Serialized byte/Boolean expansion and its connection to these nonfuel
/// carriers is unwaived OBL.G4/StepInv, not implemented by this schema helper.
pub fn literal_schema(c: &LiteralCapacities) -> SchemaResult<LiteralSchema> {
    let [n, nb, o, op, x, xc, f, p, r, a, t, q, qf, k, kf] = c.program.map(|v| v as u128);
    let [d, df, e, v, s, control] = c.state.map(|v| v as u128);
    if x == 0 || k == 0 || control == 0 || t < 4 || q < 4 || qf < 5 {
        return Err(LiteralConstructionError::MinimumShape);
    }
    let value_capacity = k.max(d);
    let mut expression = vec![tag("tag", 8), id("occurrenceId", o)];
    expression.extend(value_ref("valueRef", value_capacity));
    expression.extend([id("nameId", n), tag("binaryOp", 7), tag("unaryOp", 2)]);
    expression.extend(slice("childSlice", xc)); expression.extend(slice("armSlice", a));
    let mut function = vec![id("sourceTableIndex", f), id("nameId", n)];
    function.extend(slice("parameterSlice", p)); function.extend([id("returnType", t), id("bodyExprId", x)]);
    let mut constructor = vec![id("owningType", t), count("sourceOrder", q)];
    constructor.extend(slice("fieldTypeSlice", qf));
    let path_position = xc.max(a.checked_add(1).ok_or(LiteralConstructionError::SizeOverflow)?);
    let program_rows = vec![
        vec![count("byteStart", nb), count("byteLength", nb)],
        vec![unsigned("byte", 8, None)],
        vec![tag("ownerTag", 2), id("functionIndex", f), count("pathStart", op), count("pathLength", op)],
        vec![count("childPosition", path_position)], expression,
        vec![id("expression", x)], function,
        vec![id("nameId", n), id("typeId", t)],
        vec![tag("tag", 5), id("name0", n), id("name1", n)],
        vec![id("patternId", r), id("bodyExprId", x)],
        vec![tag("kind", 4), count("ctorStart", q), count("ctorLength", q)], constructor,
        vec![id("typeId", t)], node(t, q, kf), value_ref("value", value_capacity),
    ];
    let program_names = ["Names", "NameBytes", "Occurrences", "OccurrencePaths", "Expressions", "ExprChildren", "Functions", "Parameters", "Patterns", "Arms", "Types", "Constructors", "ConstructorFieldTypes", "Constants", "ConstantFields"];
    let program_tables: Vec<_> = program_rows.into_iter().enumerate().map(|(i, lanes)|
        LiteralTable { name: program_names[i], capacity: c.program[i], lanes }).collect();
    let mut header = vec![id("entryExprId", x)];
    for (i, name) in program_names.iter().enumerate() {
        header.push(count(&format!("active{name}"), c.program[i] as u128));
    }
    let mut frame = vec![tag("tag", 9), id("nameId", n), tag("binaryOp", 7), tag("unaryOp", 2), id("typeId", t), id("expr0", x), id("expr1", x)];
    frame.extend(env_root("savedEnv", e)); frame.extend(value_ref("valueRef", value_capacity));
    frame.extend(slice("completedSlice", v)); frame.extend(slice("remainingExprSlice", xc)); frame.extend(slice("armSlice", a));
    let mut cr = vec![tag("tag", 10), id("exprId", x)];
    cr.extend(value_ref("valueRef", value_capacity)); cr.extend(prefixed("pendingFrame", frame.clone()));
    cr.extend(env_root("environment", e)); cr.push(id("nameId", n)); cr.extend(slice("valueSlice", v));
    cr.extend([boolean("functionPresent"), id("functionId", f), boolean("bindingsPresent")]);
    cr.extend(env_root("bindings", e)); cr.push(id("typeId", t)); cr.extend(slice("armSlice", a));
    cr.extend([boolean("primitiveIsOk"), unsigned("errorCode", 8, None)]);
    let mut env = vec![id("nameId", n)]; env.extend(value_ref("valueRef", value_capacity)); env.extend(env_root("parent", e));
    let state_rows = vec![node(t, q, df), value_ref("value", value_capacity), env, value_ref("value", value_capacity), frame, cr];
    let state_names = ["Data", "DataFields", "EnvCells", "ValueSlots", "FrameSlots", "ControlRows"];
    let state_tables: Vec<_> = state_rows.into_iter().enumerate().map(|(i, lanes)|
        LiteralTable { name: state_names[i], capacity: c.state[i], lanes }).collect();
    let mut root = vec![count("activeData", d), count("activeDataFields", df), count("activeEnvCells", e), count("activeValueSlots", v), count("depth", s), count("controlLength", control)];
    root.extend(env_root("environment", e)); root.push(count("stackLimit", c.stack_limit));
    root.extend(bytes("remainingFuel")); root.extend(bytes("initialFuel")); root.push(tag("outcomeTag", 4));
    root.extend(value_ref("outcomeValue", value_capacity)); root.push(unsigned("outcomeError", 8, None));
    // Host construction arithmetic only, not a resource bound/admission.
    let mut cells = header.len().checked_add(root.len().checked_mul(2).ok_or(LiteralConstructionError::SizeOverflow)?)
        .ok_or(LiteralConstructionError::SizeOverflow)?;
    for (tables, copies) in [(&program_tables, 1usize), (&state_tables, 2usize)] {
        for table in tables {
            cells = cells.checked_add(table.capacity.checked_mul(table.lanes.len()).and_then(|v| v.checked_mul(copies))
                .ok_or(LiteralConstructionError::SizeOverflow)?).ok_or(LiteralConstructionError::SizeOverflow)?;
        }
    }
    cells.checked_mul(32).ok_or(LiteralConstructionError::SizeOverflow)?;
    Ok(LiteralSchema { program_header: header, program_tables, state_root: root, state_tables, capacities: c.clone() })
}

pub const STATEMENT_FORMAT: &str = "moriarty-sherlock-a102-literal-statement/1";
pub const BINDING_VERSION: &str = "moriarty-sherlock-a102-literal-binding/1";
pub const PROFILE: &str = "sherlock-k-a/1";
pub const SCOPE: &str = "one-charged-transition:A102-Eval-Literal-to-Return";
pub const RULE_START_LINE: u128 = 102;
pub const RULE_TAG: u128 = 3;
pub const RULE_CHARGE: u128 = 1;
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
#[derive(Clone, Debug)]
pub struct LiteralTransitionRequest {
    pub capacities: LiteralCapacities,
    pub identity: LiteralSourceIdentity,
    pub values: LiteralValues,
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
pub(crate) fn construction(e: LiteralConstructionError) -> Error {
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
    pub(crate) schema: LiteralSchema,
    pub(crate) capacities: LiteralCapacities,
    pub(crate) identity: LiteralSourceIdentity,
    pub(crate) lanes: Vec<(String, LiteralLane)>,
    pub(crate) layout_hash: [u8; 32],
    pub(crate) relation_hash: [u8; 32],
    pub(crate) context: [u8; 32],
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
            include_str!("literal_host.rs"),
            include_str!("literal_step.rs"),
            include_str!("literal_ui_valid.rs"),
            include_str!("literal_bridge.rs"),
            include_str!("literal_witness.rs"),
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
    pub(crate) fn limbs(&self) -> [u128; 2] {
        let (lo, hi) = self.context.split_at(16);
        [u128::from_le_bytes(lo.try_into().expect("16 bytes")), u128::from_le_bytes(hi.try_into().expect("16 bytes"))]
    }
    pub(crate) fn record(&self) -> CapacityRecord {
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
}

#[cfg(mori_sdk_free)]
pub use self as literal_host;
#[cfg(mori_sdk_free)]
pub mod literal_witness;
#[cfg(all(test, mori_sdk_free))]
mod literal_witness_normal_tests;
#[cfg(mori_sdk_free)]
pub mod return_publication_witness;
#[cfg(all(test, mori_sdk_free))]
mod return_publication_witness_normal_tests;
