# Astra lane B, phase 2: adversarial review of the slice B specs

Your phase 1 draft is `research/sherlock-e2e-spec-20261115/astra-B/DRAFT.md`. Fable-B has now written the slice B capability specs. Review them against your draft and against the sources. Be adversarial: the aim is to find what is wrong or missing, not to confirm.

## Rules (same as phase 1)

- Write only inside `research/sherlock-e2e-spec-20261115/astra-B/`. No git commands. No network. No builds, provers or tests. Read files and use `rg` only.
- Every finding names the spec file and requirement ID, and the `file:line` or commit of the source that shows the problem. Simple English. No hedges.

## What to review

The 13 specs under `openspec/changes/sherlock-e2e-prototype-20261115/specs/`:
`sherlock-e2e-leaf-proving` (LFP), `sherlock-e2e-l-transcript` (LTR), `sherlock-e2e-universal-keys` (UKS), `sherlock-e2e-node-aggregation` (NAG), `sherlock-e2e-gp-hooks` (GPH), `sherlock-e2e-call-binding` (CLB), `sherlock-e2e-elastic-proving` (ELA), `sherlock-e2e-setup-mode` (STM), `sherlock-e2e-box-fleet` (BXF), `sherlock-e2e-gpu-proving` (GPV), `sherlock-e2e-prover-daemon` (PDM), `sherlock-e2e-zero-knowledge` (ZKE), `sherlock-e2e-stark-security` (SSG).

Each requirement has the EARS sentence, scenarios, then `Verification:`, `Evidence:` and `Status:` lines. The specs reuse older requirement IDs by reference and do not restate them (the reuse lists are in `research/sherlock-e2e-spec-20261115/bus/design-B.md` if that file exists). Also read `research/sherlock-e2e-spec-20261115/bus/CLAIMS.md` and, if they exist, `bus/ddd-B.md` and `bus/decisions-B.md`. Do not review the specs of slices A and C.

## Output: `research/sherlock-e2e-spec-20261115/astra-B/REVIEW.md`

A numbered list of findings `R-01`, `R-02`, ... Each finding has: severity (BLOCKER, MAJOR, MINOR), class, the requirement ID or "missing", the claim, the evidence (`file:line`), and the fix you propose. Classes:

1. **Missed component**: a slice B component, LOOP row, hard rule or item of your draft that no slice B requirement and no reused ID covers. Check every hard rule of BRIEF.md that touches slice B, and rows T3, T6, T7, T7c, T8, T8b, T10q, T11, T12, T13, T14, T14b, T16, T24 (leaf side), T25.
2. **Wrong claim against code or ruling**: open the cited file and line and check it. Check at least 40 citations, spread over all 13 specs, and list which ones you checked. Report a wrong path, a wrong line, a wrong constant, a test name that does not exist, a status word that the source contradicts, a ruling quoted with the wrong authority.
3. **Untestable or weak EARS**: a requirement with no measurable response, a verification that does not check the requirement, a scenario that would pass on a broken system, two obligations in one sentence.
4. **Decision without authority**: a design choice presented as settled that has no owner ruling and no recorded lead pick.
5. **Duplicate or conflict**: a requirement that restates or contradicts a reused ID (name both), or conflicts with a slice A or C claim in `bus/CLAIMS.md`.
6. **Better in your draft**: a requirement of your DRAFT.md that is stronger or more exact than the Fable-B one; quote yours.

End with a table: per spec, the count of findings by severity, and one line "verdict" (ACCEPT, ACCEPT WITH FIXES, REWORK).

Your final message: a 10-line summary of REVIEW.md.
