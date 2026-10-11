//! Strict owner-intent statement compiled into the financial transition
//! circuit, plus the shared host reconstruction used by prepare-intent,
//! verify-intent, prove and verify. SHA256/JSON values are host commitments;
//! the circuit binds their reconstructed limbs, not their computation.
use crate::authorization::{
    self, AuthorizationBinding, AuthorizationType, AuthorizationValue, ExpectedAuthority,
    FinancialValue, INTENT_PROFILE, MAX_SIGNED_ASSETS, PREPARED_PROFILE, PreparedIntent,
    SignedIntent, SignedPolicy, UnsignedPayload,
};
use crate::{
    ARTIFACT_FORMAT, BalanceDescriptor, Builder, CircuitArtifact, EffectDescriptor, Error,
    LiabilityDescriptor, LiabilityIdentity, PUBLIC_ABI_VERSION, PrivateBinding, PrivateType,
    PublicBinding, RELATION_VERSION, Target, digest, error, immediate, proving, schnorr,
};
use midnight_transient_crypto::{curve::Fr, hash::transient_hash, proofs::ProofPreimage};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Instruction, IrSource};
use mori_core::{AssetId, BinaryOp, Effect, Execution, Program, Scenario, Statement, Type, UInt};
use serde::Serialize;
use sha2::{Digest, Sha256};

/// MOI1 message domain (little-endian bytes `4d4f4931`) and profile version.
pub const MESSAGE_DOMAIN: u64 = 0x31494f4d;
const MESSAGE_VERSION: u64 = 1;
const CONTEXT_FORMAT: &str = "moriarty-financial-context/3";

/// Source-fixed signed layout, derived after source-wide admission.
pub(crate) struct Layout {
    owner: String,
    beneficiary: String,
    /// Strictly ascending full AssetIds with their SHA256 limbs.
    assets: Vec<(AssetId, [u128; 2])>,
}

/// Source-wide single-owner admission, before any proportional allocation.
/// Every action counts, including actions that a scenario never selects.
pub(crate) fn admit(
    program: &Program,
    policy: &SignedPolicy,
    registry: &[LiabilityIdentity],
) -> Result<Layout, Error> {
    if program.assets.len() > MAX_SIGNED_ASSETS {
        return Err(error(
            "limit",
            "Signed profile admits at most 16 declared assets.",
        ));
    }
    let refuse = |message: String| error("signed", message);
    let mut assets = Vec::with_capacity(program.assets.len());
    for decl in &program.assets {
        authorization::validate_label(&decl.name)?;
        authorization::validate_asset(&decl.identity)?;
        assets.push(decl.identity.clone());
    }
    assets.sort();
    if assets.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(refuse(
            "Signed profile refuses source asset aliases with one full AssetId.".into(),
        ));
    }
    for action in &program.actions {
        authorization::validate_label(&action.name)?;
        for statement in &action.statements {
            match statement {
                Statement::Transfer { from, to, .. } | Statement::Fee { from, to, .. } => {
                    authorization::validate_label(from)?;
                    authorization::validate_label(to)?;
                    if *from != policy.owner {
                        return Err(refuse(format!(
                            "Action {:?} pays from {from:?}; the signed profile admits only owner {:?}.",
                            action.name, policy.owner
                        )));
                    }
                }
                Statement::Liability {
                    debtor, creditor, ..
                } => {
                    authorization::validate_label(debtor)?;
                    authorization::validate_label(creditor)?;
                    if *debtor != policy.owner {
                        return Err(refuse(format!(
                            "Action {:?} creates a duty owed by {debtor:?}; the signed profile admits only owner {:?}.",
                            action.name, policy.owner
                        )));
                    }
                }
                _ => {}
            }
        }
    }
    // Discharges name registry entries; each registered debtor is checked here.
    if let Some(other) = registry.iter().find(|r| r.debtor != policy.owner) {
        return Err(refuse(format!(
            "Registered duty {:?} has a debtor other than the owner.",
            other.name
        )));
    }
    let assets = assets
        .into_iter()
        .map(|asset| {
            let limbs = authorization::digest_limbs(digest(&asset)?);
            Ok((asset, [limbs[0].0, limbs[1].0]))
        })
        .collect::<Result<_, Error>>()?;
    Ok(Layout {
        owner: policy.owner.clone(),
        beneficiary: policy.beneficiary.clone(),
        assets,
    })
}

#[derive(Serialize)]
struct Projection<'a> {
    position: u32,
    role: &'a str,
    name: &'a str,
    ty: Type,
    asset: &'a Option<AssetId>,
}

/// SHA256 of the canonical 18-element context array. The projection excludes
/// every constant, so the two context constants never hash themselves.
#[allow(clippy::too_many_arguments)]
pub(crate) fn context_hash(
    target: &Target,
    registry: &[LiabilityIdentity],
    program: &Program,
    action: &str,
    bindings: &[PublicBinding],
    effect_frame: &[EffectDescriptor],
    balance_frame: &[BalanceDescriptor],
    liability_frame: &[LiabilityDescriptor],
    replay_steps: u32,
) -> Result<[u8; 32], Error> {
    let projection: Vec<_> = bindings
        .iter()
        .enumerate()
        .map(|(position, b)| Projection {
            position: position as u32,
            role: &b.role,
            name: &b.name,
            ty: b.ty,
            asset: &b.asset,
        })
        .collect();
    digest(&[
        json(&CONTEXT_FORMAT)?,
        json(&target.authorization_mode())?,
        json(&ARTIFACT_FORMAT)?,
        json(&PUBLIC_ABI_VERSION)?,
        json(&RELATION_VERSION)?,
        json(&proving::BINDING_VERSION)?,
        json(&proving::PROOF_FORMAT)?,
        json(&MAX_SIGNED_ASSETS)?,
        json(&crate::MAX_LIABILITY_REGISTRY_ENTRIES)?,
        json(&registry)?,
        json(program)?,
        json(target)?,
        json(&action)?,
        json(&projection)?,
        json(&effect_frame)?,
        json(&balance_frame)?,
        json(&liability_frame)?,
        json(&replay_steps)?,
    ])
}
fn json<T: Serialize + ?Sized>(value: &T) -> Result<serde_json::Value, Error> {
    serde_json::to_value(value).map_err(|e| error("serialization", e.to_string()))
}

