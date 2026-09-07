# Designing an Intents-First Language for Composable DeFi

## Executive design decision

The strongest architecture is **not** a general-purpose “intent language” in which arbitrary predicates or callbacks are signed and handed to solvers. It is also not a larger vocabulary of DeFi mechanisms. The most defensible design is a **small typed language for authorization-bounded financial outcomes**, compiled into a canonical semantic IR, with solver plans represented separately and settlement adapters required to prove—or directly check—that a plan refines the signed intent.

I will use the working name **IKL, Intent Kernel Language**, for the design in this report.

The key architectural separation is:

\[
\boxed{
\text{Intent}
\neq
\text{Permission}
\neq
\text{Plan}
\neq
\text{Execution}
\neq
\text{Receipt}
}
\]

A user's desired outcome does **not** imply authority to take every route that could achieve it. The core object should therefore bind both:

\[
I =
\langle
\text{subject},
\text{scope},
\text{authority},
\text{safety},
\text{goal},
\text{lifecycle},
\text{assumptions},
\text{validity}
\rangle
\]

while solver preferences are kept logically separate:

\[
Q = \langle hash(I),\ preferences,\ disclosure\_policy\rangle .
\]

The crucial property is that a solver can choose a plan \(P\), but the verifier accepts \(P\) only when every authority-consuming effect is within the signed capability envelope and the resulting trace satisfies the required safety and outcome semantics.

That design is suggested by—but should not be conflated with—several existing systems. NEAR Intents demonstrates atomic matching over internal balances and signed actions in a production-oriented verifier. Its `token_diff` construct expresses a user's accepted changes to internal balances, and matched token differences must balance per token in a batch. NEAR's own documentation also explicitly warns that cross-contract effects are asynchronous and can complete out of the order in which intents were initiated. citeturn12search0turn12search3

CAKE supplies a useful systems decomposition rather than a language: Applications, Permissions, Solving, and Settlement. Frontier defines an intent as an expected result rather than the execution path, while describing the solver layer as responsible for estimating execution and the settlement layer as responsible for carrying it out. citeturn12search1

The current ERC-7683 draft is even more important for drawing the boundary correctly. It explicitly describes itself as a **solver-facing interface**. Protocol-specific order payloads are resolved into steps, variables, payments, and assumptions for solvers; it intentionally leaves user-order creation, fund authorization, and protocol settlement design open. It should therefore influence IKL's **Plan IR/backend interface**, not become the source language that users sign. citeturn20view3

The core recommendation is consequently a hybrid of three of the alternatives in the assignment:

| Architecture | Strength | Fundamental weakness | Disposition |
|---|---|---|---|
| Typed outcome/constraint DSL | Clear user meaning; easy wallet presentation | Outcomes alone do not bound authority; weak async semantics | **Use as surface language** |
| Resource-and-obligation calculus | Excellent for spend budgets, partial fills, conservation and composition | Too low-level as the user syntax | **Use as semantic kernel** |
| Temporal contract/workflow language | Correct model for delayed withdrawals and cross-domain operations | Too heavy for every one-shot swap | **Use as an explicit lifecycle tier** |
| General-purpose host language with validators | Maximum expressiveness | Hides semantic power in callbacks; poor portability and solver predictability | **Restrict to typed adapters/FFI** |

This is not merely an aesthetic choice. The Anoma Resource Machine independently demonstrates why create-once/consume-once resources are useful for intent composition: its state is a set of active immutable resources, users may submit unbalanced transactions representing preferences, and solvers construct balanced transactions whose resource logic is verified. citeturn19search0 Essential's Pint supplies complementary prior art for a declarative, predicate-and-constraint programming model. citeturn19search1turn19search4 Marlowe supplies a useful warning that financial semantics often require explicit time and lifecycle constructs rather than a single terminal-state predicate. citeturn19search2

The proposed novelty is therefore **not** a new invention of constraints, resources, capabilities, or temporal contracts. Those ideas all have substantial prior art. The potentially original contribution is the semantic firewall among:

\[
\text{signed financial meaning}
\rightarrow
\text{bounded capabilities}
\rightarrow
\text{solver plan}
\rightarrow
\text{backend refinement}
\rightarrow
\text{settlement evidence}.
\]

That separation also resolves an important issue in the supplied DeFiFormal work. The repository describes itself as an algebra of 58 mechanisms drawn from 72 protocols and acknowledges that the vocabulary does not fully express any of the 72 protocols (`README.md:3–8, 27–33`). Its later proposed four-primitive basis `Led/Prop/Cmp/Post` depended on a \(Q/\Sigma\) distinction (`research/positive-program/basis/BASIS.md:1–5, 25–34`), but the repository's own later audit explicitly says the invariant is false and that the \(|P|=4\) claim should be withdrawn (`research/positive-program/sigma/QSIGMA-VERDICT.md:1–8`). This is exactly why **DeFi mechanism classification should remain a library and test-corpus concern rather than determine the trusted language kernel**.

The right layering is:

\[
\boxed{
\begin{array}{c}
\text{DeFi taxonomy and mechanism library}\\
\downarrow\\
\text{IKL financial source language}\\
\downarrow\\
\text{canonical Intent IR}\\
\downarrow\\
\text{solver query} \quad\rightarrow\quad \text{solver Plan IR}\\
\downarrow\\
\text{typed settlement adapter}\\
\downarrow\\
\text{small verification kernel}
\end{array}
}
\]

The DeFiFormal atlas remains useful as a coverage benchmark and source of financial behaviors. It should not be the language's primitive basis.

## Evidence from NEAR, CAKE, Ethereum standards, and prior intent systems

The most important research result is that the major intent-related systems solve **different layers of the problem**. Treating all of them as competing “intent languages” would lead to a badly confused design.

### What NEAR Intents actually contributes

NEAR Intents' primary smart contract is the Verifier under `contracts/defuse`; the repository states that the Verifier can be used independently and facilitates atomic peer-to-peer transactions between deposited balances. citeturn16view0 The user's signed payload carries identity, a deadline, nonce information, and one or more intents. citeturn12search3

The clearest primitive is `token_diff`. Conceptually, a participant signs a vector of changes to their internal balances:

\[
\Delta_u : Asset \rightarrow \mathbb Z.
\]

For a matched batch to succeed, the aggregate internal change for each asset must balance:

\[
\forall a,\quad \sum_u \Delta_u(a)=0.
\]

NEAR's documented simulation example shows Charlie accepting \(-100\) USDC and \(+100\) USDT while Drake accepts the inverse, with both payloads submitted together. `simulate_intents` runs the same `MultiPayload` without changing state and returns validation information, executed intent hashes, logs, deadline information and fee information. citeturn12search0

This is a remarkably useful model for a **settlement primitive** because it reduces an exchange to account-indexed asset deltas. But it is not by itself a general intent language.

There are three reasons.

First, the atomicity is local to the Verifier's state transition. NEAR documentation says ordered intents can initiate cross-contract calls whose completions are not guaranteed to occur in the same order because NEAR execution is asynchronous and sharded. citeturn12search3

Second, assets deposited into the Verifier become balances represented inside that contract. Withdrawals then invoke external token operations. NEAR's withdrawal documentation distinguishes ordinary transfers from `transfer_call` behavior and documents a particularly important failure boundary: for fungible-token withdrawals, omitting the `msg` field can allow refund handling, while a specified `msg` uses `ft_transfer_call` and does not receive the same refund protection if the asynchronous target call fails. citeturn12search6 This is a concrete example of why:

\[
\text{local atomic settlement}
\not\Rightarrow
\text{atomic end-to-end workflow}.
\]

Third, there is observable documentation drift around nonce representation. The general Intent Types page describes a 256-bit nonce as a four-byte salt plus 28 bytes of unique data. citeturn12search3 The current Verifier repository README describes both legacy and versioned nonce formats and a versioned layout containing a prefix, version, salt, deadline and random data, with legacy nonces expected to be phased out. citeturn16view1 For language design, that means the source semantics should say **“fresh replay capability with validity domain”**, while the precise wire-level nonce scheme belongs in the backend.

NEAR should therefore contribute four ideas to IKL:

**Internal resource-delta settlement.** This is an excellent low-level observable.

**Signed action sets with explicit validity.** Deadlines and replay protection must be first-class.

**Simulation as developer tooling.** Simulation is valuable but must remain explicitly weaker than execution guarantees.

**A hard boundary between synchronous internal accounting and asynchronous external effects.** IKL should make this boundary visible in the type/lifecycle system rather than blur it.

It should *not* contribute the assumption that a quoted `token_diff` is the universal form of an intent.

### What CAKE contributes

CAKE is best viewed as a reference architecture. Frontier's original framework identifies Applications, Permissions, Solving, and Settlement, and explicitly distinguishes the user's expected output from the route used to achieve it. citeturn12search1

IKL maps naturally onto CAKE:

| CAKE concern | IKL object |
|---|---|
| Application | Source program, financial libraries, human-facing rendering |
| Permission | Signed Intent IR and capability envelope |
| Solving | Solver Query + solver-created Plan IR |
| Settlement | Backend adapter, verifier, receipts, residual obligations |

The useful refinement is that **authority, assumptions, privacy, and recovery are cross-cutting semantics**, not a single CAKE layer.

