**Verdict: CHANGES_REQUESTED (reader: FLAGS)**

All flags are medium. No high-severity break: the central argument, the claim status and every equation and step count check out (18 = 1+1+15+1, 37 = 8+28+1, the ProRata example and the p′/a′/o′ update are all arithmetically consistent). The issues below are places where a single pass required rereading or supplying context the text never gave.

**Flags (medium, reading order)**

1. README, "Programs target Midnight's native ZKIRv3…", last sentence "These proof obligations target Midnight's native stack without an additional Lean theorem-prover dependency." — The disclaimer presupposes a prior proposal I never saw. I stopped to wonder why Lean is being ruled out. The same pattern recurs in requirements.html "The next backend contract must express…" (Lean and Mina) and language.html "Static judgments describe admissible programs…" (Lean). Minimal correction: in one place, add a clause such as "unlike earlier project drafts that considered Lean and a Mina-derived backend", and drop the repeats, or drop the disclaimers entirely. — severity: medium

2. requirements.html MPLR-014, "Under ZR06 and ZR11, correct output from an honest witness generator…" — Forward reference to identifiers not yet defined; I had to leave the section to learn what the sentence depends on. Minimal correction: gloss inline, e.g. "Under ZR06 (adversarial-witness soundness) and ZR11 (compiler/effect correspondence) below, …". — severity: medium

3. requirements.html ZR11, "through Midnight's guaranteed and fallible phases" — Midnight-specific transaction terminology used before it is taught; the second sentence depends on it. Minimal correction: one clause defining the phases, e.g. "the guaranteed section that always applies and the fallible business section that may abort". — severity: medium

4. language.html, "The version numbers name separate contracts: expression-source/1 elaborates to…" — Later text says "Core /4", "Core /2", "/3 financial reads" and "Core /4 lifecycle", and I had to reread this paragraph to map each short form back to a full contract name. Minimal correction: a three-row table (source profile → Core contract → short name used below), and use the short names consistently afterwards. — severity: medium

5. language.html, "Syntax acceptance does not imply execution. The bounded funded source profile, moriarty-funded-source/0, uses the older moriarty-successor-syntax/0…" — Together with "syntax/0" in the grammar links and "moriarty-funded-repayment/0" in the semantics, three near-identical /0 names appear, and I lost which was the parser, which the preparation API and which the K projection. Minimal correction: state once that syntax/0 abbreviates successor-syntax/0, and that funded-source/0 (TypeScript preparation API) is distinct from funded-repayment/0 (K projection). — severity: medium

6. language.html, "Both Core contract documents are proposals with scoped independent design approval and remain unregistered" — I could not tell what registration means or with whom; "unregistered composition proposal" later repeats the term. Minimal correction: name the registry once (e.g. "not yet registered in the project's contract registry") or replace with "not yet adopted". — severity: medium

**Optional preferences (not flags)**

- requirements.html MPLR-009, "any fees that the target retains on failure" — "the target" is inferable as the target chain or backend but is not stated.
- language.html intro, "the historical repayment projection" is mentioned before it is introduced in the semantics section; a short parenthetical "(the funded-repayment/0 K definition below)" would help.
- language.html profile table, "financial-agreement-source/1–5 · Successive schema, multiple-action, PRE-read, POST-read and local Originate/Accrue profiles" relies on the reader pairing the five nouns with /1–/5 in order.

Formal grammar, notation table, equations and claim-status wording need no change.