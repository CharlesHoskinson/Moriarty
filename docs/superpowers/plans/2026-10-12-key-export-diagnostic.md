# Key Export Diagnostic Implementation Plan

> Execute inline with superpowers:executing-plans; root owns independent audits,
> native resource decisions and publication.

**Goal:** Report precise, payload-free metadata for future bounded key export failures.

**Architecture:** Add typed metadata to the existing ledger Error. The existing
bounded writer records its own first refusal and the existing single tagged
serialization call consumes it. Pass already-known context from consumers and
extend ledger_operation with ASCII-safe JSON identity escaping.

**Tech Stack:** Rust 1.99.0, existing midnight-serialize and serde_json.

## Global constraints

- Keep current caps, native invocation counts, compiler/financial behavior and legacy errors.
- No genuine SRS access, native key/proof/model work, network or new framework.
- Captures: existing signed-installed-source runtime directory, key-export-diagnostic-01.

## Task 1: Bounded diagnostic and its consumer

Files: `crates/mori-ledger/src/{lib.rs,proving.rs,local_deployment.rs}`;
`crates/mori/src/commands/core_io.rs`; evidence document in `docs/evidence/`.

- [x] Add an ordinary tagged fixture which ignores a rejected write; assert
  `encode_key(...).is_err()`, then run the proving unit filter and retain Red.
- [x] Add typed KeyExportDiagnostic, KeyExportKind and KeyExportFailureReason,
  keeping Error Display unchanged. Add exact metadata/clone/equality controls.
- [x] Add checked-length/first-refusal state and pass action/kind/degree into
  encode_key. Use `accepted.checked_add(chunk)` to distinguish overflow from cap.
  Preserve one tagged_serialize invocation and inspect its result plus writer state.
- [x] Test exact-cap, multi-byte refusal, zero/empty, overflow, genuine/misleading
  serializer errors, wrapped/ignored writer errors and counting-fixture success.
- [x] Pass existing degree/action/effective caps through the three native export
  calls; inspect that no k or size probe was added.
- [x] Render typed metadata in ledger_operation. Quote action using JSON escaping
  with ASCII-safe Unicode and test round-trip plus actual quiet shell stderr.
- [x] Run focused Green, standard nonignored ledger, relevant CLI unit/integration,
  core, fmt, whitespace and ordinary mori build checks, retaining terminal logs.
- [x] Document the historical 17.0538s generic failure and its unknown key kind,
  degree and total size. Record limited ordinary guarantees, then commit/freeze after the independent Quint QA files arrive.
