Reviewing all three documents as the stated audience, without tools.

**Verdict: CHANGES_REQUESTED** (reader: FLAGS)

Cross-checks that pass: requirement counts match the table of contents (35/16/8); the Transfer-only and repayment control-step counts (18 and 37) agree with the guard table and contraction list; the ProRata example arithmetic and the obligation update equations are consistent; grammar and prose bounds agree.

**Important findings**

1. **language.html, EBNF notation table, alternatives row.** The cell reads `\|`, a leaked markdown escape. Fix: `|`.

2. **language.html, EBNF header comment, "Simultaneous bounds" lines.** Spaces are missing between names and numbers ("source65536 bytes, tokens8192, AST8192, depth64 ..."). The block claims to be typeset from the canonical file, so it must match. Fix: restore the spaces, or correct the canonical file if it is the source of the defect.

3. **language.html, "Typing judgment" paragraph.** Γ is defined as L ⊎ A ⊎ O ⊎ F, and READ-PRE uses F[f], but F is never introduced. The frame σ lists pre, W, L, A, O, D only. Fix: add one clause, e.g. "and F the declared state-field types, consulted through the pre and post views".

4. **language.html, "Current lifecycle boundary" third paragraph and the paragraph ending "Midnight acceptance".** "SP02/SP03" are used without definition. Fix: gloss on first use (the sprint contracts they name) and link to the retained deliverable, as is done for the SP02 runtime later.

5. **README, "The Federated DeFi Kernel", second paragraph.** "OWS" is never expanded and "x402" is not glossed. Fix: expand OWS on first use and add a short gloss for x402 (the HTTP 402 payment protocol), consistent with MPLR-033.

6. **requirements.html, #delivery.** "The inherited sprint contracts retain their acceptance obligations" refers to contracts the document never names. Fix: one clause identifying them, or a link to the source snapshot where they are recorded.

7. **language.html, worked-example paragraph.** The "Presentation bound." run-in heading sits inside the same `<p>` as the ProRata example, so the two topics render as one paragraph. Fix: close the paragraph before `<strong>Presentation bound.</strong>`.

**Optional preferences (not blocking)**

- language.html: "Current lifecycle boundary" is an `h2`, so the small-step and expression `h3` sections nest under it rather than under "Operational semantics". Demote it to `h3`. The TOC label "Formal semantics" also differs from the heading "Operational semantics".
- language.html: the same profile is named "successor-syntax/0", "syntax/0" and "moriarty-successor-syntax/0" in different places. Pick one.
- language.html: "PCD" is unexpanded here, though the README expands it.
- requirements.html: MPLR-010 and MPLR-011 titles read as lists without commas ("Time finality and unresolved outcomes", "Causality concurrency and interference"). Add commas if the titles are lists.
- README, "Where the project stands": "scoped Midnight Preview financial results" alongside language.html's "Preview financial settlement remain open" is consistent but easy to misread. Consider "scoped Preview evidence" to avoid implying settlement.