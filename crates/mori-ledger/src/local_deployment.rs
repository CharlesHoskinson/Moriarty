//! Source-derived complete local operation policy. Native derivation entry
//! points require separate physical resource admission when actually invoked.
use crate::{Error, Result, codec, layout, profile::*, proving::PublicDerivedCallKey, relation};
use midnight_ledger::structure::LedgerParameters;
use midnight_onchain_state::state::ContractOperation;
use mori_core::Program;
use std::{collections::BTreeMap, sync::Arc};

pub struct AdmittedSeed {
    pub(crate) frame: layout::AdmittedFrame,
    pub(crate) bindings: StaticBindings,
    pub(crate) descriptor: Deployment,
    pub(crate) source: String,
    pub(crate) program: Program,
    pub(crate) predecessor_bytes: usize,
    pub(crate) parameter_bytes: usize,
}
impl AdmittedSeed {
    pub fn action_names(&self) -> Vec<&str> {
        self.program
            .actions
            .iter()
            .map(|a| a.name.as_str())
            .collect()
    }
}
pub fn admit_local_seed(
    source: &str,
    program: &Program,
    descriptor: &Deployment,
    predecessor: &[u8],
    parameters: &[u8],
) -> Result<AdmittedSeed> {
    let abi = layout::admit_source(program, descriptor)?;
    if source.len() > codec::INPUT_BYTES
        || codec::hash(source.as_bytes()) != descriptor.source_hash
        || mori_core::parse(source).map_err(|_| Error::new("source-profile", "source-parse"))?
            != *program
    {
        return Err(Error::new("identity", "raw-source"));
    }
    let state: layout::NativeState = codec::decode_native(predecessor)?;
    let params: LedgerParameters = codec::decode_native(parameters)?;
    if codec::hash(parameters) != descriptor.parameters_hash || *state.parameters != params {
        return Err(Error::new("identity", "parameters"));
    }
    let bindings = StaticBindings::legacy(descriptor);
    let frame = layout::admit_frame(
        program,
        &abi,
        &bindings,
        &FrameBindings::legacy(descriptor),
        state,
        layout::OperationPolicy::Keyless,
    )?;
    layout::frame_gas(&frame)?;
    Ok(AdmittedSeed {
        frame,
        bindings,
        descriptor: descriptor.clone(),
        source: source.into(),
        program: program.clone(),
        predecessor_bytes: predecessor.len(),
        parameter_bytes: parameters.len(),
    })
}
#[derive(Clone)]
/// Private fields prevent imported maps from constructing source provenance.
/// ```compile_fail
/// let deployment = mori_ledger::local_deployment::PublicLocalDeployment { snapshot: Default::default() };
/// ```
/// ```compile_fail
/// let _: mori_ledger::local_deployment::PublicLocalDeployment = serde_json::from_str("{}").unwrap();
/// ```
pub struct PublicLocalDeployment {
    pub(crate) snapshot: Arc<PublicDeploymentSnapshot>,
}
pub(crate) struct PublicDeploymentSnapshot {
    pub(crate) bindings: StaticBindings,
    pub(crate) keys: BTreeMap<String, PublicDerivedCallKey>,
    pub(crate) manifest: PublicKeysetManifest,
    pub(crate) source: String,
    pub(crate) signer_policy: Option<crate::signed::FixedSignerPolicy>,
}
impl PublicLocalDeployment {
    pub(crate) fn admit_operation(&self, name: &[u8], operation: &ContractOperation) -> Result<()> {
        let name =
            std::str::from_utf8(name).map_err(|_| Error::new("identity", "operation-set"))?;
        self.snapshot
            .keys
            .get(name)
            .ok_or(Error::new("identity", "operation-set"))?
            .admit_operation(operation)
    }
}

use crate::{
    call_artifacts::{
        self, ActionKeyMetadata, ArtifactLeaves, ImportedProvingKeyset, ImportedPublicKeyset,
        ProvingKeysetManifest, PublicKeysetManifest,
    },
    parameters::PinnedCallParameters,
    proving::{self, DerivedCallMaterial, KeyGenerationLimits},
};
use midnight_transient_crypto::proofs::Zkir;
use std::path::Path;

