//! Portable bounded transition relations for the pinned ledger-9 ZKIRv3.
//! Compilation emits a relation and public ABI without a scenario or success
//! flag. Explicit offline native proofs are available for the admitted public
//! frame; authenticated ledger correspondence remains an open gate.
use midnight_transient_crypto::curve::Fr;
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::{Action, AssetId, BinaryOp, Expr, ExprId, Program, Statement, Type, Value};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

mod parameters;
pub mod proving;

pub const LEDGER_REVISION: &str = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14";
pub const ZKIR_VERSION: &str = "3.0.0-rc.2";
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
        }
    }
    pub fn from_json(text: &str) -> Result<Self, Error> {
        serde_json::from_str(text).map_err(|e| error("target", e.to_string()))
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
        Ok(())
    }
}
/// Positional ABI: every entry is read from public_transcript_outputs and
/// explicitly published by Impact in the same position in transcript_inputs.
/// Upstream PublicInput alone assigns a witness; it does not bind a public value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicBinding {
    pub role: String,
    pub name: String,
    pub ty: Type,
    pub constant: Option<String>,
    pub asset: Option<AssetId>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitArtifact {
    pub action: String,
    pub zkir: String,
    pub public_inputs: Vec<PublicBinding>,
    pub instruction_count: u32,
    pub effect_frame: Vec<EffectDescriptor>,
    pub balance_frame: Vec<BalanceDescriptor>,
    pub replay_steps: u32,
}
/// Static effect identities are committed through program/action context.
/// Every effect amount/value is separately public and constrained.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectDescriptor {
    pub step: u32,
    pub kind: String,
    pub name: Option<String>,
    pub asset: Option<AssetId>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub public_positions: Vec<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceDescriptor {
    pub asset: AssetId,
    pub account: String,
    pub pre_position: u32,
    pub post_position: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    pub instructions: u32,
    pub zkir_bytes: u32,
    pub artifact_bytes: u32,
    pub public_values: u32,
    pub max_replay_steps: u32,
    pub circuit_k: Option<u8>,
    pub fraud_max_k: Option<u8>,
    pub cost_measured: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gate {
    pub predicate: String,
    pub status: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub format: String,
    pub program_hash: String,
    pub semantics: String,
    pub target: Target,
    pub circuits: Vec<CircuitArtifact>,
    pub resources: Resources,
    pub gates: Vec<Gate>,
}
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[error("{code}: {message}")]
pub struct Error {
    pub code: String,
    pub message: String,
}
fn error(code: &str, message: impl Into<String>) -> Error {
    Error {
        code: format!("mori::backend::{code}"),
        message: message.into(),
    }
}
fn digest<T: Serialize>(value: &T) -> Result<[u8; 32], Error> {
    let bytes =
        mori_core::intent::canonical(value).map_err(|e| error("serialization", e.to_string()))?;
    Ok(Sha256::digest(&bytes).into())
}
/// Lower a validated program. Financial relations constrain the declared touched
/// balance frame. An authenticated ledger adapter must preserve all accounts
/// outside it and bind the actual predecessor; that adapter is an open gate.
pub fn compile(program: &Program, target: &Target) -> Result<Artifact, Error> {
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
    let hash = digest(program)?;
    let program_hash = hash.iter().map(|b| format!("{b:02x}")).collect();
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
    };
    for action in &program.actions {
        let circuit = lower_action(program, action, target)?;
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
            .checked_add(circuit.public_inputs.len() as u32)
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
        format: "moriarty-zkir-artifact/1".into(),
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
) -> Result<CircuitArtifact, Error> {
    let mut builder = Builder::default();
    let context = digest(&("moriarty-zkir-relation/1", program, target, &action.name))?;
    for (i, chunk) in context.chunks_exact(16).enumerate() {
        let value = u128::from_le_bytes(chunk.try_into().expect("16-byte digest limb"));
        builder.public(
            "context",
            &format!("program_target_action.{i}"),
            Type::UInt,
            Some(immediate(value)),
            Some(value.to_string()),
            None,
        );
    }
    let mut env = BTreeMap::new();
    for state in &program.states {
        env.insert(
            state.name.clone(),
            builder.public("pre_state", &state.name, state.ty, None, None, None),
        );
    }
    for input in &action.inputs {
        env.insert(
            input.name.clone(),
            builder.public("input", &input.name, input.ty, None, None, None),
        );
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
            Statement::Liability { .. } | Statement::Discharge { .. } => {
                return Err(error(
                    "unsupported",
                    "Liability presence and typed predecessor identities are not yet lowered; compile refuses this action.",
                ));
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
    let mut effect_frame = Vec::new();
    for (step, statement) in action.statements.iter().enumerate() {
        let mut effect = EffectDescriptor {
            step: step as u32,
            kind: String::new(),
            name: None,
            asset: None,
            from: None,
            to: None,
            public_positions: Vec::new(),
        };
        match statement {
            Statement::Let { name, value } => {
                let value = builder.expression(program, *value, &env)?;
                env.insert(name.clone(), value);
            }
            Statement::Requires { predicate } | Statement::Ensures { predicate } => {
                let predicate = builder.expression(program, *predicate, &env)?;
                builder.instructions.push(Instruction::Assert {
                    cond: predicate.operand,
                });
            }
            Statement::Next { state, value } => {
                let before = env[state].clone();
                let value = builder.expression(program, *value, &env)?;
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
                let wire = builder.expression(program, *value, &env)?;
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
                let amount = builder.expression(program, *amount, &env)?.operand;
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
            Statement::Liability { .. } | Statement::Discharge { .. } => {
                unreachable!("unsupported operations rejected before lowering")
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
    Ok(CircuitArtifact {
        action: action.name.clone(),
        zkir,
        public_inputs: builder.bindings,
        instruction_count,
        effect_frame,
        balance_frame,
        replay_steps: action.statements.len() as u32,
    })
}
