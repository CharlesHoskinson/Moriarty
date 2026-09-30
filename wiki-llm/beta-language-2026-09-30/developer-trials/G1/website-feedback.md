# Tutorial website proposal

Host it as a public Git repository and GitHub Pages. This seat does not deploy it. The site teaches the beta that ran here: Node `v24.21.0`, package `@moriarty-lang/beta` `0.1.0-beta.1`, command `mori`. Command results below are from this seat's command log. The starter atom split `1010 = 1000 + 10` is the expectation in `scratch-init/mori.tests.json`, and `mori test scratch-init` passed that case. Pages marked as not executed stay empty of invented results.

## Landing promise

One sentence at the top:

> Write an explicit transfer, prepare it against a fixture you supply, and read a local candidate that is still unqualified.

Under it, four short claims, in this order:

1. Amounts are exact. `2450.75 USD` at scale 2 is `245075` atoms. Addition does not round.
2. `mori check` judges syntax, names, nominal assets, and whether an action is `LocalS0` or `SpecifiedOnly`.
3. `mori simulate` of a transfer can return `PreparedUnqualified`. That result still lists four external premises. It is a local candidate.
4. The tool does not sign, authenticate a provider, build a proof, or accept a ledger transaction.

Show the help line that this seat got from `mori --help`:

```text
Local financial results remain PreparedUnqualified.
```

Do not illustrate the landing page with an AMM, a bridge, or a signed transaction. Those are later pages, and the first of them does not execute.

## Navigation and page order

Persistent left navigation, one concept per page, in this order:

1. Landing promise
2. Prerequisites and installation
3. First five minutes
4. First transfer and repayment
5. Asset, identity, and unit model
6. Source intent versus untrusted fixture
7. First rejection
8. Bounded project tests
9. Editor and AI setup
10. Support matrix
11. Proof, authentication, and ledger limits
12. API, reference, and next steps

A right-hand rail on every page that runs a command says: qualification `local-stipulation-only`, and the result status of the sample (`PreparedUnqualified`, `AuthoringRejected`, `CoreRejected`, `FormationRejected`, or `Unsupported`). The rail repeats the four premises on any page that shows a successful simulate.

Number the pages in the nav. A reader who jumps to the support matrix without the unit page will misuse economic ids.

## Prerequisites and installation

Prerequisites from this run:

- Node `v24.21.0` satisfied `engines.node` of `>=24`. Print `node -v` in the page.
- The command installed here is `./node_modules/.bin/mori` after a local `file:` install. The package README also describes `npx mori` for a packed `.tgz`. Show both, and say which one the reader's install produced.
- `mori --help` lists `init`, `check`, `fmt`, `inspect`, `expand` / `simulate`, `test`, `lsp`, and `mcp`.

`init` needs an empty path. The first call in this seat wrote `scratch-init/` and returned:

```json
{ "status": "Initialized", "qualification": "local-stipulation-only" }
```

The second call exited 1 with `mori: EEXIST: file already exists, mkdir ...`. Tell the reader to pick a new directory. Do not tell them to delete a directory they did not create.

The generated README says to check, test, and simulate, and that the fixture is an untrusted local stipulation. Keep that sentence on the installation page.

## First five minutes

The page is a timed path, about six commands. Use the starter only as the first path, then say the next page replaces it with a named payroll. This seat's control was `mori test scratch-init`, which passed the case `literal fee payment` at `PreparedUnqualified`.

Commands:

```sh
node -v
mori --help
mori init scratch-init
mori check scratch-init/invoice.mori
mori test scratch-init
mori simulate scratch-init/invoice.mori --action pay --scenario scratch-init/scenario.json
```

Before the simulate block, open the generated `scenario.json` on the page. The file from this seat's init has asset `"A"`, accounts `Owner`, `Recipient`, and `Fee`, and amounts `"10000"`, `"0"`, and `"0"`. The invoice source uses symbol `USD` and the quantities `10.00 USD` and `0.10 USD`. The starter test expectation, which the passing test compared, debits `1010` atoms, credits the recipient `1000`, and credits the fee account `10`.

Say this in one paragraph, next to those two files:

> The scenario asset is the economic id `A`, not the symbol `USD`. The amounts are scale-2 atoms. `10.00` is `1000` atoms and `0.10` is `10` atoms. Gross is `1010`.

