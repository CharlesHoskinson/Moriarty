//! Bounded immutable artifact snapshots. A parsed manifest is untrusted data;
//! only original-source native regeneration constructs deployment capabilities.
use crate::{Error, Result, codec, profile::*};
use midnight_serialize::{Deserializable, Serializable, Tagged};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::{Read, Write},
    path::Path,
};

fn refusal(code: &'static str) -> Error {
    Error::new("call-artifact", code)
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Leaf {
    pub sha256: String,
    pub bytes: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionKeyMetadata {
    pub name: String,
    pub ordinal: usize,
    pub ir_hash: String,
    pub manifest_hash: String,
    pub vk_hash: String,
    pub degree: u8,
    pub srs_digest: String,
    pub srs_bytes: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicKeysetManifest {
    pub profile: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub signer_policy: Option<crate::signed::FixedSignerPolicy>,
    pub bindings: StaticBindings,
    pub build: String,
    pub actions: Vec<ActionKeyMetadata>,
    pub identity: String,
    pub leaves: BTreeMap<String, Leaf>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvingKeysetManifest {
    pub profile: String,
    pub public_identity: String,
    pub action: String,
    pub prover_key: Leaf,
}
pub struct ImportedPublicKeyset {
    pub(crate) manifest: PublicKeysetManifest,
    pub(crate) source: Vec<u8>,
    pub(crate) action_bytes: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
    pub(crate) snapshot_total: usize,
}
impl ImportedPublicKeyset {
    pub fn action_count(&self) -> usize {
        self.action_bytes.len()
    }
}
pub struct ImportedProvingKeyset {
    pub(crate) public: ImportedPublicKeyset,
    pub(crate) action: String,
    pub(crate) pk: Vec<u8>,
    pub(crate) counts: KeysetByteCounts,
}
/// Acquired/exported transport bytes only; SRS has its existing separate budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct KeysetByteCounts {
    pub(crate) public: usize,
    pub(crate) prover: usize,
    pub(crate) physical: usize,
}
pub(crate) fn keyset_counts(
    public: usize,
    prover: usize,
    limits: &NativeWorkLimits,
) -> Result<KeysetByteCounts> {
    limits.validate()?;
    if prover > limits.prover_key_cap()? {
        return Err(refusal("leaf-size"));
    }
    let public_cap = NativeWorkLimits::cap(limits.max_artifact_bytes)?;
    let physical = public
        .checked_add(prover)
        .ok_or(refusal("aggregate-size"))?;
    let physical_cap = match limits.version()? {
        WorkLimitsVersion::V1 => public_cap,
        WorkLimitsVersion::V2 => public_cap
            .checked_add(limits.prover_key_cap()?)
            .ok_or(refusal("aggregate-size"))?,
    };
    if public > public_cap || physical > physical_cap {
        return Err(refusal("aggregate-size"));
    }
    Ok(KeysetByteCounts {
        public,
        prover,
        physical,
    })
}
pub fn public_identity(m: &PublicKeysetManifest) -> Result<String> {
    if let Some(policy) = &m.signer_policy {
        codec::object_hash(&(&m.profile, &m.bindings, &m.build, &m.actions, policy))
    } else {
        codec::object_hash(&(&m.profile, &m.bindings, &m.build, &m.actions))
    }
}
pub(crate) fn ordinal_leaf(index: usize, suffix: &str) -> String {
    format!("action-{index:06}.{suffix}")
}
pub(crate) fn add_total(total: usize, next: usize, cap: usize) -> Result<usize> {
    total
        .checked_add(next)
        .filter(|n| *n <= cap)
        .ok_or(refusal("aggregate-size"))
}
// Positive acquisitions must admit their remaining aggregate budget before
// opening a file. Reused in the new-degree SRS path before decoder/init work.
pub(crate) fn remaining_read_cap(total: usize, per_file: u64, aggregate: usize) -> Result<u64> {
    let per_file = NativeWorkLimits::cap(per_file)?;
    let remaining = aggregate
        .checked_sub(total)
        .filter(|n| *n > 0)
        .ok_or(refusal("aggregate-size"))?;
    u64::try_from(per_file.min(remaining)).map_err(|_| refusal("aggregate-size"))
}
pub fn read_bounded_file(path: &Path, cap: u64) -> Result<Vec<u8>> {
    read_file_snapshot(path, NativeWorkLimits::cap(cap)?)
}
// An admitted empty leaf has a zero byte cap. This does not permit zero caller
// limits: the public reader and marker/work-limit admission still reject them.
fn read_file_snapshot(path: &Path, cap: usize) -> Result<Vec<u8>> {
    let sentinel = cap
        .checked_add(1)
        .and_then(|n| u64::try_from(n).ok())
        .ok_or(refusal("size"))?;
    let cap_u64 = u64::try_from(cap).map_err(|_| refusal("size"))?;
    #[cfg(unix)]
    let file = {
        use std::os::unix::fs::OpenOptionsExt;
        std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(path)
    };
    #[cfg(not(unix))]
    let file = std::fs::File::open(path);
    let file = file.map_err(|_| refusal("open"))?;
    let metadata = file.metadata().map_err(|_| refusal("metadata"))?;
    if !metadata.is_file() || metadata.len() > cap_u64 {
        return Err(refusal("file-kind-or-size"));
    }
    let mut bytes = Vec::new();
    file.take(sentinel)
        .read_to_end(&mut bytes)
        .map_err(|_| refusal("read"))?;
    if bytes.len() > cap {
        return Err(refusal("size"));
    }
    Ok(bytes)
}
fn read_leaf(
    directory: &Path,
    name: &str,
    leaf: &Leaf,
    cap: u64,
    total: &mut usize,
    aggregate: usize,
) -> Result<Vec<u8>> {
    if leaf.bytes > cap {
        return Err(refusal("leaf-size"));
    }
    let length = usize::try_from(leaf.bytes).map_err(|_| refusal("leaf-size"))?;
    let next = add_total(*total, length, aggregate)?;
    // The declared length is already admitted against both caps. Snapshot under
    // that exact length, so lying metadata cannot consume the larger per-leaf
    // budget before identity refusal. Opened-FD size checks precede any read;
    // the one-byte sentinel detects growth beyond this admitted snapshot.
    let bytes = read_file_snapshot(&directory.join(name), length)?;
    if bytes.len() != length || codec::hash(&bytes) != leaf.sha256 {
        return Err(refusal("leaf-identity"));
    }
    *total = next;
    Ok(bytes)
}
pub fn snapshot_public_keyset(
    directory: &Path,
    limits: &NativeWorkLimits,
) -> Result<ImportedPublicKeyset> {
    snapshot_public_profile(directory, limits, false)
}
pub(crate) fn snapshot_public_profile(
    directory: &Path,
    limits: &NativeWorkLimits,
    signed: bool,
) -> Result<ImportedPublicKeyset> {
    limits.validate()?;
    let aggregate = NativeWorkLimits::cap(limits.max_artifact_bytes)?;
    let cap = remaining_read_cap(
        0,
        limits.max_leaf_bytes.min(codec::INPUT_BYTES as u64),
        aggregate,
    )?;
    let marker = read_bounded_file(&directory.join("public-keyset.json"), cap)?;
    let manifest: PublicKeysetManifest =
        codec::json(std::str::from_utf8(&marker).map_err(|_| refusal("manifest-utf8"))?)?;
    if manifest.profile
        != if signed {
            crate::signed::KEYSET
        } else {
            "moriarty-native-call-public-keyset/1"
        }
        || manifest.signer_policy.is_some() != signed
        || manifest.actions.is_empty()
        || manifest.actions.len() > NativeWorkLimits::cap(limits.max_actions)?
        || manifest.identity != public_identity(&manifest)?
    {
        return Err(refusal("public-manifest"));
    }
    let mut total = add_total(0, marker.len(), aggregate)?;
    let mut required = BTreeSet::from(["source.mori".to_owned()]);
    let mut names = BTreeSet::new();
    for (i, a) in manifest.actions.iter().enumerate() {
        if a.ordinal != i || a.name.is_empty() || !names.insert(&a.name) {
            return Err(refusal("action-set"));
        }
        for suffix in ["zkir.bin", "manifest.json", "vk.bin"] {
            required.insert(ordinal_leaf(i, suffix));
        }
    }
    // Optional producer leaves are ignored without stat/open/checksum/size work.
    // In particular no PK or proving marker is required by cold public import.
    for name in manifest.leaves.keys() {
        if !required.contains(name)
            && ![
                "selected.pk.bin",
                "proving-keyset.json",
                "deployment.json",
                "predecessor.bin",
                "parameters.bin",
            ]
            .contains(&name.as_str())
        {
            return Err(refusal("leaf-name"));
        }
    }
    if !required.iter().all(|n| manifest.leaves.contains_key(n)) {
        return Err(refusal("missing-public-leaf"));
    }
    let source = read_leaf(
        directory,
        "source.mori",
        &manifest.leaves["source.mori"],
        limits.max_input_bytes.min(codec::INPUT_BYTES as u64),
        &mut total,
        aggregate,
    )?;
    let mut action_bytes = Vec::new();
    for (i, a) in manifest.actions.iter().enumerate() {
        let mut leaves = Vec::new();
        for (suffix, hash, cap) in [
            ("zkir.bin", &a.ir_hash, limits.max_leaf_bytes),
            (
                "manifest.json",
                &a.manifest_hash,
                limits.max_leaf_bytes.min(codec::INPUT_BYTES as u64),
            ),
            (
                "vk.bin",
                &a.vk_hash,
                limits.max_verifier_key_bytes.min(limits.max_leaf_bytes),
            ),
        ] {
            let name = ordinal_leaf(i, suffix);
            let leaf = &manifest.leaves[&name];
            if &leaf.sha256 != hash {
                return Err(refusal("action-leaf-identity"));
            }
            leaves.push(read_leaf(
                directory, &name, leaf, cap, &mut total, aggregate,
            )?);
        }
        let mut leaves = leaves.into_iter();
        action_bytes.push((
            leaves.next().unwrap(),
            leaves.next().unwrap(),
            leaves.next().unwrap(),
        ));
    }
    Ok(ImportedPublicKeyset {
        manifest,
        source,
        action_bytes,
        snapshot_total: total,
    })
}
pub fn snapshot_proving_keyset(
    directory: &Path,
    action: &str,
    limits: &NativeWorkLimits,
) -> Result<ImportedProvingKeyset> {
    snapshot_proving_profile(directory, action, limits, false)
}
pub(crate) fn snapshot_proving_profile(
    directory: &Path,
    action: &str,
    limits: &NativeWorkLimits,
    signed: bool,
) -> Result<ImportedProvingKeyset> {
    let public = snapshot_public_profile(directory, limits, signed)?;
    let aggregate = NativeWorkLimits::cap(limits.max_artifact_bytes)?;
    let marker_cap = remaining_read_cap(
        public.snapshot_total,
        limits.max_leaf_bytes.min(codec::INPUT_BYTES as u64),
        aggregate,
    )?;
    let bytes = read_bounded_file(&directory.join("proving-keyset.json"), marker_cap)?;
    let marker: ProvingKeysetManifest =
        codec::json(std::str::from_utf8(&bytes).map_err(|_| refusal("manifest-utf8"))?)?;
    if marker.profile
        != if signed {
            crate::signed::KEYSET
        } else {
            "moriarty-native-call-proving-keyset/1"
        }
        || marker.action != action
        || marker.public_identity != public.manifest.identity
    {
        return Err(refusal("proving-manifest"));
    }
    if public
        .manifest
        .leaves
        .get("selected.pk.bin")
        .is_some_and(|leaf| *leaf != marker.prover_key)
        || public
            .manifest
            .leaves
            .get("proving-keyset.json")
            .is_some_and(|leaf| *leaf != self::leaf(&bytes))
    {
        return Err(refusal("proving-leaf-identity"));
    }
    let public_total = add_total(
        public.snapshot_total,
        bytes.len(),
        NativeWorkLimits::cap(limits.max_artifact_bytes)?,
    )?;
    // Admit both declared domains and checked physical addition before PK open.
    let pk_length = usize::try_from(marker.prover_key.bytes).map_err(|_| refusal("leaf-size"))?;
    let counts = keyset_counts(public_total, pk_length, limits)?;
    // Retain exact length/hash acquisition. The PK-only counter has its own
    // cap in v2; legacy counts above still enforce its shared aggregate.
    let mut pk_total = 0;
    let pk_cap = limits.prover_key_cap()?;
    let pk = read_leaf(
        directory,
        "selected.pk.bin",
        &marker.prover_key,
        u64::try_from(pk_cap).map_err(|_| refusal("leaf-size"))?,
        &mut pk_total,
        pk_cap,
    )?;
    Ok(ImportedProvingKeyset {
        public,
        action: action.into(),
        pk,
        counts,
    })
}
struct BoundedWriter {
    bytes: Vec<u8>,
    cap: usize,
}
impl Write for BoundedWriter {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        if self
            .bytes
            .len()
            .checked_add(data.len())
            .is_none_or(|n| n > self.cap)
        {
            return Err(std::io::Error::other("caller artifact cap"));
        }
        self.bytes.extend_from_slice(data);
        Ok(data.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
pub(crate) fn encode_bounded<T: Serializable + Tagged>(value: &T, cap: u64) -> Result<Vec<u8>> {
    let mut writer = BoundedWriter {
        bytes: Vec::new(),
        cap: NativeWorkLimits::cap(cap)?,
    };
    midnight_serialize::tagged_serialize(value, &mut writer).map_err(|_| refusal("encode-size"))?;
    Ok(writer.bytes)
}
pub(crate) fn decode_bounded<T: Deserializable + Serializable + Tagged>(
    bytes: &[u8],
    cap: u64,
) -> Result<T> {
    if bytes.len() > NativeWorkLimits::cap(cap)? {
        return Err(refusal("decode-size"));
    }
    let value =
        midnight_serialize::tagged_deserialize(bytes).map_err(|_| refusal("decode-tag-or-eof"))?;
    if encode_bounded(&value, cap)? != bytes {
        return Err(refusal("decode-noncanonical"));
    }
    Ok(value)
}
pub(crate) fn json_bytes(value: &impl Serialize, cap: u64) -> Result<Vec<u8>> {
    let mut writer = BoundedWriter {
        bytes: Vec::new(),
        cap: NativeWorkLimits::cap(cap)?,
    };
    serde_json::to_writer_pretty(&mut writer, value).map_err(|_| refusal("json-size"))?;
    Ok(writer.bytes)
}
pub(crate) fn leaf(bytes: &[u8]) -> Leaf {
    Leaf {
        sha256: codec::hash(bytes),
        bytes: bytes.len() as u64,
    }
}
pub struct ArtifactLeaves {
    pub leaves: BTreeMap<String, Vec<u8>>,
    pub marker_name: &'static str,
    pub marker: Vec<u8>,
}
impl ArtifactLeaves {
    pub(crate) fn check(&self, limits: &NativeWorkLimits) -> Result<()> {
        limits.validate()?;
        let cap = NativeWorkLimits::cap(limits.max_artifact_bytes)?;
        let mut total = add_total(0, self.marker.len(), cap)?;
        for (name, bytes) in &self.leaves {
            if name == self.marker_name || !valid_leaf(name) {
                return Err(refusal("leaf-name"));
            }
            if bytes.len() > NativeWorkLimits::cap(limits.max_leaf_bytes)? {
                return Err(refusal("leaf-size"));
            }
            total = add_total(total, bytes.len(), cap)?;
        }
        Ok(())
    }
    // This private context is constructed only by the genuine keyset factory
    // entry below; generic ArtifactLeaves never classifies by a PK-like name.
    fn check_keyset(
        &self,
        selected: SelectedProverKeyLength,
        limits: &NativeWorkLimits,
    ) -> Result<KeysetByteCounts> {
        if self.marker_name != "public-keyset.json" {
            return Err(refusal("keyset-context"));
        }
        let pk = self
            .leaves
            .get("selected.pk.bin")
            .ok_or(refusal("keyset-context"))?;
        if pk.len() != selected.0 {
            return Err(refusal("keyset-context"));
        }
        // v1 uses the exact original public-only flat checker.
        if limits.version()? == WorkLimitsVersion::V1 {
            self.check(limits)?;
        }
        limits.validate()?;
        let cap = NativeWorkLimits::cap(limits.max_artifact_bytes)?;
        let mut public = add_total(0, self.marker.len(), cap)?;
        for (name, bytes) in &self.leaves {
            if name == self.marker_name || !valid_leaf(name) {
                return Err(refusal("leaf-name"));
            }
            if name == "selected.pk.bin" {
                continue;
            }
            if bytes.len() > NativeWorkLimits::cap(limits.max_leaf_bytes)? {
                return Err(refusal("leaf-size"));
            }
            public = add_total(public, bytes.len(), cap)?;
        }
        keyset_counts(public, pk.len(), limits)
    }
}
struct SelectedProverKeyLength(usize);
pub(crate) fn check_keyset_export(
    output: &ArtifactLeaves,
    selected: &crate::proving::DerivedCallMaterial,
    limits: &NativeWorkLimits,
) -> Result<KeysetByteCounts> {
    output.check_keyset(
        SelectedProverKeyLength(selected.prover_key_bytes().len()),
        limits,
    )
}

/// Existing export input checks, separate from public/prover transport counts.
pub(crate) fn check_export_input_lengths(
    source: usize,
    predecessor: usize,
    parameters: usize,
    limits: &NativeWorkLimits,
) -> Result<()> {
    let cap = NativeWorkLimits::cap(limits.max_input_bytes)?;
    if predecessor > cap
        || parameters > cap
        || (limits.version()? == WorkLimitsVersion::V2 && source > cap)
    {
        return Err(Error::new("local-deployment", "export-work-limit"));
    }
    Ok(())
}
/// Descriptor inputs keep their existing one-pass bounded JSON writer.
pub(crate) fn deployment_json_bytes<T: Serialize>(
    descriptor: &T,
    limits: &NativeWorkLimits,
) -> Result<Vec<u8>> {
    let mut cap = limits.max_leaf_bytes.min(codec::INPUT_BYTES as u64);
    if limits.version()? == WorkLimitsVersion::V2 {
        cap = cap.min(limits.max_input_bytes);
    }
    json_bytes(descriptor, cap)
}

#[cfg(test)]
mod prover_key_domain_tests {
    use super::*;
    fn limits() -> NativeWorkLimits {
        NativeWorkLimits {
            profile: "moriarty-native-call-work-limits/2".into(),
            max_degree: 19,
            max_actions: 4,
            max_srs_bytes: 1024,
            max_prover_key_bytes: 32 * 1024 * 1024,
            max_verifier_key_bytes: 65536,
            max_input_bytes: 1048576,
            max_leaf_bytes: 16777216,
            max_artifact_bytes: 16777216,
            max_operations: 10000,
            max_public_inputs: 10000,
        }
    }
    #[test]
    fn real_boundaries_and_physical_addition_use_counts_without_allocation() {
        let mut l = limits();
        let public = 16 * 1024 * 1024;
        let prover = 32 * 1024 * 1024;
        assert_eq!(
            keyset_counts(public, prover, &l).unwrap(),
            KeysetByteCounts {
                public,
                prover,
                physical: public + prover
            }
        );
        assert_eq!(
            keyset_counts(public + 1, prover, &l).unwrap_err().code,
            "aggregate-size"
        );
        assert_eq!(
            keyset_counts(public, prover + 1, &l).unwrap_err().code,
            "leaf-size"
        );
        assert_eq!(
            keyset_counts(usize::MAX, 1, &l).unwrap_err().code,
            "aggregate-size"
        );
        l.max_prover_key_bytes = 64 * 1024 * 1024;
        assert_eq!(
            keyset_counts(public, 64 * 1024 * 1024, &l)
                .unwrap()
                .physical,
            80 * 1024 * 1024
        );
        l.max_artifact_bytes = 3;
        l.max_prover_key_bytes = 4;
        l.max_leaf_bytes = 2;
        assert_eq!(keyset_counts(3, 4, &l).unwrap().physical, 7);
        l.profile = "moriarty-native-call-work-limits/1".into();
        assert!(keyset_counts(3, 4, &l).is_err());
    }
    #[test]
    fn hard_v2_ceilings_preserve_exact_unsigned_and_signed_v1_admission() {
        let original = limits();
        assert!(original.validate().is_ok());
        for field in [
            "max_prover_key_bytes",
            "max_leaf_bytes",
            "max_artifact_bytes",
            "max_input_bytes",
            "max_verifier_key_bytes",
        ] {
            let mut json = serde_json::to_value(&original).unwrap();
            let max = match field {
                "max_prover_key_bytes" => 67108864,
                "max_input_bytes" => 1048576,
                "max_verifier_key_bytes" => 65536,
                _ => 16777216,
            };
            json[field] = serde_json::json!(max);
            assert!(NativeWorkLimits::from_json(&json.to_string()).is_ok());
            json[field] = serde_json::json!(max + 1);
            assert!(NativeWorkLimits::from_json(&json.to_string()).is_err());
            json["profile"] = serde_json::json!("moriarty-native-call-work-limits/1");
            let legacy = NativeWorkLimits::from_json(&json.to_string()).unwrap();
            if matches!(
                field,
                "max_leaf_bytes" | "max_artifact_bytes" | "max_input_bytes"
            ) {
                assert_eq!(
                    crate::signed::validate_work_limits(&legacy)
                        .unwrap_err()
                        .code,
                    "ordinary-envelope"
                );
            } else {
                assert!(crate::signed::validate_work_limits(&legacy).is_ok());
            }
        }
        let mut bad = original;
        bad.profile = "moriarty-native-call-work-limits/02".into();
        assert!(bad.validate().is_err());
    }
    #[test]
    fn only_validated_keyset_context_gets_singleton_pk_domain_and_counts_markers() {
        let mut l = limits();
        l.max_leaf_bytes = 3;
        l.max_artifact_bytes = 7;
        l.max_prover_key_bytes = 8;
        let mut output = ArtifactLeaves {
            leaves: [
                ("selected.pk.bin".into(), vec![0; 8]),
                ("source.mori".into(), vec![0; 2]),
                ("proving-keyset.json".into(), vec![0; 3]),
            ]
            .into(),
            marker_name: "public-keyset.json",
            marker: vec![0; 2],
        };
        let mut generic = l.clone();
        generic.max_artifact_bytes = 15;
        assert_eq!(output.check(&generic).unwrap_err().code, "leaf-size");
        generic.max_leaf_bytes = 8;
        generic.max_artifact_bytes = 7;
        assert_eq!(output.check(&generic).unwrap_err().code, "aggregate-size");
        let counts = output.check_keyset(SelectedProverKeyLength(8), &l).unwrap();
        assert_eq!(
            counts,
            KeysetByteCounts {
                public: 7,
                prover: 8,
                physical: 15
            }
        );
        l.max_artifact_bytes = 6;
        assert_eq!(
            output
                .check_keyset(SelectedProverKeyLength(8), &l)
                .unwrap_err()
                .code,
            "aggregate-size"
        );
        l.max_artifact_bytes = 7;
        output
            .leaves
            .get_mut("proving-keyset.json")
            .unwrap()
            .push(0);
        assert_eq!(
            output
                .check_keyset(SelectedProverKeyLength(8), &l)
                .unwrap_err()
                .code,
            "leaf-size"
        );
        output.leaves.get_mut("proving-keyset.json").unwrap().pop();
        for name in [
            "renamed.pk.bin",
            "action-000000.pk.bin",
            "../selected.pk.bin",
            "selected.pk.bin/extra",
        ] {
            output.leaves.insert(name.into(), vec![0; 8]);
            assert!(output.check_keyset(SelectedProverKeyLength(8), &l).is_err());
            output.leaves.remove(name);
        }
        assert_eq!(
            output
                .check_keyset(SelectedProverKeyLength(7), &l)
                .unwrap_err()
                .code,
            "keyset-context"
        );
        output.marker_name = "proven-call.json";
        assert_eq!(
            output
                .check_keyset(SelectedProverKeyLength(8), &l)
                .unwrap_err()
                .code,
            "keyset-context"
        );
    }
    #[test]
    fn signed_descriptor_replacement_and_lowered_reopen_caps_recheck_domains() {
        let mut l = limits();
        l.max_leaf_bytes = 4;
        l.max_artifact_bytes = 6;
        l.max_prover_key_bytes = 8;
        let mut output = ArtifactLeaves {
            leaves: [
                ("selected.pk.bin".into(), vec![0; 8]),
                ("deployment.json".into(), vec![0; 4]),
            ]
            .into(),
            marker_name: "public-keyset.json",
            marker: vec![0; 2],
        };
        assert!(output.check_keyset(SelectedProverKeyLength(8), &l).is_ok());
        output.leaves.get_mut("deployment.json").unwrap().push(0);
        assert_eq!(
            output
                .check_keyset(SelectedProverKeyLength(8), &l)
                .unwrap_err()
                .code,
            "leaf-size"
        );
        output.leaves.get_mut("deployment.json").unwrap().pop();
        output.marker.push(0);
        assert_eq!(
            output
                .check_keyset(SelectedProverKeyLength(8), &l)
                .unwrap_err()
                .code,
            "aggregate-size"
        );
        assert!(keyset_counts(6, 8, &l).is_ok());
        l.max_prover_key_bytes = 7;
        assert!(keyset_counts(6, 8, &l).is_err());
        l.max_prover_key_bytes = 8;
        l.max_artifact_bytes = 5;
        assert!(keyset_counts(6, 8, &l).is_err());
    }
    #[test]
    fn strict_v2_fields_refuse_invalid_values_without_any_acquisition() {
        let good = serde_json::to_value(limits()).unwrap();
        for field in [
            "max_actions",
            "max_srs_bytes",
            "max_prover_key_bytes",
            "max_verifier_key_bytes",
            "max_input_bytes",
            "max_leaf_bytes",
            "max_artifact_bytes",
            "max_operations",
            "max_public_inputs",
        ] {
            for invalid in [
                serde_json::json!(0),
                serde_json::json!(-1),
                serde_json::json!(1.5),
                serde_json::json!(u64::MAX),
            ] {
                let mut value = good.clone();
                value[field] = invalid;
                assert!(
                    NativeWorkLimits::from_json(&value.to_string()).is_err(),
                    "{field}"
                );
            }
            let mut value = good.clone();
            value.as_object_mut().unwrap().remove(field);
            assert!(
                NativeWorkLimits::from_json(&value.to_string()).is_err(),
                "{field}"
            );
        }
        let text = good.to_string();
        assert!(NativeWorkLimits::from_json(&text.replacen('{', "{\"unexpected\":1,", 1)).is_err());
        assert!(
            NativeWorkLimits::from_json(&text.replacen('{', "{\"max_degree\":19,", 1)).is_err()
        );
    }
    #[test]
    fn lowered_v2_source_predecessor_and_parameter_inputs_refuse_before_output() {
        let mut l = limits();
        l.max_input_bytes = 4;
        assert!(check_export_input_lengths(4, 4, 4, &l).is_ok());
        assert!(check_export_input_lengths(5, 1, 1, &l).is_err());
        assert!(check_export_input_lengths(1, 5, 1, &l).is_err());
        assert!(check_export_input_lengths(1, 1, 5, &l).is_err());
        l.profile = "moriarty-native-call-work-limits/1".into();
        assert!(check_export_input_lengths(5, 1, 1, &l).is_ok());
    }
    #[test]
    fn lowered_v2_unsigned_and_signed_descriptor_inputs_use_current_caller_cap() {
        for descriptor in [
            serde_json::json!({"deployment":"opaque"}),
            serde_json::json!({"deployment":"opaque", "signer_policy":"public"}),
        ] {
            let encoded = serde_json::to_vec_pretty(&descriptor).unwrap();
            let mut l = limits();
            l.max_input_bytes = encoded.len() as u64;
            assert_eq!(deployment_json_bytes(&descriptor, &l).unwrap(), encoded);
            l.max_input_bytes -= 1;
            assert!(deployment_json_bytes(&descriptor, &l).is_err());
            l.profile = "moriarty-native-call-work-limits/1".into();
            assert_eq!(deployment_json_bytes(&descriptor, &l).unwrap(), encoded);
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofContext {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub signed: Option<crate::signed::SignedProofContext>,
    pub source_hash: String,
    pub keyset_identity: String,
    pub deployment_hash: String,
    pub predecessor_hash: String,
    pub parameters_hash: String,
    pub request_hash: String,
    pub action: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProvenManifest {
    profile: String,
    context: ProofContext,
    leaves: BTreeMap<String, Leaf>,
}
pub struct ImportedProvenCall {
    pub(crate) context: ProofContext,
    pub(crate) intent: crate::proving::ProvenIntent,
    pub(crate) verification: Vec<u8>,
    final_readback: Vec<u8>,
    inspection: BTreeMap<String, Vec<u8>>,
}
impl ImportedProvenCall {
    pub fn admit_context(&self, context: &ProofContext) -> Result<()> {
        if codec::object_hash(&self.context)? != codec::object_hash(context)? {
            return Err(refusal("proof-context"));
        }
        Ok(())
    }
    pub fn admit_inspection(&self, prepared: &crate::prepare::PreparedInstalledCall) -> Result<()> {
        if self.inspection != prepared.bundle().leaves {
            return Err(refusal("prepared-inspection-identity"));
        }
        Ok(())
    }
    pub fn intent(&self) -> &crate::proving::ProvenIntent {
        &self.intent
    }
    pub fn admit_readback(
        &self,
        prepared: &crate::prepare::PreparedInstalledCall,
        verification: &crate::proving::CallVerification,
        limits: &NativeWorkLimits,
    ) -> Result<()> {
        if self.verification != json_bytes(verification, limits.max_leaf_bytes)?
            || self.final_readback
                != json_bytes(
                    &crate::prepare::final_installed_readback(
                        &prepared.bundle().readback,
                        verification,
                    ),
                    limits.max_leaf_bytes,
                )?
        {
            return Err(refusal("verification-readback"));
        }
        Ok(())
    }
}
pub fn snapshot_proven_call(
    directory: &Path,
    limits: &NativeWorkLimits,
) -> Result<ImportedProvenCall> {
    snapshot_proven_profile(directory, limits, false)
}
pub(crate) fn snapshot_proven_profile(
    directory: &Path,
    limits: &NativeWorkLimits,
    signed: bool,
) -> Result<ImportedProvenCall> {
    limits.validate()?;
    let cap = NativeWorkLimits::cap(limits.max_artifact_bytes)?;
    let marker_cap =
        remaining_read_cap(0, limits.max_leaf_bytes.min(codec::INPUT_BYTES as u64), cap)?;
    let marker = read_bounded_file(&directory.join("proven-call.json"), marker_cap)?;
    let m: ProvenManifest =
        codec::json(std::str::from_utf8(&marker).map_err(|_| refusal("manifest-utf8"))?)?;
    if m.profile
        != if signed {
            crate::signed::PROOF_CONTEXT
        } else {
            "moriarty-native-call-proven/1"
        }
        || m.context.signed.is_some() != signed
        || !m.leaves.contains_key("final-intent.bin")
        || !m.leaves.contains_key("verification.json")
        || !m.leaves.contains_key("final-readback.json")
    {
        return Err(refusal("proven-manifest"));
    }
    let mut total = add_total(0, marker.len(), cap)?;
    let mut leaves = BTreeMap::new();
    for (name, desc) in &m.leaves {
        if !valid_leaf(name) {
            return Err(refusal("leaf-name"));
        }
        leaves.insert(
            name.clone(),
            read_leaf(
                directory,
                name,
                desc,
                limits.max_leaf_bytes,
                &mut total,
                cap,
            )?,
        );
    }
    let intent = decode_bounded(&leaves["final-intent.bin"], limits.max_leaf_bytes)?;
    admit_intent_work(&intent, limits)?;
    leaves.remove("final-intent.bin");
    Ok(ImportedProvenCall {
        context: m.context,
        intent,
        verification: leaves.remove("verification.json").unwrap(),
        final_readback: leaves.remove("final-readback.json").unwrap(),
        inspection: leaves,
    })
}
pub fn export_proven_call(
    proven: &crate::proving::ProvenInstalledCall,
    limits: &NativeWorkLimits,
) -> Result<ArtifactLeaves> {
    limits.validate()?;
    admit_intent_work(proven.intent(), limits)?;
    let context = proven.context().clone();
    let mut inspection = proven.inspection().clone();
    inspection.insert(
        "final-readback.json".into(),
        json_bytes(proven.readback(), limits.max_leaf_bytes)?,
    );
    inspection.insert(
        "final-intent.bin".into(),
        encode_bounded(proven.intent(), limits.max_leaf_bytes)?,
    );
    inspection.insert(
        "verification.json".into(),
        json_bytes(proven.verification(), limits.max_leaf_bytes)?,
    );
    let manifest = ProvenManifest {
        profile: if context.signed.is_some() {
            crate::signed::PROOF_CONTEXT
        } else {
            "moriarty-native-call-proven/1"
        }
        .into(),
        context,
        leaves: inspection
            .iter()
            .map(|(n, b)| (n.clone(), leaf(b)))
            .collect(),
    };
    let output = ArtifactLeaves {
        leaves: inspection,
        marker_name: "proven-call.json",
        marker: json_bytes(
            &manifest,
            limits.max_leaf_bytes.min(codec::INPUT_BYTES as u64),
        )?,
    };
    output.check(limits)?;
    Ok(output)
}

pub fn verification_bytes(
    verification: &crate::proving::CallVerification,
    limits: &NativeWorkLimits,
) -> Result<Vec<u8>> {
    json_bytes(verification, limits.max_leaf_bytes)
}

/// Count actual operation fields before any public-input vector allocation.
/// In particular an untrusted Noop width cannot request a huge zero vector.
pub(crate) fn admit_intent_work<
    P: midnight_ledger::structure::ProofKind<midnight_storage::db::InMemoryDB>,
>(
    intent: &midnight_ledger::structure::Intent<
        (),
        P,
        midnight_transient_crypto::commitment::PedersenRandomness,
        midnight_storage::db::InMemoryDB,
    >,
    limits: &NativeWorkLimits,
) -> Result<()> {
    use midnight_transient_crypto::repr::FieldRepr;
    limits.validate()?;
    if intent.actions.len() > NativeWorkLimits::cap(limits.max_actions)? {
        return Err(refusal("intent-action-work"));
    }
    let mut operations = 0;
    let mut fields = 0;
    for call in intent.calls() {
        fields = add_total(fields, 2, NativeWorkLimits::cap(limits.max_public_inputs)?)?;
        for transcript in [
            call.guaranteed_transcript.as_deref(),
            call.fallible_transcript.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            operations = add_total(
                operations,
                transcript.program.len(),
                NativeWorkLimits::cap(limits.max_operations)?,
            )?;
            for op in transcript.program.iter() {
                fields = add_total(
                    fields,
                    op.field_size(),
                    NativeWorkLimits::cap(limits.max_public_inputs)?,
                )?;
            }
        }
    }
    Ok(())
}

pub(crate) fn check_json_input(value: &impl Serialize, cap: u64) -> Result<()> {
    let mut writer = BoundedWriter {
        bytes: Vec::new(),
        cap: NativeWorkLimits::cap(cap.min(codec::INPUT_BYTES as u64))?,
    };
    serde_json::to_writer(&mut writer, value).map_err(|_| refusal("typed-input-size"))?;
    Ok(())
}

fn valid_leaf(name: &str) -> bool {
    !name.is_empty()
        && !name.starts_with('.')
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
}

#[cfg(test)]
mod aggregate_read_tests {
    use super::*;
    #[test]
    fn lying_leaf_length_bounds_actual_file_before_read_and_hash() {
        let root = std::env::temp_dir().join(format!("mori-leaf-budget-{}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        std::fs::write(root.join("leaf.bin"), b"ab").unwrap();
        let leaf = Leaf {
            bytes: 1,
            sha256: codec::hash(b"a"),
        };
        let mut total = 0;
        let err = read_leaf(&root, "leaf.bin", &leaf, 64, &mut total, 1).unwrap_err();
        std::fs::remove_dir_all(root).unwrap();
        assert_eq!(err.code, "file-kind-or-size");
        assert_eq!(total, 0);
    }
    #[test]
    fn zero_byte_leaf_is_exact_even_with_exhausted_aggregate() {
        let root =
            std::env::temp_dir().join(format!("mori-empty-leaf-budget-{}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        std::fs::write(root.join("leaf.bin"), b"").unwrap();
        let leaf = Leaf {
            bytes: 0,
            sha256: codec::hash(b""),
        };
        let mut total = 1;
        assert!(
            read_leaf(&root, "leaf.bin", &leaf, 64, &mut total, 1)
                .unwrap()
                .is_empty()
        );
        std::fs::write(root.join("leaf.bin"), b"x").unwrap();
        let err = read_leaf(&root, "leaf.bin", &leaf, 64, &mut total, 1).unwrap_err();
        std::fs::remove_dir_all(root).unwrap();
        assert_eq!(err.code, "file-kind-or-size");
        assert_eq!(total, 1);
    }
}
