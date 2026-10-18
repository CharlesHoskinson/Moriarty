pub const PUBLIC_ABI_VERSION: &str = "moriarty-public-abi/3";
pub const RELATION_VERSION: &str = "moriarty-zkir-relation/3";
/// Explicit authorization discriminator of an unsigned v3 artifact.
pub const UNSIGNED_MODE: &str = "unsigned";

pub const LEDGER_REVISION: &str = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14";
pub const ZKIR_VERSION: &str = "3.0.0-rc.2";
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Target {
    pub profile: String,
    pub network: String,
    pub anchor: String,
    pub ledger_revision: String,
    pub zkir_version: String,
    pub max_instructions: u32,
    pub max_artifact_bytes: u32,
    /// Absent (omitted) for unsigned targets; explicit null refuses.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present_policy"
    )]
    pub signed_policy: Option<SignedPolicy>,
}
