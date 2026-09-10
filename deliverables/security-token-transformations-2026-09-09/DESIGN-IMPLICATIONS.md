# Assets, claims and transformations in Moriarty

Status: **S2 proposed design**, 2026-09-09. This is a brainstorming result and proposed sprint crosswalk, not an accepted language extension or a replacement roadmap. Source: SRC-0110, the supplied security-token report. [Original report](sources/report.md); [graph](graphify-out/graph.html).

## Recommendation

Make an asset's representation, the holder's claim, and the operations that change that claim explicit. Use a small, bounded semantic model with reusable product/policy profiles. Keep chain interfaces in capability adapters and preserve Midnight as the implementation target.

The report contributes a sharper distinction between technical transfer control and financial servicing. Its lifecycle section (lines 317–385) separates issuance, transfer, record dates, distributions, pledges, recovery and redemption from identity and transfer policy. Its conformance section (386–473) asks whether all normal and exceptional paths preserve the intended controls. These are source assertions and design arguments; the report's opaque external citations were not independently resolved during this intake.

This extends existing Moriarty work: token-indexed amounts, nominal liabilities, residual duties, authorization, request states and external assumptions. SP08 already explicitly says ERC-7943 transfer checks do not prove legal rights. The useful addition is a systematic transformation model and cases showing which rights survive each operation.

## Three approaches considered

| Approach | Benefit | Cost or missing guarantee | Disposition |
| --- | --- | --- | --- |
| Transfer filters and token metadata only | Smallest immediate implementation | Cannot express changing claims, encumbrances, record-date rights or residual debt on its own | Use as one capability layer |
| A Core constructor for each token standard or financial product | Direct correspondence to named frameworks | Large, version-sensitive kernel; interface names do not specify economic behavior | Do not adopt |
| Bounded assets, claims and transformation rules, with library profiles | Shared financial invariants; explicit exceptions; readable source | Requires careful scope, static rules and complete lifecycle tests | **Recommended** |

Do not add arbitrary hooks, a general TypeScript runtime, unlimited asset graphs, or a jurisdiction engine. A new primitive needs a case that cannot be expressed safely in the accepted Core, bounded semantics, lowering evidence and review. Existing primitives should be reused when sufficient.

## Proposed semantic vocabulary

| Object | Minimum information and distinction |
| --- | --- |
| Asset descriptor | Domain/network, issuer or policy, asset identifier, denomination and version. Identical tickers are not identity. |
| Position / claim | Holder and obligor, underlying reference, quantity/unit, entitlement and redemption conditions. Custody, economic exposure and legal title remain distinct. |
| Encumbrance | Pledge or settlement hold, beneficiary, bounded quantity, priority and release conditions. A compliance freeze is a separate restriction. |
| Policy capability | Operation-specific predicates, accepted credential issuers/schemas, freshness and policy version. No single `kyc: true` claim should stand for the entire trust model. |
| Authority | Holder action, issuance, recovery, seizure and migration grants with asset, operation, quantity, destination and time scope. |
| Transformation | Input/output positions, economic effects, retained or discharged obligations, required authority, policy version and declared external assumptions. |

These are proposed semantic records, not new accepted EBNF keywords. The syntax/DX research branch should try compact TypeScript-style declarations over this model and test whether users can identify what they own, what changes, and who may act. Source sugar must elaborate into the same explicit typed Core used by the evaluator, K and backend.

Candidate operation families are transfer; issue/burn; wrap/redeem; pledge/release; liquidation; split/merge or tranching; refinance/novation; and recovery/migration. Membership does not imply that every operation needs a primitive or is admitted in the atomic profile. Arbitrary minting remains outside the initial Core. Issuance/burn can require a separately admitted native capability.

A transformation record should bind the pre-state, operation and profile version, input identities/quantities, output identities/quantities, fees and rounding, authorization, changed claims and residual duties. It must distinguish an external redemption request from fulfilled redemption. Cash receipt alone cannot establish legal discharge.

## Invariants worth carrying through the language

