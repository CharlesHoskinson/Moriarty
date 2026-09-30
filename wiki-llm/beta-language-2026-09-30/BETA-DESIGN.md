# Moriarty beta language design

**Status:** candidate v1 for five independent GPT-6.1 Sol medium PL reviews.
**Baseline:** `870f998b36ecda04622fa4274132e74902942d0b`.

## Product and scope

Build a real, standalone authoring beta: `.mori` files, a bounded parser,
financial name/type checks, exact quantity sugar, canonical formatting,
source-to-Core inspection, local S0 simulation, project initialization and
scenario tests, a language server, syntax highlighting and read-only AI tools.
The broader design covers all eight DeFi families and composed episodes. The
only executable economic operations in beta/1 are the existing S0 literal-fee
transfer and funded AccrualFirst repayment. Other operations receive explicit
specified/open status and a named unsupported-execution result. This is a beta
of authoring and local preparation, not a claim of native or ledger acceptance.

The public language/toolchain requires no maintainer receipt, campaign, registry
permission or hosted service. Objective supported-language restrictions apply.
Moriarty continues to target Midnight and its native proof stack. General
programs, solver holes, certification, private history and ledger qualification
remain in the full-language roadmap; this beta does not reduce those goals.

## Why this approach

Compare [surface](domains/01-surface.md), [types](domains/02-types.md),
[financial languages](domains/03-agreements.md), [workflows](domains/04-workflows.md),
[toolchain](domains/05-toolchain.md), [editors](domains/06-editors.md),
[AI](domains/07-ai.md), [meta tools](domains/08-meta-tools.md), and
[ecosystems](domains/09-ecosystems.md).

Choose an external brace language with a TypeScript frontend over a host-language
embedding or a workbench migration. An embedding permits host execution and
coercion; a workbench does not supply financial semantics and adds migration
cost. Langium remains a later option when measured editor needs justify it.
Use one analysis service for CLI, LSP and AI tools. No second financial evaluator,
duplicate compiler grammar or hosted solver enters this beta.

## Concrete source

This is the exact proposed local transfer surface. IDs and hashes are illustrative
opaque claims; none is an authenticated chain address or signature.

```mori
profile "moriarty-beta/1";
agreement Invoice {
  domain Preview = { id: "Midnight", chain: "midnight", network: "preview" };
  account Buyer = { domain: Preview, id: "Owner" };
  account Seller = { domain: Preview, id: "Recipient" };
  account Treasury = { domain: Preview, id: "Fee" };
  asset USD = {
    domain: Preview, id: "A", scale: 2,
    representation: "canonical", symbol: "USD",
  };
  const price: Qty<USD> = 10.00 USD;
  const fee = 0.10 USD;
  intent Payment = {
    domain: Preview, asset: USD,
    signer: Buyer, key: "key1", nonce: "n1", pre_head: "h0",
    valid: rounds(domain: Preview, from: 100, to: 200),
    gross_cap: price + fee, fee_cap: fee, net_floor: price,
    operation: transfer(from: Buyer, to: Seller, fee_to: Treasury,
                        value: price, fee: fee),
    source_hash: "src1", policy_digest: "policy1",
    failure: SuccessOnly,
    observations: [], disclosures: [], retained_effects: [], retained_duties: [],
    delegation: None, recovery: None,
  };
  action pay uses Payment;
}
```

An action names a source-fixed request/intent. A scenario supplies untrusted
snapshot and completion claims separately. Transfer endpoints and values cannot
be silently filled from the scenario. For repayment, declare an obligation ID
with its domain/asset; the bound debtor/creditor and accounting components come
from the stipulated obligation cell, as in S0, and appear in inspection output.
No source spelling promotes a claim to an authenticated value.

## Grammar and sugar

Select `moriarty-beta/1`; keep all earlier source profiles unchanged. The bounded
grammar is an agreement containing declarations and action selectors:

```ebnf
program = 'profile', STRING, ';', 'agreement', IDENT, '{', declaration*, '}', EOF;
declaration = kind, IDENT, [':', type], '=', expression, ';'
            | 'action', IDENT, 'uses', IDENT, ';';
kind = 'domain' | 'account' | 'asset' | 'const' | 'intent' | 'obligation'
     | 'pool' | 'instrument' | 'observation' | 'policy' | 'grant'
     | 'stage' | 'episode' | 'party' | 'share_class';
type = IDENT, ['<', type, (',', type)*, '>'];
expression = precedence-expression;
primary = STRING | INTEGER | QUANTITY | 'true' | 'false' | IDENT
        | '[', [expression, (',', expression)*, [',']], ']'
        | '{', [IDENT, ':', expression, (',', IDENT, ':', expression)*, [',']], '}'
        | qualified-name, '(', [IDENT, ':', expression,
                               (',', IDENT, ':', expression)*, [',']], ')'
        | '(', expression, ')';
```

The implemented pure expression subset has `+`, `-`, `*` with conventional
precedence and checked UInt128 intermediates. Addition/subtraction require the
same quantity identity or unsigned scalar type; multiplication is scalar by
scalar or scalar by quantity, never quantity by quantity. `min`/`max` are closed
pure functions with named `a`/`b` arguments. No division, rounding, implicit
conversion, floating point, assignment, arbitrary host calls, loop, recursion,
exception handler or effectful expression is accepted in the executable slice.

Quantities such as `10.00 USD` and `1_000 USD` elaborate using that nominal asset's
declared scale, with no rounding. Reject excess decimal places, malformed digit
separators, leading-zero integers and out-of-range atom values. Bare numbers are
scalars, never money. `atoms(asset: USD, value: 1000)` is the explicit alternative.
Use ASCII identifiers <=64 characters and JSON strings; permit line/block
comments and trailing commas. Declaration order is explicit: constants may only
refer to preceding declarations. Duplicate names, fields or call arguments reject.

All constructs retain exact UTF-8 byte spans. The lexer also retains comments
and token spelling for a formatter that changes only whitespace, preserving
comments and string/literal spellings. Formatting preserves financial meaning
but changes source bytes; it can invalidate source commitments or signatures.
No formatter result is automatically re-signed or sent.

Resource bounds: source <=65536 UTF-8 bytes; <=8192 tokens including EOF;
<=8192 AST nodes; nesting <=64; <=256 declarations; <=64 record fields, call
arguments or type arguments; decoded strings <=1024 UTF-8 bytes. Reject invalid
Unicode scalar strings, unknown profile/constructs and trailing input. Keep
finite expression/expansion work counters; do not expand aliases exponentially.

## Financial types and identity

- A `Domain` has a nominal ledger ID plus explicit chain and network metadata.
  Network/ledger IDs must be unique; equal address bytes on different domains
  never identify one account. Metadata is a claim until a provider binds it.
- `Account<D>` is distinct from `PartyId`, `AssetId`, `ObjectId`, `ProgramId`,
  `AgreementId`, `ActionId`, `StageId` and `EpisodeId`. No implicit casts.
- `Asset<D,Representation,Scale>` has a nominal ID, domain, representation and
  exact scale 0..18. Symbol is display metadata. Reject conflicting duplicate
  economic identity declarations; an alias never creates new supply or authority.
- `Qty<A>` is a checked nominal quantity, distinct from balance cells, signed
  deltas, positions, liabilities, share claims and spendable ledger objects.
- Accounts, asset and action intent must share a domain for local S0. Wrapped
  assets are distinct representations; foreign signatures and conversion need
  explicit adapter/profile/evidence contracts.
- Retain U0's Base units per Quote convention for `Price<Base,Quote,Scale>`.
  The opposite recommendation in domain02 is a recorded conflict, not an adopted
  reversal. Show numerator/denominator units; no implicit reciprocals.
- General price, clock conversion, signed positions, rights/duties and calendars
  remain specified/open in beta/1; `rounds` is the one closed S0 validity window.

