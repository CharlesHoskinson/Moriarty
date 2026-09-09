# Moriarty website — content specification

This is the authoritative source of substance for the website. Everything on the
site must trace to this document, and everything here traces to the repository.
Whoever builds the site must not invent protocol behavior, guarantees, numbers
or syntax. If something is needed and is not here, it does not go on the site.

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

### What the loan example is actually asking

The repository's loan example is the quickest way to see why the ordinary
tools are not enough. The agreement starts with a notional of 5,000,000,000
micro-USD. The first period accrues a principal installment of 500,000,000 and
interest of 33,972,602, and the total due is 533,972,602. When the borrower
settles that total, the episode closes, and the remaining notional is
4,500,000,000 micro-USD.

Computing 33,972,602 is arithmetic, and any language can do it. The work is in
what happens around the number. The payment must discharge two specific dues,
one for principal and one for interest, and not some third thing. It must
reach the lender the agreement names rather than whoever submitted the
transaction. It must leave the 4,500,000,000 standing, because settling one
period is not settling the loan. And it must not be replayable, because a
payment that can be presented twice discharges twice. A general contract
language expresses none of those requirements directly. It stores integers,
and the integers can be moved in ways that look fine and are wrong.

An audit reads the finished code and tries to show that none of those things
went wrong. A type system and an operational semantics refuse to build the
program in which they can go wrong. That is the difference the brief is
pointing at when it says these questions belong to the language rather than to
the audit. The loan example makes the last of them a literal guard:

```mori
guard state.notional == const.expected_outstanding_notional,
      "episode closure cannot discharge remaining notional";
```

The guard runs after the settlement writes and after the episode is marked
closed. If closing the episode had touched the notional, the whole action would
reject, and the ledger would see nothing. That is the shape of every Moriarty
protection: a named condition, checked at a fixed point, whose failure is a
named rejection with no partial effects.

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

The ACTUS point about rounding deserves a sentence of explanation, because it
is where "reproduce the behavior" stops being a slogan. An ACTUS fixture is a
worked contract with dated events and the cash flow each event produces. Two
implementations that both "implement an annuity" will disagree on the last
digit of an interest payment if one rounds toward zero and the other rounds to
nearest, and over a schedule those disagreements compound into a different
balance. Matching a fixture therefore means matching the date convention and
the rounding rule, not just the formula. The loan example records its own
rounding choice in the source, down to the fraction that was discarded, so the
comparison against the fixture is exact rather than approximate.

---

## 2. The DeFi category model

This is the site's organizing spine. Two levels: **economic families** (what kind
of financial thing it is) and **facets** (orthogonal properties every instance
has). Product labels are not categories — the same protocol carries several
facets and can appear in more than one family.

A single flat list cannot say what a thing is without also saying how it is
built, and those are separate questions. The historical list mixed product
functions, instruments, mechanisms, infrastructure, asset provenance and legal
properties in one set of labels, so that "bridge" sat beside "lending" as if
they were the same kind of category, and a protocol that lends across a bridge
had no home. Splitting the economic function from the properties every
instance carries fixes that. The family answers what financial thing this is,
and the facets answer how it executes, settles, holds custody, depends on law,
backs its obligations, consumes evidence, grants authority and finds a price.

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

The families are deliberately about economic function rather than mechanism.
A constant-product pool and a central limit order book are built in very
different ways, and both belong to F1 because both convert one asset into
another and produce a price in the process. A lending market and a
collateralized debt position look different to a user, and both belong to F2
because both create an obligation to repay that is secured and managed through
accrual, partial performance and default. The architecture record states the
discipline directly: the family taxonomy must never add a constructor merely
because a market aggregator has a label for it.

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

The intent placement follows from asking what an intent changes. A trader who
signs an intent to receive at least a certain amount of one asset for at most
a certain amount of another is still doing an exchange. The economic function
is unchanged. What changed is who chooses the route and when the choice is
made, and that is a question about execution. Filing intents as a family would
put a swap-by-intent in a different category from a swap-by-plan when they are
the same financial act with a different authorization shape.

The bridge split is about who you are trusting. A message-verified bridge
releases funds on the destination when it can verify a message that the source
chain actually committed the corresponding lock, so the trust is in the
verification of that message. A custodial bridge releases funds when a party
who holds the assets says to, so the trust is in that party. They can look
identical to a user and fail in completely different ways, and a settlement
facet that does not distinguish them cannot say what happens when the bridge
fails. Section 7 returns to this.

### 2.3 The action targets

The families are the human-facing map. The action targets are what the language
must actually execute, and they are the real content. `DA01` to `DA24` come from
`deliverables/defi-language-design-2026-09-07/action-targets.csv`. Each row has a
semantic requirement (what the action must mean) and a **distinguishing test**
(the case that separates a correct implementation from a plausible-looking wrong
one). The distinguishing tests are the most persuasive content on the site.

A distinguishing test is not a unit test. It is the one input on which the
right implementation and the tempting wrong one give different answers. Take
DA04. A naive lending pool models a depositor's claim as a balance and a
withdrawal as a subtraction. When the pool lacks liquidity, the withdrawal
fails, and in the naive model the natural way to fail is to leave the balance
alone and revert. That looks correct. The distinguishing test asks what the
system records about the claim afterwards. A pool that has borrowed out its
liquidity still owes the depositor, and if the failed withdrawal is modelled
as "nothing happened", the pool has no representation of the fact that the
depositor tried to leave and could not. Under stress that is exactly the fact
that matters. The requirement "insufficient pool liquidity rejects without
erasing the claim" separates a system that knows it owes from a system that
merely has not yet paid.

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

The `orthogonal` rows are the ones that make the others composable. They carry
observation, authorization, governance, messaging and composition, and DA24 names
the operators: **sequence, parallel, interleave, synchronize** and **message**.

The orthogonal rows are orthogonal in the geometric sense: every family needs
them and none owns them. A swap consumes a price observation and a loan
consumes a time observation, and both are DA20. A vault redemption and a
staking exit are both pending commitments with a delayed completion, and both
are DA23's machinery under a different family's name.

### 2.4 The composition result

From the DeFiFormal audit, reproduced locally: of 1,830 eligible protocol pairs,
1,645 compose cleanly. The rest fail, and almost every failure crosses a category
boundary. Within a category the rate is 2.10%. Across categories it is 10.79%,
which is **5.14 times** as often.

This is the empirical argument for the work. Composition across financial
categories is where this breaks, and composition is what a type system and an
operational semantics can police. An earlier informal claim of sixty times was
checked against the corpus and is wrong.