For example, a user's permission might allow at most 1,000 USDC to leave an account, but a cross-chain settlement can lock that authorization for several minutes. The permission layer defines the cap; the settlement workflow defines whether it is *available*, *locked*, *spent*, or *refundable*.

Frontier also distinguishes information transfer from value transfer and frames cross-chain execution in terms of trade-offs involving execution guarantee, fees, and speed. citeturn12search1 The later OneBalance work introduces resource locks as commitments that reserve user state while a solver fulfills an external request. citeturn12search4 This is strongly compatible with IKL's resource/obligation model.

However, CAKE's “trilemma” should not be encoded as a theorem of the language. The source presents it as a design trade-off and organizes candidate architectures around it; it does not establish a formal impossibility result over a precisely defined distributed-computing model. citeturn12search1 The language should instead expose relevant assumptions: bounded finality, solver collateral, escrow, liquidity guarantees, messaging guarantees, and timeout/recovery behavior.

### What the Ethereum standards actually standardize

The standards are most useful when grouped by semantic boundary.

| Standard | Current role | Language lesson | IKL placement |
|---|---|---|---|
| ERC-7521 | Draft generalized smart-wallet intent framework with extensible `bytes[]` intent segments and solver-built `IntentSolution`s. citeturn13search0 | Extensibility matters, but opaque arbitrary segments are too weak a universal semantic core | Historical/reference adapter |
| ERC-7683 | Draft solver-facing cross-chain order resolution into steps, variables, payments and assumptions. citeturn20view3 | Excellent inspiration for Plan IR; wrong level for source intent | Solver/backend adapter |
| ERC-4337 | Final account-abstraction system built around `UserOperation`, account-defined validation and bundlers. citeturn15view0 | Execution/validation transport, not financial intent semantics | EVM execution backend |
| EIP-7702 | Lets EOAs install delegated code and is motivated by batching, sponsorship and privilege de-escalation. citeturn14search1 | Powerful account substrate; authority must still be separately constrained | EVM account backend |
| ERC-7579 | Draft minimal modular-account interfaces for validators, executors and other modules. citeturn20view1 | Natural place for an IKL enforcement module | EVM adapter |
| ERC-7710 | Draft capability delegation between contracts or EOAs. citeturn13search1 | Delegation is distinct from outcome specification | Permission adapter |
| ERC-7715 | Draft wallet RPC for requesting execution permissions and rules. citeturn13search2 | Strong evidence for typed wallet permissions; widening authority must never happen silently | Wallet permission adapter |
| ERC-7821 | Draft minimal atomic batch executor for delegations. citeturn14search3 | Batching is execution semantics, not intent semantics | Execution adapter |
| EIP-5792 | Final Wallet Call API supporting batches, capability negotiation and requested atomicity. citeturn20view0 | Useful backend capability discovery | Wallet/backend adapter |
| EIP-712 | Final typed structured-data hashing/signing; explicitly excludes replay protection. citeturn18view0 | Useful EVM serialization, insufficient semantic authorization by itself | Signing codec |
| ERC-1271 | Final contract-signature validation method. citeturn18view1 | Signer verification can be account-specific | Signing adapter |
| ERC-6492 | Signature validation for predeploy contract accounts. citeturn18view2 | Account deployment state should not change intent meaning | Signing adapter |
| ERC-7730 | Draft clear-signing format for structured contract calls/messages. citeturn13search3 | Wallet display needs semantic metadata | Rendering adapter |
| ERC-7540 | Final asynchronous extension to ERC-4626 with Pending → Claimable → Claimed request lifecycle. citeturn20view2 | Strong regression test for temporal intents | Financial adapter |
| ERC-7930 | Chain-qualified binary interoperable addresses. citeturn21search2 | Domain-qualified identity is necessary but does not imply asset equivalence or bridge safety | Address codec |

Two standards deserve particular emphasis.

ERC-7521 makes the user's `UserIntent` a wallet address plus arbitrary intent-standard byte segments and a signature; solvers can combine intents into an `IntentSolution`. citeturn13search0 This is extensible, but from a language-theory perspective it delegates semantics to whatever external intent standards those bytes invoke. That is analogous to saying a programming language's only expression is “call arbitrary plugin.” It is an interoperability envelope, not a small semantic calculus.

The current ERC-7683 draft has moved in a different and, for IKL, more useful direction. It says an order is an offer of payment for satisfying requirements; a resolver translates a protocol-specific payload into general steps, variables, payments, and named assumptions. citeturn20view3 Its `Call` steps can have dependency relationships and attributes such as `SpendsERC20`, while resolvers may surface assumptions they cannot validate themselves. citeturn20view3 This belongs downstream of the user's signed semantic contract.

The live ERC-7683 page also retains an explicit “Previous Draft” section in its revision history structure. citeturn20view3 Implementations therefore need to pin an exact draft/version rather than use “ERC-7683” as though every historical order struct meant the same thing.

ERC-7715 reveals another important semantic hazard. Its permissions are intentionally extensible, and its current text lets wallets modify a requested permission when adjustment is allowed. citeturn13search2 IKL should impose a stronger rule:

\[
Authority_{granted} \subseteq Authority_{requested}
\]

may happen without changing the user's risk, but:

\[
Authority_{granted} \supset Authority_{requested}
\]

must **always** require new explicit authorization. “Adjustment” must mean attenuation for a security-sensitive language.

EIP-712 reinforces the distinction. It standardizes deterministic typed signing and domain separation but explicitly says it does not include replay protection. citeturn18view0 IKL must therefore bind replay semantics, lifecycle version, adapter version, and authority scope as part of its own signed meaning rather than assuming typed signing solves authorization.

### Lessons from CoW, UniswapX, Anoma, Essential, and Marlowe

CoW Protocol demonstrates the value of keeping the user's trade intent less prescriptive than an execution route and using solvers/batch auctions to find execution. Its current documentation describes solver-based intent execution and combinatorial batch-auction price finding. citeturn21search3turn21search14 It is useful prior art for matching and competition, but its core domain remains predominantly trading.

UniswapX is an even cleaner architectural precedent. Swappers create signed orders, fillers compete using arbitrary fill strategies, and order-specific Reactors validate, resolve, execute, and verify fulfillment. citeturn21search1 This strongly supports a **Reactors-as-adapters** model: IKL should not encode individual protocols into the language kernel; instead, typed adapters should expose verifiable effects and refinement contracts.

Anoma goes deeper by making resources the state primitive and attaching proofs for balance, compliance and resource logic. citeturn19search0 IKL should borrow affine/linear **authorization accounting**, but not automatically inherit arbitrary user predicates. An arbitrary predicate can encode arbitrary computation and move the supposedly small language kernel into a hidden validator.

Essential Pint takes the declarative direction further: contracts consist of typed predicates and constraints. citeturn19search1turn19search4 Its lesson is that declarative constraints are workable as a source model, but the language must still distinguish constraints used to describe acceptable outcomes from the authority needed to realize those outcomes.

Marlowe and Cardano's time model show why asynchronous finance cannot be an afterthought. Marlowe is a financial DSL in which timeouts and deadlines control contract evolution, while Cardano's transaction validity intervals make time an explicit deterministic input. citeturn19search2 IKL needs a temporal tier for withdrawal queues, maturity, cross-domain settlement, recurring authority, cancellation and recovery.

The synthesis is therefore:

> **Use outcome constraints for expression, affine resources for authority, temporal state machines for persistence, and typed adapters for imperative execution.**

## The recommended language and semantic kernel

The central design principle is to keep the surface language expressive enough for finance while keeping the verification kernel much smaller than a general-purpose smart-contract VM.

### Semantic objects

IKL should define the following terms precisely.

**Goal.** A hard statement about an acceptable outcome, such as “Alice receives at least 0.48 WETH.”

**Safety constraint.** A condition that must hold at a specified observation phase, possibly every observable prefix, such as “collateralization never drops below the agreed threshold during an atomic refinance.”

**Authority.** An upper bound on actions the execution system may take from the user's resources, such as “debit at most 1,000 USDC and only transfer it to one of these settlement contracts.”

**Assumption.** A proposition the verifier cannot establish from local state and therefore requires declared evidence, such as an oracle price with source and freshness.

**Preference.** A ranking among already valid solutions. A preference may influence solver selection but never make a safety violation acceptable.

**Plan.** A solver-selected concrete execution strategy.

**Commitment.** A stateful reservation that prevents the same authority/resource from being reused concurrently.

**Obligation.** A claim that remains after an asynchronous step and must later become fulfilled, refundable, cancelled, or failed.

**Receipt.** Evidence of an execution or lifecycle transition, tied to the intent hash and relevant state anchors.

These are not synonyms. Most current intent systems expose subsets of them; IKL's purpose is to preserve the distinctions across compilation.

### Core types

A minimal type universe should be substantially more precise than Solidity-like `address` and `uint256`:

```text
Domain
Principal<D>
Asset<D, Issuer, AssetRef, ClaimKind>
Amount<A>
Rate<A, B>
Price<A, Numeraire>
Time<D>
Duration
Capability<E>
Obligation<K>
Observation<T, Source>
StateRef<D>
SettlementState
Bool
Nat
Int
```

An `Asset` is not identified only by its ticker.

For example:

