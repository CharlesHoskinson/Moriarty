//! Source-static native-call constraints. Ordinary checking is not a proof.
pub(crate) mod arithmetic;
pub(crate) mod compiler;
mod event;
mod process;
use crate::{
    Error, Result, codec, layout,
    profile::{Deployment, StaticBindings},
};
use midnight_transient_crypto::proofs::ProofPreimage;
use midnight_zkir_v3::IrSource;
use mori_core::Program;
pub use process::{ProcessedOps, process};
use serde::{Deserialize, Serialize};

pub const FORMAT: &str = "moriarty-native-call-relation/1";
fn refusal(code: &'static str) -> Error {
    Error::new("native-relation", code)
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Group {
    pub field_count: usize,
    pub origin: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub format: String,
    pub source_hash: String,
    pub program_hash: String,
    pub action: String,
    pub abi: layout::LedgerAbi,
    pub ir_hash: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub signed: Option<crate::signed::SignedManifest>,
    #[serde(default)]
    pub input_count: usize,
    pub groups: Vec<Group>,
    pub qualification: String,
    pub compiler_hash: String,
    pub open_predicates: Vec<String>,
}
#[derive(Debug, Clone, Serialize)]
pub struct CheckedReadback {
    pub scope: &'static str,
    pub processed_call_use: &'static str,
    pub ir_hash: String,
    pub activity: Vec<Option<usize>>,
    pub raw_op_count: usize,
    pub processed_op_count: usize,
    pub noop_count: usize,
    pub inactive_fields: usize,
    pub noop_cost_hex: String,
    pub processed_measured_gas_hex: String,
    pub processed_declared_gas_hex: String,
    pub parent_binding_commitment_hex: String,
    pub public_inputs: Vec<String>,
}
pub struct Artifact {
    pub ir: IrSource,
    pub manifest: Manifest,
}
impl Artifact {
    pub fn ir_bytes(&self) -> Result<Vec<u8>> {
        codec::encode(&self.ir)
    }
    pub fn manifest_bytes(&self) -> Result<Vec<u8>> {
        let b = if self.manifest.signed.is_some() {
            let mut value =
                serde_json::to_value(&self.manifest).map_err(|_| refusal("manifest-json"))?;
            value
                .as_object_mut()
                .ok_or(refusal("manifest-json"))?
                .remove("input_count");
            serde_json::to_vec_pretty(&value).map_err(|_| refusal("manifest-json"))?
        } else {
            serde_json::to_vec_pretty(&self.manifest).map_err(|_| refusal("manifest-json"))?
        };
        if b.len() > codec::OUTPUT_BYTES {
            return Err(refusal("artifact-size"));
        }
        Ok(b)
    }
}
pub fn compile(source: &str, p: &Program, d: &Deployment, action: &str) -> Result<Artifact> {
    if source.len() > codec::INPUT_BYTES
        || codec::hash(source.as_bytes()) != d.source_hash
        || mori_core::parse(source).map_err(|_| refusal("source-parse"))? != *p
    {
        return Err(refusal("source-identity"));
    }
    layout::admit_source(p, d)?;
    compile_bindings(source, p, &StaticBindings::legacy(d), action)
}
pub(crate) fn compile_bindings(
    source: &str,
    p: &Program,
    d: &StaticBindings,
    action: &str,
) -> Result<Artifact> {
    let validated = layout::validate_static(source, p, d)?;
    let abi = validated.abi().clone();
    let (ir, groups, input_count) = compiler::compile(&validated, action)?;
    let ir_hash = codec::hash(&codec::encode(&ir)?);
    let manifest = Manifest {
        format: FORMAT.into(),
        source_hash: d.source_hash.clone(),
        program_hash: abi.program_hash.clone(),
        action: action.into(),
        abi,
        ir_hash,
        input_count,
        signed: None,
        groups,
        qualification: "ordinary-ir-unproven".into(),
        compiler_hash: codec::hash(
            concat!(
                include_str!("mod.rs"),
                include_str!("compiler.rs"),
                include_str!("arithmetic.rs"),
                include_str!("event.rs"),
                include_str!("process.rs")
            )
            .as_bytes(),
        ),
        open_predicates: [
            "authenticated predecessor and deployed key",
            "payer/account/funding ownership",
            "global prefix balances and custody correspondence",
            "economic intention authority",
            "final intent/history/PCD",
            "cryptographic proof and ledger settlement",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
    };
    let a = Artifact { ir, manifest };
    if a.ir_bytes()?
        .len()
        .checked_add(a.manifest_bytes()?.len())
        .is_none_or(|n| n > codec::OUTPUT_BYTES)
    {
        return Err(refusal("artifact-size"));
    }
    Ok(a)
}
pub fn check(a: &Artifact, p: &ProofPreimage) -> Result<Vec<Option<usize>>> {
    let width = a
        .manifest
        .signed
        .as_ref()
        .map_or(a.manifest.input_count, |m| m.flattened_field_count);
    if !a.ir.do_communications_commitment
        || !a.ir.outputs.is_empty()
        || a.ir
            .instructions
            .iter()
            .filter(|i| matches!(i, midnight_zkir_v3::Instruction::Output { .. }))
            .count()
            != 1
        || a.ir
            .instructions
            .iter()
            .any(|i| matches!(i, midnight_zkir_v3::Instruction::PrivateInput { .. }))
        || p.inputs.len() != width
        || !p.private_transcript.is_empty()
        || p.key_location.0.as_ref() != a.manifest.action
        || p.public_transcript_inputs.len() > codec::OUTPUT_BYTES / 32
        || p.public_transcript_outputs.len() > codec::OUTPUT_BYTES / 32
        || codec::hash(&a.ir_bytes()?) != a.manifest.ir_hash
    {
        return Err(refusal("preimage-domain"));
    }
    let activity = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| p.check(&a.ir)))
        .map_err(|_| refusal("sdk-check-panic"))?
        .map_err(|_| refusal("unsatisfied"))?;
    if activity.len() != a.manifest.groups.len()
        || activity
            .iter()
            .zip(&a.manifest.groups)
            .any(|(s, g)| s.is_some_and(|n| n != g.field_count))
    {
        return Err(refusal("activity-layout"));
    }
    Ok(activity)
}

/// Compare caller-supplied bytes against a fresh independently supplied source
/// and static deployment compile. No caller manifest chooses its own relation.
pub fn admit(
    source: &str,
    p: &Program,
    d: &Deployment,
    action: &str,
    ir: &[u8],
    manifest: &[u8],
) -> Result<Artifact> {
    if ir
        .len()
        .checked_add(manifest.len())
        .is_none_or(|n| n > codec::OUTPUT_BYTES)
    {
        return Err(refusal("artifact-size"));
    }
    let a = compile(source, p, d, action)?;
    if a.ir_bytes()? != ir || a.manifest_bytes()? != manifest {
        return Err(refusal("artifact-identity"));
    }
    Ok(a)
}