fn authorization_input(
    b: &mut Builder,
    bindings: &mut Vec<AuthorizationBinding>,
    name: String,
    ty: AuthorizationType,
    asset: Option<AssetId>,
) -> Operand {
    let output = b.fresh();
    b.instructions.push(Instruction::PublicInput {
        guard: None,
        val_t: IrType::Native,
        output: output.clone(),
    });
    let operand = Operand::Variable(output);
    if ty == AuthorizationType::UInt {
        b.range(operand.clone(), Type::UInt);
    }
    b.instructions.push(Instruction::Impact {
        guard: immediate(1),
        inputs: vec![operand.clone()],
    });
    bindings.push(AuthorizationBinding {
        position: (b.bindings.len() + bindings.len()) as u32,
        role: "authorization".into(),
        name,
        ty,
        constant: None,
        asset,
    });
    operand
}
fn private(b: &mut Builder, val_t: IrType) -> Operand {
    let output = b.fresh();
    b.instructions.push(Instruction::PrivateInput {
        guard: None,
        val_t,
        output: output.clone(),
    });
    Operand::Variable(output)
}
fn coordinates(b: &mut Builder, point: Operand) -> (Operand, Operand) {
    let (x, y) = (b.fresh(), b.fresh());
    b.instructions.push(Instruction::IntoCoordinates {
        point,
        outputs: (x.clone(), y.clone()),
    });
    (Operand::Variable(x), Operand::Variable(y))
}
fn hash(b: &mut Builder, inputs: Vec<Operand>) -> Operand {
    let output = b.fresh();
    b.instructions.push(Instruction::TransientHash {
        inputs,
        output: output.clone(),
    });
    Operand::Variable(output)
}
fn scalar(b: &mut Builder, native: Operand) -> Operand {
    let output = b.fresh();
    b.instructions.push(Instruction::JubjubScalarFromNative {
        native,
        output: output.clone(),
    });
    Operand::Variable(output)
}
fn assert_true(b: &mut Builder, cond: Operand) {
    b.instructions.push(Instruction::Assert { cond });
}
fn assert_not(b: &mut Builder, cond: Operand) {
    let negated = b.unary(cond, "not");
    assert_true(b, negated);
}
/// Typed subgroup points are nonidentity exactly when x != 0: the only
/// other x=0 curve point, (0,-1), has order two and is outside the subgroup.
fn assert_nonidentity(b: &mut Builder, x: Operand) {
    let zero = b.binary(x, immediate(0), BinaryOp::Eq);
    assert_not(b, zero);
}
fn assert_le(b: &mut Builder, small: Operand, large: Operand) {
    let exceeded = b.binary(large, small, BinaryOp::Lt);
    assert_not(b, exceeded);
}
fn checked_sum(b: &mut Builder, wires: impl IntoIterator<Item = Operand>) -> Operand {
    wires
        .into_iter()
        .reduce(|sum, wire| b.arithmetic(sum, wire, BinaryOp::Add))
        .unwrap_or_else(|| immediate(0))
}

