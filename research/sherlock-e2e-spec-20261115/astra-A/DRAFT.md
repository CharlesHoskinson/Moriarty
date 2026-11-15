# Slice A: language, execution and integration

Independent draft. Evidence read on 2026-11-15. No command built or tested the software.

## Evidence and scope

This draft covers the language boundary, execution meaning, kit interfaces and integration acceptance. Slice B owns proof construction. Slice C owns wrapping, settlement and operations.

`built` means an implementation and a named check exist in the inspected files. It does not mean that this draft ran that check. `partly built` means an implementation exists but a required connection or acceptance result is absent. `design only` means the inspected sources provide a design without its implementation.

Source aliases below expand to exact paths. A citation such as `MYC/compiler/src/lower.rs:187` means that file at the stated line. These working trees can change. Historical results retain their source's date and commit. The supplied OpenVM baseline is `dcf1b4c`. No git command checked that baseline.

| Alias | Path |
|---|---|
| ROOT | `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec` |
| LOOP | `/home/hoskinson/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md` |
| CHECK | `/home/hoskinson/.cache/lead/CHECKLIST-20261115.md` |
| AUDIT | `/home/hoskinson/Projects/Moriarty-wt/spec-audit/research/sherlock-spec-audit-20261115/AUDIT.md` |
| COMPLETE | `/home/hoskinson/Projects/Moriarty-wt/spec-audit/openspec/changes/sherlock-completion-20261115` |
| FINAL | `ROOT/openspec/changes/sherlock-processor-final-20261111` |
| MOD | `ROOT/openspec/changes/sherlock-modular-leaf-20261111` |
| UI | `ROOT/openspec/changes/sherlock-universal-interpreter-design` |
| DECISIONS | `ROOT/research/sherlock-final-design-20261111/DECISIONS.md` |
| MYC | `/home/hoskinson/Projects/Moriarty-wt/mycroft-factory/mycroft` |
| OVM | `/home/hoskinson/.cache/m7-wrap/src/openvm-rekey-chips` |
| RSL | `OVM/crates/sherlock-leaf` |
| SECP | `/home/hoskinson/.cache/rekey-chips/SECP-INTERFACE.md` |
| REKEY | `/home/hoskinson/.cache/rekey-t11/LIST.md` |
| T23 | `/home/hoskinson/Projects/Moriarty-wt/t23-suite` |
| ARGS | `/home/hoskinson/Projects/Moriarty-wt/t23-args` |
| MAP | `ARGS/research/sherlock-t23-args-20261115/MAP.md` |
| TERMS | `/home/hoskinson/Projects/Moriarty-wt/pipeline-anim/research/sherlock-final-design-20261111/TERMINOLOGY.md` |
| ARCH-PROFILE | `/home/hoskinson/Projects/software-factory/skills/specs/spec-architect/references/architecture-decisions.md` |

`TERMINOLOGY.md` was NOT FOUND under `/home/hoskinson/Projects/Moriarty/research`, including hidden and ignored files. The wiki points to TERMS. See `/home/hoskinson/Projects/Moriarty/llm-wiki/wiki/sources/sherlock-terminology.md:8`.

Use the category name “universal, elastic proving co-processor for Midnight.” A machine defines execution. A chip proves execution. A claim is the accepted statement. See TERMS:9, TERMS:29, TERMS:40.

Two statement forms must remain distinct:

- The R_SL execution result is `comm` followed by native effect words `W`. The code produces this order at `RSL/src/rsl/compile.rs:889`.
- The ledger call vector belongs to the entry being called. For T23, that entry is `SherlockLedger.acceptRun`. Its vector is not the original Compact entry's vector. See MAP:23, MAP:43, MAP:45.

The hard rules remain binding: every circuit has `k <= 21`, zero knowledge holds end to end, and seeded proofs are test-only. No helper receives a plain witness. Security is at least 100 bits. One universal key set serves all programs. The ledger 9.1.0.1 verifier stays unchanged. The wrap VK is at most 50,000 bytes. There is one BLS12-381 wrap. Leaves and nodes are small-field STARKs. GP-1 through GP-4 remain intact. Midnight writes are batched, with one root per checkpoint. Only case A is in scope. Case B waits for Veritee. `K_MAX = 2`, MERGE-WORK is 5,400 columns, and the `x_j` lock form is canonical. Setup mode replaces F-3. Design parameters and key handling can change before the prototype completes. Sources: `ROOT/research/sherlock-e2e-spec-20261115/BRIEF.md`, “Hard rules”, LOOP:101, LOOP:102, LOOP:105, LOOP:106.

## 1. Component inventory

| Component | Location and evidence | State | LOOP row |
|---|---|---|---|
| Compact source admission and native reference | Stage 1 consumes stock ZKIR 3.1 and `contract-info.json`: `MYC/spec/specs/frontend/spec.md:11`. Native suite results: LOOP:40. | partly built as a complete user tool | T18, T23 |
| ZKIR reader and typed expansion | `MYC/compiler/src/zkir.rs:118`, `:175`, `:559`. Secp inputs expand into raw tape words and `SecpDecode` at `:568`. | built for inspected forms | T18, T24 |
| GIR to R_SL lowering | `Program`, `Abi`, `Refusal`: `MYC/compiler/src/lower.rs:40`, `:47`, `:64`. `compile` at `:187`. | built | T18 |
| Static ABI and effect limit | `tape_len`, `slots`, `l_e`, `n_e`, `kits`: `MYC/compiler/src/lower.rs:47`. Compile-time effect limit: `:200`. | built for ordinary calls | T18, T20 |
| Tape builder and reference execution | `MYC/compiler/src/sem.rs:592`, `:655`, `:716`. Tape order is salt, selector, rand, arguments, layout. | built for inspected forms | T18 |
| Guarded effects and communication commitment | Tests `impact_words_and_false_guard`, `comm_over_args_and_outputs`: `MYC/compiler/src/tests_lowering.rs:148`, `:169`. | built at host level | T18, T23 |
| Typed ledger values, ADTs, coins and cross-contract effects | Existing obligations: `MYC/spec/specs/values/spec.md:11`, `ledger-ops/spec.md:11`, `builtins/spec.md:97`, `lowering/spec.md:116`. Suite comparison: `MYC/compiler/SUITE-COVERAGE.md:3`. | partly built and partly checked by native transcripts | T18, T23 |
| Host call report | `mycroft-call/1` contains a debug string for `program`: `MYC/compiler/src/bin/mycroft-suite.rs:110`. | built diagnostic artifact | T18, T23 |
| Executable call serialization | `rsl-call/1` contains instruction objects and `n_out`: `RSL/tools/mycroft-emit/src/main.rs:120`. Consumer: `RSL/src/rsl/compile.rs:589`. | partly built as a stable product interface | T18, T23 |
| R_SL micro-op compiler | Rule table: `RSL/src/rsl/compile.rs:73`. Execution builder: `:589`. Dispatch differs from slice CORE: `RSL/src/rsl/mod.rs:5`. | built at app level | T6, T18, T23 |
| ALU and canonical words | WORD and ALU AIRs: `RSL/src/rsl/air.rs:97`, `:131`. Modulus-specific checks: SECP:95. Mutation test: `RSL/tests/rsl_secp.rs:253`. | built, final proof integration open | T11, T24, T25 |
| Fr Poseidon and transient commitments | `RSL/src/rsl/compile.rs:336`, `:375`, `:889`. Fixed-length sponge correction: REKEY:25. Test: `RSL/tests/rsl_exec.rs:40`. | built | T11, T18, T23 |
| SHA-256 and byte alignment | `RSL/src/rsl/compile.rs:396`, `:844`. Tests: `RSL/tests/rsl_sha.rs:37`, `:51`, `:100`. Recorded measurement: COMPLETE `specs/sherlock-rsl-chipset/spec.md:78`. | built, recorded app tests | T11, T23, T25 |
| Keccak-256 | Dispatch and mixed-hash refusal: `RSL/src/rsl/compile.rs:844`. Positive and negative tests: `RSL/tests/rsl_keccak.rs:31`, `:50`, `:105`, `:150`. | partly built for full T23 chain | T24, T25 |
| Jubjub | `RSL/src/rsl/jubjub.rs`, with checks named by REKEY:28 and REKEY:36. Tests: `RSL/tests/rsl_jubjub.rs:22`, `:69`, `:96`, `:119`. Typed compiler test: `MYC/compiler/src/tests_secp.rs:351`. | partly built, wide-input gap remains | T11, T18, T23, T25 |
| Secp256k1 | Typed lowering: `MYC/compiler/src/zkir.rs:454`, `:568`. R_SL dispatch: `RSL/src/rsl/compile.rs:455`. Tests: `RSL/tests/rsl_secp.rs:69`, `:129`, `:195`, `:253`. Interface: SECP:20. | partly built for full T23 chain | T24, T25 |
| App AIR bucket and leaf compatibility | `RSL/src/rsl/trace.rs:183`, `:442`. Tests: `RSL/tests/rsl.rs:171`, `:191`. Five-call probe recorded at REKEY:33. | partly built, shape evidence only | T11, T23, T25 |
| R_SL result to authenticated recursion statement | `PublicAir` has no public values: `RSL/src/rsl/air.rs:199`. FP-1 probe carries zeros: `:217`, `RSL/src/rsl/trace.rs:447`. | design only for the required binding | T7, T11, T23, T25 |
| R_SL program identity and input admission | Missing `prog_d` and INIT connections are recorded at MAP:81. Existing formula: `MYC/spec/specs/claims/spec.md:167`. | design only for R_SL proof binding | T7, T18, T23, T25 |
| Compact seal, entry binding and anchor | Host frame: `MYC/compiler/src/pipeline.rs:159`, `:302`. Design: FINAL `specs/sherlock-compact-frontend/spec.md:52`. Slot collision refusal remains open in `MYC/factory/REPORT.md:111`. | partly built | T9, T12, T18, T23 |
| General interpreter and resumable host state | `ROOT/crates/mori-core/src/sherlock/reference.rs:9`, `:29`, `:43`. Profile and outcomes: `reference_types.rs:4`, `:129`, `:162`. | built host subset, partly built full language | T5, T6, T8, T13 |
| Nominal program image and full reference bridge | `ROOT/crates/mori-core/src/sherlock/program_image.rs:106`, `:126`. Full-language obligation: MOD `specs/sherlock-reference-bridge/spec.md:3`. | partly built | T5, T6, T13 |
| Universal interpreter AIR coverage | Five slice rules and two value rules: `RSL/src/dec.rs:23`, `:40`. Full-family obligation: MOD `specs/sherlock-chip-core/spec.md:109`. | partly built, not full-language proof coverage | T6, T8b, T13 |
| Compiler proof and conformance evidence | `MYC/lean/Mycroft/Lowering/Theorems.lean:232`, `:554`. Model limits: `MYC/lean/SORRIES.md:13`. KRS results: `MYC/factory/REPORT.md:12`. | partly built, named assumptions remain | T18, T13 |
| T23 Part B driver and comparison | `T23/tools/t23-suite/t23.py:146`, `:271`. Dry-run result: `T23/research/sherlock-t23-suite-20261115/T23-DRYRUN.md:3`. | partly built | T23 |
| Statement-to-acceptRun builder | `ARGS/tools/t23-suite/accept_args.py:287`, `:305`, `:347`. Host oracle tests: `test_accept_args.py:134`, `:171`. | built host mapping, proof connection absent | T23 |
| User command surface | FINAL `specs/sherlock-compact-frontend/spec.md:10` specifies six commands. A complete command chain was NOT FOUND in inspected Mycroft compiler sources and T23 tools. | design only as a unified surface | T17, T18, T23 |

