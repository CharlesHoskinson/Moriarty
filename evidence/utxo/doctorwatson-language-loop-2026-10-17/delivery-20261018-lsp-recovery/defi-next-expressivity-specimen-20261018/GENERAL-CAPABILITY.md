# Proposed UInt floor division for source-computed financial amounts

Status: source-only proposal, not adopted, implemented, parsed or executed. Base: main `74a909d69639df8823a527a4d608a7516ccd69a2`. Root selects any later implementation after actual source review. The placeholder selector `moriarty/proposed-uint-floor-div/1` deliberately names an unsupported proposed profile; it allocates no real version.

The proposed capability is `floor_div(numerator: UInt, denominator: UInt) -> UInt`. It computes a quotient from source expressions. It is general arithmetic usable in exchange, pro-rata distribution, share conversion, bounded payment allocation and other developer programs. No agreement, action, protocol, asset or account name selects compiler behavior.

Repository observations are pinned in `SOURCE-MANIFEST.json`. At the base, Core/1 has UInt128, Bool, checked `+ - *`, ordinary state, transfers, fees and liabilities. The public source parser recognizes liability-read calls and otherwise parses ordinary names; its operator table has no division. The runtime's binary expression cases also have no division. The README explicitly says division is unimplemented. Existing Core/2 reserves its own staged profile and is not a place to silently insert this proposal. The older beta AMM example and legacy vault `floor_div` specimen use separate profiles; their syntax does not establish current Rust support.

The direct source-computation gap is smaller than general AMM implementation. A caller-supplied quotient can already be checked by bounded multiplication/comparison guards in suitable domains. Therefore this proposal does not claim that the relation is mathematically inexpressible in existing Core. It removes a candidate-result input/API burden and lets the action derive its own amount. It does not claim to close all DA01 or any original protocol row.

| Approach | General change | Consequence |
| --- | --- | --- |
| Proposed `floor_div(UInt, UInt)` | One typed expression node and matching versioned parser, reference semantics and work rule | Smallest direct computation change for the bounded specimen; checked UInt128 intermediate overflow remains explicit |
| Keep quotient as an action input with source verification inequalities | No new operation for suitably bounded examples | Caller proposes the quotient; complete domain/overflow conditions still need explicit source treatment; this does not deliver the requested source-computed action |
| Add wide `mul_div` and multiple rounding modes | Wider arithmetic representation and more semantic rules | More financial range; larger change and independently reviewable numeric/cost/backend work |

This recommendation selects the first row as a proposed next increment. The other numeric domains, wider products, fee models, iterative invariants and financial families remain open.

## Proposed syntax, typing and evaluation

Proposed EBNF addition, confined to the new profile:

```ebnf
floor_div_expression = "floor_div", "(", expression, ",", expression, ")" ;
```

`floor_div` followed by `(` recognizes exactly two arguments. A bare identifier with that spelling remains an ordinary name under the chosen profile's existing name rules. Dynamic calls, implicit FFI, optional arguments, defaults and automatic numeric widening are not introduced. Both arguments type as UInt; the result types as UInt. Arity and Bool-operand errors are deterministic parser/type diagnostics. The new expression consumes one arena node; its two children allocate left-to-right before the parent. Existing source/node/depth limits continue to apply under the selected profile.

The numerical definition is unsigned Euclidean floor: for evaluated numerator `n` and denominator `d > 0`, return the unique `q` satisfying `n = q*d + r` and `0 <= r < d` over mathematical integers. The result fits UInt128 because `q <= n`. Evaluate the numerator first and denominator second, propagating the first existing expression refusal. A zero denominator returns proposed error code `division_by_zero` at the containing action statement, with no committed execution/post/effects. There is no implicit rounding provenance, signed division, reciprocal approximation or Float32 conversion.

Every enclosing multiplication, addition and subtraction retains checked UInt128 behavior. The primitive is not a fused wide multiply/divide. For example `floor_div(a*b,d)` refuses if `a*b` overflows, even when the mathematical final quotient would fit. This is an explicit numeric-domain boundary to requalify, not permission to omit larger-domain financial obligations.

## Proposed public API and consumer integration

The reference API remains the public source pipeline. A proposed profile-aware source loader returns a versioned Program; versioned validation checks its expression types; simulation/replay visit the new expression; metered simulation reports its attempted work. No public `swap`, `Uniswap` or fixture lookup API is added.

Proposed reference-operation signature for implementation planning only:

```text
floor_div_uint(n: UInt, d: UInt) -> Result<UInt, Error>
```

This is a proposed general helper consumed by the expression evaluator. Adding the helper alone is insufficient. The source parser, public IR validation, ordinary and metered simulation, replay comparison, source selection, program serialization and diagnostic paths must all expose consistent versioned behavior. A new profile remains explicitly unsupported in any backend or staged consumer until that consumer is actually extended and independently qualified.

