# Nominal host observation record

The output format is `sherlock-nominal-host-artifact/1`. The production selected handler publishes exact source/config/target digests, typed config, descriptor-only target, owner admission and preparation, and plan/evidence dispositions. `admission.program_view` contains the full23-field ProgramView wire. Decimal indices use the existing output wire; arrays retain order and multiplicity.

`evaluation_performed` is false. `body_typechecking` is `not-performed`; `semantic_correspondence` is `unproved`; source semantics are null and not separately qualified. Native export is unsupported and native/target evidence are unrun. The output is not an executable codec, checkpoint or source-authentication proof.

Actual wrapped artifact:14441 bytes, SHA256 `1cfa727553a2f12ad80859a04bc5cbbfb3d9f3b055b1db58072c600821cd2fbc`. Actual mutual artifact:19109 bytes, SHA256 `87f43fc40f2792109811b14c149efd05a1db455e356da8eb86f796c445b5b1b8`. Both are retained in the [public evidence selection](../../results/sherlock-public-nominal-compile-20261019/README.md); stdout captures add exactly one newline.

The offline normalizer and expected-format adapter are QA artifacts under that result package. They are never invoked by the public compile flow and are not semantic evaluators. Their synthetic test receipts and Root’s wrapped comparison are separate observations. No mutual independent complete-table result or full CLI run is implied.
