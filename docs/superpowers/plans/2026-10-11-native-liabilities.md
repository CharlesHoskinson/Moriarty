# Native persistent liability implementation plan

> **For agentic workers:** Use superpowers:subagent-driven-development task-by-task with a fresh implementer, independent task review and the required final Sol HIGH/Grok HIGH pair.

**Goal:** Compile existing typed liability accrual/discharge and preserve every supported duty through the actual local native proof consumer.

**Architecture:** A program-wide source-derived registry defines the finite complete liability ABI for every action. Existing checked arithmetic and public Impact bindings constrain presence, amounts and ordered effects. The actual proof library reconstructs that same ABI from independently simulated snapshots; no alternate evaluator or application compiler is added.

**Tech Stack:** Existing Rust workspace, pinned ledger-9 ZKIRv3, existing bounded Core JSON/canonicalization and Quint QA.

## Global constraints

- Moriarty source only; external Fenrir repositories read-only.
- PR25 is merged at60b3657a5a840c07cf82b7d59b3e6ed3a8b67bb1 with exact reviewed tree d8d5b4459d7ce921b1ff52700377532f96441bf7; both independent next design votes APPROVE. Preserve both frozen histories.
- Existing source/interpreter semantics unchanged; supported compiler subset restricts identity closure explicitly.
- Registry keys are unique by name with immutable exact asset/debtor/creditor tuple; maximum16 identities, refused before inserting17th or allocating circuit frames.
- Every action preserves every registry slot; absent is presencefalse/amount0, present0 remains distinct and consumes capacity. Discharge never implies payment or erases a slot.
- Explicit artifact `moriarty-zkir-artifact/2`, ABI `moriarty-public-abi/2`, relation `moriarty-zkir-relation/2`, proof `moriarty-native-transition-proof/2`, binding `moriarty-native-transition-binding/2`; bind version/profile/registry in native circuit/claim context, reject version1 artifacts/proofs in the version2 consumer, preserve historical version1 evidence. Existing source recompiles under version2.
- No new native proof/SRS/keygen/verifier experiment until exact candidate circuits are measured and separately authorized by both selected resource votes. Prior one-payment allocation is consumed.
- Signed authority, actual predecessor/history/completeness, ledger phases, recovery coordinator, recursive/private history and full roadmap gates remain open.

### Task 1: Fixed registry and complete liability circuit frame

**Files:** Create `crates/mori-backend/src/liabilities.rs`, `crates/mori-backend/tests/liabilities.rs`; modify `crates/mori-backend/src/lib.rs` and affected ordinary compile tests. Keep shared manifests/dependencies unchanged.

**Interfaces:**
```rust
pub const MAX_LIABILITY_REGISTRY_ENTRIES: usize = 16;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LiabilityIdentity {
    pub name: String,
    pub asset: mori_core::AssetId,
    pub debtor: String,
    pub creditor: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiabilityDescriptor {
    pub name: String,
    pub asset: mori_core::AssetId,
    pub debtor: String,
    pub creditor: String,
    pub pre_presence_position: u32,
    pub pre_amount_position: u32,
    pub post_presence_position: u32,
    pub post_amount_position: u32,
}
// Private module: validate existing literal identity closure, return sorted slots.
pub(crate) fn registry(program: &mori_core::Program)
    -> Result<Vec<LiabilityIdentity>, crate::Error>;
```
`CircuitArtifact` gains trusted `liability_frame`; `Artifact` explicitly identifies ABI2; resources record registry slots and profile ceiling. New artifact2 types reject unknown fields and stale profile/version decoding; full typed effect identity and create_or_accrue/discharge operation metadata must be explicit, without repurposing balance transfer fields. No Target schema change or padded16-slot ABI. Introduce named version constants, not unrelated numeric magic strings. Same registry is passed into every `lower_action` invocation.

- [ ] Add failing compile tests with independently authored source: two identities declared across creation/repayment/untouched actions; inspect every action's sorted full frame and exact source identities. Assert absent/zero presence and amount positional roles/types/constants and contextual revision. Existing compiler must fail unsupported before implementation.
- [ ] Implement bounded registry construction using a BTreeMap keyed by name. For each Liability literal resolve the declared exact asset, compare an existing full tuple, refuse conflicts; refuse the17th distinct name before insertion. Then traverse every Discharge and refuse unresolved names. Return map values sorted by name. No scenario-derived registry or new parser grammar.
- [ ] Build opening public Bool presence and UInt amount for all entries. For each opening constrain `(1-p)*amount=0` with existing checked Boolean/field instructions and unconditional public Impact. The amount is range128 and p is Bool. Store sequential wires keyed by registry name.
- [ ] Liability statements: expression delta is checked UInt; capture before `(p,a)`, checked-add amount, set presence literaltrue. Discharge: Assert current presence, checked-sub delta, preserve presence. Bind exact before/after presence+amount effect positions and source operation kind/identity. All existing ordered balance/state/event behavior remains.
- [ ] Publish closing presence/amount for every slot with equality to final sequential wires and canonical absent amount constraint. Include all closing wires in actual output shape. Commit ABI/relation2 and registry/profile to context; emit artifact2. Do not infer authenticated state or debtor consent.
- [ ] Add compiler/refusal/preimage checks for duplicate/conflicting identities, unknown discharge, absent discharge0, presentzero, create0, full/partial discharge, unchanged untouched debt, sequential overflow/underflow and16/17 identities. Existing admitted liability-free programs remain valid after explicit revision. Run `cargo test -p mori-backend --locked --lib --test compile --test liabilities` with persistent shared target; no ignored/native tests. Capture red/green output and commit task only after checks pass.

