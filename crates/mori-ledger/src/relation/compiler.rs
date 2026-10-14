use super::{
    Group,
    arithmetic::{Builder, imm},
    refusal,
};
use crate::{Result, codec, layout, ops, profile::StaticBindings};
use midnight_base_crypto::fab::AlignedValue;
use midnight_coin_structure::coin::{PublicAddress, TokenType, UnshieldedTokenType, UserAddress};
use midnight_onchain_runtime::context::ClaimedUnshieldedSpendsKey;
use midnight_transient_crypto::{fab::AlignedValueExt, repr::FieldRepr};
use midnight_zkir_v3::{
    Identifier, Instruction, IrSource,
    ir::{IrMinorVersion, Operand, TypedIdentifier},
    ir_types::IrType,
};
use mori_core::{Action, BinaryOp, Expr, ExprId, Program, Statement, Type, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

#[derive(Clone)]
struct Wire {
    val: Operand,
    ty: Type,
}
pub(crate) struct Compiler<'a> {
    p: &'a Program,
    d: &'a StaticBindings,
    action: &'a str,
    b: Builder,
    env: BTreeMap<String, Wire>,
    states: BTreeMap<String, Type>,
    duties: BTreeMap<String, (Operand, Operand)>,
    claims: BTreeMap<String, Operand>,
    signed: Option<&'a crate::signed::SignedStaticBindings>,
    wires: Vec<(crate::signed::WireDescriptor, Operand)>,
    payments: Vec<(String, String, String, bool, Operand)>,
    statement_index: usize,
}
impl Compiler<'_> {
    fn wires_effect_index(&self) -> usize {
        self.statement_index
    }
    fn record(&mut self, descriptor: String, ty: Type, val: Operand) {
        if self.signed.is_some() {
            self.wires.push((
                crate::signed::WireDescriptor {
                    position: self.wires.len(),
                    descriptor,
                    ty,
                },
                val,
            ));
        }
    }
    fn authority_observation(
        &mut self,
        post: bool,
        nonce: Operand,
        pre: Option<Operand>,
    ) -> Result<Operand> {
        let count = self.b.open(imm(1));
        self.b.range(count.clone(), Type::UInt);
        let cap =
            self.b.expression(
                count.clone(),
                imm(crate::signed::nonce::AUTHORIZATION_MAP_CAPACITY as u128
                    - if post { 0 } else { 1 }),
                BinaryOp::Le,
            );
        self.b.assert(cap);
        let nonempty = self.b.expression(imm(0), count.clone(), BinaryOp::Lt);
        self.b.assert(nonempty);
        if let Some(pre) = pre {
            let expected = self.b.expression(pre, imm(1), BinaryOp::Add);
            self.b.equal(count.clone(), expected);
        }
        for (i, op) in ops::map_size(3, 0).into_iter().enumerate() {
            let r = if i == 3 {
                vec![(op.field_size() - 1, count.clone())]
            } else {
                vec![]
            };
            self.b.op(
                op,
                imm(1),
                &r,
                if post {
                    "signed-authority-post-count"
                } else {
                    "signed-authority-pre-count"
                },
            )?;
        }
        let member = self.b.open(imm(1));
        self.b.range(member.clone(), Type::Bool);
        self.b.equal(member.clone(), imm(u128::from(post)));
        let template = ops::membership(3, 0u128.into(), post);
        let last = template.len() - 1;
        for (i, op) in template.into_iter().enumerate() {
            let r = if i == 2 {
                vec![(op.field_size() - 1, nonce.clone())]
            } else if i == last {
                vec![(op.field_size() - 1, member.clone())]
            } else {
                vec![]
            };
            self.b.op(
                op,
                imm(1),
                &r,
                if post {
                    "signed-authority-post-member"
                } else {
                    "signed-authority-pre-member"
                },
            )?;
        }
        if post {
            self.record(
                "post.authority.nonce_count".into(),
                Type::UInt,
                count.clone(),
            );
            self.record("post.authority.nonce.present".into(), Type::Bool, member);
        }
        Ok(count)
    }
    fn authority_write(&mut self, nonce: Operand) -> Result<()> {
        for (i, op) in ops::write_cell(3, 0u128.into(), true.into())
            .into_iter()
            .enumerate()
        {
            let r = if i == 1 {
                vec![(op.field_size() - 1, nonce.clone())]
            } else {
                vec![]
            };
            self.b.op(op, imm(1), &r, "signed-authority-write")?;
        }
        Ok(())
    }
    fn observations(&mut self, post: bool) -> Result<()> {
        let phase = if post { "post" } else { "pre" };
        let state_count = self.b.open(imm(1));
        self.b.equal(state_count, imm(self.p.states.len() as u128));
        for op in ops::map_size(0, self.p.states.len()) {
            self.b.op(op, imm(1), &[], "signed-state-cardinality")?;
        }
        for state in &self.p.states {
            self.read(&state.name)?;
            self.record(
                format!("{phase}.state.{}", state.name),
                state.ty,
                self.env[&state.name].val.clone(),
            );
        }
        let duties = self.signed.ok_or(refusal("signed-layout"))?.duties.clone();
        let count = self.b.open(imm(1));
        self.b.range(count.clone(), Type::UInt);
        for (i, op) in ops::map_size(1, 0).into_iter().enumerate() {
            let r = if i == 3 {
                vec![(op.field_size() - 1, count.clone())]
            } else {
                vec![]
            };
            self.b.op(op, imm(1), &r, "signed-duty-cardinality")?;
        }
        let mut sum = imm(0);
        for d in &duties {
            let (member, amount) = self.observe_duty(&d.name)?;
            sum = self.b.expression(sum, member.clone(), BinaryOp::Add);
            self.record(
                format!("{phase}.duty.{}.present", d.name),
                Type::Bool,
                member.clone(),
            );
            self.record(
                format!("{phase}.duty.{}.amount", d.name),
                Type::UInt,
                amount.clone(),
            );
            self.duties.insert(d.name.clone(), (member, amount));
        }
        self.b.equal(count, sum);
        Ok(())
    }

    fn observe_duty(&mut self, name: &str) -> Result<(Operand, Operand)> {
        let binding = self
            .d
            .duties
            .iter()
            .find(|d| d.name == name)
            .ok_or(refusal("duty-binding"))?;
        let key = layout::name_key("duty", name);
        let member = self.b.open(imm(1));
        self.b.range(member.clone(), Type::Bool);
        if let Some((expected, _)) = self.duties.get(name) {
            self.b.equal(member.clone(), expected.clone());
        }
        let template = ops::membership(1, key.clone(), false);
        let last = template.len() - 1;
        for (i, op) in template.into_iter().enumerate() {
            let r = if i == last {
                vec![(op.field_size() - 1, member.clone())]
            } else {
                vec![]
            };
            self.b.op(op, imm(1), &r, "duty-membership")?;
        }
        let sample = layout::duty_value(binding, 0, &self.d.roles)?;
        let mut constants = vec![];
        sample.value_only_field_repr(&mut constants);
        let mut openings = vec![];
        for (i, constant) in constants.iter().enumerate() {
            let open = self.b.open(member.clone());
            if i + 1 < constants.len() {
                let expected =
                    self.b
                        .select(member.clone(), super::arithmetic::field(*constant), imm(0));
                self.b.equal(open.clone(), expected);
            } else {
                self.b.range(open.clone(), Type::UInt);
                let absent = self.b.select(member.clone(), imm(0), open.clone());
                self.b.equal(absent, imm(0));
                if let Some((_, known)) = self.duties.get(name) {
                    let expected = self.b.select(member.clone(), known.clone(), imm(0));
                    self.b.equal(open.clone(), expected);
                }
            }
            openings.push(open);
        }
        let before = openings.last().cloned().ok_or(refusal("duty-template"))?;
        let template = ops::read_cell(1, key.clone(), sample.clone());
        let last = template.len() - 1;
        for (i, op) in template.into_iter().enumerate() {
            let r = if i == last {
                let start = op.field_size() - openings.len();
                openings
                    .iter()
                    .enumerate()
                    .map(|(j, v)| (start + j, v.clone()))
                    .collect()
            } else {
                vec![]
            };
            self.b.op(op, member.clone(), &r, "duty-old-read")?;
        }
        // Retain every observation as the current ordered snapshot. Repeated
        // leaves must agree until a source duty write replaces this pair.
        self.duties
            .insert(name.to_owned(), (member.clone(), before.clone()));
        Ok((member, before))
    }
    fn duty(&mut self, name: &str, value: Operand, discharge: bool) -> Result<()> {
        let binding = self
            .d
            .duties
            .iter()
            .find(|d| d.name == name)
            .ok_or(refusal("duty-binding"))?;
        let key = layout::name_key("duty", name);
        let sample = layout::duty_value(binding, 0, &self.d.roles)?;
        let (member, before) = self.observe_duty(name)?;
        if discharge {
            self.record(
                format!(
                    "effect.{}.discharge.{name}.amount",
                    self.wires_effect_index()
                ),
                Type::UInt,
                value.clone(),
            );
        }
        self.record(
            format!(
                "effect.{}.liability.{name}.before",
                self.wires_effect_index()
            ),
            Type::UInt,
            before.clone(),
        );
        if discharge {
            self.b.assert(member.clone());
        }
        let next = self.b.expression(
            before,
            value,
            if discharge {
                BinaryOp::Sub
            } else {
                BinaryOp::Add
            },
        );
        for (i, op) in ops::write_cell(1, key, sample).into_iter().enumerate() {
            let r = if i == 2 {
                vec![(op.field_size() - 1, next.clone())]
            } else {
                vec![]
            };
            self.b.op(op, imm(1), &r, "duty-write")?;
        }
        self.record(
            format!(
                "effect.{}.liability.{name}.after",
                self.wires_effect_index()
            ),
            Type::UInt,
            next.clone(),
        );
        self.duties.insert(name.into(), (imm(1), next));
        Ok(())
    }
    fn claim(&mut self, asset: &str, to: &str, amount: Operand) -> Result<()> {
        let id = &self
            .p
            .assets
            .iter()
            .find(|a| a.name == asset)
            .ok_or(refusal("asset"))?
            .identity;
        let token: UnshieldedTokenType = codec::from_hex(
            &self
                .d
                .assets
                .iter()
                .find(|a| a.identity == *id)
                .ok_or(refusal("asset-binding"))?
                .token_hex,
        )?;
        let address: UserAddress = codec::from_hex(
            &self
                .d
                .roles
                .iter()
                .find(|r| r.role == to)
                .ok_or(refusal("role-binding"))?
                .address_hex,
        )?;
        let key = ClaimedUnshieldedSpendsKey::from_inner(
            TokenType::Unshielded(token),
            PublicAddress::User(address),
        );
        let label = codec::native_hex(&AlignedValue::from(key.clone()))?;
        let total = self.claims.get(&label).cloned().unwrap_or_else(|| imm(0));
        let zero = self.b.binary(amount.clone(), imm(0), BinaryOp::Eq);
        let positive = self.b.unary(zero, true);
        let prior_zero = self.b.binary(total.clone(), imm(0), BinaryOp::Eq);
        let prior = self.b.unary(prior_zero, true);
        self.b.range(positive.clone(), Type::Bool);
        self.b.range(prior.clone(), Type::Bool);
        let old_guard = self
            .b
            .binary(positive.clone(), prior.clone(), BinaryOp::Mul);
        self.b.range(old_guard.clone(), Type::Bool);
        let next = self.b.expression(total.clone(), amount, BinaryOp::Add);
        let member = self.b.open(positive.clone());
        self.b.range(member.clone(), Type::Bool);
        let expected_member = self.b.select(positive.clone(), prior, imm(0));
        self.b.equal(member.clone(), expected_member);
        let old = self.b.open(old_guard.clone());
        self.b.range(old.clone(), Type::UInt);
        let expected_old = self.b.select(old_guard.clone(), total, imm(0));
        self.b.equal(old.clone(), expected_old);
        // Build the complete repeated template with scalar placeholders. All
        // duplicated native keys remain immutable source/deployment constants.
        for (i, op) in ops::claim_update(key, Some(0), 1)?.into_iter().enumerate() {
            let guard = if (6..9).contains(&i) {
                old_guard.clone()
            } else {
                positive.clone()
            };
            let dynamic = match i {
                5 => Some(member.clone()),
                8 => Some(old.clone()),
                10 => Some(next.clone()),
                _ => None,
            };
            let r = dynamic
                .map(|v| vec![(op.field_size() - 1, v)])
                .unwrap_or_default();
            self.b.op(op, guard, &r, "source-claim")?;
        }
        self.claims.insert(label, next);
        Ok(())
    }
    fn read(&mut self, name: &str) -> Result<()> {
        let ty = *self.states.get(name).ok_or(refusal("state-name"))?;
        let val = self.b.open(imm(1));
        self.b.range(val.clone(), ty);
        if let Some(prior) = self.env.get(name) {
            self.b.equal(val.clone(), prior.val.clone());
        }
        let zero = match ty {
            Type::UInt => 0u128.into(),
            Type::Bool => false.into(),
        };
        let mut template = ops::read_cell(0, layout::name_key("state", name), zero);
        let popeq = template.pop().ok_or(refusal("read-template"))?;
        for op in template {
            self.b.op(op, imm(1), &[], "state-read")?;
        }
        self.b.op(
            popeq.clone(),
            imm(1),
            &[(popeq.field_size() - 1, val.clone())],
            "state-opening",
        )?;
        self.env.insert(name.into(), Wire { val, ty });
        Ok(())
    }
    fn names(
        &self,
        id: ExprId,
        names: &mut BTreeSet<String>,
        seen: &mut BTreeSet<ExprId>,
    ) -> Result<()> {
        if !seen.insert(id) {
            return Ok(());
        }
        match self
            .p
            .expressions
            .get(id as usize)
            .ok_or(refusal("expression-index"))?
        {
            Expr::Name { name } if self.states.contains_key(name) => {
                names.insert(name.clone());
            }
            Expr::Binary { lhs, rhs, .. } => {
                self.names(*lhs, names, seen)?;
                self.names(*rhs, names, seen)?;
            }
            Expr::Not { value } => self.names(*value, names, seen)?,
            _ => {}
        }
        Ok(())
    }
    fn expr(&mut self, id: ExprId, cache: &mut BTreeMap<ExprId, Wire>) -> Result<Wire> {
        if let Some(w) = cache.get(&id) {
            return Ok(w.clone());
        }
        let w = match self
            .p
            .expressions
            .get(id as usize)
            .ok_or(refusal("expression-index"))?
            .clone()
        {
            Expr::LiabilityPresent { name } => {
                let (member, _) = self.observe_duty(&name)?;
                Wire {
                    val: member,
                    ty: Type::Bool,
                }
            }
            Expr::LiabilityAmount { name } => {
                let (_, amount) = self.observe_duty(&name)?;
                Wire {
                    val: amount,
                    ty: Type::UInt,
                }
            }
            Expr::Literal { value } => Wire {
                val: imm(match value {
                    Value::UInt(n) => n.0,
                    Value::Bool(b) => u128::from(b),
                }),
                ty: value.ty(),
            },
            Expr::Name { name } => self
                .env
                .get(&name)
                .cloned()
                .ok_or(refusal("unopened-name"))?,
            Expr::Not { value } => {
                let a = self.expr(value, cache)?;
                Wire {
                    val: self.b.unary(a.val, true),
                    ty: Type::Bool,
                }
            }
            Expr::Binary { op, lhs, rhs } => {
                let a = self.expr(lhs, cache)?;
                let b = self.expr(rhs, cache)?;
                Wire {
                    val: self.b.expression(a.val, b.val, op),
                    ty: if matches!(op, BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul) {
                        Type::UInt
                    } else {
                        Type::Bool
                    },
                }
            }
        };
        self.b.range(w.val.clone(), w.ty);
        cache.insert(id, w.clone());
        Ok(w)
    }
    fn statement(&mut self, s: &Statement) -> Result<()> {
        let id = match s {
            Statement::Let { value, .. }
            | Statement::Next { value, .. }
            | Statement::Emit { value, .. } => *value,
            Statement::Requires { predicate } | Statement::Ensures { predicate } => *predicate,
            Statement::Transfer { amount, .. }
            | Statement::Fee { amount, .. }
            | Statement::Liability { amount, .. }
            | Statement::Discharge { amount, .. } => *amount,
        };
        let mut names = BTreeSet::new();
        self.names(id, &mut names, &mut BTreeSet::new())?;
        for name in &names {
            self.read(name)?;
        }
        let value = self.expr(id, &mut BTreeMap::new())?;
        match s {
            Statement::Let { name, .. } => {
                self.env.insert(name.clone(), value);
            }
            Statement::Requires { .. } | Statement::Ensures { .. } => self.b.assert(value.val),
            Statement::Next { state, .. } => {
                if !names.contains(state) {
                    self.read(state)?;
                }
                self.record(
                    format!("effect.{}.state.{state}.before", self.statement_index),
                    value.ty,
                    self.env[state].val.clone(),
                );
                self.record(
                    format!("effect.{}.state.{state}.after", self.statement_index),
                    value.ty,
                    value.val.clone(),
                );
                let sample: AlignedValue = match value.ty {
                    Type::UInt => 0u128.into(),
                    Type::Bool => false.into(),
                };
                let template = ops::write_cell(0, layout::name_key("state", state), sample);
                for (i, op) in template.into_iter().enumerate() {
                    let replacements = if i == 2 {
                        vec![(op.field_size() - 1, value.val.clone())]
                    } else {
                        vec![]
                    };
                    self.b.op(op, imm(1), &replacements, "state-write")?;
                }
                self.env.insert(state.clone(), value);
            }
            Statement::Transfer {
                asset, from, to, ..
            }
            | Statement::Fee {
                asset, from, to, ..
            } => {
                let fee = matches!(s, Statement::Fee { .. });
                self.record(
                    format!(
                        "effect.{}.{}.{asset}.{from}.{to}.amount",
                        self.statement_index,
                        if fee { "fee" } else { "transfer" }
                    ),
                    Type::UInt,
                    value.val.clone(),
                );
                self.payments.push((
                    asset.clone(),
                    from.clone(),
                    to.clone(),
                    fee,
                    value.val.clone(),
                ));
                self.claim(asset, to, value.val)?
            }
            Statement::Liability { name, .. } => self.duty(name, value.val, false)?,
            Statement::Discharge { name, .. } => self.duty(name, value.val, true)?,
            Statement::Emit { name, .. } => {
                self.record(
                    format!("effect.{}.event.{name}.value", self.statement_index),
                    value.ty,
                    value.val.clone(),
                );
                super::event::emit(&mut self.b, name, self.action, value.val, value.ty)?
            }
        }
        Ok(())
    }
}
pub fn compile(
    validated: &layout::ValidatedStaticBindings<'_>,
    action: &str,
) -> Result<(IrSource, Vec<Group>, usize)> {
    compile_inner(validated, action, None, None).map(|(ir, g, n, _)| (ir, g, n))
}
pub(crate) fn compile_signed(
    validated: &layout::ValidatedStaticBindings<'_>,
    action: &str,
    signed: &crate::signed::SignedStaticBindings,
    context: Option<[u128; 2]>,
) -> Result<(
    IrSource,
    Vec<Group>,
    usize,
    Vec<crate::signed::WireDescriptor>,
)> {
    compile_inner(validated, action, Some(signed), context)
}
fn compile_inner(
    validated: &layout::ValidatedStaticBindings<'_>,
    action: &str,
    signed: Option<&crate::signed::SignedStaticBindings>,
    context: Option<[u128; 2]>,
) -> Result<(
    IrSource,
    Vec<Group>,
    usize,
    Vec<crate::signed::WireDescriptor>,
)> {
    let p = validated.program();
    let d = validated.bindings();
    let a: &Action = p
        .actions
        .iter()
        .find(|a| a.name == action)
        .ok_or(refusal("action"))?;
    let mut c = Compiler {
        p,
        d,
        action: &a.name,
        b: Builder::default(),
        env: BTreeMap::new(),
        states: p.states.iter().map(|s| (s.name.clone(), s.ty)).collect(),
        duties: BTreeMap::new(),
        claims: BTreeMap::new(),
        signed,
        wires: vec![],
        payments: vec![],
        statement_index: 0,
    };
    let mut inputs = vec![];
    for (i, input) in a.inputs.iter().enumerate() {
        let id = Identifier(format!("%input.{i}"));
        let typed: TypedIdentifier =
            serde_json::from_value(serde_json::json!({"name":id,"type":IrType::Native}))
                .map_err(|_| refusal("input-type"))?;
        inputs.push(typed);
        let val = Operand::Variable(id);
        c.b.range(val.clone(), input.ty);
        c.record(format!("input.{}", input.name), input.ty, val.clone());
        c.env.insert(input.name.clone(), Wire { val, ty: input.ty });
    }
    if let Some(s) = signed {
        crate::signed::constraints::arguments(&mut inputs, a.inputs.len(), s.assets.len())?;
    }
    let consuming = signed.is_some_and(|s| s.version == crate::signed::SignedVersion::ConsumingV2);
    let mut guard = ops::abi_guard();
    if consuming {
        guard[2] = midnight_onchain_vm::ops::Op::Popeq {
            cached: false,
            result: 4u64.into(),
        };
    }
    for op in guard {
        if matches!(op, midnight_onchain_vm::ops::Op::Popeq { .. }) {
            let v = c.b.open(imm(1));
            c.b.equal(v, imm(if consuming { 4 } else { 3 }));
        }
        c.b.op(
            op,
            imm(1),
            &[],
            if consuming {
                "signed-abi-root"
            } else {
                "abi-root"
            },
        )?;
    }
    let nonce = Operand::Variable(Identifier(format!("%input.{}", a.inputs.len() + 5)));
    let authority_count = if consuming {
        let s = signed.ok_or(refusal("signed-layout"))?;
        let domain: AlignedValue =
            midnight_base_crypto::hash::HashOutput(crate::signed::nonce::nonce_domain(s)?).into();
        let mut fields = vec![];
        domain.value_only_field_repr(&mut fields);
        for expected in fields {
            let actual = c.b.open(imm(1));
            c.b.equal(actual, super::arithmetic::field(expected));
        }
        for op in ops::read_cell(3, crate::signed::nonce::domain_key(), domain) {
            c.b.op(op, imm(1), &[], "signed-authority-domain")?;
        }
        Some(c.authority_observation(false, nonce.clone(), None)?)
    } else {
        None
    };
    if signed.is_some() {
        c.observations(false)?;
        if let Some(count) = &authority_count {
            c.record(
                "pre.authority.nonce_count".into(),
                Type::UInt,
                count.clone(),
            );
            c.record("pre.authority.nonce.present".into(), Type::Bool, imm(0));
        }
    }
    for (i, statement) in a.statements.iter().enumerate() {
        c.statement_index = i;
        c.statement(statement)?;
    }
    if let Some(signed) = signed {
        c.observations(true)?;
        if let Some(count) = authority_count {
            c.authority_write(nonce.clone())?;
            c.authority_observation(true, nonce, Some(count))?;
        }
        if let Some(context) = context {
            crate::signed::constraints::append(
                &mut c.b,
                signed,
                context,
                a.inputs.len(),
                &c.wires,
                &c.payments,
                &c.duties,
            )?;
        }
    }
    c.b.instructions.push(Instruction::Output { vals: vec![] });
    if c.b.instructions.len() > 1_000_000 || c.b.groups.len() > 100_000 {
        return Err(refusal("compiled-work-bound"));
    }
    Ok((
        IrSource {
            version: IrMinorVersion::V0,
            inputs,
            outputs: vec![],
            do_communications_commitment: true,
            instructions: Arc::new(c.b.instructions),
        },
        c.b.groups,
        a.inputs.len() + signed.map_or(0, |s| 13 + 4 * s.assets.len()),
        c.wires.into_iter().map(|(d, _)| d).collect(),
    ))
}
