**Verdict: APPROVED (reader: FLAGS)**

I read all three documents as a PL/DeFi developer. The cross-document claim status is consistent: ZKIRv3 as current target, ZKIRv4 as a proposed workstream, March 2027 as a planning assumption, and K/Preview results as scoped. The grammar, the repayment reduction system and the expression rules are internally consistent. I checked the step counts (18 = START, EXPAND, 15 guards, PREPARE-T; 37 = 8 contractions, 28 guards, PREPARE-R), the guard counts against the ordered-checks table, the DP helper against the worked ProRata example, and the obligation equations. All agree. Nothing blocks approval.

**Reader flags**

1. "Both Core contract documents are proposals with scoped independent design approval and remain unregistered" (language.html, first paragraph of "Expression small-step semantics") — "unregistered" is never explained, and the only registry mentioned elsewhere is the project registry that MPLR-015 says must not gate programs. I had to guess whether this is a status label for contract documents or the same registry — severity: medium
2. "Bounded fan-in permits growing finite histories, provided their authorized episode and lifetime budgets remain intact" (requirements.html, ZR09) — "episode" is not introduced anywhere; MPLR-009 speaks only of signed cumulative bounds. I had to infer that episode means one workflow run — severity: medium

**Findings with minimal corrections**

Important (recommended before publication):

- **language.html, "Expression small-step semantics", para 1.** Define "unregistered" once, e.g. append "in the project's contract registry" or replace with "not yet registered as a versioned contract". Same fix applies to "unregistered composition proposal" in the "Funded composition" paragraph.
- **requirements.html, ZR09.** Replace "authorized episode and lifetime budgets" with "authorized per-run and lifetime budgets", or add a parenthetical defining episode where it first appears.

Optional (preferences, not defects):

- **language.html, "Statements" paragraph.** The E-CONTEXT rule closes only expression configurations ⟨e, w⟩. There is no displayed rule lifting an expression step into the statement head ⟨s°(C[e]); S, σ, w⟩. The prose sentence "A statement's expression operand reduces by the E-rules under the current σ" covers it informally; one displayed lifting rule would make the system self-contained.
- **language.html, "Work" paragraph.** C is displayed only for And and Or. The generic case stated in prose could be displayed as one line: C(K(e₁,…,eₙ)) = 1 + Σᵢ C(eᵢ) over evaluated operands, K ∉ {And, Or}.
- **language.html, grammar header comment.** "financialPostRead is legal only in ensures" does not say whether this is a parse or static rule. The primary production admits it anywhere and the typing text later rejects TYPE_POST_SCOPE, so it is static. Adding "static" to that comment line would remove the ambiguity.
- **requirements.html, MPLR-014.** "Under ZR06 and ZR11" forward-references identifiers defined two sections later. Anchoring them as links would help the reader who arrives via the table of contents.
- **README.md, "Where the project stands".** "K definitions" assumes the reader knows the K Framework. A single expansion on first use, "K Framework definitions", would suffice.
- **language.html, "Syntax and source profiles", para 1.** The contract family for financial-expression-source/1 is not named, unlike the other three profiles listed. Adding it would complete the version-mapping sentence.