# FOREMAN_REPORT

- run_id: moriarty-tutorial-20261001
- role: implement
- slug: G2
- branch: foreman/moriarty-tutorial-20261001/implement/G2
- worktree: /home/charl/Moriarty/.worktrees/moriarty-beta-20260930-wt-moriarty-tutorial-20261001-implement-G2
- base_sha: df8b155143e90450168af04fbfd20f7620043620
- status: complete
- requested provider: Grok 4.7 xhigh
- returned identity: Grok 4.7, as named by this session. The previous invocation ended with signal 143 and left no provider receipt. No second auditor ran, and none was substituted.

## Summary

`site/src/tutorial/RiskGovernance.tsx` exports `RiskGovernance` with sections `options`, `oracles` and `governance`. The page teaches the shipped `moriarty-beta/1` files `derivatives.mori`, `oracle.mori` and `governance.mori`: economic use, identities and atom counts, intent versus checker versus Core versus the optional kernel, the authoring pattern, a hostile case grounded in `frontend.ts`, the check command shape, and the complete file. Proposed horizon lifecycles stay in closed details and are marked as not beta grammar. Changes are uncommitted. Root still integrates the module, styles, images and verification.

## Findings

- Options are `packages/moriarty-beta/examples/derivatives.mori`, agreement `DerivativesDemo`. Collateral `500.00 USD` is 50000 atoms at scale 2. Payoff `100.00 USD` is 10000 atoms and must use settlement asset alias `USD`. Strike `"100.00"` is a string and is not converted. Bob is declared and is not an operation argument. Support is SpecifiedOnly. Financial relations are Open. Expand and simulate are Unsupported.
- Oracles are `oracle.mori`, agreement `OracleDemo`, action `read_fixing`. `observed_round` 140 and `maximum_age` 5 are separate scalars. `finality` `"finalized"` and `provenance` `"provider binding open"` are strings. The unit string does not reference assets `USD` or `GOLD`. No issuer field is present. The file digest is not a price.
- Governance is `governance.mori`, agreement `GovernanceDemo`. Epoch 3, `next_epoch` 4, `earliest_round` 180 and `veto_before` 179 are scalars. `earliest_round` and `veto_before` sit only on the Queue intent. `source_hash` is the claim string `terms-v3`, which is not the file digest. Queue, execute and veto are agreement actions. Maintainer review is outside the agreement.
- Designer notes applied inside this module: beta profile and completeness labels, SpecifiedOnly / Open / Unsupported text, LocalS0 and PreparedUnqualified named only for transfer and repayment, identity tables, excerpt versus complete file, copy and download outside the scroller, 15px code with `font-size: inherit`, keyboard-focusable scroll regions, 44px controls, `role="status"`, `scroll-margin-top`, ink-muted rather than faint, no green success mark, horizon text not offered as a runnable program.
- Navigation, search, mobile shell, shared `guide-*` CSS and the two tutorial images belong to root. This worktree has no `site/public/tutorial-assets`.

## Evidence

Startup, exit 0. No pending transactions. The blocked action is the unrelated loan-swap campaign, and this task did not follow it.

```text
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json
```

File digests from `python3` `hashlib.sha256` on the checkout bytes. Each file ends with a newline. `packages/moriarty-beta/dist/cli.js` is absent, so these digests are not CLI `sourceHash` stdout.

| File | Bytes | SHA-256 |
| --- | --- | --- |
| packages/moriarty-beta/examples/derivatives.mori | 1244 | 17b0e9ea666e0867b032c40605a5a89b02beddc043affa9a1d1d6256cf187449 |
| packages/moriarty-beta/examples/oracle.mori | 897 | 2da41606e65fe6385df8e148bc4ade41a802d377245323d8bc7838944372ac04 |
| packages/moriarty-beta/examples/governance.mori | 971 | 1a9eb7c670f957429d5a4c82d89ed0f3dd1e8147210c3e524bbfd34885b2af4f |

Displayed check lines are the readable formatter in `packages/moriarty-beta/src/cli.ts` applied to source order and to the SpecifiedOnly assertions in `packages/moriarty-beta/tests/examples.test.mjs`. They are not a captured command receipt. The Unsupported message quoted on the page is the string in `packages/moriarty-beta/src/bridge.ts`.

Atom counts follow `frontend.ts`: scale 2 appends two fractional digits, so `500.00` is 50000 and `100.00` is 10000.

Unperformed here: `npm ci`, `npm run verify`, browser checks, `node dist/cli.js`, package tests, and any native or proof command. `site/node_modules` has no local `tsc`.

## Open questions

None for this module. Root integrates `RiskGovernance` into `GuideApp`, confirms `guide-code` CSS does not override the inline code size with `!important`, runs verify and the browser checks, and places the two generated images. This report does not claim those steps, the other modules, or a deployment.

## Render polish, same author, continuation after signal 143

The earlier invocation exited 143 before this polish. The owned source was unchanged at the start of this continuation. This pass edited only `site/src/tutorial/RiskGovernance.tsx` and these two reports. No test, build, browser, network, native command, git write, install, or other module edit.

- Public check text no longer says this checkout has no `dist/cli.js`. The commands remain `node dist/cli.js check examples/<file>` and the same command with `--json`, run from `packages/moriarty-beta` after `npm run build`.
- The readable check panel is labelled "Illustrative readable result, derived from the formatter in `packages/moriarty-beta/src/cli.ts`, not a capture." Its `<pre>` has `data-copy="none"`.
- The three proposed lifecycle `<details>` have `data-copy="none"`. They contain no `<pre>`. Complete beta files stay in their own details, with the same byte counts and SHA-256 values.
- Each area now shows purpose, the support line, then the exact excerpt, then units and schema tables. Identity rows, atom counts, bounds, and source bytes are unchanged.
- Repeated kernel, signature-field, and maintainer prose points at `#trust`. The local qualification stays: `BETA_PROFILE_UNSUPPORTED`, the bridge message, `publishedEffects` null, `publishedPost` null, `scenarioValidation` NotAppliedUnsupported, and `local-stipulation-only`. `keyAuthority` Unverified, LocalStipulationOnly, NotChecked, NotSubmitted, and `ledger_accepted` false stay in the short intent paragraph.
- Options say "The sketch names Alice as holder". Governance says the intended policy change would become eligible at round 180 and that the authored integers and strings do not enforce that behavior.
- Section and heading scroll margin is `var(--guide-offset)`.
- Each table has one `guide-table-wrap` region, `tabIndex={0}`, and an `aria-label`. The table is `display: table` with `overflow: visible`. The wrapper is the only horizontal scroller. The standing "Scroll sideways" sentence is gone.
- Missing spaces before inline `<code>`, `<DocLink>`, and `<a>` were closed with explicit `{' '}` nodes, including the O3 sites around `net_floor`, `domain`, and the evidence link lists. Spaces were not inserted inside `<pre>` source.

Unperformed in this pass: typecheck, site build, browser render, clipboard behavior, and any check of the shell's `data-copy="none"` injector. Those remain root checks.
