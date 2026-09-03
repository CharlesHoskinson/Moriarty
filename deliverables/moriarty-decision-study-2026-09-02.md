# Moriarty: a verified financial DSL for Compact and ZKIR

**Decision study and stakeholder packet — research cut 2026-09-02**

Moriarty is the new language name. “Marlowe” below refers only to the upstream
language, repositories, deployed Cardano system, and migration baseline.

## 1. Executive decision

Proceed with Moriarty as a **strictly finite, typed financial-agreement DSL with
a small normative Core and an ahead-of-time Compact backend**. Compile
Moriarty to reviewable Compact, then use the pinned Compact compiler to emit
TypeScript, ZKIR 3, and proof artifacts. Do not compile directly to ZKIR in the
first production release and do not market a production “DeFi kernel” until
proof cost, ledger cost, client verification, audit, and demand gates pass.

The enduring purpose is precise: Moriarty lets application developers deploy
finite financial state machines whose parties, assets, timeouts, value effects,
privacy disclosures, external assumptions, maximum lifetime, transition count,
and backend resource envelope are known before deployment.

The proposal is viable. A finite escrow lowering was compiled with Compact
compiler 0.34.100, language 0.26.0, runtime 0.19.100, ledger
9.1.0.0-rc.3, and the ZKIR 3 feature. It emitted three ZKIR 3.0 circuits, all
accepted by the current ZKIR mock compiler. This demonstrates a working
language-to-backend path, not production assurance.

Moriarty V2.0 should contain:

- a finite Core with closure, transfer, conditional, binding, assertion,
  ordered-any and atomic-all waits, explicit deadlines, typed diagnostics, and
  no recursion or runtime-sized iteration;
- a typed surface language with modules, definitions, templates, records,
  token-indexed amounts, timestamps, durations, ratios, compile-time schedules,
  and statically bounded loops that elaborate away;
- `public`, `private`, `committed`, and `revealed` visibility tracking;
- mandatory valid-initial-state, conservation, lifetime, transition, state,
  continuation, and resource certificates;
- canonical Core serialization and stable hashes;
- an ahead-of-time Compact backend, translation-validation manifest, generated
  source maps, and explicit Compact/ZKIR/ledger/runtime versions;
- a client-side transaction verifier and a stateless planner interface;
- V1 import, trace comparison, and side-by-side version routing, without any
  claim that immutable V1 Cardano contracts move chains.

Moriarty V2.1 should investigate partial/threshold action collection,
`EntryDeposit`-style dynamic participation, bounded runtime iteration,
fixed-depth Merkle account/continuation state, typed attested oracles,
authorization rotation/recovery, and capability-declared composition. Arbitrary
Compact calls, arbitrary minting, unbounded containers, general recursion,
implicit disclosure, and silent Runtime trust remain outside Core or are
rejected.

The primary users are Midnight DApp and fintech teams that need private or
selectively disclosed financial terms with an auditable state machine. The
first four target applications are private-term escrow, collateralized loan
servicing, milestone treasury/grant release, and atomic settlement.

DeFiFormal supplies the broader product test. Its 72-row corpus should not
become 72 language features or a flat twelve-way Core. Moriarty uses M2+M3—six
economic families plus Prediction over mandatory facets—for human navigation,
and M5 formal behaviors internally. M4+ and D01–D12 remain historical migration
and regression views. The primary suite has seven family demonstrations; 13
legacy patterns preserve coverage of D01–D11 and the two unrelated parts of
D12. Both views are specified in
[`moriarty-defiformal-category-strawmen.md`](moriarty-defiformal-category-strawmen.md).

A credible first-year team is 10–12 full-time equivalents: two language/compiler
engineers, two formal-methods engineers, two Compact/ZK/Midnight engineers, two
Runtime/SDK/tooling engineers, one security engineer, one financial-domain and
product lead, plus part-time independent audit and UX research. Planning range:
USD 2.5–4.0 million for 12 months and USD 4–7 million through 24 months. These
are staffing-model estimates, not vendor quotes.

The three largest unresolved risks are:

1. Compact, ZKIR, and ledger version churn can invalidate backend assumptions.
2. Ahead-of-time specialization makes compiler correspondence and artifact/key
   governance part of the trusted base.
3. Public evidence does not yet establish enough user demand to justify a new
   language rather than a Compact library.

**Decision confidence: medium-high for technical feasibility; medium for the
architecture; low-to-medium for product demand and economic practicality.**

The focused execution prompt and advisory record were added on 2026-09-03:
[`moriarty-defi-kernel-deep-research-prompt-2026-09-03.xml`](moriarty-defi-kernel-deep-research-prompt-2026-09-03.xml)
and
[`moriarty-agentic-council-record-2026-09-03.md`](moriarty-agentic-council-record-2026-09-03.md).
They add an early backend boundedness stop test and preserve audited Compact
libraries as a planned outcome if the distinct language has no measured
advantage.

## 2. Evidence inventory and status

The controlling assignment is preserved byte-for-byte at
[`../raw/assignments/modernizing-marlowe-assignment-2026-09-02.xml`](../raw/assignments/modernizing-marlowe-assignment-2026-09-02.xml),
SHA-256 `5984339c8da25b562a1cdd755971ae816614c315061eb8368b141c472de15964`.
The imported report is preserved at
[`../raw/sources/user-deep-research-report-6-2026-09-02.md`](../raw/sources/user-deep-research-report-6-2026-09-02.md),
SHA-256 `31c018ab6059d651a48aadbe1e9dd1bb3808d3b57900bc5b1a1c41eeb8cfd25b`.
Its internal `turn...` citations are not portable, so it is treated as a design
input rather than evidence.

The full acquisition ledger is
[`../evidence/source-inventory.csv`](../evidence/source-inventory.csv). Every
online search or page acquisition in this study used Scrapling. Git clones were
used only after source discovery and enumeration. Important inventories are:

- 36 Marlowe repositories:
  [`../evidence/repository-classification-2026-09-02.tsv`](../evidence/repository-classification-2026-09-02.tsv);
- 74 Midnight repositories:
  [`../evidence/midnightntwrk-repositories-2026-09-02.tsv`](../evidence/midnightntwrk-repositories-2026-09-02.tsv);
- exact Midnight clone heads:
  `../repos/midnightntwrk/clone-manifest.json`;
- all three LFDT Minokawa repositories, including active Compact source:
  `../repos/LFDT-Minokawa/clone-manifest.json`;
- current live Marlowe docs, acquired 100/100 from the working host:
  [`../evidence/marlowe-docs-live-acquisition-2026-09-02.json`](../evidence/marlowe-docs-live-acquisition-2026-09-02.json).
- the combined 36-repository/documentation graph evidence:
  [`../evidence/marlowe-org-full-graph-2026-09-02.json`](../evidence/marlowe-org-full-graph-2026-09-02.json);
