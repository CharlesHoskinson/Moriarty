//! Key-free source-static native-call export and caller-controlled recompilation.
//! No installed, preparation or proving capability can be constructed here.
use crate::{
    Error, Result,
    call_artifacts::{ArtifactLeaves, Leaf},
    codec,
    profile::{NativeWorkLimits, StaticBindings},
    signed::{FixedSignerPolicy, SignedVersion},
};
use mori_core::Program;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    path::Path,
};

pub const PROFILE: &str = "moriarty-native-call-static-bundle/1";
pub const QUALIFICATION: &str = "source-static-unproven-uninstalled";
pub const MARKER: &str = "static-compile.json";
fn refuse(code: &'static str) -> Error {
    Error::new("static-compile", code)
}
fn without_null(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Null => false,
        serde_json::Value::Array(values) => values.iter().all(without_null),
        serde_json::Value::Object(values) => values.values().all(without_null),
        _ => true,
    }
}
fn strict<T: DeserializeOwned>(text: &str) -> Result<T> {
    let value = mori_core::json::parse(text).map_err(|_| refuse("strict-json"))?;
    if !without_null(&value) {
        return Err(refuse("explicit-null"));
    }
    serde_json::from_value(value).map_err(|_| refuse("public-schema"))
}
/// New text profile rejects explicit null; typed None and legacy DTOs are unchanged.
pub fn parse_static_bindings(text: &str) -> Result<StaticBindings> {
    strict(text)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "mode")]