That paragraph is the difference between a passing first simulate and a fixture that fails identity or integer checks. The package README shows the `.mori` quantities and does not show this JSON. Init is where a reader first sees it. The website should show it without requiring a read of the compiler.

End the page with `mori inspect scratch-init/invoice.mori` and the sentence that identities come back `authenticated: false`. This seat saw that flag on every payroll identity.

## First transfer and repayment

### Transfer, executed

Use a payroll with two independent intents, not a renamed copy of the invoice. This seat's program is `payroll/payroll.mori`.

Walk one payment in display units and in atoms:

- Alice wage `2450.75 USD` = `245075`
- Alice fee `12.25 USD` = `1225`
- Gross `2450.75 + 12.25` = `2463.00` = `246300`
- `gross_cap` is the sum, `fee_cap` is the fee, `net_floor` is the wage

The profile has no division. Do not write a 0.5 percent fee. `12.25 / 2450.75` is not an exact one-half percent, and the language cannot round it. The fee is an agreed quantity.

The second intent pays Bob `1875.50 USD` plus an agreed fee `9.40 USD` = gross `188490` atoms, with a different recipient, nonce, key, source-hash claim, and pre-head. Show both action lines from `check`:

```text
AuthoringChecked PayrollWeek
  pay_alice: LocalS0
  pay_bob: LocalS0
  adopt_schedule: SpecifiedOnly
Local preparation remains PreparedUnqualified.
```

Then show two simulates. One action, one scenario. Alice's candidate debits `PayrollDesk` `246300`, credits `AliceChen` `245075`, and credits `PayrollProcessor` `1225`. Bob's candidate debits `188490`. The opening balances in the two fixtures are different on purpose. Preparing Alice does not change Bob's file.

Quote the result status and qualification together:

```text
status: PreparedUnqualified
qualification: local-stipulation-only
```

And the premises array from the Alice simulate. Leave the full effect list on the page. Hide the Source/6 origin map behind a disclosure. The map is large and it is a proposal; the candidate comes from simulate.

Point at the replay line. Source/6 contains `use_replay "payroll-2026-09-w39-alice"`. The candidate key is:

```text
["NightPayroll","PayrollDesk","payroll-2026-09-w39-alice"]
```

A reader who copies the Source/6 nonce into `mori.tests.json` will fail the effect comparison.

### Repayment, not executed here

The README names funded AccrualFirst repayment as the other local preparation, also `PreparedUnqualified`. `init` does not generate a repay file. None of the eight files in `examples/` is a repay. This seat did not run `repay`, so this page must not invent a post-state, an effect list, or a passing test.

The page should still exist, and it should say what is missing:

- A complete `.mori` repay: an obligation declaration, a payer who is the signer and the debtor, an amount in the obligation's asset, and the same signed-bound fields as a transfer (`gross_cap`, `fee_cap`, `net_floor`, rounds, empty observations, `SuccessOnly`, `delegation: None`, `recovery: None`).
- A scenario that includes an `obligation` object. The transfer scenarios in this trial were rejected by the schema if they added unknown fields, and they have three balance cells. A repay fixture is a different closed shape. Publish that shape from the tool, with a worked numeric example, only after someone runs it and calculates principal, accrued, and outstanding independently.
- The same four premises on a successful repay candidate.
- A rejection where the fixture's outstanding does not equal principal plus accrued, shown as a real diagnostic, not as a paraphrase.

Until that run exists, the repayment page is a stub with the README sentence and the words "no executed sample in the G1 trial."

## Asset, identity, and unit model

Teach five layers with the payroll names:

| Layer | Payroll example | What the tool did |
| --- | --- | --- |
| Domain | id `NightPayroll`, chain `night-ledger`, network `preview-week-39` | `inspect` authenticated flag is false |
| Account | declaration `Desk`, economic id `PayrollDesk` | Scenario account string is the economic id |
| Asset | symbol `USD`, economic id `PayrollUSD`, scale 2, representation `canonical` | Scenario asset is `PayrollUSD` |
| Quantity | `12.25 USD` | Atoms `1225`. These payroll quantities use two decimals, which scale 2 accepted |
| Intent bounds | gross `246300`, fee `1225`, net `245075` | Checked as `Qty<USD>` at authoring time. Compared with the transfer amounts only during prepare |

`const alice_wage: Qty<USD> = 2450.75 USD` needs a space or a comment between the number and `USD`. Bare decimals without an asset are rejected. Scalars and quantities do not add. Two different assets do not add: that is the next page's rejection.

