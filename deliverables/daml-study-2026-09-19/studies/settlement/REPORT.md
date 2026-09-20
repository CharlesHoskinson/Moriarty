# Daml settlement: conditional delivery, allocations and recovery

Completed source study, 2026-09-19. This track compares current captured Canton/Splice documentation with `canton-network/splice@18f490ae5a0275bc76088821f54e4a0448a56bc2` and `digital-asset/daml-finance@155f931b6ebe7d3662fd72788cb17f0bfb5a7ba6`. The Finance pin's last commit is dated 2025-04-07; it is not evidence that Finance V4 and Splice v2 are one current deployed system.

Eighteen source claims have exact URLs, file hashes and line ranges in [claims.json](claims.json). [reading-coverage.json](reading-coverage.json) distinguishes the initial 190-page collected corpus from 22 settlement-specific files actually read/source-checked. Only the listed ranges are claimed as read. No examples were executed and no theorem or production deployment was verified. Seven proposals, without allocated MPLR IDs, appear in [MPLR-candidates.json](MPLR-candidates.json). [SECURITY-PCD.md](SECURITY-PCD.md) translates the evidence into an explicitly proposed security/provability direction.

## Explanation: what Daml contributes

Daml offers a stronger comparison for the user's conditional-settlement requirement than the NEAR verifier alone. A workflow can record a proposal, gather distinct parties' authorizations in successive contracts, reserve holdings, and finally execute a transaction whose body performs the specified transfers. The pending business workflow and the final atomic ledger transaction are different things. Splice allocation instructions even expose a pending result when further registry steps are required. They do not label initial acceptance as economic delivery. [DS01–DS05, DS10–DS11]

The user's requirement remains: submit a transaction addressed to a destination, but withhold specified settlement until the signed combination of additional signatures, documents, proofs, recipient acceptance or other supported conditions holds. Daml's allocation and approval protocols exemplify parts of that behavior. They do not establish a general document-truth oracle, a universal proof-carrying program language, or the particular Moriarty proof relation. Document predicates and attestations would need explicit application semantics and trust assumptions.

The detailed ledger model separates consistency, conformance and authorization. Consistency prevents using an already consumed contract. Conformance requires the actual action tree to match the template's behavior; a DvP with one required transfer consequence missing fails conformance. Authorization concerns signatories/controllers. Thus a bag of valid signatures and a conserved balance are not sufficient substitutes for checking the whole intended action. These are model rules enforced by the Daml/Canton stack; this study establishes no zero-knowledge execution proof or compiler-to-ledger theorem. [DS01]

## Reference: Splice v1 and v2 are materially different

| Topic | Allocation v1 | Allocation v2 |
|---|---|---|
| Meaning | Funded allocation for one specified transfer leg | Account authorization for send/receive sides, potentially multiple legs, with net funding |
| Execution authority | Executor, sender and receiver jointly control execution/cancellation | Caller supplies actors; implementation must validate them; default settle uses admin and executors |
| Funding and timing | Settlement has allocation and settlement times; holdings may be off-ledger | Optional settlement deadline, separate commitment flag, explicit successor funding |
| Withdrawal | Sender-controlled interface choice | Account-party policy; committed allocation withdrawal is restricted |
| Repetition | One allocation's specified leg | Iterated settlement is explicitly enabled; `None` differs from an empty funding map |
| Consumption | Consuming interface choices | Nonconsuming interface choices whose implementation bodies MUST consume the old allocation |
| Outcome | Transfer/cancel/withdraw result structures | Pending, settled with optional successor, cancelled, withdrawn |

Sources: DS02–DS09. “MUST” and “SHOULD” in interface comments are interface obligations/recommendations, not proof that every implementing template satisfies them.

In v1 an allocation factory can return an allocation immediately or an allocation instruction that evolves through multiple steps. A failed instruction result means, by the documented interface contract, that reserved funding was released. A pending result does not mean failure, and a request's creation does not guarantee funding. Original instruction identity records the lineage. [DS03]

V2 `AllocationRequest_Accept` has an even narrower meaning: it signals that allocations were or will be created. Wallets may combine acceptance with creation in one transaction to avoid duplicates. Applications must clean requests independently because acceptance need not be called. This is a concrete warning against deriving completed funding or delivery from a friendly status name. [DS10]

V2 separates an account authorizing receipt from an account funding outbound value. An allocation can authorize several transfer sides and reserve only the net required amounts. `nextIterationFunding = None` disables iteration; `Some(empty map)` enables iteration without reserving additional funds. The latter is useful when later iterations expect incoming value. In the sample, a successor allocation clears the completed transfer sides, carries remaining funding and original lineage, and increments an iteration counter. This is not, by itself, an economic partial-fill policy with price floors, fee caps or cumulative quantity limits. Those remain application obligations. [DS04, DS09]

