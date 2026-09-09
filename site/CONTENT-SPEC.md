# Moriarty website — content specification

This is the authoritative source of substance for the website. Everything on the
site must trace to this document, and everything here traces to the repository.
Design agents must not invent protocol behavior, guarantees, numbers or syntax.
If something is needed and is not here, it does not go on the site.

Repository: `~/Moriarty`, branch `website`. Primary sources: `README.md`,
`ROADMAP.md`, `wiki/moriarty-architecture.md`, `wiki/defiformal-taxonomy.md`,
`wiki/formal-assurance.md`, `wiki/security.md`,
`deliverables/defi-language-design-2026-09-07/action-targets.csv`,
`docs/superpowers/specs/2026-09-06-r2b-outcome-intents-design.md`,
`docs/research/2026-09-06-intents-report-integration.md`,
`experiments/moriarty-language/spec/`.

---

## 1. What Moriarty is

Moriarty is a language and toolchain for **bounded financial contracts on
Midnight**. A developer describes financial state, permitted actions, payment
obligations and authorization rules in a domain-specific language. Those
descriptions compile to Compact, and every transaction carries a proof that its
execution — and the contract history it extends — satisfies the agreement.

The one-line framing for the site:

> **Financial meaning that survives compilation, proof and settlement.**

The problem it addresses: Compact and general smart-contract languages give you
circuits and state. They do not give you *which asset an amount denotes, how
interest rounds, when a payment becomes due, what a participant authorized, and
which obligations survive a transaction.* Those are the questions that decide
whether a repayment actually discharged the right debt to the right creditor
without erasing the remaining principal. Moriarty makes them part of the type
system and the operational semantics rather than part of the audit.

Moriarty is a new bounded financial-agreement language. It is not a renamed
Marlowe and not a general-purpose Compact dialect.

### Design ancestry (three named influences)

- **ACTUS** — the Algorithmic Contract Types Unified Standards. Supplies
  reference behavior for events, state transitions and cash flows: interest,
  principal repayment, maturity. Moriarty must reproduce the financial behavior
  including dates and rounding. Naming a contract type is not enough.
  Scope commitment: 277 ACTUS fixtures, 18 executable types, 32 source-backed
  taxonomy dispositions.
- **The DeFi corpus** — the project's catalogue of decentralized-finance
  behaviors: swaps, liquidity, lending, composition. 72 protocol rows across the
  historical 12-category set. Supplies implementation and conformance
  requirements; it is not a runtime service.
- **Marlowe** — a financial-contract DSL built so contract behavior can be
  analyzed before execution. Moriarty adopts that goal and builds its own
  authoring, proof and settlement architecture for Midnight.

---

## 2. The DeFi category model

This is the site's organizing spine. Two levels: **economic families** (what kind
of financial thing it is) and **facets** (orthogonal properties every instance
has). Product labels are not categories — the same protocol carries several
facets and can appear in more than one family.

### 2.1 Economic families

| ID | Family | Examples of what lands here |
|----|--------|------------------------------|
| **F1** | Exchange and price discovery | AMMs, order books, aggregators, routing |
| **F2** | Credit and collateralized debt | Lending markets, CDPs, debt-backed stablecoins, flash loans |
| **F3** | Derivatives | Perpetuals, futures, options, structured payoffs |
| **F4** | Consensus-position claims | Staking, liquid staking, restaking, slashing exposure |
| **F5** | Tokenized off-chain claims | RWAs, attested external claims, custodial receipts |
| **F6** | Delegated asset management | Vaults, yield strategies, ERC-4626 / ERC-7540 |
| **P**  | Prediction markets | Event-contingent claims, outcome-indexed positions |

These identifiers organize packages and evidence. **They are not Moriarty source
syntax.** The site must not imply the language has an `F2` keyword.

### 2.2 Mandatory facets

Every instance carries values on all eight. This is what makes the model a
classification rather than a list.

1. **Execution** — how the action gets performed (includes intents and solvers)
2. **Settlement** — how value actually moves (includes bridges)
3. **Custody** — who holds the asset during the operation
4. **Legal dependence** — what off-chain enforcement the claim relies on
5. **Collateral and solvency** — what backs the obligation
6. **Oracles** — what external evidence the behavior consumes
7. **Authorization and mandate** — who permitted what, and how far
8. **Price discovery** — where the number comes from

