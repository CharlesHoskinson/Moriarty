# Prepare type tables for an owned generic program

Use the public Rust preparation API when you already have a typed `ExtProgram`
and an explicit `LayoutSchemaC`. It retains your complete source alongside the
unified names, declaration-driven tables and T3 records. It does not parse a new
generic language, evaluate constructor operands or select a CLI mode.

1. Construct your source with the public carriers in
   `shared::owner_ext_source_program`, `owner_ext_t3_uses` and
   `owner_ext_type_builder`. Retain source declaration/function/field order.
2. Supply your explicit layout/bounds/name capacities and domain. The observed
   positive fixtures use `ExtDomain::GENERIC_ABSENT` and HostOnly layout. Their
   [exact fixture](../../../crates/mori-core/tests/fixtures/generic-adt-one-builder/FIXTURE-LAYOUT-AND-DOMAIN.json)
   is a bounded example, not a default configuration.
3. Pass the whole owned program to the existing API. Handle its error without
   assuming that ExtTypeBuildError implements Debug:

```rust
use mori_core::sherlock::helper_machine::shared::{
    prepare_ext_program_types, LayoutSchemaC, PreparedExtProgramSource,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::{
    ExtDomain, ExtProgram,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::ExtTypeBuildError;

fn prepare(program: ExtProgram, layout: &LayoutSchemaC)
    -> Result<PreparedExtProgramSource, ExtTypeBuildError>
{
    prepare_ext_program_types(program, layout, ExtDomain::GENERIC_ABSENT, None)
}
```

4. Inspect `prepared.program`, `domain`, `interned_names`, `decl_name_ids`,
   `tables` and `t3_records` on the returned value. Keep full vectors and source
   fields when comparing results; names or row counts alone cannot establish
   instantiation and source-retention behavior.

The [two actual preparation checks](../../results/sherlock-generic-one-builder-preparation-20261018/README.md)
compare complete independent values for T3-only Chain/Tag specializations and
signature-only Ribbon reuse. They do not execute Schedule, generic constructors
or matching. Their complete assertions are public source; no fabricated prepared
JSON printout is shown here. This guide's small wrapper is an API example, not a
separately captured compiler result.

For runtime work, retain the prepared source and follow the separate admission,
C/P lowering, ConstructorFields and ordered-match requirements. Preparation does
not close UIREQ034 or provide native/target evidence. See the
[API reference](../reference/generic-program-preparation-api.md).
