# Beta convergence: selected contract v2

2026-09-30. Five independent seats were dispatched with explicit host arguments
`model: gpt-6.1-sol`, `reasoning_effort: medium`, `fork_turns: none`, in two waves.
The host acknowledged all five tasks. It exposes no separate returned provider
identity/effort receipt; reports preserve that limitation. All reviewed the same
v1 SHA `6af4ca5356ed3fd58818b5717401d5b218b672b65769e342f708d4a4e0cd061a`.
Seats 4/5 also inspected the recorded supplementary mockup. These are design
votes, not proof or acceptance evidence.

## Selection and preserved dissent

All five agree substantively on a standalone bounded brace DSL, exact nominal
quantities, explicit financial identity, existing Source/6–Core/5 reuse, separate
untrusted completion/evidence, and keeping native/ledger gates open. At least
seats 1/2/3 agree on shared authoring services and preserving price orientation.
Choose those approaches. Preserve all original reports under `audits/`.

The domain02 proposal of Quote-per-Base is rejected for this iteration: the
controlling U0 convention remains Base-per-Quote. No implicit reciprocal.
The initial mockup's prose interfaces do not meet all full-language requirements;
replace them with explicit typed horizon contracts and composed signed bounds.
No generic records receive a financial-typing claim.

## Disposition of every substantive finding

| Findings | Disposition |
| --- | --- |
| 01 S1; 05 identity | Adopt resolved economic IDs, explicit transport restrictions below; declaration names only bind/navigation. |
| 01 S2; 02 F01; 03 TS03; 04 Qty; 05 range | Adopt UInt128 expression values, field-specific S127 narrowing, no float/coercion. |
| 01 S3; 02 F02; 05 rejection | Adopt total proposal builder with unchanged syntactic debt placeholder on overpayment; Core decides economic failure. |
| 01 S4 | Adopt separated quantity tokens, canonical numeric spelling, comment-preserving formatter and semantic/idempotence tests. |
| 01 S5; 03 TS02; 05 resource | Adopt explicit per-input, work, output and service limits below. |
| 01 S6; 03 TS04; 05 origin | Adopt input digests, byte spans, scenario JSON pointers and bounded derived dependencies; UTF16 conversion at editor boundary. |
| 02 F03; 04 typed interfaces/composed binding/lifecycle | Complete typed horizon mockup with promised bounds, explicit stages/duties, omitted financial lifecycle operations and construct statuses. |
| 02 scenario; 03 TS01 | Adopt raw duplicate-safe closed scenario schema, exact ordered cells and no hidden defaults. |
| 03 TS05; 04 status | Adopt closed operation registry, separate recognized/name/quantity/local-preparation coverage; unknown names/arguments reject. |
| 05 observations/validation | Adopt explicit comparison domain and independent Source/6 expectations, complete effects/post/work/head and negative probes; formal claims remain open. |

## Exact amendments to v1

Economic IDs, not declaration names, are emitted into Source/6 and scenario keys.
Domain/account/asset/obligation IDs and agreement names must satisfy Source/6
identifier syntax `[A-Za-z][A-Za-z0-9_]{0,63}` and its reserved-word exclusion
when used by S0. Do not replace an unrepresentable ID. Reject duplicate domain
IDs, duplicate account identity within a domain and duplicate asset economic ID
within a domain (including conflicting representation/scale). Explicit aliases
are unsupported. Domain chain/network metadata and asset representation/scale
are unverified claims not carried as additional authenticated Core fields.

All scalar/Qty expressions use `0..2^128−1`, checked at every operation. S0 values,
fees, caps, floors and obligation principal/accrued/outstanding use
`0..2^127−1`; gross debit, balances, counters and rounds use UInt128. Zero action
amount remains syntactically expressible; Core rejects its intent. Lowering does
not pre-evaluate caps, stale head, validity against current round or economic
balance sufficiency. These judgments stay in Core.

Numeric spelling is `(0|[1-9][0-9]*(?:_[0-9]+)*)(?:\.[0-9]+(?:_[0-9]+)*)?`,
with no leading zeros after removing separators. A decimal needs a following
asset reference, separated by whitespace or a comment; `10.00USD` rejects.
Newlines/comments may separate quantity tokens. Bare scalars are integers.
No unary minus; binary operators associate left; qualified calls use dot-separated
ASCII identifiers. Every declaration refers only to prior declarations.

