# Nominal declaration preparation API

This reference describes the installed eight-file `candidate-03-pass-b-names`
declaration-preparation unit. Root adopted its exact source and independent
positive result in the [declaration adoption record](../../results/sherlock-nominal-declaration-preparation-20261018/ROOT-DECLARATION-ADOPTION.json).
Normal Core compiled (exit0, 3.987870814 seconds); the new external consumer
compiled, listed its exact one test and passed that test. Its complete independent
comparison covers seven type rows, one type-argument slot, eight constructor
rows, ten constructor-field slots and six local type occurrences, including all
returned metadata. Both author cfg(test) cases remain unrun; negative cases and
formal/runtime obligations remain separate.
Candidate03 preserves seven candidate02 files and replaces only the declaration
name collector. Earlier author ERROR terminals remain historical; process exit0
never changed those terminal statuses. No nominal runtime or whole
UIREQ-004/007/034 acceptance is claimed.

## Entry points and inputs

The installed API publicly re-exports both functions and their output types from
`mori_core::sherlock::helper_machine::shared`. The implementation remains in the
private `shared::owner_prepare` module.

```rust
pub fn prepare_nominal_declarations(
    decls: &[TypeDeclSrc],
    layout: &LayoutSchemaC,
    domain: GrammarDomain,
    locator: Option<LocatorC>,
) -> Result<PreparedNominalDeclarations, ExtTypeBuildError>;

pub fn prepare_program_nominal_declarations(
    program: &CoreProgram,
    decls: &[TypeDeclSrc],
    layout: &LayoutSchemaC,
    domain: GrammarDomain,
    locator: Option<LocatorC>,
) -> Result<PreparedNominalDeclarations, ExtTypeBuildError>;
```

`TypeDeclSrc` and `ExtTypeBuildError` belong to
`shared::owner_ext_type_builder`. The other signature types are in `shared`.
These functions take typed Rust data, not a new source-text grammar or parser.
The `CoreProgram` argument is the existing B source carrier, not an extended
nominal program AST.

| Typed input | Exact fields or variants |
| --- | --- |
| `TypeDeclSrc` | `name: String`, `parameters: Vec<String>`, `ctors: Vec<CtorDeclSrc>` |
| `CtorDeclSrc` | `name: String`, `fields: Vec<CtorFieldSrc>` |
| `CtorFieldSrc` | `name: String`, `ty: TypeExprSrc` |
| `TypeExprSrc` | `UInt`, `Bool`, `List`, `Tree`, `Named { name }`, `Apply { name, args }`, `Param { name }` |

The caller supplies the existing `LayoutSchemaC`, including its bounds and name
capacities, an explicit `GrammarDomain` and optional locator. No frontend
configuration or default grammar is selected by these functions. Owner checks
return `ExtTypeBuildError`; its variants are `CoreTooling`,
`ExtCapacityNotAdmitted`, `ExtAdmissionRejected`, `ExtUnsupportedFeature`, `Host`.
This page describes the interface, not a separate assessment of those checks.

## Output and name-pool scope

`PreparedNominalDeclarations` has exactly five fields:

```rust
pub struct PreparedNominalDeclarations {
    pub scope: PreparedDeclarationScope,
    pub domain: GrammarDomain,
    pub interned_names: Vec<String>,
    pub decl_name_ids: PassBTypeNameIds,
    pub tables: BuiltAdtTypeTables,
}
```

| Entry | Returned scope | Name pool |
| --- | --- | --- |
| `prepare_nominal_declarations` | `PreparedDeclarationScope::DeclarationOnly` | Declaration type names, constructor names and field names only |
| `prepare_program_nominal_declarations` | `PreparedDeclarationScope::ProgramAndDeclarations` | The declaration names plus existing Core program function/parameter names, expression binders/callees and pattern binders |

Declaration type-parameter names and names appearing only inside type uses do
not add NameIds. The same spelling can still be present when it independently
occurs as a declaration/constructor/field name or a collected Core program name.
This corrects candidate02's extra-name policy; the frozen independent oracle is
unchanged.

