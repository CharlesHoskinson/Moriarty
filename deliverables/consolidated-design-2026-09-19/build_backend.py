from pathlib import Path
R=Path(__file__).parent; C=R/'candidate';p=C/'docs/MORIARTY-BACKEND-REQUIREMENTS.md'
s=(R/'astra-backend-requirements.md').read_text(); table=s[s.index('| ID and class'):s.index('## Semantic requirements')]
header='''# Requirements for the next ZKIR and Midnight recursion release

Status: explicit proposed interface contract, specified-only. Owner: joint Midnight backend / Moriarty integration work. Date: 2026-09-19. The user expects comprehensive Midnight recursion in about six months, approximately March 2027. This is the planning horizon for the full architecture, not a independently verified release date. Current execution targets ZKIRv3; “next version” does not assert a successor version number.

These requirements make the platform dependency actionable. They request comprehensive native recursion for developer-defined relations, portable compliance and private bounded multi-parent composition—not merely proof aggregation. No Lean dependency or alternate proof backend is introduced. [Unified roadmap](../ROADMAP.md): U0 freezes this contract; U1 investigates compatibility/cost; U2 integrates current supported paths; U4 qualifies full recursion/private composition; U5 qualifies external adapters. No current limitation permanently narrows that target.

“SHALL” below states the requested behavior. All acceptance tests are specified, not executed by this design work. Cryptographic soundness and correspondence need arguments as well as tests. Performance targets remain unquantified until measurements support them; no arbitrary latency or proof-size promise is imposed.

## Requested capability and correctness list

C = mandatory correctness/functional capability. P = measured performance or service objective. Present-support notes refer to local historical source pins; they are not current production claims.

'''
footer='''
## Responsibility and acceptance ownership

| Requirements | Primary implementation owner | Moriarty responsibility / milestone |
|---|---|---|
| ZR01 | ZKIR/compiler/proof-runtime and ledger release interface | Pin and validate compatible tuple, including node proof-verification configuration; U0/U1 |
| ZR02, ZR05, ZR08, ZR09 | Native recursive proof API, constraint implementation and ledger final verifier | Express legitimate compatible financial relations, full parents and duties; U4 |
| ZR03, ZR06, ZR07, ZR11 | ZKIR semantics/constraints, compiler and ledger context binding | Prove source/intent/effect correspondence and mandatory-predicate enforcement; U1/U2/U4 |
| ZR04 | Native recursion soundness plus application base/preservation relation | Authenticate funded genesis and well-founded financial history; U4 |
| ZR10 | Ledger current-state/consumption interfaces plus application contracts | Head/absence checks, entitlement uniqueness and complete state; U3/U4 |
| ZR12, ZR13 | ZKIR, prover/verifier resource interfaces and ledger costing | Bind per-stage/cumulative work and reserves; U1/U3/U4 |
| ZR14 | Native private-input interface and proving tools; kernel where joint proving is selected | Privacy policy, witness transfer and availability, separate-party tests; U4 |
| ZR15 | Ledger key/state evolution interfaces plus application policy | Consent-preserving migration, duty/replay continuity; U4/U5 |
| ZR16 | Native library/tooling and actual ledger configuration | Independent retained proof and effect verification; U4/U7 |

Each delivered row must record the owner, exact implementation/source/API pin, semantic statement and assumptions, positive and hostile fixtures, actual command and runtime configuration, retained artifacts and complete results. A row is not complete because an opcode or function with a plausible name exists.

## Integration requirements that refine the list

**Authoritative context and fees (ZR03/10/11).** Publish which network/genesis identity, contract/operation, domain, time/expiry, read set, complete effects and phase fields come from the ledger and which are application commitments. Enforce their linkage. A committed prover-selected network or clock is not authoritative context. The signed maximum fee, denomination and permitted failure fee policy must be enforced at actual debit; exact fees decided later can be checked by the ledger against that cap instead of being fictitiously known during proving. Test cross-network replay, changed fee asset/recipient, phase relocation and stale reads. State which controls invalidate proof bytes and which fail ledger checks.

**Complete verification (ZR05/07/08/16).** Distinguish Certificate from ContractCallProof and define supported transcript/statement/parameter formats. If direct recursive verification of contract-call proofs is unsupported, supply a sound certificate bridge for the required statement rather than a byte/tag conversion. Outer-proof validity is not final validity if accumulators or decider checks remain. Test wrong-but-well-formed inner proofs, wrong public instances, dropped or reordered accumulators, zero guards, trailing bytes and every final verifier path with real proof verification enabled. Mandatory proof invalidity must not authorize any protected financial effect; receipts must expose any separately specified network admission charges.

**Currentness versus proof reuse (ZR04/09/10).** Proofs can remain reusable evidence while spendable resources are consumed once. Test competing valid successors, duplicate join resources, same-identity genesis, omitted private reservations, stale roots and old/new migration races. Correct recursive history is necessary but does not decide present entitlement. Authenticated nonmembership and the complete state domain must be linked to current ledger checks.

**Retained and private artifacts (ZR14/16).** Define canonical export/import of statements, keys, parameters, commitments, accumulators and proof bytes, including length/EOF checks. Reproducible compilation and verification do not require identical randomized proof bytes. Specify required successor witness material, recipients, access/prove/spend/recover rights and missing-witness behavior. Test Alice/Bob isolation, withheld/corrupt handoff, authorized joint proving and leakage under the declared observer model. Possession of a proof or viewing key must not become spending permission.

**External evidence (ZR02/03/11).** Comprehensive native recursion does not automatically verify every foreign proof system, establish foreign consensus/finality or guarantee real-world document truth. For each supported imported profile, name the claim, verification/translation relation, domain/root/epoch/freshness, finality and data-availability assumptions. The kernel must bind proof-for-X to exactly transaction-X; unsupported decoding or an unsigned evidence downgrade fails. Inclusion, application success and final settlement are distinct claims.

**Costs (ZR12/13).** Measure same-relation chains and heterogeneous joins across depth, fan-in and statement sizes, including all final checks, deferred pairings and parsing. Report circuit rows/degree, public and private input sizes, proof/key/accumulator sizes, host time/peak memory, verifier work and fee coverage on named hardware. Compact retained certificates and stable verification cost across long histories are desirable targets, not permission to omit checks. Publish the supported envelope and explicit over-bound behavior.

## Compatibility and migration choices left open

A dedicated join instruction versus repeated bounded proof verification, fixed keys versus policy-checked dynamic keys, IVC versus a bounded DAG wrapper, exact accumulator representation and safe batching remain engineering choices. Financial words such as debt, escrow and intent need not become opcodes. The backend must faithfully constrain the relation that expresses them.

Keep all historical native failures and resource limits. Full MC03 requires real recursive financial steps; MC06 requires private successor/split/join behavior. Ledger induction, external signatures, mocked proofs or aggregation of unrelated statements cannot close those obligations. Verify the released tuple before dependent proving campaigns, and requalify changed correspondence claims.

## Evidence basis and limits

The supplemental [semantic requirements review](../deliverables/consolidated-design-2026-09-19/astra-backend-requirements.md) and [ledger integration review](../deliverables/consolidated-design-2026-09-19/astra-backend-integration.md) inspected locally pinned ZKIR, ledger, native IVC and MPS-0014 sources. Pins include ZKIR `7dff84a685cd8baed2e69a2b66ae63573fe5a575`, native proof library `695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7`, and historical ledger objects `3fa0d1d15a3cfabd41c806546a006b1ce406b7b2` / `a8ab82ba2124c36f92795c683e70bd888bc1d1fb`. No fresh upstream fetch, build, proof or deployed capability verification occurred for this list. Historical transcript, guard and verification-build observations motivate tests; they are not claims of current production defects.

[PR17 applicability](../deliverables/aeon-study-2026-09-19/PR17-APPLICABILITY.md) remains conditional older-surface evidence. ZR06 requires its relevant witness-shape and concrete constraint assumptions to be discharged for the emitted supported profile.
'''
p.write_text(header+table+footer)
# Add normative backend capability summary with linkage rather than duplicating all clauses.
f=C/'openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md'
f.write_text(f.read_text()+'''### Requirement: UNI-017 Next native backend contract
When the next ZKIR/recursion interface is qualified for Moriarty, the integration SHALL satisfy ZR01–ZR16 in the backend requirements contract for its claimed scope, preserving full native recursive financial history and private bounded multi-parent composition.

#### Scenario: Complete native recursive path
- **WHEN** A compatible released tuple supports the specified financial base, two native steps and a private join with all final checks.
- **THEN** Retained independent verification and actual ledger controls qualify only the evidenced scope.

#### Scenario: Incomplete cryptographic or release evidence
- **WHEN** A proposed instruction, outer-only check, six-month forecast or different proof format is offered as completed backend support.
- **THEN** The affected requirements remain open and no weaker mechanism is relabeled as full recursive compliance.
''')
print(p)
