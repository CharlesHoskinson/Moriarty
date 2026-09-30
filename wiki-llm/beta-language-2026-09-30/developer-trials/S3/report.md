# S3 report — AMM + oracle authoring, nominal quantities, truthful unsupported execution

Model: claude-sonnet-5-5. CLI: `./node_modules/.bin/mori` (package `0.1.0-beta.1` per MCP serverInfo), Node v24.21.0.

## Artifacts
- `src/pool_oracle.mori` — original EUR/XAU pool: `oracle.select`, `amm.swap_exact_input`, `amm.mint`, `amm.redeem`, consts with `+`. All four actions are **SpecifiedOnly**.
- `src/swap_fee.mori` — original **LocalS0** transfer: 250.00 EUR to a pool vault plus fee `0.25 EUR * 3` = 0.75 EUR to a protocol account.
- `scenarios/swap_fee_ok.json`, `scenarios/swap_fee_underfunded.json` (untrusted local stipulations).
- `variants/` — 5 invalid variants plus 5 probes (all `AuthoringRejected`).
- `cases/good/` and `cases/bad/` — two self-contained `mori test` projects, each with its own `mori.tests.json` and copies of files. `cases/bad` also holds two malformed scenarios.
- `command-log.json` (every command is also re-run by `mklog.py`; re-run outputs were byte-identical), `outputs/`, `mcp_client.py`, `website-feedback.md`.
- `starter/` is the unmodified `mori init` output, kept only for reference; it is not my deliverable.

## Tested outcomes and truthful status
| Item | Result | Status |
|---|---|---|
| `check` both programs | `AuthoringChecked`, exit 0 | structural/name/nominal-quantity only |
| `swap_fee` simulate | `PreparedUnqualified` | **Local, actually executed by Core/5**; qualification `local-stipulation-only`. No signature, provider authentication, proof or ledger commit. 4 premises and 4 unverified bindings remain open. |
| Underfunded scenario | `CoreRejected`, `S0_EFFECT_RANGE`, `publishedPost/Effects: null` | local |
| AMM/oracle `simulate`/`expand` | `Unsupported`, `BETA_PROFILE_UNSUPPORTED`, exit 1, `publishedEffects: null` | **Specified only.** No swap output, pool invariant, fee or oracle freshness is computed. |
| `mori test` | good: 4/4 pass, exit 0; bad: 5/5 match intended rejections, exit 0 | local |

My hand arithmetic (before reading the output): fee 3×25 = 75 atoms; gross 25000+75 = 25075; Trader 100000−25075 = 74925; Vault 5000+25000 = 30000; Protocol 75; allowance remaining 74925, spent 25075; work 4→3 remaining, 1 spent. It matched. Round in post-state is unchanged (5); I inferred this from the starter, because the docs do not say. `inspect` also showed the exact AMM atoms (50000, 2400, 150, 50150, 3000), matching scale 2 and scale 4.

**Not established by anything here:** the pool's invariant, whether 0.2400 XAU is a fair floor for 500.00 EUR, oracle freshness (`observed_round: 200, maximum_age: 3` are only recorded claims), fee correctness for AMM actions, any obligation. `check` says `financialRelations: Open` for those actions, and `inspect` lists **empty** `requiredPremises`/`unverifiedBindings` for them. An empty list looks like "nothing open" but actually means "nothing modelled". The `openGates` list in `check --json` is the real caveat.

