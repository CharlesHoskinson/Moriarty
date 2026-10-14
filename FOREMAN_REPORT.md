# FOREMAN_REPORT

- run_id: moriarty-tutorial-20261001
- role: implement
- slug: G3
- branch: foreman/moriarty-tutorial-20261001/implement/G3
- worktree: /home/charl/Moriarty/.worktrees/moriarty-beta-20260930-wt-moriarty-tutorial-20261001-implement-G3
- base_sha: df8b155143e90450168af04fbfd20f7620043620
- status: complete
- committed: false

## Summary

Added `site/src/tutorial/CrossChain.tsx`, a named export `CrossChain` with section ids `bridges`, `staking` and `trust`. The page teaches the shipped bridge and staking examples: use, intent versus kernel evidence, the pending-claim and share-class patterns, the timeout/refund race, and a support matrix for the eight family examples. A later polish pass on the same file retargeted three matrix hrefs, added the lending LocalS0 repayment link, and changed the copy and overflow controls. Changes are uncommitted. This pass edited only `CrossChain.tsx` and these two report files.

## Findings

- Bridge and staking actions are `SpecifiedOnly`. `check` exits 0. `expand` and `simulate` exit 1 with `BETA_PROFILE_UNSUPPORTED`, `scenarioValidation: NotAppliedUnsupported`, and `publishedEffects: null`.
- `inspect` atom strings: escrow/claim/recover and deposit use 10000; reward 500; slash 300; withdraw minimum 9000. Unbond and withdraw share atoms are the scalar 100. `USDCanonical` and `WrappedUSD` are different assets that both display 100.00 at scale 2.
- `inspect` returns `requiredPremises: []` and `unverifiedBindings: []` for both files. `check --json` open gates are `authentication`, `nativeProof`, `financialCorrespondence`, `atomicLedgerAcceptance`. Evidence is `AuthoringOnly`.
- Every identity `inspect` returned for the bridge file has `authenticated: false`.
- Custody split was rechecked on the existing trial probes, not presented as package examples. `bridge-misaligned.mori` rejects with `BETA_DOMAIN_MISMATCH` / `Quantity asset has different domain`. `stake-cross-account.mori` rejects with `BETA_DOMAIN_MISMATCH` / `Operation argument domains differ`.
- Matrix links now use the sibling ids read from the primary worktree, not the example-file stems. See Polish. Getting-started links remain `#getting-started`, `#transfer`, `#repayment`, `#open-premises`. Syntax remains `#syntax`.

## Evidence

Guarded startup, exit 0, no pending transactions:

`python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json`

The loan-swap campaign is blocked. This website edit is the assigned task and was not dispatched through that campaign.

Primary dist, same commit `df8b155143e90450168af04fbfd20f7620043620`, source files byte-identical to this worktree for the eight examples and `src/frontend.ts`, `src/bridge.ts`, `src/cli.ts`, `src/index.ts`:

`/home/charl/Moriarty/.worktrees/moriarty-beta-20260930/packages/moriarty-beta/dist/cli.js`

sha256 `3aa03ad353089f29c9fd49751c812a18ca5d4ea9f36c02f6831265ecbf3c926d`

Node `v24.21.0`. Commands from this worktree. Stderr on each Node invocation was `NO_COLOR` ignored because `FORCE_COLOR` is set. That warning is not mori output. Stdout:

```
node dist/cli.js check packages/moriarty-beta/examples/bridge.mori
AuthoringChecked BridgeDemo
  escrow: SpecifiedOnly (execution unsupported; financial relations open)
  claim: SpecifiedOnly (execution unsupported; financial relations open)
  recover: SpecifiedOnly (execution unsupported; financial relations open)
exit 0
source sha256 ed1899d32b244e5f2d7cab829baa4135c13722cc3340fbe90febaa08aaf12c27
```

```
node dist/cli.js check packages/moriarty-beta/examples/staking.mori
AuthoringChecked StakingDemo
  deposit: SpecifiedOnly (execution unsupported; financial relations open)
  reward: SpecifiedOnly (execution unsupported; financial relations open)
  slash: SpecifiedOnly (execution unsupported; financial relations open)
  unbond: SpecifiedOnly (execution unsupported; financial relations open)
  withdraw: SpecifiedOnly (execution unsupported; financial relations open)
exit 0
source sha256 301cfa8c15f1a848db7b6a23aa3b246fd646a970b8212458d5990e1da8a9b385
```

