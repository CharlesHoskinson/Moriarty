# Modernizing Marlowe: Language, Semantics, Runtime, Tooling, Governance, and Adoption

## Executive decision

**Research date: September 2, 2026.**

The strongest modernization path for Marlowe is **not** to turn it into a more general smart-contract language. It is to make the distinction between its **small, bounded, formally analyzable financial-contract kernel** and its **authoring language/platform** much sharper.

My recommendation is therefore a **Verified Marlowe Core 2 plus a strongly typed surface language**, accompanied by a trust-minimized Runtime, explicit extension boundaries, versioned conformance artifacts, and a substantially stronger release/governance process.

That conclusion follows from an important asymmetry in Marlowe today. The small continuation-based language is responsible for most of what makes Marlowe unusual: finite execution, explicit timeouts, a small transition system, analyzability, conservation properties, quiescence properties, and precomputable structural bounds. At the same time, those exact restrictions make repetitive financial agreements verbose, abstraction weak, type distinctions coarse, large contracts difficult to execute under ledger limits, and integration with external systems awkward. The official April 2026 V2 report has reached essentially the same pressure points: simplified `When`, action sets, state compression, bounded iteration, cryptographic operations, and stronger typing. citeturn3view1turn8view0

The architectural error would be to solve every usability problem by enlarging the trusted semantic kernel. A better solution is:

> **Keep Core 2 very small. Put human-oriented abstraction, financial-domain typing, modules, schedules, packages, bounded compile-time construction, and most conveniences in a typed source language that elaborates into an explicitly finite Core 2 program.**

That model resembles the successful separation found in other high-assurance systems: Simplicity deliberately uses a small statically analyzable core with formal semantics; Scilla separates pure computation, state mutation, and communication; Aiken demonstrates the developer-experience value of purpose-built typing, integrated tests, friendly diagnostics, formatting, and an LSP; Move demonstrates useful type-level distinctions between different kinds of assets and package/module boundaries. Those lessons can be imported without importing those systems' full expressiveness. citeturn24search10turn24search5turn25search8turn25search3

The most consequential research finding is also a warning: **Marlowe's semantic safety claims must be stated more carefully than they often have been.** The formal semantics establish significant properties, but Cardano executability is a separate question. Marlowe's own validator test report explicitly demonstrates that a semantically valid transition can be impossible to execute because of ledger protocol limits, and invalid initial states can invalidate assumptions behind the semantics and even make funds permanently inaccessible. citeturn12view2

That means V2 should adopt a layered definition of assurance:

**semantic validity → validator correspondence → ledger feasibility → transaction constructability → continuation availability → authorization validity → wallet correctness → participant liveness.**

“Safe Marlowe contract” should no longer collapse those layers into one claim.

The recommended scope is consequently:

| Area | Recommendation |
|---|---|
| Marlowe Core | **Preserve and minimally evolve** |
| `When` timeout semantics | **Simplify in V2.0** |
| Atomic action sets / `WhenAll` | **V2.0, after precise semantics and proofs** |
| State compression | **V2.0 implementation priority** |
| Invalid initial-state checking | **Mandatory V2.0 release gate** |
| Strong financial types | **V2.0 surface language; later selectively reflect into Core** |
| Modules / reusable definitions / packages | **Surface language** |
| Bounded iteration | **V2.1**, not rushed into V2.0 |
| Business calendars / recurring schedules | **Surface libraries/compiler** |
| `EntryDeposit` / dynamic participation | **V2.1 after authorization proof** |
| Oracle data model | **Typed protocol boundary, not implicit `Choice` convention** |
| Minting / arbitrary validator composition | **Outside Core** |
| General-purpose loops or recursion | **Reject** |
| Arbitrary Plutus/Aiken calls from Core | **Reject** |
| Hydra execution | **Specialized backend research, not V2 dependency** |
| Multi-chain portability | **Defer** |
| AI authoring | **Untrusted frontend only** |

The scorecard in this research gives the verified-core-plus-surface design **4.11/5**, versus 3.45 for a conservative V2, 3.61 for a highly composable protocol DSL, 3.24 for V1 stabilization alone, and 3.20 for portability/L2 as the primary strategy. Those values are architectural judgments rather than experimental measurements; their assumptions are exposed below.

A source limitation is worth stating explicitly. The official April 28, 2026 Marlowe V2 blog summary was accessible and provides the V2.0/V2.1 categorization, while the linked Google document titled *Marlowe language V2 Final Report* did not expose its full body through the research browser because its rendering requires JavaScript. I therefore treat the official blog summary and subsequent public proposal material as verified sources, while details present only in the inaccessible document should be revalidated before specification work begins. citeturn3view1turn4view0


## Marlowe today

Marlowe remains one of the more unusual Cardano smart-contract systems because it is simultaneously a DSL, an executable semantics, a set of Plutus validators, a transaction protocol, a Runtime, a family of SDKs and authoring tools, and a formal-methods project. The Cardano developer documentation presents it as a framework for creating, testing, and deploying financial smart contracts, with Marlowe itself plus Haskell, JavaScript, TypeScript, Blockly, Playground, Runtime/APIs, CLI tooling, and starter tooling. citeturn0search0

The language's basic algebra remains deliberately tiny:

```text
Contract =
    Close
  | Pay Account Payee Token Value Contract
  | If Observation Contract Contract
  | When [Case] Timeout Contract
  | Let ValueId Value Contract
  | Assert Observation Contract
```

with inputs built principally around deposits, choices, and notifications. The specification models parties as addresses or roles, supports internal accounts and token-denominated values, evaluates observations, and processes contracts by repeatedly reducing them until quiescence before applying external inputs. citeturn8view0turn12view2

This continuation-passing structure is not accidental. The language contains no ordinary recursive functions or looping construct, and each contract constructor is consumed as execution progresses. This structural restriction underlies termination arguments and enables calculations involving maximum contract lifetime and transaction-count bounds. The V1 specification also makes clear how limited its ordinary data universe is: values are primarily integer-oriented, observations Boolean, and Core lacks user-defined records, tuples, strings, arbitrary algebraic data types, or general functions. citeturn8view0turn10view0

Marlowe already contains an embryonic version of the “rich source → small core” idea. The specification distinguishes Core from an Extended representation, with Extended constructs compiled away before execution. The documented V1 Extended layer primarily provides template parameters rather than a complete typed language, and its semantics are not the centerpiece of the formal Core specification. That is a strong precedent for making the boundary explicit in V2 rather than extending Core every time authors need another abstraction. citeturn8view0

The lifecycle can be reconstructed approximately as:

```text
Financial intent / template
           |
           v
Marlowe source / TypeScript / Blockly / JSON
           |
           v
link / instantiate / analyze / simulate
           |
           v
Core contract + state + continuation DAG
           |
           v
Runtime transaction planning
           |
           v
wallet approval and signatures
           |
           v
Cardano transaction
           |
           v
Marlowe semantics validator
           |
      +----+------+
      |           |
 continuation    payouts
 / new state        |
      |             v
      +------> payout validator
                     |
                     v
                 recipient
```

The Runtime architecture is important because Marlowe is not merely “a validator.” The documented Runtime separates chain synchronization/indexing, Marlowe synchronization/indexing, and transaction construction. It follows Cardano-node state and exposes protocols/APIs through which clients can discover and progress contracts. Wallet interaction can occur through browser wallet standards or more traditional transaction-signing workflows. citeturn13view2

Large contracts are handled partly through **Merkleization**. Rather than placing every future continuation into the active on-chain datum, continuations can be represented by hashes and provided when needed. This reduces immediate on-chain size but moves part of the problem into content-addressed continuation availability. Marlowe Runtime can store continuations, but clients involved in producing Merkleized contracts are responsible for ensuring the hash-to-continuation mapping remains available. citeturn2search1

This has an important architectural consequence for V2: **Merkleization changes storage placement; it does not eliminate the data-availability problem.** A contract whose future branch is known only by hash remains semantically meaningful, but transaction progression depends on somebody recovering the corresponding continuation. The continuation store therefore belongs in Marlowe's security and availability model, even though it need not be trusted for integrity because the hash can authenticate the content. citeturn2search1

Marlowe's Object Bundle format addresses another Core limitation. Core has little named abstraction beyond `Let`/`UseValue`, so repeated structures create duplication. Object bundles let authors define and reference reusable objects. However, current bundle construction is not inherently type-safe: type mismatches and unresolved references are detected at linking. Its dependency ordering also prevents cycles. This is strong evidence that reusable abstraction belongs above Core, but V2 should move those errors from link time into a proper typed authoring environment. citeturn13view0

The repository landscape confirms that Marlowe has become a platform rather than one monolith. As of September 2, 2026, the Marlowe GitHub organization lists **36 repositories** and identifies the project as community-maintained under the Marlowe Language Community Interest Company. Current organization activity shows recent work in repositories including `marlowe-plutus` and `marlowe-oracle-protocol`, while several older components have quieter histories or have been archived. citeturn0search4

A useful status reconstruction is:

| Repository/component | Evidence-backed interpretation |
|---|---|
| `marlowe` | **V1 language/specification authority in practice.** Contains Haskell semantics, Isabelle material, specification tests and related research; its README still carries historical transition language, demonstrating documentation drift. citeturn19view0 |
| `marlowe-cardano` | **Legacy integrated Cardano implementation and major assurance corpus.** Its test report is one of the most valuable records of validator assumptions and failure modes. citeturn12view2 |
| `marlowe-plutus` | **Active split-out on-chain component.** Contains Marlowe semantics/payout validator work and has recent 2026 organization activity. citeturn7view0turn0search4 |
| `marlowe-agda` | **Experimental formal/executable implementation**, explicitly described as work in progress rather than established normative semantics. citeturn7view1 |
| `marlowe-runtime-ng` | **Experimental seed**, not a demonstrated replacement Runtime: its public repository showed only one commit and no releases at inspection time. citeturn7view2 |
| `marlowe-ts-sdk` | **Principal application-facing JavaScript/TypeScript integration layer.** It provides a family of TS/JS libraries and documents compatibility with Runtime releases and CIP-30 wallets, but also contains stale transition-era prose. citeturn19view1 |
| `marlowe-playground` | **Mature historical authoring/simulation frontend**, including symbolic and visual tooling, but architecturally rooted in the older platform generation. citeturn20view1 |
| `MIPs` | **Defined but lightly used technical-governance process.** The public index currently contains no “Reviewed Proposals”; MIP-1 is Proposed, MIP-2 reserved for versioning, and MIP-3 Universal Validator is Draft. citeturn22search0 |
| `marlowe-oracle-protocol` | **Active protocol-design effort**, with a developing CIP document rather than evidence of a finished protocol standard. citeturn22search1 |
| `actus-core` | **Financial-domain integration asset**, important for standardized financial-contract generation. The broader ACTUS project formalizes machine-readable contract algorithms and reference testbeds. citeturn26search0turn26search1 |
| `real-world-marlowe` | **Archived example/application corpus**, so it should not be mistaken for an actively maintained production application suite. citeturn0search4 |

There is also a clear **versioning/documentation problem**. The official versioning material still describes an older pre-regular-cadence ecosystem and historical Runtime/CLI compatibility information, while subsequent official material refers to Runtime 1.0.0 and the repositories have since moved into community stewardship. Version numbers in repositories, validator versions, Runtime API versions, language versions, and product names cannot currently be assumed to denote the same layer. citeturn13view1turn12view0turn0search3

That leads to the first foundational V2 requirement:

> **Marlowe must separate language version, Core semantics version, serialized-schema version, validator-set version, Runtime API version, SDK version, and package version.**

A contract should be able to declare something conceptually like:

```text
language        = "marlowe-surface/2.0"
core            = "marlowe-core/2.0"
serialization   = "marlowe-cbor/2"
validator-set   = "<registry-id + script hashes>"
runtime-api     = "2.x"
network         = "cardano-mainnet"
```

and tooling should reject ambiguous combinations before value is committed.

The Cardano execution environment makes that separation increasingly important. Current Cardano documentation describes transaction size and execution budgets as governance-controlled protocol parameters rather than eternal constants, and current platform documentation includes Plutus V3 capabilities introduced after the Plutus V2 generation used in the major Marlowe validator test report. Existing validator audit evidence therefore cannot automatically be generalized to every future validator build or ledger era. citeturn11search2turn12view2


## Assurance reality and proof gaps

Marlowe's formal-methods work is genuinely valuable. It should be the center of modernization, not marketing decoration.

The V1 specification formalizes the Core semantics in Isabelle and gives machine-checked or formalized results covering several structural and accounting properties. The historical implementation paper also describes the rationale for running a common Marlowe interpreter validator rather than compiling each financial agreement to a separate validator: keeping the executable system close to one semantics makes review, testing, and assurance easier. citeturn8view0turn8view1

The strongest properties can be summarized this way:

| Property | What the existing evidence supports | Qualification for V2 |
|---|---|---|
| Termination | Reduction is justified using a decreasing bound; Core contains no unrestricted loops/recursion. citeturn8view0 | Preserve structurally. Any iterator must have a static bound participating in the termination proof. |
| Maximum lifetime | Contract structure and finite timeouts permit a maximum-time calculation. citeturn8view0 | Preserve, but distinguish semantic lifetime from a practical guarantee that somebody can submit the final transaction. |
| Positive internal accounts | Semantics properties assume valid state/account conditions. citeturn12view2 | Make initial-state validity machine-enforced rather than an external assumption. |
| Conservation of value | Formal semantics provide money-preservation results around valid transitions. citeturn8view0turn10view1 | Extend theorem statement to assets, minting boundaries and external effects explicitly. |
| Reduction to quiescence | Formalized; repeated reduction after reaching quiescence is idempotent. citeturn8view0turn10view2 | Preserve as a Core invariant. |
| Input splitting | The specification establishes an equivalence property for processing grouped inputs versus suitable sequences of individual-input transactions. citeturn8view0turn10view2 | Revisit for `WhenAll`, where transaction-level atomicity becomes deliberately semantically significant. |
| Closure/no residual contract funds | The semantics support closure-related results. citeturn8view0 | Never translate this into an unconditional “assets can never be locked on Cardano” claim. |
| Transaction-count bound | Finite contract structure supports an upper bound on semantic transactions. citeturn8view0turn12view2 | Add **transaction resource bounds**, not just counts. |
| Authorization correctness | Role/address behavior exists operationally, but the major formal theorem family does not establish correctness of arbitrary role-token monetary policies. citeturn12view2 | Add explicit authorization assumptions/capabilities. |
| Cross-implementation equivalence | Tests exist, but the reviewed evidence does not establish a single machine-checked equivalence theorem covering Haskell, Plutus, TypeScript, PureScript and Agda. citeturn12view2turn7view1 | Create mandatory conformance vectors and differential-testing gates. |

One particularly valuable example of why Marlowe needs theorem-grade language around its claims appears inside the V1 specification itself. In the section discussing contracts always closing, the source includes a FIXME questioning whether the theorem actually proves the surrounding narrative. That is not evidence that Marlowe is unsafe; it is evidence that **the prose claim and formal statement were not perfectly aligned**, which is exactly the kind of gap V2 should eliminate before release. citeturn8view0

The validator test report is even more consequential. It records roughly two hundred property-based tests plus on-chain tests and compares the abstract semantics with the Cardano realization. It explicitly identifies two broad risk classes: invalid initial states can produce behavior outside the assumptions of the semantic model, and transactions accepted by the Marlowe semantics may nevertheless be rejected by Cardano because of protocol limitations. citeturn12view2

The invalid-state issue is especially serious. The test report documents cases involving nonpositive account state, duplicate map entries, malformed role-related state, time-boundary behavior and other preconditions that `computeTransaction` itself does not necessarily enforce. Some malformed initial states can make a contract permanently unexecutable. citeturn12view2

That should lead to a simple V2 rule:

> **No V2 contract creation transaction should be buildable unless the complete initial contract/state pair passes a normative well-formedness checker.**

The checker should not merely be a warning produced by a friendly UI. The same predicate should exist:

1. in the formal specification;
2. in the compiler/reference interpreter;
3. in SDKs;
4. in Runtime transaction construction;
5. in conformance vectors;
6. and, where economically feasible, at the validator boundary.

The second problem is ledger feasibility. The Marlowe on-chain limitations documentation has historically warned that transaction size and execution limits can lock contracts if a future required transition cannot fit Cardano's constraints. It also notes the effect of wallet UTxO selection on transaction size and explains why Merkleization reduces contract size without solving every state-size problem. citeturn12view1

Current Cardano transaction-building documentation continues to describe a maximum transaction size around **16 KB**, but—more importantly—it emphasizes that such parameters are protocol/governance controlled. Therefore V2 must not encode a statement such as “a Marlowe contract may have N accounts” as a permanent language rule derived from an old benchmark. It must evaluate the contract against a **protocol-parameter profile** and retain sufficient headroom for realistic wallet inputs and outputs. citeturn11search1turn11search2

The 2024 Marlowe optimization work illustrates both the progress and the limitation. Validator/compiler improvements and `AsData` representation changes materially reduced execution memory and validator size in measured workloads, with the official report describing typical combined improvements around the order of 50% in its benchmark set. But the same analysis explicitly notes that Marlowe places no semantic upper bound on every arithmetic or bookkeeping computation inside one transaction merely by virtue of being finite; complicated transitions can still run into Cardano limits. citeturn12view0

That distinction can be formalized as an **assurance ladder**:

| Level | Question | V1 situation | V2 requirement |
|---|---|---|---|
| Semantic | Does the transition exist and terminate? | Strong | Preserve/formalize |
| State validity | Does state satisfy semantic invariants? | Assumed in important places | Enforce |
| Validator | Does the validator implement the semantics? | Extensively tested, partially bridged | Proof/conformance gate |
| Ledger | Can the transition fit current ledger limits? | Not guaranteed | Static/resource analysis |
| Construction | Can a realistic wallet construct the tx? | Runtime-dependent | Preflight with wallet profile |
| Continuation | Is required off-chain data recoverable? | External availability assumption | Availability manifest/protocol |
| Authorization | Can the intended party legitimately act? | Role-token policy assumptions | Explicit capability manifest |
| External data | Is oracle data authentic/fresh? | Mostly application/protocol responsibility | Typed oracle boundary |
| Liveness | Will required actors/infrastructure participate? | Not guaranteed | Surface as dependency, never promise it |

This is the single most important semantic modernization I recommend: **V2 should make assumptions first-class output.**

A compiler should not merely produce:

```text
contract.marlowe
```

It should produce an assurance manifest such as:

```text
contract.core
contract.hash
contract.bounds.json
contract.assumptions.json
contract.analysis.json
contract.source-map
contract.dependencies.lock
validator-set.json
```

The manifest might state, for example:

```text
maximum_semantic_steps: 84
maximum_timeout: 2027-12-31T00:00:00Z
requires_continuations: 17
required_role_policies: [...]
oracle_dependencies: [...]
external_validator_dependencies: []
estimated_max_state_bytes: ...
ledger_profile: cardano-mainnet@protocol-parameters-hash
resource_status: PASS_WITH_HEADROOM
```

This would convert Marlowe's strongest properties from documentation claims into **machine-consumable artifacts**.

For the normative formalization, I would **retain Isabelle as the normative V2.0 proof authority while developing Agda as an executable secondary semantics**, not abruptly switch proof assistants. The current Marlowe repository already contains the established Isabelle proof corpus, whereas `marlowe-agda` is explicitly experimental/work-in-progress. A previous Catalyst V2 proposal did propose prototyping V2 changes in Agda, which is useful and should continue, but that is not the same as evidence that Agda is already the specification of record. citeturn19view0turn7view1turn16search0

A sensible V2 assurance chain is therefore:

```text
Normative mathematical semantics
             |
          Isabelle
             |
        theorem corpus
             |
       reference vectors
        /      |       \
       /       |        \
 Haskell   TypeScript   Agda
      \        |         /
       \       |        /
     differential conformance
              |
         Core 2 compiler
              |
     Plutus validator build
              |
      on-chain conformance
```