/// Append the N+11+4A authorization inputs, the five typed private inputs, the
/// MOI1 message over the actual financial wires, strict FENS and the checked
/// caps over actual ordered effect and closing-duty wires.
pub(crate) fn append(
    b: &mut Builder,
    layout: &Layout,
    context: [u8; 32],
    effects: &[EffectDescriptor],
    duties: &[LiabilityDescriptor],
) -> Result<(Vec<AuthorizationBinding>, Vec<PrivateBinding>), Error> {
    use AuthorizationType::{Native, UInt as Ranged};
    let n = b.bindings.len();
    let mut auth = Vec::with_capacity(11 + 4 * layout.assets.len());
    let mut input = |b: &mut Builder, name: &str, ty, asset: Option<&AssetId>| {
        authorization_input(b, &mut auth, name.into(), ty, asset.cloned())
    };
    let payload = [
        input(b, "payload.lo", Ranged, None),
        input(b, "payload.hi", Ranged, None),
    ];
    let key_x = input(b, "public_key.x", Native, None);
    let key_y = input(b, "public_key.y", Native, None);
    let message = input(b, "message", Native, None);
    let nonce = input(b, "nonce", Ranged, None);
    let not_before = input(b, "not_before", Ranged, None);
    let not_after = input(b, "not_after", Ranged, None);
    let predecessor = [
        input(b, "predecessor.lo", Ranged, None),
        input(b, "predecessor.hi", Ranged, None),
    ];
    let at = input(b, "at", Ranged, None);
    let mut caps = Vec::with_capacity(layout.assets.len());
    for (i, (asset, _)) in layout.assets.iter().enumerate() {
        caps.push(
            ["gross_cap", "fee_cap", "net_floor", "liability_cap"]
                .map(|field| input(b, &format!("limits.{i}.{field}"), Ranged, Some(asset))),
        );
    }
    let key = private(b, IrType::JubjubPoint);
    let r = private(b, IrType::JubjubPoint);
    let response = private(b, IrType::Native);

    // The actual typed key coordinates are the public key coordinates.
    let (px, py) = coordinates(b, key.clone());
    b.equal(px.clone(), key_x);
    b.equal(py.clone(), key_y);
    assert_nonidentity(b, px.clone());

    let limbs = authorization::digest_limbs(context);
    let mut preimage = vec![
        immediate(MESSAGE_DOMAIN.into()),
        immediate(MESSAGE_VERSION.into()),
        immediate(limbs[0].0),
        immediate(limbs[1].0),
        payload[0].clone(),
        payload[1].clone(),
        immediate(n as u128),
    ];
    for (binding, wire) in b.bindings.iter().zip(&b.wires) {
        preimage.push(immediate(tag(binding.ty)));
        preimage.push(wire.clone());
    }
    preimage.push(immediate(layout.assets.len() as u128));
    for ((_, asset_limbs), cap) in layout.assets.iter().zip(&caps) {
        preimage.extend([immediate(asset_limbs[0]), immediate(asset_limbs[1])]);
        preimage.extend(cap.iter().cloned());
    }
    preimage.extend([
        nonce,
        not_before.clone(),
        not_after.clone(),
        predecessor[0].clone(),
        predecessor[1].clone(),
    ]);
    let computed = hash(b, preimage);
    b.equal(computed.clone(), message);

    let (rx, ry) = coordinates(b, r.clone());
    assert_nonidentity(b, rx.clone());
    let challenge = hash(
        b,
        vec![
            immediate(schnorr::FENS_DOMAIN.into()),
            rx,
            ry,
            px,
            py,
            computed,
        ],
    );
    let e = scalar(b, challenge);
    // Canonical response: native -> Jubjub scalar -> native must roundtrip.
    let s = scalar(b, response.clone());
    let roundtrip = b.fresh();
    b.instructions.push(Instruction::Encode {
        input: s.clone(),
        outputs: vec![roundtrip.clone()],
    });
    b.equal(Operand::Variable(roundtrip), response);
    let (lhs, e_key, rhs) = (b.fresh(), b.fresh(), b.fresh());
    b.instructions.push(Instruction::EcMulGenerator {
        scalar: s,
        output: lhs.clone(),
    });
    b.instructions.push(Instruction::EcMul {
        a: key,
        scalar: e,
        output: e_key.clone(),
    });
    b.instructions.push(Instruction::Add {
        a: r,
        b: Operand::Variable(e_key),
        output: rhs.clone(),
    });
    let fens = b.binary(
        Operand::Variable(lhs),
        Operand::Variable(rhs),
        BinaryOp::Eq,
    );
    assert_true(b, fens);

    // Supplied logical time is public, interval-checked and outside the message.
    assert_le(b, not_before, at.clone());
    assert_le(b, at, not_after);

    for ((asset, _), cap) in layout.assets.iter().zip(caps) {
        let wire = |b: &Builder, position: u32| b.wires[position as usize].clone();
        let mut outgoing = Vec::new();
        let mut fees = Vec::new();
        let mut credit = Vec::new();
        let mut debit = Vec::new();
        for effect in effects {
            if effect.asset.as_ref() != Some(asset) {
                continue;
            }
            let amount = wire(b, effect.public_positions[0]);
            let from = effect.from.as_deref();
            match effect.kind.as_str() {
                "transfer" => {
                    if from == Some(&layout.owner) {
                        outgoing.push(amount.clone());
                    }
                    if effect.to.as_deref() == Some(&layout.beneficiary) {
                        credit.push(amount.clone());
                    }
                    if from == Some(&layout.beneficiary) {
                        debit.push(amount);
                    }
                }
                "fee" => {
                    if from == Some(&layout.owner) {
                        fees.push(amount.clone());
                    }
                    if layout.beneficiary != layout.owner && from == Some(&layout.beneficiary) {
                        debit.push(amount);
                    }
                }
                _ => {}
            }
        }
        // Complete nonnegative sums first; refunds never reduce gross.
        let owner_fees = checked_sum(b, fees);
        let transfers = checked_sum(b, outgoing);
        let gross = b.arithmetic(transfers, owner_fees.clone(), BinaryOp::Add);
        let credit = checked_sum(b, credit);
        let debit = checked_sum(b, debit);
        let net = b.arithmetic(credit, debit, BinaryOp::Sub);
        let net = b.arithmetic(net, owner_fees.clone(), BinaryOp::Sub);
        // Every registered closing owner duty for this asset, including untouched
        // and present-zero entries; canonical absence already forces amount 0.
        let closing: Vec<_> = duties
            .iter()
            .filter(|d| &d.asset == asset && d.debtor == layout.owner)
            .map(|d| wire(b, d.post_amount_position))
            .collect();
        let duty = checked_sum(b, closing);
        let [gross_cap, fee_cap, net_floor, liability_cap] = cap;
        assert_le(b, gross, gross_cap);
        assert_le(b, owner_fees, fee_cap);
        assert_le(b, net_floor, net);
        assert_le(b, duty, liability_cap);
    }
    let private_inputs = vec![
        PrivateBinding {
            name: "public_key".into(),
            ty: PrivateType::JubjubPoint,
        },
        PrivateBinding {
            name: "R".into(),
            ty: PrivateType::JubjubPoint,
        },
        PrivateBinding {
            name: "response".into(),
            ty: PrivateType::Native,
        },
    ];
    Ok((auth, private_inputs))
}
fn tag(ty: Type) -> u128 {
    match ty {
        Type::Bool => 1,
        Type::UInt => 2,
    }
}

/// Host-reconstructed signed statement for one trusted authority and time.
pub(crate) struct SignedStatement {
    pub payload: UnsignedPayload,
    pub payload_hash: [u8; 32],
    pub message: Fr,
    /// Public authorization values in positions N..N+11+4A.
    pub values: Vec<Fr>,
    pub at: UInt,
}
/// Shared reconstruction: one compile, one simulation and one framing for
/// every signed and unsigned consumer.
pub(crate) struct Reconstruction {
    pub source_hash: String,
    pub program_hash: String,
    pub circuit: CircuitArtifact,
    pub registry: Vec<LiabilityIdentity>,
    pub execution: Execution,
    pub financial: Vec<mori_core::Value>,
    pub signed: Option<SignedStatement>,
}
impl Reconstruction {
    pub fn public_values(&self) -> Vec<Fr> {
        let mut values: Vec<Fr> = self.financial.iter().map(proving::value_fr).collect();
        if let Some(signed) = &self.signed {
            values.extend(signed.values.iter().copied());
        }
        values
    }
}