`min` and `max` exist in the grammar. This payroll did not need them. The README states that there is no division and that intermediates fit an unsigned 128-bit range, with a tighter bound on S0 cap fields. This payroll is far inside those bounds and did not hit them. The page can cite the README there without a giant-integer demo.

Identity claims are records. The economic ids in this payroll are ASCII identifiers such as `PayrollDesk`, and the scenarios repeat those strings. Nonces are strings; this seat used `payroll-2026-09-w39-alice`, including hyphens, and prepare accepted it.

The intent claim `source_hash: "claim-src-alice-w39"` is not the file digest. Put both on this page:

- Claim inside the intent: `claim-src-alice-w39`
- `check` `sourceHash` of the hand-written file: `18efde69e2361cff9a05a2c6ecc38c6e321d462d7bcfc38fdb18287e17214b98`

`fmt` changes the second and does not rewrite the first. After formatting, this seat's Alice effects stayed equal and the digest became `0bae92e2be696c7d607de81daaf1ca831a97a9f9b69181d7b50ea28172bdefd3`.

## Source intent versus untrusted fixture

Two columns.

Left, source, trusted as the author's text and still not a signature. The intent fixes domain, asset, signer, nonce, pre-head, validity rounds `40..52`, caps, and the transfer endpoints. A scenario cannot change the recipient or the fee.

Right, scenario, profile `moriarty-local-scenario/1`, kind `local-stipulation`. Required fields seen in this trial: `domain`, `asset`, `head`, `predecessor`, `round`, `balances`, `allowance`, `replay`, `work_remaining`, `work_spent`, `post_head`. Amounts are canonical decimal strings. `replay` is `unused` or `consumed`. No comment syntax, no duplicate keys, no extra fields.

Balance order for a transfer is the economic ids of `from`, `to`, and `fee_to`. Alice's file is `PayrollDesk`, `AliceChen`, `PayrollProcessor`. Bob's file replaces the middle id with `BobOkoye`. Declaration names `Desk` and `Alice` are the wrong strings.

The allowance owner is the signer id. Work counters are strings. In the Alice and Bob runs, `post_head` differs from `head`, and the candidate head becomes that `post_head`. This seat did not run a fixture where those two strings are equal. Round `44` sits inside source rounds `40..52`, and both successful candidates keep round `44`.

The page should say that a passing `check` has not looked at this file. The stale-head scenario is byte-for-byte the Alice fixture except `head` is `desk-head-stale`, while the intent still says `pre_head` `desk-head-alice-w38`. `check` exits 0. `simulate` returns `CoreRejected`, judgment `history`, code `S0_HISTORY_STALE`, and null effects.

## First rejection

Show three rejections, in this order. All three were run.

### 1. Wrong asset at authoring

`payroll-wrong-asset.mori` declares `BonusPoints` and writes:

```mori
const alice_wage: Qty<USD> = 2450.75 USD;
const alice_fee = 12.25 Points;
const alice_gross = alice_wage + alice_fee;
```

`mori check` exits 1:

```text
AuthoringRejected PayrollWrongAsset
  BETA_ASSET_MISMATCH at 26:23: Addition/subtraction require identical numeric types and nominal assets
```

`fmt --write` exits 1 and does not change the file. Simulate publishes no effects. This is the rejection to teach first, because the message names the rule and the line.

### 2. Fee above the fee cap

`payroll-fee-over-cap.mori` keeps gross equal to wage plus fee, and sets `fee_cap` to `10.00 USD` (`1000` atoms) while the fee stays `12.25 USD` (`1225` atoms).

`mori check` exits 0, `AuthoringChecked`, `pay_alice: LocalS0`.

`mori simulate` exits 1:

```text
status: CoreRejected
judgment: intent
code: S0_INTENT_SCOPE
publishedEffects: null
publishedPost: null
```

Put these two commands in one figure. The lesson is that a green check has not applied the cap inequality.

### 3. SpecifiedOnly is not a soft success

`adopt_schedule` is a `governance.queue` intent in the same agreement. `check` lists it as `SpecifiedOnly`. `expand` and `simulate` exit 1 with `BETA_PROFILE_UNSUPPORTED` and the message `Recognized authoring profile has no local execution`. `publishedEffects` is null. The same status came back for `examples/staking.mori` action `deposit`.

