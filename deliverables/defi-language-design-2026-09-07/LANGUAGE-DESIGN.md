# Proposed Moriarty surface and semantics

Status: S2, specified-only successor design. The user selected `.mori`, BNF-family syntax specification and K formal semantics. This document recommends EBNF with brace-delimited financial declarations. It does not change the accepted `moriarty-bounded-atomic/1` semantics. New constructs require a new versioned profile, grammar, canonical encoding and reviewed implementation.

## Specification layers

Use ISO/IEC 14977 EBNF notation for the syntactic grammar: `=` introduces a production, `,` concatenates, `|` selects alternatives, `[...]` is optional and `{...}` repeats. Quoted punctuation belongs to the source. [ISO's record](https://www.iso.org/standard/26153.html) identifies the standard; [RFC 5234](https://datatracker.ietf.org/doc/html/rfc5234) specifies ABNF, a different metalanguage. A regular-expression lexer must state its own dialect rather than embed unexplained regex operators in EBNF.

Separate five artifacts: lexical rules; complete EBNF; typing/scoping judgments; executable K Core semantics; named correctness and correspondence claims. Explanatory prose accompanies them. If K also parses the source surface, its grammar must agree with the published EBNF on accepted syntax and disambiguation. A K definition that accepts typed Core instead requires source-to-Core elaboration correspondence; its input grammar need not resemble the source grammar.

The current grammar file retains its original proposal-era header even though a parser now exists. It remains the experimental profile artifact, not evidence of a finalized normative grammar. A successor must correct the status and establish grammar/parser agreement explicitly; the historical file is not silently relabeled ISO-conformant.

## Surface choice

Use braced declarations, explicit financial keywords and infix arithmetic. Pure locals use `let`. Persistent changes use `next.field = expression;`. Guards use `requires`; postconditions use `ensures`. Effects use typed constructors within `emit`. This introduces no ambient mutation, implicit transfer, overloading by arbitrary user functions, pointers, unbounded loops, source recursion, runtime evaluation or executable macros.

S-expressions remain useful as an optional diagnostic rendering of Core, not a second supported source language. JSON remains an interchange/debug format and must still pass canonical schema checks. Neither diagnostic format is an alternate trusted compiler input.

The following is a proposed action fragment, not runnable code. Its enclosing agreement must declare the asset units, bound participants, custody, fee policy, supported claims and execution bounds.

```text
action swap(amount_in: Amount<A>, min_out: Amount<B>) {
  requires amount_in > amount(0, A);

  let adjusted = amount_in * fee_numerator;
  let numerator = adjusted * pre.reserve_b;
  let denominator = pre.reserve_a * fee_denominator + adjusted;
  let output = floor_div(numerator, denominator);

  requires output >= min_out;
  requires output < pre.reserve_b;

  next.reserve_a = pre.reserve_a + amount_in;
  next.reserve_b = pre.reserve_b - output;

  emit Transfer { asset: asset_a, from: trader, to: pool, amount: amount_in };
  emit Transfer { asset: asset_b, from: pool, to: trader, amount: output };

  ensures post.reserve_a == pre.reserve_a + amount_in;
  ensures post.reserve_b == pre.reserve_b - output;
}
```

`A` and `B` are nominal units. `asset_a` and `asset_b` are distinct typed asset identities supplied by settlement bindings that map each unit to its ledger asset and quantum. A unit name is not itself an asset identifier.

`pre` is immutable. `next` writes a tentative field exactly once; unwritten fields retain their pre-state value. All right-hand sides use pre-state, parameters, constants, observations or earlier immutable locals. Reads of `next` are forbidden. `post` is available only in final `ensures` clauses after the tentative state is assembled. Clauses execute in lexical order, and `ensures` clauses form a suffix; the first failed clause gives the deterministic diagnostic. There is no externally visible intermediate state.

This differs from the existing `set`/`state` profile, where later reads can see earlier writes. Migration must explicitly translate those reads to named locals or reconstructed post-state values. A file extension rename alone never opts into the proposed behavior.

## Lexical and grammar contract

Proposed lexical decisions: UTF-8 input; ASCII identifiers matching `[A-Za-z][A-Za-z0-9_]*` with registered length bounds; case-sensitive keywords; decimal nonnegative integer tokens without ambiguous leading zeros; JSON-compatible strings with Unicode scalar validation; ASCII token whitespace. Choose `//` line comments and non-nesting `/* ... */` comments for the successor, counted in source-byte limits and excluded from executable claims. Preserve exact source hashes separately from normalized semantic hashes. Comments do not prove a property or modify authorization.

No floating-point literals, implicit unit coercions or plain division. Named `floor_div` and `ceil_div` require positive denominators and a registered rounding policy; division-by-zero and out-of-range intermediates reject. The existing profile only implements its current division forms; `ceil_div` is a proposed extension motivated by share conversion targets.

This EBNF excerpt defines the action statement shapes. It is deliberately not a complete grammar: `identifier`, `type`, `expression`, `fieldname` and effectrecord productions are supplied by the successor's full lexical/expression/type grammar before admission.

```ebnf
action = "action", identifier, "(", [ parameters ], ")",
         "{", { statement }, { postcondition }, "}" ;
parameters = parameter, { ",", parameter } ;
parameter = identifier, ":", type ;
statement = requirement | binding | update | emission ;
requirement = "requires", expression, ";" ;
binding = "let", identifier, "=", expression, ";" ;
update = "next", ".", fieldname, "=", expression, ";" ;
emission = "emit", effectrecord, ";" ;
postcondition = "ensures", expression, ";" ;
```

Specify precedence from tightest to loosest: parentheses/calls/projections; multiplication; addition/subtraction; one non-chained comparison; `not`; `and`; `or`. Thus `not a == b` means `not (a == b)` in this proposal. This is another explicit change from the atomic profile. Expressions evaluate left to right, and arithmetic is not reassociated without an intermediate-bound preservation argument. `and`/`or` short-circuit left to right; the compiler still rejects programs whose conservative static bound exceeds the registered maximum. Parenthesized Boolean groups are encouraged by formatter hints.

Before freeze: write all productions, keyword/reserved-name rules, token priority, string decoding, expression precedence and diagnostic ordering. Generate positive and negative cases, including Unicode spans and nesting limits. Require formatter idempotence and preservation of normalized AST/Core, not equality of source hashes after whitespace edits.

## Static semantics

Use `Γ; Δ ⊢ e : τ ! ε` for expressions and `Γ; Δ ⊢ action ⇒ Δ' ! E` for actions. `Γ` contains ordinary bindings and scope; `Δ` contains owned resources/capabilities; `ε` and `E` describe effects. Pure expressions have empty effects. Typing is paired with a resource bound judgment and explicit visibility constraints.

Proposed type families:

- `Amount<Asset>` for nonnegative token quantities; `Quantity<Unit>` and bounded signed quantities for prices, cash-flow calculations and nominal obligations where required. Debt uses debtor/creditor orientation rather than an unexplained sign convention.
- `Rate`, `Price<A,B>`, `Shares<Vault>`, `Party`, `AssetId` and identified `Position`/`Obligation`/`Request` resources. Shares are distinct from underlying assets.
- Separate observation timestamps, payment dates, exercise windows and authorization expiry. Calendars, day-count and business-day policies are named financial definitions; ACTUS supplies their reference cases.
- Finite records, enums, options and collections with registered maximum sizes. Public/private annotations constrain disclosure, with explicit authorized declassification; commitments do not make private data publicly available.

For example, addition requires identical asset units:

```text
Γ ⊢ x : Amount<A>     Γ ⊢ y : Amount<A>
-------------------------------------
Γ ⊢ x + y : Amount<A>
```

Arithmetic range and overflow obligations remain additional judgments or runtime rejection conditions. Ownership prevents duplication/loss of identities but does not establish quantitative conservation. Mint/burn requires a defining authority and accounting rule. Unsupported types, observations, effects or claims reject before proof work.

## Operational semantics in K

Define the meaning of the typed Core, with source elaboration checked separately. [K's configuration/rewrite model](https://kframework.org/docs/user_manual/) is the selected executable framework. Begin with cells for control, immutable pre-state, tentative writes, locals, obligations, ordered effects, observations, authority, predecessor descriptors, remaining work and result status.

The semantic boundary is:

```text
Eval(profile, program, pre, action, observations, authority, predecessors)
  = Reject(code, span)
  | Prepared(post, effects, duties, residualAuthority, successors, workRemaining)
```

Rules evaluate finite expressions, validate requirements, stage each write/effect, assemble post-state and check postconditions. A reject result discards all tentative financial changes. The language's rejection rule cannot promise that an attempted network submission incurred no external fee.

`Prepared` records a candidate, not acceptance. No K test or host-computed success flag can replace native verification. The K model needs separate acceptance rules and an explicit ledger/environment interface for observations, consumption, ordering and finality.

Every admitted profile registers bounds on source size, value/intermediate arithmetic, expression depth, collections, schedule events, observations, writes, effects, duties, predecessor fan-in and accepted lifetime work. A finite well-founded measure must decrease under each evaluation rule or bounded expansion. Recursive proofs do not enable source recursion. Splits partition remaining work and authority; joins cannot replenish spent allowance. Rejected external submissions are outside the accepted-lifecycle count.

## Obligations and workflows

An obligation has an identity, parties, denomination, outstanding amount or bounded calculation, trigger/payment dates, allocation rule and status. A payment discharges only the amount specified by that rule. Changes to nominal principal, capitalized interest, fees and token transfers are separate effects. No cash movement does not mean no financial transition.

Pending requests retain identity, controller, committed assets/shares, claimable entitlement and residual amount. Their statuses are explicit; queue capacity and continuation work are bounded. Cancellation, default, write-off and loss allocation need named authority and effects.

Closure requires duties to be discharged, explicitly resolved under authority, or assigned exactly once to an identified successor that preserves them. Expiry and exhausted work cannot implement deletion. Reserve recovery work when admitting an agreement; if required future steps cannot fit, reject the profile or instance rather than promise unlimited continuation.

Composition has five distinct operators in the semantic design: sequencing, disjoint parallel composition, shared-state interleaving, atomic synchronization and asynchronous messaging. Exact spellings remain to be specified with examples. Each requires different conflict, custody, authority and residual rules. A generic `and` is only Boolean conjunction and cannot stand for financial composition.

Private succession must provide the next participant with usable witness material, accepted predecessor evidence, residual duties/authority/work and recovery information. A commitment is insufficient if the recipient cannot construct the next witness. MC06 must demonstrate this in an isolated participant environment.

## Mandatory proof acceptance

A deployment fixes allowed semantic versions, claim specifications and verifier/key lineage. Authorization has two modes. An exact plan commits to the selected execution payload. An outcome intent commits to constraints before the solver selects execution: network/deployment domain, permitted programs/profiles, mandatory claim root, principal, nonce, expiry, observation policy, gross spending, net delivery, recipients/calls/fees, liability authority and partial-fill/residual rules. It need not fix a concrete next state, route or complete effect list. Predecessor constraints may pin a state or describe an admitted context; their interpretation is explicit in the profile.

The concrete execution statement binds the signed authorization digest to the selected program/profile, predecessor identities, actual observations, next state, complete effects, duties, successors and remaining authority/work. IntentRefinement proves that this concrete execution satisfies the signed constraints. It must not require a second exact-plan signature in outcome mode. Exact-plan mode additionally checks equality with its signed execution commitment.

Use a versioned non-circular construction: canonical unsigned authorization payload, authorization digest/signature, then concrete execution statement referencing that digest, then proof. For exact-plan mode, the authorization payload carries the hash of an execution body that excludes the authorization digest, signature and proof; the later statement binds both. No proof bytes or self-referential statement hash enter the payload they authenticate.

Acceptance requires canonical/bounded inputs, authenticated current authorization, admissible observations, unique current predecessor consumption, native verification of the exact statement and exact binding to ledger effects. Four claims remain mandatory: ContractInvariant, IntentRefinement, TransitionValidity and HistoryCompliance. Genesis and administrative transitions need explicit cases. No source option disables them; unsupported mandatory claims fail closed.

History compliance does not prove oracle truth, prevent global double-spending by itself or establish timely settlement. The ledger and observation interface have those separate responsibilities. Proof-system soundness, key trust and private witness availability remain stated assumptions or independently tested obligations.

For partial fills, acceptance persists consumption by authorization identity (network/deployment/principal/nonce): cumulative gross debits, fees, net delivered amounts, outstanding liabilities, residual rights and remaining work. Every successor proves an update of that record; refunds cannot restore consumed gross allowance. Atomic ledger consumption prevents concurrent fills from spending the same residual authority. Reloading or changing a solver does not reset the record. MC04/MC05 must implement and test these cumulative rules before accepting multi-transaction outcome intents.

K work must establish named claims for bounded evaluation, type/state preservation, failure atomicity, quantitative accounting, residual-duty preservation, authority non-amplification and work conservation. Correspondence compares all observable obligations, fees, resource identities, locks, messages and outputs across source/Core, K, the TypeScript evaluator, Compact and Midnight acceptance. Matching only final balances is insufficient.

## Decision and next implementation slice

Proceed with this surface as the working design, subject to matched developer tasks and independent review. Implement the complete grammar/lexical/static contract and a narrow K Core slice for checked arithmetic, a guard, a staged update and a transfer candidate. Then add a partial-payment case with a surviving obligation before extending the proof relation. Use ACTUS and the action matrix to choose each extension.

This prevents a formalization campaign from outrunning the financial target study. The next deliverable must show source, typed Core, K trace, evaluator result and an independent expected financial result for the same case. Native proving and ledger settlement remain separate admission milestones.