pub(crate) fn reconstruct(
    source: &str,
    action: &str,
    scenario: &Scenario,
    target: &Target,
    signed: Option<(&ExpectedAuthority, UInt)>,
) -> Result<Reconstruction, Error> {
    let statement = reconstruct_statement(source, action, scenario, target, signed)?;
    if let Some(s) = &statement.signed {
        check_caps(&statement.execution, &s.payload)?;
    }
    Ok(statement)
}
/// Reconstruction before the host cap comparison; tests use it to show that
/// the compiled circuit independently refuses a re-signed tighter cap.
fn reconstruct_statement(
    source: &str,
    action: &str,
    scenario: &Scenario,
    target: &Target,
    signed: Option<(&ExpectedAuthority, UInt)>,
) -> Result<Reconstruction, Error> {
    match (&target.signed_policy, &signed) {
        (Some(_), None) => {
            return Err(error(
                "target",
                "A signed Target requires the signed intent and proof entry points; it never falls back to unsigned mode.",
            ));
        }
        (None, Some(_)) => {
            return Err(error(
                "target",
                "Signed entry points require a trusted Target with signed_policy.",
            ));
        }
        _ => {}
    }
    if let Some((authority, _)) = &signed {
        authority.validate()?;
    }
    let program = mori_core::parse(source).map_err(|e| error("program", e.to_string()))?;
    let artifact = crate::compile_mode(&program, target)?;
    let mut selected = artifact.circuits.into_iter().filter(|c| c.action == action);
    let circuit = selected
        .next()
        .ok_or_else(|| error("action", "Selected action is absent."))?;
    if selected.next().is_some() {
        return Err(error("action", "Selected action is ambiguous."));
    }
    let registry = crate::liabilities::registry(&program)?;
    proving::liability_slots(&registry, &scenario.liabilities)?;
    let execution = mori_core::simulate(&program, action, scenario)
        .map_err(|e| error("scenario", e.to_string()))?;
    let financial = proving::financial_values(&program, &circuit, scenario, &execution)?;
    let source_hash = proving::hex(&Sha256::digest(source.as_bytes()));
    let signed = match (signed, &target.signed_policy) {
        (Some((authority, at)), Some(policy)) => Some(signed_statement(
            &program,
            &artifact.program_hash,
            &source_hash,
            action,
            scenario,
            target,
            policy,
            authority,
            at,
            &circuit,
            &financial,
        )?),
        _ => None,
    };
    Ok(Reconstruction {
        source_hash,
        program_hash: artifact.program_hash,
        circuit,
        registry,
        execution,
        financial,
        signed,
    })
}

#[allow(clippy::too_many_arguments)]
fn signed_statement(
    program: &Program,
    program_hash: &str,
    source_hash: &str,
    action: &str,
    scenario: &Scenario,
    target: &Target,
    policy: &SignedPolicy,
    authority: &ExpectedAuthority,
    at: UInt,
    circuit: &CircuitArtifact,
    financial: &[mori_core::Value],
) -> Result<SignedStatement, Error> {
    let target_hash = proving::hex(&digest(target)?);
    if authority.network != target.network
        || authority.anchor != target.anchor
        || authority.target_hash != target_hash
        || authority.program_address != policy.program_address
        || authority.owner != policy.owner
        || authority.beneficiary != policy.beneficiary
    {
        return Err(error(
            "authorization",
            "Expected authority differs from the trusted Target, its hash or its signed policy.",
        ));
    }
    if at < authority.not_before || at > authority.not_after {
        return Err(error(
            "authorization",
            format!(
                "Logical time {} is outside the signed interval {}..{}.",
                at.0, authority.not_before.0, authority.not_after.0
            ),
        ));
    }
    let payload = UnsignedPayload {
        profile: INTENT_PROFILE.into(),
        source_hash: source_hash.into(),
        program_hash: program_hash.into(),
        semantics: program.semantics.clone(),
        relation: RELATION_VERSION.into(),
        relation_hash: circuit.context_hash.clone(),
        abi: PUBLIC_ABI_VERSION.into(),
        action: action.into(),
        scenario_hash: proving::hex(&digest(scenario)?),
        target_hash,
        network: target.network.clone(),
        anchor: target.anchor.clone(),
        program_address: policy.program_address.clone(),
        owner: policy.owner.clone(),
        beneficiary: policy.beneficiary.clone(),
        public_key_hex: authority.public_key_hex.clone(),
        nonce: authority.nonce,
        predecessor: authority.predecessor.clone(),
        not_before: authority.not_before,
        not_after: authority.not_after,
        limits: authority.limits.clone(),
    };
    let source_assets: Vec<_> = program.assets.iter().map(|a| a.identity.clone()).collect();
    payload.validate_assets(&source_assets)?;
    let payload_hash = payload.hash()?;
    let context = authorization::decode_digest(&circuit.context_hash)?;
    let predecessor = authorization::digest_limbs(payload.predecessor_hash()?);
    let payload_limbs = authorization::digest_limbs(payload_hash);
    let context_limbs = authorization::digest_limbs(context);
    let mut preimage = vec![
        Fr::from(MESSAGE_DOMAIN),
        Fr::from(MESSAGE_VERSION),
        Fr::from(context_limbs[0].0),
        Fr::from(context_limbs[1].0),
        Fr::from(payload_limbs[0].0),
        Fr::from(payload_limbs[1].0),
        Fr::from(financial.len() as u64),
    ];
    for (binding, value) in circuit.public_inputs.iter().zip(financial) {
        preimage.push(Fr::from(tag(binding.ty)));
        preimage.push(proving::value_fr(value));
    }
    preimage.push(Fr::from(payload.limits.len() as u64));
    let mut caps = Vec::with_capacity(4 * payload.limits.len());
    for limit in &payload.limits {
        let limbs = authorization::digest_limbs(digest(&limit.asset)?);
        preimage.extend([Fr::from(limbs[0].0), Fr::from(limbs[1].0)]);
        let fields = [
            limit.gross_cap,
            limit.fee_cap,
            limit.net_floor,
            limit.liability_cap,
        ]
        .map(|n| Fr::from(n.0));
        preimage.extend(fields);
        caps.extend(fields);
    }
    preimage.extend(
        [
            payload.nonce,
            payload.not_before,
            payload.not_after,
            predecessor[0],
            predecessor[1],
        ]
        .map(|n| Fr::from(n.0)),
    );
    let message = transient_hash(&preimage);
    let key = schnorr::decode_key(&payload.public_key_hex)?;
    let (key_x, key_y) = schnorr::coordinates(key);
    let mut values = vec![
        Fr::from(payload_limbs[0].0),
        Fr::from(payload_limbs[1].0),
        key_x,
        key_y,
        message,
    ];
    values.extend(
        [
            payload.nonce,
            payload.not_before,
            payload.not_after,
            predecessor[0],
            predecessor[1],
            at,
        ]
        .map(|n| Fr::from(n.0)),
    );
    values.extend(caps);
    if values.len() != circuit.authorization_bindings.len() {
        return Err(error(
            "abi",
            "Authorization values differ from the trusted descriptor count.",
        ));
    }
    Ok(SignedStatement {
        payload,
        payload_hash,
        message,
        values,
        at,
    })
}

