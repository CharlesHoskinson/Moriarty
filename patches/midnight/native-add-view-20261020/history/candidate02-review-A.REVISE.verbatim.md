Verdict: **REVISE**.

Reviewer identity: Muse Code powered by muse-wonder-1.0, acting as fresh non-author reviewer 1. This was a static source/API/design review only. I did not rely on hashes as approval, and no compiler, test, native, or Lean execution is claimed.

## Blocking corrections

### 1. The recording guard is not thread-affine in the type system

In [native_add_view.rs](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/batch44-actual-native-add-assignment-correspondence/literal-sdk-owner-candidate02/afterimage/midnight-proofs-0.8.3/src/plonk/native_add_view.rs), `NativeAddRecordingGuard` contains only `CapturePhase` and `Option<RecordedSession>`, both of which are thread-mobile. The guard is therefore structurally `Send`, despite recording being thread-local.

Mandatory correction:

- Make the guard `!Send`, for example with `PhantomData<Rc<()>>` or an equivalent non-`Send` marker.
- Document that the guard must be created, held, and dropped on the same thread.
- Document that nested guards must be dropped in LIFO order.

Without this, moving the guard to another thread makes `Drop` operate on that other thread’s `ACTIVE` slot. It can leave the origin thread’s session active indefinitely and can steal or replace an unrelated session on the drop thread.

Out-of-order nested drops have the same class of problem:
dropping an outer guard first consumes the inner session as `finished`, and dropping the inner guard afterwards restores the already-consumed outer session into `ACTIVE`.

### 2. `join_native_add_view` does not fully validate the NativeAdd shape it claims to produce

`join_native_add_view` checks only that `binding.value_cols` and `binding.coeff_cols` have equal non-zero length, then later constructs a view documented as containing the five `NB_ARITH_COLS` value cells and the corresponding gate cells.

A caller-constructible session with three value and coefficient columns can therefore produce a `JoinedNativeAddView` whose documented “five advice cells” shape is false. The packet explicitly says public forged record dimensions are a validation concern, so this cannot be treated as trusted-binding behaviour.

Mandatory correction:

- Require exactly five value columns and exactly five coefficient columns for this NativeChip view, before any indexing or gate-encoding collection.
- Keep the constant, `q_next`, `mul_ab`, `mul_ac`, and converted `q_arith` checks tied to that fixed shape.

There is also an unchecked arithmetic path:

- `num_original_fixed_columns + binding.q_arith_selector` can overflow for a forged session.
- `add_row + 1` is constructed without a checked bound.

Because `join_native_add_view` returns `Result<_, JoinError>`, mandatory correction is to use checked arithmetic and return a structural `JoinError` instead of panicking in debug builds or wrapping in release builds.

### 3. The public source documentation overstates the supported scope

The module documentation in [native_add_view.rs](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/batch44-actual-native-add-assignment-correspondence/literal-sdk-owner-candidate02/afterimage/midnight-proofs-0.8.3/src/plonk/native_add_view.rs) describes the feature as recording “exactly one `NativeChip` add”, but the actual `NativeChip::linear_combination` source in [native_chip.rs](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/batch44-actual-native-add-assignment-correspondence/literal-sdk-owner-candidate02/afterimage/midnight-circuits-7.2.4/src/field/native/native_chip.rs) has early-return branches:

- empty terms after filtering returns `assign_fixed(constant)`;
- a single remaining term with coefficient `1` and zero constant returns the input clone;
- neither branch creates a `"Linear combination"` region.

Join correctly refuses those cases because the required region is absent, so this is fail-closed. But the public claim is broader than the supported shape.

Mandatory documentation correction:

- State that the supported joined shape requires the ordinary multi-term `linear_combination` path that actually creates one `"Linear combination"` region.
- State that zero-term and singleton-identity adds are unsupported and refuse at join.
- State that recording captures full keygen fixed/selector tables and merged proving advice columns for the synthesized circuit, not only the final joined add cells.
- State that retained `LAST_KEYGEN` / `LAST_PROVING` thread-local sessions hold those encodings until taken or replaced, with the corresponding witness-privacy and memory implications.
- State the armed floor-planner behaviour in the source docs: while armed, `SingleChipLayouter::assign_region` invokes the region-name closure once extra for every region, not only for `"Assign fixed"` and `"Linear combination"`.

The owner prose acknowledges several of these limits, but SDK users see the source documentation. The source documentation is what must be corrected.

