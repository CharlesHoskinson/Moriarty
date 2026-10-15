# Source export consumer

This standalone Cargo workspace depends on the actual `mori-core` library. It
avoids that crate's dev-dependencies and the parent workspace's native suites.
It calls source parsing/export only; it does not evaluate actions or call an SDK,
Aiken, a prover or a ledger.

From the repository root:

```sh
cargo test --offline --locked \
  --manifest-path crates/mori-core/tests/source-export-consumer/Cargo.toml \
  --target-dir /tmp/mori-core-source-export-target --lib
```

The adjacent fixture directory preserves two independently authored raw sources
and complete expected typed Programs. Its provenance receipt identifies the
original immutable oracle freeze. Expected hash constants were computed from
those fixtures before the export implementation, without calling Moriarty.

The tests compare every typed Program field, exact source and canonical Program
identities, public JSON representation, whitespace-only versus semantic source
changes, current-profile selection and ordinary source validation refusal.
These are source interchange results, not financial execution or target evidence.
The public record is inspectable metadata; its fields do not establish provenance
or admission authority for serialized data supplied by a caller.
