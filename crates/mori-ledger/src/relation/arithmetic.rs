use super::{DynamicField, Group, refusal};
use crate::{Result, ops::NativeOp};
use midnight_transient_crypto::{curve::Fr, repr::FieldRepr};
use midnight_zkir_v3::{Identifier, Instruction, ir::Operand, ir_types::IrType};
use mori_core::{BinaryOp, Type};

pub fn imm(n: u128) -> Operand {
    Operand::Immediate(Fr::from(n))
}
pub fn field(n: Fr) -> Operand {
    Operand::Immediate(n)
}

#[derive(Default)]
pub struct Builder {
    pub instructions: Vec<Instruction>,
    pub groups: Vec<Group>,
    pub dynamic: Vec<DynamicField>,
    serial: u32,
}
impl Builder {
    pub fn fresh(&mut self) -> Identifier {
        let id = Identifier(format!("%native.{}", self.serial));
        self.serial += 1;
        id
    }
    pub fn range(&mut self, val: Operand, ty: Type) {
        self.instructions.push(match ty {
            Type::UInt => Instruction::ConstrainBits { val, bits: 128 },
            Type::Bool => Instruction::ConstrainToBoolean { val },
        });
    }
    pub fn bits(&mut self, val: Operand, bits: u32) {
        self.instructions
            .push(Instruction::ConstrainBits { val, bits });
    }
    pub fn equal(&mut self, a: Operand, b: Operand) {
        self.instructions.push(Instruction::ConstrainEq { a, b });
    }
    pub fn assert(&mut self, cond: Operand) {
        self.instructions.push(Instruction::Assert { cond });
    }
    pub fn unary(&mut self, a: Operand, not: bool) -> Operand {
        let output = self.fresh();
        self.instructions.push(if not {
            Instruction::Not {
                a,
                output: output.clone(),
            }
        } else {
            Instruction::Neg {
                a,
                output: output.clone(),
            }
        });
        Operand::Variable(output)
    }
    pub fn binary(&mut self, a: Operand, b: Operand, op: BinaryOp) -> Operand {
        let output = self.fresh();
        self.instructions.push(match op {
            BinaryOp::Add => Instruction::Add {
                a,
                b,
                output: output.clone(),
            },
            BinaryOp::Mul => Instruction::Mul {
                a,
                b,
                output: output.clone(),
            },
            BinaryOp::Eq => Instruction::TestEq {
                a,
                b,
                output: output.clone(),
            },
            BinaryOp::Lt => Instruction::LessThan {
                a,
                b,
                bits: 128,
                output: output.clone(),
            },
            _ => unreachable!("internal primitive operator"),
        });
        Operand::Variable(output)
    }
    pub fn select(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let output = self.fresh();
        self.instructions.push(Instruction::CondSelect {
            bit,
            a,
            b,
            output: output.clone(),
        });
        Operand::Variable(output)
    }
    pub fn split(&mut self, val: Operand, bits: u32) -> (Operand, Operand) {
        let high = self.fresh();
        let low = self.fresh();
        self.instructions.push(Instruction::DivModPowerOfTwo {
            val,
            bits,
            outputs: vec![high.clone(), low.clone()],
        });
        (Operand::Variable(high), Operand::Variable(low))
    }
    fn checked_mul(&mut self, a: Operand, b: Operand) -> Operand {
        let (ah, al) = self.split(a, 64);
        let (bh, bl) = self.split(b, 64);
        for limb in [&ah, &al, &bh, &bl] {
            self.bits(limb.clone(), 64);
        }
        // All local integer equations are <2^129, far below the native modulus.
        let p = self.binary(al.clone(), bl.clone(), BinaryOp::Mul);
        self.bits(p.clone(), 128);
        let (carry, low) = self.split(p, 64);
        self.bits(carry.clone(), 64);
        self.bits(low.clone(), 64);
        let c1 = self.binary(ah.clone(), bl, BinaryOp::Mul);
        self.bits(c1.clone(), 128);
        let c2 = self.binary(al, bh.clone(), BinaryOp::Mul);
        self.bits(c2.clone(), 128);
        let cross = self.binary(c1, c2, BinaryOp::Add);
        self.bits(cross.clone(), 129);
        let cross = self.binary(cross, carry, BinaryOp::Add);
        self.bits(cross.clone(), 129);
        let (overflow, middle) = self.split(cross, 64);
        self.bits(overflow.clone(), 65);
        self.bits(middle.clone(), 64);
        let high_product = self.binary(ah, bh, BinaryOp::Mul);
        self.bits(high_product.clone(), 128);
        let high = self.binary(high_product, overflow, BinaryOp::Add);
        self.bits(high.clone(), 129);
        self.equal(high, imm(0));
        let shifted = self.binary(middle, imm(1u128 << 64), BinaryOp::Mul);
        self.bits(shifted.clone(), 128);
        let result = self.binary(shifted, low, BinaryOp::Add);
        self.bits(result.clone(), 128);
        result
    }
    pub fn expression(&mut self, a: Operand, b: Operand, op: BinaryOp) -> Operand {
        let result = match op {
            BinaryOp::Add => self.binary(a, b, BinaryOp::Add),
            BinaryOp::Sub => {
                let neg = self.unary(b, false);
                self.binary(a, neg, BinaryOp::Add)
            }
            BinaryOp::Mul => return self.checked_mul(a, b),
            BinaryOp::Eq => self.binary(a, b, BinaryOp::Eq),
            BinaryOp::Ne => {
                let eq = self.binary(a, b, BinaryOp::Eq);
                self.unary(eq, true)
            }
            BinaryOp::Lt => self.binary(a, b, BinaryOp::Lt),
            BinaryOp::Gt => self.binary(b, a, BinaryOp::Lt),
            BinaryOp::Le => {
                let gt = self.binary(b, a, BinaryOp::Lt);
                self.unary(gt, true)
            }
            BinaryOp::Ge => {
                let lt = self.binary(a, b, BinaryOp::Lt);
                self.unary(lt, true)
            }
            BinaryOp::And => self.binary(a, b, BinaryOp::Mul),
            BinaryOp::Or => {
                let na = self.unary(a, true);
                let nb = self.unary(b, true);
                let both = self.binary(na, nb, BinaryOp::Mul);
                self.unary(both, true)
            }
        };
        self.range(
            result.clone(),
            if matches!(op, BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul) {
                Type::UInt
            } else {
                Type::Bool
            },
        );
        result
    }
    pub fn open(&mut self, guard: Operand) -> Operand {
        let output = self.fresh();
        self.instructions.push(Instruction::PublicInput {
            guard: Some(guard.clone()),
            val_t: IrType::Native,
            output: output.clone(),
        });
        // The SDK guard alone is not an in-circuit restriction.
        self.select(guard, Operand::Variable(output), imm(0))
    }
    pub fn op(
        &mut self,
        op: NativeOp,
        guard: Operand,
        replacements: &[(usize, Operand)],
        origin: &str,
    ) -> Result<()> {
        let mut inputs = op.field_vec().into_iter().map(field).collect::<Vec<_>>();
        if inputs.is_empty() {
            return Err(refusal("empty-operation"));
        }
        for (index, val) in replacements {
            let cell = inputs.get_mut(*index).ok_or(refusal("template-offset"))?;
            *cell = val.clone();
            self.dynamic.push(DynamicField {
                group: self.groups.len(),
                offset: *index,
            });
        }
        self.groups.push(Group {
            field_count: inputs.len(),
            origin: origin.into(),
        });
        self.instructions
            .push(Instruction::Impact { guard, inputs });
        Ok(())
    }
}
