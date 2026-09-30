# G3 trial: bridge and staking authoring

Seat G3, package `@moriarty-lang/beta` 0.1.0-beta.1, Node v24.21.0. The program is an original agreement, `CrossStakeBridge`. It names a home canonical asset and a foreign bridge-claim asset, then authors bridge and staking actions as separate intents. Execution of those actions is unsupported. Local transfer and accrual-first repayment were run only as fixture probes and as the init starter.

No signature, provider authentication, proof, or ledger submission was attempted. Local success means `PreparedUnqualified`.

## Artifact list

| Path | Role |
| --- | --- |
| `bridge-stake/vault-bridge.mori` | Original agreement. Final source hash `58324510cab48d8c7ecf587130746db22415a646135e45049eefa102302c9fa5` |
| `bridge-stake/invalid-cross-stake.mori` | Rejected composition: foreign face deposited into the home share class |
| `bridge-stake/invalid-nominal-sum.mori` | Rejected sum of the two USD symbols |
| `bridge-stake/invalid-alias.mori` | Rejected second asset with economic id `RailUSDCanonical` |
| `bridge-stake/scenarios/home-rail.json` | Closed local fixture for `HomeRail` / `RailUSDCanonical` |
| `bridge-stake/scenarios/foreign-claim.json` | Closed local fixture for `ForeignClaim` / `ClaimUSDNominal` |
| `bridge-stake/scenarios/malformed-extra.json` | Same home fixture plus unknown field `evidence_fresh` |
| `bridge-stake/mori.tests.json` | 18 cases. This is the only manifest `mori test bridge-stake` reads |
| `bridge-stake/README.md` | Short project note. The init starter files were removed after their outputs were saved |
| `probes/` | Discovery files. Not part of `mori test` |
| `tools/protocol-probe.mjs` | Local LSP and MCP client |
| `outputs/` | Captured stdout and stderr |
| `command-log.json` | Commands, exit codes, expected results, observed results |
| `website-feedback.md` | Tutorial-site proposal |

`outputs/starter/` keeps the init invoice after `fmt --write`. Its pre-format check hash was `fb507bfb848a8859a55c020faf410e52f9e40c00fc3938a90ea3c763e1cf1d6c`. The formatted hash is `c7090e6be5124795fd0c60d74860d8f5d221090a9ab88f0defcdc73e7d65a6d8`.

## Tested outcomes

`mori test bridge-stake` exited 0. Status `TestsPassed`. The summary field `qualification` is `local-stipulation-only`. The 18 case statuses are not local preparation:

| Cases | Status | Code |
| --- | --- | --- |
| 13 bridge and staking actions, plus escrow against `malformed-extra.json` | `Unsupported` | `BETA_PROFILE_UNSUPPORTED` |
| `compose_escrow_and_deposit` | `AuthoringRejected` | `BETA_ACTION_UNKNOWN` |
| foreign face staked on home shares | `AuthoringRejected` | `BETA_ASSET_MISMATCH` |
| `250.00 RailUSD + 250.000000 ClaimUSD` | `AuthoringRejected` | `BETA_ASSET_MISMATCH` |
| second asset id `RailUSDCanonical` | `AuthoringRejected` | `BETA_DUPLICATE_ID` |

Those expectations were written from the documented SpecifiedOnly rule and from the checker codes for asset mismatch, unknown action, and duplicate economic id. They were not filled in from a passing run. The malformed-scenario case was predicted to stay `Unsupported`, because SpecifiedOnly returns before scenario parsing. That prediction held.

`check` on `vault-bridge.mori` is `AuthoringChecked`. Text mode lists 13 actions as `SpecifiedOnly` and does not print the PreparedUnqualified line. JSON `evidence` is `AuthoringOnly`. `openGates` are `authentication`, `nativeProof`, `financialCorrespondence`, and `atomicLedgerAcceptance`. Per-action coverage is syntax, names, and quantities checked; `financialRelations` is `Open`; `localPreparation` is `Unsupported`.

