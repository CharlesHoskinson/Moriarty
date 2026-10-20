# Public nominal external inputs

Use the paired `--nominal-inputs FILE` and `--nominal-function NAME` flags with
`--entry`, `--execution-profile sherlock-helper-microstep/1` and
`--source-profile moriarty-sherlock-nominal-body-typed/1`. The pair requires both
flags and conflicts with `--action` and `--execution-inputs`. The existing Core
UInt/Bool action-input map retains its separate scope. With the nominal pair
omitted, the original closed entry remains active.

The source header remains `moriarty-sherlock-nominal/1`. In bound mode, source
parsing retains declarations, functions and the original entry provenance. The
selected named function receives concrete visible argument constants in a new
entry call. That entry replaces the original before first type preparation,
body checking, owner admission and genesis. These values are public program
constants; private witness compilation and fixed circuit identity remain open.

Input JSON has exactly `format` and `arguments` at the root:

```json
{"format":"sherlock-nominal-external-inputs/1","arguments":[]}
```

`arguments` is an array in function-parameter declaration order. Each member has
exactly `name` and `value`. The name must match the declared parameter. A value
is one of the following exact shapes:

- UInt: `{"kind":"UInt","value":"7"}`, a canonical decimal string in UInt128.
- Bool: `{"kind":"Bool","value":true}`, using a native JSON boolean.
- Constructor: `{"kind":"Constructor","type":TYPE,"ctor":"NAME","fields":FIELDS}`.

A constructor field is `{"name":"DECLARED_NAME","value":VALUE}`. Fields retain
declaration order, including explicit empty arrays for nullary constructors.
`TYPE` is `{"kind":"UInt"}`, `{"kind":"Bool"}`,
`{"kind":"Named","name":"TYPE_NAME"}`, or
`{"kind":"Apply","name":"GENERIC_NAME","args":[TYPE,...]}`. Concrete generic
arguments are explicit. Constructor owner, field types, names and order must
agree with the selected function and declarations. List/Tree builtin aliases
are outside this input wire; declared recursive nominal types are supported.

The existing bounded file reader and strict JSON parser are reused. Duplicate
keys, trailing data, oversized/deep JSON, malformed scalars, unknown/ambiguous
functions and incompatible values refuse. No missing value or numerical default
is supplied. Existing parser occurrence and structural-depth extents also
account for the synthesized call, literals, value and annotation nodes; the
original parsed entry occurrences remain counted. This accounting does not
certify whole-process H1 work or memory.

The handler reads source before selection. Within the selected handler, pairing
and profile checks precede config/target/input reads. Config and target decode
precede nominal JSON decode; nominal JSON decode precedes bounded source parsing.
Nominal input-file IO uses stage `inputs`; malformed nominal JSON and binding errors use
`nominal-input`. Source parse, first preparation, body typing, owner admission
and genesis retain their existing stages/payloads. Decoder errors contain
`NominalInput.variant` and the exact original named fields.

Compile output retains `sherlock-nominal-body-typed-host-artifact/1`. Bound output
adds `nominal_binding`: exact input-file digest, selected function/source index,
declaration-order argument metadata/visible values, counts and stale-entry
provenance. The original Entry sites remain source-only; synthesized entry
diagnostics have no source span. Function-body diagnostics retain their original
span rules. Ordinary metadata counts use JSON numbers; existing owner wire
counters/IDs retain their established representations.

`--observe` uses `sherlock-nominal-host-observation/1`, preserving the complete
compile artifact under `preparation` and the actual owner execution report under
`execution`. It runs the same admitted pair once. The nested ProgramView retains
all23 fields. Witness/constraint plans remain missing and export unsupported.

The additive Rust entries are `compile_nominal_source_host_bounded_with_nominal_inputs`
in `sherlock_compile_artifact::nominal_body_compile`, and
`compile_with_nominal_inputs` / `observe_with_nominal_inputs` in
`sherlock_compile_artifact::nominal_body_command`. The typed request owns the
function name and decoded JSON. Existing compile/observe signatures and the
three-element artifact `into_parts` remain. Downstream exhaustive error matches
and public struct literals may need the new variant/optional fields.

See the [actual results](../../results/sherlock-nominal-external-inputs-20261020/README.md)
for the exact tested shared-handler scope and open backend/native obligations.
