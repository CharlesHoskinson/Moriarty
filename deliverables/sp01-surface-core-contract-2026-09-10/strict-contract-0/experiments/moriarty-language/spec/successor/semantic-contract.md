# Proposed successor expression contract

Status: **PROPOSAL — SP01.3 expression layer only.** Contract identifier
`moriarty-expression-contract/0`; this is not a registered execution profile.
It does not replace `moriarty-funded-source/0`, the syntax profile, or archived
candidate04. No full RP01, SP02, SP03, theorem, signature or Midnight acceptance
follows from this document. All new choices below require the current independent
GPT-6/Grok reviews. The 38 proposed financial operations are not defined here.

This document and [static-semantics.md](static-semantics.md) give the meaning of
all 40 expression constructors declared in candidate04 B.1. The complete index
is [expression-signatures.json](expression-signatures.json). The exact mathematical
JSON embedding and every byte/node aggregate are defined by
[representation.md](representation.md); no implementation may choose another
layout for these semantic bounds. Structural validation
of that index is distinct from the [specified derivation cases](expression-cases.json).
The documents, not the structural validator, define the proposed semantics.

## Authority and decisions

Inherited requirements: immutable pre-state; sequential immutable locals;
one tentative write per field; no `next` reads; post-state available only in the
final `ensures` suffix; first lexical failed clause; financial rollback; finite
work; distinct nominal debt and tokens; no host flag establishes acceptance.
Sources: [LANGUAGE-DESIGN](../../../../deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:46),
[SP01.3](../../../../openspec/sprints/sp01-financial-contract-and-execution-admission.md:49),
[SP03](../../../../openspec/sprints/sp03-executable-bounded-semantics-in-k.md:39).

The following are **new proposed choices**, not inherited approvals:

| ID | Choice and reason |
| --- | --- |
| EX-D1 | Strict left-to-right operands, including `And`/`Or`. Every subtree is checked and evaluated; an overflowing right operand is not hidden by false/true on the left. A later short-circuit language must use a distinct reviewed contract. |
| EX-D2 | Checked same-type scalar arithmetic, indexed amount/share addition and subtraction, explicit quantity scale/unit arithmetic. No implicit asset, scale, width or debt conversion. This makes overflow and rounding visible. |
| EX-D3 | `ReadPre` has a closed view operand `pre|post`; `post` is legal only in Ensure's condition. No new 41st constructor and no `ReadNext` are introduced. Read-view is part of Core encoding, never inferred from a name. |
| EX-D4 | Every entered node costs one ordinary work unit. Finite helper operations do not each count as nodes. Recovery work is unchanged here. Failed preparation reports consumed local work, but publishes no candidate financial state/effects. This is a proposed local cost semantics, not native proof cost or authority consumption. |
| EX-D5 | AccessField/AccessIndex are exact aliases of ProjectField/ProjectIndex, including errors and work. Core preserves their names; an optimizer may not delete their node charge without a separate correspondence argument. |
| EX-D6 | Emit appends a typed operation descriptor only. Effects and financial state changes require the separately defined financial-operation transition. There is no default Transfer, repayment or other execution guessed from the descriptor. |
| EX-D7 | Finite parameters below are proposal limits, independent of existing parser/funded bounds. UTF-8 spans are half-open byte offsets. Whole-action static rejection precedes any execution. |
| EX-D8 | Unit is statement-result-only; schema fields are classified ordinary/financial. Raw NextWrite cannot bypass financial transitions. These are proposed type/registry mechanisms enforcing the inherited no-debt-erasure requirement. |

The earlier surface proposal explicitly selects short-circuit `and`/`or`
([line71](../../../../deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:71)).
EX-D1 does not override it or establish source/Core correspondence. Resolve that
choice before a surface/elaboration freeze. Its AMM fragment also needs mixed
Amount products/division absent from this proposed overload table; those target
requirements remain open, not removed by TYPE_MISMATCH here.

Unresolved privacy, observation authentication, operation effects and signing
policy are explicit external obligations. Mathematical values below do not
establish permission to disclose a value, condition, error or state. A future
public execution profile must bind its confidentiality and observation policy;
this proposal cannot accept a private workflow by omitting that policy.

## V — finite domains and values

A candidate binds a finite schema environment Σ and input snapshot before
checking the action. No host callback, arbitrary JSON, mutable object reference,
user function, recursion or implicit cast is a Core value.