`expand` and `simulate` for `escrow_home`, `claim_foreign`, `deposit_home`, `deposit_foreign`, `reward_home`, and `withdraw_foreign` return `Unsupported`, code `BETA_PROFILE_UNSUPPORTED`, `publishedEffects` null, `publishedPost` null. Exit code 1. The shipped `examples/bridge.mori` and `examples/staking.mori` behave the same way.

`inspect` on the final agreement: every domain, account, and asset claim has `authenticated: false`. `localS0Actions` is empty. `requiredPremises` and `unverifiedBindings` are empty. `coverage.authentication` and `coverage.ledgerCommit` are `open`.

### Local results, kept separate

These are `PreparedUnqualified` with qualification `local-stipulation-only`. They retain four premises (`canonical-intent-signature`, `snapshot-to-head`, `head-extension`, `atomic-ledger-compare-and-consume`) and four unverified bindings (`agreement-id`, `selected-program`, `asset-scale`, `authenticated-predecessor`). They are not signatures, proofs, or ledger accepts.

Init starter `pay`, before any edit. Scale 2. `10.00` is 1000 atoms, `0.10` is 10, gross debit is 1010. Opening balance 10000. Post balances Owner 8990, Recipient 1000, Fee 10. Allowance remaining 8990, spent 1010. Work remaining 9, spent 1. Head `h0` to `h1`. The simulate output matched this arithmetic.

`probes/local-rail-transfer.mori` uses `scenarios/home-rail.json`, the same file `escrow_home` refuses to execute. Transfer 250.00 `RailUSDCanonical` (25000 atoms), fee 0. Opening StakerPrincipal 1000000, VaultCustodian 0, RewardTreasury 50000. Expected post 975000, 25000, 50000. Allowance remaining 975000, spent 25000. Work 7 and 2. No fee credit, because the fee is zero. Simulate matched. Escrow of that same scenario remains `Unsupported` with a null effect vector.

`probes/local-repay.mori` pays 1.25 `RailUSDCanonical` (125 atoms) against principal 5000 and accrued 125, outstanding 5125. Accrual-first clears the 125 accrued atoms and leaves principal 5000, outstanding 5000, status `Outstanding`. Balances become Borrower 9875 and Lender 125. Work 3 and 1. Simulate matched, including `SetObligation`.

`probes/local-claim-transfer.mori` against `foreign-claim.json` is `CoreRejected`, code `S0_EFFECT_RANGE`. The holder balance is 0 and the transfer is 1 atom (scale 6). No post was published. The fixture did parse.

A starter scenario whose `candidate_effects` debit 1 atom instead of 1010 is `CoreRejected`, code `S0_EFFECT_MISMATCH`. The fixture cannot replace the transfer arithmetic.

### Author arithmetic the checker resolved

`RailUSDCanonical` scale 2. `ClaimUSDNominal` scale 6. Both use display symbol `USD`. Representation is `canonical` on the home asset and `bridge-claim` on the foreign asset. Declaration names are `RailUSD` and `ClaimUSD`. Scenario strings use the economic ids, not the declaration names.

| Quantity | Atoms shown by inspect |
| --- | --- |
| 250.00 RailUSD, and `atoms(25000)` | 25000. Their difference is the escrow `fee_cap`, atoms 0 |
| 250.000000 ClaimUSD, and `atoms(250000000)` | 250000000. Claim `fee_cap` atoms 0 |
| Reward amount 4.50 RailUSD | 450. `gross_cap` 5.75 is 575. `fee_cap` 0 is `5.75 - (4.50 + 1.25)` |
| Foreign reward amount 4.500000 ClaimUSD | 4500000. `gross_cap` 5.750000 is 5750000. `fee_cap` 0 |
| Slash 1.25 RailUSD / 1.250000 ClaimUSD | 125 / 1250000 |
| Home withdraw floor `min(96.00, 100.00)` | `minimum_backing` 9600. Gap `fee_cap` 400 atoms, which is 4.00 RailUSD |
| Foreign withdraw floor 96.000000 under a 100.000000 slice | `minimum_backing` 96000000. Gap `fee_cap` 4000000 atoms, which is 4.000000 ClaimUSD |
| `10000 * 5` and `25000 * 2` | Unbond `earliest_round` is 0. `share_atoms` is 10000 |

