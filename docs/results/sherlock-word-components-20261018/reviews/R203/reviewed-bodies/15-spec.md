### Requirement: UIREQ-030 Typed arithmetic component API
WHEN arithmetic lowering selects a word layout, it SHALL expose named full-result components with the exact checked UInt128 semantic wrapper.

#### Scenario: Carry, borrow and full product
- **WHEN** b is 32 or 64 and checkedU128Add/Sub/Mul use little-endian limbs.
- **THEN** addCarry_b(a,bv,carryIn) returns named {low,carryOut}, subBorrow_b(a,bv,borrowIn) returns {low,borrowOut}, and mulWide_b(a,bv) returns {low,high}; carry/borrow bits and multiply-accumulate whole-word carries have distinct types. The wrappers retain K mathematical-result/error10 meaning, including the full up-to-256-bit product before checked UInt128 selection. Independently specified normal results are (2^32−3)+8 = 2^32+5, (2^64+9)−7 = 2^64+2, and (2^48+1)·3 = 3·2^48+3; no layout or row-cost winner is selected.