The default batch helper rejects duplicate transfer IDs, nonpositive amounts, mismatched settlement/admin references, repeated authorizations, missing authorizations and superfluous authorizations. The matching relation includes both sides of every transfer. Its negative-test source includes duplicate authorization through both identical and distinct contract IDs. These tests were inspected, not run. [DS07]

There is an important interface boundary. The helper exercises each allocation and returns its result. The result type allows `Pending`; the helper does not itself assert that every result is `Settled`. The inspected sample's normal admin/executor settlement transition completes, but a structural interface alone cannot certify arbitrary token implementations. A custom implementation could preserve pending state, omit a promised consumption, or implement a different economic effect. A caller's proof must depend on the right semantic contract, not merely a matching method name. This is an architectural counterexample candidate, not an executed vulnerability in the deployed token standard. [DS05, DS08]

## Explanation: committed funding, deadlines and cancellation

A committed v2 allocation protects executors from unilateral withdrawal before its settlement deadline. The default helper requires ledger time strictly greater than that deadline for committed withdrawal. If no deadline exists, that helper denies authorizer withdrawal. Executor cancellation, settlement or an implementation's admin expiry path may remain possible, but autonomous recovery by the authorizer is not guaranteed. An application must disclose this commitment and its cooperation assumptions. [DS06]

Settlement and withdrawal compete over an allocation's consumable identity. In a conforming implementation, both cannot successfully consume the same current allocation. If a transition returns a successor pending contract, later operations must address that successor and respect the carried state. This is a ledger consistency and implementation-consumption argument, not a claim that every cross-chain race is resolved. [DS01, DS05]

Daml Finance V4 has a different timing policy. Its optional `settlementTime` is explicitly a preferred time, not an enforced cutoff. The default batch and instruction implementations intentionally allow flexible early/late timing; applications can delay allocation/approval or implement stronger guards. Treating that field as equivalent to Splice's enforced deadline would be a requirements bug. [DS12]

Finance cancellation is also not an unconditional refund button. It needs instructor/consenter authorization and the release authority of recorded signed senders and receivers. Pledges and approvals are released under those rules. Earlier separately committed business steps are not retroactively erased by a failed or canceled final settlement attempt. [DS11, DS14]

A concrete late-arrival limitation appears in Splice's example OTC trading application. Its consuming cancel choice handles the allocations supplied at that moment. The source explicitly warns that the venue cannot cancel allocations created afterward and suggests retaining a marker until the settlement deadline. This is the strongest new recovery lesson: a canceled parent request may still have lawful in-flight or future children. Cancellation must retain the evidence, authority and state needed to reject, return or resolve them. A terminal label alone does not discharge those duties. [DS17]

## Explanation: Finance holdings and off-ledger acknowledgements

Finance separates sender allocation from receiver approval. A pledged holding must match the exact instrument and quantity; its lock is acquired. The receiving side specifies and authorizes its destination account. Once all instructions are allocated and approved, the batch orders pass-through chains and executes all instructions within one Daml transaction. The transaction aborts if a required instruction fails. It does not undo separately committed allocation/approval transactions. [DS11]

A holding may represent a claim at a custodian. Intermediated settlement can debit a customer's holding at Bank A, transfer a claim between banks at a central bank, and credit a new holding at Bank B. The code distinguishes transfers from custodian credit/debit, and pass-through avoids requiring an intermediary's pre-existing holding for an eligible chain. Therefore per-token conservation alone is insufficient to describe economic correctness: issuer/custodian liabilities, authorized credit/debit and redemption assumptions require their own typed relations. [DS18]

Finance's `SettleOffledger` plus `SettleOffledgerAcknowledge` is especially instructive. Authorized parties agree to that mode, and the execution branch returns `None` rather than carrying out an on-ledger holding transfer. Mixed on/off-ledger modes within that instruction reject. The acknowledgement can be valid Daml execution without proving that a bank transfer or another blockchain transfer actually occurred. Any such guarantee depends on the evidence and trust model around the acknowledgement. [DS13]

## Reference: atomicity boundary matrix

| Boundary | What source supports | What it does not establish |
|---|---|---|
| One Daml transaction | Atomic action execution under ledger validity and runtime assumptions | Atomicity of prior proposal/funding transactions or independent external effects |
| Finance batch | All configured instructions are executed together; missing approval/allocation rejects | External solvency or delivery merely because an off-ledger acknowledgement was authorized |
| Splice default per-admin batch | Exact transfer-side matching and calls over allocations | Arbitrary implementation postconditions solely from interface membership |
| Example cross-admin OTC trade | One choice iterates all required admin groups and calls their settlement paths | Arbitrary independent-chain atomic commit |
| Multiple Canton synchronizers | Reassign inputs to a suitable common synchronizer, then execute there | A single instantaneous transaction spanning independently active input domains |
| Independent chain or bank | Application-specific evidence/bridge/custodian protocol | Delivery, nonexecution, finality or refunds from Canton transaction success alone |
| Moriarty/Midnight | Required future language and proof behavior | Any imported NEAR/Daml phase behavior without target correspondence |