The measurement deserves to be understood rather than quoted. The DeFiFormal
corpus specifies protocols as constructions over a shared algebra, and a pair
composes cleanly when the two constructions can be combined without violating
either one's stated obligations. The audit took every eligible pair, sorted
each pair by whether both members sat in the same legacy category, and counted
failures on each side. The exact denominators are 143 within-category pairs
and 1,687 cross-category pairs. Of 185 failures, 182 crossed a category
boundary and 3 did not. The unrounded rates are 2.0979% within and 10.7884%
across, and their ratio is 5.1425.

What the number says is narrower than it sounds and more useful. It does not
say that cross-category composition is unsafe. It says that when a lending
construction meets a derivative construction, or a vault meets a bridge, the
obligations they carry are far more likely to conflict than when exchange
constructions meet, and that the conflict is visible to a checker that
understands the obligations. The sixty-times figure that circulated earlier
was checked against the pinned corpus with an explicit pair-eligibility rule
and does not survive, and the site must not use it.

---

## 3. The formalization model

Moriarty source files use the **`.mori`** extension. The specification separates
what a program looks like from what it means.

| Layer | Specification method | What it fixes |
|-------|---------------------|---------------|
| Lexical structure | Separate token rules and regular expressions | Identifiers, literals, whitespace, comments, source locations |
| Syntax | EBNF (ISO/IEC 14977) | Valid combinations of declarations, actions and expressions |
| Static semantics | Typing and scoping judgments, `Γ ⊢ e : τ` | Name resolution, asset units, resource use, admissible bounds |
| Dynamic semantics | Executable operational semantics in the **K Framework** | State transitions, financial effects, obligations, rejection |
| Correctness claims | Explicit properties over those semantics | What must be established about agreement, execution and history |

A grammar says which strings are programs. A type system says which programs
are admissible. An operational semantics says what an admissible program does
when it runs, and a property says what must be true of what it does. A `.mori`
file passes through each layer in turn and can be refused at each one for a
different kind of reason.

Why each choice, for the researcher mode:

- **EBNF** describes the grammar and deliberately does not decide surface style.
  ABNF (RFC 5234) is the protocol-spec sibling; Moriarty chose EBNF.
- **K** describes execution through configurations and rewrite rules. Typing
  judgments define admissible programs; contract properties and Hoare-style
  assertions state what must be proved. Denotational models can support
  particular financial analyses but do not replace the execution definition.
- The control layer is presented using **Felleisen–Hieb reduction semantics**.

**What the grammar layer does and does not decide.** EBNF extends plain BNF
with notation for repetition and optionality, so a rule can say "zero or more
declarations" or "an optional parameter list" without writing out the
recursion. That is all it does. It does not say whether the language should
look like Lisp or like a language with braces, and the repository's surface
comparison is explicit that no syntax study establishes one style as
universally better.

**What a typing judgment says.** The notation `Γ ⊢ e : τ` reads "in the
environment Γ, the expression e has type τ". The environment is the set of
names in scope with their declared types, and the judgment is a claim that can
be checked by following the declaration of each name back to where it was
introduced. In Moriarty the type carries a unit vector alongside the base
type, so the judgment is really "e has type τ and unit q". Addition and
subtraction require the two operands to have the same type and the same unit
vector, and they keep that vector. Multiplication adds the exponents, so an
amount of A times a fee ratio has a different unit from the amount of A that
went in. Division exists only as `floor_div`, and it subtracts the
denominator's exponents. A program that adds an amount of one asset to an
amount of another does not have a type, and the judgment fails at that line
with a source span. That is what "asset units" in the static-semantics row
means in practice: the unit is checked before anything runs.

**What a K configuration is.** K describes a running program as a
configuration, which is a structured term holding everything the execution
needs: the remaining computation, the state it is reading and writing, and
any bookkeeping. A rewrite rule is a pattern that matches part of a
configuration and replaces it with another pattern. Execution is the repeated
application of rules until none applies, and the final configuration is the
answer. The repository's repayment definition makes this concrete. The
program term is `run_H(k)`, where `k` is a sequence of pending instructions
and `H` is the digest of the input packet, and the instructions are things
like `convert`, `divide`, `round`, `fund`, `allocate` and `split`. A rule such
as

```text
convert(P,s,r) ↝ g(n*m ≤ U, OVERFLOW) ▷ divide(P,s,r,n*m)
```

says that when the next instruction is `convert`, replace it with a guard that
the product of nominal and mantissa fits in UInt128, followed by a `divide`
instruction that carries that product forward. The guard is itself an
instruction, `ensure(H, b, code, i)`, and it has two rules: when `b` is true it
rewrites to the empty computation and execution continues, and when `b` is
false the whole program rewrites to `rejected(H, code, i)`. That second rule is
called ABORT, and it discards everything after the guard, including
finalization, so a rejected run has no tentative state and no partial effects.
This is the operational meaning of "a failure is a named rejection" that runs
through the whole brief.

**Why the stages are ordered the way they are.** The sequence of instructions
is not decorative. Settlement is `nominal × mantissa / 10^scale`. If the
definition computed `10^scale` before checking that the scale is at most 18, an
adversarial packet with a huge scale would force an enormous exponentiation
before any guard could refuse it. If it divided before checking that
`nominal × mantissa` fits UInt128, an overflowing product could be divided
down to an answer that looks in range. So the scale guard sits in the state
stage before `convert` is reached, and the product guard sits inside `convert`
before `divide`. The definition uses K instructions rather than eager numeric
helpers over the whole future computation precisely so that these boundaries
are real, and a reader who understands that understands why the semantics is
staged at all.

**A worked repayment.** Take an obligation of principal 100 and accrued
interest 10, and a nominal payment of 7. The allocation rule decides how much
of the 7 discharges principal. Under AccrualFirst the payment goes to accrued
interest first, so the principal discharge is `7 − min(7, 10) = 0`, and the
result is principal 100 and accrued 3. Under PrincipalFirst it is `min(7, 100)
= 7`, giving principal 93 and accrued 10. Under ProRata it is
`floor(7 × 100 / 110) = floor(700 / 110) = 6`, giving principal 94 and accrued
9. In every case the outstanding amount is 103, because 110 minus 7 is 103
however it is allocated, but the split between the components differs,
and that split is what a later interest calculation reads. This is the
distinguishing test for DA06 made numeric: partial repayment must preserve the
allocation, and three correct systems with three declared policies give three
different allocations, while a system that merely subtracts 7 from a balance
of 110 has thrown the allocation away.