1. **Typed conservation:** conserve each relevant asset with explicit authorized issuance/burn and fees. Wrapping is not numerical equality between unlike share and asset units; name the conversion and rounding rule.
2. **Claim continuity:** map each consumed claim to an authorized successor, an explicit discharge, or a carried residual duty. Neither a token burn nor a zero cash flow silently deletes debt.
3. **Policy-path completeness:** transfer, wrapper receipt transfer, redemption, liquidation, recovery and migration must each check the applicable policy or an explicit scoped exception.
4. **Authority separation:** an issuer/agent for asset A cannot modify unrelated asset B or exercise an ordinary holder grant as a seizure grant.
5. **Encumbrance accounting:** pledged, settlement-held and frozen amounts are distinct. Partial release/liquidation changes exact quantities and preserves outstanding liabilities and priority.
6. **Versioned freshness:** a preview is advisory. Execution binds the applicable policy/credential state and rejects stale evidence or replay under the specified model.
7. **Bounded composition:** finite positions, dependencies, credentials, events and nesting. Reject incompatible policies without partial unintended effects; report a blocked exit without claiming unconditional liveness.
8. **Explicit trust:** a valid signature or eligibility proof verifies its declared predicate under assumptions. It does not establish the truth of an issuer's external facts or legal enforceability.

For Felleisen–Hieb semantics, extend the existing state components and reduction rules only after SP01 specifies these judgments. Evaluation contexts order expression evaluation; stateful reductions describe claim/obligation changes and ordered effects. Normal, exceptional and rejected transitions need distinct premises, with deterministic evaluation/rounding and unchanged state on rejection. K must execute the same bounded rules; this paragraph is not a complete semantics or proof.

## Proposed fit in the existing agenda

Owners and sprint names below are read from the current OpenSpec files. Add these as scoped refinements after review; no task is completed by this intake.

| Sprint / owner | Proposed work | Evidence before calling it complete |
| --- | --- | --- |
| **SP01 — Financial contract and execution admission** / MC01 and existing RP owners | Define asset/claim identity, transformation accounting, exceptional authority and external trust in the semantic contract/challenge map. Extend the existing RP01 debt/loss challenge where applicable. | Independent positive/negative expected traces, exact bounds and admission scope; no extra SP13. |
| **SP02 — Complete .mori authoring frontend** / MC01, MC08 | Typed assets versus shares/claims; scoped authorities and effects; useful diagnostics. Evaluate surface sugar on the existing syntax/DX research branch. | Parser/EBNF/static semantics agreement and rejection of unit, identity, rights and authority confusion; README examples match the accepted profile. |
| **SP03 — Executable bounded semantics in K** / MC01, MC04, MC05 | Felleisen–Hieb reductions and executable K for admitted transformations and residual duties. | Independent evaluator/K agreement on complete ordered effects and post-state, negative controls, declared bounds; concrete executions distinguished from proofs. |
| **SP04 — Complete native verifier component feasibility** / MC03, MC04 | When an accepted profile changes the statement, bind asset/claim, policy and authority identities in native feasibility fixtures. | Requalify affected statement components; do not restart unaffected feasibility work merely because a report was ingested. |
| **SP05 — Financial integration on Preview** / MC02, MC04 | First use current loan/swap evidence. Then add a small restricted collateral or receipt lifecycle once its semantics and backend are admitted. | Docker first, then Midnight Preview; finalized transaction IDs and public state/effect readback. Source tests and generated keys are insufficient. |
| **SP06 — Real recursive financial history** / MC03 | Carry obligation identity and authorized transformation history, including policy versions. | Actual recursive history binds prior/new claims; reject stale predecessors, duplicate novation and erased residual duties. |
| **SP07 — ACTUS obligations and lifecycle semantics** / MC01, MC07 | Bond coupons, record-date rights, capitalization, refinance, default and redemption servicing. Distinguish event calculation from token controls. | Preserve all 277 fixtures, 18 executable types and 32 dispositions; add only justified supplementary cases. |
| **SP08 — DeFi actions and outcome intents** / MC01, MC05, MC07, MC08 | Wrapper/receipt, collateral/liquidation, pending redemption, administrative and migration behavior in the existing DA libraries. | Preserve 72 rows, DA01–DA24 and all eight intent cases; add transformation cases with complete expected rights and effects. |
| **SP09 — Mandatory PCD and ledger correspondence** / MC01, MC04, MC05 | Bind changed claims, asset domains, versions, authority and exceptional-action reasons into the accepted proof/ledger statement. | Actual correspondence and mandatory-history checks; a ledger transfer is not independently a legal-rights proof. |
| **SP10 — Private handoff and bounded composition** / MC06 | Private eligibility/claim handoff, scoped disclosure, wrapper policy closure, batch compatibility and recovery effects on connected positions. | Accepted private proof and composition cases plus stated revocation/disclosure assumptions; no unbounded discovery graph. |
| **SP11 — Full financial and formal conformance** / MC01, MC04–MC07 | End-to-end positive/negative transformation suite and behavioral capability checks. | Complete source/Core/K/lowering/proof/ledger evidence within each claimed scope; preserve all existing held-outs and regressions. |
| **SP12 — Developer release and reproducible evidence** / MC08 | Human signing display, asset/claim examples, policy failure messages, complete EBNF/semantics documentation and versioned capability limits. | Developers can explain rights, obligations, administrator powers and exit conditions from the source and signing display; reproducible evidence links. |

