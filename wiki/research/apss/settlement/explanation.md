---
title: "APSS settlement: explanation"
diataxis: explanation
status: research-draft
created: 2026-09-19
updated: 2026-09-19
type: research
tags: [moriarty, apss, research]
---

# Why settlement needs more than a valid execution proof

This explanation is for Moriarty language designers. It treats the language as permissionless infrastructure for Midnight developers. Managed routing and cross-chain services are optional applications, not prerequisites for deploying a supported program. Source observations below are distinguished from proposed Moriarty consequences. Acquisition and reading limits are in [reference.md](reference.md) and `manifest.json`.

## The critical Midnight distinction

**Primary-source observation:** the current [Midnight transaction semantics](https://docs.midnight.network/concepts/how-midnight-works/semantics#transaction-fallibility), captured September 19 with a September 18 update marker, separates well-formedness, guaranteed execution and fallible execution. Failure in the guaranteed phase excludes the transaction from the ledger. Failure in the fallible phase leaves guaranteed-phase effects in place, produces partial success, and does not refund fees collected in the guaranteed phase. See that section's opening three paragraphs and “Phase execution” for the application order.

**Moriarty inference:** an evaluator's atomic rejection is not sufficient evidence for atomicity of the compiled ledger transaction. The compiler, proof predicate, authorization and receipt must agree about phase placement. A financial action can fail its intended business outcome while incurring an authorized fee or preserving another guaranteed effect. Conversely, merely checking a proof or finding a transaction in a block does not establish successful settlement. This is a correspondence obligation, not a reason to weaken the financial success predicate.

An intention should bound every permitted terminal and unresolved outcome: total debit, fees, outstanding authority, liabilities and recoverable claims. Success additionally requires the requested net outcome. Partial success needs its own evidence and state transition; renaming it “failed” would lose real effects, while calling it settled success would overstate delivery. The exact deployed version and transaction shape must be examined before mapping this documentation to a concrete Moriarty run.

## Four claims that must travel separately

1. **Execution correctness:** a particular relation holds between committed inputs, state, outputs and effects. Its strength depends on the program semantics, encoding, proof system and compiler/ledger correspondence.
2. **Observation authenticity:** a source, quorum or light client authenticated an observation. Signature verification proves endorsement; source-ledger proofs additionally require the named consensus and client assumptions. Neither establishes arbitrary external truth.
3. **Accepted history and finality:** a specific deployed ledger accepted a transaction and finalized the relevant state/effects according to its finality policy. Prior proof validity does not ensure currentness or prevent a competing spend by itself.
4. **Liveness:** a participant can eventually obtain the intended outcome or a specified remedy. This requires availability, liquidity, witnesses, inclusion and finality assumptions beyond a finite execution bound.

The [Compact reference](https://docs.midnight.network/compact/reference/compact-reference) explicitly treats witness results as untrusted inputs in “Witness declarations.” An honest reference implementation of a witness is not the contract's security boundary. The [Kachina paper](https://eprint.iacr.org/2020/543.pdf), Appendix F, likewise makes the absence of liveness in its base model explicit before discussing a stronger ledger model. These are useful checks against treating “uses ZK” as a complete assurance statement.

## What atomicity means in adversarial finance

[Herlihy's atomic-swap model](https://arxiv.org/abs/1801.09515) distinguishes acceptable outcomes from a party paying without receiving the assets it expects. Its protocol relies on a known publication/confirmation delay and specified graph structure. This is not an all-purpose guarantee for arbitrary chains that halt or censor.

[Cross-chain Deals](https://www.vldb.org/pvldb/vol13/p100-herlihy.pdf) broadens the model to participant-specific acceptable payoffs and separates safety, release of escrow and successful completion. This suggests a financial language should express acceptable partial progress explicitly instead of offering one ambiguous atomicity flag. Residual debt and authority must remain visible; a compensating transfer is a new action, not deletion of a finalized old one.

The [IBC packet specification](https://github.com/cosmos/ibc/blob/6eb8792e987220d7afcc8c926426f3af5695cb7b/spec/core/ics-004-channel-and-packet-semantics/README.md#timeouts) supplies a concrete example: safe timeout processing depends on destination-side proof, rather than silence at the sender. An unavailable destination can delay recovery. Its acknowledgement format also distinguishes transport evidence from application success. Moriarty should preserve the same conceptual separation even where it does not implement IBC.

## History proofs do not consume a ledger object

[Proof-carrying data](https://conference.iiis.tsinghua.edu.cn/ICS2010/content/paper/Paper_25.pdf) describes compliance over a computation history. [Nova](https://eprint.iacr.org/2021/370.pdf) offers a construction for incrementally proving repeated computation. Neither paper makes arbitrary input observations true, prevents a ledger from accepting two competing successors, or supplies Moriarty's financial relation automatically.

Moriarty therefore still needs durable unique predecessor consumption, cumulative intent accounting, explicit genesis, complete financial effects and mandatory ContractInvariant, IntentRefinement, TransitionValidity and HistoryCompliance on every permitted path. Proof verification must connect to the actual acceptance boundary. A recursively maintained accumulator is not automatically a final, independently verified proof or a deployed verifier.

## Routing freedom within signed constraints

[CAKE](https://frontier.tech/the-cake-framework) separates permission, solving and settlement. Its permission layer concerns user authorization, while its solver access policies are independent choices. [ERC-7683's captured draft](https://eips.ethereum.org/EIPS/eip-7683) standardizes resolver descriptions rather than a universal secure settlement contract. Its explicit assumptions are useful; a solver's local vetting process must not become administrative permission to deploy Moriarty programs.

An outcome-oriented intention can leave routing open while fixing asset identity or a bounded substitution predicate, domain, beneficiary, maximum gross debit, minimum successful net credit, fee and liability limits, expiry, replay identity and permitted evidence classes. A solver proves refinement of those terms; it cannot widen them by renaming a wrapped asset or changing an alias table.

CAKE's fees/speed/execution trilemma is a design taxonomy, not a proved impossibility theorem. [Flash Boys 2.0](https://arxiv.org/abs/1904.05234) supplies historical evidence that ordering and revealed intent affect costs and security. Its Ethereum measurements do not establish current Midnight MEV levels. For Moriarty the actionable implication is to preserve fee caps and information-flow policies even for execution-valid routes.

The proposed phase-aware contract amendment is in contract-amendment.md (`.raw/captured/apss-2026-09-19/settlement/contract-amendment.md`). It preserves the Midnight target and existing acceptance obligations; it does not launch a foreign-adapter program.


Evidence archive: [captured source manifest and reading records](evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.
