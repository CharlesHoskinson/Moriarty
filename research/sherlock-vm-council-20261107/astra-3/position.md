# astra-3 independent position

Base: `8fd34ed794eda7186c32495f715609ec54a9a297`. Lens: scope, sequencing, failure diagnosis and production compatibility. This position precedes reading other members' positions.

Finish the existing native recursion path through a qualified network. Make the dispatch test tractable as a separate task. Preserve the complete interpreter requirements, but make each intermediate result state its exact supported relation and evidence class.

## Keep

- Keep Midnight's native proof stack, a terminal decider, and ZKIRv3 as the execution target. The product contract requires these boundaries. The current run decider verifies the node proof, folds the carried accumulator and performs a pairing check in circuit. Removing that last obligation would change the accepted relation. Sources: `docs/MORIARTY-PRODUCT-CONTRACT.md:19-33`, `crates/mori-recursion/src/run_decider.rs:208-276`.
- Keep explicit program identity, call frames, bounded immutable values, exact UInt128, complete state continuity and residual financial duties. These are language and soundness requirements, rather than optional performance work. Sources: `openspec/changes/sherlock-universal-interpreter-design/design.md:7-35`, `security-requirements.md` in that directory, UISEC-001–011.
- Keep the distinction between semantic execution, helper work, representation capacity, proof cost and ledger fees. Different K, Quint and helper profiles have different clocks and refusal behavior. Sources: `openspec/changes/sherlock-universal-interpreter-design/design.md:25-35`, `docs/sherlock/design.md`, “Profile and observation adapter matrix.”
- Keep the factory loop and its measured heavy-run controls. Use existing commands and records. Public developers must remain able to compile, prove and deploy without factory approval. Sources: `AGENTS.md`, `models/doctorwatson/factory/CONTRACT.md`, `docs/MORIARTY-PRODUCT-CONTRACT.md:3-17,51-62`.

## Improve

### Diagnose A8 as a deployment-specific failure

A8 succeeded on October 25 and became false after the new-wallet deployment. The run records preserve both outcomes. A fresh read-only check today again finds only deployment block 1,032,519 at the currently pinned address. That supports “the required call has not been observed on this deployment.” It does not establish a recursion soundness failure. Sources: `qa/sherlock/runs.jsonl:93-94,391`, `docs/sherlock/HANDOFF-2026-10-26.md:7-17`, [fresh observations](observations.md).

Split diagnosis into artifact/key identity, actual call binding, SDK encoding, proving, balancing, submission, finality and independent readback. Preserve the exact failure boundary. Reuse passed evidence only when its inputs and target still match. Do not restart broad VM design work because a deployment-specific predicate is false.

A8 currently accepts any `ContractCall` at its pinned address. It does not match the intended entry point, statement or post-state. Keep A8 as an inclusion check and require A9/A10 plus candidate-specific bindings for deployment acceptance. A green A8 alone must not mean “recursive settlement works.” Sources: `scripts/qa/check-a8.sh:8-35`, `qa/sherlock/predicates.yaml:70-95`.

### Treat C6 as a test-cost problem until a trace shows otherwise

The last dispatch run stopped at 16,000 seconds with exit 4 and `capExceeded=wall`. The mutation test passed, but factorial had not completed. The handoff reports memory near 50 GB and warns that the service wrapper still reported success. Sources: `docs/sherlock/HANDOFF-2026-10-26.md:15-16`, `qa/sherlock/runs.jsonl:394`, commit `44fa11bcc`.

The test retains a circuit per rule and checks every supported rule at each sixteenth pre-state. It also checks neighboring rules and disabled dispatch. These are concrete sources of work, but their relative costs remain unmeasured. Source: `crates/mori-backend/src/sherlock_machine/c6_dispatch_tests.rs:65-137`.

This consumer invokes `ProofPreimage::check`, explicitly a nonproving preprocessing check. It is not synthesis, key generation, proving or native verification. A completed dispatch receipt must retain that evidence class. Source: `crates/mori-backend/src/sherlock_machine/c6_dispatch_consumer.rs:1-8`.

Measure preparation, circuit construction, witness conversion and each check separately. Start with one state and one rule. Expand by rule family and trace prefix. Investigate bounded circuit retention or separate processes before increasing the wall cap. Preserve the required negative-case coverage and record any reduced diagnostic scope. A reduced reproducer cannot replace the complete acceptance run.

A dispatch receipt alone cannot close U023. Its current evaluator also requires complete exercised-rule consumers, verified fragment receipts and C6 recursion wiring. The inventory still reports `c6_fragments_wired: false`. Make those separate tasks instead of promising that one successful long run completes the predicate. Sources: `qa/holdout/lib/uireq.py:1195-1237`, `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:805-813`.

### Make evidence current and specific

`sherlock-next` reads stored results. Today's displayed counts still come from October 26 at another commit. A source revision, deployment change or chain reset can invalidate only part of that evidence. Bind each acceptance result to its relevant source, configuration, artifacts and target, and report stale results separately. Sources: `models/doctorwatson/factory/CONTRACT.md`, `qa/sherlock/status.json:3-4`, [fresh observations](observations.md).