`check` on `amm`, `lending`, `stablecoin`, `derivatives`, `oracle` and `governance` also exited 0, every action `SpecifiedOnly`. Action names on the page are those stdout lines.

Scenario file bytes were `{}\n` (sha256 `ca3d163bab055381827226140568f3bef7eaac187cebd76878e0b63e9e442356`).

```
node dist/cli.js expand packages/moriarty-beta/examples/bridge.mori --action escrow --scenario /tmp/mori-unused-scenario.json
status Unsupported, code BETA_PROFILE_UNSUPPORTED, scenarioValidation NotAppliedUnsupported, publishedEffects null, publishedPost null
exit 1
```

```
node dist/cli.js simulate packages/moriarty-beta/examples/staking.mori --action withdraw --scenario /tmp/mori-unused-scenario.json
status Unsupported, code BETA_PROFILE_UNSUPPORTED, span start 1270 end 1300, publishedEffects null
exit 1
```

```
node dist/cli.js check wiki-llm/beta-language-2026-09-30/developer-trials/G3/probes/bridge-misaligned.mori
AuthoringRejected MisalignedBridge
  BETA_DOMAIN_MISMATCH at 8:53: Quantity asset has different domain
exit 1
```

```
node dist/cli.js check wiki-llm/beta-language-2026-09-30/developer-trials/G3/probes/stake-cross-account.mori
AuthoringRejected CrossAccountStake
  BETA_DOMAIN_MISMATCH at 7:28: Operation argument domains differ
exit 1
```

`check --json` on `bridge.mori`: `evidence` `AuthoringOnly`; `openGates` authentication, nativeProof, financialCorrespondence, atomicLedgerAcceptance. `inspect` coverage `financialRelations` Open, `localPreparation` Unsupported, `authentication` open, `ledgerCommit` open.

Intent preparation was not executed. `prepareOwnerIntent` calls `expand` and returns `BETA_AUTH_SOURCE` unless status is `Expanded` (`packages/moriarty-beta/src/auth.ts`). The observed expand status is `Unsupported`, so the page says that signing flow does not prepare these actions. No crypto binary was invoked.

Remote confirmed with `git remote -v`: `https://github.com/CharlesHoskinson/Moriarty.git`.

## Polish

Same G3 author. Requested provider remains Grok 4.7 at xhigh. Host statement for this session: Grok 4.7 (xAI). No separate effort receipt.

Startup for this pass: `AGENTS.md`, installed `plugins/moriarty-dev/skills/develop/SKILL.md`, and `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json` (exit 0, `pendingTransactions: []`). The loan-swap campaign remains blocked. This edit was not dispatched through it.

Read-only ids in `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`. Those files were not edited:

- `site/src/tutorial/Markets.tsx`: section `stablecoins` at line 406. Section `lending` at line 212. Heading `lending-repay` at line 278 is the LocalS0 `repay_loan` repayment ("The repayment you can run").
- `site/src/tutorial/RiskGovernance.tsx`: sections `options` (line 263), `oracles` (line 556), `governance` (line 760).
- `site/src/tutorial/GettingStarted.tsx`: section `repayment` at line 253 is also LocalS0. The matrix lending row does not use that id.
- `site/src/tutorial/GuideApp.tsx` lines 316–318 skip an injected copy bar only when `dataset.copyReady` is set or the previous sibling has class `guide-copybar`. That effect does not read `data-copy-owned`.

Applicable review mechanics, read only: D1 asks for `#stablecoins` and a lending-row link to the local repayment, distinct from specified origination, roll and liquidation (`Markets.tsx` line 278). D2 and D3 ask for one owned copy control, `role="status"`, try/catch around a missing clipboard, exact complete-source bytes, and a keyboard-focusable overflow target. CSS was not edited.

Exact edits in `CrossChain.tsx`:

