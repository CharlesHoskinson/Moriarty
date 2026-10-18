use super::*;
use crate::{
    call_artifacts::{self, ArtifactLeaves, ImportedProvingKeyset, ImportedPublicKeyset},
    local_deployment::{self, AdmittedSeed, DerivedLocalDeployment, PublicLocalDeployment},
    proving,
};
use std::path::Path;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedInstalledDeployment {
    pub profile: String,
    pub qualification: String,
    pub phase_policy: String,
    pub storage_abi: String,
    pub signed_abi: String,
    #[serde(deserialize_with = "strict_field")]
    pub bindings: StaticBindings,
    pub frame: FrameBindings,
    pub keyset_identity: String,
    pub signer_policy: FixedSignerPolicy,
}
impl SignedInstalledDeployment {
    pub fn from_json(text: &str) -> Result<Self> {
        let d: Self = codec::json(text)?;
        d.validate()?;
        Ok(d)
    }
    pub fn version(&self) -> Result<SignedVersion> {
        for v in [SignedVersion::LegacyV1, SignedVersion::ConsumingV2] {
            if self.profile == v.profile()
                && self.storage_abi == v.storage_abi()
                && self.signed_abi == v.signed_abi()
            {
                return Ok(v);
            }
        }
        Err(refuse("signed-version"))
    }
    pub(crate) fn validate(&self) -> Result<()> {
        self.version()?;
        if self.qualification != QUALIFICATION
            || self.phase_policy != "guaranteed-only"
            || self.bindings.economic_bounds.is_some()
            || self.bindings.abi != ABI
            || self.bindings.anchor != "local-unqualified"
        {
            return Err(refuse("deployment-profile"));
        }
        self.signer_policy.validate()
    }
}
pub struct SignedAdmittedSeed {
    pub(crate) seed: AdmittedSeed,
    pub(crate) static_bindings: SignedStaticBindings,
}

/// Derive a descriptor for an independently supplied current snapshot. This
/// does not prove ancestry, freshness, or ledger application of earlier calls.
#[allow(clippy::too_many_arguments)]
pub fn derive_signed_supplied_frame(
    source: &str,
    program: &Program,
    descriptor: &SignedInstalledDeployment,
    policy: &FixedSignerPolicy,
    predecessor: &[u8],
    parameters: &[u8],
    public: &SignedPublicLocalDeployment,
    limits: &NativeWorkLimits,
) -> Result<SignedInstalledDeployment> {
    let s = SignedStaticBindings::from_source(
        source,
        program,
        &descriptor.bindings,
        policy,
        descriptor.version()?,
    )?;
    if s.version != public.static_bindings.version
        || s.source != public.static_bindings.source
        || s.bindings != public.static_bindings.bindings
        || s.policy != public.static_bindings.policy
        || descriptor.keyset_identity != public.inner.identity()
        || descriptor.signer_policy != *policy
    {
        return Err(refuse("supplied-frame-policy"));
    }
    let artifacts = local_deployment::compile_all_policy(
        source,
        program,
        &descriptor.bindings,
        limits,
        Some((policy, s.version)),
    )?;
    for artifact in &artifacts {
        public
            .inner
            .key(&artifact.manifest.action)?
            .admit(artifact)?;
    }
    derive_signed_supplied_frame_core(
        &s,
        descriptor,
        predecessor,
        parameters,
        limits,
        layout::OperationPolicy::Installed {
            public: &public.inner,
            limits,
        },
    )
}

