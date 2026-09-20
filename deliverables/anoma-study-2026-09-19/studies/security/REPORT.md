# Anoma ARM proof and security boundary

Anoma is substantial prior art for proof-carrying intention settlement. Its executable design separates resource-logic validity, resource-machine compliance and transaction balance. It does not merely attach signatures to solver output. The useful Moriarty comparison is therefore which authenticated policy, stage transition and target-runtime relation is certified—not whether Anoma has proofs at all.

This study read 15 key pinned source files, supplemental type/guest wrappers, five versioned official specification pages, and selected pages of a historical primary audit. Exact hashes, commits, URLs and line anchors are in `claims.json`; coverage is in `reading-coverage.json`. No proofs, tests or deployments were executed. Witness examples below are proposed tests derived from code, not results of running them.

Pinned implementations: ARM `c72c1d3d05908ed63f2f6920c18de2691ce830b4`; EVM adapter `3b1cb0299a097318281abff23e58a0a0c49f30db`; generic-call resource `d135d644948086ad7dfeacb7b7b44f8e99af0f1b`. Their independently current heads are not a demonstrated compatible release. Root produced graph extraction concurrently; the selected ARM structural navigation is saved in `graph-navigation.json`, while behavioral conclusions come from source reading.

## Explanation: what acceptance establishes

For the selected Rust ARM, a resource commitment covers the logic reference, label, quantity, value reference, ephemeral flag, nonce, nullifier-key commitment and derived randomness. Nullification requires a key matching that commitment. This establishes knowledge of the resource's nullifier secret, not automatically an owner's complete signed mandate. Applications can impose additional authorization through resource logic. The authority gadget provides ECDSA signing with an `ARM_AUTH_V1` prefix and caller-provided domain. [ARM-SEC-01,02,20]

Compliance reconstructs tags and circuit references from witnesses. Persistent resources compute a root from their commitment membership path; ephemeral resources deliberately use a supplied ephemeral root instead. Created-resource nonces are derived from the input-nullifier digest and output index. Delta points commit to signed quantities grouped by kind. The current optimization can take kind points from a supplied table, commits to that table, and relies on transaction verification checking the expected table commitment. An arbitrary table is not made trustworthy simply because a compliance proof hashes it. [03–06,09]

An action supplies the logic proof for every consumed/created tag. Verification checks count, canonical position and circuit reference; it reconstructs the action root and consumed flag for the public instance. Therefore an unrelated passing circuit cannot stand in for a resource's declared logic. The logic itself still determines application policy: proof soundness cannot repair omitted authorization or economic predicates. Padding logic is a useful counterexample to blanket restrictions—it intentionally accepts zero-quantity ephemeral resources. [07,21]

Transaction verification demands a delta proof, rejects duplicate nullifiers within the transaction, binds the expected kind-table commitment, and verifies either actions or their aggregation. Supplying both representations is rejected. Aggregated acceptance reconstructs the journal under the selected encoding and checks the expected compliance key. Delta proof verification uses the summed curve point and a transaction-bound signed message; balance relies on the circuit's delta construction and cryptographic binding assumptions, not an independent economic-value oracle. [08–13]

This library verification is not the entire ledger acceptance boundary. It does not itself consult a chain's persistent nullifier set or historical commitment-root database. An adapter must supply those state checks. The reviewed EVM adapter verifies historical roots, records nullifiers with duplicate rejection, checks declared logic against compliance references, verifies proof selector/image/journal, and requires the transaction delta proof for nonempty state changes. [14–18]

## Reference: boundaries that must remain separate

| Layer | Established by the reviewed code | Not established merely by that layer |
|---|---|---|
| Resource commitment/nullifier | Witness/tag binding and key knowledge | Human owner consent or service delivery |
| Compliance | Resource-machine construction constraints | Correctness of arbitrary application policy |
| Resource logic | Execution of the declared predicate with its bound public instance | That the predicate captures every intended restriction |
| Delta | Cryptographically bound transaction balance | Equal market value, best execution or solvency |
| Adapter | Local ledger-state checks and local EVM execution | Foreign-chain finality or external-world fulfillment |
| Batch aggregation | Proof-backed transaction statement with encoding/key binding | Recursive safety of a history of committed stages |

### Unbalanced candidates are not committed partial workflows

`compose` concatenates actions and combines delta witnesses. `verify` rejects `Delta::Witness`; the shielded v1.0.0 specification permits valid delta proofs only for balanced transactions. This supports candidate composition before acceptance. It does not imply that half of an unbalanced candidate may be committed and later globally rolled back.

A multistage workflow can instead use separately valid, balanced transitions that create explicit escrow or obligation resources. A later transition consumes the obligation upon authenticated delivery or an authorized timeout/refund branch. Such an application may be expressible with resource logic, but this study did not build or verify it. Moriarty should carry stage-local authority and residual obligations in the proved state, rather than calling candidate imbalance evidence of committed partial settlement.

### External calls and locality

The adapter calls an untrusted forwarder with the carrier logic reference and checks the returned bytes against expected bytes. A mismatch reverts. Local EVM effects therefore participate in the same transaction's revert semantics. A returned acknowledgement is only as meaningful as the forwarder and target relation enforce; it does not certify asynchronous foreign-chain completion.

The generic-call witness requires an ephemeral resource, hashes the forwarder address into the label, hashes encoded calls into the value, and emits bound external-call payload. Its `constrain` function contains no owner-signature check. That is not automatically a vulnerability: the forwarder, caller resource composition and asset-holding authorization may enforce the required authority elsewhere. It does rule out treating generic-call correctness alone as an owner-intent theorem. [17,19]