Automatic proposal construction computes transfer lines without testing balance
or cap acceptance. For repay amount above well-formed outstanding, emit unchanged
principal/accrued/outstanding/status as the **documented invalid proposal**, plus
the requested debit/credit and administrative lines. Core rejects Intent first
if applicable, otherwise Effect range, before comparing that placeholder. Never
publish it as a prepared post-state. This rule is proposal totality, not a debt
transition or acceptance check. Explicit candidate effects remain available and
must satisfy Source/6's representable shape; malformed candidate shape is a
formation failure. No first-failure equivalence is claimed outside representable
Source/6 inputs. Within that domain compare exact wrapper observations including
formation/Core boundary, first judgment/code, null published results or complete
candidate state/effects, work, premises and bindings.

Scenario transport `moriarty-local-scenario/1` requires exactly:
`profile`, `kind` (`local-stipulation`), `domain`, `asset`, `head`, `predecessor`,
`round`, `balances`, `allowance`, `replay` (`unused|consumed`), `work_remaining`,
`work_spent`, `post_head`; optional `obligation` only for repay and
`candidate_effects`. Balances are ordered `{account, amount}` rows, exactly three
for transfer (including the distinct fee account when fee=0), exactly two for
repay. Allowance is `{owner, remaining, spent}`. Obligation is
`{id, debtor, creditor, asset, principal, accrued, outstanding, status}` with
status `Outstanding`. All integers are canonical decimal strings; every record
is closed and every raw decoded duplicate JSON key rejects before JSON.parse
can erase it. No AST/object scenario API invokes caller getters. Obligation
sum/counter totals and endpoint/cell shape follow strict Source/6 formation.

Closed horizon operation registry: `amm.swap_exact_input`, `amm.redeem`,
`amm.mint`, `lending.originate`, `lending.liquidate`, `lending.roll_forward`,
`stablecoin.mint`, `stablecoin.redeem`, `stablecoin.emergency_settle`,
`option.fix`, `option.exercise`, `option.settle`, `oracle.select`,
`governance.queue`, `governance.execute`, `governance.veto`,
`bridge.escrow`, `bridge.claim`, `bridge.recover`, `staking.deposit`,
`staking.reward`, `staking.slash`, `staking.unbond`, `staking.withdraw`.
Each has a fixed named argument schema (published by the checker). Structural
name/reference/quantity checks are distinct from open financial relation checks.
Only transfer/repay lower. Full-language typed lifecycle contracts remain proposed
syntax in a separate horizon document, not silently accepted executable grammar.

Budgets: source 65536 bytes, 8192 tokens/nodes, depth64, declarations256,
fields/arguments64, strings1024 bytes, identifiers64; expression edges32768;
scenario65536 bytes/depth32/nodes4096; generated Source/6<=65536 bytes and
8192 tokens; origins<=2048; protocol header8192 bytes/body262144 bytes;
serialized response524288 bytes; LSP open documents32/aggregate1048576 bytes;
one synchronous running analysis (no unbounded promise queue); MCP admission
one synchronous request per input frame. Enforce transport admission before
body buffering; close framing violations. No partial generated artifact on bound
failure. Memoize declaration values without expanding reference DAGs.

Public APIs accept source/scenario **text**, recheck exact bytes and return
digests. No caller AST eligibility flag. LSP Full edits invalidate prior analysis;
only monotonically increasing versions, unique open URIs and current generation
publish diagnostics. Formatting rejects invalid source. All source byte offsets
convert through one UTF16 helper. Inputs/derived origins are tagged separately;
generated selection is a generated rule, never fabricated source authentication.

## Implementation authority and remaining gates

The user authorized AFK implementation after convergence. Apply these amendments
to the candidate and implement the bounded beta; no further user checkpoint is
needed. Panel concurrence on amendments is recorded separately when returned.
Do not mark the mockup complete before its typed-horizon repairs, or mark native
financial settlement, authentication, K/Quint correspondence or plugin activation
complete from authoring tests.

## Final bounded authoring repairs

Final inspection limits the expanded signed-scope projection to depth64 as well
as8192 work nodes/524288 serialized bytes; shared declaration DAGs remain valid
source until an actual authoring rule rejects them. Projection refusal reports
InspectionRejected with original authoringStatus, not a source error.

Bridge owner/custody and amount use the same local domain; only named source/
destination parameters may name the foreign domain. Provided horizon domain,
asset and signer headers align with that operation domain, including rounds.
Provided cap quantities use the declared asset; bridge can infer its local amount
asset when omitted. AMM swap net_floor uses the explicit output asset. These are
nominal authoring checks, never financial relation/proof qualification.
Share-class IDs are unique within their declared domain and resource kind.
Intent retained-duty hints are text or arrays of text; stage retained-duty hints
are text only. Stage signed-floor hints are
text or a Qty on that stage domain. Their financial truth remains open.
Unsupported simulation explicitly does not validate or apply its scenario.