SP07 retains first ownership of shared financial K/Core additions; SP08 integrates after those reviewed changes under the existing one-writer rule. The syntax research branch supplies proposals, not silently adopted grammar. This intake changes no dispatch, resource or MC/RP acceptance gates.

## Concrete proposed cases

Identifiers AT01–AT08 are local to this dossier. All are **unexecuted proposals**. Independent expected results must be prepared before implementation.

| Case | Positive case | Distinguishing negative case | Agenda |
| --- | --- | --- | --- |
| AT01 Restricted wrapper | Eligible holder deposits underlying, obtains a typed receipt, transfers under the declared claim policy, and redeems through an available exit. | Unrestricted receipt movement bypasses the declared entitlement restriction; reject or explicitly identify an external unsupported guarantee. | SP01–03, SP08–11 |
| AT02 Pledge and liquidation | Of 10 receipt units, pledge 6; liquidate 2 under authority; retain 4 pledged and 4 unpledged units. Apply sale proceeds to the separately denominated debt and preserve its residual. | Release all 6 after liquidating 2, seize another asset, or erase debt because collateral was burned. | SP01/RP01, SP03, SP05, SP08–11 |
| AT03 Pending redemption | A request changes holdings into a persistent claim/duty; partial fulfillment leaves an exact remainder and permitted exit/cancel rules. | Burn the claim at request time without preserving the obligation; treat a NAV quote as paid cash. | SP03, SP08–11 |
| AT04 Scoped recovery | Recovery uses an explicit asset-scoped exceptional grant and records a new transition with remaining claims/encumbrances. | Holder transfer authority becomes an administrator grant; asset A recovery changes asset B; prior history disappears. | SP01–03, SP06, SP09–11 |
| AT05 Record-date servicing | A bounded snapshot and event ordering assign a coupon once; transfer and servicing rules determine who retains it. | Pay both pre- and post-transfer holders or assume current token possession alone determines an earlier record-date claim. | SP03, SP07, SP11 |
| AT06 Stale policy and migration | Authorized migration maps old to new identities under a pinned policy and retires the old authoritative representation. | Accept an expired credential, stale preflight, duplicate successor or unrestricted old version. | SP02–03, SP06, SP08–11 |
| AT07 Mixed-policy batch | Each leg of a bounded multi-asset settlement satisfies its applicable normal or exceptional rule. | One asset's authority bypasses another's restriction, or failed settlement leaves only one leg transferred. Cardano mixed-UTxO details remain comparative requirements, not Midnight behavior claims. | SP03–05, SP10–11 |
| AT08 Private eligibility | Prove a scoped eligibility predicate and required freshness under a declared issuer trust model with minimal specified disclosure. | Replay a proof across asset/policy domains, accept revoked evidence outside the declared time model, or leak unnecessary identity through a diagnostic. | SP02–03, SP09–12 |

## Implementation order and next decision

1. Finish the current admitted SP05 loan/swap slice and preserve its local/public evidence; this research must not restart its machinery.
2. Review the smallest SP01 asset/claim/authority amendment with AT02 as the first financial challenge, reusing the RP01 loss/residual-debt model where it truly matches.
3. Thread that one case through SP02 types/diagnostics and SP03 reductions/K. Test a complete path before broadening the library.
4. After required admissions, run the accepted case on Docker and Preview; post actual transaction IDs. Add AT01/AT03 through SP08, AT05 through SP07, and privacy/composition cases through their existing gates.
5. Requalify affected SP06/SP09 statements and finish SP11/12 coverage. A new report never waives a prior obligation.

Open choices for specification review: when a claim transfer changes its controller versus beneficiary; which restrictions follow a wrapper versus apply only at redemption; whether an operation can be represented with existing Core effects; priority semantics for partial encumbrance; and whether authority revocation is immediate or checked against a bounded snapshot. Model the chosen product rules explicitly. Do not hard-code one legal interpretation for every asset.

## Evidence limits

The report contains opaque citation tokens and links to two unavailable sandbox catalogs. Its assertions about current standard status, audits, deployment, legislation and CIP-0113 are **unverified in this intake**. Its disputed composition/third-party/unfracking concerns are questions for assurance, not established vulnerabilities. Before adopting normative interface or legal claims, acquire the exact primary source/revision with Scrapling and preserve receipts. No network requests, conformance executions, deployments or transaction proofs were performed by this research intake.
