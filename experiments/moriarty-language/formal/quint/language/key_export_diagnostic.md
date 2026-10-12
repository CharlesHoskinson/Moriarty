# Bounded key-export diagnostic: independent symbolic QA

This model checks only the ordinary bounded-writer/serialization diagnostic contract. Ground truth is `key-export-diagnostic-independent-expectations-01.md`, SHA256 `57f466a7109397576836c924f87dfe39ff532d640d6d5cf89b1ecc023c72cc7e`, with the root-adopted ordinary repair design. It does not execute a key encoder, inspect any native key/SRS, prove Rust correspondence, measure complete serialized key size or establish native feasibility.

One caller shares the writer's local state. `begin` starts the existing tagged-encoding adapter; writes either admit complete chunks or refuse before append. The first refusal freezes the exact accepted prefix and its evidence. `finish` prioritizes that evidence over successful, misleading, wrapped or replaced serializer returns. A serializer error without writer refusal stays Serialization. A successful adapter result requires no writer refusal and an exact admitted prefix. No failed result carries token/prefix data.

The executable domain has byte caps0..4, chunks of at most4 payload-free TagToken/PayloadToken positions, Prover/Verifier kinds and opaque action/degree metadata. TagToken positions count toward the same cap as payload positions. `serializerCalls` counts the outer tagged-encode invocation, not the `Serializable::serialize` payload callback: Rust can refuse a tag header before calling the payload callback. `sizeProbeCalls` models an additional adapter size pass, which remains absent. The model does not reconstruct real SDK tags or claim any tiny successful stream is a valid encoded native key.

Arithmetic overflow is a separate pure checked-length probe: symbolic `usize::MAX-1 +2`, cap `usize::MAX`, Overflow reason and no numeric attempted minimum. It allocates no huge prefix and makes no architecture-specific word-width assumption. Rust checked-add semantics and standard library behavior are correspondence premises, not proved by this abstraction. Finite cap refusal records exactly accepted prefix length plus the first rejected chunk length, a lower bound on the attempted stream. Later/unattempted writes cannot turn that bound into a complete serialized size. Serializer-only failure has neither rejected chunk nor attempted minimum.

The model's empty-write action represents a direct `Write::write` call. After refusal, direct empty writes refuse in the implementation. `Write::write_all(empty)` may bypass that call and return Ok; it appends nothing and does not clear recorded failure, so final encoding still refuses. Flush is a no-op outside this model; it also cannot clear failure or admit a partial encoding.

## Actual bounded checks

Installed Quint0.32.0, TypeScript backend, no package downloads or exhaustive verification. Both files typecheck; **25 deterministic tests pass**. Controls include exact-cap3 then one-byte rejection,2+2 under cap3, empty/zero boundaries, symbolic overflow, serializer failure before/after an admitted prefix, misleading messages, ignored/wrapped/replaced errors, smaller and empty post-refusal writes, exact context/degree0, header counting, one-pass admitted success and terminal-state guards.

The final sampled run used seed20261012, max500 traces and max12 steps, and exited0 in16.133s. **No counterexample was observed in these500 traces.** This is a bounded sampling result, not a proof. Every target witness was reached:

| Witness | Reached traces /500 |
| --- | --- |
| Begin / admitted write / empty write / later refusal | 272 /90 /33 /40 |
| Cap / overflow probe / serializer / success | 82 /228 /152 /37 |
| Ignored / wrapped / replaced writer error | 18 /18 /19 |
| Misleading serializer-only error | 40 |
| Verifier metadata / degree0 metadata | 231 /182 |

Two test-only fault actions are excluded from production `step`. A one-transition run after the initialized refused-write fixture that ignores writer failure exits1 with `noTruncatedSuccess` violated: accepted prefix2 is returned as Encoded after rejecting2 under cap3. A second run admits a smaller fitting byte after refusal and exits1 with `firstFailureSticky` violated: prefix grows2→3 while the frozen prefix remains2. Both have actual retained ITF counterexamples. Routine tests pass against the guarded model.

