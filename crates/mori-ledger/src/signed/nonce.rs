//! Supplied-state nonce consumption. This module does not authenticate history.
use super::*;
use midnight_base_crypto::{fab::AlignedValue, hash::HashOutput};
use midnight_onchain_state::state::StateValue;
use midnight_storage::db::InMemoryDB;

/// Includes the immutable domain sentinel. Whole snapshot budgets may bind first.
pub(crate) const AUTHORIZATION_MAP_CAPACITY: usize = 12_153;
pub(crate) fn domain_key() -> AlignedValue {
    HashOutput(layout::digest(
        "moriarty-signed-authority-domain-key/1",
        &[],
    ))
    .into()
}
pub(crate) fn nonce_domain(s: &SignedStaticBindings) -> Result<[u8; 32]> {
    let d = &s.bindings;
    let hex = codec::object_hash(&serde_json::json!([
        "moriarty-signed-authority-domain/1",
        s.version.storage_abi(),
        s.version.relation(),
        s.program.language,
        s.program.semantics,
        d.source_hash,
        d.program_hash,
        d.network,
        d.anchor,
        s.policy
    ]))?;
    codec::unhex(&hex)?
        .try_into()
        .map_err(|_| refuse("authority-domain"))
}
pub(crate) fn root_maps<'a>(
    s: &SignedStaticBindings,
    root: &'a StateValue<InMemoryDB>,
) -> Result<[&'a layout::NativeMap; 3]> {
    if s.version == SignedVersion::LegacyV1 {
        return layout::root_maps(root);
    }
    match root {
        StateValue::Array(a) if a.len() == 4 => Ok([
            layout::map(a.get(0).unwrap())?,
            layout::map(a.get(1).unwrap())?,
            layout::map(a.get(2).unwrap())?,
        ]),
        _ => Err(refuse("signed-storage-root")),
    }
}
pub(crate) fn authority_map<'a>(
    s: &SignedStaticBindings,
    root: &'a StateValue<InMemoryDB>,
) -> Result<&'a layout::NativeMap> {
    root_maps(s, root)?;
    let StateValue::Array(a) = root else {
        return Err(refuse("signed-storage-root"));
    };
    layout::map(a.get(3).ok_or(refuse("signed-storage-root"))?).map_err(|_| refuse("authority-map"))
}
pub(crate) fn validate_authority(
    s: &SignedStaticBindings,
    root: &StateValue<InMemoryDB>,
) -> Result<()> {
    if s.version == SignedVersion::LegacyV1 {
        return Ok(());
    }
    let map = authority_map(s, root)?;
    if map.size() == 0 || map.size() > AUTHORIZATION_MAP_CAPACITY {
        return Err(refuse("authority-capacity"));
    }
    let key = domain_key();
    let sentinel = map.get(&key).ok_or(refuse("authority-domain"))?;
    if layout::cell(&sentinel)? != AlignedValue::from(HashOutput(nonce_domain(s)?)) {
        return Err(refuse("authority-domain"));
    }
    for row in map.iter() {
        let (k, v) = &*row;
        if **k == key {
            continue;
        }
        let n = u128::try_from(&k.value[..]).map_err(|_| refuse("authority-map"))?;
        if AlignedValue::from(n) != **k || layout::cell(v)? != AlignedValue::from(true) {
            return Err(refuse("authority-map"));
        }
    }
    Ok(())
}
/// Constructed only after checking the entire predecessor authorization map.
#[derive(Clone)]
pub(crate) struct NonceObservation {
    nonce: UInt,
    pre_count: usize,
    predecessor: layout::NativeMap,
}
impl NonceObservation {
    pub(crate) fn nonce(&self) -> UInt {
        self.nonce
    }
    pub(crate) fn pre_count(&self) -> usize {
        self.pre_count
    }
    pub(crate) fn post_count(&self) -> usize {
        self.pre_count + 1
    }
    pub(crate) fn check_predecessor(
        &self,
        s: &SignedStaticBindings,
        root: &StateValue<InMemoryDB>,
    ) -> Result<()> {
        validate_authority(s, root)?;
        if *authority_map(s, root)? != self.predecessor {
            return Err(refuse("authority-observation"));
        }
        Ok(())
    }
    pub(crate) fn successor(&self) -> layout::NativeMap {
        self.predecessor
            .insert(self.nonce.0.into(), crate::ops::cell(true.into()))
    }
}
pub(crate) fn admit_authority(
    s: &SignedStaticBindings,
    root: &StateValue<InMemoryDB>,
    nonce: UInt,
) -> Result<NonceObservation> {
    validate_authority(s, root)?;
    let map = authority_map(s, root)?;
    if map.get(&nonce.0.into()).is_some() {
        return Err(refuse("nonce-used"));
    }
    if map.size() >= AUTHORIZATION_MAP_CAPACITY {
        return Err(refuse("authority-capacity"));
    }
    Ok(NonceObservation {
        nonce,
        pre_count: map.size(),
        predecessor: map.clone(),
    })
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NonceAuthorityReadback {
    pub pre_nonce_count: UInt,
    pub pre_nonce_present: bool,
    pub post_nonce_count: UInt,
    pub post_nonce_present: bool,
}
impl NonceAuthorityReadback {
    pub(crate) fn checked(
        s: &SignedStaticBindings,
        observation: &NonceObservation,
        successor: &StateValue<InMemoryDB>,
    ) -> Result<Self> {
        validate_authority(s, successor)?;
        if *authority_map(s, successor)? != observation.successor() {
            return Err(refuse("authority-successor"));
        }
        Ok(Self {
            pre_nonce_count: UInt(observation.pre_count() as u128),
            pre_nonce_present: false,
            post_nonce_count: UInt(observation.post_count() as u128),
            post_nonce_present: true,
        })
    }
}

pub(crate) fn check_successor_input(
    s: &SignedStaticBindings,
    predecessor: &layout::NativeState,
    successor: &midnight_onchain_state::state::ChargedState<InMemoryDB>,
    input_cap: u64,
) -> Result<()> {
    let cap = input_cap.min(codec::INPUT_BYTES as u64);
    crate::call_artifacts::encode_bounded(successor, cap)
        .map_err(|_| refuse("successor-input-size"))?;
    let address = codec::from_hex(&s.bindings.address_hex)?;
    let mut snapshot = predecessor.clone();
    let mut contract = snapshot.index(address).ok_or(refuse("contract-absent"))?;
    contract.data = successor.clone();
    snapshot.contract = snapshot.contract.insert(address, contract);
    crate::call_artifacts::encode_bounded(&snapshot, cap)
        .map_err(|_| refuse("successor-input-size"))?;
    Ok(())
}