/// Host reference cap check over the complete ordered effects and closing
/// duties, with checked integers; the circuit enforces the same inequalities.
fn check_caps(execution: &Execution, payload: &UnsignedPayload) -> Result<(), Error> {
    let overflow = || error("authorization", "Checked cap sum overflows UInt128.");
    let add = |sum: &mut u128, n: UInt| -> Result<(), Error> {
        *sum = sum.checked_add(n.0).ok_or_else(overflow)?;
        Ok(())
    };
    let (owner, beneficiary) = (&payload.owner, &payload.beneficiary);
    for limit in &payload.limits {
        let (mut outgoing, mut fees, mut credit, mut debit) = (0u128, 0u128, 0u128, 0u128);
        for effect in &execution.effects {
            match effect {
                Effect::Transfer {
                    asset,
                    from,
                    to,
                    amount,
                } if *asset == limit.asset => {
                    if from == owner {
                        add(&mut outgoing, *amount)?;
                    }
                    if to == beneficiary {
                        add(&mut credit, *amount)?;
                    }
                    if from == beneficiary {
                        add(&mut debit, *amount)?;
                    }
                }
                Effect::Fee {
                    asset,
                    from,
                    amount,
                    ..
                } if *asset == limit.asset => {
                    if from == owner {
                        add(&mut fees, *amount)?;
                    }
                    if beneficiary != owner && from == beneficiary {
                        add(&mut debit, *amount)?;
                    }
                }
                _ => {}
            }
        }
        let gross = outgoing.checked_add(fees).ok_or_else(overflow)?;
        let net = credit
            .checked_sub(debit)
            .and_then(|n| n.checked_sub(fees))
            .ok_or_else(|| error("authorization", "Checked net position underflows."))?;
        let mut duty = 0u128;
        for l in &execution.post.liabilities {
            if l.asset == limit.asset && &l.debtor == owner {
                add(&mut duty, l.amount)?;
            }
        }
        let refusal = if gross > limit.gross_cap.0 {
            Some(format!("gross {gross} exceeds gross_cap {}", limit.gross_cap.0))
        } else if fees > limit.fee_cap.0 {
            Some(format!("fee total {fees} exceeds fee_cap {}", limit.fee_cap.0))
        } else if net < limit.net_floor.0 {
            Some(format!("net {net} is below net_floor {}", limit.net_floor.0))
        } else if duty > limit.liability_cap.0 {
            Some(format!(
                "closing owner liability {duty} exceeds liability_cap {}",
                limit.liability_cap.0
            ))
        } else {
            None
        };
        if let Some(message) = refusal {
            return Err(error("authorization", message));
        }
    }
    Ok(())
}

/// Strict signed acceptance shared by verify-intent, prove and verify: the
/// exact wrapper equals the reconstruction, the attachment satisfies FENS
/// under the reconstructed message and key, and the compiled relation accepts
/// that witness under the ordinary ZKIR checker. Returns the raw private
/// witness `[PK.x,PK.y,R.x,R.y,s]`.
pub(crate) fn accept(
    source: &str,
    action: &str,
    scenario: &Scenario,
    target: &Target,
    authority: &ExpectedAuthority,
    at: UInt,
    signed: &SignedIntent,
) -> Result<(Reconstruction, Vec<Fr>), Error> {
    signed.validate()?;
    let statement = reconstruct(source, action, scenario, target, Some((authority, at)))?;
    let s = statement.signed.as_ref().expect("signed reconstruction");
    if mori_core::intent::canonical(&signed.payload)
        .map_err(|e| error("serialization", e.to_string()))?
        != mori_core::intent::canonical(&s.payload)
            .map_err(|e| error("serialization", e.to_string()))?
    {
        return Err(error(
            "authorization",
            "Signed payload differs from the independently reconstructed statement.",
        ));
    }
    schnorr::verify_attachment(&s.payload.public_key_hex, &signed.signature_hex, s.message)?;
    let key = schnorr::decode_key(&s.payload.public_key_hex)?;
    let signature = schnorr::decode_signature(&signed.signature_hex)?;
    let (key_x, key_y) = schnorr::coordinates(key);
    let (r_x, r_y) = schnorr::coordinates(signature.r);
    let witness = vec![key_x, key_y, r_x, r_y, signature.response];
    check_relation(&statement.circuit, &statement.public_values(), &witness)?;
    Ok((statement, witness))
}