No generic `$USD` implies token identity; no `@Alice` implies verified authority.
Use explicit nominal declarations and later adapter-specific native-address
constructors. Sui capability/object freshness, Solidity payable/native units,
Anchor signer/account constraints and Cairo typed addresses are inspiration,
not inherited Midnight checks. Daml participant roles remain distinct; FpML
party/account IDs and currency codes never become chain asset identity.

## Intent, completion and local preparation

Executable intents require all fields in the example, including the six explicit
empty/None fields. `operation` is a closed typed `transfer` or `repay` request;
unknown/extra/missing fields reject. The operation, source/policy claims, signer,
key, nonce, head, validity, gross/fee/net bounds and empty failure policy are
source-fixed. General solver holes are designed separately and unsupported here.

The scenario is bounded JSON with `kind: "local-stipulation"`. It supplies current
head, predecessor, round, exact ordered balance cells, allowance owner and
remaining/spent counters, optional existing obligation, replay state, remaining/
spent work and proposed post-head. It may supply a complete candidate effect
vector for hostile controls; otherwise expansion constructs a candidate vector
from the declared request and stipulated pre-state. Always send it through the
existing strict Source/6 parser and Core/5 preparer; a candidate is never proof.
Reject missing cells, duplicate/extra inputs and inconsistent domain/asset claims;
never default missing balances or spent counters to zero. The scenario cannot
override source intent. Represent only Source/6's closed replay transport, not a
claim of complete authenticated history.

Expansion emits all explicit Source/6 fields and a field map with source origins.
Transfer: debit value+fee; credit recipient value; fee credit iff fee>0; consume
gross allowance, replay and one head extension in the existing order. Repayment:
credit the obligation's bound creditor, discharge accrued first, then principal;
preserve remaining liability/status; consume allowance, replay and head. Core
derives and compares effects again. Do not introduce a second financial acceptance
engine. Preserve `stage → intent → effect → authority → history → failure` and
first-failure/no-published-effect results. Invalid source may fail earlier at
formation; call that an authoring/formation error, not a Core judgment.

Return `PreparedUnqualified` with the original required external premises and
unverified agreement/program/scale/predecessor bindings. Native proof, signature,
snapshot, head-extension and atomic-ledger consumption stay open. No synthetic
flag, receipt, matching hash or successful AI call supplies them.

## Full-language financial profile horizon

The common notation can describe named profile operations and stages using
records, nominal references and named calls. For non-S0 operations, parser and
name/quantity checks are authoring support only. Report `SpecifiedOnly`; reject
expansion/simulation with `BETA_PROFILE_UNSUPPORTED`, without effects. Do not
claim complete static financial validation of these profiles.

| Family | First proposed operations | Further choices visible in mockup |
| --- | --- | --- |
| AMMs and exchanges | Exact-input pool swap; LP mint/redeem | Reserve/custody invariant, fee rounding, surplus; routes and clearing |
| Lending | S0 funded repayment; origination, collateral lock, liquidation | Aggregate locks, roll-forward, default/loss waterfall |
| Stablecoins/synthetics | Mint, burn/redemption, emergency settlement | Debt/backing/supply authority, peg evidence, shutdown |
| Derivatives | Fully collateralized option fixing/exercise/settlement | Premium/reserve/payoff; perpetual margin/funding |
| Oracles | Selected typed observation; stale/disputed result | Value/unit/source/time/finality authentication; aggregation |
| Governance | Queue/execute/veto/amend | Epoch/revocation/quorum and existing duty preservation |
| Bridges | Source escrow, destination claim, qualified recovery | Paired claims, foreign proof, partial/unknown status; no timeout refund |
| Staking/restaking/yield | Deposit shares, reward/slash, unbond/withdraw | Rounding beneficiary, slash priority, pending withdrawal duty |

One-domain stages compose as ledger-linked episodes with separate attempt and
economic-effect IDs. Local rejection is atomic; actual phase failures can retain
fees/effects only under an explicit signed failure relation. Unknown finality
retains reservation and duty. No ordinary async/await, implicit retry, global
rollback or fallback validates only part of the signed relation.

Full-language functions, ADTs/exhaustive match, certified arithmetic, user
libraries, generics, holes and stage transitions have explicit future obligations.
They are not silently parsed as strings or erased to make S0 run.

