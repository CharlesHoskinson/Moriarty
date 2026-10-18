# Finite independent financial observations through the compiled relation

GPT-6.1 Sol high source author. Source-only candidate; actual Rust/SDK/IR/relation checks, designated-owner assessment and independent source/result audits are UNRUN. The patch adds one integration test source with two positive test functions. A separate narrow RUST-GUIDELINES patch records the proposed literal-oracle exception to snapshot guidance. No source, original proposal, freeze, checkout or Git state was changed.

## Actual consumer and next predicate

The real public route is `mori_core::parse` → `mori_backend::compile` → emitted `CircuitArtifact.zkir` → `midnight_zkir_v3::IrSource::load` → ordinary `ProofPreimage::check`. `mori-backend/src/lib.rs:367–425` validates and compiles all actions; `:1232–1237` already serializes/reloads IR inside compilation. Existing real consumers are `mori-backend/tests/compile.rs` and `tests/liabilities.rs`; both construct public transcripts and call the same SDK check, but derive expected financial observations from `simulate`. This draft supplies frozen independent observations instead. It does not call `simulate`, reconstruct/prepare helpers, a new lowerer, or any proving/VM interface.

Predicates: the exact invoice whole Program compiles and its collect relation accepts each of the frozen collect47 and linked collect80 financial observations; the exact contrasting reserve whole Program compiles and its allocate relation accepts the frozen amount50/bondAmount7 observation. The source also checks exact full parsed Program equality against the independent Program DTO files. It does not construct expected IR or a complete Artifact golden.

The test Target inputs remain the previously proposed `Target::ledger9("local", "financial-invoice-compiler-test")` and `...financial-reserve-compiler-test`, with existing constructor defaults unchanged. These are local test-domain inputs requiring root's domain review, not selected deployments or measured resource bounds. No signing/context rule is changed. No StaticBindings/Deployment object is introduced into this backend API.

## Independent inputs and exact custody

The complete financial freeze is `5fca1721249e603226df21d1a2d7ad886a27e66232d1b722dfd76011611d510d`. All 32 original payload hashes and their exact existing copies in `crates/fixtures/financial-report-contract/` were checked before preparing this draft. The complete derivation/provenance and source values were read. The candidate reuses those current fixtures directly; it changes or copies none. `SOURCE-INVENTORY.json` pins every consumed fixture and source basis.

The custodian's expectations are post-adoption/public-DTO/source-derived, not blinded or preimplementation. This test author has inspected the compiler. The expected values come exclusively from the frozen Program/Scenario/Execution DTOs; the adapter does not evaluate expressions, compute balances, execute statements or copy any compiler-produced expected result. Public `v1::Program`/`v1::Execution` decoding and their existing `From` conversions are used because the shared Program/Execution types are Serialize-only (`mori-core/src/ir.rs:177–187,253–271,324–452`). The freeze's complete typed values and ordering are retained, not rewritten.

## Exact semantic/representation crosswalk

The draft uses the existing role/name/typed-field ABI contract and the same original honest unsigned preimage shape as the existing compile/liability tests: empty inputs/private transcript, identical public transcript inputs/outputs, binding_input zero, no communications commitment and an explicitly unqualified key-location label. No key is read or generated. The public adapter is new test-only code; it must receive designated-owner review and does not inherit approval from the production adapter or earlier compiler-only proposal.

| Frozen observation | Actual consumer comparison |
| --- | --- |
| Entire source Program and action order | Actual parse equals full frozen Program, including all leaves/edges and uninvoked register action. Artifact circuits equal both source action names in order. |
| All state fields, affected and preserved | Both typed pre_state and post_state bindings include every frozen declared state in declaration order. Their transcript values come from complete independent pre/post snapshots, including marker77. |
| All declared action inputs | Exact input names/types/order are checked for register and collect/allocate. Selected amounts, day, approval and bondAmount are read from frozen Scenarios. |
| All registered duty identities | Each action's complete liability frame equals the frozen closing identity list, including invoice/warranty and allocation/bond/emergency, with all four exact identity members. Descriptor positions must point to correct typed role/name/asset bindings. |
| Duty presence versus amount | Independent pre/post and per-effect snapshots supply presence separately from amount. An absent allocation slot is false/0; closing allocation/invoice present-zero stays true/0. Unchanged warranty9/emergency2 and residual invoice80/bond3 are included. |
| Touched balances | Exact asset/account set is derived from independently frozen Transfer/Fee effects and compared with the complete actual balance frame, rejecting duplicate frame keys. Each descriptor's pre/post position must reference the correct alias/account role, UInt and complete AssetId; its values come from frozen snapshots. |
| Ordered effects | Every actual effect descriptor is zipped with the frozen flattened Step.effects in source order. The test compares step/kind/name/asset/from/to/liability/operation and exact field arity. Each referenced position must name the exact step.field, type and asset. State before/after, transfer/fee amount, emitted value and duty before/after presence/amount all receive frozen values. |
| Positional completeness for selected actions | Each financial ABI slot is assigned once, must match its descriptor and type/identity, and every actual slot must be assigned before checking. There is no fallback zero or skipped unknown role. Physical position numbers come from emitted descriptors and are checked referentially, not independently frozen as a complete positional layout. |
| Uninvoked register | Both register actions compile; their state/input/duty metadata and complete declaration-effect identity/type/position crosswalk are checked against frozen Program/source identities. Their full expected public slot count is checked. No register Scenario/Execution exists in the freeze, so no register relation observation or expected numerical result is invented. |