The inventory does not convert a successful host execution into a successful proof. The existing test named `rsl_public_values_are_the_statement` reads trace cells. It does not check public-value binding. See `RSL/tests/rsl.rs:144` and `RSL/src/rsl/air.rs:199`.

## 2. Capability list

Keep one normative home for each ID. “Reuse” below means migration or explicit reference, not a second independent obligation. A proposed replacement retains the existing ID and records its changed meaning. New `AIF` and `ASI` IDs are lane proposals. The integrator assigns their final homes.

| Capability | ID prefix | Purpose | Components |
|---|---|---|---|
| `sherlock-compact-frontend` | CF, existing MYC-FE | Admit Compact and produce a deterministic, typed R_SL program. | source admission, ZKIR, GIR, ABI, type refusals |
| `sherlock-rsl-execution` | existing MYC-LOW, MYC-TAPE | Preserve native execution, tape consumption and effect order. | tape, interpreter, effects, commitment, ledger operations |
| `sherlock-rsl-chipset` | existing RSL, MYC-BLT | Execute the required kits within the universal recursion interface. | ALU, range, Poseidon, SHA-256, Keccak, Jubjub, secp256k1 |
| `sherlock-execution-statement` | AIF, existing MYC-CHIP, MYC-CLM, CF, ANC | Bind program, input and result to the statement consumed by B and C. | serialization, program identity, input admission, result binding, anchor, entry label |
| `sherlock-universal-interpreter` | existing UIREQ, RFB, CORE | Preserve general-machine meaning beyond the Compact straight-line path. | calls, values, outcomes, checkpoints, rule coverage |
| `sherlock-compact-suite` | ASI, existing STL, LPC | Account for every suite call and verify the final chain against native results. | corpus, Part B, native oracle, provenance, formal assumptions |
| `sherlock-developer-flow` | existing CF, ZK, CPR | Connect user commands to owned-machine proving and Preview settlement. | compiler entry points, proof request, completion gate |

STL-001 through STL-004 remain shared references to C/B requirements. RSL-001 and RSL-003 require B's evidence. Their internal proving mechanisms belong to B.

## 3. EARS draft

Each requirement has one normative sentence. Each Verification line states whether its check exists or must be built. No result below claims a fresh run.

### Capability: sherlock-compact-frontend

#### Requirement: CF-001 Stage 1 compiler input

When Stage 1 compiles Compact, Mycroft shall consume stock compactc 0.36.0 ZKIR 3.1 and the matching `contract-info.json`.

Lineage: proposed replacement of FINAL `specs/sherlock-compact-frontend/spec.md:3`, CF-001. Reuse MYC-FE-001 at `MYC/spec/specs/frontend/spec.md:11`. Preserve the `Lflattened` path as Stage 2 under MYC-FE-008. The Stage 1 replacement needs explicit integration into the governing change.

#### Scenario: Paired compiler artifacts
- WHEN t02 is compiled from one stock compiler output directory.
- THEN Mycroft uses that directory's ZKIR and contract information.

Verification: existing `mycroft-suite`, `MYC/compiler/src/bin/mycroft-suite.rs:31`. Build a mismatched-build artifact test before treating provenance validation as complete.

#### Requirement: MYC-FE-002 Pinned IR version

If the ZKIR version differs from 3.1, then Mycroft shall refuse the input with the observed version.

Lineage: reuse `MYC/spec/specs/frontend/spec.md:20`. No equivalent atomic requirement was found in COMPLETE or FINAL.

#### Scenario: Changed minor version
- WHEN a t02 fixture declares ZKIR 3.2.
- THEN compilation returns an IR-version refusal.

Verification: existing `compile_refusals`, `MYC/compiler/src/tests_lowering.rs:178`, and theorem `compile_refuses_version`, `MYC/lean/Mycroft/Lowering/Theorems.lean:397`. Build the serialized ZKIR mutation test.

#### Requirement: MYC-FE-010 Witness-independent compilation

Mycroft shall derive each program image, ABI and static instruction schedule from source and configuration only.

Lineage: reuse `MYC/spec/specs/frontend/spec.md:74`. This preserves CPR-003 in FINAL `specs/sherlock-general-purpose/spec.md:35`.

#### Scenario: Two tapes
- WHEN one entry is prepared with two different valid tapes.
- THEN its program image and ABI remain equal.

Verification: existing `RSL/tests/rsl.rs:60`, `rsl_key_independent_of_tape`, and `compile_inv`, `MYC/lean/Mycroft/Lowering/Theorems.lean:49`. Build a serialized program-and-ABI equality test.

#### Requirement: MYC-FE-012 Immediate byte order

When Mycroft reads a ZKIR immediate, Mycroft shall decode its bytes in little-endian order under the native field rules.

Lineage: reuse `MYC/spec/specs/frontend/spec.md:88`.

#### Scenario: Multi-byte immediate
- WHEN the input immediate is `0x0010`.
- THEN the value is 4096.

Verification: existing `h2_immediates_little_endian`, `MYC/compiler/src/zkir.rs:620`. Add malformed, odd-length and noncanonical fixtures if not covered by that test.

#### Requirement: CF-013 Secp256k1 lowering

When a circuit uses supported secp256k1 values, Mycroft shall lower those values and operations through the RSL-005 kit interface.

Lineage: reuse COMPLETE `specs/sherlock-compact-frontend/spec.md:3`. Its old status at `:12` is superseded by `MYC/compiler/src/zkir.rs:568`.

#### Scenario: ECDSA calls
- WHEN Mycroft compiles s02 `ecdsa-valid` and `ecdsa-bad`.
- THEN both calls use typed `Secp*` operations with native tape encodings.

Verification: existing `ecdsa_valid_equal_to_native`, `ecdsa_bad_equal_to_native`, `ecdsa_frontend_shape`, `MYC/compiler/src/tests_secp.rs:92`, `:97`, `:103`. Full-chain checks remain to be built under STL-002.

#### Requirement: CF-014 Unsupported input type

If an input type has no supported lowering kit, then Mycroft shall refuse the circuit and name the type.

Lineage: reuse COMPLETE `specs/sherlock-compact-frontend/spec.md:14`.

#### Scenario: Unsupported curve
- WHEN an input uses `Scalar<Curve25519>`.
- THEN compilation reports the unsupported type.

Verification: existing `secp_r1_and_25519_unsupported`, `MYC/compiler/src/tests_secp.rs:135`. Do not require the old `InputType` variant where the current frontend reports `Unsupported`.

#### Requirement: MYC-FE-005 Pure exports

When an exported circuit has no native proof circuit, Mycroft shall emit no Sherlock proof entry for that export.

Lineage: reuse `MYC/spec/specs/frontend/spec.md:39` and MYC-CLM-002. No additional ID is needed.

#### Scenario: Pure nativeMethods
- WHEN the suite processes h02 `nativeMethods`.
- THEN the result is pure, with no proof request.

Verification: existing suite case identified at `MYC/compiler/SUITE-COVERAGE.md:52`, `:132`. Build a product CLI assertion that no proof artifact is emitted.

#### Requirement: CF-007 Effect capacity

If ordinary-call effects exceed `L_TX - A`, then Mycroft shall refuse compilation with the entry name and required effect length.

Lineage: reuse the pre-chunk obligation of FINAL `specs/sherlock-compact-frontend/spec.md:45`. Keep the chunk extension under ANC-004 and ANC-012, with `K_MAX = 2`.

#### Scenario: Oversized ordinary entry
- WHEN `L_TX = 1024`, `A = 9` and `L_e = 1016` without chunking.
- THEN compilation refuses instead of truncating W.

Verification: existing `compile_refusals`, `MYC/compiler/src/tests_lowering.rs:178`, and `compile_refuses_long`, `MYC/lean/Mycroft/Lowering/Theorems.lean:415`. Build entry-name diagnostic coverage.

### Capability: sherlock-rsl-execution

#### Requirement: MYC-LOW-006 Native instruction meaning

The R_SL execution layer shall preserve each non-input instruction's native bindings, effects, outputs or rejection.

Lineage: reuse `MYC/spec/specs/lowering/spec.md:46`. CF-005 supplies the kit routing, not a substitute semantics.

#### Scenario: False assertion
- WHEN a native assertion fails on a fixed input.
- THEN R_SL rejects the same call without a successful result.

