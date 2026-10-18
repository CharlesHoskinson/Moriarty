# Inspect a canonical signature through the real owner

Use the tested public path to follow a type identity from preparation to runtime. The four builtin cases and separate nominal-storage case passed in the [recorded QA](../../results/sherlock-canonical-runtime-types-20261018/ACTUAL-TEN-TESTS-SCOPE.json).

1. Open the [ordinary consumer](../../results/sherlock-canonical-runtime-types-20261018/source/candidate/crates/mori-core/tests/support/sherlock_resolved_signature_positive.rs). Its three adjacent independent fixtures define builtin identities, runtime expectations and nominal storage checks. The builtin inputs use fuel 64 and stack 8.
2. Start at `host_owner_engine().prepare_core(...)`. Read each function and parameter through the returned `ProgramView`; resolve the name, body and argument from those rows.
3. Follow `admit` and `admit_run`, then `complete_observation::{capture,record_step}`. Check `PairTask.sort` against the parameter row, and `Bound.return_type` and `ReturnF.return_type` against the function row. `Scalar.expected` carries that continuation identity.
4. Inspect the bound environment value, final logical result, snapshot continuity and event-delta conservation assertions. These checks bind the type to consumed state throughout the existing engine.
5. Read `nominal_ids_roundtrip_in_all_six_real_storage_roles_only` separately. It extracts 18 stored cells for IDs 4, 5 and 17 without submitting those IDs to admission or runtime.

For receipts, begin with the [actual result summary](../../results/sherlock-canonical-runtime-types-20261018/ACTUAL-TEN-TESTS-SCOPE.json). Test09 passed once but the capture helper rejected its multiline result; Root accepted those same bytes offline after review. Only the four remaining tests ran afterwards. The archive retains every original failure and stream.

The [reference](../reference/canonical-runtime-types.md) lists the interfaces. Printed work counts are observations; absolute independent B clocks remain unbound. This check does not establish nominal runtime or close original UIREQ034. The [Root adoption record](../../results/sherlock-canonical-runtime-types-20261018/ROOT-CANONICAL-TYPEID-COMPATIBILITY-ADOPTION.json) accompanies installation.
