# Source proposal: one actual compiler-positive local check

Status: **SOURCE-ONLY / PROPOSED / UNRUN / NOT SECURITY-CLASSIFIED OR ADMITTED**. GPT-6.1 Sol high, nonsecurity analysis/preparation, 2026-10-18. No test result, compiler result, execution authority or parent acceptance is claimed.

## Concrete finding and exact proposed selection

An existing individually selectable positive test does invoke the genuine production compiler on two contrasting source-authored programs:

`crates/mori-backend/tests/stages.rs:374::legacy_counter_abi_and_payment_effects_match_existing_frozen_snapshots`.

It parses `tests/programs/counter.mori` (IndependentCounter, two actions) and `tests/programs/payment.mori` (IndependentPayment, one action), calls `mori_backend::compile` twice, and compares the resulting Counter public ABI role/name sequence and Payment effect frame against existing frozen snapshot text. Production lowering constructs actual pinned SDK instructions and invokes the pinned SDK IR loader once per emitted circuit. The selected case therefore reaches the actual general source->IR->ZKIR compiler, not an Artifact DTO, fixture dispatcher or equality between two wrappers.

**One exact proposed command, not executed or authorized here:**

```sh
cargo test --frozen -j 1 -p mori-backend --test stages legacy_counter_abi_and_payment_effects_match_existing_frozen_snapshots -- --exact --test-threads=1
```

`--test stages` compiles that integration target and its required library/dependencies; the exact test filter selects this one positive runtime case. It does not run the other stage tests or the ignored baseline-fixture test. `-j 1` and one test thread bound selection/concurrency; they are not measured physical memory, time or native-resource limits. No wrapper, retries, parameter/cache preparation, fixture generation, snapshot acceptance/update, new budget or cap change is proposed. The existing target constructor's instruction/byte ceilings remain unchanged. Owner-defined existing controls must still govern any future invocation.

This is the smallest existing useful production-compiler check identified in the inspected paths. It is **partial compiler structure/serialization evidence**, not a complete general-compiler qualification test. The programs were authored historically; they are not newly authored in this session. No existing test inspected simultaneously provides fresh non-fixture programs, independently specified complete compiler output/grammar coverage, complete financial/work observations and absence of SDK relation/model operations. That stronger predicate remains a concrete missing test/evidence dependency.

## Exact in-repository call and data path

| Call / source | Selected normal path and data |
|---|---|
| `tests/stages.rs:374` | Two compile-time embedded source strings and two embedded snapshot strings. No selected-case environment-variable or filesystem access. Parses Counter then compiles it, formats selected ABI metadata and compares frozen text; parses Payment then compiles and compares its full effect-frame debug text. Standard test harness output only. |
| `mori_core::parse` | Public lexer/parser builds ordinary Core/1 Program, then validates it. Both source files use the same language/semantics path. No evaluator/scenario supplies the Program. |
| `Target::ledger9` | Counter target network `local`, anchor `counter-anchor`; Payment target network `local`, anchor `payment-anchor`. Both are unsigned ledger-9, exact ledger revision `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`, ZKIR `3.0.0-rc.2`, max_instructions100000, max_artifact_bytes16777216. Anchors differ and are local test labels; no shared deployed target tuple is demonstrated. |
| `lib.rs:367 compile` -> `compile_mode` | Ordinary profile check delegates through backend stage::require_core1 to Core stage/profile.rs (identity plus statement-profile scan); exact target validation; complete Core IR validation; exact asset network check; unsigned path sets signed layout None; builds source-wide liability registry; canonical Program serialization and SHA256; loops through every action. Both specimens have empty liability registry. |
| `lower_action` -> `lower_action_segment(...,stage=None)` | Real `Builder` emits SDK instruction enums, public transcript ABI, checked arithmetic/range/boolean constraints, ordered effects, pre/post state and touched balance frames. Counter produces increment/reset circuits; Payment produces pay. No application-specific compiler is selected. |
| `signed_relation::context_hash` in unsigned lowering | Builds the canonical 18-element context array, including source Program, target/action/frame/ABI and fixed identity labels; hashes via canonical JSON/SHA256. The module name does not mean this selected unsigned path verifies a signature. The signed layout append branch is not selected. |
| `authorization::digest_limbs` / `immediate` | Splits hash bytes into two u128 values and constructs SDK field constants via Fr conversion. This uses SDK field/type code; it is not SDK-free. |
| `lib.rs:1225..1240` | Constructs genuine `IrSource` with instructions, empty private inputs, declared outputs and communications commitment disabled; serializes SDK IR, sets wire version major3/minor0, creates actual ZKIR JSON, calls `IrSource::load(zkir.as_bytes())`, compares loaded IR to the constructed IR. This is the exact external SDK boundary requiring owner inspection. The source count is three generated circuits and therefore three loader calls on normal completion. |
| `compile_mode` finalization | Aggregates emitted counts/bytes/public values/max source replay steps, serializes complete Artifact envelope to stabilize its own byte count within at most12 iterations, enforcing the unchanged target caps. Sets k/fraud-k None, cost_measured false and six gates `unperformed`. Native proof/cost/ledger gates remain explicit. |
| Test assertions | Checks all Counter action names and public role/name positions against the Counter snapshot, and Payment effect descriptors against its snapshot. Compiler internal loader/round-trip assertions supply actual emitted-IR parse/equality observations if the run eventually passes. The test does not independently compare every instruction, public type/constant, identity/hash, resource field or full Artifact envelope to a separate complete oracle. |

