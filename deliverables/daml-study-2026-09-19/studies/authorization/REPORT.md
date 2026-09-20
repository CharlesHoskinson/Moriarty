# Daml authorization, obligation formation and scoped delegation

Status: bounded independent source study, 2026-09-19. Author configured as GPT-6 Astra, medium effort; no independent runtime identity receipt is available. This study reads 13 complete captured documentation pages and targeted excerpts from two further pages. It compares tutorial code with the full textual detailed ledger model. [Claims](claims.json), [exact references](references.md) and [reading coverage](reading-coverage.json) distinguish documented rules, derived negative witnesses and Moriarty design inferences. No Daml example was executed, no compiler/runtime implementation was audited, and no legal enforceability is claimed.

Daml's strongest lesson is to make consent a property of each action in its actual authorization context. A party's appearance in a record, a workflow label, a document hash, a signature log or a projected subtransaction does not by itself establish that party's consent to the resulting economic obligation. This reinforces Moriarty's proof-carrying formal intention requirement; it does not supply Moriarty's proof system or ZKIRv3 correspondence.

## Explanation: what the authorization semantics establishes

The detailed ledger model gives separate conditions for consistency, conformance and authorization. Consistency tracks active contracts and their consuming uses. Conformance requires the entire action tree, including consequences, to match template code. Authorization checks whether the parties whose authority is needed are available in the context. None substitutes for the others. The specification's example replaces an unauthorized asset owner with another actor: authorization then succeeds but conformance fails because the actor no longer matches the choice controller. DA01/DA02/DA14/DA15.

Let `req(a)` be an action's required parties and `ctx(a)` its available authorization context. The documented rule is:

```text
req(Create c)   = signatories(c)
req(Exercise a) = actors(a)
req(Fetch a)    = actors(a)
ctx(root)      = requesters(commit)
ctx(child)     = actors(parent exercise) ∪ signatories(parent input)
accept authorization only if req(a) ⊆ ctx(a), for every relevant action
```

The child context is reconstructed at the immediate parent boundary. It is not a growing union of every ancestor's privileges. The tutorial's `TryB -> TryA` example is decisive: Alice and Bob authorize the first consequence, but only Alice is actor/signatory at the next boundary. Creation requiring Bob then fails. A helper cannot retain Bob merely because he was present earlier. DA01/DA03.

This is a useful form of scoped delegation, not a ready-made generic capability calculus. A contract's signatories authorize consequences that its conformant choices permit. Controllers may use that standing authorization without asking every signatory to sign again at each use. All controllers named on a choice must authorize it; a numeric threshold in the body does not alter this controller rule. A reusable role is represented by an active contract and a nonconsuming choice. A revoked or consumed role cannot authorize a fresh exercise under consistency, but revocation does not automatically invalidate authority already transformed into a separately authorized successor. Moriarty must make that lifecycle policy explicit. DA01/DA05/DA06.

Daml signatories are semantic parties, not necessarily fresh external cryptographic signatures attached to every descendant action. Internal parties delegate submission signing to hosting validators. External parties retain authorization keys but still use validators for state validation and confirmation. Ledger API credentials are a further service boundary: a token permits a client to ask a participant to serve a request; it does not replace the Daml authorization judgment. DA16/DA18.

## Explanation: obligations require consent, but ownership is not a universal liability type

The tutorial's issuer-only `SimpleIou` permits the issuer to archive the IOU after receiving goods. Adding the owner as a signatory protects against that unilateral action, and also prevents unilateral creation of an IOU purporting to carry that owner's signature. Its negative-amount example explains why this matters: receipt of a contract can represent an unwanted negative position. DA04.

Propose-accept solves the authorization problem by storing issuer authority in a proposal and adding the recipient's authority in the acceptance choice. The resulting agreement has both as signatories. A standing role can instead authorize a bounded class in advance. The shown `IouSender` checks positivity and existing ownership but has no issuer whitelist, currency predicate, expiry, per-period amount or aggregate liability bound. It is a small illustration, not complete financial policy. DA05/DA06/DA29.

MPLR-019 should remain about introduction or increase of enforceable protocol obligations and their material terms. Do not require interactive recipient acceptance for every positive-value receipt. Do not equate a positive numeric field with harmlessness: asset issuer, redemption conditions, fees and downstream duties still matter. Conversely, Daml's `signatory` declaration does not prove that arbitrary off-ledger law creates or discharges a legal obligation. Separate protocol-enforced liabilities from external legal assumptions.

