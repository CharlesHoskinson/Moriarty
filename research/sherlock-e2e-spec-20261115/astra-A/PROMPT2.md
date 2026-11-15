# Astra lane A, phase 2: adversarial review

Your DRAFT.md is done. Now review the Fable-A lead's work against your draft and the sources. Same rules as phase 1: write only inside `research/sherlock-e2e-spec-20261115/astra-A/`; no git; no network; no builds; read and `rg` only. You may now read `openspec/changes/sherlock-e2e-prototype-20261115/` and `research/sherlock-e2e-spec-20261115/bus/`.

Review these files:
- `openspec/changes/sherlock-e2e-prototype-20261115/proposal.md`
- `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-{compact-compiler,rsl-statement,rsl-kits,universal-interpreter,compact-suite,user-flow}/spec.md` (prefixes ECC, ERS, EKT, EUI, ECS, EUF)
- `openspec/changes/sherlock-e2e-prototype-20261115/architecture/decisions/D-001.md` to `D-027.md`
- if present: `architecture/DDD.md`, `architecture/decisions/INDEX.md`, `architecture/COVERAGE.md`

The lead already took these findings from your draft: ERS-012 (FP-1 binding gap), ERS-013 (two call formats), ERS-014 (unchecked JSON), ERS-015 (MYC-REF-052 conflict), EUI-010 (5 + 2 rules of 120), ECC-014 (Jubjub inputs), ECS-012 (vacuous zero mismatch), ECS-013 (call identity). Check that each is stated correctly; do not repeat them as new findings.

Write `research/sherlock-e2e-spec-20261115/astra-A/REVIEW.md` with numbered findings `R-NN`, each with: severity (HIGH, MEDIUM, LOW), the file and requirement or decision ID, the evidence (file:line or commit), and the exact change you ask for. Cover these classes:
1. Components of slice A in your inventory that no ECC, ERS, EKT, EUI, ECS or EUF requirement or cited ID covers.
2. Claims that the code or the current rulings contradict (wrong file:line, wrong status word, wrong count, stale ruling).
3. EARS requirements that are not testable, have two obligations, name the wrong system, or whose Verification names a check that cannot show the requirement.
4. Decisions with wrong or missing authority (OWNER against LEAD PICK confirmed against LEAD PICK for T15), missing rejected options, or confirmation that does not bind.
5. Requirements that duplicate an existing ID (MYC, CF, RSL, STL, ANC, ZK, LPC) and must be a reference.
6. Hard-rule violations or text that narrows GP-1 to GP-4.
End with a table: finding, severity, one-line fix. Then reply with a summary of 10 lines or fewer.