The pinned `ir::v1` wire reuses the shared Expr type. A new shared enum variant must not accidentally broaden frozen Core/1 decoding. Implementation must isolate the new wire/profile or retain explicit old-profile constructor refusal, with the applicable compatibility design reviewed first. Do not mutate historical Core/1 or Core/2 meanings, hashes or logical-cost identities under the same label. The exact allocated version and source/API migration are a root design choice still open.

Relevant existing source consumers at the pin:

- `crates/mori-core/src/parser.rs`: explicit language routing, expression grammar, statement grammar.
- `crates/mori-core/src/ir.rs`: expression arena, versioned wire, Step and Execution structures.
- `crates/mori-core/src/runtime.rs`: expression typing, validated interpretation and first refusal, complete snapshot/effects.
- `crates/mori-core/src/logical_cost.rs`: metered interpretation and replay wrappers.
- Existing downstream compiler, Sherlock/K/Quint/Lean-Clean and ledger consumers: separate correspondence obligations, not inspected or implemented by this proposal.

## Proposed logical-work rule

The examples propose a new logical-cost identity `moriarty-proposed-reference-logical-cost-floor-div/1`. It carries the existing reference convention of one unit per attempted statement and one per uncached expression node in that statement, with fresh per-statement cache and eager left-to-right child evaluation. The new division node costs one logical unit before its children/operation, including a later zero-divisor refusal. No work is inferred from host speed.

The source has 23 exact-input statements and 22 exact-output statements. Independently authored node-count schedules in `SEMANTIC-EXAMPLES.json` predict 83 and 85 logical units respectively. These are proposed author-derived semantic numbers, not an adopted schedule, observed result or independently reviewed oracle. Any selected implementation needs a separately frozen expectation set and applicable source/result review before comparisons are scored.

Parsing, normalization, snapshot allocation, wall time, DUST, service fees, circuit constraints, target execution costs and recovery/lifetime reserves remain distinct. The arithmetic increment adds no lifetime authority, closure-reserve policy or financial acceptance rule.

## Complete specimen and semantic scope

`constant-product-swap.proposed.mori` is a complete two-action source specimen under the proposed profile. Both actions use arbitrary input quantities and current reserve state, static account/asset bindings, checked intermediate arithmetic, positive reserve guards, an explicit 997/1000 pool-retained fee weight, complete two-leg transfer effects, reserve/revision updates and exact amount events. It introduces no host-calculated amount input, mint or burn.

The exact-input formula is:

```text
out = floor(d*997*y / (x*1000 + d*997))
```

The exact-output formula, for `0 < t < y` and positive numerator, is:

```text
in = floor((x*t*1000 - 1) / ((y-t)*997)) + 1
```

The second formula derives ceiling from the same floor primitive without the potentially overflowing `numerator + denominator - 1` idiom. Positive-reserve and desired-output guards occur first. Actual checked intermediate overflow remains a refusal.

The fee weight is part of the explicitly selected source formula. All input A goes into the pool; there is no separate source Fee or treasury transfer. For the displayed `d=10000`, the embedded input fee is exactly 30 A. For other inputs the fee weight may represent a rational amount; this specimen does not invent an additional rounded token fee. Other fee destinations/models require separate source behavior.

Every displayed example supplies all state fields, exact asset identities, every account/asset balance, a residual duty14 and a present-zero duty. Unchanged spectator/treasury balances, marker, paused flag and duties are retained. The first successful result becomes the independently specified input of the second example, so repeating the original constant output would disagree with the proposed meaning.

Initial reserve-state equality with pool balances is an explicit example premise. The displayed source maintains it along the shown successful lineage, but source simulation does not authenticate arbitrary initial snapshots. There is no implemented balance-read invariant over all caller-provided states. The existing complete-frame, target state projection, authorization, signed intent and ledger acceptance obligations remain separate and unresolved; their adequacy belongs to the designated owners.

## Preserved acceptance and implementation boundary

The chosen lineage is DA01 and a constant-product child of retained row1. Row1 also requires concentrated positions, hooks, singleton net settlement, fee checkpoints and burn/distribution distinctions. A passing future constant-product child would leave those original obligations open. The exact-output source action does not close the retained exact-output signed-intent case. No general stable-swap, concentrated-liquidity, iteration, liquidity-share issuance, async request, liquidation or protocol compatibility claim follows.

`PRESERVED-SCOPE.json` retains all 72 full original rows, all DA01–DA24 original requirements, all three heldouts, eight intent cases, eight regression classes, five composition operators and twelve additive product source obligations. The four exact pinned input copies preserve original normative/inventory bytes. ACTUS 277 fixtures/18 types/32 dispositions/DS01–07 and all U/M/MC/SP/G, mandatory proof, private, recursion and target exits retain their original scope.

Next possible root decision is an actual source review of this complete proposal and its numeric/version/work contract. This packet itself requests no dispatch and grants no execution. No runtime, compiler, model, SDK, tests, proof, provider, wallet, network or ledger action was performed. No shared repository or Git mutation was made. Security and proof judgments are explicitly outside this author's assignment.
