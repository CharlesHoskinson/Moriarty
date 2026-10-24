//! Portable bounded transition relations for the pinned ledger-9 ZKIRv3.
//! Compilation emits a relation and public ABI without a scenario or success
//! flag. Explicit offline native proofs are available for the admitted public
//! frame; authenticated ledger correspondence remains an open gate. A signed
//! Target appends the strict owner-intent statement to the same circuit.
use authorization::{AuthorizationBinding, SignedPolicy};
use midnight_transient_crypto::curve::Fr;
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::{Action, AssetId, BinaryOp, Expr, ExprId, Program, Statement, Type, Value};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

pub mod authorization;
pub mod replay;
pub mod schnorr;
pub mod sherlock_child;
pub mod sherlock_link;
pub mod sherlock_machine;
pub mod signed_relation;
pub mod source_projection;
pub mod stage;

mod liabilities;
pub use liabilities::{
    LiabilityDescriptor, LiabilityIdentity, LiabilityOperation, MAX_LIABILITY_REGISTRY_ENTRIES,
};
pub mod parameters;
pub mod proving;
pub mod qa_mutation;

pub const ARTIFACT_FORMAT: &str = "moriarty-zkir-artifact/3";
pub const PUBLIC_ABI_VERSION: &str = "moriarty-public-abi/3";
pub const RELATION_VERSION: &str = "moriarty-zkir-relation/3";
/// Explicit authorization discriminator of an unsigned v3 artifact.
pub const UNSIGNED_MODE: &str = "unsigned";