- the locally reproduced DeFiFormal corpus, current family/facet crosswalk, and
  historical M4+ crosswalk:
  [`../evidence/defiformal-corpus-reconstruction-2026-09-02.json`](../evidence/defiformal-corpus-reconstruction-2026-09-02.json) and
  [`../evidence/defiformal-72-protocol-family-facet-crosswalk-2026-09-02.csv`](../evidence/defiformal-72-protocol-family-facet-crosswalk-2026-09-02.csv) and
  [`../evidence/defiformal-72-protocol-m4plus-crosswalk-2026-09-02.csv`](../evidence/defiformal-72-protocol-m4plus-crosswalk-2026-09-02.csv).

Status is attached to claims, not repositories:

- S0 historical or superseded;
- S1 discussion only;
- S2 written design;
- S3 prototype or experiment;
- S4 implemented and tested but unreleased;
- S5 released but not deployed or broadly supported;
- S6 deployed and supported;
- S7 deployed, independently audited, and operationally evidenced.

The Moriarty design is S2. The compiled escrow is S3. No Moriarty component is
S4 or higher. Marlowe V2 workshop proposals are S2 unless a separate pinned
implementation proves a higher status. The active
`marlowe-plutus/paluh/validator-enhancements` branch is a broad validator and
Runtime migration experiment; it is not evidence that `WhenAll`,
`EntryDeposit`, or a V2 language is implemented.

Material contradictions are tracked in
[`../wiki/contradictions.md`](../wiki/contradictions.md). The most important are
the stale Marlowe sitemap, Haskell/Isabelle interval ambiguity, missing Isabelle
Merkleization, the placeholder `marlowe-runtime-ng`, the Midnight Compact
repository relocation, Compact toolchain version tuples, and mixed ZKIR 2/3
artifacts. The user-supplied taxonomy report's public-access limitation is also
reconciled there: the authorized local checkout at
`8ae0bbfaa3193078d1cabf6999db1382985b7f95` reproduces 72 corpus rows and 60
construction specs. Those constructions contain 1,259 obligations, of which
570 are covered and 689 remain residue; none currently has a `COMPLETE`
verdict. This makes the corpus a requirements source and falsification suite,
not evidence of finished market coverage.

## 3. Current Marlowe architecture and semantics

The pinned practical V1 baseline is `marlowe-lang/marlowe-cardano` commit
`99f432d8ef9dbd1b52b7fa089254de15913b490f`. The reconstructed data model and
source locators are in
[`../wiki/marlowe-baseline.md`](../wiki/marlowe-baseline.md).

```text
Party      = Address | Role
AccountId  = Party
Payee      = Account AccountId | Party Party
Action     = Deposit | Choice | Notify
Case       = Case Action Contract | MerkleizedCase Action Hash
Contract   = Close | Pay | If | When | Let | Assert
State      = accounts + choices + boundValues + minTime
```

Values include available money, constants, negation, addition, subtraction,
multiplication, division, choices, interval start/end, bound variables, and
conditionals. Observations include Boolean connectives, choice existence, and
integer comparisons. Tokens are Cardano policy/currency-symbol plus token-name
pairs. Money is multi-asset value.

The executable flow is:

```text
author/template/SDK/Blockly/JSON
  -> instantiate, link, simulate, analyze, optionally Merkleize
  -> Core contract + state + continuation objects
  -> Runtime plan and coin/UTxO selection
  -> wallet review and signatures
  -> Cardano transaction
  -> semantics validator
  -> new continuation/state or payout output
  -> indexing, rollback handling, later progression or payout
```

Within a transaction the evaluator fixes the interval, reduces internal steps
to quiescence, applies the first matching case for an input, and repeats. Case
ordering is semantic. Close creates abstract refunds. Merkleization replaces a
continuation with a hash and moves the body off chain. Object bundles add named
link-time reuse but do not turn Core into a typed modular language.

Warnings are non-positive deposit, non-positive payment, partial payment,
shadowed bound value, and failed assertion. Errors include ambiguous, invalid,
or past intervals; no matching input; useless transaction; and continuation
hash mismatch. Moriarty must preserve attribution and severity rather than
reproducing these only as strings.

Authorization differs by backend. Marlowe addresses use payment credentials;
roles use possession of role tokens; open roles add a separate validator;
payouts are protected by the payout validator. Correctness depends on the role
token monetary policy and registry assumptions, which are not consequences of
the abstract semantics. Moriarty replaces role-token assumptions with explicit
typed credential/capability rules and never claims that a witness callback is
automatically an authenticated wallet signer.

The V1 trusted base varies by workflow and includes source constructor, linker,
serializer/hash implementation, analysis/simulator, Runtime planner and index,
wallet, registered validator/payout scripts, continuation availability, Cardano
ledger, and web application. This is much larger than the abstract semantics.

## 4. Formal guarantee audit

The Isabelle baseline is pinned at `marlowe-lang/marlowe` commit
`7b5b1e90c171eae2674a6fc08aa7d8caa92b16af`. Exact theorem excerpts are in
`../raw/research-notes/isabelle-major-theorems-7b5b1e90.txt`. The local
`marlowe-spec-test` run passed 59 tests; its generator-distribution warning
qualifies, but does not invalidate, that result.

The complete working matrix is
[`../wiki/formal-assurance.md`](../wiki/formal-assurance.md). The decision-grade
summary is:

| Claim | Defensible formulation | Gap for Moriarty |
|---|---|---|
| Termination | V1 reduction terminates because finite syntax decreases | Prove the new Core measure and surface elaboration finiteness |
| Lifetime | A finite timeout maximum exists for well-formed finite syntax | Prove each deployable manifest has a finite absolute bound |
| Conservation | Successful V1 transactions preserve money under valid-state assumptions | Relate Core accounting to actual Midnight receive/send/mint impacts |
| Positive accounts | Valid transactions preserve valid positive accounts from a valid state | Constructor and every backend must enforce initial validity |
| Eventual closure | Certain semantic traces reach Close under formal premises | No guarantee of prover, witness, participant, continuation, or ledger liveness |
| Quiescence/idempotence | Pinned semantics proves reduction properties | Preserve across action-set and privacy additions |
| Split inputs | Conditional equivalence exists for suitable V1 inputs | Atomic-all deliberately changes grouping semantics |
| Transaction bound | Syntax yields an abstract bound | Add circuit, state, proof, and transaction-resource bounds |
| Authorization | Evaluator checks address/role conditions | Prove the chosen Midnight credential/nullifier model |
| Hash consistency | Implementations hash continuations and scripts | Specify canonical Core/package/artifact hashes across languages |