Two placements the site should call out explicitly because they are where most
taxonomies get it wrong:

- **Intents are an execution facet**, not a family. An intent is a way of
  authorizing an action, not a kind of financial product.
- **Bridges are a settlement or infrastructure facet**, and they split by
  **message-verified vs. custodial trust**. Those are different trust
  assumptions, and collapsing them is a category error the site should show.

### 2.3 The 24 action targets

The families are the human-facing map; the action targets are what the language
must actually execute. This is the real content — `DA01`–`DA24` from
`deliverables/defi-language-design-2026-09-07/action-targets.csv`. Each row has a
semantic requirement (what the action must mean) and a **distinguishing test**
(the case that separates a correct implementation from a plausible-looking wrong
one). The distinguishing tests are the most persuasive content on the site.

| ID | Family | Action | Semantic requirement | Distinguishing test |
|----|--------|--------|----------------------|---------------------|
| DA01 | F1 | swap exact input / exact output | asset-indexed exchange; fee and slippage accounting | rounding; reserve safety; net minimum |
| DA02 | F1 | provide / remove liquidity | share mint/burn; reserve contributions | proportional entitlement; donation and zero-supply boundaries |
| DA03 | F1 | open / adjust / close liquidity position | bounded position identity and range | fee allocation and finite tick/range traversal |
| DA04 | F2 | supply / redeem lending claims | claim shares and liquidity-constrained withdrawal | insufficient pool liquidity rejects without erasing claim |
| DA05 | F2 | post / release collateral | encumbrance and debt-dependent release | release cannot violate collateral rule |
| DA06 | F2 | borrow / accrue / repay | nominal debt distinct from transfers; rate/time arithmetic | partial repayment preserves principal and interest allocation |
| DA07 | F2 | liquidate / recognize default | authorized seizure; loss and residual debt allocation | partial liquidation and close factor; maturity before forfeiture |
| DA08 | F2 | flash borrow / repay atomically | atomic multi-leg settlement with fee | every loan repaid within the same atomic transaction |
| DA09 | F2 | refinance / novate / capitalize | workflow over old debt, new debt, collateral and signed liability authority | reject refinance without old-debt discharge; capitalization changes liability |
| DA10 | F2 | issue / burn debt-backed stablecoin | issuance authority plus debt and collateral | burn amount and released collateral obey the specified debt rule |
| DA11 | F3 | open / margin / fund / close derivative | position notional, margin and funding obligations | funding signs; insolvency; precise settlement convention |
| DA12 | F3, P | write / exercise / expire contingent claim | choice authority and exercise/payment dates | expiry does not erase an already exercised payment duty |
| DA13 | P | split / merge / resolve event claims | outcome-indexed claims; resolution evidence | no duplicate winning claim; invalid resolution rejects |
| DA14 | F4 | stake / account rewards / slash | share-rate or rebase accounting; loss allocation | the same nominal token balance can have changing entitlement |
| DA15 | F4 | request unstake / claim exit | pending exit identity; custody; delayed completion | an exit request is not immediate token delivery |
| DA16 | F5 | issue / redeem external claim | attested claim; custody and legal assumptions | missing external settlement evidence cannot discharge the duty |
| DA17 | F6 | deposit / mint / withdraw / redeem vault shares | asset/share conversions with method-specific rounding | fees, rounding direction, initial donation and zero shares |
| DA18 | F6 | request / fulfill / claim asynchronous redemption | Pending → Claimable → Claimed; residual request amount | partial claim and changed exchange rate; double claim rejects |
| DA19 | F6 | allocate / harvest / reinvest / unwind / rebalance | bounded workflow of trades, debt, shares and fees | losses and debt persist through unwind; liquidity shortage |
| DA20 | orthogonal | observe price / time / external event | typed observations with provenance, freshness and domain | stale or unauthorized evidence rejects |
| DA21 | orthogonal | authorize exact plan / refine outcome intent | gross debit; net receipt; recipients; calls; new liabilities | refund cannot restore gross capacity; fees count against the net goal |
| DA22 | orthogonal | change parameters / pause / migrate | bounded administrative action under fixed claim policy | cannot downgrade claims or reset work; affects existing positions |
| DA23 | orthogonal | send / receive / refund pending message | bounded pending commitments and finality evidence | delayed or duplicate delivery; explicit refund duty |
| DA24 | orthogonal | sequence / parallel / interleave / synchronize / message | operator-specific authority, duties, conflicts and fan-in | split partitions work and claims; join cannot duplicate resource |

