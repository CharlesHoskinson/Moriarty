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
