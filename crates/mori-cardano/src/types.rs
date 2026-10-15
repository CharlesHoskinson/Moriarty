use crate::CoreSourceExportV1;
use mori_core::{Action, AssetDecl, AssetId, StateDecl};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AccountClass {
    ScriptCustodied,
    KeyHeldExternal,
}

/// Symbolic account classification only; not a physical credential binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AccountBinding {
    pub name: String,
    pub class: AccountClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum InspectionTarget {
    Unit,
    /// Reserved until the complete pinned-target identity interface is frozen.
    PinnedCardano,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DutyMetadata {
    pub name: String,
    pub asset: AssetId,
    pub debtor: String,
    pub creditor: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FramedCellMetadata {
    pub account: String,
    pub asset: AssetId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActionBoundsMetadata {
    pub action: String,
    pub statements: usize,
    pub liability_statements: usize,
    pub trace_cells_upper_bound: usize,
}

/// Partial layout descriptors, not an encoded datum or applied script layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LayoutMetadataV1 {
    pub format: String,
    pub states: Vec<StateDecl>,
    pub assets: Vec<AssetDecl>,
    pub accounts: Vec<AccountBinding>,
    pub duties: Vec<DutyMetadata>,
    pub actions: Vec<Action>,
    pub framed_cells: Vec<FramedCellMetadata>,
    pub projected_balance_cells: usize,
    pub action_bounds: Vec<ActionBoundsMetadata>,
}

/// Inspectable UNIT metadata. Serialized fields are never admission authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CardanoInspectionMetadataV1 {
    pub format: String,
    pub qualification: String,
    pub profile: String,
    pub target: InspectionTarget,
    pub source: CoreSourceExportV1,
    pub layout: LayoutMetadataV1,
    pub layout_hash: Option<String>,
    pub unresolved: Vec<String>,
}

impl CardanoInspectionMetadataV1 {
    /// The layout preimage is not yet frozen; no value is invented from metadata.
    pub fn require_layout_hash(&self) -> Result<&str, InspectionError> {
        Err(InspectionError::Unresolved("layout_hash_preimage"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InspectionError {
    Core(mori_core::Error),
    UnsupportedDomain(String),
    BindingMismatch(String),
    Unresolved(&'static str),
}

impl std::fmt::Display for InspectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Core(error) => write!(f, "{error}"),
            Self::UnsupportedDomain(reason) => write!(f, "Unsupported metadata domain: {reason}"),
            Self::BindingMismatch(reason) => write!(f, "Symbolic binding mismatch: {reason}"),
            Self::Unresolved(field) => write!(f, "Unresolved interface: {field}"),
        }
    }
}
impl std::error::Error for InspectionError {}
