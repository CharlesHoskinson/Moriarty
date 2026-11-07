# astra-2 position: close the proof boundaries before scaling

Base: `8fd34ed794eda7186c32495f715609ec54a9a297`. This is an independent position. I have not read another council member's position. Recommendations below are proposed design decisions, not claims of implemented assurance.

## Keep

1. Keep Sherlock's bounded interpreter, explicit machine state and native recursion route. Keep the program, initial state, final state and work count in the accepted statement. The IVC node already binds consecutive state digests. The run decider pins the node key and checks the final pairing. These mechanisms are the right foundation. Sources: `crates/mori-recursion/src/ivc.rs:191`, `crates/mori-recursion/src/ivc.rs:295`, `crates/mori-recursion/src/run_decider.rs:206` and `crates/mori-recursion/src/run_decider.rs:247`.
2. Keep the custom ledger-9 decider as the contract operation key. Do not make `VerifyProof` or ledger 10 a prerequisite. Preserve real proof verification, balancing, signatures and limits. The transaction verifier explicitly refuses weakened strictness. Sources: `crates/mori-ledger/src/sherlock_literal.rs:1` and `crates/mori-ledger/src/sherlock_transaction.rs:483`.
3. Retain ZR01–ZR16, MNR01–MNR08 and UISEC-001–012 as distinct obligations. Preserve effects, duties, pending events, exact arithmetic, authenticated genesis and unique consumption. A pure scalar run is an implementation milestone, not the full financial language. Sources: `docs/MORIARTY-BACKEND-REQUIREMENTS.md`, `openspec/changes/sherlock-universal-interpreter-design/security-requirements.md` and its `traceability.md:99`.
4. Keep the current DoctorWatson loop. Attach security acceptance checks to concrete implementation predicates. The factory already has a mutation-test tier and exact commands for checks, heavy work and retries. Source: `models/doctorwatson/factory/CONTRACT.md`.

## Improve

### Make the composed leaf contract explicit

The current C6 digest adapter produces six public lanes, including binding. The IVC consumer expects seven, including `steps`. This is an actual interface mismatch, not an observed exploit. Sources: `crates/mori-backend/src/sherlock_c6_child.rs:817`, `crates/mori-recursion/src/ivc.rs:191` and `crates/mori-recursion/tests/sherlock_gp.rs:184`.

Adopt one versioned leaf statement and one checked producer/consumer implementation. Specify which circuit constrains each lane. Bind rule identity, program/configuration identity, canonical pre/post state, outcome, result, start index and actual segment length. Pin layout, semantic profile and bounds through the authorized relation or a binding statement commitment. Reject incompatible schemas before expensive proving.

The C6 adapter embeds externally supplied outcome constants and publishes rule/height without constraining their semantic meaning itself. The honest witness stream derives outcomes from `HostSnapshot`. That does not make host derivation an adversarial-witness guarantee. Fixed authorized keys may discharge some obligations, but the authorization and correspondence argument must say how. Sources: `crates/mori-backend/src/sherlock_c6_child.rs:560`, `crates/mori-backend/src/sherlock_c6_child.rs:598` and `crates/mori-backend/src/sherlock_c6_witness_stream.rs:138`.

### State and discharge the canonicality invariant

For each authorized leaf, require: a valid canonical pre-state plus satisfying constraints implies the intended step and a valid canonical post-state. Authenticate the initial invariant. Require the terminal decider to use the same outcome/result representation. Alternatively, enforce full pre/post canonicality directly in every leaf. Either approach needs an explicit proof boundary.

A concrete challenge is admin publication. It copies Return reference lanes into Halted. The shared pre-image checker validates roots, frames and capacities but does not visibly validate all Return payload/padding lanes. Dispatch selects this rule from control tag, empty stack, quiet helper state and Running outcome. Sources: `crates/mori-backend/src/sherlock_machine/c6_admin_publish_step.rs:88`, `c6_step_kit.rs:521` and `c6_dispatch_step.rs:1229` in the same directory.

Test a paired pre/post mutation of Return reference tag or unused control padding. This is a source-level candidate for a standalone canonicality gap. I have not demonstrated a satisfying malicious witness or an accepted full run. A valid-genesis and preservation argument may exclude it. The distinction must remain explicit.

