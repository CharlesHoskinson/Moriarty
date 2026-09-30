# Payroll trial, seat G1

Moriarty beta `0.1.0-beta.1` on Node `v24.21.0`. The assignment program is an original week-39 payroll with two Local S0 transfer intents. Expected money was calculated before `simulate` and before `mori test`. The tool agreed. No source repair was required.

`PreparedUnqualified` here means a local candidate with four external premises still open. The premises are `canonical-intent-signature`, `snapshot-to-head`, `head-extension`, and `atomic-ledger-compare-and-consume`. The unverified bindings are `agreement-id`, `selected-program`, `asset-scale`, and `authenticated-predecessor`. This seat did not sign, prove, send, or submit anything to a ledger.

## Artifact list

Assignment sources, under `payroll/`:

| File | Role |
| --- | --- |
| `payroll.mori` | Two transfer intents, `pay_alice` and `pay_bob`, plus SpecifiedOnly `adopt_schedule` |
| `payroll-wrong-asset.mori` | Invalid gross sum of `Qty<USD>` and `Qty<Points>` |
| `payroll-fee-over-cap.mori` | Authoring-valid Alice transfer whose fee exceeds `fee_cap` |
| `scenario-alice.json` | Closed local fixture for Alice |
| `scenario-bob.json` | Independent closed fixture for Bob |
| `scenario-alice-stale-head.json` | Same cells as Alice, head `desk-head-stale` |
| `scenario-wrong-asset.json` | Same cells as Alice, asset economic id `BonusPoints` |
| `mori.tests.json` | Seven cases. This is the only assignment manifest |

`mori test payroll` is the assignment run. `mori test .` fails because `G1/mori.tests.json` does not exist.

`scratch-init/` is the unmodified `mori init` result: `invoice.mori`, `scenario.json`, `mori.tests.json`, `README.md`. It is a control, not the payroll solution. `mori test scratch-init` passes the starter case `literal fee payment`.

`command-log.json` has 41 entries. Captures are under `outputs/`. Hand arithmetic is `outputs/00-arithmetic.json`. The hand-written payroll sha256 is `18efde69e2361cff9a05a2c6ecc38c6e321d462d7bcfc38fdb18287e17214b98`. The wrong-asset file sha256 is `0f19b1a8e2d7c08a55e09b2810071207892581dbb3c6aa551f3fc417c605ff5e`.

## What the payroll says

Domain claim `NightPayroll` on chain `night-ledger`, network `preview-week-39`. Accounts: `PayrollDesk`, `AliceChen`, `BobOkoye`, `PayrollProcessor`. Asset economic id `PayrollUSD`, symbol `USD`, scale 2, representation claim `canonical`. `inspect` marks every identity `authenticated: false`.

Fees are agreed quantities. The profile has no division, so a percentage fee cannot be written. Scale-2 atoms:

| Person | Wage | Fee | Gross |
| --- | ---: | ---: | ---: |
| Alice | 2450.75 USD = 245075 | 12.25 USD = 1225 | 2463.00 USD = 246300 |
| Bob | 1875.50 USD = 187550 | 9.40 USD = 940 | 1884.90 USD = 188490 |

Each intent sets `gross_cap` to that sum, `fee_cap` to the fee, and `net_floor` to the wage. The intents differ in recipient, nonce, key, source-hash claim, and pre-head. The signer is the same desk. Each transfer's three accounts are distinct.

Alice nonce `payroll-2026-09-w39-alice`, pre-head `desk-head-alice-w38`. Bob nonce `payroll-2026-09-w39-bob`, pre-head `desk-head-bob-w38`. The intent field `source_hash` is the claim `claim-src-alice-w39` or `claim-src-bob-w39`. That string is not the sha256 `sourceHash` of the file.

`adopt_schedule` queues policy `Week39Schedule` from epoch 39 to 40. It is SpecifiedOnly.