pub enum StaticAuthorization {
    #[serde(rename = "unsigned")]
    Unsigned,
    #[serde(rename = "signed-v2")]
    SignedV2 { signer_policy: FixedSignerPolicy },
}
impl<'de> Deserialize<'de> for StaticAuthorization {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        use serde::de::Error as _;
        let value = mori_core::json::deserialize_strict(d)?;
        let object = value
            .as_object()
            .ok_or_else(|| D::Error::custom("authorization object"))?;
        if !without_null(&value) {
            return Err(D::Error::custom("explicit null"));
        }
        match object.get("mode").and_then(|m| m.as_str()) {
            Some("unsigned") if object.len() == 1 => Ok(Self::Unsigned),
            Some("signed-v2") if object.len() == 2 && object.contains_key("signer_policy") => {
                let signer_policy = serde_json::from_value(object["signer_policy"].clone())
                    .map_err(D::Error::custom)?;
                Ok(Self::SignedV2 { signer_policy })
            }
            _ => Err(D::Error::custom(
                "explicit unsigned or consuming signed-v2 authorization",
            )),
        }
    }
}
pub fn parse_authorization(text: &str) -> Result<StaticAuthorization> {
    strict(text)
}
pub fn parse_signer_policy(text: &str) -> Result<FixedSignerPolicy> {
    if text.len() > crate::signed::SIGNED_JSON_BYTES {
        return Err(refuse("policy-input-size"));
    }
    let policy: FixedSignerPolicy = strict(text)?;
    policy.validate()?;
    Ok(policy)
}
/// CLI's explicit /2 profile; typed API /1 still obeys hard codec ceilings.
pub fn parse_work_limits(text: &str) -> Result<NativeWorkLimits> {
    let limits: NativeWorkLimits = strict(text)?;
    limits.validate()?;
    if limits.profile != "moriarty-native-call-work-limits/2" {
        return Err(refuse("work-limits-profile"));
    }
    if text.len() > input_cap(&limits)? {
        return Err(refuse("input-work-limit"));
    }
    Ok(limits)
}
fn input_cap(l: &NativeWorkLimits) -> Result<usize> {
    Ok(NativeWorkLimits::cap(l.max_input_bytes)?.min(codec::INPUT_BYTES))
}
fn leaf_cap(l: &NativeWorkLimits) -> Result<usize> {
    Ok(NativeWorkLimits::cap(l.max_leaf_bytes)?.min(codec::OUTPUT_BYTES))
}
fn aggregate_cap(l: &NativeWorkLimits) -> Result<usize> {
    Ok(NativeWorkLimits::cap(l.max_artifact_bytes)?.min(codec::OUTPUT_BYTES))
}
fn marker_cap(l: &NativeWorkLimits) -> Result<usize> {
    Ok(input_cap(l)?.min(leaf_cap(l)?))
}
fn add(total: usize, next: usize, cap: usize) -> Result<usize> {
    total
        .checked_add(next)
        .filter(|n| *n <= cap)
        .ok_or(refuse("aggregate-work-limit"))
}
fn json_size(value: &impl Serialize, cap: usize) -> Result<usize> {
    // Bounded writer avoids allocating an arbitrarily large typed serialization.
    Ok(crate::call_artifacts::json_bytes(
        value,
        u64::try_from(cap).map_err(|_| refuse("input-size"))?,
    )?
    .len())
}
/// Account max(raw file, conservative typed pretty-JSON image) for each input.
/// The bindings size-only image intentionally keeps legacy None -> null.
/// Authorization's raw size includes any separately supplied public policy file.
pub fn check_input_budget(
    source: usize,
    bindings: &StaticBindings,
    authorization: &StaticAuthorization,
    limits: &NativeWorkLimits,
    raw_bindings: usize,
    raw_authorization: usize,
    raw_limits: usize,
) -> Result<usize> {
    limits.validate()?;
    let cap = input_cap(limits)?;
    let sizes = [
        source,
        raw_bindings.max(json_size(bindings, cap)?),
        raw_authorization.max(json_size(authorization, cap)?),
        raw_limits.max(json_size(limits, cap)?),
    ];
    let mut total = 0;
    for size in sizes {
        if size > cap {
            return Err(refuse("input-work-limit"));
        }
        total = add(total, size, aggregate_cap(limits)?)?;
    }
    if let StaticAuthorization::SignedV2 { signer_policy } = authorization {
        if json_size(signer_policy, cap)? > crate::signed::SIGNED_JSON_BYTES {
            return Err(refuse("policy-input-size"));
        }
    }
    Ok(total)
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionRow {
    name: String,
    ordinal: usize,
    ir_leaf: String,
    manifest_leaf: String,
    ir_hash: String,
    manifest_hash: String,
    compiler_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Marker {
    profile: String,
    qualification: String,
    authorization: StaticAuthorization,
    bindings: StaticBindings,
    source_hash: String,
    program_hash: String,
    build: String,
    actions: Vec<ActionRow>,
    leaves: BTreeMap<String, Leaf>,
    identity: String,
}
fn marker_object(marker: &Marker, identity: bool) -> Result<serde_json::Value> {
    let mut object = serde_json::to_value(marker).map_err(|_| refuse("marker-json"))?;
    if marker.bindings.economic_bounds.is_none() {
        object["bindings"]
            .as_object_mut()
            .ok_or(refuse("marker-json"))?
            .remove("economic_bounds");
    }
    if !identity {
        object
            .as_object_mut()
            .ok_or(refuse("marker-json"))?
            .remove("identity");
    }
    Ok(object)
}
fn marker_identity(marker: &Marker) -> Result<String> {
    codec::object_hash(&marker_object(marker, false)?)
}
fn leaf(bytes: &[u8]) -> Result<Leaf> {
    Ok(Leaf {
        sha256: codec::hash(bytes),
        bytes: u64::try_from(bytes.len()).map_err(|_| refuse("leaf-size"))?,
    })
}
fn ordinal(index: usize, suffix: &str) -> String {
    format!("action-{index:06}.{suffix}")
}
fn digest(text: &str) -> bool {
    text.len() == 64
        && text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

pub struct StaticBundle {
    output: ArtifactLeaves,
    marker: Marker,
    total: usize,
}
impl StaticBundle {
    pub fn leaves(&self) -> &ArtifactLeaves {
        &self.output
    }
    pub fn action_names(&self) -> Vec<&str> {
        self.marker
            .actions
            .iter()
            .map(|a| a.name.as_str())
            .collect()
    }
    pub fn identity(&self) -> &str {
        &self.marker.identity
    }
    pub fn total_bytes(&self) -> usize {
        self.total
    }
}
/// Private source provenance; this exposes no installed or native execution API.
pub struct AdmittedStaticBundle {
    bundle: StaticBundle,
}
impl AdmittedStaticBundle {
    pub fn identity(&self) -> &str {
        self.bundle.identity()
    }
    pub fn action_names(&self) -> Vec<&str> {
        self.bundle.action_names()
    }
    pub fn total_bytes(&self) -> usize {
        self.bundle.total_bytes()
    }
    pub fn leaves(&self) -> &ArtifactLeaves {
        self.bundle.leaves()
    }
}
pub fn compile(
    source: &str,
    program: &Program,
    bindings: &StaticBindings,
    authorization: &StaticAuthorization,
    limits: &NativeWorkLimits,
) -> Result<StaticBundle> {
    check_input_budget(source.len(), bindings, authorization, limits, 0, 0, 0)?;
    if program.actions.is_empty()
        || program.actions.len() > NativeWorkLimits::cap(limits.max_actions)?
    {
        return Err(refuse("action-work-limit"));
    }
    mori_core::validate(program).map_err(|_| refuse("core-validation"))?;
    let policy = match authorization {
        StaticAuthorization::Unsigned => None,
        StaticAuthorization::SignedV2 { signer_policy } => {
            Some((signer_policy, SignedVersion::ConsumingV2))
        }
    };
    let artifacts =
        crate::local_deployment::compile_all_policy(source, program, bindings, limits, policy)?;
    let mut leaves = BTreeMap::from([("source.mori".into(), source.as_bytes().to_vec())]);
    let mut rows = Vec::new();
    for (i, artifact) in artifacts.into_iter().enumerate() {
        let ir = artifact.ir_bytes()?;
        let manifest = artifact.manifest_bytes()?;
        let ir_leaf = ordinal(i, "ir.bin");
        let manifest_leaf = ordinal(i, "manifest.json");
        rows.push(ActionRow {
            name: artifact.manifest.action,
            ordinal: i,
            ir_leaf: ir_leaf.clone(),
            manifest_leaf: manifest_leaf.clone(),
            ir_hash: codec::hash(&ir),
            manifest_hash: codec::hash(&manifest),
            compiler_hash: artifact.manifest.compiler_hash,
        });
        leaves.insert(ir_leaf, ir);
        leaves.insert(manifest_leaf, manifest);
    }
    let metadata = leaves
        .iter()
        .map(|(n, b)| Ok((n.clone(), leaf(b)?)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    let mut marker = Marker {
        profile: PROFILE.into(),
        qualification: QUALIFICATION.into(),
        authorization: authorization.clone(),
        bindings: bindings.clone(),
        source_hash: bindings.source_hash.clone(),
        program_hash: bindings.program_hash.clone(),
        build: crate::proving::source_closure(),
        actions: rows,
        leaves: metadata,
        identity: String::new(),
    };
    marker.identity = marker_identity(&marker)?;
    let marker_bytes = crate::call_artifacts::json_bytes(
        &marker_object(&marker, true)?,
        u64::try_from(marker_cap(limits)?).map_err(|_| refuse("marker-size"))?,
    )?;
    // Ensure our new-profile marker fits the same strict reader, including depth/nodes.
    let _: Marker = strict(std::str::from_utf8(&marker_bytes).map_err(|_| refuse("marker-utf8"))?)?;
    let mut total = add(0, marker_bytes.len(), aggregate_cap(limits)?)?;
    for bytes in leaves.values() {
        if bytes.len() > leaf_cap(limits)? {
            return Err(refuse("leaf-work-limit"));
        }
        total = add(total, bytes.len(), aggregate_cap(limits)?)?;
    }
    Ok(StaticBundle {
        output: ArtifactLeaves {
            leaves,
            marker_name: MARKER,
            marker: marker_bytes,
        },
        marker,
        total,
    })
}
/// No-follow handle checked reads. Stable ancestors are required; no filesystem
/// isolation or simultaneous whole-directory snapshot is asserted.
pub fn read_static_input(path: &Path, cap: usize) -> Result<Vec<u8>> {
    read_snapshot(path, cap.min(codec::INPUT_BYTES))
}
#[cfg(unix)]
fn read_snapshot(path: &Path, cap: usize) -> Result<Vec<u8>> {
    use std::os::unix::fs::OpenOptionsExt;
    let before = std::fs::symlink_metadata(path).map_err(|_| refuse("file-metadata"))?;
    if !before.is_file() || before.file_type().is_symlink() {
        return Err(refuse("file-kind"));
    }
    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)
        .map_err(|_| refuse("file-open"))?;
    let metadata = file.metadata().map_err(|_| refuse("file-metadata"))?;
    let maximum = u64::try_from(cap).map_err(|_| refuse("file-size"))?;
    if !metadata.is_file() || metadata.len() > maximum {
        return Err(refuse("file-kind-or-size"));
    }
    let sentinel = maximum.checked_add(1).ok_or(refuse("file-size"))?;
    let mut bytes = Vec::new();
    file.take(sentinel)
        .read_to_end(&mut bytes)
        .map_err(|_| refuse("file-read"))?;
    if bytes.len() > cap {
        return Err(refuse("file-size"));
    }
    Ok(bytes)
}
#[cfg(not(unix))]
fn read_snapshot(_: &Path, _: usize) -> Result<Vec<u8>> {
    Err(refuse("no-follow-host-unsupported"))
}

pub fn reopen(
    source: &str,
    program: &Program,
    bindings: &StaticBindings,
    authorization: &StaticAuthorization,
    directory: &Path,
    limits: &NativeWorkLimits,
) -> Result<AdmittedStaticBundle> {
    // Validate independent trusted inputs before any imported provenance or leaf reads.
    check_input_budget(source.len(), bindings, authorization, limits, 0, 0, 0)?;
    crate::layout::validate_static(source, program, bindings)?;
    let directory_metadata =
        std::fs::symlink_metadata(directory).map_err(|_| refuse("directory-metadata"))?;
    if !directory_metadata.is_dir() || directory_metadata.file_type().is_symlink() {
        return Err(refuse("directory-kind"));
    }
    let marker_bytes = read_snapshot(
        &directory.join(MARKER),
        marker_cap(limits)?.min(aggregate_cap(limits)?),
    )?;
    let marker: Marker =
        strict(std::str::from_utf8(&marker_bytes).map_err(|_| refuse("marker-utf8"))?)?;
    if marker.profile != PROFILE
        || marker.qualification != QUALIFICATION
        || marker.bindings != *bindings
        || marker.authorization != *authorization
        || marker.source_hash != bindings.source_hash
        || marker.program_hash != bindings.program_hash
        || marker.build != crate::proving::source_closure()
        || !digest(&marker.identity)
        || marker.identity != marker_identity(&marker)?
        || marker.actions.len() != program.actions.len()
        || marker.actions.len() > NativeWorkLimits::cap(limits.max_actions)?
    {
        return Err(refuse("marker-identity"));
    }
    let mut expected_names = BTreeSet::from(["source.mori".to_owned()]);
    for (i, (action, row)) in program.actions.iter().zip(&marker.actions).enumerate() {
        if row.name != action.name
            || row.ordinal != i
            || row.ir_leaf != ordinal(i, "ir.bin")
            || row.manifest_leaf != ordinal(i, "manifest.json")
            || !digest(&row.ir_hash)
            || !digest(&row.manifest_hash)
            || !digest(&row.compiler_hash)
        {
            return Err(refuse("action-metadata"));
        }
        expected_names.insert(row.ir_leaf.clone());
        expected_names.insert(row.manifest_leaf.clone());
    }
    if marker.leaves.keys().cloned().collect::<BTreeSet<_>>() != expected_names {
        return Err(refuse("leaf-set"));
    }
    let mut advertised_total = marker_bytes.len();
    for (name, metadata) in &marker.leaves {
        if !digest(&metadata.sha256) {
            return Err(refuse("leaf-digest"));
        }
        let size = usize::try_from(metadata.bytes).map_err(|_| refuse("leaf-size"))?;
        if size > leaf_cap(limits)? || (name == "source.mori" && size > input_cap(limits)?) {
            return Err(refuse("leaf-work-limit"));
        }
        advertised_total = add(advertised_total, size, aggregate_cap(limits)?)?;
    }
    for row in &marker.actions {
        if marker.leaves[&row.ir_leaf].sha256 != row.ir_hash
            || marker.leaves[&row.manifest_leaf].sha256 != row.manifest_hash
        {
            return Err(refuse("action-leaf-identity"));
        }
    }
    expected_names.insert(MARKER.into());
    let mut actual_names = BTreeSet::new();
    for entry in std::fs::read_dir(directory)
        .map_err(|_| refuse("directory-read"))?
        .take(
            expected_names
                .len()
                .checked_add(1)
                .ok_or(refuse("entry-count"))?,
        )
    {
        let entry = entry.map_err(|_| refuse("directory-read"))?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| refuse("entry-name"))?;
        if !expected_names.contains(&name) || !actual_names.insert(name) {
            return Err(refuse("entry-set"));
        }
        let kind = entry.file_type().map_err(|_| refuse("entry-kind"))?;
        if !kind.is_file() || kind.is_symlink() {
            return Err(refuse("entry-kind"));
        }
    }
    if actual_names != expected_names {
        return Err(refuse("entry-set"));
    }
    let mut acquired = BTreeMap::new();
    let mut total = marker_bytes.len();
    for (name, metadata) in &marker.leaves {
        let declared = usize::try_from(metadata.bytes).map_err(|_| refuse("leaf-size"))?;
        let cap = declared.min(leaf_cap(limits)?).min(
            aggregate_cap(limits)?
                .checked_sub(total)
                .ok_or(refuse("aggregate-work-limit"))?,
        );
        let bytes = read_snapshot(&directory.join(name), cap)?;
        if bytes.len() != declared || codec::hash(&bytes) != metadata.sha256 {
            return Err(refuse("leaf-identity"));
        }
        total = add(total, bytes.len(), aggregate_cap(limits)?)?;
        acquired.insert(name.clone(), bytes);
    }
    let fresh = compile(source, program, bindings, authorization, limits)?;
    if fresh.output.marker != marker_bytes
        || fresh.output.leaves != acquired
        || fresh.total != total
        || total != advertised_total
    {
        return Err(refuse("fresh-source-mismatch"));
    }
    // Decode only after exact fresh-producer equality; hostile tagged lengths
    // must not control allocations in the SDK decoder.
    for (name, bytes) in &fresh.output.leaves {
        if name.ends_with(".ir.bin") {
            let _: midnight_zkir_v3::IrSource = codec::decode_artifact(bytes)?;
        }
    }
    Ok(AdmittedStaticBundle { bundle: fresh })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checked_aggregate_refuses_overflow() {
        assert!(add(usize::MAX, 1, usize::MAX).is_err());
        assert_eq!(add(3, 4, 7).unwrap(), 7);
        assert!(add(3, 4, 6).is_err());
    }
    #[test]
    fn null_arrays_nested_fields_and_decoded_duplicates_refused() {
        assert!(strict::<serde_json::Value>("{\"x\":[null]}").is_err());
        assert!(strict::<serde_json::Value>(r#"{"x":1,"\u0078":2}"#).is_err());
    }
}
