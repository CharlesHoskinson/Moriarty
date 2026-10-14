//! Local fixed-policy signed calls. Snapshot authenticity, funding and replay
//! consumption remain independent open predicates.
pub(crate) mod constraints;
mod context;
mod deployment;
pub(crate) mod nonce;
mod preparation;
pub mod schnorr;
use crate::{Error, Result, codec, layout, profile::*};
pub use context::*;
pub use deployment::*;
use mori_core::{AssetId, Program, Statement, UInt, Value};
pub use preparation::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
pub const PROFILE: &str = "moriarty-ledger-installed-signed-local/1";
pub const QUALIFICATION: &str = "local-source-derived-installed-signed-unqualified";
pub const RELATION: &str = "moriarty-native-call-signed-relation/1";
pub const SIGNED_ABI: &str = "moriarty-ledger-signed-abi/1";
pub const CONSUMING_PROFILE: &str = "moriarty-ledger-installed-signed-local/2";
pub const CONSUMING_RELATION: &str = "moriarty-native-call-signed-relation/2";
pub const CONSUMING_SIGNED_ABI: &str = "moriarty-ledger-signed-abi/2";
pub const CONSUMING_STORAGE_ABI: &str = "moriarty-ledger-signed-public-abi/2";
pub const CONSUMING_SEED: &str = "moriarty-ledger-signed-call-seed/2";
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignedVersion {
    LegacyV1,
    ConsumingV2,
}
impl SignedVersion {
    pub fn profile(self) -> &'static str {
        match self {
            Self::LegacyV1 => PROFILE,
            Self::ConsumingV2 => CONSUMING_PROFILE,
        }
    }
    pub fn relation(self) -> &'static str {
        match self {
            Self::LegacyV1 => RELATION,
            Self::ConsumingV2 => CONSUMING_RELATION,
        }
    }
    pub fn storage_abi(self) -> &'static str {
        match self {
            Self::LegacyV1 => ABI,
            Self::ConsumingV2 => CONSUMING_STORAGE_ABI,
        }
    }
    pub fn signed_abi(self) -> &'static str {
        match self {
            Self::LegacyV1 => SIGNED_ABI,
            Self::ConsumingV2 => CONSUMING_SIGNED_ABI,
        }
    }
}
pub const POLICY: &str = "moriarty-ledger-fixed-jubjub-policy/1";
pub const REQUEST: &str = "moriarty-ledger-signed-call-request/1";
pub const PACKAGE: &str = "moriarty-ledger-call-signing-package/1";
pub const ATTACHMENT: &str = "moriarty-ledger-signed-call-attachment/1";
pub const PAYLOAD: &str = "moriarty-ledger-call-owner-payload/1";
pub const CONTEXT: &str = "moriarty-ledger-call-signing-context/1";
pub const PROJECTION: &str = "moriarty-ledger-call-wire-projection/1";
pub const KEYSET: &str = "moriarty-ledger-signed-call-keyset/1";
pub const PROOF_CONTEXT: &str = "moriarty-ledger-signed-call-proof-context/1";
pub const MESSAGE_DOMAIN: u128 = 0x31434c4d;
pub const SIGNED_JSON_BYTES: usize = 65536;
pub(crate) fn refuse(code: &'static str) -> Error {
    Error::new("signed-call", code)
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixedSignerPolicy {
    pub profile: String,
    pub owner: String,
    pub beneficiary: String,
    pub public_key_hex: String,
}
impl FixedSignerPolicy {
    pub fn from_json(text: &str) -> Result<Self> {
        let v: Self = codec::json(text)?;
        v.validate()?;
        Ok(v)
    }
    pub fn validate(&self) -> Result<()> {
        if self.profile != POLICY {
            return Err(refuse("policy-profile"));
        }
        label(&self.owner)?;
        label(&self.beneficiary)?;
        schnorr::decode_key(&self.public_key_hex)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedCallRequest {
    pub profile: String,
    pub action: String,
    #[serde(deserialize_with = "strict_field")]
    pub inputs: BTreeMap<String, Value>,
    pub selected_spends: Vec<String>,
    pub segment: u16,
    pub ttl_hex: String,
    pub nonce: UInt,
    pub block_context_hex: String,
    pub communication_randomness_hex: String,
    pub binding_randomness_hex: String,
    pub not_before: UInt,
    pub not_after: UInt,
    pub at: UInt,
    pub limits: Vec<mori_core::intent::AssetLimit>,
}
impl SignedCallRequest {
    pub fn from_json(text: &str) -> Result<Self> {
        if text.len() > SIGNED_JSON_BYTES {
            return Err(refuse("request-size"));
        }
        let v: Self = codec::json(text)?;
        if v.profile != REQUEST {
            return Err(refuse("request-profile"));
        }
        Ok(v)
    }
    pub(crate) fn native(&self) -> CallRequest {
        CallRequest {
            action: self.action.clone(),
            inputs: self.inputs.clone(),
            selected_spends: self.selected_spends.clone(),
            segment: self.segment,
            ttl_hex: self.ttl_hex.clone(),
            nonce: self.nonce,
            block_context_hex: self.block_context_hex.clone(),
            communication_randomness_hex: self.communication_randomness_hex.clone(),
            binding_randomness_hex: self.binding_randomness_hex.clone(),
            economic_bounds: None,
        }
    }
    pub(crate) fn admit(&self, s: &SignedStaticBindings, action: &str) -> Result<()> {
        if self.profile != REQUEST
            || self.action != action
            || self.not_before.0 > self.at.0
            || self.at.0 > self.not_after.0
            || self.segment != 1
        {
            return Err(refuse("request-scope"));
        }
        let a = s
            .program
            .actions
            .iter()
            .find(|a| a.name == action)
            .ok_or(refuse("action"))?;
        if self.inputs.len() != a.inputs.len()
            || a.inputs
                .iter()
                .any(|i| self.inputs.get(&i.name).is_none_or(|v| v.ty() != i.ty))
        {
            return Err(refuse("source-inputs"));
        }
        if self.limits.len() != s.assets.len()
            || self
                .limits
                .iter()
                .zip(&s.assets)
                .any(|(l, a)| l.asset != *a || l.beneficiary != s.policy.beneficiary)
        {
            return Err(refuse("complete-ordered-limits"));
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedCallAttachment {
    pub profile: String,
    pub payload: OwnerPayload,
    pub signature_hex: String,
}
impl SignedCallAttachment {
    pub fn from_json(text: &str) -> Result<Self> {
        if text.len() > SIGNED_JSON_BYTES {
            return Err(refuse("attachment-size"));
        }
        let a: Self = codec::json(text)?;
        if a.profile != ATTACHMENT {
            return Err(refuse("attachment-profile"));
        }
        schnorr::decode_signature(&a.signature_hex)?;
        Ok(a)
    }
}
#[derive(Clone)]
pub struct SignedStaticBindings {
    pub(crate) version: SignedVersion,
    pub(crate) source: String,
    pub(crate) program: Program,
    pub(crate) bindings: StaticBindings,
    pub(crate) policy: FixedSignerPolicy,
    pub(crate) assets: Vec<AssetId>,
    pub(crate) duties: Vec<DutyBinding>,
}
impl SignedStaticBindings {
    pub fn from_source(
        source: &str,
        program: &Program,
        bindings: &StaticBindings,
        policy: &FixedSignerPolicy,
        version: SignedVersion,
    ) -> Result<Self> {
        policy.validate()?;
        if bindings.economic_bounds.is_some() {
            return Err(refuse("economic-bounds-forbidden"));
        }
        if program.assets.len() > 16 || bindings.duties.len() > 16 {
            return Err(refuse("scope-limit"));
        }
        if bindings.payer != policy.owner {
            return Err(refuse("sole-payer"));
        }
        layout::validate_static(source, program, bindings)?;
        label(&program.name)?;
        for s in &program.states {
            label(&s.name)?;
        }
        let mut assets = Vec::new();
        for a in &program.assets {
            label(&a.name)?;
            for p in [
                &a.identity.network,
                &a.identity.issuer,
                &a.identity.reference,
            ] {
                label(p)?;
            }
            assets.push(a.identity.clone());
        }
        assets.sort();
        if assets.windows(2).any(|a| a[0] == a[1]) {
            return Err(refuse("asset-alias"));
        }
        if !bindings.roles.iter().any(|r| r.role == policy.owner)
            || !bindings.roles.iter().any(|r| r.role == policy.beneficiary)
        {
            return Err(refuse("policy-role"));
        }
        for r in &bindings.roles {
            label(&r.role)?;
        }
        let mut duties = bindings.duties.clone();
        duties.sort_by(|a, b| a.name.cmp(&b.name));
        let mut keys = BTreeSet::new();
        for d in &duties {
            for p in [&d.name, &d.debtor, &d.creditor] {
                label(p)?;
            }
            if d.debtor != policy.owner
                || !assets.contains(&d.asset)
                || !keys.insert(codec::native_hex(&layout::name_key("duty", &d.name))?)
            {
                return Err(refuse("duty-registry"));
            }
        }
        for a in &program.actions {
            label(&a.name)?;
            for i in &a.inputs {
                label(&i.name)?;
            }
            for st in &a.statements {
                match st {
                    Statement::Transfer {
                        from, to, asset, ..
                    }
                    | Statement::Fee {
                        from, to, asset, ..
                    } => {
                        label(from)?;
                        label(to)?;
                        label(asset)?;
                        if *from != policy.owner {
                            return Err(refuse("sole-payer"));
                        }
                    }
                    Statement::Liability {
                        name,
                        debtor,
                        creditor,
                        ..
                    } => {
                        label(name)?;
                        label(debtor)?;
                        label(creditor)?;
                        if *debtor != policy.owner {
                            return Err(refuse("sole-debtor"));
                        }
                        if !duties.iter().any(|d| d.name == *name) {
                            return Err(refuse("missing-duty-binding"));
                        }
                    }
                    Statement::Discharge { name, .. } => {
                        label(name)?;
                        if !duties.iter().any(|d| d.name == *name) {
                            return Err(refuse("missing-duty-binding"));
                        }
                    }
                    Statement::Let { name, .. } | Statement::Emit { name, .. } => label(name)?,
                    _ => {}
                }
            }
        }
        Ok(Self {
            version,
            source: source.into(),
            program: program.clone(),
            bindings: bindings.clone(),
            policy: policy.clone(),
            assets,
            duties,
        })
    }
    pub fn admit_request(&self, request: &SignedCallRequest, action: &str) -> Result<()> {
        request.admit(self, action)
    }
    pub fn policy(&self) -> &FixedSignerPolicy {
        &self.policy
    }
}
fn label(s: &str) -> Result<()> {
    if s.is_empty()
        || s.len() > 1024
        || s.chars()
            .any(|c| c <= '\u{1f}' || ('\u{7f}'..='\u{9f}').contains(&c))
    {
        Err(refuse("label"))
    } else {
        Ok(())
    }
}
#[cfg(test)]
mod tests;

pub(crate) fn strict_field<'de, D: serde::Deserializer<'de>, T: serde::de::DeserializeOwned>(
    decoder: D,
) -> std::result::Result<T, D::Error> {
    use serde::de::Error as _;
    let value = mori_core::json::deserialize_strict(decoder)?;
    serde_json::from_value(value).map_err(D::Error::custom)
}

/// Caller budgets may reduce the existing ordinary envelopes, never widen them.
pub fn validate_work_limits(limits: &NativeWorkLimits) -> Result<()> {
    limits.validate()?;
    if limits.max_input_bytes > codec::INPUT_BYTES as u64
        || limits.max_leaf_bytes > codec::OUTPUT_BYTES as u64
        || limits.max_artifact_bytes > codec::OUTPUT_BYTES as u64
    {
        return Err(refuse("ordinary-envelope"));
    }
    Ok(())
}
