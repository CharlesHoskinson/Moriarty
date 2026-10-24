//! Sherlock composed two-step (`A102 -> A80`) Literal native statement bridge.
//!
//! Binds one immutable `ProgramBlock`, three complete `StateBlock` images
//! (`init`, `intermediate`, `done`), and both fixed step envelopes
//! (`literal_envelope = (3, 1)` for line 102 and `return_envelope = (0, 0)` for
//! line 80) into a distinct statement layout and circuit relation that invokes
//! [`emit_literal_complete_execution`] with one shared `program` image and one
//! exact aliased `intermediate` image (`A102.post == A80.pre`).
//!
//! # Critical Step-Mode Qualification & Unwaived Canonical Obligations
//! - **Non-Canonical Step-Mode Endpoint (`done`)**: Both `intermediate` and `done`
//!   are Step-mode Constant-space images (`done.root.outcomeTag == 1`,
//!   `done.root.outcomeValue.space == 0`, `activeData == 0`, `activeDataFields == 0`).
//!   `done` is **never** a canonical-boundary `StateBlock` (`space == 1`).
//!   Unfolding reachable `Constant` nodes into `Data`/`DataFields` (`projectA`),
//!   `Opus F3` unfolded capacity admission, and `Opus F5` incremental host
//!   unfolding remain unimplemented and block canonical checkpoint, chunk, or
//!   external canonical-state claims.
//! - **Unimplemented `F4` / `StepInv` / `OBL.G4` / 39-Row Dispatch**: Whole-program
//!   `prepareA(source)` equality (`GAP-1` / `F4`), general `StepInv` (`O1`..`O4`),
//!   `OBL.G4` canonical byte serialization, line-84 zero-fuel startup exhaustion,
//!   and general 39-row `emitStepA` dispatch remain open caller obligations.
//! - **Ledger-9 Typed Carrier Seam**: Existing `mori_ledger::sherlock_literal`
//!   and `mori_ledger::sherlock_transaction` concretely store single-step A102
//!   `LiteralLayout` and `LiteralValues`; they do not accept this composed
//!   layout/request without a subsequent newtyped ledger carrier seam.
//! - **Native Authority**: `0` (source-only; compile/keygen/prove/verify unobserved).

use super::literal_bridge::CircuitMode;
pub use super::literal_bridge::{CircuitMode as CompleteCircuitMode, local_max_statement_lanes};
pub use super::literal_complete_execution::{
    COMPLETE_LITERAL_EXECUTION_SCOPE_STEP1_SUFFIX, COMPLETE_LITERAL_EXECUTION_SCOPE_STEP2_SUFFIX,
    COMPLETE_LITERAL_EXECUTION_STEP1_RULE_CHARGE, COMPLETE_LITERAL_EXECUTION_STEP1_RULE_START_LINE,
    COMPLETE_LITERAL_EXECUTION_STEP1_RULE_TAG, COMPLETE_LITERAL_EXECUTION_STEP2_RULE_CHARGE,
    COMPLETE_LITERAL_EXECUTION_STEP2_RULE_START_LINE, COMPLETE_LITERAL_EXECUTION_STEP2_RULE_TAG,
    COMPLETE_LITERAL_EXECUTION_TOTAL_CHARGE, LiteralCompleteExecutionBlock,
    LiteralCompleteExecutionInputs, emit_literal_complete_execution,
    is_reserved_literal_complete_execution_variable,
};
pub use super::literal_host::{
    CapacityRecord, Error, ImageValues, LEDGER_REVISION, LiteralCapacities, LiteralLane,
    LiteralSchema, LiteralSourceIdentity, LiteralTable, PROFILE, ZKIR_VERSION, decode_digest,
    literal_schema,
};
use super::literal_host::{construction, error, hex};
use super::return_publication_witness::{
    ProducedLiteralReturnPublication, produce_a102_literal_return_publication,
};
use super::*;
use midnight_zkir_v3::ir::TypedIdentifier;
use mori_core::sherlock::Program;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const COMPLETE_STATEMENT_FORMAT: &str =
    "moriarty-sherlock-a102-a80-literal-complete-statement/1";
pub const COMPLETE_BINDING_VERSION: &str = "moriarty-sherlock-a102-a80-literal-complete-binding/1";
pub const COMPLETE_RELATION_DOMAIN: &str = "moriarty-sherlock-a102-a80-literal-complete-relation/1";
pub const COMPLETE_CONTEXT_DOMAIN: &str = "moriarty-sherlock-a102-a80-literal-complete-context/1";
pub const COMPLETE_PROFILE: &str = PROFILE;
pub const COMPLETE_SCOPE: &str = "two-step-step-mode-literal-execution:Init-A102-Eval-Literal-to-Return-then-A80-Return-to-StepDone-noncanonical-constant-space";
pub const COMPLETE_EXECUTION_SCOPE: &str = "sherlock.a102_a80.complete";
pub const COMPLETE_STATEMENT_VARIABLE_PREFIX: &str = "%sherlock.a102_a80.statement";
pub const COMPLETE_KEY_LOCATION: &str = "moriarty-sherlock-a102-a80-literal-complete/1";

