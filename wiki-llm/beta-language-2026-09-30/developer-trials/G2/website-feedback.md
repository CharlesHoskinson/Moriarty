# Tutorial website proposal

Host a small documentation site from Git. GitHub Pages is enough. The site teaches local authoring of one transfer and one repayment. It should say, in the first screen, what this beta prepares and where it stops.

Suggested landing line:

> Write a transfer or a loan repayment with exact amounts and an explicit round window. Check it, then prepare an unqualified local result. Moriarty beta 0.1.0-beta.1 does not sign, authenticate a provider, generate a proof, or accept a ledger transaction.

Do not use a wallet button, a yield figure, or a "deploy lending" call to action. The product that exists is a local authoring CLI.

## Navigation and page order

Use one sidebar, in this order. A persistent banner on every page can repeat: local S0 results are `PreparedUnqualified`.

1. Landing promise
2. Prerequisites and installation
3. First five minutes
4. Asset, identity, and unit model
5. First transfer
6. First repayment
7. Source intent versus untrusted fixture
8. First rejection
9. Bounded project tests
10. Editor and AI setup
11. Support matrix
12. Proof, authentication, and ledger limits
13. API and reference
14. Next steps

A short "Leave this for later" box can live on the landing page and again before the support matrix. The omit list below is that box.

## Prerequisites and installation

State Node 24 or newer first. This trial ran Node v24.21.0 against the installed package `@moriarty-lang/beta` 0.1.0-beta.1. The command used was `./node_modules/.bin/mori`. The package README also describes `npm ci`, `npm run build`, and `npx mori` for a packed tarball. This trial did not re-run the build. The install page should show both the tarball path and the local `mori` binary, and it should say the package includes the S0 modules so a separate compiler install is not required.

`mori --help` is one line:

```text
Moriarty beta: init DIR | check FILE [--json] | fmt FILE [--write] | inspect FILE | expand/simulate FILE --action NAME --scenario FILE | test DIR | lsp | mcp
Local financial results remain PreparedUnqualified.
```

That line is not enough to write a scenario. The install page should link straight to the scenario shape instead of telling the reader to discover it from `init` alone.

Record this session fact somewhere small, not as product behavior: if both `FORCE_COLOR` and `NO_COLOR` are set, Node prints a warning on stderr. The CLI result is still on stdout. Do not document that warning as a Moriarty error.

## First five minutes

The first path is the starter, then a stop to read the status word.

```sh
mori init note-hello
mori check note-hello/invoice.mori
mori test note-hello
mori simulate note-hello/invoice.mori --action pay --scenario note-hello/scenario.json
```

This trial did that in `loan-desk` before replacing the starter. `init` printed `Initialized` and `qualification: "local-stipulation-only"`. `mori test` printed `TestsPassed` for `literal fee payment` with status `PreparedUnqualified`. The starter scenario uses domain id `Midnight`, asset id `A`, and account ids `Owner`, `Recipient`, and `Fee`. The display symbol `USD` is not the asset id. The payment is 10.00 USD plus a 0.10 USD fee, so the prepared debit is 1010 atoms and the credits are 1000 and 10. Say that in atoms on this page so the scale is visible before the reader writes a loan.

`init` refuses an existing directory. Today that is a raw `EEXIST` string on stderr, exit 1, and no JSON. Tell the reader to pick a new directory. Also tell them exit 1 on `simulate` can mean a diagnosed refusal with a JSON body on stdout, not a crashed process.

`fmt FILE` prints the formatted source. `fmt FILE --write` prints nothing on success. A second format of the note agreement was byte-identical. `fmt` does not rewrite an invalid file.

Stop the five-minute page after the starter payment prepares. Do not introduce lending, interest, or MCP there.

## First transfer and first repayment

Keep the starter transfer as the first money page. Then add one repayment page using a closed note, with the numbers this trial prepared.

Use scale 2. Show the atom conversion next to every display amount.

Draw, action `disburse`, window rounds 10 through 20, scenario round `"10"`:

- `1_250.00 USD` is 125000 atoms. Fee `12.50 USD` is 1250. Gross debit is 126250.
- Three balances, in order: Lender 126250, Borrower 0, Servicer 0.
- After preparation: Lender 0, Borrower 125000, Servicer 1250.
- Allowance owner is the signer, Lender. Remaining 126250 and spent 0 become remaining 0 and spent 126250.
- Work remaining 8 and spent 0 become 7 and 1. The round stays 10. Head `svc-head-10` becomes `svc-head-11`.

Installment, action `service_installment`, window 44 through 52, scenario round `"52"`:

- Obligation principal 100000, accrued 1875, outstanding 101875, status `Outstanding`.
- Payment 21875. Accrual-first takes 1875 of accrued, then 20000 of principal.
- Post obligation: principal 80000, accrued 0, outstanding 80000, status `Outstanding`.
- Balances, in order: Borrower then Lender. Borrower 21875 becomes 0. Lender 0 becomes 21875.
- Fee cap and net floor in source are `0.00 USD`. Gross cap equals the payment.
- Work 3 and 4 become 2 and 5. Round stays 52.

Payoff page, only after those two succeed. Window 80 through 88, round `"80"`, principal 80000, accrued 625, payment 80625. Post status `Settled`, all three figures 0. Work 1 and 9 become 0 and 10. Say in the same paragraph that this payoff fixture is a new stipulation. It is not produced by feeding the installment post back into the tool.

The expand excerpt readers should see for the installment:

```text
selected RepayAccrualFirst source_hash "claim-installment-source" digest "claim-service-policy";
valid 44..52;
gross_cap 21875;
fee_cap 0;
net_floor 0;
signed_action repay obligation NoteSeven payer Borrower amount 21875 conversion identity;
```

`source_hash` in that line is the claim written in the `.mori` file. It is not the SHA-256 printed by `inspect`. For the formatted note, `inspect` reported `c47dfcdcbbd4cfbe49b8d1b2e753414c4ee77b40470568a1f635febb365ce176`.

## Asset, identity, and unit model

Teach four different strings before any loan:

| Text | What it is in the note |
| --- | --- |
| Declaration name `Desk` | Source binder |
| Domain id `DeskNet` | Scenario `domain` and replay key |
| Declaration name `USD` | Quantity suffix, as in `12.50 USD` |
| Asset id `UsdNote` | Scenario `asset` and effect `asset` |
| Symbol `USD` | Display metadata. Rejected if used as the scenario asset |
| Obligation name `Note` versus id `NoteSeven` | Scenario obligation id is `NoteSeven` |

Rules the repayment page will hit immediately:

- Scale is an integer 0 through 18. Scale 2 means two fractional digits, no rounding.
- `12.505 USD` at scale 2 is `BETA_PRECISION` (`invalid-precision.mori`, CLI position 10:23).
- Bare numbers are unsigned scalars. A decimal without an asset is rejected.
- The same nominal asset is required for addition and subtraction. `min` and `max` exist. Division does not. `1 / 2` is `BETA_CHARACTER`.
- S0 money fields must fit `2^127-1`. The note amounts are far inside that bound. This trial did not press the bound.
- Identity records are claims. `inspect` marks them `authenticated: false`.
- Transfer effects use economic ids, and the debit amount is value plus fee.

## Source intent versus untrusted fixture

Put this on its own page. It was the largest modeling surprise.

The `.mori` file fixes the action: who pays, how much, the window, and the caps. The scenario file supplies balances, the obligation snapshot, replay, work, heads, and the round. There is no default balance and no default spent counter. Unknown JSON fields and duplicate keys are rejected. A scenario cannot override the source amount.

The note source names accrued `18.75 USD` and principal `1_000.00 USD`, and it computes the accrual-first split with `min`. Those constants are checked arithmetic. They do not constrain the scenario. A probe that set accrued to 5000 atoms and outstanding to 105000, with the same 21875 payment, prepared principal 83125, accrued 0, outstanding 83125. That is `100000 - (21875 - 5000)`. The source comment still said 80000. Show both outputs side by side.