pub(crate) fn derive_signed_supplied_frame_core(
    s: &SignedStaticBindings,
    descriptor: &SignedInstalledDeployment,
    predecessor: &[u8],
    parameters: &[u8],
    limits: &NativeWorkLimits,
    operations: layout::OperationPolicy<'_>,
) -> Result<SignedInstalledDeployment> {
    validate_work_limits(limits)?;
    descriptor.validate()?;
    if descriptor.version()? != SignedVersion::ConsumingV2
        || s.version != SignedVersion::ConsumingV2
        || s.bindings != descriptor.bindings
        || s.policy != descriptor.signer_policy
    {
        return Err(refuse("supplied-frame-policy"));
    }
    call_artifacts::check_json_input(descriptor, limits.max_input_bytes)?;
    let state: layout::NativeState =
        call_artifacts::decode_bounded(predecessor, limits.max_input_bytes)?;
    let params: midnight_ledger::structure::LedgerParameters =
        call_artifacts::decode_bounded(parameters, limits.max_input_bytes)?;
    if codec::hash(parameters) != descriptor.frame.parameters_hash || *state.parameters != params {
        return Err(refuse("parameters"));
    }
    let mut derived = descriptor.clone();
    derived.frame.predecessor_hash = codec::hash(predecessor);
    derived.frame.native_state_hash = codec::native_hex(&state.state_hash())?;
    let abi = layout::admit_bindings(&s.program, &s.bindings)?;
    let frame = layout::admit_signed_frame(s, &abi, &derived.frame, state, operations)?;
    layout::frame_gas(&frame)?;
    // Every other frame commitment and the complete operation map were checked
    // by admission against the independently bound installed/keyless policy.
    Ok(derived)
}
pub fn admit_signed_local_seed(
    source: &str,
    p: &Program,
    d: &Deployment,
    predecessor: &[u8],
    parameters: &[u8],
    policy: &FixedSignerPolicy,
) -> Result<SignedAdmittedSeed> {
    if d.profile == crate::profile::PROFILE {
        return Err(refuse("legacy-signed-seed"));
    }
    if d.profile != CONSUMING_SEED
        || d.qualification != "local-unproven"
        || d.phase_policy != "guaranteed-only"
    {
        return Err(refuse("signed-version"));
    }
    let bindings = StaticBindings::legacy(d);
    let static_bindings = SignedStaticBindings::from_source(
        source,
        p,
        &bindings,
        policy,
        SignedVersion::ConsumingV2,
    )?;
    let state: layout::NativeState = codec::decode_native(predecessor)?;
    let params: midnight_ledger::structure::LedgerParameters = codec::decode_native(parameters)?;
    if codec::hash(parameters) != d.parameters_hash || *state.parameters != params {
        return Err(refuse("parameters"));
    }
    let abi = layout::admit_bindings(p, &bindings)?;
    let frame = layout::admit_signed_frame(
        &static_bindings,
        &abi,
        &FrameBindings::legacy(d),
        state,
        layout::OperationPolicy::Keyless,
    )?;
    layout::frame_gas(&frame)?;
    if nonce::authority_map(&static_bindings, frame.contract.data.get_ref())?.size() != 1 {
        return Err(refuse("signed-seed-authority"));
    }
    let seed = AdmittedSeed {
        frame,
        bindings,
        descriptor: d.clone(),
        source: source.into(),
        program: p.clone(),
        predecessor_bytes: predecessor.len(),
        parameter_bytes: parameters.len(),
    };

    Ok(SignedAdmittedSeed {
        seed,
        static_bindings,
    })
}
/// A parsed keyset cannot mint an original-source public capability.
/// ```compile_fail
/// let _: mori_ledger::signed::SignedPublicLocalDeployment = serde_json::from_str("{}").unwrap();
/// ```
#[derive(Clone)]
pub struct SignedPublicLocalDeployment {
    pub(crate) inner: PublicLocalDeployment,
    pub(crate) static_bindings: SignedStaticBindings,
}
/// ```compile_fail
/// let _: mori_ledger::signed::SignedDerivedLocalDeployment = serde_json::from_str("{}").unwrap();
/// ```
#[derive(Clone)]
pub struct SignedDerivedLocalDeployment {
    pub(crate) inner: DerivedLocalDeployment,
    pub(crate) static_bindings: SignedStaticBindings,
}
impl SignedDerivedLocalDeployment {
    pub fn public_deployment(&self) -> SignedPublicLocalDeployment {
        SignedPublicLocalDeployment {
            inner: self.inner.public.clone(),
            static_bindings: self.static_bindings.clone(),
        }
    }
}
pub struct SignedLocalInstalledSnapshot {
    pub(crate) inner: local_deployment::LocalInstalledSnapshot,
    pub(crate) descriptor: SignedInstalledDeployment,
}
impl SignedLocalInstalledSnapshot {
    pub fn descriptor(&self) -> &SignedInstalledDeployment {
        &self.descriptor
    }
    pub fn predecessor_bytes(&self) -> &[u8] {
        self.inner.predecessor_bytes()
    }
    pub fn parameter_bytes(&self) -> &[u8] {
        self.inner.parameter_bytes()
    }
}
pub async fn derive_signed_local_deployment(
    seed: &SignedAdmittedSeed,
    action: &str,
    params_dir: &Path,
    limits: &NativeWorkLimits,
) -> Result<SignedDerivedLocalDeployment> {
    validate_work_limits(limits)?;
    if seed.seed.predecessor_bytes > NativeWorkLimits::cap(limits.max_input_bytes)?
        || seed.seed.parameter_bytes > NativeWorkLimits::cap(limits.max_input_bytes)?
    {
        return Err(refuse("seed-input-work-limit"));
    }
    call_artifacts::check_json_input(&seed.seed.descriptor, limits.max_input_bytes)?;
    let s = &seed.static_bindings;
    let artifacts = local_deployment::compile_all_policy(
        &s.source,
        &s.program,
        &s.bindings,
        limits,
        Some((&s.policy, s.version)),
    )?;
    let (public, selected) = local_deployment::derive_all(
        &s.source,
        &s.program,
        &s.bindings,
        Some(action),
        params_dir,
        limits,
        artifacts,
    )
    .await?;
    Ok(SignedDerivedLocalDeployment {
        inner: DerivedLocalDeployment {
            public,
            selected: selected.ok_or(refuse("action"))?,
            action: action.into(),
        },
        static_bindings: s.clone(),
    })
}
pub fn build_signed_local_installed_snapshot(
    seed: SignedAdmittedSeed,
    derived: &SignedDerivedLocalDeployment,
    limits: &NativeWorkLimits,
) -> Result<SignedLocalInstalledSnapshot> {
    if seed.static_bindings.policy != derived.static_bindings.policy
        || seed.static_bindings.version != SignedVersion::ConsumingV2
        || seed.static_bindings.version != derived.static_bindings.version
    {
        return Err(refuse("seed-policy"));
    }
    let inner = local_deployment::build_snapshot(seed.seed, &derived.inner, limits)?;
    let d = inner.descriptor();
    let descriptor = SignedInstalledDeployment {
        profile: derived.static_bindings.version.profile().into(),
        qualification: QUALIFICATION.into(),
        phase_policy: d.phase_policy.clone(),
        storage_abi: derived.static_bindings.version.storage_abi().into(),
        signed_abi: derived.static_bindings.version.signed_abi().into(),
        bindings: d.bindings.clone(),
        frame: d.frame.clone(),
        keyset_identity: d.keyset_identity.clone(),
        signer_policy: derived.static_bindings.policy.clone(),
    };
    Ok(SignedLocalInstalledSnapshot { inner, descriptor })
}
pub fn export_signed_local_deployment(
    s: &SignedLocalInstalledSnapshot,
    d: &SignedDerivedLocalDeployment,
    l: &NativeWorkLimits,
) -> Result<ArtifactLeaves> {
    validate_work_limits(l)?;
    // Check the Signed input replacement before the inner factory copies its
    // source/PK leaves. Generated output JSON and marker caps are unchanged.
    let replacement = if l.version()? == crate::profile::WorkLimitsVersion::V2 {
        call_artifacts::check_export_input_lengths(
            d.inner.public.snapshot.source.len(),
            s.inner.predecessor_bytes().len(),
            s.inner.parameter_bytes().len(),
            l,
        )?;
        Some(call_artifacts::deployment_json_bytes(&s.descriptor, l)?)
    } else {
        None
    };
    let mut out = local_deployment::export_local_deployment(&s.inner, &d.inner, l)?;
    out.leaves.insert(
        "deployment.json".into(),
        match replacement {
            Some(bytes) => bytes,
            None => call_artifacts::deployment_json_bytes(&s.descriptor, l)?,
        },
    );
    let mut manifest = d.inner.public.snapshot.manifest.clone();
    manifest.leaves = out
        .leaves
        .iter()
        .map(|(n, b)| (n.clone(), call_artifacts::leaf(b)))
        .collect();
    out.marker =
        call_artifacts::json_bytes(&manifest, l.max_leaf_bytes.min(codec::INPUT_BYTES as u64))?;
    call_artifacts::check_keyset_export(&out, &d.inner.selected, l)?;
    Ok(out)
}
pub fn snapshot_signed_public_keyset(
    path: &Path,
    l: &NativeWorkLimits,
    version: SignedVersion,
) -> Result<ImportedPublicKeyset> {
    validate_work_limits(l)?;
    call_artifacts::snapshot_public_profile(path, l, Some(version))
}
pub fn snapshot_signed_proving_keyset(
    path: &Path,
    action: &str,
    l: &NativeWorkLimits,
    version: SignedVersion,
) -> Result<ImportedProvingKeyset> {
    validate_work_limits(l)?;
    call_artifacts::snapshot_proving_profile(path, action, l, Some(version))
}
pub(super) fn preflight(
    source: &str,
    p: &Program,
    d: &SignedInstalledDeployment,
    policy: &FixedSignerPolicy,
    i: &ImportedPublicKeyset,
    l: &NativeWorkLimits,
) -> Result<(SignedStaticBindings, Vec<crate::relation::Artifact>)> {
    d.validate()?;
    validate_work_limits(l)?;
    call_artifacts::check_json_input(d, l.max_input_bytes)?;
    let s = SignedStaticBindings::from_source(source, p, &d.bindings, policy, d.version()?)?;
    if *policy != d.signer_policy
        || i.manifest.signer_policy.as_ref() != Some(policy)
        || i.manifest.bindings != d.bindings
        || i.manifest.build != proving::source_closure()
        || i.source != source.as_bytes()
        || i.manifest.identity != d.keyset_identity
        || i.snapshot_total > NativeWorkLimits::cap(l.max_artifact_bytes)?
    {
        return Err(refuse("source-static-build-policy"));
    }
    let artifacts = local_deployment::compile_all_policy(
        source,
        p,
        &d.bindings,
        l,
        Some((policy, d.version()?)),
    )?;
    if artifacts.len() != i.manifest.actions.len() || i.action_bytes.len() != artifacts.len() {
        return Err(refuse("action-set"));
    }
    for (index, a) in artifacts.iter().enumerate() {
        let m = &i.manifest.actions[index];
        let (ir, manifest, vk) = &i.action_bytes[index];
        if m.name != a.manifest.action
            || m.ordinal != index
            || *ir != a.ir_bytes()?
            || *manifest != a.manifest_bytes()?
            || vk.len() > NativeWorkLimits::cap(l.max_verifier_key_bytes.min(l.max_leaf_bytes))?
            || m.degree > l.max_degree
            || crate::parameters::expected_digest(m.degree) != Some(m.srs_digest.as_str())
            || m.srs_bytes == 0
            || m.srs_bytes > l.max_srs_bytes
        {
            return Err(refuse("source-ir-parameter-metadata"));
        }
    }
    Ok((s, artifacts))
}
fn compare(public: &PublicLocalDeployment, i: &ImportedPublicKeyset) -> Result<()> {
    if public.identity() != i.manifest.identity {
        return Err(refuse("regenerated-public-identity"));
    }
    for (m, (_, _, vk)) in i.manifest.actions.iter().zip(&i.action_bytes) {
        if public.key(&m.name)?.verifier_key_bytes() != vk {
            return Err(refuse("regenerated-vk"));
        }
    }
    Ok(())
}
pub async fn rederive_signed_local_public_deployment(
    source: &str,
    p: &Program,
    d: &SignedInstalledDeployment,
    policy: &FixedSignerPolicy,
    i: &ImportedPublicKeyset,
    params_dir: &Path,
    l: &NativeWorkLimits,
) -> Result<SignedPublicLocalDeployment> {
    let (s, artifacts) = preflight(source, p, d, policy, i, l)?;
    let (public, _) =
        local_deployment::derive_all(source, p, &d.bindings, None, params_dir, l, artifacts)
            .await?;
    compare(&public, i)?;
    Ok(SignedPublicLocalDeployment {
        inner: public,
        static_bindings: s,
    })
}
#[allow(clippy::too_many_arguments)]
pub async fn reopen_signed_local_proving_deployment(
    source: &str,
    p: &Program,
    d: &SignedInstalledDeployment,
    policy: &FixedSignerPolicy,
    i: &ImportedProvingKeyset,
    action: &str,
    params_dir: &Path,
    l: &NativeWorkLimits,
) -> Result<SignedDerivedLocalDeployment> {
    match l.version()? {
        crate::profile::WorkLimitsVersion::V1 => {
            // Preserve legacy short-circuit refusal priority. Unrelated limits
            // are validated later by the original source preflight.
            if i.action != action
                || i.pk.len() > NativeWorkLimits::cap(l.max_prover_key_bytes.min(l.max_leaf_bytes))?
                || i.counts.physical > NativeWorkLimits::cap(l.max_artifact_bytes)?
            {
                return Err(refuse("import-work-limit"));
            }
        }
        crate::profile::WorkLimitsVersion::V2 => {
            validate_work_limits(l)?;
            if i.action != action
                || i.pk.len() > l.prover_key_cap()?
                || call_artifacts::keyset_counts(i.counts.public, i.pk.len(), l).is_err()
            {
                return Err(refuse("import-work-limit"));
            }
        }
    }
    let (s, artifacts) = preflight(source, p, d, policy, &i.public, l)?;
    let (public, selected) = local_deployment::derive_all(
        source,
        p,
        &d.bindings,
        Some(action),
        params_dir,
        l,
        artifacts,
    )
    .await?;
    compare(&public, &i.public)?;
    let selected = selected.ok_or(refuse("action"))?;
    if selected.prover_key_bytes() != i.pk {
        return Err(refuse("regenerated-pk"));
    }
    Ok(SignedDerivedLocalDeployment {
        inner: DerivedLocalDeployment {
            public,
            selected,
            action: action.into(),
        },
        static_bindings: s,
    })
}
