//! Compilation through the actual frozen compiler and the descriptor of every identity a
//! compiled SettlementHashV2 relation binds.
//!
//! Nothing here builds constraints or identities by hand. `compile_selected` calls
//! `mori_r1cs::compile_source::<Fr>` (Core `language` routing, `parse`, `validate`,
//! `require_core1`, plan, layout, probe synthesis). A `moriarty/2` source is admitted by Core's
//! stage loader and refused by the compiler with `StagedNotLowered`; the driver surfaces that
//! refusal unchanged.
use crate::error::DriverError;
use crate::hexutil;
use crate::input::{RelationSelector, SELECTOR_FORMAT};
use ark_bls12_381::Fr;
use ark_ff::{BigInteger, PrimeField};
use ark_relations::r1cs::ConstraintMatrices;
use mori_r1cs::abi2::{FAMILY_V2, HEADER_LEN, STMT_TAG};
use mori_r1cs::identity::{self, Digest32};
use mori_r1cs::{AccountingBinding, CapBinding, CompileOptions, CompiledRelation, CompilerBuild, FrameCells, StatementAdapter};
use serde::{Deserialize, Serialize};

pub const LANGUAGE: &str = "moriarty/1";
pub const SEMANTICS: &str = "moriarty-core/1";
pub const CAP: &str = "NotBound";
pub const ACCOUNTING: &str = "GrossDebitAndFees";
pub const FRAME: &str = "TouchedOnly";
/// SettlementHashV2 public inputs: the two 128-bit digest halves.
pub const PUBLIC_INPUTS: u8 = 2;
/// arkworks counts the constant-one instance variable: 1 + 2.
pub const INSTANCE_VARIABLES: u64 = 3;

/// Refuses every selector value outside the one supported profile. Runs before compilation.
pub fn check_selector(s: &RelationSelector) -> Result<(), DriverError> {
    let fixed = [
        ("format", s.format.as_str(), SELECTOR_FORMAT),
        ("language", s.language.as_str(), LANGUAGE),
        ("semantics", s.semantics.as_str(), SEMANTICS),
        ("adapter", s.adapter.as_str(), crate::ADAPTER),
        ("cap", s.cap.as_str(), CAP),
        ("accounting", s.accounting.as_str(), ACCOUNTING),
        ("frame", s.frame.as_str(), FRAME),
    ];
    for (what, got, want) in fixed {
        if got != want {
            return Err(DriverError::Unsupported(format!("selector {what} {got:?}; this driver supports only {want:?}")));
        }
    }
    if s.action.is_empty() || s.action.len() > 1024 {
        return Err(DriverError::Input("selector action must contain 1..1024 bytes".into()));
    }
    Ok(())
}

/// The selector that recompiles the relation a descriptor names (verify with `--source`).
pub fn selector_for(desc: &RelationDescriptor) -> RelationSelector {
    RelationSelector {
        format: SELECTOR_FORMAT.into(),
        language: desc.language.clone(),
        semantics: desc.semantics.clone(),
        action: desc.action.clone(),
        adapter: desc.adapter.clone(),
        cap: CAP.into(),
        accounting: ACCOUNTING.into(),
        frame: FRAME.into(),
        expected_source_sha256: desc.source_sha256.clone(),
        expected_program_hash: Some(desc.program_hash.clone()),
    }
}

/// Every identity of one compiled SettlementHashV2 relation, as lowercase hex or decimal.
/// Built only by [`RelationDescriptor::of`] from an actual compilation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RelationDescriptor {
    pub relation_format: String,
    pub relation_version: u64,
    pub language: String,
    pub semantics: String,
    pub logical_cost_profile: String,
    pub profile_digest: String,
    pub program_hash: String,
    /// Raw source bytes; declared provenance, never bound in-circuit.
    pub source_sha256: Option<String>,
    pub action: String,
    pub action_index: u16,
    /// mori-r1cs `mori-r1cs-shape/1` digest (SHA-256), a header constant inside the predicate.
    pub relation_shape_digest: String,
    pub compiler_source_manifest_sha256: String,
    pub core_closure_sha256: String,
    pub compiler_build_digest: String,
    pub adapter: String,
    pub logical_work: String,
    /// `RelationIdentity::digest()`; statement bytes 48..80 and the catalogue entry.
    pub predicate_digest: String,
    /// BLAKE2b-256 of the `MRG16-PVC-SHP-02` bytes; statement bytes 80..112.
    pub abi2_shape_digest: String,
    pub abi2_shape_bytes_hex: String,
    pub body_len: u32,
    pub statement_tag_hex: String,
    pub family_id_hex: String,
    pub header_len: u32,
    pub field_modulus_le_hex: String,
    /// `mori-r1cs-matrices/1` digest of the probe synthesis.
    pub r1cs_sha256: String,
    pub constraints: u64,
    pub instance_variables: u64,
    pub witness_variables: u64,
    pub public_input_count: u8,
}

fn count(n: usize, what: &str) -> Result<u64, DriverError> {
    u64::try_from(n).map_err(|_| DriverError::Backend(format!("{what} exceeds u64")))
}