pub(crate) use sherlock_machine::literal_host::error;
pub use sherlock_machine::literal_host::{Error, LEDGER_REVISION, ZKIR_VERSION};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Target {
    pub profile: String,
    pub network: String,
    pub anchor: String,
    pub ledger_revision: String,
    pub zkir_version: String,
    pub max_instructions: u32,
    pub max_artifact_bytes: u32,
    /// Absent (omitted) for unsigned targets; explicit null refuses.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present_policy"
    )]
    pub signed_policy: Option<SignedPolicy>,
}
fn present_policy<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<SignedPolicy>, D::Error> {
    SignedPolicy::deserialize(d).map(Some)
}
impl Target {
    pub fn ledger9(network: impl Into<String>, anchor: impl Into<String>) -> Self {
        Self {
            profile: "ledger-9".into(),
            network: network.into(),
            anchor: anchor.into(),
            ledger_revision: LEDGER_REVISION.into(),
            zkir_version: ZKIR_VERSION.into(),
            max_instructions: 100_000,
            max_artifact_bytes: 16_777_216,
            signed_policy: None,
        }
    }
    pub fn from_json(text: &str) -> Result<Self, Error> {
        let value = mori_core::json::parse(text).map_err(|e| error("target", e.to_string()))?;
        serde_json::from_value(value).map_err(|e| error("target", e.to_string()))
    }
    /// The explicit authorization discriminator selected by this Target.
    pub fn authorization_mode(&self) -> &'static str {
        if self.signed_policy.is_some() {
            authorization::INTENT_PROFILE
        } else {
            UNSIGNED_MODE
        }
    }
    fn validate(&self) -> Result<(), Error> {
        if self.profile != "ledger-9"
            || self.ledger_revision != LEDGER_REVISION
            || self.zkir_version != ZKIR_VERSION
        {
            return Err(error(
                "target",
                "Only the exact pinned ledger-9 ZKIR 3.0.0-rc.2 target is supported.",
            ));
        }
        if self.network.is_empty()
            || self.anchor.is_empty()
            || self.network.len() > 1024
            || self.anchor.len() > 1024
        {
            return Err(error(
                "target",
                "Explicit bounded network and anchor identifiers are required.",
            ));
        }
        if self.max_instructions == 0 || self.max_artifact_bytes == 0 {
            return Err(error(
                "limit",
                "Instruction and artifact limits must be positive.",
            ));
        }
        if let Some(policy) = &self.signed_policy {
            policy.validate()?;
            authorization::validate_label(&self.network)?;
            authorization::validate_label(&self.anchor)?;
        }
        Ok(())
    }
}
/// Positional ABI: every entry is read from public_transcript_outputs and
/// explicitly published by Impact in the same position in transcript_inputs.
/// Upstream PublicInput alone assigns a witness; it does not bind a public value.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicBinding {
    pub role: String,
    pub name: String,
    pub ty: Type,
    /// Explicit null when absent; omission refuses rather than defaulting.
    #[serde(deserialize_with = "Option::deserialize")]
    pub constant: Option<String>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub asset: Option<AssetId>,
}
/// Typed private witness descriptor; points are assigned as curve values.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivateType {
    #[serde(rename = "Point<Jubjub>")]
    JubjubPoint,
    Native,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PrivateBinding {
    pub name: String,
    pub ty: PrivateType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CircuitArtifact {
    pub action: String,
    pub zkir: String,
    /// SHA256 of the canonical 18-element source-fixed context preimage.
    pub context_hash: String,
    /// Financial positions 0..N, including context constants 0/1.
    pub public_inputs: Vec<PublicBinding>,
    /// Signed positions N..N+11+4A; empty for unsigned artifacts.
    pub authorization_bindings: Vec<AuthorizationBinding>,
    /// Typed private inputs, flattened `[PK.x,PK.y,R.x,R.y,s]`; empty if unsigned.
    pub private_inputs: Vec<PrivateBinding>,
    pub instruction_count: u32,
    pub effect_frame: Vec<EffectDescriptor>,
    pub balance_frame: Vec<BalanceDescriptor>,
    pub liability_frame: Vec<LiabilityDescriptor>,
    pub replay_steps: u32,
}
/// Static effect identities are committed through program/action context.
/// Every effect amount/value is separately public and constrained.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectDescriptor {
    pub step: u32,
    pub kind: String,
    // Static frame members are explicit null when absent; omission refuses.
    #[serde(deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub asset: Option<AssetId>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub from: Option<String>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub to: Option<String>,
    /// Liability effects use [before_presence, before_amount, after_presence, after_amount].
    pub public_positions: Vec<u32>,
    /// Full source-fixed duty identity; balance transfer fields remain separate.
    #[serde(deserialize_with = "Option::deserialize")]
    pub liability: Option<LiabilityIdentity>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub operation: Option<LiabilityOperation>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BalanceDescriptor {
    pub asset: AssetId,
    pub account: String,
    pub pre_position: u32,
    pub post_position: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Resources {
    pub instructions: u32,
    pub zkir_bytes: u32,
    pub artifact_bytes: u32,
    pub public_values: u32,
    pub max_replay_steps: u32,
    #[serde(deserialize_with = "Option::deserialize")]
    pub circuit_k: Option<u8>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub fraud_max_k: Option<u8>,
    pub cost_measured: bool,
    #[serde(deserialize_with = "decode_registry_entries")]
    pub liability_registry_entries: u32,
    #[serde(deserialize_with = "decode_registry_ceiling")]
    pub max_liability_registry_entries: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gate {
    pub predicate: String,
    pub status: String,
}
/// Decoding validates the whole artifact: versions, Target, the explicit mode
/// against the Target, and every circuit's exact descriptor shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "ArtifactFields")]
pub struct Artifact {
    pub format: String,
    pub abi: String,
    pub relation: String,
    pub authorization_mode: String,
    pub program_hash: String,
    pub semantics: String,
    pub target: Target,
    pub circuits: Vec<CircuitArtifact>,
    pub resources: Resources,
    pub gates: Vec<Gate>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ArtifactFields {
    #[serde(deserialize_with = "decode_artifact_format")]
    format: String,
    #[serde(deserialize_with = "decode_abi_version")]
    abi: String,
    #[serde(deserialize_with = "decode_relation_version")]
    relation: String,
    #[serde(deserialize_with = "decode_authorization_mode")]
    authorization_mode: String,
    program_hash: String,
    semantics: String,
    #[serde(deserialize_with = "decode_artifact_target")]
    target: Target,
    circuits: Vec<CircuitArtifact>,
    resources: Resources,
    gates: Vec<Gate>,
}
impl TryFrom<ArtifactFields> for Artifact {
    type Error = Error;
    fn try_from(f: ArtifactFields) -> Result<Self, Error> {
        if f.authorization_mode != f.target.authorization_mode() {
            return Err(error(
                "artifact",
                "Authorization mode differs from the Target signed_policy; recompile.",
            ));
        }
        authorization::decode_digest(&f.program_hash)?;
        for circuit in &f.circuits {
            authorization::decode_digest(&circuit.context_hash)?;
            signed_relation::validate_shape(
                &f.authorization_mode,
                &circuit.public_inputs,
                &circuit.authorization_bindings,
                Some(&circuit.private_inputs),
            )?;
        }
        Ok(Self {
            format: f.format,
            abi: f.abi,
            relation: f.relation,
            authorization_mode: f.authorization_mode,
            program_hash: f.program_hash,
            semantics: f.semantics,
            target: f.target,
            circuits: f.circuits,
            resources: f.resources,
            gates: f.gates,
        })
    }
}
fn decode_version<'de, D: serde::Deserializer<'de>>(
    d: D,
    expected: &str,
) -> Result<String, D::Error> {
    let value = String::deserialize(d)?;
    if value != expected {
        return Err(serde::de::Error::custom(format!(
            "Expected {expected}; recompile for this backend revision."
        )));
    }
    Ok(value)
}
fn decode_artifact_format<'de, D: serde::Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    decode_version(d, ARTIFACT_FORMAT)
}
fn decode_abi_version<'de, D: serde::Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    decode_version(d, PUBLIC_ABI_VERSION)
}
fn decode_relation_version<'de, D: serde::Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    decode_version(d, RELATION_VERSION)
}
fn decode_authorization_mode<'de, D: serde::Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    let value = String::deserialize(d)?;
    if value != UNSIGNED_MODE && value != authorization::INTENT_PROFILE {
        return Err(serde::de::Error::custom(
            "Unknown authorization mode; recompile for this backend revision.",
        ));
    }
    Ok(value)
}
fn decode_registry_entries<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u32, D::Error> {
    let value = u32::deserialize(d)?;
    if value as usize > MAX_LIABILITY_REGISTRY_ENTRIES {
        return Err(serde::de::Error::custom(
            "Liability registry exceeds the compiler profile.",
        ));
    }
    Ok(value)
}
fn decode_registry_ceiling<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u32, D::Error> {
    let value = u32::deserialize(d)?;
    if value as usize != MAX_LIABILITY_REGISTRY_ENTRIES {
        return Err(serde::de::Error::custom(
            "Stale liability compiler profile.",
        ));
    }
    Ok(value)
}
fn decode_artifact_target<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Target, D::Error> {
    let value = Target::deserialize(d)?;
    value.validate().map_err(serde::de::Error::custom)?;
    Ok(value)
}
fn digest<T: Serialize>(value: &T) -> Result<[u8; 32], Error> {
    let bytes =
        mori_core::intent::canonical(value).map_err(|e| error("serialization", e.to_string()))?;
    Ok(Sha256::digest(&bytes).into())
}
/// Lower a validated program. Financial relations constrain the declared touched
/// balance frame. An authenticated ledger adapter must preserve all accounts
/// outside it and bind the actual predecessor; that adapter is an open gate.
/// This unsigned entry point refuses a signed Target rather than dropping it.
pub fn compile(program: &Program, target: &Target) -> Result<Artifact, Error> {
    stage::require_core1(program)?;
    if target.signed_policy.is_some() {
        return Err(error(
            "target",
            "A signed Target requires the signed compiler; unsigned compilation refuses it.",
        ));
    }
    compile_mode(program, target)
}
/// Lower every action with the strict owner signature and checked caps in the
/// same circuit. Source-wide signed admission precedes any layout allocation.
pub fn compile_signed(program: &Program, target: &Target) -> Result<Artifact, Error> {
    stage::require_core1(program)?;
    if target.signed_policy.is_none() {
        return Err(error(
            "target",
            "Signed compilation requires a Target signed_policy.",
        ));
    }
    compile_mode(program, target)
}
pub(crate) fn compile_mode(program: &Program, target: &Target) -> Result<Artifact, Error> {
    stage::require_core1(program)?;
    target.validate()?;
    mori_core::validate(program).map_err(|e| error("program", e.to_string()))?;
    if program
        .assets
        .iter()
        .any(|asset| asset.identity.network != target.network)
    {
        return Err(error(
            "target",
            "Asset network differs from the target domain; cross-domain assets are unsupported.",
        ));
    }
    // Signed label/owner admission precedes the registry's own refusals.
    let signed = match &target.signed_policy {
        Some(policy) => Some(signed_relation::admit(program, policy)?),
        None => None,
    };
    let registry = liabilities::registry(program)?;
    let hash = digest(program)?;
    let program_hash = mori_core::intent::hex(&hash);
    let mut circuits = Vec::with_capacity(program.actions.len());
    let mut resources = Resources {
        instructions: 0,
        zkir_bytes: 0,
        artifact_bytes: 0,
        public_values: 0,
        max_replay_steps: 0,
        circuit_k: None,
        fraud_max_k: None,
        cost_measured: false,
        liability_registry_entries: registry.len() as u32,
        max_liability_registry_entries: MAX_LIABILITY_REGISTRY_ENTRIES as u32,
    };
    for action in &program.actions {
        let circuit = lower_action(program, action, target, &registry, signed.as_ref())?;
        resources.instructions = resources
            .instructions
            .checked_add(circuit.instruction_count)
            .ok_or_else(|| error("limit", "Instruction count overflow."))?;
        resources.zkir_bytes = resources
            .zkir_bytes
            .checked_add(
                u32::try_from(circuit.zkir.len())
                    .map_err(|_| error("limit", "Artifact too large."))?,
            )
            .ok_or_else(|| error("limit", "Artifact size overflow."))?;
        resources.public_values = resources
            .public_values
            .checked_add(
                (circuit.public_inputs.len() + circuit.authorization_bindings.len()) as u32,
            )
            .ok_or_else(|| error("limit", "Public ABI size overflow."))?;
        resources.max_replay_steps = resources.max_replay_steps.max(circuit.replay_steps);
        if resources.instructions > target.max_instructions
            || resources.zkir_bytes > target.max_artifact_bytes
        {
            return Err(error(
                "limit",
                "Program exceeds target instruction or ZKIR byte limit.",
            ));
        }
        circuits.push(circuit);
    }
    let mut artifact = Artifact {
        format: ARTIFACT_FORMAT.into(),
        abi: PUBLIC_ABI_VERSION.into(),
        relation: RELATION_VERSION.into(),
        authorization_mode: target.authorization_mode().into(),
        program_hash,
        semantics: program.semantics.clone(),
        target: target.clone(),
        circuits,
        resources,
        gates: [
            "native-proof-generation",
            "circuit-cost-and-k-measurement",
            "fraud-proof-cost-and-k-measurement",
            "authenticated-ledger-frame-and-phase-correspondence",
            "signed-intent-and-predecessor-history-verification",
            "live-ledger-deployment",
        ]
        .into_iter()
        .map(|p| Gate {
            predicate: p.into(),
            status: "unperformed".into(),
        })
        .collect(),
    };
    // The measured envelope includes its own decimal byte count. Reach the
    // fixed point (at most ten digit changes for u32), using the CLI's exact
    // pretty JSON serialization. ABI and frame metadata count toward the cap.
    for _ in 0..12 {
        let bytes = serde_json::to_vec_pretty(&artifact)
            .map_err(|e| error("serialization", e.to_string()))?;
        let count = u32::try_from(bytes.len())
            .map_err(|_| error("limit", "Artifact envelope exceeds u32 byte count."))?;
        if count > target.max_artifact_bytes {
            return Err(error(
                "limit",
                "Artifact envelope exceeds target byte limit.",
            ));
        }
        if count == artifact.resources.artifact_bytes {
            return Ok(artifact);
        }
        artifact.resources.artifact_bytes = count;
    }
    Err(error(
        "serialization",
        "Artifact size count did not stabilize.",
    ))
}
#[derive(Clone)]
struct Wire {
    operand: Operand,
    ty: Type,
}
fn immediate(n: u128) -> Operand {
    Operand::Immediate(Fr::from(n))
}
#[derive(Default)]
struct Builder {
    instructions: Vec<Instruction>,
    bindings: Vec<PublicBinding>,
    /// Actual public wire of each financial position, in binding order.
    wires: Vec<Operand>,
    serial: u32,
}
impl Builder {
    fn fresh(&mut self) -> Identifier {
        let id = Identifier(format!("%mori.{}", self.serial));
        self.serial += 1;
        id
    }
    fn unary(&mut self, a: Operand, op: &str) -> Operand {
        let output = self.fresh();
        self.instructions.push(match op {
            "neg" => Instruction::Neg {
                a,
                output: output.clone(),
            },
            "not" => Instruction::Not {
                a,
                output: output.clone(),
            },
            _ => unreachable!(),
        });
        Operand::Variable(output)
    }
    fn binary(&mut self, a: Operand, b: Operand, op: BinaryOp) -> Operand {
        let output = self.fresh();
        self.instructions.push(match op {
            BinaryOp::Add => Instruction::Add {
                a,
                b,
                output: output.clone(),
            },
            BinaryOp::Mul => Instruction::Mul {
                a,
                b,
                output: output.clone(),
            },
            BinaryOp::Eq => Instruction::TestEq {
                a,
                b,
                output: output.clone(),
            },
            BinaryOp::Lt => Instruction::LessThan {
                a,
                b,
                bits: 128,
                output: output.clone(),
            },
            _ => unreachable!(),
        });
        Operand::Variable(output)
    }
    fn range(&mut self, val: Operand, ty: Type) {
        self.instructions.push(match ty {
            Type::UInt => Instruction::ConstrainBits { val, bits: 128 },
            Type::Bool => Instruction::ConstrainToBoolean { val },
        });
    }
    fn equal(&mut self, a: Operand, b: Operand) {
        self.instructions.push(Instruction::ConstrainEq { a, b });
    }
    fn canonical_liability(&mut self, presence: Operand, amount: Operand) {
        let absent = self.unary(presence, "not");
        let absent_amount = self.binary(absent, amount, BinaryOp::Mul);
        self.equal(absent_amount, immediate(0));
    }
    fn split64(&mut self, val: Operand) -> (Operand, Operand) {
        let high = self.fresh();
        let low = self.fresh();
        self.instructions.push(Instruction::DivModPowerOfTwo {
            val,
            bits: 64,
            outputs: vec![high.clone(), low.clone()],
        });
        (Operand::Variable(high), Operand::Variable(low))
    }
    // Limb products are <=128 bits and cross terms <=129 bits, so no
    // intermediate wraps the native field. Reject high limbs of the product.
    fn checked_mul(&mut self, a: Operand, b: Operand) -> Operand {
        let (ah, al) = self.split64(a);
        let (bh, bl) = self.split64(b);
        let low_product = self.binary(al.clone(), bl.clone(), BinaryOp::Mul);
        let (carry, low) = self.split64(low_product);
        let cross1 = self.binary(ah.clone(), bl, BinaryOp::Mul);
        let cross2 = self.binary(al, bh.clone(), BinaryOp::Mul);
        let cross = self.binary(cross1, cross2, BinaryOp::Add);
        let cross = self.binary(cross, carry, BinaryOp::Add);
        let (overflow, middle) = self.split64(cross);
        let high_product = self.binary(ah, bh, BinaryOp::Mul);
        let high = self.binary(high_product, overflow, BinaryOp::Add);
        self.equal(high, immediate(0));
        let shifted = self.binary(middle, immediate(1u128 << 64), BinaryOp::Mul);
        let product = self.binary(shifted, low, BinaryOp::Add);
        self.range(product.clone(), Type::UInt);
        product
    }
    fn arithmetic(&mut self, a: Operand, b: Operand, op: BinaryOp) -> Operand {
        let result = match op {
            BinaryOp::Add => self.binary(a, b, op),
            BinaryOp::Sub => {
                let negative = self.unary(b, "neg");
                self.binary(a, negative, BinaryOp::Add)
            }
            BinaryOp::Mul => return self.checked_mul(a, b),
            _ => unreachable!(),
        };
        // Addition <2^129<field; subtraction underflow wraps near modulus.
        self.range(result.clone(), Type::UInt);
        result
    }
    fn public(
        &mut self,
        role: &str,
        name: &str,
        ty: Type,
        expected: Option<Operand>,
        constant: Option<String>,
        asset: Option<AssetId>,
    ) -> Wire {
        let output = self.fresh();
        self.instructions.push(Instruction::PublicInput {
            guard: None,
            val_t: IrType::Native,
            output: output.clone(),
        });
        let operand = Operand::Variable(output);
        self.range(operand.clone(), ty);
        if let Some(expected) = expected {
            self.equal(operand.clone(), expected);
        }
        self.instructions.push(Instruction::Impact {
            guard: immediate(1),
            inputs: vec![operand.clone()],
        });
        self.wires.push(operand.clone());
        self.bindings.push(PublicBinding {
            role: role.into(),
            name: name.into(),
            ty,
            constant,
            asset,
        });
        Wire { operand, ty }
    }
    fn expression(
        &mut self,
        program: &Program,
        id: ExprId,
        env: &BTreeMap<String, Wire>,
        liabilities: &BTreeMap<String, (Wire, Wire)>,
    ) -> Result<Wire, Error> {
        let mut todo = vec![(id, false)];
        let mut values = BTreeMap::<ExprId, Wire>::new();
        while let Some((id, ready)) = todo.pop() {
            if values.contains_key(&id) {
                continue;
            }
            let expression = program
                .expressions
                .get(id as usize)
                .ok_or_else(|| error("expression", "Invalid expression handle."))?;
            if !ready {
                match expression {
                    Expr::Binary { lhs, rhs, .. } => {
                        todo.push((id, true));
                        todo.push((*rhs, false));
                        todo.push((*lhs, false));
                        continue;
                    }
                    Expr::Not { value } => {
                        todo.push((id, true));
                        todo.push((*value, false));
                        continue;
                    }
                    _ => {}
                }
            }
            let wire = match expression {
                Expr::LiabilityPresent { name } | Expr::LiabilityAmount { name } => {
                    let (presence, amount) = liabilities.get(name).ok_or_else(|| {
                        error("unsupported", format!("Unknown liability read {name:?}."))
                    })?;
                    if matches!(expression, Expr::LiabilityPresent { .. }) {
                        presence.clone()
                    } else {
                        amount.clone()
                    }
                }
                Expr::Literal { value } => match value {
                    Value::UInt(n) => Wire {
                        operand: immediate(n.0),
                        ty: Type::UInt,
                    },
                    Value::Bool(b) => Wire {
                        operand: immediate(u128::from(*b)),
                        ty: Type::Bool,
                    },
                },
                Expr::Name { name } => env
                    .get(name)
                    .cloned()
                    .ok_or_else(|| error("name", format!("Unbound name {name:?}.")))?,
                Expr::Not { value } => Wire {
                    operand: self.unary(values[value].operand.clone(), "not"),
                    ty: Type::Bool,
                },
                Expr::Binary { op, lhs, rhs } => {
                    let a = values[lhs].operand.clone();
                    let b = values[rhs].operand.clone();
                    let operand = match op {
                        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul => self.arithmetic(a, b, *op),
                        BinaryOp::Eq => self.binary(a, b, BinaryOp::Eq),
                        BinaryOp::Ne => {
                            let eq = self.binary(a, b, BinaryOp::Eq);
                            self.unary(eq, "not")
                        }
                        BinaryOp::Lt => self.binary(a, b, BinaryOp::Lt),
                        BinaryOp::Gt => self.binary(b, a, BinaryOp::Lt),
                        BinaryOp::Le => {
                            let gt = self.binary(b, a, BinaryOp::Lt);
                            self.unary(gt, "not")
                        }
                        BinaryOp::Ge => {
                            let lt = self.binary(a, b, BinaryOp::Lt);
                            self.unary(lt, "not")
                        }
                        BinaryOp::And => self.binary(a, b, BinaryOp::Mul),
                        BinaryOp::Or => {
                            let na = self.unary(a, "not");
                            let nb = self.unary(b, "not");
                            let neither = self.binary(na, nb, BinaryOp::Mul);
                            self.unary(neither, "not")
                        }
                    };
                    let ty = if matches!(op, BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul) {
                        Type::UInt
                    } else {
                        Type::Bool
                    };
                    self.range(operand.clone(), ty);
                    Wire { operand, ty }
                }
            };
            values.insert(id, wire);
        }
        values
            .remove(&id)
            .ok_or_else(|| error("expression", "Expression produced no value."))
    }
}
fn lower_action(
    program: &Program,
    action: &Action,
    target: &Target,
    registry: &[LiabilityIdentity],
    signed: Option<&signed_relation::Layout>,
) -> Result<CircuitArtifact, Error> {
    lower_action_segment(program, action, target, registry, signed, None)
        .map(|(circuit, _)| circuit)
}

