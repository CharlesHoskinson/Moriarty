# Repair report: interactive Federated DeFi Kernel page, after GPT-6 audit 01

Date: 2026-09-19 (session ran into 2026-09-20 UTC). Implementer: Claude Fable 5.1 at medium effort, selected by the user as implementation author. This report answers `GPT6-AUDIT-01.md` finding by finding, records what was actually run, and lists what remains open. `FABLE-REPORT.md` is preserved unchanged. Nothing was committed or pushed; no campaign, chain, wallet or proving operation was performed.

Worktree: `/home/charl/Moriarty-pages-20260919`, branch `docs/two-reference-pages-20260919`, base `0ce871a3`. Startup: applied the checked-in develop skill and ran `cli.py --repo . status --json`; the unrelated `SP01.6 loan-swap-subset` block persists with no pending transactions and does not gate these authorized file edits.

## Findings and what changed

### R1 (High): observed external success forgotten when a predicate is missing. Fixed.

The reducer now separates the observed external result from agreement acceptance.

- On `observe late-success` the account moves unconditionally: reservation converts to debit, fee is incurred, custody falls, confirmed receipt rises, and the attempt identifier is consumed. That is what the success fixture establishes: the destination executed the transfer, so 5.5 A left custody (0.5 A of it as fee) and 10 B sits at `recipient-b-account`.
- If every predicate holds, the fill is accepted in the same step (unchanged transcript numbers). If any predicate is missing, the state enters a new `observed` phase: the fill count does not advance, `fill-2` is not consumed, the delivery and remedy duties stay open, and the event record names the unmet predicates.
- A new event `accept-observed-fill` reconciles the observed result exactly once when every predicate holds. It moves no money (delta null). A second acceptance is refused.
- `terminalConflict` refuses any later observation against a resolved attempt, in every phase, with the reason named: a later failure or unknown report cannot release the debit or make the receipt unhappen; a later success cannot be credited to a consumed failed attempt.
- `duplicate-observation` is no longer hard-coded. It re-presents the resolved attempt's own outcome through the same conflict check and records the refusal.

Tests: four stale/missing/withheld-acceptance/withheld-signature variants each go success, then conflicting failure, unknown and repeat success (all refused, account and duties unchanged), then restore, accept once, second accept refused. Browser: the same path is driven with wrong-issuer evidence, then fresh evidence, then acceptance.

### R2 (High): failed first fill permits reuse of `attempt-1`. Fixed by enforcing the approved ordering.

The fixture now follows the approved story only: the first fill is finalized locally, the second is reserved and submitted externally. `finalize-fill` is refused unless `fillsDone === 0`; `reserve-fill` is refused unless `fillsDone === 1`. `availableEvents` offers only the matching control, and the reducer refuses the rest regardless of the UI. A consumed-identifier check on `reserve-fill` additionally refuses reuse of `attempt-2`; because the fee cap already refuses that in this policy, the test exercises the consumption rule under a more generous policy (`grossCap 2000, feeCap 200`) so the refusal is the identifier rule and not the cap. A walk over every preset asserts the consumed list never contains a duplicate.

Retry attempts with fresh identifiers are not modeled; the refusal text says so.

### R3 (High): React render failure hides the fallback. Fixed with a real runtime mechanism.

`main.tsx` now wraps each root in a class error boundary. The static block is hidden only when a `useEffect` inside the mounted tree runs, i.e. after React's first commit; `createRoot().render()` returning no longer marks anything. On a render or update error, `componentDidCatch` restores the static block, removes `data-enhanced`, sets `data-restored`, and unmounts the root on a microtask so the failed tree leaves nothing behind. `onUncaughtError` on the root does the same for anything the boundary does not see. No debug switch was added to the page.

Browser tests inject the fault through Playwright, not the page: an init script makes `document.createElement('fieldset')` throw before the production bundle runs (first-render failure), and a second context lets the explorer mount and then patches `createElement('code')` so the first finalized fill's consumed-identifier rendering throws during an update. Both contexts check the static transcripts return, the explorer root is un-enhanced and empty, the separately rooted inspector still responds, and print shows the full fallback. The whole-bundle abort and throw-on-load contexts remain.

### R4 (Medium): remedy duty left open after failure-free completion. Fixed.

`finishIfComplete` discharges `remedy-on-failure` only when its status is still `accepted`; a `pending` remedy is never cleared by completion. Tests assert all duty statuses on success (both discharged, `openDuties` empty), failure (delivery accepted, remedy pending), unknown and prefix states. Browser checks the remedy status on the success and failure branches.

### R5 (Medium): false narrative arithmetic. Fixed.

The out-of-scope first-attempt branches are now unreachable (R2), and the two narratives are derived from state anyway. `second-solver-reserve` names the exclusive pending reservation first and adds the cap arithmetic only when the sum actually exceeds the cap (in the reachable state: 11 A plus 5.5 A reaches 16.5 A against 11 A). The failure narrative uses `capacityPhrase`, which prints the remaining gross and fee capacity from the account; a test confirms "5 A of ordinary capacity and no fee capacity" under the fixture policy and "14 A ... and 1 A of fee capacity" under the generous one.

### R6 (Medium): evidence claims too broad. Fixed in `evidence.ts` and the static table.