The two fixtures are independent. Alice does not spend Bob's opening balances. Alice opens at desk `5000000`, employee `15000`, processor `250`, allowance remaining `1000000` spent `200000`, work remaining `8` spent `2`, round `44`. Bob opens at desk `3200000`, employee `0`, processor `8800`, allowance remaining `800000` spent `50000`, work remaining `5` spent `1`, round `44`.

## Tested outcomes

`mori test payroll` exit 0, `TestsPassed`, qualification `local-stipulation-only`. Repeated after the probes, same seven passes (`outputs/39-test-again.out`).

| Case | Status | Code | Money check |
| --- | --- | --- | --- |
| pay Alice week 39 | PreparedUnqualified | none | Debit 246300. Credits 245075 and 1225. Post desk 4753700, Alice 260075, processor 1475. Allowance 753700 / 446300. Work 7 / 3. Head `desk-head-alice-w39` |
| pay Bob week 39 | PreparedUnqualified | none | Debit 188490. Credits 187550 and 940. Post desk 3011510, Bob 187550, processor 9740. Allowance 611510 / 238490. Work 4 / 2. Head `desk-head-bob-w39` |
| reject gross sum across USD and points | AuthoringRejected | BETA_ASSET_MISMATCH | No effects |
| reject fee above fee cap | CoreRejected | S0_INTENT_SCOPE | No effects. `check` of this file exits 0 |
| reject stale payroll head | CoreRejected | S0_HISTORY_STALE | No effects. `check` of the source exits 0 |
| reject scenario asset that differs from source | FormationRejected | BETA_SCENARIO_IDENTITY | No effects |
| schedule adoption is specified only | Unsupported | BETA_PROFILE_UNSUPPORTED | No effects |

Alice conservation: credits 245075 + 1225 = debit 246300. Allowance 1000000 + 200000 = 753700 + 446300 = 1200000. Work 8 + 2 = 7 + 3 = 10. Round stays `44`. Bob conservation: 187550 + 940 = 188490. Allowance 800000 + 50000 = 611510 + 238490 = 850000. Work 5 + 1 = 4 + 2 = 6.

Replay keys in the candidate are `["NightPayroll","PayrollDesk","payroll-2026-09-w39-alice"]` and the Bob nonce twin. Obligations stay empty.

Local status: both payments are `LocalS0` and the successful simulates are `PreparedUnqualified` with qualification `local-stipulation-only`. Specified status: `adopt_schedule` and the shipped `staking.mori` action `deposit` are `SpecifiedOnly`; expand and simulate return `BETA_PROFILE_UNSUPPORTED` with `publishedEffects: null`. Open status: `check` reports `evidence: AuthoringOnly` and open gates `authentication`, `nativeProof`, `financialCorrespondence`, `atomicLedgerAcceptance`. `inspect` reports authentication open and ledger commit open.

`check` on `payroll-fee-over-cap.mori` exits 0. `simulate` exits 1 with `S0_INTENT_SCOPE`. The fee is 1225 atoms and `fee_cap` is 10.00 USD = 1000 atoms. Gross cap and net floor still match the wage and the fee. Structural checking accepts the file. The financial bound is applied only in local preparation, and that preparation publishes nothing.

The stale-head fixture uses the same source. `check` cannot see the fixture head, so it cannot judge freshness.

The public JS API (`check`, `inspect`, `simulate`, `expand`, `format`) returned the same Alice debit, the same premises, and the same schedule rejection as the CLI (`outputs/35-library-api.json`).

## Syntax, units, accounts, assets, intents, scenarios

The README's quantity syntax worked on the first check: `2450.75 USD` needs whitespace before the asset name, scale 2 accepts two decimals, and `alice_wage + alice_fee` is exact integer atoms. `9.40` is a canonical spelling. A leading zero such as the shipped `ai/examples/invalid.mori` value `01` is rejected by the grammar; this payroll did not need that case.