The same packet shows conversion. With mantissa 3, scale 1 and floor rounding,
the settlement is `floor(7 × 3 / 10) = floor(21 / 10)`, and the division gives
quotient 2 with remainder 1. Floor takes 2. Ceil would take 3, because the
remainder is nonzero. The `none` mode requires an exact result and rejects
this packet with `INEXACT_CONVERSION`, because a remainder of 1 is not zero.
If floor had produced 0, the definition would reject with `DUST`, because a
positive nominal payment that converts to no settlement at all is not a
payment. And the converted amount, 2, is what gets checked against the cash
the preceding transfer actually moved; the nominal 7 is not, because nominal
and settlement have different roles and confusing them is the bug the
definition exists to prevent.

**Why reduction semantics for the control layer.** Felleisen–Hieb reduction
semantics presents execution as a small set of primitive contractions plus a
notion of evaluation context that says where in a term the next contraction
may happen. The context is written with a hole, `□`, and the rule

```text
run_H(E[a]) → run_H(E[k'])   when a ↝ k'
```

says that if the instruction in the hole contracts to `k'`, the whole program
steps by replacing it. The contexts in the repayment definition are only `□`
and `E ▷ k`, which means the hole is always the first pending instruction and
the rest of the sequence waits. There is no context that skips an unfinished
instruction, and no context that reaches inside packet data or inside a
terminal answer. That restriction is what makes the ordering argument above a
theorem about the presentation rather than a hope about the implementation: a
guard cannot be bypassed because there is no context in which the instruction
after it is reachable while it is still pending.

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

The deferred design would have bought something real. Going straight from
Core to ZKIR removes one stage from the chain of things that must be trusted.
There would be no dependence on `compactc`, no Compact source to keep in step
with a moving compiler, and one fewer translation whose correctness has to be
argued. If ZKIR were a stable published target, that would be a strong case.

It is not, and the cost of treating it as one is what decided the question.
ZKIR is a straight-line circuit representation with guarded impacts, and it
has no notion of an asset, a unit, an obligation or a party, so every
financial distinction Moriarty introduces would be erased at the moment of
generation with nothing in the target to check it against. It is coupled to
the ledger and it is evolving, so a direct generator would chase a moving
target and every ledger change would be a Moriarty compiler change. And a
direct generator would have to reimplement what the Compact toolchain already
provides and supports: source maps, runtime bindings, the ledger operations
and the proving and verifier artifacts. Generating Compact instead leaves a
human-readable artifact between Core and the circuit. A reviewer can read the
generated Compact and compare it against the Core it came from, and the
correspondence manifest records what that comparison is supposed to show. The
compiler is still not silently trusted, which is why the architecture requires
translation evidence and an independent Core-versus-generated-Compact trace
validator, but the trust is placed on an artifact that can be read.

A second decision sits in the same place. The backend could have been a single
universal interpreter circuit that takes any Core program as input and
executes it, which would preserve one audited evaluator for all agreements.
The architecture record explains why it is not: such a circuit's proving key
and cost would be fixed by the worst case over every AST shape, depth, account
count and action set it could ever be asked to run, and it would also have to
prove bounded decoding and dispatch of the program itself. Ahead-of-time
specialization, one circuit per agreement, is cheaper now and stays within
reach of review, provided the compiler is checked rather than trusted. That
proviso is the same one as above, and the decisions reinforce each other.

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

"Elaborate away" is the discipline that keeps the trusted part small. A
surface loop that runs a bounded number of times becomes, in Core, that many
copies of its body, and a schedule becomes explicit dated actions. Nothing in
the surface has a meaning of its own, because its meaning is whatever Core it
produces, so every proof and every conformance test is stated once, about
Core. The decreasing structural measure is what makes termination a property
of Core rather than a promise. A Core step is required to reduce a
well-founded quantity, so no sequence of steps can run forever, and a surface
construct that could not be elaborated into something that decreases would be
refused.

The deployable **manifest** records: Core hash, source hash, compiler versions,
maximum lifetime, maximum transition count, maximum accounts/obligations,
continuation roots, visibility policy, external capabilities, and backend
artifact hashes.

The manifest is what a verifier, a wallet and an auditor share. It pins the
exact Core, so a proof about that Core cannot be presented for a different
program, and it pins the compiler versions and artifact hashes, so a circuit
from a different toolchain cannot be substituted. It lists the external
capabilities, which is the honest list of everything the agreement depends on
that the proof cannot establish.

### 3.3 Midnight realization

A long-lived agreement is **a sequence of bounded one-transition proofs**, not
one circuit that executes an entire lifetime. This is the key architectural idea
and deserves a visualization.

The alternative is worth stating so the choice reads as a choice. One could
imagine a circuit that takes the initial state and every action the agreement
will ever receive, runs the whole lifetime, and proves the final state. That
circuit would have to be sized for the longest lifetime the agreement permits
and the largest state it could reach, and it could not be run until the last
action was known. A loan with a schedule of payments would wait years for its
proof. The one-transition design proves each step as it happens, against the
state it started from, and relies on a separate obligation to show that the
starting state was itself the result of a proved step. The Marlowe theorem
inventory's quiescence and idempotence results are what make this sound: a
contract that has quiesced after a step can be resumed from that state
without loss, so the lifetime can be cut at every step boundary. The
repository states the obligation explicitly as "do not require one giant
circuit to reduce a whole lifetime".

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

**What a circuit constrains, and what a witness is.** A zero-knowledge circuit
is a fixed set of arithmetic relations over some inputs. Some inputs are
public and some are private, and the private ones are supplied by a witness,
which on Midnight is a piece of TypeScript that runs on the prover's machine
and returns whatever values it likes. The circuit does not verify the witness
code. It verifies only that the returned values satisfy the relations. So if a
circuit needs a quotient, and the witness supplies one, the circuit must
itself check that the quotient times the divisor plus a remainder equals the
dividend and that the remainder is smaller than the divisor. If it does not,
the witness can supply any quotient it likes and the proof will pass. The
repository's Compact mapping does exactly this: the generated kernels take an
explicit hints struct of multiplication limbs and a quotient and remainder,
the circuits constrain those hints against the dynamic operands, and mutating
any supplied limb, quotient or remainder is covered by a rejection test. The
phrase "supplying a quotient or multiplication limb does not make it trusted"
is the whole principle. A witness proves possession of values that satisfy the
circuit; it proves nothing about where they came from.

**What `disclose()` does.** Compact runs an information-flow analysis that
refuses to let private-derived data reach public state. `disclose()` is the
escape hatch that tells the analysis a particular flow is intended. It is an
authorization, not a safety argument: the compiler stops objecting, and
whether the disclosure was a good idea is left to the author. Moriarty
therefore does not let generated code call it freely. A `disclose()` appears
in generated Compact only where the source made an explicit visibility
transition, so every disclosure traces to a line an author wrote and a
reviewer can find in the manifest.