- `#stablecoin` is now `#stablecoins`. `#derivatives` is now `#options`. `#oracle` is now `#oracles`. `#governance` was already the sibling id.
- The lending row still links `#lending` and `lending.mori`. Its actions cell now says borrow, roll and liquidate are specified origination, roll and liquidation in `lending.mori`, and adds a separate link `#lending-repay` with the text "LocalS0 repayment". The row's SpecifiedOnly and Open badges still describe that family file.
- `CopyButton` is `async` with one `try/catch`. A missing `navigator.clipboard`, a missing `writeText`, a synchronous throw, and a rejected write all take the `catch` path. `Copied.` is set only after `await` fulfills. The failure text is "Copy failed. Select the text and copy it manually." on a sibling `span` with `role="status"`. The button label stays "Copy program", "Copy command", or "Copy recorded output". It has class `guide-copy` and inline `minWidth` and `minHeight` of `44px`. The handler does not move focus.
- Each code `figure` has `data-copy-owned="true"`, `tabIndex={0}`, and an `aria-label` ending in "complete source", "command", or "recorded output". Copy writes the `text` argument. `bridgeSource`, `stakingSource`, `BRIDGE_CHECK`, `BRIDGE_EXPAND`, `STAKING_CHECK`, `STAKING_SIMULATE`, and the two command strings were not rewritten and are not trimmed.
- The staking quantities table and the support table have `tabIndex={0}` and `aria-labelledby` pointing at their existing captions. Those are the elements `styles.css` gives `overflow-x: auto`.

## Unperformed

- `npm ci`, `npm run verify`, Vite build, and browser checks. This worktree has no `site/node_modules`. This polish pass did not repeat them.
- A first `tsc` attempt failed on a bad `--types` flag. A second `tsc -p` against the pre-polish file, using the base worktree's TypeScript, `@types/react` and `vite/client.d.ts`, exited 0 under `strict`, `noUnusedLocals`, `noUnusedParameters`, `verbatimModuleSyntax` and `jsx: react-jsx`. That result does not cover the polish bytes. This pass did not run `tsc`.
- No install, network fetch, Cargo, native verifier, proof, wallet read, git command, agent setup, cleanup, or process kill on this pass.
- Clipboard success, unavailable-clipboard failure, synchronous-throw failure, duplicate-control suppression, 44px hit target, focus retention, and keyboard scrolling were not executed in a browser.
- The inspected primary `GuideApp` still injects a second copy bar for these figures, because it does not read `data-copy-owned`. Suppression is unverified until root honors that attribute.
- `.guide-code` overflow and `white-space: pre` remain on the figure in `styles.css`, which this pass did not edit. The caption is still inside that scrollport. `.guide-copy` there is `min-height: 2.5rem`; the 44px minimum is only the inline style.
- Other tutorial modules, GuideApp navigation, filter behaviour, project-subpath refresh, and deployment were not tested.
- The displayed `printf` line was run once, before this polish, to create the scenario file. The page copies that command; the earlier CLI calls used the resulting file.

## Root actions

- Import `CrossChain` from `site/src/tutorial/CrossChain.tsx` in GuideApp. Section ids: `bridges`, `staking`, `trust`.
- Skip copy injection when a `.guide-code` figure has `data-copy-owned="true"`. The inspected primary effect does not do this.
- Move preserved whitespace and horizontal overflow from `figure.guide-code` to its inner `pre` in CSS, without changing complete-source or recorded-output bytes. Keep the 44px target if the shared `.guide-copy` rule stays at `2.5rem`.
- Confirm matrix targets `#stablecoins`, `#options`, `#oracles`, `#governance`, and `#lending-repay` against the integrated Markets and RiskGovernance bytes. Confirm `#lending` still opens the SpecifiedOnly family.
- Style `guide-section`, `guide-code`, `guide-status`, `guide-grid`, `guide-callout`, and the tables.
- Confirm `?raw` imports from `packages/moriarty-beta/examples/*.mori` resolve under the site Vite root.
- Run `npm ci` and `npm run verify`, then the browser checks for navigation, copy success and both failure paths, filter, the eight sections, the lending distinction, keyboard overflow, downloads, and project-subpath refresh.

## Provider

Requested: Grok 4.7, xhigh, worker G3. Session identity stated by the host: Grok 4.7 (xAI). No separate returned-effort receipt and no independent identity check beyond that host statement.
