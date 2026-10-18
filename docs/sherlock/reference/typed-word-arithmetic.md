# Typed word arithmetic

Public Rust path: `mori_core::sherlock_word_arithmetic`. Core's library root now
exports the R196 host leaf, matching the R206 source whose normal library and
external public consumer compiled successfully. R214 also compiled the
[canonical test wrapper](../../../crates/mori-core/tests/sherlock_word_arithmetic.rs)
after its fixture include path changed. These are compilation results; the
canonical test binary was neither listed nor executed. The
[how-to](../how-to/use-typed-word-arithmetic.md) contains one usage example whose
status is source-inspected only.

## Layout and component meaning

Layout identity: `sherlock-typed-words-le/1`. For word width `b` equal to 32 or 64,
let `B = 2^b` and `N = 128/b`. A UInt128 uses four u32 or two u64 limbs.
Limb zero is least significant: `join(words) = sum(words[i] * B^i)`.
A full product's `low[i]` weighs `B^i`; `high[i]` weighs `2^128 * B^i`.

| Component | Exact mathematical equation |
| --- | --- |
| `add_carry_32/64` | `a + bv + carry_in = low + B * carry_out` |
| `sub_borrow_32/64` | `a - bv - borrow_in = low - B * borrow_out` |
| `mul_wide_32/64` | `a * bv = low + B * high` |
| `mul_accumulate_32/64` | `a * bv + addend + carry_in = low + B * carry_out` |

`CarryBit(bool)` and `BorrowBit(bool)` are distinct bit types. Multiply-accumulate
uses a whole word through `MulCarry32(u32)` or `MulCarry64(u64)`. For example, a
whole-word incoming carry of 3 is representable by `MulCarry64(3)`; it is not a
bit. Component outputs have named fields, so high/low ordering is explicit.

## Types

All 24 public types derive `Clone`, `Copy`, `Debug`, `PartialEq` and `Eq`.
These are the frozen declarations; arrays have fixed lengths.

```rust
pub struct CarryBit(pub bool);
pub struct BorrowBit(pub bool);
pub struct MulCarry32(pub u32);
pub struct AddCarry32 { pub low: u32, pub carry_out: CarryBit }
pub struct SubBorrow32 { pub low: u32, pub borrow_out: BorrowBit }
pub struct MulWide32 { pub low: u32, pub high: u32 }
pub struct MulAccumulate32 { pub low: u32, pub carry_out: MulCarry32 }
pub struct FullAdd32 { pub low: [u32; 4], pub carry_out: CarryBit }
pub struct FullSub32 { pub low: [u32; 4], pub borrow_out: BorrowBit }
pub struct FullProduct32 { pub low: [u32; 4], pub high: [u32; 4] }
pub struct CheckedUInt128Add32 { pub full: FullAdd32, pub checked: Result<u128, u8> }
pub struct CheckedUInt128Sub32 { pub full: FullSub32, pub checked: Result<u128, u8> }
pub struct CheckedUInt128Mul32 { pub full: FullProduct32, pub checked: Result<u128, u8> }
pub struct MulCarry64(pub u64);
pub struct AddCarry64 { pub low: u64, pub carry_out: CarryBit }
pub struct SubBorrow64 { pub low: u64, pub borrow_out: BorrowBit }
pub struct MulWide64 { pub low: u64, pub high: u64 }
pub struct MulAccumulate64 { pub low: u64, pub carry_out: MulCarry64 }
pub struct FullAdd64 { pub low: [u64; 2], pub carry_out: CarryBit }
pub struct FullSub64 { pub low: [u64; 2], pub borrow_out: BorrowBit }
pub struct FullProduct64 { pub low: [u64; 2], pub high: [u64; 2] }
pub struct CheckedUInt128Add64 { pub full: FullAdd64, pub checked: Result<u128, u8> }
pub struct CheckedUInt128Sub64 { pub full: FullSub64, pub checked: Result<u128, u8> }
pub struct CheckedUInt128Mul64 { pub full: FullProduct64, pub checked: Result<u128, u8> }
```

## Functions

The 14 public functions accept typed words, bits or the original UInt128 pair.
No caller resource extent or default is part of this arithmetic API.

