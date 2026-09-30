# Website feedback: Moriarty beta tutorial site (Git/GitHub hosted)

Based on my first run (seat S3). The example outputs below come from real runs saved in `outputs/`.

## Landing promise
"Write a money movement with exact, typed quantities. See the exact local effects. Know precisely what was *not* proven." Show one 12-line result block and one `Unsupported` block side by side, so the honesty is the feature.

## Layout and framework
Plain Markdown in `docs/`, built with mdBook or Docusaurus/Astro Starlight and deployed by GitHub Pages Actions. Every code block must be a real file under `examples/` and be executed in CI (`mori check`/`mori test`), with expected output pasted from CI. A link checker is also useful. Pages need stable URLs for diagnostic codes (`/errors/BETA_PRECISION`).

## Navigation / page order
1. Landing
2. Prerequisites & install (Node ≥ 24; `npm ci`; `npx mori --help`; note that `init` refuses existing dirs)
3. First five minutes (`init`, `check`, `simulate`, `test`)
4. First transfer, then repayment (walk the 15 fields; the repayment page is where funded AccrualFirst appears)
5. Assets, identities, units (id vs name vs symbol; scale; nominal types; `* 3` scalar multiplication; no division/rounding)
6. Source intent vs untrusted fixture
7. First rejection (underfunded payer → `S0_EFFECT_RANGE`; EUR+USD → `BETA_ASSET_MISMATCH`)
8. Bounded project tests (schema, limits 1–64 cases, relative paths, separate good/bad folders)
9. Editor and AI setup (LSP, MCP session; AGENTS.md)
10. Support matrix
11. Proof, authentication and ledger limits
12. Reference (CLI, JSON shapes, operation schemas, diagnostic codes, test schema)
13. Next steps (SpecifiedOnly families: AMM, oracle, lending...)

## Content missing in the shipped docs (found in run 1)
- The `mori.tests.json` schema (I had to read `dist/cli.js`).
- The scenario JSON schema: field meanings, that it is single-asset, and that `round` stays unchanged in the post-state.
- Name vs `id` in scenarios/effects, and the two `UseReplay` key spellings.
- What each of the 15 transfer fields does, and that `source_hash`/`policy_digest` are free strings.
- What `selected.actionId` means (`TransferLiteralFee`).
- What "empty premises" means for SpecifiedOnly.
- Which exit code each status gives (Unsupported and CoreRejected both gave 1; `test` gives 0 when expected rejections match).

## Example outputs to include
- Local success: effects Debit 25075 / Credit 25000 / Credit 75 / UseAllowance / UseReplay / AdvanceHead; post balances 74925, 30000, 75; label `PreparedUnqualified`, `local-stipulation-only`.
- Unsupported: `{"status":"Unsupported","diagnostics":[{"code":"BETA_PROFILE_UNSUPPORTED",...}],"publishedPost":null,"publishedEffects":null}`.
- Test run: `TestsPassed` with per-case statuses.

## First-rejection tutorial
Start from a passing file; change one thing; show the code. Suggested three: over-precision (`0.255 EUR` → `BETA_PRECISION`), cross-asset add (`BETA_ASSET_MISMATCH`), underfunded scenario (`CoreRejected`). Then run it as a `mori test` case with `expect.code`.

## Proof/authentication/ledger limits
A dedicated page and a repeated footer: no signature, no provider authentication, no proof, no ledger acceptance; four premises and four unverified bindings are listed on every local result. A matching hash or passing test closes none of them. Formatting changes the source bytes.

## Support matrix
| Family | check | expand/simulate | test |
| transfer, funded repayment | yes | LocalS0, PreparedUnqualified | yes |
| amm.*, oracle.*, lending, stablecoin, option, governance, bridge, staking | yes (structure, names, nominal quantities) | Unsupported | only as `Unsupported` expectations |

## Omit from getting started
AMM/oracle internals, LSP/VS Code packaging, MCP, the wiki's "full language" horizon, JSON span details, and formatter theory. Mention them only on later pages. Do not show a SpecifiedOnly example before the learner has seen one local success, or they will assume it runs.

## Tutorial acceptance checklist
- [ ] A fresh Node 24 machine reaches a `PreparedUnqualified` result in 5 minutes with copy-paste only.
- [ ] Every example is executed in CI and its output is pasted from CI.
- [ ] Each page states its qualification level (Authoring / LocalS0 / Specified).
- [ ] The learner derives expected balances by hand before seeing output.
- [ ] A rejection is shown for each of: authoring, scenario formation, Core.
- [ ] Test schema and exit codes documented.
- [ ] Source vs fixture is explained with the claim that the fixture cannot override source.
- [ ] A deliberate failing `mori test` is shown, with an explanation of how to read it (today: no diff).
- [ ] Limits page is linked from every result block.

## Minority preference
I would keep a "raw intent" appendix showing the entire 15-field transfer with annotations. Some readers want to see every signed bound rather than a hidden default, which is the language's strength.
