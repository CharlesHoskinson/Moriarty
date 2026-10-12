use super::{
    Group,
    arithmetic::{Builder, imm},
    refusal,
};
use crate::{Result, codec, layout, ops, profile::Deployment};
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
struct Compiler<'a> {
    p: &'a Program,
    d: &'a Deployment,
    action: &'a str,
    b: Builder,
    env: BTreeMap<String, Wire>,
    states: BTreeMap<String, Type>,
    duties: BTreeMap<String, (Operand, Operand)>,
    claims: BTreeMap<String, Operand>,
}
impl Compiler<'_> {
    fn duty(&mut self, name: &str, value: Operand, discharge: bool) -> Result<()> {
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
            Statement::Transfer { asset, to, .. } | Statement::Fee { asset, to, .. } => {
                self.claim(asset, to, value.val)?
            }
            Statement::Liability { name, .. } => self.duty(name, value.val, false)?,
            Statement::Discharge { name, .. } => self.duty(name, value.val, true)?,
            Statement::Emit { name, .. } => {
                super::event::emit(&mut self.b, name, self.action, value.val, value.ty)?
            }
        }
        Ok(())
    }
}
pub fn compile(p: &Program, d: &Deployment, action: &str) -> Result<(IrSource, Vec<Group>, usize)> {
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
        c.env.insert(input.name.clone(), Wire { val, ty: input.ty });
    }
    for op in ops::abi_guard() {
        if matches!(op, midnight_onchain_vm::ops::Op::Popeq { .. }) {
            let v = c.b.open(imm(1));
            c.b.equal(v, imm(3));
        }
        c.b.op(op, imm(1), &[], "abi-root")?;
    }
    for statement in &a.statements {
        c.statement(statement)?;
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
        a.inputs.len(),
    ))
}
