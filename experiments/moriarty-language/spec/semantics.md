# Moriarty bounded atomic profile: static and dynamic semantics

Status: S2, specified-only. Profile `moriarty-bounded-atomic/1`. This document defines a candidate profile; no frontend, proof system, or ledger acceptance path implements it.

## Domains and judgments

Let `U = {0..2^128-1}`. A nominal unit is a declared identifier. A unit vector `q` maps at most eight declared units to nonzero exponents in `[-16,16]`; absent components have exponent zero. `1` is the empty vector and `unit(X)` maps `X` to exponent one.

Types are `UInt128`, `Text`, `Bool`, `Amount<X>`, and expression-only `Quantity<q>`. `Bool` and nontrivial `Quantity` cannot occur in genesis state, arguments, observations, constants, or stored results. `Amount` without a unit parameter is legal only in an effect schema: its emitted expression supplies the nominal unit, which the bound manifest records.

The static judgments are:

```text
G |- e : T ! q             expression e has type T and unit vector q
G ; S ; L |- stmt => S'    statement is typed; L contains earlier locals only
P |- agreement => typed    all declarations and actions check under profile P
typed |- elaborate => M    typed AST deterministically produces manifest M
M ; sigma ; a ; A ; O |- step => R
R ::= Rejected(diagnostics) | Complete(receipt) | Pending(progress)
```

`G` contains declared units, constants, state fields, observations, arguments, effect schemas, settlement bindings, and field policies. `S` is the state schema, not values. Duplicate declarations reject per namespace. A bare identifier denotes only an earlier `let`. State writes must name declared dynamic state. Constants and checked genesis metadata cannot be written.

## Expression typing and arithmetic

Literals have these types: `uint(n): UInt128 ! 1`, `text(s): Text`, `amount(n,X): Amount<X> ! unit(X)`, and Boolean literals `Bool`. Numeric text is never coerced. Source integer tokens are canonical unsigned decimal and must be in `U`.

For `+` and `-`, operands and results have the same numeric type and unit vector. For `*`, vectors add and the result is `UInt128`, `Amount<X>`, or expression-only `Quantity<q>` according to the resulting vector. `floor_div(n,d)` subtracts the denominator vector and is the only division construct. It evaluates both operands left-to-right, rejects zero `d`, and returns `floor(n/d)` with the classified result unit. Each operand and each multiplication/addition/subtraction result must be in `U`; a later division cannot rescue an overflowing intermediate. Subtraction rejects underflow. Reassociation or cancellation is forbidden when it changes evaluation, overflow, or rounding.

Ordering requires equal numeric types and vectors. Equality requires identical types and, for numeric values, identical vectors. `and` and `or` require `Bool` and short-circuit left-to-right; `not` requires `Bool`. Assignment requires exact type and vector equality. Unit checking occurs during typed checking and is reified into the manifest; proof statements bind those checked vectors and rounding nodes. Source unit annotations are not optional documentation and cannot erase units.

## Declarations, effects, and policy binding

`lifetime` is a positive UInt128 genesis allowance. `horizon` is an exclusive UInt128 UTC epoch-second upper bound. Both, the initial state, schemas, source/profile versions, declarations, action bodies, unit map, policies, and settlement bindings are checked and authenticated genesis metadata. State values, revision, remaining allowance, effects, and outstanding obligations are dynamic checked values. Continuation, redeployment, split, or join cannot reset a genesis lifetime or horizon.

A settlement binding has a nominal unit, ledger-asset text value, and positive quantum of that unit. It identifies a proposed asset mapping; it does not authenticate custody, movement, or ledger acceptance.

Every calculated financial state field or effect amount must resolve to exactly one `field` policy in the manifest. Its fields bind: result unit; derivation text; the named `FloorDiv` node or `none`; remainder disposition; comparison policy; and proof statement identifier. Elaboration rejects an absent, duplicate, stale, or unit-inconsistent policy. The proof public-input manifest includes the policy digest; this document supplies no proof implementation.

Effect schemas are closed. `Transfer` and `Fee` are proposed asset movements; `DueCreated` and `DueSettled` are obligation records. An obligation is not an asset transfer. An emitted transfer is not authenticated movement until an authorized wrapper validates counterparties, assets, balances, signatures, proof inputs, and ledger application. Unknown/missing effect fields or types reject before action execution.

## Validation order and diagnostics

Implementations must preserve this order, returning source-located diagnostics in deterministic `(startByte,endByte,code)` order within a stage:

1. Decode UTF-8, enforce the 65,536-byte source limit, and reject invalid scalar encoding.
2. Lex with longest operator match; reject forbidden comments, noncanonical integers, reserved/overlong identifiers, and malformed JSON strings.
3. Parse the complete file; no recovery may create an executable AST.
4. Enforce structural bounds and declaration uniqueness/closed schemas.
5. Resolve names and declaration-before-use.
6. Type expressions, units, assignments, effects, settlements, and policies.
7. Enforce lifecycle, horizon, action/effect, expression, and manifest representability bounds.
8. Elaborate deterministically and canonicalize the bound manifest; recompute its program hash.
9. At evaluation, validate profile/program/instance/state/action/authority/observation envelope schemas and canonical encodings.
10. Check genesis binding, current revision/remaining, authenticated `obs.now < horizon`, action existence, authority domain/principal/nonce/validity/predecessors/claims, and exact-plan or outcome-refinement binding.
11. Execute guards and instructions sequentially in a private working copy.
12. Validate all effects, obligations, gross debits including fees, net goals, permitted recipients/calls, writes, resource counts, and authority consumption.
13. Commit atomically by incrementing revision and decrementing remaining once, then produce the receipt. Any rejection exposes no partial state or effects.

A source span is a half-open pair `[startByte,endByte)` over the original UTF-8 byte sequence. The file begins at byte zero. Token spans include the token bytes but exclude inter-token whitespace. AST spans cover the first through last token of the construct. Synthetic elaboration nodes carry an ordered, nonempty list of originating spans plus a generated-node tag; they never claim a fabricated source position. Line/column display is derived only after byte spans are fixed.

## Evaluation relation

Evaluation inputs are `(manifest,state,action,authority,observations)`. State must match the manifest schema, program/instance/revision, and remaining allowance. Observations are declared typed inputs; `now` is mandatory, authenticated externally, and never read from the host clock. Authority is a tagged `ExactPlan` or `IntentRefinement` envelope. Historical `IntentEffects` signatures remain in their historical domain and cannot be relabeled.

Statements observe the working state in source order. `guard false` rejects. `let` defines once. `set` updates the working state. `emit` captures values at its position. On success, effect/policy/authority checks run against the entire trace; revision increases by one and remaining decreases by one. Revision overflow, exhausted allowance, expired horizon, invalid authority, or unrepresentable output rejects.

`Complete` means the selected atomic plan and its declared outcome completed. It does not mean all future contractual obligations are discharged. The receipt separately carries episode status, agreement status, outstanding obligations, and remaining notional/capacity. This profile does not execute partial plans and can never derive `Pending`; an action or wrapper request that requires residual progress rejects with `UNSUPPORTED_PENDING`. The API reserves `Pending` for a future semantic version whose progress envelope conserves residual authority, obligations, lifetime, and horizon.

## Frozen candidate limits

All limits in `bounds.json` are simultaneous. In particular: UInt128 operands and intermediates; 64 declarations/fields/instructions/locals; 16 effects and expression depth; 256 expression nodes per action; eight unit-vector components; 65,536-byte source and public envelope; source/AST decoded depth 40; actual signing-envelope decoded depth 16; 8,192 nodes; 64 keys per record; signing arrays 128; signing text both 4,096 UTF-8 bytes and 4,096 JavaScript code units. The signing envelope contains a program hash and bound manifest, not a nested source AST. Finite bounds do not establish proof fit or financial correctness.

The first profile has no user functions, recursion, loops, signed values, calendars, quantity-price conventions, bounded convergence, collections, split/join, or Pending. Each requires an explicit extension and, if meaning changes, a new semantic version.

## Version invalidation and correctness scope

Changing numeric width, evaluation order, rounding, unit meaning, codec, bounds, effect/obligation schema, authority/claim tags, lifecycle, horizon, canonicalization, or lowering relation creates a new semantic profile. It recomputes program IDs/hashes and invalidates affected signature domains, proofs, certificates, theorem instances, comparison vectors, and audits. Historical artifacts remain bound to their original bytes and predicates.

Parsing, typed checking, local evaluation, native proof acceptance, history compliance, source/ACTUS/DeFi conformance, compiler correspondence, and ledger acceptance are distinct predicates. The loan and swap files are specified-only examples. Their presence establishes none of those predicates.

## Host reconciliation before review

Effect record labels permit keyword spellings `asset` and `amount` through `field_label`. All other name restrictions remain. Hash preimages and the omitted self-hash fields are defined in typed-schemas.md. These corrections do not establish profile acceptance.