The four `orthogonal` rows (DA20–DA24) are the ones that make the others
composable: observation, authorization, governance, messaging and composition.
DA24 names the **five composition operators**: sequence, parallel, interleave,
synchronize, message.

### 2.4 The composition result (a real measured number)

From the DeFiFormal audit, independently reproduced locally: over 1,830 eligible
protocol pairs, 1,645 compose cleanly and 185 fail. Of those failures **182 are
cross-category and 3 are within-category**. Within-category failure rate 2.10%;
cross-category 10.79% — a **5.14×** difference.

This is the empirical argument for the whole project: *composition across
financial categories is where DeFi breaks, and that is exactly what a type
system and an operational semantics can police.* Use the real 5.14× figure. An
earlier informal claim of "sixty times" was checked and is wrong; do not use it.

---

## 3. The formalization model

Moriarty source files use the **`.mori`** extension. The specification separates
what a program looks like from what it means, in five layers.

| Layer | Specification method | What it fixes |
|-------|---------------------|---------------|
| Lexical structure | Separate token rules and regular expressions | Identifiers, literals, whitespace, comments, source locations |
| Syntax | EBNF (ISO/IEC 14977) | Valid combinations of declarations, actions and expressions |
| Static semantics | Typing and scoping judgments, `Γ ⊢ e : τ` | Name resolution, asset units, resource use, admissible bounds |
| Dynamic semantics | Executable operational semantics in the **K Framework** | State transitions, financial effects, obligations, rejection |
| Correctness claims | Explicit properties over those semantics | What must be established about agreement, execution and history |

Why each choice, for the researcher mode:

- **EBNF** describes the grammar and deliberately does not decide surface style.
  ABNF (RFC 5234) is the protocol-spec sibling; Moriarty chose EBNF.
- **K** describes execution through configurations and rewrite rules. Typing
  judgments define admissible programs; contract properties and Hoare-style
  assertions state what must be proved. Denotational models can support
  particular financial analyses but do not replace the execution definition.
- The control layer is presented using **Felleisen–Hieb reduction semantics**.

### 3.1 The compilation pipeline

```
Moriarty source (.mori)
  → typed elaboration + finite resource/lifetime certificate
  → canonical Moriarty Core
  → readable generated Compact + correspondence manifest
  → compactc
  → ZKIR 3 + generated TypeScript + proving/verifier artifacts
  → Midnight ledger and wallet
```

Direct source-to-ZKIR generation is deliberately deferred. ZKIR is a typed
straight-line circuit IR with guarded impacts and no source-level financial
concepts, and it is ledger-coupled and evolving. Generating Compact preserves a
reviewable backend artifact and reuses the supported compiler, source maps,
runtime bindings and ledger operations. This is a design decision the site
should show as a decision, with its reason.

### 3.2 The Core / surface split

**Moriarty Core** retains: finite continuations, explicit actions and waits,
accounting, value conservation, explicit timeouts, typed warnings and errors,
and a decreasing structural measure. Beyond Marlowe V1 it adds visibility and
trust information — **every datum is classified `public`, `private`, `committed`
or `revealed`, and every oracle or external effect has a named capability and
assurance boundary.**

The **surface language** adds modules, named definitions, schedules,
token-indexed amounts, durations, records, packages, bounded compile-time
loops/folds and templates. All of it must elaborate away into Core.

The deployable **manifest** records: Core hash, source hash, compiler versions,
maximum lifetime, maximum transition count, maximum accounts/obligations,
continuation roots, visibility policy, external capabilities, and backend
artifact hashes.

### 3.3 Midnight realization

A long-lived agreement is **a sequence of bounded one-transition proofs**, not
one circuit that executes an entire lifetime. This is the key architectural idea
and deserves a visualization.

- Sealed ledger fields bind Core hash, version, parties/capabilities, token
  policy, deadlines and resource limits.
- Mutable public state holds phase, sequence number, accounting/obligation
  commitments and bounded continuation roots.
- Private witness callbacks are unverified TypeScript, so a generated circuit
  must constrain **every** witness result.
- Compact's `disclose()` authorizes a flow past the compiler's privacy analysis;
  it does not make that flow semantically safe. Moriarty therefore generates
  `disclose()` **only** from an explicit source visibility transition.