### Bound counters in the accepted relation

The IVC currently checks nonzero `steps`, then adds it to `prev_h` as field arithmetic. Its Rust witness fields are `u64`, but that type is not an in-circuit range proof. Sources: `crates/mori-recursion/src/ivc.rs:116` and `crates/mori-recursion/src/ivc.rs:308`.

Specify bounds for both operands and the result, plus the leaf's exact length. Test zero, maximum, maximum plus one and field-wrap values through a raw witness interface. A composed lower layer may supply a bound, but its proof obligation must be linked. Do not claim a demonstrated recursive forgery from this source observation alone.

### Authenticate the complete verifier and deployment tuple

Treat a key cache as an optimization. Its identity must cover relation bytes, semantic/layout version, shape, proof/transcript version, SRS and relevant compiler/backend pins. Authenticate the installed operation key against the selected relation. Treat changes as migrations requiring state/duty preservation and applicable authority. Sources for the obligations: ZR01, ZR05, ZR15 and MNR06 in `docs/MORIARTY-BACKEND-REQUIREMENTS.md`.

Keep proof validity separate from fresh chain state. The extraction helper explicitly treats supplied state, chain association and freshness as external assumptions. Require network and chain-instance identity, authenticated state, exact call inclusion, finality and observed effects at deployment gates. Source: `crates/mori-ledger/src/sherlock_transaction.rs:850`.

## Cut

- Cut host-only validation as the sole justification for an accepted-witness property. Retain it for diagnostics and early refusal. Requirement source: ZR06 and UISEC-012.
- Cut any claim that a digest, outer proof, SDK codec round trip or successful submission establishes final correctness. The existing SDK seam deliberately stops before proof generation. Source: `docs/sherlock/midnight-sdk-seam.md`.
- Cut broad performance promises before profiling. Do not weaken a check to fit a budget. Only C0 measurement is agreed in the performance tracker. Source: `wiki-llm/recursion-perf/CONSENSUS.md`.
- Cut retired reviewer and adoption machinery from the new implementation workflow. Preserve reproducible evidence and substantive security requirements. Source: `AGENTS.md` and `models/doctorwatson/factory/CONTRACT.md`.

## Implementation plan, with ordered milestones

1. **Pin the target and the next executable failure.** Record network/chain identity and the complete compatible toolchain. Recheck the A8 receiver with that tuple. My read-only `scripts/sherlock-next` run reported A8 first, with A 7/10, B 4/9, C 5/6, U 34/35 and F 11/13. Source: `qa/sherlock/status.json`, read on 2026-11-07.
2. **Close the leaf interface and canonicality contract.** Implement the seven-lane schema, exact step count, outcome/result binding and explicit canonical-state invariant. Add small hostile witnesses that bypass the host. Keep the dispatch factorial timeout separate from the correctness tests. The historical run stopped at its wall cap without a receipt. Source: `docs/sherlock/HANDOFF-2026-10-26.md`.
3. **Qualify the complete recursive closure.** Prove leaves, node and final decider for the smallest real source program. Verify retained artifacts in a separate process. Reject wrong keys, wrong instances, omitted/invalid predecessors, accumulator substitutions and premature terminal claims. Extend to recursion after the smallest path passes. Requirement sources: ZR03–ZR08, ZR16 and UISEC-010.
4. **Exercise the target verifier.** Install the custom decider key on the chosen test network. Retain exact proof/statement/key identities, finalized call and state/effects readback. Add actual target rejection and replay cases. A8 and U023 have distinct scopes. Sources: `qa/sherlock/predicates.yaml` and `openspec/changes/sherlock-stagenet-recursive-rollup/requirements.md`.
5. **Extend the supported semantics and optimize measured costs.** Carry failure, exhaustion, effects, duties and pending-state obligations through every added rule. Preserve required financial composition as later tasks. Run the phase profiler before selecting optimizations. Sources: UISEC-006–011, ZR09–ZR15 and `openspec/changes/recursion-phase-profile/design.md`.
6. **Promote the qualified tuple to Mainnet.** Re-run compatibility, key identity and deployment checks against Mainnet's actual state. Preserve obligations during migration and rollback. A test-network receipt alone cannot establish Mainnet currentness. Requirement sources: ZR01, ZR10, ZR15 and SRPOC-007.

