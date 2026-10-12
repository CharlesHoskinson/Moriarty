//! Constraint-only adaptation of frozen Fenrir FENS. All identifiers come
//! from the ledger Builder; no auth Impact or private transcript is emitted.
use super::*;
use crate::relation::arithmetic::{Builder, field, imm};
use midnight_zkir_v3::{
    Identifier, Instruction,
    ir::{Operand, TypedIdentifier},
    ir_types::IrType,
};
use mori_core::{BinaryOp, Type};
use std::collections::BTreeMap;
pub(crate) fn arguments(out: &mut Vec<TypedIdentifier>, n: usize, assets: usize) -> Result<()> {
    for i in 0..11 + 4 * assets {
        out.push(serde_json::from_value(serde_json::json!({"name":format!("%input.{}",n+i),"type":if i<2 {IrType::JubjubPoint}else{IrType::Native}})).map_err(|_|refuse("typed-arguments"))?);
    }
    Ok(())
}
fn arg(i: usize) -> Operand {
    Operand::Variable(Identifier(format!("%input.{i}")))
}
fn coordinates(b: &mut Builder, p: Operand) -> (Operand, Operand) {
    let (x, y) = (b.fresh(), b.fresh());
    b.instructions.push(Instruction::IntoCoordinates {
        point: p,
        outputs: (x.clone(), y.clone()),
    });
    (Operand::Variable(x), Operand::Variable(y))
}
fn hash(b: &mut Builder, inputs: Vec<Operand>) -> Operand {
    let output = b.fresh();
    b.instructions.push(Instruction::TransientHash {
        inputs,
        output: output.clone(),
    });
    Operand::Variable(output)
}
fn scalar(b: &mut Builder, native: Operand) -> Operand {
    let output = b.fresh();
    b.instructions.push(Instruction::JubjubScalarFromNative {
        native,
        output: output.clone(),
    });
    Operand::Variable(output)
}
fn nonidentity(b: &mut Builder, x: Operand) {
    let zero = b.binary(x, imm(0), BinaryOp::Eq);
    let ok = b.unary(zero, true);
    b.assert(ok);
}
fn le(b: &mut Builder, a: Operand, c: Operand) {
    let ok = b.expression(a, c, BinaryOp::Le);
    b.assert(ok);
}
fn sum(b: &mut Builder, v: Vec<Operand>) -> Operand {
    v.into_iter()
        .fold(imm(0), |a, c| b.expression(a, c, BinaryOp::Add))
}
pub(crate) fn append(
    b: &mut Builder,
    s: &SignedStaticBindings,
    context: [u128; 2],
    n: usize,
    wires: &[(WireDescriptor, Operand)],
    payments: &[(String, String, String, bool, Operand)],
    duties: &BTreeMap<String, (Operand, Operand)>,
) -> Result<()> {
    let (key, r, response) = (arg(n), arg(n + 1), arg(n + 2));
    let ranged = (0..8 + 4 * s.assets.len())
        .map(|i| arg(n + 3 + i))
        .collect::<Vec<_>>();
    for val in &ranged {
        b.range(val.clone(), Type::UInt);
    }
    let (px, py) = coordinates(b, key.clone());
    let (kx, ky) = schnorr::coordinates(schnorr::decode_key(&s.policy.public_key_hex)?);
    b.equal(px.clone(), field(kx));
    b.equal(py.clone(), field(ky));
    nonidentity(b, px.clone());
    let mut preimage = vec![
        imm(MESSAGE_DOMAIN),
        imm(1),
        imm(context[0]),
        imm(context[1]),
        ranged[0].clone(),
        ranged[1].clone(),
        imm(wires.len() as u128),
    ];
    for (d, v) in wires {
        preimage.push(imm(if d.ty == Type::Bool { 1 } else { 2 }));
        preimage.push(v.clone());
    }
    preimage.push(imm(s.assets.len() as u128));
    for (i, asset) in s.assets.iter().enumerate() {
        let limbs = digest_limbs(&codec::object_hash(asset)?)?;
        preimage.extend([imm(limbs[0]), imm(limbs[1])]);
        preimage.extend(ranged[8 + 4 * i..12 + 4 * i].iter().cloned());
    }
    preimage.extend([
        ranged[2].clone(),
        ranged[3].clone(),
        ranged[4].clone(),
        ranged[5].clone(),
        ranged[6].clone(),
    ]);
    let message = hash(b, preimage);
    let (rx, ry) = coordinates(b, r.clone());
    nonidentity(b, rx.clone());
    let challenge = hash(
        b,
        vec![imm(schnorr::FENS_DOMAIN.into()), rx, ry, px, py, message],
    );
    let e = scalar(b, challenge);
    let response_scalar = scalar(b, response.clone());
    let roundtrip = b.fresh();
    b.instructions.push(Instruction::Encode {
        input: response_scalar.clone(),
        outputs: vec![roundtrip.clone()],
    });
    b.equal(Operand::Variable(roundtrip), response);
    let (lhs, ek, rhs) = (b.fresh(), b.fresh(), b.fresh());
    b.instructions.push(Instruction::EcMulGenerator {
        scalar: response_scalar,
        output: lhs.clone(),
    });
    b.instructions.push(Instruction::EcMul {
        a: key,
        scalar: e,
        output: ek.clone(),
    });
    b.instructions.push(Instruction::Add {
        a: r,
        b: Operand::Variable(ek),
        output: rhs.clone(),
    });
    let valid = b.binary(Operand::Variable(lhs), Operand::Variable(rhs), BinaryOp::Eq);
    b.assert(valid);
    le(b, ranged[3].clone(), ranged[7].clone());
    le(b, ranged[7].clone(), ranged[4].clone());
    for (i, asset) in s.assets.iter().enumerate() {
        let mut outgoing = vec![];
        let mut fees = vec![];
        let mut credits = vec![];
        let mut debits = vec![];
        for (name, from, to, fee, v) in payments {
            if s.program
                .assets
                .iter()
                .find(|a| &a.name == name)
                .is_none_or(|a| &a.identity != asset)
            {
                continue;
            }
            if *fee {
                if *from == s.policy.owner {
                    fees.push(v.clone());
                } else if *from == s.policy.beneficiary {
                    debits.push(v.clone());
                }
            } else {
                if *from == s.policy.owner {
                    outgoing.push(v.clone());
                }
                if *to == s.policy.beneficiary {
                    credits.push(v.clone());
                }
                if *from == s.policy.beneficiary {
                    debits.push(v.clone());
                }
            }
        }
        let f = sum(b, fees);
        let t = sum(b, outgoing);
        let g = b.expression(t, f.clone(), BinaryOp::Add);
        let credit = sum(b, credits);
        let debit = sum(b, debits);
        let net = b.expression(credit, debit, BinaryOp::Sub);
        let net = b.expression(net, f.clone(), BinaryOp::Sub);
        let closing = s
            .duties
            .iter()
            .filter(|d| d.asset == *asset)
            .map(|d| duties[&d.name].1.clone())
            .collect();
        let duty = sum(b, closing);
        le(b, g, ranged[8 + 4 * i].clone());
        le(b, f, ranged[9 + 4 * i].clone());
        le(b, ranged[10 + 4 * i].clone(), net);
        le(b, duty, ranged[11 + 4 * i].clone());
    }
    Ok(())
}
