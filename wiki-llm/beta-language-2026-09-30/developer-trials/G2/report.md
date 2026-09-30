# Loan servicing trial

Seat G2 used `@moriarty-lang/beta` 0.1.0-beta.1 with Node v24.21.0 and `./node_modules/.bin/mori`. The program is an original note-service agreement: one exact draw, one accrual-first installment, and one exact payoff, plus specified-only origination, roll, and liquidation. Local preparation returned `PreparedUnqualified`. It did not sign, authenticate a provider, generate a proof, or accept a ledger transaction.

The first `mori test loan-desk` passed 14 of 14 cases. A later run, after probes that sit outside `mori.tests.json`, passed the same 14. Expected effects and post-states were written from the arithmetic below before that test. None of those expected values were revised to match tool output.

## Artifacts

| Path | Role |
| --- | --- |
| `loan-desk/servicing.mori` | Original agreement, 179 lines after `fmt --write` |
| `loan-desk/invalid-window.mori` | Invalid program: round window `from: 90, to: 44` |
| `loan-desk/invalid-precision.mori` | Invalid program: `12.505 USD` at scale 2 |
| `loan-desk/repay-with-fee.mori` | Authoring-valid repay whose `fee_cap` is `12.50 USD` |
| `loan-desk/scenarios/` | Closed local scenario files |
| `loan-desk/mori.tests.json` | 14 cases, profile `moriarty-beta-tests/1` |
| `loan-desk/probes/` | Discovery fixtures. `mori test` does not run them |
| `loan-desk/tools/protocol-clients.mjs` | Local MCP and LSP stdio clients |
| `loan-desk/README.md` | Project note for the note desk |
| `outputs/` | Captured stdout and stderr |
| `outputs/init-starter/` | Copy of `mori init` before it was replaced |
| `command-log.json` | 61 command records |
| `website-feedback.md` | Tutorial-site proposal |

`mori init loan-desk` created the starter invoice, scenario, tests, and README. `mori test` on that starter passed `literal fee payment` as `PreparedUnqualified`. Those files were copied to `outputs/init-starter/` and then removed from `loan-desk`, so the project test runs only the note-service manifest.

Package files under `node_modules` were not edited. No signature, proof, transaction, or credential was requested.

## Outcomes

`check` on `servicing.mori` prints:

```text
AuthoringChecked NoteService
  disburse: LocalS0
  service_installment: LocalS0
  close_note: LocalS0
  originate: SpecifiedOnly
  roll: SpecifiedOnly
  liquidate: SpecifiedOnly
Local preparation remains PreparedUnqualified.
```

`inspect` reports `authenticated: false` on every domain, account, asset, and obligation. The file hash is `c47dfcdcbbd4cfbe49b8d1b2e753414c4ee77b40470568a1f635febb365ce176`. That hash is the SHA-256 of the formatted source bytes. The in-source strings `claim-draw-source`, `claim-installment-source`, and `claim-payoff-source` are opaque claims. Formatting did not rewrite them.

Local actions carry these required premises:

- `canonical-intent-signature`
- `snapshot-to-head`
- `head-extension`
- `atomic-ledger-compare-and-consume`

and these unverified bindings:

- `agreement-id`
- `selected-program`
- `asset-scale`
- `authenticated-predecessor`

Specified-only actions carry empty premise and binding lists. `check --json` also reports `evidence: "AuthoringOnly"` and open gates `authentication`, `nativeProof`, `financialCorrespondence`, and `atomicLedgerAcceptance`.