`simulateExecute` can skip RISC Zero verification but always ends by reverting, while ordinary `execute` hardcodes verification enabled. Simulation acceptance is not a persistent proof-bypass path. An owner can emergency-stop the adapter; permissionless submission and availability governance are different properties. [14,22]

### Privacy and trust

Private resource witnesses and cryptographic proofs can hide resource contents. The selected code nevertheless exposes resource logic references, public tags and app-data statements; `ComplianceWitness` contains an explicit TODO for function-privacy inputs. The versioned shielded specification describes data privacy and accommodation for function privacy, not an unconditional claim that every current field is hidden.

The EVM adapter receives app-data in calldata. A deletion criterion that suppresses an event does not erase public calldata or a solver's prior knowledge. Forwarder inputs/output are emitted by the adapter. A solver/prover given plaintext witnesses can learn them; zero knowledge toward the verifier does not hide a witness from the party computing on it. Secret-sharing, TEE isolation, delegated proving and selective-disclosure protocols require separate evidence. No default ZK/MPC/TEE federation was established by this study.

## Version correspondence and audit scope

The reviewed ARM uses vectors of consumed/created resources, an output-index/nullifier-digest nonce, a kind-table commitment and action-tree-root delta messages. The reviewed Solidity adapter uses one consumed and one created resource per compliance unit, a fixed older compliance format/key, and a tag-based delta message. The generic-call repository pins ARM `v2.0.0-rc.4`; the root ARM package declares `2.0.0-rc.5`. These facts are a compatibility obligation, not evidence of an exploit. An integration must select matching circuit ELF/image IDs, journals, encodings and adapters and test the relation.

The Informal Systems report, revised November24 2025, covers ARM v0.8.2 and adapter v1.0.0-rc.3. Its overview says two medium findings in v0.8.1 were resolved in v0.8.2; those are not asserted as current defects here. Its scope excludes `aggregation/pcd.rs` and sequential aggregation. All99 physical PDF pages were rendered with PixelRAG; physical pages4 and6 were visually inspected. This selected historical review does not certify the current heads, all99 pages, or recursive workflow correctness. [Local audit](../../repos/arm-risc0/audits/2025-11-24_Informal_Systems_RISC_Zero_RM_&_EVM_Protocol_Adapter.pdf)

## How to turn the findings into discriminating tests

Use one pinned, format-compatible implementation pair. Construct actual valid resources, proofs and adapter state before mutating one property. Each negative test must preserve the other validity conditions so that rejection demonstrates the intended boundary. These are test designs, not executed results:

| Positive witness | Hostile mutation | Required discrimination |
|---|---|---|
| Balanced transfer with known historical root, unused nullifier and declared owner-checking logic | Substitute a proof from a trivial passing circuit | Reject circuit-reference mismatch, even if substituted proof verifies in isolation |
| Matching consumed secret and commitment | Change only the nullifier secret | Reject invalid key commitment |
| Valid persistent membership at an accepted root | Use a mathematically consistent path ending at an unknown root | Adapter rejects unknown root; standalone proof validity is insufficient |
| Zero-quantity ephemeral padding satisfying trivial logic | Treat absence of prior commitment as an automatic error | Positive must remain admissible: ephemerality intentionally changes existence constraints |
| Valid resource using the chain's accepted kind table | Supply a different table with colliding/equal chosen kind points | Reject expected-table mismatch; do not rely on compliance proof alone |
| Two valid actions with distinct nullifiers | Duplicate the same consumed resource across actions | Reject intra-transaction duplication; repeat after commitment also rejects in adapter state |
| Authoritative aggregation with matching journal and no raw actions | Attach unrelated raw actions or change journal encoding | Reject ambiguous representation or invalid receipt |
| Compatible output nonce derived from input set and index | Reuse a nonce derived for another output index | Reject output-nonce mismatch |
| Bound generic call that returns the declared bytes | Mutate its target/calldata or expected result | Resource-value/proof binding or adapter output check rejects; an unchanged acknowledgement alone does not prove delivery |
| Two precommit complementary candidates whose combined balance is zero | Submit the first candidate with only a delta witness | Rejected as unready; no ledger prefix has been committed |
| Application-defined escrow stage followed by authorized delivery stage | Try to refund after delivery or drop an unresolved obligation | Application/recursive statement must reject; this property is not automatically supplied by balancing |

For authority-domain construction, use fixed-size or length-delimited context including network, adapter, policy/circuit version and role. The gadget concatenates its supplied domain and message without delimiting their boundary: `(domain="a", message="bc")` and `(domain="ab", message="c")` yield identical signing bytes. This is an encoding counterexample, not a demonstrated attack on an application using fixed domains.

## Transfer to Moriarty

Retain the separation of application logic, kernel compliance and transaction conservation. Bind every accepted predicate to its circuit identity and the exact resource/action instance. Add an explicit certified relation from authenticated formal intention to the bounded Midnight ZKIR v3 program: successful execution of a submitted circuit is not by itself proof that the circuit implements the owner's policy.

For recursive staged execution, the initial statement should bind policy, authority, initial resources, domain and obligations. A step must consume unique prior state, preserve aggregate budgets and conservation, authenticate external evidence and update remaining obligations. A terminal statement must close the selected branch's obligations. A refund terminal may differ from successful delivery. Resource-machine balance is useful inside these statements but does not replace them.

Permissionless admission should allow any developer to submit a bounded policy/program and any solver to propose a valid witness, while retaining private-data access controls and explicit verifier governance. Positive witness construction must demonstrate this is usable rather than vacuously safe. Turing incompleteness can support termination and resource bounds; it cannot establish economic adequacy, oracle truth or eventual service fulfillment. Those distinctions are the transferable security lesson, not a blanket finding that Anoma is unsafe or that Moriarty has solved them.
