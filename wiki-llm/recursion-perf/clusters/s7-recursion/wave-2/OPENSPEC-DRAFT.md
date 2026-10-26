# Draft OpenSpec change: add-sherlock-direct-fixed-pair-terminal

**ESTIMATE — Status: specified only.** This is E1's proposed implementation package after C0. It is not an adopted change or acceptance receipt. All proposed behavior and SHALL requirements below are **ESTIMATE — normative design requirements**, not claims about implemented behavior. Source facts are separately labeled. Unresolved implementation facts are labeled OPEN QUESTION.

**ESTIMATE — Future package layout:** split this draft into `openspec/changes/add-sherlock-direct-fixed-pair-terminal/proposal.md`, `tasks.md`, and one delta at `specs/sherlock-direct-terminal/spec.md`. No files in that package are created by this research wave.

## Proposal

### Why

**SOURCE —** The recorded fixed children use k13 and k12. The current recursive design prepares both proofs in P, then verifies P and decides its accumulated KZG obligation in D. A3 recorded check intervals are 315.880–382.278 seconds; those intervals include modeling/key work and verification. Exact child row counts are not recorded. Sources: [local audit](../../../raw/s7-recursion/wave-2/local-c1-audit.md), [prior Astra-2](../../../raw/s7-recursion/astra2-prior.md).

**ESTIMATE —** For the bounded available-witness pair, directly constraining the transitions can remove both recursive verification layers. This is distinct from fusing P with D while retaining child proofs. C1 has no in-circuit pairing obligation. Ledger verification still uses the ordinary outer PLONK/KZG proof, BLS12-381, Blake2b and official SRS. The provisional advice budget is 20,466 rows; k15 and k16 require measured fit. The wave-1 h thresholds are 7.0968% and 4%, respectively. These are sizing screens, not guaranteed speedups. Sources and arithmetic: [FINDINGS.md](FINDINGS.md), [model-results.json](model-results.json).

### What changes

**ESTIMATE —** Add one opt-in `FusedSherlockTerminal` for the pinned A102 Eval(Literal)→Return followed by A80 Return→StepDone relation. Refactor the existing fragment emitters to accept shared private operands. Preserve the standalone child paths for differential checking. Constrain the complete semantic statement and compiled `acceptValidity` call opening in one stock relation. Emit one Blake2b outer proof. Do not construct child proofs, P, D, or an accumulator on the candidate path.

**ESTIMATE —** Add a bounded `sherlock-fused-e1` comparison harness. Its commands are `census`, `equivalence`, `qualify`, and `benchmark`. Common options are `--manifest PATH` and `--output NEW_DIRECTORY`. The census accepts `--k 15,16`. Qualification accepts `--geometry PATH`. Benchmarking accepts `--qualification PATH --pairs 5 --warmups 1 --order alternate --backend cpu`. An in-memory SDK adapter supplies fresh call openings; an explicit `--sdk-opening-stdin` mode supports its pipe. Logs and manifests contain no private openings or witnesses.

**ESTIMATE —** Introduce a versioned deployment manifest with explicit `recursive-parent` and `direct-fixed-pair` variants. The existing fixture remains recursive until independent readback validates a candidate deployment. Implement exact-key rejection for both variants. The direct variant carries semantic-source identities instead of pretending to have a parent proof key.

**ESTIMATE —** Preserve the `Verified` and `Consumed` observability points with typed direct-proof evidence. The direct variant attests transition satisfaction, not old child-proof production. The adapter hands a fully checked, accumulator-free statement to settlement. It does not invent child proof receipts or mark submission/finality complete.

### Impact

**ESTIMATE — Affected implementation paths:**