```text
USDC<Ethereum, Circle, 0x..., RedeemableToken>
USDC<Base,     Circle, 0x..., RedeemableToken>
USDC<Near,     BridgeX, nep141:..., BridgedClaim>
```

are distinct values unless a separate adapter explicitly establishes a conversion or equivalence relationship.

ERC-7930 is useful for unambiguously representing target addresses together with their chains, but its job is addressing, not proving that assets on two domains are economically identical. citeturn21search2

Quantities are integers in asset-native smallest units. Floating-point arithmetic should not exist in the semantic IR. Conversions should make rounding explicit:

```text
mul_div_down(x, n, d)
mul_div_up(x, n, d)
```

A rate carries its units:

```text
Rate<USDC, WETH>
```

so this is a type error:

```text
10 USDC + 2 WETH
```

and so is:

```text
Price<ETH, USD> + Duration
```

This preserves the useful insight behind DeFiFormal's attempt to distinguish financially meaningful quantities without requiring the failed \(Q/\Sigma\) primitive partition in `BASIS.md`.

### Surface syntax

A representative grammar can remain small:

```ebnf
module          ::= "module" QName "version" Version "{"
                      declaration*
                    "}"

declaration     ::= asset_decl
                  | account_decl
                  | policy_decl
                  | intent_decl
                  | workflow_decl

intent_decl     ::= "intent" Ident "(" parameter* ")" "{"
                      validity?
                      authority*
                      requirement*
                      assumption*
                      preference*
                      lifecycle_ref?
                    "}"

validity        ::= "valid" temporal_range ";"

authority       ::= "authorize" capability_expr ";"

requirement     ::= "require" phase ":" formula ";"

assumption      ::= "assume" observation_decl
                    "satisfies" formula
                    "fresh_for" duration ";"

preference      ::= "prefer" objective
                    ("then" objective)* ";"

phase           ::= "always"
                  | "before(" Event ")"
                  | "after(" Event ")"
                  | "at(" State ")"
                  | "terminal"

workflow_decl   ::= "workflow" Ident "{"
                      state_decl+
                    "}"

state_decl      ::= "state" Ident "{"
                      transition*
                    "}"

transition      ::= "on" Event
                    ("when" formula)?
                    "->" Ident
                    ("produces" obligation_expr)?
                    ";"
```

The most important statement classes are therefore not `swap`, `lend`, `bridge`, `stake`, or `oracle`. Those belong in libraries. They are:

```text
authorize
require
assume
prefer
lifecycle
```

This is a much more defensible kernel than making DeFi market categories language primitives.

### A basic exchange

```text
intent BuyWETH {
    valid now .. 2026-09-06T21:00:00Z;

    authorize spend(
        asset  = Ethereum.USDC,
        max    = 1_000 USDC,
        from   = alice,
        to     = SettlementWhitelist
    );

    require terminal:
        balance(alice, Ethereum.WETH)
          - initial_balance(alice, Ethereum.WETH)
        >= 0.48 WETH;

    require always:
        debit(alice, Ethereum.USDC) <= 1_000 USDC;

    prefer maximize received(Ethereum.WETH)
      then minimize total_fees;
}
```

The source deliberately does **not** say Uniswap, Curve, CoW, an RFQ market maker, or a route. Those are solver choices unless the user constrains them.

A route restriction is authority/policy:

```text
authorize calls only_to { UniswapXReactor, CoWSettlement };
```

not a preference.

### Hard conditions and soft ranking

IKL must make this distinction impossible to miss:

\[
Valid(P,I)
=
Authorized(P,I)
\land
Safe(P,I)
\land
Goal(P,I).
\]

Only after validity is established do preferences apply:

\[
P^*
=
\arg\max_{P\in Valid(I)} Preference(P).
\]

A solver may return “valid but not proven optimal.” That is acceptable.

It must not return:

> “I violated your minimum amount because the route had lower latency.”

That is a category error: minimum output is a hard condition; latency may be a preference.

IKL v1 should support lexicographic preferences:

```text
prefer maximize receive
then minimize fees
then minimize settlement_latency;
```

and defer general weighted utility functions. Weighted utilities often hide unit normalization and produce unintuitive trade-offs.

### Capabilities as affine resources

Authority should be consumed, not treated as an indefinitely reusable Boolean permission.

For a spending capability:

\[
Cap(A,u,B,R)
\]

where \(A\) is the asset, \(u\) the owner, \(B\) the remaining budget and \(R\) the recipient policy, execution of debit \(d\) yields:

\[
Cap(A,u,B,R)
\xrightarrow{d}
Cap(A,u,B-d,R)
\]

provided:

\[
0\le d\le B.
\]

This gives partial fills a precise semantics.

Suppose the user authorizes 1,000 USDC. A solver fills 300 USDC:

\[
B_1=1000-300=700.
\]

A later solver can fill at most 700, not another 1,000.

This sounds obvious, but it is exactly the property that becomes difficult when multiple independent solvers hold copies of the same signed request. Static types alone cannot prevent two domains from consuming the same 700-unit residual concurrently. The authority's **home settlement system** must provide a linearized consume/lock operation, UTXO-like resource consumption, or equivalent credible commitment. Frontier's resource-lock design is motivated by precisely the problem of preventing equivocation/double-spending while a solver acts on a user's request. citeturn12search4

### Temporal intents and obligations

A one-shot terminal predicate is insufficient for ERC-7540-style vaults. ERC-7540 defines explicit Pending, Claimable and Claimed states and mandates that the request and claim are separate lifecycle steps. citeturn20view2

IKL should represent this as:

```text
workflow AsyncRedeem {
    state Open {
        on request -> Pending
            produces RedeemClaim;
    }

    state Pending {
        on claimable -> Claimable;
        on timeout   -> RecoveryRequired;
    }

    state Claimable {
        on claim -> Settled;
    }

    state RecoveryRequired {
        on refund -> Refunded;
    }

    state Settled {}
    state Refunded {}
}
```

Not every backend supports every transition. ERC-7540, for example, intentionally does not standardize a general request-cancellation flow. citeturn20view2 A compiler therefore cannot silently lower `on cancel -> Refunded` to ERC-7540 and pretend the semantics survived.

The correct result is:

```text
compile error:
  backend erc7540 does not provide semantic capability:
  cancellable_pending_redemption
```

or an explicit alternative adapter requiring new assumptions.

### Foreign calls

The biggest danger to a “small” language is a construct like:

```text
require arbitrary_callback(bytes) == true
```

because that makes the callback language the real kernel.

IKL should instead allow foreign calls only through **typed adapter specifications**:

```text
adapter Vault4626 {
    fn redeem(
        shares: Amount<Share>,
        receiver: Principal<Ethereum>
    )
    requires Cap<Burn<Share>>
    produces Credit<Asset>
    effects {
        debit(subject, Share) <= shares;
        credit(receiver, Asset) >= 0;
    }
    trust {
        code_hash = 0x...;
        semantics_version = "4626-adapter-1";
    }
}
```

The adapter may contain arbitrary backend code, but its authority and observable effects are bounded by the kernel contract.

A v1 escape hatch should be named something deliberately alarming, such as:

```text
unsafe_backend_call
```

and should require explicit wallet re-approval with the unresolved effects shown. It must never silently inherit the user's existing intent signature.

## Formal semantics, composition, and theorem program

The language should define what counts as an acceptable **trace**, not merely an acceptable final state.

Let a system state be:

\[
\sigma =
\langle
L,\ C,\ O,\ X
\rangle
\]

where:

- \(L\) is asset-indexed ledger/resource state;
- \(C\) is outstanding capability state;
- \(O\) is the set of obligations;
- \(X\) is backend-specific external state abstracted through declared observations.

A trace is:

\[
\tau=
\sigma_0
\xrightarrow{e_1}
\sigma_1
\xrightarrow{e_2}
\cdots
\xrightarrow{e_n}
\sigma_n.
\]

The denotation of an intent is the set of traces it permits under an environment \(E\):

\[
\llbracket I \rrbracket_E
=
\{
\tau
\mid
Auth_I(\tau)
\land
Safe_I(\tau)
\land
Life_I(\tau)
\land
Goal_I(\tau)
\}.
\]

This is stronger than defining only:

\[
Goal_I(\sigma_n).
\]

A malicious plan could satisfy a final-state goal while briefly transferring all of the user's assets to an attacker and returning enough at the end. A terminal predicate would accept such a trace unless authority or prefix safety were modeled separately.

### Verification judgment

The core judgment should look like:

\[
\Gamma;\sigma_0;E
\vdash
\langle I,P,W\rangle
\Downarrow
\langle \rho,I_r\rangle
\]

where:

- \(\Gamma\) contains types and adapter specifications;
- \(I\) is the signed intent;
- \(P\) is the solver plan;
- \(W\) is settlement/evidence material;
- \(\rho\) is the receipt;
- \(I_r\) is any residual authority or obligation.

Acceptance requires:

\[
\begin{aligned}
ValidSig(I) &\\
\land Fresh(I,\sigma_0) &\\
\land Typed(P,\Gamma) &\\
\land Effects(P)\preceq Authority(I) &\\
\land PrefixSafe(P,I) &\\
\land LifecycleValid(P,I) &\\
\land EvidenceValid(W,I,E) &\\
\land GoalSatisfied(P,I) &.
\end{aligned}
\]

That ordering matters. A plan that achieves the goal but exceeds authority must fail.

