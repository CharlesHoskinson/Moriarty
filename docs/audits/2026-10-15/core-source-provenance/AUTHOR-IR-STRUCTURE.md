# Actual IR structural correspondence test source

Status: source only, not built or executed. Added exactly `source_step_fields_match_independent_ir_oracle` to the existing `crates/mori-ledger/tests/mapped_role_static.rs`. The only existing change in this file remains the authorized mapped relation `/2` format literal. No production source, other test file, Cargo dependency or oracle was changed for this task.

The test embeds the exact402-byte immutable five-step source, SHA-256 `f8e9f77d16743637a1a08d3e23a821673180d2de4ad2f4f2372fd714ab8f455d`. Literal expectations were extracted from frozen `ORACLE-DESIGN.json`, SHA-256 `33e37dc74dd01bfec329b38eee3adeea462cdb4514e24f0116b75a3081cc1c88`, not compiler output. There is no runtime-directory dependency.

Existing helper inspection: `bindings` parses Core and constructs the public ABI, using existing literal ECDSA public-point decoding, public address conversion, fixed token bytes, state-name keys and public serialization/hashes. Its points/token/domain match the existing public financial-duty binding fixture. `mode` selects only unsigned-mapped-roles. `limits` retains its existing explicit bounded static values. The test invokes only public `static_compile::compile`, the existing manifest projection and `codec::decode_artifact::<midnight_zkir_v3::IrSource>` on the emitted action bytes. The latter performs tagged format decoding/re-encoding. No helper call to relation checking, SDK degree/model queries, instruction evaluation, VM, proving, key generation or ledger submission is present in this test. This is a source-call inventory; exact runtime command classification remains Opus-owned and pending.

Assertions cover actual decoded IR, independently of the map's claimed coordinates: all58 kth-Impact instruction indices and widths; frozen origins matched to their group rows;52 actual claim guard operand identifiers explicitly present in the oracle; all13 listed replacement operand identifiers at their actual decoded `(group, offset)`; full exact five-entry statement metadata;155 instructions, one typed action input, final empty Output, and335 operation fields. Six prelude/state-read guards have no literal guard row in the frozen JSON and are not guessed. No instruction is evaluated, and no runtime-value/native semantics claim is made.

Source-only rustfmt and `git diff --check` passed. No build/test/product/native command, commit or staging occurred. The direct ZKIR type is available through the existing mori-ledger dependency; no new dependency/helper API is added.

Candidate exact filter for separate Opus classification, **not executed**:

```bash
cargo +1.99.0 --offline --locked test -p mori-ledger --test mapped_role_static -j2 source_step_fields_match_independent_ir_oracle -- --exact --test-threads=1
```

Final test file: 27092 bytes; SHA-256 `002e6f0171d781121776a4ca09132698634202cc0a64bdc2c9f0b9739c8d0e41`.
