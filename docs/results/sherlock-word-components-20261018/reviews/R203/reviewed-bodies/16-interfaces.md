## Arithmetic component boundary

```text
Word_b = {n:Nat | n < 2^b}; Bit = {0,1}; b in {32,64}
addCarry_b(a:Word_b,bv:Word_b,carryIn:Bit) -> {low:Word_b,carryOut:Bit}
subBorrow_b(a:Word_b,bv:Word_b,borrowIn:Bit) -> {low:Word_b,borrowOut:Bit}
mulWide_b(a:Word_b,bv:Word_b) -> {low:Word_b,high:Word_b}
checkedU128Add(a:UInt128,b:UInt128) -> Value(UInt128) | Error(10)
checkedU128Sub(a:UInt128,b:UInt128) -> Value(UInt128) | Error(10)
checkedU128Mul(a:UInt128,b:UInt128) -> Value(UInt128) | Error(10)
```

The defining mathematical equations are `a+bv+carryIn = low + 2^b*carryOut`, `a-bv-borrowIn = low - 2^b*borrowOut`, and `a*bv = low + 2^b*high`. Named fields avoid high/low tuple-order ambiguity. UInt128 limbs are little-endian: `value = sum(limb[i]*2^(b*i))`. Checked wrappers select the exact mathematical result iff it lies in `[0,2^128)`; subtraction underflow/addition overflow/multiplication high128 nonzero yield legacy error10. A multiply-accumulate word carry is a distinct `Word_b` output, never a `Bit` carry. Full multiplication retains all 256 product bits before outcome selection. Division and financial rounding are additional semantic APIs, not implied by these three wrappers.

The byte-based public Clean representation is the comparison baseline. Four public U32 values and two public U64 values both occupy sixteen byte-valued fields; two/four packed-field layouts are new alternatives. This is an inventory observation, not a row estimate. No limb layout or primitive implementation wins without complete-path measurements and the separately owned component review.