## Friction (exact)
- **Discovering LocalS0 shape.** Only `init` output shows the `transfer` intent, with about 15 mandatory fields (`key`, `nonce`, `pre_head`, `source_hash`, `policy_digest` and so on), none explained. Per README, AMM and oracle intents are much lighter, so the two kinds look unrelated.
- **Local scenario is single-asset**, so an AMM cannot be simulated locally in any form. A two-asset pool scenario is impossible. I found this only by reading the README sentence on local transfer and repayment.
- **Account/asset ids vs names.** Effects use the `id:` claim (`TraderT`), not the declared name (`Trader`). The scenario needs ids. `UseReplay` key differs: `"swap-7"` inside `ast`, but `["Midnight","TraderT","swap-7"]` in the candidate effects. Expected values must use the latter.
- **Action id.** `selected.actionId` is `TransferLiteralFee`, not my `settle_swap_fee`. `source_hash: "swapfee-src"` is an unverified free string and does not match the real `sourceHash`.
- **Test schema** is undocumented in the README. I had to read `dist/cli.js` (`runCases`) to learn: `expect.status` ∈ 6 values, optional `code`/`effects`/`post`; effects and post are deep-equal; paths must be relative and inside the project directory, so good and bad suites need separate directories with copied files. A manifest outside its own directory (e.g. `../src/x.mori`) is refused. I did not test that refusal; I am going by the source.
- **Failed expectation gives no diff**: `passed:false` only (`outputs/test_wrong_expectation.json`).
- **`S0_EFFECT_RANGE` for an underfunded payer** names neither balance nor shortfall.
- **Formatter** expands every record onto many lines (my compact file roughly tripled in lines) and changes the source hash. The README warns about this. Idempotent (verified).
- **`fmt` without `--write`** prints to stdout; I captured it, but the verb suggests in-place.
- Two checks I expected to be loose are actually strict and good: net_floor must match `output_asset`, and output must be a declared pool member. Both are caught at `check`. `probe_output_same_as_input` was caught only incidentally by the net_floor type, not by a message about pool semantics.
- **Diagnostic quality.** Codes are stable and clear (`BETA_ASSET_MISMATCH`, `BETA_PRECISION`, `BETA_MISSING_FIELD`, `BETA_REFERENCE`, `BETA_UINT128_BOUND`). Messages for mismatch on `+` do not name which operands. I did not check whether a span points at the right place beyond offsets.
- `BETA_ASSET_MISMATCH` for `EUR+USD` fires at `gross_cap`, not at the `const` where USD was introduced. Sensible, but the user must trace it.
- Unsupported exits 1 and prints JSON on stdout; there is no human-readable summary in the CLI.

## Editor / AI
- MCP: **used** via a real stdio client (`mcp_client.py`): `initialize`, `notifications/initialized`, `tools/list`, `preview` (local transfer, Prepared), `check`. Worked first time. Arguments are `source`/`action`/`scenario` text. Read-only hints present.
- LSP, VS Code, Neovim, AI adapters: **not used**; no rating.
- Only the docs guidance in `AGENTS.md` was applied: I kept the source/scenario separation, didn't weaken bounds, and labelled results PreparedUnqualified.

## Bugs vs preferences
No confirmed defects. Nothing crashed, and results reproduced byte-for-byte on re-run.
Possible defects (low, unconfirmed intent):
1. (Low) `inspect` shows empty `requiredPremises`/`unverifiedBindings` for SpecifiedOnly actions, which is misleading. It may be intended.
2. (Low) `mori test` failure lacks any diff, which hurts usability even if not a bug.

Preferences:
- Document the `mori.tests.json` schema and scenario schema in README/`--help`.
- `mori simulate --explain` or a text summary including a balance table.
- A minority preference worth preserving: keep the heavy explicit 15-field transfer intent. It shows every signed bound. I'd want the formatter to keep short-record compact style optional, but exploding records is also defensible because it aids diffs.
- Show an effect-id/name cross-reference (name → id) in `inspect`.
- Show which `check` sub-coverage applies to AMM (`quantities: Checked`).

## Repair suggestions and uncertainty
Document schemas; add diffs to test failures; put "0 modelled premises" wording for SpecifiedOnly; name operands in mismatch errors. Uncertain: round-unchanged semantics; whether inclusive `valid` range `to: 9` matters (only round 5 was tried); behaviour of repayment, which I did not try since it is out of my assigned scope; whether the `Unsupported` exit 1 is wanted for CI use.
