# S1 report — Merchant invoice (Moriarty beta 0.1.0-beta.1, model claude-sonnet-5-5)

## Artifacts
- `merchant/merchant.mori` — original program: EURC (scale 3), price 125.375, fee 1.250, `gross = price + fee`, `net = gross - fee`, one LocalS0 action `pay_invoice`.
- `merchant/scenarios/` — `good.json`, `low_allowance.json`, `expired.json` (closed local scenarios); `bad_unknown_field.json`, `bad_missing_allowance.json`, `bad_dup_key.json` (formation errors).
- `merchant/invalid/` — `unit_mix.mori`, `unknown_name.mori`, `scale_mismatch.mori` (check-rejected); `gross_cap_too_low.mori` (checks, Core-rejects); `specified_only_lending.mori` (copy of shipped lending example, SpecifiedOnly).
- `merchant/mori.tests.json` — 4 bounded cases (1 accept, 3 rejects). `merchant/negative-control/` — same cases with a deliberately wrong code to prove `test` can fail.
- `command-log.json` (25 entries), `outputs/*.txt` (full stdout/stderr/exit per command), `outputs/merchant.formatted.mori`, `outputs/22-mcp-session.txt`, `run.mjs`/`mcp_client.mjs` (my harness), `inv/` (unchanged init scaffold, only used to read the schema).
- `website-feedback.md`.

## Status of what was tested
| Item | Status |
|---|---|
| `check`/`inspect`/`fmt` on merchant.mori | **Local, actually run**: AuthoringChecked, `pay_invoice` = LocalS0. |
| `simulate good.json` | **Local, actually run**: `PreparedUnqualified`. Matches my independent arithmetic: gross 126625 atoms, Customer7 500000→373375, Merchant42 +125375, PlatformFees +1250, allowance remaining 200000→73375 spent 126625, head-0→head-1, work 10→9. |
| Insufficient authority | **Local**: allowance 100000 < 126625 → `CoreRejected`, `S0_AUTH_SCOPE`, `publishedPost/publishedEffects: null`, exit 1. |
| Other rejects | gross_cap=price and round 25 outside `0..20` both give `S0_INTENT_SCOPE`. |
| SpecifiedOnly (lending) | **Specified, not executable**: check exit 0 (syntax/names/quantities Checked, financialRelations Open, localPreparation Unsupported); expand and simulate exit 1 `BETA_PROFILE_UNSUPPORTED`, no effects. |
| Signature, provider authentication, proof, ledger acceptance | **Open / not performed.** Every success is `PreparedUnqualified`, `qualification: local-stipulation-only`, with four required premises (canonical-intent-signature, snapshot-to-head, head-extension, atomic-ledger-compare-and-consume) and four unverified bindings (agreement-id, selected-program, asset-scale, authenticated-predecessor). Passing tests close none. |
| MCP | **Actually used** with a small Node client: initialize, initialized, tools/list (check/inspect/expand/preview, closed schemas), preview of low_allowance → CoreRejected. |
| LSP, VS Code, Neovim | **Not used.** Only read docs. |

## Friction (exact)
1. **Atoms vs decimals split.** Source uses `125.375 EURC`; scenario and test expectations use bare atom strings (`"126625"`). Correct but a manual conversion step; nothing in docs says scenario amounts are atoms (I inferred it from init's 10.10 USD → "1010").
2. **Account/asset names vs ids.** Source refers to `Customer`/`EURC`; scenario, effects and post use claimed ids (`Customer7`, `EuroCoinCanonical`). Scenario `asset` and `domain` are ids, not names. Easy to mix up.
3. **Undocumented fixed action id.** Source/6 shows `selected TransferLiteralFee` regardless of my action name `pay_invoice`; not explained.
4. **Replay key inconsistency.** `ast.submitted.effects` shows `UseReplay key "inv-2026-0001"`, while `candidate.effects` and `consumedReplay` show `["Midnight","Customer7","inv-2026-0001"]`. Test expectations must use the latter. Undocumented.
5. **Identical code for different causes.** `S0_INTENT_SCOPE` covers both cap-too-low and expired validity window; I had to read the rest of output to tell them apart (there is no message field in rejection).
6. **Rejection expectation schema undocumented.** I guessed `expect: {status, code}` for rejects; it worked, and `code` is enforced (negative control failed as it should). Not stated anywhere I read.
7. **`test` output** for failures does not say what mismatched (`passed:false` only).
8. **Size of output.** `check --json` and `inspect` print thousands of lines (every reference/fieldUse span); simulate embeds the whole AST. A `--summary` mode would help. Positions are UTF-8 byte spans; no line/column, so locating BETA_ASSET_MISMATCH at 592–601 needs manual counting.
9. **`fmt` style.** Output is valid and idempotent (checked) but explodes call args onto lines with odd indentation (`valid: rounds(domain: Preview,` then `from: 0,` at the field indent). It changes sourceHash (55eb…→8e17…), which matters because the intent has `source_hash: "merchant-src-1"` — the literal is not tied to the real hash; docs say it never rewrites claimed hashes. `fmt` of invalid source exits 1 with the diagnostic, no edit (good).
10. **`init` refusing existing dir** gives a raw `EEXIST` message (exit 1), not a Moriarty diagnostic.
11. `--help` is one line; no flags/options per subcommand. README "Start" uses `node dist/cli.js` paths that don't apply to the installed package; `./node_modules/.bin/mori` worked.
12. A variant with `1.25 EURC` at scale 3 is accepted (equal to 1.250); `1.2505` gives BETA_PRECISION. Fine, but trailing-zero canonical spelling is a question for the docs ("noncanonical spelling" is mentioned for invalid.mori).

## Errors and discovery
- First attempt had no errors. My "scale_mismatch" variant initially was valid (1.25 ≤ 3 decimals); I repaired it to 1.2505.
- Diagnostics I saw: `BETA_ASSET_MISMATCH` (Qty + bare scalar), `BETA_REFERENCE` ("Unknown or forward reference fees"), `BETA_PRECISION`, `BETA_SCENARIO_SCHEMA` ("Unknown field bogus (/)", "Missing field allowance (//allowance)"), `BETA_JSON_DUPLICATE`, `BETA_ACTION_UNKNOWN`, `BETA_PROFILE_UNSUPPORTED`. Messages were clear; the `(//allowance)` path has a doubled slash.
- I did not try `min`/`max` or `atoms(...)` in my program, so those are unexercised. I did not test multiple actions per agreement, other fee recipients, or delegation/recovery fields beyond `None`.

## Experience ratings (only what I used)
- Diagnostics: 4/5 (codes good, spans byte-only). Formatter: 3/5. CLI commands: 4/5. MCP: 4/5 (worked first try from the docs; schemas clear). AI docs: read only; AGENTS.md was clear about the PreparedUnqualified caveat. Editor: not rated.

## Bugs vs preferences
Nothing crashed, and I found no wrong financial result against my hand arithmetic.
- Low severity defects/inconsistencies: replay key spelled two ways in one output (#4); doubled slash in schema path (#err); raw EEXIST from init (#10).
- Preferences: line/column in diagnostics; `--summary` output; fmt layout that keeps short calls on one line; explicit reject-expectation documentation; scenario amounts optionally accepting decimals (I would keep atoms as the default, since exactness is the point — minority preference to keep it strict).

## Repair suggestions / uncertainty
Document the scenario/test schema in README with a rejection example; include an `inv`-style worked table name→id; add a `reason` to rejections distinguishing cap vs window. Uncertain: whether `S0_INTENT_SCOPE` sharing is intentional, and whether the fixed `TransferLiteralFee` id is a placeholder.