### 4. The owner usage fragment is not a ready caller

The owner’s illustrative fragment obtains `Option<RecordedSession>` from `take_last_keygen_recording()` / `take_last_proving_recording()` but passes those values directly to `join_native_add_view`, which requires `RecordedSession`.

This is not candidate source, so it is not a source defect by itself. But it must be corrected before the fragment is presented as caller guidance: explicitly unwrap, match, or error-handle the `Option` values before calling join.

## Static API and hook correspondence that did check out

Subject to the corrections above, I found no additional static API obstruction in the supplied cuts:

- `midnight_proofs::plonk::native_add_view` is exposed from [mod.rs](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/batch44-actual-native-add-assignment-correspondence/literal-sdk-owner-candidate02/afterimage/midnight-proofs-0.8.3/src/plonk/mod.rs).
- `Column::index`, `Selector::index`, and `Column::column_type` are actual public APIs in the supplied circuit source.
- The `NativeChip::configure` binding is built from the actual `value_columns`, fixed-column slice, `q_next_col`, `mul_ab_col`, `mul_ac_col`, `constant_col`, `fixed_values_col`, and `q_arith.index()` locals before `NativeConfig` is constructed.
- In [keygen.rs](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/batch44-actual-native-add-assignment-correspondence/literal-sdk-owner-candidate02/afterimage/midnight-proofs-0.8.3/src/plonk/keygen.rs), fixed and selector recording occurs after synthesis and before `batch_invert_rational` and selector conversion; converted selector recording occurs after `directly_convert_selectors_to_fixed` and before the converted columns are appended.
- In [prover.rs](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/batch44-actual-native-add-assignment-correspondence/literal-sdk-owner-candidate02/afterimage/midnight-proofs-0.8.3/src/plonk/prover.rs), advice recording occurs after each phase synthesis and before that phase’s `batch_invert_rational`.
- In [permutation/keygen.rs](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/batch44-actual-native-add-assignment-correspondence/literal-sdk-owner-candidate02/afterimage/midnight-proofs-0.8.3/src/plonk/permutation/keygen.rs), copy recording occurs after column-position and bounds resolution and before cycle merging, so it is a copy-call log rather than the final permutation mapping.
- In [single_pass.rs](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/batch44-actual-native-add-assignment-correspondence/literal-sdk-owner-candidate02/afterimage/midnight-proofs-0.8.3/src/circuit/floor_planner/single_pass.rs), the inactive path checks `is_recording_active()` before invoking `name()`, and region recording occurs after `region_start` is computed and before it is pushed.
- No production entry point in the supplied diff calls `begin_recording`.
- The join documentation correctly limits the result to structural shape agreement. It does not authenticate same-circuit, same-key, VK/PK provenance, gate satisfaction, copy satisfaction, or verifier acceptance.

## Non-blocking hardening and deferred debts

These are not the basis for the REVISE verdict, but they should remain explicit:

- `PermutationColumnDesc.column_kind_debug` plus `contains("Advice")` works against the supplied pinned `Any` Debug implementation and fails closed if the advice position cannot be uniquely resolved. A typed classification captured from `Any::Advice(_)` at record time would be stronger than relying on Debug text.
- `with_active` holds a `RefMut` while invoking the config-binding closure and while encoding. The actual in-tree closures do not re-enter the recorder, but the public helper contract should either document non-reentrancy or be restructured so user-supplied construction does not run under the borrow.
- Full-table snapshots, TLS retention of the last sessions, and full-table cloning at join are real resource and private-witness retention costs. They are opt-in, but they must be documented as in correction 3.
- Caller-supplied `phase` and `circuit_count`, plus public caller-constructible `RecordedSession` values, provide no provenance. That is acceptable only because join is explicitly downgraded to shape agreement.
- The following remain open and must not be claimed from this candidate: same-circuit/key/VK/PK provenance, arith-gate satisfaction, permutation/copy satisfaction, adversarial accepted-assignment implications, `Fq`/`Fr` canonical-byte to `Fin r` operation preservation, BLST/FFI correctness, verifier relation, ledger acceptance, SDK typecheck/tests, native/target checks, EARS, and any Lean refinement or bridge proof.
- B43 and its checked `Fin` model are not modified by this candidate, and no fake Lean/SDK correspondence is delivered.

Final disposition: **REVISE** for the guard thread-affinity defect, incomplete join shape/bounds validation, and inaccurate public scope documentation.