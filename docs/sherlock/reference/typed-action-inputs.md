# Typed action input reference

Status: source-only/UNRUN. This is an additive Core/1 action mode within
sherlock-helper-microstep/1 and sherlock-host-artifact/1. It does not change the
first parameter-free batch or establish whole B04 acceptance.

The optional --execution-inputs FILE requires --execution-profile and --action,
and conflicts with --entry. Omitted source profile or explicit moriarty/1 selects
Core actions. The option does not select a backend or a numerical default.

The public Core API is:

```rust
pub fn compile_host_with_inputs(
    source: &str, action_name: &str, config_source: &str,
    target_source: &str, inputs_source: &str, observe: bool,
) -> Result<CompileArtifact, CompileFailure>;
```

compile_host retains its old signature and empty-map behavior. InputPolicy adds
optional public binding: Option<InputBinding>, omitted when None. InputBinding
serializes fields values then file_digest. values is exactly
BTreeMap<String, mori_core::Value>; file_digest is intent::hash_bytes over the
original input text bytes, lowercase SHA-256 hex without a prefix. Public
InputPolicy/Args struct-literal callers must supply the new optional field.

| Mode | kind | declared | supplied | binding |
| --- | --- | --- | --- | --- |
| Flag absent | parameter-free | empty | empty | omitted |
| Explicit file | explicit-typed-map | source order | ascending map names | typed values and raw file digest |
| Existing Draft | closed-entry | empty | empty | omitted |

Other artifact members, full 23-field ProgramView projection, preparation,
admission, plans, observation and evidence retain the existing serializer. A map
with different whitespace can have equal values/program membership but a
different file_digest. Different values may change admitted program membership.

Selection order is execution profile, existing source/action/entry selection,
then the new Core-only scope check. Its exact select-stage message is:

```text
--execution-inputs is supported only for Core/1 action compilation
```

That check precedes existing Draft/structural-nominal observation refusals and
config-path presence or file I/O. Invalid original selection still wins first.
Source bytes are read by the outer command first. The selected handler then reads
config, target, and optional input text in that order. Thus an input I/O error
can precede a malformed already-read config's content error.

Core decoding keeps config parse/consumer conversion, target JSON parse, and
existing validate_config first. Explicit input strict JSON plus existing
Value/UInt serde decode follows, before source parse/global preflight/action
lookup and input count/name preflight. The existing prepare/run consumer retains
missing inputs in declaration order, then type mismatches in declaration order,
then extra inputs in ascending map order, with existing extent checks earlier.

Input file I/O and serde failures use stage inputs and their original display
text under actual_error.message. Strict JSON failures use stage inputs and the
complete existing Core error object. Other staged errors retain their existing
payloads and execution flags. No clamp, coercion, input filtering or default is
added. Existing output-directory collision failure preserves completed-run flags
and the actual forwarded-event count. No reachable CLI string-serialization
failure or later hardlink failure is claimed by the proposed tests.