- Timeouts are permissionless exported transitions guarded by ledger block-time
  predicates. They do not execute autonomously.

---

## 4. The formal guarantees

The honest structure here is a **layered assurance vocabulary**. Each layer is a
separate obligation; establishing one does not establish the next. The site
should show this as a chain, because collapsing it is the standard marketing lie
in this space.

```
semantic validity
  → Moriarty-to-Compact translation validity
  → Compact-to-ZKIR compiler correctness
  → circuit / proof correctness
  → Midnight ledger feasibility
  → transaction construction and wallet correctness
```

### 4.1 The property inventory

Twelve properties. Each has evidence in the Marlowe/Isabelle lineage
(reproduced from pinned sources at `marlowe-lang/marlowe`
`7b5b1e90c171eae2674a6fc08aa7d8caa92b16af`), an assumption that qualifies it,
and the corresponding Moriarty obligation.

| Property | Assumption that qualifies it | Moriarty obligation |
|---|---|---|
| Termination | V1 core only; no new recursive construct | Core step decreases a well-founded measure; surface loops elaborate to finite Core |
| Finite maximum lifetime | every wait has a finite absolute bound | reject unbounded subscriptions; certify the bound in the manifest |
| Conservation of value | valid positive state and successful transaction | prove abstract conservation, and separately prove correspondence to Midnight kernel effects |
| Positive accounts | valid initial state | make validity a constructor and transaction precondition |
| Closure / no residual internal value | a semantic execution path exists and can be submitted | separate semantic closure from proving, ledger, witness and participant liveness |
| Quiescence and idempotence | pinned V1 evaluator | preserve in Core; do not require one giant circuit to reduce a whole lifetime |
| Grouped/split input equivalence | only under stated valid transaction conditions | reprove around atomic action sets — atomicity intentionally changes some equivalences |
| Determinism | canonical interval and serialization agreement | total deterministic step relation and canonical input ordering |
| Transaction count bound | abstract transactions, not ledger resource feasibility | certificate carries max transitions and per-entry-point resource estimates |
| Authorization | does not validate arbitrary role-token policy | model credentials/capabilities and nullifier/replay rules explicitly |
| Continuation integrity | hash comparison does not imply availability | bind continuation roots in ZK; specify replicated availability separately |
| Cross-implementation correspondence | no single theorem covers Isabelle, Agda, Haskell, TypeScript, Plutus | normative Core plus conformance vectors, differential tests, translation validation |

### 4.2 The four mandatory proof claims

Every accepted transaction must carry all four. Fixed names, fixed order, none
optional:

1. **ContractInvariant** — the agreement's own rules hold.
2. **IntentRefinement** — what executed refines what the principal authorized.
3. **TransitionValidity** — this state transition is legal.
4. **HistoryCompliance** — the predecessor history this extends is itself compliant.

Point four is the one that distinguishes Moriarty from contract-level
verification: a valid proof of a valid transition against an *invalid history*
must not be accepted.

### 4.3 Security and trust boundaries

Thirteen threat rows from `wiki/security.md`, each with attack path, required
control and **residual risk**. The residual-risk column is the point — it is the
part nobody publishes. High-value rows for the site:

| Threat | Required control | Residual risk |
|---|---|---|
| Malicious witness | circuit constrains commitments, signatures, units, ranges, freshness, authorization | compromised local secrets and selective withholding |
| Accidental disclosure | visibility types + Compact information-flow check; generated `disclose()` reviewed in manifest | traffic analysis; intentionally disclosed correlations |
| Compiler mistranslation | independent Core interpreter, translation validation, differential traces, pinned toolchain | compiler and validator bugs until proof coverage is complete |
| Token or unit confusion | token-indexed amounts and distinct time/duration types before lowering | Compact/ZKIR erase some domain distinctions |
| Authorization replay | domain separation, sequence numbers, nonce/nullifier, deadline | wallet/key compromise |
| Oracle manipulation | signed typed observation: source, feed, unit, timestamp, freshness, sequence, bounds, fallback | the source can still lie |
| Continuation loss | replicated content-addressed storage, preflight availability proof | integrity does not create availability |
| Runtime substitution | client recomputes Core hash, entry point, public effects, destinations, fees, versions, artifact hashes before signing | a compromised wallet UI can still deceive the user |
| External contract call | excluded from V0; later capability manifest, allowlist, effect summary | external behavior cannot inherit Moriarty guarantees |