## Cut

- Cut a replacement zkVM or foreign proof backend from this change. The product contract already fixes the native target. A replacement adds correspondence work without closing A8 or dispatch. Source for the retained boundary: `docs/MORIARTY-PRODUCT-CONTRACT.md:19-33`.
- Cut simultaneous production implementations of every candidate machine layout. Use the existing bounded representation and rule inventory as the baseline. Require measured complete-path improvement and preserved semantics before selecting a replacement. The existing design leaves several realization choices open. Source: `openspec/changes/sherlock-universal-interpreter-design/design.md:7-15,37-57`.
- Cut any dependency that makes complete generic ADT or financial lowering a prerequisite for requalifying the already implemented recursive anchor. Preserve those language obligations as later, separately accepted milestones. A deployed fragment must be named as a fragment. Sources for the broader requirements: `docs/MORIARTY-PRODUCT-CONTRACT.md`, `docs/sherlock/design.md`, “Outcome and retained contract.”
- Cut speculative ISA expansion for ledger-9 signatures, hashes, events or cross-contract calls from the first recovery milestone. First qualify the existing Sherlock relation on the actual production stack. New ledger features do not discharge existing machine or recursion obligations.
- Cut new review schedulers, approval registries and duplicate evidence databases. Extend the factory only where an executable acceptance check or target identity is missing. The current charter retires earlier routing and adoption machinery. Source: `AGENTS.md`.
- Cut aggregate “percent complete” claims from predicate totals. A native fragment, local checker pass, model trace and finalized call answer different questions. Sources: `models/sherlock/README.md:139-140`, `docs/sherlock/midnight-sdk-seam.md:37-45`.

## Implementation plan, with ordered milestones

These are proposed exits. They do not assert that missing capabilities already exist.

1. **Freeze the compatibility baseline.** Record the repository revision, dependency lock, compiler/ZKIR version, SRS identity, relation/key hashes, network ID, genesis, runtime, ledger version and deployment address. Reconcile current Cargo pins with the production release. Record stagenet and Preview as separate targets. Acceptance: a deterministic preflight rejects one changed field at a time and preserves historical evidence.
2. **Requalify the existing recursive endpoint.** Reproduce local proof verification with the selected artifacts. Produce the terminal proof using the actual final call bindings. Complete the positive call, required hostile calls and independent state/effects/fee readback. First use a qualified non-Mainnet target. Use Preview for production ledger-9 rehearsal. Acceptance: candidate-specific evidence closes the equivalent of A8–A10 for that exact target, without claiming full interpreter coverage.
3. **Complete dispatch and the bounded rule inventory.** Instrument the dispatch path, shrink the reproducer, then complete the required positive and negative cases within measured caps. This work can run beside milestone 2 when it uses separate artifacts and obeys the heavy lock. Complete required consumers, canonical boundaries and native coverage for the declared profile. Acceptance: explicit rule-to-consumer-to-check mapping, no HostOnly row presented as constrained, and no nonproving receipt presented as a native proof.
4. **Prove the public compiler path across programs.** Drive two independently specified programs through the same admitted representation and fixed relation inventory. Compare complete traces, proof cuts, resumes and terminal outcomes. Include recursive data and the required financial boundary cases before claiming those features. Acceptance: independent expectations, hostile witness tests, native terminal verification and exact-target execution cover each promoted capability.
5. **Scale and promote the accepted capability.** Re-run the rollup, performance and fee experiments on the selected production-compatible stack. Choose batch shape from complete-path measurements. Repeat the positive/negative/readback suite on Preview, then perform an explicitly scoped Mainnet promotion. Define forward recovery for contract/key changes and preserve existing state and authorities. Full product completion still requires the retained interpreter and financial obligations.

Use `scripts/sherlock-next`, `scripts/doctorwatson heavy --id … --budget … --hypothesis … -- …`, `scripts/doctorwatson check <ID>` and `scripts/doctorwatson stop-rule`. Add new predicates for production compatibility and candidate-specific acceptance where existing checks cannot express them. Record the exact CLI additions before depending on them. Do not implement through retired delivery templates. Sources: `AGENTS.md`, `models/doctorwatson/factory/CONTRACT.md`. Current wall-bound enforcement: `models/doctorwatson/factory/heavy.py:367-419`.

## Assurance strategy

Require a connected argument for each supported profile: source admission → reference transitions → physical encoding → constrained transitions → recursive composition → terminal decider → ledger state/effects. A test at one link does not close its neighbors. The current K and Quint records explicitly describe bounded, partial model coverage. Source: `models/sherlock/README.md:22-54,139-140`.

Use independent complete expected traces for deterministic semantics. Use adversarial witnesses for constraints, including changed program identity, unsupported selectors, malformed lengths, field/range errors, wrong state linkage, altered budgets, dropped duties and disabled-row behavior. Test base/recursive discrimination, accumulator substitution, unauthorized keys and omitted terminal discharge. Retain all UISEC obligations, with a requirement-to-test/evidence map. Source for these obligations: `openspec/changes/sherlock-universal-interpreter-design/security-requirements.md`.