**Why timeouts do not run themselves.** A ledger does not execute code on a
clock. A timeout in Moriarty is an exported transition that anyone may submit,
guarded by a predicate over the ledger's block time, so that it can succeed
only after the deadline. Someone still has to submit it, construct the
transaction, generate the proof and have the data available, and Section 4
returns to that under closure and liveness.

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

Each arrow is a place where a true statement on the left can be accompanied by
a false one on the right. A Core program can be semantically valid, meaning
its properties hold over its stated domain, while the Compact generated from
it computes something else; that is a translation failure, and the semantic
proof says nothing about it. The Compact can be right while `compactc`
miscompiles it. The circuit can be right while the proof system that checks
it has a soundness bug. The proof can be valid while the transaction that
carries it does not fit the ledger's limits. And all of that can be right
while the wallet shows the user a different transaction from the one it
signs. The formal-assurance record extends the chain further, to private
witness correctness and availability and then to participant and oracle
liveness, and states the rule that governs all of it: a proof at one arrow
never silently discharges the next. The site should never write "proven"
without saying which arrow.

### 4.1 The property inventory

Each property has evidence in the Marlowe and Isabelle lineage, reproduced from
pinned sources at `marlowe-lang/marlowe`
`7b5b1e90c171eae2674a6fc08aa7d8caa92b16af`. Each also has an assumption that
qualifies that evidence, and an obligation that falls to Moriarty. The three
never collapse into one.

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

**Closure against liveness.** The Marlowe closure theorems say that a contract
in a valid state has a path to a state with no value left inside it. That is a
statement about the semantics: such a path exists. It is not a statement that
anyone will walk it. Walking it requires a participant who chooses to submit,
a transaction that can be constructed, a proof that can be generated, witness
data that is still available, and a ledger that accepts. Any of those can be
missing while the semantic path still exists. A system that reports "funds
cannot be stuck" on the strength of the closure theorem has collapsed them,
and the obligation in the table is to keep them apart: prove semantic closure,
and state proving, ledger, witness and participant liveness as separate
assumptions with their own evidence.

**Integrity against availability.** A continuation is the rest of the
agreement after the current step. Core keeps continuations behind hashes, and
in the Merkleized form it keeps a root that commits to every branch. Checking
a supplied continuation against its hash proves that it is the one that was
committed; nothing else could produce that hash. It does not prove that
anyone still has it. If the only copy of a branch is lost, the root is still
valid and the agreement cannot proceed down that branch. That is why the
obligation splits into binding the roots in zero knowledge, which gives
integrity, and specifying a replicated availability protocol, which is a
separate engineering problem that no hash can solve. The security row for
continuation loss states the residual plainly: integrity does not create
availability.

**Grouped and split inputs.** Marlowe proves, under conditions, that applying
a group of inputs in one transaction is equivalent to applying them one at a
time. Moriarty deliberately breaks part of that equivalence, because an atomic
action set is supposed to succeed or fail as a unit, and a sequence of
separate transactions is not. A flash loan is the clearest case: borrowing and
repaying in one atomic transaction is a legitimate capability, and borrowing
in one transaction and repaying in the next is a different thing entirely. So
the obligation is not to inherit the theorem but to reprove the equivalences
that survive around the atomic boundary and to name the ones that do not.

**Authorization.** Marlowe's operational checks confirm that an input was
authorized by an address or a role token. They cannot say whether the policy
that governs the role token is sensible, because that policy is external to
the contract. Moriarty's obligation is to model the credential explicitly and
to model the nullifier and replay rules that stop a valid authorization from
being used twice. Section 6 gives the rules.

### 4.2 The mandatory proof claims

Every accepted transaction carries all of them. The names are fixed, the order is
fixed, and none is optional.

1. **ContractInvariant** — the agreement's own rules hold.
2. **IntentRefinement** — what executed refines what the principal authorized.
3. **TransitionValidity** — this state transition is legal.
4. **HistoryCompliance** — the predecessor history this extends is itself compliant.

Point four is the one that distinguishes Moriarty from contract-level
verification: a valid proof of a valid transition against an *invalid history*
must not be accepted.

The repayment example gives each claim a concrete meaning. ContractInvariant
is the `ensures post.principal == pre.principal` line: the agreement declared
that paying interest leaves principal alone, and the proof must show it did.
IntentRefinement is the relation between what the borrower signed and what
ran: if the borrower authorized a payment of at most a certain amount to the
lender, the executed payment must be within that amount and must go to the
lender. TransitionValidity is the operational semantics: given this
pre-state, this action and these observations, the semantics produces exactly
this post-state and these effects, and nothing else. Each of those is a claim
about one step.

HistoryCompliance is a claim about every step before it. Consider an agreement
whose initial state was constructed with the notional set to a different
figure from the one the parties agreed, or whose second transition was
accepted by a verifier that did not check IntentRefinement. Every transition
after that could be locally perfect. Each would carry a valid ContractInvariant
proof, a valid IntentRefinement proof and a valid TransitionValidity proof,
and each would be a correct step from a state that should never have existed.
Contract-level verification stops at the step. HistoryCompliance requires the
new proof to establish that its predecessor was itself accepted under the same
mandatory claims, which reaches back, step by step, to an allowed initial
state. That is what proof-carrying data means here, and it is why recursive
proofs are the proposed mechanism: the new proof checks the old proof inside
itself. The repository is clear that recursion in the proof system does not
add recursion to the source language, and that no native recursive Moriarty
proof has yet been produced.

The claims are fixed because a prover who could choose them could drop one.
The deployment policy fixes the permitted claim specifications and verifier
versions, the participant's signed authorization commits to the mandatory
claims, and acceptance must reject a transaction whose evidence is missing a
claim, uses an unsupported claim, or leaves a dependency unresolved. A prover
cannot select a permissive verifier and cannot remove a requirement the signer
committed to. The atomic profile's bounds document binds the claim list into
the genesis of every instance through a domain-separated root, so the list is
part of what the parties agreed to at the start.

### 4.3 Security and trust boundaries

The threat rows come from `wiki/security.md`. Each carries an attack path, a
required control and a **residual risk**, and the residual risk is the point,
because it is the part nobody publishes. These rows carry the most weight:

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

The residual column is where the site earns trust, because each residual is a
statement of the form "here is what the control does not do". The pattern
across the rows is worth teaching once so a reader can apply it themselves.

**A signature proves possession, not truth.** The oracle row's control is a
signed typed observation carrying source, feed, unit, timestamp, freshness,
sequence, bounds and fallback. Every one of those fields is checkable: the
signature shows that the holder of a key produced this statement, the
timestamp and freshness window show it is not stale, the sequence number
shows it is not a replay, the unit shows it is not being read in the wrong
denomination, and the bounds show it is not wildly outside its declared
range. What none of them shows is that the price is correct. A key holder who
signs a wrong number produces a perfectly valid signed observation. The
residual says so plainly, and the safe-DeFi rule in Section 8 repeats it: a
valid oracle signature does not establish economic truth.

