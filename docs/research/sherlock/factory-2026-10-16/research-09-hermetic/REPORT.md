# Hermetic builds and QA isolation — missing source

**Result:** I could not fetch a primary source within the research budget. Two generic searches found Bazel’s official Hermeticity documentation, but opening both returned HTTP 404. Search result text is not treated as a fetched source. No source claims are therefore reported.

**Source metadata:** No page successfully opened; exact attempted URL: `https://bazel.build/versions/8.0.0/basics/hermeticity`; title shown in search: “Hermeticity | Bazel”; publisher: Bazel; retrieved: 2026-10-16 (fetch failed). Publication/update date unknown.

**Method and authority:** Two web search queries for official Bazel hermeticity documentation; two open attempts, both failed with 404. Primary-source authority is unavailable. No claims from search results are cited.

**Proposal / inference from the supplied Sherlock brief:** The smallest useful isolation is a repo-local command layer with named, independently invocable checks: a cheap Rust/reference and static QA target as the ordinary default, and explicit opt-in targets for Lean/Clean, Quint, SDK/model/degree, and native proof operations. Keep each target’s declared inputs and outputs separate, and avoid having cheap checks silently launch heavyweight targets. Record exact command and result per target so callers can select a bounded check without implying that broader obligations passed. This is a structural convention, not a scheduler or a claim of hermeticity.

**Limitation:** Separate command names alone do not make a task hermetic or reproducible; the brief provides no source establishing isolation guarantees, and heavyweight targets may share tools or generated state.