pub const COMPLETE_OPEN_OBLIGATIONS: [&str; 9] = [
    "canonical ProgramBlock equals prepareA(source) (GAP-1 unimplemented); the F4 least-equal mechanism is unselected and applies only at canonical boundaries",
    "both intermediate (A102.post == A80.pre) and done (A80.post) are Step-mode Constant-space images (done.root.outcomeValue.space == 0, activeData == 0), never canonical-boundary StateBlocks; unfolding reachable Constant nodes into Data/DataFields (projectA), Opus F3 unfolded capacity admission, and Opus F5 incremental overflow refusal remain unimplemented and block canonical checkpoint, chunk, or external state claims",
    "full StepInv/O1-O4 bounds, acyclicity, Opus F2 suffix slices, variant padding, and Constants typing/literal uiValid beyond the enforced subset, which no ledger operation VK enforces",
    "OBL.G4 canonical-byte projection of nonfuel carriers and its in-circuit correspondence to native field lanes",
    "the other 37 rows of the 39-row public rule_line_tags dictionary, line-84 zero-fuel startup exhaustion, K's invalid-Literal sibling head, and general multi-step dispatch",
    "trace, chunk, and profile binding across proofs and K/reference/program source correspondence; the three source digests in LiteralSourceIdentity are caller-asserted metadata, not canonicality proofs",
    "tree source-closure reconciliation: Root 47c21c32 (PR167) mod.rs (46c8d321) and mori-ledger/src/proving.rs (b439e768) register sibling A82 (error_publication_step.rs, d9062342) and A84 (fuel_exhaustion_step.rs, 7d7cd9a0); LiteralCompleteLayout::new retains its exact 13-file executed-closure include_str! list (excluding uncalled A82/A84 bodies, matching LiteralLayout::new), and registering literal_complete_bridge in mod.rs + proving.rs will deterministically update both LiteralLayout and LiteralCompleteLayout relation_hash/context digests",
    "ledger-9 concrete type barrier: mori_ledger::sherlock_literal and mori_ledger::sherlock_transaction concretely store A102 LiteralLayout and LiteralValues; a newtyped ledger carrier seam is required before derive_keys, raw_call_intent, prove_call, verify_call, or transaction assembly can consume LiteralCompleteLayout and LiteralCompleteValues",
    "SRS provenance, keygen correspondence, native compilation/proving/verification execution, and adversarial soundness review remain unobserved (native_authority == 0)",
];

pub type Result<T> = std::result::Result<T, Error>;

/// Public wire envelope values `(rule_tag, charge)` for one step in the composed statement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiteralEnvelopeValues {
    pub rule_tag: u128,
    pub charge: u128,
}

impl LiteralEnvelopeValues {
    /// Fixed step-1 envelope for `A102` (`rule_tag = 3`, `charge = 1`).
    pub const fn step1() -> Self {
        Self {
            rule_tag: COMPLETE_LITERAL_EXECUTION_STEP1_RULE_TAG,
            charge: COMPLETE_LITERAL_EXECUTION_STEP1_RULE_CHARGE,
        }
    }

    /// Fixed step-2 envelope for `A80` (`rule_tag = 0`, `charge = 0`).
    pub const fn step2() -> Self {
        Self {
            rule_tag: COMPLETE_LITERAL_EXECUTION_STEP2_RULE_TAG,
            charge: COMPLETE_LITERAL_EXECUTION_STEP2_RULE_CHARGE,
        }
    }
}

/// Concrete logical lanes for the two-step `Init -> PendingConstantReturn -> StepDone` statement.
///
/// Every value is public. `intermediate` and `done` are Step-mode Constant-space
/// images (`space == 0`, `activeData == 0`), never canonical-boundary `StateBlock`s.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiteralCompleteValues {
    pub program: ImageValues,
    pub init: ImageValues,
    pub intermediate: ImageValues,
    pub done: ImageValues,
    pub literal_envelope: LiteralEnvelopeValues,
    pub return_envelope: LiteralEnvelopeValues,
}

impl LiteralCompleteValues {
    /// Construct complete-execution values with fixed `(3, 1)` and `(0, 0)` step envelopes.
    pub fn new(
        program: ImageValues,
        init: ImageValues,
        intermediate: ImageValues,
        done: ImageValues,
    ) -> Self {
        Self {
            program,
            init,
            intermediate,
            done,
            literal_envelope: LiteralEnvelopeValues::step1(),
            return_envelope: LiteralEnvelopeValues::step2(),
        }
    }
}

