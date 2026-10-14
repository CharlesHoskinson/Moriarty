//! Actual native Op construction. Financial arithmetic stays checked u128 on
//! the host; VM Add/Sub/Lt are u64 and are never used by these templates.
use crate::{Error, Result};
use midnight_base_crypto::fab::AlignedValue;
use midnight_onchain_runtime::context::ClaimedUnshieldedSpendsKey;
use midnight_onchain_state::state::StateValue;
use midnight_onchain_vm::{
    ops::{Key, Op},
    result_mode::ResultModeVerify,
};
use midnight_storage::{arena::Sp, db::InMemoryDB};
pub type NativeOp = Op<ResultModeVerify, InMemoryDB>;
pub fn cell(value: AlignedValue) -> StateValue<InMemoryDB> {
    StateValue::Cell(Sp::new(value))
}
pub fn push(value: AlignedValue) -> NativeOp {
    Op::Push {
        storage: false,
        value: cell(value),
    }
}
pub fn key(value: AlignedValue) -> Key {
    Key::Value(value)
}
pub fn abi_guard() -> Vec<NativeOp> {
    vec![
        Op::Dup { n: 0 },
        Op::Size,
        Op::Popeq {
            cached: false,
            result: 3u64.into(),
        },
    ]
}
pub fn claim_update(
    claim: ClaimedUnshieldedSpendsKey,
    previous: Option<u128>,
    amount: u128,
) -> Result<Vec<NativeOp>> {
    if amount == 0 {
        return Ok(vec![]);
    }
    let next = previous
        .unwrap_or(0)
        .checked_add(amount)
        .ok_or(Error::new("semantic", "claim-overflow"))?;
    let k: AlignedValue = claim.into();
    let mut ops = vec![
        Op::Swap { n: 0 },
        Op::Idx {
            cached: false,
            push_path: true,
            path: vec![key(8u8.into())].into(),
        },
        Op::Dup { n: 0 },
        push(k.clone()),
        Op::Member,
        Op::Popeq {
            cached: false,
            result: previous.is_some().into(),
        },
    ];
    if let Some(old) = previous {
        ops.extend([
            Op::Dup { n: 0 },
            Op::Idx {
                cached: false,
                push_path: false,
                path: vec![key(k.clone())].into(),
            },
            Op::Popeq {
                cached: false,
                result: old.into(),
            },
        ]);
    }
    ops.extend([
        push(k),
        push(next.into()),
        Op::Ins {
            cached: false,
            n: 2,
        },
        Op::Swap { n: 0 },
    ]);
    Ok(ops)
}

pub fn map_size(slot: u8, size: usize) -> Vec<NativeOp> {
    vec![
        Op::Dup { n: 0 },
        Op::Idx {
            cached: false,
            push_path: false,
            path: vec![key(slot.into())].into(),
        },
        Op::Size,
        Op::Popeq {
            cached: false,
            result: (size as u64).into(),
        },
    ]
}
pub fn read_cell(slot: u8, k: AlignedValue, expected: AlignedValue) -> Vec<NativeOp> {
    vec![
        Op::Dup { n: 0 },
        Op::Idx {
            cached: false,
            push_path: false,
            path: vec![key(slot.into()), key(k)].into(),
        },
        Op::Popeq {
            cached: false,
            result: expected,
        },
    ]
}
pub fn membership(slot: u8, k: AlignedValue, present: bool) -> Vec<NativeOp> {
    vec![
        Op::Dup { n: 0 },
        Op::Idx {
            cached: false,
            push_path: false,
            path: vec![key(slot.into())].into(),
        },
        push(k),
        Op::Member,
        Op::Popeq {
            cached: false,
            result: present.into(),
        },
    ]
}
pub fn write_cell(slot: u8, k: AlignedValue, value: AlignedValue) -> Vec<NativeOp> {
    vec![
        Op::Idx {
            cached: false,
            push_path: true,
            path: vec![key(slot.into())].into(),
        },
        push(k),
        Op::Push {
            storage: true,
            value: cell(value),
        },
        Op::Ins {
            cached: false,
            n: 2,
        },
    ]
}

