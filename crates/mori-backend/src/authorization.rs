//! Strict signed-native envelope types and host commitments.
//! JSON/SHA checks here are host reconstruction, not native constraints.
use crate::{Error, PublicBinding, error, schnorr};
pub use mori_core::intent::AssetLimit;
use mori_core::{AssetId, Execution, UInt, Value};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};

pub const INTENT_PROFILE: &str = "moriarty-jubjub-owner-intent/1";
pub const PREPARED_PROFILE: &str = "moriarty-jubjub-prepared-intent/1";
pub const MAX_SIGNED_ASSETS: usize = 16;
pub const MAX_ENVELOPE_BYTES: usize = 65_536;
const MAX_REPORT_BYTES: usize = 1_048_576;

// Keep one field list for serialization and strict validated deserialization.
// Cross-field checks also apply to nested/direct serde decoding, while public
// JSON entry points first apply Core's bounded duplicate-decoded-key parser.
macro_rules! validated_struct {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        #[derive(Debug, Clone, Serialize)]
        pub struct $name { $(pub $field: $ty),* }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                #[derive(Deserialize)]
                #[serde(deny_unknown_fields)]
                struct Fields { $($field: $ty),* }
                let fields = Fields::deserialize(d)?;
                let value = Self { $($field: fields.$field),* };
                value.validate().map_err(serde::de::Error::custom)?;
                Ok(value)
            }
        }
    };
}
validated_struct!(SignedPolicy {
    owner: String,
    beneficiary: String,
    program_address: String
});
impl SignedPolicy {
    pub fn validate(&self) -> Result<(), Error> {
        for value in [&self.owner, &self.beneficiary, &self.program_address] {
            validate_label(value)?;
        }
        Ok(())
    }
}
validated_struct!(ExpectedAuthority {
    profile: String, network: String, anchor: String, program_address: String,
    target_hash: String, owner: String, beneficiary: String, public_key_hex: String,
    nonce: UInt, predecessor: String, not_before: UInt, not_after: UInt,
    limits: Vec<AssetLimit>,
});
validated_struct!(UnsignedPayload {
    profile: String, source_hash: String, program_hash: String, semantics: String,
    relation: String, relation_hash: String, abi: String, action: String,
    scenario_hash: String, target_hash: String, network: String, anchor: String,
    program_address: String, owner: String, beneficiary: String, public_key_hex: String,
    nonce: UInt, predecessor: String, not_before: UInt, not_after: UInt,
    limits: Vec<AssetLimit>,
});
validated_struct!(SignedIntent {
    payload: UnsignedPayload,
    signature_hex: String
});
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthorizationType {
    UInt,
    Native,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationBinding {
    pub position: u32,
    #[serde(deserialize_with = "authorization_role")]
    pub role: String,
    pub name: String,
    pub ty: AuthorizationType,
    #[serde(deserialize_with = "null_constant")]
    pub constant: Option<String>,
    #[serde(deserialize_with = "required_asset")]
    pub asset: Option<AssetId>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinancialValue {
    pub position: u32,
    #[serde(deserialize_with = "financial_binding")]
    pub binding: PublicBinding,
    pub value: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationValue {
    pub position: u32,
    pub binding: AuthorizationBinding,
    #[serde(deserialize_with = "native_string")]
    pub value: String,
}
validated_struct!(PreparedIntent {
    profile: String, payload: UnsignedPayload, payload_hash: String,
    context_hash: String, message_hex: String,
    financial_projection: Vec<FinancialValue>,
    authorization_projection: Vec<AuthorizationValue>, review: Execution,
});

fn read<T: DeserializeOwned>(text: &str) -> Result<T, Error> {
    let value = mori_core::json::parse(text).map_err(|e| error("authorization", e.to_string()))?;
    serde_json::from_value(value).map_err(|e| error("authorization", e.to_string()))
}
pub fn validate_label(label: &str) -> Result<(), Error> {
    if label.is_empty()
        || label.len() > 1024
        || label
            .chars()
            .any(|c| matches!(c as u32, 0..=31 | 127..=159))
    {
        return Err(error(
            "authorization",
            "Expected a 1..1024 UTF-8 byte label without C0/C1 controls.",
        ));
    }
    Ok(())
}
pub use crate::sherlock_machine::literal_host::decode_digest;
pub fn digest_limbs(digest: [u8; 32]) -> [UInt; 2] {
    [
        UInt(u128::from_le_bytes(
            digest[..16].try_into().expect("16 bytes"),
        )),
        UInt(u128::from_le_bytes(
            digest[16..].try_into().expect("16 bytes"),
        )),
    ]
}
fn canonical_bound(value: &impl Serialize, max: usize) -> Result<Vec<u8>, Error> {
    let bytes =
        mori_core::intent::canonical(value).map_err(|e| error("authorization", e.to_string()))?;
    if bytes.len() > max {
        return Err(error(
            "authorization",
            format!("Canonical JSON exceeds {max} bytes."),
        ));
    }
    Ok(bytes)
}
pub fn validate_asset(asset: &AssetId) -> Result<(), Error> {
    for label in [&asset.network, &asset.issuer, &asset.reference] {
        validate_label(label)?;
    }
    Ok(())
}
fn limits(limits: &[AssetLimit], beneficiary: &str) -> Result<(), Error> {
    if limits.len() > MAX_SIGNED_ASSETS {
        return Err(error("authorization", "More than 16 signed assets."));
    }
    for (i, limit) in limits.iter().enumerate() {
        validate_asset(&limit.asset)?;
        validate_label(&limit.beneficiary)?;
        if limit.beneficiary != beneficiary || (i > 0 && limits[i - 1].asset >= limit.asset) {
            return Err(error(
                "authorization",
                "Limits require matching beneficiary and strictly ascending full AssetIds.",
            ));
        }
    }
    Ok(())
}
impl ExpectedAuthority {
    pub fn from_json(text: &str) -> Result<Self, Error> {
        read(text)
    }
    pub fn validate(&self) -> Result<(), Error> {
        if self.profile != INTENT_PROFILE || self.not_before > self.not_after {
            return Err(error(
                "authorization",
                "Wrong profile or inverted validity interval.",
            ));
        }
        for label in [
            &self.network,
            &self.anchor,
            &self.program_address,
            &self.owner,
            &self.beneficiary,
            &self.predecessor,
        ] {
            validate_label(label)?;
        }
        decode_digest(&self.target_hash)?;
        schnorr::decode_key(&self.public_key_hex)?;
        limits(&self.limits, &self.beneficiary)?;
        Ok(())
    }
}
impl UnsignedPayload {
    pub fn from_json(text: &str) -> Result<Self, Error> {
        read(text)
    }
    pub fn validate(&self) -> Result<(), Error> {
        if self.profile != INTENT_PROFILE
            || self.semantics != "moriarty-core/1"
            || self.relation != "moriarty-zkir-relation/3"
            || self.abi != "moriarty-public-abi/3"
            || self.not_before > self.not_after
        {
            return Err(error(
                "authorization",
                "Wrong signed-native version or inverted validity interval.",
            ));
        }
        for label in [
            &self.action,
            &self.network,
            &self.anchor,
            &self.program_address,
            &self.owner,
            &self.beneficiary,
            &self.predecessor,
        ] {
            validate_label(label)?;
        }
        for digest in [
            &self.source_hash,
            &self.program_hash,
            &self.relation_hash,
            &self.scenario_hash,
            &self.target_hash,
        ] {
            decode_digest(digest)?;
        }
        schnorr::decode_key(&self.public_key_hex)?;
        limits(&self.limits, &self.beneficiary)?;
        canonical_bound(self, MAX_ENVELOPE_BYTES)?;
        Ok(())
    }
    pub fn hash(&self) -> Result<[u8; 32], Error> {
        self.validate()?;
        Ok(Sha256::digest(canonical_bound(self, MAX_ENVELOPE_BYTES)?).into())
    }
    pub fn predecessor_hash(&self) -> Result<[u8; 32], Error> {
        validate_label(&self.predecessor)?;
        Ok(Sha256::digest(canonical_bound(
            &["moriarty-predecessor-label/1", &self.predecessor],
            MAX_ENVELOPE_BYTES,
        )?)
        .into())
    }
    /// Compare intrinsic sorted limits with trusted source identities. Candidate
    /// limits are never sorted; source aliases and missing/extra identities refuse.
    pub fn validate_assets(&self, source_assets: &[AssetId]) -> Result<(), Error> {
        self.validate()?;
        if source_assets.len() > MAX_SIGNED_ASSETS {
            return Err(error("authorization", "More than 16 source assets."));
        }
        for asset in source_assets {
            validate_asset(asset)?;
        }
        let mut assets: Vec<_> = source_assets.iter().collect();
        assets.sort();
        if assets.windows(2).any(|pair| pair[0] == pair[1])
            || assets.len() != self.limits.len()
            || assets
                .iter()
                .zip(&self.limits)
                .any(|(asset, limit)| **asset != limit.asset)
        {
            return Err(error(
                "authorization",
                "Limits differ from the exact distinct source asset set.",
            ));
        }
        Ok(())
    }
}
impl SignedIntent {
    pub fn from_json(text: &str) -> Result<Self, Error> {
        read(text)
    }
    pub fn validate(&self) -> Result<(), Error> {
        self.payload.validate()?;
        schnorr::decode_signature(&self.signature_hex)?;
        canonical_bound(self, MAX_ENVELOPE_BYTES)?;
        Ok(())
    }
}
impl PreparedIntent {
    pub fn from_json(text: &str) -> Result<Self, Error> {
        read(text)
    }
    pub fn validate(&self) -> Result<(), Error> {
        self.payload.validate()?;
        if self.profile != PREPARED_PROFILE
            || decode_digest(&self.payload_hash)? != self.payload.hash()?
            || self.context_hash != self.payload.relation_hash
        {
            return Err(error(
                "authorization",
                "Prepared report profile or digest binding mismatch.",
            ));
        }
        decode_digest(&self.context_hash)?;
        schnorr::decode_native_hex(&self.message_hex)?;
        for v in &self.authorization_projection {
            schnorr::decode_native_hex(&v.value)?;
        }
        canonical_bound(self, MAX_REPORT_BYTES)?;
        Ok(())
    }
}
fn authorization_role<'de, D: serde::Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    let role = String::deserialize(d)?;
    if role != "authorization" {
        return Err(serde::de::Error::custom("Expected authorization role."));
    }
    Ok(role)
}
fn null_constant<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    let value = Option::<String>::deserialize(d)?;
    if value.is_some() {
        return Err(serde::de::Error::custom(
            "Authorization constant must be explicit null.",
        ));
    }
    Ok(value)
}
fn required_asset<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<AssetId>, D::Error> {
    let value = Option::<AssetId>::deserialize(d)?;
    if let Some(asset) = &value {
        validate_asset(asset).map_err(serde::de::Error::custom)?;
    }
    Ok(value)
}
fn native_string<'de, D: serde::Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    let value = String::deserialize(d)?;
    schnorr::decode_native_hex(&value).map_err(serde::de::Error::custom)?;
    Ok(value)
}
fn financial_binding<'de, D: serde::Deserializer<'de>>(d: D) -> Result<PublicBinding, D::Error> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Fields {
        role: String,
        name: String,
        ty: mori_core::Type,
        #[serde(deserialize_with = "Option::deserialize")]
        constant: Option<String>,
        #[serde(deserialize_with = "Option::deserialize")]
        asset: Option<AssetId>,
    }
    let f = Fields::deserialize(d)?;
    Ok(PublicBinding {
        role: f.role,
        name: f.name,
        ty: f.ty,
        constant: f.constant,
        asset: f.asset,
    })
}