Current multi-synchronizer documentation contains a discrepancy worth preserving. The introduction describes the move as atomic for stakeholders, while its detailed protocol expressly describes non-atomic unassignment and assignment. After unassignment, the contract is pending assignment and cannot be used until assignment completes. The transaction router waits for reassignment before submitting the Daml transaction to one selected synchronizer. Package vetting, party hosting and appropriate reassigning participants can block this preparation. The precise detailed model controls this study's conclusion. [DS15]

A pending-assignment contract is not the same as a partially executed asset swap. Likewise, a valid atomic swap after reassignment does not prove that reassignment always finishes or that every user sees a causally ordered combined update stream. Public Moriarty language validity must not inherit Canton participant/package administration as a project approval requirement.

## Tutorial: an evidence-conditioned DvP with adversarial variants

This is a conceptual trace illustrating required distinctions, not executed Moriarty or a claim of turnkey support for every condition in Daml.

1. Buyer and seller authorize exact asset identities, quantities, destination accounts, allowed fees, document/proof predicates and timing policy. A request is recorded. Neither recording nor acceptance signals alone establish reserved funds.
2. Each party supplies the required allocation/approval. A pending instruction may gather account-provider authorization before a final allocation exists. The application tracks identity and current successor contract, not only the initial request.
3. A document attestor supplies a statement bound to the document bytes, schema/predicate, request and validity cut. A recipient supplies acceptance over the same terms. A proof can establish the defined predicate; an issuer's statement remains a named external trust assumption where applicable.
4. Once the exact combination holds, settlement runs inside its declared atomic region. Each economic leg must appear in the effect frame. Net funding may optimize movements only while preserving gross authorization, fees and liabilities.
5. A valid partial iteration carries remaining quantities, funding, aggregate fees and duties. `Settled` for that iteration is not necessarily global completion.
6. If cancellation wins, remaining authority/state still handles permitted late allocations or results. If an external leg has known success, recovery accounts for it. Missing external evidence does not establish nonexecution; a local time guard cannot authorize double refund.

Decisive negative variants: substitute a recipient signature from another request; omit one DvP leg; duplicate a receiver-side authorization under a new contract ID; enable iteration without original consent; withdraw a committed allocation prematurely; cancel the only parent authority before late funding arrives; return `Pending` while labeling a batch delivered; treat preferred settlement time as an enforced deadline; record an off-ledger acknowledgement without delivery evidence; report reassignment preparation as finished settlement. Each exercises a different acceptance predicate. [DS01–DS18]

## How-to: reproduce and extend this study

Check the repository pins with `git rev-parse HEAD`. For each claim, recompute the recorded file's SHA-256 and inspect its inclusive line range. Compare v1 and v2 at the pinned source, then inspect implementation bodies and helpers before interpreting interface comments as guarantees. Read negative-test source as expected behavior only; executing the relevant Daml test suite is a separate step with its own evidence.

Use [MPLR-candidates.json](MPLR-candidates.json) to refine existing MPLR-001..019 or consider three distinct additions: explicit settlement-domain scope, gross-to-net semantic refinement, and semantic contracts for interchangeable settlement implementations. No new MPLR number is allocated here. The other proposals refine commitments, cancellation residual state, timing claim kinds and recursive security induction.

For Moriarty, retain a permissionless language for all Midnight DeFi developers, actual ZKIRv3 execution and proofs bound to signed formal intention. The inspected Daml mechanisms are evidence for needed behavior, not a mandate to adopt their syntax, enterprise deployment model or a closed token-template registry. General program semantics, compiler correctness, arbitrary-witness soundness and actual ledger correspondence remain open proof obligations.

## Reference: acquisition and evidence limits

Additional source was obtained through pinned public Git checkouts. Scrapling 0.4.15 probes found `docs.daml.com` disallow-all robots instructions, so its initially retrieved Finance index is excluded from claim evidence and no further pages were collected there. The Finance README's GitHub Pages settlement URL returned 404. The study consequently uses the Finance repository's pinned documentation source and executable code. The extra-capture manifest preserves these outcomes; no cookies or credentials were saved.

No live package versions, validator permissions, production settlement, Daml-to-LF correctness, Canton consensus implementation, issuer reserve solvency or private-information-flow proof was audited. Private observer groups and actor fields were inspected only where they affect the settlement boundary. The parent may collect the remaining current sitemap; collection does not expand this track's reading or proof status.