- Threshold: the claim is now that a signature over exactly these bytes verifies under the group key for the bound membership epoch; that enough honest participants took part, and that each checked the policy, is an inference under the protocol and corruption assumptions; no informed approval is attributed to any individual, and generic MPC supplies no per-signer approvals.
- TEE: the claim is now that a report under the vendor root identifies the measured code and configuration and the report data the enclave bound; any "ran this code on this input and produced this output" claim is explicitly conditional on the measured code binding its input and output into the report and on trusting that code and the verification policy. A fourth assumption states this.
- Correlation: the lede and inspector now say that where one operator controls the keys, hardware or code behind several mechanisms those checks share a failure and count once, while the distinct assumptions still stand on their own. The "three attestations from one operator are one assumption" sentence is gone.

Browser checks read both the enhanced panel and the static table for the same narrowed wording.

### R7 (Medium): four acceptance judgments absent. Fixed.

The responsibilities section gains a "Four acceptance judgments, kept apart" block explaining contract properties, intent refinement, transition validity and history compliance in practical terms, with a disclosure mapping each to the fixture's checks and stating that the page's checks are host-side JavaScript that produce none of the native proof obligations. The explorer's candidate panel carries a one-paragraph note tying its checks to the same four names. The disclosure count is now three.

### Nonblocking items

- Assets now carry a `reference` field and the fixture note names the concrete toy identities (issuer, domain, reference, kind) for A and B.
- Model tests now sample the observed-pending and observed-ready states and cover the adverse orderings above. The state machine is bounded to the approved scenario rather than expanded into a workflow engine.
- Screen-reader use and true browser zoom remain unperformed (see below).

## What was run

All from the worktree, on the built production bundle.

| Command | Result |
|---|---|
| `npm --prefix site run verify` (tsc, Node tests, build) | pass; 51 Node tests, 0 failures (40 in `kernel-scenarios.test.mjs`) |
| `python3 site/test/kernel.spec.py` at `http://localhost:8891/` | 190/190 |
| `python3 site/test/kernel.spec.py` at `http://localhost:8892/Moriarty/` | 190/190 |
| `python3 site/test/site.spec.py` | 49/49 |
| `python3 site/test/docs.spec.py` | pass, exit 0 |

Protected baseline digests (`protected-baseline.json`) all still match. Screenshots from the root run are in `/tmp/kshots`, including `kernel-observed-accepted.png` and `kernel-render-failure.png`.

During the repair two of my own new browser checks failed and were corrected, not preserved: forcing a click on a control whose `disabled` prop is set does nothing in React (it drops the synthetic event), so those checks could not reach the reducer; they now assert the UI offering and leave reducer refusal to the Node suite, which exercises every out-of-order event directly. The focus-outline check moved to a control that is enabled at that point. The old Node test asserting a fee-cap refusal for a local second fill after failure now asserts the ordering refusal, which fires first.

## Changed files and digests (sha256)

```
1d1f44cf2c9a1222170ee41f7993ab48c7136dc9738115934fa36599706eb859  site/kernel.html
f4fafb4de4d327167720605e8811bab5848d57e89e2ef38d665cb5fe402da7a8  site/src/data/kernel-scenarios.mjs
906ef089b8ccebdb387524d87175a9c9d7a4948ba8847f7cc447edce326e5b88  site/src/data/kernel-scenarios.d.mts
b1239b73c007f6fb5eba9bb3a5c4f2226d80963edbda755b42e53f5371214423  site/src/data/kernel-scenarios.test.mjs
3368fe0d6e5325578af01e3ed3f5dac1a2128bda6d120afe315ba574600665db  site/src/kernel/EvidenceInspector.tsx
8c91f4d6c99fc6b963f7f284e22f5ba7f79b1b2127e79ed9cb3405cc8b9238c6  site/src/kernel/KernelExplorer.tsx
2782ab532f310a47645fe663150ed7ef322cd05cab2d98435f066d94c4621b9c  site/src/kernel/evidence.ts
3283b83e810b917913b382f202a19e34f28936275987b54b08b8612525b6ff55  site/src/kernel/main.tsx
c50757b3a69218839aed3fd0f434d03d465b74fb789b57a31ae1ad3545601dda  site/src/kernel/styles.css
c33ea0410639edf32935914f6ec082bc7dc37608e466685cc16ed33b92729c2e  site/test/kernel.spec.py
6dcbee33cfe2fbc61c0750030ba5abc183da7194788868e3edd110cb104d3b27  site/test/README.md
```

Unchanged from candidate 01: `.github/workflows/site.yml`, `README.md`, `site/CONTENT-SPEC.md`, `site/package.json`, `site/src/App.tsx`, `site/vite.config.ts`. The static transcript numbers are unchanged and still checked against the reducer. An untracked `deliverables/kernel-page-implementation-2026-09-19/` directory holding copies of the research artifacts appeared in the worktree during this session; I did not create or modify it.

## Remaining limitations

- No screen reader was driven. The suites check status-region role, text labels, focus visibility and tab order, which are preconditions, not the pass the acceptance criteria require.
- The "200%" check remains root-font scaling in a half-width viewport, not actual browser zoom.
- Retry attempts with fresh identifiers after an authenticated failure are not modeled; the fixture refuses them by name. A real adapter's retry policy is out of this page's scope.
- The reducer's refusal of out-of-order events is proven in Node, not by clicking disabled controls in the browser, because React discards clicks on controls whose `disabled` prop is set. The browser suite proves the UI offers only the approved ordering.
- The evidence prose is a source-contract correction only. No attestation, threshold signature or proof was verified by anything on this page.
- Live publication is not established by these local runs; the deployed route needs its own byte check after a deploy.
