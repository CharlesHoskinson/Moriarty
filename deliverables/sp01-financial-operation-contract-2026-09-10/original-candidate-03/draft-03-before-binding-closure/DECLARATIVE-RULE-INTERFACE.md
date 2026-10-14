# Finite financial-rule interface — proposed draft02

This extends the preserved draft01; it is not an accepted change to expression
candidate02. It supplies a concrete alternative to undefined host RuleRef calls.
A full product profile still has to populate each required rule body and schema.
The absence of a body is a static unsupported-profile error, not a runtime choice
to trust an amount supplied by a user. This document specifies the proposed rule
mechanism; it does not yet close all38 operation records or full RP01.

## DR1: A rule is typed immutable Core, not a function pointer

A `RuleDefinition` has exactly: `id`, `version`, `kind`, `parameters`, `resultType`,
`body`, `allowedObservationNames`, `maxNodeEntries`. `id` and `version` are bounded
identifiers, not hashes masquerading as callable bodies. The bound profile stores
the exact body and commits to its canonical representation; a hash-only missing
body rejects. No dynamic import, callback, recursion, nested rule invocation,
user-supplied predicate, opaque native function or arbitrary JavaScript is allowed.
The `body` is one pure Core expression. Its syntax is the expression contract's
pure constructor subset, with no Let/Require/NextWrite/Emit/Ensure and no reads of
locals, persistent state or observations. It may use ReadArg only for the finite
parameter names in the definition. Literal IDs/constants cannot evade typing.

Parameters are a lexical ordered list of distinct `{name,type,binding}` records,
at most64. Each type is one of the closed finite Σ types for this profile. No Unit
or Operation value is admitted in a rule input/result, recursively. `binding` is
one of four exact tagged forms:

- `Operand(name,fieldPath)` selects a typed current operation operand;
- `State(recordKind,identityOperand,fieldPath)` selects that operation's current
  overlay object, not arbitrary state supplied separately by the caller;
- `Context(name,fieldPath)` selects an immutable authenticated context projection;
- `Constant(name)` selects a literal in the immutable profile constant registry.

Every path is a list of at most8 declared field names; no wildcard, list index,
computed property or fallback resolution. List-valued parameters select the
whole bounded collection. A rule may inspect its elements only with the existing
explicit bounded Core projections, so there is no uncharged loop hidden in a
selector. `recordKind` is from the financial registry; the kind/identity operand
pair and every context name must be on that operation's explicit read-footprint
allowlist. Unknown object/field/identity rejects before entering the rule.
A body cannot obtain a new authority or fresh observation by constructing a
similar-looking record.

Each operation kind has a **RuleSignature** fixed by the reviewed financial
contract: ordered parameter names/types/binding roles and exact output type.
An instantiated asset/vault/party type substitutes only declared identities,
never arbitrary type erasure. A profile may choose a body satisfying that signature;
it may not widen the signature's accessible state or add an output setter. The
[rule-kind table](rule-signatures.json) records the current proposed signature
families and their remaining schema work. Polymorphic notation is not a wire
codec: a complete profile has only concrete Σ types after substitution.

## DR2: Admission, evaluation and errors

Static profile admission verifies the exact RuleDefinition shape, unique ID,
version and kind; resolves/monomorphizes its RuleSignature; verifies all parameter
binding paths against the operation footprint; then types the whole pure body.
Result type must exactly equal the signature, including asset/holder/scale and
record identity. A Bool from a caller operand is not a policy result: policy
signatures below deliberately have no `accepted`, `verified`, `authorized` or
other precomputed decision parameter. They accept factual fields and immutable
policy constants. Any required cryptographic credential/observation authentication
is a separate mandatory premise of context admission, not an input Bool shortcut.

All bodies are at most4,096 nodes/depth64 and must fit candidate02's exact Core
byte bound. One profile binds at most128 rule definitions, whose **combined**
canonical rule table is at most65,536 bytes/4,096 Core nodes; separate per-rule
bounds do not permit128 independent maximum bodies. Definitions are acyclic
because rule calls do not exist. Every input, output and context aggregate also
obeys the exact schema-directed value representation bounds. Extra mathematical
arithmetic types in MIXED-ARITHMETIC-PROPOSAL need their own reviewed version and
codec extension; an unextended expression profile cannot admit those bodies.

At an operation, resolve each input from its declared current operand/overlay/
context source exactly once. Validate the complete parameter record before
reduction. Evaluate the body with left-to-right pure expression semantics with the proposed Boolean /1 short circuit;
charge each entered node once. Errors are the originating expression code plus
rule ID and body node path, with the calling Emit's real source span retained.
The public error may expose only code/rule ID/allowed span; it must not serialize
private parameters. A well-typed result is checked against result encoding/range
bounds before any consuming state update. No rule result is treated as a new
trusted observation, ledger receipt or signature.