The normative strategy is one mathematical Moriarty Core specification, one
small executable reference interpreter, and at least two independently
maintained implementations checked by conformance vectors and differential
tests. Use the proof environment for which two maintainers commit before the
implementation starts. Reuse Isabelle results through theorem correspondence
where practical. Agda extraction is not selected by default because the current
Marlowe Agda code is experimental and uses explicit termination escapes.

Proof obligations frozen before V2.0 implementation are: type preservation;
progress or typed terminal error; determinism; termination; lifetime and
transition bounds; non-negative accounts; conservation excluding declared mint
effects; quiescence and idempotence; atomic-all order independence;
ordered-any determinism; timeout progress; authorization soundness under a
stated credential model; visibility noninterference/declassification policy;
canonical serialization injectivity; continuation integrity; valid initial
state; surface elaboration soundness; and Core-to-Compact step correspondence.

## 5. Repository and implementation archaeology

The Marlowe organization had 36 public repositories at acquisition. The exact
classification, head dates, commits, languages, issue counts, releases, and
replacement observations are in the TSV inventory. Recommended dispositions:

- retain `marlowe` as the frozen V1 semantic/conformance source;
- retain `marlowe-cardano` and `marlowe-plutus` for V1 LTS and Cardano migration
  evidence, with validators and registries versioned independently;
- retain `marlowe-agda` as experimental proof/reference work until it meets
  normative gates;
- do not represent `marlowe-runtime-ng` as a replacement until it contains an
  implemented vertical slice;
- put the TypeScript SDK, Runtime API schemas, CLI, explorer, and UI in V1 LTS
  mode unless a funded owner accepts them;
- archive or label superseded playground/runner/example material explicitly;
- move ACTUS generation and oracle protocols into versioned packages and
  extension protocols, not Core.

The Midnight organization had 74 public repositories; active Compact source is
now in the three-repository LFDT Minokawa organization. The detailed analysis is
in
[`../wiki/midnight-repositories.md`](../wiki/midnight-repositories.md). Key
observations are:

- `midnightntwrk/compact` is an S5 release mirror even though the API archive
  flag is false;
- `LFDT-Minokawa/compact` is a large integrated compiler/runtime/spec/tooling
  source at commit `11e7ec5abeecb99297c4faa74d30ef9adc7b51f3`;
- `midnightntwrk/midnight-zkir` default branch `zkir-v3` is at
  `2ffe2d17bbb736aec36fb300aeaca679a10d2278`;
- that ZKIR workspace pins ledger dependencies to
  `9a8777c4d035fc7f38ae286bcf5f8656668efd9f`, showing tight backend coupling;
- `compact-js` and `platform-js` are S1 template placeholders despite recent
  push dates;
- code-only Graphify extraction across the key Midnight repositories produced
  26,750 nodes and 78,550 edges; direct source inspection remains necessary for
  semantics because generated/vendor trees and parser coverage distort graph
  centrality.

The target Moriarty repository is one monorepo until conformance stabilizes:

```text
spec/                 normative prose, math, and proof sources
core/                 reference types, evaluator, canonical encoding
surface/              parser, types, elaborator, formatter, LSP
backends/compact/     Compact generator and translation validator
conformance/          vectors, differential runners, mutations, fuzzing
runtime/              stateless planner and client verifier
sdk/typescript/       generated and hand-written application API
tools/                 analyzer, simulator, debugger, provenance, migration
examples/              representative suite and pilots
registry/              schemas only; signed deployed artifacts live separately
docs/                  versioned language and operational documentation
mips/                  public proposals and decision records
```

Do not hand-maintain Core type/serialization definitions in each language.
Generate codecs and API types from the normative schema, then retain independent
evaluators for differential confidence.

## 6. Empirical contract and cost study

The structural benchmark is reproducible from
[`../scripts/run_structural_benchmarks.py`](../scripts/run_structural_benchmarks.py)
and its result
[`../experiments/structural-benchmarks-2026-09-02.json`](../experiments/structural-benchmarks-2026-09-02.json).
It covers pinned escrow, collateral escrow, swap, bond, revenue loan, covered
call, oracle and non-oracle CFDs, ACTUS PAM, and NFT financing examples.

Recurring unrolling is linear: 10/100/1,000 periods produce
3,687/36,807/368,007 canonical JSON bytes, 241/2,401/24,001 nodes, and
depth 42/402/4,002. A 1,000-account state serializes to 85,068 bytes and entails
1,000 abstract refunds. These results explain the pressure for schedules,
iteration, compression, and staged closure; they do not prove that a particular
optimization is cheaper on Midnight.

The current Cardano reference profile at observation time had 16,384 maximum
transaction bytes, 16.5 million transaction execution memory, 10 billion
steps, fee coefficients 44/byte plus 155,381, and 4,310 coins per UTxO byte.
These moving values must be refreshed for any Cardano comparison.

The Moriarty escrow experiment and precise output hashes are in
[`../wiki/benchmarks.md`](../wiki/benchmarks.md). Compact emitted ZKIR circuits
of 3,995, 5,657, and 5,330 bytes for fund, release, and refund. Current ZKIR
mock compilation reported `(k=13, rows=2072)`, `(k=13, rows=2127)`, and
`(k=8, rows=189)`. Six acceptance checks and 44 ZKIR library tests passed.

Not yet reproduced: proving-key size, proof latency/memory, verifier cost,
Midnight transaction bytes/fees, live deployment, wallet configurations,
rollback, or an end-to-end malicious-planner rejection. These are mandatory
gates, not optional future polish.

## 7. Language usability and target users

The existing public evidence does not justify “non-programmers want to author
smart contracts” as the principal thesis. Marlowe contracts are commonly
constructed through JSON, Haskell/TypeScript/JavaScript, templates, and visual
tools; Runtime, wallets, Merkleization, roles, payouts, and ledger constraints
still require engineering knowledge. Blockly remains valuable for explanation
and bounded template editing, not as proof of a no-code market.

Moriarty is primarily for DApp/fintech developers and security-sensitive product
teams. Financial experts participate as template designers, reviewers, and
policy owners. Auditors need stable Core traces and proof assumptions;
operators need manifests and observability; legal/risk teams need human-readable
obligations, disclosure policies, and exception paths.

The required user study compares Moriarty surface, raw Compact, TypeScript
construction, Marlowe V1, and a visual view on the same escrow, loan schedule,
and milestone contract. At least 5 participants in each of developer and
financial-domain cohorts must complete authoring/review tasks. Measure completion
rate, time, semantic errors, ability to identify timeout/payout/disclosure, and
comprehension of analyzer counterexamples. No usability claim in this report is
marked reproduced.

## 8. Audit of existing Marlowe V2 proposals

The full workshop report is preserved as SRC-0009, SHA-256
`b9e630ca0ef32fad39d432045790a324049b2e30ef47bbb793ab9deca520f1cb`.
The dispositions below are reassessed for Moriarty rather than copied from its
V2.0/V2.1 headings.

