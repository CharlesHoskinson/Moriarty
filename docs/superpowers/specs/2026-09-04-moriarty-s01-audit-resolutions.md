# S01 audit resolutions

Status: execution specification under the user's 2026-09-04 instruction to
finish the complete XML prompt. Classification: recommendation and specification.
This document adds definitions to the pinned S01 design. It does not change Core.

## Inputs and alternatives

The inputs are XML v1.3, the approved 2026-09-03 S01 design, and the
[execution audit](../reviews/2026-09-04-moriarty-v1.3-execution-audit.md).
Their original bytes and the Core scope `0.0.0-e00.2` remain unchanged.

Three interpretations were considered for authorization observations. A display
alone can omit effects. Requiring every actor to see every private value defeats
the confidentiality boundary. The selected interface separates display data
from complete authorization evidence.

These definitions specify obligations for later mechanisms. They do not prove
that a confidential implementation, compiler, or ledger satisfies those obligations.

## A01: complete authorization evidence

`DisplayProjection` selects data visible to an actor. It is not authorization evidence.
`AuthorizationProjection` accounts for every effect in the complete trace and
outcome, including private effects, fees, refunds, change, and failed-path effects.

`view(actor, D, L, T, O)` is an authorization view with two distinct components:
the display projection and the authorization projection. An implementation may
represent private effects with commitments and verification evidence. It must
bind that evidence to the complete trace, outcome, domain, settlement level,
signed intent, and applicable hard predicates.

Commitments alone do not establish effect completeness. A complete projection
has evidence of both inclusion and exclusion: every actual effect is covered,
and no undeclared effect can satisfy its relation. `verifyPlan` may return a
valid full certificate only after the selected profile's completeness checks pass.
Missing, stale, substituted, or uncheckable evidence produces `unavailable` or `invalid`.

`authorizedAt` evaluates every hard predicate over the authorization projection.
It never derives permission from the display projection. Declassification changes
visibility only. It never adds an allowed effect or expands a capability.

The S01 Python checker compares supplied transfers only. It implements neither
projection authentication nor a full `verifyPlan` certificate. Every local result
continues to deny signing permission. S04 and later sprints must discharge the
projection correspondence obligations before a stronger claim is allowed.

## A04: signing-order profiles

`SignAfterResolve` resolves a concrete plan before the signer authorizes it.
Its signing request requires the full plan, authenticated state, complete effect
evidence, all hard predicates, and every selected-profile premise.

`SignBeforeResolve` authorizes a bounded intent before a concrete plan exists.
Its initial signing request requires validation of the intent's hard bounds,
domain, nonce, validity, cancellation policy, signers, capabilities, disclosures,
and the identity of the execution-boundary enforcement mechanism. It must not
claim approval of an unresolved plan. The mechanism must check the later plan
against the authorization before consuming assets or exercising authority.

The candidate theorem's `plan-valid` premise describes the full plan check at
the applicable authority boundary. For `SignBeforeResolve`, that boundary is
execution. For `SignAfterResolve`, the check precedes signing and is revalidated
at execution when required by state freshness or authorization rules.

T09 and sprint S07's general pre-sign wording apply to concrete-plan signing in
`SignAfterResolve`. The XML's explicit signing-order rules and prohibited shortcut
against pre-sign approval of unresolved plans govern `SignBeforeResolve`.
Record this reconciliation in the ambiguity registry with both source locators.

Neither profile is implemented by the local transfer checker. Model and test
both profiles before a production signing or execution authority is issued.

## A06: terminology and atomicity

The terminology registry must cover every XML `required_terminology/term`, all
27 W4 lifecycle objects, and all W5 and theorem terms listed in the S01 plan.
Use a canonical noun and explicit source aliases. Distinct concepts must not
share an alias that changes authority or settlement meaning.

Replace unqualified `partial_fill` at authority boundaries with one of these
explicit semantic kinds:

- `DivisibleEconomicFill`: a quantity fraction with a conserved residual obligation.
- `PartialOutputCompletion`: some required outputs exist while others remain outstanding.
- `SameChainTransactionAtomicity`: all state changes commit in one ledger transaction or none commit.
- `ContiguousWalletExecution`: ordered wallet calls execute contiguously under the declared wallet mechanism.
- `CrossDomainAllOrRefund`: either all required domains fulfill, or all affected
  domains satisfy the jointly authorized refund predicate. Mixed fulfillment
  and refund does not satisfy this kind. Compensation is not a refund.
- `EndToEndAtomicity`: the declared application-wide outcome commits as one indivisible semantic result.

The XML term `partial_fill` names this ambiguous family only. It is not itself
an authorization predicate. Each use must select one kind and settlement level.
Cross-domain all-or-refund is conditional on named finality and liveness
assumptions. It does not assert simultaneous commits or end-to-end atomicity.

## G17: distinct empirical obligations

ACTUS is an automated benchmark. It cannot express a human preference.
Keep the S01 plan's explicit resolved predicate:

`two-human-pilots-prefer-workflow AND actus-g19-through-g24-pass`.

Preserve the original G17 text and locator in the ambiguity registry. Human
pilot results and ACTUS results require separate evidence. Neither has passed
merely because this interpretation is recorded.

## Validator and evidence corrections

Keep the ten named S01 gates. Enforce the complete required sets, unique IDs,
exact theorem premise expressions, exact conclusion, required bindings, and
candidate-unmechanized status. Include projection and signing-profile bindings.
Hard predicates must enter authorization. Preferences must never enter it.

Reconstruct the canonical Core settlement independently of the stored vector.
Require exact baseline policy and effects, one exact extra third-party effect,
the stable rejection reason, restored-baseline equality, and no signing permission.

Pin the original prompt, approved design, this supplement, scope snapshot,
and relevant Core sources as immutable inputs. Hash the checker, validator,
schema, normative spec, and each required artifact. Reject missing, duplicate,
undeclared, escaping, or substituted paths and inconsistent artifact identities.

Separate gate computation from receipt comparison. An explicit report-writing
mode may emit reports only after semantic gates pass. Read-only validation must
recompute gates, verify all digests, and compare recorded results with actual results.
Do not create passing receipts by copying the plan's expected output.

## Exit condition

S01 may close as a tested specification freeze when its validator and mutation
tests pass. Its candidate theorem remains unmechanized. S02 retains architecture
selection, S04 retains mechanization, and later sprints retain production authority.
No empirical, proof, backend, ledger, ACTUS, or pilot gate passes from this document.
