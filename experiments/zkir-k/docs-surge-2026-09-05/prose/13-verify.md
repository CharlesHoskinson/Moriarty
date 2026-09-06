# Preservation check of 13-known-divergences.md

Verdict: PRESERVED

## Meaning changes

None. Every change in the diff is a sentence split, a semicolon replaced by a full stop or "and", a clause reordered, or a passive turned active. Checked item by item:

- Introduction: the three relevance classes (soundness, completeness, availability) and their glosses are moved into a second sentence unchanged.
- Reproducing the cases: "It runs `checkedJob` ... is reported as `panic`" becomes "The script runs `checkedJob` ... it reports ... as `panic`"; the count (twenty), the preimage fields (`inputs`, `binding_input` `42`, commitment), the exit code 101, the PASS line format, `20/20 divergence cases behave as expected`, the evidence file name, and the `k02.json` contents are identical.
- K1: "ok on both sides, identical memories, gate violated" becomes three clauses with the same three facts; "the review's findings" becomes "those of the arc-zkir review" (the same document, named as such in the section heading and in the original's `wiki/zkir/zkir-v3-divergence-review.md` reference).
- K2: the `Idx +Int encodedLen(T)` condition is moved to the front of the sentence; the panic message, the source location `zkir-v3/src/ir_vm.rs:379:33`, the rule and the `panic(S)` status are unchanged. The case description keeps the unguarded `private_input`, the empty transcript, panic on both sides, gate `private_input` unknown, availability only, and the `wiki/contradictions.md` disposition note.
- K3: input `[2^248, 0]`, `Bytes<32>`, `reverse_bytes`, panic both sides, gate unknown, availability, candidate robustness finding: all unchanged.
- K4: the two gate checks and `synthErr("chip not initialised for ...")` unchanged; case names, inputs, ok/ok, synthErr, completeness, the finding 13 reference and PR #656/#667 unchanged.
- K5: "so bits 253 and 254 pass" becomes "Bits 253 and 254 therefore pass"; case `k05`, bits 253, inputs 1 and 2, ok/ok, synthErr, completeness, same class as finding 10, report upstream: unchanged.
- K6: "rejects the element cleanly" becomes "rejects the element with an ordinary decode error", which restates the section's own opening ("an ordinary decode error in the 2ffe2d1 crate"); the `k6.json` preimage, both outputs, the `--ext` status and message, the 92e8bdd3 assertion text and the base-only availability note are identical.
- Extension `test_eq`: rules, program, preimage, both commands, both outputs, the `all_verdicts` message and the pin at 92e8bdd3 are identical; "Outside the review" becomes "outside the arc-zkir review".
- Findings 2, 6, 7, 8, 4, 13, 11, 5: each keeps its off-circuit behaviour, in-circuit behaviour, K symbol, case name, inputs, expected off-circuit statuses, gate outcome, relevance class and upstream status (retired / PR #656 / PR #656 and #668 / PR #656 and #667 / midnight-circuits 7.2.4). Finding 7 folds "Completeness. Partially fixed in PR #656" into one clause, "a completeness finding partially fixed in PR #656", with the same content.
- Loader difference: program, preimage, the format error text, exit code 2, the oracle path and `["2"]` result unchanged.

## Dropped material

None. Every sentence and list item of the original has a counterpart in the edited text. Code blocks (five `k`/`sh`/`json` blocks plus the two inline outputs), the "Other cases" table and the summary table are byte-identical.

## Rule violations

None found.

- No em or en dashes in the file.
- No placeholders, no TODO.
- Process words: the word "review" occurs only as part of the name of the pinned upstream document ("the arc-zkir review", `wiki/zkir/zkir-v3-divergence-review.md`), exactly as in the original; there is no mention of drafting, editing, personas, agents or "this document".
- Exactly one `#` title.
- No code block, table, link target or inline code span was altered; the diff touches prose lines only.

## Residual machine tells

None. The edited sentences are short declaratives; the one long sentence (finding 7, "is ok on both sides with the gate synthErr, a completeness finding partially fixed in PR #656; the in-circuit arm remains open") is dense but reads as technical prose, not as a generated pattern.