A future migration of the normative model to Agda or Lean should occur only if it buys demonstrable executable extraction, proof maintainability, and contributor capacity. Reproving the existing corpus in a different prover merely for fashion would consume scarce maintenance budget without improving contracts.


## V2 language and architecture

The official V2 process provides a strong starting point. The April 28, 2026 report describes four community sessions and separates proposals into essential V2.0, desirable V2.1, and later possibilities. Its V2.0 priorities are to simplify `When` so timeout closes rather than accepting an arbitrary timeout continuation, generalize waiting to sets of actions, and implement state compression. Bounded iteration is identified as a V2.1 candidate because repetitive agreements such as recurring interest payments currently need to be expanded explicitly. citeturn3view1

An earlier public Catalyst proposal reveals the broader design space considered by the V2 effort: bounded loops, cryptographic operations, time arithmetic, token-minting sublanguages, better handling of large participant sets, and finer-grained value kinds/types. That proposal was not approved in Fund 13, so its feature list must not be confused with a funded or shipped specification; it is useful as design evidence. citeturn16search0

Subsequent 2026 proposal material adds concrete ideas including `WhenAll`, state compression and `EntryDeposit`, with `EntryDeposit` intended to reduce or replace some of the separate machinery currently used to allow dynamic/open role acquisition. Earlier open-role discussion documents why this is attractive: the existing design brings an additional Plutus script, thread-token/monetary-policy considerations, and nontrivial security assumptions. citeturn16search3turn18search0turn16search1

My disposition is:

| Proposal | Recommendation | Reason |
|---|---|---|
| `When` always closes at timeout | **V2.0 — retain** | Removes a source of control-flow complexity and makes “wait until deadline, otherwise terminate” the default semantics. This aligns with the official essential-V2 category. citeturn3view1 |
| `WhenAll` / all-of action set | **V2.0 — retain but formalize narrowly** | Useful for atomic multi-party synchronization, but it changes assumptions behind input-splitting properties and therefore needs new proofs. citeturn3view1turn10view2 |
| Any-of action sets | **Represent through ordinary cases initially** | Existing `When` already expresses alternatives; adding another primitive offers less value than `WhenAll`. |
| Threshold `k-of-n` actions | **V2.1 or library elaboration** | Semantics around duplicate actors, ordering, timeout and authorization need more design. |
| State compression | **V2.0 — high priority** | Directly targets Cardano execution-memory/state pressure identified by V2 and historical limit analysis. citeturn3view1turn12view1 |
| Better lazy/large close | **V2.0 implementation investigation; V2.1 semantics if necessary** | High participant count is an operational risk, but an implementation optimization is preferable to changing semantics unless unavoidable. citeturn16search0turn12view1 |
| Strong value kinds/types | **V2.0 surface language** | Prevents classes of unit/token errors without expanding Core; official V2 work already identified finer typing as a safety direction. citeturn16search0 |
| Bounded iteration | **V2.1** | Valuable, but it touches termination proofs, resource analysis, serialization and validator cost. citeturn3view1 |
| `EntryDeposit` | **V2.1** | Promising simplification of dynamic entry, but authorization and monetary-policy properties deserve a dedicated proof/audit cycle. citeturn16search1turn18search0 |
| Time/duration arithmetic | **Surface language/library first** | Business schedules can elaborate to absolute Core timeouts while avoiding timezone/calendar complexity in consensus semantics. |
| Cryptographic primitives | **V2.1 only where required by explicit protocols** | Keep primitive set small; do not make Core a generic cryptographic application language. |
| Oracle protocol | **External typed protocol boundary** | The oracle repository is itself an evolving CIP effort, reinforcing the value of a standardized interface rather than embedding oracle trust into Core. citeturn22search1 |
| Token minting | **Outside Core** | Minting policy correctness is a separate security domain; embedding arbitrary policy behavior would invalidate simple Marlowe assurance claims. |
| Arbitrary validator invocation | **Reject inside Core** | General composability should use an explicit transaction-capability layer whose external scripts do not inherit Marlowe guarantees. |
| Hydra/L2 backend | **Research track** | Useful for certain known-participant workflows, but Hydra's channel model has different liveness/participation assumptions and should not define Core 2. citeturn11search4 |
| General recursion / unbounded loops | **Reject** | Would directly undermine the structural termination/resource thesis on which Marlowe differentiates itself. citeturn8view0 |

The core design I recommend is deliberately conservative. Conceptually:

```text
CoreContract :=
    Close
  | Pay(...)
  | If(...)
  | WhenOne(cases, deadline)
  | WhenAll(actions, deadline, continuation)
  | Let(...)
  | Assert(...)
```

The notable V2.0 semantic changes would be that the wait primitive has **no arbitrary timeout continuation**—expiration means closure—and an explicitly atomic all-of-actions form exists. Everything else remains recognizably Marlowe.

The richer source language can look completely different while compiling into that core:

```text
module FixedRateLoan

type USD
type Principal = Amount<USD>
type Rate      = Ratio
type Payment   = Amount<USD>

contract loan(
  lender: Party,
  borrower: Party,
  principal: Principal,
  annualRate: Rate,
  start: Timestamp,
  maturity: Timestamp
) {
  require principal > 0 USD
  require start < maturity

  borrower deposits principal to lender
    before start + 3 days
    else close

  let schedule =
    monthly_schedule(start, maturity)

  for paymentDate in schedule bounded_by 36 {
    borrower deposits
      installment(principal, annualRate, paymentDate)
      before paymentDate + 5 days
      else close
  }
}
```

In V2.0, this loop construct could be **compile-time elaboration** only when expansion stays within a configured resource envelope. In V2.1, a compact bounded iterator could be considered as a Core construct if—and only if—the semantics prove termination and a static analyzer can calculate a safe worst-case transition/resource bound.

The surface type system should introduce distinctions that finance users actually care about:

```text
Amount<Token>
NonNegativeAmount<Token>
Ratio
Percentage
Timestamp
Duration
Party
Role<Policy>
Choice<T>
OracleValue<T, Unit, Source>
Hash<T>
Credential
```

An operation such as:

```text
100 USD + 50 EUR
```

should be a compile-time error, while:

```text
100 USD * 5 Percent
```

should have a well-defined amount result. Move's phantom-type examples demonstrate the general value of making identical underlying integers represent distinct asset types, while ACTUS demonstrates how much financial modeling benefits from standardized terms, schedules, contract types, and explicit time-consistency rules. citeturn25search3turn26search0turn26search18

This does **not** mean adopting Move's general programming model. Move supports modules, generics and looping control flow suitable for general smart contracts; Marlowe should borrow its type distinctions and module/package discipline while retaining a much more restrictive execution target. citeturn25search9turn25search18

