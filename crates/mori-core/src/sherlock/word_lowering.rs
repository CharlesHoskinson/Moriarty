//! Selected host arithmetic lowering, consumed by the real Scalar family.
//! SOURCE_ONLY / UNRUN. No guard, constraint/native ABI or cost winner.
//! The existing typed component leaf remains the sole limb implementation.
use crate::sherlock_word_arithmetic::{
    CheckedUInt128Add32, CheckedUInt128Add64, CheckedUInt128Mul32, CheckedUInt128Mul64,
    CheckedUInt128Sub32, CheckedUInt128Sub64, checked_u128_add_32, checked_u128_add_64,
    checked_u128_mul_32, checked_u128_mul_64, checked_u128_sub_32, checked_u128_sub_64,
};

/// Caller-selected host realization; independent of wire field/counter widths.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WordLayout {
    Word32,
    Word64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UInt128Operation {
    Add,
    Sub,
    Mul,
}

/// A source arithmetic operation lowered to one exact component family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArithmeticLowering {
    pub layout: WordLayout,
    pub operation: UInt128Operation,
}

/// Named, complete existing component outputs. No intermediate is truncated
/// before the component's original checked UInt128 selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FullArithmeticResult {
    Add32(CheckedUInt128Add32),
    Sub32(CheckedUInt128Sub32),
    Mul32(CheckedUInt128Mul32),
    Add64(CheckedUInt128Add64),
    Sub64(CheckedUInt128Sub64),
    Mul64(CheckedUInt128Mul64),
}

impl ArithmeticLowering {
    pub fn new(layout: WordLayout, operation: UInt128Operation) -> Self {
        Self { layout, operation }
    }

    pub fn component_name(self) -> &'static str {
        match (self.layout, self.operation) {
            (WordLayout::Word32, UInt128Operation::Add) => "checked_u128_add_32",
            (WordLayout::Word32, UInt128Operation::Sub) => "checked_u128_sub_32",
            (WordLayout::Word32, UInt128Operation::Mul) => "checked_u128_mul_32",
            (WordLayout::Word64, UInt128Operation::Add) => "checked_u128_add_64",
            (WordLayout::Word64, UInt128Operation::Sub) => "checked_u128_sub_64",
            (WordLayout::Word64, UInt128Operation::Mul) => "checked_u128_mul_64",
        }
    }

    /// Execute the chosen component on actual already-evaluated source operands.
    /// Each wrapper retains its full intermediate and its original error10 rule.
    pub fn execute(self, left: u128, right: u128) -> FullArithmeticResult {
        match (self.layout, self.operation) {
            (WordLayout::Word32, UInt128Operation::Add) => {
                FullArithmeticResult::Add32(checked_u128_add_32(left, right))
            }
            (WordLayout::Word32, UInt128Operation::Sub) => {
                FullArithmeticResult::Sub32(checked_u128_sub_32(left, right))
            }
            (WordLayout::Word32, UInt128Operation::Mul) => {
                FullArithmeticResult::Mul32(checked_u128_mul_32(left, right))
            }
            (WordLayout::Word64, UInt128Operation::Add) => {
                FullArithmeticResult::Add64(checked_u128_add_64(left, right))
            }
            (WordLayout::Word64, UInt128Operation::Sub) => {
                FullArithmeticResult::Sub64(checked_u128_sub_64(left, right))
            }
            (WordLayout::Word64, UInt128Operation::Mul) => {
                FullArithmeticResult::Mul64(checked_u128_mul_64(left, right))
            }
        }
    }
}

impl FullArithmeticResult {
    /// Project the existing checked selection AFTER the complete result exists.
    pub fn checked(self) -> Result<u128, u8> {
        match self {
            Self::Add32(result) => result.checked,
            Self::Sub32(result) => result.checked,
            Self::Mul32(result) => result.checked,
            Self::Add64(result) => result.checked,
            Self::Sub64(result) => result.checked,
            Self::Mul64(result) => result.checked,
        }
    }
}