`proving::hex`, used for the context label during lowering, is only a re-export of `mori_core::intent::hex` at proving.rs:225; the selected path does not call a proof-producing function merely because that module supplies the alias.

The public CLI's Core1 compile branch calls `source_projection::compile_reported`, which dispatches to the same unsigned `compile` or signed `compile_signed` based on Target, then adds bounded source-projection metadata. CLI serialization/publication creates a new output directory and writes actual circuit files, projection and completion manifest. The proposed library test reaches the shared compiler but **does not establish CLI dispatch, publication, projection or separate-process reopen behavior**. Those require their own public consumer observations. It is intentionally not a claim about all features of `mori compile`.

The PR191 head identity supplied by root is `0c221ca846b331f709877a878671db569999b2dd`, tree `518ed8c4aca3babc7682d862a70079cc8aa99a7b`. Root identifies its publication tests as frozen DTO-based tests, intentionally native-linked/unrun and outside compiler qualification. It was not needed for this selection and its local Git object/source was not queried. Do not transfer its proposed publication-test results into this compiler predicate.

## Alternative existing tests: why this single case is selected

- `tests/compile.rs::emits_real_upstream_ir_with_public_pre_and_post_state` compiles a manually assembled single Program and parses real emitted IR, but it has neither source parsing nor the second contrasting source program.
- `tests/compile.rs::authored_counter_relations_match_reference_and_refuse_changed_state` and `authored_payment_constrains_complete_balance_and_fee_effects` use authored source and real compiler, but then call Core simulate and `ProofPreimage::check`. They do not meet the requested compile-only path; no such command is proposed.
- `source_projection::tests::reported_compile_preserves_target_artifact_and_contains_projection` compiles two contrasting expression programs but focuses on old/reported wrapper equality. It does not independently qualify production lowering or complete financial behavior.
- The ignored `legacy_artifact_and_every_ir_file_match_baseline_bytes` needs an external baseline directory and separately generated baseline fixtures. It is one supplied program, has runtime file/env effects and missing baseline authority/provenance requirements. It is not selected or activated.

The existing stage-target name is misleading if interpreted as requiring staged execution for every test: the selected `legacy_counter...` case calls ordinary compile directly. Conversely its presence in a structural target does not settle transitive SDK behavior.

## Full known effects inventory and unresolved transitive boundary

This is a descriptive source inventory for the designated security owner, **not a security classification, run admission or promise of no native effects**.

