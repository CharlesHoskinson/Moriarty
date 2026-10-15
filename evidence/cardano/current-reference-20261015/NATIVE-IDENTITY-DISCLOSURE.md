# C-1 repair and identity disclosure

The source-owned export moved unchanged, except its import path, from mori-core to mori-cardano. mori-core/src/lib.rs is restored byte-exact to 9c63048a and mori-core/src/export.rs is removed. Only the lib/types/projection use paths changed. Export field names/order, hash/parse operations and serialized records are unchanged. No advisory production change was applied.

The retained root Cargo.lock mori-cardano entry alters mori_ledger::proving::source_closure() and therefore the declared native build-identity strings, including compiler_hash, static compile/reopen build identity and PublicDerivedCallKey admission. This expected identity delta is disclosed; no ledger/SDK check or native qualification is inferred. Before merge the ledger source-closure selection still requires separately scoped re-execution, which is not authorized here.

Opus-authored PRJ-01 wording amendment: through the source-owned export (current mori_core::parse + canonical Program hash). The earlier exact attributed reports remain unchanged. Other historical export_source path mentions describe their historical source, not the repaired public API path.

Only Run A is authorized here after preflight. Proposed staged consumer remains unapplied. No root workspace build, manifest/lock mutation, new package, network or SDK/native action is permitted.
