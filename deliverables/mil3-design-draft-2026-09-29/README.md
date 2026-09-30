# MIL/3 proposal record

The [integrated specification draft](DESIGN-MIL3-DRAFT.md) is a specified-only proposal derived from the eight [MIL/2 DeFi category sections](../mil2-deep-research-2026-09-29/CATEGORY-COVERAGE.md) and five fresh, independent Claude Opus 5.5 drafting assignments. The agents ran sequentially in the isolated worktree at baseline `983a4bb49e3ccae399ee2514da2f6fa3f03593fd`. They read the Moriarty development skill and guarded status. They did not edit files, run tests or claim acceptance.

| Lens | Full draft | Raw CLI receipt | Returned model | Reported cost USD |
| --- | --- | --- | --- | ---: |
| Formal core | [1](agents/draft-01-core.md) | [JSON](agents/raw-01-core.json) | `claude-opus-5-5` | 1.3474478 |
| Financial resources and arithmetic | [2](agents/draft-02-finance.md) | [JSON](agents/raw-02-finance.json) | `claude-opus-5-5` | 2.1225186 |
| Evidence, authority and history | [3](agents/draft-03-evidence.md) | [JSON](agents/raw-03-evidence.json) | `claude-opus-5-5` | 1.5756274 |
| Eight category profiles | [4](agents/draft-04-profiles.md) | [JSON](agents/raw-04-profiles.json) | `claude-opus-5-5` | 1.82746 |
| Adversarial integration | [5](agents/draft-05-adversarial-retry.md) | [JSON](agents/raw-05-adversarial-retry.json) | `claude-opus-5-5` | 1.8107052 |

All five listed receipts report terminal reason `completed`; aggregate CLI-reported cost is USD 8.683759. Prompts and stderr logs are retained in `agents/`. The first fifth-agent attempt returned a short correction unrelated to the requested full draft despite reporting `completed`. Its [text](agents/draft-05-adversarial.md) and [raw receipt](agents/raw-05-adversarial.json) are retained, excluded from the five full drafts, and the assignment was rerun with an explicit fresh session ID. The correction noted that a MIL/2 citation to `lean/DefiKernel/Typed/Transition.lean` is not in this Moriarty checkout; any use of that external precedent needs a pinned DeFiFormal source. The integrated draft does not rely on it.

The root reconciliation chose to distinguish sub-unit division remainders from integer allocation residue, retain outstanding debt through impairment, cap the first cash-settled call, and separate exact-rational CPMM fees from rounded-fee economics. Those choices are candidate text for review, not adopted project decisions. The [decision table](DESIGN-MIL3-DRAFT.md#10-decisions-and-dissent-retained-for-review) keeps unresolved alternatives and required evidence visible.