Use `scripts/doctorwatson check <ID>` for each registered predicate and `scripts/doctorwatson heavy` for heavy work. Add named checks for missing predicates to `qa/sherlock/predicates.yaml`. Preserve failure traces and supply the required retry hypothesis. Source: `models/doctorwatson/factory/CONTRACT.md`.

## Assurance strategy

Maintain a boundary table: semantic rule → canonical representation → emitted constraints → leaf statement → recursion/decider → ledger operation. For each boundary, record assumptions, invariant, adversarial tests, formal argument and exact evidence scope. UISEC-012 and ZR16 already require these distinctions.

Require both positive boundary cases and semantically invalid witnesses. Use coordinated mutations, including changes to both copied endpoints or a key and its matching proof. Also challenge inactive values, continuation domains and accumulator encodings. Single-cell rejection cannot establish all these properties. The existing T1 harness covers selected circuits and maintains explicit exceptions. Source: `crates/mori-backend/tests/soundness_mutation.rs:1`.

For recursion, require an inductive statement with an authenticated base, authorized predecessor relation, exact continuity, bounded work and terminal closure. Test the final decider with a well-formed invalid inner proof, wrong instance and mismatched carried accumulator. Keep subgroup, infinity, transcript and SRS assumptions in the cryptographic argument. Sources: ZR04–ZR08 and `crates/mori-recursion/src/run_decider.rs:206`.

K and Quint evidence support their stated semantics and finite models. They do not prove arbitrary-witness circuit correspondence, hash binding or ledger finality. Source: `openspec/changes/sherlock-universal-interpreter-design/security-requirements.md`, evidence-class table.

## Midnight ledger 9 impact

The current upstream guide lists Mainnet and Preview on ledger 9.1.0.1 with node 2.1.101. Preprod remains ledger 8. It describes ZKIR v3, new unshielded contract calls, changed fee calculation and a Preview chain reset. Old Preview receipts cannot establish current deployment identity. Recompilation changes verifier keys. Preserve old artifacts or use an authorized maintenance update/redeployment. Source: <https://docs.midnight.network/relnotes/ledger-9>.

Mainnet's supported stack includes Compact 0.36.0 and proof server 9.0.0. Sherlock's Cargo dependencies instead pin ledger source `04c9c5d9...`, proofs 0.8.3 and circuits 7.2.4. Compatibility must be demonstrated for the custom decider. The hard fork alone establishes no new recursion theorem. Sources: <https://docs.midnight.network/getting-started/installation> and `Cargo.toml:28`.

Require a proof-to-execution representation regression at the ledger boundary. The upstream field-alias advisory describes equal proof representations with different VM behavior, and lists 8.1.3 as patched. I do not claim the current Sherlock pin remains vulnerable. Verify the exact target's fix lineage and reject noncanonical aliases. Source: <https://github.com/midnightntwrk/midnight-ledger/security/advisories/GHSA-wr7g-rr4v-jmj8>.

The deployment path should identify the actual stagenet/Preview chain rather than equate their names. Re-estimate DUST/fees against current parameters. Confirm contract-call encoding, installed VK and real verification on that target before Mainnet promotion. Source: <https://docs.midnight.network/relnotes/ledger-9>. The scout's refreshed source pins remain an input to the next phase.

## Open questions

- Which concrete producer will supply the seven-lane GP leaf, and where will each lane's semantics be enforced?
- Which canonicality premises are checked per leaf, and which are carried by an authenticated induction invariant?
- What exact upstream commit and node configuration are deployed on the chosen test chain and Mainnet?
- Is the current custom decider's transcript template compatible with the released ledger 9 tuple?
- Which security predicates already run against retained proofs, and which remain planned?

These questions follow the interface, canonicality and target gaps cited above. None is permission to omit a mandatory check.

## Confidence, with reasons

High confidence in retaining the native recursion architecture, final decider and explicit trust boundaries. Their source mechanisms and existing requirements agree. High confidence in the six-versus-seven lane mismatch from both producer and consumer code. Moderate confidence in the proposed publication counterexample until a raw witness check runs. No claim of complete soundness audit, exploit reproduction, target compatibility or Mainnet readiness. This review ran source reads, graph navigation, web verification and `scripts/sherlock-next`, with no proof builds or deployments.