| Proposal | Problem and current workaround | Moriarty disposition | Status and obligations |
|---|---|---|---|
| Timeout closes by default | Non-close timeout paths obscure liveness; authors write explicit Close | **Revise/retain V2.0:** surface default is close, Core always stores explicit fallback | Marlowe design S2; prove timeout determinism and migration policy |
| All/any action sets | V1 ordering and sequential submission are verbose or racy | **V2.0:** ordered-any plus atomic-all only | Prove determinism, atomicity, timeout interaction; benchmark multi-party proving |
| Threshold/partial sets | Multi-party collection across transactions | **V2.1** | Requires intermediate state, replay, cancellation, privacy, and fairness semantics |
| `EntryDeposit`/open role | Dynamic participant joins currently need special validator | **V2.1 revised:** typed join capability, not Cardano role token | Credential binding, duplicate/replay, refund, revocation, and identity proofs |
| State compression | Datum/state growth | **V2.0 bounded profile:** fixed cells/roots; **V2.1 general proof-backed root** | Measure proof cost and availability; compression may relocate cost |
| Improved Close | High fan-out refund exceeds transaction limits | **V2.0 cap plus deterministic payout queue** | Prove no loss/duplication and terminal accessibility; benchmark queue griefing |
| Bounded iteration | Repeated payments unroll linearly | **Surface V2.0 unrolling; Core V2.1 only** | Prove bound, analyzer abstraction, circuit cost, and lifetime effects |
| Hash/crypto primitives | Oracles, commitments, credentials need crypto | **Restricted library/backend boundary** | Domain separation, canonical bytes, backend correspondence, cost |
| Fine-grained value kinds | Integer-only surface permits unit/token/time confusion | **V2.0 surface and manifest** | Type preservation through erased Compact/ZKIR representation |
| Token-aware arithmetic | Cross-token arithmetic errors | **V2.0 surface** | Token-indexed amount; only explicit conversion via attested rate |
| Time/duration arithmetic | Absolute integer times are hard to author | **V2.0 surface** | Business calendars compile off chain; Core stores absolute ledger time |
| Minting language | Financial flows may issue assets | **Outside Core; V2.1 restricted capability experiment** | Minting has separate conservation and policy audit boundary |
| Oracle improvements | `Choice` conventions omit identity, freshness, unit, replay | **V2.1 typed attestation protocol** | Source/feed/unit/time/freshness/nonce/signature/bounds/fallback |
| Runtime changes | Existing services are heavy and trusted by many workflows | **V2.0 stateless planner + local verifier** | Malicious planner, rollback, idempotency, multi-provider tests |
| Validator restructuring | Universal validator centralizes audit but can be costly | **Use specialized circuits with common Core evidence** | Universal bounded interpreter remains a measured alternative |
| L2/portable execution | Repeated transitions may benefit from other environments | **Deferred** | Need concrete demand and backend-specific security/liquidity model |

## 9. Comparative patterns and architecture options

The comparative corpus is preserved under
`../raw/sources/comparative-primary-2026-09-02/`. Transferable ideas are narrow:

| System or research family | Transfer to Moriarty | Do not transfer |
|---|---|---|
| Aiken | Integrated formatter, tests, diagnostics, LSP, typed Cardano data | General validator expressiveness inside Core |
| DAML/Canton | Parties, choices, obligations, workflow-centric views, privacy participants | Runtime/operator trust assumptions without an explicit Midnight model |
| ACTUS/Findel | Contract templates, schedules, cash-flow algorithms, compositional financial vocabulary | Unbounded or calendar-dependent evaluation in consensus semantics |
| Move/Scrypto | Resource and asset distinctions, modules/packages, explicit authority | General mutable object programming in Core |
| Scilla | Separation of computation, state update, and communication | Its full language/runtime model |
| Pact | Human-readable authorization and governance lessons | Unbounded general contract language features |
| Simplicity | Small analyzable core, explicit resource analysis, formal semantics | Bitcoin-specific execution model |
| BitML/session/typestate research | Explicit protocol states, participants, choice and liveness dependencies | Claims of fairness without matching ledger and participant assumptions |
| Verified compilers | Translation validation and proof-producing artifacts | Treating a large evolving backend compiler as already verified |

The original Marlowe architecture scorecard is retained as a baseline:

| Marlowe option | Assurance | Ledger | Expression | Authoring | Migration | Runtime | Demand | Maintenance | Interop | Weighted |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| V1 stabilization | 4.5 | 3.0 | 1.5 | 2.5 | 5.0 | 3.0 | 2.0 | 4.0 | 2.5 | 3.24 |
| Conservative V2 | 4.0 | 3.5 | 3.0 | 3.2 | 4.0 | 3.5 | 2.8 | 3.3 | 3.0 | 3.45 |
| **Verified Core + surface** | **4.7** | **4.0** | **4.2** | **4.5** | **3.5** | **4.2** | **4.0** | **3.2** | **3.8** | **4.11** |
| Composable protocol DSL | 3.5 | 3.2 | 4.8 | 3.8 | 2.5 | 3.5 | 4.0 | 2.5 | 5.0 | 3.61 |
| Portable/L2-first | 3.2 | 4.2 | 3.8 | 2.8 | 2.5 | 2.8 | 2.5 | 2.0 | 4.3 | 3.20 |

The Midnight-specific scorecard is in
[`../wiki/decision.md`](../wiki/decision.md). Ahead-of-time verified Core to
Compact scores 4.00, ahead of portable multi-backend Core 3.70, direct ZKIR
3.51, universal interpreter 3.42, and a Compact library 3.30. Values are design
judgments with approximately ±0.4 uncertainty on backend economics and demand.

The winning hybrid is exact: normative portable Moriarty Core; first supported
backend is ahead-of-time generated Compact/ZKIR; an explicit optional
composition layer comes later; no promise of other chains until a funded pilot
needs one.

## 10. Preliminary Moriarty V2 specification

### 10.1 Core syntax

The following abstract grammar is a design target, not yet normative:

```text
CoreContract C ::=
    Done
  | Transfer Account Payee Asset Amount C
  | Branch Observation C C
  | Await WaitMode [Case] Deadline C
  | Bind ValueId Expr C
  | Require ObligationId Observation C

WaitMode ::= OrderedAny | AtomicAll
Case     ::= Action ActionId C | HashedCase Action ActionId CoreHash
Action   ::= Deposit Account Party Asset Amount
           | Choose ChoiceId Party Range
           | Attest OracleSchema Party Digest

State ::= {
  balances, choices, bindings, minTime, phase, sequence,
  continuationRoot, obligationRoot, capabilityRoot
}
```