Proposed simultaneous limits: 65,536 source UTF-8 bytes; 4,096 AST nodes;
64 nesting depth; 256 declarations and 256 statements; 64 fields per record;
128 members per collection/enum; 1,024 UTF-8 bytes per Text; identifiers match
`[A-Za-z][A-Za-z0-9_]{0,63}`; at most 65,536 canonical UTF-8 bytes and 4,096
nodes in each complete input value or assembled candidate state; at most 128
operation descriptors, each within the same value limits; work 0..65,536.
A value node is one scalar or compound value constructor; names, type metadata
and tags are not additional value nodes. A compound contributes one plus its
child value nodes, counting repeated occurrences separately. AST nodes count
entered constructor occurrences, not metadata or list containers.

All aggregate collections, including Σ, arguments, observations and pre-state,
are checked against those byte/node/depth bounds as complete records, not only
against per-field bounds. Σ type definitions form an acyclic graph. Cardinalities
must fit simultaneously: products of individual maxima do not grant larger
aggregate limits. Inputs failing a bound never enter evaluation.

These maxima are proposed contract parameters motivated by the retained bounded
proposal and parser. They have no native fit, runtime performance or usability
claim. Changing a maximum changes this contract's eventual profile digest.

Values have exact types, recursively:

- `UInt64`: 0..2^64−1; `UInt128`: 0..2^128−1;
  `SInt128`: −2^127..2^127−1; `Bool`. `Unit` is a statement-result marker only,
  not a storable, argument, observation, field or collection/option element type.
- `Text`: a well-formed Unicode scalar sequence whose UTF-8 is at most 1,024
  bytes. No normalization; comparison is exact decoded scalar equality.
- `Amount<A>`: UInt128 quanta indexed by a declared asset identifier A.
- `Shares<V,H>`: UInt128 share quanta indexed by a declared vault V and holder H.
- `Rate<S>`: signed SInt128 mantissa with type-level scale S, 0..18; value m/10^S.
- `Price<A,B,S>`: UInt128 mantissa, declared distinct base A and quote B,
  scale S, direction fixed as base-per-quote; value m/10^S A per B.
  In particular Price<AssetB,AssetA,4>(19743) is 1.9743 AssetB per AssetA,
  matching the retained swap convention; parameter order is not interchangeable.
- `Quantity<U,S>`: SInt128 mantissa with scale S and a canonical unit vector U over the separate declared Σ.units roster.
  U contains at most eight distinct identifier/exponent pairs in increasing ASCII
  identifier order. Exponents are nonzero integers −128..127. An empty vector
  is dimensionless. Values denote m/10^S times the product of symbol powers.
- `Record<R>`: every field of the declared record R exactly once, with its
  declared type. Field names are sorted by ASCII identifier for value encoding;
  source constructor operands retain lexical order for evaluation.
- `Enum<E>`: one member of declared enum E. Payload enums are not implicit;
  use a record plus an option when required.
- `Option<T>`: `None<T>` or `Some<T>(v:T)`.
- `Collection<T,N>`: ordered values all of T, length 0..N, 0≤N≤128. Capacity is
  in the type; equal lengths do not make different capacities/types equal.
- `Operation<O>`: descriptor `{operation:O, fields:Record<R_O>}` for a declared
  operation O with exact operand schema R_O. It contains no executable callback.
  This descriptor is formed by Emit; it is not a ledger effect.

Record schemas may encode financial identities and debt records, but these
expressions confer no authority to create/change a financial resource. Persistent
field registry entries include `writeClass=ordinary|financial`; only ordinary
fields may be targeted by NextWrite. Financial fields are read-only at this
layer. A profile must define the complete financial operation relation and
reconcile its footprint with staged ordinary writes before financial execution.
This prevents a raw next-write from erasing debt or bypassing cash effects.

Core metadata comprises identifiers, integers, type references, record field
names, enum members and read views. It is bounded, not executable. Numeric
metadata and values use canonical decimal strings: unsigned `0|[1-9][0-9]*`;
signed adds `-[1-9][0-9]*`. No `+`, leading zeros or `-0`. Booleans use JSON
booleans. Each Core node is a closed tagged record with its named operands and
span, no extra keys. No duplicate JSON keys. Canonical serialization uses sorted
object keys, ordered arrays, UTF-8 with no BOM, no insignificant whitespace and
no Unicode normalization; escaping is JSON minimal (quote/backslash/control
characters escaped, other scalar characters emitted). Quote/backslash use their
short JSON escapes; backspace, form-feed, newline, carriage-return and tab use
\b, \f, \n, \r and \t respectively; other U+0000..U+001F use lowercase
\u00xx. This fixes the escaping choice. Any future wire codec
must implement and test this; the structural checker is not that codec.

## E — evaluation contexts and frames

The configuration is `⟨control, pre, writes, locals, args, obs, descriptors,
workInitial, workRemaining, result⟩`. Pre/args/obs/Σ are immutable finite snapshots.
Writes and locals begin empty; descriptors begin empty. Existing duties,
authority and recovery reserve are part of the supplied state, not fresh budget.
Snapshots are assumed supplied consistently; this layer does not authenticate them.

