# Preservation check of 02-getting-started.md

Verdict: PRESERVED

The unified diff touches ten prose lines. Every code block (48 fences before and after), both tables, every inline code span, every file path, symbol, number, message and chapter cross-reference are identical in the two versions. The changes are sentence splits, reorderings within a paragraph, and word substitutions with the same meaning.

## Meaning changes

None. The following rewordings were checked and judged identical in meaning:

- Intro: "goes from a clean checkout" / "takes a clean checkout"; "Every command is issued from the repository root" / "Run every command from the repository root".
- Prerequisites: "It is not a default dependency: it lives in ..." split into two sentences with pyk named as the subject.
- Toolchain check: "Run it after a K or pyk upgrade" moved from the end of the paragraph to the front; "It changes into" / "The script changes into".
- Compiling: semicolon replaced by colon before "it runs `checkedJob`".
- First program output: one paragraph split into three; "On the base surface the two names agree (`bytes32V` has variant and type `Bytes32`)" / "The two names agree on the base surface, where `bytes32V` has variant and type `Bytes32`"; `cursors` clause merged with the `pi_skips` sentence with the three cell names kept.
- `--ext`: "(`zkir-vm.k`, the rule for `genJob` sets `<genMode>` to true)" made a separate sentence with the same file and cell.
- Reading a failure: "`reassignment.zkir` is the dual" / "is the opposite case"; "demands that the condition pass `asBool` ..., that is, equal `1`" / "requires the condition to pass `asBool` ..., that is, to equal `1`"; "only requires `C` non-zero" / "requires only that `C` be non-zero".
- Check suites: each sentence fragment given the script name as subject; "Needs the 92e8bdd3 oracle" folded into the same sentence for `unit_hash.py` ("so it needs", justified by "against the oracle" in the same sentence) and `divergence_tests.py` ("It too needs").
- `diff_test.py` flags: semicolon replaced by ", and".

## Dropped material

- "`needs` is filled only by `--gen` (below)." lost the parenthetical "(below)". The `--ext` section that shows `--gen` follows two sections later; the pointer was a convenience, not a claim. Harmless cut.

## Rule violations

None. No em or en dashes; no placeholders; no process words; exactly one `#` title; code blocks, tables, link targets and inline code unchanged.

## Residual machine tells

None found. The rewritten sentences are short, declarative and specific.
