# Muse documentation-delta review — core source provenance (nonsecurity)

Reviewer: fresh independent Muse Wonder 1.0, high effort, non-author.
Scope: NONSECURITY documentation delta only. All security judgments are
Opus 5.5-owned and are not made, repeated as approval, or substituted here.

Candidate: `b5619dc57e75ed2ce60ee56207ca9a2cd5f26000`
Parent: `47cbd1ce69669320d465456695af257c3e9a2a8a`
Worktree: `/home/charl/Moriarty/.worktrees/core-source-provenance-20261015`

## Public verdict: APPROVE NARROW SUCCESSOR

This approves only the documentation successor to `47cbd1ce`: correction of
the stale unexecuted-test status, removal of one duplicate roadmap
paragraph, retention of the initial reviews/receipts, and the separated
current/original scope pins. It is not a new source, runtime, whole-grammar,
native, semantic, financial, or security certificate.

## Review chain and premise

My preserved initial review
(`docs/audits/2026-10-15/core-source-provenance/MUSE-FINAL-REVIEW.md`,
SHA-256 `3ff3285e…f29`, matching its terminal receipt) returned NEEDS
CHANGES on frozen `47cbd1ce` for exactly one mandatory issue, M1:
`docs/NATIVE-CORE.md` stated local checks "have not yet run" while the
dossier recorded ten passed selected tests. No mandatory nonsecurity
functional defect was found then. The initial Opus review (preserved file
SHA-256 `e99f1033…d0`, matching its terminal receipt) approved the narrow
scope with the same documentation correction as D1; its security judgments
remain Opus-owned.

This delta review takes that prior full source/actual-result audit as an
explicit premise. It is not blind approval of an unseen successor: I
reviewed the full `47cbd1ce..b5619dc5` diff (11 files, docs/dossier only)
and recomputed the bindings below read-only, which prove the audited code
is byte-identical in the successor. Later append-only review receipts do
not change the product.

## M1 resolved

The stale sentence is gone. `docs/NATIVE-CORE.md` now links the
2026-10-15 dossier, records "ten passing focused checks, including public
export/reopen and actual IR field positions," and retains "close no
U0/U2 or other parent exit." This matches the dossier README and the
four captures. The duplicate `ROADMAP.md` provenance paragraph is removed;
one provenance link/heading remains. The dossier README reconciliation
accurately states both initial reviews bound `47cbd1ce`, that fresh
documentation-delta reviews were pending at commit time (this is one),
and that no tests were rerun. No misclaim, oracle change, result tuning,
or history rewrite was found: `47cbd1ce` is the direct parent, history is
one append-only commit, captures/oracle/expectations are untouched.

## Pins: what still binds, and what does not

Verified by read-only git/Python SHA-256 and gzip recomputation only:

- No `.rs`, test, `Cargo.toml`, or `Cargo.lock` change in
  `47cbd1ce..HEAD` (`git diff --exit-code` over those paths is 0). The six
  code/test pins are unchanged and recompute equal in both scope files.
- Current `scope.json` source projection recomputes fully, including the
  new guide hash `docs/NATIVE-CORE.md` `14efb29a…`. The preserved
  `scope-47cbd1ce.json` is byte-equal to the parent's `scope.json`, and
  current `original_tested_source_projection` equals it exactly. The only
  current-versus-original projection difference is the guide hash
  (`544064b0…` tested, `14efb29a…` current), separately preserved as
  required, with `documentation_only_post_test_changes` naming both edits.
- I do **not** claim all old start pins match the new guide. In each of
  the four `actual/*/start.json` files, 9 of 10 source pins recompute
  against the current tree; the sole mismatch is `docs/NATIVE-CORE.md`
  (`544064b0…` at capture, `14efb29a…` now), which is exactly the intended
  documentation delta and binds the captures to the original tested tree.
  Runtime/oracle pins are unchanged: baseline `89ae66c4…eff28e`,
  expectation `33e37dc7…`, and all gzip streams decompress to the recorded
  `result.json`/`ACTUAL-CHECK-SUMMARY.json`/`scope.json` hashes (10 tests).
- `DOC-DELTA-CLOSURE-CHECK.json` was not trusted, only checked: its 61-file
  closure list equals, in order, the `include_str!` list in
  `proving::source_closure()` at HEAD; neither `docs/NATIVE-CORE.md` nor
  `ROADMAP.md` is in that closure, so the doc edits cannot churn compiler
  or closure hashes; `tests_repeated: false` matches the diff.

## Limits and exclusions

I ran no cargo, product, replay, SDK, proof, native, or execution command;
verification was read-only file/git/Python JSON/hash/gzip inspection.
The old source/control/actual-result evidence is unchanged evidence for
the unchanged code — not a whole-grammar certificate (one five-statement
specimen, Variable-only replacements, as previously disclosed). Native
authority remains zero; all U0–U7, M1–M17, and MC/SP/G parent exits remain
open. Security disposition of this successor, including whether the doc
delta affects any Opus-owned judgment, awaits the separate Opus
documentation-delta review; this Muse approval does not supply it.
