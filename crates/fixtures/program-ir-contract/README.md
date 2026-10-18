# Full Program IR contract fixtures

The `.mori` and expected JSON files are exact byte copies from the independent
full-Program oracle packet frozen by `FREEZE.sha256` (SHA-256
`c1b5027d0cab397134337d4346e2c956fffde532d4a9e56843b725df212cba99`).
The copied `DERIVATION.md` and `INVENTORY.json` retain its public-data premises,
post-adoption provenance and limits. This README is outside that freeze.

`program_ir_contract.rs` compares real `mori_core::parse` Program serialization
with the complete expected JSON values. Arrays retain order; object member
order is not a byte serialization contract. Counter includes all eleven nodes.
The empty-action source retains its uninvoked declaration and its global
literal node. Expected files must not be regenerated from parser output.

These two finite representation comparisons establish no full frontend,
compiler, native, target, financial or parent-roadmap acceptance.
