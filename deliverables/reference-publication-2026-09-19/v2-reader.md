**Verdict: CHANGES_REQUESTED (reader: FLAGS)**

Equations, guard counts, step counts and the worked example in language.html check out internally. The 35/16/8 counts match the sections. All flags below are medium; none is high. Claim status is preserved throughout and should not be touched.

## Findings (medium, should fix)

1. **README, "Where the project stands", and requirements.html, "Recursion workstream" second paragraph** — "a user-supplied planning assumption dated September 19, 2026". A developer reading a public README does not know who "the user" is, and the phrase reads as a leftover from a drafting tool. Minimal correction: "a project planning assumption recorded on September 19, 2026" in both places.

2. **requirements.html, "Proposed ZKIRv4 contract" intro paragraph** — "they introduce neither a Lean dependency nor a replacement with Mina cryptography." Neither Lean nor Mina has been introduced, so the reader cannot tell what alternative is being ruled out. The later "Mina-derived refinements" and MNR01 supply half the context but too late. Minimal correction: one clause of motivation, e.g. "...neither a Lean dependency nor a replacement of Midnight's proof system with Mina's; the MNR refinements below borrow Mina's recursion lessons, not its cryptography." The bare "Moriarty has no Lean dependency" sentences in README ("Design choices") and language.html ("Operational semantics" first paragraph) have the same problem; either delete them or give them the same one-clause reason.

3. **requirements.html, "Implementation direction"** — "The consolidated U0–U7 sequence" and "The inherited SP01–SP12 sprint contracts". Both label sets are internal and undefined. The U labels are glossed inline and survive; the SP labels are not, and "inherited" from what is unstated. language.html later refers to SP02 and SP03 as if known. Minimal correction: "The project's earlier SP01–SP12 sprint contracts retain..." plus a link to wherever they are recorded, or drop the SP sentence if no public record exists.

4. **language.html, "Funded composition" and "Typed financial reads" paragraphs** — Core versions appear as "Core /2", "Core /4" and "the 40-constructor Core", and source versions as "/3" and "/5", with no stated mapping. The reader has to infer that agreement-source/3 elaborates to Core /2 and agreement-source/5 to Core /4, and cannot tell where the 40-constructor Core sits in that numbering. Minimal correction: add a Core column to the profile table under "Syntax and source profiles", or a single sentence there: "expression-source/1 elaborates to Core /1 (40 constructors); financial-agreement-source/3 to Core /2; financial-agreement-source/5 to Core /4." Use whatever the actual version numbers are; the point is that the mapping be stated once.

5. **language.html, "Terms and frames" paragraph** — "reduce under a fixed frame σ, which reads consult and which no expression rule changes". I had to reread to parse "which reads consult". Minimal correction: "a fixed frame σ that reads consult and that no expression rule changes."

## Optional preferences (not blocking)

- **MPLR-010 and MPLR-011 titles** — "Time finality and unresolved outcomes" and "Causality concurrency and interference" read as compound terms; commas after "Time" and "Causality" would match the three-item pattern of the other titles.
- **language.html intro vs. "Small-step semantics"** — the same K projection is called "historical" in the intro and "provisional" in the semantics section. Pick one.
- **language.html, "Work" block** — C is defined only for And and Or. A general line C(K(e1..en)) = 1 + Σ C(ei) for K ∉ {And, Or} would close the definition without changing any claim.
- **language.html, "Entry and context closure"** — it is not stated whether entering a statement constructor (Require, Let, NextWrite, Emit, Ensure) costs a work unit under E-ENTER or only expression constructors do. The "Successful debit is E + N" accounting later depends on this. One clarifying clause would settle it.
- **ZR11** — "Midnight's guaranteed and fallible phases" is Midnight-specific vocabulary; a five-word gloss would help PL readers who are not Midnight developers.