### Task 2: Actual proof-library and CLI consumption

**Files:** Modify `crates/mori-backend/src/proving.rs`, `crates/mori-backend/tests/proving.rs`, `crates/mori/tests/native_proofs.rs`; add `examples/core/native-liability/` source/scenario/test manifest. No parallel authority evaluator.

**Interfaces:** Public `prove`, `verify`, `ProofOptions`, complete execution receipt and explicit target retain existing shape unless required new version fields are documented. `ProofBundle`/claim/binding use version2 and explicitly commit ABI/relation profile and trusted registry. Actual trusted slot reconstruction consumes Task1 descriptors, no externally supplied registry or verifier key.

- [ ] Add failing ordinary tests: source duty operations and nonempty registered predecessor must proceed through preparation to the missing-SRS boundary; unknown/mismatched scenario duties refuse before SRS. Internal prepare/public-value tests assert exact slots and complete ordered snapshots/effects, distinguish absent versus presentzero and reject duplicate/out-of-bounds descriptor positions.
- [ ] Replace nonempty-liability/source-effect blanket refusals only with the registry relation's actual consumer. Validate all scenario/step/pre/post duty identities against the source registry. Populate every absent slot presencefalse/amount0 and every present slot true/actualUInt, never amount>0 inference. Match exact Liability versus Discharge statement, identity, before/after amount/presence and effect order against step snapshots. Preserve complete execution in claim and receipt.
- [ ] Set explicit bundle/binding2 commitments including profile/registry; refuse stale version1 input rather than reinterpreting it. Update ordinary format tests and canonical vectors intentionally; do not overwrite historical version1 audit/evidence.
- [ ] Add real CLI consumer tests on newly authored duties, preserving unsupported versus semantic failures, output bounds, no publication on refused scenarios and all current FIFO/Unicode/exclusive-file controls. Add two contrasting source fixtures and a test manifest with hand-derived exact complete effects/poststates; invoke actual `mori test` and compile/simulate consumers. No native tests yet.
- [ ] Run affected ordinary core/backend/CLI suites and fmt/diff checks; inspect errors collectively, repair within approved scope, capture all results. Commit source/consumer/fixtures with report including limits and required native measurement scope.

### Task 3: Quint correspondence, measurement and release review

**Files:** Add `experiments/moriarty-language/formal/quint/language/liability_transition.qnt` and `_test.qnt`; update QA README and `docs/NATIVE-CORE.md`; add exact current scoped result evidence after execution.

- [ ] Model fixed16-cap registry identities, explicit absence/presentzero, sequential accrual/discharge, unchanged untouched duties and atomic refusal. Deterministic witnesses cover zero creation/full discharge/absent discharge0/partial progress/identity substitution/overflow/hidden duty/cap. Keep native authenticity/history/signed authority as external premises.
- [ ] Typecheck/test and run seeded10000-sample simulations with success and refusal witnesses, retain actual output and correspondence to Rust slots/effects; no exhaustive or crypto-soundness claim.
- [ ] Run full ordinary workspace and beta compatibility checks appropriate to changes. Measure exact authored liability circuits at registry widths1,2,16 for k/rows using upstream model/IR only, no SRS/keygen/prove. A shape/cap failure requires an explicit revised design before native work; never omit duties. Freeze exact source/target/scenario hashes and full candidate.
- [ ] Request separate two-provider resource decision for one bounded exact native liability positive and specified fresh-key/changed-statement controls. Do not dispatch on a missing vote, shape drift or consumed prior allocation. Preserve required process/resource/accounting evidence and original failure history.
- [ ] After eligible actual experiment, obtain fresh full exact-candidate and actual-result Sol6.1HIGH/Grok4.6HIGH audits, fix material findings with affected checks and new current pair as needed, publish/merge scoped result. Leave every authenticated ledger/history/recovery/intent/fraud/recursion/full-roadmap exit open until demonstrated.