/// Ordinary off-circuit ZKIR checker; not native proving or verification.
pub(crate) fn check_relation(
    circuit: &CircuitArtifact,
    values: &[Fr],
    witness: &[Fr],
) -> Result<(), Error> {
    let ir = IrSource::load(circuit.zkir.as_bytes()).map_err(|e| error("zkir", e.to_string()))?;
    use midnight_transient_crypto::proofs::{KeyLocation, Zkir};
    let preimage = ProofPreimage {
        inputs: vec![],
        private_transcript: witness.to_vec(),
        public_transcript_inputs: values.to_vec(),
        public_transcript_outputs: values.to_vec(),
        binding_input: Fr::from(0u64),
        communications_commitment: None,
        key_location: KeyLocation("moriarty-ordinary-relation-check".into()),
    };
    ir.check(&preimage)
        .map_err(|e| error("relation", format!("Compiled relation refused: {e}")))?;
    Ok(())
}

/// Trusted preparation of the unsigned payload and native message. The
/// report is generated review data, not authority; nothing is signed here.
pub fn prepare_intent(
    source: &str,
    action: &str,
    scenario: &Scenario,
    target: &Target,
    authority: &ExpectedAuthority,
    at: UInt,
) -> Result<PreparedIntent, Error> {
    let statement = reconstruct(source, action, scenario, target, Some((authority, at)))?;
    let signed = statement.signed.expect("signed reconstruction");
    let financial_projection = statement
        .financial
        .into_iter()
        .zip(&statement.circuit.public_inputs)
        .enumerate()
        .map(|(position, (value, binding))| FinancialValue {
            position: position as u32,
            binding: binding.clone(),
            value,
        })
        .collect();
    let authorization_projection = statement
        .circuit
        .authorization_bindings
        .iter()
        .zip(&signed.values)
        .map(|(binding, value)| AuthorizationValue {
            position: binding.position,
            binding: binding.clone(),
            value: schnorr::native_hex(*value),
        })
        .collect();
    let prepared = PreparedIntent {
        profile: PREPARED_PROFILE.into(),
        payload: signed.payload,
        payload_hash: proving::hex(&signed.payload_hash),
        context_hash: statement.circuit.context_hash,
        message_hex: schnorr::native_hex(signed.message),
        financial_projection,
        authorization_projection,
        review: statement.execution,
    };
    prepared.validate()?;
    Ok(prepared)
}

/// Host acceptance of an exact signed wrapper; returns the complete local
/// reference execution. This is not ledger acceptance or key authority.
pub fn verify_intent(
    source: &str,
    action: &str,
    scenario: &Scenario,
    target: &Target,
    authority: &ExpectedAuthority,
    at: UInt,
    signed: &SignedIntent,
) -> Result<Execution, Error> {
    let (statement, _) = accept(source, action, scenario, target, authority, at, signed)?;
    Ok(statement.execution)
}

#[cfg(test)]
mod tests {
    //! Ordinary off-circuit ZKIR checker controls only (`ProofPreimage::check`):
    //! no SRS, key generation, proving, MockProver or native verification.
    //! A checker refusal is not evidence of a native proof failure.
    use super::*;
    use midnight_curves::pairing::group::{Group, ff::Field};
    use midnight_curves::{Fq, Fr as Scalar, JubjubAffine, JubjubExtended, JubjubSubgroup};

    const ALTERNATIVE: &str = "fec0b07c971073c462c88d722fb5fe8df764311114dbcc79e46263b74a863850e92947318a0258f88a6869c98a7e96733662780d4936401c0e732a4c87fb651a98b9a4ecb5630fefa96bd5f061388d910c95dabfa7f2c089c2d3651ca422e900";

    struct Case {
        source: String,
        action: &'static str,
        scenario: Scenario,
        target: Target,
        authority: ExpectedAuthority,
        signed: SignedIntent,
    }
    fn golden() -> Case {
        let g: serde_json::Value = serde_json::from_str(include_str!(
            "../../../examples/core/native-owner-intent/golden-vector.json"
        ))
        .unwrap();
        let mut authority = serde_json::Map::new();
        for k in [
            "profile",
            "network",
            "anchor",
            "program_address",
            "target_hash",
            "owner",
            "beneficiary",
            "public_key_hex",
            "nonce",
            "predecessor",
            "not_before",
            "not_after",
            "limits",
        ] {
            authority.insert(k.into(), g["payload"][k].clone());
        }
        Case {
            source: g["source_utf8"].as_str().unwrap().into(),
            action: "repay",
            scenario: serde_json::from_value(g["scenario"].clone()).unwrap(),
            target: Target::from_json(g["target_canonical_utf8"].as_str().unwrap()).unwrap(),
            authority: ExpectedAuthority::from_json(&serde_json::Value::Object(authority).to_string())
                .unwrap(),
            signed: SignedIntent::from_json(&g["signed_wrapper"].to_string()).unwrap(),
        }
    }
    fn statement(c: &Case, at: u128) -> Reconstruction {
        reconstruct_statement(
            &c.source,
            c.action,
            &c.scenario,
            &c.target,
            Some((&c.authority, UInt(at))),
        )
        .unwrap()
    }
    fn raw(key: JubjubSubgroup, signature: &str) -> Vec<Fr> {
        let s = schnorr::decode_signature(signature).unwrap();
        let (kx, ky) = schnorr::coordinates(key);
        let (rx, ry) = schnorr::coordinates(s.r);
        vec![kx, ky, rx, ry, s.response]
    }
    fn golden_key() -> JubjubSubgroup {
        JubjubSubgroup::generator() * Scalar::from(7u64)
    }
    fn check(r: &Reconstruction, values: &[Fr], witness: &[Fr]) -> Result<(), Error> {
        check_relation(&r.circuit, values, witness)
    }
    fn qa_sign(secret: u64, nonce: u64, message: Fr, domain: u64, swap: bool) -> String {
        let g = JubjubSubgroup::generator();
        let (x, k) = (Scalar::from(secret), Scalar::from(nonce));
        let (rx, ry) = schnorr::coordinates(g * k);
        let (px, py) = schnorr::coordinates(g * x);
        let order = if swap {
            [Fr::from(domain), px, py, rx, ry, message]
        } else {
            [Fr::from(domain), rx, ry, px, py, message]
        };
        let h = transient_hash(&order);
        let mut wide = [0u8; 64];
        wide[..32].copy_from_slice(&h.as_le_bytes());
        let s = k + Scalar::from_bytes_wide(&wide) * x;
        let mut bytes = rx.as_le_bytes();
        bytes.extend(ry.as_le_bytes());
        bytes.extend(s.to_bytes());
        mori_core::intent::hex(&bytes)
    }