The first inspect, before a source repair, showed the home reward operation amount as 575. That was the reward-plus-slash sum placed in the reward call. The source now keeps the reward at 4.50 and leaves 5.75 only on `gross_cap`. The checker accepts that cap even though it is larger than the call amount. Nothing in this beta applies the cap to a balance.

`const unchecked_atom_count_gap = 250_000_000 - 25_000` is 249975000 and has no unit. Inspect does not print consts, so that figure is independent arithmetic only. Check passing does not display it. The same limit applies to `home_proportion_gap` and `foreign_proportion_gap`, which are `5 * slice - 2 * face` and are zero by that arithmetic: home 500.00 against 500.00, foreign 500.000000 against 500.000000.

### Pending duties

The agreement contains four stages and one episode, `BridgeThenStake`. Duty, status, failure, and `retained_duties` on those stages are strings. The episode `pending` string says the actions are separate and SpecifiedOnly. The episode can list stages from both domains. None of this text is an obligation, a balance, or a freshness proof.

`inspect` does not return stage, episode, policy, grant, party, observation, or share-class bodies. The duty discussion is visible in the source and invisible in inspect unless a copy sits on an intent field.

`probes/unlinked-duty.mori` is `AuthoringChecked` with `reads: ["no-such-stage"]`, `retained_duties: 4`, `signed_floor` set to a quantity, and `earliest_round: 99` beside `rounds` 12..48. Structural checking does not validate evidence freshness, financial rules, or obligations. The round window on `LockRail` (`from` 12, `to` 48) is type-checked and then unused by simulate.

Local S0 is the other duty rule. A transfer intent must set `retained_duties` to an empty array, `failure` to `SuccessOnly`, and `delegation` and `recovery` to `None`. The bridge file's duty sentences are legal only because the actions are horizon operations.

### Composition and horizon boundary

One agreement may declare both families. There is still no composed action. `compose_escrow_and_deposit` is `BETA_ACTION_UNKNOWN`. Staking `ClaimUSD` into `HomeShares` is `BETA_ASSET_MISMATCH` (`Expected Qty<RailUSD>, received Qty<ClaimUSD>`). Adding the two faces is the same code with the message that addition requires identical nominal assets. A repeated `claim_id` string does not join escrow, claim, and recover.

Shipped examples already show each family alone. This file's extra claim is that putting both families in one agreement does not create a lifecycle. `check` coverage stops at syntax, names, and quantities.

## Syntax, units, accounts, assets, intents, scenarios

What worked on the first successful authoring pass:

- Profile must be the string `moriarty-beta/1`. One agreement. Prior-only names. Trailing commas are accepted.
- Quantity spelling needs a space before the asset. Scale is exact: 2 fractional digits for RailUSD, 6 for ClaimUSD. `25_000` is canonical and `fmt` keeps the underscores.
- `atoms(asset, value)` and the decimal spelling meet at a zero quantity when the integer matches the scale. `min` of two same-asset quantities works. There is no division and no implicit conversion.
- Economic ids are Source/6 identifiers: a letter, then letters, digits, or underscores, at most 64 characters. `claim-rail-250` is legal as a string. It is not legal as a domain id. Chain and network are free strings, so `home-rail` is legal there and illegal as an id.
- Scenario account and asset strings are economic ids. `Staker` is the declaration. `StakerPrincipal` is the id. The starter hides this by using declaration `USD` and id `A`, and declaration `Buyer` and id `Owner`.
- Symbol `USD` on both assets does not make them one asset.
- A local scenario is a closed record: profile `moriarty-local-scenario/1`, kind `local-stipulation`, and the balance, allowance, replay, work, head, and round fields. No unknown field. No duplicate key. Integers are canonical decimal strings. Replay is `unused` or `consumed`.
- For a local transfer the balance order is signer, recipient, fee recipient, three distinct accounts, even when the fee is zero. Round 12 must sit inside the intent window. `pre_head` must equal the scenario head.
- For repayment the balance order is debtor, creditor. `outstanding` must equal principal plus accrued. Core requires fee cap 0 and net floor 0. Payment applies to accrued before principal.