Also show the overpayment expand. Outstanding 10000 against payment 21875 stays `Expanded`, with the note that the proposal preserves the unchanged obligation and that only core decides the failure. The proposal text still contains `debit Borrower 21875` and `set_obligation ... outstanding 10000`. The following `simulate` is `CoreRejected`, `S0_EFFECT_RANGE`, `publishedEffects: null`. Tell the reader to treat `Expanded` as a proposal.

`predecessor` is required in the scenario and appears in Source/6. Replacing it with `unrelated-predecessor` still prepared, and the candidate post has no predecessor field. The unverified binding name is `authenticated-predecessor`. Say that on this page so nobody treats a successful prepare as a check of chain history.

## First rejection

Use three fixtures that differ from the successful installment by one field, plus the authoring rejection.

| Change | Status | Judgment | Code | Exit |
| --- | --- | --- | --- | --- |
| `replay: "consumed"` | CoreRejected | history | `S0_HISTORY_REPLAY` | 1 |
| Borrower balance `"21874"` | CoreRejected | effect | `S0_EFFECT_RANGE` | 1 |
| `work_remaining: "0"` with allowance still 21875 | CoreRejected | authority | `S0_AUTH_SCOPE` | 1 |
| Round `"53"` after window 44..52 | CoreRejected | intent | `S0_INTENT_SCOPE` | 1 |
| `rounds` from 90 to 44 | AuthoringRejected |  | `BETA_ROUND_WINDOW` | 1 |

Windows are inclusive. Round `"44"` and round `"52"` prepared. Round `"43"` and round `"53"` did not.

Say, next to the table, that several different mistakes share a code. Overpayment is also `S0_EFFECT_RANGE`. A zero allowance is also `S0_AUTH_SCOPE`. A nonzero repay fee cap is also `S0_INTENT_SCOPE`, and `check` accepts that fee cap. The JSON does not contain a sentence naming which comparison failed. Readers should diff their scenario against the successful one.

Every core rejection in this trial had `publishedEffects: null` and `publishedPost: null`.

Specified-only refusal belongs here too, as a different kind of no:

```text
Unsupported
BETA_PROFILE_UNSUPPORTED
Recognized authoring profile has no local execution
publishedEffects: null
```

`originate`, `roll`, and `liquidate` on the note did this. Shipped `examples/lending.mori` action `borrow` did this. `roll` did this even when the scenario file was not JSON. Syntax checking does not judge collateral, evidence freshness, or a roll's financial rule.

## Bounded project tests

Show `mori.tests.json` with profile `moriarty-beta-tests/1`. Each case has `name`, `source`, `action`, `scenario`, and `expect`. Paths are relative to the directory passed to `mori test`. `mori test` reads that one manifest. Extra `.mori` files and `probes/` are ignored until a case names them. This trial kept invalid programs in the manifest with `expect.status` of `AuthoringRejected`, and kept one-off probes out of it.

`expect.status` is one of `PreparedUnqualified`, `CoreRejected`, `SourceRejected`, `AuthoringRejected`, `FormationRejected`, `Unsupported`. Optional `code`, `effects`, and `post` are exact. Effects and post are compared only for `PreparedUnqualified`.

Tell the reader to compute atoms on paper before running `simulate`, then type `effects` and `post`. This trial's first test passed with those hand-written figures. Copying a simulate body into `expect` would hide a wrong schedule. The success report looks like:

```text
TestsPassed
qualification: local-stipulation-only
disburse-exact passed true status PreparedUnqualified code null
```

The post object uses core names: `workRemaining`, `workSpent`, `allowances`, `consumedReplay`, `core: "moriarty-core/5"`. The scenario file uses `work_remaining`, `work_spent`, and a single `allowance`. Document that rename.

The replay key in `effects` is a JSON text of three parts, domain id, signer id, and nonce. For the installment it is `["DeskNet","Borrower","installment-3"]`.

## Editor and AI setup

Separate what this trial ran from what it did not.

Ran:

- LSP stdio against `mori lsp`, full sync, UTF-16. Hover, definition, document symbols, and diagnostics worked on the note and on the inverted window. CLI position 14:12 is LSP line 13, character 11.
- MCP stdio against `mori mcp`. Tools are `check`, `inspect`, `expand`, and `preview`. Send `notifications/initialized` before `tools/list`. `preview` is the simulate tool. A roll preview is `isError: true` and has no effects.
- The shipped headless Neovim smoke from the package: `MORIARTY_BETA_CLI="$PWD/dist/cli.js" nvim --headless -u NONE -l editor/nvim/smoke.lua`. It passed. Its message went to stderr. It does not load the reader's loan file.

Did not run, so the site must not claim them:

- VS Code activation or a VSIX install. The editor README already says packaging does not establish activation.
- Copying `ai/adapters/*` or `AGENTS.md` into a project. Those files are inert until an owner asks for them. No provider in this trial was configured.
- Any statement that an AI completion authenticates a scenario.

Editor page facts worth teaching: semantic tokens and debugging are absent; a full change over the size limit drops analysis until a later valid full change; formatting can change source bytes and therefore the `inspect` file hash, and it does not rewrite claimed `source_hash` strings.

Hover today says `identity metadata is an unverified claim` even for a `const` quantity. The site can say identities are unverified. It should not repeat the const hover noun as if a quantity were an identity record.

## Support matrix

Columns: syntax check, local preparation, financial rule, authentication, ledger. Fill only cells this trial measured, and mark the rest as documented by the package examples rather than re-executed.

| Action | Syntax | Local preparation | Financial rule in this beta | Authentication | Ledger |
| --- | --- | --- | --- | --- | --- |
| `transfer` (`disburse`) | Checked | `PreparedUnqualified` | Literal value and fee. Core checks caps, balance, allowance, work, replay, head | Open. Premise `canonical-intent-signature` | Open |
| `repay` (`service_installment`, `close_note`) | Checked | `PreparedUnqualified` | Accrual-first, identity conversion, fee cap 0, net floor 0 | Open | Open |
| `lending.originate`, `lending.roll_forward`, `lending.liquidate` | Checked on the note and on `examples/lending.mori` | `Unsupported`, no effects | Open. A scenario does not supply the missing rule | Open | Open |
| AMM, stablecoin, option, oracle, governance, bridge, staking examples | Present in the eight example files and in the 30 `check` schemas | Documented SpecifiedOnly. Not simulated in this trial | Open | Open | Open |

`check` coverage for the local repay was `financialRelations: "DelegatedToCore"` and `localPreparation: "AvailableUnqualified"`. `inspect` uses the longer phrase `DelegatedToCoreDuringLocalPreparation`. The matrix should pick one phrase and mention the other so readers do not think they differ.

## Proof, authentication, and ledger limits

Give this page the four premises and four bindings from `inspect`, verbatim. State that tests, matching hashes, MCP, and LSP close none of them.

Also state the limits that showed up while servicing the note:

- `PreparedUnqualified` is the success status of local preparation.
- Key, nonce, `source_hash`, and `policy_digest` in source are nonempty claims. They are not checked against a key store or against the file hash.
- One allowance row, two balance rows for repay, three for transfer.
- Replay is only unused or consumed for this nonce.
- Work is a single counter, decreased by 1 on success.
- The round is not advanced.
- Interest is an input cell. The language has no rate and no division.
- A repay cannot pay a fee inside the repay action.
- Specified-only families do not become executable by adding a careful scenario.
- Native settlement and a general K/Quint correspondence remain open. The README says the wiki lifecycle grammar is not beta syntax. Do not teach it as legal `.mori`.

## API, reference, and next steps

Show the text API from the README, because this trial called it:

```js
import { check, simulate } from "@moriarty-lang/beta";
const report = check(sourceText);
const local = simulate(sourceText, "service_installment", scenarioJsonText);
```

The call returned the same `PreparedUnqualified` installment and the same `Unsupported` roll as the CLI (`outputs/53-api-preview.stdout`).