Likewise, ACTUS is better viewed as an important **financial-domain library/source generator** than as a replacement Core semantics. ACTUS distinguishes standardized financial contract terms from algorithms that produce contract events/cash flows and provides reference testbeds containing terms and expected schedules. That makes it a natural source of Marlowe packages such as loans, swaps and bonds, while Marlowe remains responsible for executable settlement state and authorization on Cardano. citeturn26search4turn26search1

A package could therefore look conceptually like:

```text
import actus.loan@3
import marlowe.oracle.rate@1

contract sofrLoan =
  actus.loan.from_terms({
    principal: 1_000_000 USD,
    rate: SOFR + 250 bps,
    maturity: 2030-06-30,
    ...
  })
```

with the compiler emitting the exact ACTUS version, source package hash, generated Core contract and oracle dependency into the assurance manifest.

The comparative evidence favors this layered architecture.

**Aiken** shows that a Cardano-specific language can give developers static types, modern diagnostics, integrated formatting, editor tooling and tests without pretending to be a general-purpose host language. Its tooling explicitly emphasizes modular design and interoperability. Marlowe should aim for the same level of language ergonomics, with a narrower semantic target. citeturn25search2turn25search8

**Simplicity** is particularly relevant to Core design because its current documentation emphasizes static analyzability and statically bounded resource cost, and the language's foundations were designed around formal semantics rather than unrestricted execution. Marlowe should go one step beyond its present transaction-count bound and attempt a comparable **resource envelope** for each contract under a parameterized Cardano cost model. citeturn24search10turn24search18

**Scilla** provides a useful model for extension boundaries: it explicitly separates pure computation, state manipulation and external communication, with transitions treated as atomic units. Marlowe's own financial-transition model could benefit from a similarly explicit separation between Core state transitions and external effects such as oracle attestations, minting policies and other validators. citeturn24search5

**Daml/Canton** demonstrates the usefulness of treating parties, authority, templates, choices, packages and APIs as first-class parts of a financial-contract developer platform rather than treating a contract as an isolated script. Current Daml documentation includes contract packages, parties/authority, choices, interfaces, testing and separate JSON APIs. Marlowe should borrow the notion of a coherent financial developer platform while retaining its UTxO/Cardano-specific execution model. citeturn23search4turn23search1

The decision scorecard is:

| Architecture | Assurance | Ledger predictability | Expressiveness | DX | Migration | Runtime/tooling | Adoption | Maintainability | Interop | Weighted |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| V1 stabilization | 4.5 | 3.0 | 1.5 | 2.5 | 5.0 | 3.0 | 2.0 | 4.0 | 2.5 | **3.24** |
| Conservative V2 | 4.0 | 3.5 | 3.0 | 3.2 | 4.0 | 3.5 | 2.8 | 3.3 | 3.0 | **3.45** |
| **Verified Core 2 + typed surface** | **4.7** | **4.0** | **4.2** | **4.5** | **3.5** | **4.2** | **4.0** | **3.2** | **3.8** | **4.11** |
| Composable protocol DSL | 3.5 | 3.2 | 4.8 | 3.8 | 2.5 | 3.5 | 4.0 | 2.5 | 5.0 | **3.61** |
| Portable/L2-first | 3.2 | 4.2 | 3.8 | 2.8 | 2.5 | 2.8 | 2.5 | 2.0 | 4.3 | **3.20** |

The main sensitivity is straightforward: if evidence showed that essentially all future Marlowe demand requires arbitrary validator composition, the composable-protocol option would become stronger. Conversely, if the community cannot sustain a compiler/LSP/specification stack, conservative V2 becomes the safer option. Given the existing official V2 direction, active TypeScript SDK, formal assets, and obvious abstraction/type pressure, Core 2 plus a typed surface currently gives the strongest risk-adjusted result. citeturn3view1turn19view1turn19view0


## Runtime, tooling, security, and empirical validation

The Runtime should be redesigned around a simple security principle:

> **Runtime is a transaction planner and indexer, not an authority whose output users must trust.**

The existing Runtime already separates transaction construction from chain synchronization and indexing. V2 should make that separation a formal client contract: every Runtime-proposed transaction should be independently inspectable and locally checked by the SDK before wallet signing. citeturn13view2

A V2 client should verify at least:

```text
expected network
expected Marlowe validator set
expected contract input
expected current datum/state hash
expected Marlowe inputs
expected semantic transition
expected continuing output
expected continuation hash
expected payments and payout outputs
expected role authorization
expected validity interval
expected oracle attestations
expected external capabilities/scripts
expected fees and collateral envelope
unexpected outputs / minting / withdrawals = reject
```

That transforms a compromised Runtime from “potential signer deception” into “malicious proposal that the local verifier rejects,” assuming the wallet presents and signs the locally checked transaction.

`marlowe-runtime-ng` should not presently be considered the production successor merely because its name says “ng.” Its inspected public repository is extremely small and has no releases, whereas the existing Runtime architecture is far more substantial. The right V2 process is to define the security/API contract first and allow a lightweight runtime implementation to prove conformance against it. citeturn7view2turn13view2

The runtime decomposition I recommend is:

```text
                   Marlowe Client
                        |
        +---------------+---------------+
        |               |               |
        v               v               v
 Local semantics   Tx verifier     Wallet signer
        ^               ^
        |               |
        +------- Tx Plan API
                        |
                 Runtime Planner
                  /     |      \
                 /      |       \
         chain provider |    continuation store
                        |
                 Marlowe index
```

The planner can be replaceable. The **semantics checker and transaction verifier belong in the client library** and should be reusable without running the complete Runtime stack.

The authoring system is at least as important. Marlowe currently has an unusually broad collection of representations—text, JSON, TypeScript construction, visual Blockly/Playground tooling, Haskell and others—but that breadth increases the risk that no representation feels canonical. The current Object Bundle linker catches type errors relatively late, while Aiken demonstrates what modern Cardano developers now expect from diagnostics, automatic formatting, editor integration and language-server support. citeturn13view0turn25search8

V2 should therefore ship one canonical source toolchain:

```text
marlowe fmt
marlowe check
marlowe build
marlowe test
marlowe simulate
marlowe analyze
marlowe cost
marlowe diff
marlowe migrate
marlowe verify-tx
marlowe deploy
```

with an LSP supplying type errors, resource diagnostics, missing timeout warnings, continuation information, source-level execution traces and generated contract documentation.

Visual programming should become another projection of the same typed AST, not a separate language. Text → AST → visual and visual → AST → formatted text should round-trip with stable semantics. A contract hash should be calculated over a canonical Core serialization rather than editor representation.

The static analyzer should move from “find semantic warning paths” toward a broader deployment analysis:

```text
Semantic analysis
  + state well-formedness
  + role-policy analysis
  + continuation completeness
  + ledger parameter profile
  + transaction-size envelope
  + execution-unit envelope
  + wallet UTxO stress profiles
  + payout fan-out
  + close fan-out
  + oracle assumptions
  + external script manifest
  + minimum-value requirements
  = deployment readiness report
```

