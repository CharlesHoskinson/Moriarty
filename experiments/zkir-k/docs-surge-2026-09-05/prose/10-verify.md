# Preservation check of 10-well-formedness-and-static-checks.md

Verdict: PRESERVED

The diff touches twelve prose sentences in eight paragraphs. No table row, code block, link target, inline code span, number, path, symbol, command or message changed. Every predicate in the `wf` table, the `#checkArity` table, the negative-program table and the crate-comparison table is byte-identical to the original.

## Meaning changes

### C1 minor "The corpus check"
Before: Programs whose `version.major` is not 3 are skipped. Every file is loaded on the base surface, so `corpus/midnight-zkir-2ffe2d1-tests` is not among its corpora.
After: It skips programs whose `version.major` is not 3 and loads every file on the base surface, which is why `corpus/midnight-zkir-2ffe2d1-tests` is not among its corpora.
What changed: The original attributes the exclusion of the midnight-zkir tests to the base-surface load alone. The merged sentence lets "which is why" attach to both clauses, so a reader may take the major-version skip as a second cause. No fact, number or path differs.
Fix: It skips programs whose `version.major` is not 3. It loads every file on the base surface, which is why `corpus/midnight-zkir-2ffe2d1-tests` is not among its corpora.

### C2 minor "Why single assignment matters"
Before: Single assignment is the condition under which evaluation over the final memory coincides with evaluation at each instruction; `#wfWrites` enforces that condition, no more and no less.
After: Single assignment is the condition under which evaluation over the final memory coincides with evaluation at each instruction, and `#wfWrites` enforces that condition alone.
What changed: "no more and no less" states two things: the predicate enforces exactly that condition and nothing weaker. "alone" keeps the first and can also be read as "only `#wfWrites` enforces it". The next sentence ("It does not inspect values, so writing the same value twice is rejected as well") still carries the "no less" half, so no claim is lost.
Fix: ...evaluation at each instruction, and `#wfWrites` enforces that condition, no more and no less.

All other edits are rewordings with identical meaning: active voice for `#wfInstrs` (line 17), "The `N` in these rules" (line 24), semicolons split into sentences (lines 28, 86, 104, 124, 141), the `encode` clause reordered (line 54), "A file cannot produce two of the `wf` failures" (line 76), "Of the 63 programs, 43 are..." with the same counts (line 86), and "The VM evaluates the verdicts" (line 143), which names the agent the preceding paragraph already names.

## Dropped material

None. Every claim, sentence and list item of the original is present after the edit. The "Must cover" items of briefs/10.md (the `wf` parts as predicates, `reads`/`writes`, `#checkArity`, ZKIR-CHECK and the `check` command, `check_corpus.py` and the manifest note, static check against the crate with `job`/`checkedJob`, the run-time checks that remain, the negative-program table, single assignment and gate evaluation) are all intact.

## Rule violations

None. No em or en dash, no placeholder, no process word, exactly one `#` title. Code blocks, tables, link targets and inline code are unchanged.

## Residual machine tells

None found. The one sentence that could be tightened is "The columns differ in more than wording" (line 124), unchanged from the original and acceptable as a lead-in.
