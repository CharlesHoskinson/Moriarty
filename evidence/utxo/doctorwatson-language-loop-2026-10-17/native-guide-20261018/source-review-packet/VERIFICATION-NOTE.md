# Explicit Rust CLI selection — verification note

GPT-6.1 Sol high prepared a documentation-only, unapplied patch to
`docs/NATIVE-CORE.md`, using the earlier public-developer-path proposal and
current checkout source. Fresh independent Gemini audit remains with root.

Predicate: every native-guide command invocation selects the same Rust CLI
explicitly while preserving its existing operands, package, cwd and feature
scope. All guide invocations now use `cargo run -p mori --locked --` from the
repository root. Generated starter README commands retain their project cwd;
the guide instructs replacing their executable with the absolute Rust binary
path before following that README. The separately installed Node beta remains
a distinct source-profile consumer. Native feature defaults and the narrower
`--no-default-features` build are explicitly distinguished using current CLI
help source; no feature flag was added to existing example commands.

Source/data checks performed: all shell blocks compare exactly to the original
after normalizing only the executable selector; continuations, arguments,
relative paths, placeholders and redirects are unchanged. No unqualified
`mori ...` invocation remains in the guide. The existing workspace QA command
is unchanged. The patch touches only this document. Current checkout content
matches the before SHA-256 below. SOURCE-PINS.json records full source-basis,
before/after and patch digests; complete before/after copies are provided.

Before: `14efb29a17b61ad69e24bbb5163a7ab62df464cbfc976a20b24b9ea52ee9f00b`.
After: `904bddfaa9eabe49f5e935a6e704ca65500a2edb975e17500bd56088b783adce`.
Patch: `93b381975b780cf6f320db27f9cc176e9fca428d7dde4e5444c6a1cc05da923a`.

Only file inspection, text/hash checks and the required read-only plugin
status were executed. Cargo, tests, SDK, runtime, package, network and private
reads were not performed. No Git or checkout mutation was made. The patch
claims no standalone installation or runtime success. All original release,
U/M/MC/SP/G, financial, composition and UTXO exits and counters remain open or
unchanged according to their existing records. Security/proof/resource judgments
remain owner-unassessed; this patch changes no such policy or requirement.