Separate source review, frontend/typecheck results, bounded model checking, execution comparisons, nonproving preimage checks, native proof verification and target acceptance. Every result names scope and unproved premises. Review consensus, SDK callback smoke tests, historical contract existence and a wrapper's exit status do not establish the missing links. Sources: `models/sherlock/README.md`, `docs/sherlock/midnight-sdk-seam.md`, `docs/sherlock/HANDOFF-2026-10-26.md:16`.

At each milestone, require the applicable rejection tests and exact artifact binding before promotion. Run performance measurements separately for cold and reused setup. Record peak memory for the measured process scope, and distinguish timeouts from correctness failures. The existing factory records and budgets are the implementation location, rather than a new workflow system. Source: `models/doctorwatson/factory/CONTRACT.md`.

Reuse the phase-profile work. Its real on/off proof qualification and overhead calibration remain open. The performance consensus explicitly has no agreed speedup row, and warns against adding incompatible phase shares. Require one qualified profile before selecting GPU work, persistent workers or a new aggregation shape. Sources: `openspec/changes/recursion-phase-profile/tasks.md:44-52`, `wiki-llm/recursion-perf/CONSENSUS.md:19`.

## Midnight ledger 9 impact

**Proof system.** Midnight's current proof repository describes PLONK with KZG commitments and an aggregation toolkit. It evolved from Halo2. These facts support retaining the native architecture. They do not establish that Sherlock's particular recursion artifacts are accepted by the production node. [Primary proof-system source](https://github.com/midnightntwrk/midnight-zk).

**ZKIR and verifier keys.** Compact 0.36 emits ZKIR 3.1 and requires runtime 0.21. Recompilation can change deployed keys. The 0.36 cross-contract fix binds the callee entry point and requires replacement of affected caller keys, including removal of old vulnerable keys. Sherlock's handwritten path needs an explicit compatibility test, even though it does not compile all circuits through Compact. [Compact 0.36 release notes](https://docs.midnight.network/relnotes/compact/toolchain-0.36.0).

**Contract calls.** Ledger 9 adds cross-contract calls for unshielded data. This does not establish private multi-parent proof composition. Treat call identity, public arguments and phase effects as explicit adapter obligations. [Ledger 9 release notes](https://docs.midnight.network/relnotes/ledger-9).

**Fees and operational failures.** Ledger 9 changes fee calculation and adds a floor. Current release notes identify single-DUST-spend rejection, a shielded `writeCoin` gas failure, and key-removal fee mismatch with older clients. Qualify only the paths actually exercised, and keep transaction failure distinct from invalid proofs. [Ledger 9 release notes](https://docs.midnight.network/relnotes/ledger-9).

**Target sequence.** Mainnet and reset Preview run ledger 9.1.0.1. Preprod remains on ledger 8.1.3. Preview is the public rehearsal target for ledger 9. Its retained network name does not preserve its old chain. The supported stack includes Midnight.js 5, Wallet SDK 2 and indexer 4.4.101. [Release compatibility overview](https://docs.midnight.network/relnotes/overview), [ledger upgrade](https://docs.midnight.network/relnotes/ledger-9).

The current Sherlock stagenet endpoint reports ledger `=1.0.0` and node `2.0.0-d9729c13`. Cargo already pins `midnight-ledger-v9`, despite the stale ledger-8 inventory page. Thus the task is an exact compatibility reconciliation, not a generic ledger-8-to-9 migration. [Fresh observations](observations.md), `Cargo.toml:27-42`, `wiki/midnight-repositories.md:41`.

## Open questions

1. Which exact production ledger commit, proof dependencies and SRS accept the current handwritten terminal-decider serialization? The release name alone does not answer this.
2. What fraction of dispatch cost comes from retained circuits, repeated preprocessing, witness conversion or trace materialization? The current timeout does not separate them.
3. Can the chosen production client execute the necessary key maintenance safely, or must the first qualified contract use a fresh address? Preserve old contract state and keys until the selected migration is validated.
4. Which currently green predicates consume evidence from a different source revision, wallet, deployment or chain? Refresh by dependency, rather than discarding all historical work.
5. What complete-path latency and memory budget defines a useful first recursive product? Fix that budget before accepting a batch-shape optimization.

## Confidence, with reasons

High confidence in the immediate diagnosis: the current A8 result was reproduced, the prior success is recorded, and dispatch's nonproving scope is explicit in code. High confidence that the current stagenet receipt cannot establish production compatibility by itself. Sources: [fresh observations](observations.md), `qa/sherlock/runs.jsonl:93-94,391,394`, `crates/mori-backend/src/sherlock_machine/c6_dispatch_consumer.rs:1-8`.

Moderate confidence in the proposed sequence. It preserves the existing native path and isolates independently testable boundaries. Low confidence in any predicted speedup or production acceptance date until phase measurements and exact-release compatibility checks exist. Those are proposed experiments, not established results.
