# Nominal HostOnly observation report

The [ordinary results](../../results/sherlock-nominal-host-observe-20261020/README.md)
retain a successful full normal Core build, ten new exact handler tests, one
changed legacy test and two complete actual observations. This qualifies the
selected-handler route. Both complete source-symbolic terminal values matched
separately prefrozen independent expectations; raw state and cost fields are
retained without a whole-artifact semantic equality claim. Historical
compile-only receipts keep their own scope.

The new public entry is
`mori_core::sherlock_compile_artifact::nominal_body_command::observe(source,
config_source, target_source) -> Result<NominalHostObservation, CompileFailure>`.
The output-only Serialize type has private fields and no public constructor.
`failure(stage, message)` belongs to that outer type, not the shared
`ExecutionReport`. No executable JSON import codec is provided.

The outer fields are `format`, `qualification`, `preparation` and `execution`.
The format is `sherlock-nominal-host-observation/1`. The exact qualification is
`HostOnly nominal body-typed owner execution; no proof/native/target qualification`.

`preparation` retains the complete existing body-typed compile artifact: all 23
program fields, source/config/target digests, actual admission/preparation,
completed static judgments and unchanged missing/unsupported plans. Both retained
positive artifacts contain this full nested projection. It still says
`evaluation_performed:false`; no genesis-image field is invented in that format.

`execution` reuses the existing `ExecutionReport`: `evaluation_performed`,
`completed`, actual `summary`, decimal `events_forwarded`, `event_capture`, full
post-run `image` and actual `resource_report`. Capture is `not-retained`; the
observer counts each actual forwarded event and stores no event vector. No full
trace continuity claim follows from this terminal report.

Config, parse, first preparation, body type, owner admission and genesis errors
keep existing complete payloads/stages. A run tooling failure retains true
evaluation, the actual partial prefix and `completed:false`. A later bound-image
observation refusal retains its actual summary/count and `completed:true`.
Runtime error metadata wraps the whole original error under `error`, with
`source_profile`, `span:null` and completed `body_typechecking`. Runtime errors
never use the compile-only failure helper that resets evaluation metadata.

Later serialization/publication failures retain evaluation/completion/count and
body completion through the outer report's `failure`. Their message-shaped
payload has `message`, `source_profile`, `body_typechecking`, and no span key.
The manifest-serialization stage is `serialization`; publication is `publish`.
Existing NEWDIR and non-overwriting publication rules remain unchanged.

A genuine public-route partial-prefix HostViolation or post-run observation
refusal fixture has not been identified. Those paths remain source-reviewed and
unexercised. Current report fields give no known reachable CLI string-serializer
error. The passing writer-I/O test exercises the outer failure method after a
real observation; it does not exercise that CLI serializer. The passing
publication-refusal test covers NEWDIR reservation collision after execution,
not a later hard-link publication failure. No backend-linked executable, native,
proof, target or whole-EARS acceptance follows from these checks.