Verification: existing `exec_lower`, `MYC/lean/Mycroft/Lowering/Theorems.lean:32`, and `assert_rejects`, `MYC/compiler/src/tests_lowering.rs:124`. Builtins remain a shared theorem parameter, as recorded at `MYC/lean/SORRIES.md:25`.

#### Requirement: CF-003 Native effect equality

When an admitted Compact call executes, Mycroft shall produce W equal to the native public-effect vector word for word.

Lineage: reuse FINAL `specs/sherlock-compact-frontend/spec.md:17`. Consolidate MYC-LOW-010 and MYC-CLM-005 as references to this obligation.

#### Scenario: Guarded ledger impact
- WHEN a ledger impact has a false guard.
- THEN W contains the native zero words at the same indices.

Verification: existing `impact_words_and_false_guard`, `MYC/compiler/src/tests_lowering.rs:148`, and suite comparison at `MYC/compiler/src/bin/mycroft-suite.rs:102`. Build final proof-to-native comparison under ASI-002.

#### Requirement: CF-004 Static entry shape

Mycroft shall assign one instruction count and one effect length to an entry for all tapes of that entry.

Lineage: reuse FINAL `specs/sherlock-compact-frontend/spec.md:24` and MYC-LOW-012. This counts language instructions, not micro-op rows or proof time.

#### Scenario: Opposite guards
- WHEN two tapes select opposite values of one guard.
- THEN `N_e` and `L_e` are equal for both tapes.

Verification: existing `static_shape`, `MYC/lean/Mycroft/Lowering/Theorems.lean:351`, and ABI construction at `MYC/compiler/src/lower.rs:204`. Build a serialized two-tape ABI test.

#### Requirement: MYC-TAPE-005 Tape order

When the host prepares a call, the host shall write salt, selector, commitment randomness, arguments and native transcript input slots in their defined order.

Lineage: reuse `MYC/spec/specs/witness-tape/spec.md:37`. Replace CF-006's abstract tape description with this concrete Stage 1 layout without changing native effect order.

#### Scenario: Interleaved reads and witnesses
- WHEN an entry consumes public reads and private witness results.
- THEN their tape slots follow native execution order after the fixed header and arguments.

Verification: existing `host_tape_with`, `MYC/compiler/src/sem.rs:716`, and `forward`, `MYC/lean/Mycroft/Lowering/Theorems.lean:232`. Build wide-type end-to-end tape coverage.

#### Requirement: MYC-TAPE-006 Exact consumption

If the supplied tape has missing or unused words, then R_SL shall reject the call.

Lineage: reuse `MYC/spec/specs/witness-tape/spec.md:44`.

#### Scenario: Appended tape word
- WHEN one word is appended to a valid t02 tape.
- THEN execution rejects the modified tape.

Verification: existing `tape_shape_rejections`, `MYC/compiler/src/tests_lowering.rs:193`, and `rsl_rejected_tapes_refused`, `RSL/tests/rsl.rs:75`.

#### Requirement: MYC-TAPE-003 Guarded filler

While an input guard is false, R_SL shall require the typed filler in that input's tape slot.

Lineage: reuse `MYC/spec/specs/witness-tape/spec.md:23`. The typed extension must preserve MYC-TAPE-004, which prevents the inactive witness callback.

#### Scenario: Changed inactive input
- WHEN an inactive scalar input slot contains 5 instead of 0.
- THEN execution rejects it as bad filler.

Verification: existing `inputs_guarded_and_filler`, `MYC/compiler/src/tests_lowering.rs:131`. Typed point integration must be built. `filler_point` exists at `MYC/lean/Mycroft/Lowering/WideArgs.lean:163`.

#### Requirement: MYC-LOW-009 Communication commitment

When a call halts, R_SL shall compute `comm = transientCommit(argument_words ++ output_words, rand)`.

Lineage: reuse `MYC/spec/specs/lowering/spec.md:67`. MYC-BLT-002 defines the randomness-first sponge input.

#### Scenario: Changed output
- WHEN one output word changes under fixed arguments and randomness.
- THEN the recomputed commitment follows the native transient-commit function.

Verification: existing `comm_over_args_and_outputs`, `MYC/compiler/src/tests_lowering.rs:169`, and `RSL/src/rsl/compile.rs:889`. Build a final-proof mutation check for forged comm.

### Capability: sherlock-rsl-chipset

#### Requirement: CF-005 Stage 1 kit dispatch

When Stage 1 lowers a supported builtin, Mycroft shall emit its versioned R_SL kit operation.

Lineage: proposed atomic replacement of FINAL `specs/sherlock-compact-frontend/spec.md:31`, using the audit correction at AUDIT, “OpenSpec fixes”, CF-005. Keep Compact Field emulated under OD-14. The detailed kit obligations below replace stale routing prose.

#### Scenario: Missing kit
- WHEN an entry requires a kit absent from its machine version.
- THEN compilation reports the missing kit.

Verification: existing `compile_refusals`, `MYC/compiler/src/tests_lowering.rs:178`, and `secp_missing_kit`, `MYC/compiler/src/tests_secp.rs:122`.

#### Requirement: RSL-002 AIR bucket

The R_SL chip set shall expose six or nine app AIRs, including FP-1, for each admitted app.

Lineage: reuse COMPLETE `specs/sherlock-rsl-chipset/spec.md:14`. This is a current implementation choice, not permission to narrow GP-1.

#### Scenario: Program without a hash
- WHEN an app has no hash operation.
- THEN its six-AIR form includes an empty SHA_IN AIR.

Verification: existing `rsl_air_count_in_universal_leaf_bucket`, `RSL/tests/rsl.rs:191`. Build the complete admitted-suite count check after all kits and statement bindings are integrated.

#### Requirement: RSL-001 Universal leaf compatibility

When an R_SL app enters the leaf layer, the leaf prover shall produce the universal N1 pre-hash.

Lineage: reuse COMPLETE `specs/sherlock-rsl-chipset/spec.md:3`. B owns the proof implementation and its final-key check.

#### Scenario: Different app shapes
- WHEN a05, t02, s01, d04 and z02 enter the leaf layer.
- THEN all produce the same universal N1 pre-hash.

Verification: existing odin r10 probe recorded at REKEY:33, commit `6f773b2`. B must rerun it with authenticated R_SL statements and all T25 changes.

#### Requirement: RSL-007 SHA-256

When a circuit calls `persistentHash`, the R_SL kit shall prove native-aligned SHA-256 through SHA_IN, SHA_REQ and the OpenVM block hasher.

Lineage: reuse COMPLETE `specs/sherlock-rsl-chipset/spec.md:69`.

#### Scenario: Corrupt digest limb
- WHEN a witness changes one SHA-256 digest limb.
- THEN the kit rejects the forged trace.

Verification: existing `rsl_sha_calls_prove`, `rsl_sha_forged_rows_refused`, `rsl_sha_unfit_chunk_refused`, `RSL/tests/rsl_sha.rs:37`, `:51`, `:100`.

#### Requirement: RSL-004 Keccak-256

When a supported circuit calls `keccak256`, the R_SL kit shall prove that call inside the six-or-nine-AIR bucket.

Lineage: reuse COMPLETE `specs/sherlock-rsl-chipset/spec.md:36`. T24 removes the earlier kit deferral. See LOOP:46.

#### Scenario: Native keccak call
- WHEN s02 executes its keccak call.
- THEN its output and comm match native execution and its app proof verifies.

Verification: existing `rsl_keccak_call_proves` and `rsl_keccak_forged_rows_refused`, `RSL/tests/rsl_keccak.rs:50`, `:105`. B must build the final-key leaf compatibility check.

#### Requirement: RSL-005 Secp256k1

When a supported secp256k1 operation executes, the R_SL kit shall enforce the typed semantics and canonical encodings in SECP sections 2 through 4.

Lineage: reuse COMPLETE `specs/sherlock-rsl-chipset/spec.md:47`, refined by SECP:20, SECP:38, SECP:55. Preserve its AIR-bucket condition through RSL-002.

#### Scenario: Invalid encoded scalar
- WHEN a scalar input uses a noncanonical encoding at or above n.
- THEN the kit rejects the call.

Verification: existing `rsl_secp_refusals`, `rsl_secp_ecdsa_proves`, `rsl_secp_word_soundness`, `RSL/tests/rsl_secp.rs:129`, `:195`, `:253`. The last test covers values above Fr and below p.

#### Requirement: RSL-006 Jubjub

When a Jubjub point enters a curve operation, the R_SL kit shall enforce on-curve and subgroup membership through micro-ops without another AIR.

Lineage: reuse COMPLETE `specs/sherlock-rsl-chipset/spec.md:58`. The subgroup relation is `8Q = P`, recorded at REKEY:28.

#### Scenario: Point outside the subgroup
- WHEN a point outside the subgroup enters ecMul.
- THEN no accepting micro-op witness exists.

Verification: existing `rsl_jubjub_matches_native`, `rsl_jubjub_leaf_proof`, `rsl_jubjub_s01_ec_call`, `RSL/tests/rsl_jubjub.rs:22`, `:69`, `:119`. Build an explicit out-of-subgroup forged-trace test if those tests lack that mutation.

#### Requirement: MYC-BLT-001 Poseidon parity

When R_SL executes `transientHash`, the kit shall use the native fixed-length Fr Poseidon sponge over the same ordered words.

Lineage: reuse `MYC/spec/specs/builtins/spec.md:11`. CF-005's older `SPONGE(n)` description is refined by REKEY:25.

#### Scenario: Empty and nonempty inputs
- WHEN input lengths include zero, one and a multi-rate length.
- THEN each digest equals midnight-circuits 7.3.0.

Verification: existing `rsl_transient_hash_fixed_length`, `RSL/tests/rsl_exec.rs:40`, checks three suite fixtures. Build explicit empty, one-word and multi-rate vectors for this scenario. The sponge implementation is at `RSL/src/rsl/compile.rs:375`.

#### Requirement: RSL-008 Canonical ALU and Poseidon inputs