Early parse failures (reserved names, escaped-string grammar and a missing parameterized-action return annotation) are retained. The first fault attempts failed to parse; **those are not counterexamples**. Only `fault-swallowed-02` and `fault-later-fit-02`, whose stderr reports Invariant violated and whose ITFs contain the violating states, qualify as intentional negative controls. Action text is opaque here; actual JSON/control/Unicode escaping is a separate Rust consumer predicate.

## Current bounded source correspondence

| Independent obligation | Model | Observed Rust source |
| --- | --- | --- |
| Overflow before cap classification; checked lower bound only | Symbolic `probeOverflow`; finite `firstRejected` / `truthfulAttemptedMinimum` | `proving.rs::check_key_write` performs checked_add then cap comparison; first refused write stores checked attempted minimum |
| Refuse before append; first failure terminal and prefix exact | `reject`, `refuseAgain`, `firstFailureSticky` | `BoundedWriter::write` checks existing first_failure before any append, records first accepted/rejected lengths, then refuses; bytes extend only on success |
| Wrapped/replaced/ignored writer failures cannot succeed | `finishResult`, `noTruncatedSuccess`, `faithfulFailure` | `encode_key` invokes tagged_serialize once, prioritizes writer.first_failure over serializer result and returns typed error before publishing bytes |
| Serializer-only failure is not classified by error message/Kind | `serializerDiagnostic`, misleading-return controls | `encode_key` classifies serialized.err only when no recorded writer failure; it retains no raw serializer error text |
| Actual action/kind/discovered degree/effective cap forwarded | Exact Context and literal diagnostic records | Selected PK/VK calls pass Prover/Verifier, action, available degree and existing byte bounds; public VK uses manifest action, forwarded available degree and effective cap from `local_deployment.rs::derive_all` |
| Typed metadata, zero is a value; legacy error category unchanged | Typed Reason/Count/Measurement/Context | `lib.rs::KeyExportDiagnostic`, `KeyExportFailureReason`, `Error::key_export`; new metadata attached to existing native-proving/generated-key-size-or-encode error |
| Quiet metadata, exact escaped action and no raw payload | Outside symbolic scope; source inspected only | `core_io.rs::ledger_operation`, `key_export_identity`; ordinary tests cover valid JSON escaping once, ASCII-safe UTF-16 Unicode escapes and Quiet/ColorNever actual stderr |

Pins for the inspected source functions are in `source-correspondence-pins.json`. At that inspection, proving.rs SHA256 `140bea254f6c68ce544f8fadcd38c0ff55316c233c0d0d7d3a62a40df5f1c1ba`; lib.rs `cbeb160fbac08b4250a7b808dc5aafc7f60032fb02b0aaade61b2cdb4d384e8c`; local_deployment.rs `bef75fc124abe160a1107d879c88782ee6e60e59b36f64cb69080cd5bb67e74e`; core_io.rs `c8a8a14f751804d892cfd8aee45d10e8a41bebc811cd77eb2688ae6ddf09f1ca`. This is focused read-only correspondence, not full frozen-source approval. No mismatch was established in these windows. Rust test execution/results remain the author's separate captures; this QA agent ran only Quint.

Structural review: one grouped variable, enums, pure guards/updates, thin actions, complete main step, per-action witnesses and non-trivial state-dependent invariants. Init and deterministic stage paths execute; every action assigns the full state. Symbolic overflow and opaque action/serializer tokens are explicit abstractions, not concrete word-width/JSON/native proofs.

Runtime captures: `/home/charl/Moriarty/.moriarty-dev/runtime/installed-call-20261012/signed-installed-source-20261012/key-export-diagnostic-01/quint-qa-01`. Final ordinary checks, sampled run, targeted ITFs, source pins and final manifest retain actual commands/terminal codes and identities. Update this model before asserting correspondence after changed writer failure/append semantics, diagnostic counts/reasons, adapter return priority or context forwarding.

No native k/model/cost/synthesis/Mock/SRS payload read/stat/hash/decode/init/keygen/prove/Real, native owners, signatures, network or wallet work occurred. Original Signed QA/source/producer/native failure records remain unchanged. The historical generic key export failure still identifies no exact key kind, degree or complete size. This diagnostic repair does not retroactively classify it, repair it, grant a retry, or close financial/source/SRS/cryptographic/native acceptance predicates.
