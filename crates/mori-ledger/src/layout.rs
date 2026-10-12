//! Typed source-wide ABI and complete public native predecessor admission.
use crate::{Error, Result, codec, profile::*};
use midnight_base_crypto::{cost_model::RunningCost, fab::AlignedValue, hash::HashOutput};
use midnight_coin_structure::{
    coin::{UnshieldedTokenType, UserAddress},
    contract::ContractAddress,
};
use midnight_ledger::structure::{LedgerState, SignatureVerifyingKey, Utxo, UtxoMeta, UtxoSpend};
use midnight_onchain_runtime::context::{BlockContext, CallContext};
use midnight_onchain_state::state::{ContractState, StateValue};
use midnight_storage::{db::InMemoryDB, storage::HashMap};
use mori_core::{AssetId, Balance, Liability, Program, Scenario, Statement, Type, UInt, Value};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
pub type NativeState = LedgerState<InMemoryDB>;
pub type NativeMap = HashMap<AlignedValue, StateValue<InMemoryDB>, InMemoryDB>;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StateField {
    pub name: String,
    pub ty: Type,
    pub key_hex: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LedgerAbi {
    pub profile: String,
    pub language: String,
    pub semantics: String,
    pub program_hash: String,
    pub states: Vec<StateField>,
    pub actions: Vec<mori_core::Action>,
    pub assets: Vec<AssetBinding>,
    pub roles: Vec<RoleBinding>,
    pub duties: Vec<DutyBinding>,
    pub templates_hash: String,
}
pub struct NativeRole {
    pub key: SignatureVerifyingKey,
    pub address: UserAddress,
}
pub struct AdmittedPredecessor {
    pub ledger: NativeState,
    pub contract: ContractState<InMemoryDB>,
    pub address: ContractAddress,
    pub context: CallContext<InMemoryDB>,
    pub scenario: Scenario,
    pub roles: BTreeMap<String, NativeRole>,
    pub assets: BTreeMap<AssetId, UnshieldedTokenType>,
    pub duties: BTreeMap<String, DutyBinding>,
    pub all_utxos: Vec<(Utxo, UtxoMeta)>,
    pub selected: Vec<UtxoSpend>,
    pub gas: RunningCost,
}
fn identity(code: &'static str) -> Error {
    Error::new("identity", code)
}
fn layout_error(code: &'static str) -> Error {
    Error::new("source-profile", code)
}
pub fn digest(domain: &str, parts: &[&[u8]]) -> [u8; 32] {
    let mut bytes = Vec::new();
    bytes.extend((domain.len() as u32).to_le_bytes());
    bytes.extend(domain.as_bytes());
    bytes.extend((parts.len() as u32).to_le_bytes());
    for p in parts {
        bytes.extend((p.len() as u32).to_le_bytes());
        bytes.extend(*p);
    }
    let hex = codec::hash(&bytes);
    let mut result = [0; 32];
    for i in 0..32 {
        result[i] = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).expect("hash encoding");
    }
    result
}
pub fn name_key(category: &str, name: &str) -> AlignedValue {
    HashOutput(digest(&format!("{ABI}:{category}"), &[name.as_bytes()])).into()
}
pub fn asset_digest(asset: &AssetId) -> HashOutput {
    HashOutput(digest(
        &format!("{ABI}:asset"),
        &[
            asset.network.as_bytes(),
            asset.issuer.as_bytes(),
            asset.reference.as_bytes(),
            &[asset.scale],
        ],
    ))
}
pub fn role_digest(role: &RoleBinding) -> Result<HashOutput> {
    Ok(HashOutput(digest(
        &format!("{ABI}:role"),
        &[
            role.role.as_bytes(),
            &codec::unhex(&role.public_key_hex)?,
            &codec::unhex(&role.address_hex)?,
        ],
    )))
}
pub fn value(v: &Value) -> AlignedValue {
    match v {
        Value::UInt(n) => n.0.into(),
        Value::Bool(b) => (*b).into(),
    }
}
pub fn duty_value(d: &DutyBinding, amount: u128, roles: &[RoleBinding]) -> Result<AlignedValue> {
    let debtor = roles
        .iter()
        .find(|r| r.role == d.debtor)
        .ok_or(layout_error("duty-role"))?;
    let creditor = roles
        .iter()
        .find(|r| r.role == d.creditor)
        .ok_or(layout_error("duty-role"))?;
    let parts = [
        AlignedValue::from(asset_digest(&d.asset)),
        AlignedValue::from(role_digest(debtor)?),
        AlignedValue::from(role_digest(creditor)?),
        AlignedValue::from(amount),
    ];
    Ok(AlignedValue::concat(parts.iter()))
}
pub fn templates_hash() -> String {
    codec::hash(include_str!("ops.rs").as_bytes())
}
pub fn admit_source(p: &Program, d: &Deployment) -> Result<LedgerAbi> {
    mori_core::validate(p).map_err(|_| layout_error("core-validation"))?;
    if d.toolchain != Toolchain::pinned() || d.limits != Limits::supported() {
        return Err(identity("toolchain-or-limits"));
    }
    if d.profile != PROFILE
        || d.qualification != "local-unproven"
        || d.phase_policy != "guaranteed-only"
    {
        return Err(identity("local-profile"));
    }
    admit_bindings(p, &StaticBindings::legacy(d))
}
pub(crate) fn admit_bindings(p: &Program, d: &StaticBindings) -> Result<LedgerAbi> {
    mori_core::validate(p).map_err(|_| layout_error("core-validation"))?;
    if d.toolchain != Toolchain::pinned() || d.limits != Limits::supported() {
        return Err(identity("toolchain-or-limits"));
    }
    if d.abi != ABI || d.network.is_empty() || d.anchor != "local-unqualified" {
        return Err(identity("local-profile"));
    }
    let mut roles = BTreeSet::new();
    let mut addresses = BTreeSet::new();
    for r in &d.roles {
        if r.role.is_empty() || r.role.len() > 1024 || !roles.insert(r.role.clone()) {
            return Err(layout_error("duplicate-role"));
        }
        let vk: SignatureVerifyingKey = codec::from_hex(&r.public_key_hex)?;
        let address = UserAddress::from(vk);
        if codec::native_hex(&address)? != r.address_hex || !addresses.insert(address) {
            return Err(identity("role-address"));
        }
    }
    if !roles.contains(&d.payer) {
        return Err(identity("payer-role"));
    }
    let mut assets = BTreeMap::new();
    let mut tokens = BTreeSet::new();
    for a in &d.assets {
        let token: UnshieldedTokenType = codec::from_hex(&a.token_hex)?;
        if assets.insert(a.identity.clone(), token).is_some() || !tokens.insert(token) {
            return Err(identity("asset-mapping"));
        }
    }
    if p.assets.iter().any(|a| !assets.contains_key(&a.identity)) {
        return Err(identity("source-asset"));
    }
    let mut duties = BTreeMap::new();
    let mut key_set = BTreeSet::new();
    for duty in &d.duties {
        if duty.name.is_empty()
            || duty.name.len() > 1024
            || !roles.contains(&duty.debtor)
            || !roles.contains(&duty.creditor)
            || !assets.contains_key(&duty.asset)
            || duties.insert(duty.name.clone(), duty).is_some()
            || !key_set.insert(name_key("duty", &duty.name))
        {
            return Err(identity("duty-mapping"));
        }
    }
    for action in &p.actions {
        for s in &action.statements {
            match s {
                Statement::Transfer {
                    asset, from, to, ..
                }
                | Statement::Fee {
                    asset, from, to, ..
                } => {
                    if from != &d.payer
                        || !roles.contains(to)
                        || !p.assets.iter().any(|a| &a.name == asset)
                    {
                        return Err(layout_error("one-public-payer"));
                    }
                }
                Statement::Liability {
                    name,
                    asset,
                    debtor,
                    creditor,
                    ..
                } => {
                    let a = p
                        .assets
                        .iter()
                        .find(|a| &a.name == asset)
                        .ok_or(layout_error("duty-asset"))?;
                    let bound = duties.get(name).ok_or(identity("source-duty"))?;
                    if bound.asset != a.identity
                        || &bound.debtor != debtor
                        || &bound.creditor != creditor
                    {
                        return Err(identity("source-duty-identity"));
                    }
                }
                Statement::Discharge { name, .. } => {
                    let duty = duties.get(name).ok_or(identity("source-duty"))?;
                    if !p.assets.iter().any(|a| a.identity == duty.asset) {
                        return Err(layout_error("undeclared-duty-asset"));
                    }
                }
                _ => {}
            }
        }
    }
    if d.roles.len() + d.assets.len() + d.duties.len() > mori_core::MAX_TRACE_CELLS {
        return Err(layout_error("binding-work"));
    }
    let mut state_keys = BTreeSet::new();
    let states = p
        .states
        .iter()
        .map(|s| {
            let k = name_key("state", &s.name);
            if !state_keys.insert(k.clone()) {
                return Err(identity("state-key-collision"));
            }
            Ok(StateField {
                name: s.name.clone(),
                ty: s.ty,
                key_hex: codec::native_hex(&k)?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(LedgerAbi {
        profile: ABI.into(),
        language: p.language.clone(),
        semantics: p.semantics.clone(),
        program_hash: codec::object_hash(p)?,
        states,
        actions: p.actions.clone(),
        assets: d.assets.clone(),
        roles: d.roles.clone(),
        duties: d.duties.clone(),
        templates_hash: templates_hash(),
    })
}
pub fn map(v: &StateValue<InMemoryDB>) -> Result<&NativeMap> {
    match v {
        StateValue::Map(m) => Ok(m),
        _ => Err(layout_error("expected-map")),
    }
}
pub fn root_maps(root: &StateValue<InMemoryDB>) -> Result<[&NativeMap; 3]> {
    match root {
        StateValue::Array(a) if a.len() == 3 => Ok([
            map(a.get(0).unwrap())?,
            map(a.get(1).unwrap())?,
            map(a.get(2).unwrap())?,
        ]),
        _ => Err(layout_error("abi-root")),
    }
}
pub fn cell(v: &StateValue<InMemoryDB>) -> Result<AlignedValue> {
    match v {
        StateValue::Cell(c) => Ok((**c).clone()),
        _ => Err(layout_error("expected-cell")),
    }
}
pub fn decode_value(v: &StateValue<InMemoryDB>, ty: Type) -> Result<Value> {
    let a = cell(v)?;
    let decoded = match ty {
        Type::UInt => Value::UInt(UInt(
            u128::try_from(&a.value[..]).map_err(|_| layout_error("uint-alignment"))?,
        )),
        Type::Bool => {
            Value::Bool(bool::try_from(&a.value[..]).map_err(|_| layout_error("bool-alignment"))?)
        }
    };
    if value(&decoded) != a {
        return Err(layout_error("typed-alignment"));
    }
    Ok(decoded)
}
#[derive(Clone, Copy)]
pub(crate) enum OperationPolicy<'a> {
    Keyless,
    Installed {
        public: &'a crate::local_deployment::PublicLocalDeployment,
        limits: &'a NativeWorkLimits,
    },
}
pub(crate) struct AdmittedFrame {
    pub ledger: NativeState,
    pub contract: ContractState<InMemoryDB>,
    pub address: ContractAddress,
    pub scenario: Scenario,
    pub roles: BTreeMap<String, NativeRole>,
    pub assets: BTreeMap<AssetId, UnshieldedTokenType>,
    pub duties: BTreeMap<String, DutyBinding>,
    pub all_utxos: Vec<(Utxo, UtxoMeta)>,
    pub gas_hex: String,
}
pub fn decode_predecessor(
    p: &Program,
    abi: &LedgerAbi,
    d: &Deployment,
    state: NativeState,
    request: &CallRequest,
) -> Result<AdmittedPredecessor> {
    if request.segment != 1 || request.economic_bounds != d.economic_bounds {
        return Err(identity("request-scope"));
    }
    let binding = StaticBindings::legacy(d);
    let frame = admit_frame(
        p,
        abi,
        &binding,
        &FrameBindings::legacy(d),
        state,
        OperationPolicy::Keyless,
    )?;
    attach_request(frame, &binding, request)
}
pub(crate) fn admit_frame(
    p: &Program,
    abi: &LedgerAbi,
    d: &StaticBindings,
    f: &FrameBindings,
    state: NativeState,
    policy: OperationPolicy<'_>,
) -> Result<AdmittedFrame> {
    let policy_cap = match policy {
        OperationPolicy::Keyless => codec::OUTPUT_BYTES as u64,
        OperationPolicy::Installed { limits, .. } => limits.max_input_bytes,
    };
    let address: ContractAddress = codec::from_hex(&d.address_hex)?;
    if scope_hash(&state, policy_cap)? != f.predecessor_hash
        || codec::native_hex(&state.state_hash())? != f.native_state_hash
        || state.network_id != d.network
    {
        return Err(identity("predecessor"));
    }
    state
        .check_night_balance_invariant()
        .map_err(|_| Error::new("native-invariant", "night-supply"))?;
    let contract = state.index(address).ok_or(identity("contract-absent"))?;
    if scope_hash(&contract.operations, policy_cap)? != f.operations_hash
        || scope_hash(&contract.maintenance_authority, policy_cap)? != f.maintenance_hash
        || scope_hash(&contract.balance, policy_cap)? != f.balance_hash
    {
        return Err(identity("contract-frame"));
    }
    let declared: BTreeSet<_> = p
        .actions
        .iter()
        .map(|a| a.name.as_bytes().to_vec())
        .collect();
    let mut actual = BTreeSet::new();
    for e in contract.operations.iter() {
        match policy {
            OperationPolicy::Keyless => {
                if e.1.v2.is_some() || e.1.v3.is_some() || e.1.ir.is_some() {
                    return Err(identity("unproven-operation"));
                }
            }
            OperationPolicy::Installed { public, .. } => public.admit_operation(&e.0.0, &e.1)?,
        }
        actual.insert(e.0.0.clone());
    }
    if actual != declared {
        return Err(identity("operation-set"));
    }
    if codec::object_hash(abi)? != d.abi_hash
        || abi.templates_hash != d.templates_hash
        || abi.program_hash != d.program_hash
    {
        return Err(identity("source-abi"));
    }
    let mut roles = BTreeMap::new();
    for r in &d.roles {
        roles.insert(
            r.role.clone(),
            NativeRole {
                key: codec::from_hex(&r.public_key_hex)?,
                address: codec::from_hex(&r.address_hex)?,
            },
        );
    }
    let mut assets = BTreeMap::new();
    for a in &d.assets {
        assets.insert(a.identity.clone(), codec::from_hex(&a.token_hex)?);
    }
    let duties: BTreeMap<_, _> = d
        .duties
        .iter()
        .map(|d| (d.name.clone(), d.clone()))
        .collect();
    let maps = root_maps(contract.data.get_ref())?;
    if maps[0].size() != p.states.len() {
        return Err(identity("state-membership"));
    }
    let mut scenario = Scenario {
        inputs: Default::default(),
        ..Default::default()
    };
    for s in &p.states {
        let v = maps[0]
            .get(&name_key("state", &s.name))
            .ok_or(identity("missing-state"))?;
        scenario
            .state
            .insert(s.name.clone(), decode_value(&v, s.ty)?);
    }
    let mut seen = BTreeSet::new();
    for row in maps[1].iter() {
        let binding = d
            .duties
            .iter()
            .find(|d| name_key("duty", &d.name) == *row.0)
            .ok_or(identity("unknown-duty"))?;
        let encoded = cell(&row.1)?;
        if encoded.value.0.len() != 4 {
            return Err(identity("duty-cell"));
        }
        let amount = u128::try_from(&encoded.value[3..4]).map_err(|_| identity("duty-amount"))?;
        if duty_value(binding, amount, &d.roles)? != encoded {
            return Err(identity("duty-cell-identity"));
        }
        seen.insert(binding.name.clone());
        if p.assets.iter().any(|a| a.identity == binding.asset) {
            scenario.liabilities.push(Liability {
                name: binding.name.clone(),
                asset: binding.asset.clone(),
                debtor: binding.debtor.clone(),
                creditor: binding.creditor.clone(),
                amount: UInt(amount),
            });
        }
    }
    scenario.liabilities.sort_by(|a, b| a.name.cmp(&b.name));
    let mut all_utxos = Vec::new();
    for row in state.utxo.utxos.iter() {
        all_utxos.push(((*row.0).clone(), (*row.1).clone()));
    }
    if all_utxos.len() > mori_core::MAX_TRACE_CELLS {
        return Err(identity("utxo-work"));
    }
    all_utxos.sort();
    for (role, r) in &roles {
        for a in &p.assets {
            let token = assets[&a.identity];
            let amount = all_utxos
                .iter()
                .filter(|(u, _)| u.owner == r.address && u.type_ == token)
                .try_fold(0u128, |n, (u, _)| {
                    n.checked_add(u.value)
                        .ok_or(Error::new("semantic", "balance-overflow"))
                })?;
            scenario.balances.push(Balance {
                account: role.clone(),
                asset: a.identity.clone(),
                amount: UInt(amount),
            });
        }
    }
    Ok(AdmittedFrame {
        ledger: state,
        contract,
        address,
        scenario,
        roles,
        assets,
        duties,
        all_utxos,
        gas_hex: f.gas_hex.clone(),
    })
}
pub(crate) fn frame_gas(frame: &AdmittedFrame) -> Result<RunningCost> {
    let gas: RunningCost = codec::from_hex(&frame.gas_hex)?;
    let params = &frame.ledger.parameters;
    if gas.read_time > params.limits.block_limits.read_time
        || gas.compute_time > params.limits.block_limits.compute_time
        || gas.bytes_written > params.limits.block_limits.bytes_churned
        || gas.bytes_deleted > params.limits.block_limits.bytes_churned
    {
        return Err(identity("gas-cap"));
    }
    Ok(gas)
}
pub(crate) fn attach_request(
    mut frame: AdmittedFrame,
    d: &StaticBindings,
    request: &CallRequest,
) -> Result<AdmittedPredecessor> {
    if request.segment != 1 || request.economic_bounds != d.economic_bounds {
        return Err(identity("request-scope"));
    }
    frame.scenario.inputs = request.inputs.clone();
    let mut selected = Vec::new();
    let mut outpoints = BTreeSet::new();
    for hex in &request.selected_spends {
        let spend: UtxoSpend = codec::from_hex(hex)?;
        if spend.owner != frame.roles[&d.payer].key
            || !frame.assets.values().any(|tt| *tt == spend.type_)
            || !outpoints.insert((spend.intent_hash, spend.output_no))
            || !frame
                .all_utxos
                .iter()
                .any(|(u, _)| u == &Utxo::from(spend.clone()))
        {
            return Err(identity("selected-utxo"));
        }
        selected.push(spend);
    }
    selected.sort();
    let block: BlockContext = codec::from_hex(&request.block_context_hex)?;
    let context = CallContext {
        own_address: frame.address,
        tblock: block.tblock,
        tblock_err: block.tblock_err,
        parent_block_hash: block.parent_block_hash,
        caller: if selected.is_empty() {
            None
        } else {
            Some(midnight_coin_structure::coin::PublicAddress::User(
                frame.roles[&d.payer].address,
            ))
        },
        balance: frame.contract.balance.clone(),
        com_indices: Default::default(),
        last_block_time: block.last_block_time,
    };
    let gas = frame_gas(&frame)?;
    Ok(AdmittedPredecessor {
        ledger: frame.ledger,
        contract: frame.contract,
        address: frame.address,
        context,
        scenario: frame.scenario,
        roles: frame.roles,
        assets: frame.assets,
        duties: frame.duties,
        all_utxos: frame.all_utxos,
        selected,
        gas,
    })
}

fn scope_hash<T: midnight_serialize::Serializable + midnight_serialize::Tagged>(
    value: &T,
    cap: u64,
) -> Result<String> {
    Ok(codec::hash(&crate::call_artifacts::encode_bounded(
        value, cap,
    )?))
}

/// Validated immutable source/program/ABI view. No serialized binding or caller
/// artifact can construct this private compiler input without full validation.
pub(crate) struct ValidatedStaticBindings<'a> {
    program: &'a Program,
    bindings: &'a StaticBindings,
    abi: LedgerAbi,
}
impl ValidatedStaticBindings<'_> {
    pub(crate) fn program(&self) -> &Program {
        self.program
    }
    pub(crate) fn bindings(&self) -> &StaticBindings {
        self.bindings
    }
    pub(crate) fn abi(&self) -> &LedgerAbi {
        &self.abi
    }
}
pub(crate) fn validate_static<'a>(
    source: &str,
    p: &'a Program,
    d: &'a StaticBindings,
) -> Result<ValidatedStaticBindings<'a>> {
    if source.len() > codec::INPUT_BYTES
        || codec::hash(source.as_bytes()) != d.source_hash
        || mori_core::parse(source).map_err(|_| Error::new("native-relation", "source-parse"))?
            != *p
    {
        return Err(Error::new("native-relation", "source-identity"));
    }
    let abi = admit_bindings(p, d)?;
    if abi.program_hash != d.program_hash
        || abi.templates_hash != d.templates_hash
        || codec::object_hash(&abi)? != d.abi_hash
    {
        return Err(Error::new("native-relation", "static-abi-identity"));
    }
    Ok(ValidatedStaticBindings {
        program: p,
        bindings: d,
        abi,
    })
}