Moriarty also needs protection against erasing obligations. Merely forbidding unauthorized new debt is insufficient if a holder of archival authority can remove a debt record without an authorized discharge. The source's issuer-only IOU is a concrete negative witness for MPLR-017 and the persistent-duty requirements, as well as MPLR-019. A universal “signatories can destroy every resource” primitive should therefore not be imported unchanged.

## Reference: evidence boundaries and contradictions

| Topic | Bounded conclusion | What must not be inferred |
|---|---|---|
| Ledger validity | Consistency, conformance and authorization are separate checks; Canton validity is for honest parties under topology assumptions | Unconditional global correctness, or a general cryptographic theorem about all economic intentions |
| Native signature | External signing commits to an interpreted transaction and its metadata | The transaction refines a separately specified intention, or every attested external fact is true |
| Projection | Internal authorization can survive projection; root authorization context can disappear | A visible action's local proof establishes the original requester's consent |
| Disclosure | A party may receive/use contract data without gaining its owner's transfer authority | Possessing a contract ID or private witness is a spending capability |
| Upgrade | Compatible target templates can change choice bodies/controllers subject to documented rules | Structural compatibility is economic equivalence or renewed consent |
| API token | Participant-specific client rights are checked separately from ledger validity | Token/service onboarding is a Moriarty language-wide program deployment gate |

The detailed ledger model explicitly shows an issuer projection that does not reveal a missing asset-owner authorization at the parent. This makes private proof composition more demanding: carry or prove the original authorization context and predecessor linkage, rather than treating every internally valid subtree as independently authorized. The formal model also states that conformance constrains information flow within an action, while submitters choose cross-action flow. Business history must be encoded and checked; it does not follow from valid record types. DA12/DA15.

Several tutorial statements need qualification:

- **Visibility:** composition documentation says only stakeholders see contracts and observers cannot act. The formal model includes ancestor witnesses and divulgence to nonstakeholders. An observer may separately be a controller, as the choices tutorial demonstrates. Observation alone grants no authority, but it does not prohibit independently granted authority. DA10/DA11.
- **Atomicity:** the multi-party tutorial contrasts built-in atomicity with partial workflow states. The formal model scopes atomicity to one transaction. Propose, accept and later settle can be separate commits; externally delivered goods or another chain do not become atomic because an on-ledger state transition is atomic. DA15/DA24.
- **JWT revocation:** the API guide describes JWTs as nonrevocable but also dynamically checks a user's current rights. Token-byte revocation and denial under changed account rights are different mechanisms. Do not turn either sentence into a universal delegation-revocation claim. DA17.
- **Version selection:** production guidance loosely describes the newest uploaded body, whereas package-selection and detailed upgrading rules distinguish static package references and dynamic selection. Pin the actual target interpretation used, including any authorized upgrade policy. DA25/DA27.
- **Hashing:** the current page's enum omits V4 although its table and algorithm include V4; metadata pseudocode and Python use different field names; V2 pseudocode includes `max_record_time` while V2 example code omits it. Further differences in optional-key pseudocode and example code reinforce the need for implementation test vectors. These are unresolved documentation inconsistencies, not a finding that production signatures are exploitable. DA21/DA22.

The external signing API excerpt is explicit that an untrusted preparing participant requires content display and independent hash recomputation. The signature is over a concrete result, and the subsequent validator checks remain necessary. Moriarty may authorize a class of alternative valid plans through its formal intention, so copying “sign the entire one prepared transaction” would unnecessarily collapse that expressivity unless it is an intentional mode. In either mode, the displayed constraints and signed canonical object must agree. DA19/DA20.

## Reference: adversarial witnesses

These are deductions from the inspected examples and formal rules, not observed Daml runtime test results.

| Witness | Rule-derived outcome | Moriarty implication |
|---|---|---|
| `TryB` carries Bob into an inner Alice-only `TryA`, which creates a Bob-signed object | Reject: Bob is absent from the immediate child context | No ambient accumulation of authority through helpers |
| Initiator directly creates tutorial `Workflow(state=Approved)` | The shown template does not require the approver's authority for that create | A state label must not stand in for authenticated transition history |
| Unrelated `newOwner` receives the delegated tutorial `Asset.Transfer` | Replacement create lacks new owner's signatory authority in a standalone exercise | Check all introduced obligations and required parties at nested creates |
| Initialize `Vote(votes=[])` | Empty signatory set violates the model | A threshold workflow needs a valid bootstrapping state |
| Two of three vote yes, but only those two authorize `TallyAndExecute` | `controller voters` still requires the third voter | Numerical evidence threshold and authorization threshold are distinct |
| Invoke tutorial `TradeSettlement.Settle` | `pure ()` encodes no delivery; default consuming choice archives the record | Delivery requires explicit complete effects or bound external evidence |
| Sender uses `IouSender` with a positive IOU from an unintended issuer | The shown role has no issuer restriction to reject it | Consent must cover material asset and obligation terms, not only sign |
| Issuer alone archives an issuer-only IOU after receiving goods | The tutorial permits this archival | Preservation/discharge of liabilities needs an invariant beyond creator authority |
| Disclose Carol's asset to Alice/Bob and attempt transfer | Disclosure does not provide Carol's controller authority | Knowledge and authority must be separate types/judgments |

