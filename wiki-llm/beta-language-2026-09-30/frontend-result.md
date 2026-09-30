# Strict frontend implementation result

2026-09-30. Scope: Task 1 in IMPLEMENTATION-PLAN.md, BETA-DESIGN.md and
CONVERGENCE.md v2, including final Source/6 alphabet concurrence in audits 01/02.
This is implementation and experiment evidence for authoring only. No financial,
proof, authentication, correspondence or ledger acceptance claim follows.

## Authored files and interface

- `packages/moriarty-beta/src/frontend.ts`
- `packages/moriarty-beta/tests/frontend.test.mjs`
- This scoped result.

The frontend exports the contracted Span, Diagnostic, Value, Declaration, Action,
Analysis, analyze, check, format and byteToPosition. At the root integrator's
request it also provides `Analysis.agreementSpan` for the exact agreement identifier,
exports Reference and adds `Analysis.references`: bounded rows
`{name,useSpan,declarationSpan}`. References preserve the shared immutable Value
DAG. Quantity.asset names the nominal asset declaration. Entity.name names its
binding; fields.id carries its economic ID. Integers and atoms remain canonical
strings, never JavaScript numbers. No caller-built AST is accepted by these APIs.

`check` returns a JSON-safe summary without declaration Value graphs, plus the
closed operationSchemas catalog, per-action coverage and open gates. Catalog
schemas are immutable. Invalid analysis returns diagnostics and empty declaration,
action and reference lists. Formatting refuses invalid or incomplete edits.

## Implemented checks

Repository observation: lexer retains UTF8 byte spans, literal spelling and
comment trivia. Numeric and asset tokens need a whitespace/comment gap. Decimals
need asset references, use the declared scale exactly and never round. The lexer
rejects malformed separators, leading zeros, non-ASCII identifiers, invalid JSON
strings, lone surrogates, trailing input and unterminated comments.

Repository observation: evaluation is restricted to checked unsigned scalar/Qty
addition, subtraction, multiplication, min/max and atoms. Binary associativity
and precedence are explicit. Qty multiplication by Qty is rejected; addition,
subtraction and min/max require identical nominal units. Every intermediate is
UInt128. Only assigned S0 amounts, fees, caps and floors narrow to S127. Zero
operation amounts remain authorable; acceptance stays in Core.

Repository observation: every declaration refers to prior bindings. Duplicate
names/record keys/call args and duplicate economic identities reject. Domain,
account, asset and obligation records are closed. Assets have integer scale
0..18. S0 IDs and agreement/action transport names use the unchanged Source/6
ASCII alphabet and reserved-word exclusion. Account identity is scoped by domain;
asset representation/scale aliases are rejected. Exact Source/6 reserved words
were inspected in the existing Source/6 frontend.

Repository observation: transfer/repay intents have the full sealed required-field
schema, source-fixed typed operation, matching domain/asset/signer/window and
explicit SuccessOnly/None/empty fields. Transfer requires three distinct account
endpoints, including a zero-fee account; signer must be from. Repay matches its
obligation asset/domain and signer must be payer. Current head, balance sufficiency,
cap acceptance, round validity against a snapshot, repayment outstanding and
financial failure ordering are not evaluated in this frontend.

Repository observation: horizon declarations have a finite field catalog;
operation roles check entity kind, nominal quantities, applicable asset membership
and domain compatibility. Instrument calls require the asset-role declarations
needed by their signature. Textual evidence/policy/footprint fields retain string
or string-array roles. Financial reserve, supply, collateral, governance, option,
bridge, staking and lifecycle relations remain open. Every horizon action reports
SpecifiedOnly and unsupported local preparation.

Repository observation: limits are checked before extending bounded token/node,
declaration, field, type-argument and reference structures: source 65536 bytes,
8192 tokens including comments/EOF, 8192 analysis nodes, depth64,
256 declarations/actions, 64 fields/call/type args, 1024 decoded string bytes,
64 identifier characters and 32768 expression edges. Declaration references are
memoized directly; repeated DAG edges do not recursively clone or serialize
values. Formatted replacement source also has the 65536-byte bound.

