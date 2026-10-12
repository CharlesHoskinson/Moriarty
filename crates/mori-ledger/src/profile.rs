use crate::{Result, codec};
use mori_core::{AssetId, UInt, Value};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
pub const PROFILE: &str = "moriarty-ledger-call-preparation/1";
pub const ABI: &str = "moriarty-ledger-public-abi/1";
pub const SDK: &str = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14";
pub const COMPAT: &str = "a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Toolchain {
    pub sdk: String,
    pub compatibility: String,
    pub rust: String,
    pub packages: BTreeMap<String, String>,
}
impl Toolchain {
    pub fn pinned() -> Self {
        Self {
            sdk: SDK.into(),
            compatibility: COMPAT.into(),
            rust: "1.99.0".into(),
            packages: [
                ("ledger-v9", "1.0.0"),
                ("onchain-runtime", "4.0.0"),
                ("onchain-state", "4.0.0"),
                ("onchain-vm", "4.0.0"),
                ("base-crypto", "1.1.0"),
                ("transient-crypto", "3.0.0"),
                ("transient-crypto-compat", "2.2.0"),
                ("coin-structure", "3.0.0"),
                ("storage", "2.0.1"),
                ("serialize", "1.2.0"),
                ("zkir-v3", "3.0.0-rc.2"),
                ("curves", "0.3.1"),
            ]
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    pub input_bytes: usize,
    pub output_bytes: usize,
    pub ops: usize,
    pub trace_cells: usize,
    pub declarations: usize,
    pub expressions: usize,
    pub statements: usize,
}
impl Limits {
    pub fn supported() -> Self {
        Self {
            input_bytes: codec::INPUT_BYTES,
            output_bytes: codec::OUTPUT_BYTES,
            ops: 100_000,
            trace_cells: mori_core::MAX_TRACE_CELLS,
            declarations: mori_core::MAX_DECLARATIONS,
            expressions: mori_core::MAX_EXPRESSIONS,
            statements: mori_core::MAX_STATEMENTS,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssetBinding {
    pub identity: AssetId,
    pub token_hex: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoleBinding {
    pub role: String,
    pub public_key_hex: String,
    pub address_hex: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DutyBinding {
    pub name: String,
    pub asset: AssetId,
    pub debtor: String,
    pub creditor: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bounds {
    pub owner: String,
    pub limits: Vec<mori_core::intent::AssetLimit>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Deployment {
    pub profile: String,
    pub abi: String,
    pub qualification: String,
    pub phase_policy: String,
    pub source_hash: String,
    pub program_hash: String,
    pub abi_hash: String,
    pub templates_hash: String,
    pub toolchain: Toolchain,
    pub limits: Limits,
    pub network: String,
    pub anchor: String,
    pub address_hex: String,
    pub payer: String,
    pub predecessor_hash: String,
    pub native_state_hash: String,
    pub parameters_hash: String,
    pub gas_hex: String,
    pub operations_hash: String,
    pub maintenance_hash: String,
    pub balance_hash: String,
    pub assets: Vec<AssetBinding>,
    pub roles: Vec<RoleBinding>,
    pub duties: Vec<DutyBinding>,
    pub economic_bounds: Option<Bounds>,
}
impl Deployment {
    pub fn from_json(s: &str) -> Result<Self> {
        codec::json(s)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CallRequest {
    pub action: String,
    pub inputs: BTreeMap<String, Value>,
    pub selected_spends: Vec<String>,
    pub segment: u16,
    pub ttl_hex: String,
    pub nonce: UInt,
    pub block_context_hex: String,
    pub communication_randomness_hex: String,
    pub binding_randomness_hex: String,
    pub economic_bounds: Option<Bounds>,
}
impl CallRequest {
    pub fn from_json(s: &str) -> Result<Self> {
        codec::json(s)
    }
}

pub const INSTALLED_PROFILE: &str = "moriarty-ledger-installed-local/1";
pub const INSTALLED_QUALIFICATION: &str = "local-source-derived-installed-unqualified";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StaticBindings {
    pub abi: String,
    pub source_hash: String,
    pub program_hash: String,
    pub abi_hash: String,
    pub templates_hash: String,
    pub toolchain: Toolchain,
    pub limits: Limits,
    pub network: String,
    pub anchor: String,
    pub address_hex: String,
    pub payer: String,
    pub assets: Vec<AssetBinding>,
    pub roles: Vec<RoleBinding>,
    pub duties: Vec<DutyBinding>,
    pub economic_bounds: Option<Bounds>,
}
impl StaticBindings {
    pub(crate) fn legacy(d: &Deployment) -> Self {
        Self {
            abi: d.abi.clone(),
            source_hash: d.source_hash.clone(),
            program_hash: d.program_hash.clone(),
            abi_hash: d.abi_hash.clone(),
            templates_hash: d.templates_hash.clone(),
            toolchain: d.toolchain.clone(),
            limits: d.limits.clone(),
            network: d.network.clone(),
            anchor: d.anchor.clone(),
            address_hex: d.address_hex.clone(),
            payer: d.payer.clone(),
            assets: d.assets.clone(),
            roles: d.roles.clone(),
            duties: d.duties.clone(),
            economic_bounds: d.economic_bounds.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrameBindings {
    pub predecessor_hash: String,
    pub native_state_hash: String,
    pub parameters_hash: String,
    pub gas_hex: String,
    pub operations_hash: String,
    pub maintenance_hash: String,
    pub balance_hash: String,
}
impl FrameBindings {
    pub(crate) fn legacy(d: &Deployment) -> Self {
        Self {
            predecessor_hash: d.predecessor_hash.clone(),
            native_state_hash: d.native_state_hash.clone(),
            parameters_hash: d.parameters_hash.clone(),
            gas_hex: d.gas_hex.clone(),
            operations_hash: d.operations_hash.clone(),
            maintenance_hash: d.maintenance_hash.clone(),
            balance_hash: d.balance_hash.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstalledDeployment {
    pub profile: String,
    pub qualification: String,
    pub phase_policy: String,
    pub bindings: StaticBindings,
    pub frame: FrameBindings,
    pub keyset_identity: String,
}
impl InstalledDeployment {
    pub fn from_json(s: &str) -> Result<Self> {
        codec::json(s)
    }
    pub(crate) fn validate_profile(&self) -> Result<()> {
        if self.profile != INSTALLED_PROFILE
            || self.qualification != INSTALLED_QUALIFICATION
            || self.phase_policy != "guaranteed-only"
            || self.bindings.anchor != "local-unqualified"
            || self.bindings.abi != ABI
            || self.bindings.network.is_empty()
        {
            return Err(crate::Error::new("identity", "installed-local-profile"));
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeWorkLimits {
    pub profile: String,
    pub max_degree: u8,
    pub max_actions: u64,
    pub max_srs_bytes: u64,
    pub max_prover_key_bytes: u64,
    pub max_verifier_key_bytes: u64,
    pub max_input_bytes: u64,
    pub max_leaf_bytes: u64,
    pub max_artifact_bytes: u64,
    pub max_operations: u64,
    pub max_public_inputs: u64,
}
impl NativeWorkLimits {
    pub fn from_json(s: &str) -> Result<Self> {
        let value: Self = codec::json(s)?;
        value.validate()?;
        Ok(value)
    }
    pub fn validate(&self) -> Result<()> {
        let version = self.version()?;
        if self.max_degree == 0 {
            return Err(crate::Error::new("input", "native-work-limits"));
        }
        for n in [
            self.max_actions,
            self.max_srs_bytes,
            self.max_prover_key_bytes,
            self.max_verifier_key_bytes,
            self.max_input_bytes,
            self.max_leaf_bytes,
            self.max_artifact_bytes,
            self.max_operations,
            self.max_public_inputs,
        ] {
            Self::cap(n)?;
        }
        if self.max_srs_bytes > 128 * 1024 * 1024 {
            return Err(crate::Error::new("input", "native-work-limits"));
        }
        if version == WorkLimitsVersion::V2
            && (self.max_prover_key_bytes > 64 * 1024 * 1024
                || self.max_leaf_bytes > codec::OUTPUT_BYTES as u64
                || self.max_artifact_bytes > codec::OUTPUT_BYTES as u64
                || self.max_input_bytes > codec::INPUT_BYTES as u64
                || self.max_verifier_key_bytes > 65_536)
        {
            return Err(crate::Error::new("input", "native-work-limits"));
        }
        Ok(())
    }
    pub(crate) fn version(&self) -> Result<WorkLimitsVersion> {
        match self.profile.as_str() {
            "moriarty-native-call-work-limits/1" => Ok(WorkLimitsVersion::V1),
            "moriarty-native-call-work-limits/2" => Ok(WorkLimitsVersion::V2),
            _ => Err(crate::Error::new("input", "native-work-limits")),
        }
    }
    /// Only v2 keyset transport separates PK from the public leaf cap.
    pub(crate) fn prover_key_cap(&self) -> Result<usize> {
        Self::cap(match self.version()? {
            WorkLimitsVersion::V1 => self.max_prover_key_bytes.min(self.max_leaf_bytes),
            WorkLimitsVersion::V2 => self.max_prover_key_bytes,
        })
    }
    pub(crate) fn cap(value: u64) -> Result<usize> {
        usize::try_from(value)
            .ok()
            .filter(|n| *n > 0 && n.checked_add(1).is_some())
            .ok_or(crate::Error::new("input", "native-work-limits"))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WorkLimitsVersion {
    V1,
    V2,
}