Both pools deduplicate names and sort them with Rust string ordering. The program
entry and all function bodies contribute names to the second scope. Declaration
NameIds are bound against that actual pool by the owner; callers do not forge
`PassBTypeNameIds`' private owner marker. The carrier exposes `name_count` and
`decls`; each `DeclNameIds` contains `type_name_id`, `ctor_name_ids`,
`field_name_ids`.

The program variant expands name-pool scope only. It does not traverse nominal
type uses throughout program expressions/signatures or instantiate types from
such uses. Additional program names may shift NameIds; a declaration-only pool
cannot be substituted for the program-and-declarations pool. Neither returned
value is a `ProgramView`, `Admitted` value or runtime `MachineImage`.

## Materialized T1/T2 tables

Both functions create the checked declaration view, walk declaration field-type
occurrences, then call `owner_ext_prepare_types::instantiate_declared_types`.
This returns real populated vectors, not a table-count report:

| `BuiltAdtTypeTables` field | Representation |
| --- | --- |
| `types` | `Vec<TypeRowC>`; each row has `name_id`, optional declaration index `decl`, argument `SliceC`, `ctor_start: CtorIdC`, `ctor_len` |
| `type_args` | `Vec<TypeIdC>` addressed by type-row argument slices |
| `constructors` | `Vec<CtorRowExt>` with `owning_type`, `name_id`, `source_order`, `field_start`, `field_len` |
| `ctor_fields` | `Vec<CtorFieldRowExt>` with `name_id`, `type_id` |
| `decl_param_counts`, `decl_ctor_counts` | One count per source declaration |
| `max_arity`, `name_count` | Selected builder arity bound and actual name-pool count |

The builder starts with UInt/Bool/List/Tree at TypeIds 0/1/2/3. T1 registers every
non-parameterized declaration in source order before resolving any fields.
Its final TypeId therefore exists when a mutually recursive field refers to it.
Registration queues each such type for T2. A parameterized declaration is a
template; it does not receive an instance merely because it is declared.

T2 drains the owner's FIFO. For each dequeued type it visits constructors in
source order and fields in source order. The owner resolves each type
expression under the enclosing instance's parameter bindings and appends the
actual constructor/field rows. Newly discovered declaration-plus-TypeId-argument
keys are interned, assigned a TypeId and queued once. Nested application arguments
are resolved left to right before their enclosing application. The FIFO is
drained before `finish()` returns the completed table vectors. Unused
parameterized templates remain uninstantiated by this leaf.

The ordinary occurrence adapter retains declaration index, constructor ordinal,
field ordinal and an ordered type-expression child path. These are local
declaration occurrences, not ProgramImage `SourceOccurrenceId`s or expanded
runtime-step identities. The supplied locator is forwarded through the builder;
this leaf does not construct a full-program occurrence mapping.

Sources: [public exports](../../../crates/mori-core/src/sherlock/helper_machine/shared.rs),
[connected producers](../../../crates/mori-core/src/sherlock/helper_machine/shared/owner_prepare.rs),
[type builder and carriers](../../../crates/mori-core/src/sherlock/helper_machine/shared/owner_ext_type_builder.rs),
[T1/T2 orchestration](../../../crates/mori-core/src/sherlock/helper_machine/shared/owner_ext_prepare_types.rs),
[declaration occurrence walk](../../../crates/mori-core/src/sherlock/helper_machine/shared/nominal_declaration_order.rs).
The source links describe the installed declaration-preparation API. Its scoped
adoption adds no generic runtime dispatch or whole-clause acceptance.

For the distinction between declaration preparation and actual constructor
execution, see [the explanation](../explanation/nominal-declaration-boundary.md).

## Separate whole-program source preparation

The newer [prepare_ext_program_types API](generic-program-preparation-api.md)
accepts an owned ExtProgram and retains it with the complete unified names, tables
and T3 records. One checked builder processes T1/T2 declarations before T3
signatures and entry/function-body uses. Its [actual two-fixture result](../../results/sherlock-generic-one-builder-preparation-20261018/README.md)
does not change the declaration-only scope of the two functions on this page,
execute the original Schedule scenario or close UIREQ034.