| Phase | Source-grounded effects and remaining inspection |
|---|---|
| Invocation/toolchain | Cargo entry may first pass through the toolchain manager pinned by `rust-toolchain.toml` to1.99.0. `--frozen` constrains Cargo lock/network behavior, but does not prove behavior of a preceding wrapper/toolchain installation. Installed tools/configuration were not inspected; no command is admitted based on assumed availability. Missing tooling/dependencies must be a terminal observed limitation, not automatic provisioning or warming. |
| Cargo build selection | Reads workspace/package manifests, immutable lock and selected source; may use normal Cargo metadata/download/source/build caches, create build locks and target artifacts, invoke rustc/linker/proc-macros/dependency build scripts/native compiler tools. No cache contents, private configuration or external dependency source were read here. Cargo was not run. No target-directory/environment override is supplied to redirect or expand work. |
| Transitive dependency graph | Backend manifest directly links Core, SDK ZKIR/transient crypto/curves, serde/JSON/SHA/thiserror/futures/rand and Unix libc, plus insta/proptest dev deps. ZKIR's lock entry transitively includes midnight-base-crypto, midnight-circuits7.2.4, midnight-proofs0.8.3, midnight-serialize, midnight-zk-stdlib and further dependencies. The adjacent conservative lock inventory lists every reachable lock package/edge, including platform/dev edges. It is not a Cargo-resolved target/feature graph and cannot establish which build scripts or native code execute. |
| Backend compilation | The complete backend library is compiled, including proving/parameter/signed/Sherlock modules and their dependencies, even though their runtime operations are not selected by this integration test. Compiling this target is not an SDK-free frontend check. No crate-local build.rs was found in the backend file inventory; dependency build scripts remain uninspected. |
| Test input/observations | Source and snapshot bytes are embedded with include_str at compile time; selected test uses memory allocation, deterministic parsing, source validation, hashing, string/JSON serialization and comparisons. No selected-case file create/read/write API, environment capture hook, subprocess, randomness, signer, prover parameter path or installed ledger state is present in the inspected test body. Test harness/Cargo stdout/stderr/build outputs remain ordinary process effects. |
| Compile runtime | Emits SDK types/field constants and in-memory artifact/ZKIR bytes; invokes SDK Serialize/PartialEq and IrSource::load on actual output. In the inspected repository call path there is no explicit .model(), .k(), .check(), keygen, proof, SRS, VM/transcript execution, network client, cache read/write or parameter loader. That source fact is not a claim that the external loader/types have no such transitive effects. |
| SDK/build-script boundary | Exact pinned upstream IrSource::load, SDK serializers/type operations, trait equality, field conversion and each selected dependency build script/proc-macro/native link step must be inspected by the designated owner before dependent execution. Their implementation source was not accessed via cache/network. This is the concrete unresolved transitive-source dependency; the owner inventory cannot be marked complete or execution-ready here. |
| Limits/termination | Existing target output/instruction bounds and source bounds remain. Three action circuits, two whole-program compiles and at most12 artifact-size fixed-point iterations per compile are source-bounded logical operations; no measured physical/native budget is inferred. Preserve every recorded resource cap, debit/counter, failed native attempt and recovery rule. No automatic retry, cap raise, proof parameter fetch, cache warmup or new campaign is proposed. |

Required owner decisions are specific: classify the exact build/runtime dependency closure after external source inspection; reconcile existing resource limits with that exact selection; name the admitted command/candidate if any; and define any independently owned execution observations. A timeout, unknown SDK effect or missing dependency is not approval. This document makes none of those decisions.

## Independent normal financial/work expectation needs before acceptance

The compiler consumes no Scenario and does not itself execute balances. Therefore the proposed compile-only test cannot demonstrate the following financial outcomes merely by compiling or matching frames. Before any corresponding normal behavior/constraint acceptance run, an independent custodian must freeze complete typed pre/post snapshots, every statement/expression observation, ordered effects and logical work from source semantics, together with exact asset/recipient identities and numeric assumptions. No full compiler output golden may be generated by calling the compiler under audit.

The following arithmetic worksheet fixes concrete positive specimen inputs and needs. It is manually derived from the source statements, not an evaluator/compiler-generated expected output; this source-inspecting author is not claiming a blinded independent oracle or accepted freeze.

| Specimen/action | Proposed positive input/prestate | Complete outcome needs | Reference logical-work derivation |
|---|---|---|---|
| Counter/increment | count5, enabledtrue; amount7; no balances/duties | Requires enabledtrue; capture total12; total<=1000 true; ordered State count5->12 then Emit new_count UInt12; final count12/enabledtrue; all absent balance/duty frames stay empty | Six statement increments2+4+4+2+4+2=18: each statement1 plus its uncached expression-node visits |
| Counter/reset | count5, enabledtrue; no inputs; no balances/duties | One State count5->0; final count0/enabledtrue; no event; empty balances/duties preserved | One Next with literal RHS:1+1=2 |
| Payment/pay | paid0; amount10, acceptedtrue; exact USD `(local,issuer,usd,scale2)` balances payer40/recipient3/sponsor0; duties[] | Requires acceptedtrue and amount>0 true; Transfer10 payer->recipient; Fee2 payer->sponsor; State paid0->10; Emit receipt UInt10, in that order. Final payer28/recipient13/sponsor2, paid10, duties[]. Gross payer debit12 including fee2; recipient net increase10; conserved token total43; no reserve/supply/duty creation or discharge. Here account name sponsor is the fee recipient, not an authenticated sponsorship claim | Six statement increments2+4+2+2+4+2=16 under the same primitive schedule |