**A hash proves integrity, not availability.** The continuation row is the
same shape. Content-addressed storage means the address of a continuation is
its hash, so a wrong continuation cannot be substituted for the right one. It
does not mean the right one is anywhere. Replication and a preflight
availability proof reduce the risk, and the residual remains.

**A circuit constrains values, not the code that produced them.** The
malicious-witness row is the case from Section 3.3. The circuit can constrain
every commitment, signature, unit, range, freshness and authorization it is
given, and it should, because a witness is unverified code. What it cannot do
is make a compromised machine hand over the right secret, or force a witness
to reveal a value it chooses to withhold.

**A client can recompute, but it can only show what it is allowed to show.**
Runtime substitution is the attack in which the planner offers a different
transaction from the one the user thinks they are approving. The client
recomputes the Core hash, entry point, public effects, destinations, fees,
versions and artifact hashes from the canonical objects before signing, so a
substituted transaction will not match. A wallet interface that has itself
been compromised can still render whatever it likes, and no computation
upstream of the display can fix that.

Also worth stating plainly: **a single "Moriarty audit" is not an adequate
claim.** Audits split by normative Core and proofs; parser/type
checker/elaborator; Compact backend and translation validator; generated
circuits and artifact registry; runtime/client verifier; SDK/UI; and optional
oracle/composition protocols.

Each boundary is a different kind of artifact with a different kind of failure
and a different specialist who can find it. An auditor who has verified the
Core theorems has not read the parser, and an auditor who has read the
generated circuits has not examined the wallet's rendering path. The honest
version of the claim names which boundary was audited.

---

## 5. The developer interface — the DSL

### 5.1 Two syntax profiles (they are not interchangeable)

- **`moriarty-bounded-atomic/1`** — the implemented profile. Grammar, parser,
  type checker, canonical encoding, local evaluator and restricted Compact
  lowering. The loan and swap examples run under it.
- **`moriarty-successor-syntax/0`** — the provisional successor profile with
  separate lexical rules, EBNF grammar, bounded parser, canonical formatter and
  a read-only CLI (`check-syntax`, `format`).

Both profiles exist because the language is being redesigned while the
existing one keeps running. The atomic profile is what the evaluator, the
Compact mapper and the local demonstrations actually execute, and its source
uses `guard`, `let`, `set` and `emit` inside actions. The successor profile is
the shape the language is moving toward, with `requires`, `next` and
`ensures` and explicit pre- and post-state, and its parser and formatter work
while its semantics is not yet frozen. A reader who expects the atomic
profile's `policy` blocks in successor source will not find them, because the
successor grammar lists `policy`, `observation`, `settlement`, `status`,
`reserve` and effect-schema forms among the productions it does not yet
include. The site must show which profile every snippet belongs to.

A narrower profile may also be mentioned because the K definition executes
it. `moriarty-funded-source/0` uses the successor header
and admits a typed subset with `unit`, `party`, `asset` and `action`
declarations and explicit `Transfer` and `Repay` emissions. It rejects `state`,
`const`, `requires`, `let`, `next` and `ensures` even though the grammar
admits them. Its values are limited to UInt128, and it produces local
`Prepared` candidates rather than anything authorized or accepted.

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

**Nominal debt against transfer.** The reason `Debt<USD>` is a different type
from `Amount<USD>` is that they answer different questions. An amount is a
quantity of an asset that somebody holds and can move. A debt is a quantity
that somebody owes to somebody else, in some denomination, and it moves only
in the sense that it is discharged. You can hand an amount to a stranger. You
cannot hand a debt to a stranger; you can only pay it down, and paying it down
requires an amount to move from the debtor to the creditor and a record that
the movement was applied to this obligation rather than to some other. The
naive model collapses the two into one balance, so that "borrower owes 110"
is stored the same way as "borrower holds 110", and a repayment is a
subtraction from one and an addition to the other. That model cannot express
a partial payment correctly, because it has nowhere to record which part of
the 110 was reduced. It cannot express a payment to the wrong creditor,
because it has no creditor. And it cannot express the difference between a
payment that arrived and an obligation that was cleared, because it has only
the balance. Making `Debt` its own type means the compiler refuses to add a
debt to an amount, refuses to transfer a debt, and requires every discharge to
name the obligation it discharges.

In the funded profile a payment names its transfer, its allocation and the
obligation it discharges as separate identified objects:

```mori
emit Transfer { id: TransferId("T1"), from: Payer, to: Lender,
                settlementAsset: Cash, amount: cash };
emit Repay    { allocationId: AllocationId("Alloc1"), transferId: TransferId("T1"),
                obligationId: ObligationId("Due100"), payer: Payer, nominalAmount: nominal };
```

The Transfer is the cash moving. The Repay is the claim that this cash, under
this allocation, was applied to this obligation. They are linked by
`TransferId("T1")`, and the K definition checks the link: a Repay must name a
prior transfer with matching payer, creditor and asset, and the converted
settlement of the nominal amount is checked against what that transfer
actually funded. If the transfer moved 2 units of cash and the nominal payment
converts to 3, the repayment rejects with `INSUFFICIENT_UNALLOCATED`. A system
with one balance has no way to ask that question.

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

**The arithmetic, worked.** The `let` lines are the whole constant-product
formula in integers, and every intermediate is an integer the site can show.

```text
effective_input   = 10,000 × 997                     = 9,970,000
numerator         = 9,970,000 × 2,000,000            = 19,940,000,000,000
denominator       = 1,000,000 × 1,000 + 9,970,000    = 1,009,970,000
output_calculated = floor(19,940,000,000,000 / 1,009,970,000)
                  = 19,743   remainder 162,290,000
```

The fee is applied by scaling the input to 997 parts in 1,000 before it
touches the reserves, which is why 9,970,000 appears rather than 10,000. The
numerator multiplies that by the output reserve, and the denominator is the
input reserve scaled to the same 1,000 parts plus the effective input. The
only division in the action is the last line, and it is the only place a
non-integer could arise, so it is the only place a rounding decision exists.
The true quotient is a little over 19,743, and `floor_div` takes 19,743. The
next integer, 19,744, would need a numerator of at least 19,940,847,680,000,
which is more than the pool has, and that is exactly why asking for 19,744 as
a minimum fails at `guard arg.min_out <= output_calculated` with the message
"minimum output not met".

