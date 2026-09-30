# Get started with Moriarty beta

This package checks `.mori` financial authoring and locally prepares two S0
operations. **PreparedUnqualified means a candidate, not an authenticated,
proved, signed or settled transaction.** The local scenario is a stipulation.
The other eight-family examples check structure only (`SpecifiedOnly`).

## Install and run

Use Node 24 or later. From `packages/moriarty-beta` in a Moriarty checkout:

```sh
npm ci
npm run build
node dist/cli.js init /tmp/invoice
node dist/cli.js check /tmp/invoice/invoice.mori
node dist/cli.js test /tmp/invoice
node dist/cli.js simulate /tmp/invoice/invoice.mori --action pay --scenario /tmp/invoice/scenario.json
node dist/cli.js init /tmp/loan --template repay
node dist/cli.js test /tmp/loan
node dist/cli.js simulate /tmp/loan/repayment.mori --action repay_loan --scenario /tmp/loan/scenario.json
```

`init` refuses an existing directory. `--template transfer` is the default;
`--template repay` creates `repayment.mori` instead of `invoice.mori`.
For an installation outside the checkout, run `npm pack` here, then install that
local `.tgz` into your project with `npm install /absolute/path/to/archive.tgz`.
Run `npx mori` there. No npm registry publication is claimed.
`mori COMMAND --help` shows the command's arguments. Exit 0 means the requested
local operation completed: a preparation remains unqualified. Exit 1 means
rejection, unsupported execution, test mismatch or a command/file error.

## Read the program

A program begins `profile "moriarty-beta/1"; agreement Name { ... }`.
Declarations refer only to previous immutable declarations. Account and asset
names are source names; their `id` fields are economic identity claims.
`Buyer` may have `id: "Owner"`: the scenario and Core effects use `Owner`.
Changing a source alias does not change that economic identity. A symbol such as
`USD` is display metadata, not an asset identity or conversion permission.
Different assets/domains never mix implicitly. Asset scale 2 means `10.00 USD`
is 1000 atoms. JSON monetary amounts are canonical decimal **atom strings**.
No floating point, division, rounding or implicit asset conversion exists.
`Qty<USD>`, amount literals, `atoms(asset: USD,value: 1000)`, checked arithmetic,
named calls, comments and trailing commas provide authoring convenience.

An `intent` supplies explicit signer/key/nonce/head, inclusive round window,
gross cap (including fees), fee cap, net floor and fixed operation. Empty
observations/disclosures/retained effects/duties and `None` delegation/recovery
are explicit signed terms. `action pay uses Payment;` selects that intent.
There are no hidden financial defaults. A funded repayment requires zero fee cap and net floor, and uses accrued interest
first, then principal; paying above outstanding rejects at Core's Effect judgment.

## Understand the two local examples

Transfer: price 1000 atoms, fee 10, debit 1010. Owner 10000 becomes 8990;
Recipient receives 1000 and Fee receives 10. Allowance decreases by 1010;
work decreases by 1; replay is consumed and head advances h0→h1.

Repayment: principal 100000, accrued 1000, outstanding 101000. Payment 3000
pays all 1000 accrued and 2000 principal. Principal/outstanding become 98000,
accrued becomes 0, and status remains Outstanding. Payer 200000 becomes 197000;
Creditor receives 3000. Allowance/work/replay/head change explicitly.
The shipped [repayment fixture](examples/local/repay/repayment.mori) includes
complete independently derived effect and post-state expectations.

`TransferLiteralFee` and `RepayAccrualFirst` in expansion identify the selected
Source/6 profile, not your action's name. Source/6 shows bare nonce `n1`; Core
consumes the domain/signer/nonce tuple encoded as `["Midnight","Payer","n1"]`.
A fixture's round is the observation round and is not automatically advanced.

## Local scenario reference

JSON profile `moriarty-local-scenario/1`, kind `local-stipulation`.
Every top-level field below is required. Unknown fields and duplicate decoded
keys reject, including escaped spellings of the same key. Files are bounded to
65536 bytes and JSON to depth 32 / 4096 nodes; no comments in JSON.

| Field | Meaning and shape |
| --- | --- |
| `profile`, `kind` | Exact strings above |
| `domain`, `asset` | Economic IDs matching the selected source action |
| `head`, `predecessor`, `post_head` | Explicit nonempty scalar text; source `pre_head` is a signed term |
| `round` | Canonical unsigned decimal string; source window endpoints are inclusive |
| `balances` | Exact ordered `{account,amount}` cells: transfer signer, recipient, fee recipient; repayment payer, creditor |
| `allowance` | `{owner,remaining,spent}`; owner is signer; counters are atom strings |
| `replay` | Exactly `unused` or `consumed` |
| `work_remaining`, `work_spent` | Canonical unsigned decimal strings |
| `obligation` | Required for repayment; forbidden for transfer; shape below |
| `candidate_effects` | Optional hostile proposal override, at most 16 entries, checked through Source/6 and actual Core |

Economic IDs passed into Source/6 use `[A-Za-z][A-Za-z0-9_]{0,63}`.
UInt128 integers use `0` or a nonzero leading digit followed by digits, no sign,
leading zeros, exponent or decimal point. S0 signed monetary fields fit 2^127−1.
Balance/counter transport fields fit UInt128; allowance and work totals cannot
overflow. Scenario shape/identity/accounting errors are `FormationRejected`.
Caps, funds, authority, expiry, replay and complete financial effects are decided
by existing Core, in Stage→Intent→Effect→Authority→History→Failure order.
`check` can succeed while Core rejects a particular scenario; this is intentional.

