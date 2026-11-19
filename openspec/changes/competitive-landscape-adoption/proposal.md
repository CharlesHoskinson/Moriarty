## Why

The assessment exercised nine partial order models and a separate native-local custody wrapper. It could measure unfilled cancellation on that wrapper. It could not measure a complete canonical exact-input fill. A supported path from signed source to accepted asset transfers would change that decision. Each order needs its own evidence.

The report revision is `1ced5fe1e0a83d665184bb496bea01bcb9940449`. It assessed Moriarty at `4b5697f7df594972a7d8790250231299b699139e`. This proposal reconciles both report copies against fork HEAD `875666133b80e2fc21ec2312fdd2350819bc675c` on `main`. The adoption sections agree. [Report](https://github.com/input-output-hk/SOTA-DeFiFormal-Moriarty/blob/1ced5fe1e0a83d665184bb496bea01bcb9940449/moriarty/reports/moriarty_competitors_report.html) · [Recommendations](https://github.com/input-output-hk/SOTA-DeFiFormal-Moriarty/blob/1ced5fe1e0a83d665184bb496bea01bcb9940449/onboarding/moriarty_web/report-content.json)

## What Changes

- Reproduce the assessed language suite. Account for 21 failed tests. Add revision-bound factory checks with scoped coverage.
- Connect generated kernels to authenticated order authority, current ledger state, funded settlement, recovery, continuations and named observations.
- Check scoped financial invariants. Exercise O1, O5, O8 and O9 under their individual policies.
- Measure complete O1 fill and cancellation. Exercise optional executor fallback. Release the supported lifecycle with a clean maintainer replay.
- Record whether a concrete integration needs O10. Defer the outside-developer replay and external venue pilot.

## Capabilities

### New Capabilities

- `supported-order-lifecycle` defines the supported application interface, native checks and evidence required for each report result.

### Modified Capabilities

None. The change refines children of the existing [U0–U7 task register](../consolidated-language-kernel/tasks.md). Its [UNI requirements](../consolidated-language-kernel/requirements.md) and [delivered-child reconciliation](../consolidated-language-kernel/reconciliation-2026-10-14.md#existing-task-and-child-capability-reconciliation) retain their original scope. The design maps every report ID to that register.

## Impact

The proposed delivery uses the bounded-atomic frontend in `experiments/moriarty-language/`, Midnight custody and receipt code in `experiments/moriarty-midnight-financial/`, and a public order package under `packages/moriarty-orders/`. Factory checks belong under `scripts/qa/`. Per-task evidence belongs under `deliverables/competitor-report/`.

This change is S2 under [WIKI_SCHEMA.md](../../../WIKI_SCHEMA.md). The terminal decision concerns the named child operations. Whole-language certification, public matcher operation, liquidity, downstream adoption, O10 execution and a new hosted CI service are outside this delivery. Developers retain permissionless access to supported programs under the [product contract](../../../docs/MORIARTY-PRODUCT-CONTRACT.md).
