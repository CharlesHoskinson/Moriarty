# DeFi reference actions, source syntax and K semantics

Status: specified-only amendment to RP01 and MC01-MC08. This records the user's selection of `.mori` source files, a BNF-family grammar standard and K formal semantics, plus the [research-led design](../deliverables/defi-language-design-2026-09-07/README.md). It preserves the [three-report reconciliation](REPORT-RECONCILIATION-2026-09-07.md), registered atomic profile and all existing acceptance gates. It does not dispatch a campaign.

The [sprint schedule](sprints/README.md) assigns this amendment to concrete delivery gates. SP01-SP03 cover the financial contract, source specification and K; SP07-SP11 implement and qualify the complete financial domain.

## Decisions

Use `.mori` for source files. Specify the successor grammar in ISO/IEC 14977 EBNF, with separate lexical rules and typing/scoping judgments. K defines executable operational semantics of the typed Core; elaboration and backend correspondence remain separate obligations. Brace-delimited financial declarations with infix arithmetic are the working surface recommendation. The successor adds explicit pre/post state semantics only through a new profile; the existing atomic profile is unchanged.

Keep F1-F6/P economic families and multi-label facets. The [action matrix](../deliverables/defi-language-design-2026-09-07/action-targets.csv) adds behavior-level targets. A SoK category, protocol label or named property does not establish a Core primitive, conformance or theorem.

## Work within existing packages

| Package | Required addition | Acceptance evidence |
| --- | --- | --- |
| RP01 / MC07 | Map every action target to existing ACTUS fixtures/DeFi rows or a new pinned lifecycle source. Distinguish atomic actions, pending workflows and trust facets. | No lost source rows; independent financial expectations, rejection cases and explicit source gaps |
| MC01 | Publish complete lexical rules, EBNF, typing/effect/resource judgments and deterministic diagnostics for the successor. Compare three surface styles on matched tasks. | Grammar/parser agreement, formatter preservation, representative source examples and a scoped usability report |
| MC01 | Define and run the bounded Core in K, beginning with arithmetic, guards, staged updates, effects and one surviving obligation. | Pinned K version, runnable definition, positive/rejection traces and evaluator comparison; no ZKIR artifact substituted |
| MC03 / MC05 | Bind the eventual native transition relation to the new semantic profile and mandatory claims. | Updated relation/resource review before a native retry; actual recursive and rejection evidence |
| MC04 | Extend correspondence and ledger projection as types/effects expand. | Same program/authority/effects/duties across compiler, proof and ledger, including rejected conflicting spends |
| MC06 | Specify request lifecycles, residual authority, private successor artifacts and five composition operators. | Isolated private handoff; split/join with conserved work and duties; duplicate-consumption rejection |
| MC08 | Update the developer guide when implemented profiles change. | Cold-reader source-to-simulation-to-acceptance walkthrough with implemented/proposed boundaries |

## Bounded execution sequence

1. Preserve `.mori` source bytes and active tooling references; run the existing frontend, evaluator and lowering tests.
2. Bind the action matrix to normative fixtures. First follow-up: lending shares, partial liquidation and ERC-4626/7540 share/request behavior. Keep held-out refinance, nominal-debt and pending-redemption cases.
3. Complete the successor lexical/EBNF/static design and matched syntax specimens. Decide comments, Boolean precedence, update semantics, rounding and visibility explicitly. Do not mutate the registered bounds in place.
4. Admit a small K experiment with a declared command, pinned version, input, resource limit and stopping condition. Demonstrate the same partial-payment trace in K, the evaluator and an independently derived expectation. Do not restart the archived K/ZKIR campaign.
5. Extend only the operations required by the target matrix. Requalify the proof relation, compiler and ledger projection when the semantic domain changes.

For each target, closure needs source/version, initial state, authorized action, observations, complete expected effects, post-state, residual obligations/authority, bounds and at least one distinguishing rejection case. A complete required target without a primary lifecycle source remains a source gap rather than disappearing from coverage.

## Change and recovery boundary

The `.mori` migration is a path-only change to the two current source examples and active readers. Their byte hashes remain the same. Historical evidence files retain `.moriarty` paths and can be recovered at their recorded commit. This does not claim that old review manifests bind the changed readers; new maintenance checks and reviews apply only to this migration and design dossier.

The TypeScript evaluator remains the implementation under test until a K definition exists and correspondence is established. This amendment does not report source semantics, native proofs, financial Preview settlement or any MC package complete.