Declaration names (`Desk`, `Alice`) are not the scenario account strings. The scenario must repeat the economic id (`PayrollDesk`, `AliceChen`) in transfer order: from, to, fee-to. Amounts are decimal atom strings, not display amounts. The starter makes this sharp: symbol `USD`, economic id `A`, scenario asset `"A"`, and `10.00 USD + 0.10 USD` is the atom string `"1010"`. I read that starter before writing the payroll fixtures, so the payroll scenarios did not fail on the first simulate. The trap is still the main fixture hazard.

Balance length is exact. A transfer fixture has three ordered cells and one allowance owned by the signer. There is no default balance and no omitted cell. An unknown field `note` is `BETA_SCENARIO_SCHEMA`. A duplicate key `profile` is `BETA_JSON_DUPLICATE`. A scenario asset `BonusPoints` against source asset `PayrollUSD` is `BETA_SCENARIO_IDENTITY`. The fixture cannot retarget the intent.

`source_hash` on the intent is an author claim. `check`'s `sourceHash` is the sha256 of the `.mori` bytes. Expansion copies the claim into Source/6 and also returns the file digest. They are different strings. I kept the claims obviously non-digest: `claim-src-alice-w39`.

Source/6 prints `use_replay "payroll-2026-09-w39-alice"`. The published effect key is the JSON triple of domain, signer, and nonce. A test expectation that copies the Source/6 nonce line will not match the candidate.

One `simulate` or `expand` takes one action. Two payroll intents are two preparations. There is no command that posts Alice and then pays Bob under one head.

`adopt_schedule` typechecks beside the transfers. Execution stops with `BETA_PROFILE_UNSUPPORTED` before the scenario is judged. The same code comes back for shipped staking. The eight files in `examples/` are authoring sketches of that class. I read all eight. I simulated only `staking` `deposit`, plus my own `governance.queue`.

Init refuses an existing directory with a raw `EEXIST` message. `mori test` on a directory without `mori.tests.json` is a raw `ENOENT`. Usage errors (`simulate` without flags, unknown command `dance`) go to stderr as `mori: ...` and leave stdout empty. Successful and domain failures print JSON, or the short text form of `check`.

Every CLI stderr in this seat also contains Node's warning that `NO_COLOR` is ignored because `FORCE_COLOR` is set. That is this environment, not a Moriarty diagnostic. A wrapper that treats any stderr as failure will misread a passing command.

## Errors and discovery

The three payroll programs got their intended `check` status on the first run. The seven-case test passed on the first `mori test payroll`. I did not edit a bound, a recipient, a nonce, or an expected amount after seeing tool output.

Discovery probes, all retained:

- Second `init` of `scratch-init` exits 1 with `EEXIST`.
- `fmt --write` on the wrong-asset file exits 1 and leaves the one-line domain declaration in place.
- Duplicate JSON key and unknown scenario field fail formation.
- A throwaway manifest outside this project expected debit `"1"`. `mori test` exited 1 with `passed: false` and no diff (`outputs/36-test-mismatch.out`).
- `mori test .` does not see `payroll/mori.tests.json`.
- The starter test passes on its own manifest.
- MCP rejects a tool argument named `path` with JSON-RPC `-32602` and the message `Tool arguments must match the closed text schema`.
- The first LSP hover and definition positions landed inside a string and a comment, and the server returned null. A follow-up on `Desk`, `alice_wage`, and `pay_alice` returned hover text and definition ranges.

I did not run a repay, so I have no executed obligation result. I did not install the VSIX and I did not run the Neovim smoke. That smoke is documented to run with the working directory inside the package, and I did not want a tool writing there.

## Diagnostics, formatter, commands, editor, AI

`check` text positions are 1-based. The wrong-asset sum is `26:23`, on `alice_wage + alice_fee`. The JSON diagnostic is UTF-8 bytes 881 through 903. LSP full sync reports the same span as UTF-16 line 25, character 22, which is the 0-based form of that point. The message names the rule: addition and subtraction need the same nominal asset. That diagnostic is specific enough to fix the source.

Core rejections are also specific: `S0_INTENT_SCOPE` with judgment `intent`, and `S0_HISTORY_STALE` with judgment `history`. Both set `publishedPost` and `publishedEffects` to null and `diagnosticWork` to 1. They do not say the atom values that failed. For this fee cap I already knew 1225 and 1000. A larger intent would need `inspect` to recover the atoms.

