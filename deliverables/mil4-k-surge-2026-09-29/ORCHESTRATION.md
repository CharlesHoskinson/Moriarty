# MIL/4 K semantics surge

**Status:** research implementation of a proposal, not an admitted Core profile or Midnight result. The current SP01.6 guarded lane remains blocked by its recorded operational evidence gaps. Work here is isolated under this deliverable directory.

## Nine seats and ownership

| Cohort | Static language | Transition and history | DeFi families and kernel boundary |
| --- | --- | --- | --- |
| GPT-6 Sol | Draft independent `sol-static.k` | Draft independent `sol-transition.k` | Draft independent `sol-defi.k` |
| Claude Opus 5.5 | Independently derive K rules and challenge static draft | Independently derive K rules and challenge transitions | Independently derive K rules and challenge eight profiles |
| Grok 4.7 | Adversarial rule and gap audit | Adversarial rule and race audit | Adversarial family and boundary audit |

The lead integrates the three Sol modules only after reconciling Opus and Grok findings. External model calls retain raw JSON, readable answer, stderr and returned model identity. A failed or incomplete call does not count as a seat. Sol's host assignment is recorded separately from any returned runtime identity.

## Completed execution

The three Sol authors worked on separate static, transition and DeFi files. Each later received an audit repair assignment for its own files. The Opus transition call completed on the first attempt; static and DeFi timed out and completed on bounded tool-free retries. Initial Grok calls cancelled before a substantive final answer; all three completed on bounded tool-free retries. The returned Opus canonical model is `claude-opus-5-5`; Grok returned `grok-4.7-build` for the requested `grok-4.7` alias. Exact receipts and the audit disposition are in [AUDIT-RECONCILIATION.md](AUDIT-RECONCILIATION.md).

The integrated K definition is [mil4-proposal.k](mil4-proposal.k). Local family calculations use `m4FamilyProjection`, while `m4AdmitFamily` rejects every first family until its authenticated stage adapter exists. This distinction is a result of the independent reviews. The bounded escrow `m4Accept` relation has all six named judgments but relies on explicit native and ledger premises. The final K compilation and the remaining admission gaps are in [RESULT.md](RESULT.md).

## Rule inventory

The K proposal must define syntax and formation, exact numeric evaluation, intent refinement, staged transition, complete effects, authority consumption, history/continuation, observation and evidence, failure and recovery, and a first bounded relation for each of the eight DeFi areas. It must reject every unsupported or unresolved constructor explicitly. Kernel calls remain external observations; they do not become financial effects merely by returning success.

## Integration gate

The final output is one composable K definition plus a coverage table mapping every proposed constructor and judgment to a rule, explicit rejection or open empirical premise. Compile the K definition to check syntax and rule well-formedness. Compilation does not prove soundness, language-to-Core correspondence, native enforcement, or live financial acceptance. Any such claim stays open for the roadmap's later gates.
