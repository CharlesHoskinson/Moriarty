# Proposed Rust Core/3 public consumers

This D1/D2/D3 source candidate is unadopted and unrun. It adds the existing
`language "moriarty/3";` local staged library profile to public ordinary tools.
It does not change Core/3 grammar, financial semantics or profile admission in
the library. Maintainer design/source review is pending; this document creates
no approval prerequisite for public developers.

- `mori check FILE --json` calls `stage::load_v3`. Its existing validity
  summary adds Core/3 semantics and stage labels. It is not a full Program or
  compiler report.
- `mori inspect FILE` prints every source declaration including duties, every
  action/expression and the complete stage plan. Its enforcement report is null
  because source inspection did not compile a target map.
- `mori simulate FILE --action NAME --scenario FILE` starts the real Core/3
  evaluator; `--resume FILE --event FILE` continues from the supplied actual
  continuation. Complete staged outcome envelopes and existing local labels are
  preserved. Authentication is none; consumption is untracked.
- `mori test DIRECTORY` accepts the additive closed `moriarty-tests/3` manifest.
  It uses only `load_v3`, compares whole expected reports and chains actual
  continuations. `moriarty-tests/2` keeps its exact Core/2 loader and behavior.

A tests/3 manifest has exactly profile and cases. Each case has name, source,
scenario, action, logical_cap, expected and events; logical_cap is a canonical
UInt string or explicit null. Each event has event and expected paths. All keys
are required and unknown fields refuse. Existing case/event/read/export limits
and path resolution remain. Expected reports never become runtime boundaries,
inputs, captures or work counters. The aggregate retains all actual reports
when an expected observation is wrong and exits1 for any mismatch.

Legacy `parse`, `language`, `stage::load`, shared v1 decoding and all `/1` and
`/2` ordinary report shapes remain unchanged. Header selection recognizes an
exact /3 header; unknown/malformed headers preserve existing legacy producer
diagnostics. One exact loader is selected; a failed admission is never retried
under another language. Core/3 is not a Beta/Source6/Core5/Sherlock embedding.

`inspect --target` has no Core/3 enforcement map. Compile, native trace,
proof/key/verification, signed/call and ledger consumers keep their existing
profile boundaries. The actual staged backend loads only /2, derives its
registry from liability statements rather than declared /3 duties, and accepts
only Core/2 contexts/artifacts. Connecting Core/3 requires its designated-owner
registry/context/artifact and downstream contracts followed by real lowering
and target qualification. This ordinary source candidate enables none of them.
A local check or stage result cannot replace the required ZKIRv3 compiler,
native verifier or actual Midnight effects.

The two existing P1/P2 programs contrast non-awaiting and awaiting discharge of
a declared duty. Their independent financial/work freeze stays unchanged and
conditional. Full financial Program/ExprId/wire/hash/pretty-byte expectations
are missing; no implementation output may fill them. The new empty-action
complete report and source/shape literals were manually specified by the source
author after implementation inspection, so they are not independent oracles.
An independent full ordinary report/wire/byte freeze must precede a scored run.

All27 grammar obligations,148 requirements/292 scenarios/20 families and all
U0–U7/M1–M17/MC/SP/G, financial/composition/native/Preview/Fenrir/release exits
remain. No new resource or security decision, native authority or acceptance
claim follows from this proposal.