Repayment obligation fields: `id`, `debtor`, `creditor`, `asset`, `principal`,
`accrued`, `outstanding`, `status`. IDs must match the source/payer/asset;
creditor differs from payer; status is `Outstanding`; principal + accrued equals
outstanding. Unused source constants named principal or accrued do not constrain
scenario accounting; only operative intent fields bind the request. The required
predecessor is echoed into Source/6 but remains unauthenticated, and is not a
Core equality check. Amounts are atom strings. Partial/full/interest-only repayment uses
the same operation, with different explicit source amount and fixture.

The override's Source/6 effect variants are `Debit`, `Credit`, `SetObligation`,
`UseAllowance`, `UseReplay`, `AdvanceHead`. Inspect `expand` for their closed field
shapes. Override replay uses the bare nonce at this boundary; prepared Core
outputs use the tuple key. Overrides are fault-injection proposals, never facts.

## Test and diagnose

`mori.tests.json` is a closed record `{profile:"moriarty-beta-tests/1",cases:[...]}`
with 1–64 cases. A case has exactly `name`, `source`, `action`, `scenario`, `expect`.
Names are unique nonempty strings; source/scenario are relative files contained
in the project, including after resolving symlinks.
`expect` requires `status`; optional `code`, `effects`, `post` are exact assertions.
Statuses: `PreparedUnqualified`, `CoreRejected`, `SourceRejected`,
`AuthoringRejected`, `FormationRejected`, `Unsupported`.
`effects` checks the full ordered vector, and `post` the full exact record, with
no omitted or extra fields. The starter tests include both. Passing only a status
checks less; it does not establish financial completeness.

A mismatch gives up to four first differences, one each for status/code/effects/
post, with an RFC6901 pointer and bounded expected/actual summaries. For example
`/post/balances/0/amount`, expected `8991`, actual `8990`. Containers are summarized
instead of dumped. Test result `qualification` is `local-stipulation-only`.
`inspect` resource refusal is `InspectionRejected` with the original
`authoringStatus`; it is not a newly discovered source error.

`check` defaults to a short readable report; use `--json` for machines.
`simulate` retains the full local Core result, ordered effects and candidate post.
`expand` exposes generated Source/6 and origins back to source/scenario fields.
CLI files require valid UTF-8 without a BOM. Invalid encoding is refused before
`fmt --write` can change bytes. API inputs are already decoded text.
Formatting preserves comments/spelling and compact named calls, but changes
bytes and therefore source commitments; it never updates or signs claimed hashes.

## Editors, AI and the implementation boundary

Follow [editor setup](editor/README.md) for VS Code highlighting/snippets/LSP or
Neovim, and [AI setup](ai/README.md) for MCP and project-local provider adapters.
MCP accepts bounded text only, with no filesystem paths, shell, signing or sending.
AI completions are proposals subject to the same compiler/Core checks.

`check` returns `status`, `sourceHash`, `agreement`, `diagnostics`, `actions`,
`operationSchemas`, `declarations`, `agreementSpan`, `references`, `fieldUses`,
`evidence`, `openGates`. References and field-use
byte spans are additive JSON-safe authoring metadata; they do not authenticate IDs.
Use named fields; the API does not promise a permanently closed result key set.
`inspect` reports per-action support and actual signed terms without recursively
expanding the declaration DAG. Empty S0 premise/binding arrays on SpecifiedOnly
mean that S0's particular catalog does not apply: authentication and financial
relations remain Open, not premise-free or qualified.

Computed `sourceHash`/`scenarioHash` digest actual input text. Source
`source_hash`/`policy_digest` remain unverified signed claims. Four local S0
external premises are canonical intent signature, snapshot-to-head, head extension,
and atomic ledger compare-and-consume. Four unverified bindings are agreement ID,
selected program, asset scale and authenticated predecessor. Neither a hash nor
a successful local test closes these obligations. Blockchain address codecs,
cryptographic signatures, ZK proving, financial ledger settlement and general
formal correspondence remain open.


### SpecifiedOnly authoring hints

Provided domain/asset/signer headers must agree with the operation's local domain;
bridge source/destination may be foreign, while owner and amount share the local
domain. Caps use the declared asset, or bridge's local amount asset when absent;
AMM swap net floor uses its output asset. These checks establish nominal types,
not financial conservation or authority. Share-class IDs are unique per domain.
Intent `retained_duties` hints accept text or arrays of text; stage
`retained_duties` hints accept text only. Stage `signed_floor` accepts
text or a Qty in that stage domain. Prose hints and partial policy records,
including an empty policy, are stored authoring data. They do not establish duties,
evidence, policy preservation, stage linkage or a valid financial floor.
Unsupported expansion/simulation reports `scenarioValidation: NotAppliedUnsupported`:
the scenario was not schema-checked or applied. An Unsupported expected test can
pass with an invalid fixture because it tests the support refusal, not that
fixture. For a real local financial test, assert the complete effects and post.

Result shapes follow the command: expand/simulate/test/init carry a local scope
label; authoring check/inspect and CLI formation refusals describe authoring or
rejection directly. A missing `qualification` field never means qualification.
Init repayment source and the shipped example differ by a terminal newline;
`sourceHash` therefore differs. Always digest the actual source bytes you use.