If an R_SL arithmetic input is noncanonical for its declared modulus, then the kit shall reject the row.

Lineage: reuse COMPLETE `specs/sherlock-rsl-chipset/spec.md:80`. Refine “canonical” by the row modulus. A valid secp word above Fr is not a noncanonical Fr input.

#### Scenario: Equivalent residue with a larger encoding
- WHEN a witness substitutes `m + x` for x under modulus m.
- THEN the range and canonicality constraints reject it.

Verification: existing `rsl_secp_word_soundness`, `RSL/tests/rsl_secp.rs:253`, and `rsl_forged_rows_refused`, `RSL/tests/rsl.rs:103`. Build an explicit noncanonical Poseidon-input test and rerun both through the stock verifier.

#### Requirement: RSL-003 App proofs for suite calls

When the suite prover processes the 142-call R_SL baseline, the app prover shall produce a stock-verifier-accepted proof for each call.

Lineage: reuse COMPLETE `specs/sherlock-rsl-chipset/spec.md:25`. Keep this baseline distinct from the full T23 corpus and later kit additions.

#### Scenario: One failed app proof
- WHEN one baseline app proof fails verification.
- THEN the report identifies the call and verifier failure without an app-proof pass.

Verification: existing `rsl_suite`, `RSL/examples/rsl_suite.rs:14`, and `prove_call`, `RSL/src/rsl/mod.rs:30`. A complete final-key proof run must be built and recorded by B. REKEY:30 records execution equality only.

### Capability: sherlock-execution-statement

#### Requirement: AIF-001 Versioned executable call

When Mycroft sends a call to the execution adapter, Mycroft shall encode the versioned typed instruction sequence and its static ABI.

Lineage: new integration obligation. `mycroft-call/1` and `rsl-call/1` differ at `MYC/compiler/src/bin/mycroft-suite.rs:110` and `RSL/tools/mycroft-emit/src/main.rs:120`.

#### Scenario: Executable instruction round trip
- WHEN Mycroft emits a valid typed program and ABI.
- THEN the execution adapter recovers the same instruction sequence and ABI from the artifact.

Verification: build `rsl_call_schema_roundtrip_and_refusal`. Include schema version, operand types, missing fields and unknown opcodes. Current unchecked reads occur at `RSL/src/rsl/compile.rs:592`, `:622`.

#### Requirement: AIF-003 Invalid executable artifact

If an executable call violates its versioned schema, then the execution adapter shall refuse it before proof generation.

Lineage: new refusal obligation for the AIF-001 handoff. No schema-validation requirement was found in COMPLETE's RSL capability.

#### Scenario: Diagnostic string supplied as a program
- WHEN the consumer receives a debug-string program instead of instruction objects.
- THEN it returns a named schema refusal without starting a proof.

Verification: build `rsl_call_schema_roundtrip_and_refusal`, with a proof-start spy and the malformed artifacts listed under AIF-001.

#### Requirement: MYC-CLM-023 Authenticated program identity

The planner shall derive `prog_d` from the program image, plan, key root and machine version under the defined program domain.

Lineage: reuse `MYC/spec/specs/claims/spec.md:167`. B owns the proof binding. This is not permission to accept a host-supplied digest without authentication.

#### Scenario: Changed program under an old identity
- WHEN one instruction changes while the submitted `prog_d` stays fixed.
- THEN proof verification rejects the claimed identity.

Verification: build `rsl_wrong_program_identity_refused` on a real R_SL proof. MAP:82 records the missing R_SL connection. B supplies the canonical digest encoding and key-root membership evidence.

#### Requirement: AIF-002 Authenticated R_SL input

When R_SL admits a run, the execution adapter shall bind `input_d` to that run's actual tape under the versioned input encoding.

Lineage: new integration obligation for the gap at MAP:83. Proposed encoding follows MAP:73: `val_d = vd(tape)`, fresh digest salt, fuel zero and stack limit zero.

#### Scenario: Changed private tape
- WHEN a proof request substitutes another tape while retaining the original input commitment.
- THEN the proof fails its input binding.

Verification: build `rsl_input_tape_binding_refused`, including zero-length, changed-header and changed-witness cases. B supplies the authenticated INIT relation. A host hash test alone is insufficient.

#### Requirement: MYC-CHIP-004 Authenticated R_SL result

When an R_SL call halts, the execution adapter shall bind `Halted([comm] ++ W)` to the authenticated recursion outcome.

Lineage: proposed replacement of `MYC/spec/specs/chipset/spec.md:29`. Its old requirement exposes a 1,026-word call vector as leaf public values. The current leaf interface has 54 FP-1 words. See `RSL/src/core.rs:43`. Use ANC-018 for the later call-lane binding. Do not create a second ANC-018.

#### Scenario: Changed effect with unchanged claimed result
- WHEN one proven effect word is replaced while the claimed outcome remains fixed.
- THEN the recursion verifier rejects the proof.

Verification: build `rsl_outcome_binding_refused`, with changed comm, changed last effect, reordered effects and omitted effect mutations. Existing trace test `RSL/tests/rsl.rs:144` is insufficient. The gap is explicit at `RSL/src/rsl/air.rs:199`.

#### Requirement: STL-001 Settlement argument mapping

When the settlement adapter prepares acceptRun, it shall derive its public arguments from the authenticated Mycroft statement and check the wrap's public lanes.

Lineage: reuse COMPLETE `specs/sherlock-suite-settlement/spec.md:3`. Correct its terminology using MAP:23: arguments are `c_p`, `input_pub`, `outcome_pub`. The contract computes `claim_pub`.

#### Scenario: Outcome mismatch
- WHEN the reported `outcome_pub` differs from the statement-derived value.
- THEN the adapter refuses the bundle.

Verification: existing `test_call_vector_lanes`, `test_negative_changed_effect_word`, `test_prove_result_crosscheck`, `ARGS/tools/t23-suite/test_accept_args.py:152`, `:179`, `:188`. Build the authenticated-proof version with B and C.

#### Requirement: ANC-015 Anchor collision refusal

If a Compact layout uses top-level slot 15, then Mycroft shall refuse that layout before deployment.

Lineage: reuse COMPLETE `specs/sherlock-anchor-chunks/spec.md:25`. Consolidate MYC-SEAL-021 at `MYC/spec/specs/seal/spec.md:155` under this requirement. Preserve the field name in the diagnostic.

#### Scenario: User field in slot 15
- WHEN a source contract allocates its sixteenth top-level field at slot 15.
- THEN compilation reports the colliding field.

Verification: build `T-ANCHOR-DISJOINT` and a slot-15 layout refusal test. They are listed as missing at `MYC/spec/specs/seal/spec.md:162`. C supplies the deployed anchor-slot constant under ANC-013 and ANC-014.

#### Requirement: CF-008 Entry-label binding

When a direct Compact call is proved, Mycroft shall bind its authenticated entry label through the placement-V SHA-256 preimage check.

Lineage: reuse FINAL `specs/sherlock-compact-frontend/spec.md:52`. MYC-BIND-001 through MYC-BIND-008 retain the exact domain, offset, label encoding and final-call normalization rules.

#### Scenario: Cross-entry substitution
- WHEN a proof for entry e is attached to a call for a different entry.
- THEN verification rejects the substituted entry.

Verification: build a real Mycroft cross-entry proof test on the final profile. FINAL `specs/sherlock-compact-frontend/spec.md:55` describes the earlier prototype control. Current R_SL `bind_label` integration was NOT FOUND in `RSL/src/rsl/compile.rs` or `MYC/compiler/src/lower.rs`.

#### Requirement: CF-009 Versioned label layout

When the host constructs the binding preimage, the execution adapter shall bind its domain and label-window layout to the wrap's authenticated layout constant.

Lineage: reuse FINAL `specs/sherlock-compact-frontend/spec.md:63`, placement V+.

#### Scenario: Changed label offset
- WHEN the preimage uses another label offset under the same wrap profile.
- THEN the layout check rejects it.

Verification: build `mycroft_layout_constant_refused`. C supplies the layout encoding and key constant. The direct-call test is separate from T23 acceptRun settlement.

### Capability: sherlock-universal-interpreter

#### Requirement: UIREQ-001 Fixed machine relation

When the interpreter executes admitted programs under one machine version, it shall apply the same versioned transition relation to each program.

Lineage: reuse UI `specs/universal-interpreter/spec.md:7`. Preserve GP-001 and CPR-005. Compact's R_SL path does not replace the general interpreter.

#### Scenario: Factorial and list sum
- WHEN both programs execute under one machine version.
- THEN program data changes while the transition relation remains fixed.

Verification: existing host `Machine::step`, `ROOT/crates/mori-core/src/sherlock/reference.rs:43`. Build the two-program proof check across the complete interpreter AIR inventory.

#### Requirement: UIREQ-003 Lexical call scope

When a named call enters its body, the interpreter shall use the callee's parameter bindings and declared global function table.

Lineage: reuse UI `specs/universal-interpreter/spec.md`, UIREQ-003.

#### Scenario: Caller-local variable
- WHEN a callee binds x to 2 while its caller has x equal to 7.
- THEN return restores the caller's x equal to 7.

Verification: existing `uireq003_callee_local_x2_preserves_caller_x7`, `ROOT/crates/mori-core/tests/sherlock_original_ears_host_positive.rs:196`. Build the corresponding complete-interpreter proof case.

#### Requirement: UIREQ-004 Evaluation order

When an expression evaluates multiple operands, the interpreter shall evaluate each operand once in left-to-right order.

Lineage: reuse UI `specs/universal-interpreter/spec.md`, UIREQ-004.

#### Scenario: Nested calls
- WHEN `add(double(2), increment(3))` executes.
- THEN double finishes before increment starts and the result is 8.

Verification: existing `uireq004_double_completes_before_increment_starts_and_add_returns8`, `ROOT/crates/mori-core/tests/sherlock_original_ears_host_positive.rs:257`. Build a proof-backed event-order mutation test.

#### Requirement: UIREQ-005 Recursive calls

