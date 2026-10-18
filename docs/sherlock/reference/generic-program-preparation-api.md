# Whole-program generic type preparation API

`mori_core::sherlock::helper_machine::shared::prepare_ext_program_types` accepts
a complete owned extended source program and returns its prepared names, type
tables and T3 use records with that original source retained. This is a typed Rust
preparation API; it adds no source-text parser or CLI flag.

```rust
pub fn prepare_ext_program_types(
    program: owner_ext_source_program::ExtProgram,
    layout: &LayoutSchemaC,
    domain: owner_ext_source_program::ExtDomain,
    locator: Option<LocatorC>,
) -> Result<PreparedExtProgramSource, owner_ext_type_builder::ExtTypeBuildError>;

pub struct PreparedExtProgramSource {
    pub program: owner_ext_source_program::ExtProgram,
    pub domain: owner_ext_source_program::ExtDomain,
    pub interned_names: Vec<String>,
    pub decl_name_ids: owner_ext_type_builder::PassBTypeNameIds,
    pub tables: owner_ext_type_builder::BuiltAdtTypeTables,
    pub t3_records: Vec<owner_ext_t3_uses::T3Record>,
}
```

These modules and types are under `shared`; `owner_prepare` remains private and
its entry point/output are publicly re-exported. The caller supplies the existing
layout, every bound/capacity, explicit domain and optional locator. The observed
positive domain is `ExtDomain::GENERIC_ABSENT`: GenericNominalExtensionReserved,
FinanceAbsent, HostOnly layout, no locator. The fixture's arity4 and capacities
are explicit test inputs, not production defaults. Error remains ExtTypeBuildError;
its result need not support a generic Debug-based `expect` call.

## Names and one builder

The global sorted name pool includes declaration type/constructor/field names
and whole-program function/parameter/binder/callee names. Type-parameter labels
and text appearing only in type references do not add names. Declaration NameIds
are bound against this final pool, including the actual owner-verification flag.

One checked builder starts with builtin UInt/Bool/List/Tree IDs0/1/2/3. T1 assigns
all non-parameterized declaration IDs before fields; T2 drains declaration-field
keys in FIFO order. T3 visits parameter types and return types in function table
order first, then entry and function-body uses. Each use resolves against that
same builder and drains newly discovered keys before recording its resolved
constructor ID. Existing keys are reused; the builder finishes once.

`tables` retains all eight fields: `types`, `type_args`, `constructors`,
`ctor_fields`, `decl_param_counts`, `decl_ctor_counts`, `max_arity`, `name_count`.
Type rows retain name/declaration IDs, argument slices and constructor slices;
constructor rows retain owning type, name/source ordinal and field slices;
field rows retain name and type ID. No count-only report replaces these vectors.

Each `T3Record` has `site`, `type_id` and optional `ctor_id`. Sites are ParamType,
ReturnType, Construct, LiteralCtor and CtorPattern. Expression ordinals are
owner-local; value ordinals are literal-local and pattern ordinals arm-local.
They are not ProgramImage SourceOccurrenceIds. This preparation does not adopt
a disputed global pattern/scrutinee traversal policy or execute a match arm.

## Observed scope

The [2026-10-18 result](../../results/sherlock-generic-one-builder-preparation-20261018/README.md)
records normal Core/external consumer compilation and two passing exact tests.
Their full independent comparisons cover T3-only Chain/Tag specialization and
signature-only Ribbon discovery/reuse, all returned names/tables/records and
owned source fields. All427 input pins matched whole-attempt pre/post checks.
Complete values were compared in memory; no prepared JSON capture was persisted.
Two candidate unused warnings and nine other compiler warnings remain in stderr.

The six reviewed consumer files exclude optional Schedule preparation cases.
UIREQ034 remains OPEN and its original Schedule runtime scenario UNRUN. This
value is not an admitted generic ProgramView, runtime MachineImage, ConstructorFields
execution, C/P lowering, financial runtime or native result. The older
[declaration-only API](nominal-declaration-api.md) remains a distinct interface.
See [the guide](../how-to/prepare-generic-program-types.md) and
[the runtime boundary](../explanation/whole-program-types-and-runtime.md).
