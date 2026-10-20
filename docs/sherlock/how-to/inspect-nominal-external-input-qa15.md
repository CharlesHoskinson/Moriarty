# Inspect the nominal external-input QA result

Open the [dated archive](../../results/sherlock-nominal-external-input-qa15-20261020/README.md) and copy manifest. Read `actual/qa04/03/stdout.bin` for 14 retained passes and failed test 12, then `actual/qa05/03/stdout.bin` for the single new exact pass. The QA5 inverse record under `history/qa05` binds all 14 unchanged bodies. Do not report these cuts as 15 new runtime passes.

Inspect the current Rust consumer, its five real production imports and two `include_str!` fixtures. The source-applicability record checks those seven public inputs byte-for-byte. It also identifies three current Core differences from the qualified library basis, without claiming the latest public tree was rebuilt.

The raw COMMAND files document the observed direct-rustc recipe, environment, working directory and compile/list/runtime limits. They use original local paths. Dependencies and binaries are not shipped; these commands are not a new invocation to run unchanged, a complete CLI qualification or an authorization to replay a cohort. Read acceptance and complete streams before interpreting exit codes. Failed QA3 partial dep-info must not be consumed.

Source-only/UNRUN labels in historical records predate later observations. Use each cut's own receipt and the archive README for the qualified disposition. Private goldens and provider bodies are intentionally absent.
