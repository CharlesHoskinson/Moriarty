# Independent numeric fixture derivation

Scope: 22 new complete input/expected objects and 42 retained unique cases. Expectations were derived from `spec/successor/repayment-kernel.md` and `spec/successor/funded-source.md` under `experiments/moriarty-language/`. No K, codec, evaluator, financial execution, proof, or audit output was used to derive these expectations. A serialization helper filled unchanged fields and explicitly supplied financial results; it did not execute conversion or allocation rules.

Each new case uses exactly two balance rows, one allowance, one obligation, empty used-ID arrays, and either Transfer or Transfer then Repay. Successful Transfer debits gross cash and allowance, preserves reserve 13, records its ID, and emits the complete action. Two-action success changes work 9/5 to 7/7; Transfer-only changes it to 8/6. Rejections contain only status, code, and actionIndex.

| Case | Independent arithmetic or contract reason |
| --- | --- |
| `numeric-none-exact` | 10*3/10 = 3 exactly; accrual falls by 10. |
| `numeric-floor-fraction` | 4*3 = 12; quotient 1, remainder 2; floor cash 1. |
| `numeric-ceil-fraction` | Same 12/10; ceil cash 2 distinguishes floor. |
| `numeric-none-inexact` | 12 modulo 10 is 2; none rejects. |
| `numeric-floor-dust` | floor(1/10)=0 cannot discharge positive nominal debt. |
| `numeric-ceil-rescues-dust` | ceil(1/10)=1 funds nominal 1. |
| `numeric-nominal-exceeds-cash` | 30*2/10=6 cash; accrual 10 and principal 20 discharged. |
| `numeric-insufficient-converted-funding` | Converted cash 6 exceeds transfer funding 5. |
| `numeric-conversion-product-overflow` | 2*UInt128max overflows before division by 10^18, although the mathematical quotient fits. |
| `numeric-zero-mantissa-before-work-transfer` | Mantissa 0 invalidates the entire input before insufficient work or zero Transfer. |
| `numeric-scale-nineteen` | Scale 19 exceeds admitted maximum 18. |
| `numeric-scale-uint128max` | Canonical UInt128 scale is parsed, then rejected as >18 before exponentiation. |
| `numeric-max-mantissa-exact` | 1*UInt128max fits; exact scale-0 settlement equals max, fully funded without credit or allowance overflow. |
| `numeric-prorata-partial` | floor(7*100/110)=6 principal; remainder nominal 1 pays accrued. |
| `numeric-prorata-full` | 110*100/110=100; accrued 10; obligation retained with Settled status. |
| `numeric-prorata-zero-principal` | 7*0/10=0 principal; all 7 goes to accrued. |
| `numeric-prorata-zero-accrued` | 7*100/100=7 principal, no accrued discharge. |
| `numeric-prorata-product-overflow` | Conversion cash 2 is exact and funded; 2*(max-1) overflows before division by max. |
| `numeric-conversion-before-prorata-overflow` | Conversion 2/10 fails none before the overflowing allocation product can be evaluated. |
| `numeric-transfer-preserves-conversion-prorata` | Transfer alone preserves ProRata P100/A10 and max-mantissa scale18 none metadata; conversion and allocation are not executed. |
| `numeric-transfer-settled-prorata` | Transfer-only preserves settled P0/A0/O0 ProRata; no allocation division occurs. See discrepancy disposition below. |
| `numeric-scale-eighteen-exact` | 10^18*1/10^18=1 exactly at maximum admitted scale; full nominal discharge. |

## Specification discrepancy and boundary limits

The stable-error table includes ProRata `P+A == 0` under INVARIANT without locating that guard precisely. The explicit admission invariant list permits Settled obligations with zero components; the ProRata transition is an allocation helper, and the Transfer contract states that Transfer-only cash movement leaves debt unchanged. Therefore `numeric-transfer-settled-prorata` expects Prepared and preserves the settled obligation. This case does not introduce a new zero-total admission restriction. A Repay against that same settled obligation rejects NOT_OUTSTANDING before allocation, as covered by the retained branch fixture.

No direct ceil-increment-overflow fixture is reachable: scale 0 has zero remainder; with positive scale Q>=10, the quotient plus one remains below UInt128max when product fits. A runtime ProRata zero divisor is similarly unreachable after Outstanding and positive-nominal checks. P+A overflow is an admission invariant failure, not a reachable post-admission allocation boundary. The retained corpus covers outstanding-invariant failure; the new cases focus on the independently reachable numeric branches. Floor/ceil exact-remainder branches are not separately sampled; exact none, fraction floor/ceil, dust, and the scale endpoints are sampled. Finite fixtures do not establish universal correspondence.

## Retained bytes

The final 42 rows are exact raw object slices, in order: all 16 `cases.json` rows, all 16 `branches.json` rows, and only the first 10 `transfer-only.json` rows. The six duplicates at the end of transfer-only are excluded. Input and expected object bytes, key order, and internal whitespace are retained unchanged. Outer array separator indentation is not part of these objects.

| Source | Retained rows | SHA-256 of complete source file |
| --- | --- | --- |
| `cases.json` | 16 | `d46679961af39a10adec03cb86b4481a39fc9393d15ceb48b37b10b8ce28f832` |
| `branches.json` | 16 | `2a9596d52e934533629354a7ecdbd60282f192efcacd3ef9ec7a95ff6d0845dd` |
| `transfer-only.json` | 10 | `2d5120c5eb7ed451d1bd23fd29d4a793673139e2642c0b3777bfea2bd63dfa75` |

Local structural checks: JSON parses, 64 rows, 64 unique IDs, 22 new rows, and every retained full raw case slice appears verbatim. These checks do not run financial semantics.