`Await` always contains an explicit deadline and fallback; surface syntax uses
`Done` when omitted. `OrderedAny` applies the lowest declaration-order matching
action and rejects ambiguity that cannot be resolved by that rule.
`AtomicAll` accepts exactly one input for every case in the same transition,
evaluates each action against the pre-state, rejects conflicting deposits or
choice identifiers, and commits all effects atomically. Partial and threshold
collection are not V2.0 semantics.

`Transfer` debits only available positive balance. If a contractual amount
exceeds the account, the Core produces a typed `PartialPayment` obligation or a
hard failure according to an explicit source policy; there is no implicit
warning mode. `Done` creates deterministic pull-based payout claims, bounded by
the deployment profile, rather than requiring unbounded fan-out in one step.

### 10.2 Surface sketch

```text
module      ::= "module" Name Version "{" declaration* "}"
declaration ::= import | asset | party | oracle | template | contract | test
contract    ::= "contract" Name typeParams? params effect? block
statement   ::= pay | await | require | let | if | forStatic | emit | close
type        ::= Bool | Int | Nat<bound> | Amount<asset> | Ratio<scale>
              | Timestamp<clock> | Duration<unit> | Party<capability>
              | Oracle<type,unit> | Hash<schema> | Record | Enum | Vector<n,t>
visibility  ::= public | private | committed | revealed
```

Surface functions and macros are total compile-time construction only. Imports
are content-addressed and version-locked. Generics are monomorphized. Records,
patterns, schedules, and business calendars elaborate away. A loop bound must
be a compile-time natural or fixed vector length. Recursion is rejected.

### 10.3 Kinds, effects, and visibility

Core kinds distinguish value, asset, unit, time, credential, digest, and
visibility. `Amount<USD>` cannot add to `Amount<NIGHT>`. A conversion requires a
typed rate whose source, time, unit, precision, and rounding are explicit.
`Timestamp<ledger>` plus `Duration<seconds>` is valid; timestamp plus amount is
not. Calendar rules and daylight-saving behavior run in schedule generation and
yield absolute ledger timestamps recorded in Core.

Effects are a closed set: receive, transfer, payout-claim, read-ledger-time,
read-attestation, reveal, and declared external capability. V2.0 deployable Core
rejects mint and external-call effects. The effect summary is hashed into the
manifest.

Private values may flow through private computation or commitments. Flow to
public state, public result, or cross-contract call requires an explicit
`reveal` with an audience and purpose. The Compact backend lowers an approved
reveal to `disclose()`. A generated `disclose()` without a corresponding Core
visibility transition is a translation-validator failure.

### 10.4 Elaboration and canonical form

Elaboration performs, in order:

1. resolve signed, content-addressed packages and lock versions;
2. parse, kind-check, type-check, and visibility/effect-check;
3. instantiate templates and monomorphize generics;
4. resolve business calendars off chain and require an explicit time anchor;
5. unroll compile-time loops/folds and reject recursion;
6. normalize expressions and assign stable action/obligation identifiers;
7. link and hash continuations using canonical Core encoding;
8. calculate lifetime, transition, accounts, payouts, state, continuation, and
   backend resource bounds;
9. emit Core, source map, explanation, assumptions, warnings, and certificate;
10. generate Compact and validate trace/effect correspondence.

Canonical source is formatted text for review, not identity. Canonical Core is
deterministic binary encoding with an explicit schema identifier and rejection
of unknown consensus fields. JSON is a diagnostic interchange view, not the
long-term hash basis. The manifest identifies surface, Core, encoding, backend,
Compact compiler, runtime, ZKIR major/minor, ledger, and proof-parameter set.

### 10.5 Operational semantics

Given valid state `s`, ledger time `t`, contract `c`, and an atomic input set
`i`, `step(s,t,c,i)` first performs deterministic internal reductions until
quiescence. It then either applies the selected wait policy, takes the timeout
fallback if `t` is at or after the deadline, or returns a typed not-enabled
result. The next sequence number is exactly the previous number plus one. Every
receive/send/payout effect is returned in an effect multiset that the backend
must implement exactly. No Core rule performs ambient wallet, oracle, or network
access.

### 10.6 Resource certificate

Every deployable artifact contains exact structural maxima and backend estimates:

```text
max_lifetime
max_transitions
max_actions_per_transition
max_internal_reductions
max_balances / choices / obligations / payouts
max_continuation_bytes and fixed Merkle depth
per-entry-point ZKIR version, k, rows, public/private inputs
proving-key hash/bytes, verifier-key hash/bytes
measured proof p50/p95 time and peak memory on named hardware
ledger-state bytes and transaction-effect upper bound
```

V2.0 profile candidates—not language constants—are 16 actions per wait, 32
accounts, 32 pending payouts, 4,096 transitions, and 64 KiB canonical Core.
Measurements may force lower limits. No profile ships until its complete
representative suite retains at least 25% resource headroom.

### 10.7 Before and after examples

The examples are surface sketches; each elaborates to finite Core.

```text
// Escrow
contract Escrow(buyer, seller, Amount<T> price, deadline) {
  await deposit buyer -> escrow price until deadline;
  await buyer.release -> pay seller price until deadline else pay buyer price;
}
```

```text
// Atomic swap
atomic all {
  alice deposits 10 Amount<TokenA>;
  bob deposits 20 Amount<TokenB>;
} until expiry;
pay alice 20 Amount<TokenB>; pay bob 10 Amount<TokenA>;
```

```text
// Zero-coupon bond
await issuer deposits principal until issueDate;
pay investor principal;
await investor deposits faceValue until maturity else require Default;
pay issuer faceValue;
```

```text
// Recurring loan: surface loop, finite Core expansion in V2.0
for static payment in amortize(terms, periods = 24) {
  await borrower deposits payment.amount until payment.due + grace;
  pay lender payment.amount;
}
```

```text
// Covered call
await writer deposits collateral and holder deposits premium until open;
await ordered any { holder.exercise; timeout(expiry) };
branch exercisePrice <= attest priceFeed fresh 5m { settle } { refund };
```

```text
// CFD with typed oracle
let px: Oracle<Amount<USD>, USD_per_asset> =
  attest FeedX at settlement fresh 60s range [floor, cap];
pay long payoff(px); pay short collateral - payoff(px);
```

```text
// Revenue split: fixed participant vector
contract Split<const N>(members: Vector<N, Share>, Amount<T> revenue) {
  require sum(members.share) == 100%;
  for static m in members { pay m.party revenue * m.share; }
}
```

```text
// Milestone grant
for static milestone in schedule {
  await grantee.proof and committee.approval until milestone.review;
  pay grantee milestone.amount else pay treasury milestone.amount;
}
```

Compared with Marlowe V1, these examples name domain values, prevent cross-unit
arithmetic, express schedules without hand-built nested JSON, and expose oracle
and privacy assumptions. Compared with raw Compact, they have a uniform
financial transition model and predeployment certificate.