```rust
pub fn add_carry_32(a: u32, bv: u32, carry_in: CarryBit) -> AddCarry32;
pub fn sub_borrow_32(a: u32, bv: u32, borrow_in: BorrowBit) -> SubBorrow32;
pub fn mul_wide_32(a: u32, bv: u32) -> MulWide32;
pub fn mul_accumulate_32(a: u32, bv: u32, addend: u32, carry_in: MulCarry32) -> MulAccumulate32;
pub fn checked_u128_add_32(a: u128, bv: u128) -> CheckedUInt128Add32;
pub fn checked_u128_sub_32(a: u128, bv: u128) -> CheckedUInt128Sub32;
pub fn checked_u128_mul_32(a: u128, bv: u128) -> CheckedUInt128Mul32;
pub fn add_carry_64(a: u64, bv: u64, carry_in: CarryBit) -> AddCarry64;
pub fn sub_borrow_64(a: u64, bv: u64, borrow_in: BorrowBit) -> SubBorrow64;
pub fn mul_wide_64(a: u64, bv: u64) -> MulWide64;
pub fn mul_accumulate_64(a: u64, bv: u64, addend: u64, carry_in: MulCarry64) -> MulAccumulate64;
pub fn checked_u128_add_64(a: u128, bv: u128) -> CheckedUInt128Add64;
pub fn checked_u128_sub_64(a: u128, bv: u128) -> CheckedUInt128Sub64;
pub fn checked_u128_mul_64(a: u128, bv: u128) -> CheckedUInt128Mul64;
```

## Full intermediate and checked result

Each checked entrypoint returns both `full` and `checked`. Addition retains
`join(full.low) + 2^128 * full.carry_out`; subtraction retains the signed integer
`join(full.low) - 2^128 * full.borrow_out`. Multiplication retains every one of
its 256 product bits as `join(full.low) + 2^128 * join(full.high)` before checked
selection. A word component's carry can be true while the complete UInt128
calculation still fits.

`checked: Result<u128, u8>` is `Ok(value)` when the mathematical result lies in
`[0, 2^128)`. The inspected source selects legacy `Err(10)` for final addition
carry, final subtraction borrow, or a nonzero high128 product. The retained full
intermediate remains in the return object. Those whole-UInt128 error branches
were not executed by the normal cohort described below; error10 selection is a
source observation here.

The independently specified R197 original examples give these successful values
in both layouts:

| Case | Inputs and operation | Exact checked value |
| --- | --- | --- |
| D26 | `(2^32 - 3) + 8` | `4294967301` |
| D27 | `(2^64 + 9) - 7` | `18446744073709551618` |
| D28 | `(2^48 + 1) * 3` | `844424930131971` |

For D26, the first 32-bit component returns low 5 and carry true; the full
32-bit layout has low `[5, 1, 0, 0]` and final carry false. For D28, the full
32-bit product has low `[3, 196608, 0, 0]` and high `[0, 0, 0, 0]`. The 64-bit
layout has low `[844424930131971, 0]` and high `[0, 0]`. Both retain the padding
words and select the same exact checked value.

## Observed scope

| Retained receipt | Actual observation |
| --- | --- |
| R201 external harness, attempt 01 | Compilation, exact membership listing and execution passed: 17 normal tests, comprising 5 author tests and 12 independent tests |
| R206 public integration candidate, attempt 01 | Normal Core rlib compile passed in 4.0259 seconds; external public API test consumer compile passed in 0.1656 seconds |
| R206 generated external test binary | Its 12 test bodies were compiled through the public alias; the binary was neither listed nor executed |
| R214 canonical test wrapper | Compilation against the R206 normal Core library passed in 0.1789 seconds, with all 404 input pins unchanged; no test listing or execution |

The R206 and R214 compilations did not rerun the R201 cohort or establish a
canonical 12-test execution. The canonical wrapper differs from the R206 wrapper
only in its fixture include path; its historical compile-only provenance comment
is retained. These receipts describe their frozen source and consumers.
They supply no arithmetic-lowering integration, circuit constraints, row-cost
winner, native proof or ledger result, and do not close original UIREQ-030.
See [profiles and results](profiles-results.md) for the broader evidence boundaries.
