# Authority security and proof-carrying lineage

Research proposal, not a theorem or implemented feature. This note applies the user's security direction to the Daml authority evidence in [REPORT.md](REPORT.md). Turing incompleteness and proof-carrying data (PCD) are intended to reduce attack surfaces; neither is sufficient by itself. The target remains permissionless supported-program development and actual Midnight ZKIRv3 execution.

## Adversary and explicit assumptions

An attacker may author a supported program, submit arbitrary validly encoded states, impersonate a proposed economic role in data, invoke public operations outside the advertised UI flow, choose proof witnesses, use a modified prover, replay signatures or predecessor proofs, compose privately revealed subtransactions, race revocation against use, and select a newer implementation where resolution permits. An authorized controller may be malicious or may exercise a broadly granted role against the principal's interests. A preparing service may return a misleading transaction display or hash.

Assume only the specifically named cryptographic primitives, bound verifier relation and keys, authenticated ledger state/uniqueness mechanisms, and declared external-observation premises. Compiler, interpreter, signer display and proof-producing service correctness are separate obligations, not implicit trust inherited from being the usual client. Current Canton documentation is evidence for the comparison; Canton hosting or package vetting is not a Moriarty public deployment rule.

## Attack-to-obligation derivation

| Attack grounded in the study | Desired invariant and local check | Composed obligation / adversarial witness |
|---|---|---|
| Obligation injection: a transfer manufactures recipient agreement; positive numeric amount hides issuer/fee/duty terms | Every new or increased protocol obligation is covered by that party's direct consent or applicable standing policy over material terms | Preserve the policy across transfers, partial fills and amendments. A positive token position plus unauthorized guarantee must reject; an authorized passive credit without new duty remains admissible. DA04/DA05/DA06 |
| Ambient privilege escalation: a helper inherits all outer authorizers | Derive authority at each action boundary from explicit scoped grants; validate both required roles and action semantics | Nested `TryB -> TryA` cannot recover Bob's dropped authority. A deliberately granted attenuated role must still work. DA01/DA02/DA03 |
| Forged workflow provenance: attacker directly creates `Approved` | Genesis must establish legitimate initial state; successor states need authorized predecessor transitions | A valid-shaped approved record without approval lineage rejects. A genuine proposal/acceptance lineage passes, including private proof of the hidden parent context. DA07/DA12/DA15 |
| Replay and race: reuse a role, signature or predecessor proof after consumption/revocation | Bind domain, workflow/stage/attempt identity, consumed state/resource IDs and the signed revocation policy | Two individually valid successor proofs must not both spend one resource. Ledger uniqueness or equivalent shared-state evidence is required; PCD alone cannot prevent a fork. DA01/DA23 |
| Unauthorized upgrade: preserve field shapes but change controller, release condition or beneficiary | Resolve and bind exact semantics; require the original allowed evolution policy or fresh scoped consent | An upgrade proof preserving record shape but widening a role rejects. Certified equivalent optimization within the authorized profile remains possible. DA25/DA26/DA27 |
| Liability erasure: authorized archival is treated as discharge | Distinguish consuming a representation from paying, releasing or otherwise authorized discharge of its economic duty | The issuer-only IOU archival and `Settle = pure ()` cannot prove payment merely from record disappearance. DA04/DA24 |
| Projection laundering: an internally valid subtree is presented as fully authorized | Prove its actual root context or a valid scoped authorization import | The issuer's projection of a transfer lacking Carol's consent is insufficient. Revealing contract data never supplies Carol's spending authority. DA12/DA13 |

## Local proof relation and recursive composition

A prospective compliance statement binds program/semantic version, property specification, target artifact and verifier relation, signed intention and amendment policy, authorization principals and applicable keys, execution domain, predecessor states/proofs, complete effects, residual duties, resource identities and bounds, and external observations. A commitment may hide a component, but the relation must still constrain its correct value and equality to the component consumed by execution.

The local relation checks at least:

