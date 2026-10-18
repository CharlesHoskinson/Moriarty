/// Engineering admission ceiling, not a measured proving-resource bound.
pub const MAX_LIABILITY_REGISTRY_ENTRIES: usize = 16;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LiabilityIdentity {
    pub name: String,
    pub asset: AssetId,
    pub debtor: String,
    pub creditor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiabilityDescriptor {
    pub name: String,
    pub asset: AssetId,
    pub debtor: String,
    pub creditor: String,
    pub pre_presence_position: u32,
    pub pre_amount_position: u32,
    pub post_presence_position: u32,
    pub post_amount_position: u32,
}

/// The source statement determines the operation; core effects are amount-only.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LiabilityOperation {
    CreateOrAccrue,
    Discharge,
}