Friction:

- Readable `check` for this file is 14 lines and never says execution is unsupported. The help trailer says local financial results remain PreparedUnqualified, which is true of local results and easy to apply to bridge output.
- `inspect` is JSON only. The final inspect file is about 66KB. The final `check --json` file is about 140KB on disk and about 84KB of JSON, mostly spans and references. The embedded operation catalog is only about 2.4KB.
- Horizon `fee_cap` was the only optional quantity slot in which a zero identity gap could be made visible to inspect. That overloads a field whose name says fee. The home withdraw `fee_cap` of 400 atoms is the 4.00 gap under the 100.00 slice, not a transfer fee. The relation string in the source says so.
- `gross_cap` of 575 against a reward of 450 is accepted. Caps are not tied to the call.
- Bridge destination and source domains are supposed to differ. The checker also skips owner-versus-amount alignment. See bugs.
- `mori test` requires a scenario path even when the action cannot use it. A broken fixture still satisfies an `Unsupported` expectation.
- The test summary always says `local-stipulation-only`, including when every case is `Unsupported` or `AuthoringRejected`. The simulate failure object omits `qualification`. The success object includes it.
- `init` into an existing directory and `test` on a directory with no manifest throw Node `EEXIST` and `ENOENT` text on stderr, not a JSON diagnostic.
- Every CLI process in this environment prints a Node warning that `NO_COLOR` is ignored because `FORCE_COLOR` is set. That warning is not produced by the Moriarty CLI source.

## Errors and discovery

The hand-written agreement checked on the first `check`, before formatting. Formatting was idempotent after the last edit.

The reward amount was repaired after inspect showed 575 atoms on `staking.reward`. That was an author mistake, not a compiler change. The final inspect shows 450.

Discovery probes outside the test manifest:

| Probe | Result |
| --- | --- |
| Home account escrows `ClaimUSD` | `AuthoringChecked` |
| Foreign account deposits into home shares backed by `RailUSD` | `BETA_DOMAIN_MISMATCH` |
| `fee_cap` in `ClaimUSD` on a `RailUSD` escrow | `AuthoringChecked` |
| Intent `domain: HomeRail` with `asset: ClaimUSD` and a foreign signer | `AuthoringChecked` |
| Duplicate share-class id `HomeVaultShare` | `AuthoringChecked` |
| `policy Bare = {}` | `AuthoringChecked` |
| Unlinked and untyped duty fields | `AuthoringChecked` |
| Unknown scenario field on starter `pay` | `FormationRejected` `BETA_SCENARIO_SCHEMA` |
| Same unknown field on `escrow_home` | `Unsupported` `BETA_PROFILE_UNSUPPORTED` |
| Duplicate JSON key `round` | `FormationRejected` `BETA_JSON_DUPLICATE` |
| Lying `candidate_effects` | `CoreRejected` `S0_EFFECT_MISMATCH` |

Shipped bridge and staking examples were checked and simulated read-only. They were not edited.

## Diagnostics, formatter, commands, editor, AI

Commands used: `init`, `check`, `check --json`, `fmt`, `fmt --write`, `inspect`, `expand`, `simulate`, `test`, `lsp`, `mcp`. Exit 0 for `AuthoringChecked`, `Expanded`, `PreparedUnqualified`, and a test run whose expectations match. Exit 1 for `Unsupported`, `AuthoringRejected`, `FormationRejected`, and `CoreRejected`.

Text diagnostics are 1-based line and column. `invalid-cross-stake.mori` reports `BETA_ASSET_MISMATCH at 11:37`. JSON diagnostics are byte spans. LSP positions are UTF-16. All three showed up in this seat.

`fmt` expands records onto separate lines once the brace indent is at least 2. It keeps comments and numeric underscores. It changes bytes, so it changes `sourceHash`. A second format of the final file matched the file exactly. `fmt --write` on the invalid cross-stake file exited 1 and did not replace the file. Call arguments break awkwardly: the second argument is indented at the intent's level, so it lines up with `operation` rather than with the first argument. The result is stable.

