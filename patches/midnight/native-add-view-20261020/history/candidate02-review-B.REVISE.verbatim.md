## Verdict: REVISE

Top finding: the inactive opt-in path is largely sound, but the guard lifecycle and join validation do not support the unqualified “one production call / exactly one NativeAdd” claims.

No compiler, test, native, or Lean execution was performed.
Hashes were not treated as approval.
The supplied source is sufficient for this static verdict, so abstention is not warranted.

### What checks out statically

- No definite syntax obstruction was found in the seven supplied postimages.
- The terminating import semicolon in `keygen.rs` is present.
- `Column::index`, `Column::column_type`, `Selector::index`, `Rational::evaluate`, and `PrimeField::to_repr` uses match the supplied API cuts.
- `Any`’s supplied `Debug` implementation renders advice columns as `Advice`, so the current `contains("Advice")` check is fail-closed if that rendering changes.
- Production entry points in the supplied bodies do not call `begin_recording`.
- When inactive, the hooks do not encode field elements, clone selector tables, build config vectors, build permutation descriptors, or invoke the floor-planner name closure.
- `record_keygen_fixed_and_selectors` is called before batch inversion and before selectors are consumed or converted.
- `record_converted_selector_fixed` receives the selector columns produced by `directly_convert_selectors_to_fixed`, in selector order.
- `record_copy_pair` is called after permutation-column resolution and row-bounds checks, and before cycle merging.
- `record_advice_columns` is called after proving synthesis and before batch inversion of the current phase columns.
- Region resolution correctly distinguishes absent, unique, and ambiguous outcomes.
- Join requires equal unique bases from both sessions.
- The narrowed claim that join is structural shape agreement, not same-key, same-circuit, gate-satisfaction, copy-satisfaction, or verifier acceptance, is mostly stated honestly.

## Blocking corrections

### 1. Fix guard thread affinity and drop ordering

`NativeAddRecordingGuard` is structurally `Send` because it only contains `CapturePhase` and `Option<RecordedSession>`.
Dropping it on another thread operates on that thread’s `ACTIVE` slot.
It can finalize an unrelated session there, lose the originating thread’s session, and restore `previous` onto the wrong thread.

Out-of-order same-thread drops are also incorrect:

- begin guard A, then guard B;
- drop A: B’s active session is finalized and A’s saved state is restored;
- drop B: A’s session is resurrected as active after its guard is gone.

Mandatory correction:

- Make the guard non-`Send`.
- Require same-thread, LIFO drop, or redesign the TLS state as an explicit stack/session-identity mechanism.
- An out-of-order drop must not finalize the currently active inner session or resurrect an outer session.
- `Drop` should use a non-panicking TLS access path so guard destruction during TLS teardown or unwinding cannot panic inside `Drop`.
- Correct the documentation claim “any previously active session is restored” so it states the enforced discipline.

### 2. Do not hold the TLS `RefMut` across callbacks or conversions

`with_active` holds `borrow_mut()` while invoking the callback.
For `record_native_config_binding_with`, that callback is caller-supplied.
A callback that records, begins a nested session, or drops a guard will panic with a `RefCell` borrow error.
The same borrow is also held across field evaluation, encoding, and `Debug` formatting in other hooks.

Mandatory correction:

- Restructure the hooks so no caller callback, field conversion, encoding, or formatting runs while the session borrow is held.
- At minimum, check activity, release the borrow, construct the value, then reacquire the borrow only to store it.
- Document any remaining panic or reentrancy contract explicitly.

### 3. Make phase and circuit-count claims truthful

`CapturePhase` and `circuit_count` are caller-supplied labels.
They are not measured from the production call.
A guard can span zero calls, several production calls, or a production call of the opposite phase.
In that case hooks merge state unevenly: dimensions and fixed/advice tables overwrite, regions and copy pairs append, and config bindings deduplicate.

Therefore these statements are too strong as written:

- “number of circuits synthesized in this session”;
- “both phases synthesized exactly one circuit”;
- “begin recording one synthesis phase”;
- any implication that taking a session proves it came from exactly one matching production call.

Mandatory correction, one of:

- rename and document both values as caller-declared and state that join checks only those declarations; or
- add production-derived session accounting that refuses zero, repeated, or mixed production calls.

The owner’s illustrative caller fragment is also not compilable as written: `take_last_keygen_recording()` returns `Option<RecordedSession>`, while `join_native_add_view` takes `RecordedSession`.
It must be corrected or explicitly kept as pseudocode.
That fragment is not candidate source, so it is not itself a source defect.

### 4. Constrain the joined structural representation

`join_native_add_view` can currently produce a view whose recorded tables do not contain the cells it names.

Blocking gaps:

- `binding.value_cols` and `binding.coeff_cols` are only required to be non-empty and equal in length.
- The documented “five advice cells” and “five fixed coefficient cells” are not enforced.
- `q_arith_enabled` is computed but not required to be `true`.
- `add_row + 1` is used for `next_out_cell` without a checked bound against `num_rows`.
- Proving advice columns are not required to exist or to contain `add_row` and `add_row + 1`.
- Referenced fixed, selector, and converted-selector table dimensions are only partly checked through individual cell lookups.

Mandatory correction:

- Require exactly `NB_ARITH_COLS`-equivalent recorded lengths: five value columns and five coefficient columns.
- Require `q_arith_enabled == true`, with a named join failure if false.
- Use checked arithmetic for `add_row + 1` and require it to be within `num_rows`.
- Require the proving advice columns for the bound value columns to exist and cover the add row and next row.
- Require internally consistent lengths for every table exposed as a full recorded table in `JoinedNativeAddView`.

This is recorded-shape validation only.
It does not prove gate satisfaction.

### 5. Narrow or substantiate the NativeAdd identification

The current checks do not distinguish a NativeAdd from another two-term `linear_combination`.
A subtraction-shaped or otherwise generic linear-combination region can satisfy the present region, binding, and copy shape checks.
Join records gate fixed encodings but does not validate the add-shaped coefficient pattern.

Mandatory correction, one of:

- Rename and document the result as a linear-combination-shaped structural view, not an established NativeAdd view; or
- validate the recorded add pattern before using the NativeAdd name: `q_arith` enabled, coefficient encodings corresponding to `(-1, 1, 1, 0, 0)` for the result and two operands, and zero constant, `q_next`, `mul_ab`, and `mul_ac` encodings.

The supported subset must also be stated in the source documentation:

- the zero term must actually create the `"Assign fixed"` region rather than come only from the chip cache;
- neither add operand may be filtered out as the cached zero;
- the operation must not fall into the empty or singleton early-return branches of `linear_combination`;
- both phases must use the recording-aware single-pass floor planner;
- no other operation may create additional distinct `"Assign fixed"` or `"Linear combination"` bases;
- no second distinct `NativeChip` binding may be configured under the same guard.

Outside that subset the code often refuses, which is acceptable.
It must not claim coverage of all NativeAdd layouts.

### 6. Correct armed-mode and private-data documentation in the source

While armed, `SingleChipLayouter::assign_region` invokes `name()` once extra for every region of every circuit laid out on that thread.
That closure may panic, mutate interior state, be non-idempotent, or reenter recording.
Armed mode is therefore not behavior-preserving for arbitrary circuits.
The filter for `"Assign fixed"` and `"Linear combination"` happens only after the name has already been computed in the caller.

Mandatory correction:

- State this in `native_add_view` and at the floor-planner hook, not only in owner prose.
- State that supported armed use requires pure, idempotent region-name closures.
- State that any `NativeChip::configure` call on the armed thread can contribute a binding, including configure-only or key-reading contexts.

Private-data retention also needs an in-source correction:

- `LAST_KEYGEN` and `LAST_PROVING` retain complete recorded tables until taken, replaced, or thread exit.
- Proving sessions contain witness advice encodings.
- Any code running on the same thread can call the public `take_last_*` functions.
- Derived `Debug` for `RecordedSession` and `JoinedNativeAddView` prints the full recorded byte tables.

Mandatory correction before execution or adoption:

- Replace derived `Debug` for recorded and joined sessions with redacted summaries, such as dimensions and counts only.
- Document retention until take, replacement, or thread exit.
- Document that callers must take the session promptly and must not treat thread reuse as a privacy boundary.

## Deferred items that do not block this narrowed revision

These remain unproved and must remain unclaimed:

- same-circuit, same-key, VK, or PK provenance;
- actual gate satisfaction;
- actual permutation or copy satisfaction;
- adversarial accepted-assignment implications;
- canonical `Fq`/`Fr` byte decoding into `Fin r`;
- operation preservation across the BLST/FFI boundary;
- verifier acceptance;
- any Lean bridge from this view.

The checked B43 model and proof are not modified by this candidate.
No Lean/SDK correspondence is delivered here.

## Missing evidence after correction

A future approval still needs, at minimum:

- the pinned SDK compiler/typecheck result for both affected crates;
- feature-configuration coverage used by the target build;
- inactive-path behavior tests;
- armed keygen and proving capture tests;
- guard nesting, wrong-thread, reentrancy, and panic tests;
- region-absent, region-ambiguous, zero-operand, singleton, and multi-operation refusal tests.

Until the six blocking corrections above are made, the candidate should not be approved or executed as SDK instrumentation.