| Path | Proposed responsibility |
|---|---|
| `crates/mori-backend/src/sherlock_machine/literal_bridge.rs` | Reusable private assignment wrapper around the original A102 fragment. |
| `crates/mori-backend/src/sherlock_link.rs` | Equivalent wrapper around A80; preserve existing public emitter behavior. |
| `crates/mori-recursion/src/fused_sherlock.rs` (new) | Direct relation, witness, typed statement, serialization and stock architecture. |
| `crates/mori-recursion/src/lib.rs` | Export the opt-in direct relation and typed result. |
| `crates/mori-recursion/src/bin/sherlock-fused-e1.rs` (new) | Geometry, equivalence, ledger qualification and matched timing harness. |
| `crates/mori-recursion/tests/sherlock_fused.rs` (new) | Constraint-level, reference-equivalence and reader tests. |
| `crates/mori-recursion/tests/sherlock_terminal.rs` | Explicit family-aware fixture decoding and installed-key checks. |
| `crates/mori-recursion/tests/fixtures/stagenet-verifier.json` | Re-pin only after new deployment/readback; retain legacy information in versioned fixtures. |
| `qa/sherlock/e1/manifest.json` (new) | Public C0/input/source/key/SRS/template provenance for comparisons. |
| `target/s7-e1/{geometry,equivalence,qualification,timing-r4}/` | Fresh public experimental artifacts; no private witness persistence. |
| `scripts/qa/check-sherlock-fused.sh` (new), `qa/sherlock/predicates.yaml` | Run the implemented checks through the factory; add measured budgets. |

**ESTIMATE — Operational impact:** generate a new VK/PK family, invalidate incompatible caches, and register a new stock verifier. Budget a new deployment and independent readback. Update caller routing and the strict fixture schema together. Preserve existing wallets, keys, contract state and legacy A4 checks. A fresh address requires an explicit initial root/height/instance/replay policy. Keep deployment fees, synchronization, fee proofs and finality outside steady-state proving claims. Estimated fixed-fixture engineering is 2–4 weeks plus 2–5 days of deployment-adapter qualification, excluding C0 and unresolved state migration.

**SOURCE —** A4 checks parent VK, relation hash and installed verifier bytes separately. The fixture currently requires all four of address, VK hash, parent VK hash and relation hash. Wrong-template and wrong-parent hypotheses occur in historical records. The latest failed A4 trace is not available in this evidence set, so its cause remains unknown. [Local audit](../../../raw/s7-recursion/wave-2/local-c1-audit.md).

**SOURCE —** Pinned stdlib 2.3.5 reconstructs the stock constraint system from architecture and k. It passes the chosen transcript hash to host proving/verification. The official catalog matches the existing k20 SRS hash. Pinned proofs 0.8.3 can downsize parameters by truncating powers and recomputing Lagrange bases. [Stdlib source](../../../raw/s7-recursion/wave-2/stdlib-2.3.5-public.md), [parameter source](../../../raw/s7-recursion/wave-2/params-0.8.3-source.md), [catalog](../../../raw/s7-recursion/wave-2/official-srs-catalog.md).

**ESTIMATE — Non-goals:** general 39-rule interpreter completion, arbitrary proof aggregation, 16-transaction tree redesign, Nova/folding integration, new curve/PCS/transcript, custom ledger gates, GPU implementation, retaining the installed recursive VK, proof-of-possession of old child proofs, or a stronger source-provenance statement. Do not assume independent wallet secrets become available. No new trusted-setup ceremony is proposed.

**OPEN QUESTION —** Actual row/table counts, candidate width/degree, host Blake2b share, warm complete-chain speed, live migration API and DUST/finality costs remain to be measured. k16 is the only geometry fallback in this package. Any wider relation or migration protocol requires a separate scope decision.

## Tasks

**ESTIMATE — All tasks are unimplemented.** Use [EXPERIMENTS.md](EXPERIMENTS.md) for exact command lines and output contracts. Procedural tasks below refer to the future implementation checkout.

