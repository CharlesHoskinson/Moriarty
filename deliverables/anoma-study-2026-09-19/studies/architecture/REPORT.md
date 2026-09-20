# Anoma architecture and transfer boundaries

The checked-out project is a family of implementations and tooling, not one interchangeable binary. The node, common specification, Juvix compiler/libraries, RISC0 ARM, EVM protocol adapter, app SDK and resource examples have different version and assurance boundaries. Their individual HEADs do not establish a compatible release tuple. [AR01–04]

The productive pattern is separation of intent construction, solver completion, resource logic, structural compliance, conservation/balance and final ledger acceptance. Moriarty can adopt this separation while making the signed financial and lifecycle contract explicit. A proof of local resource validity does not automatically account for every outstanding external duty. An unbalanced resource transaction is incomplete before settlement; it is not an accepted partial business workflow with committed effects.

The Rust ARM separates shared core types from proving and concrete verifier backends. Circuit artifacts have ELF/ImageID identities; reproducible source-to-artifact generation is a distinct task. Aggregating a transaction's proofs reduces verification packaging but does not by itself prove an application's entire past or eventual completion. Moriarty should distinguish a recursive proof wrapper from the actual history relation and legitimate genesis. [AR02–03]

The generic-call example is a useful concrete bridge: resource predicates bind the chosen forwarder and exact ABI-encoded calls, and the proof output supplies payloads for execution. That closes an important byte-substitution boundary. It still requires application logic and authenticated state to connect those bytes to the user's intended assets, fees, liabilities and outcomes. A correctly bound call may implement the wrong economic policy. This is a scope distinction, not a reported vulnerability. [AR05]

The token-resource example packages witness logic, embedded guest artifacts and explicit migration versions. Its signature domains distinguish v1 from v2. This motivates a coherent compatibility manifest for source semantics, resource kind, circuit identity, public-input layout, adapter and migration policy. Current unrelated heads should never be certified just because each compiles. [AR06]

The SDK has useful ergonomics: separate discovery, encryption and authorization keys, typed resources and prepared transfer parameters. Backend parameter construction is not proof of execution or recipient receipt. Any delegated prover's access to resource secrets must be part of the disclosure model; a ZK proof hides inputs from its verifier, not necessarily from the service constructing it. The source shows parameter structure, not a complete operational privacy audit. [AR07]

## Reuse recommendations

Adopt resource-indexed obligation design, explicit solver completion, decomposed proof judgments, exact payload binding and versioned artifact identity as research directions. Adapt them to the actual Midnight guaranteed/fallible phases and the MPLR accounting/recovery contract. Keep the reference semantics independent of a specific prover or wallet service.

Do not import RISC0 receipts as if Midnight already accepts them, replace ZKIRv3 with another VM, assume Juvix proof scaffolding is discharged, or impose a project-wide deployment registry. Code reuse must follow the recorded component licenses; concepts and APIs do not imply one repository-wide license.

A minimal useful experiment is a two-party conditional exchange with one incomplete intent, a valid solver completion, an invalid added fee, a wrong resource-kind substitution and a later unresolved external leg. Show which stage each proof checks and which residual duties remain. This experiment is proposed, not executed.
