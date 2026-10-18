# Exact plain source readiness

Read `SOURCE-NOTE.md` for scope and remaining gaps. `FROZEN.json` binds all packet file pins through `MANIFEST.json`.

| Source | Exact contents |
|---|---|
| `index/*.jsonl` | Three full authoritative raw sparse entries: lsp-server, gen-lsp-types, crossbeam-channel |
| `INDEX-VERSION-ENTRIES.json` | Full exact adopted version entries and all three matching non-yanked crossbeam-channel stable alternatives, including dependency/features/yank/rust_version/checksum fields |
| `registry/*.json` | Exact raw crates.io crate, version and dependency API records |
| `RETRIEVALS.json` | 21 public HTTP 200 retrievals, with requested/canonical URL, UTC times, raw-body SHA-256/bytes; no supplied credentials |
| `archives/` | Two byte-reused adopted archives and three fresh crossbeam-channel archives |
| `archive-verification/` | Fresh identical static-registry bytes for the two reused archives, with current HTTP receipts |
| `original-provenance/` | Original retrieval/source-pin documents retained intact; their unrelated alternative-package records are historical provenance, not additional adopted graph members |
| `source/` | Complete available text package files for five versions (159 files / 2,565,027 bytes); no archive members omitted |
| `PUBLIC-FILE-PROVENANCE.json` | Every source file size/SHA, archive member/identity/SHA and index/registry checksum binding |
| `PACKAGES.json` | Full per-version index/API fields, origin distinction and archive checksums |
| `MANIFEST-DECLARATIONS.json` | Literal build/lib/proc-macro/features/normal/dev/build/target declarations; no resolved selection claim |
| `DECLARATION-AND-LOCK-COMPARISON.json` | Three absent names against exact current main lock; existing same-name pins and explicit unresolved/conditional edges |
| `repo-source/`, `REPO-SOURCE-PINS.json` | Exact 74a909 commit source blobs, not stale local-main/worktree substitutions |

No Cargo binary sparse cache, registry global mutation, lock generation, resolver, build, compiler or acquired package code was invoked. A future actual resolver can choose among the crossbeam alternatives and can alter compatible transitive pins; no solver result is implied by these source facts.
