# Host and C6 constraint evidence

Repository: `/home/hoskinson/Projects/Moriarty`  
Revision: `8fd34ed794eda7186c32495f715609ec54a9a297`

The composed publication fragment has a concrete gap between its constraints and the host decoder's accepted representation. A complete-run exploit is not established. Authenticated genesis and preservation of the state invariant could exclude the malformed pre-states described below.

This is a static source review. No builds, tests, proofs or hostile witnesses were executed. Key reviewed files matched HEAD. No other review positions were read. Language-server tools were unavailable, so navigation used the existing graph and targeted source reads.

## H1. Publication does not constrain the Return reference or unused control lanes

**Facts.** `KAdminPublishValue` constrains the post outcome tag to Halted. It copies the pre Return reference into the post outcome and preserves every other image lane. It composes dispatch and the shared canonicalization blocks. See [publication body](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_admin_publish_step.rs:88).

The dispatch publication predicate checks Running, absent helper/result, Return control and zero frame depth. It does not inspect the Return reference. See [P1 predicate](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:1229).

The shared pre-image validator checks roots, capacities, active counts and frames. It does not validate the control union or outcome payload. The composed canonicalization covers META, PROG, CTX, DATA and HELPER. See [pre-image validation](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_step_kit.rs:521) and [composition](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_step_kit.rs:675).

The host codec accepts reference tags 0 and 1 only. Return uses control offsets 0–2 and requires offsets 3–16 to be zero. Its checked decoder also bounds the reference index. See [reference decoding](/home/hoskinson/Projects/Moriarty/crates/mori-core/src/sherlock/helper_machine/c6_phys.rs:347), [Return decoding](/home/hoskinson/Projects/Moriarty/crates/mori-core/src/sherlock/helper_machine/c6_phys.rs:996), [padding and checked decoding](/home/hoskinson/Projects/Moriarty/crates/mori-core/src/sherlock/helper_machine/c6_phys.rs:1142) and [reference capacity](/home/hoskinson/Projects/Moriarty/crates/mori-core/src/sherlock/helper_machine/c6_phys.rs:588).

**Proposed attacks, not executed.** Start with a genuine enabled publication witness.

- Set pre and post `IMAGE[L_CONTROL + 1]` to 2. Set post `IMAGE[L_OUTCOME + 1]` to 2.
- Separately, set pre and post `IMAGE[L_CONTROL + 3]` to 1.
- Separately, replace the pre/post Return index and copied Halted index with an index beyond the selected table capacity.

The first mutation has no `RefC` interpretation. The second violates union padding. The third violates the checked codec. The reviewed constraints do not reject these paired mutations. Here `L_CONTROL = 2` and `L_OUTCOME = 23`, so the first mutation touches absolute image lanes 3 and 24, and the second touches lane 5. See [offsets](/home/hoskinson/Projects/Moriarty/crates/mori-core/src/sherlock/helper_machine/c6_phys.rs:58).

Existing publication tests mutate individual post lanes or change selection tags. Those tests do not cover these coordinated pre/post mutations. See [post mutations](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_admin_publish_tests.rs:92) and [pre-selection tests](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_admin_publish_tests.rs:183).

**Limit and required guarantee.** This is a standalone correspondence gap. Valid genesis plus invariant-preserving predecessors would exclude these inputs. The kit writes all control lanes when changing control, and the reviewed legacy fragment starts from a zeroed control record. See [complete control write](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_step_kit.rs:627) and [legacy initialization](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_legacy_control_step.rs:129).

The contract must establish that induction for every authorized relation, or constrain complete IMAGE well-formedness. For Return, require a boolean reference kind, an index below the selected active table length and zero unused lanes. Require canonical outcome payloads as well.

## H2. Structural canonicalization does not establish the state invariant

**Facts.** DATA reference validation checks the selected capacity, not active membership. Constructor rows check type/constructor capacity and slice capacity. They do not check declared arity, field sorts or child-before-parent ordering. See [reference checks](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_data_canonical.rs:298) and [constructor checks](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_data_canonical.rs:389).

The module explicitly limits its claim to structural encoding. HELPER has the same limitation. PROG slices receive no table-end or arity check from the canonical parser. See [DATA scope](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_data_canonical.rs:3), [HELPER scope](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_helper_canonical.rs:3) and [PROG scope](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_program_context_canonical.rs:1).

The existing security contract requires active-length reference bounds, typed rows and child-before-parent ordering. It separately identifies genesis establishment and step preservation as unproved obligations. See [A4](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-universal-interpreter-design/security-candidate-b.md:64).

**Proposed attacks, not executed.** Preserve both sides of a publication transition while changing an existing data-field reference to its parent or an inactive slot below capacity. Separately, change an active constructor's field count to a different count within capacity. The structural checks alone admit these patterns. The complete composed relation and any authenticated predecessor must be tested separately.