Also worth stating plainly: **a single "Moriarty audit" is not an adequate
claim.** Audits split by normative Core and proofs; parser/type
checker/elaborator; Compact backend and translation validator; generated
circuits and artifact registry; runtime/client verifier; SDK/UI; and optional
oracle/composition protocols.

---

## 5. The developer interface — the DSL

### 5.1 Two syntax profiles (they are not interchangeable)

- **`moriarty-bounded-atomic/1`** — the implemented profile. Grammar, parser,
  type checker, canonical encoding, local evaluator and restricted Compact
  lowering. The loan and swap examples run under it.
- **`moriarty-successor-syntax/0`** — the provisional successor profile with
  separate lexical rules, EBNF grammar, bounded parser, canonical formatter and
  a read-only CLI (`check-syntax`, `format`).

### 5.2 Real source, for the site to show

Successor profile — a partial payment that must preserve its residual obligation:

```mori
profile "moriarty-successor-syntax/0";

agreement PartialPayment {
  unit USD;
  party borrower;
  party lender;
  state principal: Debt<USD> = debt(100, USD);
  state interest:  Debt<USD> = debt(10, USD);
  action payInterest(payment: Debt<USD>) {
    requires payment > debt(0, USD);
    requires payment <= pre.interest;
    next.interest = pre.interest - payment;
    ensures post.principal == pre.principal;
  }
}
```

Note the shape: `requires` / `next` / `post`, explicit pre- and post-state,
`Debt<USD>` as a type distinct from a transferable `Amount<USD>`. The
`ensures post.principal == pre.principal` line is the whole thesis in one
statement — paying interest must not silently touch principal.

Atomic profile — a constant-product swap, showing what the implemented language
actually enforces:

```mori
action swap(actor: Text, recipient: Text, asset_in: Text, asset_out: Text,
            amount_in: Amount<AssetA_quantum>, min_out: Amount<AssetB_quantum>) {
  guard obs.now < uint(2000000000), "horizon expired";
  guard remaining > uint(1), "final allowance is reserved for closure";
  guard arg.actor == const.trader, "trader authority required";
  guard arg.amount_in <= state.trader_a, "insufficient synthetic balance";
  let effective_input   = arg.amount_in * const.fee_numerator;
  let numerator         = effective_input * state.reserve_b;
  let denominator       = state.reserve_a * const.fee_denominator + effective_input;
  let output_calculated = floor_div(numerator, denominator);
  guard arg.min_out <= output_calculated, "minimum output not met";
  guard output_calculated < state.reserve_b, "output would empty reserve";
  ...
  emit Transfer { asset: arg.asset_in,  from: const.trader, to: const.pool,     amount: arg.amount_in };
  emit Transfer { asset: arg.asset_out, from: const.pool,   to: arg.recipient,  amount: output_calculated };
}
```

Reserves 1,000,000 A and 2,000,000 B; a 10,000 A input returns **19,743 B** under
the integer formula with a 997/1000 fee. Ask for 19,744 and you get a named
slippage failure, not a silent adjustment. These are the real numbers from the
repository — use them.

### 5.3 Declarations the language has

`unit`, `party`, `const`, `state`, `observation`, `settlement`, `status`,
`policy`, `reserve`, `effect`, `action`. Inside an action: `guard` (with a named
failure message), `let`, `set`, `emit`. Plus `lifetime` and `horizon` as
explicit bounds on the agreement itself.

The `policy` block is distinctive and deserves attention on the site — it makes
**rounding a declared, named, provable artifact** rather than an accident of
integer division:

```mori
policy accrued_interest targets write(accrue, interest_due), effect(accrue, 1, amount) {
  unit USD_micro;
  derivation "notional*8*31/(100*365)";
  rounding floor(accrue, interest_calculated);
  remainder "discard 54/73 micro-USD for this sample only";
  comparison "exact integer sample value; no ACTUS tolerance claim";
  proof "loan_first_period_interest_floor_v1";
}
```

Every policy names its unit, its derivation, its rounding direction, what happens
to the remainder, how it is compared, and the proof obligation it discharges.
That is the "financial semantics above Compact" claim made concrete.

### 5.4 The developer workflow

```
Import or author agreement
  → Check types and bounds
  → Inspect events and simulate   ┐
  → Analyze named properties      ┘
  → Prepare and verify plan
  → Review and sign intent
  → Prove authorized transition
  → Verify statement and proof
  → Check live ledger state and submit
```