**Where the remainder goes, and why that is the argument.** The remainder,
162,290,000, is the part of the numerator that did not divide evenly. In
output units it is a fraction of one B, and the pool cannot pay a fraction.
Someone has to keep it. Rounding down means the trader receives 19,743 and the
fraction stays in the pool's reserve, which is what the policy block declares
in the line `remainder "unpaid output remains in reserve_b"`. Rounding up
would mean the pool pays 19,744, which is more than the formula entitles the
trader to, and the pool absorbs the difference as a loss. On one swap that
loss is under one unit and looks harmless. It is not harmless, because a
trader who can choose their input can choose one whose remainder is as large
as possible, and can repeat the swap as many times as the lifetime allows, and
every repetition extracts a fraction the formula did not grant. Over enough
swaps the direction of a single rounding is the difference between a pool
whose invariant grows with every trade and a pool that leaks. The post-trade
reserves here are 1,010,000 A and 1,980,257 B, and their product is
2,000,059,570,000 against 2,000,000,000,000 before, so the fee and the floor
together have grown the invariant. That is the correct sign. The rounding
direction is a security property because it decides which side of every trade
the remainder lands on, and a system that leaves the direction to whatever
`/` happens to do has not decided it at all.

**Reserve safety is not implied by the formula.** In real numbers the
constant-product output is always strictly less than the output reserve, so a
pool can never be emptied by a swap. In integers with a fee and a floor the
same inequality holds, and the guard `output_calculated < state.reserve_b`
still exists, because the semantics is meant to be checked rather than
believed. If a later change to the formula, or a different fee, or a bug in
the arithmetic ever produced an output equal to the reserve, the guard would
turn a destroyed pool into a named rejection. The guard is cheap and the
failure it prevents is total, and that trade is the reason it is there.

### 5.3 Declarations the language has

`unit`, `party`, `const`, `state`, `observation`, `settlement`, `status`,
`policy`, `reserve`, `effect`, `action`. Inside an action: `guard` (with a named
failure message), `let`, `set`, `emit`. Plus `lifetime` and `horizon` as
explicit bounds on the agreement itself.

A `unit` is a nominal denomination such as `USD_micro` or `AssetB_quantum`,
and every amount carries one. A `settlement` binds a unit to a ledger asset
and a quantum, so the evaluator knows how many nominal subunits make one
ledger unit. That conversion is exact: with a quantum of 10 a nominal 20
converts to 2, and a nominal 15 is refused rather than rounded. A `status`
rule derives whether the episode is closed and whether the agreement still
has anything outstanding, and they are kept separate so that a closed episode
never implies a settled agreement. In the loan, the settle action closes the
episode and the agreement still reports 4,500,000,000 outstanding. A
`reserve` declaration holds back the final unit of execution allowance for a
named closure action, so the swap's `guard remaining > uint(1)` line is what
keeps the pool from spending its last step on a trade and then being unable
to close. `lifetime` is the number of successful actions the instance may
ever execute, and `horizon` is the exclusive time bound past which no action
is accepted, and both are frozen at genesis where nothing can raise them.

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

**The loan interest, worked.** The derivation string is the formula the accrue
action computes, and the numbers are these:

```text
interest_numerator   = 5,000,000,000 × 8 × 31   = 1,240,000,000,000
interest_denominator = 100 × 365                = 36,500
interest_calculated  = floor(1,240,000,000,000 / 36,500)
                     = 33,972,602   remainder 27,000
```

The remainder of 27,000 over a denominator of 36,500 reduces to 54/73, and
that is the fraction of one micro-USD the `remainder` line says is discarded.
The policy is not describing the rounding from the outside; it is recording
the exact fraction that this sample threw away, so an independent reader with
a calculator can confirm the number and the direction. The `comparison` line
says how the result should be judged, and it is careful: the sample is
compared as an exact integer, and no claim is made that this satisfies an
ACTUS tolerance rule, because that claim would need the fixture, not just the
formula.

**What the checker does with a policy.** The `targets` clause lists the places
in the program the policy governs, as `write(action, field)` for a state write
and `effect(action, ordinal, field)` for an emitted effect field. The rule is
that every amount-valued write and every amount-valued effect field in the
program must be covered by exactly one policy target, and no target may
cover something that does not exist. Coverage is by occurrence, so copying an
amount, zeroing it or emitting it unchanged still needs a target. This is the
mechanism that stops rounding from being a side effect: an author cannot
write an amount anywhere without saying which policy governs it.

The `rounding` clause is checked, not merely recorded. `rounding none` means
the value written at the target must carry no division at all, and `rounding
floor(accrue, interest_calculated)` means the value must derive from exactly
the one `floor_div` in the named `let` and from no other. The evaluator tracks
this by attaching to every computed value the set of division nodes it passed
through, starting empty for literals, constants, arguments, observations and
committed state and growing as arithmetic combines values. A target declared
`none` whose value carries a division rejects. A target declared `floor` whose
value carries the wrong division, two divisions or none rejects. In the swap,
the provenance of `output_calculated` flows through the reserve and trader
writes and the output transfer, which is why the `swap_output` policy targets
each of them and the `swap_input` policy, which has no division, declares
`rounding none`.

The `derivation`, `remainder`, `comparison` and `proof` lines are documentary
strings. They are hashed into the program, so they cannot be changed without
changing the program's identity, but they are not parsed or compared against
the source. The `proof` line in particular does not prove anything by
appearing. It names a claim that the mandatory ContractInvariant evidence must
later discharge, and the atomic profile's claim root must include every policy
proof identifier, so a program cannot carry a policy whose proof obligation
was quietly left out of what the parties signed.

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

**Simulation against static analysis.** A simulation runs the agreement on one
input and shows what happened. A static analysis reasons about every input the
agreement could receive and shows what must happen. The loan simulation
produces 33,972,602 and closes the episode with 4,500,000,000 outstanding, and
that is evidence that the program did the right thing on that input. It is
not evidence that principal is preserved under every interest payment, because
the simulation did not try every payment. The property list on the workspace
is a list of claims about all inputs, and the workspace refuses to mark a
claim satisfied because one example went well. Both are shown side by side in
the workflow because a developer needs both, and the discipline is in never
letting one stand in for the other.

**A valid proof against a stale predecessor.** A proof is about a transition
from a specific pre-state, identified by hash. It shows that if the agreement
was in that state, then this action produces that post-state. It does not show
that the agreement is still in that state. If another transaction has already
been accepted from the same pre-state, the state has been consumed and the
proof, though valid, is about a state that no longer exists. The last step in
the workflow, checking live ledger state before submitting, is where this is
caught, and the property inventory lists it as one of the things a valid
history proof does not establish: that a previous state has not already been
spent. The atomic profile's evaluator enforces a related rule on every input,
that `revision + remaining == lifetime`, so a state that has been advanced
cannot be presented with a stale allowance.