1. The current signed policy is authentic for the bound principal and relevant domain; any key/delegation revocation follows the declared stage policy.
2. The authenticated input state and its authorization context satisfy the action's preconditions. A record's `owner` or `Approved` field cannot manufacture the authority it names.
3. Required authority is available at each nested boundary and the actor set agrees with the bound action semantics. Merely relabeling an actor cannot evade a missing signature.
4. New/increased liabilities have consent and preexisting liabilities have an explicit conserved/discharged successor. Every fee, transfer, reservation and recovery effect is accounted for.
5. The target constraint fragment enforces the same allowed success and failure outcomes, including effects retained by Midnight phase semantics. Correct honest witness generation is not enough; adversarial accepted witnesses must also satisfy the relation.

PCD adds recursively checked provenance and state lineage. A base proof validates initial authority, initial resources and permissible initial state. A successor verifies predecessor proofs under the designated relation and binds their outputs to its actual inputs. A join preserves every relevant resource and duty from its required branches; it cannot substitute an unrelated valid predecessor or duplicate authority by citing the same proof twice. Hidden parent context can be proved without publishing every party, but the public statement still needs the appropriate binding to the actual state and verifier.

A list of signed events, a hash chain, an authenticated DAG, or a service saying “verified” is not PCD. Those may bind provenance bytes without establishing the induction step or excluding invalid history. Likewise, independently valid branch proofs do not establish mutually exclusive consumption: the actual ledger must enforce uniqueness and current state, or the relation must incorporate sufficient authenticated shared-state evidence. An authenticated stale root is not automatically a current root.

Safety induction is conditional: valid genesis, sound recursive verification and sound local compliance can preserve the stated invariant along accepted transitions, provided the ledger and named external premises hold. It does not manufacture document truth, recipient cooperation, asset solvency, inclusion, liquidity, chain finality or eventual settlement. An unresolved remote leg remains unresolved even when the local PCD proof is correct.

## Language restrictions to investigate

Security should determine the supported language profile, rather than arrive as an audit checklist after syntax selection. Candidate restrictions are:

- A small total semantic core, bounded data and iteration, explicit costs, and no unchecked native escape hatch into the certified acceptance relation. A finite computation can still implement theft or exceed practical resource budgets; totality is only one premise.
- Explicit authority parameters and attenuation at nested calls; no implicit capture of an ambient global signer set. Consider authorization/effect judgments or capabilities, without prematurely requiring Daml's exact context rule.
- Controlled introduction of provenance-bearing states, through validated genesis or transitions. Ordinary constructors can remain available for nonauthoritative data; the protected claim must not be forgeable by populating a record.
- Separate affine spending permission from persistent liability and reusable evidence. Reuse of a proof is not reuse of spendable resources.
- Versioned late binding with semantics/authority/resource obligations. Do not treat matching interface shape or package-upload eligibility as proof of intention preservation.
- Explicit conditional-evidence predicates and disclosure effects. Attestation authenticity and the truth of the asserted external proposition require different assumptions.

These are hypotheses to compare against positive and negative witnesses. Bounded on-ledger stages may compose into an unbounded-duration workflow; Turing incompleteness must not be misreported as eventual completion. Off-ledger search can remain unrestricted because its proposed results pass the same objective public verifier.

## Non-vacuity and next evidence

A useful implementation must accept a legitimate buyer/seller conditional exchange with authorized standing delegation, private evidence and one recoverable partial outcome. It must also accept an independent proposer satisfying the same signed application policy. An implementation that rejects every workflow is not a solution.

The minimum hostile family mutates one field or premise at a time: obligation recipient, issuer, amount, fee, controller, parent context, predecessor root, semantic version, circuit/key, revocation state, and consumed resource ID. Include two concurrently valid-looking proofs competing for one resource, a late release after a refund attempt, and a current-state proof replaced by a signed stale state. The expected outcomes distinguish rejection, pending/unresolved state and permitted recovery; they do not collapse every failure into “unknown.”

No PCD implementation, backend proof, model checker run or Daml execution was performed in this track. [MPLR-candidates.json](MPLR-candidates.json) records the behavioral refinements and open theory choices for later verification.
