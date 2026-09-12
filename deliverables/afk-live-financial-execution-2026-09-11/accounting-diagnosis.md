# Accounting and current I2 admission diagnosis

Repository observation, 2026-09-11. The investigation read code and non-secret resource records. It did not execute a worker, read wallet snapshots, submit transactions, or change live accounting. Exact observed hashes, modification times and totals are in `accounting-diagnosis-observations.json`.

## Current blockers

Guarded status reports stale `openspec/sprints/sp01-financial-contract-and-execution-admission.md` in both the binding and candidate inputs, missing `.moriarty-dev/runtime/current-accounting.json`, and unavailable live resource state. The registered SP01 action verifies a historical static design; it does not execute I2. `.moriarty-dev/verify-loan-design.py` also reads the original historical binding internally, so updating only the current binding cannot repair that caller.

The old master is `/home/charl/.local/state/moriarty/mc01-supervised-20260907/budget.json`. It has master limit 210890 seconds, consumed/reserved total 210883.04733050402 seconds, and only 6.952669495978625 seconds remaining. MC01 dispatches are 52/52; aggregate dispatches are 83/83. All successor envelopes are exhausted. Its Preview counters remain zero even though later separate financial run records exist. It is a historical lower bound, not current public accounting.

## Producer and consumer

No tracked plugin script produces current accounting. The historical producer pattern is `/home/charl/.local/state/moriarty/moriarty-dev-plugin-20260908/successor-binding-03/run.py`, lines 24–47. It locks the old master, validates reviewed envelope authority and candidate scope, transfers reserved seconds into a full-bound charge, increments dispatches, and writes by atomic replacement before launch. Do not rerun this superseded worker.

`plugins/moriarty-dev/scripts/moriarty_dev/records.py:2166` consumes `moriarty.supervised-accounting/1`; allowed top-level keys are at line 330. It requires the original pinned authority SHA-256, package identity, nonnegative planning/overhead and charge lists, positive limits, and applicable action/dispatch/envelope capacity. Store reconciliation provenance separately; arbitrary extra top-level fields fail its closed schema.

`runner.py:123` requires one existing debit matching charge ID, seconds, action ID, candidate hash and runner digest. The digest is SHA-256 of the sorted compact JSON runner plan. The plan commits executable, launcher, argv files and action. `records._verify_candidate` derives candidate hash from sorted compact owned-file hashes and separately verifies input hashes. Order construction as: reviewed source/requirements manifest; candidate hash; action and runner plan; runner digest; binding hash and campaign pointer; exact debit. Mutable accounting and self-referential binding/control records must not be owned candidate bytes.

The current reader does not enforce the master aggregate dispatch ceiling or authenticate the amendment chain. `records._verify_resource` also adds amendment allocation, successor increments and binding allocation. The producer must enforce aggregate limits and prevent representing the same allocation twice; passing the consumer alone is insufficient authority.

## Smallest repair recommendation

After the first-item OpenSpec requirements receive independent approval, preserve every historical grant, charge, reservation, failure and accepted candidate. Reconcile later immutable run records into a current cumulative ledger with explicit source hashes and uncovered periods. A reviewed successor amendment must add bounded capacity to consumed totals; an eight-hour AFK window does not reset them. Connect this ledger to a new bounded current I2 action and admission through the existing plugin. Preserve old SP01 accepted artifacts. Avoid a new scheduling framework.

`deliverables/sp05-financial-integration-2026-09-09/preview-loan-01/gpt6-accounting-containment-01.json` is one reconciliation source: it records prior-plus-actual reserved submissions 15 and DUST 4500000000000015 SPECK. Do not sum cumulative snapshots as independent charges or equate DUST SPECK with tNight or indexer fee units.

## Independent failure expectations for the first item

1. Missing, stale, unsupported or non-contained source/accounting paths fail before a debit or child launch. Changed requirements invalidate their candidate commitment and current reviews.
2. The exhausted old master and zeroed successor envelopes cannot create credit. Omitting an external package, lowering historical charges, reusing a spent grant, or granting refunds after interruption fails reconciliation.
3. Amendments must retain the authority chain, exact reviewed scope and both required substantive votes. Duplicate amendment identifiers or repeated application cannot increase capacity twice.
4. One action must map to one exact debit and runner digest. Duplicate charge IDs, altered action/candidate/seconds, or plan bytes changed after reservation must deny execution. Candidate construction must terminate without hash self-reference.
5. Two concurrent reservations must serialize under the existing master lock. Combined limits, external dispatches, per-package quotas and audit reserves remain valid after both attempts. One last available dispatch cannot launch two children.
6. A crash before debit commits cannot launch. A crash after debit but before launch retains that charge; recovery is explicit and cannot silently refund. A crash between master and projection writes must be detectable; stale projection must not provide spendable credit.
7. Failed/ambiguous processes retain reservations and consumed bounds. Producer state and plugin SQLite action reservations must agree on ownership; an existing charge cannot replay a completed action.
8. Public reservations, attempts, observed submissions, inclusion/finality, gross debit and fees remain distinct. Unknown totals or unit correspondence block dependent live work rather than defaulting to zero.
9. Existing historical acceptance cannot approve a current I2 action. The new action must commit the actual caller and current source/requirements; the SP01 static verifier cannot count as financial ledger acceptance.
10. Strict schema edge cases include booleans used as numbers, negative/nonfinite values, duplicate debit rows, unknown top-level fields, wrong package, wrong worktree and depleted envelope. Resource allocation must not be counted once in an amendment and again in a binding.

## Safe observation commands

From the repository, `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json` and `report --json` read guarded status. `stat -c '%a %s %y %n' /home/charl/.local/share/moriarty/test-wallets/preview-runtime-20260907/.midnight-wallet-state/preview/*.json` reads wallet metadata only. A bounded POST containing GraphQL `query { block { height hash timestamp } }` to `https://indexer.preview.midnight.network/api/v4/graphql` observes public tip readiness, not wallet balance.

Do not use `npm run check-balance` as a strictly read-only probe: `check-balance.ts` calls `getOrCreateWallet` and `persistWalletState`. No current balance was asserted by this diagnosis.