Two rules the workspace enforces and the site should repeat, because they are
good design opinions: **simulation and static analysis are distinct — a
successful example does not turn the property list green.** And **a valid proof
does not make a stale predecessor live.**

The workspace is three columns: AGREEMENT (terms and parties, package version,
numeric profile, bounds and horizon, observations) · BEHAVIOR (event timeline,
calculation date, payment date, due/paid amounts, state changes) · TRANSACTION
(predecessor states, authorized action, asset movements and fees, signers and
expiry, proof and ledger status).

---

## 6. Intents

The intents model separates five artifacts that most systems conflate:

**Intent · Permission · Plan · Execution · Receipt.**

- **IntentIR** = principal + scope + authority + requirements + assumptions +
  lifecycle + validity/replay + semantic versions. It fixes *what outcome you
  authorize*, before any route is chosen.
- **PlanIR** = intent identity + bounded steps + adapter identities +
  dependencies. It is *one chosen route*.
- **Receipt** = intent/plan identities + checked effects + status + residual
  resources.

Two signing profiles:

- **Exact-plan signing** — you sign the concrete plan. Narrower meaning, no
  solver choice after signing. This is the working restricted profile.
- **Outcome-intent signing** — you fix authority, goals, permitted agreement
  programs, validity and nonce *before* a plan exists. Solvers then search
  outside the finite checker; ranking cannot excuse invalid authority or a failed
  goal, and there is no global best-price claim.

The authority rules that make this safe are precise and worth showing:

- Authority is **affine**. Partial completion consumes it. Residual authority
  cannot grow or reset an epoch.
- **Gross, not net.** For the signed principal, sum gross outgoing transfers
  *and* fees by asset across all recipients. **A refund never restores
  allowance.** Fees have their own cap and still consume gross authority.
- Every recipient must be on the permitted list — including refunded movements.
- Goals compare **net** final-minus-initial credits. Fees count against the net
  goal.
- **Asset identity is the entire `{domain, issuer, reference, kind}` tuple.**
  No ticker aliases. A receipt token is not the delivered asset.
- Validity is `notBefore <= now < expiresAt`, with a nonce, and an outcome-intent
  domain separator on the signature. An envelope's key cannot appoint itself
  authoritative — verification uses the caller's independent trust record.
- **Pending progress has its own judgment.** A pending receipt retains
  obligations; it can never show a terminal goal as settled.
- Anti-vacuity: a compiler that always rejects is not a working compiler. Require
  trace inclusion *and* a feasible positive witness.

Wallet rendering is derived from the canonical signed meaning: asset domains,
gross budgets, allowed recipients, net goals, fees, validity/nonces, assumptions
and liabilities — a deterministic semantic signing summary, not a hex blob.

---

## 7. Multichain transactions

The honest position, and a genuinely differentiating one. From the contradiction
register:

- Marketing overviews claim atomic cross-chain execution with automatic refunds.
  The underlying verifier documentation says external calls complete
  **asynchronously**, simulation **excludes** them, and deposit, withdrawal,
  storage and indexer paths include detached, nonrefundable or manually
  recovered steps.
- "Non-custodial" descriptions coexist with contract-held internal balances,
  temporary transfers to a trusted swapping agent, and a treasury plus
  proof-of-authority bridge.
- Confidentiality labels like `basic` and `advanced` can lack public normative
  leakage definitions.

Moriarty's three rules in response:

1. **Define atomicity per layer and per route.** Never lift an internal
   ledger-batch atomicity up to bridge fulfillment.
2. **Publish a route-specific custody and authority manifest before approval.**
3. **Model confidentiality as a named adapter profile** with explicit disclosure
   and custody assumptions.

The decisive test case, which the site should state as a challenge: *construct a
cross-chain swap trace in which the internal batch succeeds but the bridge
withdrawal fails or is rolled back.* Any model that cannot represent that trace —
and reject it — is not modelling cross-chain settlement. Operator, relay,
treasury and bridge attestations cannot discharge a proof obligation.

Bridges split into **message-verified** and **custodial** trust. Those are
different security stories and the site must not merge them.

---

## 8. Safe DeFi

The synthesis section. Four mechanisms, each traceable to a category of real
failure:

1. **Typed asset identity and units.** Token-indexed amounts, distinct
   time/duration types, `Debt<T>` distinct from `Amount<T>`, the full
   `{domain, issuer, reference, kind}` tuple. Kills unit-confusion and
   ticker-aliasing bugs at the type level.
2. **Declared rounding and remainder policy.** Every conversion names its
   direction, its remainder disposition and its proof obligation. Kills the
   rounding-direction class of vault and share-conversion exploits.
3. **Affine authority with gross accounting.** Refunds do not restore allowance;
   fees consume authority; every recipient is enumerated. Kills the
   authorization-scope class.
4. **History compliance as a proof obligation.** A valid transition against an
   invalid predecessor is rejected. Kills the class where each step looks locally
   fine.

Adversarial modelling requirement, from the surveyed literature: derive fixtures
with **separate actor capability, vulnerable layer, precondition, ordered
effects, violated predicate and loss outcome.** Two rules that follow:
**ABI shape does not establish semantic compatibility**, and **a valid oracle
signature does not establish economic truth.** Flash borrowing is a capability
with legitimate uses, not a vulnerability.

---

## 9. Delivery roadmap — the consolidated status section

One section, at the end. Twelve sprints, each with its decisive completion
evidence.

| Sprint | Deliverable | Decisive completion evidence |
|---|---|---|
| SP01 | Freeze behavior and reuse accepted foundations | complete behavior/source/authority crosswalk; bounded native go/no-go |
| SP02 | Finish the language contract and authoring tools | full lexical/EBNF/static specification plus working check/format |
| SP03 | Make K execute the financial distinctions | runnable K/evaluator agreement; discharged base-domain correspondence claims |
| SP04 | Decide whether the full native verifier works | all native/outer verifier controls including the final accumulator decision |
| SP05 | Finalize loan and swap effects on Preview | both finalized on Preview with independently checked effects |
| SP06 | Produce and verify real recursive history | real two-step recursive proof; retained bytes verified; mutations rejected |
| SP07 | Implement ACTUS without losing debt or fields | 277 fixtures, all fields, 18 executable types, 32 dispositions |
| SP08 | Behavior-driven DeFi and intent libraries | all 72 DeFi rows, DA24, intent/request lifecycles, modeled regressions |
| SP09 | Mandatory proof and authority at ledger acceptance | all four mandatory claims; proved correspondence; Preview acceptance |
| SP10 | Private handoff and bounded composition | isolated private handoff; real split/join; all five composition operators |
| SP11 | Full financial and formal conformance | every required behavior qualified across semantics, proof and Preview |
| SP12 | Developer release and reproducible evidence | usable end-to-end flow, two clean builders, two non-toy pilots, gates G01–G24 |

Three parallel tracks run now: **SP01→SP02→SP03** (language),
**SP01 F0→SP04→SP06** (native feasibility and proofs), and
**accepted atomic + loan/swap subset→SP05** (financial Preview integration).

Concrete milestones already reached, usable as evidence on the site:

- Bounded K definition executes Transfer-only and Transfer-then-Repay with
  AccrualFirst / PrincipalFirst / ProRata allocation and explicit
  none/floor/ceil conversion rounding. **All 16 frozen cases** match independent
  financial expectations, including exact rejection code and index. Transfer-only
  carries **15 guards** and 18 presentation steps; repayment carries **28
  guards** across its stages and 37 steps.
- The successor syntax profile has lexical rules, EBNF grammar, bounded parser,
  canonical formatter and read-only CLI.
- A hello-world Compact deployment and call were finalized on a local Midnight
  network with exact state readback and indexed blocks below the finalized head.

---

## 10. Voice and rules for the site

- **Financial precision is the brand.** Exact integers, named rounding, named
  failures. Never round a number for aesthetics. 19,743 is 19,743.
- **Every number on the site is real** and traceable to this document. No
  invented benchmarks, TPS figures, TVL, user counts or logos.
- **Name the assumption next to the claim** wherever a claim has one. This is the
  project's actual differentiator and it reads as confidence, not hedging.
- **No invented verified badges.** Nothing may display a checkmark, "audited",
  "proven" or a green state that the repository does not support.
- **The five composition operators, the four proof claims, the eight facets, the
  seven families, the 24 action targets** — these counts are fixed. Do not round
  them, extend them or drop members.
- No Claude, AI, agent or model attribution anywhere in the site, its source, its
  comments or its commits.
