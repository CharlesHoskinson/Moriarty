# Source index — UNADMITTED one compiler check

Candidate revision/tree are supplied by the root integrator; this lane made no Git query. Source bytes are pinned independently. Full files are separate from the<55KB literal runtime excerpts.

## Direct selected functions

| Function | Source path:line | Scope |
|---|---|---|
| selected test | [crates/mori-backend/tests/stages.rs:374](full-originals/repository/crates/mori-backend/tests/stages.rs) | parse Counter/Payment; Target::ledger9; production compile twice; old snapshot assertions |
| Core parse | [crates/mori-core/src/parser.rs:5](full-originals/repository/crates/mori-core/src/parser.rs) | parse_profile/lexer, full file separate; partial primary head |
| Target::ledger9/validate | [crates/mori-backend/src/lib.rs:64](full-originals/repository/crates/mori-backend/src/lib.rs) | unsigned Target constructor; validate at88 |
| compile | [crates/mori-backend/src/lib.rs:367](full-originals/repository/crates/mori-backend/src/lib.rs) | Core1 guard then compile_mode |
| compile_mode | [crates/mori-backend/src/lib.rs:389](full-originals/repository/crates/mori-backend/src/lib.rs) | Core validate/registry/canonical digest/lower actions/artifact size JSON |
| Core validate | [crates/mori-core/src/runtime.rs:120](full-originals/repository/crates/mori-core/src/runtime.rs) | full body separate; primary head only |
| registry | [crates/mori-backend/src/liabilities.rs:42](full-originals/repository/crates/mori-backend/src/liabilities.rs) | source registry helper |
| immediate/Builder | [crates/mori-backend/src/lib.rs:508](full-originals/repository/crates/mori-backend/src/lib.rs) | instruction/expression/arithmetic/public helpers |
| lower_action/segment | [crates/mori-backend/src/lib.rs:766](full-originals/repository/crates/mori-backend/src/lib.rs) | full body primary; context/frame/IR serde/reopen |
| context_hash/json | [crates/mori-backend/src/signed_relation.rs:152](full-originals/repository/crates/mori-backend/src/signed_relation.rs) | canonical context helper, signed entry bodies outside primary |
| digest_limbs | [crates/mori-backend/src/authorization.rs:130](full-originals/repository/crates/mori-backend/src/authorization.rs) | context digest limbs |
| hex reexport | [crates/mori-backend/src/proving.rs:225](full-originals/repository/crates/mori-backend/src/proving.rs) | Core hex alias; proving methods outside primary |
| canonical/hex | [crates/mori-core/src/intent.rs:72](full-originals/repository/crates/mori-core/src/intent.rs) | canonical serde sort/hex support |
| IrSource::load | [pinned-sdk/zkir-v3/src/ir.rs:955](full-originals/pinned-sdk/zkir-v3/src/ir.rs) | SDK serde reader/version/load path; model/prove methods outside primary |

## Complete current source originals

All29 exact current repository files are copied under full-originals/repository. MANIFEST.json lists each path,byte count,SHA256 and original source root; the direct-function table above links the runtime consumers. All comparable historical source hashes match. No new financial test source was read.

## Existing source packets and owner reports

- [75-file build-root packet](full-inputs/compiler-build-root-owner-packet/PRIMARY-SOURCE-INDEX.md):33 root scripts,33 manifests,nine literal helpers.
- [22-file additional helpers](full-inputs/compiler-admission-helper-sources/PRIMARY-EXCERPTS-INDEX.md):16 AWS lists/probes,five frozen SDK sources,ff source; verification classes preserved.
- [Observed build surface](full-inputs/compiler-observed-build-inventory/SOURCE-NOTE.md): tree feature strings separated from workspace unification; no compilation-unit graph.
- [Original compiler proposal](full-inputs/compiler-local-check-proposal/COMPILER-LOCAL-CHECK-PROPOSAL.md): historical ABI/effect snapshot scope and U0/U1/U2 gaps; not financial compiler qualification.

