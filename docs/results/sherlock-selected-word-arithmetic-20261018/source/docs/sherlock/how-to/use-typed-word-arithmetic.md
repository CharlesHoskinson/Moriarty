# Use typed word arithmetic

Use a checked UInt128 entrypoint when you need the exact checked value together
with its full limb result. The public path is
`mori_core::sherlock_word_arithmetic`; its installed source matches the R206
compiled public export. The [reference](../reference/typed-word-arithmetic.md)
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
arithmetic. The inspected source represents an out-of-range whole result with
`Err(10)` while retaining the full intermediate; the normal receipts do not
exercise those error branches. Do not discard `high` before checking a product.

For addition or subtraction, use `checked_u128_add_32/64` or
`checked_u128_sub_32/64` and inspect the named final `carry_out` or `borrow_out`
in `full`. When composing individual words, pass `CarryBit` for addition,
`BorrowBit` for subtraction, and the distinct `MulCarry32/64` whole-word type
for multiply-accumulate.

Both layouts implement the same checked mathematical meaning. The available
host results do not select a circuit layout or establish that compiler lowering
uses these components. The [reference's observed scope](../reference/typed-word-arithmetic.md#observed-scope)
keeps the public compile result, executed normal tests and remaining UIREQ-030
obligations separate.

## Select the arithmetic implementation for a real draft run

The source-only selected integration adds this Rust call:

```rust,ignore
use mori_core::sherlock_compile_artifact::draft_execution::run_draft_host_with_word_layout;
use mori_core::sherlock_word_lowering::WordLayout;

let selected = run_draft_host_with_word_layout(
    source, config_json, target_json, WordLayout::Word32, &mut |_| {},
)?;
assert_eq!(selected.word_layout, WordLayout::Word32);
let actual_post_run = selected.run.observe()?;
```

Use `Word64` to select the64bit components. Both choices use the same actual
source preparation/owner run and semantic error mapping. The selected family
lowers Add/Sub/Mul through the existing typed wrappers; unrelated operations
retain the default family implementation. An Ok run wrapper can contain a real
Rejected or Exhausted outcome, so inspect the actual completed summary.

This snippet was source-inspected only and has no compilation/execution receipt.
The actual returned owner constructor is composed in the new source revision;
the path still awaits fresh review and Root execution selection. Existing configuration, resource qualifications, caller sink
retention premise and all separate constraint/native obligations remain.