DA03–DA09/DA13/DA24 identify exact source locations. The direct-creation witness is particularly relevant to proof-carrying data: a recursive provenance proof should reject a fabricated approved state even when its record has the right shape. It also needs a valid genesis rule; blindly accepting a signed initial `Approved` state would reproduce the same hole.

## How-to: check a consent claim without assuming the workflow

1. Identify the exact template, package/semantic version, source of each party identity, and whether the operation is a create, exercise or read. Do not infer controllers from a role name.
2. Derive required authorizers and immediate context for each action, including nested creates. Separately check code conformance, field predicates, contract activeness and all effects.
3. Try direct creation and direct choice exercise outside the advertised happy path. The formal model treats these as public operations when their objective requirements are met.
4. Distinguish one committed transaction from a persistent business workflow. Check cancellation, role archival and acceptance races against actual active state; preserve surviving obligations.
5. For a private proof or projection, identify which context was hidden and how its authority/history claim is established. Disclosure, signing and proof verification have separate roles.
6. For signing or upgrade claims, pin canonical encoding and resolved semantics. Cross-check current documentation against a source implementation and published vectors before treating it as a byte-exact adapter specification.

A future execution study should run the witnesses in a pinned Daml SDK, inspect resulting transaction trees, and compare them with a pinned Canton authorization implementation. That work was not performed here. The current artifacts provide the exact sources and candidate traces to make that study reproducible.

## Tutorial: destination submission with mixed evidence

Take a buyer, seller, destination recipient, custodian and document attestor. The buyer signs the asset, amount, recipient, liability and fee bounds, permitted partial outcomes, evidence policy and recovery rules. The request is addressed to the destination but delivery remains withheld until the bound combination of signatures, document predicates, proofs, recipient actions and other supported conditions is satisfied. This is conditional settlement with composable evidence requirements; a funded version is programmable escrow.

A Daml-inspired proposal can store the buyer's standing consent and expose acceptance to the specified recipient. A Moriarty continuation must additionally carry authenticated state lineage, remaining authority and outstanding duties. A private document witness can establish a defined predicate under named issuer assumptions, but cannot create authority to debit the custodian or establish the document's unmodeled truth. A document hash alone only binds bytes.

If an attestor adds evidence, check that the evidence introduction does not give it the buyer's spending authority. If a helper validates that evidence, reconstruct the helper's scoped context rather than inherit all outer parties. If an upgraded condition evaluator is selected, verify that the original signed upgrade policy permits the new meaning; byte/schema compatibility alone is insufficient.

When release is requested, prove the original consent lineage, live relevant authority/revocation policy, condition satisfaction, all accounting and complete destination effects. Funding recorded earlier is not delivery. If one remote effect is unresolved, keep its duty and apply the signed recovery policy; a successful local proof cannot resolve an unknown external outcome. A later recursive proof must validate its predecessors, not merely attach their signed logs.

This sketch is a requirements example, not implemented Moriarty syntax or a claim that Daml already supplies those recursive proofs. Public developers must be able to implement any supported composition on Midnight ZKIRv3 without project admission. Their applications may restrict counterparties and evidence issuers under owner-approved policy.

## Requirement disposition and limits

[MPLR-candidates.json](MPLR-candidates.json) proposes refinements to existing requirements, avoiding duplicate stable IDs. The highest-value additions are nested authority contexts, valid genesis/history evidence, bounded obligation formation and discharge, threshold distinctions, consent-preserving semantic evolution, and private authorization provenance. [SECURITY-PCD.md](SECURITY-PCD.md) derives restrictions and proof obligations from concrete attacks.

This study does not establish Daml compiler correctness, Canton implementation correspondence, deployed behavior, legal truth, cross-chain finality, unconditional liveness, or Moriarty proof feasibility. The formal ledger document itself describes part of interoperability as an evolving vision. It is evidence for a semantic boundary, not a proof that every advertised cross-ledger mechanism is deployed. No new public acquisition or vault mutation was performed.
