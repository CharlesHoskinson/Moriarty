## Vote: `APPROVE_WITH_SCOPE`

Top finding: the new private consumer derives its aux/2 arithmetic and fuel inputs from the decoded program/state rows rather than from test-supplied guards or operand data, and its nonsecurity source logic matches the adopted aux/2 formulas and owner normal specimens.

### Basis for approval

- **Real compilation interface.** The 15-file closure is internally coherent: the harness enters through the private `sherlock` shim, which includes `reference_types`, the already-reviewed arithmetic/fuel witness module, and the layout-A shim. The layout-A shim includes the existing `rows`, `limits`, `constants`, `prepare`, `state`, `state_encode`, and `state_decode` modules plus the two new leaves. No `Machine`/evaluator module is included. The expected set is nine normal tests: five existing arithmetic/fuel witness tests and four new consumer tests.

- **Decoder-first consumer flow.** `materialize_numeric_aux_a` first calls `decode_observed_a(..., DecodeMode::Step)` and propagates any failure. Only after that does it read `root`, `control_rows`, `frame_slots`, `data`, and program constants from the supplied image. This binds the computation to the checked full program, supplied `ProgramBlock`, and state domain. The implementation does not use `Arc` identity or a digest as a substitute for that structural check.

- **Fuel derivation matches §5/§8.**  
  `en_evt` is derived as `outcome_tag == 0 && control_length != 0`.  
  `term` is derived as terminal `Return` at depth zero or `Error` control.  
  Those values are passed to `materialize_fuel`, which computes the zero probe, `charged = en_evt && !term && !zp[16]`, the borrow chain, and the preserved-or-decremented 16-byte post-fuel value. No rule inventory or caller-supplied charge flag is used. This respects the governing rule-agnostic `en_chg` reading and does not depend on the incomplete illustrative fuel list.

- **Binary-operation derivation matches §5/§6.**  
  `en_bin` requires a charged event, a head `Return` control row, positive depth, and head frame tag `2` (`Right`). The raw frame `binary_op` is then mapped as `0→Add`, `1→Sub`, `2→Mul`, `4→Lt`; Eq, Cons, Branch, and other values produce no arithmetic operation. This preserves the intended separation from Eq/Cons dispatch.

- **Headers and fetch match ARH-1/ARH-2 at host-source level.** When an arithmetic operation is selected, `op_l` comes from the actual head Right-frame `valueRef`, and `op_r` comes from the actual head `Return` control `valueRef`. Otherwise both remain the default zero reference. `fetch` selects Constants or Data by `space`, checks the index against the corresponding active length, and clones the actual node row. The decoder has already bound the supplied program block to the prepared block, so using the supplied block’s constant rows after successful decoding is consistent.

- **Type separation matches ARH-3 and T-1/T-2.** Left and right byte arrays are supplied to `materialize_arithmetic` independently only when the corresponding fetched node has `type_id == 0`. A non-UInt operand therefore leaves the arithmetic selectors visible but forces `uu == false` and zero limb blocks. Eq and Cons leave all arithmetic selectors false and keep the header zero.

- **Arithmetic and fuel columns are the host columns already specified.** `NumericAuxA` contains only the header, `ArithmeticWitness`, and `FuelWitness`. It does not pack aux/2 lanes, append Data, validate a transition, construct a post-state as accepted, or claim native/constraint binding. The disabled-shape expectations are consistent: disabled Add/Sub/Mul blocks are zero, `ovf` and `lt_out` are false when `uu` is false, and `h_zero` remains false unless an enabled Mul computes a zero high half.

- **Literal normal expectations are correctly represented.**
  - P-1 checks the exact mixed-space arrangement: constant left operand `7`, Data right operand `5`, fetched header nodes, result `12`, charged fuel `3→2`.
  - A-3 checks `255+1=256` with only the first carry set.
  - S-3 checks `256-1=255` with only the first borrow set.
  - L-4 checks the full-width `2^120` versus `255` subtraction remainder and correctly gives `lt_out == false`.
  - M-6 checks the full MAX×MAX low result, high bytes, `c[16] == 4079`, `c[32] == 0`, `h_sum == 4079`, and overflow.
  - T-1/T-2 check type rejection shape without running limb arithmetic.
  - T-3/T-4 check that Eq and Cons do not activate ArithScratch while fuel charge remains rule-agnostic.
  - The fuel integration cases reuse owner fuel byte values on ordinary Eval controls and explicitly do not claim the original rule lines; that narrower claim is accurately stated.

### Scope of this approval

This approval covers only the private host-side source computation in the two new leaves, within the existing selected A aux/2 design and the supplied unchanged 15-file compilation closure.

It does **not** cover or imply:

- runtime compilation or test passage — all execution remains `UNRUN` for this vote;
- canonical-image acceptance, because the consumer intentionally uses `DecodeMode::Step`, not `Canonical`;
- transition validation, POP/W1 wiring, appendData, post-state acceptance, or rule dispatch completeness;
- aux/2 packing, native witness ABI, circuit constraints, emitter acceptance, or native binding;
- any change to C4, aux/1 prefix, schemas, profiles, public API, or native authority, which remains `0`;
- security guards, hostile cases, runtime guards, or security judgments, which remain outside this Muse nonsecurity audit.

No blocking nonsecurity source defect was found in the reviewed consumer or its normal-test expectations.