This responds directly to the documented mismatch between semantic validity and Cardano resource feasibility. citeturn12view2turn12view1turn11search1

Marlowe should also add **source maps from the typed surface language to Core constructors and on-chain state**. When an explorer shows that a contract is waiting for a particular deposit, a user should see the corresponding source line, business label, original package/template field, expected party, amount, deadline and fallback—not a raw JSON subtree.

For AI-assisted authoring, the trust boundary should be uncompromising:

```text
Natural-language request
        |
       LLM
        |
   candidate source
        |
   parser + type checker
        |
      elaborator
        |
   static analysis
        |
 simulator / tests
        |
 resource checker
        |
 human transaction review
        |
      signing
```

The AI should have **zero special authority**. Its explanation of a contract is informational; the compiler-generated semantics, state transition, proof/conformance artifacts and wallet transaction are authoritative.

The highest-priority security threats follow directly from the current evidence:

| Threat | V2 response |
|---|---|
| Invalid initial state | Normative `WellFormed(state, contract)` predicate enforced during creation and validator conformance. Existing test evidence shows why this matters. citeturn12view2 |
| Ledger-limit lock | Predeployment resource envelope plus protocol-parameter versioning and execution headroom. citeturn12view1turn11search2 |
| Role-token policy weakness | Authorization manifest naming policy IDs and assumptions; provide audited standard policies without pretending arbitrary policies are safe. citeturn12view2 |
| Missing Merkle continuation | Replicated content-addressed storage, complete continuation manifest, local cache and predeployment availability check. citeturn2search1 |
| Runtime transaction substitution | Client-side semantic and transaction verification. |
| Oracle replay/staleness | Typed oracle datum with source, unit, observed time, expiry, attestation and replay domain. The active oracle-protocol effort should define the external standard. citeturn22search1 |
| External validator compromise | Explicit capability declaration; external scripts never inherit “Marlowe verified” labeling. |
| Version confusion | Hash-bound validator registry and independent language/schema/API versions. The current documentation/repository version skew demonstrates the need. citeturn13view1turn19view0 |
| Package supply chain | Content-addressed dependencies, lockfile, source provenance and reproducible compiler/validator builds. |
| Governance failure | MIP ownership, security review requirements, versioning MIP and specification authority. Current MIP inventory is too thin for a major language release. citeturn22search0 |

The empirical portion of this research can establish several limits from existing primary measurements, but it should **not fabricate a fresh mainnet benchmark**. No Cardano node or complete V2 prototype was executed in this research session, so source size, fees and execution-unit numbers for hypothetical V2 contracts are not presented as measured facts.

What existing evidence already tells us is significant. Current Cardano documentation still places transaction size around the 16 KB scale and makes transaction/execution constraints governance-controlled. Historical Marlowe benchmarking shows that compiler/data-representation improvements can produce substantial memory savings, but complicated transitions can remain resource-bound. Marlowe documentation also explicitly warns about state, output and wallet-input effects on transaction executability. citeturn11search1turn11search2turn12view0turn12view1

A V2 benchmark suite should therefore become a **release artifact**, not a one-time research paper. At minimum it should continuously build and execute escrow, swap, fixed-rate loan, floating-rate loan, bond, recurring payments, option, auction, oracle CFD, milestone treasury agreement, multi-party distribution, ACTUS-derived agreement, large-participant close, and open-entry workflows against preview/pre-production protocol profiles.

The most revealing benchmark will likely be recurring payments. The official V2 report explicitly identifies repeated interest/payment structures as a case where V1's no-loop architecture forces expansion and motivates bounded iteration. V2 should benchmark 10, 100 and 1,000-period schedules in at least three forms: V1 unrolling, surface-level expansion into Core 2, and compact bounded iteration. citeturn3view1

For each benchmark, the release dashboard should report:

```text
surface source bytes
Core AST nodes
serialized Core bytes
Merkleized continuation count
maximum active state bytes
maximum accounts
maximum payouts
maximum transaction bytes
maximum execution CPU/memory
minimum and worst tested wallet inputs
transaction count
fees
analysis runtime
continuation-storage footprint
required signers
maximum lifetime
resource headroom
```

The important output is not “V2 is 37% faster.” It is:

> **For every deployable Marlowe V2 contract, can we demonstrate that all reachable transitions remain executable within a declared Cardano parameter profile plus a conservative safety margin?**

That would materially strengthen Marlowe beyond the semantic guarantees V1 already provides.


## Governance, migration, and adoption

Governance is now part of the language architecture. IOG's transition material states that active development moved away from direct IOG stewardship toward an independent community/nonprofit structure, and the current organization identifies Marlowe Language CIC as the community-maintenance vehicle. citeturn0search3turn0search4

The positive side is that Marlowe is no longer dependent on being a product line inside one company. The risk is that a formally verified financial language has unusually high maintenance requirements: Core semantics, theorem proofs, validator builds, ledger compatibility, Runtime infrastructure, TypeScript packages, web tooling, audits and documentation can drift independently.

The MIP mechanism is intended to govern language evolution, and the main `marlowe` repository explicitly directs semantic changes through that process. However, as of the current inspection the MIP inventory remains sparse: no reviewed proposals are listed; MIP-1 is Proposed, MIP-2 is reserved for Versioning, and Universal Validator is still Draft. A major V2 release is therefore larger than the governance process has yet demonstrated it can absorb. citeturn22search0turn19view0

Before freezing V2 semantics, the community should ratify at least:

```text
MIP: Version and compatibility model
MIP: Marlowe Core 2 semantics
MIP: Canonical serialization and hashing
MIP: Typed surface language / elaboration boundary
MIP: Validator-set registry
MIP: Runtime transaction-plan protocol
MIP: V1-to-V2 migration semantics
MIP: Conformance test suite
MIP: Oracle / external capability boundary
```

These proposals need named maintainers and implementation artifacts. The MIP process itself already expects implementation evidence as proposals advance; V2 should enforce that rigor rather than approving prose-only language features. citeturn22search0

Repository consolidation should follow authority boundaries rather than historical accidents. A target arrangement could be:

```text
marlowe-spec
  Isabelle semantics, language schemas, conformance vectors

marlowe-compiler
  parser, types, elaborator, formatter, LSP, analyzer

marlowe-plutus
  audited Cardano validator implementations

marlowe-runtime
  transaction planner, chain adapter, index, continuation service

marlowe-sdk-ts
  local semantics, tx verifier, wallet/runtime client

marlowe-finance
  ACTUS integration and standardized financial packages

marlowe-protocols
  oracle and standardized external-capability protocols

marlowe-tools
  Playground, visual authoring, explorer integrations
```

