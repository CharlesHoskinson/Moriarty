# Sherlock implementation journal

This journal records implementation findings, design decisions, short rationales, measured results and open questions as Sherlock develops. It is a working narrative in `wiki-llm/`, separate from the canonical `wiki/` source/claim ledger and the acceptance roadmap. It contains no private reasoning transcripts or private-fork material.

- [October 16: complete observations and the first implementation comparisons](2026-10-16.md)
- [Implementation plan](../../docs/SHERLOCK-IMPLEMENTATION-PLAN.md)
- [Correctness loop](../../docs/SHERLOCK-CORRECTNESS-LOOP.md)
- [Circuit atlas concept log](../sherlock-circuit-atlas/README.md)
- [DoctorWatson improvement log](../doctorwatson-improvement-log.md): observed factory lessons and proposals for a later update

After each frozen implementation iteration, record the source or result identity, what was learned, the resulting implementation choice, the check that supports it, and the next unresolved question. Label observations, interpretations and proposals separately. Preserve failed attempts and earlier entries; add a dated correction when evidence changes. Root integrates entries from parallel workers so their edits do not collide.

The circuit atlas remains the separate visual concept log, with its own authorship and source snapshots. A journal entry is not a new circuit iteration, a security audit or a milestone completion.

- [October18: public source, report observations and pending runtime](2026-10-18-profile-runtime-knowledge.md) — dated source/results/review boundaries; no new closure or circuit semantics.

- [October18: public draft runtime observed](2026-10-18-public-runtime-observed.md) — adopted two-case host results, retained earlier failures and limited mock evidence; no new EARS closure.

- [October18: generic preparation and actual paired models](2026-10-18-generic-preparation-and-paired-models.md) — whole typed-source preparation, finite F03 comparison and retained publication failures; no generic-runtime or circuit result.

- [October 18: retained native06 R3 audit and durable cleanup](2026-10-18-native06-r3-audit.md) — scoped provenance accepted by two independent reviewers; preservation and cleanup completed; outputs remain quarantined and target ELF unrun.