When a named function recurses, the interpreter shall apply its ordinary call and return rules.

Lineage: reuse UI `specs/universal-interpreter/spec.md`, UIREQ-005.

#### Scenario: Self and mutual recursion
- WHEN factorial(3), even(4) and odd(5) execute within their declared bounds.
- THEN their results are 6, true and true.

Verification: existing tests at `ROOT/crates/mori-core/tests/sherlock_original_ears_host_positive.rs:315`, `:352`, `:372`. Build final-key proof cases without per-program universal keys.

#### Requirement: UIREQ-012 Explicit bounded outcome

If an execution cannot perform its next transition within its declared runtime bounds, then the interpreter shall return the profile-defined bound outcome.

Lineage: reuse UI `specs/universal-interpreter/spec.md`, UIREQ-012. Preserve the distinction between admission refusal, execution rejection, exhaustion and a halted value.

#### Scenario: Exhausted fuel
- WHEN an otherwise valid run reaches its fuel boundary.
- THEN its outcome records exhaustion instead of an application return.

Verification: existing outcome types at `ROOT/crates/mori-core/src/sherlock/reference_types.rs:129` and Quint cases at `ROOT/models/sherlock/quint/interpreter-chunks/interpreterChunks_test.qnt:20`. Build every-outcome proof coverage under RFB-001.

#### Requirement: UIREQ-016 Technical chunk transparency

When proof production inserts a technical chunk boundary, the interpreter shall preserve the program's source-visible result, effects and cumulative logical work.

Lineage: reuse UI `specs/universal-interpreter/spec.md`, UIREQ-016. This is distinct from ledger transaction chunking under ANC-004.

#### Scenario: Cut before caller addition
- WHEN a technical boundary falls between callee return and caller addition.
- THEN resumption produces the same result and work as the uncut trace.

Verification: existing `boundary_between_callee_return_and_caller_addition_is_transparent`, `ROOT/crates/mori-core/tests/sherlock_technical_chunks_positive.rs:195`, and `resume_reproduces_the_checkpoint_and_refuses_inconsistent_ones` at `:255`. Build the matching proof test.

#### Requirement: RFB-003 Complete rule projection

The reference bridge shall report total state projection and coverage of all 120 enabled rules, including reasons for unreachable rules.

Lineage: proposed atomic consolidation of MOD `specs/sherlock-reference-bridge/spec.md:17`. Preserve its prohibition on `UnsupportedGeneric` and `UnmappedRule` in the accepted corpus.

#### Scenario: Unmapped constructor case
- WHEN a corpus constructor state lacks a reference projection.
- THEN the coverage gate fails and records the gap.

Verification: build or locate `bridge_rules_fired_cover_inventory` and `bridge_corpus_zero_gaps` on the complete current machine. A complete passing run was NOT FOUND in the inspected MOD specs, `ROOT/crates/mori-core/src/sherlock` and LOOP T6 evidence.

#### Requirement: CORE-011 Rule refinement

When a chip proves a rule-family transition, the chip shall enforce the corresponding reference transition.

Lineage: reuse MOD `specs/sherlock-chip-core/spec.md:109`. B owns the constraints and proof checks. A owns the reference meaning and event projection.

#### Scenario: Forged next state
- WHEN a witness changes one next-state field without a valid transition.
- THEN the constraint checker and stock verifier reject it.

Verification: build one honest case and one forged-state case per enabled rule. `RSL/src/dec.rs:23` and `:40` provide only the inspected slice and value subsets. Their existence does not close this requirement.

### Capability: sherlock-compact-suite

#### Requirement: ASI-001 Complete suite accounting

When T23 Part B runs, the suite runner shall account for every recorded call of every stock-compilable program in the pinned corpus.

Lineage: new integration accounting obligation from LOOP:45. STL-004 still owns accepted settlement. STL-006 still owns the native Part A reference run.

#### Scenario: Refused and pure calls
- WHEN the baseline contains 208 calls across 33 stock-compilable programs.
- THEN every call has a named result, including native refusal and pure cases.

Verification: existing enumeration at `T23/tools/t23-suite/t23.py:158`. Build `t23_manifest_no_omissions`, with duplicate, omitted and extra-call mutations. The 36-source denominator and three stock compile refusals remain visible through MYC-FE-004.

#### Requirement: STL-005 Native result comparison

If a proved call differs from the native reference, then the suite runner shall classify the call as a mismatch.

Lineage: reuse COMPLETE `specs/sherlock-suite-settlement/spec.md:47`. Keep the reference's source, version and normalization explicit.

#### Scenario: Changed effect word
- WHEN a proved call has one effect word different from the native transcript.
- THEN the comparison records its call ID and first differing word.

Verification: existing T23 `selftest` and comparison implementation in `T23/tools/t23-suite/t23.py`, introduced at `:20`. Build a final-proof fixture test. Address-substituted comparisons must remain separately labeled.

#### Requirement: ASI-002 Evidence required for a suite pass

When a proof-bearing call receives a Part B pass, the report shall contain its complete final-key proof, verification, settlement and native-comparison evidence.

Lineage: new integration acceptance predicate over RSL-003, STL-001, STL-002, STL-003, STL-004 and STL-005. It adds no proving or settlement algorithm.

#### Scenario: Host equality without a proof
- WHEN host execution matches native but no proof exists.
- THEN the call remains blocked and receives no Part B pass.

Verification: build `t23_pass_requires_complete_evidence`. Delete each evidence field in turn, including proof hash, key-set ID, verifier receipts, Preview transaction, block and readback. Existing dry-run evidence states zero proved comparisons at `T23/research/sherlock-t23-suite-20261115/T23-DRYRUN.md:21`.

#### Requirement: ASI-003 Zero-mismatch completion

When T23 reports completion, the report shall show zero mismatches and zero unresolved proof-bearing calls across the full pinned corpus.

Lineage: new non-vacuous completion predicate from LOOP:45. Native-refused calls and pure calls retain separate dispositions under MAP:75 and MYC-FE-005.

#### Scenario: Empty successful comparison set
- WHEN no proof-bearing call reaches comparison.
- THEN a zero-mismatch count does not produce a completion result.

Verification: build `t23_zero_comparisons_cannot_complete`, using the existing dry run with 145 blocked calls as the negative fixture. Source: `T23/research/sherlock-t23-suite-20261115/T23-DRYRUN.md:11`.

#### Requirement: LPC-013 Compiler conformance gates

When T18(d) is accepted, the Mycroft conformance suite shall pass its pinned gates and warning-free Lean build.

Lineage: reuse COMPLETE `specs/sherlock-loop-completion/spec.md:135`. These are development checks, not public developer prerequisites.

#### Scenario: Surviving mutation
- WHEN a mutation survives a gate.
- THEN the report gives its accepted argument or keeps the gate unresolved.

Verification: existing KRS emit, oracle-equal, golden, coverage, mutations and compare runs, `MYC/factory/REPORT.md:12`. Requalification must include new typed kits. No test ran in this draft.

#### Requirement: LPC-019 Formal assumption disclosure

When Mycroft reports formal coverage, the report shall identify every remaining theorem premise and model limitation.

Lineage: refine COMPLETE `specs/sherlock-loop-completion/spec.md:201`, LPC-019, without replacing its specific S1 obligation. Consolidate MYC-LOW-019 as a reference.

#### Scenario: Builtin supplied as a parameter
- WHEN a lowering theorem assumes builtin semantics.
- THEN the report names that premise and the separate kit checks.

Verification: existing `MYC/lean/SORRIES.md:13`, `:14`, `:25`, `:83`. Build an assumption-report check that covers S1, S2, wide arguments, secp types and the missing Rust-to-AIR refinement.

### Capability: sherlock-developer-flow

#### Requirement: CF-002 User command surface

The Sherlock toolchain shall provide compile, plan, simulate, deploy, prove and call operations with versioned artifact handoffs.

Lineage: reuse FINAL `specs/sherlock-compact-frontend/spec.md:10`. A owns compile and simulate. B owns plan and prove. C owns deploy, call and their public CLI integration.

#### Scenario: Owned-machine Preview run
- WHEN a developer supplies Compact source and the user supplies private inputs on owned machines.
- THEN the operations produce a Preview settlement receipt tied to the same compiled program and claim.

Verification: build `compact_to_preview_user_flow` using the ordered steps in section 4. A complete unified run was NOT FOUND in the inspected Mycroft sources, T23 tools and MAP:77.

#### Requirement: ZK-018 Case B refusal

If a proof request requires an unowned helper host, then the prover shall refuse that request in the prototype.

Lineage: reuse COMPLETE `specs/sherlock-zero-knowledge/spec.md:69`. B/C own enforcement. A preserves the trust-domain field and supplies no private tape to case B.

#### Scenario: External helper request
- WHEN a caller requests proof work on a host outside the user's case-A trust domain.
- THEN the request is refused before private artifacts are transferred.

Verification: build or consume B/C's case-B dispatch refusal test, including a transfer spy. The required boundary follows OD-9 at DECISIONS:23 and TERMS:13.

#### Requirement: CPR-005 Generality remains intact

The toolchain shall preserve GP-001 through GP-004 and GP-ZK when it reports implementation limits.

Lineage: reuse FINAL `specs/sherlock-general-purpose/spec.md:49`. MYC-REF-018 supplies the capacity-refusal reporting rule.

#### Scenario: Unsupported mixed-hash app
- WHEN the current chip set cannot admit a program using both SHA-256 and Keccak.
- THEN the report identifies an implementation gap without narrowing the language contract.

Verification: existing refusal test `RSL/tests/rsl_keccak.rs:150`. Build a report assertion and a generality coverage matrix. The refusal itself does not prove GP-1.

### Required inherited obligations

These requirements remain in their current homes. Do not duplicate them in slice A. The integrator must preserve their checks and explicit ownership.