#[derive(Clone)]
/// Only native source derivation can construct this selected proving capability.
/// ```compile_fail
/// let deployment = mori_ledger::local_deployment::DerivedLocalDeployment { public: Default::default(), selected: Default::default() };
/// ```
/// ```compile_fail
/// let _: mori_ledger::local_deployment::DerivedLocalDeployment = serde_json::from_str("{}").unwrap();
/// ```
pub struct DerivedLocalDeployment {
    pub(crate) public: PublicLocalDeployment,
    pub(crate) selected: DerivedCallMaterial,
    pub(crate) action: String,
}
impl DerivedLocalDeployment {
    pub fn public_deployment(&self) -> PublicLocalDeployment {
        self.public.clone()
    }
}
pub struct LocalInstalledSnapshot {
    pub(crate) descriptor: InstalledDeployment,
    pub(crate) predecessor: Vec<u8>,
    pub(crate) parameters: Vec<u8>,
}
impl LocalInstalledSnapshot {
    pub fn descriptor(&self) -> &InstalledDeployment {
        &self.descriptor
    }
    pub fn predecessor_bytes(&self) -> &[u8] {
        &self.predecessor
    }
    pub fn parameter_bytes(&self) -> &[u8] {
        &self.parameters
    }
}
fn refuse(code: &'static str) -> Error {
    Error::new("local-deployment", code)
}
// Metadata only: the native caller supplies its one discovered k result and,
// after acquisition/cache lookup, the snapshot's already-known degree.
fn check_degree(action: &str, degree: u8, ceiling: u8, cached: Option<u8>) -> Result<()> {
    use crate::{DegreeRefusalDiagnostic, DegreeRefusalReason};
    let pinned_supported = crate::parameters::expected_digest(degree).is_some();
    let reason = if degree > ceiling {
        Some(DegreeRefusalReason::CallerCeilingExceeded)
    } else if !pinned_supported {
        Some(DegreeRefusalReason::UnsupportedPinnedDegree)
    } else {
        cached
            .filter(|cached| *cached != degree)
            .map(|cached_degree| DegreeRefusalReason::CachedDegreeMismatch { cached_degree })
    };
    if let Some(reason) = reason {
        return Err(refuse("exact-degree-unavailable").with_degree_refusal(
            DegreeRefusalDiagnostic {
                action: action.into(),
                discovered_degree: degree,
                caller_ceiling: ceiling,
                pinned_supported,
                reason,
            },
        ));
    }
    Ok(())
}
pub(crate) fn compile_all(
    source: &str,
    p: &Program,
    bindings: &StaticBindings,
    limits: &NativeWorkLimits,
) -> Result<Vec<relation::Artifact>> {
    compile_all_policy(source, p, bindings, limits, None)
}
pub(crate) fn compile_all_policy(
    source: &str,
    p: &Program,
    bindings: &StaticBindings,
    limits: &NativeWorkLimits,
    policy: Option<&crate::signed::FixedSignerPolicy>,
) -> Result<Vec<relation::Artifact>> {
    let signed = policy
        .map(|policy| crate::signed::SignedStaticBindings::from_source(source, p, bindings, policy))
        .transpose()?;
    limits.validate()?;
    if source.len() > NativeWorkLimits::cap(limits.max_input_bytes)? {
        return Err(refuse("source-input-work-limit"));
    }
    call_artifacts::check_json_input(bindings, limits.max_input_bytes)?;
    if p.actions.is_empty() || p.actions.len() > NativeWorkLimits::cap(limits.max_actions)? {
        return Err(refuse("action-work-limit"));
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut artifacts = Vec::new();
    let mut total = 0;
    for a in &p.actions {
        if a.name.is_empty() || !seen.insert(&a.name) {
            return Err(refuse("action-set"));
        }
        let artifact = if let Some(signed) = &signed {
            crate::signed::compile_signed(signed, &a.name)?
        } else {
            relation::compile_bindings(source, p, bindings, &a.name)?
        };
        let pi = artifact.manifest.groups.iter().try_fold(2usize, |n, g| {
            n.checked_add(g.field_count)
                .ok_or(refuse("public-input-work-limit"))
        })?;
        if artifact.manifest.groups.len() > NativeWorkLimits::cap(limits.max_operations)?
            || pi > NativeWorkLimits::cap(limits.max_public_inputs)?
        {
            return Err(refuse("relation-work-limit"));
        }
        let ir = artifact.ir_bytes()?;
        let manifest = artifact.manifest_bytes()?;
        if ir.len() > NativeWorkLimits::cap(limits.max_leaf_bytes)?
            || manifest.len() > NativeWorkLimits::cap(limits.max_leaf_bytes)?
        {
            return Err(refuse("relation-leaf-limit"));
        }
        total = call_artifacts::add_total(
            total,
            ir.len(),
            NativeWorkLimits::cap(limits.max_artifact_bytes)?,
        )?;
        total = call_artifacts::add_total(
            total,
            manifest.len(),
            NativeWorkLimits::cap(limits.max_artifact_bytes)?,
        )?;
        artifacts.push(artifact);
    }
    Ok(artifacts)
}
// One new-degree acquisition, shared by the native consumer and ordinary early
// refusal tests. The caller's degree cache bypasses this for an existing snapshot.
fn load_new_parameters(
    params_dir: &Path,
    degree: u8,
    limits: &NativeWorkLimits,
    parameter_total: &mut usize,
) -> Result<PinnedCallParameters> {
    let snapshot_cap = call_artifacts::remaining_read_cap(
        *parameter_total,
        limits.max_srs_bytes,
        NativeWorkLimits::cap(limits.max_artifact_bytes)?,
    )?;
    let snapshot = PinnedCallParameters::load(params_dir, degree, snapshot_cap)?;
    *parameter_total = call_artifacts::add_total(
        *parameter_total,
        usize::try_from(snapshot.snapshot_bytes()).map_err(|_| refuse("parameter-size"))?,
        NativeWorkLimits::cap(limits.max_artifact_bytes)?,
    )?;
    Ok(snapshot)
}
pub(crate) async fn derive_all(
    source: &str,
    p: &Program,
    bindings: &StaticBindings,
    selected: Option<&str>,
    params_dir: &Path,
    limits: &NativeWorkLimits,
    artifacts: Vec<relation::Artifact>,
) -> Result<(PublicLocalDeployment, Option<DerivedCallMaterial>)> {
    if selected.is_some_and(|name| !p.actions.iter().any(|a| a.name == name)) {
        return Err(refuse("selected-action"));
    }
    let signer_policy = artifacts
        .first()
        .and_then(|a| a.manifest.signed.as_ref())
        .map(|s| s.signer_policy.clone());
    let mut params = BTreeMap::new();
    let mut keys = BTreeMap::new();
    let mut actions = Vec::new();
    let mut material = None;
    let mut total = source.len();
    let mut parameter_total = 0;
    for (ordinal, artifact) in artifacts.into_iter().enumerate() {
        // Native model selection can scan 9..25. This discovered-degree bound
        // does not contain those scans; keygen_vk/keygen each repeat SDK k work.
        let degree = artifact.ir.k();
        check_degree(&artifact.manifest.action, degree, limits.max_degree, None)?;
        if !params.contains_key(&degree) {
            let snapshot = load_new_parameters(params_dir, degree, limits, &mut parameter_total)?;
            params.insert(degree, snapshot);
        }
        let snapshot = &params[&degree];
        check_degree(
            &artifact.manifest.action,
            degree,
            limits.max_degree,
            Some(snapshot.degree()),
        )?;
        let public = proving::derive_public_artifact(
            &artifact,
            snapshot,
            NativeWorkLimits::cap(limits.max_verifier_key_bytes.min(limits.max_leaf_bytes))?,
        )
        .await?;
        let name = artifact.manifest.action.clone();
        if selected == Some(name.as_str()) {
            let derived = proving::derive_artifact_material(
                artifact,
                &name,
                snapshot.params(),
                degree,
                KeyGenerationLimits {
                    max_degree: limits.max_degree,
                    max_prover_key_bytes: NativeWorkLimits::cap(
                        limits.max_prover_key_bytes.min(limits.max_leaf_bytes),
                    )?,
                    max_verifier_key_bytes: NativeWorkLimits::cap(
                        limits.max_verifier_key_bytes.min(limits.max_leaf_bytes),
                    )?,
                },
            )
            .await?;
            if derived.public_key().verifier_key_bytes() != public.verifier_key_bytes() {
                return Err(refuse("selected-vk-reproducibility"));
            }
            total = call_artifacts::add_total(
                total,
                derived.prover_key_bytes().len(),
                NativeWorkLimits::cap(limits.max_artifact_bytes)?,
            )?;
            material = Some(derived);
        }
        for bytes in [
            public.ir_bytes(),
            public.source_manifest_bytes(),
            public.verifier_key_bytes(),
        ] {
            total = call_artifacts::add_total(
                total,
                bytes.len(),
                NativeWorkLimits::cap(limits.max_artifact_bytes)?,
            )?;
        }
        actions.push(ActionKeyMetadata {
            name: name.clone(),
            ordinal,
            ir_hash: codec::hash(public.ir_bytes()),
            manifest_hash: codec::hash(public.source_manifest_bytes()),
            vk_hash: codec::hash(public.verifier_key_bytes()),
            degree,
            srs_digest: snapshot.digest().into(),
            srs_bytes: snapshot.snapshot_bytes(),
        });
        keys.insert(name, public);
    }
    let mut manifest = PublicKeysetManifest {
        profile: if signer_policy.is_some() {
            crate::signed::KEYSET
        } else {
            "moriarty-native-call-public-keyset/1"
        }
        .into(),
        signer_policy: signer_policy.clone(),
        bindings: bindings.clone(),
        build: proving::source_closure(),
        actions,
        identity: String::new(),
        leaves: BTreeMap::new(),
    };
    manifest.identity = call_artifacts::public_identity(&manifest)?;
    Ok((
        PublicLocalDeployment {
            snapshot: Arc::new(PublicDeploymentSnapshot {
                bindings: bindings.clone(),
                keys,
                manifest,
                source: source.into(),
                signer_policy,
            }),
        },
        material,
    ))
}
pub async fn derive_local_deployment(
    seed: &AdmittedSeed,
    action: &str,
    params_dir: &Path,
    limits: &NativeWorkLimits,
) -> Result<DerivedLocalDeployment> {
    limits.validate()?;
    if seed.predecessor_bytes > NativeWorkLimits::cap(limits.max_input_bytes)?
        || seed.parameter_bytes > NativeWorkLimits::cap(limits.max_input_bytes)?
    {
        return Err(refuse("seed-input-work-limit"));
    }
    call_artifacts::check_json_input(&seed.descriptor, limits.max_input_bytes)?;
    let artifacts = compile_all(&seed.source, &seed.program, &seed.bindings, limits)?;
    let (public, selected) = derive_all(
        &seed.source,
        &seed.program,
        &seed.bindings,
        Some(action),
        params_dir,
        limits,
        artifacts,
    )
    .await?;
    Ok(DerivedLocalDeployment {
        public,
        selected: selected.ok_or(refuse("selected-action"))?,
        action: action.into(),
    })
}
fn preflight(
    source: &str,
    p: &Program,
    d: &InstalledDeployment,
    imported: &ImportedPublicKeyset,
    limits: &NativeWorkLimits,
) -> Result<Vec<relation::Artifact>> {
    limits.validate()?;
    d.validate_profile()?;
    call_artifacts::check_json_input(d, limits.max_input_bytes)?;
    if imported.snapshot_total > NativeWorkLimits::cap(limits.max_artifact_bytes)? {
        return Err(refuse("import-work-limit"));
    }
    if imported.manifest.bindings != d.bindings
        || imported.manifest.build != proving::source_closure()
        || imported.source != source.as_bytes()
        || d.keyset_identity != imported.manifest.identity
    {
        return Err(refuse("source-static-build-identity"));
    }
    let artifacts = compile_all(source, p, &d.bindings, limits)?;
    if imported.manifest.actions.len() != artifacts.len() {
        return Err(refuse("action-set"));
    }
    for (i, a) in artifacts.iter().enumerate() {
        let m = &imported.manifest.actions[i];
        let (ir, manifest, vk) = &imported.action_bytes[i];
        if vk.len()
            > NativeWorkLimits::cap(limits.max_verifier_key_bytes.min(limits.max_leaf_bytes))?
        {
            return Err(refuse("import-work-limit"));
        }
        if m.name != a.manifest.action
            || a.ir_bytes()? != *ir
            || a.manifest_bytes()? != *manifest
            || m.degree > limits.max_degree
            || crate::parameters::expected_digest(m.degree) != Some(m.srs_digest.as_str())
            || m.srs_bytes == 0
            || m.srs_bytes > limits.max_srs_bytes
        {
            return Err(refuse("source-ir-parameter-metadata"));
        }
    }
    Ok(artifacts)
}
fn compare_public(derived: &PublicLocalDeployment, imported: &ImportedPublicKeyset) -> Result<()> {
    if call_artifacts::public_identity(&derived.snapshot.manifest)? != imported.manifest.identity {
        return Err(refuse("regenerated-public-identity"));
    }
    for (m, (_, _, vk)) in imported.manifest.actions.iter().zip(&imported.action_bytes) {
        if derived.snapshot.keys[&m.name].verifier_key_bytes() != vk {
            return Err(refuse("regenerated-vk-bytes"));
        }
    }
    Ok(())
}
pub async fn rederive_local_public_deployment(
    source: &str,
    p: &Program,
    d: &InstalledDeployment,
    imported: &ImportedPublicKeyset,
    params_dir: &Path,
    limits: &NativeWorkLimits,
) -> Result<PublicLocalDeployment> {
    let artifacts = preflight(source, p, d, imported, limits)?;
    let (public, _) =
        derive_all(source, p, &d.bindings, None, params_dir, limits, artifacts).await?;
    compare_public(&public, imported)?;
    Ok(public)
}
pub async fn reopen_local_proving_deployment(
    source: &str,
    p: &Program,
    d: &InstalledDeployment,
    imported: &ImportedProvingKeyset,
    action: &str,
    params_dir: &Path,
    limits: &NativeWorkLimits,
) -> Result<DerivedLocalDeployment> {
    limits.validate()?;
    if imported.pk.len()
        > NativeWorkLimits::cap(limits.max_prover_key_bytes.min(limits.max_leaf_bytes))?
        || imported.snapshot_total > NativeWorkLimits::cap(limits.max_artifact_bytes)?
    {
        return Err(refuse("import-work-limit"));
    }
    if imported.action != action {
        return Err(refuse("selected-action"));
    }
    let artifacts = preflight(source, p, d, &imported.public, limits)?;
    let (public, selected) = derive_all(
        source,
        p,
        &d.bindings,
        Some(action),
        params_dir,
        limits,
        artifacts,
    )
    .await?;
    compare_public(&public, &imported.public)?;
    let selected = selected.ok_or(refuse("selected-action"))?;
    // Imported PK remains opaque forever. Only freshly regenerated material is
    // used by the provider after exact canonical reproducibility comparison.
    if selected.prover_key_bytes() != imported.pk {
        return Err(refuse("regenerated-pk-bytes"));
    }
    Ok(DerivedLocalDeployment {
        public,
        selected,
        action: action.into(),
    })
}
impl PublicLocalDeployment {
    pub(crate) fn identity(&self) -> &str {
        &self.snapshot.manifest.identity
    }
    pub(crate) fn key(&self, name: &str) -> Result<&PublicDerivedCallKey> {
        self.snapshot
            .keys
            .get(name)
            .ok_or(refuse("selected-action"))
    }
    pub(crate) fn admit_source(
        &self,
        source: &str,
        p: &Program,
        d: &InstalledDeployment,
        limits: &NativeWorkLimits,
    ) -> Result<Vec<relation::Artifact>> {
        if self.snapshot.signer_policy.is_some()
            || d.bindings != self.snapshot.bindings
            || d.keyset_identity != self.identity()
            || source != self.snapshot.source
            || p.actions.len() != self.snapshot.keys.len()
        {
            return Err(refuse("installed-source-policy"));
        }
        let artifacts = compile_all(source, p, &d.bindings, limits)?;
        for artifact in &artifacts {
            self.key(&artifact.manifest.action)?.admit(artifact)?;
        }
        Ok(artifacts)
    }
}
pub fn build_local_installed_snapshot(
    seed: AdmittedSeed,
    derived: &DerivedLocalDeployment,
    limits: &NativeWorkLimits,
) -> Result<LocalInstalledSnapshot> {
    if derived.public.snapshot.signer_policy.is_some() {
        return Err(refuse("unsigned-policy"));
    }
    build_snapshot(seed, derived, limits)
}
pub(crate) fn build_snapshot(
    seed: AdmittedSeed,
    derived: &DerivedLocalDeployment,
    limits: &NativeWorkLimits,
) -> Result<LocalInstalledSnapshot> {
    limits.validate()?;
    if seed.bindings != derived.public.snapshot.bindings
        || seed.source != derived.public.snapshot.source
    {
        return Err(refuse("seed-source-policy"));
    }
    let mut operations = Default::default();
    for (name, key) in &derived.public.snapshot.keys {
        operations = midnight_storage::storage::HashMap::insert(
            &operations,
            name.as_bytes().into(),
            key.local_operation()?,
        );
    }
    let ledger = replace_operations(
        &seed.frame.ledger,
        seed.frame.address,
        &seed.frame.contract,
        operations,
    );
    let contract = ledger
        .index(seed.frame.address)
        .ok_or(refuse("contract-absent"))?;
    ledger
        .check_night_balance_invariant()
        .map_err(|_| refuse("night-supply"))?;
    let predecessor =
        call_artifacts::encode_bounded(&ledger, limits.max_input_bytes.min(limits.max_leaf_bytes))?;
    let parameters = call_artifacts::encode_bounded(
        &*ledger.parameters,
        limits.max_input_bytes.min(limits.max_leaf_bytes),
    )?;
    let frame = FrameBindings {
        predecessor_hash: codec::hash(&predecessor),
        native_state_hash: codec::native_hex(&ledger.state_hash())?,
        parameters_hash: codec::hash(&parameters),
        gas_hex: seed.descriptor.gas_hex,
        operations_hash: codec::hash(&call_artifacts::encode_bounded(
            &contract.operations,
            limits.max_input_bytes,
        )?),
        maintenance_hash: codec::native_hash(&contract.maintenance_authority)?,
        balance_hash: codec::native_hash(&contract.balance)?,
    };
    let descriptor = InstalledDeployment {
        profile: INSTALLED_PROFILE.into(),
        qualification: INSTALLED_QUALIFICATION.into(),
        phase_policy: "guaranteed-only".into(),
        bindings: seed.bindings,
        frame,
        keyset_identity: derived.public.identity().into(),
    };
    // Full charged data/maintenance/balance and all non-target ledger components
    // are preserved by cloning; compare whole ledgers with only operations reset.
    let mut restored = ledger.clone();
    restored.contract = restored
        .contract
        .insert(seed.frame.address, seed.frame.contract.clone());
    if restored != seed.frame.ledger {
        return Err(refuse("seed-preservation"));
    }
    Ok(LocalInstalledSnapshot {
        descriptor,
        predecessor,
        parameters,
    })
}
pub fn export_local_deployment(
    snapshot: &LocalInstalledSnapshot,
    derived: &DerivedLocalDeployment,
    limits: &NativeWorkLimits,
) -> Result<ArtifactLeaves> {
    limits.validate()?;
    if derived.public.snapshot.manifest.actions.len() > NativeWorkLimits::cap(limits.max_actions)?
        || derived
            .public
            .snapshot
            .manifest
            .actions
            .iter()
            .any(|a| a.degree > limits.max_degree || a.srs_bytes > limits.max_srs_bytes)
        || derived.selected.prover_key_bytes().len()
            > NativeWorkLimits::cap(limits.max_prover_key_bytes)?
        || derived
            .public
            .snapshot
            .keys
            .values()
            .any(|k| k.verifier_key_bytes().len() as u64 > limits.max_verifier_key_bytes)
        || snapshot.predecessor.len() > NativeWorkLimits::cap(limits.max_input_bytes)?
        || snapshot.parameters.len() > NativeWorkLimits::cap(limits.max_input_bytes)?
    {
        return Err(refuse("export-work-limit"));
    }
    if snapshot.descriptor.keyset_identity != derived.public.identity() {
        return Err(refuse("snapshot-keyset"));
    }
    let mut leaves = BTreeMap::from([
        (
            "source.mori".into(),
            derived.public.snapshot.source.as_bytes().to_vec(),
        ),
        (
            "deployment.json".into(),
            call_artifacts::json_bytes(
                &snapshot.descriptor,
                limits.max_leaf_bytes.min(codec::INPUT_BYTES as u64),
            )?,
        ),
        ("predecessor.bin".into(), snapshot.predecessor.clone()),
        ("parameters.bin".into(), snapshot.parameters.clone()),
        (
            "selected.pk.bin".into(),
            derived.selected.prover_key_bytes().to_vec(),
        ),
    ]);
    for m in &derived.public.snapshot.manifest.actions {
        let key = derived.public.key(&m.name)?;
        for (suffix, bytes) in [
            ("zkir.bin", key.ir_bytes()),
            ("manifest.json", key.source_manifest_bytes()),
            ("vk.bin", key.verifier_key_bytes()),
        ] {
            leaves.insert(
                call_artifacts::ordinal_leaf(m.ordinal, suffix),
                bytes.to_vec(),
            );
        }
    }
    let proving = ProvingKeysetManifest {
        profile: if derived.public.snapshot.signer_policy.is_some() {
            crate::signed::KEYSET
        } else {
            "moriarty-native-call-proving-keyset/1"
        }
        .into(),
        public_identity: derived.public.identity().into(),
        action: derived.action.clone(),
        prover_key: call_artifacts::leaf(derived.selected.prover_key_bytes()),
    };
    leaves.insert(
        "proving-keyset.json".into(),
        call_artifacts::json_bytes(
            &proving,
            limits.max_leaf_bytes.min(codec::INPUT_BYTES as u64),
        )?,
    );
    let mut manifest = derived.public.snapshot.manifest.clone();
    manifest.leaves = leaves
        .iter()
        .map(|(n, b)| (n.clone(), call_artifacts::leaf(b)))
        .collect();
    let output = ArtifactLeaves {
        leaves,
        marker_name: "public-keyset.json",
        marker: call_artifacts::json_bytes(
            &manifest,
            limits.max_leaf_bytes.min(codec::INPUT_BYTES as u64),
        )?,
    };
    output.check(limits)?;
    Ok(output)
}

#[cfg(test)]
#[path = "../tests/support/mod.rs"]
mod ordinary_support;
#[cfg(test)]
mod ordinary_tests {
    use super::*;
    use crate::{DegreeRefusalDiagnostic, DegreeRefusalReason};

    #[test]
    fn degree_guard_preserves_reasons_precedence_and_current_map_action() {
        for (action, degree, ceiling, cached, supported, reason) in [
            (
                "freeze",
                14,
                13,
                None,
                true,
                DegreeRefusalReason::CallerCeilingExceeded,
            ),
            (
                "unused-map-action",
                20,
                25,
                None,
                false,
                DegreeRefusalReason::UnsupportedPinnedDegree,
            ),
            (
                "both-fail",
                20,
                13,
                Some(12),
                false,
                DegreeRefusalReason::CallerCeilingExceeded,
            ),
            (
                "freeze",
                13,
                13,
                Some(12),
                true,
                DegreeRefusalReason::CachedDegreeMismatch { cached_degree: 12 },
            ),
            (
                "zero",
                0,
                0,
                Some(1),
                true,
                DegreeRefusalReason::CachedDegreeMismatch { cached_degree: 1 },
            ),
        ] {
            let error = check_degree(action, degree, ceiling, cached).unwrap_err();
            assert_eq!(error.layer, "local-deployment");
            assert_eq!(error.code, "exact-degree-unavailable");
            assert_eq!(
                error.degree_refusal(),
                Some(&DegreeRefusalDiagnostic {
                    action: action.into(),
                    discovered_degree: degree,
                    caller_ceiling: ceiling,
                    pinned_supported: supported,
                    reason,
                })
            );
        }
    }

    #[test]
    fn degree_guard_boundaries_only_admit_metadata() {
        for (degree, ceiling, cached) in [
            (0, 0, None),
            (0, 0, Some(0)),
            (13, 13, None),
            (13, 13, Some(13)),
            (19, 25, Some(19)),
        ] {
            assert!(check_degree("freeze", degree, ceiling, cached).is_ok());
        }
        assert!(check_degree("freeze", 14, 13, None).is_err());
        assert!(check_degree("freeze", 20, 25, None).is_err());
        assert_eq!(refuse("selected-action").degree_refusal(), None);
    }
    fn limits() -> NativeWorkLimits {
        NativeWorkLimits {
            profile: "moriarty-native-call-work-limits/1".into(),
            max_degree: 19,
            max_actions: 4,
            max_srs_bytes: 1024,
            max_prover_key_bytes: 1024,
            max_verifier_key_bytes: 1024,
            max_input_bytes: 1_048_576,
            max_leaf_bytes: 1_048_576,
            max_artifact_bytes: 4_194_304,
            max_operations: 10000,
            max_public_inputs: 10000,
        }
    }
    #[test]
    fn exhausted_parameter_aggregate_refuses_before_open() {
        let mut limits = limits();
        limits.max_artifact_bytes = 4;
        let mut charged = 4;
        assert_eq!(
            load_new_parameters(
                Path::new("/missing-mori-parameter-budget-test"),
                9,
                &limits,
                &mut charged
            )
            .err()
            .unwrap()
            .code,
            "aggregate-size"
        );
        assert_eq!(charged, 4);
    }
    #[test]
    fn next_unique_degree_uses_remaining_parameter_budget_before_hash() {
        let root =
            std::env::temp_dir().join(format!("mori-next-parameter-budget-{}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        std::fs::write(root.join("bls_midnight_2p10"), [0, 0]).unwrap();
        let mut limits = limits();
        limits.max_artifact_bytes = 4;
        let mut charged = 3;
        let err = load_new_parameters(&root, 10, &limits, &mut charged)
            .err()
            .unwrap();
        std::fs::remove_dir_all(root).unwrap();
        assert_eq!(err.code, "parameter-size");
        assert_eq!(charged, 3);
    }
    #[test]
    fn valid_source_cannot_bypass_smaller_explicit_library_input_limit() {
        let f = ordinary_support::payment();
        let bindings = StaticBindings::legacy(&f.deployment);
        let mut limits = limits();
        limits.max_input_bytes = 1;
        assert_eq!(
            compile_all(&f.source, &f.program, &bindings, &limits)
                .err()
                .unwrap()
                .code,
            "source-input-work-limit"
        );
        limits.max_input_bytes = f.source.len() as u64;
        assert_eq!(
            compile_all(&f.source, &f.program, &bindings, &limits)
                .err()
                .unwrap()
                .code,
            "typed-input-size"
        );
    }
    #[test]
    fn source_only_preflight_checks_unused_action_ir_without_minting_keys() {
        let source = include_str!("../tests/fixtures/ToggleCounter.mori");
        let f = ordinary_support::fixture(source, "step", mori_core::Scenario::default());
        let bindings = StaticBindings::legacy(&f.deployment);
        let limits = limits();
        let artifacts = compile_all(source, &f.program, &bindings, &limits).unwrap();
        let action_bytes = artifacts
            .iter()
            .map(|a| {
                (
                    a.ir_bytes().unwrap(),
                    a.manifest_bytes().unwrap(),
                    b"untrusted-opaque-vk".to_vec(),
                )
            })
            .collect::<Vec<_>>();
        let actions = artifacts
            .iter()
            .enumerate()
            .map(|(ordinal, a)| ActionKeyMetadata {
                name: a.manifest.action.clone(),
                ordinal,
                ir_hash: codec::hash(&action_bytes[ordinal].0),
                manifest_hash: codec::hash(&action_bytes[ordinal].1),
                vk_hash: codec::hash(&action_bytes[ordinal].2),
                degree: 9,
                srs_digest: crate::parameters::expected_digest(9).unwrap().into(),
                srs_bytes: 4,
            })
            .collect();
        let mut manifest = PublicKeysetManifest {
            signer_policy: None,
            profile: "moriarty-native-call-public-keyset/1".into(),
            bindings: bindings.clone(),
            build: proving::source_closure(),
            actions,
            identity: String::new(),
            leaves: Default::default(),
        };
        manifest.identity = call_artifacts::public_identity(&manifest).unwrap();
        let mut imported = ImportedPublicKeyset {
            manifest,
            source: source.as_bytes().to_vec(),
            action_bytes,
            snapshot_total: 0,
        };
        let mut descriptor = InstalledDeployment {
            profile: INSTALLED_PROFILE.into(),
            qualification: INSTALLED_QUALIFICATION.into(),
            phase_policy: "guaranteed-only".into(),
            bindings,
            frame: FrameBindings::legacy(&f.deployment),
            keyset_identity: imported.manifest.identity.clone(),
        };
        // This only admits ordinary metadata/source compilation. Arbitrary VK
        // bytes remain untrusted; native regeneration is deliberately unrun.
        assert!(preflight(source, &f.program, &descriptor, &imported, &limits).is_ok());
        imported.action_bytes[1].0.push(0);
        imported.manifest.actions[1].ir_hash = codec::hash(&imported.action_bytes[1].0);
        imported.manifest.identity = call_artifacts::public_identity(&imported.manifest).unwrap();
        descriptor.keyset_identity = imported.manifest.identity.clone();
        assert_eq!(
            preflight(source, &f.program, &descriptor, &imported, &limits)
                .err()
                .unwrap()
                .code,
            "source-ir-parameter-metadata"
        );
        imported.manifest.build = "changed".into();
        assert_eq!(
            preflight(source, &f.program, &descriptor, &imported, &limits)
                .err()
                .unwrap()
                .code,
            "source-static-build-identity"
        );
    }
}

fn replace_operations(
    ledger: &layout::NativeState,
    address: midnight_coin_structure::contract::ContractAddress,
    contract: &midnight_onchain_state::state::ContractState<midnight_storage::db::InMemoryDB>,
    operations: midnight_storage::storage::HashMap<
        midnight_onchain_state::state::EntryPointBuf,
        ContractOperation,
        midnight_storage::db::InMemoryDB,
    >,
) -> layout::NativeState {
    let mut contract = contract.clone();
    contract.operations = operations;
    let mut ledger = ledger.clone();
    ledger.contract = ledger.contract.insert(address, contract);
    ledger
}
#[cfg(test)]
mod frame_preservation_tests {
    use super::*;
    #[test]
    fn operations_only_replacement_preserves_whole_ledger_and_charged_data() {
        let f = ordinary_support::payment();
        let mut ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
        let address = codec::from_hex(&f.deployment.address_hex).unwrap();
        let token = midnight_coin_structure::coin::UnshieldedTokenType(
            midnight_base_crypto::hash::HashOutput([97; 32]),
        );
        let mut utxo = (*ledger.utxo).clone();
        let sentinel = midnight_ledger::structure::Utxo {
            owner: Default::default(),
            value: 17,
            type_: token,
            intent_hash: midnight_ledger::structure::IntentHash(
                midnight_base_crypto::hash::HashOutput([98; 32]),
            ),
            output_no: 8,
        };
        utxo.utxos = utxo.utxos.insert(
            sentinel.clone(),
            midnight_ledger::structure::UtxoMeta {
                ctime: midnight_base_crypto::time::Timestamp::from_secs(3),
            },
        );
        ledger.utxo = midnight_storage::arena::Sp::new(utxo);
        ledger.treasury = ledger.treasury.insert(
            midnight_coin_structure::coin::TokenType::Unshielded(token),
            23,
        );
        let contract = ledger.index(address).unwrap();
        // Empty/keyless operation bytes exercise cloning only. No derived key
        // capability, installed admission or proof-success object is minted.
        let result = replace_operations(&ledger, address, &contract, Default::default());
        let changed = result.index(address).unwrap();
        assert_eq!(changed.data, contract.data);
        assert_eq!(changed.balance, contract.balance);
        assert_eq!(
            changed.maintenance_authority,
            contract.maintenance_authority
        );
        assert_eq!(result.utxo, ledger.utxo);
        assert_eq!(result.treasury, ledger.treasury);
        let mut restored = result;
        restored.contract = restored.contract.insert(address, contract);
        assert_eq!(restored, ledger);
    }
}