// The full-range legacy path and every continuation segment use the same
// expression and financial statement lowerer. None preserves the /3 layout.
fn lower_action_segment(
    program: &Program,
    action: &Action,
    target: &Target,
    registry: &[LiabilityIdentity],
    signed: Option<&signed_relation::Layout>,
    stage: Option<(
        &mori_core::stage::StagedProgram,
        &stage::frame::SegmentLayout,
    )>,
) -> Result<(CircuitArtifact, Option<stage::PendingPositions>), Error> {
    let mut builder = Builder::default();
    // Stage positions 0/1 with placeholders. Their constants are the hash of
    // the frozen financial layout, inserted only after that layout exists.
    let mut context_checks = Vec::with_capacity(2);
    for i in 0..2 {
        builder.public(
            "context",
            &format!("program_target_action.{i}"),
            Type::UInt,
            Some(immediate(0)),
            None,
            None,
        );
        context_checks.push(builder.instructions.len() - 2);
    }
    let mut env = BTreeMap::new();
    for state in &program.states {
        env.insert(
            state.name.clone(),
            builder.public("pre_state", &state.name, state.ty, None, None, None),
        );
    }
    if stage.is_none_or(|(_, layout)| layout.entry_site == 0) {
        for input in &action.inputs {
            env.insert(
                input.name.clone(),
                builder.public("input", &input.name, input.ty, None, None, None),
            );
        }
    }
    let mut keys = BTreeSet::new();
    for statement in &action.statements {
        match statement {
            Statement::Transfer {
                asset, from, to, ..
            }
            | Statement::Fee {
                asset, from, to, ..
            } => {
                keys.insert((asset.clone(), from.clone()));
                keys.insert((asset.clone(), to.clone()));
            }
            _ => {}
        }
    }
    let assets: BTreeMap<_, _> = program
        .assets
        .iter()
        .map(|a| (a.name.clone(), a.identity.clone()))
        .collect();
    let mut balances = BTreeMap::new();
    let mut balance_frame = Vec::new();
    for (asset, account) in &keys {
        let identity = assets[asset].clone();
        let pre_position = builder.bindings.len() as u32;
        let key = serde_json::to_string(&(asset, account))
            .map_err(|e| error("serialization", e.to_string()))?;
        let wire = builder.public(
            "pre_balance",
            &key,
            Type::UInt,
            None,
            None,
            Some(identity.clone()),
        );
        balances.insert((asset.clone(), account.clone()), wire);
        balance_frame.push(BalanceDescriptor {
            asset: identity,
            account: account.clone(),
            pre_position,
            post_position: 0,
        });
    }
    let mut liabilities = BTreeMap::new();
    let mut liability_frame = Vec::with_capacity(registry.len());
    for identity in registry {
        let pre_presence_position = builder.bindings.len() as u32;
        let presence = builder.public(
            "pre_liability_presence",
            &identity.name,
            Type::Bool,
            None,
            None,
            Some(identity.asset.clone()),
        );
        let pre_amount_position = builder.bindings.len() as u32;
        let amount = builder.public(
            "pre_liability_amount",
            &identity.name,
            Type::UInt,
            None,
            None,
            Some(identity.asset.clone()),
        );
        builder.canonical_liability(presence.operand.clone(), amount.operand.clone());
        liabilities.insert(identity.name.clone(), (presence, amount));
        liability_frame.push(LiabilityDescriptor {
            name: identity.name.clone(),
            asset: identity.asset.clone(),
            debtor: identity.debtor.clone(),
            creditor: identity.creditor.clone(),
            pre_presence_position,
            pre_amount_position,
            post_presence_position: 0,
            post_amount_position: 0,
        });
    }
    let pre_frame = match stage {
        Some((_, layout)) => stage::frame::emit_pre(&mut builder, layout)?,
        None => None,
    };
    if let Some(frame) = &pre_frame {
        for (name, wire) in frame.captures.iter().chain(&frame.events) {
            env.insert(name.clone(), wire.clone());
        }
    }
    if let Some((_, layout)) = stage
        && let Some(guard) = layout.guard
    {
        let wire = builder.expression(program, guard, &env, &liabilities)?;
        stage::frame::assert_guard(&mut builder, wire)?;
    }
    let statements = stage.map_or(0..action.statements.len(), |(_, layout)| {
        layout.statements.clone()
    });
    let mut effect_frame = Vec::new();
    for (step, statement) in action
        .statements
        .iter()
        .enumerate()
        .skip(statements.start)
        .take(statements.end - statements.start)
    {
        let mut effect = EffectDescriptor {
            step: step as u32,
            kind: String::new(),
            name: None,
            asset: None,
            from: None,
            to: None,
            public_positions: Vec::new(),
            liability: None,
            operation: None,
        };
        match statement {
            Statement::Await { .. } => return Err(stage::core1_refusal(program)),
            Statement::Let { name, value } => {
                let value = builder.expression(program, *value, &env, &liabilities)?;
                env.insert(name.clone(), value);
            }
            Statement::Requires { predicate } | Statement::Ensures { predicate } => {
                let predicate = builder.expression(program, *predicate, &env, &liabilities)?;
                builder.instructions.push(Instruction::Assert {
                    cond: predicate.operand,
                });
            }
            Statement::Next { state, value } => {
                let before = env[state].clone();
                let value = builder.expression(program, *value, &env, &liabilities)?;
                effect.kind = "state".into();
                effect.name = Some(state.clone());
                for (field, wire) in [("before", before), ("after", value.clone())] {
                    effect.public_positions.push(builder.bindings.len() as u32);
                    builder.public(
                        "effect",
                        &format!("{step}.{field}"),
                        wire.ty,
                        Some(wire.operand),
                        None,
                        None,
                    );
                }
                env.insert(state.clone(), value);
            }
            Statement::Emit { name, value } => {
                let wire = builder.expression(program, *value, &env, &liabilities)?;
                effect.kind = "emit".into();
                effect.name = Some(name.clone());
                effect.public_positions.push(builder.bindings.len() as u32);
                builder.public(
                    "effect",
                    &format!("{step}.value"),
                    wire.ty,
                    Some(wire.operand),
                    None,
                    None,
                );
            }
            Statement::Transfer {
                asset,
                from,
                to,
                amount,
            }
            | Statement::Fee {
                asset,
                from,
                to,
                amount,
            } => {
                let amount = builder
                    .expression(program, *amount, &env, &liabilities)?
                    .operand;
                let from_key = (asset.clone(), from.clone());
                let to_key = (asset.clone(), to.clone());
                let debited = builder.arithmetic(
                    balances[&from_key].operand.clone(),
                    amount.clone(),
                    BinaryOp::Sub,
                );
                balances.insert(
                    from_key,
                    Wire {
                        operand: debited,
                        ty: Type::UInt,
                    },
                );
                let credited = builder.arithmetic(
                    balances[&to_key].operand.clone(),
                    amount.clone(),
                    BinaryOp::Add,
                );
                balances.insert(
                    to_key,
                    Wire {
                        operand: credited,
                        ty: Type::UInt,
                    },
                );
                effect.kind = if matches!(statement, Statement::Fee { .. }) {
                    "fee"
                } else {
                    "transfer"
                }
                .into();
                effect.asset = Some(assets[asset].clone());
                effect.from = Some(from.clone());
                effect.to = Some(to.clone());
                effect.public_positions.push(builder.bindings.len() as u32);
                builder.public(
                    "effect",
                    &format!("{step}.amount"),
                    Type::UInt,
                    Some(amount),
                    None,
                    effect.asset.clone(),
                );
            }
            Statement::Liability { name, amount, .. } | Statement::Discharge { name, amount } => {
                let delta = builder.expression(program, *amount, &env, &liabilities)?;
                builder.range(delta.operand.clone(), Type::UInt);
                let (before_presence, before_amount) = liabilities[name].clone();
                let discharge = matches!(statement, Statement::Discharge { .. });
                if discharge {
                    builder.instructions.push(Instruction::Assert {
                        cond: before_presence.operand.clone(),
                    });
                }
                let after_amount = Wire {
                    operand: builder.arithmetic(
                        before_amount.operand.clone(),
                        delta.operand,
                        if discharge {
                            BinaryOp::Sub
                        } else {
                            BinaryOp::Add
                        },
                    ),
                    ty: Type::UInt,
                };
                let after_presence = if discharge {
                    before_presence.clone()
                } else {
                    Wire {
                        operand: immediate(1),
                        ty: Type::Bool,
                    }
                };
                let identity = registry
                    .iter()
                    .find(|entry| &entry.name == name)
                    .expect("closed liability registry")
                    .clone();
                effect.kind = "liability".into();
                effect.liability = Some(identity.clone());
                effect.operation = Some(if discharge {
                    LiabilityOperation::Discharge
                } else {
                    LiabilityOperation::CreateOrAccrue
                });
                for (field, wire) in [
                    ("before_presence", before_presence),
                    ("before_amount", before_amount),
                    ("after_presence", after_presence.clone()),
                    ("after_amount", after_amount.clone()),
                ] {
                    effect.public_positions.push(builder.bindings.len() as u32);
                    builder.public(
                        "effect",
                        &format!("{step}.{field}"),
                        wire.ty,
                        Some(wire.operand),
                        None,
                        Some(identity.asset.clone()),
                    );
                }
                liabilities.insert(name.clone(), (after_presence, after_amount));
            }
        }
        if !effect.kind.is_empty() {
            effect_frame.push(effect);
        }
        if builder.instructions.len() > target.max_instructions as usize {
            return Err(error("limit", "Action exceeds target instruction limit."));
        }
    }
    let mut outputs = Vec::new();
    for state in &program.states {
        let wire = builder.public(
            "post_state",
            &state.name,
            state.ty,
            Some(env[&state.name].operand.clone()),
            None,
            None,
        );
        outputs.push(wire.operand);
    }
    for ((asset, account), frame) in keys.iter().zip(balance_frame.iter_mut()) {
        frame.post_position = builder.bindings.len() as u32;
        let key = serde_json::to_string(&(asset, account))
            .map_err(|e| error("serialization", e.to_string()))?;
        let wire = builder.public(
            "post_balance",
            &key,
            Type::UInt,
            Some(balances[&(asset.clone(), account.clone())].operand.clone()),
            None,
            Some(assets[asset].clone()),
        );
        outputs.push(wire.operand);
    }
    for frame in &mut liability_frame {
        let (presence, amount) = &liabilities[&frame.name];
        frame.post_presence_position = builder.bindings.len() as u32;
        let presence = builder.public(
            "post_liability_presence",
            &frame.name,
            Type::Bool,
            Some(presence.operand.clone()),
            None,
            Some(frame.asset.clone()),
        );
        frame.post_amount_position = builder.bindings.len() as u32;
        let amount = builder.public(
            "post_liability_amount",
            &frame.name,
            Type::UInt,
            Some(amount.operand.clone()),
            None,
            Some(frame.asset.clone()),
        );
        builder.canonical_liability(presence.operand.clone(), amount.operand.clone());
        outputs.extend([presence.operand, amount.operand]);
    }
    let pending_positions = match stage {
        Some((_, layout)) => {
            let live = layout
                .slots
                .iter()
                .take(layout.len_out as usize)
                .map(|slot| {
                    env.get(&slot.name)
                        .cloned()
                        .map(|wire| (slot.name.as_str(), wire))
                        .ok_or_else(|| error("name", format!("Unbound capture {:?}.", slot.name)))
                })
                .collect::<Result<Vec<_>, _>>()?;
            stage::frame::emit_post(&mut builder, layout, pre_frame.as_ref(), &live)?
        }
        None => None,
    };
    let replay_steps = stage.map_or(action.statements.len() as u32, |(_, layout)| {
        layout.replay_steps
    });
    let context = match stage {
        Some((sp, layout)) => stage::context::stage_context_hash(
            sp,
            target,
            registry,
            layout,
            &builder.bindings,
            &effect_frame,
            &balance_frame,
            &liability_frame,
        )?,
        None => signed_relation::context_hash(
            target,
            registry,
            program,
            &action.name,
            &builder.bindings,
            &effect_frame,
            &balance_frame,
            &liability_frame,
            replay_steps,
        )?,
    };
    for (i, index) in context_checks.into_iter().enumerate() {
        let value = authorization::digest_limbs(context)[i].0;
        builder.instructions[index] = Instruction::ConstrainEq {
            a: builder.wires[i].clone(),
            b: immediate(value),
        };
        builder.bindings[i].constant = Some(value.to_string());
    }
    let (authorization_bindings, private_inputs) = match signed {
        Some(layout) => signed_relation::append(
            &mut builder,
            layout,
            context,
            &effect_frame,
            &liability_frame,
        )?,
        None => (Vec::new(), Vec::new()),
    };
    if builder.instructions.len() > target.max_instructions as usize {
        return Err(error("limit", "Action exceeds target instruction limit."));
    }
    let output_types = vec![IrType::Native; outputs.len()];
    builder
        .instructions
        .push(Instruction::Output { vals: outputs });
    let instruction_count = u32::try_from(builder.instructions.len())
        .map_err(|_| error("limit", "Instruction count overflow."))?;
    let ir = IrSource {
        version: Default::default(),
        inputs: Vec::new(),
        outputs: output_types,
        do_communications_commitment: false,
        instructions: Arc::new(builder.instructions),
    };
    let mut disk = serde_json::to_value(&ir).map_err(|e| error("serialization", e.to_string()))?;
    disk["version"] = serde_json::json!({"major":3,"minor":0});
    let zkir = serde_json::to_string(&disk).map_err(|e| error("serialization", e.to_string()))?;
    let parsed = IrSource::load(zkir.as_bytes()).map_err(|e| error("zkir", e.to_string()))?;
    if parsed != ir {
        return Err(error("zkir", "Pinned upstream round-trip changed the IR."));
    }
    Ok((
        CircuitArtifact {
            action: action.name.clone(),
            zkir,
            context_hash: proving::hex(&context),
            public_inputs: builder.bindings,
            authorization_bindings,
            private_inputs,
            instruction_count,
            effect_frame,
            balance_frame,
            liability_frame,
            replay_steps,
        },
        pending_positions,
    ))
}
