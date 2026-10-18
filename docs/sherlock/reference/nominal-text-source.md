# Nominal text source interface

The public Rust module is `mori_core::sherlock_nominal_source`.

| API | Result |
| --- | --- |
| `parse_nominal_source_bounded(&str, DraftParseBudget)` | `Result<ParsedNominal, DraftError>` |
| `prepare_nominal_source_types_bounded(&str, DraftParseBudget, &LayoutSchemaC, ExtDomain, Option<LocatorC>)` | `Result<PreparedNominalText, NominalSourceError>` |

`ParsedNominal` owns `name`, the actual `ExtProgram`, and `NominalProvenance`.
`PreparedNominalText` owns `name`, provenance, and the existing
`PreparedExtProgramSource`. `NominalSourceError` distinguishes `Parse` from
`Preparation`; preparation preserves the actual owner error. Unbounded host
variants remain explicitly H1 APIs.

The explicit source header is `moriarty-sherlock-nominal/1`. Declarations precede
functions and entry, each retaining source order:

```text
type Box<T> { Box(value: T); }
fn wrap(x: UInt): Box<UInt> { construct Box<UInt>::Box(x) }
entry wrap(7);
```

This grammar fragment illustrates the interface; the executed complete fixtures
are linked in the evidence archive. Constructor expressions use `construct
Type::Ctor(fields)` and patterns use `Type::Ctor(patterns)`. Applications are
explicit, for example `Box<Box<UInt>>`. Nullary use/pattern includes `()`;
nullary declaration may omit it. A bare parameter reference inside its own type
declaration is `Param`; other bare type names remain `Named`. `wildcard` maps to
the actual wildcard carrier without changing the lexer identifier rules.

Provenance retains declaration/signature spans, entry-first and then function-order
expression child paths, and nested pattern `(owner, match_path, arm, field_path)`.
Type annotation spans cover complete annotation roots; no separate type-child span
or runtime expansion identity is implied. Pattern/field/arm source order is retained
for later consumers; runtime matching is not performed here.

The existing owner performs name binding, declaration checking and T1/T2/T3 type
preparation under caller layout/domain. Generic expression checking, complete
nominal admitted image/MatchPlan/runtime integration, public CLI compilation,
witness/export and full original034/refinements remain open.
