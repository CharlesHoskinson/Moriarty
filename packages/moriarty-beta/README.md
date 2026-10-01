# Moriarty beta

A standalone `.mori` authoring language for explicit financial intent, requiring
Node 24 or later. Local transfer and funded AccrualFirst repayment reuse Source/6
and Core/5 and return **PreparedUnqualified**. The signed owner-intent flow adds native Rust signature checks before local
preparation. Provider authentication, financial proof and ledger commit remain open.

Read the [getting started guide](GETTING-STARTED.md) for both operations, complete
scenario/test schemas and troubleshooting. Read the [signed intent walkthrough](SIGNED-INTENT.md)
for external signing, native verification and three public ready-to-verify examples.

## Start

From this package:

```sh
npm ci
npm run build
node dist/cli.js init /tmp/my-mori-project
node dist/cli.js check /tmp/my-mori-project/invoice.mori --json
node dist/cli.js test /tmp/my-mori-project
node dist/cli.js simulate /tmp/my-mori-project/invoice.mori --action pay --scenario /tmp/my-mori-project/scenario.json
```

For a packed installation, install the `.tgz` with npm and use `npx mori`.
The bundle includes the strict S0 modules and needs no repository metadata or
global TypeScript compiler. `init` refuses existing targets.

## Exact amounts and names

```mori
const price: Qty<USD> = 10.00 USD;
const fee = 0.10 USD;
const cap = price + fee;
const exact_atoms = atoms(asset: USD, value: 1000);
```

Declare USD's domain, economic ID, representation and scale first. Symbol is
display metadata. Bare numbers are unsigned scalars. Addition/subtraction require
the same nominal asset; scalar multiplication and named `min`/`max` are checked.
Every intermediate fits UInt128. S0 operation/cap/liability fields additionally
fit `2^127−1`. No floating point, rounding, division or implicit conversion occurs.

Agreements contain prior-only immutable declarations, closed named calls and
`action NAME uses INTENT;`. Comments and trailing commas are supported. The starter
shows every signed bound and explicit empty/None field. A separate scenario supplies
**untrusted local data** with no default balances or spent counters. Unknown fields
and decoded duplicate JSON keys reject. It cannot override source intent.

## Tools

| Command | Result |
| --- | --- |
| `init DIR [--template transfer\|repay]` | Explicit transfer or repayment project, no overwrite |
| `check FILE [--json]` | Syntax/name/nominal checks and action support |
| `fmt FILE [--write]` | Comment/spelling-preserving format, stdout by default |
| `inspect FILE` | Identity claims, operative bounds and open premises |
| `expand FILE --action NAME --scenario FILE` | Source/6 and input origin map |
| `intent ... [--review\|--json]` | Native owner signing bytes; possession unchecked |
| `verify-intent ... [--review\|--json]` | Native signature check then unqualified local Core preparation |
| `simulate ...` | Actual local Core result, effects and candidate post-state |
| `test DIR` | Bounded cases, optionally exact effects/post expectations |
| `lsp` | Full-sync stdio language server |
| `mcp` | Read-only stdio check/inspect/expand/preview tools |

Formatting changes source bytes and can invalidate commitments. It never signs or
rewrites claimed hashes; invalid source receives no edit. Expansion is a proposal.
Core decides the first economic failure and publishes no effects on rejection;
Source/6-unrepresentable inputs are formation errors.

[Editor instructions](editor/README.md) cover highlighting, snippets, VS Code
packaging and Neovim. [AI instructions](ai/README.md) cover portable skills and
project-scoped provider adapters. These files change no global client settings.

## Financial horizon and API

The closed catalog covers AMMs, lending, stablecoins, options, oracles, governance,
bridges and staking. `check` publishes argument schemas and checked coverage.
Those actions are `SpecifiedOnly`: structural support exists; financial relations,
authentication and execution remain open. Expansion/simulation return
`BETA_PROFILE_UNSUPPORTED` with no effect vector. The wiki's proposed typed lifecycle
grammar is a separate full-language horizon, not accepted beta syntax.

```js
import { check, format, inspect, expand, simulate } from '@moriarty-lang/beta';
const report = check(sourceText);
const local = simulate(sourceText, 'pay', scenarioJsonText);
```

Public APIs accept bounded text and digest both local inputs. Origins use UTF8 byte
ranges; editor positions use UTF16. Prepared results retain four external premises
and four unverified bindings. A verified native signature is scoped evidence; account authority and the remaining
state/ledger obligations stay open. Matching hashes and AI tools establish no authority. Native financial settlement and general K/Quint correspondence remain open.