## Developer experience and AI tools

Ship a compiled JS CLI and public library in `packages/moriarty-beta`, with
Node >=24 and pinned build tools. Keep the existing experimental source profiles
and regression tests. Package the strict S0 modules into the distributed artifact
so a clean installation outside the repo has no maintainer metadata dependency.

- `mori init DIR`: create a starter project, source, local fixture and case file;
  refuse existing targets and do not overwrite user files.
- `mori check FILE [--json]`: authoring diagnostics, declarations/actions and
  per-action support. Zero diagnostics do not mean proof or financial settlement.
- `mori fmt FILE`: output formatted source; optional explicit `--write` only.
- `mori inspect FILE`: expose nominal identities, source status and signed scope.
- `mori expand FILE --action NAME --scenario FILE`: explicit Source/6 plus map.
- `mori simulate ...`: invoke the actual local preparer; unqualified status.
- `mori test DIR`: execute finite case expectations through the same API/CLI.
- `mori lsp` and `mori mcp`: stdio services, isolated from command output/logging.

Implement LSP 3.17 Full synchronization, UTF-16 editor coordinates, bounded frames,
diagnostics, context-limited completion, hover, definition, document symbols and
formatting only when backed by actual shared services. Partial edits may provide
lexical help; no recovered tree reaches strict financial lowering. Server results
respect document versions. Errors identify asset/domain mismatches and missing
premises without weakening a signed bound as a repair.

Ship TextMate lexical grammar, snippets/brackets and a VS Code extension/client;
reuse the stdio server with Neovim and documented other clients. Semantic tokens,
debugger and per-IDE integrations require their own implementation and tests; do
not advertise them speculatively. Test a real installed Neovim client. A VSIX or
protocol test alone does not establish VS Code activation or JetBrains support.

AI tooling uses the same source checker and emits typed read-only MCP results.
Tools accept bounded source/scenario text, never caller ASTs or arbitrary paths/
shell commands. Check, inspect, expand and local preview cannot sign, send, prove,
deploy, alter policies or configure hosted accounts. Include project-scoped
AGENTS/skills, Copilot instructions, Claude/Codex/Cursor adapter instructions and
valid/invalid examples. No claim of model training, AI understanding or vendor
plugin activation follows from shipping these files.

## Required validation and formal obligations

Use test-first implementation and independent expected outcomes. Cover exact
quantity/scalar typing, financial domain/account mismatch, same-symbol different
assets, duplicate fields/names/identities, Unicode/CRLF coordinates, resource
bounds, formatter idempotence/comment preservation, schema closure and unsupported
profiles. Compare transfer and repayment field by field with independent explicit
Source/6 fixtures, including exact pre/post state, effects, caps, consumed work,
replay/head, partial/full repayment and first Core failure. Challenge missing fee/
credit, stale head, replay, insufficient allowance/work, wrong creditor/asset and
overflow through real command paths. Run existing regressions and type checks.

Exercise LSP framing and document updates with a real client; exercise MCP through
a real protocol client; build/install the packed CLI in a clean temporary project.
Validate distributable editor metadata and package contents. Keep unavailable
application activation and all unperformed financial/native experiments explicit.

Every added sugar needs typed total elaboration, origin preservation and financial
observation equivalence over its declared comparison domain. General K/Quint
elaboration, first-failure correspondence, complete footprint/framing, native
numeric correspondence, authentication, atomic ledger consumption and persistent
episode/duty proofs remain separate obligations. Preserve W-D0–W-D4, M4-C1–C5 and
the U roadmap gates; beta authoring tests do not close them.

## Panel task

Five reviewers receive this same frozen candidate, research memos and mockup
requirements. Independently challenge the scope, grammar, identity/type rules,
authority/completion split, effect/error ordering, tool security and evidence
claims. Give high/medium defects, exact discriminators and recommended changes;
preserve abstention and dissent. Convergence requires disposition of every
substantive finding and at least two substantive agreeing votes per consequential
choice. Model agreement is design review, never executable or formal evidence.