- [ ] 1. Validate C0 receipt hashes and reproduce its same-fixture baseline. Write `qa/sherlock/e1/manifest.json` without private material.
- [ ] 2. Add independent reference and mutation tests in `crates/mori-recursion/tests/sherlock_fused.rs` before changing behavioral code.
- [ ] 3. Refactor both fragment wrappers for private shared operands. Preserve standalone child emitter behavior and keys where unchanged.
- [ ] 4. Implement `fused_sherlock.rs`. Constrain both transitions, ordered linkage, all statement fields and the bound call template.
- [ ] 5. Implement the four harness modes. Make stale manifests and output-directory collisions fail before heavy work.
- [ ] 6. Run E1 `census` at k15 and k16. Write `target/s7-e1/geometry/{k15,k16,selected}.json`.
- [ ] 7. Run E2 `equivalence`. Write `target/s7-e1/equivalence/result.json` with constraint-level mutation outcomes.
- [ ] 8. Run E2 `qualify` with authenticated official parameters. Write public verifier/proof artifacts and `qualification/result.json`.
- [ ] 9. Implement typed `Verified` and `Consumed` events. Distinguish direct semantic coverage from recursive proof consumption.
- [ ] 10. Add versioned deployment-family fixtures and mismatch tests. Preserve the old A4 exact-key requirements.
- [ ] 11. Run E3's five alternating matched pairs. Write `target/s7-e1/timing-r4/results.json` and phase traces.
- [ ] 12. Add `scripts/qa/check-sherlock-fused.sh` and its factory predicate. Register measured runtime and memory budgets.
- [ ] 13. Run formatting, workspace clippy and workspace tests through required factory controls. Run the new predicate and stop-rule.
- [ ] 14. Prepare the public deployment bundle and exact initial-state/replay policy. Qualify the pinned ledger's registration path.
- [ ] 14a. Run the charter's T3 cold review through `codex exec -s read-only`. Apply its one-hour no-answer rule.
- [ ] 15. Register on stagenet through the existing deployment workflow. Preserve the old contract and wallet state.
- [ ] 16. Read back the verifier and anchor state independently. Re-pin the fixture and routing from that receipt.
- [ ] 17. Run the family-specific A4-equivalent check and one settlement/readback. Record deployment and finality costs separately.

**ESTIMATE — Required Rust verification commands:** use `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --locked -- -D warnings`; `cargo test --workspace --locked`. Run heavy checks through `scripts/doctorwatson heavy`. Execute the new registered predicate with `scripts/doctorwatson check <assigned-ID>`. Follow `scripts/doctorwatson stop-rule`. Keep holdout fixtures evaluator-owned. This draft does not run those commands or assert they pass.

## Spec delta — `specs/sherlock-direct-terminal/spec.md`

**ESTIMATE — Normative draft:** this is the package's single proposed spec delta. Every SHALL below is a proposed acceptance condition.

## ADDED Requirements

### Requirement: C0-pinned bounded relation

The implementation SHALL consume a validated C0 manifest before qualification or timing. The manifest SHALL pin the exact semantic fixture, capacities, source identities, dependency graph, SRS, template and baseline keys. Candidate and baseline SHALL use the same complete logical input. The selected relation SHALL remain the bounded A102→A80 pair. The implementation SHALL distinguish historical k13/k12 evidence from newly measured geometry.

#### Scenario: Comparable baseline and candidate

- **GIVEN** a successful C0 receipt and matching public provenance manifest,
- **WHEN** the harness constructs both paths,
- **THEN** it SHALL compare identical logical inputs and record separate cold and warm timing boundaries.

#### Scenario: Stale or different fixture

- **GIVEN** changed capacities, template, dependencies or semantic-source pins,
- **WHEN** the supplied manifest describes the old values,
- **THEN** the harness SHALL refuse the comparison before key generation and report the mismatched identity.

### Requirement: Direct constrained transitions and exact ordered statement