## 11. Platform, Runtime, API, and tooling architecture

```text
IDE / visual inspector / CI
  -> Moriarty compiler + analyzer + simulator
  -> canonical Core + certificate + Compact + source maps
  -> independent translation validator
  -> compactc + ZKIR/key generation
  -> signed artifact registry
  -> stateless transaction/proof planner
  -> local client verifier
  -> wallet/custody signer
  -> Midnight node
  -> replaceable index/event provider
```

The planner may select inputs, estimate fees, assemble public effects, fetch
continuations, and request witness/proof work. It does not receive authority to
change contract meaning. Before signing, the client independently verifies:

- network, ledger, contract address, Core hash, source provenance, and versions;
- current sequence/phase and selected action identifier;
- public inputs and disclosed values against the visibility policy;
- every receive, send, mint, payout, destination, token, and amount;
- deadline and oracle source/freshness/nonce;
- continuation and package hashes;
- expected next state/effect digest;
- fee ceiling and proof/key/registry hashes.

APIs are versioned OpenAPI plus event schemas with idempotency keys, explicit
rollback events, deterministic request hashes, bounded retries, and typed
generated clients. The planner is stateless where possible. Index and chain
providers are replaceable, and clients can compare multiple providers.
Continuation storage is content-addressed and replicated; users can export a
complete availability package.

The authoring environment includes canonical syntax, formatter, linter, LSP,
completion, source maps, visual execution traces, time-travel simulation,
symbolic paths, counterexamples, property tests, semantic diffs, cost/bound
visualization, package lock/signature verification, deployment manifests, and
SBOM/provenance. Visual editing operates on typed surface AST and must pass a
round-trip equivalence check.

AI assistance may generate or explain Moriarty source, but the model is outside
the trusted core. AI output passes the same parser, type/visibility checker,
elaborator, analyzer, simulator, compiler, conformance suite, and human approval
as any other source. Explanations link to Core nodes, assumptions, counterexample
traces, and certificate entries; natural-language assurance is never a proof.

## 12. Security plan and threat model

The concise threat matrix is
[`../wiki/security.md`](../wiki/security.md). The full release review uses this
structure for every item: asset, adversary, boundary, precondition, path,
consequence, detection, prevention, recovery, and residual risk.

| Threat family | Mandatory controls and recovery |
|---|---|
| Credential/policy misconfiguration | Domain-separated credential commitments, signer/capability manifest, rotation and recovery path, deployment dry run |
| Counterfeit artifact or registry | Reproducible build, independent hash recomputation, threshold-signed registry, delayed revocation and incident bulletin |
| Invalid state/arithmetic/unit error | Constructor validity proof, refined/token-indexed types, checked arithmetic and rounding, negative tests |
| Timeout ambiguity/liveness | One ledger clock, explicit inclusive boundary, permissionless timeout entry point, prebuilt recovery plan |
| Continuation substitution/loss | Canonical hash plus proof path, replicated store, export package, availability alarm; hash does not solve loss |
| Oracle staleness/equivocation/replay | Typed signed statement, feed/source/unit/time/freshness/sequence/bounds, multi-source or fallback policy |
| External composition | Excluded V2.0; later effect/capability manifest, code-hash allowlist, separate assurance badge and kill/recovery plan |
| Runtime/transaction substitution | Local Core/effect recomputation, destination/token/amount/fee/version UI, hardware-wallet digest where supported |
| Web/wallet compromise | CSP, dependency pinning/SBOM, isolated signing origin, human-readable intent, withdrawal/fee caps, incident revocation |
| Resource denial | Static profile caps, proof/transaction preflight, admission control, cost regression gates, staged payout without unbounded work |
| Rollback/API skew | Confirmed/finalized state model, compensating event, idempotent requests, explicit version negotiation |
| Serialization/compiler disagreement | Canonical vectors, duplicate independent decoders, differential/mutation/fuzz tests, pinned build |
| Governance capture/upgrade confusion | Public MIPs, named owners, supermajority semantic changes, delayed activation, immutable artifact history |

Independent audits are separately scoped to Core/spec/proofs; parser/type
checker/elaborator; backend/translation validator; generated circuits and key
process; Runtime/client verifier; SDK/UI; and oracle/composition extensions.
No umbrella audit statement may imply coverage beyond its signed scope and
commit hashes.

## 13. Conformance, verification, and release gates

The assurance pipeline is:

```text
normative Core prose + math
  -> executable reference semantics
  -> mechanized proofs
  -> canonical vectors
  -> independent interpreters
  -> differential/property/mutation/fuzz tests
  -> Compact lowering + translation validator
  -> ZKIR parse/model/proof tests
  -> live-ledger and wallet-state tests
  -> reproducible signed release
```

Generators create correlated valid and invalid states, contracts, inputs,
visibility annotations, capabilities, and oracle statements. Fuzz targets are
parsers, canonical encoders/decoders, linkers, package resolver, source maps,
manifest verifier, Runtime APIs, and generated clients. Mutation tests must
demonstrate that the suite catches changed timeout inclusivity, reordered any
cases, omitted balance debit, altered recipient, missing witness constraint,
unauthorized reveal, stale oracle, duplicate sequence, and continuation swap.

Numeric V2.0 release gates:

- zero unresolved normative ambiguity;
- 100% Core grammar/rule coverage by conformance vectors and 100% maintained
  implementation agreement;
- zero known nondeterministic canonical encodings over at least one million
  generated round trips;
- all listed safety proofs checked without admitted axioms specific to the
  claims, or each absent proof explicitly blocks production status;
- 100,000 correlated differential traces with no unexplained divergence;
- at least 10 million parser/serializer fuzz executions and seven continuous
  days without a unique crash before release candidate;
- mutation score at least 90% overall and 100% for conservation,
  authorization, timeout, disclosure, and recipient mutations;
- every representative contract fits the chosen profile with at least 25%
  headroom in proof rows, state, transaction, and supported limits;
- proof generation p95 at most 30 seconds and peak memory at most 4 GiB on the
  published reference machine for ordinary transitions; exceptions require a
  separate “heavy” profile and explicit UX;
- no unresolved critical or high audit findings and all medium findings have an
  owner and documented residual risk;
- malicious planner suite rejects 100% of altered recipient, amount, token,
  fee, version, proof-key, Core-hash, disclosure, and next-state cases;
- V1 equivalence is reproduced for every contract claimed mechanically
  compatible;
- documentation, binaries, schemas, registries, and source maps share one
  signed release manifest;
- two non-toy pilot applications complete testnet rehearsals and one completes
  an incident/rollback exercise.

## 14. Migration and compatibility

