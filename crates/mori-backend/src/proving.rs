//! Public local native transition proofs. The circuit encodes its declared ABI
//! frame. Reconstruction and the binding commitment supply the complete local
//! report framing; neither authenticates ledger state, owner authority or history.
use crate::{
    CircuitArtifact, Error, LiabilityIdentity, LiabilityOperation, PUBLIC_ABI_VERSION,
    RELATION_VERSION, Target, compile, digest, error, parameters::OfflineParameters,
};
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{KeyLocation, PARAMS_VERIFIER, Proof, ProofPreimage, VERIFIER_MAX_DEGREE, Zkir},
};
use midnight_zkir_v3::IrSource;
use mori_core::{Effect, Execution, Liability, Program, Scenario, Statement, UInt, Value};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub const MAX_PROOF_BYTES: usize = 262_144;
pub const MAX_K: u8 = 19;
const FORMAT: &str = "moriarty-native-transition-proof/2";
const BINDING: &str = "moriarty-native-transition-binding/2";
#[derive(Debug, Clone)]
pub struct ProofOptions {
    pub params_dir: PathBuf,
    pub max_k: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofBundle {
    pub abi: String,
    pub relation: String,
    pub binding: String,
    pub profile: String,
    pub registry: Vec<LiabilityIdentity>,
    pub format: String,
    pub source_hash: String,
    pub program_hash: String,
    pub target_hash: String,
    pub scenario_hash: String,
    pub execution_hash: String,
    pub action: String,
    pub circuit_k: u8,
    pub proof_hex: String,
}
impl ProofBundle {
    fn validate_versions(&self) -> Result<(), Error> {
        if self.format != FORMAT
            || self.abi != PUBLIC_ABI_VERSION
            || self.relation != RELATION_VERSION
            || self.binding != BINDING
        {
            return Err(error(
                "claim",
                "Stale or unsupported native proof, ABI, relation or binding revision.",
            ));
        }
        Ok(())
    }
    pub fn from_json(text: &str) -> Result<Self, Error> {
        let value = mori_core::json::parse(text).map_err(|e| error("json", e.to_string()))?;
        let bundle: Self =
            serde_json::from_value(value).map_err(|e| error("json", e.to_string()))?;
        bundle.validate_versions()?;
        Ok(bundle)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofReceipt {
    pub qualification: String,
    pub execution: Execution,
    pub circuit_k: u8,
    pub proof_bytes: u32,
    pub open_gates: Vec<String>,
}
#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct Claim {
    abi: String,
    relation: String,
    binding: String,
    profile: String,
    registry: Vec<LiabilityIdentity>,
    format: String,
    source_hash: String,
    program_hash: String,
    target_hash: String,
    scenario_hash: String,
    execution_hash: String,
    action: String,
    circuit_k: u8,
}
struct Prepared {
    claim: Claim,
    execution: Execution,
    ir: IrSource,
    values: Vec<Fr>,
    binding: Fr,
}
pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn value_fr(value: &Value) -> Fr {
    match value {
        Value::UInt(n) => Fr::from(n.0),
        Value::Bool(b) => Fr::from(*b),
    }
}

fn prepare(
    source: &str,
    action: &str,
    scenario: &Scenario,
    target: &Target,
    options: &ProofOptions,
) -> Result<Prepared, Error> {
    if options.max_k == 0 || options.max_k > MAX_K {
        return Err(error("limit", "Native k cap must be between 1 and 19."));
    }
    let program = mori_core::parse(source).map_err(|e| error("program", e.to_string()))?;
    let artifact = compile(&program, target)?;
    let mut selected = artifact.circuits.iter().filter(|c| c.action == action);
    let circuit = selected
        .next()
        .ok_or_else(|| error("action", "Selected action is absent."))?;
    if selected.next().is_some() {
        return Err(error("action", "Selected action is ambiguous."));
    }
    let registry = crate::liabilities::registry(&program)?;
    liability_slots(&registry, &scenario.liabilities)?;
    let execution = mori_core::simulate(&program, action, scenario)
        .map_err(|e| error("scenario", e.to_string()))?;
    let values = public_values(&program, circuit, scenario, &execution)?;
    let ir = IrSource::load(circuit.zkir.as_bytes()).map_err(|e| error("zkir", e.to_string()))?;
    if ir.do_communications_commitment || !ir.inputs.is_empty() {
        return Err(error(
            "unsupported",
            "Only the current public, communications-disabled ABI is supported.",
        ));
    }
    let k = ir.k();
    if k == 0 || k > options.max_k || k > MAX_K {
        return Err(error(
            "limit",
            format!(
                "Compiled circuit k={k} exceeds the explicit native k cap {}.",
                options.max_k
            ),
        ));
    }
    // The pinned embedded verifier SRS has degree 14. Circuit degree may be
    // larger, but its public statement must fit that verifier's input capacity.
    if values.len() + 1 > (1usize << VERIFIER_MAX_DEGREE) {
        return Err(error(
            "limit",
            "Public statement exceeds the pinned verifier capacity.",
        ));
    }
    let claim = Claim {
        abi: PUBLIC_ABI_VERSION.into(),
        relation: RELATION_VERSION.into(),
        binding: BINDING.into(),
        profile: target.profile.clone(),
        registry,
        format: FORMAT.into(),
        source_hash: hex(&Sha256::digest(source.as_bytes())),
        program_hash: artifact.program_hash,
        target_hash: hex(&digest(target)?),
        scenario_hash: hex(&digest(scenario)?),
        execution_hash: hex(&digest(&execution)?),
        action: action.into(),
        circuit_k: k,
    };
    // SHA-256 is interpreted little-endian as low128 + 2^128 * high128,
    // reduced modulo the pinned Fr by field arithmetic. This is not truncation.
    // binding_input binds this host-reconstructed claim; it does not prove an
    // in-circuit hash of the entire local report.
    let hash = digest(&(BINDING, &claim))?;
    let low = u128::from_le_bytes(
        hash[..16]
            .try_into()
            .map_err(|_| error("serialization", "Invalid digest limb."))?,
    );
    let high = u128::from_le_bytes(
        hash[16..]
            .try_into()
            .map_err(|_| error("serialization", "Invalid digest limb."))?,
    );
    let binding = Fr::from(low) + (Fr::from(u128::MAX) + Fr::from(1u64)) * Fr::from(high);
    // The exported report must itself remain admissible to the public bounded
    // JSON reader; validate it before native key generation or proving.
    // Reserve the largest admitted decimal proof count in the actual escaped
    // receipt so a size refusal cannot first appear after expensive proving.
    let report = receipt(execution.clone(), k, MAX_PROOF_BYTES as u32);
    validate_json_profile(&report)?;
    Ok(Prepared {
        claim,
        execution,
        ir,
        values,
        binding,
    })
}

fn validate_json_profile(value: &impl Serialize) -> Result<(), Error> {
    mori_core::json::export(value).map_err(|e| {
        error(
            "limit",
            format!("Export exceeds the bounded JSON profile: {e}"),
        )
    })?;
    Ok(())
}
fn receipt(execution: Execution, circuit_k: u8, proof_bytes: u32) -> ProofReceipt {
    ProofReceipt {
        qualification: "local-native-transition-proof".into(),
        execution,
        circuit_k,
        proof_bytes,
        open_gates: [
            "adversarial-source-to-constraint-soundness",
            "authenticated-ledger-frame-and-phase-correspondence",
            "signed-intent-and-predecessor-history-verification",
            "recursive-private-history",
            "fraud-proof-cost-and-k-measurement",
            "live-ledger-deployment",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
    }
}
fn statement(prepared: &Prepared) -> Vec<Fr> {
    std::iter::once(prepared.binding)
        .chain(prepared.values.iter().copied())
        .collect()
}
pub fn prove(
    source: &str,
    action: &str,
    scenario: &Scenario,
    target: &Target,
    options: &ProofOptions,
) -> Result<(ProofBundle, ProofReceipt), Error> {
    let prepared = prepare(source, action, scenario, target, options)?;
    let params = OfflineParameters::load(&options.params_dir, prepared.claim.circuit_k)?;
    let expected = statement(&prepared);
    let preimage = ProofPreimage {
        inputs: vec![],
        private_transcript: vec![],
        public_transcript_inputs: prepared.values.clone(),
        public_transcript_outputs: prepared.values.clone(),
        binding_input: prepared.binding,
        communications_commitment: None,
        key_location: KeyLocation("moriarty-local-native-transition/2".into()),
    };
    let proof = futures::executor::block_on(async {
        let (pk, vk) = prepared
            .ir
            .keygen(&params)
            .await
            .map_err(|e| error("keygen", e.to_string()))?;
        let (proof, actual, skips) = prepared
            .ir
            .prove(rand::rngs::OsRng, &params, pk, &preimage)
            .await
            .map_err(|e| error("proving", e.to_string()))?;
        if actual != expected
            || skips.len() != prepared.values.len()
            || skips.iter().any(Option::is_some)
        {
            return Err(error(
                "statement",
                "Native producer returned a different or skipped public statement.",
            ));
        }
        if proof.0.is_empty() || proof.0.len() > MAX_PROOF_BYTES {
            return Err(error(
                "limit",
                "Native proof exceeds the nonempty 262144-byte cap.",
            ));
        }
        vk.verify(&PARAMS_VERIFIER, &proof, expected.into_iter())
            .map_err(|e| error("verification", e.to_string()))?;
        Ok(proof)
    })?;
    let count = proof.0.len() as u32;
    let claim = prepared.claim;
    let bundle = ProofBundle {
        abi: claim.abi,
        relation: claim.relation,
        binding: claim.binding,
        profile: claim.profile,
        registry: claim.registry,
        format: claim.format,
        source_hash: claim.source_hash,
        program_hash: claim.program_hash,
        target_hash: claim.target_hash,
        scenario_hash: claim.scenario_hash,
        execution_hash: claim.execution_hash,
        action: claim.action,
        circuit_k: claim.circuit_k,
        proof_hex: hex(&proof.0),
    };
    let report = receipt(prepared.execution, bundle.circuit_k, count);
    validate_json_profile(&bundle)?;
    validate_json_profile(&report)?;
    Ok((bundle, report))
}
fn decode_proof(text: &str) -> Result<Proof, Error> {
    if text.is_empty() || text.len() > MAX_PROOF_BYTES * 2 || text.len() % 2 != 0 {
        return Err(error(
            "proof",
            "Expected nonempty bounded even-length proof hex.",
        ));
    }
    fn nibble(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            _ => None,
        }
    }
    let mut bytes = Vec::with_capacity(text.len() / 2);
    for pair in text.as_bytes().chunks_exact(2) {
        let hi = nibble(pair[0])
            .ok_or_else(|| error("proof", "Proof hex must use canonical lowercase digits."))?;
        let lo = nibble(pair[1])
            .ok_or_else(|| error("proof", "Proof hex must use canonical lowercase digits."))?;
        bytes.push((hi << 4) | lo);
    }
    Ok(Proof(bytes))
}
pub fn verify(
    source: &str,
    action: &str,
    scenario: &Scenario,
    target: &Target,
    bundle: &ProofBundle,
    options: &ProofOptions,
) -> Result<ProofReceipt, Error> {
    bundle.validate_versions()?;
    let prepared = prepare(source, action, scenario, target, options)?;
    let c = &prepared.claim;
    if bundle.abi != c.abi
        || bundle.relation != c.relation
        || bundle.binding != c.binding
        || bundle.profile != c.profile
        || bundle.registry != c.registry
        || bundle.format != c.format
        || bundle.source_hash != c.source_hash
        || bundle.program_hash != c.program_hash
        || bundle.target_hash != c.target_hash
        || bundle.scenario_hash != c.scenario_hash
        || bundle.execution_hash != c.execution_hash
        || bundle.action != c.action
        || bundle.circuit_k != c.circuit_k
    {
        return Err(error(
            "claim",
            "Proof metadata differs from the independently reconstructed local transition.",
        ));
    }
    let proof = decode_proof(&bundle.proof_hex)?;
    let params = OfflineParameters::load(&options.params_dir, c.circuit_k)?;
    let expected = statement(&prepared);
    futures::executor::block_on(async {
        let vk = prepared
            .ir
            .keygen_vk(&params)
            .await
            .map_err(|e| error("keygen", e.to_string()))?;
        // The pinned real verifier enforces transcript EOF. No exported VK,
        // supplied statement, status flag or metadata is verification authority.
        vk.verify(&PARAMS_VERIFIER, &proof, expected.into_iter())
            .map_err(|e| error("verification", e.to_string()))
    })?;
    let report = receipt(prepared.execution, c.circuit_k, proof.0.len() as u32);
    validate_json_profile(&report)?;
    Ok(report)
}

// Presence follows membership, including amount zero. Every supplied identity must
// match the complete source-derived registry; callers cannot supply a registry.
fn liability_slots(
    registry: &[LiabilityIdentity],
    duties: &[Liability],
) -> Result<Vec<(bool, UInt)>, Error> {
    let mut values = vec![(false, UInt(0)); registry.len()];
    for l in duties {
        let i = registry
            .iter()
            .position(|r| r.name == l.name)
            .ok_or_else(|| error("unsupported", "Duty name is outside the source registry."))?;
        let r = &registry[i];
        if values[i].0 || r.asset != l.asset || r.debtor != l.debtor || r.creditor != l.creditor {
            return Err(error(
                "unsupported",
                "Duplicate duty or immutable duty identity differs from the source registry.",
            ));
        }
        values[i] = (true, l.amount);
    }
    Ok(values)
}

fn public_values(
    program: &Program,
    c: &CircuitArtifact,
    s: &Scenario,
    e: &Execution,
) -> Result<Vec<Fr>, Error> {
    let registry = crate::liabilities::registry(program)?;
    if c.liability_frame.len() != registry.len()
        || c.liability_frame.iter().zip(&registry).any(|(f, r)| {
            f.name != r.name
                || f.asset != r.asset
                || f.debtor != r.debtor
                || f.creditor != r.creditor
        })
    {
        return Err(error(
            "abi",
            "Duty frame differs from the canonical source registry.",
        ));
    }
    let pre = liability_slots(&registry, &e.pre.liabilities)?;
    if pre != liability_slots(&registry, &s.liabilities)? {
        return Err(error(
            "abi",
            "Predecessor duty frame differs from the scenario.",
        ));
    }
    let post = liability_slots(&registry, &e.post.liabilities)?;
    let action = program
        .actions
        .iter()
        .find(|a| a.name == c.action)
        .ok_or_else(|| error("abi", "Missing source action."))?;
    if e.action != c.action
        || e.steps.len() != action.statements.len()
        || c.replay_steps as usize != e.steps.len()
        || e.steps
            .iter()
            .zip(&action.statements)
            .enumerate()
            .any(|(i, (step, stmt))| step.index as usize != i || &step.statement != stmt)
        || e.steps
            .iter()
            .flat_map(|step| step.effects.iter())
            .ne(e.effects.iter())
    {
        return Err(error(
            "abi",
            "Complete ordered steps differ from the source action.",
        ));
    }
    let step_slots = e
        .steps
        .iter()
        .map(|step| liability_slots(&registry, &step.snapshot.liabilities))
        .collect::<Result<Vec<_>, _>>()?;
    let mut previous = &pre;
    for (i, step) in e.steps.iter().enumerate() {
        let current = &step_slots[i];
        let changed = match &step.statement {
            Statement::Liability { name, .. } | Statement::Discharge { name, .. } => {
                Some(name.as_str())
            }
            _ => None,
        };
        for (j, r) in registry.iter().enumerate() {
            if Some(r.name.as_str()) != changed && previous[j] != current[j] {
                return Err(error("abi", "An untouched duty slot changed."));
            }
        }
        previous = current;
    }
    if previous != &post {
        return Err(error("abi", "Closing duties differ from the final step."));
    }
    let mut slots: Vec<Option<Value>> = vec![None; c.public_inputs.len()];
    let set = |slots: &mut Vec<Option<Value>>,
               pos: usize,
               role: &str,
               name: &str,
               asset: Option<&mori_core::AssetId>,
               value: Value|
     -> Result<(), Error> {
        let b = c
            .public_inputs
            .get(pos)
            .ok_or_else(|| error("abi", "Public position is outside the ABI."))?;
        if b.role != role
            || b.name != name
            || b.asset.as_ref() != asset
            || b.ty != value.ty()
            || slots[pos].is_some()
        {
            return Err(error(
                "abi",
                "Trusted ABI descriptor does not match its reconstructed value.",
            ));
        }
        slots[pos] = Some(value);
        Ok(())
    };
    for (pos, b) in c.public_inputs.iter().enumerate() {
        let value = match b.role.as_str() {
            "context" => {
                let value = b
                    .constant
                    .as_ref()
                    .ok_or_else(|| error("abi", "Missing context constant."))?
                    .parse::<u128>()
                    .map_err(|_| error("abi", "Invalid context constant."))?;
                Some(Value::UInt(UInt(value)))
            }
            "pre_state" => Some(
                e.pre
                    .state
                    .get(&b.name)
                    .ok_or_else(|| error("abi", "Missing predecessor state."))?
                    .clone(),
            ),
            "post_state" => Some(
                e.post
                    .state
                    .get(&b.name)
                    .ok_or_else(|| error("abi", "Missing successor state."))?
                    .clone(),
            ),
            "input" => Some(
                s.inputs
                    .get(&b.name)
                    .ok_or_else(|| error("abi", "Missing selected input."))?
                    .clone(),
            ),
            "pre_balance"
            | "post_balance"
            | "effect"
            | "pre_liability_presence"
            | "pre_liability_amount"
            | "post_liability_presence"
            | "post_liability_amount" => None,
            _ => return Err(error("abi", "Unsupported public binding role.")),
        };
        if let Some(value) = value {
            set(&mut slots, pos, &b.role, &b.name, None, value)?;
        }
    }
    for (i, frame) in c.liability_frame.iter().enumerate() {
        for (pos, role, value) in [
            (
                frame.pre_presence_position,
                "pre_liability_presence",
                Value::Bool(pre[i].0),
            ),
            (
                frame.pre_amount_position,
                "pre_liability_amount",
                Value::UInt(pre[i].1),
            ),
            (
                frame.post_presence_position,
                "post_liability_presence",
                Value::Bool(post[i].0),
            ),
            (
                frame.post_amount_position,
                "post_liability_amount",
                Value::UInt(post[i].1),
            ),
        ] {
            set(
                &mut slots,
                pos as usize,
                role,
                &frame.name,
                Some(&frame.asset),
                value,
            )?;
        }
    }
    for frame in &c.balance_frame {
        for (pos, role, snapshot) in [
            (frame.pre_position, "pre_balance", &e.pre),
            (frame.post_position, "post_balance", &e.post),
        ] {
            let binding = c
                .public_inputs
                .get(pos as usize)
                .ok_or_else(|| error("abi", "Balance position is outside the ABI."))?;
            let (asset_name, account): (String, String) = serde_json::from_str(&binding.name)
                .map_err(|_| error("abi", "Invalid balance identity descriptor."))?;
            if account != frame.account || asset_name.is_empty() {
                return Err(error("abi", "Balance descriptor identity differs."));
            }
            let amount = snapshot
                .balances
                .iter()
                .find(|b| b.asset == frame.asset && b.account == frame.account)
                .ok_or_else(|| error("abi", "Missing explicit framed balance."))?
                .amount;
            set(
                &mut slots,
                pos as usize,
                role,
                &binding.name,
                Some(&frame.asset),
                Value::UInt(amount),
            )?;
        }
    }
    if e.effects.len() != c.effect_frame.len()
        || c.effect_frame.iter().map(|d| d.step).ne(e
            .steps
            .iter()
            .filter(|step| !step.effects.is_empty())
            .map(|step| u32::from(step.index)))
    {
        return Err(error(
            "abi",
            "Complete ordered effects differ from the circuit frame.",
        ));
    }
    for (descriptor, effect) in c.effect_frame.iter().zip(&e.effects) {
        let step = e
            .steps
            .get(descriptor.step as usize)
            .ok_or_else(|| error("abi", "Effect step is outside the execution."))?;
        if step.index as u32 != descriptor.step
            || step.effects.as_slice() != std::slice::from_ref(effect)
        {
            return Err(error(
                "abi",
                "Effect order differs from the exact execution step.",
            ));
        }
        let (kind, name, asset, from, to, fields): (
            &str,
            Option<&str>,
            Option<&mori_core::AssetId>,
            Option<&str>,
            Option<&str>,
            Vec<(&str, Value)>,
        ) = match effect {
            Effect::State {
                name,
                before,
                after,
            } => (
                "state",
                Some(name),
                None,
                None,
                None,
                vec![("before", before.clone()), ("after", after.clone())],
            ),
            Effect::Emit { name, value } => (
                "emit",
                Some(name),
                None,
                None,
                None,
                vec![("value", value.clone())],
            ),
            Effect::Transfer {
                asset,
                from,
                to,
                amount,
            } => (
                "transfer",
                None,
                Some(asset),
                Some(from),
                Some(to),
                vec![("amount", Value::UInt(*amount))],
            ),
            Effect::Fee {
                asset,
                from,
                to,
                amount,
            } => (
                "fee",
                None,
                Some(asset),
                Some(from),
                Some(to),
                vec![("amount", Value::UInt(*amount))],
            ),
            Effect::Liability {
                name,
                asset,
                debtor,
                creditor,
                before,
                after,
            } => {
                let i = registry
                    .iter()
                    .position(|r| r.name == *name)
                    .ok_or_else(|| error("abi", "Unknown duty effect."))?;
                let identity = &registry[i];
                let old = if descriptor.step == 0 {
                    pre[i]
                } else {
                    step_slots[descriptor.step as usize - 1][i]
                };
                let new = step_slots[descriptor.step as usize][i];
                let operation = match &step.statement {
                    Statement::Liability {
                        name: source_name,
                        asset: source_asset,
                        debtor: source_debtor,
                        creditor: source_creditor,
                        ..
                    } if source_name == name
                        && source_debtor == debtor
                        && source_creditor == creditor
                        && program
                            .assets
                            .iter()
                            .any(|a| a.name == *source_asset && a.identity == *asset) =>
                    {
                        LiabilityOperation::CreateOrAccrue
                    }
                    Statement::Discharge {
                        name: source_name, ..
                    } if source_name == name => LiabilityOperation::Discharge,
                    _ => {
                        return Err(error(
                            "abi",
                            "Duty effect differs from its exact source operation.",
                        ));
                    }
                };
                let delta = match step.value {
                    Some(Value::UInt(n)) => n.0,
                    _ => return Err(error("abi", "Missing duty step amount.")),
                };
                let expected = match operation {
                    LiabilityOperation::CreateOrAccrue => old.1.0.checked_add(delta),
                    LiabilityOperation::Discharge if old.0 => old.1.0.checked_sub(delta),
                    LiabilityOperation::Discharge => None,
                };
                if descriptor.liability.as_ref() != Some(identity)
                    || descriptor.operation != Some(operation)
                    || identity.asset != *asset
                    || identity.debtor != *debtor
                    || identity.creditor != *creditor
                    || old.1 != *before
                    || new.1 != *after
                    || !new.0
                    || expected != Some(after.0)
                {
                    return Err(error(
                        "abi",
                        "Duty identity, presence or ordered amount differs.",
                    ));
                }
                (
                    "liability",
                    None,
                    None,
                    None,
                    None,
                    vec![
                        ("before_presence", Value::Bool(old.0)),
                        ("before_amount", Value::UInt(*before)),
                        ("after_presence", Value::Bool(new.0)),
                        ("after_amount", Value::UInt(*after)),
                    ],
                )
            }
        };
        if (kind != "liability"
            && (descriptor.liability.is_some() || descriptor.operation.is_some()))
            || descriptor.kind != kind
            || descriptor.name.as_deref() != name
            || descriptor.asset.as_ref() != asset
            || descriptor.from.as_deref() != from
            || descriptor.to.as_deref() != to
            || descriptor.public_positions.len() != fields.len()
        {
            return Err(error(
                "abi",
                "Effect identity differs from the trusted circuit frame.",
            ));
        }
        for (pos, (field, value)) in descriptor.public_positions.iter().zip(fields) {
            set(
                &mut slots,
                *pos as usize,
                "effect",
                &format!("{}.{}", descriptor.step, field),
                if kind == "liability" {
                    descriptor.liability.as_ref().map(|r| &r.asset)
                } else {
                    asset
                },
                value,
            )?;
        }
    }
    // Compile constrains the touched frame; host reconstruction must preserve
    // every balance outside it. All complete snapshots remain in execution_hash.
    for initial in &e.pre.balances {
        if !c
            .balance_frame
            .iter()
            .any(|f| f.asset == initial.asset && f.account == initial.account)
        {
            for snapshot in
                std::iter::once(&e.post).chain(e.steps.iter().map(|step| &step.snapshot))
            {
                if !snapshot.balances.contains(initial) {
                    return Err(error("abi", "An untouched balance changed."));
                }
            }
        }
    }
    slots
        .into_iter()
        .map(|value| {
            value
                .map(|v| value_fr(&v))
                .ok_or_else(|| error("abi", "An ABI position was not reconstructed."))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mori_core::{AssetId, Balance};
    use std::collections::BTreeMap;

    // Ordinary interpreter check of the production reconstruction; no SRS,
    // key generation, proof, or native verifier execution.
    fn assert_honest_preimage(prepared: &Prepared) {
        ProofPreimage {
            inputs: vec![],
            private_transcript: vec![],
            public_transcript_inputs: prepared.values.clone(),
            public_transcript_outputs: prepared.values.clone(),
            binding_input: prepared.binding,
            communications_commitment: None,
            key_location: KeyLocation("ordinary-consumer-check-unqualified".into()),
        }
        .check(&prepared.ir)
        .unwrap();
    }

    #[test]
    fn proof_hex_is_bounded_and_canonical_before_binary_verification() {
        for malformed in [
            "".to_owned(),
            "0".to_owned(),
            "AA".to_owned(),
            "gg".to_owned(),
            "🦀".to_owned(),
            "00".repeat(MAX_PROOF_BYTES + 1),
        ] {
            assert_eq!(
                decode_proof(&malformed).unwrap_err().code,
                "mori::backend::proof"
            );
        }
        assert_eq!(decode_proof("00ff81").unwrap().0, vec![0, 255, 129]);
    }
    #[test]
    fn financial_abi_has_exact_order_and_complete_nonframe_commitment() {
        let source = include_str!("../tests/programs/payment.mori");
        let target = Target::ledger9("local", "anchor");
        let asset = AssetId {
            network: "local".into(),
            issuer: "issuer".into(),
            reference: "usd".into(),
            scale: 2,
        };
        let scenario = Scenario {
            inputs: BTreeMap::from([
                ("amount".into(), Value::UInt(UInt(5))),
                ("accepted".into(), Value::Bool(true)),
            ]),
            balances: [
                ("payer", 20),
                ("recipient", 1),
                ("sponsor", 2),
                ("observer", 7),
            ]
            .into_iter()
            .map(|(account, n)| Balance {
                account: account.into(),
                asset: asset.clone(),
                amount: UInt(n),
            })
            .collect(),
            ..Default::default()
        };
        let options = ProofOptions {
            params_dir: PathBuf::from("/unused"),
            max_k: 19,
        };
        let prepared = prepare(source, "pay", &scenario, &target, &options).unwrap();
        let expected: Vec<Fr> = [0u128, 5, 1, 20, 1, 2, 5, 2, 0, 5, 5, 5, 13, 6, 4]
            .into_iter()
            .map(Fr::from)
            .collect();
        assert_eq!(&prepared.values[2..], expected.as_slice());
        assert_eq!(
            prepared
                .execution
                .pre
                .balances
                .iter()
                .find(|b| b.account == "observer"),
            prepared
                .execution
                .post
                .balances
                .iter()
                .find(|b| b.account == "observer")
        );
        let mut different = scenario.clone();
        different.balances[3].amount = UInt(8);
        let other = prepare(source, "pay", &different, &target, &options).unwrap();
        assert_eq!(
            prepared.values, other.values,
            "untouched account is outside the declared circuit frame"
        );
        assert_ne!(prepared.claim.execution_hash, other.claim.execution_hash);
        assert_ne!(
            prepared.binding, other.binding,
            "complete host report remains proof-bound"
        );
    }
    #[test]
    fn digest_reduction_uses_high_limb_and_preserves_separate_source_identity() {
        let source = r#"language "moriarty/1"; agreement A { action run() { } }"#;
        let target = Target::ledger9("local", "anchor");
        let options = ProofOptions {
            params_dir: PathBuf::from("/unused"),
            max_k: 9,
        };
        let first = prepare(source, "run", &Scenario::default(), &target, &options).unwrap();
        let second = prepare(
            &format!("{source}\n"),
            "run",
            &Scenario::default(),
            &target,
            &options,
        )
        .unwrap();
        assert_eq!(first.claim.program_hash, second.claim.program_hash);
        assert_ne!(first.claim.source_hash, second.claim.source_hash);
        assert_ne!(first.binding, second.binding);
        let bytes = digest(&(BINDING, &first.claim)).unwrap();
        let low = Fr::from(u128::from_le_bytes(bytes[..16].try_into().unwrap()));
        let high = Fr::from(u128::from_le_bytes(bytes[16..].try_into().unwrap()));
        assert_eq!(
            first.binding,
            low + (Fr::from(u128::MAX) + Fr::from(1u64)) * high
        );
        assert_ne!(low, first.binding);
    }
    #[test]
    fn zero_duty_slots_are_membership_based_and_mutations_refuse() {
        let source = include_str!("../../../examples/core/native-liability/zero-lifecycle.mori");
        let s: Scenario = serde_json::from_str(include_str!(
            "../../../examples/core/native-liability/present-zero.scenario.json"
        ))
        .unwrap();
        let program = mori_core::parse(source).unwrap();
        let target = Target::ledger9("local", "anchor");
        let artifact = compile(&program, &target).unwrap();
        let c = &artifact.circuits[0];
        let e = mori_core::simulate(&program, "close", &s).unwrap();
        let values = public_values(&program, c, &s, &e).unwrap();
        let f = &c.liability_frame[0];
        for (pos, n) in [
            (f.pre_presence_position, 0u128),
            (f.pre_amount_position, 0),
            (f.post_presence_position, 1),
            (f.post_amount_position, 0),
        ] {
            assert_eq!(values[pos as usize], Fr::from(n));
        }
        let expected = [[0u128, 0, 1, 0], [1, 0, 1, 0], [1, 0, 1, 2], [1, 2, 1, 0]];
        for (d, want) in c
            .effect_frame
            .iter()
            .filter(|d| d.kind == "liability")
            .zip(expected)
        {
            assert_eq!(
                d.public_positions
                    .iter()
                    .map(|p| values[*p as usize])
                    .collect::<Vec<_>>(),
                want.map(Fr::from)
            );
        }
        for i in 0..e.steps.len() {
            let mut bad = e.clone();
            bad.steps[i]
                .snapshot
                .liabilities
                .push(mori_core::Liability {
                    name: "shadow".into(),
                    asset: f.asset.clone(),
                    debtor: "writer".into(),
                    creditor: "holder".into(),
                    amount: UInt(0),
                });
            assert!(public_values(&program, c, &s, &bad).is_err());
        }
        let mut bad = c.clone();
        bad.liability_frame[0].post_presence_position = f.pre_presence_position;
        assert!(public_values(&program, &bad, &s, &e).is_err());
        bad = c.clone();
        bad.liability_frame[0].pre_amount_position = u32::MAX;
        assert!(public_values(&program, &bad, &s, &e).is_err());
    }

    #[test]
    fn complete_settlement_slots_steps_and_claim_commitments_are_exact() {
        let source = include_str!("../../../examples/core/native-liability/settlement.mori");
        let scenario: Scenario = serde_json::from_str(include_str!(
            "../../../examples/core/native-liability/partial.scenario.json"
        ))
        .unwrap();
        let program = mori_core::parse(source).unwrap();
        let target = Target::ledger9("local", "anchor");
        let options = ProofOptions {
            params_dir: "/not-present/task2".into(),
            max_k: 19,
        };
        let artifact = compile(&program, &target).unwrap();
        let c = &artifact.circuits[1];
        let prepared = prepare(source, "repay", &scenario, &target, &options).unwrap();
        assert_honest_preimage(&prepared);
        assert_eq!(prepared.claim.abi, PUBLIC_ABI_VERSION);
        assert_eq!(prepared.claim.relation, RELATION_VERSION);
        assert_eq!(prepared.claim.binding, BINDING);
        assert_eq!(prepared.claim.profile, target.profile);
        assert_eq!(
            prepared.claim.registry,
            crate::liabilities::registry(&program).unwrap()
        );
        assert_eq!(
            prepared.claim.execution_hash,
            hex(&digest(&prepared.execution).unwrap())
        );
        for (f, expected) in c
            .liability_frame
            .iter()
            .zip([[1u128, 10, 1, 3], [1, 0, 1, 0]])
        {
            let positions = [
                f.pre_presence_position,
                f.pre_amount_position,
                f.post_presence_position,
                f.post_amount_position,
            ];
            assert_eq!(
                positions.map(|p| prepared.values[p as usize]),
                expected.map(Fr::from)
            );
        }
        let duty = c
            .effect_frame
            .iter()
            .find(|d| d.kind == "liability")
            .unwrap();
        assert_eq!(duty.step, 3);
        assert_eq!(
            duty.public_positions
                .iter()
                .map(|p| prepared.values[*p as usize])
                .collect::<Vec<_>>(),
            [1u128, 10, 1, 3].map(Fr::from)
        );
        for (i, step) in prepared.execution.steps.iter().enumerate() {
            let payment = step
                .snapshot
                .liabilities
                .iter()
                .find(|l| l.name == "payment")
                .unwrap();
            assert_eq!(payment.amount, UInt(if i < 3 { 10 } else { 3 }));
            assert_eq!(
                step.snapshot
                    .liabilities
                    .iter()
                    .find(|l| l.name == "reserve")
                    .unwrap()
                    .amount,
                UInt(0)
            );
        }
        let mut json = serde_json::to_value(&prepared.claim).unwrap();
        json["proof_hex"] = "00".into();
        let good: ProofBundle = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(
            verify(source, "repay", &scenario, &target, &good, &options)
                .unwrap_err()
                .code,
            "mori::backend::parameters"
        );
        for field in [
            "abi",
            "relation",
            "binding",
            "profile",
            "source_hash",
            "program_hash",
            "target_hash",
            "scenario_hash",
            "execution_hash",
            "action",
        ] {
            let mut bad = json.clone();
            bad[field] = "changed".into();
            let bundle: ProofBundle = serde_json::from_value(bad).unwrap();
            assert_eq!(
                verify(source, "repay", &scenario, &target, &bundle, &options)
                    .unwrap_err()
                    .code,
                "mori::backend::claim",
                "{field}"
            );
        }
        for field in ["name", "debtor", "creditor"] {
            let mut bad = json.clone();
            bad["registry"][0][field] = "changed".into();
            let bundle: ProofBundle = serde_json::from_value(bad).unwrap();
            assert_eq!(
                verify(source, "repay", &scenario, &target, &bundle, &options)
                    .unwrap_err()
                    .code,
                "mori::backend::claim"
            );
        }
        for (field, value) in [
            ("network", serde_json::json!("changed")),
            ("issuer", serde_json::json!("changed")),
            ("reference", serde_json::json!("changed")),
            ("scale", serde_json::json!(1)),
        ] {
            let mut bad = json.clone();
            bad["registry"][0]["asset"][field] = value;
            let bundle: ProofBundle = serde_json::from_value(bad).unwrap();
            assert_eq!(
                verify(source, "repay", &scenario, &target, &bundle, &options)
                    .unwrap_err()
                    .code,
                "mori::backend::claim"
            );
        }
        for field in ["circuit_k", "registry"] {
            let mut bad = json.clone();
            bad[field] = if field == "registry" {
                serde_json::json!([])
            } else {
                serde_json::json!(1)
            };
            let bundle: ProofBundle = serde_json::from_value(bad).unwrap();
            assert_eq!(
                verify(source, "repay", &scenario, &target, &bundle, &options)
                    .unwrap_err()
                    .code,
                "mori::backend::claim"
            );
        }
        let mut old = good;
        old.format = "moriarty-native-transition-proof/1".into();
        assert_eq!(
            verify(
                "invalid source",
                "missing",
                &Scenario::default(),
                &target,
                &old,
                &options
            )
            .unwrap_err()
            .code,
            "mori::backend::claim"
        );
    }

    #[test]
    fn duty_reconstruction_refuses_identity_presence_order_and_descriptor_mutations() {
        let source = include_str!("../../../examples/core/native-liability/zero-lifecycle.mori");
        let scenario: Scenario = serde_json::from_str(include_str!(
            "../../../examples/core/native-liability/present-zero.scenario.json"
        ))
        .unwrap();
        let program = mori_core::parse(source).unwrap();
        let artifact = compile(&program, &Target::ledger9("local", "anchor")).unwrap();
        let c = &artifact.circuits[0];
        let e = mori_core::simulate(&program, "close", &scenario).unwrap();
        for snapshot_index in 0..e.steps.len() + 2 {
            for mutation in 0..9 {
                let mut bad = e.clone();
                let snapshot = if snapshot_index == 0 {
                    &mut bad.pre
                } else if snapshot_index == 1 {
                    &mut bad.post
                } else {
                    &mut bad.steps[snapshot_index - 2].snapshot
                };
                // Pre and guard snapshots are absent: inject a registered present-zero
                // to ensure that a zero amount cannot disguise a membership mutation.
                if snapshot.liabilities.is_empty() {
                    snapshot.liabilities.push(e.post.liabilities[0].clone());
                }
                let l = &mut snapshot.liabilities[0];
                match mutation {
                    0 => l.name = "unknown".into(),
                    1 => l.debtor = "other".into(),
                    2 => l.creditor = "other".into(),
                    3 => l.asset.network = "other".into(),
                    4 => l.asset.issuer = "other".into(),
                    5 => l.asset.reference = "other".into(),
                    6 => l.asset.scale = 1,
                    7 => {
                        let copy = l.clone();
                        snapshot.liabilities.push(copy);
                    }
                    _ => snapshot.liabilities.clear(),
                }
                // Clearing an originally absent snapshot is the original valid case.
                let originally_absent = if snapshot_index == 0 {
                    true
                } else if snapshot_index == 1 {
                    false
                } else {
                    e.steps[snapshot_index - 2].snapshot.liabilities.is_empty()
                };
                if mutation == 8 && originally_absent {
                    continue;
                }
                assert!(
                    public_values(&program, c, &scenario, &bad).is_err(),
                    "snapshot={snapshot_index} mutation={mutation}"
                );
            }
        }
        for mutation in 0..7 {
            let mut bad = c.clone();
            match mutation {
                0 => bad.liability_frame.push(c.liability_frame[0].clone()),
                1 => bad.liability_frame[0].creditor = "other".into(),
                2 => bad.effect_frame[0].operation = Some(LiabilityOperation::Discharge),
                3 => bad.effect_frame[1].operation = Some(LiabilityOperation::CreateOrAccrue),
                4 => bad.effect_frame[0].public_positions.swap(0, 2),
                5 => bad.effect_frame[0].step = 2,
                _ => bad.effect_frame.swap(0, 1),
            }
            assert!(
                public_values(&program, &bad, &scenario, &e).is_err(),
                "descriptor={mutation}"
            );
        }
        let mut bad = e.clone();
        bad.effects.swap(0, 1);
        // Identical zero-valued effects are distinguished by their source operation
        // and ordered snapshots; alter their exact step statements as well.
        bad.steps.swap(1, 2);
        assert!(public_values(&program, c, &scenario, &bad).is_err());
        for i in [1usize, 2, 3, 4] {
            let mut bad = e.clone();
            bad.steps[i].value = Some(Value::UInt(UInt(7)));
            assert!(public_values(&program, c, &scenario, &bad).is_err());
        }
        let mut bad = e.clone();
        bad.post.liabilities[0].amount = UInt(1);
        assert!(public_values(&program, c, &scenario, &bad).is_err());
        let mut registered = scenario.clone();
        registered.liabilities = e.post.liabilities.clone();
        let present = prepare(
            source,
            "close",
            &registered,
            &Target::ledger9("local", "anchor"),
            &ProofOptions {
                params_dir: "/unused".into(),
                max_k: 19,
            },
        )
        .unwrap();
        let absent = prepare(
            source,
            "close",
            &scenario,
            &Target::ledger9("local", "anchor"),
            &ProofOptions {
                params_dir: "/unused".into(),
                max_k: 19,
            },
        )
        .unwrap();
        assert_honest_preimage(&present);
        assert_honest_preimage(&absent);
        assert_ne!(present.values, absent.values);
        assert_ne!(present.binding, absent.binding);
    }
}