Also show the fixture-asset rejection once: scenario asset `BonusPoints` against source asset `PayrollUSD` is `FormationRejected`, `BETA_SCENARIO_IDENTITY`, message `Scenario domain/asset does not match source (/)`. Tell the reader the current message does not say which field mismatched. That is a tool gap, recorded in `report.md`. The tutorial should still tell the reader to compare economic ids by eye until the diagnostic improves.

## Bounded project tests

One directory, one `mori.tests.json`, profile `moriarty-beta-tests/1`. Paths are relative. `mori test payroll` runs only that manifest. This seat's root `mori test .` exited 1 with `ENOENT` for `G1/mori.tests.json`.

A case has `name`, `source`, `action`, `scenario`, and `expect`. `expect.status` is one of `PreparedUnqualified`, `CoreRejected`, `SourceRejected`, `AuthoringRejected`, `FormationRejected`, `Unsupported`. Optional `code`, `effects`, and `post`.

The page must show one success expectation that a human calculated, not a paste of simulate. Use Alice:

- Debit `246300` of `PayrollUSD` from `PayrollDesk`
- Credit `245075` to `AliceChen`
- Credit `1225` to `PayrollProcessor`
- Allowance use `246300`
- Replay key as the JSON triple above
- Advance head from `desk-head-alice-w38` to `desk-head-alice-w39`
- Post desk balance `4753700`, Alice `260075`, processor `1475`
- Allowance remaining `753700`, spent `446300`
- Work remaining `7`, spent `3`
- `core` `moriarty-core/5`, obligations `[]`

`mori test payroll` then prints `TestsPassed` and `local-stipulation-only`, with `code: null` on the successes.

Also show a negative case object, for example status `AuthoringRejected` and code `BETA_ASSET_MISMATCH`. The wrong-asset source stays in the tree. It is not the file the success cases point at.

Warn about failure output. A throwaway case that expected debit `"1"` produced only:

```json
{ "name": "wrong expected debit", "passed": false, "status": "PreparedUnqualified", "code": null }
```

The tutorial should say the runner does not print a diff yet, so the author has to compare the expectation to a simulate capture by hand. Do not teach people to paste simulate into `expect` to make the red case green.

`fmt` before committing a test corpus changes `sourceHash` and, today, the call layout. It does not change the Alice atoms. Say that, and show the ugly `rounds(domain: Night,` break so people do not treat formatter output as the house style until it indents arguments.

## Editor and AI setup

Split the page into "ran in this trial" and "shipped, not activated here."

Ran:

- A local Content-Length client against `mori lsp`. Server `moriarty-beta` `0.1.0-beta.1`. Full sync, UTF-16. Hover on `pay_alice` reports `Support: LocalS0` and that local preparation does not qualify proofs or settlement. Hover on `Desk` says the identity metadata is an unverified claim. Diagnostics for the Points sum match `check`.
- A local newline JSON-RPC client against `mori mcp`. Initialize, then `notifications/initialized`, then `tools/list`. Tools: `check`, `inspect`, `expand`, `preview`. Each schema has `additionalProperties: false` and `readOnlyHint: true`. `preview` of `pay_alice` returned `PreparedUnqualified` and the four premises. `preview` of `adopt_schedule` returned `isError: true` and `BETA_PROFILE_UNSUPPORTED`. A call that added a `path` argument returned `-32602`.

Not activated here, so the page may describe the shipped files and must not claim this seat saw them work inside VS Code or Neovim:

- VS Code packaging in `editor/README.md` (`prepare.mjs`, `vsce`, Node on `PATH` or `moriarty.nodePath`). The README itself says packaging does not establish activation.
- Neovim `editor/nvim/moriarty.lua` and `smoke.lua` for Neovim 0.11+, started with an absolute CLI path. This seat did not run that smoke.
- Snippets: agreement, asset, action. The asset snippet uses id `"A"`. There is no snippet for a transfer intent. The website's copy-paste block should be a full intent, not that snippet.
- AI adapters under `ai/adapters/` and `ai/AGENTS.md`. They are inert until a project owner copies one in. Copy `AGENTS.md` with the adapter. Do not present them as installed intelligence. The check skill says to pass source text, not a path, and to keep premises in the answer. `ai/examples/valid.mori` is scalar arithmetic, not a payment.

Recommended client entry, as documentation only:

```json
{ "command": "node", "args": ["/absolute/path/to/dist/cli.js", "mcp"] }
```

The host schema differs. The page should say the project owner turns that on. The tutorial repo must not commit a user's global client config.

## Support matrix

