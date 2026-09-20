# Design boundaries

Moriarty specifies permitted transitions. Solvers propose candidates. The kernel checks evidence, coordinates work and records external effects. Midnight executes compiled ZKIRv3 contracts.

An accepted proof must bind intention, program, target relation, predecessor state, authority, complete effects and residual duties. Prove legitimate base cases and preservation. Prove constraint soundness against arbitrary witnesses. Keep privacy and liveness claims separate.

OWS is an interoperability interface. Its local policy path is not cryptographic attenuation of a recoverable wallet secret. A constrained destination or explicitly trusted signer must enforce delegation. No current Midnight adapter is assumed.

x402 provides scheme-specific payment states. Authorization, reservation, finality, result availability and recipient delivery remain distinct. Preserve concurrent budgets and logical request identity across retries.

ZK, MPC and TEE evidence must bind the same effect. Declare threshold, hardware, external-state and finality assumptions. A destination accepting a bare threshold signature retains the threshold-corruption premise.

No project reviewer, service membership or solver registration is a language deployment prerequisite. Maintainer verification governs repository delivery only.

Source design: [AI solver study](../../../wiki/research/ows-x402/explanation.md), [security direction](../../../wiki/research/daml/security-provability.md), [Simplicity](../../../wiki/research/simplicity/index.md).