    #[test]
    fn honest_golden_witness_and_alternative_attachment_share_one_statement() {
        let c = golden();
        let r = statement(&c, 15);
        let values = r.public_values();
        assert_eq!(values.len(), 43);
        check(&r, &values, &raw(golden_key(), &c.signed.signature_hex)).unwrap();
        // A second valid attachment satisfies the same public statement.
        check(&r, &values, &raw(golden_key(), ALTERNATIVE)).unwrap();
        let mut alternative = c.signed.clone();
        alternative.signature_hex = ALTERNATIVE.into();
        let (a, _) = accept(&c.source, c.action, &c.scenario, &c.target, &c.authority, UInt(15), &alternative)
            .unwrap();
        assert_eq!(a.public_values(), values);
        // Changing private R or s alone refuses in the checker.
        let honest = raw(golden_key(), &c.signed.signature_hex);
        for i in 2..5 {
            let mut bad = honest.clone();
            bad[i] = bad[i] + Fr::from(1u64);
            assert!(check(&r, &values, &bad).is_err(), "private {i}");
        }
    }

    #[test]
    fn disconnected_public_message_key_payload_caps_and_time_refuse() {
        let c = golden();
        let r = statement(&c, 15);
        let witness = raw(golden_key(), &c.signed.signature_hex);
        let n = r.financial.len();
        // Every authorization position and every financial position is bound:
        // payload limbs, key, message, metadata, at and caps.
        for i in 0..r.public_values().len() {
            let mut values = r.public_values();
            values[i] = values[i] + Fr::from(1u64);
            let result = check(&r, &values, &witness);
            if i == n + 10 {
                // at=16 is in-interval and outside the message: same signed
                // message, a different (valid) native public statement.
                result.unwrap();
            } else {
                assert!(result.is_err(), "public position {i}");
            }
        }
        // A different valid key in both public and private positions still needs
        // a signature over the same message under that key.
        let other = JubjubSubgroup::generator() * Scalar::from(13u64);
        let mut values = r.public_values();
        let (ox, oy) = schnorr::coordinates(other);
        values[n + 2] = ox;
        values[n + 3] = oy;
        let mut swapped = witness.clone();
        swapped[0] = ox;
        swapped[1] = oy;
        assert!(check(&r, &values, &swapped).is_err());
        // Private key coordinates must equal the public key coordinates.
        let mut moved = witness.clone();
        moved[0] = ox;
        moved[1] = oy;
        assert!(check(&r, &r.public_values(), &moved).is_err());
        // Interval endpoints accept; outside them the circuit refuses.
        for at in [10u128, 20] {
            let s = statement(&c, at);
            check(&s, &s.public_values(), &witness).unwrap();
        }
        for at in [9u128, 21] {
            let mut values = r.public_values();
            values[n + 10] = Fr::from(at);
            assert!(check(&r, &values, &witness).is_err(), "at {at}");
        }
    }

    #[test]
    fn strict_fens_tag_order_canonical_response_and_points() {
        let c = golden();
        let r = statement(&c, 15);
        let values = r.public_values();
        let message = r.signed.as_ref().unwrap().message;
        for (domain, swap) in [(0x46454e54u64, false), (0x46454e53, true), (0, false)] {
            let sig = qa_sign(7, 11, message, domain, swap);
            assert!(check(&r, &values, &raw(golden_key(), &sig)).is_err());
        }
        check(
            &r,
            &values,
            &raw(golden_key(), &qa_sign(7, 11, message, 0x46454e53, false)),
        )
        .unwrap();
        let honest = raw(golden_key(), &c.signed.signature_hex);
        // r = Jubjub scalar order; s+r and r refuse by roundtrip, 0/1/r-1 by equation.
        let order = Fr::from_le_bytes(&[
            183, 44, 247, 214, 94, 14, 151, 208, 130, 16, 200, 204, 147, 32, 104, 166, 0, 59, 52,
            1, 1, 59, 103, 6, 169, 175, 51, 101, 234, 180, 125, 14,
        ])
        .unwrap();
        for s in [
            honest[4] + order,
            order,
            order + Fr::from(1u64),
            Fr::from(0u64),
            Fr::from(1u64),
            order - Fr::from(1u64),
        ] {
            let mut bad = honest.clone();
            bad[4] = s;
            assert!(check(&r, &values, &bad).is_err());
        }
        // Identity R is a typed subgroup point and refuses by the nonidentity check;
        // torsion and mixed points refuse typed subgroup assignment.
        let identity = [Fr::from(0u64), Fr::from(1u64)];
        let torsion = JubjubAffine::from_raw_unchecked(Fq::ZERO, -Fq::ONE);
        let mixed = JubjubAffine::from(
            JubjubExtended::from(JubjubSubgroup::generator()) + JubjubExtended::from(torsion),
        );
        for (x, y) in [
            (identity[0], identity[1]),
            (Fr(torsion.get_u()), Fr(torsion.get_v())),
            (Fr(mixed.get_u()), Fr(mixed.get_v())),
        ] {
            let mut bad = honest.clone();
            bad[2] = x;
            bad[3] = y;
            assert!(check(&r, &values, &bad).is_err());
            let mut key = honest.clone();
            key[0] = x;
            key[1] = y;
            let mut public = values.clone();
            let n = r.financial.len();
            public[n + 2] = x;
            public[n + 3] = y;
            assert!(check(&r, &public, &key).is_err());
        }
    }

