# Moriarty Webs: proposed collateral

Editorial status: r2 review candidate with terminal Opus errata applied. The copy below describes the proposed product concept. Calls to action name document sections, not deployed services or an existing website.

## Homepage copy

**Moriarty Webs · Proposed Cardano concept**

# Bring the required proofs to the transition.

**Proof-carrying financial workflows for Cardano, with explicit conditions before settlement.**

A transaction has more to say than “send this amount.” It has owners, limits, fees, conditions, and duties that continue after a partial payment.

Moriarty Webs proposes a way to bring the proofs a script requires into a financial transition. Give each proof a defined subject. Keep the intended effects in view. Make the handoff from program to proof to consuming script explicit. On the experimental route described by the reviewed V2 source, each proof is checked when its claim is issued; settlement consumes those claim outputs. [ERR-12/ERR-26, source reading only]

**Explore the invoice walkthrough** · **Read the evidence**

> **Current maturity.** Moriarty Webs is a concept under review. Supplied records include one current-Rust invoice-partial Groth16 roundtrip and seven finite local compiled-C2-wrapper cases, with a single-party test setup and caller-fabricated proof contexts. Only a conditional lowering argument is adopted. V2 consumer descriptions below are Opus readings of unadopted, uncompiled source, not observed behavior or exact-source correspondence to those runs. General Web integration, genuine intent authorization, onchain settlement, and all-family compatibility remain unestablished in this packet. Local VM cost is not ledger cost. [S07 `/scope`; S08 `/predicate`, `/scopeLimitations`; A01 `/scope`; A03 §0; ERR-05/ERR-17/ERR-19/ERR-20]

### Give every proof a defined job.

A **Web** is the proposed script-bound collection of **Strands** the consuming contract requires for one transition. Acceptance is proposed to require the whole set for that step. Unsupported classes are refused by the profile; broader certificate, history, private, recursive, and multiparent tracks remain required. [SEC06-P01 PROPOSED; ERR-01] A Strand is an individual proof with its claim, statement, and binding metadata. A **claim specification** defines the reusable requirement. **Weaving** describes their composition.

For a dapp, the useful question is: which proofs belong to this action, and what must the script do with them? For a transaction builder, it is: which statements, inputs, outputs, and consumer identities must agree? The Web concept makes that handoff explicit.

The transaction still invokes the validators and enforces the actual effects. Collecting proofs does not move funds by itself.

### Show the payment. Show the fee. Show what remains.

Start with a partial invoice payment. In the proposed walkthrough, paying 35 CASH with a separate 2 CASH fee debits escrow by 37. An invoice duty of 100 becomes 65. The fee is not debt repayment.

The walkthrough distinguishes source behavior from a proposed backing policy: source funding 110 leaves cash 73 against duties 75 after that payment. Funding 112 would fully back those duties for one partial, but the source does not require it. Each later partial charges another fee. It also distinguishes proof evidence from settlement. [ERR-22] Its purpose is simple: let a developer explain the financial action before hiding it behind a success badge. [S12 `action partial`; illustrative scenario in CONCEPT.md]

### Build around the conditions your application needs.

**Dapp teams:** organize execution requirements around exact assets, recipients, fees, and continuing duties.

**Asset issuers:** make issuance and servicing conditions explicit, with supply accounting separate from redemption promises.

**Wallet and infrastructure teams:** connect an inspectable statement to the required proofs, owner authorization, transaction construction, and effect readback.

These are proposed integration roles. The learning path starts small and exposes the first unsupported boundary.

### Start with one useful action.

The proposed learning kit begins with partial invoice payment, then adds a contrasting conditional-reserve allocation. Its source action needs non-negative custody of 23 A and 4 B; full backing at 25 A and 10 B is a separate proposed policy. The source has no action discharging allocation_reserve. A nonzero reserve duty blocks Close under the reviewed V2 source. [ERR-21] From there, explore batch orders, capped issuance, loan servicing, treasury allocation, and staged escrow.

