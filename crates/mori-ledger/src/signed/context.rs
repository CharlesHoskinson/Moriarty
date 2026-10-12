use super::*;
use crate::relation::{Artifact, Manifest};
use midnight_transient_crypto::{curve::Fr, hash::transient_hash};
use mori_core::{Effect, Execution, Type};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WireDescriptor {
    pub position: usize,
    pub descriptor: String,
    #[serde(rename = "type")]
    pub ty: Type,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WireProjection {
    pub position: usize,
    pub descriptor: String,
    #[serde(rename = "type")]
    pub ty: Type,
    #[serde(serialize_with = "serialize_scalar")]
    pub value: Value,
}
fn serialize_scalar<S: serde::Serializer>(
    value: &Value,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error> {
    match value {
        Value::UInt(n) => n.serialize(serializer),
        Value::Bool(b) => b.serialize(serializer),
    }
}
impl<'de> Deserialize<'de> for WireProjection {
    fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> std::result::Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Scalar {
            UInt(UInt),
            Bool(bool),
        }
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Field {
            position: usize,
            descriptor: String,
            #[serde(rename = "type")]
            ty: Type,
            value: Scalar,
        }
        let f = Field::deserialize(decoder)?;
        let value = match f.value {
            Scalar::UInt(n) => Value::UInt(n),
            Scalar::Bool(b) => Value::Bool(b),
        };
        if f.ty != value.ty() {
            return Err(serde::de::Error::custom("projection scalar/type mismatch"));
        }
        Ok(Self {
            position: f.position,
            descriptor: f.descriptor,
            ty: f.ty,
            value,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArgumentDescriptor {
    pub position: usize,
    pub name: String,
    pub ty: String,
    pub encoded_fields: usize,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedManifest {
    pub storage_abi: String,
    pub signed_abi: String,
    #[serde(deserialize_with = "strict_field")]
    pub context: serde_json::Value,
    pub context_sha256: String,
    pub signer_policy: FixedSignerPolicy,
    pub typed_argument_descriptors: Vec<ArgumentDescriptor>,
    pub typed_argument_count: usize,
    pub flattened_field_count: usize,
    pub semantic_wire_descriptors: Vec<WireDescriptor>,
}
pub(crate) fn digest_limbs(hex: &str) -> Result<[u128; 2]> {
    let b = schnorr::unhex(hex, 32)?;
    Ok([
        u128::from_le_bytes(b[..16].try_into().map_err(|_| refuse("digest"))?),
        u128::from_le_bytes(b[16..].try_into().map_err(|_| refuse("digest"))?),
    ])
}
fn args(s: &SignedStaticBindings, action: &str) -> Result<Vec<ArgumentDescriptor>> {
    let a = s
        .program
        .actions
        .iter()
        .find(|a| a.name == action)
        .ok_or(refuse("action"))?;
    let mut out = vec![];
    let mut push = |name: String, ty: &str, fields| {
        out.push(ArgumentDescriptor {
            position: out.len(),
            name,
            ty: ty.into(),
            encoded_fields: fields,
        });
    };
    for i in &a.inputs {
        push(
            format!("source.{}", i.name),
            if i.ty == Type::UInt { "UInt" } else { "Bool" },
            1,
        );
    }
    push("PK".into(), "Point<Jubjub>", 2);
    push("R".into(), "Point<Jubjub>", 2);
    push("s".into(), "Native", 1);
    for name in [
        "P.lo",
        "P.hi",
        "nonce",
        "not_before",
        "not_after",
        "predecessor.lo",
        "predecessor.hi",
        "at",
    ] {
        push(name.into(), "UInt", 1);
    }
    for (i, _) in s.assets.iter().enumerate() {
        for name in ["gross_cap", "fee_cap", "net_floor", "liability_cap"] {
            push(format!("limits.{i}.{name}"), "UInt", 1);
        }
    }
    Ok(out)
}
pub fn compile_signed(s: &SignedStaticBindings, action: &str) -> Result<Artifact> {
    let validated = layout::validate_static(&s.source, &s.program, &s.bindings)?;
    let (template, groups, n, wires) =
        crate::relation::compiler::compile_signed(&validated, action, s, None)?;
    let arguments = args(s, action)?;
    let ordinal = s
        .program
        .actions
        .iter()
        .position(|a| a.name == action)
        .ok_or(refuse("action"))?;
    let templates=template.instructions.iter().filter_map(|i|if let midnight_zkir_v3::Instruction::Impact{guard,inputs}=i {Some(serde_json::json!({"guard":guard,"fields":inputs}))}else{None}).zip(&groups).map(|(v,g)|serde_json::json!({"origin":g.origin,"field_count":g.field_count,"template":v})).collect::<Vec<_>>();
    let d = &s.bindings;
    let context = serde_json::json!([
        CONTEXT,
        PROFILE,
        RELATION,
        SIGNED_ABI,
        d.source_hash,
        d.program_hash,
        mori_core::LANGUAGE,
        mori_core::SEMANTICS,
        crate::proving::source_closure(),
        d.toolchain,
        d.templates_hash,
        s.policy,
        d.network,
        d.anchor,
        d.address_hex,
        d.payer,
        canonical_roles(s),
        canonical_assets(s),
        s.duties,
        action,
        ordinal,
        arguments,
        wires,
        templates
    ]);
    let context_sha256 = codec::object_hash(&context)?;
    let (ir, actual_groups, actual_n, actual_wires) = crate::relation::compiler::compile_signed(
        &validated,
        action,
        s,
        Some(digest_limbs(&context_sha256)?),
    )?;
    if actual_groups != groups || actual_wires != wires || n != actual_n {
        return Err(refuse("static-layout-drift"));
    }
    let manifest = Manifest {
        format: RELATION.into(),
        source_hash: d.source_hash.clone(),
        program_hash: d.program_hash.clone(),
        action: action.into(),
        abi: validated.abi().clone(),
        ir_hash: codec::hash(&codec::encode(&ir)?),
        input_count: n,
        groups,
        qualification: QUALIFICATION.into(),
        compiler_hash: crate::proving::source_closure(),
        signed: Some(SignedManifest {
            storage_abi: ABI.into(),
            signed_abi: SIGNED_ABI.into(),
            context,
            context_sha256,
            signer_policy: s.policy.clone(),
            typed_argument_count: arguments.len(),
            flattened_field_count: n,
            typed_argument_descriptors: arguments,
            semantic_wire_descriptors: wires,
        }),
        open_predicates: vec![
            "authenticated account/deployment/predecessor/history/replay/time".into(),
            "funding/settlement/native crypto and compiler correspondence".into(),
        ],
    };
    Ok(Artifact { ir, manifest })
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerPayload {
    pub profile: String,
    pub installed_profile: String,
    pub qualification: String,
    pub relation: String,
    pub ir_sha256: String,
    pub storage_abi: String,
    pub signed_abi: String,
    pub context_sha256: String,
    pub source_sha256: String,
    pub program_sha256: String,
    pub semantics: String,
    pub compiler_sha256: String,
    #[serde(deserialize_with = "strict_field")]
    pub toolchain: Toolchain,
    pub action: String,
    pub action_ordinal: usize,
    pub signer_policy_sha256: String,
    pub owner: String,
    pub beneficiary: String,
    pub public_key_hex: String,
    pub network: String,
    pub anchor: String,
    pub address_hex: String,
    pub duties: Vec<DutyBinding>,
    pub assets: Vec<AssetBinding>,
    pub predecessor_sha256: String,
    pub native_state_hash_hex: String,
    pub parameters_sha256: String,
    pub request_sha256: String,
    pub selected_spends: Vec<String>,
    pub ttl_hex: String,
    pub nonce: UInt,
    pub not_before: UInt,
    pub not_after: UInt,
    pub limits: Vec<mori_core::intent::AssetLimit>,
    pub wire_projection: Vec<WireProjection>,
    pub wire_projection_sha256: String,
    pub signing_readback_sha256: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CallSigningPackage {
    pub profile: String,
    #[serde(deserialize_with = "strict_field")]
    pub context: serde_json::Value,
    pub payload: OwnerPayload,
    pub payload_sha256: String,
    pub message_hex: String,
    #[serde(deserialize_with = "strict_field")]
    pub signing_readback: serde_json::Value,
}
impl CallSigningPackage {
    pub fn from_json(text: &str) -> Result<Self> {
        let p: Self = codec::json(text)?;
        if p.profile != PACKAGE
            || codec::object_hash(&p.payload)? != p.payload_sha256
            || codec::object_hash(&p.context)? != p.payload.context_sha256
            || codec::object_hash(&p.signing_readback)? != p.payload.signing_readback_sha256
            || codec::object_hash(&p.payload.wire_projection)? != p.payload.wire_projection_sha256
            || schnorr::native_hex(message(&p.payload)?) != p.message_hex
        {
            return Err(refuse("package-projections"));
        }
        Ok(p)
    }
}
pub fn message(p: &OwnerPayload) -> Result<Fr> {
    let c = digest_limbs(&p.context_sha256)?;
    let payload = digest_limbs(&codec::object_hash(p)?)?;
    let pred = digest_limbs(&p.predecessor_sha256)?;
    let mut preimage = vec![
        Fr::from(MESSAGE_DOMAIN),
        Fr::from(1),
        Fr::from(c[0]),
        Fr::from(c[1]),
        Fr::from(payload[0]),
        Fr::from(payload[1]),
        Fr::from(p.wire_projection.len() as u128),
    ];
    for v in &p.wire_projection {
        if v.value.ty() != v.ty {
            return Err(refuse("projection-type"));
        }
        preimage.extend([
            Fr::from(if v.ty == Type::Bool { 1 } else { 2 }),
            Fr::from(match v.value {
                Value::UInt(n) => n.0,
                Value::Bool(b) => u128::from(b),
            }),
        ]);
    }
    preimage.push(Fr::from(p.limits.len() as u128));
    for l in &p.limits {
        let limbs = digest_limbs(&codec::object_hash(&l.asset)?)?;
        preimage.extend([
            Fr::from(limbs[0]),
            Fr::from(limbs[1]),
            Fr::from(l.gross_cap.0),
            Fr::from(l.fee_cap.0),
            Fr::from(l.net_floor.0),
            Fr::from(l.liability_cap.0),
        ]);
    }
    preimage.extend([
        Fr::from(p.nonce.0),
        Fr::from(p.not_before.0),
        Fr::from(p.not_after.0),
        Fr::from(pred[0]),
        Fr::from(pred[1]),
    ]);
    Ok(transient_hash(&preimage))
}
pub(crate) fn projection(
    s: &SignedStaticBindings,
    e: &Execution,
    request: &SignedCallRequest,
    action: &str,
) -> Result<Vec<WireProjection>> {
    let mut out = vec![];
    let mut push = |descriptor: String, value: Value| {
        out.push(WireProjection {
            position: out.len(),
            descriptor,
            ty: value.ty(),
            value,
        });
    };
    let a = s
        .program
        .actions
        .iter()
        .find(|a| a.name == action)
        .ok_or(refuse("action"))?;
    for i in &a.inputs {
        push(format!("input.{}", i.name), request.inputs[&i.name].clone());
    }
    for state in &s.program.states {
        push(
            format!("pre.state.{}", state.name),
            e.pre.state[&state.name].clone(),
        );
    }
    for d in &s.duties {
        let l = e.pre.liabilities.iter().find(|l| l.name == d.name);
        push(
            format!("pre.duty.{}.present", d.name),
            Value::Bool(l.is_some()),
        );
        push(
            format!("pre.duty.{}.amount", d.name),
            Value::UInt(l.map_or(UInt(0), |l| l.amount)),
        );
    }
    for step in &e.steps {
        for effect in &step.effects {
            match effect {
                Effect::State {
                    name,
                    before,
                    after,
                } => {
                    push(
                        format!("effect.{}.state.{name}.before", step.index),
                        before.clone(),
                    );
                    push(
                        format!("effect.{}.state.{name}.after", step.index),
                        after.clone(),
                    );
                }
                Effect::Transfer {
                    asset,
                    from,
                    to,
                    amount,
                }
                | Effect::Fee {
                    asset,
                    from,
                    to,
                    amount,
                } => {
                    let name = &s
                        .program
                        .assets
                        .iter()
                        .find(|a| a.identity == *asset)
                        .ok_or(refuse("asset"))?
                        .name;
                    push(
                        format!(
                            "effect.{}.{}.{name}.{from}.{to}.amount",
                            step.index,
                            if matches!(effect, Effect::Fee { .. }) {
                                "fee"
                            } else {
                                "transfer"
                            }
                        ),
                        Value::UInt(*amount),
                    );
                }
                Effect::Liability {
                    name,
                    before,
                    after,
                    ..
                } => {
                    if matches!(step.statement, Statement::Discharge { .. }) {
                        push(
                            format!("effect.{}.discharge.{name}.amount", step.index),
                            Value::UInt(UInt(
                                before.0.checked_sub(after.0).ok_or(refuse("discharge"))?,
                            )),
                        );
                    }
                    push(
                        format!("effect.{}.liability.{name}.before", step.index),
                        Value::UInt(*before),
                    );
                    push(
                        format!("effect.{}.liability.{name}.after", step.index),
                        Value::UInt(*after),
                    );
                }
                Effect::Emit { name, value } => push(
                    format!("effect.{}.event.{name}.value", step.index),
                    value.clone(),
                ),
            }
        }
    }
    for state in &s.program.states {
        push(
            format!("post.state.{}", state.name),
            e.post.state[&state.name].clone(),
        );
    }
    for d in &s.duties {
        let l = e.post.liabilities.iter().find(|l| l.name == d.name);
        push(
            format!("post.duty.{}.present", d.name),
            Value::Bool(l.is_some()),
        );
        push(
            format!("post.duty.{}.amount", d.name),
            Value::UInt(l.map_or(UInt(0), |l| l.amount)),
        );
    }
    Ok(out)
}

pub(crate) fn canonical_roles(s: &SignedStaticBindings) -> Vec<RoleBinding> {
    let mut roles = s.bindings.roles.clone();
    roles.sort_by(|a, b| a.role.cmp(&b.role));
    roles
}
pub(crate) fn canonical_assets(s: &SignedStaticBindings) -> Vec<AssetBinding> {
    s.assets
        .iter()
        .map(|id| {
            s.bindings
                .assets
                .iter()
                .find(|a| a.identity == *id)
                .expect("admitted asset mapping")
                .clone()
        })
        .collect()
}
