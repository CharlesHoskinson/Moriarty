# Moriarty roadmap, design, and ten-commit review

**Examined:** 2026-09-29. **Revision:** `983a4bb49e3ccae399ee2514da2f6fa3f03593fd`. This note records the repository state used to choose the [MIL/2 research sprint](SPRINT-PLAN.md).

## Roadmap and design position

The [consolidated roadmap](../../ROADMAP.md) has one delivery sequence, U0–U7. U0 requires a versioned source/Core semantic contract, six stage judgments, a target enforcement map, and explicit trust premises. U1 must certify a minimum native basis; U2 must demonstrate a general supported single-stage path; U3 then admits conditional settlement and recovery. Source research can proceed while U0 is open, but it does not close a milestone. The former queue in `docs/MORIARTY_ROADMAP.md` is archived.

The [MIL/2 design](../../concepts/intent-language/DESIGN-MIL2.md) is specified-only and supersedes MIL/1. The [execution summary](../../concepts/intent-language/EXECUTION-SUMMARY.md) reports that nine reviewers would not freeze MIL/1 because its escrow, completion, and evidence anchoring rules failed. MIL/2 proposes repairs but has no implementation, compiler/prover/ledger evidence, or second review. Its six §17 obligations remain open. Five owner decisions in the summary govern the proposed Φ cut, flash loans, joins, stage signer arity, and concentrated liquidity; they are design choices with reversal costs, not evidence of semantic soundness.

The repository's [successor static semantics](../../experiments/moriarty-language/spec/successor/static-semantics.md), [semantic contract](../../experiments/moriarty-language/spec/successor/semantic-contract.md), and [source/5 lifecycle](../../experiments/moriarty-language/spec/successor/financial-agreement-source-v5.md) provide the existing admission, typing, and local-failure conventions used in the separate [proposed-semantics PDF](MIL2-PROPOSED-SEMANTICS.pdf). The PDF labels its additional intent relation as proposed; it does not claim an implemented lowering.

## Last ten commits on the examined branch

| Commit | Date | Change | Bearing on this sprint |
| --- | --- | --- | --- |
| `983a4bb4` | 2026-09-28 | Add intent-language execution summary | States the outstanding obligations and return order. |
| `f7773eff` | 2026-09-28 | Merge MIL/2 and nine-reviewer work | Makes the new design and review record the current concept baseline. |
| `08bc0dd8` | 2026-09-28 | Add intent language and review | Introduces MIL/1, MIL/2, category map, and reviewer evidence. |
| `8f737840` | 2026-09-23 | Add U0 T7 exit gate | Keeps U0 acceptance tied to explicit gates. |
| `e8708268` | 2026-09-23 | Integrate U0 T4 | Advances target-pin work without completing MIL/2 semantics. |
| `b376ff69` | 2026-09-23 | Correct U0 T4 audit findings, round 9 | Shows the target-pin evidence needed repeated correction. |
| `3e855462` | 2026-09-23 | Integrate U0 T6 | Advances trust-premise and backend-matrix work. |
| `110599b7` | 2026-09-23 | Integrate U0 T5 | Advances one U0 component; no MIL/2 freeze follows. |
| `9f69405c` | 2026-09-23 | Integrate U0 T3 | Advances one U0 component; no MIL/2 freeze follows. |
| `495f5de6` | 2026-09-23 | Integrate U0 T2 | Advances the numeric-profile component used by later semantics. |

The most recent three commits shift the research baseline to MIL/2; the next seven are U0 gate and integration work. The immediate useful contribution is therefore a source-grounded semantics correction and a bounded research plan, with each proposed rule traced to the current source/Core conventions and its native enforcement obligation.
