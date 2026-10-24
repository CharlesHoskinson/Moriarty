# Typed word arithmetic

Public Rust paths are `mori_core::sherlock_word_arithmetic` for typed components,
`mori_core::sherlock_word_lowering` for explicit Word32/Word64 selection, and
`mori_core::sherlock_compile_artifact::draft_execution` for the selected host run.
The installed Scalar lowering uses the existing components on actual evaluated
UInt128 operands. [Root adoption](../../results/sherlock-selected-word-arithmetic-20261018/ROOT-WORD-ADOPTION.json)
closes the whole original UIREQ-030 host arithmetic clause after source/result
review and the nine observed normal/error operations. It establishes no circuit,
native-proof, charged-work or target result, and selects no layout/cost winner.

Earlier R201/R206/R214 observations below retain their exact artifacts and scope.
The [how-to](../how-to/use-typed-word-arithmetic.md) includes a standalone product
snippet with no separate execution receipt; the later selected-path tests are
different executed artifacts.

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

## Selected host lowering

`mori_core::sherlock_word_lowering` exports `WordLayout::{Word32, Word64}`,
`UInt128Operation::{Add, Sub, Mul}`, `ArithmeticLowering` and
`FullArithmeticResult::{Add32, Sub32, Mul32, Add64, Sub64, Mul64}`.
`ArithmeticLowering::new(layout, operation)` returns a value whose **method**
`execute(self, left: u128, right: u128)` returns the complete
`FullArithmeticResult`. `component_name(self)` identifies its checked component;
`FullArithmeticResult::checked(self)` projects `Result<u128, u8>` only after the
full intermediate has been produced. There is no associated free `execute`
function or implicit choice of word width.

The public dedicated-source function has this signature:

```rust
pub fn run_draft_host_with_word_layout(
    source: &str,
    config_source: &str,
    target_source: &str,
    word_layout: WordLayout,
    sink: &mut dyn FnMut(&StepEventC),
) -> Result<WordLoweredDraftHostRun, CompileFailure>;
```

`WordLoweredDraftHostRun` has `word_layout: WordLayout` and `run: DraftHostRun`.
Read `run.completed` for the actual summary/event count and `run.observe()` for
the borrowed post-run observation. `run.preparation` remains its original
unstepped genesis report; its wire widths do not become circuit/field layout
claims. An `Ok` run can contain a real `Rejected { code: 10 }` or exhausted
outcome; it does not assert successful application return.

The selected path is `run_draft_host_with_word_layout` →
`host_owner_engine_with_word_layout` → `WordFamilyPlans` → Scalar Add/Sub/Mul →
`ArithmeticLowering::new(...).execute(...)`. Other family plans delegate to the
existing implementation. Existing `run_draft_host` and `host_owner_engine`
retain their default path. Configured parser/type/copy/runtime extents, operand
order and sink/failure contracts are shared; selecting a host word layout adds
no public developer permission, new runtime budget or native exporter.

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
intermediate remains in the return object. The later selected error cohort
executed addition overflow, subtraction underflow and a nonzero-high128 product
through both Word32 and Word64 public paths, observing semantic error10. The
historical normal-only cohorts did not execute those branches.

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
Those historical receipts alone established no selected lowering or whole030
closure. The later [selected-word archive](../../results/sherlock-selected-word-arithmetic-20261018/README.md)
adds the following actual observations:

| Adopted batch | Actual observation |
| --- | --- |
| Five normal operations | Changed combined Core compilation, normal-consumer compilation and three exact tests passed; all 500 input pins unchanged. Full component checks and six public normal runs cover the original D26/D27/D28 equations in both layouts. |
| Four error10 operations | New consumer compilation and three exact tests passed using the same Core library; all 505 input pins unchanged. Full intermediates and six public error runs cover max+1, 0−1 and max×2. No Core rebuild or positive replay. |
| Whole original030 adoption | Source inspection and actual scoped results accepted; six prior closures retained after compatibility review. [Currentness readback](../../results/sherlock-selected-word-arithmetic-20261018/ROOT-CURRENTNESS-READBACK.json) reports 13 adopted originals with no findings. |

The normal and error cohorts comprise six tests, not an exhaustive input-space
execution. General implementation meaning was assessed by source review; these
finite runs supply scoped observations. The source reviewer authored the new
error tests and relied on independent reviewer29 for their review; independent reviewer29's
production constructor was separately reviewed by a3ce. Raw process137/terminal
SUCCESS, later clean confirmation and the qualified single-JSON-fence observer
failure remain in the archive. Circuit constraints, row costs, native proofs,
charged-work equivalence and ledger/target acceptance remain unestablished.
See [profiles and results](profiles-results.md) for the broader evidence boundaries.
