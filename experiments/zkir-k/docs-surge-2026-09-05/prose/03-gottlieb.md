# Gottlieb edit: 03-program-model.md

Paragraph numbers count prose paragraphs only (tables and code blocks skipped), in order from the title.

## Critical findings

1. **Machinery in the prose (locator apposition).** Paragraphs 9, 13, 14, 17 and 18 name a location by stacking comma-separated code spans: "come from `zkir-syntax.k`, `IrType` and `encodedLen`" (p9), "Pinned `ir.rs`, `Operand::deserialize`, uses" (p13), "distinct in `zkir-syntax.k`, `Guard` and `Instr`" (p14), "follow `zkir-syntax.k`, `Instr`, `reads` and `writes`, and Python `instruction`" (p17), "skips its inputs in `zkir-vm.k`, `#impact`" (p18). Each reads as a list of three peers when it means one symbol inside one file. Fix: rewrite as "`X` in `file`" or "`file` (`X` and `Y`)"; keep every name.

2. **The "not X but Y" reflex.** Eleven contrastive tails in 2,400 words: "not bytes" (p9), "not the reverse" (p6), "not JSON formatting" (p8), "not to loading" (p4), "rejected, not reduced" (p11), "not the spelling" (p13), "not a guarantee" (p18), "not whether" (p16), "not transcript or return-value effects" (p17), "not message text" (p22), "not input syntax" (p25). Fix: cut "not the reverse" (p6; the example already shows the head, the tail adds no claim); recast p8, p14/p16 and p4 in positive form; keep the ones that guard against a real misreading (bytes vs elements, rejected vs reduced, value vs spelling, inventory vs guarantee, status vs text).

3. **Semicolon chains standing in for sentence shape.** Twenty-four semicolons; p3, p14, p17, p18, p22, p26 and p28 each end with "; see NN-..." or glue two unrelated facts with a semicolon. The result is one uniform mid-length cadence across the chapter (mean 31 words, no paragraph varying its shape). Fix: give each cross-reference its own clause or a parenthesis, join the pairs that are genuinely one thought ("Both default to ...; `kast` needs no ..." becomes "while"), and let a short sentence stand where it lands a point ("The accepted spellings differ in one case.").

## Important findings

4. **Docent's elbow at table heads.** "The following 13 base types" (p9), "These 34 rows follow" (p17), "The following message templates" (p19), "These commands use existing corpus artifacts" (p26). Each is needed once to tie the table to its sources; the pointing words are not. Fix: "The 13 base types in the table below", "The 34 rows below", "The templates in the table"; leave p26.

5. **Assertion by adverb.** "come directly from its builders" (p19), "additionally accepts" (p10), "have a direct counterpart ... with the same message content" (p21). The specific claim ("same message content", "quoted from") already certifies; the adverb does not. Fix: cut "directly" and "direct", replace "additionally" with "also".

6. **Ambiguous pronouns.** "Its arity is checked" (p18; the antecedent is the outputs list two clauses back), "Its `load_constant` branch" (p20; the antecedent is a flag), "its message" (p19). Fix: name the noun.

7. **Run of short declaratives as default register.** p15 ("Python `alignment` builds ... Each `Segment` is ... Atoms are ... An option carries ...") is four sentences of the same shape in a row where the first two are one thought. Fix: subordinate the second to the first.

8. **Subject buried under a long list.** p21's last sentence stacks three subjects (one with a parenthesis) before the verb "escape". Fix: put the verb first and list after the colon.

## Notes on voice and cadence

The concrete examples are the chapter's strength and need no touch: "`0x0100` denotes 1 and `0x0001` denotes 256" (p11), the `"0x0x01"` divergence with its const-hex version pinned (p13), the `0x0\n` escape traced to `$` and `bytes.fromhex` (p12), and "still exits zero, so callers must inspect the result" (p27), which lands the one warning a reader of this section needs. "Oversized positive values are rejected, not reduced. Negation applies only after validation" (p11) is the right use of two short sentences at a boundary that matters. The rejection table and the 34-row constructor table carry facts that would be unreadable as prose; the surrounding paragraphs are short, which is correct for reference text.

## Verdict

This is sound reference documentation whose sentences were assembled from locators and contrasts rather than written: the facts are all present and correctly placed, and the reader is slowed by apposition lists that look like enumerations, by a contrastive tail on every third sentence, and by semicolons doing the work of sentence boundaries. The fix is rewriting at clause level, not deletion; almost nothing should be cut except "not the reverse" and three adverbs. Every number, path, symbol, message string and command stays exactly where it is.
