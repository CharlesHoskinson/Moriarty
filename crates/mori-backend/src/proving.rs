//! Public local native transition proofs. The circuit encodes its declared ABI
//! frame. Reconstruction and the binding commitment supply the complete local
//! report framing; neither authenticates ledger state, owner authority or history.
use crate::{
    CircuitArtifact, Error, Target, compile, digest, error, parameters::OfflineParameters,
};
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{KeyLocation, PARAMS_VERIFIER, Proof, ProofPreimage, VERIFIER_MAX_DEGREE, Zkir},
};
use midnight_zkir_v3::IrSource;
use mori_core::{Effect, Execution, Scenario, Snapshot, UInt, Value};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub const MAX_PROOF_BYTES: usize = 262_144;
pub const MAX_K: u8 = 19;
const FORMAT: &str = "moriarty-native-transition-proof/1";
#[derive(Debug, Clone)]
pub struct ProofOptions {
    pub params_dir: PathBuf,
    pub max_k: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofBundle {
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
    pub fn from_json(text: &str) -> Result<Self, Error> {
        let value = mori_core::json::parse(text).map_err(|e| error("json", e.to_string()))?;
        serde_json::from_value(value).map_err(|e| error("json", e.to_string()))
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
    if !scenario.liabilities.is_empty() {
        return Err(error(
            "unsupported",
            "Residual liability presence and preservation are not encoded.",
        ));
    }
    let execution = mori_core::simulate(&program, action, scenario)
        .map_err(|e| error("scenario", e.to_string()))?;
    let values = public_values(circuit, scenario, &execution)?;
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
    let hash = digest(&("moriarty-native-transition-binding/1", &claim))?;
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
    let report = receipt(execution.clone(), k, 0);
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
    let text =
        serde_json::to_string_pretty(value).map_err(|e| error("serialization", e.to_string()))?;
    mori_core::json::parse(&text).map_err(|e| {
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
        key_location: KeyLocation("moriarty-local-native-transition/1".into()),
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
    let prepared = prepare(source, action, scenario, target, options)?;
    let c = &prepared.claim;
    if bundle.format != c.format
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

fn public_values(c: &CircuitArtifact, s: &Scenario, e: &Execution) -> Result<Vec<Fr>, Error> {
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
            "pre_balance" | "post_balance" | "effect" => None,
            _ => return Err(error("abi", "Unsupported public binding role.")),
        };
        if let Some(value) = value {
            set(&mut slots, pos, &b.role, &b.name, None, value)?;
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
    if e.effects.len() != c.effect_frame.len() {
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
            Effect::Liability { .. } => {
                return Err(error(
                    "unsupported",
                    "Residual liabilities are not encoded.",
                ));
            }
        };
        if descriptor.kind != kind
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
                asset,
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
    if [&e.pre, &e.post]
        .into_iter()
        .chain(e.steps.iter().map(|s| &s.snapshot))
        .any(|s: &Snapshot| !s.liabilities.is_empty())
    {
        return Err(error(
            "unsupported",
            "Residual liabilities are not encoded.",
        ));
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
        let bytes = digest(&("moriarty-native-transition-binding/1", &first.claim)).unwrap();
        let low = Fr::from(u128::from_le_bytes(bytes[..16].try_into().unwrap()));
        assert_ne!(low, first.binding);
    }
}