Build the matrix from `check`'s action support and from executed commands. Mark the rows this seat did not simulate.

| Action class | Authoring | This seat's execution |
| --- | --- | --- |
| `transfer` | `LocalS0` | `PreparedUnqualified` for Alice and Bob. `S0_INTENT_SCOPE` when the fee exceeds the cap. `S0_HISTORY_STALE` when the fixture head disagrees with `pre_head` |
| `repay` | `LocalS0` in the catalog | Not run. Do not fill the result column |
| `governance.queue` as `adopt_schedule` | `SpecifiedOnly` | `BETA_PROFILE_UNSUPPORTED`, no effects |
| `staking.deposit` in `examples/staking.mori` | `SpecifiedOnly` | Same unsupported result |
| Other dotted families in the eight examples (AMM, lending, stablecoin, option, oracle, the rest of governance, bridge, the rest of staking) | SpecifiedOnly in those files and in the schema catalog | Not simulated one by one. The example README says expansion and simulation reject them as `SpecifiedOnly` |

`check` text for a Local S0 file adds `Local preparation remains PreparedUnqualified.` SpecifiedOnly actions do not get a local candidate.

Open gates printed on the payroll check: `authentication`, `nativeProof`, `financialCorrespondence`, `atomicLedgerAcceptance`.

## Proof, authentication, and ledger limits

One page, short, using the Alice result as the picture.

The candidate includes `requiredPremises`:

1. `canonical-intent-signature`
2. `snapshot-to-head`
3. `head-extension`
4. `atomic-ledger-compare-and-consume`

And `unverifiedBindings`:

1. `agreement-id`
2. `selected-program`
3. `asset-scale`
4. `authenticated-predecessor`

`inspect` set `authenticated` to false on `Night`, `Desk`, `Alice`, `Bob`, `Processor`, and `USD`. The key string `desk-key-alice-w39` is a claim inside the intent. This trial never used it as a signing key. The website must not add a button, a CLI flag, or a paragraph that produces a signature.

A matching test does not close the premises. MCP `preview` does not close them. The library `simulate` result carries the same list.

The financial horizon in the README (AMMs, lending, stablecoins, options, oracles, governance, bridges, staking) is authoring coverage. It is not a second execution engine. Link that matrix. Do not teach a swap on this page.

The wiki's typed lifecycle grammar is out of beta syntax. One sentence is enough, aimed at people who will search for it and try to paste it.

## API, reference, and next steps

Public JS import, used in this seat:

```js
import { check, format, inspect, expand, simulate } from "@moriarty-lang/beta";
```

Inputs are strings. `simulate(sourceText, "pay_alice", scenarioText)` returned `PreparedUnqualified`, qualification `local-stipulation-only`, and Alice's debit of `246300`. `expand` of `adopt_schedule` returned `Unsupported` and `publishedEffects: null`. `format` returned different text and did not return null for the valid payroll.

Reference index, each entry a heading and the diagnostic or field this trial actually saw:

- Profiles: `moriarty-beta/1`, `moriarty-local-scenario/1`, `moriarty-beta-tests/1`
- Statuses: `AuthoringChecked`, `AuthoringRejected`, `Expanded`, `PreparedUnqualified`, `CoreRejected`, `FormationRejected`, `Unsupported`, `TestsPassed`, `TestsFailed`, `Initialized`
- Codes: `BETA_ASSET_MISMATCH`, `BETA_PROFILE_UNSUPPORTED`, `BETA_SCENARIO_IDENTITY`, `BETA_SCENARIO_SCHEMA`, `BETA_JSON_DUPLICATE`, `S0_INTENT_SCOPE`, `S0_HISTORY_STALE`
- CLI exits: 0 for check/simulate/test success as defined above; 1 for rejections, usage errors, `EEXIST`, and `ENOENT`

Next steps for a reader who finished the payroll:

1. Read the premises on their own simulate output and leave them in any note they publish.
2. Add a second scenario when they add a second intent. Do not reuse Alice's balances for Bob unless they mean that snapshot.
3. Keep an invalid file, and point a test case at it, so `mori test` fails if the wrong-asset sum starts to typecheck.
4. Wait for a worked repay example before teaching obligation settlement.
5. Do not format-and-commit if something else has hashed the source bytes, until they accept the new digest.

## What to omit from getting started

Leave these off the first three pages. Link them from the support matrix or the limits page.