| Case | Status | Code | Exit |
| --- | --- | --- | --- |
| `disburse-exact` | PreparedUnqualified |  | 0 |
| `installment-accrual-first` | PreparedUnqualified |  | 0 |
| `close-note-settled` | PreparedUnqualified |  | 0 |
| `installment-replay-consumed` | CoreRejected | `S0_HISTORY_REPLAY` | 1 |
| `installment-balance-short` | CoreRejected | `S0_EFFECT_RANGE` | 1 |
| `installment-work-exhausted` | CoreRejected | `S0_AUTH_SCOPE` | 1 |
| `installment-round-after-window` | CoreRejected | `S0_INTENT_SCOPE` | 1 |
| `installment-overpay-outstanding` | CoreRejected | `S0_EFFECT_RANGE` | 1 |
| `installment-nonzero-fee-cap` | CoreRejected | `S0_INTENT_SCOPE` | 1 |
| `originate-specified-only` | Unsupported | `BETA_PROFILE_UNSUPPORTED` | 1 |
| `roll-forward-specified-only` | Unsupported | `BETA_PROFILE_UNSUPPORTED` | 1 |
| `liquidate-specified-only` | Unsupported | `BETA_PROFILE_UNSUPPORTED` | 1 |
| `inverted-round-window` | AuthoringRejected | `BETA_ROUND_WINDOW` | 1 |
| `overprecise-service-fee` | AuthoringRejected | `BETA_PRECISION` | 1 |

Successful simulate results include `qualification: "local-stipulation-only"`. Rejections set `publishedEffects` and `publishedPost` to null. A diagnosed rejection is still exit 1, with the JSON body on stdout.

Shipped `examples/lending.mori` checks as SpecifiedOnly for `borrow`, `roll`, and `liquidate`. Simulating `borrow` returned `BETA_PROFILE_UNSUPPORTED` and no effects. The examples README says the same for all eight family files. This trial executed that refusal for lending only.

Final project command: `mori test loan-desk` → `TestsPassed`, `local-stipulation-only`, 14 passed (`outputs/58-final-test.stdout`).

## Arithmetic used for the expected effects

`UsdNote` scale is 2, so display amounts become atoms by appending zeros to the fractional part. `1_250.00 USD` is 125000 atoms. `GoldGram` scale is 3, so `2.500 GOLD` is 2500 atoms. There is no division and no rounding. Windows are inclusive. Preparation spends one unit of work and does not change the round. Repayment applies to accrued first, then principal. A repay whose `fee_cap` or `net_floor` is nonzero is outside the local rule.

Draw, window 10 through 20, scenario round 10:

- Principal 125000, service fee 1250, gross 126250.
- Lender balance and allowance start at 126250 and finish at 0. Spent allowance becomes 126250.
- Borrower receives 125000. Servicer receives 1250.
- Work 8/0 becomes 7/1. Head `svc-head-10` advances to `svc-head-11`. Round stays 10.
- Replay key is `["DeskNet","Lender","draw-1"]`.

Installment, window 44 through 52, scenario round 52:

- Source schedule: accrued 1875, principal open 100000, outstanding 101875, payment 21875.
- Interest taken is `min(21875, 1875) = 1875`. Principal taken is `21875 - 1875 = 20000`.
- Post obligation: principal 80000, accrued 0, outstanding 80000, status Outstanding.
- Borrower balance and allowance start at 21875 and finish at 0. Lender balance goes from 0 to 21875.
- Work 3/4 becomes 2/5. Head `svc-head-52` advances to `svc-head-53`. Round stays 52.
- Replay key is `["DeskNet","Borrower","installment-3"]`.

Payoff, window 80 through 88, scenario round 80. This fixture is a separate stipulation. It reuses the 80000 residual principal from the installment arithmetic and adds a new accrued amount of 625. It is not the installment candidate post, and the balances are not chained.

- Payoff 80625. Accrued taken 625. Principal taken 80000.
- Post obligation: 0, 0, 0, Settled.
- Borrower balance and allowance start at 80625 and finish at 0. Lender balance goes from 0 to 80625.
- Work 1/9 becomes 0/10. Head `svc-head-80` advances to `svc-head-81`. Round stays 80.
- Replay key is `["DeskNet","Borrower","payoff-1"]`.

