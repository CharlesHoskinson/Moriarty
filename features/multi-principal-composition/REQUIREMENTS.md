# Multi-principal intent composition: requirements

EARS requirements. IDs use the prefix **MPC**. Each row is a design requirement with a planned verification. No command, harness or proof exists yet, and none is claimed. Concept references point to [CONCEPT.md](CONCEPT.md).

## Composition and acceptance

| ID | Requirement | Concept | Planned verification |
|---|---|---|---|
| MPC-001 | The language **shall** let a covenant over one governed object bind a set of components, each signed by its own principal, through the reserved stage arity of MIL/2 decision 4. | §3.1–3.2 | Grammar and canonical-form tests; digest covers every component digest |
| MPC-002 | Each component **shall** separate constraints from grants, and the composition **shall** intersect constraints and take a scoped union of grants. | §3.1 | Typing rules; negative test in which a grant appears inside a constraint |
| MPC-003 | When a stage touches a component's scope, acceptance **shall** require that component's constraints to hold, unless the effect is exempt under a consent that the component's principal signed. | §3.3 (B), §3.4 | Positive and negative scenarios per component; hostile exemption claimed without consent |
| MPC-004 | When a stage writes or debits a cell, acceptance **shall** require an unconsumed, unrevoked right covering that cell, rooted in a grant by the cell's owner. | §3.3 (A) | Hostile scenario: a regulator or venue component attempts a spend with no owner grant |
| MPC-005 | The language **shall not** give any role precedence. Only signed consent capabilities and a signed `Order` **shall** resolve overlap between components. | §1, §3.4, §5 | Review check; negative test in which a role label alone overrides a constraint |
| MPC-006 | When two rights contend on one cell at one position, acceptance **shall** select by the covenant's signed `Order`. | §5 | Freeze-vs-spend and seize-vs-seize scenarios |
| MPC-007 | Every live obligation and encumbrance **shall** be preserved or explicitly discharged by an accepted stage, including duties a component requires (e.g. withholding). | §3.3 (C), §10.1 | Omission controls; missing-withholding negative case |

## Safety of the composition

| ID | Requirement | Concept | Planned verification |
|---|---|---|---|
| MPC-010 | `Compose` **shall** reject a component set that admits no effect reaching a terminal or exit state, using a decidable Φ₀ check that fails closed when undecided. | §5 | Difference-logic checker; unsatisfiable-pair corpus |
| MPC-011 | An admitted covenant **shall** keep at least one exit route enabled at every reachable state, except during a consented encumbrance with a bounded duration. | §4 law 5 | Proof obligation; finite model check of the restricted profile |
| MPC-012 | When a principal amends its component, existing obligations and encumbrances **shall** keep the pinned digest, and the amendment **shall** apply only to new actions whose affected principals endorsed it. | §5 | Amendment-drift scenarios |
| MPC-013 | Removing a component **shall** require its own principal, and **shall not** remove another principal's protection. | §4 law 3 | Hostile removal by a non-author |

## Agents, identity, evidence and skein

| ID | Requirement | Concept | Planned verification |
|---|---|---|---|
| MPC-020 | The language **shall** express a mandate from a principal to an agent DID, with an outcome, affine budget, expiry, revocation and sub-delegation that can only narrow. | §3.5 | Mandate grammar; over-budget and widened-sub-delegation negatives |
| MPC-021 | When an agent proposes a stage, acceptance **shall** check the mandate and every touched component, including the principal's own. | §3.5 | Corrupt-agent scenarios that propose stages outside the composition |
| MPC-022 | Principals and grant holders **shall** be DIDs resolved at the signing position. | §6 | Resolution at stale and rotated key positions |
| MPC-023 | The language **shall** provide a typed evidence class for credential presentations bound to the stage context, subject to MIL/2 source-set propagation. | §6 | Non-laundering tests: a presentation from an issuer the component did not name |
| MPC-024 | Each component's evaluation **shall** receive only the evidence its audience is permitted to see. | §6 | Audience-restriction tests |
| MPC-025 | Every accepted stage of a bound covenant **shall** emit a skein entry as part of its canonical statement, and a stage without one **shall not** be accepted. | §6 | Canonical-statement tests; a missing entry is rejected |
| MPC-026 | The canonical stage statement **shall** represent each component only as a commitment to its digest and its acceptance result. | §7 | Statement-schema review; leakage checklist |

## Profile and placement

| ID | Requirement | Concept | Planned verification |
|---|---|---|---|
| MPC-030 | A restricted profile (≤ 5 components, Φ₀ constraints only, enforcement only through consent, one mandate level) **shall** be specified first. | §9 | Profile header; Monument example expressed in it |
| MPC-031 | Layered governance **shall** be placed separately from n-party clearing, which stays post-U4. | §2 | Roadmap amendment, pending owner decision 1 |

## Obligations before adoption

Each must be proved, or checked by an executed differential, before this feature is adopted into MIL/2:
1. every principal is honored;
2. grant locality;
3. monotonicity;
4. the `⊗` algebra;
5. exit safety;
6. refinement under completion for the composite;
7. soundness of the static conflict check for Φ₀.

All are recorded as **[obligation]** in CONCEPT.md §4, and none is claimed.