Editor, actually run:

- A small stdio client against `mori lsp`. Server `moriarty-beta` 0.1.0-beta.1. Capabilities: `positionEncoding` utf-16, full sync, completion, hover, definition, document symbols, formatting. 88 symbols on `vault-bridge.mori`. Hover on `escrow_home` returned `Support: SpecifiedOnly; local preparation does not qualify proofs or settlement.` Completion at `bridge.` returned `bridge.escrow`, `bridge.claim`, and `bridge.recover`. The valid buffer published an empty diagnostic list. A tiny buffer published `BETA_SYNTAX`, and after a full-text change to `01` it published `BETA_NUMBER`. Shutdown returned null, which is a normal LSP shutdown result. This client did not call formatting.
- Neovim 0.11.6 ran the shipped `editor/nvim/smoke.lua` with `-u NONE`. It printed that initialize, open, definition, symbols, format, change, diagnostics, and shutdown passed. That smoke uses a two-const sample, not this agreement. It does assert that invalid source produces no format edit.
- VS Code packaging and activation were not run. Unrated.

AI material, actually used:

- Read `ai/AGENTS.md`, `ai/README.md`, the check-source skill, and `ai/adapters/moriarty.mdc`. They match the CLI: check exact bytes, keep nominal identity, treat scenarios as untrusted, and refuse to read SpecifiedOnly or PreparedUnqualified as settlement. Adapters were not copied into a client. No provider was configured.
- `mori mcp` from the same probe process. Initialize instructions say identity and snapshot data are untrusted and that PreparedUnqualified still needs proof, authentication, and ledger premises. `tools/list` returned `check`, `inspect`, `expand`, and `preview`. `check` on the agreement was `AuthoringChecked`, every action `SpecifiedOnly`. `preview` of `escrow_home` was `isError: true`, status `Unsupported`, code `BETA_PROFILE_UNSUPPORTED`, `publishedEffects` null. MCP `inspect` and `expand` were not called.
- The agent instructions say to ask the project owner before a consequential edit. This seat had no owner to ask. That instruction does not cover an unattended run.

## Bugs and preferences

Bugs are reproducible from the files named here. Preferences are separate. Open financial formulas are not listed as bugs: the package says those relations stay open for SpecifiedOnly actions.

### Bugs

1. High. `bridge.escrow` does not require the owner domain to match the amount asset. `probes/bridge-misaligned.mori` is `AuthoringChecked`: account `StakerPrincipal` on `HomeRail` escrows `250.000000 ClaimUSD`. The same shape for staking is rejected. `probes/stake-cross-account.mori` is `BETA_DOMAIN_MISMATCH` at 7:28, "Operation argument domains differ". The bridge skip covers the whole argument list, not only `destination` and `source`.
2. High. Horizon intent headers are not required to agree. `probes/intent-domain-mismatch.mori` is `AuthoringChecked` with `domain: HomeRail`, `asset: ClaimUSD`, and a signer on `ForeignClaim`, while `bridge.claim` moves `ClaimUSD`. A local intent with that header disagreement is `BETA_DOMAIN_MISMATCH`.
3. Medium. A horizon `fee_cap` may be a different asset from the operation amount. `probes/cap-other-asset.mori` is `AuthoringChecked` with a `ClaimUSD` fee cap on a `RailUSD` escrow. Local caps are checked against the intent asset.
4. Medium. Some duty fields accept any value. In `probes/unlinked-duty.mori`, stage `signed_floor` is a quantity and intent `retained_duties` is the number 4. Neighboring duty fields must be strings. The file is `AuthoringChecked`.
5. Medium. Share-class ids are not in the economic-identity set. `probes/duplicate-share-id.mori` declares two share classes with id `HomeVaultShare` and is `AuthoringChecked`. The asset form of that alias is `BETA_DUPLICATE_ID`, "aliases are unsupported".
6. Medium. SpecifiedOnly simulate hashes the scenario and does not schema-check it. `probes/bad-scenario.json` is `FormationRejected` `BETA_SCENARIO_SCHEMA` for starter `pay`, and `Unsupported` `BETA_PROFILE_UNSUPPORTED` for `escrow_home`. A `mori test` expectation of `Unsupported` therefore passes when the fixture is garbage. Exit code 1 still happens, for the support failure rather than the fixture failure.
7. Low. `policy Bare = {}` is `AuthoringChecked` and can be attached to an intent (`probes/empty-policy.mori`). Every policy field is optional, including `id` and `duty_preservation`.
8. Low. `init` on an existing directory prints `mori: EEXIST: file already exists, mkdir ...` and exits 1. `test /tmp` prints `mori: ENOENT ... /tmp/mori.tests.json`. Other failures use a JSON object with `status` and `diagnostics`.

