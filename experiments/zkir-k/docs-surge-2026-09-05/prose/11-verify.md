# Preservation check of 11-tooling-reference.md

Verdict: PRESERVED

Method: `diff -u` of the original against the chapter (39 changed prose lines, all in paragraphs), then both versions read in full. Mechanical checks: the multiset of inline-code spans differs only by eleven additions, each a tool file name (`zkir_kast.py`, `zkir_run.py`, `diff_test.py`, `divergence_tests.py`, `unit_values.py`, `unit_hash.py`, `check_corpus.py`, `gen_handmade.py`, `gen_constants.py`, `extract_test_inputs.py`, `zkir_values.py`) now used as the subject of the first sentence under its own heading; no span was removed or altered. Code blocks, tables, headings (one `#` title) and link targets are byte-identical. Every flag, exit code, output format, message and file name in the prose is unchanged.

Consequence words added by the pass, each checked against the original:
- "never recompile, so rebuild as in `02-getting-started.md` after a `.k` edit": the original stated both halves with a semicolon; the rebuild is the stated consequence of the tools never recompiling.
- "reads only `cc[0]` and `cc[1]` of the value, so a longer array is accepted and its extra elements are ignored": the original introduced the longer-array behaviour with a colon as the first consequence of reading only two elements.
- "exits 0 for every reported `status`, ..., so callers must read `status`": the original stated "callers must read `status`" after a semicolon following the same exit-0 fact.
- "A non-zero oracle exit is `CalledProcessError`, because the oracle is invoked with `subprocess.run(..., check=True)`": the original had the same causal "so" in the other direction.
- "On `error`/`panic` it also requires matching `err_class`": the original made `status` a requirement in every case and `err_class` an additional one on `error`/`panic`.

## Meaning changes

### C1 minor `divergence_tests.py`
Before: Otherwise the target gate is selected exactly from `all_verdicts`:
After: Otherwise the target gate is selected from `all_verdicts` as follows:
What changed: the word "exactly" was dropped. The exact selection rule is still given in full by the code block that follows, so no fact is lost; the chapter brief's "exact gate selection" item remains covered.
Fix: none required; to restore the emphasis, "Otherwise the target gate is selected from `all_verdicts` by exactly this rule:".

No blocking or major finding.

## Dropped material

- "exactly" in the gate-selection sentence (C1): harmless cut.
- Parentheses "(whitespace condensed here)", "(the harnesses write strings)", "(the oracle receives it through a temporary file that is deleted on close)", "(no oracle)", "(and writes its files)": each was rewritten as a clause or sentence with the same content; nothing dropped.

## Rule violations

None. No em or en dashes, no placeholders, no process words, exactly one `#` title, code blocks, tables, inline code and link targets unaltered.

## Residual machine tells

None found. The repeated openers "The tool depends on ..." and "Exit status is 0 iff ..." are uniform reference-entry phrasing rather than machine tells, and the five-fold "It parses no arguments: every argument, including `--help`, is ignored" template was already present in the original.