Existing V1 Cardano contracts and validators are immutable and remain on V1.
Moriarty migration means source/design translation or a new Midnight deployment,
never moving an existing UTxO or rewriting history.

The migration tool classifies contracts as:

1. **Mechanically trace-equivalent:** supported V1 syntax, explicit closing
   timeouts, fixed roles/addresses mapped by a declared credential policy, no
   backend-specific behavior that changes traces.
2. **Policy decision required:** non-Close timeout, role-token transferability,
   partial-pay warning policy, address/role mapping, visibility, or payout model
   needs human choice.
3. **Representable but changed:** rewritten with atomic-all, private state,
   staged payout, typed oracle, or different authorization.
4. **Do not translate:** active on-chain contracts, unsupported state, missing
   continuation, irreconcilable backend assumption, or no economic reason.

The translator parses canonical V1, validates initial state, emits Moriarty
surface/Core plus an assumption and difference report, explores bounded traces,
and invokes an equivalence checker over observable actions, payments, warnings,
timeouts, and closure. State snapshots may seed a new deployment only through a
separately authorized settlement/migration transaction. Dry run shows both
chains' effects and never signs automatically.

V1 and Moriarty coexist through distinct language/Core/backend/validator
identifiers. Explorer and SDK route by manifest. V1 continuation stores remain
read-only compatible. V1 receives published LTS dates and critical-security
support; no deadline is announced until inventory and user contact identify
affected contracts. Rollback means abandoning the new unsigned/deployment plan
or using the new contract's explicit cancellation/settlement path—not reversing
confirmed V1 history.

## 15. Adoption and product strategy

Positioning:

> Moriarty is the high-assurance, privacy-aware financial state-machine layer
> for Midnight applications that need finite agreements, analyzable value
> flows, explicit liveness assumptions, and predeployment resource evidence.

| Segment | Current alternative | Reason to choose Moriarty | Success evidence |
|---|---|---|---|
| Escrow/marketplaces | Raw Compact or application custody | Standard finite deposit/release/refund semantics and local transaction verification | Two integrations; lower audit/implementation time; zero ambiguous timeout tests |
| Treasury/grants | Multisig plus bespoke workflow | Typed milestones, bounded approvals, deterministic refund/payout, selective evidence | One real program completes lifecycle and incident drill |
| Loans/bonds | Bespoke Compact/ledger application | Schedules, token units, obligations, oracle freshness, ACTUS adapter | 100-payment benchmark and pilot servicing data |
| Atomic settlement | Custom Zswap/Compact logic | Conserved typed asset exchange and atomic-all semantics | Adversarial front-run/partial-input tests and measured settlement cost |
| Institutional private workflow | Private database or DAML-like platform | Public commitment plus selective disclosure and ZK enforcement | Legal/risk sign-off on generated audit package |
| Education | Compact tutorials | Clear finite semantics | Useful secondary outcome, not investment thesis |

The project stops after the 90-day phase if no two pilot teams provide written
use cases, representative contracts, integration owners, and measurable
acceptance criteria. GitHub stars, downloads, and workshop enthusiasm are not
substitutes.

## 16. Repository, governance, and roadmap

Specification authority lives in the versioned `spec/` tree and an accepted
Moriarty Improvement Proposal, not in whichever interpreter changed last. Each
normative rule has a test-vector ID and proof-status link. Semantic versions are
separate for surface, Core, encoding, backend, Runtime API, SDK, and registry.

Code ownership requires at least two reviewers for semantics/proofs, compiler,
backend, Runtime/client verifier, and registry. A release train is quarterly
while experimental and no faster than the audit/conformance gates. Validator/
proof artifacts are reproducibly built by two independent builders; threshold
signatures publish hashes. Security policy, disclosure contact, supported
versions, response times, and incident procedure are mandatory.

Draft proposals:

- MIP-MOR-001: version identifiers and compatibility matrix;
- MIP-MOR-002: Core syntax, state, transition semantics, and bounds;
- MIP-MOR-003: surface type/kind/effect/visibility system;
- MIP-MOR-004: canonical encoding, hashes, packages, and source maps;
- MIP-MOR-005: Compact/ZKIR backend and artifact/key versioning;
- MIP-MOR-006: planner, client verifier, API, rollback, and provider model;
- MIP-MOR-007: oracle and external capability boundary;
- MIP-MOR-008: Marlowe V1 import, equivalence, coexistence, and LTS;
- MIP-MOR-009: conformance, proof, audit, cost, and release gates.

### First 90 days — USD 550k–900k, 8–10 FTE

- Freeze Core strawman, version taxonomy, threat model, and proof obligations.
- Turn the escrow into an end-to-end proof/testnet slice with client verification.
- Prototype loan schedule and atomic swap.
- Measure AOT versus universal interpreter on three contracts.
- Secure two pilot letters and run formative interviews.
- Gate: no technical path, unacceptable ordinary proof latency, or no pilots
  reduces scope to a Compact financial library and analysis tool.

### Months 4–6 — cumulative USD 1.2m–2.0m

- Reference interpreter, canonical encoding, typed elaborator subset,
  translation validator, conformance generator, and first mechanized proofs.
- Stateless planner and malicious-transaction rejection harness.
- Escrow/swap/milestone testnet pilots and external design/security review.
- Gate: deterministic Core/Compact traces, fixed version tuple, ≥25% resource
  headroom, and pilot integration ownership.

### Months 7–12 — cumulative USD 2.5m–4.0m

- V2.0 vertical release: language server, simulator/analyzer, signed packages,
  audited circuits/backend, Runtime API/SDK, explorer provenance, migration dry
  run, docs, and two non-toy pilots.
- Gate: all section 13 release criteria. If proofs or audit lag, release remains
  S3/S4 testnet software and no value-bearing production claim is made.

### Months 13–24 — cumulative USD 4m–7m

- V2.1 experiments: threshold/partial action sets, typed oracles, authorization
  recovery, fixed-depth Merkle state, bounded runtime iteration, ACTUS adapter,
  restricted composition capabilities.
- Independent audit of each accepted extension and production pilot monitoring.
- Gate each feature independently; failed features remain outside Core without
  delaying V2.0 LTS.

### Later work

Universal interpreter circuits, other ledgers, state channels/L2, and general
external composition require demonstrated demand and a separate security model.
They are not presumed roadmap destinations.

## 17. Required experiments: reproduced and outstanding