### Preferences

The design can stay as it is and these would still be worth doing.

- Readable `check` could print "execution unsupported; financial relations open" beside `SpecifiedOnly`. The hover text from the LSP already says that. The text checker does not.
- `inspect` could have a short text mode and could include const, share class, stage, episode, policy, and grant values. Empty `requiredPremises` on a SpecifiedOnly file is accurate for the local-preparation list and easy to read as "nothing is open". `check.openGates` still has four entries, including `nativeProof`, which inspect does not name.
- The test summary could use a qualification other than `local-stipulation-only` when no case prepared a local result. Unsupported simulate results could carry an explicit "scenario not applied" marker.
- An equality form would avoid parking a zero gap in `fee_cap`.
- Formatter could indent call arguments under the call. Current wrapping is stable, so this is cosmetic.
- `reads` and `writes` are unchecked strings. That is consistent with a string schema. A warning that names a missing stage would still help, and so would a warning when `earliest_round` sits outside `rounds`. Those are open relations, not silent mis-executions, because these actions do not execute.
- `check --json` could offer a summary that omits reference spans. The full dump is usable.

## Repair suggestions and uncertainty

For bridge calls, require the owner domain to equal the amount asset's domain. Keep `destination` and `source` as the fields that may name the other domain. Reject the misaligned probe with `BETA_DOMAIN_MISMATCH`.

For a horizon intent that sets `domain`, `asset`, or `signer`, require those three to agree with each other and with the operation's account and quantity domains, using the same rule as local intents. Require `gross_cap`, `fee_cap`, and `net_floor` to use that asset.

Type `signed_floor` and horizon `retained_duties`. A string matches the other duty fields. A bare number should be `BETA_TYPE`.

Put `share_class` ids through the same duplicate-id check as assets, or document that the id is a label and not an economic id. Until one of those happens, two share classes can share an id and still check.

On `Unsupported`, include the sentence that the scenario bytes were hashed and not schema-checked. Optionally parse the scenario anyway and attach a second diagnostic. Do not publish effects either way.

Wrap `init` and missing-manifest `test` errors in the JSON diagnostic shape.

Uncertainty: the total skip of bridge domain alignment may be an accidental consequence of allowing a foreign `destination`, rather than a decision that a home account may escrow a foreign asset. The shipped `bridge.mori` keeps the owner and the amount on the same domain, and uses the other domain only as `destination` or `source`. I did not find a test in this package that accepts the misaligned form on purpose. Staking's rejection of the sibling mistake is why I rank the bridge case as a bug.

Duplicate share-class ids cannot change a post state in this beta, because staking does not execute. The authoring alias is still accepted.

Empty policy records may be intentional partial data. The low rank reflects that. If they are intentional, the tutorial has to say that `duty_preservation: "required-but-unchecked"` is a comment the checker stores and does not apply. The valid agreement uses that string. Inspect does not show it.

I did not install the VS Code extension, did not call MCP `inspect` or `expand`, and did not run an LSP formatting request from my own client. Those paths are unrated beyond the Neovim smoke's format assertion and the capability list. I did not fuzz AMM, lending, stablecoin, option, oracle, or governance beyond reading their examples. I did not try a repayment inside `vault-bridge.mori`; the repayment result is `probes/local-repay.mori` only.

The scalar gap `249975000` is not projected by inspect. I calculated it from the two face atom counts. I do not claim the CLI printed it.
