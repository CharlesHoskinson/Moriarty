## Verdict: APPROVE — static source only

Reviewer: Muse Code powered by muse-wonder-1.0, acting as fresh independent non-author reviewer 1. No tools, commands, compilation, tests, or execution were performed. Hashes were not treated as approval.

The current source addresses the mandatory corrections from both prior REVISE opinions at source level. I found no remaining source blocker in SDK API usage, guard lifecycle, hook restructuring, join validation, scope narrowing, privacy documentation, or test construction.

## Prior mandatory corrections

- **M1 — guard affinity and drop order:** addressed. `NativeAddRecordingGuard` is `!Send`/`!Sync` through `PhantomData<Rc<()>>`, has a `compile_fail` doctest for moving it into `std::thread::spawn`, uses session IDs on a TLS stack, publishes only a top-frame drop, discards an out-of-order frame without disturbing inner frames or resurrecting the dropped session, and uses fallible TLS/borrow access so `Drop` does not panic during teardown or unwinding.
- **M2 — no work under `RefMut`:** addressed. Hooks read the active session ID under a brief borrow, release it, perform caller callbacks, `Rational::evaluate`, `PrimeField::to_repr` encoding, selector cloning, or `Debug` formatting externally, then store by session ID. A value whose originating frame ended during external work is silently discarded; that behavior is documented. The remaining theoretical session-ID collision after `u64` wraparound is not a practical blocker.
- **M3 — caller-declared phase/count:** addressed. Fields and documentation are consistently renamed to declared labels, and `join_linear_combination_view` checks only that both declarations equal one. It no longer claims that exactly one matching production synthesis occurred. The earlier invalid `Option`-to-join caller fragment is not retained as executable guidance.
- **M4 — joined shape validation:** addressed. Join requires exactly five value and five coefficient columns, checked conversion-column arithmetic, `add_row + 1 < num_rows`, enabled `q_arith` with `QArithNotEnabled`, advice coverage at the add and next rows, internally consistent lengths for all exposed byte/bool tables, converted-selector width matching the selector table, sequential permutation descriptors, and copy-pair bounds.
- **M5 — generic scope instead of claimed NativeAdd:** addressed. The public result is `JoinedLinearCombinationView`; the module explicitly says the gate encodings are opaque and that the add pattern `(-1, 1, 1, 0, 0)` with zero constant, `q_next`, `mul_ab`, and `mul_ac` is not validated. It also states the zero/cache/filter, singleton, multi-operation, second-binding, and single-pass-floor-planner restrictions.
- **M6 — armed behavior and privacy:** addressed in both module documentation and the floor-planner hook. Armed recording invokes `name()` once extra for every region before filtering, requires pure/idempotent closures, and can record bindings from configure-only calls. Full-table retention, same-thread public `take_last_*` access, absence of a thread-reuse privacy boundary, prompt-take guidance, and redacted `Debug` for sessions and joined views are documented and implemented.
- **M7 — concrete tests:** addressed as source. The recorder module contains 18 unit tests plus the guard `compile_fail` doctest; the circuits crate contains three MockProver chip tests; the proofs crate contains one real `keygen_vk_with_k`/`keygen_pk`/`create_proof` structural-capture test.

## Tests reviewed

The 18 unit tests are non-vacuous for the areas they claim to cover: inactive callback non-invocation, active storage, LIFO and out-of-order drops, unwind publication, reentrant callback plus nested session, thread locality, valid joining, invalid column counts, selector-index overflow, disabled selector, next-row bounds, shortened/ragged tables, absent/ambiguous regions, duplicate bindings, declared-count handling, and redacted session `Debug`. Their predicates match the join and guard code paths.

The three MockProver tests use `run_test_native_gadget!`, `NativeGadget`, and the actual `NativeChip::linear_combination` dispatch. Empty terms, singleton identity, and multi-term add should respectively produce no, no, and exactly one distinct `"Linear combination"` region base. Their assertions that MockProver sessions have no dimensions or keygen/proving tables are consistent with that synthesis path and prevent those sessions from being presented as production joins.

The production test uses an actual `NativeChip`, real key generation, and real `create_proof`. Its constructor/configure path, `ComposableChip` import correction, `Pow2RangeChip::configure` usage, gate/lookup configuration, two instance columns, transcript type, `K = 6`, and structural assertions correspond to the supplied APIs. It correctly avoids verifier, gate-satisfaction, copy-satisfaction, add-pattern, and key-authentication claims. `ParamsKZG::unsafe_setup` remains test-only.

## Required later checks, not source blockers

Before adoption or any runtime claim, Root must separately run the pinned isolated build and tests, including at minimum:

- confirmation that the installed target tree actually has the source-repo proofs manifest behavior: automatic test discovery and the `midnight-circuits` dev-dependency, rather than the normalized registry `Cargo.toml` with autotests disabled;
- confirmation that the workspace resolves circuits against the same local `midnight-proofs` candidate, not a second registry copy;
- typecheck/lint/doc checks for both affected crates under the intended feature set;
- all 18 unit tests, the `compile_fail` doctest, all three MockProver tests, and the one real keygen/proving capture test.

These are unrun runtime and installation premises, not failures identified in the supplied source.

## Deferred and not approved

This approval does not establish actual add-pattern identification, accepted gate or copy satisfaction, same-circuit/same-key/VK/PK provenance, adversarial refinement, `Fr`/`Fq`-to-`Fin` homomorphism, BLST/FFI correctness, verifier or ledger acceptance, EARS closure, native/target closure, or any Lean bridge. B43 remains unchanged and unrerun.