### Semantic refinement

Compilation correctness should be stated as trace inclusion.

For a source program \(S\) and compiled intent IR \(C(S)\):

\[
\boxed{
\llbracket C(S)\rrbracket
\subseteq
\llbracket S\rrbracket
}
\]

The compiled artifact may be **more restrictive** than the source—perhaps a backend supports only one route—but must not permit a behavior forbidden by the source.

For an adapter lowering \(A_B\) to backend \(B\):

\[
Traces_B(A_B(C(S)))
\subseteq
\llbracket C(S)\rrbracket.
\]

This gives the compiler a strong rule:

> **A backend may reject unsupported semantics. It may not weaken them.**

There is also a necessary anti-vacuity condition. A compiler that translates every intent to `false` satisfies subset refinement trivially.

Therefore define:

\[
Feasible_B(S)
\Rightarrow
\exists \tau \in Traces_B(A_B(C(S))).
\]

This need not be proved universally; it can be a backend capability obligation or solver witness. But it prevents “always reject” from being labeled a successful compiler.

### Assumptions and observations

Environmental facts must carry provenance.

Instead of:

```text
assume ETH_price > 3000 USD
```

use:

```text
assume observation Chainlink.ETH_USD {
    value > 3000 USD;
    observed_at >= now - 60s;
    finality >= Ethereum.Safe;
}
```

An observation therefore has:

\[
Obs =
\langle source,\ value,\ domain,\ anchor,\ time,\ finality\rangle.
\]

A zero-knowledge proof can prove that a computation was performed over an input; it does not prove the external input was economically truthful. The trust assumption on the oracle, custodian, bridge, issuer, or attester remains explicit.

### Asset conservation versus solvency

IKL should make several propositions distinct.

Ledger conservation:

\[
\sum_{u} \Delta L(u,a)
=
Mint(a)-Burn(a).
\]

Debt accounting:

\[
Assets - Liabilities = Equity.
\]

Collateral adequacy:

\[
Value(Collateral)\cdot LTV
\ge
Value(Debt).
\]

Solvency:

\[
RealizableValue(Assets)
\ge
Obligations.
\]

Liquidity:

\[
AvailableCash(t)
\ge
DuePayments(t).
\]

These are not equivalent.

A stablecoin contract may perfectly conserve token supply while its off-chain reserve is insufficient. A lending protocol may conserve all token balances while borrowers are insolvent. A bridge may conserve wrapped supply relative to recorded lock events while the custodian or validator set is compromised.

That distinction is especially important given DeFiFormal's earlier attempt to make a ledger conservation primitive carry much of the financial model (`research/positive-program/basis/BASIS.md:43–67`). Conservation is valuable, but it is a local accounting property rather than a universal theory of financial soundness.

### Composition

IKL needs distinct operators:

\[
I_1 \land I_2
\]

for conjunction,

\[
I_1 \oplus I_2
\]

for alternatives,

\[
I_1 ; I_2
\]

for sequence,

\[
I_1 \parallel I_2
\]

for parallel intent execution,

and a bounded temporal repetition operator:

\[
repeat(I,n,period).
\]

These cannot have unconditional algebraic laws.

For conjunction:

\[
\llbracket I_1\land I_2\rrbracket
=
\llbracket I_1\rrbracket
\cap
\llbracket I_2\rrbracket
\]

only if their assumptions and lifecycle observations are interpreted in the same world.

For alternative:

\[
\llbracket I_1\oplus I_2\rrbracket
=
\llbracket I_1\rrbracket
\cup
\llbracket I_2\rrbracket,
\]

but the source must declare who chooses the branch: user, solver, protocol, or environment.

For parallel composition, a useful frame law is available only under separation:

\[
Effects(I_1)\cap Resources(I_2)=\varnothing
\]

and vice versa.

Under that condition, an action of \(I_1\) should preserve invariants about resources exclusively owned by \(I_2\).

Without separation, the law fails. Two individually safe intents can both be authorized against the same collateral pool and jointly exceed the user's limit.

This is precisely where resource-oriented semantics earn their complexity.

### Asynchronous composition

Cross-domain execution should not be modeled as a transaction that somehow becomes atomic across independently finalized systems.

Instead:

\[
Open
\rightarrow
Locked
\rightarrow
Executing
\rightarrow
Fulfilled
\rightarrow
Settled
\]

with branches:

\[
Executing\rightarrow Refundable,
\]

\[
Executing\rightarrow RecoveryRequired,
\]

\[
Locked\rightarrow Expired.
\]

Each transition produces evidence and consumes or produces obligations.

A cross-domain workflow might create:

\[
Obligation(
solver,
user,
100\ USDC_{dest},
deadline
).
\]

Settlement either consumes that obligation with destination-chain evidence or transforms it into a refund/recovery obligation.

**Compensation is not rollback.**

If a destination NFT purchase fails after the origin asset has been locked, paying the user back later is a separate economic action, not reversal of history.

CAKE's own architecture emphasizes that cross-chain execution introduces asynchrony and sub-transaction failure possibilities. citeturn12search1 NEAR's external-call ordering warning and withdrawal behavior provide a concrete implementation-level example of the same distinction. citeturn12search3turn12search6

### The theorem ledger

The following is the recommended positive theorem program. These are **targets and proof sketches**, not all claimed as completed formal proofs in this report.

| Theorem | Claim | Necessary assumptions | Status |
|---|---|---|---|
| Type soundness | Well-typed plans cannot mix incompatible asset/unit/domain types | Adapter signatures correct | Design target |
| Authority non-amplification | Accepted execution effects are a subset of signed capabilities | Kernel mediates all authority-consuming effects | Design target |
| Asset-indexed accounting | Net asset change equals explicit mint minus burn | All relevant deltas observable | Design target |
| Safe residualization | Sequential fills cannot exceed an affine budget | Each fill atomically consumes residual capability | Proof straightforward |
| Replay uniqueness | One-shot authority cannot be consumed twice | Nonce/intent ID consumed atomically in authority domain | Conditional theorem |
| Frame theorem | Disjoint-resource actions preserve the other's resource-local invariant | Separation/noninterference | Design target |
| Obligation preservation | Async transitions cannot silently discard a live obligation | Every transition consumes, fulfills, refunds, or carries obligation forward | Design target |
| Compiler refinement | Lowered backend behavior is a subset of source behavior | Correct compiler and adapter spec | Central mechanization target |
| Extension conservativity | Adding an adapter does not change existing program meaning | Versioned namespaces; no semantic override | Design target |
| Liveness | A valid intent eventually settles | Requires external assumptions: chain progress, solver/liquidity availability, message delivery, etc. | Never unconditional |

The smallest theorem was encoded as a Lean target in the accompanying prototype:

\[
d_1\le B
\land
d_2\le B-d_1
\Rightarrow
d_1+d_2\le B.
\]

The current research execution environment did not have `lean` or `lake` installed, so I do **not** label that artifact mechanized here. The included file contains the intended Lean statement and uses `omega`.

### Decidable and solvable fragments

A major language-design improvement is to separate:

**Verification expressiveness** from **automatic solver expressiveness**.

IKL verification may evaluate total deterministic functions over a concrete proposed plan, including fixed-point `mulDiv`, hashes, finite maps, comparisons, and adapter-supplied outputs.

IKL does **not** have to promise that every possible hard predicate can be efficiently solved.

A backend advertises solver capabilities, for example:

```text
supports:
    linear_asset_constraints
    fixed_rate_constraints
    finite_route_choice
    exact_mul_div_validation
    bounded_workflows

does_not_support:
    arbitrary_non_linear_search
    recursive_predicates
    unbounded_quantification
```

This avoids a recurring mistake in “intent languages”: Turing-complete predicates make everything expressible while making general solving impossible.

The kernel should prohibit unbounded loops, recursion and unconstrained arbitrary callbacks in v1. Financial mechanisms whose computation is complicated—such as Curve invariant iteration—can live in typed protocol adapters. The solver computes the concrete values; the verifier checks the adapter's declared semantics.

## Compiler, IR, runtime, signing, and backend mappings

IKL should have four separately serialized artifacts.

\[
\boxed{
Source
\rightarrow
SignedIntent
\rightarrow
SolverQuery
\rightarrow
Plan
\rightarrow
Receipt
}
\]

The solver must never mutate the `SignedIntent`.

### Canonical Intent IR

A machine-readable representation could start as:

```json
{
  "version": "ikl/0.1",
  "intentId": "0x...",
  "subject": {
    "principal": "eip155:1:0x..."
  },
  "validity": {
    "notBefore": 1788740000,
    "expires": 1788742800
  },
  "replay": {
    "nonce": "0x...",
    "authorityDomain": "eip155:1",
    "mode": "oneShot"
  },
  "authority": [
    {
      "kind": "spend",
      "asset": {
        "domain": "eip155:1",
        "issuer": "circle",
        "reference": "erc20:0x..."
      },
      "maxAmount": "1000000000",
      "recipients": [
        "eip155:1:0xSettlement..."
      ]
    }
  ],
  "requirements": [
    {
      "phase": "terminal",
      "op": "minCredit",
      "account": "eip155:1:0xAlice...",
      "asset": {
        "domain": "eip155:1",
        "reference": "erc20:0xWETH..."
      },
      "amount": "480000000000000000"
    }
  ],
  "assumptions": [],
  "lifecycle": {
    "kind": "atomic"
  },
  "extensions": []
}
```