The workspace is three columns: AGREEMENT (terms and parties, package version,
numeric profile, bounds and horizon, observations) · BEHAVIOR (event timeline,
calculation date, payment date, due/paid amounts, state changes) · TRANSACTION
(predecessor states, authorized action, asset movements and fees, signers and
expiry, proof and ledger status).

The behavior column is what the semantics says happens, with dates and
amounts. The transaction column is the ledger's view of the same step. A
developer who can see both at once can tell a semantic question from a
settlement question, and that is the distinction the whole toolchain is built
around.

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

The separation is easiest to see by asking what changes when. An intent is
signed once and does not change, and the hash the signature covers commits
only to the intent and to no plan. A plan is proposed afterwards, possibly by
someone other than the signer, and it names the intent it serves by hash, and
it can be replaced by a different plan for the same intent without touching
the signature. A receipt is produced after execution and names both. Systems
that conflate them typically sign the plan and call it an intent, so that
changing the route means signing again, or they sign a vague goal and let the
executor fill in everything else, so that the signer does not know what they
authorized.

Two signing profiles:

- **Exact-plan signing** — you sign the concrete plan. Narrower meaning, no
  solver choice after signing. This is the working restricted profile.
- **Outcome-intent signing** — you fix authority, goals, permitted agreement
  programs, validity and nonce *before* a plan exists. Solvers then search
  outside the finite checker; ranking cannot excuse invalid authority or a failed
  goal, and there is no global best-price claim.

Exact-plan signing gives up route choice for certainty. The signer sees the
exact action, the exact writes and the exact effects, and nothing else can
run. Outcome-intent signing gives up that certainty for route choice. The
signer fixes an envelope, and any plan inside the envelope may run, so the
signer does not know in advance which pool their swap will go through. What
makes the trade safe is that the envelope is checked by a finite, independent
checker and the solver's search happens outside it. A plan that exceeds the
signed budget or misses the signed goal is rejected however well it ranks,
and there is no claim that the plan chosen was the best available, because
"best" is a claim about the market and the checker only knows the envelope.

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

**The naive version, and where it fails.** Suppose an authorization system
tracks net spending: the principal grants an allowance of 100 units of an
asset, and the system checks that the principal's balance never ends more than
100 below where it started. This is the obvious design and it is the one most
people write first. Now a plan sends 100 to a recipient, the recipient sends
100 back, and the plan sends 100 to the recipient again. Net spending is 100,
the check passes, and 200 has left the principal's account in gross terms.
Repeat the refund and the system will authorize any gross flow at all as long
as the money keeps coming back. The failure is not that the principal loses
money in this trace. The failure is that every one of those intermediate
movements was authorized by a check that could not see it, and any one of them
could have gone somewhere else. A recipient who returns 100 and then receives
100 has been given the ability to route the principal's funds through
themselves without limit, and the principal signed nothing of the kind.

The gross rule closes this. For the signed principal, the checker sums every
outgoing transfer and every fee, by asset, across every recipient, and
compares the sum against the cap. A refund is an incoming transfer and adds
nothing to the outgoing sum, so it does not restore the cap. In the trace
above the second send takes the gross total to 200, and with a cap of 100 the
plan rejects. The design spec lists "refund cap evasion" as a required
negative control, which is the trace above run against the checker and
refused.

Fees are handled the same way and then some. A fee is money leaving the
principal, so it counts toward the gross total, and the fee cap is a separate
limit on top of that, so a plan cannot exhaust the gross budget through fees
alone and cannot pay a large fee within the gross budget while claiming the
fee was small. On the goal side, the rule is the mirror image: the goal is a
minimum net credit, final minus initial, and fees count against it, so a plan
whose output meets the minimum before fees and falls below it after fees has
not met the goal. "Net output reduced below minimum by a fee" is another
required negative control.

**Why authority is affine.** A capability that can be used twice is a
capability that can be replayed. Affine means at most once: a partial
completion consumes the authority it used, the residual is what is left and
no more, and nothing can grow it back or reset the epoch it belongs to. In the
atomic profile this is stark: any unused authority expires on commit and is
not a new reusable capability. In the later profiles with partial fills, the
receipt carries the residual forward and a subsequent step can use only that
residual. The staking-exit and vault-redemption lifecycles in the category
tabs use the same rule, which is why a partially completed exit consumes its
authorization and the remainder cannot grow.

**Why asset identity is a tuple.** A ticker is a name, and names can be
reused. Two tokens on different domains can both be called by the same
symbol, and a receipt token that represents a deposit of an asset is not that
asset. If the checker matched assets by name, an intent to receive one thing
could be satisfied by delivering another with the same name. The tuple
`{domain, issuer, reference, kind}` is the whole identity, the `kind` field
separates a token from a claim, and the design's negative controls include
"same ticker in a different domain" as a case that must reject.

**Why validity, nonce and domain separation.** The nonce makes each intent
single-use in a registry keyed by network, deployment, principal and nonce
rather than by intent hash, so editing and re-signing an intent does not give
it a fresh nonce. The domain separator on the signature stops a signature made
for one purpose from being presented as a signature for another, because the
bytes that were signed include the domain. The trust-record rule is the last
piece. An envelope carries a public key and a signature, and it would be
circular to let the envelope declare that its own key is the principal's, so
verification looks the principal up in the caller's own record of who holds
which key.

**Why pending has its own judgment.** A pending receipt describes a plan that
has performed a prefix of its steps. The prefix is held to every authority and
safety rule, and the receipt must show an allowed pending state, the residual
authority and the obligations still outstanding. What it may not show is the
terminal goal as achieved, because it has not been. The unified judgment in
the intents report gives `Complete` and `Progress` as separate relations for
exactly this reason.

**Why anti-vacuity.** A checker that rejects everything satisfies every safety
property trivially. Requiring trace inclusion says that every accepted trace
is one the semantics permits; requiring a feasible positive witness says that
at least one trace is accepted. The R2b evidence lists positive controls
alongside the negative ones, and the design says plainly that always rejecting
does not constitute a working compiler.

Wallet rendering is derived from the canonical signed meaning: asset domains,
gross budgets, allowed recipients, net goals, fees, validity/nonces, assumptions
and liabilities — a deterministic semantic signing summary, not a hex blob.

If the wallet showed a summary produced by the planner, a planner could show
one thing and submit another. The summary is instead recomputed from the
decoded canonical intent, so what the signer reads is a deterministic function
of what the signer signs.

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