The short-balance fixture changes only the borrower balance to 21874. The work fixture changes only `work_remaining` to `"0"` while leaving the allowance at 21875. The replay fixture changes only `replay` to `"consumed"`. The window fixture changes only `round` to `"53"`.

A probe with accrued 5000 and outstanding 105000, payment still 21875, prepared principal `83125`. Hand arithmetic: `100000 - (21875 - 5000) = 83125`. The source names `principal_open` and `accrued_at_service` do not bind the scenario. `outputs/40-probe-accrual-mismatch.stdout` matched that prediction.

## Syntax, units, accounts, assets, intents, scenarios

Declaration names and economic ids differ on purpose. `Desk` has id `DeskNet`. `USD` has id `UsdNote` and symbol `USD`. `Note` has id `NoteSeven`. Scenario `domain` and `asset` must be the ids. Using symbol `USD` or declaration name `Desk` returns `BETA_SCENARIO_IDENTITY`. The starter teaches the same split: asset id `A`, symbol `USD`, account declaration `Buyer`, id `Owner`.

Quantities need whitespace before the asset name. Underscores in `1_250.00` are accepted and preserved by `fmt`. A third decimal at scale 2 is `BETA_PRECISION`. `1 / 2` is `BETA_CHARACTER`. The shipped `ai/examples/invalid.mori` fails first on `01` with `BETA_NUMBER` and does not also report the unknown name.

An S0 intent requires domain, asset, signer, key, nonce, pre_head, `rounds` on that domain, gross cap, fee cap, net floor, operation, source hash claim, policy digest claim, `SuccessOnly`, empty observation and duty arrays, and `None` delegation and recovery. The signer must be the transfer `from` account or the repay `payer`. Transfer needs three distinct accounts. Repay balances must be ordered payer, then creditor. The allowance owner must be the signer. Obligation `outstanding` must equal principal plus accrued, and the status in the scenario must be `Outstanding`.

`fee_cap: 12.50 USD` on a repay passes `check` and fails in core with `S0_INTENT_SCOPE`. Local accrual-first repayment prepares only when fee cap and net floor are both zero atoms. A servicing fee is a separate transfer, which is what `disburse` does.

Scenario JSON is closed. Unknown field `memo`, replay tag `used`, a numeric round, a missing obligation, and a duplicate `round` key each fail formation. Duplicate keys are rejected rather than last-key-wins. Integer amounts are canonical decimal strings.

Round 44 and round 52 prepare. Round 43 and round 53 return `S0_INTENT_SCOPE`. The window in source is the inclusive pair on the intent. The scenario round is the untrusted clock. `valid: rounds(domain: Desk, from: 90, to: 44)` fails authoring with `BETA_ROUND_WINDOW` at CLI position 14:12. The same diagnostic in LSP is 0-based line 13, character 11.

`predecessor` is required and is copied into Source/6. Changing it to `unrelated-predecessor` still expands and still prepares. The candidate post has no predecessor field. `inspect` already lists `authenticated-predecessor` as unverified. Head must equal `pre_head` (`S0_HISTORY_STALE` otherwise). `post_head` must differ from head (`S0_HISTORY_SUCCESSOR` otherwise).

Replay `consumed` means this domain, signer, and nonce are already consumed. The key is `["DeskNet","Borrower","installment-3"]` with no spaces. The scenario cannot list other consumed nonces. Work is one counter: a successful preparation subtracts 1 from remaining and adds 1 to spent. Zero remaining is `S0_AUTH_SCOPE`, and so is a zero allowance.

Specified-only `roll` returns `BETA_PROFILE_UNSUPPORTED` even when the scenario file is the text `not-json`. Structural checking does not validate evidence freshness, collateral, accrual policy, or the obligation cells for originate, roll, or liquidate.