Older repositories should be clearly marked **supported**, **compatibility-only**, **experimental**, **superseded**, or **archived**. Merely leaving an old README online with historical compatibility information creates security risk when validators and APIs are involved.

Migration must assume V1 contracts are immutable and may remain live for years. V2 should therefore **not** try to upgrade existing on-chain state in place.

The migration architecture should be:

```text
                   Marlowe platform
                         |
              +----------+----------+
              |                     |
             V1                    V2
              |                     |
       V1 validator set      V2 validator set
              |                     |
       V1 semantics lib      Core 2 semantics
              \                     /
               \                   /
                Runtime version router
                         |
                   common explorer
```

A V1 contract remains a V1 contract. New Runtime and explorer software can understand both.

A migration tool should classify a V1 source contract as:

**Equivalent translation.** Its timeout behavior already closes, it uses only constructs with straightforward Core 2 equivalents, and a semantics checker establishes trace equivalence.

**Policy-dependent translation.** For example, a V1 `When` has a non-`Close` timeout continuation. Because V2's proposed timeout behavior differs, a human must choose whether to preserve the original semantics through an elaborated structure or adopt V2 closure semantics.

**Representable but changed.** The contract can be rewritten using new action sets, roles or state mechanisms, but the behavior is intentionally different.

**Do not migrate.** The existing on-chain contract should simply finish under its immutable V1 validator.

Migration reports should show trace-level differences rather than only source diffs.

The adoption strategy should also change.

Early Marlowe positioning emphasized making financial smart contracts accessible to finance experts without requiring the full programming background demanded by general-purpose languages. The current platform, however, increasingly exposes TypeScript SDKs, Runtime APIs, wallets and application infrastructure, while the strongest differentiator remains analyzability and predictable financial behavior. citeturn8view0turn19view1

The primary V2 positioning should therefore become:

> **Marlowe is the high-assurance financial state-machine layer for applications that need finite, analyzable and auditable financial agreements on Cardano.**

The primary user should be a **DApp/fintech developer or product team**, not an assumed non-programmer manually writing blockchain code.

Financial-domain experts remain essential, but as template designers, reviewers and users of domain-level tooling. Blockly and visual tools should stay valuable for inspection and constrained authoring, but “no-code blockchain programming” should not be the strategic center.

That positioning gives Marlowe a clearer relationship with Aiken and Plutus. Aiken is a purpose-built Cardano validator language with modern types and developer experience; Marlowe does not need to beat it at general validator programming. The Marlowe organization itself describes Marlowe as complementary to more general Cardano contract technologies. citeturn25search2turn0search4

The choice becomes:

```text
Need arbitrary Cardano protocol logic?
        |
       Aiken / Plutus / other general validator tooling

Need a finite financial agreement with analyzable state,
timeouts, payments, roles and bounded behavior?
        |
       Marlowe

Need both?
        |
       Marlowe + explicit typed external capability boundary
```

ACTUS is strategically important under this product thesis. ACTUS standardizes machine-readable financial contract types, contractual terms and algorithms for producing contract events/cash flows; its official ecosystem includes reference testbeds for validating implementations. A strong ACTUS→Marlowe pipeline could make Marlowe useful for serious loans, bonds and derivatives without forcing users to manually construct huge Core trees. citeturn26search2turn26search1turn26search7

Public evidence does **not** currently justify claiming broad institutional Marlowe adoption. The official ecosystem establishes mainnet capability, community maintenance, active tooling and ongoing V2/oracle work, but the inspected primary sources do not expose a large, independently measurable corpus of major production institutions using Marlowe at scale. `real-world-marlowe` is archived, and public V2 work itself continues to focus on making the language more usable and scalable. That is an inference from the available public evidence, not proof that private or unpublicized adoption does not exist. citeturn0search4turn3view1

Consequently V2 funding should be gated by **pilot evidence**, not GitHub activity alone. Before a full V2.1 investment, the project should secure at least two non-toy applications that need features such as action synchronization, long schedules, ACTUS generation, dynamic participants or typed oracle data.


## Roadmap and release strategy

The next three years should be staged so that the project gets value even if later language work stops.

**The first 90 days should establish the specification and evidence baseline.** Freeze and publish an authoritative V1 compatibility corpus. Reconcile Haskell semantics, Isabelle, current Plutus validators, TypeScript representations and deployed validator registries. Create the language/validator/API version taxonomy. Ratify or substantially advance MIP-2 versioning. Define the Core 2 change set and a machine-readable well-formedness predicate. Build an initial cross-language semantics corpus. This work directly addresses the current mismatch among old documentation, repository versions, validator generations and evolving community ownership. citeturn13view1turn19view0turn22search0

**By six months, the project should have a V2 semantics prototype rather than only a design document.** Implement close-on-timeout `When`, the narrow `WhenAll` semantics and state compression in reference implementations. Extend Isabelle proofs for termination, quiescence, value preservation and transaction behavior. Prototype the same semantics in Agda. Add invalid-state rejection. Establish differential tests and benchmark V1 versus Core 2 against current Cardano protocol profiles. The existing V2 report identifies exactly these first semantic/scaling targets. citeturn3view1

**By twelve months, V2.0 should be releasable only if the complete vertical slice exists.** That means audited Plutus validators, canonical serialization, deterministic hashes, Runtime support, TypeScript local verification, the first typed surface compiler, formatter/LSP, migration tooling, explorer support, resource analysis, documentation and at least two realistic pilot applications. A language specification without a supported Runtime/SDK/tooling release should not be called Marlowe V2.0.

**During months twelve through twenty-four, V2.1 should attack expressiveness that genuinely needs new semantics.** Bounded iteration is the highest-value candidate because the official V2 process already identifies unrolled repeated payments as a concrete problem. `EntryDeposit`, richer action thresholds, selected cryptographic/oracle primitives and more sophisticated resource certification belong in this phase after V2.0's conformance infrastructure is operating. citeturn3view1turn18search0

**Hydra should remain a parallel experiment.** Current Hydra documentation describes a state-channel model with known participants and specific liveness/fallback considerations. That could be compelling for repeated bilateral or closed-group financial flows, but it is not a universal execution model for open Marlowe contracts. A loan-servicing or high-frequency settlement pilot would be more informative than designing Core 2 around Hydra in advance. citeturn11search4

Release gates should be strict:

| Gate | Requirement |
|---|---|
| Semantics | No known ambiguity in normative Core rules |
| Formal | Termination, quiescence, conservation and new `WhenAll` properties proved or given explicit proof status |
| State | All deployment paths enforce valid initial state |
| Conformance | Reference vectors pass all supported implementations |
| Serialization | Canonical and deterministic |
| Validator | Reproducible build and published script hashes |
| Ledger | Every benchmark passes current protocol profile with defined headroom |
| Runtime | Malicious-transaction rejection tests pass client verifier |
| Security | No unresolved critical/high findings |
| Migration | V1/V2 coexistence and translator tested |
| Documentation | Versioned alongside artifacts |
| Adoption | At least two non-toy V2 pilots |