**What "atomic" means at each layer.** Inside one ledger, a batch of
operations is atomic when the ledger applies all of them or none. That is a
property the ledger enforces, and it is real. A cross-chain swap has an
internal batch on the source ledger, a message or a custodial instruction that
crosses to the destination, and a withdrawal on the destination ledger. The
source batch can be atomic, the destination withdrawal can be atomic, and the
whole is not, because nothing enforces that the destination step happens
given that the source step did. When a system says the swap is atomic it is
usually describing the source batch and letting the reader assume the rest.
The first rule forbids that: atomicity is stated for the layer and the route
it actually holds on, and the bridge step gets its own honest statement,
which for the documented systems is that it completes asynchronously and may
require manual recovery.

A custody manifest for a route says who holds the assets at each step and
under whose authority they move. A user approving a cross-chain action is
approving those custody transitions, and a manifest published afterwards
cannot inform an approval that has already been given.

The decisive test case, which the site should state as a challenge: *construct a
cross-chain swap trace in which the internal batch succeeds but the bridge
withdrawal fails or is rolled back.* Any model that cannot represent that trace —
and reject it — is not modelling cross-chain settlement. Operator, relay,
treasury and bridge attestations cannot discharge a proof obligation.

The test is decisive because it is the trace that the atomic story says cannot
happen and the verifier documentation says can. A model of cross-chain
settlement has to be able to write it down: source batch applied, message
sent, destination withdrawal failed, and the user's assets now sitting in the
bridge's custody with a refund duty outstanding. In Moriarty's terms that is
DA23's pending message with an explicit refund duty, and the refund is a
`Debt`-typed obligation that survives until evidence of its discharge exists.
A model that can only represent success cannot represent the state the user
is actually in. The last sentence follows from Section 4.3: an attestation
from an operator, a relay, a treasury or a bridge is a signature, and a
signature proves that the signer said so. The proof obligation is that the
withdrawal happened, and only evidence of the withdrawal discharges it.

Bridges split into **message-verified** and **custodial** trust. Those are
different security stories and the site must not merge them.

A message-verified bridge fails when the verification is wrong, because a
forged or replayed message is accepted or a genuine one is rejected. A
custodial bridge fails when the custodian does, because the party holding the
assets is compromised, insolvent or unwilling. A facet that records only
"bridge" tells a reader nothing about which failure to plan for.

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

Unit confusion is the bug where the same integer is read as different assets
or as an amount and a time, and the type system refuses the addition, so the
program cannot be written. The rounding class is the bug where a share
conversion rounds in the depositor's favour on deposit and the vault's favour
on withdrawal, or rounds the same way on both when the standard says they
differ, and the drain is slow and legal. A declared policy makes the direction
a checked fact at every conversion, so ERC-4626's method-specific directions
become something the checker can confirm rather than something a reviewer has
to remember. The authorization class is the refund trace from Section 6, and
the history class is the invalid-initial-state case from Section 4.2, where a
reviewer who examines any one step finds nothing wrong.

Adversarial modelling requirement, from the surveyed literature: derive fixtures
with **separate actor capability, vulnerable layer, precondition, ordered
effects, violated predicate and loss outcome.** Two rules that follow:
**ABI shape does not establish semantic compatibility**, and **a valid oracle
signature does not establish economic truth.** Flash borrowing is a capability
with legitimate uses, not a vulnerability.

The fixture shape matters because an incident description that says "the
attacker drained the pool" cannot be turned into a test. A fixture that names
what the actor could do, which layer they used, what had to be true first,
what happened in order, which predicate was violated and what was lost can be
run against a model, and the model either represents and rejects it or it
does not. A contract that exposes the right function signatures is compatible
in shape and may be incompatible in meaning, so a composition check that stops
at the interface has not checked anything, and a price that arrives with a
valid signature is a price someone signed rather than a price that is right.
Flash borrowing belongs in the same list for the opposite reason. It is often
named as the attack when it was the vehicle, and a model that forbids it
forbids a legitimate atomic multi-leg settlement, which is DA08, without
removing the vulnerability the borrowed funds exploited.

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

Each sprint closes on an artifact that could not exist if the work were not
done, rather than on a report that says it is. SP06 closes on a real two-step
recursive proof whose retained bytes were verified in a separate process and
whose mutations were rejected, and SP05 closes on loan and swap effects
finalized on Preview and compared, effect by effect, against independently
derived expectations. The roadmap says these are delivery gates, not calendar
or compute estimates, and the site must not present them with dates.

Three parallel tracks run now: **SP01→SP02→SP03** (language),
**SP01 F0→SP04→SP06** (native feasibility and proofs), and
**accepted atomic + loan/swap subset→SP05** (financial Preview integration).

The tracks are parallel because their blocking questions are independent. The
language track asks whether the successor specification can be completed and
executed in K. The native track asks whether Midnight's recursive proof
backend can verify a Moriarty history at all, which is open after the first
experiment ran out of rows at k17. The Preview track asks whether the
already-accepted atomic subset can settle real loan and swap effects on the
public network. None needs the others' answer to make progress, and each has
an acceptance boundary where it must wait before the mandatory-proof sprint
can begin.

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

What each milestone establishes is narrower than it can be made to sound, and
the site should say so next to it. The K result means that on a fixed shape of
input, one transfer optionally followed by one repayment, the K definition,
the source-preparation path and an independent financial calculation agree on
every output number and on the exact code and index of every rejection. It is
a finite comparison, and the repository states that finite comparisons do not
establish source-to-Core-to-K correspondence, do not complete SP03, and do not
constitute mandatory proof-carrying data or finalized settlement. The guard
and step counts are control-presentation counts under the reduction
semantics, not K rewrites, execution fees or a mechanized termination proof.
The successor profile's tools cover syntax only; they do not type or execute
a partial payment. And the hello-world deployment shows that the pinned
Docker and SDK route to a Midnight network works end to end, with a contract
deployed, called and read back exactly; it does not run a Moriarty financial
relation, and it does not consume a Moriarty history proof. Each of these is
real evidence for a real claim, and the claim is the narrow one.

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
- **The fixed sets are fixed.** The composition operators, the proof claims, the
  facets, the families and the action targets are not rounded off, extended or
  trimmed. The test at `src/data/completeness.test.mjs` enforces this.
- **Do not count in prose.** See [`VOICE.md`](VOICE.md). The register and the ban
  on tallies govern every line of copy on the site.
- No tooling or vendor attribution anywhere in the site, its source, its
  comments, its assets or its commits. The work is the author's alone, and
  nothing that helped produce it is named in it.

The rule about assumptions is the one a builder is most likely to soften.
Every strong claim in this document arrives with the thing it depends on: the
closure theorem with its liveness assumptions, the oracle control with its
lying source, the K result with its finite domain. A site that keeps the claim
and drops the assumption will read as confident for about as long as it takes
a careful reader to notice, and then it will read as the marketing this
project is defining itself against.
