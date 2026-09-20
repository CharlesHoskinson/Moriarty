# NEAR teardown: common design conclusions, revision 1

This is a research/design candidate, not an implementation proof, release approval or permission to deploy Moriarty programs. Six requested experts are three Claude Fable5.1 medium and three GPT6Astra medium. Earlier inherited-configuration workers supplied preparation only; their outputs do not substitute for the configured final studies.

## Controlling user intent and terminology

Moriarty is a permissionless programming language for all Midnight DeFi developers. Its compiled contracts run on Midnight through a pinned ZKIRv3 route and carry actual evidence for authenticated formal intention. Project review, registry membership and hosted service credentials are not developer deployment requirements. Deployed contracts and signed application policies may restrict their own participants, counterparties or evidence issuers; permissionlessness does not waive owner authority.

Moriarty MUST support partial transactions, NEAR-style staged/asynchronous patterns and the user's precisely defined contingent settlement: submit a request addressed to a destination, withhold the specified delivery until the configured combination of signatures, documents, proofs, recipient actions and other supported conditions is satisfied. Recommended umbrella term: **conditional settlement with composable evidence requirements**. Retain **contingent settlement** as an alias; **programmable escrow** names the funded/locked variant. Recorded submission or committed funding is distinct from final economic delivery. Conditions can be conjunctions, explicit alternatives or thresholds; their exact policy is bound before authorization. A bare document hash identifies bytes, not document truth or predicate satisfaction.

## Source-grounded NEAR conclusions

1. Separate the NEAR native runtime, MPC/Chain Signatures, the Intents verifier/application and hosted1Click/solver services. None is a complete proof-carrying programming language. A signature establishes authorization under a policy; native execution/accounting checks do not by themselves prove general user-intent refinement.
2. Native transactions create receipts and asynchronous continuations. Local action rollback does not erase all durable dependency, fee or other allowed effects. Join readiness can mean results are available, including failures; it is not automatically all-success. A returned receipt chain does not establish completion of every detached branch.
3. Current code/version flags, documentation, repository master and deployed state are distinct evidence. The studies retain discrepancies in GasKeys version, refund claims and final-status explanations rather than silently harmonizing them. No deployment/admin-holder or production-assurance claim follows from reading source.
4. The pinned Intents verifier supports a defined intent algebra, signature/key/nonce/deadline bindings and token-accounting constraints. AuthCall and withdrawal can schedule later effects. Simulations can omit downstream behavior. An engine-success result must not be called complete external settlement. Native failed execution can establish local failure while other consequences remain unresolved; never flatten every failure into success or into a wholly unknown state.
5. The adjacent escrow-swap contract supplies a narrower funded-waiting/partial-fill precedent. It does not establish support for arbitrary combined documentary/signature/proof conditions. Recovery/administrative boundaries and maker/taker asymmetries must remain explicit. Moriarty needs general checked constructs, not an allowlist of these templates.
6. Chain Signatures sign bound payloads under the actual MPC policy. A signature alone does not prove an external fact, inclusion, finality or destination delivery. Bridge routes have chain/direction-specific verifier and trust boundaries. TEE attestation, operator votes and RPC-derived observations remain named assumptions unless the exact claimed property is independently proved. Marketing descriptions are not security evidence.
7. Current1Click signed-intent paths and older SDKs can differ. Service timing terms, correlation identifiers and quote signatures do not imply end-to-end idempotency, settlement or automatic refunds. A returned signature is not a delivered external transaction.

## Required Moriarty semantics

A stage judgment binds program/semantics/version, workflow and stage identities, authenticated predecessor, canonical signed intent and conditions, relevant state, authority, complete effect frame, outstanding duties, observations and cumulative resource bounds. Its target correspondence explicitly accounts for Midnight guaranteed/fallible phases. General proof feasibility remains open.

A pending workflow is a durable typed object with enough authenticated data for its allowed continuations. Dispatch, observed execution, conditional readiness, economic delivery, partial completion, recovery and unresolved outcomes are distinct where the program needs them. Do not freeze an inadequate universal enum before the supported pattern study.

Continuation correlation must distinguish request content from unique occurrence; duplicate/late messages cannot duplicate financial effects. Consumed evidence, affine permission and persistent liabilities obey different rules. Joins declare whether availability, all-success, any-success or a threshold is required, and retain duties for late/unselected branches.

Per-stage authority, consent, nonce/replay use, expiry, revocation and fees follow the signed lifecycle policy. Not every continuation requires a fresh user signature, and an expired quote/request does not automatically invalidate every already-authorized recovery. Check the applicable stage policy and current relevant facts, not a blanket cached state or blanket fresh-expiry rule.

Conditional delivery requires the specified evidence to be valid under its bound evaluation policy at release. Document identity, schema/content predicates, issuer authority, proof statement, freshness and external assumptions must be distinguished. Accumulated evidence cannot silently change destination, asset, amount, conditions or recovery rules. Private evidence requires explicit disclosure and witness-availability properties.

Partial fills require signed per-fill and cumulative rules, remaining authorization and residual duties. Keep separate typed equations for per-asset custody/transfer/supply/fees, quantitative authority and liability creation/accrual/discharge. Do not add permission units to refund amounts or liabilities to token supply.

Timeout is lack of the required observation unless reliable evidence establishes more. Cancellation and refund require the relevant still-controlled assets or enforceable claims and the signed recovery rule. Compensation is a new forward action after a committed effect, not erasure of history or an invented inverse. Late success and refund/compensation races require an explicit resolution policy. Any liveness claim names its scheduling, cooperation, evidence, resource and finality assumptions.

A conditional program may be limited by the target's supported proof/observation capabilities. Unsupported guarantees must fail validation or remain explicitly conditional; the compiler must not silently weaken them, substitute administrator discretion, or call a finite test a theorem.

## MPLR theory log and implementation plan

MPLR-001..018 are stable research-draft IDs covering staged semantics; persistent continuations; conditional settlement/evidence; partial fulfillment; recovery; correlated outcomes; joins; authority; costs; time/finality; concurrency; simulation; privacy; certified lowering; open workflow language; application/project authority separation; complete accounting; and inspectable conditional intent. They describe required behavior while leaving PL techniques open for future research. Daml is the next source study after NEAR; it may add new MPLRs or strengthen existing ones.

Translate accepted behavioral requirements into a new OpenSpec/EARS change and bounded Foreman Pel implementation package. Relate it to existing P0/P2/P4/P6/P7 work. Pel is internal engineering coordination only. Research/code/documentation counts and a panel vote do not close compiler, proof, ledger or conformance tasks.

## Deliberation corrections to retain

Some first-round proposals were too broad: mandatory partial-fill fields on every intent; unconditional blindness to counterparty identity; every timeout or failed promise forced to unknown; addition of authority/refund quantities; all continuations rechecking a universal deadline; and equating NEAR source flags with live deployment. This candidate qualifies these claims as above. Preserve the original reports as evidence of deliberation; use the corrected statements for the new requirements.

## Review response

Endorse only if the candidate accurately captures your inspected evidence and is a coherent research/specification direction. Return ENDORSE or CHANGES_REQUESTED, exact candidate SHA256, blocking changes, nonblocking caveats and evidence that would change your assessment. A scoped endorsement is not a proof or a claim to have read the complete corpus.