Proposed deterministic additional errors: `RULE_MISSING` (absent body),
`RULE_KIND`, `RULE_SIGNATURE`, `RULE_BINDING`, `RULE_FORBIDDEN_CONSTRUCTOR`,
`RULE_PROFILE_BOUND`, `RULE_RESULT_BOUND`; then the existing expression errors.
A computed policy false becomes the operation's `POLICY_REJECTED`; it is not a
syntax error or a proof-system result. A computed amount differing from a proposed
operand becomes `RULE_AMOUNT_MISMATCH`. No caller field names the error to return.
Malformed rule definition rejects profile admission before action execution;
malformed action/static error precedes input resolution; rule arithmetic failure
precedes funding/cap checks; final Ensure remains after all operations. Thus an
unfunded payout with an overflowing rule does not falsely report only liquidity.

## DR3: Computed numbers do not grant arbitrary state writes

The financial operation consumes a RuleResult through its own fixed equations.
Rules return only the signature's scalar/closed decision fields. They cannot return
a financial snapshot, balance map, effect list, receipt, authority token, predecessor,
remaining-work quantity or instruction to delete a claim. The base operation
constructs all actual movements and effects from fixed roles, typed computed
quantities and checked input resources. It validates conservation, bounds,
backing, receipt allocation, signed authority and residual duties independently.

For example an EventPayout rule computes the finite contingent entitlement q from
stake, the resolved supported outcome and fixed policy coefficients. Resolve
stores q as a duty of the existing named obligor to the original claimant.
Settle accepts at most that duty and requires actual matching funded receipt
allocations. Returning q=1,000,000 for stake1 is not automatically invalid for all
derivatives; it is admissible only if that exact reviewed product formula and
signed creation/payout authority permit it and its funding/credit model is explicit.
The archived underdetermination was allowing an arbitrary payout without such a
formula/obligation/funding relation. A fixed1-payout fixture must reject1,000,000.

A RequestConversion rule takes inputFill and the bound rate/accounting inputs;
it returns only outputEntitlement and explicit fee quanta, never a status.
RequestFulfill applies P−=fill,F+=fill,E+=entitlement, exact two-resource transfers,
fee movements with fixed recipients and output escrow backing. Claimable is E−K;
the profile cannot replace it with an independently supplied claimed/filled label.
Rules may authorize future fill only under a bound guard. Cancellation preserves
E and K even if a policy rule says future fill is closed.

A debt allocation rule returns `{principalDelta,interestDelta}`; the base operation
checks dP<=P,dI<=I,dP+dI=nominal. Its separate settlement rule computes the exact
required token quantity/fee/refund allocation. It cannot make nominal writeoff a
payment, spend the same receipt twice or discharge a differently denominated debt.
A rule defining a financial model must itself satisfy the relevant contract
property and intent/history assumptions. Well-typed pure Core is not proof that
a chosen interest formula is economically appropriate or faithful to ACTUS.

## DR4: Complete finite cost requirement and residual boundary

Proposed work debit for operation k:
`1 + sum(entered nodes of invoked rule bodies) + explicit fixed helper costs(k)`.
The descriptor's earlier Emit/operand expression costs are charged separately
once. Pure rule body nodes are not counted again when reading their result.
Static `maxNodeEntries` must equal the complete syntax body node count, including
both Boolean branches, not an author-selected optimistic number. Actual cost
counts only entered nodes under Boolean /1. A rule failing early
charges entered nodes only; static admission reserves the conservative full bound.
No hidden loops, iterations or recursion can make actual selected-node cost exceed it.

The **remaining concrete cost task** is a fixed table for complete state-operation
helpers: bounded record lookup, receipt allocation scans, movement/policy legs,
reservation/claim updates, and full result-bound checks. It must state whether
cost is logical per action or per visited collection cell, and how whole-profile
lifetime reserve accounts for it. Draft01's nominal one-op charge alone is not
that table. A future bounded fold or iterative AMM needs an explicit Core form,
registered maximum iterations and termination/rejection rule; it is not smuggled
in via these pure rule definitions. No native circuit-row estimate is inferred
from logical node work.

Until that table, the concrete schemas/identity sorts and all operation consumers
are specified, this remains a bounded **interface draft**, not a full generic
financial evaluator contract. SP07/SP08 may instantiate independently sourced
product equations after the mechanism is accepted; they cannot defer the generic
state-update, authorization, rounding, failure and claim-continuity relation.
