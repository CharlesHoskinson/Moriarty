# FOREMAN_REPORT

- run_id: moriarty-tutorial-20261001
- role: implement
- slug: G1
- branch: foreman/moriarty-tutorial-20261001/implement/G1
- worktree: /home/charl/Moriarty/.worktrees/moriarty-beta-20260930-wt-moriarty-tutorial-20261001-implement-G1
- base_sha: df8b155143e90450168af04fbfd20f7620043620
- status: complete

## Summary

Worker G1 wrote `site/src/tutorial/Markets.tsx`. The module exports `Markets`. Section ids are `amm`, `lending` and `stablecoins`. Snippets are slices of the shipped beta sources. Changes are uncommitted. This worker did not test other modules or a deployment.

## Findings

AMM, lending origination, roll-forward and liquidation, and stablecoin mint, redeem and emergency are `SpecifiedOnly`. Financial relations stay `Open`. Local preparation is `Unsupported`.

`repay_loan` in the shipped repayment lesson is `LocalS0`. The lesson test returns `PreparedUnqualified`. Qualification is `local-stipulation-only`.

The page states canonical price orientation as base-per-quote. `Price<Base, Quote, Scale>` counts base units for one quote unit. The beta swap and stablecoin files do not write a `Price` value.

The 50500, 47500 and 7500 atom figures apply scale 2 from `lending.mori` to the horizon display amounts 505, 475 and 75. That conversion is an inference. It is not a beta simulation result.

Requested provider is Grok 4.7 at xhigh effort for worker G1. This session states the model as Grok 4.7. This process has no separate host receipt for effort xhigh.

## Evidence

Startup command:

`python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json`

Exit 0. `pendingTransactions` is empty. The blocked action is the historical loan-swap campaign. This task followed the website request.

`packages/moriarty-beta/dist/cli.js` is absent. This worker did not run `npm ci` or `npm run build`. The checks below execute `src/cli.ts` with `node --experimental-strip-types`. They are not the documented dist CLI.

From `packages/moriarty-beta`:

`node --experimental-strip-types src/cli.ts check examples/amm.mori`

Exit 0. `AuthoringChecked AmmDemo`. Actions `swap`, `mint_lp` and `redeem_lp` are `SpecifiedOnly` with execution unsupported and financial relations open.

`node --experimental-strip-types src/cli.ts check examples/lending.mori`

Exit 0. `AuthoringChecked LendingDemo`. Actions `borrow`, `roll` and `liquidate` are `SpecifiedOnly` with the same suffix.

`node --experimental-strip-types src/cli.ts check examples/stablecoin.mori`

Exit 0. `AuthoringChecked StablecoinDemo`. Actions `mint`, `redeem` and `emergency` are `SpecifiedOnly` with the same suffix.

`node --experimental-strip-types src/cli.ts check examples/local/repay/repayment.mori`

Exit 0. `AuthoringChecked LoanAgreement`. Action `repay_loan` is `LocalS0`. The text says local preparation remains `PreparedUnqualified`.

`node --experimental-strip-types src/cli.ts test examples/local/repay`

Exit 0. Status `TestsPassed`. Qualification `local-stipulation-only`. Case `interest first partial repayment` passed with status `PreparedUnqualified` and an empty mismatch list.

Simulate of `examples/amm.mori` action `swap`, `examples/lending.mori` action `liquidate`, and `examples/stablecoin.mori` action `redeem` each exited 1. Status is `Unsupported`. Code is `BETA_PROFILE_UNSUPPORTED`. `publishedPost` and `publishedEffects` are null. `scenarioValidation` is `NotAppliedUnsupported`. Source hashes are in `FOREMAN_REPORT.json`.

Page commands for `node dist/cli.js` match `packages/moriarty-beta/GETTING-STARTED.md` and the `check FILE` row in `packages/moriarty-beta/README.md`. This worker did not execute those dist commands.

Primary sources for the page:

- `packages/moriarty-beta/examples/amm.mori`
- `packages/moriarty-beta/examples/lending.mori`
- `packages/moriarty-beta/examples/stablecoin.mori`
- `packages/moriarty-beta/examples/local/repay/repayment.mori`
- `packages/moriarty-beta/examples/local/repay/scenario.json`
- `packages/moriarty-beta/examples/local/repay/mori.tests.json`
- `packages/moriarty-beta/src/frontend.ts` operation schemas and coverage fields
- `packages/moriarty-beta/src/bridge.ts` repay split and unsupported refusal
- `experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md` AccrualFirst rule
- `docs/decisions/u0-numeric-profile-decision.md` base-per-quote decision
- `wiki-llm/beta-language-2026-09-30/CONVERGENCE.md` and `FULL-LANGUAGE-HORIZON.md` sections 5 to 7
- `docs/MORIARTY-PRODUCT-CONTRACT.md` signed bounds and residual duties
- `site/VOICE.md`

`site/node_modules` is absent. This worker did not run `tsc` or the site verify script.

## Open questions

Root imports `Markets` into `GuideApp` in `tutorial/main.tsx`.

Root supplies CSS for `guide-section`, `guide-code`, `guide-status`, `guide-grid` and `guide-callout`.

Root runs `npm ci` and `npm run verify` in the integrated site.

Root checks browser navigation, the three section ids, copy, and project-subpath refresh.

Root allows Vite to import `packages/moriarty-beta` if the dev server blocks files outside `site/`.

Root builds `packages/moriarty-beta` before treating the documented `dist/cli.js` commands as executed.