Overpayment expansion stays `Expanded`. Its note says the proposal preserves the unchanged obligation and that only core decides the first failure. The proposal text still debits 21875 and sets outstanding 10000. Core then rejects with `S0_EFFECT_RANGE` and publishes nothing. `Expanded` is not a candidate settlement.

## Discovery

No expected test value was changed after a failure. The note program checked and tested on the first formatted source.

Probes outside the manifest confirmed the boundaries above. The public import `{ check, simulate } from "@moriarty-lang/beta"` agreed with the CLI for the installment and for `roll`.

`mori init loan-desk` a second time exited 1 with stderr `mori: EEXIST: file already exists, mkdir '...'`. There was no JSON body.

`fmt --write` on the inverted window exited 1 and left the file unchanged. `fmt` on the valid agreement was idempotent.

This session sets both `FORCE_COLOR=1` and `NO_COLOR=1`. Every Node process, including `mori`, prints a warning that `NO_COLOR` is ignored. That warning is session environment noise. Product JSON and text still arrived on stdout, and exit codes were independent of it.

## Diagnostics, formatter, commands, editor, AI

Text `check` is the useful human summary: status, one-based line and character, action support, and the PreparedUnqualified reminder. `check --json` is a machine report of 50835 bytes because it inlines all 30 operation schemas on every call.

`inspect` is the right place for the premise and binding lists. `expand` shows the Source/6 proposal and, for overpayment, the proposal note. `simulate` is the core decision. `test` compares status, optional code, and optional full effects and post. On success it reports `passed`, `status`, and `code: null`. It does not echo the effects.

`fmt` preserves comments and numeric spelling, refuses invalid source, and reflows call arguments without an extra indent. The result parses and checks.

CLI positions are one-based. JSON `span` values are UTF-8 byte offsets. LSP positions are zero-based UTF-16. Those three views agreed on the inverted `rounds` call.

A local MCP client spoke newline JSON-RPC to `mori mcp`. `initialize` returned protocol `2025-06-18` and server `moriarty-beta` 0.1.0-beta.1. `tools/list` before `notifications/initialized` returned error `-32002`, `Client initialization is incomplete`. After that notification the tools were `check`, `inspect`, `expand`, and `preview`. `preview` of `service_installment` was `PreparedUnqualified`, `isError: false`, with the four premises and the 80000/0/80000 obligation. `preview` of `roll` was `Unsupported`, `isError: true`, `publishedEffects: null`. MCP `preview` is CLI `simulate`.

A local LSP client used Content-Length framing. The server advertised `positionEncoding: "utf-16"` and `textDocumentSync: 1` (full documents). Document symbols listed the declarations and the six actions with `LocalS0` or `SpecifiedOnly` in `detail`. Hover on `principal` returned `const principal` plus `AuthoringChecked; identity metadata is an unverified claim.` Definition resolved to the declaration line. Diagnostics for the inverted window matched the byte span, expressed as a zero-based LSP position. Shutdown exited 0.

The shipped Neovim 0.11 smoke, run from the package directory with `MORIARTY_BETA_CLI` set to `dist/cli.js`, exited 0. Its pass line was on stderr and names initialize, open, definition, symbols, format, change, diagnostics, and shutdown. That smoke uses its own demo buffer. It does not open `servicing.mori`.

VS Code packaging and activation were not run. No adapter from `ai/adapters/` was copied into the project. `ai/README.md` says to copy an adapter only when the project owner asks. The MCP process was driven by the local client above, which is separate from installing a provider.

## Bugs, by severity

No financial mismatch turned up. The hand-computed draw, installment, and payoff effects prepared on the first test. Specified-only actions published nothing. Rejections published nothing.

Low. LSP hover uses the sentence `identity metadata is an unverified claim` for every declaration, including `const principal`, which is a quantity. Reproduction: `outputs/54-protocol.stdout`, hover field. The unverified-claim warning is useful. Calling a quantity "identity metadata" is the wrong noun.