The defining test for bounded iteration should not be “the interpreter terminates.” It should be:

```text
Can the compiler prove a finite bound?
Can it compute maximum lifetime?
Can it compute a transaction-count bound?
Can it compute a useful execution/resource envelope?
Can the analyzer explore or soundly abstract the loop?
Can the continuation representation remain practical?
Can the validator execute every reachable step?
```

If not, bounded iteration is not ready for Core.

Similarly, the defining test for external composability should be whether the trust boundary remains understandable:

```text
Marlowe guarantee
    applies here
        |
        v
+-------------------+
| Core transition   |
+-------------------+
          |
          | declared external capability
          v
+-------------------+
| Oracle / minting  |
| or other validator|
+-------------------+
          ^
          |
   separate assurance
```

A UI should visibly label that distinction. “Built with Marlowe” must never imply that arbitrary attached Aiken/Plutus validators received Marlowe's formal guarantees.


## Final recommendation

**Preserve.** Preserve Marlowe as a **finite financial-contract DSL with a small semantic kernel**, explicit waits and deadlines, continuation-style execution, internal-account/value semantics, deterministic reduction, analyzability and a universal or small set of heavily audited execution validators. Preserve formal semantics and strengthen the connection from mathematical model to deployment. The absence of unrestricted recursion and loops is an asset, not technical debt. citeturn8view0turn8view1

**Change in V2.0.** Introduce a clearly versioned Core 2 with close-on-timeout `When`, an atomic and narrowly specified `WhenAll`, state compression, normative state well-formedness, canonical serialization, explicit validator-set identification and machine-readable assurance manifests. Build a typed source language above Core 2 with modules, named definitions, `Amount<Token>`, ratios, percentages, timestamps, durations, typed roles, package imports, good diagnostics and source maps. The V2.0 semantic additions align closely with the official April 2026 priorities, while the typed surface layer solves usability problems without overloading Core. citeturn3view1turn16search0

**Develop for V2.1.** Develop truly bounded iteration, `EntryDeposit`/dynamic-entry semantics, richer multi-party thresholds, carefully selected cryptographic primitives and stronger resource certificates. Each must carry explicit proof obligations and Cardano execution benchmarks. Bounded iteration should proceed only after it can preserve termination, finite lifetime and analyzable resource bounds. citeturn3view1turn18search0

**Move outside the semantic core.** Put business-calendar logic, relative-time schedule generation, ACTUS templates, module systems, most user-defined data, package management, visual authoring, oracle aggregation, external identity logic, arbitrary minting policies, general validator composition and AI assistance outside Core. ACTUS in particular should become a first-class financial package/generation layer whose standardized contract terms and event algorithms compile into Marlowe agreements. citeturn26search4turn26search1

**Reject.** Reject unrestricted recursion, unbounded loops, arbitrary general-purpose contract calls from inside Core, implicit trust in Runtime, and the idea that every external validator participating in a transaction inherits Marlowe's safety claims. Also reject a near-term rewrite in a new proof assistant solely for modernization optics. The project already possesses meaningful Isabelle assets; those should be extended while Agda matures as a complementary executable specification. citeturn19view0turn7view1

**Prove and test.** Re-establish each Marlowe guarantee as a precise theorem with explicit assumptions. Repair the specification/prose mismatch around closure claims, formally incorporate state validity, prove new action-set behavior, maintain differential conformance across implementations, and add ledger-resource feasibility to deployment analysis. The validator test report demonstrates why semantic correctness alone is insufficient. citeturn8view0turn12view2

**Migrate.** Never mutate V1 contracts in place. Run V1 and V2 validator families concurrently. Give contracts explicit language/schema/validator versions. Provide a V1→V2 translator plus trace-equivalence checker and classify translations as equivalent, policy-dependent, intentionally changed or non-migratable. Contracts already deployed should normally finish under the validator whose semantics they began with.

**Build.** In the first 90 days, freeze authority/versioning/conformance. By six months, have executable and formally modeled Core 2 semantics. By twelve months, ship the complete vertical V2.0 platform—not merely syntax. During the following year, introduce bounded iteration and dynamic participation only after proof and benchmarking. Treat Hydra as a specialized execution-backend experiment for known-participant financial workflows rather than a requirement for the language. citeturn3view1turn11search4

**Govern.** Make specification authority explicit. Advance the currently sparse MIP process before freezing V2, starting with versioning. Give each normative artifact an owner. Publish reproducible validators, script hashes, dependency manifests and conformance results. Classify every repository as supported, experimental, compatibility-only, superseded or archived. The present MIP inventory and documented repository/version drift show that this is a release-engineering requirement, not administrative cleanup. citeturn22search0turn13view1turn0search4

**Validate adoption.** Position Marlowe V2 primarily as **the high-assurance financial state-machine layer for Cardano applications**, complementary to Aiken and Plutus rather than competitive with them as a general smart-contract language. Validate that proposition through production-shaped pilots in recurring lending, escrow/marketplace settlement, treasury/milestone payments, ACTUS-derived instruments and oracle-dependent financial agreements. Public sources establish an active community, mainnet-capable platform and continuing V2/oracle development, but they do not yet justify assuming broad institutional adoption; future investment should therefore be tied to usage evidence. citeturn0search0turn0search4turn22search1

The resulting architecture can be summarized in one diagram:

```text
                 DOMAIN AUTHORS / DEVELOPERS
                           |
                           v
              +--------------------------+
              | Marlowe Surface Language |
              | types · modules · ACTUS  |
              | packages · schedules     |
              +------------+-------------+
                           |
                    typed elaboration
                           |
            proofs / bounds / source maps
                           |
                           v
              +--------------------------+
              |      MARLOWE CORE 2      |
              | small · finite · formal  |
              +------------+-------------+
                           |
             normative transition semantics
                           |
             +-------------+-------------+
             |                           |
             v                           v
      Plutus validators             local verifier
             |                           |
             +-------------+-------------+
                           |
                    transaction plan
                           |
                           v
              +--------------------------+
              | Runtime / chain services |
              | index · plan · continue  |
              +------------+-------------+
                           |
                    client verifies
                           |
                           v
                         WALLET
                           |
                           v
                        CARDANO

External systems:
oracle · identity · minting · other validators · L2
                 |
        explicit capabilities
                 |
        NOT part of Core guarantee
```

That is the modernization direction most consistent with both Marlowe's existing formal assets and the actual weaknesses revealed by its own specifications, validator testing, ledger-limit experience and V2 design process: **do not make the trusted language much bigger; make the system around the trusted language dramatically better.**