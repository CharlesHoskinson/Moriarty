# Preservation check of 06-configuration-and-run-lifecycle.md

Verdict: PRESERVED

## Meaning changes

None. Every hunk in the unified diff is a rewording with identical meaning. The hunks, for the record:

- Configuration, line 7: "Initial values precede entry-point execution." became "The initial content is what each cell holds before the entry point runs." Same claim.
- Entry points, line 46: "and the following job is discarded, so input loading ... never begin;" became "and discards the following job. Input loading ... then never begin;". Same claim, same error message, same cell consequences.
- Entry points, line 50: "`tools/zkir_run.py`, `preimage_term`, converts" became "`preimage_term` in `tools/zkir_run.py` converts". Same file, same symbol.
- Entry points, line 58: "sequentially" became "in order".
- Entry points, line 60: semicolon split into two sentences before "That gate later evaluates to `unknown(...)`". Same message, same reason.
- Sequencing, line 73: "prevents job initialization altogether" became "prevents job initialization".
- Operand resolution, line 85: "An active impact uses `#impactPush` and `#impactOne` to append native values and advance `<pubInIdx>` one at a time, then appends `skipNone()` and uses `#impactCheck` to compare" became "An active impact appends native values through `#impactPush` and `#impactOne`, which advance `<pubInIdx>` one at a time, then appends `skipNone()` and compares the pushed values with the expected transcript in `#impactCheck`". Same helpers, same order, same cursor behaviour; confirmed against `zkir-vm.k` lines 472 to 491.
- Completion, line 89: "; `zkir-hash.k`, `transientCommit`, hashes" became ". `transientCommit` in `zkir-hash.k` hashes".
- Completion, line 91: "The later `#verdicts` invokes `zkir-constraints.k`, `verdicts`, using" became "`#verdicts` runs after `#finish` and invokes `verdicts` in `zkir-constraints.k` with". "Later" is made explicit as "after `#finish`", which the chapter already states at line 32 (`#finish ~> #verdicts`) and in the job schedule at line 55. No new claim.
- Completion, line 93: "simply disappears" became "disappears"; "the single-assignment check in `zkir-syntax.k`, `wf`," became "the single-assignment check of `wf` in `zkir-syntax.k`".
- Completion, line 95: two sentences about `needPubOut`/`needPriv` and `needPubIn` joined with "and an active impact records". Same records.
- Completion, line 97: `build_preimage` sentence split into two; "; apply the `needs` list" became ". Apply the `needs` list". Same steps, same bounded-loop reason, same warning.
- Runner statuses, line 101: "`tools/zkir_run.py`, `Runner.run`, checks" became "`Runner.run` in `tools/zkir_run.py` checks".
- Runner statuses, line 111: "reporting the full term" became "and reports the full term"; "from genuine stuckness" became "from a run that is stuck". Same `k_cell`, same 300-character truncation, same non-distinction.
- Runner statuses, line 119: "prints `format error: ...`; a preimage integer" became "prints `format error: ...`, and a preimage integer". Same exit code, same messages.

## Dropped material

- "altogether" (line 73), "simply" (line 93), "genuine" (line 111): intensifiers only. Harmless cut.

No sentence, list item or claim was dropped. Every item under "Must cover" in `briefs/06.md` is present after the edit exactly as before: the cell table, the three entry points and `#wfGate`, the `<status> ok()` gating and post-failure behaviour, the lifecycle helpers (`#loadInputs`, `#put`, `#seedPi`, `bindGate`, `commGate`, `resolveNatives`, `#finish`, `#verdicts`), generation mode with `<genMode>` and `<needs>`, the five runner statuses with the stuck residual, and the traced three-instruction example.

## Rule violations

- Em or en dashes: none.
- Placeholders or process words: none.
- `#` titles: exactly one.
- Code blocks: byte-identical to the original.
- Tables: byte-identical to the original.
- Link targets: identical to the original.
- Inline code: one added occurrence of `#finish` (line 91, "runs after `#finish`"), a symbol already used elsewhere in the chapter; no inline code was altered or removed. Not a violation.

## Residual machine tells

All of the following are unchanged from the original; none were introduced by the prose pass.

- Line 32: "emptiness says that processing ends, not that the witness succeeds." Negative parallelism ("X, not Y").
- Line 73: "a cell-pattern condition rather than a global restriction on every rule." Contrastive "rather than" hedge.
- Line 81: "These are explicit sequencing helpers, not an instruction-level `strict` attribute." Negative parallelism.
- Line 93: "Gates retain operands and register names, not snapshots of intermediate memory". Negative parallelism.
- Line 111: "`depth-exhausted` records that a bound was given, not that it was reached". Negative parallelism.
- Line 155: "The milestones below group helper rewrites, not individual backend steps." Negative parallelism.

Six "X, not Y" constructions in one chapter is a recognisable pattern, but each carries a real distinction and none changes meaning.