use crate::layout::{self, AdmittedPredecessor};
use midnight_coin_structure::coin::{PublicAddress, TokenType};
use midnight_onchain_runtime::context::Effects;
use midnight_onchain_vm::ops::{LogEventType, VersionedLogItem};
use midnight_transient_crypto::{fab::AlignedValueExt, repr::FieldRepr};
use mori_core::{Effect, Execution, Program, Statement};
#[derive(Debug, Clone, serde::Serialize)]
pub struct OpBinding {
    pub op: u32,
    pub step: Option<u16>,
    pub origin: String,
    pub source_expression: Option<u16>,
    pub fields: Vec<String>,
    pub static_fields: Vec<usize>,
    pub dynamic_field_offset: Option<usize>,
    pub dynamic_field_count: usize,
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct FieldBinding {
    pub op: u32,
    pub offset: usize,
    pub count: usize,
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClaimBinding {
    pub step: u16,
    pub effect_in_step: usize,
    pub source_expression: Option<u16>,
    pub asset: mori_core::AssetId,
    pub recipient: String,
    pub source_amount: mori_core::UInt,
    pub native_key_hex: String,
    pub native_amount_hex: String,
    pub expected_membership: bool,
    pub previous_aggregate: Option<mori_core::UInt>,
    pub next_aggregate: mori_core::UInt,
    pub membership_result: FieldBinding,
    pub previous_result: Option<FieldBinding>,
    pub previous_lookup_key: Option<FieldBinding>,
    pub new_value: FieldBinding,
    pub key_values: Vec<FieldBinding>,
}
pub struct LoweredOps {
    pub program: Vec<NativeOp>,
    pub bindings: Vec<OpBinding>,
    pub claim_bindings: Vec<ClaimBinding>,
    pub expected_root: StateValue<InMemoryDB>,
    pub expected_effects: Effects<InMemoryDB>,
    pub events: Vec<VersionedLogItem<InMemoryDB>>,
}
fn add(
    out: &mut LoweredOps,
    ops: Vec<NativeOp>,
    step: Option<u16>,
    origin: &str,
    expression: Option<u16>,
) {
    for op in ops {
        let fields = op.field_vec();
        let count = if origin == "abi-root-array3" || origin == "signed-abi-root" {
            0
        } else {
            match &op {
                Op::Push {
                    value: StateValue::Cell(v),
                    ..
                } => v.value_only_field_size(),
                Op::Push {
                    value: StateValue::Array(a),
                    ..
                } if origin == "source-event" => match a.get(2) {
                    Some(StateValue::Cell(v)) => v.value_only_field_size(),
                    _ => 0,
                },
                Op::Idx {
                    push_path: false,
                    path,
                    ..
                } if origin == "claim-prefix-member-old-new" => match path.get(0) {
                    Some(Key::Value(v)) => v.value_only_field_size(),
                    _ => 0,
                },
                Op::Popeq { result, .. } => result.value_only_field_size(),
                _ => 0,
            }
        };
        out.bindings.push(OpBinding {
            op: out.program.len() as u32,
            step,
            origin: origin.into(),
            source_expression: expression,
            fields: fields
                .iter()
                .map(|f| crate::codec::hex(&f.as_le_bytes()))
                .collect(),
            static_fields: (0..fields.len() - count).collect(),
            dynamic_field_offset: if count > 0 {
                Some(fields.len() - count)
            } else {
                None
            },
            dynamic_field_count: count,
        });
        out.program.push(op);
    }
}
fn update_root(
    root: &mut StateValue<InMemoryDB>,
    slot: usize,
    k: AlignedValue,
    v: AlignedValue,
) -> Result<()> {
    let StateValue::Array(a) = root else {
        return Err(Error::new("full-readback", "root-type"));
    };
    let m = layout::map(
        a.get(slot)
            .ok_or(Error::new("full-readback", "root-slot"))?,
    )?
    .insert(k, cell(v));
    let updated = a
        .insert(slot, StateValue::Map(m))
        .ok_or(Error::new("full-readback", "root-slot"))?;
    *root = StateValue::Array(updated);
    Ok(())
}
pub fn lower_ops(p: &Program, pre: &AdmittedPredecessor, e: &Execution) -> Result<LoweredOps> {
    lower_ops_inner(p, pre, e, None)
}
fn lower_ops_inner(
    p: &Program,
    pre: &AdmittedPredecessor,
    e: &Execution,
    signed: Option<&crate::signed::SignedStaticBindings>,
) -> Result<LoweredOps> {
    let mut out = LoweredOps {
        program: vec![],
        bindings: vec![],
        claim_bindings: vec![],
        expected_root: (*pre.contract.data.get()).clone(),
        expected_effects: Default::default(),
        events: vec![],
    };
    let consuming = signed.is_some_and(|s| s.version == crate::signed::SignedVersion::ConsumingV2);
    let mut guard = abi_guard();
    if consuming {
        guard[2] = Op::Popeq {
            cached: false,
            result: 4u64.into(),
        };
    }
    add(
        &mut out,
        guard,
        None,
        if consuming {
            "signed-abi-root"
        } else {
            "abi-root-array3"
        },
        None,
    );
    // Read each source reference at its statement prefix. The complete frame
    // comparator also checks untouched cells; redundant opening reads add no
    // source obligation and consume the native guaranteed partition budget.
    let mut totals = std::collections::BTreeMap::new();
    let mut prefix = e.pre.clone();
    for step in &e.steps {
        let expression = match &step.statement {
            Statement::Next { value, .. }
            | Statement::Emit { value, .. }
            | Statement::Let { value, .. } => Some(*value),
            Statement::Transfer { amount, .. }
            | Statement::Fee { amount, .. }
            | Statement::Liability { amount, .. }
            | Statement::Discharge { amount, .. } => Some(*amount),
            Statement::Requires { predicate } | Statement::Ensures { predicate } => {
                Some(*predicate)
            }
        };
        let mut names = std::collections::BTreeSet::new();
        let mut duty_reads = Vec::new();
        if let Some(id) = expression {
            let mut visited = std::collections::BTreeSet::new();
            let mut pending = vec![id];
            while let Some(id) = pending.pop() {
                if !visited.insert(id) {
                    continue;
                }
                let expr = p
                    .expressions
                    .get(id as usize)
                    .ok_or(Error::new("source-profile", "expression-index"))?;
                match expr {
                    mori_core::Expr::Name { name } => {
                        if prefix.state.contains_key(name) {
                            names.insert(name.clone());
                        }
                    }
                    mori_core::Expr::Binary { lhs, rhs, .. } => {
                        pending.push(*rhs);
                        pending.push(*lhs);
                    }
                    mori_core::Expr::Not { value } => pending.push(*value),
                    mori_core::Expr::LiabilityPresent { name }
                    | mori_core::Expr::LiabilityAmount { name } => {
                        duty_reads.push((id, name.clone()));
                    }
                    _ => {}
                }
            }
            for name in &names {
                add(
                    &mut out,
                    read_cell(
                        0,
                        layout::name_key("state", name),
                        layout::value(&prefix.state[name]),
                    ),
                    Some(step.index),
                    "source-expression-prefix-state",
                    Some(id),
                );
            }
        }
        // Match Compiler::statement: sorted state reads first, followed by
        // eager left-to-right expression DFS, once per ExprId in this statement.
        // Distinct leaves naming one duty still each observe its current cell.
        for (id, name) in duty_reads {
            if !pre.duties.contains_key(&name) {
                return Err(Error::new("identity", "duty-binding"));
            }
            let k = layout::name_key("duty", &name);
            let maps = if let Some(s) = signed {
                crate::signed::nonce::root_maps(s, &out.expected_root)?
            } else {
                layout::root_maps(&out.expected_root)?
            };
            let observed = maps[1].get(&k).map(|v| layout::cell(&v)).transpose()?;
            add(
                &mut out,
                membership(1, k.clone(), observed.is_some()),
                Some(step.index),
                "duty-membership",
                Some(id),
            );
            if let Some(value) = observed {
                add(
                    &mut out,
                    read_cell(1, k, value),
                    Some(step.index),
                    "duty-old-read",
                    Some(id),
                );
            }
        }
        for (effect_index, effect) in step.effects.iter().enumerate() {
            match effect {
                Effect::State {
                    name,
                    before,
                    after,
                } => {
                    let k = layout::name_key("state", name);
                    if !names.contains(name) {
                        add(
                            &mut out,
                            read_cell(0, k.clone(), layout::value(before)),
                            Some(step.index),
                            "state-before",
                            expression,
                        );
                    }
                    add(
                        &mut out,
                        write_cell(0, k.clone(), layout::value(after)),
                        Some(step.index),
                        "state-after",
                        expression,
                    );
                    update_root(&mut out.expected_root, 0, k, layout::value(after))?;
                }
                Effect::Liability {
                    name,
                    asset,
                    debtor,
                    creditor,
                    before,
                    after,
                } => {
                    let binding = pre
                        .duties
                        .get(name)
                        .ok_or(Error::new("identity", "duty-binding"))?;
                    if &binding.asset != asset
                        || &binding.debtor != debtor
                        || &binding.creditor != creditor
                    {
                        return Err(Error::new("identity", "duty-binding"));
                    }
                    let k = layout::name_key("duty", name);
                    let maps = if let Some(s) = signed {
                        crate::signed::nonce::root_maps(s, &out.expected_root)?
                    } else {
                        layout::root_maps(&out.expected_root)?
                    };
                    let present = maps[1].get(&k).is_some();
                    add(
                        &mut out,
                        membership(1, k.clone(), present),
                        Some(step.index),
                        "duty-membership",
                        expression,
                    );
                    if present {
                        add(
                            &mut out,
                            read_cell(
                                1,
                                k.clone(),
                                layout::duty_value(binding, before.0, &pinned_roles(pre))?,
                            ),
                            Some(step.index),
                            "duty-before",
                            expression,
                        );
                    }
                    let next = layout::duty_value(binding, after.0, &pinned_roles(pre))?;
                    add(
                        &mut out,
                        write_cell(1, k.clone(), next.clone()),
                        Some(step.index),
                        "duty-after",
                        expression,
                    );
                    update_root(&mut out.expected_root, 1, k, next)?;
                }
                Effect::Transfer {
                    asset, to, amount, ..
                }
                | Effect::Fee {
                    asset, to, amount, ..
                } => {
                    let claim = ClaimedUnshieldedSpendsKey::from_inner(
                        TokenType::Unshielded(pre.assets[asset]),
                        PublicAddress::User(pre.roles[to].address),
                    );
                    let previous = totals.get(&claim.into_inner()).copied();
                    let next = previous
                        .unwrap_or(0u128)
                        .checked_add(amount.0)
                        .ok_or(Error::new("semantic", "claim-overflow"))?;
                    if amount.0 > 0 {
                        let start = out.program.len();
                        let ops = claim_update(claim.clone(), previous, amount.0)?;
                        add(
                            &mut out,
                            ops,
                            Some(step.index),
                            "claim-prefix-member-old-new",
                            expression,
                        );
                        let tail = |index: usize| -> Result<FieldBinding> {
                            let b = out
                                .bindings
                                .get(start + index)
                                .ok_or(Error::new("full-readback", "claim-binding-index"))?;
                            Ok(FieldBinding {
                                op: b.op,
                                offset: b
                                    .dynamic_field_offset
                                    .ok_or(Error::new("full-readback", "claim-scalar-tail"))?,
                                count: b.dynamic_field_count,
                            })
                        };
                        let extra = if previous.is_some() { 3 } else { 0 };
                        out.claim_bindings.push(ClaimBinding {
                            step: step.index,
                            effect_in_step: effect_index,
                            source_expression: expression,
                            asset: asset.clone(),
                            recipient: to.clone(),
                            source_amount: *amount,
                            native_key_hex: crate::codec::native_hex(&AlignedValue::from(
                                claim.clone(),
                            ))?,
                            native_amount_hex: crate::codec::native_hex(&AlignedValue::from(
                                amount.0,
                            ))?,
                            expected_membership: previous.is_some(),
                            previous_aggregate: previous.map(mori_core::UInt),
                            next_aggregate: mori_core::UInt(next),
                            membership_result: tail(5)?,
                            previous_result: if previous.is_some() {
                                Some(tail(8)?)
                            } else {
                                None
                            },
                            previous_lookup_key: previous.map(|_| FieldBinding {
                                op: (start + 7) as u32,
                                offset: out.program[start + 7].field_size()
                                    - AlignedValue::from(claim.clone()).value_only_field_size(),
                                count: AlignedValue::from(claim.clone()).value_only_field_size(),
                            }),
                            new_value: tail(7 + extra)?,
                            key_values: vec![tail(3)?, tail(6 + extra)?],
                        });
                        totals.insert(claim.into_inner(), next);
                    }
                }
                Effect::Emit { name, value } => {
                    let bytes = event_payload(name, &e.action, value)?;
                    let payload = cell(AlignedValue::from(bytes));
                    if Sp::new(payload.clone())
                        .serialize_to_node_list_bounded(midnight_onchain_vm::vm::MAX_LOG_EMITTED)
                        .is_none()
                    {
                        return Err(Error::new("native-VM", "event-emitted-byte-limit"));
                    }
                    let wrapper = StateValue::Array(
                        vec![
                            cell(1u32.into()),
                            cell((LogEventType::Misc as u8).into()),
                            payload.clone(),
                        ]
                        .into(),
                    );
                    let ops = vec![
                        Op::Push {
                            storage: false,
                            value: wrapper,
                        },
                        Op::Log,
                    ];
                    add(&mut out, ops, Some(step.index), "source-event", expression);
                    out.events.push(VersionedLogItem {
                        version: 1,
                        event_type: LogEventType::Misc,
                        data: payload,
                    });
                }
            }
        }
        prefix = step.snapshot.clone();
    }
    out.expected_effects.claimed_unshielded_spends = totals
        .into_iter()
        .map(|((token, recipient), amount)| {
            (
                ClaimedUnshieldedSpendsKey::from_inner(token, recipient),
                amount,
            )
        })
        .collect();
    if out.program.len() > 100_000 {
        return Err(Error::new("source-profile", "operation-limit"));
    }
    Ok(out)
}

/// Canonical public event payload, shared by the actual producer and source
/// compiler's static serialization templates. Values retain SDK normalization.
pub fn event_payload(name: &str, action: &str, value: &mori_core::Value) -> Result<Vec<u8>> {
    let encoded = crate::codec::encode(&layout::value(value))?;
    let parts = [name.as_bytes(), action.as_bytes(), encoded.as_slice()];
    let domain = format!("{}:event", crate::profile::ABI);
    let mut bytes = Vec::new();
    bytes.extend((domain.len() as u32).to_le_bytes());
    bytes.extend(domain.as_bytes());
    bytes.extend((parts.len() as u32).to_le_bytes());
    for part in parts {
        bytes.extend((part.len() as u32).to_le_bytes());
        bytes.extend(part);
    }
    Ok(bytes)
}
fn pinned_roles(pre: &AdmittedPredecessor) -> Vec<crate::profile::RoleBinding> {
    pre.roles
        .iter()
        .map(|(name, r)| crate::profile::RoleBinding {
            role: name.clone(),
            public_key_hex: crate::codec::native_hex(&r.key).expect("public native encoding"),
            address_hex: crate::codec::native_hex(&r.address).expect("public native encoding"),
        })
        .collect()
}

/// Read-only signed observations use the same storage ABI and actual native
/// openings as the compiler. They change gas, never semantic effects.
pub(crate) fn lower_signed_ops(
    p: &Program,
    pre: &AdmittedPredecessor,
    e: &Execution,
    s: &crate::signed::SignedStaticBindings,
    authority: Option<&crate::signed::nonce::NonceObservation>,
) -> Result<LoweredOps> {
    if (s.version == crate::signed::SignedVersion::ConsumingV2) != authority.is_some() {
        return Err(Error::new("signed-call", "authority-observation"));
    }
    if let Some(a) = authority {
        a.check_predecessor(s, pre.contract.data.get_ref())?;
    }
    fn endpoint(
        out: &mut LoweredOps,
        p: &Program,
        s: &crate::signed::SignedStaticBindings,
        root: &StateValue<InMemoryDB>,
    ) -> Result<()> {
        let maps = crate::signed::nonce::root_maps(s, root)?;
        add(
            out,
            map_size(0, p.states.len()),
            None,
            "signed-state-cardinality",
            None,
        );
        for state in &p.states {
            let cell = maps[0]
                .get(&layout::name_key("state", &state.name))
                .ok_or(Error::new("signed-call", "missing-state"))?;
            let StateValue::Cell(value) = &*cell else {
                return Err(Error::new("signed-call", "state-type"));
            };
            add(
                out,
                read_cell(0, layout::name_key("state", &state.name), (**value).clone()),
                None,
                "signed-state-opening",
                None,
            );
        }
        add(
            out,
            map_size(1, maps[1].size()),
            None,
            "signed-duty-cardinality",
            None,
        );
        for d in &s.duties {
            let k = layout::name_key("duty", &d.name);
            let value = maps[1].get(&k);
            add(
                out,
                membership(1, k.clone(), value.is_some()),
                None,
                "signed-duty-membership",
                None,
            );
            if let Some(value) = value {
                let StateValue::Cell(value) = &*value else {
                    return Err(Error::new("signed-call", "duty-type"));
                };
                add(
                    out,
                    read_cell(1, k, (**value).clone()),
                    None,
                    "signed-duty-opening",
                    None,
                );
            }
        }
        Ok(())
    }
    let mut out = lower_ops_inner(p, pre, e, Some(s))?;
    let mut initial = LoweredOps {
        program: vec![],
        bindings: vec![],
        claim_bindings: vec![],
        expected_root: out.expected_root.clone(),
        expected_effects: Default::default(),
        events: vec![],
    };
    if let Some(a) = authority {
        add(
            &mut initial,
            read_cell(
                3,
                crate::signed::nonce::domain_key(),
                midnight_base_crypto::hash::HashOutput(crate::signed::nonce::nonce_domain(s)?)
                    .into(),
            ),
            None,
            "signed-authority-domain",
            None,
        );
        add(
            &mut initial,
            map_size(3, a.pre_count()),
            None,
            "signed-authority-pre-count",
            None,
        );
        add(
            &mut initial,
            membership(3, a.nonce().0.into(), false),
            None,
            "signed-authority-pre-member",
            None,
        );
    }
    endpoint(&mut initial, p, s, pre.contract.data.get_ref())?;
    let inserted = initial.program.len();
    for b in &mut out.bindings {
        if b.op >= 3 {
            b.op += inserted as u32;
        }
    }
    for c in &mut out.claim_bindings {
        for b in std::iter::once(&mut c.membership_result)
            .chain(c.previous_result.iter_mut())
            .chain(c.previous_lookup_key.iter_mut())
            .chain(std::iter::once(&mut c.new_value))
            .chain(c.key_values.iter_mut())
        {
            b.op += inserted as u32;
        }
    }
    for b in &mut initial.bindings {
        b.op += 3;
    }
    out.program.splice(3..3, initial.program);
    out.bindings.splice(3..3, initial.bindings);
    let root = out.expected_root.clone();
    endpoint(&mut out, p, s, &root)?;
    if let Some(a) = authority {
        add(
            &mut out,
            write_cell(3, a.nonce().0.into(), true.into()),
            None,
            "signed-authority-write",
            None,
        );
        update_root(&mut out.expected_root, 3, a.nonce().0.into(), true.into())?;
        add(
            &mut out,
            map_size(3, a.post_count()),
            None,
            "signed-authority-post-count",
            None,
        );
        add(
            &mut out,
            membership(3, a.nonce().0.into(), true),
            None,
            "signed-authority-post-member",
            None,
        );
    }
    Ok(out)
}