/// Complete two-step transition request binding capacities, caller-asserted source
/// digests, and the four public images plus two wire envelopes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiteralCompleteExecutionRequest {
    pub capacities: LiteralCapacities,
    pub identity: LiteralSourceIdentity,
    pub values: LiteralCompleteValues,
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

fn wide(name: &str) -> LiteralLane {
    LiteralLane {
        name: name.into(),
        bits: 128,
        boolean: false,
        maximum: None,
    }
}

fn image_lane_count(root: &[LiteralLane], tables: &[LiteralTable]) -> Option<usize> {
    tables.iter().try_fold(root.len(), |n, t| {
        n.checked_add(t.capacity.checked_mul(t.lanes.len())?)
    })
}

/// Checked statement length from schema shapes: two context limbs, one `ProgramBlock`,
/// three `StateBlock`s (`init`, `intermediate`, `done`), and two `(rule_tag, charge)` envelopes.
fn complete_statement_lane_count(s: &LiteralSchema) -> Option<usize> {
    let program = image_lane_count(&s.program_header, &s.program_tables)?;
    let state = image_lane_count(&s.state_root, &s.state_tables)?;
    program.checked_add(state.checked_mul(3)?)?.checked_add(6)
}

fn image_lanes(
    out: &mut Vec<(String, LiteralLane)>,
    block: &str,
    root: &[LiteralLane],
    tables: &[LiteralTable],
) {
    out.extend(root.iter().map(|l| (format!("{block}.root"), l.clone())));
    for t in tables {
        for r in 0..t.capacity {
            out.extend(
                t.lanes
                    .iter()
                    .map(|l| (format!("{block}.{}.{r}", t.name), l.clone())),
            );
        }
    }
}

fn shape(
    image: &ImageValues,
    root: &[LiteralLane],
    tables: &[LiteralTable],
    block: &str,
) -> Result<()> {
    let ok = image.root.len() == root.len()
        && image.tables.len() == tables.len()
        && image.tables.iter().zip(tables).all(|(rows, t)| {
            rows.len() == t.capacity && rows.iter().all(|r| r.len() == t.lanes.len())
        });
    if ok {
        Ok(())
    } else {
        Err(error(
            "sherlock-shape",
            format!("{block} image differs from the L4 schema."),
        ))
    }
}

fn state_root_val(s: &LiteralSchema, image: &ImageValues, name: &str) -> u128 {
    let idx = s
        .state_root
        .iter()
        .position(|l| l.name == name)
        .expect("fixed L4 state_root lane");
    image.root[idx]
}

fn state_table_val(
    s: &LiteralSchema,
    image: &ImageValues,
    table: usize,
    row: usize,
    name: &str,
) -> u128 {
    let idx = s.state_tables[table]
        .lanes
        .iter()
        .position(|l| l.name == name)
        .expect("fixed L4 state_table lane");
    image.tables[table][row][idx]
}

fn program_table_val(
    s: &LiteralSchema,
    image: &ImageValues,
    table: usize,
    row: usize,
    name: &str,
) -> u128 {
    let idx = s.program_tables[table]
        .lanes
        .iter()
        .position(|l| l.name == name)
        .expect("fixed L4 program_table lane");
    image.tables[table][row][idx]
}