Reference pages, after the tutorial, can list command exit codes, the scenario field table, effect kinds (`Debit`, `Credit`, `SetObligation`, `UseAllowance`, `UseReplay`, `AdvanceHead`), and the core codes seen here: `S0_HISTORY_REPLAY`, `S0_HISTORY_STALE`, `S0_HISTORY_SUCCESSOR`, `S0_EFFECT_RANGE`, `S0_AUTH_SCOPE`, `S0_INTENT_SCOPE`, `BETA_PROFILE_UNSUPPORTED`, `BETA_ROUND_WINDOW`, `BETA_PRECISION`, `BETA_SCENARIO_SCHEMA`, `BETA_SCENARIO_CELLS`, `BETA_SCENARIO_IDENTITY`, `BETA_SCENARIO_INTEGER`, `BETA_JSON_DUPLICATE`.

Next steps for a reader who finished the note: add their own window and their own atoms, run `mori test`, and read `inspect` before they describe the result to anyone else. Next steps for the language are outside this site's getting-started path: messages on shared core codes, `qualification` on every result object, and documentation that `predecessor` is echoed and unverified.

## What to omit from getting started

- Signing, provider login, proof generation, and ledger submission.
- Any reading of `PreparedUnqualified` or `Expanded` as settled money.
- Specified-only origination, liquidation, AMMs, oracles, bridges, staking, options, and governance as if they run.
- The wiki typed lifecycle grammar.
- Interest rates, APR, rounding modes, and partial-cent quantities.
- Default balances, default work, or a hidden spent counter.
- Chaining one simulate post into the next scenario and calling that a ledger history.
- Copying `simulate` JSON into `expect` without an independent atom calculation.
- Global editor install, MCP provider setup, and VS Code activation.
- The full 30-schema JSON from `check --json`. Point to `check` text mode and `inspect` instead.
- Claims that an AI adapter was evaluated. The package says that evaluation has not been done.

## Tutorial acceptance checklist

A change to the tutorial is ready when a new reader can do all of the following on a machine with Node 24 and the beta tarball, without asking what a field means:

- [ ] Install or unpack the beta and run `mori --help` without building a separate compiler.
- [ ] `init` a new directory, `check` the invoice, and `test` it.
- [ ] Point at asset id `A` versus symbol `USD` in the starter scenario.
- [ ] Say that the starter result is `PreparedUnqualified` and name the four premises from `inspect`.
- [ ] Explain exit 1 with a JSON body on stdout.
- [ ] Convert `218.75 USD` at scale 2 to 21875 atoms without using the CLI.
- [ ] Prepare the starter transfer and one accrual-first repayment whose post obligation they predicted.
- [ ] Show that a scenario accrued figure different from the source constant changes the post principal.
- [ ] Produce `S0_HISTORY_REPLAY`, `S0_EFFECT_RANGE`, and `S0_AUTH_SCOPE` by editing one scenario field each, and see null effects.
- [ ] Reject an inverted `rounds` window at authoring time, and see `fmt --write` leave that file unchanged.
- [ ] Run `mori test` on a manifest they wrote, with effects typed from their arithmetic.
- [ ] Simulate `lending.roll_forward` or the shipped `borrow` action and get `BETA_PROFILE_UNSUPPORTED` with no effects.
- [ ] Avoid describing the result as signed, proven, or accepted on a ledger.

## Layout

Use a static documentation theme with a left sidebar and no application chrome. Astro Starlight fits: Markdown pages, a fixed order, search, and GitHub Pages. A plain `docs/` tree on GitHub Pages is enough if a build is unwelcome.

Keep the type and color quiet. One monospaced face for `.mori` and JSON, one text face for the lesson. Put a status pill on prepared examples (`PreparedUnqualified`) and a different pill on refusals (`CoreRejected`, `Unsupported`). Do not illustrate the site with token icons or pool diagrams.

Ship the worked note as a fixture repository, or as a folder beside the docs, whose CI runs `mori test`. The website itself should not run a shared simulator in version 1. Readers prepare locally. If a later page embeds a simulator, label it as local and unqualified, and keep it off the first five minutes.

This proposal does not include deploying the site.