| Existing IDs | Home | Slice A dependency |
|---|---|---|
| GP-001 through GP-004 | `ROOT/openspec/changes/sherlock-vm-consensus-20261107/specs/sherlock-general-purpose/spec.md:3` | Universal programs, composition, foreign proofs and PCD |
| CPR-001 through CPR-004, GP-ZK | FINAL `specs/sherlock-general-purpose/spec.md:3` | Circuit limits, fixed shards, universal keys, unchanged verifier, privacy |
| CF-006, CF-010 through CF-012 | FINAL `specs/sherlock-compact-frontend/spec.md:38`, `:70` | Tape/effect details, entry-binding evidence, ordered milestones, CF-L1 |
| MYC-FE-003, 004, 006 through 009, 011 | `MYC/spec/specs/frontend/spec.md:28` | Entry table, stock refusal, scope, unsupported forms, Stage 2 equality, determinism, diagnostics |
| MYC-VAL-001 through 017, MYC-LED-001 through 011 | `MYC/spec/specs/values/spec.md:11`, `ledger-ops/spec.md:11` | Native value representation and ledger-operation meaning |
| MYC-TAPE-002, 004, 007 through 015 | `MYC/spec/specs/witness-tape/spec.md:16` | Ranges, inactive callbacks, selectors, reads, privacy, wide input words |
| MYC-BLT-002 through 016 | `MYC/spec/specs/builtins/spec.md:18` | Commitments, bytes, curve failures, coin and token expansions. Amend stale tier-3 deferral. |
| MYC-BIND-001 through 009, MYC-SEAL-001 through 022 | `MYC/spec/specs/entry-binding/spec.md:13`, `seal/spec.md:11` | Direct Compact call framing and label binding. Reconcile with acceptRun before claiming coverage. |
| ANC-013, 014, 016 through 019 | COMPLETE `specs/sherlock-anchor-chunks/spec.md:3` | Slot constant, storage, ordinary-call guard, canonical lock, FP-3 binding and slice count |
| ANC-004 through 011, ANC-012 | FINAL `specs/sherlock-anchor-chunks/spec.md:24`, LOOP:42 | Atomic chunks, legal cuts, context reads, phases and cap. ANC-012 is on the t20-batched lineage. |
| RKY-001 through 005 | COMPLETE `specs/sherlock-rekey/spec.md:3` | One complete rekey list and final size/security qualification |
| ZK-012 through 017, ZK-019 | COMPLETE `specs/sherlock-zero-knowledge/spec.md:3` | Per-deployment and per-run salts, production seed guard, one-use slots and proof privacy |
| STL-002 through 004, STL-006 | COMPLETE `specs/sherlock-suite-settlement/spec.md:14`, `:58` | Final chain, deploy, readback and native Part A oracle |
| UIREQ-002, 006 through 011, 013 through 015, 017 through 035 | UI `specs/universal-interpreter/spec.md` | Preserve profiles, values, pending frames, resource observations and extension status. No silent deletion. |
| RFB-001, 002, 004, CORE-001 through 015 | MOD `specs/sherlock-reference-bridge/spec.md:3`, `sherlock-chip-core/spec.md:3` | Complete nominal semantics, event counts, gap failure and AIR refinement |
| LPC-001 through 003, LPC-006, LPC-007 | COMPLETE `specs/sherlock-loop-completion/spec.md:3`, `:58` | GP hooks, MERGE-WORK, setup schedules and elastic execution |

## 4. End-to-end user flow

This is the required flow. A missing implementation is listed in section 7. A T23 acceptRun receipt certifies a run. It does not itself apply the original Compact contract's ledger effects. See MAP:45.

| Step | Actor and operation | Input artifact | Output artifact | Owner and binding |
|---|---|---|---|---|
| 1 | Developer writes Compact | source and dependencies | pinned source package | A, CF-001, MYC-FE-004 |
| 2 | Compiler performs native analysis | source package | stock ZKIR 3.1, contract-info, manifest and runtime | A, CF-001, MYC-FE-003 |
| 3 | Mycroft lowers each proof entry | pinned native output | typed R_SL program and static ABI | A, CF-013, CF-014, CF-004 |
| 4 | Planner admits the image and kits | program, machine version, kit set | plan, app-key commitments, `key_root`, `prog_d`, key-set identity | A supplies semantics, B owns admission and keys, MYC-CLM-023, RSL-001 |
| 5 | Setup profiles the user's machine | hardware and resource budget | `sherlock-profile/1`, execution schedule | B, setup-mode replacement of F-3, LOOP:105 |
| 6 | Deployment adapter binds the program | `prog_d`, private deployment blinder reference, final wrap profile | public `c_p`, deployed SherlockLedger address, deploy receipt | C, STL-003, ZK-012, ANC-014 |
| 7 | User simulates and prepares one call | arguments, current ledger observations, witness callbacks | private tape, typed `rsl-call/1`, expected W and comm | A, MYC-TAPE-005, CF-003, AIF-001 |
| 8 | Execution adapter admits the run | program identity, private tape, fresh run salt | authenticated input/result statement request | A/B, AIF-002, MYC-CHIP-004, ZK-013 |
| 9 | User's machines prove the run | admitted request and fixed-shape work items | app, leaf, node, L and L_Fr proof evidence | B, RSL-003, STL-002. No unowned helper. |
| 10 | Wrap adapter produces the published proof | authenticated root and private bridge inputs | one blinded BLS12-381 wrap, public call vector, VK identity | C, CPR-001, CPR-004, RKY-004, GP-ZK |
| 11 | Local settlement adapter checks the mapping | private statement bundle and wrap public lanes | `c_p`, `input_pub`, `outcome_pub`, computed `claim_pub` | A/C, STL-001. Private bundle stays on owned machines. |
| 12 | Caller submits acceptRun on Preview | wrap, three arguments, deployment identity | included transaction hash and block, or refusal | C, STL-004. Existing wallet state stays private. |
| 13 | Caller reads settlement | `claim_pub` and contract address | `runs[claim_pub] = outcome_pub` readback | C, STL-004 |
| 14 | T23 compares the accepted run | authenticated opening of `[comm] ++ W`, native Part A result, receipt | per-call disposition and measured stage times per box | A, STL-005, ASI-001 through 003 |

For operator batches, C replaces per-item writes with one collection root per checkpoint. Membership records remain off chain. User acceptRun settlement remains the T23 route. Source: LOOP:42.

If source compilation fails, the flow ends before planning. If native execution refuses a call, T23 records the refusal without settlement. Pure exports produce no proof. A stale ledger read, failed proof or changed key identity stops the dependent step. Failed work never becomes a completed receipt.

The existing transport needs reconciliation. T23 requests `mycroft-call/1` at `T23/tools/t23-suite/t23.py:281`. The R_SL consumer expects an instruction array at `RSL/src/rsl/compile.rs:622`. AIF-001 closes this handoff.

## 5. Decision list

Keep existing decision identities. The following names are source identities or source section labels, not new D-number allocations. The integrator can map them to its reserved MADR records. An accepted decision does not prove implementation. See ARCH-PROFILE:65 and the mapped MADR template at `/home/hoskinson/Projects/software-factory/skills/specs/spec-architect/references/madr-template.md:76`.