This representation should be canonicalized and hashed independently of the solver plan.

The signed semantic hash should bind at least:

\[
H(
version,
subject,
validity,
replay,
asset\ identities,
authority,
requirements,
assumptions,
lifecycle,
extension\ semantics
).
\]

EIP-712 is an appropriate **EVM codec** for typed signing, but its own specification makes clear that replay protection remains an application responsibility. citeturn18view0

For contract accounts, ERC-1271 can validate whether the account accepts the signature. citeturn18view1 ERC-6492 can help with counterfactual/predeployment account signatures. citeturn18view2 None of those standards defines IKL's financial meaning; they merely authenticate it.

### Solver Query

The solver-facing request is not identical to the signed artifact:

```json
{
  "intentHash": "0x...",
  "constraints": "...normalized subset...",
  "preferences": [
    {"maximize": "terminal.credit:WETH"},
    {"minimize": "fees"}
  ],
  "disclosure": {
    "mode": "public"
  },
  "acceptedBackends": [
    "uniswapx",
    "cow",
    "direct-amm"
  ]
}
```

A solver can decline because it does not support a feature.

It cannot reinterpret a hard requirement as a preference.

### Plan IR

ERC-7683 provides the right conceptual precedent here. The current standard translates protocol-specific orders into steps, variables, payments and assumptions and explicitly frames the interface for solvers. citeturn20view3

IKL's generic Plan IR should therefore resemble:

```json
{
  "version": "ikl-plan/0.1",
  "intentHash": "0x...",
  "steps": [
    {
      "id": "s0",
      "domain": "eip155:1",
      "adapter": "uniswapx/reactor-v3",
      "callerCapability": "cap:0",
      "arguments": {},
      "dependencies": [],
      "revertPolicy": "abort",
      "expectedEffects": []
    }
  ],
  "payments": [],
  "assumptions": [],
  "witnessRequests": []
}
```

The canonical IKL plan should be able to lower into an ERC-7683-compatible resolver representation when semantics match.

It should not depend on ERC-7683 for its existence because the standard remains a draft and has changed materially enough to retain historical draft material on its current page. citeturn20view3

### Receipt

A receipt should say what was actually established:

```json
{
  "intentHash": "0x...",
  "planHash": "0x...",
  "backend": "eip155:1/uniswapx",
  "adapterVersion": "uniswapx-v3@0x...",
  "anchors": [
    {
      "domain": "eip155:1",
      "block": "0x...",
      "finality": "safe"
    }
  ],
  "observedEffects": [],
  "obligations": [],
  "status": "settled"
}
```

For asynchronous execution:

```json
"status": "pending",
"obligations": [
  {
    "id": "ob:...",
    "kind": "redeemClaim",
    "due": "...",
    "recovery": "..."
  }
]
```

A receipt must never say `settled` merely because a source-chain transaction succeeded.

### Wallet signing

Wallet presentation should be generated deterministically from the signed semantic object itself.

A useful display is:

```text
You authorize:

SPEND
  At most 1,000.00 USDC
  Ethereum
  From 0xAlice

RECEIVE REQUIREMENT
  At least 0.48 WETH
  Into 0xAlice

EXECUTION
  Any approved settlement route
  No other asset may be debited

VALID UNTIL
  Sep 6 2026 21:00 PDT

MAXIMUM ECONOMIC EXPOSURE
  1,000 USDC + network fees

EXTERNAL ASSUMPTIONS
  None

CANCELLATION
  Available until capability is locked or consumed
```

ERC-7730 is useful prior art because it provides structured metadata to turn low-level calls and messages into human-readable clear-signing displays. citeturn13search3 IKL should go further: it should render from the **semantic authorization object**, not attempt to reconstruct user meaning after compilation from arbitrary calldata.

Unknown semantic extensions should produce:

```text
Cannot safely display or verify semantic extension:
  namespace: foo.experimental
  version: 7
SIGNING REFUSED
```

rather than a generic “unknown field” warning.

### NEAR lowering

NEAR is an excellent backend for a narrow subset.

A fully quoted matched exchange can lower to signed `token_diff` entries. The NEAR Verifier can then enforce the matching of internal asset changes atomically. citeturn12search0turn16view0

But there is an important semantic loss.

Source:

```text
receive >= 0.48 WETH
spend <= 1000 USDC
solver may choose exact fill
```

does not necessarily correspond directly to an open-ended NEAR `token_diff`, because the documented token difference is already a concrete signed balance-delta vector.

Therefore the NEAR adapter has two honest options:

1. **Quote first, sign second.** Solver produces an exact proposed delta; IKL presents the concrete resulting authorization; the user signs it. Solver freedom ends at signature time.

2. **Use a richer verifier/policy account.** A separate contract enforces the inequality/policy itself.

The compiler must report which model is being used.

The direct lowering:

```text
IKL outcome constraint
    ↓ quote/solve
exact balance-delta plan
    ↓ user signs
NEAR token_diff
    ↓
Verifier atomic internal settlement
```

is useful, but it is not equivalent to an order whose filler can choose a route and amount after signing.

External withdrawal then becomes a separate workflow because NEAR documents asynchronous cross-contract completion and refund-dependent behavior. citeturn12search3turn12search6

### EVM lowering

The EVM has almost the opposite shape: many useful standards exist, but no single one supplies the semantic language.

An IKL compiler could use:

```text
IKL SignedIntent
    |
    +-- EIP-712 serialization
    +-- ERC-1271 / ERC-6492 signature verification
    |
    +-- ERC-7710/7715-style permission adapter
    |       or account-native capability module
    |
    +-- ERC-4337 UserOperation
    |       or EIP-7702 delegated account
    |
    +-- EIP-5792 / ERC-7821 call batching
    |
    +-- UniswapX / CoW / protocol adapter
    |
    +-- ERC-7683 Plan representation for compatible cross-chain orders
```

ERC-4337's `UserOperation` contains account execution data and leaves signature authorization semantics to the smart account implementation. citeturn15view0 That makes it a transport/execution substrate rather than an intent language.

EIP-7702 similarly provides persistent code delegation to EOAs and is motivated by batching, sponsorship and privilege de-escalation. citeturn14search1 The language should treat that as an account capability, not assume a 7702 delegation is itself sufficiently scoped.

EIP-5792 explicitly defines wallet methods for sending batches of calls and lets applications require atomic execution when the wallet advertises support. citeturn20view0 Again, this is a backend feature.

UniswapX's Reactor architecture is particularly compatible with IKL because Reactors validate orders, resolve inputs/outputs, invoke filler execution and verify outputs. citeturn21search1 A Reactor adapter can therefore implement the Plan-to-Execution boundary while IKL remains protocol-independent.

### Extended-UTxO portability

The third backend should intentionally be structurally different from NEAR and EVM.

Cardano's UTxO-oriented design emphasizes locality and prospective determinism: transaction validity can be determined from the transaction and referenced state before inclusion, and validity intervals provide deterministic time constraints. citeturn19search2

That makes affine intent capabilities conceptually natural:

```text
IntentCapability UTxO
  value: 1000 USDC authority
  datum: constraints
```

A partial fill consumes the old capability output and creates:

```text
filled result
+
residual capability UTxO
```

rather than maintaining a mutable allowance.

This should be a **portability test**, not an initial production target. It tells us whether the core depends accidentally on EVM account/storage assumptions.

### Compiler pipeline

The recommended pipeline is:

```text
parse
  ↓
name + asset resolution
  ↓
type checking
  ↓
capability/effect checking
  ↓
financial-library elaboration
  ↓
temporal/lifecycle elaboration
  ↓
constraint normalization
  ↓
canonical Intent IR
  ↓
deterministic wallet rendering
  ↓
signature
  ↓
Solver Query
  ↓
solver Plan IR
  ↓
backend capability checking
  ↓
adapter lowering
  ↓
preflight/simulation
  ↓
execution
  ↓
receipt/evidence verification
  ↓
residual capability + obligations
```

Simulation belongs *after* lowering and before execution, but a successful simulation is not a promise about future state. NEAR itself treats `simulate_intents` as a non-state-mutating way to validate and inspect execution. citeturn12search0

### Prototype

I built and executed a minimal reference checker illustrating the critical distinction between goals and authority.

[Download the executable IKL prototype](sandbox:/mnt/data/intent-lang-prototype.zip)

SHA-256:

```text
33d122d424bb2a87c6f3a92beaed695c0cb39af859273b0fa60245846a62f3e3
```

The good plan:

```text
Alice debit: 1,000 USDC
Alice credit: 0.49 WETH
```

under an authorization of:

```text
max USDC debit = 1,000
min WETH credit = 0.48
```

returned:

```json
{
  "note": "concrete-plan safety checker; not a solver or optimality proof",
  "residual_authority": {
    "eip155:1/erc20:USDC": 0
  },
  "status": "ACCEPT"
}
```

The adversarial plan deliberately delivered enough WETH but debited 1,100 USDC. It was rejected:

```json
{
  "status": "REJECT",
  "reason": "authority exceeded for eip155:1/erc20:USDC"
}
```

The included unit tests passed:

```text
test_bad ... ok
test_good ... ok

Ran 2 tests

OK
```