The circuit SHALL enforce the original A102 and A80 transition equations. Both enables SHALL equal one. Both context pairs and envelopes `(3,1)` then `(0,0)` SHALL be pinned. Both transitions SHALL share the same program cells. A102 post-state SHALL equal A80 pre-state. The circuit SHALL constrain `h_mid=h_pre+1` and `h_post=h_pre+2`, with non-wrapping 64-bit heights. It SHALL constrain `n_steps=2` and `work=1`.

The circuit SHALL reproduce these 16 fields in their existing order: `keys_id`, `a102_ctx_lo`, `a102_ctx_hi`, `a80_ctx_lo`, `a80_ctx_hi`, `program_c`, `pre_root`, `h_pre`, `mid_root`, `post_root`, `h_post`, `bind_a102`, `bind_a80`, `n_steps`, `work`, `trace_c`. It SHALL preserve existing Poseidon domains and ordered trace inputs. The fixed semantic identity SHALL include the original child-key/context/shape pins. It SHALL NOT assert that old child proofs exist.

The two binding lanes SHALL retain their existing committed-value semantics. The implementation SHALL NOT claim in-circuit SHA-256 provenance authentication absent from the original relation. Host validation SHALL NOT substitute for a required circuit equation.

#### Scenario: Honest direct composition

- **GIVEN** valid complete transition images for the pinned pair,
- **WHEN** original fragments and the direct circuit are evaluated,
- **THEN** both SHALL satisfy their equations and produce equal semantic fields and ordered hashes.

#### Scenario: Broken transition or link

- **GIVEN** a disabled enable, invalid transition equation, wrong envelope, mismatched program/middle state, or overflowing height,
- **WHEN** the witness reaches the circuit below host validation,
- **THEN** the constraints SHALL reject it.

#### Scenario: Changed committed provenance lane

- **GIVEN** a modified binding lane with the original public statement fixed,
- **WHEN** the direct circuit recomputes the ordered hashes,
- **THEN** verification SHALL reject the mismatched statement.
- **AND** the receipt SHALL NOT interpret this test as SHA-256 source authentication.

### Requirement: Bound call and ledger-9 outer proof

The circuit SHALL compute `stmt=Poseidon(SHK_STMT, instance_id, batch_no, fields[0..16])`. It SHALL constrain `communication_commitment=Poseidon(comm_rand, stmt)`. It SHALL expose the exact compiled 15-slot template, including binding input, commitment, statement and constants. Private openings SHALL remain in memory and SHALL NOT enter artifacts or logs.

The prover SHALL use stock ledger-compatible PLONK/KZG over BLS12-381, Blake2b and authenticated official SRS parameters. The direct relation SHALL contain no child-proof verifier, carried accumulator or in-circuit pairing gadget. The outer ledger verification SHALL remain mandatory. Qualification SHALL use the pinned ledger-9 reader and embedded verifier parameters. Unsupported custom architecture SHALL be rejected.

#### Scenario: SDK-bound honest proof

- **GIVEN** a valid SDK call vector and its private opening,
- **WHEN** the direct proof is generated under its new key,
- **THEN** the exact tagged-key round-trip and stock ledger verification SHALL succeed.

#### Scenario: Changed call or proof

- **GIVEN** the honest proof,
- **WHEN** any public slot, vector length, proof byte, expected key or transcript is changed,
- **THEN** the applicable decoder or verifier SHALL reject it.

#### Scenario: Unbound communication opening

- **GIVEN** an otherwise valid transition witness and public call,
- **WHEN** its private communication opening does not match the public commitment,
- **THEN** the circuit SHALL reject it.

### Requirement: Explicit Verified and Consumed evidence

The direct-family result SHALL distinguish transition satisfaction from recursive proof consumption. `Verified` SHALL require independent final-proof verification against the exact key and public statement. `Consumed` SHALL record exactly one ordered A102→A80 coverage within that aggregate. The result SHALL contain no pending accumulator obligation and SHALL NOT fabricate child-proof identifiers.

