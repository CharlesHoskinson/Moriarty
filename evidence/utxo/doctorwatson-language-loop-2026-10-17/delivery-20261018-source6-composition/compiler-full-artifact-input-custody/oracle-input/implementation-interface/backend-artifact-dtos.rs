/// Positional ABI: every entry is read from public_transcript_outputs and
/// explicitly published by Impact in the same position in transcript_inputs.
/// Upstream PublicInput alone assigns a witness; it does not bind a public value.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicBinding {
    pub role: String,
    pub name: String,
    pub ty: Type,
    /// Explicit null when absent; omission refuses rather than defaulting.
    #[serde(deserialize_with = "Option::deserialize")]
    pub constant: Option<String>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub asset: Option<AssetId>,
}
/// Typed private witness descriptor; points are assigned as curve values.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivateType {
    #[serde(rename = "Point<Jubjub>")]
    JubjubPoint,
    Native,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PrivateBinding {
    pub name: String,
    pub ty: PrivateType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CircuitArtifact {
    pub action: String,
    pub zkir: String,
    /// SHA256 of the canonical 18-element source-fixed context preimage.
    pub context_hash: String,
    /// Financial positions 0..N, including context constants 0/1.
    pub public_inputs: Vec<PublicBinding>,
    /// Signed positions N..N+11+4A; empty for unsigned artifacts.
    pub authorization_bindings: Vec<AuthorizationBinding>,
    /// Typed private inputs, flattened `[PK.x,PK.y,R.x,R.y,s]`; empty if unsigned.
    pub private_inputs: Vec<PrivateBinding>,
    pub instruction_count: u32,
    pub effect_frame: Vec<EffectDescriptor>,
    pub balance_frame: Vec<BalanceDescriptor>,
    pub liability_frame: Vec<LiabilityDescriptor>,
    pub replay_steps: u32,
}
/// Static effect identities are committed through program/action context.
/// Every effect amount/value is separately public and constrained.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectDescriptor {
    pub step: u32,
    pub kind: String,
    // Static frame members are explicit null when absent; omission refuses.
    #[serde(deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub asset: Option<AssetId>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub from: Option<String>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub to: Option<String>,
    /// Liability effects use [before_presence, before_amount, after_presence, after_amount].
    pub public_positions: Vec<u32>,
    /// Full source-fixed duty identity; balance transfer fields remain separate.
    #[serde(deserialize_with = "Option::deserialize")]
    pub liability: Option<LiabilityIdentity>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub operation: Option<LiabilityOperation>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BalanceDescriptor {
    pub asset: AssetId,
    pub account: String,
    pub pre_position: u32,
    pub post_position: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Resources {
    pub instructions: u32,
    pub zkir_bytes: u32,
    pub artifact_bytes: u32,
    pub public_values: u32,
    pub max_replay_steps: u32,
    #[serde(deserialize_with = "Option::deserialize")]
    pub circuit_k: Option<u8>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub fraud_max_k: Option<u8>,
    pub cost_measured: bool,
    #[serde(deserialize_with = "decode_registry_entries")]
    pub liability_registry_entries: u32,
    #[serde(deserialize_with = "decode_registry_ceiling")]
    pub max_liability_registry_entries: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gate {
    pub predicate: String,
    pub status: String,
}
/// Decoding validates the whole artifact: versions, Target, the explicit mode
/// against the Target, and every circuit's exact descriptor shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "ArtifactFields")]
pub struct Artifact {
    pub format: String,
    pub abi: String,
    pub relation: String,
    pub authorization_mode: String,
    pub program_hash: String,
    pub semantics: String,
    pub target: Target,
    pub circuits: Vec<CircuitArtifact>,
    pub resources: Resources,
    pub gates: Vec<Gate>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ArtifactFields {
    #[serde(deserialize_with = "decode_artifact_format")]
    format: String,
    #[serde(deserialize_with = "decode_abi_version")]
    abi: String,
    #[serde(deserialize_with = "decode_relation_version")]
    relation: String,
    #[serde(deserialize_with = "decode_authorization_mode")]
    authorization_mode: String,
    program_hash: String,
    semantics: String,
    #[serde(deserialize_with = "decode_artifact_target")]
    target: Target,
    circuits: Vec<CircuitArtifact>,
    resources: Resources,
    gates: Vec<Gate>,