| Decision and outcome | Source and authority | Rejected options or missing evidence | Binding IDs and confirmation | Revisit when |
|---|---|---|---|---|
| GP-1 through GP-4 and GP-ZK remain intact | LOOP:102, FINAL `specs/sherlock-general-purpose/spec.md:49`. OWNER. | Reduced language, bounded composition, excluded foreign proofs or restricted PCD do not satisfy the hard rule. | CPR-005, RFB-003, CORE-011, GP-001 through 004. Full coverage and proof mutations. | Owner changes a hard rule. |
| Small-field leaves/nodes and one BLS12-381 wrap | BRIEF “Hard rules”, DECISIONS:35, OD-24. OWNER. | OD-24 rejects a changed ledger verifier, an oversized single-lane wrap, two BLS layers and list decoding. | CPR-001, CPR-004, STL-002, RKY-003 through 005. B/C gates. | Owner changes a hard rule. |
| Universal key set with authenticated per-program preprocessing | TERMS:23, TERMS:36, FINAL `specs/sherlock-general-purpose/spec.md:35`. OWNER. | New universal keys per program or per run length. Per-program app keys are permitted under `key_root`. | CPR-003, MYC-CLM-023, RSL-001. Cross-program key test. | Key membership or program identity changes. |
| OD-14 keeps Compact Field emulated | DECISIONS:28. OWNER. | Direct use of BabyBear as Compact Field violates the selected field meaning. Other compared options: NOT FOUND in the inspected decision row. | CF-005, MYC-VAL-001, RSL-008. Native arithmetic parity. | Owner changes field meaning or implementation proves the same semantics another way. |
| OD-5 selects placement V with V+ | DECISIONS:19, TERMS:57. OWNER. | Alternative placements are NOT FOUND in the inspected decision row. No invented rejection history. | CF-008, CF-009, MYC-BIND-001 through 008. Cross-entry and changed-layout refusals. | Ledger binding layout changes. |
| OD-7 and OD-15 fix 1,024 transcript lanes and one 1,026-word call vector | DECISIONS:21, DECISIONS:29. OWNER. | Per-program public-vector sizes are excluded by this choice. Other evaluated sizes: NOT FOUND in these rows. | CF-007, MYC-SEAL-001, MYC-CLM-004. Boundary-length tests. | Measured entries exceed capacity within the hard rules. |
| OD-8 and OD-16 support hybrid native entries with an anchor prologue | DECISIONS:22, DECISIONS:30. OWNER. | A native entry that bypasses the anchor check. | ANC-009, MYC-SEAL-006. T2-N and T2-N0 controls named in FINAL anchor spec:67. | Native routing or anchor layout changes. |
| OD-6 and OD-17 reserve mandatory log work before discretionary work | DECISIONS:20, DECISIONS:31. OWNER. | Discretionary work displacing the mandatory reservation. | MYC-CLM-025, MYC-CLM-026, B/C log requirements. Reservation overflow tests. | Log capacity or encoding changes. |
| OD-9 restricts hosts to case A | DECISIONS:23, TERMS:13, COMPLETE ZK-018:69. OWNER. | Untrusted helper proving now. Case B is deferred to Veritee by BRIEF. | ZK-018, MYC-TAPE-012, GP-ZK. Refuse before transfer. | Owner enables the case-B design. |
| F-3 is replaced by setup mode | LOOP:105. OWNER. | A fixed schedule menu inside `key_root`. | SPL-002, SPL-011, LPC-006, LPC-007. Same claim across profiles. | Hardware or scheduling mechanism changes. |
| T12-SLOT15 reserves anchor slot 15 | COMPLETE `specs/sherlock-anchor-chunks/spec.md:3`, AUDIT “OpenSpec fixes”, ANC-001. OWNER, as recorded by the audit. | Cell 0 collision and relocation of user fields. Relocation changes native W. See `MYC/spec/specs/seal/spec.md:159`. | ANC-013 through 015, MYC-SEAL-021. Forged slot and layout-collision tests. | State layout or anchor profile changes. |
| Canonical lock uses x_j, not DOM_LOCK | LOOP:106, REKEY:5. OWNER. | `P2(DOM_LOCK, r_P, prog_d, run_tag, j)` is superseded. | ANC-017, ANC-019. NT-2/NT-3 replay and truncation controls from B/C. | Lock inputs or chunk semantics change. |
| Chunk cap is K_MAX = 2 | LOOP:42, LOOP:106. OWNER. | Uncapped ledger chunk fallback. | ANC-012, MYC-REF-040, CF-007. Three-chunk refusal. | Owner changes the cap. |
| MERGE-WORK bound is 5,400 columns | LOOP:87, LOOP:106. OWNER. | Accepting the known 5,417-column total without resolving the bound. | SQG-013, SHP-008, LPC-002. B measures the complete kernel. | Owner changes the hard bound. |
| N-11 uses one ledger per program | LOOP:87. LEAD PICK confirmed. | Sharing one anchored ledger across distinct program commitments. | MYC-CLM-023, STL-001, MYC-REF-004. Wrong-program proof refused. | Program/deployment identity changes. |
| T23 refines N-11 to one ledger per contract/circuit R_SL program | MAP:74. LEAD PICK for T15. | One suite-wide instance. | STL-003, STL-001, ASI-002. Deployment identity matches `c_p`. | R_SL program identity changes from per-circuit scope. |
| Stage 1 consumes stock ZKIR. Stage 2 adds an internal compiler pass | `MYC/spec/specs/frontend/spec.md:3`, `:11`, `:63`. LEAD PICK for T15. Confirmation of this replacement of governing CF-001: NOT FOUND in LOOP or CHECK. | Requiring the `Lflattened` pass before Stage 1. | CF-001, MYC-FE-008, CF-003. Stage 2 equality against Stage 1. | ZKIR loses required semantics or stock compiler version changes. |
| R_SL uses micro-op AIRs instead of slice CORE DEC rows | `RSL/src/rsl/mod.rs:5`, REKEY:31. LEAD PICK for T15. | Direct R_SL rule rows in the 476-word slice CORE, which lack its requester state. | CF-005, RSL-002, CORE-011, MYC-CHIP-002. Rule refinement checks. | Micro-op coverage or universal shape changes. |
| SHA_IN remains present for non-hash apps | REKEY:33. LEAD PICK confirmed as part of the rekey list, according to COMPLETE RSL-002:23. | Shape variation that changes the leaf/N1 key. | RSL-002, RSL-001. Six/nine-AIR and pre-hash checks. | Kits or statement-binding AIRs change. |
| T24 includes keccak and secp256k1 now | LOOP:46. OWNER. | Earlier deferral of the three s02 calls to another chip set. | CF-013, RSL-004, RSL-005, ASI-003. All three full-chain results. | Owner changes the prototype scope. |
| Secp field elements use one 32-byte word | SECP:31. LEAD PICK for T15. | Four 64-bit words per field element. | CF-013, RSL-005, RSL-008. Above-Fr arithmetic and canonical-decode mutations. | Arithmetic representation or row modulus changes. |
| R_SL input commits to the whole tape | MAP:73. LEAD PICK for T15. | Public inputs only and `VAL_NONE`, both explicitly rejected in MAP. | AIF-002. Changed-private-tape proof refused. | A different input model preserves the required binding. |
| R_SL outcome is Halted([comm] ++ W) | MAP:72. LEAD PICK for T15. It applies the recorded FP3-1(C) outcome form. | A digest not tied to the native effect order. Other evaluated alternatives: NOT FOUND in MAP. | MYC-CHIP-004, STL-001. Changed/reordered effects refused. | Run outcome encoding changes. |
| Native-refused calls do not settle in T23 | MAP:75. LEAD PICK for T15. | Treating native refusal as an accepted Compact call. Generic Sherlock Rejected claims remain in scope. | MYC-LOW-005, MYC-CLM-007, ASI-001. Refusal accounting with no transaction. | A later protocol proves and settles explicit refusal claims. |
| One combined rekey precedes final T23 | LOOP:47, CHECK:4, CHECK:12. LEAD PICK confirmed through the owner-confirmed list recorded in COMPLETE RSL-002:23. | A separate R_SL rekey and ZK rekey. | RKY-001 through 005, STL-002, ASI-002. Final-key gates. | Another accepted change alters keyed constraints. |
| Preview redeploy follows T12 | LOOP:88. LEAD PICK confirmed under delegation at LOOP:83. | Declaring the T10 prototype-key deployment final. | STL-003, ASI-002. VK identity and deployment readback. | Final key identity changes. |
| Prototype design remains open, hard rules remain fixed | LOOP:101. OWNER. | Treating T12 as a permanent design freeze. | LPC-014, LPC-017, RKY-001. Re-run affected checks. | End-to-end prototype and owner interface sign-off. |

For T23 instance reuse, LOOP:45 records a LEAD PICK: reuse only when current state equals deploy-block state. C owns that decision's implementation. For the profile path `~/sherlock/setup/profile.json`, the same row records a LEAD PICK. B/C own profile discovery and validation. No additional A-only mechanism is specified.

## 6. Interfaces needed from slices B and C

The names below are proposed published-language types. Existing schema names retain their versions. New types are requests for integration, not claims that code implements them.

The tape's salt word and the eight-word salt in MAP's `input_d` construction are distinct fields. Do not merge their encodings. Sources: `MYC/compiler/src/sem.rs:716`, MAP:73. B defines their production randomness policy under ZK-013.

| Type or event | Producer → consumer | Required fields and invariants | Existing evidence or gap |
|---|---|---|---|
| `CompiledEntry` | A → B | source/build identity, compiler version, machine version, entry name, selector, typed instruction array, `N_e`, `L_e`, tape layout, output layout, kit IDs, image digest | `Program` and `Abi` exist at `MYC/compiler/src/lower.rs:40`. Product serialization is AIF-001. |
| `rsl-call/1` | A → execution adapter in B | schema, program identity reference, selector, `n_out`, `l_e`, `n_e`, tape length, instruction objects, private tape. Oracle words stay test-only metadata. | Existing emitted fields: `RSL/tools/mycroft-emit/src/main.rs:120`. Consumer schema validation is missing at `RSL/src/rsl/compile.rs:589`. |
| `ProgramAdmission` | B → A/C | machine version, image digest, plan digest, `key_root`, authenticated `prog_d`, app-key commitment, universal key-set ID, shape and kit admission result | R_SL program identity connection missing at MAP:82. No new B proof algorithm is prescribed. |
| `ExecutionClaim` | A ↔ B | authenticated program identity, input commitment, outcome tag and digest, dependency binding, statement version. Local opening carries tape and `[comm] ++ W`. | Slice FP-1 fields are at `RSL/src/core.rs:43`. R_SL binding is absent at `RSL/src/rsl/air.rs:217`. B defines the exact mapping into FP-1/FP-2. |
| `InputBindingProfile` | B → A | value encoding version, digest domains, canonical tape value encoding, fresh-salt width, fuel/stack encoding, INIT relation identifier | MAP:73 proposes tape vd and zero fuel/stack. B must resolve and implement it. |
| `ProofProfile` | B/C → A | machine version, complete key-set ID, kit set, `L_TX`, anchor slot, binding-layout ID, setup profile reference, qualified security/size gate receipts | A rejects mismatched profiles before proof dispatch. Gate internals belong to B/C. |
| `sherlock-profile/1` | B → A/C | profile version, host identity, resource budget, schedule, pipeline fields required by the public prover interface | Existing T23 consumer requires `wrap_tail_mib`, `evict`, `overlap`: `T23/tools/t23-suite/t23.py:73`. A needs profile identity, not GPU internals. |
| `sherlock-accept-bundle/1` | B local prover → local A/C mapper | `prog_d` plus private deployment-blinder reference or authenticated packed program operand, authenticated input opening, authenticated outcome opening, public lane values, proof/key identity | Existing host form at `ARGS/tools/t23-suite/accept_args.py:22`, `:287`. Private fields remain on owned hosts. AIF-002 and MYC-CHIP-004 supply the missing authentication. |
| `t23-prove-result/1` | B/C → A suite runner | job and call identity, schema version, four public words, ordered call indices, authenticated eff/comm comparison evidence, wrap proof path/hash, wrap VK hash, key-set ID, stock/ledger verification receipts, stage times, box/profile identity | Current request has only part of this set at `T23/tools/t23-suite/t23.py:282`. Add proof provenance and statement binding. |
| `DeploymentReady` | C → A/B | Preview network/version, contract address, deploy transaction/block, circuit/program identity, `c_p`, anchor slot, wrap VK hash, key-set ID, era identity, state-reuse check | One ledger per program: MAP:74. Actual final deployment receipt required by STL-003. |
| `AcceptRunRequest` | local A mapper → C | contract address, wrap proof, `c_p`, `input_pub`, `outcome_pub`, locally computed `claim_pub`, proof/key identity | `claim_pub` is a check value, not a fourth Compact argument. MAP:23, MAP:49. |
| `SettlementAccepted` | C → A | network/version, address, call/job ID, transaction hash, included block, `claim_pub`, `outcome_pub`, readback evidence, key/profile ID | Required by STL-004. A never treats submission alone as inclusion. |
| `NativeCallReference` | C Part A → A Part B | source/build pin, program/circuit/scenario/call index, disposition, ordered transcript words, comm-randomness reference, deployment identity, Preview receipt, normalization record | Native comparison distinguishes on-chain W from fresh-randomness comm: `T23/tools/t23-suite/t23.py:20`. No secret values enter public reports. |
| `CallRefused` | A/B/C → A report | job/call ID, stage, stable cause, requirement ID, offending entry/type/lane, evidence path, native comparison status, retry category | Named refusal stages exist at `MYC/compiler/src/pipeline.rs:125`. Cross-layer stable schema must be built. |
| `CollectionReceipt` | C → A/B consumers | checkpoint/epoch ID, root, claim membership proof, registry/era identity, inclusion receipt, availability status | Interface only. T20 batched-write policy is at LOOP:42. No `descend` algorithm is reintroduced. |