**Required guarantee.** Treat canonical encoding, semantic admission and invariant preservation as separate predicates. A verifier needs an authenticated initial state and authorized step relations with preservation evidence. A hash of arbitrary canonical bytes does not establish these predicates.

## H3. Range checks and dispatch precedence are substantive constraints

**Facts.** The kit constrains enable to boolean, fixes schema/version, ranges every carrier lane to 128 bits and equates pre/post META, PROG and CTX. These checks precede rule effects. Disabled steps require complete state identity. See [prefix constraints](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_step_kit.rs:466) and [disabled identity](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_step_kit.rs:739).

Dispatch requires Running. Publication precedes fuel exhaustion. Ordinary rules require fuel and negate earlier guards in their selected group. See [dispatch path](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:1229). The ordering matches the host driver at [owner P0–P2](/home/hoskinson/Projects/Moriarty/crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:3701).

The canonical DATA emitter zeroes inactive slots. It uses 129-bit slice-end comparisons because two 128-bit lanes can sum beyond 128 bits. See [arithmetic rationale](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_data_canonical.rs:19) and [inactive partitions](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_data_canonical.rs:512).

**Evidence limits.** The META tests include raw field values at `2^128` and nonboolean gates. The typed `C6StepWitness` exposes only `u128` lanes and a Rust boolean. Mutation tests through that type cannot exercise arbitrary field witnesses. See [raw field tests](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_meta_canonical_bound_tests.rs:771) and [typed witness](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_step_harness.rs:33).

Dispatch tests sample factorial and fold runs. They test every supported alternative every sixteenth state, with selected alternatives elsewhere. This supports sampled precedence, not universal semantic correspondence. See [selection tests](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_dispatch_tests.rs:89).

**Proposed attacks.** Inject enable 2, a lane at `2^128`, a lane at the field modulus minus one and slice-end overflow through raw field inputs. Expect rejection where range checks apply. Test disabled state changes and incorrect publication/fuel precedence independently.

## H4. The digest outcome is supplied separately from the constrained state

**Facts.** The honest stream derives its outcome by reading the captured host state. Halted references must resolve to UInt constants or data values. See [outcome extraction](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_c6_witness_stream.rs:138).

The generic child adapter instead takes `pre_outcome` and `post_outcome` arguments. It pins those arguments as circuit constants and hashes them with carrier digests. The adapter does not derive those values from IMAGE or dereference the result. See [adapter inputs](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_c6_child.rs:443) and [outcome hashing](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_c6_child.rs:551).

Verification reconstructs the key and instance using outcome arguments supplied to the verification API. This protects against a mismatched supplied key. It does not independently validate the arguments' semantic correspondence. See [verification boundary](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_c6_child.rs:998).

**Proposed attack, not executed.** Keep a genuine publication carrier returning 6. Build and verify the digest relation with `StateOutcome::halted(7)`. Separately, label a still-Running post-state as Halted. Determine whether the caller's authorized relation identity or independent outcome derivation rejects the altered relation before acceptance.

**Required guarantee and limit.** Outcome and result must follow from constrained carrier lanes or a documented trusted derivation used by verification. Merely receiving them from the prover is insufficient. This review does not establish whether the outer verifier authorizes an altered relation. UISEC-010 requires summary/data agreement and authorized relation identity at [the decider boundary](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-universal-interpreter-design/security-requirements.md:79).

## H5. The witness stream proves neither completion nor error handling

**Facts.** The stream returns a final run record only after a Halted UInt. Rejected, Exhausted, Waiting and non-UInt terminal outcomes return errors. Post-outcome extraction occurs before sending the step to the sink. Therefore an error can leave previously emitted Running records without emitting the terminal failure event. See [accepted outcomes](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_c6_witness_stream.rs:138), [partial-stream contract](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_c6_witness_stream.rs:161) and [loop order](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_c6_witness_stream.rs:198).

The CLI emits step JSON and returns failure through process status. It does not serialize the returned final `C6WitnessRun`. See [CLI](/home/hoskinson/Projects/Moriarty/crates/mori-backend/examples/c6_witness_stream.rs:5).

The common circuit checker calls SDK `ProofPreimage::check`. It does not prove or verify a cryptographic proof. See [checker](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_step_harness.rs:250). The security requirements distinguish correspondence, native proof/decider acceptance and ledger currentness at [evidence classes](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-universal-interpreter-design/security-requirements.md:14).

**Proposed attacks.** Truncate the JSON stream before publication. Ignore a producer failure after several records. Trigger zero fuel, arithmetic rejection or a non-UInt return. Consumers must reject incomplete success claims and preserve the distinction between resource exhaustion and semantic rejection.

**Required guarantee.** Final acceptance must authenticate genesis, enforce digest continuity and relation authorization, derive terminal success/result and run the decider. Host generation, continuity checks and SDK preprocessing establish none of those obligations by themselves.

