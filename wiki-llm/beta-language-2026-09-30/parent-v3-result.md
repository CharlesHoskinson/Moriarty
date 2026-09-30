# Parent beta DevEx repairs for code v3

Repository/experiment observation, 2026-09-30. Original full-candidate v2 findings remain in code-astra-v2.md and code-grok-v2.md. Companion owned source repairs are in code-repair-v3.md. This report establishes no independent approval.

- Astra L01: inbound LSP positions and the lexical comment scanner now share CR/LF/CRLF boundaries with outgoing UTF16 positions and frontend comments. Actual subprocess navigation/comment completion discriminator was RED before repair and GREEN afterward.
- Grok L02: `check` now prints readable status/action-support and located diagnostics by default; explicit `--json` retains the machine report. Actual CLI discriminator was RED before repair and GREEN afterward. No effect/preparation semantics changed.
- Grok L03: oversized Full generations explicitly mark editor results unavailable. They retain the version and resource diagnostics, discard oversized text, and answer navigation/hover/completion/format with empty results before parsing positions. A later valid Full restores results. Keeping oversized text or treating previous text as authoritative would break resource/staleness guarantees; those suggestions were not adopted. Existing stale-version and aggregate/count gates remain unchanged.
- Grok L05: engineering progress boxes now describe implemented bounded authoring/S0 bridge/tooling; horizon concurrence, final review and publication remain outstanding.
- Astra manifest wording: successor freeze states that entries are sorted lexicographically by path before constructing the UTF8 sha256-two-spaces-path-LF lines. Original v2 wording and both audit findings remain intact.

Focused receipts outside the repository: cli-output-red.txt/cli-output-green.txt; services-v3-red.txt/services-v3-green.txt; typecheck-code-v3.txt. Full candidate test and review evidence is recorded separately. Current strict typecheck passed. All Source/6–Core/5 semantics remain unmodified and financial outputs unqualified.