Use the examples to ask precise questions. What does this proof establish? Which input authenticates the state? Who pays the fee? What is still owed? Which transaction establishes settlement?

**Read the developer journey** · **Explore five application cases**

### Keep the developer path open.

Moriarty's product contract permits developers to author, compile, prove, publish, and deploy supported programs without Moriarty maintainer, council, campaign, or registry approval. Asset-owner authorization and objective target rules still apply. Optional services remain optional. The broader Moriarty language targets Midnight, alongside this Cardano experiment. [S03 introduction, “Product boundary,” “Verification versus project process”]

**Closing invitation:** Bring a workflow with exact assets, participants, fees, conditions, and remaining duties. Use the Web concept to make its proof-to-transaction handoff concrete.

## One-page brief

**Moriarty Webs**

**Bring the required proofs to the transition.**

Proposed proof-carrying financial workflows for Cardano, grounded in explicit conditions before settlement.

**For:** dapp and protocol developers, asset issuers, wallets, and transaction-building infrastructure.

**The job:** connect a program's financial requirements to concrete proof instances and the script that consumes them. Keep exact assets, authority, fees, net effects, and remaining duties visible through that handoff.

**The model:** a Web is the proposed script-bound collection of required Strands. Each Strand is an individual proof with its claim, statement, and binding metadata. A claim specification is the reusable definition. Weaving means composition under the consumer's rule. Validators invoked by a transaction enforce the actual effects.

**The first example:** a partial invoice payment. The illustrative lesson separates seller payment 35, source fee 2, gross escrow debit 37, and remaining invoice duty 65 from an opening duty 100. Funding and reserve assumptions remain explicit. The next lesson uses reserve allocation to contrast arithmetic, approval provenance, and continuing duties.

**Five exploration cases:** batched DEX orders, capped issuance, loan servicing, treasury allocation, and staged invoice escrow. Each adds requirements beyond the first example. The examples are not compatibility results.

**The delivery path:** documentation and independent expected-effect worksheets first, then a small reference app, actual required-Strand integration, and separate target qualification. The proposed workflow keeps local checks, issuance-time proof checks, authorization, submission, and settlement observation distinct. Settlement consumes claims and requires each claimant’s signature. [ERR-12/ERR-25]

**Evidence boundary:** this is a concept review candidate with narrow local proof/wrapper evidence and a conditional source-design result. The homepage maturity box states the full limits. No general Web/onchain support or performance advantage is claimed.

**Product context:** Moriarty remains a permissionless language for Midnight. Cardano is an additional experiment. A service account or internal review record is not a public developer prerequisite. [S03 “Product boundary,” “Verification versus project process”]

**Next step:** inspect the invoice source, its independent worksheet, and the first unresolved integration boundary.

## Thirty-second pitch

Moriarty Webs proposes proof-carrying financial workflows for Cardano. A Web collects the Strands a script requires for a particular action. Each Strand is an individual proof with the statement and binding metadata that explain its role. The aim is a clear handoff from program to proof to transaction, with amounts, fees, authority, and remaining duties in view. Start with a partial invoice payment, then explore how the same concepts apply to orders, assets, and servicing.

Timing is a spoken-delivery target, not an observed recording. Retain the concept label when excerpting.

## Five use-case cards

All cards describe proposed applications. Full scenarios and dependencies appear in CONCEPT.md.

**Batched DEX orders — give every order its own outcome.** Connect exact assets, minimum net receipts, and pool arithmetic to one specified batch. The consuming validator enforces actual output allocation under its profile’s strict-output or amount-partition policy. Keep partial fills and cancellation distinct. [ERR-08/ERR-23]

**Capped issuance — show where every issued unit goes.** Connect authorized supply change, recipient distribution, and supply continuation. Keep unissued capacity separate from servicing and redemption duties.