impl RelationDescriptor {
    /// Reads every identity from the compiled relation. Refuses a relation that is not
    /// SettlementHashV2 or whose predicate constant differs from its identity digest.
    pub fn of(relation: &CompiledRelation) -> Result<Self, DriverError> {
        if relation.adapter() != StatementAdapter::SettlementHashV2 {
            return Err(DriverError::Unsupported("relation adapter is not SettlementHashV2".into()));
        }
        let consts = relation
            .abi2()
            .ok_or_else(|| DriverError::Backend("SettlementHashV2 relation without ABI v2 constants".into()))?;
        let id = relation.identity();
        let predicate = id.digest().map_err(DriverError::Compile)?;
        if predicate.0 != consts.predicate_digest {
            return Err(DriverError::Backend("ABI v2 predicate constant differs from RelationIdentity::digest()".into()));
        }
        let probe = relation.probe();
        let header_len = u32::try_from(HEADER_LEN).map_err(|_| DriverError::Backend("header length".into()))?;
        Ok(Self {
            relation_format: id.relation_format.clone(),
            relation_version: id.relation_version,
            language: id.language.clone(),
            semantics: id.semantics.clone(),
            logical_cost_profile: id.logical_cost_profile.clone(),
            profile_digest: id.profile_digest.hex(),
            program_hash: id.program_hash.hex(),
            source_sha256: id.source_sha256.map(|d| d.hex()),
            action: id.action.clone(),
            action_index: id.action_index,
            relation_shape_digest: id.shape_digest.hex(),
            compiler_source_manifest_sha256: id.compiler_build.compiler_source_manifest_sha256.hex(),
            core_closure_sha256: id.compiler_build.core_closure_sha256.hex(),
            compiler_build_digest: id.compiler_build_digest.hex(),
            adapter: crate::ADAPTER.into(),
            logical_work: id.logical_work.0.to_string(),
            predicate_digest: hexutil::encode(&consts.predicate_digest),
            abi2_shape_digest: hexutil::encode(&consts.shape_digest),
            abi2_shape_bytes_hex: hexutil::encode(&consts.shape_bytes),
            body_len: consts.body_len,
            statement_tag_hex: hexutil::encode(STMT_TAG),
            family_id_hex: hexutil::encode(FAMILY_V2),
            header_len,
            field_modulus_le_hex: probe.field_modulus_le_hex.clone(),
            r1cs_sha256: probe.r1cs_sha256.hex(),
            constraints: count(probe.constraints, "constraints")?,
            instance_variables: count(probe.instance_variables, "instance variables")?,
            witness_variables: count(probe.witness_variables, "witness variables")?,
            public_input_count: PUBLIC_INPUTS,
        })
    }

    /// Syntax and profile checks on a descriptor read from a file (no recompilation).
    pub fn check_profile(&self) -> Result<(), DriverError> {
        let fixed = [
            ("language", self.language.as_str(), LANGUAGE),
            ("semantics", self.semantics.as_str(), SEMANTICS),
            ("adapter", self.adapter.as_str(), crate::ADAPTER),
        ];
        for (what, got, want) in fixed {
            if got != want {
                return Err(DriverError::Unsupported(format!("descriptor {what} {got:?}; expected {want:?}")));
            }
        }
        for (what, value) in [
            ("profileDigest", &self.profile_digest),
            ("programHash", &self.program_hash),
            ("relationShapeDigest", &self.relation_shape_digest),
            ("compilerSourceManifestSha256", &self.compiler_source_manifest_sha256),
            ("coreClosureSha256", &self.core_closure_sha256),
            ("compilerBuildDigest", &self.compiler_build_digest),
            ("predicateDigest", &self.predicate_digest),
            ("abi2ShapeDigest", &self.abi2_shape_digest),
            ("r1csSha256", &self.r1cs_sha256),
        ] {
            hexutil::decode_exact::<32>(value, what)?;
        }
        if let Some(s) = &self.source_sha256 {
            hexutil::decode_exact::<32>(s, "sourceSha256")?;
        }
        hexutil::parse_u128_decimal(&self.logical_work, "logicalWork")?;
        if self.statement_tag_hex != hexutil::encode(STMT_TAG) || self.family_id_hex != hexutil::encode(FAMILY_V2) {
            return Err(DriverError::Unsupported("descriptor statement tag or family id is not ABI v2".into()));
        }
        if self.header_len as usize != HEADER_LEN {
            return Err(DriverError::Unsupported("descriptor header length is not 343".into()));
        }
        if self.public_input_count != PUBLIC_INPUTS || self.instance_variables != INSTANCE_VARIABLES {
            return Err(DriverError::Unsupported("SettlementHashV2 has exactly two public inputs".into()));
        }
        // The ABI v2 shape digest is BLAKE2b-256 of the shape bytes the descriptor carries.
        let shape = hexutil::decode_bounded(&self.abi2_shape_bytes_hex, crate::claim::MAX_STATEMENT_BYTES, "abi2ShapeBytesHex")?;
        if hexutil::encode(&mori_r1cs::blake2b::blake2b256(&shape)) != self.abi2_shape_digest {
            return Err(DriverError::mismatch("abi2ShapeDigest", hexutil::encode(&mori_r1cs::blake2b::blake2b256(&shape)), self.abi2_shape_digest.clone()));
        }
        Ok(())
    }
}