This prototype is intentionally tiny. It is evidence that the semantic separation is executable, not evidence that the full language has been verified.

The package includes:

```text
intent-lang-prototype/
├── README.md
├── checker.py
├── Proofs.lean
├── conformance-manifest.json
├── schemas/
│   ├── intent.schema.json
│   └── plan.schema.json
├── examples/
│   ├── swap.intent.json
│   ├── swap.good.plan.json
│   └── swap.bad.plan.json
└── tests/
    └── test_checker.py
```

## Worked financial programs and adversarial cases

The core should be judged on cases that force materially different semantics, not on how many swap formats it can encode.

### Exact-output exchange

```text
intent ExactOutput {
    authorize spend Ethereum.USDC
        up_to 1_000 USDC
        from alice
        to SettlementWhitelist;

    require terminal:
        credit(alice, Ethereum.WETH) >= 0.48 WETH;

    require always:
        debit(alice, Ethereum.USDC) <= 1_000 USDC;

    prefer maximize credit(alice, Ethereum.WETH);
}
```

A valid solver plan:

```text
debit  997 USDC
credit 0.487 WETH
```

is valid.

A plan:

```text
debit 1,050 USDC
credit 0.500 WETH
```

is invalid even though the desired WETH outcome is exceeded.

This is the most basic example of why `goal != authority`.

UniswapX already separates signed orders from filler strategies and uses Reactors to enforce specific order formats, making it a good candidate backend for this example. citeturn21search1

### Partial fills and a matched batch

```text
intent SellUSDC {
    authorize spend Ethereum.USDC
        aggregate_up_to 10_000 USDC;

    require each_fill:
        receive(WETH) * 2_000 USDC
        >= spend(USDC) * 1 WETH;

    require always:
        cumulative_spend(USDC) <= 10_000 USDC;

    partial_fill allowed;
}
```

Suppose:

\[
B_0=10,000.
\]

Fill one spends:

\[
d_1=3,000
\]

so:

\[
B_1=7,000.
\]

Fill two may spend:

\[
d_2\le7,000.
\]

The signed intent is not copied into two fresh 10,000-USDC authorities.

For a matched batch:

```text
Alice:  -3000 USDC, +1.51 WETH
Bob:    +3000 USDC, -1.51 WETH
```

the local accounting condition is:

\[
\sum_u\Delta_u(USDC)=0,
\qquad
\sum_u\Delta_u(WETH)=0.
\]

This resembles NEAR Intents' documented `token_diff` balancing model. citeturn12search0

The negative control is a batch in which every user's final desired asset amount is met but the same user's spending capability was consumed twice by concurrent solvers. The goal checker alone accepts; the affine authority checker must reject.

### Atomic lending refinance

```text
intent Refinance {
    authorize repay Aave.USDC_Debt up_to 50_000 USDC;
    authorize move Aave.WETH_Collateral up_to 20 WETH;
    authorize create Morpho.USDC_Debt up_to 50_500 USDC;

    require always:
        collateral_value(subject)
          >= maintenance_ratio * debt_value(subject);

    require terminal:
        Aave.debt(subject, USDC) == 0;

    require terminal:
        Morpho.debt(subject, USDC) <= 50_500 USDC;

    require terminal:
        net_collateral_loss(subject) == 0;
}
```

This example demonstrates why liabilities are first-class.

It is insufficient to state:

```text
receive 20 WETH
```

because the financial meaning includes extinguishing one debt and creating another.

The `always` condition also matters. A plan that temporarily releases all collateral, leaves debt outstanding and later re-collateralizes could satisfy terminal conditions but create a liquidation window.

For a single EVM transaction, an adapter may be able to guarantee atomicity. EIP-5792 lets a caller request atomic execution for a batch when the wallet supports it. citeturn20view0

If the refinance spans independent domains, the same `always` guarantee may be impossible. The compiler must reject it or require a different economic construction such as prefunding or solver credit.

### Asynchronous vault redemption

```text
intent RedeemFund {
    authorize lock FundShares up_to 100 shares;

    lifecycle AsyncRedeem;

    require at(Settled):
        credit(alice, USDC) >= 9_500 USDC;

    require always:
        obligation_value(RedeemClaim)
        + settled_value_received
        >= protected_claim_floor;
}
```

Execution:

```text
Open
  --requestRedeem-->
Pending
  --vault processing-->
Claimable
  --claim-->
Settled
```

ERC-7540 explicitly defines Pending, Claimable and Claimed request stages and requires claim to be a separate action. citeturn20view2

A valid plan preserves a live claim while pending.

An invalid model deducts the user's shares and records no asset and no obligation until eventual claim. Such a model could satisfy conservation locally while erasing the user's financial claim.

This is a direct demonstration of why the kernel needs:

\[
Resource + Obligation,
\]

not merely balances.

### Recurring payments

```text
intent Subscription {
    authorize recurring_spend USDC {
        per_period <= 100 USDC;
        periods <= 12;
        aggregate <= 1_200 USDC;
        recipient = service;
    }

    period 30 days;

    revocable before next_lock;

    require each_execution:
        debit(alice, USDC) <= 100 USDC;
}
```

A recurring authorization is not 12 independent copies of a 1,200-USDC capability.

The state contains:

```text
remaining_periods
remaining_aggregate_budget
next_valid_window
revocation_state
```

A valid fifth execution after four 100-USDC payments has at most:

\[
800\ USDC
\]

aggregate authority left.

A malicious agent submitting five concurrent “month five” executions must not obtain five payments. The authority home must consume a period ticket or increment a monotonic counter atomically.

ERC-7715 explicitly lists subscription-like and automated activity among the motivations for wallet execution permissions. citeturn13search2 IKL improves that model by giving recurring aggregate budgets a canonical semantics instead of leaving them entirely to opaque permission types.

### Portfolio rebalancing with delegated agent

```text
intent AgentRebalance {
    delegate agent42 {
        authorize trade PortfolioAssets;
        max_notional_per_day = 25_000 USDC;
        max_total_fee_per_day = 50 USDC;
        allowed_protocols = AuditedVenues;
        expires = 2026-10-01T00:00Z;
    }

    require terminal:
        weight(BTC) in 45% .. 55%;

    require terminal:
        weight(ETH) in 45% .. 55%;

    require always:
        no_external_recipient_except(SettlementWhitelist);

    prefer minimize turnover
      then minimize fees;
}
```

ERC-7710 is explicitly designed around delegating capabilities, including bounded permissions for automated systems. citeturn13search1 IKL should treat such delegation as a capability relationship, not as trust in the AI agent.

The agent can reason, ask an LLM for a route, interact with solvers, or fail completely. It still cannot generate authority beyond the signed budget.

The security invariant is:

\[
Authority(agent\ generated\ plan)
\subseteq
DelegatedAuthority.
\]

Composition of subagents must preserve the same bound:

\[
\bigcup_i Authority(agent_i)
\subseteq
Authority(parent).
\]

No language model should be in the trusted authorization path.

### Contingent claim

```text
intent WeatherProtection {
    authorize premium up_to 500 USDC to Insurer;

    assume RainIndex {
        source = OracleRegistry.rainfall;
        region = "X";
        freshness <= 1 hour;
    };

    require terminal:
        if RainIndex < threshold
        then claim(alice, Insurer, USDC) >= 10_000 USDC
        else claim(alice, Insurer, USDC) == 0;
}
```

The output is a **claim**, not necessarily an immediate token transfer.

That distinction matters for prediction markets, insurance, options, undercollateralized credit and tokenized off-chain assets. A user may acquire a right against an issuer or settlement pool rather than an immediately settled asset.

An invalid abstraction rewrites:

```text
10,000 USDC claim
```

as:

```text
10,000 USDC
```

and therefore hides counterparty risk.

The oracle observation must carry source, state anchor and freshness. Proof of correct evaluation of the conditional does not prove the oracle's real-world measurement is true.

### Cross-domain payment with recovery

```text
intent CrossDomainPay {
    authorize lock Ethereum.USDC up_to 1_000 USDC;

    require at(Fulfilled):
        credit(bob, Solana.USDC) >= 995 USDC;

    lifecycle CrossDomain {
        timeout = 10 min;
        recovery = refund_origin;
    }

    assume DestinationFinality {
        policy = Solana.Finalized;
    };
}
```

The lifecycle should be:

```text
Open
  ↓ lock
Locked
  ↓ destination execution
DestinationPending
  ↓ finality evidence
Fulfilled
  ↓ source settlement
Settled
```

Failure:

```text
Locked
  ↓ timeout
Refundable
  ↓ refund
Refunded
```

The language must not promise:

```text
atomic cross-chain transfer
```

unless a precise protocol construction genuinely supplies the claimed atomicity.

ERC-7683 is useful here because the current draft makes resolver assumptions and cross-chain execution requirements explicit for solvers. citeturn20view3 CAKE likewise treats cross-chain solving and settlement as asynchronous infrastructure concerns. citeturn12search1

A destination transaction that executes but is subsequently reorganized must not count as fulfillment if the intent requires a stronger finality level.

Likewise, if the destination succeeds after the source has already entered a refund path, the protocol must define the race resolution. “Timeout” alone is not a proof that the solver can no longer complete.

### What these examples show

Across all eight programs, only a few semantic concepts recur:

\[
\boxed{
Assets,\ Authority,\ Requirements,\ Obligations,\ Time,\ Evidence
}
\]

The specific financial names—swap, vault, loan, insurance, payment—mostly belong in libraries.

That is a strong empirical reason to prefer this kernel over a list of DeFi mechanism primitives.

## Security, privacy, validation, and implementation plan

The security model must assume that solvers, applications, adapters, token contracts and external data sources can fail or behave adversarially within their declared trust boundaries.

### Core attack classes

| Attack | Why an ordinary outcome language fails | IKL defense |
|---|---|---|
| Solver exceeds spend while delivering desired output | Terminal goal still true | Independent affine capability check |
| Solver transfers assets to an unauthorized intermediate recipient | Final balances can look acceptable | Recipient/caller/effect capabilities |
| Two solvers consume same partial-fill budget | Same signature is replayed concurrently | Linear authority ticket / atomic residualization |
| Frontend displays one thing and signs another | Raw wire format differs from UI | Deterministic rendering from canonical signed IR |
| Replay on another backend/version | Message may remain cryptographically valid | Domain, adapter and semantic version binding |
| Stale oracle satisfies conditional | Predicate evaluates correctly on old data | Typed observation with freshness/finality |
| Malicious token callback creates hidden effects | Adapter assumes vanilla ERC-20 behavior | Effect summaries + post-state deltas + token policy |
| Cross-chain address confusion | Same byte address appears on multiple networks | Domain-qualified principal/asset types |
| Backend upgrade changes semantics after authorization | Same endpoint now executes new code | Adapter/code-version commitment or explicit upgrade policy |
| AI agent composes delegated permissions into broader authority | Agent can reason beyond intended scope | Capability subset rule checked by kernel |

EIP-712's own security section emphasizes that replay handling is application-specific. citeturn18view0 ERC-7715 similarly places substantial responsibility on wallets to enforce and clearly display permission scope. citeturn13search2 Those responsibilities should be made protocol-semantic in IKL rather than left as UI conventions.

### Clear signing as a proof obligation

The compiler should produce both:

\[
bytes = CanonicalEncode(I)
\]

and:

\[
display = Render(I).
\]

The wallet checks:

\[
Hash(Parse(bytes))=Hash(I)
\]

and renders from the parsed semantic object.

For every authority, show:

- maximum debit;
- asset and domain;
- possible recipient class;
- valid period;
- aggregate limits;
- external calls permitted;
- cancellation boundary.

For every desired result, show:

- minimum guaranteed outcome;
- whether it is immediate asset, contingent claim, vault share, debt change, or asynchronous obligation.

For every assumption, show:

- source;
- freshness;
- finality;
- trust dependency.

ERC-7730 provides a useful standardized vocabulary for enriching low-level structured data for wallet displays, but IKL should make semantic display a mandatory consequence of the language rather than an optional metadata overlay. citeturn13search3

### Privacy

Privacy should **not** be claimed for v1 merely because intents are submitted to private solvers.

The language should define the potential leakage of:

\[
L =
L_{goal}
\cup
L_{authority}
\cup
L_{preferences}
\cup
L_{timing}
\cup
L_{solver\ probes}
\cup
L_{settlement}.
\]

A public order necessarily leaks enough constraints for solvers to evaluate it.

A selectively disclosed extension could later reveal only:

```text
I possess authority up to B
goal predicate is committed as H(G)
```

and provide proofs over hidden values. Anoma's shielded resource-machine direction demonstrates how resources and logic proofs can support privacy-preserving variants. citeturn19search0

But privacy introduces substantial tension with open solver competition. Solvers need information to price risk, route liquidity and determine feasibility. A v1 implementation should therefore support public intent semantics and transport-level encrypted RFQs, while deferring generalized private predicates and private optimality.

### Proof-carrying plans

Proofs are useful only when the statement is sharply defined.

A settlement proof should bind:

```text
intent_hash
plan_hash
adapter_version
state_anchor(s)
public_observations
declared effects
result status
```

Potential proof classes are separate:

\[
Proof_{authorization}
\]

\[
Proof_{plan\ refinement}
\]

\[
Proof_{execution}
\]

\[
Proof_{predicate}
\]

\[
Proof_{optimality}
\]

\[
Attestation_{external\ fact}.
\]

They should never be described interchangeably.

A validity proof that:

\[
output \ge minOutput
\]

does not prove that:

\[
solver\ selected\ globally\ best\ available\ price.
\]

A proof that a reserve attestation signature is valid does not prove the reserve assets exist.

The first release should use transparent deterministic checking wherever it is inexpensive. ZK proofs should be introduced only where confidentiality or cross-system verification cost justifies them.

Compact and ZKIR should not be architectural dependencies until exact supplied specifications can be mapped to these proof obligations. No interface for either should be invented.

### Conformance testing

The initial conformance manifest should include at least:

```json
{
  "version": "0.1",
  "required_tests": [
    "goal-satisfied-authority-exceeded-must-reject",
    "authority-satisfied-goal-missed-must-reject",
    "asset-nonconservation-must-reject",
    "unknown-semantic-extension-must-reject",
    "expired-intent-must-reject",
    "plan-intent-binding-mismatch-must-reject",
    "partial-fill-residual-budget-must-decrease",
    "same-asset-different-domain-must-not-unify"
  ]
}
```

The prototype package already includes that manifest.

The full suite should add mutation tests:

```text
increase debit by 1
change recipient
change chain ID
change token address
change rounding mode
change deadline
delete an obligation
reuse residual capability
swap oracle source
reduce finality
change adapter version
insert hidden callback
reorder asynchronous messages
duplicate fulfillment receipt
deliver after cancellation
cancel after solver commitment
```

A correct verifier should reject mutations that alter protected semantics.

### Contract-to-model fidelity

For each backend adapter:

\[
AdapterSpec
\leftrightarrow
Implementation
\]

needs its own evidence.

The strongest practical validation sequence is:

```text
reference semantic model
        ↓
generated backend execution
        ↓
local simulation
        ↓
differential test against protocol implementation
        ↓
mutation/adversarial test
        ↓
formal refinement proof for critical adapters
```

NEAR's simulator is useful as a backend validation tool, but its own definition is execution without state modification, so it should remain one layer in this evidence chain rather than be treated as proof of future execution. citeturn12search0

UniswapX's Reactor separation also lends itself to adapter-specific validation because the Reactor owns validation/resolution/settlement semantics for an order type while fillers can choose execution strategies. citeturn21search1

### Relationship to the DeFi kernel

The DeFiFormal repository's most valuable pieces should be reorganized rather than discarded.

**Retain:** protocol corpus, mechanism taxonomy, Quint cases, formal counterexamples, arithmetic regressions, shared-state examples, asset accounting work, and the discipline of distinguishing theorem/measurement/conjecture.

**Retain but demote:** the mechanism atlas as a **library coverage ontology**, not as the execution or verification kernel.

**Withdraw as kernel foundation:** the four-primitive `Led/Prop/Cmp/Post` claim, because the repository itself explicitly withdrew the load-bearing \(Q/\Sigma\) invariant (`research/positive-program/sigma/QSIGMA-VERDICT.md:1–8`).

**Replace:** syntactic generation as the definition of semantic expressibility. `GENERATION.md` defines a generation criterion around allowed/banned arithmetic and reports residual definitions (`research/positive-program/basis/GENERATION.md:68–86, 88–111`); this can remain a source-code factoring experiment but should not decide whether a financial behavior is semantically representable.

**Replace:** keyword-based “certificates.” The supplied `gate33_cert_check.py` says it walks names, classifies them using strings such as `credit`, `muldiv`, `healthy`, and `price`, includes an `OTHER` tag, and calls its output a “keyword certificate seed; not signature-level” (`research/positive-program/sigma/gate33_cert_check.py:1–8, 23–47, 95–104`). A real certificate must bind typed effects, capabilities, state transitions, adapter semantics and proof obligations—not identifiers.

The resulting architecture is:

```text
DeFiFormal Atlas
    └── financial taxonomy + mechanism regression corpus

Financial Libraries
    ├── swaps
    ├── lending
    ├── vaults
    ├── derivatives
    ├── staking
    ├── payments
    ├── stablecoin claims
    └── cross-domain workflows

IKL
    └── source financial intent language

Intent Kernel IR
    ├── typed resources
    ├── capabilities
    ├── requirements
    ├── obligations
    ├── observations
    └── lifecycle

Plan IR
    └── solver-selected execution

Adapters
    ├── NEAR Verifier
    ├── ERC-4337 / 7702 accounts
    ├── UniswapX
    ├── CoW
    ├── ERC-7683 resolver
    ├── ERC-7540 vaults
    └── future eUTxO

Verification Kernel
    └── refinement + authority + accounting + evidence
```

This is a much cleaner mathematical division than asking one algebra to simultaneously classify DeFi, generate protocol behavior and prove runtime safety.

### Implementation milestones

The **first thirty days** should freeze the semantics rather than build a large solver network.

The acceptance gate should be a canonical IKL IR supporting:

```text
domain-qualified assets
integer quantities
one-shot capabilities
hard terminal requirements
prefix max-debit requirements
deadlines/nonces
lexicographic preferences
atomic lifecycle
concrete Plan IR
receipts
```

with a reference checker that passes at least:

- exact-output swap;
- unauthorized-recipient negative test;
- over-debit negative test;
- replay negative test;
- cross-domain asset-type mismatch.

The existing prototype is an initial seed, not the finished milestone.

The **first ninety days** should add financial and temporal semantics:

```text
partial-fill residualization
multi-party batch matching
explicit liabilities/claims
Pending/Claimable/Settled workflows
assumption/evidence objects
typed foreign-call effects
EVM adapter
NEAR adapter
ERC-7540 adapter
```

Acceptance should require differential tests against real backend behavior and one mechanized result more meaningful than arithmetic residualization—preferably **authority non-amplification for sequential composition** or **compiler refinement for the pure token-delta fragment**.

The most useful first mechanized theorem would be:

\[
\Gamma\vdash P:I
\land
Verify(I,P)=accept
\Rightarrow
Effects_{user}(P)\preceq Capability(I).
\]

The **first one hundred eighty days** should add:

```text
cross-domain obligation state machines
cancellation/fill race semantics
bounded recurring permissions
AI-agent delegation
adapter version commitments
wallet semantic renderer
ERC-7683 Plan lowering
extended-UTxO prototype
property-based and adversarial scheduler harness
```

The acceptance gate should include the eight worked examples in this report plus held-out DeFiFormal cases chosen *before* changing the kernel.

At that point, there should be enough evidence to decide whether IKL deserves to become a standalone language.

### First release boundaries

IKL 0.1 should deliberately exclude:

```text
arbitrary recursive predicates
general Turing-complete callbacks
unbounded loops
implicit cross-chain atomicity
implicit oracle truth
implicit asset equivalence
general private intents
global solver optimality claims
automatic incentive-compatibility proofs
economic solvency inferred from token conservation
```

That exclusion is a feature.

An intents language becomes trustworthy by saying precisely what it cannot express or guarantee.

### Extension governance

Extensions need three classes.

```text
semantic
adapter
advisory
```

A **semantic extension** changes accepted traces and therefore requires explicit compiler/verifier support and a signed version identifier.

An **adapter extension** provides a new way of implementing existing semantics. It must prove or test refinement.

An **advisory extension** may influence solving or presentation but cannot change authorization or validity.

Unknown semantic extensions:

```text
MUST reject
```

Unknown optional advisory extensions:

```text
MAY ignore
```

An adapter is substitutable only when:

\[
Traces(Adapter_{new}, I)
\subseteq
\llbracket I\rrbracket.
\]

A backend upgrade that weakens the authority boundary is therefore not “compatible” merely because its ABI remains unchanged.

### Final answers

**What is the smallest defensible semantic core?**

Not a list of DeFi primitives. It is:

\[
\boxed{
\text{typed resources}
+
\text{bounded capabilities}
+
\text{trace requirements}
+
\text{obligations}
+
\text{observations}
+
\text{lifecycle}
}
\]

with validity/replay metadata and a separate preference layer.

Operationally, the most important kernel judgment is:

\[
\boxed{
PlanEffects
\subseteq
SignedAuthority
\quad\land\quad
PlanTrace
\models
SignedRequirements
}
\]

Everything else should justify why it belongs inside that boundary.

**What belongs in libraries?**

Swaps, AMMs, lending, vault accounting, staking, derivatives, insurance, stablecoin models, liquidation, yield strategies, bridge constructions, fee algorithms and protocol-specific arithmetic. These are financially meaningful and reusable, but they do not need to be primitive syntax.

**What belongs in solvers?**

Route search, liquidity discovery, RFQs, batching, matching, optimization, gas strategy, cross-domain liquidity sourcing and transaction construction.

Solvers propose plans. They do not define what the user authorized.

**What belongs in wallets?**

Identity/key control, permission negotiation, semantic rendering, attenuation of requested authority, signing, revocation interfaces, capability inspection and risk warnings.

ERC-7715 and ERC-7730 provide useful pieces of this layer. citeturn13search2turn13search3

**What belongs in settlement adapters?**

Backend calls, protocol-specific state interpretation, effect extraction, finality rules, foreign-call semantics, bridge/message evidence, request lifecycle handling and receipt generation.

**What should be borrowed from NEAR?**

Internal asset-delta settlement, batch balancing, signed validity, replay machinery and simulation. citeturn12search0turn12search3

Do **not** borrow the assumption that local Verifier atomicity implies end-to-end atomic execution; NEAR's own asynchronous-call and withdrawal documentation demonstrates the boundary. citeturn12search3turn12search6

**What should be borrowed from CAKE?**

The separation of Application, Permission, Solving and Settlement, plus the insistence that users should express outcomes rather than manually specify cross-chain paths. citeturn12search1

Do **not** elevate CAKE's design trade-off framing into a mathematical impossibility theorem without a formal model.

**What should be borrowed from ERC-7683?**

Steps, variables, dependencies, payments and explicit solver assumptions as inspiration for **Plan IR**. citeturn20view3

Do **not** make the current 7683 resolver representation the user language.

**What should be borrowed from ERC-4337, EIP-7702, ERC-7710/7715 and EIP-5792?**

Account-defined validation, delegated execution, bounded permission concepts, batching and backend capability discovery. citeturn15view0turn14search1turn13search1turn13search2turn20view0

Do **not** confuse an account's ability to execute arbitrary calls with a user's authorization for arbitrary solver routes.

**What should be borrowed from UniswapX and CoW?**

Competition among independent execution actors and protocol-specific settlement/validation modules. UniswapX's Reactor model is especially useful as an adapter architecture. citeturn21search1 CoW supplies evidence that batch matching and solver competition can be useful while leaving execution paths open. citeturn21search14

**What should be borrowed from Anoma?**

Create/consume resource semantics, balancing and proof-oriented resource logic. citeturn19search0

Do **not** hide arbitrary computational complexity inside unrestricted resource predicates.

**How does this improve the DeFi kernel?**

It stops asking one object to do four incompatible jobs.

The DeFi taxonomy tells us **what financial systems exist**.

Financial libraries describe **common mechanisms**.

IKL specifies **what a user permits and requires**.

The verification kernel checks **whether a proposed execution stays within that meaning**.

That separation is simpler than the current DeFiFormal primitive program while giving stronger runtime properties.

**Which guarantees survive composition?**

Authority bounds survive sequential composition when residual capabilities are consumed correctly.

Asset conservation survives when issuance and burn are explicit and adapters observe all relevant deltas.

Frame/locality properties survive parallel composition under disjoint-resource or proven noninterference assumptions.

Compiler refinement survives backend substitution when each adapter proves:

\[
Traces_B(A_B(I))
\subseteq
\llbracket I\rrbracket.
\]

Liveness does **not** survive without explicit environmental assumptions.

Cross-domain atomicity does **not** emerge merely from composition.

Solvency does **not** follow from balance conservation.

Optimal execution does **not** follow from satisfying a minimum-output predicate.

**What must never be abstracted away from the user?**

The maximum value that can be lost or debited.

The exact asset and domain identities.

Who can receive or control assets.

Any newly created debt or liability.

Whether the outcome is an asset or merely a claim.

Expiration and revocation boundaries.

Whether funds become locked.

Whether execution is atomic or asynchronous.

What happens on timeout or failure.

Which external facts or counterparties the guarantee depends upon.

Those are the economic semantics of the signature.

**What experiment could disprove this design?**

Freeze a held-out set containing the difficult DeFiFormal cases plus non-EVM and asynchronous systems, then attempt to encode them **without adding arbitrary predicates to the kernel**.

The decisive experiment is:

\[
\text{Can }
\{
Resource,\ Capability,\ Requirement,\ Obligation,\ Observation,\ Lifecycle
\}
\]

represent the economically important behavior of:

```text
Uniswap-style nonlinear swaps and rounding
Curve-style iterative arithmetic
ordered redemption
bad-debt socialization
margin/funding
asynchronous ERC-7540 vaults
multi-party solver batches
recurring permissions
counterparty-dependent off-chain claims
cross-domain settlement and recovery
```

while preserving backend refinement and keeping the trusted checker small?

The recommendation should be rejected or materially revised if the answer requires repeatedly smuggling core semantics into an unrestricted callback language.

Conversely, if those behaviors can remain in typed financial/adaptor libraries while the core continues to verify authority, obligations, resources and trace conditions, then the architecture has passed a much stronger test than “how few primitive names can generate the corpus.”

**What exactly should be built first?**

Build neither a solver network nor a large DeFi standard library.

Build the **semantic firewall**:

```text
IKL parser
    ↓
typed canonical Intent IR
    ↓
deterministic signing renderer
    ↓
affine capability checker
    ↓
Plan IR
    ↓
concrete-plan verifier
    ↓
residual capability / obligation receipt
```

Then implement exactly two deliberately different backends:

```text
NEAR Intents
EVM + UniswapX or ERC-4337 account
```

and one asynchronous adapter:

```text
ERC-7540
```

The central acceptance test is simple and unforgiving:

> **For every solver plan the backend can execute under a signed intent, can the system demonstrate that the plan did not gain one unit of authority, one recipient, one asset identity, one lifecycle transition, or one external assumption that the user did not authorize?**

If that property can be made precise, mechanized, and preserved through the adapters, the project has a credible intents-first language.

If it cannot, additional DeFi primitives will not fix the underlying problem.