The settlement adapter SHALL preserve anchor checks for predecessor root, height, instance, program/context and replay state. `Consumed` SHALL NOT imply submission, finality or a ledger spend. The adapter SHALL expose these later states separately.

#### Scenario: Direct-family handoff

- **GIVEN** an independently verified direct proof,
- **WHEN** it enters the settlement adapter,
- **THEN** the adapter SHALL recognize accumulator-free direct evidence and retain all anchor checks.

#### Scenario: Duplicate coverage or replay

- **GIVEN** a duplicate transition in one aggregate or a previously settled statement,
- **WHEN** aggregation or settlement is attempted,
- **THEN** the applicable relation or anchor SHALL reject it without marking a new finalized result.

### Requirement: New verifier family and deployment readback

The direct relation SHALL use a new VK/PK and distinct cache identity. The deployment schema SHALL distinguish `recursive-parent` and `direct-fixed-pair`. Legacy recursive records SHALL retain parent-key checks. Direct records SHALL require source-semantic, relation, template, SRS and verifier identities. Unknown fields and mixed-family records SHALL be rejected.

Offline qualification SHALL NOT establish installed-key acceptance. The stagenet fixture SHALL be re-pinned only after independent operation-key and anchor-state readback. The registered verifier bytes SHALL equal the candidate bytes exactly. Caller routing SHALL use the corresponding deployment identity. Existing wallet, key and contract state SHALL be preserved. A fresh deployment SHALL explicitly define its initial state and replay namespace.

#### Scenario: A4-class mismatch

- **GIVEN** a candidate proof family and stale parent, relation, verifier, template or deployment pins,
- **WHEN** preparation or installed-key qualification runs,
- **THEN** it SHALL reject the mismatch before expensive work where the mismatch is observable.
- **AND** it SHALL NOT substitute a locally generated key for installed-key equality.

#### Scenario: Re-pinned deployment

- **GIVEN** successful offline qualification and a new registration receipt,
- **WHEN** independent readback matches the candidate verifier and declared initial state,
- **THEN** the deployment fixture MAY select the new family.
- **AND** a valid settlement and independent effects readback SHALL precede any claim of stagenet acceptance.

### Requirement: Geometry, performance evidence and stop conditions

The harness SHALL measure k15 and k16 explicitly. It SHALL report all relevant row extents, unusable rows, columns, degree and quotient extension. A selected domain SHALL fit the maximum occupied extent plus unusable rows. The harness SHALL NOT treat 20,466 as measured rows or the historical pairing floor as a C1 requirement.

The benchmark SHALL run five alternating matched pairs after warm-up. Complete intervals SHALL include witness production, all necessary proofs, packaging and independent ledger verification. Cold setup SHALL be reported separately. Every proof SHALL use fresh randomness and pass independent verification. Results SHALL include individual times, medians, CPU/RSS, phase attribution, provenance and failures. Stagenet synchronization, fees and finality SHALL be reported separately.

A tenfold local claim SHALL require `median(T_C1) ≤ median(T_baseline)/10`. The h thresholds SHALL remain conditional sizing screens. A row fit alone SHALL NOT satisfy the performance requirement. Missing semantic checks, stock-reader incompatibility, or failure to fit k16 SHALL stop promotion with a trace and hypothesis. A smaller valid speedup SHALL be reported as smaller; the implementation SHALL NOT remove obligations to reach the target.

#### Scenario: Fit without tenfold gain

- **GIVEN** a sound stock-verifiable k16 candidate,
- **WHEN** complete matched timing improves by less than tenfold,
- **THEN** the report SHALL retain its measured gain and mark the tenfold criterion unmet.

#### Scenario: Geometry or correctness failure

- **GIVEN** k16 overflow, an accepted invalid transition, or a ledger-reader mismatch,
- **WHEN** qualification runs,
- **THEN** promotion SHALL stop with the failing trace and best hypothesis.
- **AND** it SHALL NOT silently widen the domain or alter the ledger protocol.
