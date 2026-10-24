# Typed word arithmetic

Preparation status: SOURCE_ONLY / UNRUN. This document describes a candidate
host component for original UIREQ030 under adopted revision03. Root must select
and review its exact source before module registration and ordinary execution.

Use `sherlock::word_arithmetic::checked_u128_add_32(a, bv)` or the corresponding
`sub`/`mul` and `_64` functions to preserve the original UInt128 arguments and
obtain both the complete intermediate and the checked value. Every return has
`full` and `checked` fields. `checked` is `Result<u128, u8>`: successful exact
mathematical UInt128 results are `Ok(value)`; addition overflow, subtraction
underflow and a nonzero high128 product produce the existing `Err(10)`.

The component functions are `add_carry_32/64`, `sub_borrow_32/64`,
`mul_wide_32/64` and `mul_accumulate_32/64`. Addition returns named `low` and
`carry_out`; subtraction returns named `low` and `borrow_out`; multiplication
returns named `low` and `high`. `CarryBit(bool)`, `BorrowBit(bool)` and the
whole-word `MulCarry32(u32)`/`MulCarry64(u64)` are different Rust types.
Multiply-accumulate computes `a*bv+addend+carry_in` exactly and returns one
word plus a whole-word carry. It consumes `mul_wide` and `add_carry`.

The layout is `sherlock-typed-words-le/1`. Four u32 limbs or two u64 limbs
represent a UInt128, with limb0 least significant. Full addition represents
`sum(full.low[i]*2^(b*i)) + 2^128*full.carry_out`; full subtraction represents
`sum(full.low[i]*2^(b*i)) - 2^128*full.borrow_out`. Full multiplication retains
named `low` and `high` arrays: low[i] weighs `2^(b*i)` and high[i] weighs
`2^(128+b*i)`. The complete product exists before checked selection; no u128
product truncation or saturating/wrapping source operator is used.

For example, a caller can inspect the full product through either layout:

```rust
use mori_core::sherlock::word_arithmetic::checked_u128_mul_64;
let result = checked_u128_mul_64(17, 29);
assert_eq!(result.checked, Ok(493));
assert_eq!(result.full.low, [493, 0]);
assert_eq!(result.full.high, [0, 0]);
```

The component uses fixed arrays and widened host integers; it requires no SDK,
external big integer, recursion or input resource cap. Its checked entrypoints
consume the named components. Ordinary author test source and a separately owned
independent oracle are distinct evidence; no test execution is claimed here.

This component does not replace the private byte arithmetic auxiliary, alter the
reference evaluator, add circuit constraints or define a native ABI. It supplies
no circuit-cost or layout winner, proof, formal correspondence or target result.
Host checks alone cannot close the full original UIREQ030 requirement, G4/C4,
Sherlock completion or the native/financial/release exits.
