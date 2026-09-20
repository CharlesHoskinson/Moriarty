# EARS and MPLR traceability

Each behavior has a concrete positive and negative witness. Implementation and target proofs remain open.

## MPLR-019

When a transition creates or increases a party's enforceable financial or operational obligation, it SHALL establish that party's consent to the obligation and its material terms, either directly or through an applicable prior authorization policy. A transfer to an address alone SHALL not manufacture such consent.

Positive witness: When a recipient signs the bounded debt terms under the applicable policy, the implementation shall record exactly that authorized obligation.

Negative witness: When a sender transfers a contract reference that makes an unconsenting recipient liable, the implementation shall reject creation of that recipient obligation.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-019.md).

## MPLR-020

When a certified primitive substitutes for a reference expression, acceptance SHALL bind its version and establish preservation of reference semantics, preconditions, failure behavior and the relevant host/target correspondence.

Positive witness: When a jet has a verified certificate for the pinned reference relation and its call-site preconditions hold, the implementation shall preserve reference values, rejection, complete effects and the certified cost relation.

Negative witness: When a substituted jet drops a required carry check or assertion, or uses a certificate for another version, the implementation shall reject the substitution or its claimed certified status.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-020.md).

## MPLR-021

Before accepting a stage under a bounded-execution claim, the verifier SHALL establish a resource bound for the concrete admitted artifact and any late-bound component under the bound cost model.

Positive witness: When a bounded loop and all invoked components fit the checked cost certificate for the emitted artifact, the implementation shall accept the stage within that declared resource bound.

Negative witness: When late-bound code has no checked bound or the actual artifact exceeds the claimed work limit, the implementation shall reject acceptance under that resource certificate.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-021.md).

## MPLR-022

Acceptance SHALL bind program identity, semantic/profile version, signed intention, state and evidence according to their distinct commitment roles, and SHALL validate every permitted witness or program substitution against that binding.

Positive witness: When witness data opens the required commitments for the same program, intent, stage and domain, the implementation shall check the bound relation while preserving its stated privacy policy.

Negative witness: When evidence from another program, stage or chain is substituted despite matching data types, the implementation shall reject the mismatched evidence.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-022.md).

## MPLR-023

Every declared mandatory intention predicate SHALL constrain transaction acceptance through a proved control/effect relation; computing or displaying a predicate without enforcing it SHALL not satisfy that requirement.

Positive witness: When the selected acceptance branch requires a predicate and valid evidence establishes it, the implementation shall permit that branch when its other required conditions also hold.

Negative witness: When a mandatory predicate is false but its computed Boolean is discarded, the implementation shall reject that branch despite successful execution of the Boolean computation.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-023.md).

## MPLR-024

When a workflow claims atomic settlement, it SHALL identify the ledger/domain and the exact effects inside that atomic boundary; relocation and independent-chain effects SHALL retain separate pending and final states.

Positive witness: When all prepared inputs and required legs execute in one declared settlement domain, the implementation shall record atomic completion only for the effects within that domain.

Negative witness: When unassignment has completed but assignment to the next domain remains pending, the implementation shall retain pending relocation and deny the claim of completed settlement.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-024.md).

## MPLR-025

When gross obligations are netted, the transition SHALL preserve each authorized asset, party, fee, liability and residual duty under an explicit gross-to-net relation.

Positive witness: When a net schedule proves equivalence to the authorized gross assets, fees, liabilities and duties, the implementation shall accept the optimized schedule with the same authorized economics.

Negative witness: When equal final net balances hide an unauthorized fee or changed issuer liability, the implementation shall reject the net schedule.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-025.md).

## MPLR-026

When a workflow invokes an interchangeable settlement implementation, acceptance SHALL establish its required financial and authority postconditions; matching an interface alone SHALL not discharge these obligations.

Positive witness: When an implementation establishes every required transfer, consumption and authority postcondition, the implementation shall accept its use through the interface.