Low. `1 / 2` is reported as `BETA_CHARACTER` with `Unexpected source character; identifiers must be ASCII`. The character is ASCII `/`. Reproduction: `loan-desk/probes/no-division.mori`, `outputs/56-no-division.stdout`.

Low. A second `init` into an existing directory writes a raw `EEXIST` string to stderr and no JSON object. Other failures this trial produced JSON on stdout. Reproduction: `outputs/51-init-exists.stderr`.

Low. `qualification` is present on `PreparedUnqualified` and `CoreRejected`, and absent on `AuthoringRejected`, `FormationRejected`, and `Unsupported`. The results that refuse execution are the ones missing the local-only field. `publishedEffects: null` is still present. Reproduction: `outputs/27-simulate-roll.stdout` and `outputs/30-simulate-invalid-window.stdout` versus `outputs/19-simulate-installment.stdout`.

I did not find a high or medium correctness defect in the paths I ran.

## Preferences

These can stay as they are. They are recorded because they cost time on this loan.

Shared core codes do not name the comparison. Short balance and overpayment are both `S0_EFFECT_RANGE` and judgment `effect`. Round 53, round 43, and a nonzero repay fee cap are all `S0_INTENT_SCOPE` and judgment `intent`. Zero work and zero allowance are both `S0_AUTH_SCOPE` and judgment `authority`. Replay has its own code, `S0_HISTORY_REPLAY`. A message that names the failing comparison would help. The first-failure order itself behaved consistently: window and fee cap before effects, overpayment before balance, balance before allowance and work, stale head before replay, replay before an equal successor.

`check` and `inspect` use different labels for the same local coverage: `DelegatedToCore` versus `DelegatedToCoreDuringLocalPreparation`.

`check --json` always embeds the full schema catalog. A separate switch would keep ordinary check output small. The catalog is still worth shipping.

`mori test` reports pass or fail without an effects diff. This suite passed, so the missing diff did not block the work. A failing money case would be slower to read.

`fmt` argument layout is flat. I would keep comment preservation, underscore preservation, and the refusal to edit invalid source.

The checker stops at the first diagnostic. That is a sound parser bound. The shipped invalid example therefore shows only `BETA_NUMBER`, not the unknown name on the next line. The tutorial should say so.

`predecessor` is required, echoed, and then unused by core. Leaving it unauthenticated matches the unverified binding. The footgun is that a required field looks checked. Document the echo, or show the submitted predecessor on the candidate as unverified. I would not turn it into a silent equality check that looks like authentication.

MCP `preview` versus CLI `simulate` is already documented in `ai/README.md`. The one-line `--help` text does not mention `preview`.

Exit 1 for a well-formed `CoreRejected` is a reasonable process status. It needs one sentence on the first page so a script does not treat the JSON diagnosis as a crash.

## Repair suggestions and uncertainty

If anything is changed, I would add a human `message` beside the existing core code, add `qualification: "local-stipulation-only"` to every expand and simulate object, fix the const hover noun, fix the `/` diagnostic text, and return JSON for an existing `init` target. I would document, in the starter README, economic id versus symbol, balance order, repay fee cap and net floor of zero, accrual-first arithmetic, inclusive windows, the replay key shape, and the fact that source schedule constants do not constrain scenario cells.

I would not weaken the signed bounds, invent default balances, or make SpecifiedOnly actions execute.

Uncertainty. I did not run `npm ci`, `npm run build`, or VS Code VSIX packaging. The dist and CLI were already installed. I did not simulate the other seven shipped family files; I read them and confirmed lending. I did not press quantities near `2^127-1` or `2^128-1`. I did not open a provider adapter in Claude, Codex, or Copilot. I did not ask the tools to sign, prove, or submit anything, so those refusals are the documented product boundary rather than a measured negative test. Neovim activation beyond the shipped headless smoke, and LSP clients other than the local script, were not measured.
