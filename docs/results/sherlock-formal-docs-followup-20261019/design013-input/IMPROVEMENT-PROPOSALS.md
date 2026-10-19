# R-only wiki / Doctor Watson workflow proposals

These are proposals for later process review, not accepted workflow changes.

1. **Derive vectors mechanically.** Generate selector lists, membership counts, per-test exits, and duration/RSS summaries from pinned aggregate records. Preserve the exact source record digest and reject hand-entered count tables when a machine-readable source exists. This avoids transcription drift between test selection and graph state.
2. **Rekey review context on source changes.** Bind each review result to source-manifest, prompt, and readset digests. Any change to those inputs gets a new review identity; cached model context cannot stand in for review of changed bytes. Record the identity alongside the source hashes.
3. **Keep findings narrow and testable.** A mandatory review finding names the requirement, exact source path and region, impact, minimal repair, and a falsifier. Keep the repair scoped to that finding. Do not broaden the framework or adjacent runtime work unless evidence requires it. Preserve ABSTAINS and dissent.

These proposals follow the PR297 evidence that canonical compile/tests remain unrun and the current review's F1–F3 findings. They do not change product, source, or canonical records.
