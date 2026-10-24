//! SDK-free typed host arithmetic for original UIREQ030, little-endian layout v1.
//!
//! Each checked entrypoint retains its complete mathematical intermediate before
//! selecting the exact UInt128 value or legacy error 10. These host components
//! produce no circuit constraints, native ABI, proof or circuit-cost evidence.
//! The existing private byte auxiliary and reference machine are unchanged.
//! Candidate preparation status: SOURCE_ONLY / UNRUN.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarryBit(pub bool);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BorrowBit(pub bool);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MulCarry32(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddCarry32 {
    pub low: u32,
    pub carry_out: CarryBit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubBorrow32 {
    pub low: u32,
    pub borrow_out: BorrowBit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MulWide32 {
    pub low: u32,
    pub high: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MulAccumulate32 {
    pub low: u32,
    pub carry_out: MulCarry32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FullAdd32 {
    pub low: [u32; 4],
    pub carry_out: CarryBit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FullSub32 {
    pub low: [u32; 4],
    pub borrow_out: BorrowBit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FullProduct32 {
    pub low: [u32; 4],
    pub high: [u32; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckedUInt128Add32 {
    pub full: FullAdd32,
    pub checked: Result<u128, u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckedUInt128Sub32 {
    pub full: FullSub32,
    pub checked: Result<u128, u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckedUInt128Mul32 {
    pub full: FullProduct32,
    pub checked: Result<u128, u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MulCarry64(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddCarry64 {
    pub low: u64,
    pub carry_out: CarryBit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubBorrow64 {
    pub low: u64,
    pub borrow_out: BorrowBit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MulWide64 {
    pub low: u64,
    pub high: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MulAccumulate64 {
    pub low: u64,
    pub carry_out: MulCarry64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FullAdd64 {
    pub low: [u64; 2],
    pub carry_out: CarryBit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FullSub64 {
    pub low: [u64; 2],
    pub borrow_out: BorrowBit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FullProduct64 {
    pub low: [u64; 2],
    pub high: [u64; 2],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckedUInt128Add64 {
    pub full: FullAdd64,
    pub checked: Result<u128, u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckedUInt128Sub64 {
    pub full: FullSub64,
    pub checked: Result<u128, u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckedUInt128Mul64 {
    pub full: FullProduct64,
    pub checked: Result<u128, u8>,
}

/// `a + bv + carry_in = low + 2^32 * carry_out`.
pub fn add_carry_32(a: u32, bv: u32, carry_in: CarryBit) -> AddCarry32 {
    // At most 2^33-1, which fits u64.
    let total = u64::from(a) + u64::from(bv) + u64::from(carry_in.0);
    AddCarry32 {
        low: total as u32,
        carry_out: CarryBit((total >> 32) != 0),
    }
}

/// `a - bv - borrow_in = low - 2^32 * borrow_out` over mathematical integers.
pub fn sub_borrow_32(a: u32, bv: u32, borrow_in: BorrowBit) -> SubBorrow32 {
    let subtrahend = u64::from(bv) + u64::from(borrow_in.0);
    let borrow_out = u64::from(a) < subtrahend;
    // The chosen borrow makes this exact difference lie in the word range.
    let low = (u64::from(a) + (u64::from(borrow_out) << 32) - subtrahend) as u32;
    SubBorrow32 {
        low,
        borrow_out: BorrowBit(borrow_out),
    }
}

/// The complete two-word product, with named little-endian halves.
pub fn mul_wide_32(a: u32, bv: u32) -> MulWide32 {
    let product = u64::from(a) * u64::from(bv);
    MulWide32 {
        low: product as u32,
        high: (product >> 32) as u32,
    }
}

/// Multiply, add one stored word and one whole-word incoming carry.
/// The maximum total is `(2^32-1)^2 + 2*(2^32-1) = 2^64-1`.
pub fn mul_accumulate_32(a: u32, bv: u32, addend: u32, carry_in: MulCarry32) -> MulAccumulate32 {
    let product = mul_wide_32(a, bv);
    let with_addend = add_carry_32(product.low, addend, CarryBit(false));
    let with_carry = add_carry_32(with_addend.low, carry_in.0, CarryBit(false));
    // The bound above guarantees this exact high sum fits a whole word.
    let high = u64::from(product.high)
        + u64::from(with_addend.carry_out.0)
        + u64::from(with_carry.carry_out.0);
    MulAccumulate32 {
        low: with_carry.low,
        carry_out: MulCarry32(high as u32),
    }
}

/// `a + bv + carry_in = low + 2^64 * carry_out`.
pub fn add_carry_64(a: u64, bv: u64, carry_in: CarryBit) -> AddCarry64 {
    // At most 2^65-1, which fits u128.
    let total = u128::from(a) + u128::from(bv) + u128::from(carry_in.0);
    AddCarry64 {
        low: total as u64,
        carry_out: CarryBit((total >> 64) != 0),
    }
}

/// `a - bv - borrow_in = low - 2^64 * borrow_out` over mathematical integers.
pub fn sub_borrow_64(a: u64, bv: u64, borrow_in: BorrowBit) -> SubBorrow64 {
    let subtrahend = u128::from(bv) + u128::from(borrow_in.0);
    let borrow_out = u128::from(a) < subtrahend;
    // The chosen borrow makes this exact difference lie in the word range.
    let low = (u128::from(a) + (u128::from(borrow_out) << 64) - subtrahend) as u64;
    SubBorrow64 {
        low,
        borrow_out: BorrowBit(borrow_out),
    }
}

/// The complete two-word product, with named little-endian halves.
pub fn mul_wide_64(a: u64, bv: u64) -> MulWide64 {
    let product = u128::from(a) * u128::from(bv);
    MulWide64 {
        low: product as u64,
        high: (product >> 64) as u64,
    }
}

/// Multiply, add one stored word and one whole-word incoming carry.
/// The maximum total is `(2^64-1)^2 + 2*(2^64-1) = 2^128-1`.
pub fn mul_accumulate_64(a: u64, bv: u64, addend: u64, carry_in: MulCarry64) -> MulAccumulate64 {
    let product = mul_wide_64(a, bv);
    let with_addend = add_carry_64(product.low, addend, CarryBit(false));
    let with_carry = add_carry_64(with_addend.low, carry_in.0, CarryBit(false));
    // The bound above guarantees this exact high sum fits a whole word.
    let high = u128::from(product.high)
        + u128::from(with_addend.carry_out.0)
        + u128::from(with_carry.carry_out.0);
    MulAccumulate64 {
        low: with_carry.low,
        carry_out: MulCarry64(high as u64),
    }
}

fn split_words_32(value: u128) -> [u32; 4] {
    let mut limbs = [0; 4];
    for (i, limb) in limbs.iter_mut().enumerate() {
        *limb = (value >> (32 * i)) as u32;
    }
    limbs
}

fn join_words_32(limbs: [u32; 4]) -> u128 {
    let mut value = 0u128;
    for (i, limb) in limbs.iter().enumerate() {
        value |= u128::from(*limb) << (32 * i);
    }
    value
}

/// Retain the full 129-bit sum, then select the checked UInt128 outcome.
pub fn checked_u128_add_32(a: u128, bv: u128) -> CheckedUInt128Add32 {
    let left = split_words_32(a);
    let right = split_words_32(bv);
    let mut low = [0; 4];
    let mut carry_out = CarryBit(false);
    for i in 0..4 {
        let word = add_carry_32(left[i], right[i], carry_out);
        low[i] = word.low;
        carry_out = word.carry_out;
    }
    let full = FullAdd32 { low, carry_out };
    let checked = if full.carry_out.0 {
        Err(10)
    } else {
        Ok(join_words_32(full.low))
    };
    CheckedUInt128Add32 { full, checked }
}

/// Retain `low - 2^128 * borrow_out`, then select the checked UInt128 outcome.
pub fn checked_u128_sub_32(a: u128, bv: u128) -> CheckedUInt128Sub32 {
    let left = split_words_32(a);
    let right = split_words_32(bv);
    let mut low = [0; 4];
    let mut borrow_out = BorrowBit(false);
    for i in 0..4 {
        let word = sub_borrow_32(left[i], right[i], borrow_out);
        low[i] = word.low;
        borrow_out = word.borrow_out;
    }
    let full = FullSub32 { low, borrow_out };
    let checked = if full.borrow_out.0 {
        Err(10)
    } else {
        Ok(join_words_32(full.low))
    };
    CheckedUInt128Sub32 { full, checked }
}

/// Retain all 256 product bits before inspecting the high UInt128 half.
pub fn checked_u128_mul_32(a: u128, bv: u128) -> CheckedUInt128Mul32 {
    let left = split_words_32(a);
    let right = split_words_32(bv);
    let mut product = [0; 8];
    // Fixed schoolbook rows. Before row i, product[i+4] is untouched and zero;
    // previous rows end at i+4-1. Its final carry is therefore assigned exactly.
    for i in 0..4 {
        let mut carry = MulCarry32(0);
        for j in 0..4 {
            let word = mul_accumulate_32(left[i], right[j], product[i + j], carry);
            product[i + j] = word.low;
            carry = word.carry_out;
        }
        product[i + 4] = carry.0;
    }
    let mut low = [0; 4];
    let mut high = [0; 4];
    low.copy_from_slice(&product[..4]);
    high.copy_from_slice(&product[4..]);
    let full = FullProduct32 { low, high };
    let checked = if full.high.iter().any(|&word| word != 0) {
        Err(10)
    } else {
        Ok(join_words_32(full.low))
    };
    CheckedUInt128Mul32 { full, checked }
}

fn split_words_64(value: u128) -> [u64; 2] {
    let mut limbs = [0; 2];
    for (i, limb) in limbs.iter_mut().enumerate() {
        *limb = (value >> (64 * i)) as u64;
    }
    limbs
}

fn join_words_64(limbs: [u64; 2]) -> u128 {
    let mut value = 0u128;
    for (i, limb) in limbs.iter().enumerate() {
        value |= u128::from(*limb) << (64 * i);
    }
    value
}

/// Retain the full 129-bit sum, then select the checked UInt128 outcome.
pub fn checked_u128_add_64(a: u128, bv: u128) -> CheckedUInt128Add64 {
    let left = split_words_64(a);
    let right = split_words_64(bv);
    let mut low = [0; 2];
    let mut carry_out = CarryBit(false);
    for i in 0..2 {
        let word = add_carry_64(left[i], right[i], carry_out);
        low[i] = word.low;
        carry_out = word.carry_out;
    }
    let full = FullAdd64 { low, carry_out };
    let checked = if full.carry_out.0 {
        Err(10)
    } else {
        Ok(join_words_64(full.low))
    };
    CheckedUInt128Add64 { full, checked }
}

/// Retain `low - 2^128 * borrow_out`, then select the checked UInt128 outcome.
pub fn checked_u128_sub_64(a: u128, bv: u128) -> CheckedUInt128Sub64 {
    let left = split_words_64(a);
    let right = split_words_64(bv);
    let mut low = [0; 2];
    let mut borrow_out = BorrowBit(false);
    for i in 0..2 {
        let word = sub_borrow_64(left[i], right[i], borrow_out);
        low[i] = word.low;
        borrow_out = word.borrow_out;
    }
    let full = FullSub64 { low, borrow_out };
    let checked = if full.borrow_out.0 {
        Err(10)
    } else {
        Ok(join_words_64(full.low))
    };
    CheckedUInt128Sub64 { full, checked }
}

/// Retain all 256 product bits before inspecting the high UInt128 half.
pub fn checked_u128_mul_64(a: u128, bv: u128) -> CheckedUInt128Mul64 {
    let left = split_words_64(a);
    let right = split_words_64(bv);
    let mut product = [0; 4];
    // Fixed schoolbook rows. Before row i, product[i+2] is untouched and zero;
    // previous rows end at i+2-1. Its final carry is therefore assigned exactly.
    for i in 0..2 {
        let mut carry = MulCarry64(0);
        for j in 0..2 {
            let word = mul_accumulate_64(left[i], right[j], product[i + j], carry);
            product[i + j] = word.low;
            carry = word.carry_out;
        }
        product[i + 2] = carry.0;
    }
    let mut low = [0; 2];
    let mut high = [0; 2];
    low.copy_from_slice(&product[..2]);
    high.copy_from_slice(&product[2..]);
    let full = FullProduct64 { low, high };
    let checked = if full.high.iter().any(|&word| word != 0) {
        Err(10)
    } else {
        Ok(join_words_64(full.low))
    };
    CheckedUInt128Mul64 { full, checked }
}

#[cfg(test)]
#[path = "word_arithmetic_tests.rs"]
mod tests;
