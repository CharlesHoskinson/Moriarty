# Use typed word arithmetic

Use a checked UInt128 entrypoint when you need the exact checked value together
with its full limb result. The public path is
`mori_core::sherlock_word_arithmetic`. The installed selected lowering also uses
these components through `mori_core::sherlock_word_lowering`. The [reference](../reference/typed-word-arithmetic.md)
lists both layouts and every public type/function.

## Read a full product

In a Rust application with a Core library exposing that public alias, call the
32-bit wrapper and retain the returned object. This complete example uses the
R197 D28 normal input and independently specified full arrays:

```rust
use mori_core::sherlock_word_arithmetic::checked_u128_mul_32;

fn main() {
    let result = checked_u128_mul_32(281_474_976_710_657u128, 3u128);
    assert_eq!(result.full.low, [3u32, 196_608u32, 0u32, 0u32]);
    assert_eq!(result.full.high, [0u32, 0u32, 0u32, 0u32]);
    assert_eq!(result.checked, Ok(844_424_930_131_971u128));
}
```

This snippet is source-inspected only: it has no separate compilation or
execution receipt. R206's separately compiled public consumer, R214's compiled
canonical wrapper and R201's executed external harness are different artifacts.
The canonical wrapper's 12 test bodies have no canonical execution receipt.

Read each array from least to most significant word. `low` contains the first
128 product bits and `high` the next 128. Here `3 + 196608 * 2^32` is
`844424930131971`, and every high128 word is retained as zero. Using
`checked_u128_mul_64` for the same inputs yields low `[844424930131971, 0]`,
high `[0, 0]` and the same `checked` value.

## Choose the result you need

Use `checked` for the exact UInt128 value and keep `full` when inspecting limb
arithmetic. The API represents an out-of-range whole result with
`Err(10)` while retaining the full intermediate. The adopted error cohort
observed overflow, underflow and high128-product rejection under both selected
layouts; the historical normal-only receipts remain separate. Do not discard `high` before checking a product.

For addition or subtraction, use `checked_u128_add_32/64` or
`checked_u128_sub_32/64` and inspect the named final `carry_out` or `borrow_out`
in `full`. When composing individual words, pass `CarryBit` for addition,
`BorrowBit` for subtraction, and the distinct `MulCarry32/64` whole-word type
for multiply-accumulate.

## Select lowering for a dedicated source run

Choose `WordLayout::Word32` or `WordLayout::Word64` explicitly. For already
evaluated operands, create `ArithmeticLowering::new(layout, operation)` and call
its `execute(left, right)` method. Keep the returned `FullArithmeticResult` when
inspecting full intermediates; call its `checked()` method for UInt128 selection.
`execute` is an instance method, not an associated free function.

For Moriarty source, call
`mori_core::sherlock_compile_artifact::draft_execution::run_draft_host_with_word_layout`
with source text, the existing execution-config JSON, target JSON, the selected
layout and a `&mut dyn FnMut(&StepEventC)` sink. It returns
`Result<WordLoweredDraftHostRun, CompileFailure>`. The wrapper records
`word_layout`; its `run` contains the preparation and actual completed run.
Inspect `run.completed.summary.outcome` and `run.observe()` for the real result.
The preparation report remains genesis-only. A successful Rust result wrapper
can still report semantic error10 or exhaustion, so do not treat `Ok` as Halted.
The forwarded-event count is u128. Existing run/config/bounds contracts remain
applicable and the default `run_draft_host` path is unchanged.

The [adopted selected-path evidence](../../results/sherlock-selected-word-arithmetic-20261018/ROOT-WORD-ADOPTION.json)
records six normal and six error public runs, plus full component assertions,
without rerunning passed prefixes. The standalone snippet above still has no
separate execution receipt. Whole original UIREQ-030 is now closed for its host
arithmetic/lowering scope; this does not choose a circuit layout, establish row
costs or prove native/target acceptance. The [reference's observed scope](../reference/typed-word-arithmetic.md#observed-scope)
keeps historical compile-only artifacts, actual tests and that adoption distinct.