**Loan servicing — separate repayment from its fee.** Show principal, accrual, payment, fee, and remaining debt. Make period and rounding rules explicit before explaining the proof.

**Treasury allocation — follow the decision through to the reserve.** Connect the actual authorization, beneficiary outputs, fee budget, and remaining reserve or rebate duties. A supplied approval flag alone is not a distinct governance decision. The reviewed V2 source signs action inputs through required owner intents, which can express the debited account’s consent without authenticating a separate governance approver. [ERR-09/ERR-24]

**Staged invoice escrow — pay part and keep the remainder visible.** Explain funded opening, partial payment, final payment, reserve return, and recovery as distinct steps. Submission and funded custody do not themselves establish delivery. S12 has no reserve-duty-return action, and its nonzero reserve duty blocks Close under the reviewed V2 source. [ERR-22]

## FAQ

**What is a Strand?** An individual proof with its claim, statement, and binding metadata. Its reusable definition is a claim specification. An unproven claim, pattern, UTxO, or validator is not a Strand.

**What is a Web?** The proposed script-bound collection of required Strands for a specified action or composition. Its consumer must have explicit rules. No universal proof count is defined.

**Does weaving mean one aggregated or recursive proof?** No. It is the composition term. Aggregation, recursion, private handoff, and multiparent composition require their own designs and evidence.

**Can a partial payment have all its required proofs?** That is the intended distinction. A permitted partial financial step can leave a duty unpaid while requiring a complete proof set for that step. Enforcement in the proposed Web consumer remains a security-validation question. Missing required proof evidence is not accepted partial progress. Already-issued claim outputs may nevertheless exist on-chain and lock value, even while the Web is incomplete. [SEC06-P01/SEC06-P05 PROPOSED; ERR-15; decision record]

**Does a Web execute automatically?** The transaction must invoke the applicable validators and enforce its payments and continuations. Proof collection or claim-output creation does not execute those effects.

**Can the same proof be used in different apps?** A reusable claim specification is different from a reusable proof instance. Reusing a proof would require a compatible exact statement, bindings, consumer rules, and justified acceptance path. Portability remains a hypothesis.

**Do proofs make external observations true?** External facts retain their named authentication source, issuer, predicate, and freshness assumptions. Proofs over supplied values do not turn those values into authenticated ledger facts. [S03 “What provable intention means”]

**Where are the proofs checked?** In the reviewed V2 issuance design, C2 checks each proof when issuing its claim output. Settlement recomputes statements and consumes claims; it does not verify the proof again. Each consumed claim requires its claimant’s signature and token burn. Unused claims remain locked until the applicable release, including post-expiry reclaim. These are source-level descriptions, not ledger observations. [ERR-04/ERR-06/ERR-12/ERR-25]

**What exists in this packet?** The current-maturity box identifies the narrow recorded proof, compiled-wrapper, and conditional source-design results. The proposed Web developer journey is not a shipping API.

**Does the first app establish compatibility?** No. It teaches one action. Family mapping, financial conformance, and target qualification remain separate requirements.

**Is this Cardano-only?** The concept explains the Cardano experiment. Moriarty's broader contract still targets Midnight ZKIRv3 and its native proof infrastructure. [S03 “Execution target and certified basis”]

**Do I need Moriarty approval?** Supported developer tooling is permissionless. Owner authorization and objective target validity remain necessary. Internal reviewer and campaign records are project process, not developer permissions. [S03 “Verification versus project process”]

**What does “reserve” mean here?** A reserve duty is a program liability. The V2 thread reserve is lovelace returned on Close. Program-asset fees, network fees, collateral, and claim-output min-ADA are separate funding items. [ERR-25/ERR-28]

**Is it cheaper or faster?** No advantage is claimed. Future measurements must compare equivalent complete behavior, including proving, payloads, issuance and consumption where used, qualified ledger resources, fees, and latency.