// Off-chain host subset of representation and Step-mode endpoint checks.
// This is tooling validation only (never an Outcome and never a substitute for StepInv,
// F4, OBL.G4, or canonical projectA unfolding).
fn complete_representation(s: &LiteralSchema, v: &LiteralCompleteValues) -> Result<()> {
    let refuse = |what: &str| error("sherlock-representation", what.to_string());
    let padded = |rows: &Vec<Vec<u128>>, active: u128| {
        rows.iter()
            .enumerate()
            .all(|(r, row)| (r as u128) < active || row.iter().all(|c| *c == 0))
    };
    let p = &v.program;
    if !p
        .tables
        .iter()
        .enumerate()
        .all(|(i, rows)| padded(rows, p.root[i + 1]))
    {
        return Err(refuse(
            "Program rows at or above an active length are nonzero.",
        ));
    }
    for image in [&v.init, &v.intermediate, &v.done] {
        if !image
            .tables
            .iter()
            .enumerate()
            .all(|(i, rows)| padded(rows, image.root[i]))
        {
            return Err(refuse(
                "State rows at or above an active length are nonzero.",
            ));
        }
    }
    if p.root[0] != 0 || p.root[5] == 0 || p.root[14] == 0 {
        return Err(refuse(
            "Header needs entryExprId=0, activeExpressions>=1, and activeConstants>=1.",
        ));
    }
    let builtin: [(usize, Vec<Vec<u128>>); 3] = [
        (
            10,
            vec![vec![0, 0, 0], vec![1, 0, 0], vec![2, 0, 2], vec![3, 2, 2]],
        ),
        (
            11,
            vec![
                vec![2, 0, 0, 0],
                vec![2, 1, 0, 2],
                vec![3, 0, 2, 1],
                vec![3, 1, 3, 2],
            ],
        ),
        (12, vec![vec![0], vec![2], vec![0], vec![3], vec![3]]),
    ];
    for (table, rows) in &builtin {
        if p.root[*table + 1] != rows.len() as u128 || p.tables[*table][..rows.len()] != rows[..] {
            return Err(refuse(
                "Builtin Types/Constructors/ConstructorFieldTypes rows differ.",
            ));
        }
    }
    for (table, lane) in [(4usize, "occurrenceId"), (6, "sourceTableIndex")] {
        let column = s.program_tables[table]
            .lanes
            .iter()
            .position(|l| l.name == lane)
            .expect("fixed L4 column");
        if p.tables[table]
            .iter()
            .enumerate()
            .any(|(i, row)| (i as u128) < p.root[table + 1] && row[column] != i as u128)
        {
            return Err(refuse("An active row index column differs from its row."));
        }
    }

    // Entry expression must be Literal(0) in Constant space(0) within activeConstants.
    let entry_tag = program_table_val(s, p, 4, 0, "tag");
    let entry_space = program_table_val(s, p, 4, 0, "valueRef.space");
    let entry_const_idx = program_table_val(s, p, 4, 0, "valueRef.index");
    if entry_tag != 0 || entry_space != 0 || entry_const_idx >= p.root[14] {
        return Err(refuse(
            "Entry expression row 0 must be Expr::Literal with an active Constant-space ValueRef.",
        ));
    }

    // Init must match Machine::new boundary equations.
    for zero_lane in [
        "activeData",
        "activeDataFields",
        "activeEnvCells",
        "activeValueSlots",
        "depth",
        "environment.present",
        "environment.index",
        "outcomeTag",
        "outcomeValue.space",
        "outcomeValue.index",
        "outcomeError",
    ] {
        if state_root_val(s, &v.init, zero_lane) != 0 {
            return Err(refuse(
                "Init state root violates Machine::new zero equations.",
            ));
        }
    }
    if state_root_val(s, &v.init, "controlLength") != 1 {
        return Err(refuse("Init state root requires controlLength=1."));
    }
    if v.init
        .tables
        .iter()
        .flatten()
        .flatten()
        .any(|cell| *cell != 0)
    {
        return Err(refuse(
            "Init state tables must be all zero for Eval(0) with empty arenas.",
        ));
    }

    // Intermediate must be Step-mode Pending Return(ConstantRef) with empty runtime arenas.
    for zero_lane in [
        "activeData",
        "activeDataFields",
        "activeEnvCells",
        "activeValueSlots",
        "depth",
        "environment.present",
        "environment.index",
        "outcomeTag",
        "outcomeValue.space",
        "outcomeValue.index",
        "outcomeError",
    ] {
        if state_root_val(s, &v.intermediate, zero_lane) != 0 {
            return Err(refuse(
                "Intermediate state root must retain empty arenas and Pending outcome.",
            ));
        }
    }
    if state_root_val(s, &v.intermediate, "controlLength") != 1
        || state_table_val(s, &v.intermediate, 5, 0, "tag") != 1
        || state_table_val(s, &v.intermediate, 5, 0, "valueRef.space") != 0
        || state_table_val(s, &v.intermediate, 5, 0, "valueRef.index") != entry_const_idx
        || s.state_tables[5]
            .lanes
            .iter()
            .zip(&v.intermediate.tables[5][0])
            .any(|(lane, &val)| {
                !matches!(
                    lane.name.as_str(),
                    "tag" | "valueRef.space" | "valueRef.index"
                ) && val != 0
            })
    {
        return Err(refuse(
            "Intermediate ControlRows[0] must be Step-mode Return aliasing the entry Constant ValueRef with all inactive Return columns zero.",
        ));
    }

    // Done must be Step-mode Constant-space Outcome::Done(1) with empty arenas and zero ControlRows.
    for zero_lane in [
        "activeData",
        "activeDataFields",
        "activeEnvCells",
        "activeValueSlots",
        "depth",
        "controlLength",
        "environment.present",
        "environment.index",
        "outcomeValue.space",
        "outcomeError",
    ] {
        if state_root_val(s, &v.done, zero_lane) != 0 {
            return Err(refuse(
                "Done state root must remain in Step-mode Constant space (space=0, activeData=0) with controlLength=0.",
            ));
        }
    }
    if state_root_val(s, &v.done, "outcomeTag") != 1
        || state_root_val(s, &v.done, "outcomeValue.index") != entry_const_idx
    {
        return Err(refuse(
            "Done state root must publish Outcome::Done(1) aliasing the intermediate Constant ValueRef.",
        ));
    }
    if v.done.tables[5].iter().flatten().any(|cell| *cell != 0) {
        return Err(refuse(
            "Done ControlRows must be completely zeroed after A80.",
        ));
    }

    // Stack limit and fuel equations across Init -> Intermediate -> Done.
    let init_sl = state_root_val(s, &v.init, "stackLimit");
    if state_root_val(s, &v.intermediate, "stackLimit") != init_sl
        || state_root_val(s, &v.done, "stackLimit") != init_sl
    {
        return Err(refuse(
            "stackLimit must remain invariant across Init, Intermediate, and Done.",
        ));
    }
    let read_fuel_u128 = |image: &ImageValues, prefix: &str| -> Result<u128> {
        let mut le = [0u8; 16];
        for (byte, slot) in le.iter_mut().enumerate() {
            let lane_val = state_root_val(s, image, &format!("{prefix}.{byte}"));
            *slot = u8::try_from(lane_val).map_err(|_| {
                refuse("Fuel byte lane exceeds u8 range in host representation check.")
            })?;
        }
        Ok(u128::from_le_bytes(le))
    };
    let init_rem_u128 = read_fuel_u128(&v.init, "remainingFuel")?;
    let init_ini_u128 = read_fuel_u128(&v.init, "initialFuel")?;
    let mid_rem_u128 = read_fuel_u128(&v.intermediate, "remainingFuel")?;
    let mid_ini_u128 = read_fuel_u128(&v.intermediate, "initialFuel")?;
    let done_rem_u128 = read_fuel_u128(&v.done, "remainingFuel")?;
    let done_ini_u128 = read_fuel_u128(&v.done, "initialFuel")?;
    let expected_mid_rem = init_rem_u128
        .checked_sub(u128::from(COMPLETE_LITERAL_EXECUTION_STEP1_RULE_CHARGE))
        .ok_or_else(|| refuse("Init remainingFuel must be >= 1 for charged A102 transition."))?;
    if init_rem_u128 != init_ini_u128
        || mid_ini_u128 != init_ini_u128
        || done_ini_u128 != mid_ini_u128
        || mid_rem_u128 != expected_mid_rem
        || done_rem_u128 != mid_rem_u128
    {
        return Err(refuse(
            "Fuel UInt128 violates Init remaining==initial >= 1, A102 minus-1 charge, initialFuel invariance, or A80 fuel preservation.",
        ));
    }
    Ok(())
}