Scenario failures are weaker. `BETA_SCENARIO_IDENTITY` says `Scenario domain/asset does not match source (/)`. The pointer is `/`, not `/asset`, and the span is the `pay_alice` action in the `.mori` file (bytes 2114–2145). The duplicate-key and unknown-field messages do include the key name, and they still use pointer `/` and that same action span.

`fmt` preserves comments and refuses an invalid file. On a valid file it expands records cleanly, then breaks calls like this:

```mori
valid: rounds(domain: Night,
from: 40,
to: 52),
```

`from` and `to` sit at the same indent as `valid`, so they look like intent fields. The formatted file still checks, and its Alice simulate matches the hand-written effects and post. The file digest changes from `18efde69…` to `0bae92e2…`. I did not run `fmt --write` on the valid sources. The README already says formatting changes bytes and can invalidate commitments. The indent is an additional problem.

`inspect` is the command that makes the open premises visible. `check --json` also embeds `operationSchemas` for the whole catalog, which is why a passing check is about 31 KB. The text form of `check` is the one I would show a person first.

Editor, from a local stdio client only. I did not rate VS Code activation or Neovim.

- Initialize advertises `positionEncoding: utf-16`, `textDocumentSync: 1`, completion, hover, definition, document symbols, and formatting. Semantic tokens and debugging are absent, as `editor/README.md` says.
- Document symbols list 19 names. Action details are `LocalS0` or `SpecifiedOnly`.
- Hover on `pay_alice` says local preparation does not qualify proofs or settlement.
- Hover on `Desk` says identity metadata is an unverified claim. Hover on `const alice_wage` uses that same sentence. A quantity is not an identity claim.
- Definition from `signer: Desk` lands on the `Desk` declarator. Definition from the wage use lands on `const alice_wage`.
- A Full `didChange` that replaces the gross sum with `missing` publishes `BETA_REFERENCE`.
- Formatting returns one edit and contains the flat `rounds(domain: Night,` break.
- Completion on the prefix `trans` returns `transfer` with detail `Moriarty authoring` and keyword kind. It does not say LocalS0. I did not request a dotted operation completion in that follow-up. The server code path for dotted names is visible in the package; I did not exercise `governance.queue` completion.

The shipped snippets are an agreement shell, an asset with example id `"A"`, and `action pay uses Payment`. There is no transfer-intent snippet. That id `"A"` is the same economic-id trap as the starter.

AI files I read: `ai/AGENTS.md`, `ai/README.md`, `ai/skills/check-source/SKILL.md`, `ai/adapters/CLAUDE.md`, `ai/adapters/moriarty.mdc`, and `ai/examples/valid.mori` plus `invalid.mori`. I did not copy an adapter into this project and I did not change a client configuration. They stay inert. The skill says to pass exact source text to `check` and to treat a scenario as untrusted. MCP, from my client, matches that boundary: four read-only tools, text arguments only, and `preview` of the schedule action flagged `isError: true`.

The arithmetic examples under `ai/examples/` are not financial settlements. The only Local S0 sample the CLI generates is the one-action invoice.

## Bugs, then preferences

Bugs I reproduced:

1. Medium. `fmt` breaks a call after the first argument and indents the rest as if they were fields of the enclosing record. Valid source still checks, and the Alice candidate is unchanged, but the layout is easy to misread and the digest changes. Diff: `outputs/10-fmt-payroll.diff`.
2. Medium. `mori test` on a wrong effect amount exits 1 and prints `passed: false`, the actual status, and `code: null`. It does not print the expected amount, the actual amount, or a path. See `outputs/36-test-mismatch.out`, from a manifest outside this project.
3. Medium. Scenario formation diagnostics use pointer `/` and the source action span. `BETA_SCENARIO_IDENTITY` does not say whether domain or asset failed, and it does not give the expected economic id. See `outputs/22-sim-wrong-scenario.out`. Duplicate keys and unknown fields have the same span problem (`outputs/30-sim-dup-key.out`, `outputs/31-sim-unknown-field.out`).
4. Low. LSP hover for `const alice_wage` says `identity metadata is an unverified claim`. That sentence is right for `account Desk` and wrong for a quantity. See `outputs/40-lsp-followup.json`.

