## Verdict: APPROVE — static source only

Reviewer: Muse Code powered by muse-wonder-1.0, acting as fresh non-author reviewer 2. This is not an execution, adoption, typecheck, or security approval. No sibling current verdict was supplied or relied on, and hashes were not treated as approval.

Top finding: the candidate substantively addresses M1–M7 in source. I found no remaining static source blocker in the guard lifecycle, hook borrow discipline, join validation, scope documentation, test predicates, or actual SDK API usage, subject to the unrun checks below.

### Mandatory union assessment

- **M1 — guard identity and drops:** satisfied statically. `NativeAddRecordingGuard` is `!Send`/`!Sync` via `PhantomData<Rc<()>>`. Sessions live on an id-keyed TLS stack. LIFO drop publishes; out-of-order drop removes only its own frame, discards it unpublished, leaves inner frames active, and does not resurrect the dropped frame. `Drop`, publish, and take paths use `try_with`/`try_borrow_mut`, so TLS teardown or unwinding does not introduce a `Drop` panic from these accesses.

- **M2 — no external work under `RefMut`:** satisfied. Hooks first read the active session id, release the borrow, then evaluate/encode/format or invoke the caller callback, and finally store by the captured id into the originating frame. A nested session begun during the callback does not steal the store; a store whose originating frame has ended is silently discarded, and that semantics is documented. Region-name closures remain caller code under the documented armed-mode contract.

- **M3 — truthful labels:** satisfied. `declared_phase` and `declared_circuit_count` are named and documented as caller declarations. Join checks only that both declarations equal `1`; it does not claim to measure exactly one production synthesis, circuit identity, or key provenance.

- **M4 — joined shape validation:** satisfied for caller-constructible records. Join requires exactly five value and five coefficient columns, uses checked arithmetic for the converted `q_arith` column, requires `add_row + 1 < num_rows`, requires `q_arith` enabled at `add_row`, checks every exposed byte/bool table column against `num_rows`, checks converted-selector width, permutation positions, copy-pair bounds, zero-cell zero encoding, and proving advice coverage at both the add row and next row. Structural failures return `JoinError` rather than wrapping.

- **M5 — honest generic scope:** satisfied. The public result is `JoinedLinearCombinationView` / `join_linear_combination_view`, explicitly a generic linear-combination-shaped structural view. It does not validate the add coefficient pattern `(-1, 1, 1, 0, 0)` and does not claim an established `NativeAdd`. The empty-terms, singleton-identity, cached-zero, other-floor-planner, extra-region, and second-binding exclusions are stated and refuse at join.

- **M6 — armed mode, retention, privacy:** satisfied in source docs at both the module and the `SingleChipLayouter::assign_region` hook. Armed mode invokes `name()` once extra for every region before filtering; pure idempotent name closures are required; any armed-thread `NativeChip::configure` can contribute a binding, including configure-only/key-reading calls. Full keygen/proving tables are retained in TLS until taken, replaced, or thread exit; same-thread public take access and prompt-take duty are documented. `RecordedSession` and `JoinedLinearCombinationView` have redacted `Debug` showing dimensions/counts only.

### Tests and actual production consumer

The 18 unit tests are nonvacuous: they assert the inactive callback counter is `0`, stored labels/dimensions/binding, LIFO publication order, out-of-order discard without resurrection, unwind publication, reentrant-callback storage into the originating frame plus separate nested publication, cross-thread isolation, exact named `JoinError` variants for malformed counts/overflow/disabled selector/next-row/advice/fixed dimensions, region absence/ambiguity and binding multiplicity refusal, declaration-only count checking, and redacted `Debug`. The `!Send` doctest is correctly shaped as `compile_fail`. These remain unrun.

The three `MockProver` chip tests use the real `run_test_native_gadget!` harness and distinguish constructed-record tests from synthesis: empty terms and singleton identity produce no `"Linear combination"` base, while a two-term `chip.add` produces exactly one distinct base. They correctly assert no keygen/proving dimensions or tables are fabricated on that path.

The production capture test uses actual APIs correctly on the supplied source: `ComposableChip` is genuinely required for `NativeChip::configure`/`NativeChip::new` and is publicly re-exported through `midnight_circuits::types`; the owner’s one-line import correction is API-sound. `Pow2RangeChip::configure` is, by the supplied body, an inherent public method, so no second trait import is needed. The test then drives real `keygen_vk_with_k`, `keygen_pk`, and `create_proof` over `NativeChip`, with two empty instance columns for one circuit at `K = 6`, and asserts published sessions, dimensions, non-empty tables, a successful structural join, five/five cells, ten gate encodings, enabled `q_arith`, zero encoding, and a non-empty transcript. It makes no verifier, gate-satisfaction, copy-satisfaction, add-pattern, same-key, or field-homomorphism claim. `ParamsKZG::unsafe_setup` is confined to this bounded test.

### Non-blocking residuals

`record_copy_pair` rebuilds and `Debug`-formats permutation descriptors on every active copy call, although its comment says “only on the first recorded call”; the store still only installs them once. This is a truthful-comment/performance cleanup item, not a semantic blocker, because the supplied `Any` `Debug` is pure and no borrow is held. Session ids also wrap after `2^64` allocations in theory, and join clones full tables despite consuming the sessions; both are documented-cost/practicality notes, not blockers.

### Missing premises and required later checks

No compiler, test, or runtime evidence exists yet. Root must still select and run, in an isolated source-repo installation: both-crate typechecks under the intended feature matrix, all 18 unit tests plus the `compile_fail` doctest, the three `MockProver` chip tests, and the one real keygen/`create_proof` capture test. The production test premise is specifically the source-repo layout — proofs automatic test discovery plus its `../circuits` dev-dependency, and circuits’ explicit test target — not the normalized registry manifests, which must not be conflated with it.

Deferred and still unproved: actual runtime pass of the capture test, same-circuit/same-key authentication, gate/copy satisfaction, verifier acceptance, actual add-pattern correspondence, `Fr`→`Fin` homomorphism, BLST/FFI correctness, ledger/EARS/native closure, and any B43 bridge.