Repository observation: formatter changes whitespace while preserving raw
comments and token/string/number spellings, terminates line comments, validates
its output and is idempotent on tested source. byteToPosition converts UTF8 byte
offsets to UTF16 positions, with CRLF handling and scalar-boundary flooring.
Arithmetic result spans cover the expression use. Plain references retain their
shared declaration Value span and separate reference metadata records actual
use and full declaration locations, including annotations.

## Published fixed horizon signatures

The machine-readable authoritative role schemas are returned by check.
The following named arguments are required; additional arguments reject.

| Operation | Named arguments |
| --- | --- |
| amm.swap_exact_input | pool, owner, input, output_asset, net_floor, fee_cap |
| amm.redeem | pool, owner, share_atoms |
| amm.mint | pool, owner, amounts, minimum_share_atoms |
| lending.originate | obligation, debtor, creditor, principal, collateral |
| lending.liquidate | obligation |
| lending.roll_forward | obligation, to_round |
| stablecoin.mint | instrument, owner, supply, backing |
| stablecoin.redeem | instrument, owner, burn, minimum_backing |
| stablecoin.emergency_settle | instrument, owner, claim |
| option.fix | instrument, observation |
| option.exercise | instrument, holder |
| option.settle | instrument, holder, payoff |
| oracle.select | observation |
| governance.queue | policy, next_epoch |
| governance.execute | policy |
| governance.veto | policy |
| bridge.escrow | owner, amount, destination, claim_id |
| bridge.claim | owner, amount, source, claim_id |
| bridge.recover | owner, amount, claim_id |
| staking.deposit | owner, shares, backing |
| staking.reward | shares, amount |
| staking.slash | shares, amount |
| staking.unbond | owner, shares, share_atoms |
| staking.withdraw | owner, shares, share_atoms, minimum_backing |

Selection explanation: signatures already shown by PROGRAMMER-MOCKUP.md were
preserved. The remaining names received fixed conservative authoring signatures,
with explicit roles and no dynamic call fallback. These signatures are local beta
catalog choices, not previously proved financial interfaces or executable horizon
semantics. The integrator was notified before implementation.

## RED and GREEN evidence

Experiment observation: the first command
`node --test packages/moriarty-beta/tests/frontend.test.mjs` exited 1 with
0 passed / 9 failed before production code existed. The missing analyze export
was the expected failure. The initial implemented suite then exited 0: 9/9.

Experiment observation: additional discriminators produced observed RED failures
for missing stablecoin asset-role requirements, incorrect arithmetic use spans,
missing reference metadata, omitted annotation reference metadata and unconstrained
horizon textual declaration roles and missing exact agreement origin spans. Each was repaired and the scoped suite rerun.
Catalog enumeration and boundary expansion are verification extensions; they
passed on their first run and are not claimed as new RED cycles.

Final experiment observation:

- `node --test packages/moriarty-beta/tests/frontend.test.mjs`: exit 0,
  19 tests passed, 0 failed.
- `npm --prefix packages/moriarty-beta run typecheck`: exit 0,
  strict TypeScript package compilation.

The suite includes all 24 horizon names and unknown extra arguments; exact
quantities; mismatches and duplicates; S127/UInt128 boundaries; same-ID foreign
accounts; scale/string/field/token/declaration/depth rejection; shared immutable
DAG stress; Unicode/CRLF; use/definition spans; formatter comments/idempotence;
and incomplete-source rejection. Existing Source/6 and whole-package result
audits are the root integrator's scope.

Frozen scoped hashes at handoff:

- frontend.ts SHA256 `bd5f78dd59ea69fd05d820e8848bc241b62800784a478cb8f04eb909eb0b2fc8`
- frontend.test.mjs SHA256 `c05071913c944464c8a9bfa587b7f5c00726e5757fa46782bbc903b79df1dda2`

## Remaining integration obligations

This author does not claim independent result review. Root still owns Source/6
origin mapping, artifact bounds, actual Core comparison, scenario closure,
CLI/services/distribution, typed proposed horizon documentation and both fresh
whole-candidate audits. Source/6 reserved-word/alphabet compatibility is an
inspected and tested authoring restriction, not a correspondence proof.