    fn payment(caps: &serde_json::Value) -> Case {
        let mut authority: serde_json::Value = serde_json::from_str(include_str!(
            "../../../examples/core/native-owner-intent/payment.authority.json"
        ))
        .unwrap();
        for k in ["gross_cap", "fee_cap", "net_floor", "liability_cap"] {
            authority["limits"][0][k] = caps[k].clone();
        }
        let authority = ExpectedAuthority::from_json(&authority.to_string()).unwrap();
        Case {
            source: include_str!("../../../examples/core/native-owner-intent/payment.mori").into(),
            action: "settle",
            scenario: serde_json::from_str(include_str!(
                "../../../examples/core/native-owner-intent/payment.scenario.json"
            ))
            .unwrap(),
            target: Target::from_json(include_str!(
                "../../../examples/core/native-owner-intent/payment.target.json"
            ))
            .unwrap(),
            // Replaced by the freshly signed reconstruction in each test.
            signed: golden().signed,
            authority,
        }
    }

    #[test]
    fn freshly_signed_tighter_caps_refuse_in_circuit_and_host() {
        let expected: serde_json::Value = serde_json::from_str(include_str!(
            "../../../examples/core/native-owner-intent/payment.expected.json"
        ))
        .unwrap();
        let exact = payment(&expected["caps_accept"]);
        let r = statement(&exact, 15);
        let message = r.signed.as_ref().unwrap().message;
        let sig = qa_sign(7, 29, message, schnorr::FENS_DOMAIN, false);
        check(&r, &r.public_values(), &raw(golden_key(), &sig)).unwrap();
        for (caps, word) in expected["individual_cap_refusals"]
            .as_array()
            .unwrap()
            .iter()
            .zip(["gross", "fee", "net", "liability"])
        {
            let mut tight = payment(caps);
            let t = statement(&tight, 15);
            let s = t.signed.as_ref().unwrap();
            assert_ne!(s.message, message);
            let sig = qa_sign(7, 31, s.message, schnorr::FENS_DOMAIN, false);
            // The fresh attachment is valid; only the compiled cap refuses.
            schnorr::verify_attachment(&s.payload.public_key_hex, &sig, s.message).unwrap();
            let err = check(&t, &t.public_values(), &raw(golden_key(), &sig)).unwrap_err();
            assert_eq!(err.code, "mori::backend::relation", "{word}");
            tight.signed = SignedIntent {
                payload: s.payload.clone(),
                signature_hex: sig,
            };
            let err = verify_intent(
                &tight.source,
                tight.action,
                &tight.scenario,
                &tight.target,
                &tight.authority,
                UInt(15),
                &tight.signed,
            )
            .unwrap_err();
            assert!(err.message.contains(word), "{word}: {err}");
        }
    }

    #[test]
    fn self_beneficiary_net_and_checked_sum_overflow() {
        let asset = AssetId {
            network: "local".into(),
            issuer: "i".into(),
            reference: "QA".into(),
            scale: 0,
        };
        let limit = |gross: u128, fee: u128, net: u128| mori_core::intent::AssetLimit {
            asset: asset.clone(),
            beneficiary: "alice".into(),
            gross_cap: UInt(gross),
            fee_cap: UInt(fee),
            net_floor: UInt(net),
            liability_cap: UInt(0),
        };
        let transfer = |amount: u128| Effect::Transfer {
            asset: asset.clone(),
            from: "alice".into(),
            to: "alice".into(),
            amount: UInt(amount),
        };
        let fee = |amount: u128| Effect::Fee {
            asset: asset.clone(),
            from: "alice".into(),
            to: "treasury".into(),
            amount: UInt(amount),
        };
        let mut payload = golden().signed.payload;
        payload.owner = "alice".into();
        payload.beneficiary = "alice".into();
        let empty = mori_core::Snapshot {
            state: Default::default(),
            balances: vec![],
            liabilities: vec![],
        };
        let run = |effects: Vec<Effect>, l: mori_core::intent::AssetLimit| {
            let mut p = payload.clone();
            p.limits = vec![l];
            check_caps(
                &Execution {
                    semantics: "moriarty-core/1".into(),
                    action: "a".into(),
                    pre: empty.clone(),
                    post: empty.clone(),
                    steps: vec![],
                    effects,
                },
                &p,
            )
        };
        // Owner = beneficiary self-transfer 10: gross 10, net 0.
        run(vec![transfer(10)], limit(10, 0, 0)).unwrap();
        assert!(run(vec![transfer(10)], limit(9, 0, 0)).unwrap_err().message.contains("gross"));
        assert!(run(vec![transfer(10)], limit(10, 0, 1)).unwrap_err().message.contains("net"));
        // A nonzero owner fee makes the self-beneficiary net underflow.
        assert!(
            run(vec![transfer(10), fee(1)], limit(11, 1, 0))
                .unwrap_err()
                .message
                .contains("underflow")
        );
        assert!(
            run(vec![transfer(u128::MAX), transfer(1)], limit(u128::MAX, 0, 0))
                .unwrap_err()
                .message
                .contains("overflow")
        );
    }
}
