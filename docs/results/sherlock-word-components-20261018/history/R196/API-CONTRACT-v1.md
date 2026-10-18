# R196 typed word components: frozen API v1

Status: DECLARATIONS_ONLY / SOURCE_ONLY / UNRUN. This contract precedes implementation and ordinary test source. No independent expected outputs are included or consumed.

Target afterimage: `crates/mori-core/src/sherlock/word_arithmetic.rs`. Its companion normal tests are `word_arithmetic_tests.rs`, included inside the component module with `#[cfg(test)] #[path = "word_arithmetic_tests.rs"] mod tests;`. Root alone selects public module registration and execution. Standard Rust only; no SDK or external integer library.

`API-DECLARATIONS-v1.rs.txt` freezes every public declaration. All data types derive Clone, Copy, Debug, PartialEq, Eq; no other public items or methods are planned. Native u32/u64/u128 and bool establish the complete input domain, with no resource guard or arbitrary cap.

All limb arrays are little-endian. Layout version: `sherlock-typed-words-le/1`. For word width b and N=128/b, low[i] has weight 2^(b*i), high[i] has weight 2^(128+b*i). FullProduct retains the complete mathematical 256-bit product as named low/high arrays before selection. FullAdd denotes low+2^128*carry_out; FullSub denotes low-2^128*borrow_out. Bit newtypes represent one bit; MulCarry32/64 represent whole words and are distinct from both bit types.

`add_carry_b`: a+bv+carry_in = low+2^b*carry_out. `sub_borrow_b`: a-bv-borrow_in = low-2^b*borrow_out. `mul_wide_b`: a*bv=low+2^b*high. `mul_accumulate_b`: a*bv+addend+carry_in=low+2^b*carry_out. The checked entrypoints accept the original a,bv arguments, consume these component functions, retain the full result, and select Ok(exact_u128) iff the mathematical result is in [0,2^128); otherwise Err(10u8). They introduce no other error code, wrapping or saturation semantics.

Original UIREQ030 normal fixture INPUTS (exact decimal strings; shared for both layouts):

| case | entrypoint suffix | a | bv |
|---|---|---|---|
| D26 | add_32 / add_64 | 4294967293 | 8 |
| D27 | sub_32 / sub_64 | 18446744073709551625 | 7 |
| D28 | mul_32 / mul_64 | 281474976710657 | 3 |

The independent oracle may decompose inputs itself and use the component declarations and the checked entrypoints. It owns expected component outputs, full arrays and checked result independently. Populated expectations remain withheld from this author. Ordinary author tests will use separate fixed examples and algebraic assertions; they are not independent oracle evidence.

No A/reference semantics, private byte auxiliary, G4/C4, circuit/native ABI, proof constraints or row cost changes. Host observations cannot close UIREQ030 circuit obligations or the full original requirement. Root will review/audit before public export and selected checks.