Preferences, separate from those bugs:

- `check --json` inlines all 30 operation schemas on every file. The README says check publishes the schemas. A separate switch would make the per-file result easier to read. I would still want the schemas somewhere.
- Completion labels `transfer` as a generic keyword. Local S0 and SpecifiedOnly are different promises, and the symbol list already knows the difference.
- The asset snippet and the starter use economic id `A` with symbol `USD` without a nearby sentence that the scenario must say `A`.
- The eight shipped examples cannot be simulated. A second Local S0 sample, especially repay, is missing. Init is a single transfer.
- The intent field `source_hash` and the result field `sourceHash` sound alike and mean different things. I would document them on one page before renaming anything. A rename would change source bytes.
- The candidate replay key is a JSON triple, while Source/6 shows the nonce. The starter test expects the triple. The tutorial has to show both lines once.
- A payroll of two people is two simulates. That matches the one-action command. I want the tutorial to say so, not a hidden batch mode.
- There is no division. An agreed fee is the representable payroll fee. I would not add rounding in order to write a percentage.
- Second `init` and a missing test manifest report Node `EEXIST` and `ENOENT`. The path is visible. A Moriarty code would match the other errors. Low priority.
- This seat's stderr warning about `NO_COLOR` and `FORCE_COLOR` is Node, not the compiler.

Structural checking that accepts a broken fee cap, and that cannot see a stale head, is the beta boundary the README states. I am not listing it as a defect. The missing piece is the diagnostic when preparation then rejects, and the docs that tell a new author to run `simulate` anyway.

## Repair suggestions and uncertainty

Formatter: if a call does not fit one line, break before the first argument and indent the arguments one level past the call. Keep the current refusal to format invalid source.

Test runner: on an effect or post mismatch, print the case name, the first differing path, the expected value, and the actual value. Keep exit code 1. Do not print a mismatch as success.

Scenario errors: set the pointer to the field (`/asset`, `/balances/1/account`) and include the source economic id and the fixture value in the message. Keep a source span only for source errors. Say in the message that the span is not a scenario location if a source span remains.

Hover: use the unverified-claim sentence for domain, account, asset, and obligation. For a const, show the tag and, for a quantity, the asset name and atoms. `inspect` already has those atoms.

Docs, in the website notes: one worked transfer with atom arithmetic, one rejected fee cap, one stale head, one wrong asset, and an explicit SpecifiedOnly non-result. State that `PreparedUnqualified` still requires the four premises.

Uncertainty:

- I did not execute `repay`. Obligation accounting, accrual-first application, and a two-balance fixture are unread by this seat's commands. I will not quote a repay post-state.
- I did not simulate `amm`, `lending`, `stablecoin`, `option`, `oracle`, `bridge`, or the rest of `governance` and `staking`. One staking action and my `governance.queue` returned `BETA_PROFILE_UNSUPPORTED`. The other families are the same support class in `check`'s schema table and in the example headers. That is not an execution log for each action.
- MCP `preview` was compared on status and premises, not by a byte-for-byte diff against `simulate` stdout.
- I did not try a zero fee, a zero value, an alias of two transfer endpoints, a round outside 40..52, or a replay tag `consumed`.
- VS Code activation and the Neovim smoke were not run. LSP results are from a local Content-Length client.
- I did not adopt the AI adapters, so I have no evidence about a host product loading them.

Rated from use: CLI `init`, `check`, `fmt`, `inspect`, `expand`, `simulate`, `test`, and bad usage; the JS API; a local MCP client; a local LSP client; the shipped snippets and AI text by reading. Not rated: VS Code installation, Neovim, proof tools, ledger clients, or a repay run.
