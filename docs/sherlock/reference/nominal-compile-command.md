# Nominal source compilation through `mori compile`

Source candidate only; compiler, selected handler and five new handler tests
are **UNRUN**.
The earlier bounded text API's first normal Core compile timed out at 30 seconds;
its tests/capture remain unrun. This new checkout supplies source integration,
not a successful rerun or target result.

The explicit selection is:

```sh
mori compile program.mori \
  --execution-profile sherlock-helper-microstep/1 \
  --source-profile moriarty-sherlock-nominal/1 --entry \
  --execution-config nominal-config.json --target target.json --out NEWDIR
```

`--entry` selects the closed entry. `--action` conflicts with entry selection;
nominal `--observe` is unsupported. Existing omitted/Core and dedicated-draft
source selections retain their existing parsers and config formats. Simulation
continues to support only its existing dedicated-draft source selection.

The new distinct config format is `sherlock-nominal-host-config/1`, with required
`profile:sherlock-helper-microstep/1`,
`grammar:GenericNominalExtensionReserved` and `finance:Absent`. It requires the
existing 22 owner bounds, the existing HostOnly layout fields/four capacities,
all 16 existing `ExtCapacitiesC` fields under `extension_capacities`, and explicit
fuel/stack inputs. All extents use canonical decimal strings. There are no
numerical defaults. HostOnly field/width/scalar-layout selections remain
explicit nulls; owner admission retains its existing limits and refusals.

`preprocessing` requires exactly the existing eight parser extents:
`max_source_bytes`, `max_tokens`, `max_functions`,
`max_parameters_per_function`, `max_total_parameters`, `max_ast_nodes`,
`max_parse_depth`, `max_structural_ast_depth`. The last three map respectively
to Expr+Pattern occurrences, active parser recursion and Expr structural depth.
No unconsumed checker/copy budget is presented as enforced.

The actual selected handler calls
`mori_core::sherlock_compile_artifact::nominal_command::compile`, which decodes
those explicit wire inputs and calls the frozen `compile_nominal_host` dependency.
The bounded parser's owned `ExtProgram` reaches the existing owner compiler.
No source function is evaluated, unfolded or replaced by a result literal.
Generic expression body typechecking is **not performed** by this route.

The existing publication path writes `NEWDIR/sherlock-artifact.json`, after
compilation and complete serialization. The distinct output format is
`sherlock-nominal-host-artifact/1`. It includes exact source/config/target byte
digests, the actual typed config, descriptor-only target, real owner admission,
preparation and plan dispositions. The existing production `ProgramView`
serializer exhaustively retains all 23 ProgramSnapshot fields under
`admission.program_view`, including ordered expressions/children/functions,
occurrence paths, type/constructor tables, declaration templates, plans and
constants. Decimal indices follow the existing production diagnostic wire;
this JSON is output-only and is not an executable reopen or native codec.

Source semantics are explicitly `null`/`not-separately-qualified`, correspondence
is unproved, and `body_typechecking` is `not-performed`. The typed API discards
source program name/spans; the command does not fabricate them. No genesis or
identity acceptance follows from table serialization. Witness and constraint
plans remain missing; native export is unsupported and native/target evidence
is unrun. The target descriptor selects no network or backend.

The two positive source tests use a recursive `Parcel<UInt>` return and mutual
nominal declarations with a generic wrapper/match. Author expectations were
frozen before this implementation. Additional tests preserve selection and
existing structured parse/genesis errors. They have no execution result or
independent semantic approval.

Errors retain the existing structured payload. `parse` reports bounded parser
failures. `admit` includes the owner's AD-0a/AD-1/AD-2/AD-3/AD-6
admission preconditions, Pass B finance-syntax refusal, B/T declaration
preparation, C/P expression/MatchPlan preparation and V table admission
failures; it does not mean generic body typechecking occurred. `admit-run`
reports unstepped genesis admission failures. Selection and compile-only observation refusals use
`select` before config I/O. The dedicated draft route retains its separate
`typecheck` stage; the nominal route does not claim that stage.

The source/config/target digests use `intent::hash_bytes`: SHA-256 followed by
two lowercase hexadecimal digits per byte, without a prefix. A separate ordinary
capture consumer is designed to call the same production Args, selected handler
and serializer and retain complete published artifacts for both positive
sources. Capture operations and full comparisons remain unrun. The serializer
now adds explicit `PreparationReportC` and `WorkMapEntryC` projections; every preexisting
wire implementation, including all 23 program fields, is preserved byte for byte.
