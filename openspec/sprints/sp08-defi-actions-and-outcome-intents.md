# SP08: DeFi actions and outcome intents implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use superpowers:writing-plans to expand each task into a reviewed executable packet before behavioral edits. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Implement the DeFi action library and durable intent/request semantics against the pinned behavior matrix.

**Architecture:** This sprint contributes to MC01, MC05, MC07, MC08; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites.

**Tech Stack:** TypeScript/Node.js, `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): SP03. Stage scope: defi-semantics. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record.

## File and interface map

Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.

| Operation | Path | Responsibility |
| --- | --- | --- |
| create | `experiments/moriarty-language/library/defi/README.md` | Source libraries by action and lifecycle |
| create | `experiments/moriarty-conformance/src/defi.ts` | Original row and DA action crosswalk runner |
| create | `experiments/moriarty-conformance/tests/defi.test.mjs` | Independent economic regressions |
| create | `experiments/moriarty-conformance/fixtures/heldouts/pending-redemption.json` | Pending/Claimable/Claimed and unfilled duties |
| create | `experiments/moriarty-language/spec/successor/authorization.schema.json` | Canonical exact-plan and outcome signed data |
| create | `experiments/moriarty-language/tests/intent-lifecycle.test.mjs` | Partial fills, fees, cancellation and residual rights |
| modify | `experiments/moriarty-language/formal/k/moriarty.k` | Action and intent operational rules |

Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.

## SP08.1: Close action source gaps

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP08.md`.
- [ ] Crosswalk DA01 through DA24 with all 72 original DeFi rows. Pin required product/version lifecycle sources. Start with lending shares, partial liquidation and ERC-4626/7540 share/request behavior. Preserve multi-label taxonomy and external assumptions.
- [ ] Verify: Each required action has a feasible positive trace and distinguishing invalid behavior. Protocol names and category counts do not certify implementation.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP08` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP08.2: Implement economic action families

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP08.md`.
- [ ] Deliver swaps/liquidity/positions; credit/collateral/liquidation/refinance; stablecoins; vaults/strategy/rewards/staking; derivatives/margin/conditional claims; and observation/admin/message actions. Use the SP01 eight regression classes as release-blocking cases. Keep foreign events as bounded Midnight observations/messages.
- [ ] Verify: Core, K and evaluator agree on complete effects, debt, shares, fees, ordering and status. Iterative pricing algorithms use explicit finite bounds and reject exhaustion.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP08` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP08.3: Implement intent and pending request lifecycles

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP08.md`.
- [ ] Cover all eight named intent cases. Separate exact-output equality from minimum-net-receipt inequality. Track cumulative gross debit, fees, residual goals, deadlines and cancellation per authorization. Implement Pending, Claimable and Claimed with carried unfilled duties.
- [ ] Verify: A refund never restores gross authority. Overdelivery fails exact-output when equality is required. Fill/cancel races and replay reject; accepted pending redemption preserves claims and remaining work.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP08` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP08.4: Render canonical authorization

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP08.md`.
- [ ] Define signed bytes and human display from the same versioned schema. Outcome intent commits constraints; selected execution binds the authorization digest and refinement statement. Exact plans sign the defined plan body without a circular proof/transaction digest.
- [ ] Verify: Display/signature mismatch, unknown extension, recipient change, hidden liability and unsigned intermediate call reject. A solver supplies a proposal, never a trusted verdict.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP08` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP08.2 batch boundaries

| Packet under `openspec/sprints/execution/` | Action scope | Distinguishing exit evidence |
| --- | --- | --- |
| `SP08-exchange.md` | DA01-DA03, DA08 | Exact input/output, positions, fees, liquidity and atomic flash repayment |
| `SP08-credit.md` | DA04-DA07, DA09-DA10 | Shares, collateral, debt, partial liquidation, refinance and stablecoin issuance |
| `SP08-claims.md` | DA11-DA13, DA16 | Margin/funding, exercise/expiry, split/resolve and assumed external redemption |
| `SP08-staking-vaults.md` | DA14-DA15, DA17-DA19 | Rewards/slashing, queues, share rounding, async requests and strategy effects |
| `SP08-authority-environment.md` | DA20-DA23 | Authenticated observations, canonical intents, administration and bounded messages |
| `SP08-composition-model.md` | DA24 | Five operator semantic fixtures; real private proof qualification remains SP10 |

Each batch binds its original DeFi row IDs, positive/invalid complete traces, Core/K changes, source digests and resources. Shared behaviors may cite multiple packets. Missing row-specific behavior blocks the owning row even when its DA batch passes. SP08.3 separately retains all eight intent lifecycle cases.

## Shared file integration order

SP07 owns the first financial additions to shared `formal/k/**` and `src/successor/**` files. SP08 drafts its libraries and independent expectations concurrently, then rebases and integrates affected shared files after SP07's reviewed change. The orchestrator assigns one writer for each exact shared file in the executable packet and serializes their merges. No two admitted packets may own the same shared file simultaneously. Re-run affected financial and correspondence checks after integration.

## Verification entry points

Existing package commands may run only in their declared scope. Other commands are proposed interfaces that task admission must create and verify. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

```sh
npm --prefix experiments/moriarty-conformance run defi -- --all-rows
npm --prefix experiments/moriarty-language test
python3 experiments/moriarty-language/formal/k/run.py traces --all
```

Expected: exit 0 for valid supported inputs and all required controls passing. Invalid source/data must produce the documented rejection, not a crash or partial effect. Proof and public commands additionally require live RP03 admission.

## Exit gate

All required DeFi semantic rows and action targets implemented, with intent/request rejection evidence. Real acceptance remains assigned to SP09-SP11.