/// Name and values of the first descriptor field that differs, if any.
pub fn first_difference(expected: &RelationDescriptor, actual: &RelationDescriptor) -> Option<(String, String, String)> {
    let (Ok(serde_json::Value::Object(a)), Ok(serde_json::Value::Object(b))) = (serde_json::to_value(expected), serde_json::to_value(actual)) else {
        return Some(("descriptor".into(), "serializable".into(), "unserializable".into()));
    };
    for (key, va) in &a {
        let vb = b.get(key).cloned().unwrap_or(serde_json::Value::Null);
        if *va != vb {
            return Some((key.clone(), va.to_string(), vb.to_string()));
        }
    }
    for (key, vb) in &b {
        if !a.contains_key(key) {
            return Some((key.clone(), "null".into(), vb.to_string()));
        }
    }
    None
}

/// Refuses unless `actual` equals `expected` in every field.
pub fn require_same(expected: &RelationDescriptor, actual: &RelationDescriptor) -> Result<(), DriverError> {
    match first_difference(expected, actual) {
        None => Ok(()),
        Some((field, e, a)) => Err(DriverError::mismatch(format!("relation descriptor field {field}"), e, a)),
    }
}

/// A relation compiled by the actual compiler for one selector, with its descriptor.
#[derive(Debug, Clone)]
pub struct SelectedRelation {
    selector: RelationSelector,
    relation: CompiledRelation,
    descriptor: RelationDescriptor,
}

impl SelectedRelation {
    pub fn selector(&self) -> &RelationSelector {
        &self.selector
    }
    pub fn relation(&self) -> &CompiledRelation {
        &self.relation
    }
    pub fn descriptor(&self) -> &RelationDescriptor {
        &self.descriptor
    }
}

/// Compiles `source` for `selector` with the actual compiler, then checks optional pins.
pub fn compile_selected(source: &str, selector: &RelationSelector) -> Result<SelectedRelation, DriverError> {
    check_selector(selector)?;
    let options = CompileOptions {
        action: selector.action.clone(),
        cap: CapBinding::NotBound,
        frame: FrameCells::TouchedOnly,
        accounting: AccountingBinding::GrossDebitAndFees,
        compiler: CompilerBuild::embedded().map_err(DriverError::Compile)?,
    };
    let relation =
        mori_r1cs::compile_source::<Fr>(source, options, StatementAdapter::SettlementHashV2).map_err(DriverError::Compile)?;
    let descriptor = RelationDescriptor::of(&relation)?;
    if descriptor.language != selector.language || descriptor.semantics != selector.semantics {
        return Err(DriverError::mismatch(
            "language/semantics",
            format!("{}/{}", selector.language, selector.semantics),
            format!("{}/{}", descriptor.language, descriptor.semantics),
        ));
    }
    if let Some(pin) = &selector.expected_source_sha256 {
        hexutil::decode_exact::<32>(pin, "expectedSourceSha256")?;
        let actual = descriptor.source_sha256.clone().unwrap_or_default();
        if *pin != actual {
            return Err(DriverError::mismatch("source sha256", pin.clone(), actual));
        }
    }
    if let Some(pin) = &selector.expected_program_hash {
        hexutil::decode_exact::<32>(pin, "expectedProgramHash")?;
        if *pin != descriptor.program_hash {
            return Err(DriverError::mismatch("program hash", pin.clone(), descriptor.program_hash.clone()));
        }
    }
    Ok(SelectedRelation { selector: selector.clone(), relation, descriptor })
}

/// `mori-r1cs-matrices/1`: the exact byte rule of the compiler's probe (`circuit.rs:282-305`),
/// recomputed here because the probe function is private. A divergence makes `prove` refuse;
/// it can never make it accept.
pub fn matrix_digest(m: &ConstraintMatrices<Fr>) -> Digest32 {
    let mut bytes = b"mori-r1cs-matrices/1\0".to_vec();
    for n in [m.num_instance_variables, m.num_witness_variables, m.num_constraints] {
        bytes.extend_from_slice(&(n as u64).to_le_bytes());
    }
    for matrix in [&m.a, &m.b, &m.c] {
        bytes.extend_from_slice(&(matrix.len() as u64).to_le_bytes());
        for row in matrix.iter() {
            bytes.extend_from_slice(&(row.len() as u64).to_le_bytes());
            for (coefficient, index) in row.iter() {
                bytes.extend_from_slice(&(*index as u64).to_le_bytes());
                let le = coefficient.into_bigint().to_bytes_le();
                bytes.extend_from_slice(&(le.len() as u64).to_le_bytes());
                bytes.extend_from_slice(&le);
            }
        }
    }
    identity::digest_bytes(&bytes)
}