Every UInt amount is integral atomic units; asset scale does not rescale the expression amount10. These reference-work counts are not compiler instruction counts, circuit k, wall-clock/allocations, DUST or native fees. At compile-only evidence stage, preserve cost_measuredfalse/null k and all unperformed gates rather than substituting these numbers for native measurements.

Independent artifact expectations still missing beyond existing snapshots: exact complete public input types/role/name/asset/constant mappings, every ordered effect and balance/state/duty frame, source action ordering and expression coverage, actual emitted instruction obligations/range constraints, program/target/context identity mapping, pin/version meanings and resource-envelope relations. Derive these independently at rule level and freeze before a claimed correspondence comparison. Do not invent complete ZKIR byte strings, context hashes or whole-artifact oracles from this worksheet. Existing snapshot provenance is historical compiler-test evidence, not established independent full instruction semantics.

## Evidence and unchanged original U0/U1/U2 predicates

If the exact command is separately admitted and eventually succeeds, report only: two identified historical source programs parsed and compiled through production compile into three actual SDK-parsed pinned IR circuits; selected Counter ABI and Payment effect metadata match frozen snapshots; exact source/lock/test output identities and any first blocked boundary. A passing test name is not the output identity or an acceptance receipt. A failure preserves its exact result; no automatic retry or substitution.

Remaining U0: current source/Core/numeric and target matrix, complete field/rule enforcement map, semantic/compiler/ABI pins, exact trust premises, K reconciliation and actual verifier/key/ledger tuple evidence. Local anchor labels here supply none of the missing live target identities.

Remaining U1: certified arithmetic/required primitive basis, valid-witness completeness and designated-owner adversarial soundness evidence, caller preconditions, logical/target cost relation and measured native signature/phase/recursive feasibility under retained limits. SDK IR parse and host metadata equality are not those certificates.

Remaining U2: newly authored and structurally contrasting programs through the complete claimed pipeline; canonical signed intention; actual pinned source-to-ZKIR correspondence across admitted grammar; real native proof/independent verification and complete authenticated effect readback; required refinement/transition/history/replay/failure properties. This unsigned historical pair is a compiler-structure child only. The full financial/grammar/native/target and public CLI publication predicates remain open.

Retain full U0–U7, M1–M17, MC01–MC08, SP01–SP12, G01–G24 and all ACTUS/DeFi/composition/release membership and prior receipts. Preview/stagenet/Fenrir evidence remains target-specific; no Fenrir identity/version is invented. No new compiler framework, DTO replacement, mock artifact or permission requirement is introduced.

## Source/evidence provenance

Checkout: `/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017`; root-assigned context branch native-availability-tests, base63c38d05, tree prefix23a plus one uncommitted security test not read/edited. No Git query or mutation was needed. Exact inspected public file byte pins are in SOURCE-INVENTORY.json; no whole dirty-tree identity is invented.

Develop startup/AGENTS/product contract were restored; read-only development status exited0 and retains historical SP01 stale binding/candidate/accounting/resource blockers. No Cargo/rustc/rustfmt/Mori/parser/evaluator/model/SDK/VM/proof/key/wallet/network execution occurred. No public cache/private/prover-parameter/provider-config/auth source was read. All outputs are outside Git in this proposal directory. No source/test draft was needed because the bounded existing actual-compiler selection was found; the stronger missing independent qualification obligations are stated rather than replaced with generated DTOs.

Immutable lock SHA256: `659ca9ca7df02e4ff4e3c2b424fa2d6b95d73e596fdf85d0046df423ced4dbb5`. Conservative lock reachability has 381 package identities and 1013 edges; unresolved references: 0. These counts are source inventory only, not actual build selection or resource measurements.