Context constants at positions0/1 remain compiler-supplied original context values, explicitly distinct from independent financial expected values. Their representation/shape is checked under the existing adapter; this draft does not independently derive or validate program/target/action/context digests. The finite asset-alias uniqueness premise is asserted for these specimens rather than generalized silently.

## Preservation omitted by the actual backend frame

The backend explicitly documents its touched-frame boundary at `lib.rs:363–365` and gathers only Transfer/Fee endpoints at `:820–861`. Invoice observer CASH23 and reserve CASH19 have no balance-frame slots. Reserve observer BOND7 and observer CASH11 likewise have no slots. The candidate names these four omitted rows explicitly, checks that the omitted set is exactly those identities and retains their complete equal frozen pre/post rows. Those assertions check oracle/frame scope, not compiled preservation of omitted accounts. A ledger adapter's actual preservation of these rows remains unobserved and required.

All declared scalar state is represented, so marker77 receives both actual ABI positions and independent values. All source-registry duties are represented, so unchanged warranty9/emergency2 and present-zero/residual duties receive both presence and amount slots. No off-frame state/duty is dropped by this adapter. Source constants/initializers and unused expression nodes are in the full Program comparison but are not extra opening execution values or public ABI slots.

The invoice post→next pre equality is equality of the two independently frozen full snapshots. Returned Execution objects remain literal expectations; they are not compiled runtime readback. The test separately checks each complete transcript against its actual emitted relation. This supplies no authenticated persistence or two-step ledger execution.

## Exact limits and retained full-field gaps

This is a distinct positive relation-observation predicate. It does not weaken, replace or rename `installment_invoice_matches_complete_compiler_oracle` / `conditional_reserve_matches_complete_compiler_oracle` in the existing compiler-financial-test-source-proposal. That proposal's four absent complete target/artifact fixtures remain absent; its exact whole-Artifact and reopened expected-IR equality remains unobserved.

There is no independent exact IR instruction/constant golden, program/context digest, artifact byte string, instruction/resource count, complete physical ABI ordering/position golden or Gate expectation here. The test checks declared artifact format/ABI/relation/mode and frozen semantics, source-wide identity metadata and all selected financial slots; it does not compare every Artifact/CircuitArtifact/Resources/Gate field. Circuit instruction_count/resources/gates and target/digest correspondence retain their original separate obligations.

Passing future SDK check calls would be positive functional evidence that these independently specified finite public transcripts are admitted by the emitted relation. No rejection, soundness, authorization, cryptographic, proof or hostile-witness conclusion is supplied. This positive scope does not establish all valid inputs, absence of invalid inputs, full compiler correspondence, compiled per-statement state readback, Step.value or logical-work correctness, resource/native feasibility, full U2, finance-family conformance, composition, target or release acceptance.

## Verification and first unrun boundary

Performed only source reads, text construction and plain byte/hash/data checks. Exactly two test functions are authored; all fixture paths resolve under crates/. No production file, dependency, source remount, ignored test, runner or scheduler is added. The source needs root's mechanical formatting and fresh exact-candidate audits.

No parser, Rust/Cargo/rustfmt/build, SDK, IR loader, relation checker, VM, proof, key/wallet or network call was executed. Actual `compile` already invokes SDK IR construction/serialization/load, and this new source additionally invokes `ProofPreimage::check`. The existing compiler-only command proposal explicitly excludes relation checks and CANNOT be reused as execution authority for this candidate. Both first compiler/SDK use and the subsequent relation-check path remain UNRUN pending the designated owner's source/security/resource review and a separately inspected scoped command. Source preparation grants no execution authority and changes no counters or caps.

Every original release/U0–U7/M1–M17/MC/SP/G, full finance/ACTUS/DeFi, composition, UTXO, staged/private/recursive/native/target exit remains unchanged. All original freezes, failures, resource counters and proposals remain intact.
