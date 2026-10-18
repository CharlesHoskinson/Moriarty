# Independent normal typed-word expectations

This data is derived only from original UIREQ030, its mathematical equations,
frozen declaration-only R196 contracts and frozen normal inputs. The data author
has not read the candidate body, materializer, current producer outputs, new author
test source or author goldens. No subject implementation is imported. Population
uses Python standard-library arbitrary-precision integer arithmetic only; the
Rust consumer is source-only and has not been compiled or run.

INPUT-SCHEMA-FREEZE.json predates data population. Its shared public declaration
SHA256 is 61b764e2aff77c751975372e2a2d94b5c10c42261c412fda02c0a95746ebaa61.
The original D26–D28 input strings are preserved byte-for-value in INPUTS.json.
Two supplementary checked cases and two standalone word-multiply-accumulate input
cases are explicitly listed there before population. All checked cases are normal
results in [0,2^128); no overflow/underflow outcome or hostile test is designed here.

## Mathematical domain and layouts

For b=32 or64 let B=2^b and N=128/b. Each input word is in [0,B). Its little-endian
word i is floor(value/B^i) mod B. Four32-bit words and two64-bit words both preserve
all128 input bits. Output words are canonical decimal strings in JSON, bit carry
and borrow values are booleans, and width/index metadata use small JSON integers.
No mathematical word, u128 or full-product scalar goes through floating point.

The independent value-schema is QA data, not an observed producer serializer or
an invented production API. FullProduct's real callable fields are low/high;
`little_endian_256` and `mathematical_product` are additional QA accounting fields,
not claimed members of the Rust result. The Rust consumer asserts the actual
low/high arrays and their concatenation against complete independent literals.

## Original normal results

D26: (2^32−3)+8 =2^32+5 =4294967301.
D27: (2^64+9)−7 =2^64+2 =18446744073709551618.
D28: (2^48+1)*3 =3*2^48+3 =844424930131971.

For each original input pair both layouts predict the same exact checked value;
the word arrays differ only by radix. FullAdd.low and FullSub.low each retain N
limbs plus the final CarryBit or BorrowBit. Every original component call records
its actual word input, incoming bit, named low and outgoing bit/high value. No
uncharged semantic step, source evaluation or transition is inferred.

## Full add and borrow, without replicating the producer

The mathematical result R is computed once with arbitrary-precision addition or
subtraction. The full low vector is R's N little-endian base-B digits. Each checked
result is Ok(R) because these supplied pairs yield 0<=R<2^128.

To obtain each addition component i independently of a ripple implementation,
use prefix residues: carryIn_i = floor(((a mod B^i)+(bv mod B^i))/B^i), and
carryOut_i = floor(((a mod B^(i+1))+(bv mod B^(i+1)))/B^(i+1)). The low word is
floor(R/B^i) mod B. At i=0 residues mod1 are0, so the incoming carry is false.
These values satisfy a_i+b_i+carryIn_i=low_i+B*carryOut_i exactly.

Subtraction borrows are independent prefix comparisons: borrowIn_i is
(a mod B^i)<(bv mod B^i); borrowOut_i is
(a mod B^(i+1))<(bv mod B^(i+1)). The low word again comes from R's base-B digits.
Thus a_i−b_i−borrowIn_i=low_i−B*borrowOut_i exactly, including ordinary cross-word
borrow even though the entire UInt128 result remains normal/nonnegative.

Supplement N02: (2^64+1)−2 =2^64−1 =18446744073709551615. This gives a normal
checked subtraction with an ordinary borrow through word boundaries in both
layouts. This is not a whole-UInt128 underflow case. Final borrow and final add
carry are false for all normal corpus pairs; intermediate component bits are
retained individually, including true bits. No bit is represented as a whole word.

## Multiplication and complete256-bit retention

Compute P=a*bv once using Python integer mathematics. For both layouts low is the
first N base-B digits; high is the next N digits. The exact2N-word little-endian256
vector is low followed by high. All padding words are retained even when zero.
Reconstruction is sum(low[i]*2^(b*i))+sum(high[i]*2^(128+b*i))=P.

Each of the complete N*N ordered word pairs receives a separate mulWide expected
output from divmod(a_i*b_j,B): low=remainder; high=quotient. The full matrix is
retained in a-index outer / bv-index inner order, including every zero-word pair.
This does not reproduce, inspect or prescribe a production multiplication loop.
No assumption is made about which component-call schedule its wrapper chooses.
The source-only consumer calls each declared component directly on these inputs,
then calls each checked wrapper once to compare its complete returned result.

Supplement N01: (2^64−1)^2 =2^128−2^65+1
=340282366920938463426481119284349108225, strictly below2^128.
Its low128 words have nonzero high word components; the full high128 array remains
zero and is still retained. In the64-bit layout the named word-wide product has
low1 and high18446744073709551614. This is normal in-range multiplication, not a
rejection fixture. A nonzero high128 product with checked error selection is
outside this ordinary normal corpus and remains with the designated owner.

The standalone normal MAC fixture in each width has a=bv=B−1, addend2 and whole-word
carryIn3. Its exact integer total is B^2−2B+6. Thus low6 and carryOut B−2. Both
incoming3 and outgoing B−2 are words, not bits. The consumer constructs explicit
MulCarry32/64 and compares the named MulAccumulate result, preserving this type
distinction. These inputs are within the declared component domains; no new
exception or security policy is introduced.

## Complete source/data coverage and boundaries

EXPECTED-NORMAL.json includes10 checked case/layout records (six original plus
four supplementary),58 per-word component outputs and2 standalone MAC outputs.
FIELD-COVERAGE.json accounts for all738 nodes /573 scalar nodes, including every
ordered input/output limb, padding word, bit, result tag, mathematical product,
metadata field and null-free shape. It covers authored expectations, not actual
subject fields or native public inputs.

`word_arithmetic_independent_normal_tests.rs` is a direct literal consumer of only
the frozen public names. It contains12 ordinary normal tests, calls all declared
component families/checked entrypoints for both widths, and compares each named
field and complete array without producer-derived expected values. It proposes a
separate cfg(test) child module inside word_arithmetic; Root owns registration.
There is no reference producer lookup, serializer, SDK adapter, materializer or
runtime execution in this lane. Source assembly is not Rust syntax/type evidence.

The data-only checks reconstruct words/bytes and verify component equations,
canonical types, fixed input preservation, literal normal identities and full
field coverage. They do not execute any Rust test or implementation. A passing
normal corpus is neither full component coverage, circuit equivalence, proof,
original UIREQ030 closure nor row/cost/target evidence. No physical layout winner
or native resource decision follows. Root must separately confirm the frozen
expectations precede any newly authorized subject run; the data author performed
none and has not seen subject outputs.
