use super::*;
use crate::{
    call_artifacts,
    prepare::{self, PreparedBundle, PreparedInstalledCall, PreparedNativeCall},
    relation::Artifact,
};
use midnight_base_crypto::fab::AlignedValue;
use midnight_ledger::structure::LedgerParameters;
use midnight_transient_crypto::{curve::Fr, fab::AlignedValueExt};
pub const RESULT_SCOPE: &str =
    "local-signed-call-proof-fixed-jubjub-policy-supplied-state-unqualified";
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedProofContext {
    pub profile: String,
    pub policy_sha256: String,
    pub payload_sha256: String,
    pub context_sha256: String,
    pub attachment_sha256: String,
}
/// No public DTO or edited package can construct this semantic owner.
/// ```compile_fail
/// let _: mori_ledger::signed::PreparedCallSigningStatement = serde_json::from_str("{}").unwrap();
/// ```
pub struct PreparedCallSigningStatement {
    pub(crate) package: CallSigningPackage,
    pub(crate) static_bindings: SignedStaticBindings,
    pub(crate) descriptor: SignedInstalledDeployment,
    pub(crate) predecessor: Vec<u8>,
    pub(crate) parameters: Vec<u8>,
    pub(crate) request: SignedCallRequest,
    pub(crate) artifact: Artifact,
    pub(crate) public: SignedPublicLocalDeployment,
    pub(crate) limits: NativeWorkLimits,
}
impl PreparedCallSigningStatement {
    pub fn package(&self) -> &CallSigningPackage {
        &self.package
    }
}
/// Full typed input and retained native operation after original-source/keymap
/// and exact external attachment admission. No Deserialize or From exists.
/// ```compile_fail
/// let _: mori_ledger::signed::PreparedSignedInstalledCall = serde_json::from_str("{}").unwrap();
/// ```
pub struct PreparedSignedInstalledCall {
    pub(crate) inner: PreparedInstalledCall,
    pub(crate) package: CallSigningPackage,
}
impl PreparedSignedInstalledCall {
    pub fn bundle(&self) -> &PreparedBundle {
        self.inner.bundle()
    }
    pub fn package(&self) -> &CallSigningPackage {
        &self.package
    }
}
fn decode_predecessor(
    s: &SignedStaticBindings,
    d: &SignedInstalledDeployment,
    predecessor: &[u8],
    parameters: &[u8],
    request: &SignedCallRequest,
    public: &crate::local_deployment::PublicLocalDeployment,
    l: &NativeWorkLimits,
) -> Result<(layout::AdmittedPredecessor, LedgerParameters)> {
    let state: layout::NativeState =
        call_artifacts::decode_bounded(predecessor, l.max_input_bytes)?;
    let params: LedgerParameters = call_artifacts::decode_bounded(parameters, l.max_input_bytes)?;
    if codec::hash(parameters) != d.frame.parameters_hash || *state.parameters != params {
        return Err(refuse("parameters"));
    }
    let abi = layout::admit_bindings(&s.program, &d.bindings)?;
    let frame = layout::admit_signed_frame(
        s,
        &abi,
        &d.frame,
        state,
        layout::OperationPolicy::Installed { public, limits: l },
    )?;
    Ok((
        layout::attach_request(frame, &d.bindings, &request.native())?,
        params,
    ))
}
pub fn prepare_installed_call_signing_statement(
    source: &str,
    p: &Program,
    d: &SignedInstalledDeployment,
    policy: &FixedSignerPolicy,
    predecessor: &[u8],
    parameters: &[u8],
    request: &SignedCallRequest,
    action: &str,
    public: &SignedPublicLocalDeployment,
    l: &NativeWorkLimits,
) -> Result<PreparedCallSigningStatement> {
    validate_work_limits(l)?;
    d.validate()?;
    call_artifacts::check_json_input(d, l.max_input_bytes)?;
    call_artifacts::check_json_input(request, l.max_input_bytes.min(SIGNED_JSON_BYTES as u64))?;
    let s = SignedStaticBindings::from_source(source, p, &d.bindings, policy, d.version()?)?;
    request.admit(&s, action)?;
    if *policy != d.signer_policy
        || s.version != public.static_bindings.version
        || s.source != public.static_bindings.source
        || s.bindings != public.static_bindings.bindings
        || *policy != public.static_bindings.policy
        || public.inner.identity() != d.keyset_identity
    {
        return Err(refuse("derived-source-policy"));
    }
    let artifacts = crate::local_deployment::compile_all_policy(
        source,
        p,
        &d.bindings,
        l,
        Some((policy, d.version()?)),
    )?;
    for a in &artifacts {
        public.inner.key(&a.manifest.action)?.admit(a)?;
    }
    let artifact = artifacts
        .into_iter()
        .find(|a| a.manifest.action == action)
        .ok_or(refuse("action"))?;
    let (pre, params) =
        decode_predecessor(&s, d, predecessor, parameters, request, &public.inner, l)?;
    let bundle = prepare::prepare_admitted_signed(
        p,
        &d.bindings,
        &d.frame,
        pre,
        params,
        &request.native(),
        None,
        s.version.profile(),
        &codec::object_hash(d)?,
        l.max_leaf_bytes,
        l.max_artifact_bytes,
        Some(&s),
        None,
    )?;
    check_supplied_successor(&s, predecessor, &bundle, l)?;
    let package = build_package(&s, d, predecessor, parameters, request, &artifact, &bundle)?;
    call_artifacts::check_json_input(
        &package.payload,
        l.max_input_bytes.min(SIGNED_JSON_BYTES as u64),
    )?;
    call_artifacts::json_bytes(&package, l.max_leaf_bytes)?;
    Ok(PreparedCallSigningStatement {
        package,
        static_bindings: s,
        descriptor: d.clone(),
        predecessor: predecessor.into(),
        parameters: parameters.into(),
        request: request.clone(),
        artifact,
        public: public.clone(),
        limits: l.clone(),
    })
}
fn check_supplied_successor(
    s: &SignedStaticBindings,
    predecessor: &[u8],
    bundle: &PreparedBundle,
    l: &NativeWorkLimits,
) -> Result<()> {
    if s.version == SignedVersion::ConsumingV2 {
        let ledger: layout::NativeState =
            call_artifacts::decode_bounded(predecessor, l.max_input_bytes)?;
        let successor = call_artifacts::decode_bounded(
            &codec::unhex(&bundle.readback.charged_successor_hex)?,
            l.max_leaf_bytes,
        )?;
        nonce::check_successor_input(s, &ledger, &successor, l.max_input_bytes)?;
    }
    Ok(())
}
pub(crate) fn build_package(
    s: &SignedStaticBindings,
    d: &SignedInstalledDeployment,
    predecessor: &[u8],
    parameters: &[u8],
    q: &SignedCallRequest,
    a: &Artifact,
    b: &PreparedBundle,
) -> Result<CallSigningPackage> {
    let m = a
        .manifest
        .signed
        .as_ref()
        .ok_or(refuse("signed-relation"))?;
    let projection = super::context::projection(
        s,
        &b.readback.execution,
        q,
        &q.action,
        b.readback.authority.as_ref(),
    )?;
    if projection
        .iter()
        .map(|v| WireDescriptor {
            position: v.position,
            descriptor: v.descriptor.clone(),
            ty: v.ty,
        })
        .collect::<Vec<_>>()
        != m.semantic_wire_descriptors
    {
        return Err(refuse("semantic-wire-layout"));
    }
    let offer = b
        .native
        .intent
        .guaranteed_unshielded_offer
        .as_ref()
        .ok_or(refuse("offer"))?;
    let inputs=offer.inputs.iter().map(|o|Ok(serde_json::json!({"owner":codec::native_hex(&o.owner)?,"amount":UInt(o.value),"asset":codec::native_hex(&o.type_)?}))).collect::<Result<Vec<_>>>()?;
    let outputs=offer.outputs.iter().map(|o|Ok(serde_json::json!({"owner":codec::native_hex(&o.owner)?,"amount":UInt(o.value),"asset":codec::native_hex(&o.type_)?}))).collect::<Result<Vec<_>>>()?;
    let readback = serde_json::json!([PROJECTION,b.readback.execution.pre,b.readback.execution.post,b.readback.execution.effects,b.native.lowered.program.iter().map(|o|codec::native_hex(o)).collect::<Result<Vec<_>>>()?,b.readback.charged_successor_hex,b.readback.effects_hex,b.readback.native_events_hex,b.readback.measured_gas_hex,d.frame.gas_hex,b.readback.preserved_frame,b.readback.prospective_balances,{"spends":inputs,"outputs":outputs}]);
    let bindings = &s.bindings;
    let payload = OwnerPayload {
        profile: PAYLOAD.into(),
        installed_profile: s.version.profile().into(),
        qualification: QUALIFICATION.into(),
        relation: s.version.relation().into(),
        ir_sha256: a.manifest.ir_hash.clone(),
        storage_abi: s.version.storage_abi().into(),
        signed_abi: s.version.signed_abi().into(),
        context_sha256: m.context_sha256.clone(),
        source_sha256: bindings.source_hash.clone(),
        program_sha256: bindings.program_hash.clone(),
        semantics: mori_core::SEMANTICS.into(),
        compiler_sha256: crate::proving::source_closure(),
        toolchain: bindings.toolchain.clone(),
        action: q.action.clone(),
        action_ordinal: s
            .program
            .actions
            .iter()
            .position(|a| a.name == q.action)
            .ok_or(refuse("action"))?,
        signer_policy_sha256: codec::object_hash(&s.policy)?,
        owner: s.policy.owner.clone(),
        beneficiary: s.policy.beneficiary.clone(),
        public_key_hex: s.policy.public_key_hex.clone(),
        network: bindings.network.clone(),
        anchor: bindings.anchor.clone(),
        address_hex: bindings.address_hex.clone(),
        duties: s.duties.clone(),
        assets: super::context::canonical_assets(s),
        predecessor_sha256: codec::hash(predecessor),
        native_state_hash_hex: d.frame.native_state_hash.clone(),
        parameters_sha256: codec::hash(parameters),
        request_sha256: codec::object_hash(q)?,
        selected_spends: q.selected_spends.clone(),
        ttl_hex: q.ttl_hex.clone(),
        nonce: q.nonce,
        not_before: q.not_before,
        not_after: q.not_after,
        limits: q.limits.clone(),
        wire_projection_sha256: codec::object_hash(&projection)?,
        wire_projection: projection,
        signing_readback_sha256: codec::object_hash(&readback)?,
    };
    Ok(CallSigningPackage {
        profile: PACKAGE.into(),
        context: m.context.clone(),
        payload_sha256: codec::object_hash(&payload)?,
        message_hex: schnorr::native_hex(message(&payload)?),
        payload,
        signing_readback: readback,
    })
}
pub(crate) fn typed_input(
    s: &SignedStaticBindings,
    q: &SignedCallRequest,
    package: &CallSigningPackage,
    attachment: &SignedCallAttachment,
) -> Result<AlignedValue> {
    let key = schnorr::decode_key(&s.policy.public_key_hex)?;
    let sig = schnorr::decode_signature(&attachment.signature_hex)?;
    let a = s
        .program
        .actions
        .iter()
        .find(|a| a.name == q.action)
        .ok_or(refuse("action"))?;
    let mut values = a
        .inputs
        .iter()
        .map(|i| layout::value(&q.inputs[&i.name]))
        .collect::<Vec<_>>();
    let (px, py) = schnorr::coordinates(key);
    let (rx, ry) = schnorr::coordinates(sig.r);
    let payload = digest_limbs(&package.payload_sha256)?;
    let pred = digest_limbs(&package.payload.predecessor_sha256)?;
    let mut fields = vec![
        px,
        py,
        rx,
        ry,
        sig.response,
        Fr::from(payload[0]),
        Fr::from(payload[1]),
        Fr::from(q.nonce.0),
        Fr::from(q.not_before.0),
        Fr::from(q.not_after.0),
        Fr::from(pred[0]),
        Fr::from(pred[1]),
        Fr::from(q.at.0),
    ];
    for l in &q.limits {
        fields.extend([
            Fr::from(l.gross_cap.0),
            Fr::from(l.fee_cap.0),
            Fr::from(l.net_floor.0),
            Fr::from(l.liability_cap.0),
        ]);
    }
    values.extend(fields.into_iter().map(AlignedValue::from));
    let input = AlignedValue::concat(values.iter());
    let mut actual = vec![];
    input.value_only_field_repr(&mut actual);
    if actual.len() != a.inputs.len() + 13 + 4 * s.assets.len() {
        return Err(refuse("typed-input-width"));
    }
    Ok(input)
}
pub fn attach_signed_installed_call(
    statement: PreparedCallSigningStatement,
    attachment: &SignedCallAttachment,
) -> Result<PreparedSignedInstalledCall> {
    check_attachment_payload(
        &statement.static_bindings,
        &statement.package,
        attachment,
        &statement.limits,
    )?;
    attach_inner(statement, attachment)
}
pub(crate) fn check_attachment_payload(
    s: &SignedStaticBindings,
    package: &CallSigningPackage,
    attachment: &SignedCallAttachment,
    l: &NativeWorkLimits,
) -> Result<()> {
    if attachment.profile != ATTACHMENT || attachment.payload != package.payload {
        return Err(refuse("attachment-payload"));
    }
    call_artifacts::check_json_input(attachment, l.max_input_bytes.min(SIGNED_JSON_BYTES as u64))?;
    schnorr::verify_attachment(
        &s.policy.public_key_hex,
        &attachment.signature_hex,
        message(&package.payload)?,
    )?;
    Ok(())
}
fn attach_inner(
    statement: PreparedCallSigningStatement,
    attachment: &SignedCallAttachment,
) -> Result<PreparedSignedInstalledCall> {
    let s = &statement.static_bindings;
    let d = &statement.descriptor;
    let q = &statement.request;
    let l = &statement.limits;
    let input = typed_input(s, q, &statement.package, attachment)?;
    let (pre, params) = decode_predecessor(
        s,
        d,
        &statement.predecessor,
        &statement.parameters,
        q,
        &statement.public.inner,
        l,
    )?;
    let operation = (*pre
        .contract
        .operations
        .get(&q.action.as_bytes().into())
        .ok_or(refuse("operation"))?)
    .clone();
    let mut bundle = prepare::prepare_admitted_signed(
        &s.program,
        &d.bindings,
        &d.frame,
        pre,
        params,
        &q.native(),
        Some(&statement.artifact),
        s.version.profile(),
        &codec::object_hash(d)?,
        l.max_leaf_bytes,
        l.max_artifact_bytes,
        Some(s),
        Some(input),
    )?;
    check_supplied_successor(s, &statement.predecessor, &bundle, l)?;
    let rebuilt = build_package(
        s,
        d,
        &statement.predecessor,
        &statement.parameters,
        q,
        &statement.artifact,
        &bundle,
    )?;
    if rebuilt != statement.package {
        return Err(refuse("signing-readback-drift"));
    }
    mori_core::intent::check_effects(
        &s.policy.owner,
        &q.limits,
        &bundle.readback.execution.effects,
        &bundle.readback.execution.post.liabilities,
    )
    .map_err(|_| refuse("economic-caps"))?;
    bundle.readback.evidence.qualification = QUALIFICATION;
    bundle.readback.evidence.signature_verification = "strict-host-fens-fixed-compiled-policy";
    bundle.readback.evidence.relation = "source-static-signed-native-ordinary-ir-check";
    bundle.leaves.insert(
        "readback.json".into(),
        call_artifacts::json_bytes(&bundle.readback, l.max_leaf_bytes)?,
    );
    bundle.leaves.insert(
        "signing-package.json".into(),
        call_artifacts::json_bytes(&statement.package, l.max_leaf_bytes)?,
    );
    bundle.leaves.insert(
        "signed-attachment.json".into(),
        call_artifacts::json_bytes(attachment, l.max_leaf_bytes.min(SIGNED_JSON_BYTES as u64))?,
    );
    // Recompute complete inspection marker after adding public authorization.
    let mut marker: serde_json::Value =
        serde_json::from_slice(&bundle.marker).map_err(|_| refuse("marker"))?;
    marker["evidence"] =
        serde_json::to_value(&bundle.readback.evidence).map_err(|_| refuse("marker"))?;
    marker["leaves"] = serde_json::to_value(
        bundle
            .leaves
            .iter()
            .map(|(n, b)| (n.clone(), call_artifacts::leaf(b)))
            .collect::<BTreeMap<_, _>>(),
    )
    .map_err(|_| refuse("marker"))?;
    bundle.marker = call_artifacts::json_bytes(&marker, l.max_leaf_bytes)?;
    let mut aggregate = bundle.marker.len();
    for bytes in bundle.leaves.values() {
        if bytes.len() > NativeWorkLimits::cap(l.max_leaf_bytes)? {
            return Err(refuse("leaf-size"));
        }
        aggregate = call_artifacts::add_total(
            aggregate,
            bytes.len(),
            NativeWorkLimits::cap(l.max_artifact_bytes)?,
        )?;
    }
    prepare::admit_preparation_work(&bundle, l)?;
    let context = call_artifacts::ProofContext {
        signed: Some(SignedProofContext {
            profile: PROOF_CONTEXT.into(),
            policy_sha256: codec::object_hash(&s.policy)?,
            payload_sha256: statement.package.payload_sha256.clone(),
            context_sha256: statement.package.payload.context_sha256.clone(),
            attachment_sha256: codec::object_hash(attachment)?,
        }),
        source_hash: s.bindings.source_hash.clone(),
        keyset_identity: d.keyset_identity.clone(),
        deployment_hash: codec::object_hash(d)?,
        predecessor_hash: codec::hash(&statement.predecessor),
        parameters_hash: codec::hash(&statement.parameters),
        request_hash: codec::object_hash(q)?,
        action: q.action.clone(),
    };
    Ok(PreparedSignedInstalledCall {
        inner: PreparedInstalledCall {
            prepared: PreparedNativeCall {
                bundle,
                artifact: statement.artifact,
            },
            operation,
            public: statement.public.inner,
            context,
        },
        package: statement.package,
    })
}
/// ```compile_fail
/// let _: mori_ledger::signed::ProvenSignedInstalledCall = serde_json::from_str("{}").unwrap();
/// ```
pub struct ProvenSignedInstalledCall {
    pub(crate) inner: crate::proving::ProvenInstalledCall,
}
impl ProvenSignedInstalledCall {
    pub fn intent(&self) -> &crate::proving::ProvenIntent {
        self.inner.intent()
    }
    pub fn readback(&self) -> &prepare::Readback {
        self.inner.readback()
    }
    pub fn verification(&self) -> &crate::proving::CallVerification {
        self.inner.verification()
    }
}
pub async fn prove_signed_installed_call(
    prepared: PreparedSignedInstalledCall,
    derived: SignedDerivedLocalDeployment,
    rng: rand::rngs::StdRng,
) -> Result<ProvenSignedInstalledCall> {
    Ok(ProvenSignedInstalledCall {
        inner: crate::proving::prove_installed_call_inner(prepared.inner, derived.inner, rng)
            .await?,
    })
}
pub fn verify_signed_installed_call(
    prepared: &PreparedSignedInstalledCall,
    candidate: &crate::proving::ProvenIntent,
) -> Result<crate::proving::CallVerification> {
    crate::proving::verify_installed_call_inner(&prepared.inner, candidate)
}
pub fn export_signed_proven_call(
    proven: &ProvenSignedInstalledCall,
    l: &NativeWorkLimits,
) -> Result<call_artifacts::ArtifactLeaves> {
    validate_work_limits(l)?;
    call_artifacts::export_proven_call(&proven.inner, l)
}
pub struct ImportedSignedProvenCall {
    inner: call_artifacts::ImportedProvenCall,
}
pub fn snapshot_signed_proven_call(
    path: &std::path::Path,
    l: &NativeWorkLimits,
    version: SignedVersion,
) -> Result<ImportedSignedProvenCall> {
    validate_work_limits(l)?;
    Ok(ImportedSignedProvenCall {
        inner: call_artifacts::snapshot_proven_profile(path, l, Some(version))?,
    })
}
impl ImportedSignedProvenCall {
    pub fn verify(
        &self,
        expected: &PreparedSignedInstalledCall,
        l: &NativeWorkLimits,
    ) -> Result<crate::proving::CallVerification> {
        self.inner.admit_context(&expected.inner.proof_context())?;
        self.inner.admit_inspection(&expected.inner)?;
        let result = verify_signed_installed_call(expected, self.inner.intent())?;
        self.inner.admit_readback(&expected.inner, &result, l)?;
        Ok(result)
    }
}
pub fn export_call_signing_package(
    statement: &PreparedCallSigningStatement,
) -> Result<call_artifacts::ArtifactLeaves> {
    let l = &statement.limits;
    let leaves: BTreeMap<String, Vec<u8>> = BTreeMap::from([
        (
            "signing-package.json".into(),
            call_artifacts::json_bytes(&statement.package, l.max_leaf_bytes)?,
        ),
        (
            "deployment.json".into(),
            call_artifacts::deployment_json_bytes(&statement.descriptor, l)?,
        ),
    ]);
    let marker = call_artifacts::json_bytes(
        &serde_json::json!({"profile":PACKAGE,"qualification":QUALIFICATION,"leaves":leaves.iter().map(|(n,b)|(n.clone(),call_artifacts::leaf(b))).collect::<BTreeMap<_,_>>()}),
        l.max_leaf_bytes,
    )?;
    let out = call_artifacts::ArtifactLeaves {
        leaves,
        marker_name: "signing-package-complete.json",
        marker,
    };
    out.check(l)?;
    Ok(out)
}