Proposed DDD boundaries:

| Context | Aggregate and invariant | Relation |
|---|---|---|
| Compact compilation, A | `CompiledEntry`: one source/configuration determines the typed program and ABI. | Native Compact is upstream. A owns the anti-corruption layer from native IR to Sherlock. |
| Sherlock execution, A | `ExecutionClaim`: one program/input execution determines its outcome and ordered effects. | Publishes meaning and versioned statements to B. |
| Proof production, B | B defines the proof aggregate and key admissibility invariants. | Consumes execution statements. Publishes proof evidence to C. |
| Midnight settlement, C | C defines deployment and claim-consumption invariants. | Ledger semantics are upstream. C owns the ledger adapter. |
| Conformance, A | `SuiteRun`: one pinned corpus, one identity per recorded call, no omitted result. | Consumes native, execution, proof and settlement evidence. It has no power to authorize a protocol change. |

Share only the agreed value and statement encoding definitions as a Shared Kernel after A/B/C agree on their ownership. An interchange JSON schema alone is a Published Language. This distinction follows ARCH-PROFILE:33. Domain events describe observations. They do not require a message broker or distributed transaction.

## 7. Risks and gaps

| Gap | Evidence, consequence and owner | Closure |
|---|---|---|
| R_SL result is not authenticated by recursion | `RSL/src/rsl/air.rs:199` says PUBLIC words are not app public values. The probe at `:217` carries zeros. A valid app proof cannot certify the claimed Compact outcome through that probe. A/B. | MYC-CHIP-004 with a real forged-outcome proof test. |
| R_SL input and program identity are not connected | MAP:81 through MAP:85 records missing `prog_d`, INIT and private prove-result parts. A host digest does not close these gaps. A/B. | MYC-CLM-023, AIF-002 and authenticated proof-result fields. |
| Producer and consumer file formats differ | `mycroft-call/1` emits a debug string at `MYC/compiler/src/bin/mycroft-suite.rs:112`. `rsl-call/1` emits objects at `RSL/tools/mycroft-emit/src/main.rs:122`. `RSL/src/rsl/compile.rs:622` expects an array. A. | AIF-001 and a single tested product serialization path. |
| Missing schema refusals | `RSL/src/rsl/compile.rs:592` uses unchecked JSON conversions. A malformed artifact can panic before a named refusal. A/B. | AIF-003 negative parser tests. |
| Governing CF-001 contradicts the current frontend route | FINAL frontend spec:4 requires `Lflattened`. `MYC/spec/specs/frontend/spec.md:11` specifies stock ZKIR. A. | Explicit CF-001 replacement and Stage 2 equality gate. |
| Existing MYC-CHIP requirements describe another design | `MYC/spec/specs/chipset/spec.md:12` requires DEC rows. R_SL uses micro-op AIRs at `RSL/src/rsl/mod.rs:5`. `MYC-CHIP-004` requires 1,026 public values while FP-1 has 54. A/B. | Reconcile MYC-CHIP-001/004/006/008 with CF-005, RSL-001/002, CPR-003 and STL-003. |
| Key wording conflicts | MYC-REF-052 bans per-program preprocessing keys at `MYC/spec/specs/refusals/spec.md:498`. R_SL app keys are per program at `RSL/src/rsl/mod.rs:13`. TERMS:36 and CPR-003 permit authenticated per-program keys below universal keys. A/B. | Preserve the universal-key rule and amend the overbroad refusal. |
| Secp audit status is stale | COMPLETE CF-013:12 says all non-BLS inputs are refused. `MYC/compiler/src/zkir.rs:568` now handles secp inputs. A. | Keep CF-013, replace its status with current code evidence and final-chain gap. |
| Kit deferral is stale in the runner | `T23/tools/t23-suite/t23.py:146` describes secp/keccak deferral. `MYC/compiler/SUITE-COVERAGE.md:120` repeats it. LOOP:46 includes both kits now. A. | Refresh enabled kit selection, rerun corpus and remove deferral as a completion exemption. |
| Mixed SHA and Keccak programs are refused | `RSL/src/rsl/compile.rs:847`, `RSL/tests/rsl_keccak.rs:150`. This is a capacity/design gap against GP-1, even if current suite calls avoid it. A/B. | Preserve the gap until a universal-key-compatible path exists. Do not redefine supported Compact to hide it. |
| Wide Jubjub inputs remain incomplete | `MYC/compiler/src/zkir.rs:568` accepts native and secp input paths, then refuses other forms at `:582`. `MYC/lean/SORRIES.md` records unwired wide arguments. s01 internal curve operations do not establish j02 point-input coverage. A. | Wire typed input expansion and the wide-argument proofs. Run both j02 calls. |
| Slot-15 and direct entry binding need integration | `MYC/factory/REPORT.md:111` lists the slot refusal as open. `RSL/src/rsl/compile.rs` and `MYC/compiler/src/lower.rs` contain no complete placement-V binding path found by this audit. A/C. | ANC-015, CF-008, CF-009 and direct-call mutations. |
| Ordinary effect equality does not prove contract-state equivalence | MAP:45 distinguishes effect-carrier mode from general acceptRun. T23 acceptance records a run outcome in SherlockLedger. It does not establish that every original contract effect was applied there. A/C. | State this claim boundary in the suite report. Use direct Compact settlement tests for ledger-effect equivalence. |
| Zero mismatch is currently vacuous after proof | Dry-run report: zero passes, 145 blocked, one pure, 62 derived refusals, zero proved comparisons. `T23/research/sherlock-t23-suite-20261115/T23-DRYRUN.md:11`, `:21`. A. | ASI-002 and ASI-003. |
| Native oracle has its own gaps | Dry-run report:47 calls lack a passing native Preview transcript, at `T23/research/sherlock-t23-suite-20261115/T23-DRYRUN.md:34`. Earlier native results include address substitutions, LOOP:40. A/C. | Require Part A evidence or an explicit unresolved classification. Do not replace Preview evidence with host equality. |
| 142 vectors, 142 calls and 208 recorded calls are different counters | `MYC/compiler/SUITE-COVERAGE.md:5`, `:119`, REKEY:30 and LOOP:45 use different run scopes. A. | ASI-001 stores scenario/call/circuit/vector identity and preserves each denominator. |
| Full interpreter proof coverage is absent | `RSL/src/dec.rs:23` has five subset rules and `:40` has two value rules. MOD RFB-003 requires 120-rule coverage. A/B. | Full rule-to-reference map and proof cases. T6's subset chain is insufficient. |
| Interpreter status needs per-requirement evidence | UI spec:5 calls all scenarios unexecuted. Its adoption note at `UI/specs/universal-interpreter/spec.md:260` records later scoped qualification without requirement closure. Host tests exist at `ROOT/crates/mori-core/tests/sherlock_original_ears_host_positive.rs:196`, `:315`. They do not establish all UIREQ obligations or proof coverage. A. | Reconcile the introductory status, adopted refinements and current tests without claiming full closure. |
| Formal lowering does not cover the whole chain | `MYC/lean/SORRIES.md:13` keeps S1 limitations and S2's seal/run premise. Builtins remain parameters. Secp layout and wide arguments are not fully modeled there. A/B. | LPC-019, separate kit parity, Rust/Lean conformance and AIR refinement. |
| Rekey list is incomplete as a closure record | REKEY:7 calls C3.1 uncommitted, while AUDIT “Missing from the checklist” records `95990a6d3`. AUDIT also flags omitted FP-3, slot 15 and canonical-range items. B. | RKY-001/002 with a current complete manifest and all affected-key gates. |
| Probe evidence does not close final-key security | REKEY:33 reports five pre-hash probes on `6f773b2`. Later typed and curve changes are listed at REKEY:35, `:36`. A/B. | RSL-001/002 and T25/T12 reruns on one exact key set. |
| No complete user command run found | FINAL CF-002 specifies commands. Inspected Mycroft and T23 sources provide separate research tools. A/C. | Build the versioned public command chain and `compact_to_preview_user_flow`. |

Requirements whose current verification is not possible:

- A real end-to-end test of AIF-002 or MYC-CHIP-004 cannot pass through the zero-valued probe. Build authenticated R_SL statements first.
- A test of “every call settles” cannot apply to native-refused calls or pure exports. The stated T23 dispositions exclude those calls from settlement. Preserve them in the denominator.
- A test of “all Compact contracts apply their effects through acceptRun” has no defined state-transition oracle. acceptRun certifies an outcome and writes its runs entry. Separate direct-call semantics first.
- A finite corpus cannot prove GP-1 through GP-4 for every program and shape. Use formal arguments, rule refinement and targeted counterexamples. Keep their assumptions explicit.
- No single “142 calls” gate can close the 208-call corpus without an identity map. Build the map before using aggregate counts as acceptance evidence.

No new soundness verdict follows from this draft. It identifies the requirements, evidence and missing connections that the integrated specification must preserve.
