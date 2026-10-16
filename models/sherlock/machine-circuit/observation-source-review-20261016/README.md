# Sherlock observation adapter source review

This source adds canonical boundary projection and complete record/trace
materialization to the internal L4 codec. The adapter calls the real Step
decoder and canonical encoder and uses the existing full trace comparator.
It retains every Snapshot field, the full Program including unused functions,
the supplied initial image and all ordered records. A supplied zero-step trace
remains present. Rule source lines and wire enum tags remain distinct.

The module stays private under `cfg(test)`, with trusted retained PreparedA as
its program context. TraceIdentity is caller-declared comparison metadata.
The adapter checks the O1-O4 observation domain. It does not check continuity,
reachability, rule execution, fuel deltas, circuit acceptance or proof validity.

Four new normal test functions bring the complete codec source suite to 29.
Compilation and all 29 tests are **UNRUN**. The independent expectations are
hand-derived data, not evaluation or test output. No SDK, circuit, recursive
proof, ledger or Midnight stagenet result is claimed.

`SOURCE-FREEZE.json` binds all 18 source files, the four source changes, the
independent expectations and both full current-source review receipts.
The unchanged normative L4 specification and its prior host-only adoption
remain in `openspec/changes/sherlock-machine-circuit-layout/` and the sibling
`codec-source-review-20261016/` records. Prior receipts retain their old scope.
Gemini 4 Argon high via AGY supplied the temporary security-owner source review
under the user's Opus outage fallback. AGY reports its selected model in init;
it does not report a separate canonical model identity or returned effort.
Muse Wonder1.0 high supplied the fresh non-author normal source review.

The `*.md.gz` receipts preserve the exact UTF8 text from canonical result JSON;
the `*.json.gz` receipts preserve exact adapter result bytes. Each gzip is one
member with mtime zero. No text normalization or source rewriting is applied.
Source review does not grant execution. F4, P1-P6, G4, Candidate B, general
ADTs, financial coverage and all remaining roadmap acceptance requirements
remain open. Publication review records, when sealed, document publication
of these source bytes and carry no build or target acceptance.