fn take_image(
    it: &mut impl Iterator<Item = Operand>,
    root: &[LiteralLane],
    tables: &[LiteralTable],
) -> Result<LiteralImage> {
    let mut take = |n: usize| -> Result<Vec<Operand>> {
        let row: Vec<Operand> = (&mut *it).take(n).collect();
        if row.len() == n {
            Ok(row)
        } else {
            Err(error("sherlock-shape", "Statement operands ended early."))
        }
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

/// Value-independent relation identity for the composed two-step (`A102 -> A80`) circuit.
#[derive(Clone, Debug)]
pub struct LiteralCompleteLayout {
    pub(crate) schema: LiteralSchema,
    pub(crate) capacities: LiteralCapacities,
    pub(crate) identity: LiteralSourceIdentity,
    pub(crate) lanes: Vec<(String, LiteralLane)>,
    pub(crate) layout_hash: [u8; 32],
    pub(crate) relation_hash: [u8; 32],
    pub(crate) context: [u8; 32],
}

impl LiteralCompleteLayout {
    /// Construct the composed two-step statement layout after checking `max_statement_lanes`
    /// before any per-lane allocation.
    pub fn new(
        capacities: &LiteralCapacities,
        identity: &LiteralSourceIdentity,
        max_statement_lanes: usize,
    ) -> Result<Self> {
        for d in [
            &identity.k_source_sha256,
            &identity.reference_source_sha256,
            &identity.program_source_sha256,
        ] {
            decode_digest(d)?;
        }
        let schema = literal_schema(capacities).map_err(construction)?;
        let count = complete_statement_lane_count(&schema)
            .ok_or_else(|| error("limit", "Statement lane count overflows usize."))?;
        count
            .checked_mul(32)
            .ok_or_else(|| error("limit", "Statement byte count overflows usize."))?;
        if count > max_statement_lanes {
            return Err(error(
                "limit",
                "Statement lane count exceeds the caller's explicit bound.",
            ));
        }
        let mut lanes = Vec::with_capacity(count);
        lanes.push(("context".to_string(), wide("limb.0")));
        lanes.push(("context".to_string(), wide("limb.1")));
        image_lanes(
            &mut lanes,
            "program",
            &schema.program_header,
            &schema.program_tables,
        );
        image_lanes(&mut lanes, "init", &schema.state_root, &schema.state_tables);
        image_lanes(
            &mut lanes,
            "intermediate",
            &schema.state_root,
            &schema.state_tables,
        );
        image_lanes(&mut lanes, "done", &schema.state_root, &schema.state_tables);
        for env_prefix in ["literal_envelope", "return_envelope"] {
            lanes.push((
                env_prefix.into(),
                LiteralLane {
                    name: "rule_tag".into(),
                    bits: 6,
                    boolean: false,
                    maximum: Some(38),
                },
            ));
            lanes.push((
                env_prefix.into(),
                LiteralLane {
                    name: "charge".into(),
                    bits: 8,
                    boolean: false,
                    maximum: None,
                },
            ));
        }
        if lanes.len() != count {
            return Err(error(
                "sherlock-shape",
                "Statement layout differs from its admitted lane count.",
            ));
        }

        let mut h = Hasher::new(COMPLETE_STATEMENT_FORMAT);
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

        let mut h = Hasher::new(COMPLETE_RELATION_DOMAIN);
        for source in [
            include_str!("mod.rs"),
            include_str!("fetch.rs"),
            include_str!("enabled_fetch.rs"),
            include_str!("literal_host.rs"),
            include_str!("literal_step.rs"),
            include_str!("literal_ui_valid.rs"),
            include_str!("literal_bridge.rs"),
            include_str!("literal_witness.rs"),
            include_str!("return_publication_step.rs"),
            include_str!("return_publication_witness.rs"),
            include_str!("literal_complete_execution.rs"),
            include_str!("literal_complete_bridge.rs"),
            include_str!("../../../../models/sherlock/literal-step/MAPPING.md"),
        ] {
            h.bytes(source.as_bytes());
        }
        let relation_hash = h.finish();

        let mut h = Hasher::new(COMPLETE_CONTEXT_DOMAIN);
        for part in [
            COMPLETE_STATEMENT_FORMAT,
            COMPLETE_BINDING_VERSION,
            COMPLETE_PROFILE,
            COMPLETE_SCOPE,
            LEDGER_REVISION,
            ZKIR_VERSION,
            identity.k_source_sha256.as_str(),
            identity.reference_source_sha256.as_str(),
            identity.program_source_sha256.as_str(),
        ] {
            h.bytes(part.as_bytes());
        }
        for n in [
            COMPLETE_LITERAL_EXECUTION_STEP1_RULE_START_LINE,
            COMPLETE_LITERAL_EXECUTION_STEP1_RULE_TAG,
            COMPLETE_LITERAL_EXECUTION_STEP1_RULE_CHARGE,
            COMPLETE_LITERAL_EXECUTION_STEP2_RULE_START_LINE,
            COMPLETE_LITERAL_EXECUTION_STEP2_RULE_TAG,
            COMPLETE_LITERAL_EXECUTION_STEP2_RULE_CHARGE,
            COMPLETE_LITERAL_EXECUTION_TOTAL_CHARGE,
        ] {
            h.num(n);
        }
        h.bytes(&layout_hash);
        h.bytes(&relation_hash);
        let context = h.finish();

        Ok(Self {
            schema,
            capacities: capacities.clone(),
            identity: identity.clone(),
            lanes,
            layout_hash,
            relation_hash,
            context,
        })
    }

    pub const fn schema(&self) -> &LiteralSchema {
        &self.schema
    }

    pub const fn capacities(&self) -> &LiteralCapacities {
        &self.capacities
    }

    pub const fn identity(&self) -> &LiteralSourceIdentity {
        &self.identity
    }

    pub fn lanes(&self) -> &[(String, LiteralLane)] {
        &self.lanes
    }

    pub fn statement_len(&self) -> usize {
        self.lanes.len()
    }

    pub fn layout_hash(&self) -> [u8; 32] {
        self.layout_hash
    }

    pub fn relation_hash(&self) -> [u8; 32] {
        self.relation_hash
    }

    pub fn context(&self) -> [u8; 32] {
        self.context
    }

    pub fn context_hex(&self) -> String {
        hex(&self.context)
    }

    pub fn context_limbs(&self) -> [u128; 2] {
        let (lo, hi) = self.context.split_at(16);
        [
            u128::from_le_bytes(lo.try_into().expect("16 bytes")),
            u128::from_le_bytes(hi.try_into().expect("16 bytes")),
        ]
    }

    pub fn capacity_record(&self) -> CapacityRecord {
        CapacityRecord {
            program: self.capacities.program.iter().map(|c| *c as u64).collect(),
            state: self.capacities.state.iter().map(|c| *c as u64).collect(),
            stack_limit: self.capacities.stack_limit.to_string(),
        }
    }

    /// Ordered public statement after host admissibility of public data.
    pub fn statement(&self, v: &LiteralCompleteValues) -> Result<Vec<u128>> {
        let s = &self.schema;
        shape(&v.program, &s.program_header, &s.program_tables, "program")?;
        shape(&v.init, &s.state_root, &s.state_tables, "init")?;
        shape(
            &v.intermediate,
            &s.state_root,
            &s.state_tables,
            "intermediate",
        )?;
        shape(&v.done, &s.state_root, &s.state_tables, "done")?;

        let mut out = self.context_limbs().to_vec();
        for image in [&v.program, &v.init, &v.intermediate, &v.done] {
            out.extend(
                image
                    .root
                    .iter()
                    .chain(image.tables.iter().flatten().flatten())
                    .copied(),
            );
        }
        out.extend([
            v.literal_envelope.rule_tag,
            v.literal_envelope.charge,
            v.return_envelope.rule_tag,
            v.return_envelope.charge,
        ]);
        if out.len() != self.lanes.len() {
            return Err(error(
                "sherlock-shape",
                "Statement length differs from the layout.",
            ));
        }
        for (value, (path, lane)) in out.iter().zip(&self.lanes) {
            let fits = if lane.boolean {
                *value <= 1
            } else {
                lane.bits >= 128 || *value >> lane.bits == 0
            };
            if !fits || lane.maximum.is_some_and(|m| *value > m) {
                return Err(error(
                    "sherlock-range",
                    format!("{path}.{} is outside its L4 lane.", lane.name),
                ));
            }
        }
        if v.literal_envelope != LiteralEnvelopeValues::step1()
            || v.return_envelope != LiteralEnvelopeValues::step2()
        {
            return Err(error(
                "sherlock-envelope",
                "Only step-1 wire tag 3 with charge 1 (line 102) and step-2 wire tag 0 with charge 0 (line 80) are admitted.",
            ));
        }
        complete_representation(s, v)?;
        Ok(out)
    }

    /// Emit the bound two-step circuit relation and return the [`IrSource`], serialized
    /// ZKIR JSON string, and the underlying [`LiteralCompleteExecutionBlock`].
    pub fn emit_with_block(
        &self,
        mode: CircuitMode,
    ) -> Result<(IrSource, String, LiteralCompleteExecutionBlock)> {
        let local = mode == CircuitMode::LocalPublic;
        let mut code = Vec::new();
        let mut inputs: Vec<TypedIdentifier> = Vec::new();
        let mut operands = Vec::with_capacity(self.lanes.len());
        for position in 0..self.lanes.len() {
            let id = Identifier(format!("{COMPLETE_STATEMENT_VARIABLE_PREFIX}.{position}"));
            let operand = Operand::Variable(id.clone());
            if local {
                code.push(Instruction::PublicInput {
                    guard: None,
                    val_t: IrType::Native,
                    output: id,
                });
                code.push(Instruction::Impact {
                    guard: immediate(1),
                    inputs: vec![operand.clone()],
                });
            } else {
                inputs.push(
                    serde_json::from_value(serde_json::json!({"name": id, "type": IrType::Native}))
                        .map_err(|e| error("zkir", e.to_string()))?,
                );
            }
            operands.push(operand);
        }
        let [lo, hi] = self.context_limbs();
        code.push(Instruction::ConstrainEq {
            a: operands[0].clone(),
            b: immediate(lo),
        });
        code.push(Instruction::ConstrainEq {
            a: operands[1].clone(),
            b: immediate(hi),
        });
        let s = &self.schema;
        let mut rest = operands[2..].iter().cloned();
        let program = take_image(&mut rest, &s.program_header, &s.program_tables)?;
        let init = take_image(&mut rest, &s.state_root, &s.state_tables)?;
        let intermediate = take_image(&mut rest, &s.state_root, &s.state_tables)?;
        let done = take_image(&mut rest, &s.state_root, &s.state_tables)?;
        let (literal_rule_tag, literal_charge, return_rule_tag, return_charge) = match (
            rest.next(),
            rest.next(),
            rest.next(),
            rest.next(),
            rest.next(),
        ) {
            (Some(lt), Some(lc), Some(rt), Some(rc), None) => (lt, lc, rt, rc),
            _ => {
                return Err(error(
                    "sherlock-shape",
                    "Envelope operands differ from the layout.",
                ));
            }
        };

        let compose_inputs = LiteralCompleteExecutionInputs {
            program,
            init,
            intermediate: intermediate.clone(),
            done: done.clone(),
            literal_envelope: LiteralEnvelope {
                rule_tag: literal_rule_tag.clone(),
                charge: literal_charge.clone(),
            },
            return_envelope: LiteralEnvelope {
                rule_tag: return_rule_tag.clone(),
                charge: return_charge.clone(),
            },
        };
        let block = emit_literal_complete_execution(s, &compose_inputs, COMPLETE_EXECUTION_SCOPE)
            .map_err(construction)?;
        code.extend(block.instructions.iter().cloned());

        let vals: Vec<Operand> = if local {
            intermediate
                .root
                .iter()
                .chain(intermediate.tables.iter().flatten().flatten())
                .chain(done.root.iter())
                .chain(done.tables.iter().flatten().flatten())
                .cloned()
                .chain([
                    literal_rule_tag,
                    literal_charge,
                    return_rule_tag,
                    return_charge,
                    block.charged_work.clone(),
                ])
                .collect()
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
        let mut disk =
            serde_json::to_value(&ir).map_err(|e| error("serialization", e.to_string()))?;
        disk["version"] = serde_json::json!({"major": 3, "minor": 0});
        let zkir =
            serde_json::to_string(&disk).map_err(|e| error("serialization", e.to_string()))?;
        let parsed = IrSource::load(zkir.as_bytes()).map_err(|e| error("zkir", e.to_string()))?;
        if parsed != ir {
            return Err(error("zkir", "Pinned upstream round-trip changed the IR."));
        }
        Ok((ir, zkir, block))
    }

    /// Emit the full bound two-step relation in `LocalPublic` or `LedgerCall` mode.
    pub fn emit(&self, mode: CircuitMode) -> Result<(IrSource, String)> {
        let (ir, zkir, _) = self.emit_with_block(mode)?;
        Ok((ir, zkir))
    }
}

/// Adapt an H1 diagnostic [`ProducedLiteralReturnPublication`] into a
/// [`LiteralCompleteExecutionRequest`] without rerunning `Machine` or trusting a flag.
///
/// Verifies that the step-1 and step-2 `ProgramBlock` images are identical and
/// that step-1's post-state equals step-2's pre-state (`intermediate`).
pub fn build_literal_complete_execution_request(
    produced: &ProducedLiteralReturnPublication,
) -> Result<LiteralCompleteExecutionRequest> {
    let a102_values = &produced.literal_transition.request.values;
    let a80_values = &produced.values;
    if a102_values.program != a80_values.program {
        return Err(error(
            "sherlock-continuity",
            "Step-1 and step-2 ProgramBlock images differ in ProducedLiteralReturnPublication.",
        ));
    }
    if a102_values.post != a80_values.pre {
        return Err(error(
            "sherlock-continuity",
            "Step-1 post-state and step-2 pre-state intermediate images differ in ProducedLiteralReturnPublication.",
        ));
    }
    Ok(LiteralCompleteExecutionRequest {
        capacities: produced.literal_transition.request.capacities.clone(),
        identity: produced.literal_transition.request.identity.clone(),
        values: LiteralCompleteValues {
            program: a102_values.program.clone(),
            init: a102_values.pre.clone(),
            intermediate: a102_values.post.clone(),
            done: a80_values.post.clone(),
            literal_envelope: LiteralEnvelopeValues {
                rule_tag: a102_values.rule_tag,
                charge: a102_values.charge,
            },
            return_envelope: LiteralEnvelopeValues {
                rule_tag: a80_values.rule_tag,
                charge: a80_values.charge,
            },
        },
    })
}

/// Adapt a [`ProducedLiteralReturnPublication`] and check its layout and ordered
/// public statement under the caller's explicit `max_statement_lanes` bound.
pub fn build_checked_literal_complete_execution_request(
    produced: &ProducedLiteralReturnPublication,
    max_statement_lanes: usize,
) -> Result<(
    LiteralCompleteLayout,
    LiteralCompleteExecutionRequest,
    Vec<u128>,
)> {
    let request = build_literal_complete_execution_request(produced)?;
    let layout =
        LiteralCompleteLayout::new(&request.capacities, &request.identity, max_statement_lanes)?;
    let statement = layout.statement(&request.values)?;
    Ok((layout, request, statement))
}

/// Run the H1 diagnostic producer [`produce_a102_literal_return_publication`] and
/// adapt its output into a checked [`LiteralCompleteExecutionRequest`] and statement.
pub fn produce_checked_literal_complete_execution_request(
    program: Arc<Program>,
    capacities: &LiteralCapacities,
    initial_fuel: u128,
    stack_limit: usize,
    identity: &LiteralSourceIdentity,
    max_statement_lanes: usize,
) -> Result<(
    ProducedLiteralReturnPublication,
    LiteralCompleteLayout,
    LiteralCompleteExecutionRequest,
    Vec<u128>,
)> {
    let produced = produce_a102_literal_return_publication(
        program,
        capacities,
        initial_fuel,
        stack_limit,
        identity,
    )
    .map_err(|e| error("sherlock-witness", format!("{e:?}")))?;
    let (layout, request, statement) =
        build_checked_literal_complete_execution_request(&produced, max_statement_lanes)?;
    Ok((produced, layout, request, statement))
}