- doctorwatson-sdk-ir-runtime-source-owner-20261018-a1: [WHOLE](full-inputs/jobs/doctorwatson-sdk-ir-runtime-source-owner-20261018-a1/WHOLE-SCHEMA-VALID-CANDIDATES.json) + [AGREED](full-inputs/jobs/doctorwatson-sdk-ir-runtime-source-owner-20261018-a1/AGREED-REQUIRED-FIELDS.json); verdict as reported `narrow-runtime-source-inspection-complete`. Not a current execution/resource vote.
- doctorwatson-sdk-scalar-ffi-narrow-owner-20261018-a1: [WHOLE](full-inputs/jobs/doctorwatson-sdk-scalar-ffi-narrow-owner-20261018-a1/WHOLE-SCHEMA-VALID-CANDIDATES.json) + [AGREED](full-inputs/jobs/doctorwatson-sdk-scalar-ffi-narrow-owner-20261018-a1/AGREED-REQUIRED-FIELDS.json); verdict as reported `narrow-source-inspection-complete`. Not a current execution/resource vote.
- doctorwatson-sdk-blst-build-narrow-owner-20261018-a1: [WHOLE](full-inputs/jobs/doctorwatson-sdk-blst-build-narrow-owner-20261018-a1/WHOLE-SCHEMA-VALID-CANDIDATES.json) + [AGREED](full-inputs/jobs/doctorwatson-sdk-blst-build-narrow-owner-20261018-a1/AGREED-REQUIRED-FIELDS.json); verdict as reported `narrow-source-inspection-complete`. Not a current execution/resource vote.
- doctorwatson-sdk-aws-selection-owner-20261018-a1: [WHOLE](full-inputs/jobs/doctorwatson-sdk-aws-selection-owner-20261018-a1/WHOLE-SCHEMA-VALID-CANDIDATES.json) + [AGREED](full-inputs/jobs/doctorwatson-sdk-aws-selection-owner-20261018-a1/AGREED-REQUIRED-FIELDS.json); verdict as reported `narrow-source-inspection-complete`. Not a current execution/resource vote.
- doctorwatson-sdk-aws-cc-owner-20261018-a1: [WHOLE](full-inputs/jobs/doctorwatson-sdk-aws-cc-owner-20261018-a1/WHOLE-SCHEMA-VALID-CANDIDATES.json) + [AGREED](full-inputs/jobs/doctorwatson-sdk-aws-cc-owner-20261018-a1/AGREED-REQUIRED-FIELDS.json); verdict as reported `narrow-source-inspection-complete`. Not a current execution/resource vote.
- doctorwatson-sdk-aws-alternate-owner-20261018-a1: [WHOLE](full-inputs/jobs/doctorwatson-sdk-aws-alternate-owner-20261018-a1/WHOLE-SCHEMA-VALID-CANDIDATES.json) + [AGREED](full-inputs/jobs/doctorwatson-sdk-aws-alternate-owner-20261018-a1/AGREED-REQUIRED-FIELDS.json); verdict as reported `narrow-source-inspection-complete`. Not a current execution/resource vote.

[Tool/config metadata](TOOL-CONFIG-METADATA.json) includes four fixed command pins,actual cc15.2.0/Linuxx86_64 native metadata and four historical ancestor-config absence observations. No fresh query or private config read. Full hashes and provenance roots are in MANIFEST.json.

## Exact omissions

- EXCERPT-BOUNDS.json lists all omitted ranges for excerpted files; complete originals retain those bytes. Core parser/validate are only heads in the primary excerpts.
- Whole files contain unselected methods/tests, including SDK model/prove bodies. Their preservation is not execution permission.
- This selected test does not consume independently frozen financial/work or whole-IR expectations; it reuses the two historical snapshots.
- New financial test files were neither read nor selected. No181-archive or macro/library body closure was acquired.
- No proof/VM/model/k/degree/SRS/key/wallet/ledger actions; no retry/cachewarming/cap reuse.

[Unresolved owner questions](UNRESOLVED-OWNER-QUESTIONS.json) and [tool/config metadata](TOOL-CONFIG-METADATA.json) are primary inputs.