Negative witness: When an implementation returns Pending or omits a required delivery while claiming final settlement, the implementation shall retain the pending duty or reject the incorrect completion claim.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-026.md).

## MPLR-027

When an accepted transition relies on recursive history, its proof SHALL establish legitimate initial state, compatible predecessor statements and well-founded composition, including every consumed resource and residual obligation.

Positive witness: When authenticated genesis and compatible well-founded predecessors establish the current state and unused resources, the implementation shall accept a preserving transition subject to actual ledger uniqueness.

Negative witness: When a prover invents already-funded genesis or uses a cyclic or wrong-relation predecessor, the implementation shall reject the recursive compliance claim.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-027.md).

## MPLR-028

When code, verifier, policy or persistent continuation evolves, acceptance SHALL preserve the signed semantics, residual duties, recovery rights, privacy and resource constraints, or obtain applicable amendment authorization.

Positive witness: When authorized migration proves preservation of residual duties, recovery, privacy and resource constraints, the implementation shall permit the continuation under the bound new version.

Negative witness: When an interface-compatible update changes the beneficiary without applicable amendment authority, the implementation shall reject that migration.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-028.md).

## MPLR-029

When a transition relies on absence, uniqueness or completeness, its proof SHALL bind an authenticated state domain and establish the required nonmembership or completeness property under explicit ledger assumptions.

Positive witness: When a proof establishes nonmembership in the authenticated current domain required by the policy, the implementation shall use that absence fact within the declared completeness assumptions.

Negative witness: When a private lookup returns None only because an existing reservation is not visible, the implementation shall reject the global absence claim.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-029.md).

## MPLR-030

When the kernel combines proofs, attestations and signing authority, it SHALL bind them to the same intention, domain, stage, epoch, artifact and exact effects, and expose the threshold, hardware, observation and recovery assumptions.

Positive witness: When proof, required attestation and signing authorization bind the same effect, domain, stage and authorized epoch, the implementation shall accept their conjunction under the declared threshold and hardware assumptions.

Negative witness: When a proof authorizes transfer X but the signer is asked to sign transfer Y, the implementation shall reject the mismatched effect before signature release at the declared enforcement boundary.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-030.md).

## MPLR-031

When a solver requests a financial effect, acceptance SHALL enforce the owner's bounded delegation across wallet, agent and signer boundaries.

Positive witness: When an unregistered solver presents valid delegated authority for the exact permitted effect, the implementation shall permit the supported action without project registration.

Negative witness: When an agent token is used to request an effect outside the owner delegation, the implementation shall reject the effect at the stated enforcement boundary; do not claim local policy alone prevents key extraction.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-031.md).

## MPLR-032

When concurrent tasks or retries commit funds, acceptance SHALL reserve and account for spent, pending and residual amounts, including fees and sponsored execution.

Positive witness: When a ten-unit budget already reserves six units and a second request commits four including fees, the implementation shall reserve the second request without exceeding ten units.

Negative witness: When the same budget receives concurrent six-unit and five-unit commitments, the implementation shall reject or defer at least one commitment so total exposure cannot exceed ten.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-032.md).

## MPLR-033

When a service is purchased, the program SHALL distinguish payment finality, result availability and delivery to the authorized recipient, retaining each unresolved obligation.

Positive witness: When payment finality and recipient delivery each have the evidence required by the signed policy, the implementation shall record both completed obligations.

Negative witness: When payment is final but the result is available only inside a TEE vault, the implementation shall retain the recipient-delivery duty and deny whole-purchase completion.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-033.md).

## MPLR-034

When a request is retried or recovered, the workflow SHALL preserve one authenticated logical obligation and distinguish result retrieval from new billable work.

Positive witness: When a retry names the same authenticated request whose payment is already final, the implementation shall reconcile that payment and apply the permitted result-retrieval policy without a second charge.

Negative witness: When replayed transport data requests a second payment or a new billable service under the old authorization, the implementation shall reject the extra effect while preserving authorized recovery.

Source: [MPLR note](../../../wiki/research/mplr/MPLR-034.md).
