# Preservation check of 01-overview.md

Verdict: PRESERVED

## Meaning changes

None blocking or major. Every changed hunk is a rewording with identical meaning: passive to active voice (twice), a semicolon replaced by "and" or a full stop (four times), one paragraph split in two (the runner description), the four "does not establish" clauses split into three sentences with all four items kept, and the closing sentence reordered so the two established facts follow the qualifying clause instead of preceding it.

### C1 minor What the definition is
Before: so a program can be run, not only read.
After: so a program can be run.
What changed: the contrast "not only read" is dropped. No fact changes; the sentence still states that the definition is executable.
Fix: so a program can be run, not only read.

## Dropped material

- "not only read" (first paragraph): harmless cut, see C1.

Nothing else is absent. Every claim, number, path, symbol, command, message and chapter reference in the original is present in the edited version. The four items the trust statement rules out (no other witness, hash gadget fully constrained, canonical `JubjubScalar`, keygen and proving) are all kept, each with its reason.

## Rule violations

None.

- No em or en dash in the file.
- No placeholder and no process word (draft, review, audit, persona, agent, editor, "this document").
- Exactly one `#` title; all `##` and `###` headings identical to the original.
- Tables (surface table, directory map, module table, checking table, reading guide) byte-identical to the original.
- The three code blocks (pipeline picture, `checkedJob` rule, two shell commands) byte-identical to the original.
- The multiset of inline code spans is identical to the original.
- Chapter references (01 to 15) identical to the original.

## Residual machine tells

- "such a run is a failure, never a result." (pipeline section, unchanged from the original): negative parallelism used for emphasis.
- "Those relations are not the circuit." (trust statement, unchanged from the original): short declarative sentence placed for dramatic effect before a list of caveats.
- "Two things are established only for the program and preimage pairs the differential harness compared, at the two pinned commits, and only on the observables it compares: that the witness is the one `preprocess` computes, and that a failing run fails with the same status and error class." (closing paragraph, new wording): a counted-item sentence with a long qualifying clause before the colon; the meaning is intact but the construction is stiff.