A frame `C ::= [] | K(v1,…,v(i−1),C,e(i+1),…,en)` ranges over the
constructor's **evaluatedOperands**, in listed order, expanded in lexical list
order. Metadata operands are not evaluated. For Let only its value is evaluated;
NextWrite only its value; Require/Ensure only their condition. Read/literal nodes
have no expression children. Record/list children are evaluated in source order,
not canonical field order. All children of a well-typed expression are evaluated,
including both sides of And/Or, before applying its rule.

Small steps are deterministic proposals:

1. On entering a node with work=0, reject `WORK_EXHAUSTED` at that node without
   entering children. Otherwise decrement work by one and mark the node entered.
   Resuming its context does not charge it again.
2. Enter children left-to-right. The first rejection discards the entire control
   continuation, tentative writes, locals and descriptors.
3. Apply the constructor rule after all children return values. Primitive guard
   order is specified by the rule. Failures point to that constructor's span.
4. A statement's success advances to the next lexical statement. Ensure clauses
   are a suffix and see `post=pre overridden by writes`; that view does not commit.
5. After all statements/ensures pass, check aggregate candidate encoding bounds
   and return `ExpressionPrepared(post,descriptors,workRemaining)`.

Result is **ExpressionPrepared**, never accepted `Prepared`, `Complete` or
`Pending`. It is not financially executable without the operation-layer rule.
For a descriptor-free ordinary action it specifies a tentative state only.
An external financial rule must consume descriptors, account for its own work,
produce complete ordered effects, preserve duties and resolve postcondition
visibility before the combined language can execute. This missing composition
is not silently given an evaluation order here.

Rejection is `Rejected(code, span, nodePath, workUsed)` with no post-state or
financial effects; workUsed is local attempted reduction cost, not a committed
ledger/authority charge. Input admission/static rejection has workUsed=0.
Runtime workUsed=workInitial−workRemaining includes the entered failing node
unless failure is inability to enter it. No failed preparation increases a
stored work budget or permits an on-chain retry. Continuing, charging a failure
or using recovery work belongs to the future lifecycle/acceptance relation.

Span `[start,end)` is UTF-8 byte offsets, 0≤start≤end≤sourceBytes. Elaboration must
retain real node spans; synthetic Core has explicit synthetic spans and cannot
claim a source diagnostic. For malformed inputs use the first offending field's
node span when available, otherwise `[0,0)`. Synthetic spans must be explicitly
marked and exactly `[0,0)`; they cannot fabricate source offsets. `nodePath` is an ordered child-index
path from the action root; statement index is its first element. For an isolated
expression derivation, its root path is empty and children begin at index zero. For post-final
aggregate failure use the action's span/path. No wrapping private data in errors.

## R — primitive reduction rules

The exact rules and all overloads appear in the companion static contract.
Common rules apply to all 40 nodes: admission and typing first; entering/child
ordering above; no mutation by pure expressions; no implicit helper effects;
checked finite result before returning. Integer checks compare mathematical
integers to the declared bounds and never wrap. A multiplication checks its
product before any subsequent division, even when the final quotient would fit.
Literal/metadata validity is checked statically; runtime snapshot equality and
value domain checks occur during whole-input admission, not arbitrary late reads.

Post-state fields not written retain their exact pre-state values, including
financial records and identities. No integer debt subtraction is a transfer.
Only the operation-layer contract can authorize discharge/write-off/migration;
RequestFulfill/Claim, MessageRefund, RewardAccrue/Claim and EventClaimSettle
remain underdetermined in candidate04. This file invents no payout or funding rule.

## Open interfaces and theorem scope

The archived signing proposal's 38 financial rows, genesis funding, full
contextual negative histories, cancellation backing and display identity fixes
remain open. This 40-constructor document does not reconcile the full RP01
challenge map or vote on an accepted base-Core membership list.

A later combined contract must resolve financial-operation execution versus
ordinary staged writes and postconditions, authenticated/labelled observations,
privacy and disclosure, canonical codec implementation, lifecycle/authority work
and acceptance statuses. Until then, a full successor runtime must reject an
attempt to execute these missing interfaces; descriptor preparation alone cannot
be exposed as a completed financial action.

The six SP03.3 obligations remain determinism, progress-or-defined-rejection,
type preservation, finite termination, residual-duty conservation and
elaboration/evaluator correspondence **for the frozen base domain**. This source
contract and its derivations are not mechanized proofs. No K proof or frontend
correspondence is performed here. The existing Transfer/Repay evaluator, numeric
K evidence and compiled Midnight loan/swap evidence retain their original scope.