| Assignment experiment | Status on 2026-09-02 | Next acceptance criterion |
|---|---|---|
| Representative V1/V2 suite | Partial: V1 structural suite; one Moriarty escrow compiled | Compile and execute all 13+ cases on both reference and backend |
| 10/100/1,000 recurring periods | Reproduced structurally for unrolling | Compare Core loop proposal, ZK rows, proof latency, state and transitions |
| High-participant close | Reproduced structurally at 10/100/1,000 | Implement payout queue and prove/measure loss-free progress |
| Atomic-all ordering/partial/timeout | Specified only | Model-check and run generated Compact adversarial cases |
| Typed-value errors | Reproduced 5/5 small prototype | Integrate into surface type checker and negative corpus |
| State compression | Relocation probe only | Measure state, proof, transaction, storage, availability, and audit effects |
| Continuation availability failure | Specified only | Remove all stores, detect before action, restore from participant export |
| Invalid initial state | V1 risk reconstructed; Moriarty constructor positivity compiled | Generate correlated invalid deployments and reject all layers |
| Role-token attack/recovery | Cardano analysis only | Define Midnight capability substitute and replay/rotation tests |
| Oracle staleness/replay/equivocation | Specified only | Signed typed feed harness rejects stale, reused, conflicting statements |
| Runtime compromise | Specified only | Client rejects every altered effect and version case |
| Differential semantics | V1 tests observed; not complete cross-stack | 100,000 Moriarty Core/reference/Compact traces without divergence |
| V1-to-Moriarty dry run | Specified only | Classify and report representative deployed contracts |
| User study | Protocol specified; not conducted | Cohorts and metrics in section 7 |
| Proof-effort prototypes | Escrow ZKIR model only | Mechanize atomic-all, conservation/effects, and visibility lowering |

## 18. Stakeholder discussion packet

The ten-page maximum pre-read is
[`moriarty-stakeholder-preread.md`](moriarty-stakeholder-preread.md). Two
90-minute sessions are recommended.

Session 1 decides purpose and semantics: finite thesis, verified Core/surface,
V2.0 boundary, action sets, iteration, state/closure, and normative proof
strategy. Session 2 decides realization and investment: Compact/ZKIR backend,
external capabilities, V1 migration, product user, pilots, funding, governance,
and go/no-go gates.

Decision motions:

1. Moriarty remains strictly finite and every deployable agreement has an
   explicit lifetime and resource certificate.
2. Moriarty adopts a small normative Core plus typed surface language.
3. V2.0 uses ahead-of-time generated Compact and translation validation, not a
   direct ZKIR backend.
4. Bounded loops elaborate away in V2.0; runtime iteration is a V2.1 proof and
   benchmark question.
5. V2.0 state is statically capped and closure uses deterministic bounded payout
   claims; generalized compression waits for proof/cost evidence.
6. V2.0 includes ordered-any and atomic-all only; partial/threshold collection
   is deferred.
7. Arbitrary external scripts and minting remain outside Core; later composition
   uses capability manifests with separate assurance.
8. One mathematical Core spec is normative; reference and backend
   implementations pass independent conformance and correspondence gates.
9. V1 contracts stay immutable; translation is classified and trace-checked,
   never automatic value migration.
10. The primary customer is the privacy-sensitive DApp/fintech developer; full
    investment requires two non-toy committed pilots.

Stakeholder-specific questions:

- Formal team: which prover has two maintainers, and which properties can be
  completed in 12 months without admitted gaps?
- Compact/ZKIR maintainers: what version-stability window and proof-parameter
  lifecycle can Moriarty rely on?
- Ledger/runtime engineers: which effects and credential bindings can a local
  client verify independently?
- Security auditors: is AOT specialization plus translation validation an
  auditable boundary, and what mutation evidence is required?
- Product/finance teams: which agreements need privacy and bounded analysis
  enough to justify a new DSL?
- Governance/funders: who owns V1 LTS, Core authority, emergency registry
  action, and the stop decision if pilots do not materialize?

The decision record must contain motion, date, participants, evidence for and
against, declared conflicts, decision, dissent, assumptions, status, owner,
deadline, validation metric, supersession rule, and linked artifact hashes.

## Final required answer

1. **Preserve:** finite continuation-based agreements; explicit action,
   non-cooperation, timeout, payment, account, warning/error, conservation,
   quiescence, and closure models; ahead-of-deployment lifetime and transition
   bounds; small shared semantics; hashed continuations; strict separation of
   semantics from ledger realization.

2. **Change in V2.0:** ship Moriarty Core, typed surface, explicit visibility,
   ordered-any and atomic-all waits, default-close surface timeouts, valid-state
   enforcement, bounded payout claims, canonical encoding, resource
   certificates, generated Compact/ZKIR backend, translation validation, local
   transaction verification, and versioned artifacts.

3. **Develop for V2.1:** partial/threshold action collection, dynamic join
   capability, bounded runtime iteration, proof-backed compressed state, typed
   attested oracles, authorization recovery/rotation, ACTUS adapter, and
   restricted composition after proofs and measurements.

4. **Move outside the core:** modules, packages, templates, business calendars,
   schedule generation, macros, static loops, UI/Blockly, oracle transport,
   continuation replication, transaction planning, indexing, wallets, identity
   services, token policy, external calls, and AI assistance.

5. **Reject:** general recursion, Turing completeness, runtime-unbounded
   containers, implicit disclosure, arbitrary Compact/ZKIR escape hatches,
   arbitrary mint/external calls that claim Moriarty assurance, silent Runtime
   trust, automatic V1 value migration, and portability without a funded use
   case.

6. **Prove and test:** termination, bounds, determinism, type preservation,
   conservation-to-ledger-effects correspondence, valid state, quiescence,
   timeout and action-set properties, authorization, visibility policy,
   serialization, elaboration, Core-to-Compact correspondence, differential
   traces, mutation/fuzz/on-chain tests, malicious planner rejection, proof/cost
   benchmarks, and independent audits.

7. **Migrate:** keep V1/Cardano and Moriarty/Midnight side by side; route by
   explicit manifests; classify translations; require human policy decisions
   where backend or timeout/role behavior changes; prove trace equivalence only
   where claimed; preserve V1 LTS and explorer/continuation support.

8. **Build:** in 90 days complete the end-to-end escrow, three-contract AOT
   versus interpreter study, Core draft/proof obligations, client verifier, and
   pilot commitments; by 12 months deliver only an audit- and gate-qualified
   V2.0 vertical slice; by 24 months add individually gated V2.1 capabilities.

9. **Govern:** make the accepted Core spec authoritative; keep one monorepo
   until conformance stabilizes; require two-owner review, public MIPs, separate
   semantic versions, reproducible two-builder artifacts, signed registries,
   scoped audits, LTS policy, and explicit incident/supersession records.

10. **Validate demand:** require private-term escrow, loan/milestone, and atomic
    settlement candidates; obtain two written pilot commitments; measure
    authoring/review time, errors, audit cost, proof/ledger economics, and
    operational recovery; use the seven primary family demonstrations and 13
    legacy regression patterns to test all 72 DeFiFormal rows at obligation
    level; stop or reduce Moriarty to a Compact
    library if the evidence does not justify a separate language.