- Signing, key management, proof objects, provider login, and any "send" or "deploy" step. The product does not do those, and a getting-started page that shows a fake signature teaches the wrong result.
- The full operation catalog and the eight DeFi sketches. They typecheck and do not prepare.
- Source/6 origin maps, field maps, and the generated source text, until after one successful simulate and one rejection.
- VS Code packaging, `vsce`, and Neovim smoke commands.
- Copying `CLAUDE.md`, `moriarty.mdc`, or an MCP config into a global client.
- Quint, K correspondence, and the wiki lifecycle grammar.
- Percentage or pro-rata payroll math.
- A single fixture that is supposed to pay two people atomically.
- Instructions to paste `simulate` JSON into `expect` without an independent atom calculation.
- `check --json` as the first success screen. It is about 31 KB here because it inlines every schema. Show the text `check` first.

Keep these on the first transfer page even though they look internal: economic id versus symbol, atom strings, three ordered balances, gross equals value plus fee, and the `PreparedUnqualified` premises. A reader cannot get a honest green simulate without them.

## Tutorial acceptance checklist

A change to the tutorial is ready when a reviewer can tick every line against a fresh init and against the payroll samples.

- [ ] Landing page states `PreparedUnqualified` and shows the help line. It does not say the payment settled.
- [ ] Install steps include Node `>=24`, the help command, and the `EEXIST` behavior of a second `init`.
- [ ] First five minutes runs `init`, text `check`, `test`, and `simulate` on the starter, and shows asset id `A` beside symbol `USD` and atoms `1010 = 1000 + 10`.
- [ ] The worked transfer is not a renumbered invoice. It has explicit wage, fee, and gross, with atoms computed in the page.
- [ ] Two intents with different recipients and nonces are two scenarios. The page shows both statuses.
- [ ] The scenario figure uses economic ids and atom strings, in from / to / fee-to order, with no extra fields.
- [ ] `source_hash` the claim and `sourceHash` the file digest are both on the page, with different values.
- [ ] The replay section shows the nonce line and the JSON triple.
- [ ] Wrong-asset authoring shows `BETA_ASSET_MISMATCH` at a real line, and `fmt` does not rewrite that file.
- [ ] Fee-over-cap shows `check` exit 0 and `simulate` `S0_INTENT_SCOPE` with null effects.
- [ ] Stale head shows `S0_HISTORY_STALE` with null effects.
- [ ] SpecifiedOnly shows `BETA_PROFILE_UNSUPPORTED` and null effects for at least one action.
- [ ] `mori test` uses profile `moriarty-beta-tests/1`, passes a hand-calculated expectation, and documents that a mismatch report has no diff.
- [ ] Repayment is either an executed sample with independent arithmetic, or an explicit stub. It is not a fabricated post-state.
- [ ] Editor and AI instructions separate this trial's stdio clients from VSIX, Neovim, and adapter adoption.
- [ ] The support matrix marks rows that were not executed.
- [ ] The limits page lists the four premises and four bindings and has no signing walkthrough.
- [ ] Getting started omits the items in the previous section.
- [ ] Sample commands match the CLI that `mori --help` prints. Paths in the tutorial repo exist.
- [ ] No page tells the reader to weaken a cap, change a recipient, or copy tool output into an expectation to force a pass.

## Layout and framework

Use a documentation site with a fixed left nav, a content column about 72 characters wide, and the status rail on the right. Pages are Markdown or MDX so the command output stays diffable in Git.

Astro Starlight fits that layout and deploys to GitHub Pages. MkDocs Material is a smaller alternative if the authors want a Python docs repo. Either is enough. Do not build a custom application shell. The content is the product.

Typography: one monospace face for commands and diagnostics, one text face for the lesson. Put status words (`PreparedUnqualified`, `SpecifiedOnly`, `Unsupported`) in the monospace face so they match the tool. Avoid a dashboard, a card grid of DeFi products, or a hero animation.

Repository shape:

```text
docs/
  index.md
  install.md
  first-five-minutes.md
  transfer-and-repayment.md
  units.md
  fixture.md
  rejection.md
  tests.md
  editor-and-ai.md
  support-matrix.md
  limits.md
  reference.md
examples/
  payroll.mori
  scenarios/
  mori.tests.json
```

The examples directory should be runnable with `mori test examples`. The tutorial pages quote those files. When a sample and a page disagree, the page is wrong.

Ship the site from `main` with GitHub Pages. Review uses the checklist above. This trial stops at the proposal.
